// SPDX-License-Identifier: MIT-0

//! Deterministic property + workload coverage for the `exchange-hours` public
//! schedule contract.
//!
//! `exchange-hours` is a pure, stateless schedule-arithmetic library, so the
//! correct validation class is property/deterministic-fixture rather than a
//! stateful workload. These tests assert structural invariants that must hold
//! for every venue profile and every instant, using a self-contained
//! deterministic PRNG over a fixed seed set so any failure is exactly
//! reproducible.
//!
//! # Invariants proved
//!
//! - **Totality + cross-query consistency.** Every public query (`is_open`, `is_open_with`,
//!   `is_maintenance`, `session_bounds`, `next_session_after`,
//!   `next_session_open_after`, `candle_end`, `candle_end_with`,
//!   `is_closed_all_day_on`) is panic-free and agrees with the public semantic
//!   relationships below.
//! - **`is_open` equals `is_open_with(Both)`** for every instant.
//! - **Maintenance implies closed:** `is_maintenance(t)` is never true while
//!   `is_open(t)` is true.
//! - **Session bounds are ordered:** `session_bounds` and `next_session_after`
//!   never return a close before its open, and the next session never opens in
//!   the past.
//! - **Candle ends never precede the bar start;** `Seconds(s)` is a pure
//!   `t + s` offset.
//! - **Strictly-advancing session walk (deterministic workload).** Repeatedly
//!   advancing by `next_session_after(..).open` yields a strictly increasing,
//!   progress-making sequence of opens — the function never stalls or moves
//!   backward and sessions do not overlap in open order.
//! - **Always-open venues never close** and are never in maintenance.
//! - **DST stability (pinned fixture):** queries are total and ordered across
//!   spring-forward and fall-back transition instants.
//!
//! # Reproducibility
//!
//! Failures print the seed, iteration/step index, the enabled venue/resolution
//! operation space, the venue, and the offending UTC instant
//! (`TEST-DETERMINISM-01`).

#![expect(
    clippy::expect_used,
    reason = "fixture constructors assert their own literals; a bad literal must fail the test"
)]

use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Utc};
use exchange_hours::{
    CalendarResolution, Exchange, MarketHours, SessionKind, candle_end, candle_end_with,
    hours_for_exchange, hours_for_exchange_as_of, next_session_after, next_session_after_with,
    next_session_open_after, session_bounds, session_bounds_with,
};

// ---------------------------------------------------------------------------
// Deterministic PRNG (splitmix64) — no external dependency, fully reproducible.
// ---------------------------------------------------------------------------

/// Advances a `splitmix64` state and returns the next pseudo-random `u64`.
fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Returns a pseudo-random index in `0..n` (treats `n == 0` as `1`).
fn bounded(state: &mut u64, n: usize) -> usize {
    let modulus = u64::try_from(n).unwrap_or(1).max(1);
    usize::try_from(splitmix64(state) % modulus).unwrap_or(0)
}

// ---------------------------------------------------------------------------
// Operation space
// ---------------------------------------------------------------------------

/// Fixed seed set; every randomized assertion is reproducible from these.
const SEEDS: [u64; 3] = [
    0x0000_0000_5EED_0001,
    0x5EED_0000_0BAD_F00D,
    0xD15E_A5ED_1234_5678,
];

/// Sampling window start: `2013-01-01T00:00:00Z`.
const SAMPLE_START_SECS: i64 = 1_356_998_400;

/// Sampling window span (~14 years) so instants span many DST transitions,
/// weekends, and historical-profile cutover dates.
const SAMPLE_SPAN_SECS: u64 = 441_806_400;

/// Representative venues across every product family the crate models.
const PROPERTY_VENUES: [Exchange; 16] = [
    Exchange::Unknown,
    Exchange::Nasdaq,
    Exchange::Nyse,
    Exchange::CboeOptionsC1,
    Exchange::Cme,
    Exchange::Cbot,
    Exchange::Comex,
    Exchange::Nymex,
    Exchange::Cfe,
    Exchange::Eurex,
    Exchange::Iceus,
    Exchange::Iceeu,
    Exchange::Sgx,
    Exchange::Lse,
    Exchange::Xetra,
    Exchange::BinanceFutures,
];

/// Resolutions exercised by `candle_end`: fixed-grid intraday plus the
/// session-aware day/week/month boundaries.
const RESOLUTIONS: [CalendarResolution; 9] = [
    CalendarResolution::Seconds(1),
    CalendarResolution::Seconds(900),
    CalendarResolution::Minutes(1),
    CalendarResolution::Minutes(60),
    CalendarResolution::Hours(1),
    CalendarResolution::Hours(4),
    CalendarResolution::Daily,
    CalendarResolution::Weekly,
    CalendarResolution::Monthly,
];

/// Session-set selectors consulted by the kind-parameterized queries.
const KINDS: [SessionKind; 3] = [
    SessionKind::Regular,
    SessionKind::Extended,
    SessionKind::Both,
];

/// Draws a UTC instant uniformly from the sampling window.
fn random_instant(state: &mut u64) -> DateTime<Utc> {
    let offset = i64::try_from(splitmix64(state) % SAMPLE_SPAN_SECS).unwrap_or(0);
    DateTime::<Utc>::from_timestamp(SAMPLE_START_SECS + offset, 0)
        .expect("sampled timestamp lies within chrono's representable range")
}

// ---------------------------------------------------------------------------
// Shared invariant check applied to a single (venue, instant) sample.
// ---------------------------------------------------------------------------

/// Asserts every per-instant invariant for one venue/instant sample.
///
/// `label` carries the reproduction context (seed, indices, operation space)
/// and is appended to every assertion message.
fn assert_instant_invariants(
    exch: Exchange,
    hours: &MarketHours,
    instant: DateTime<Utc>,
    label: &str,
) {
    let ctx = format!("{label} venue={exch:?} instant={instant}");

    // is_open Both-consistency.
    let open_now = hours.is_open(instant);
    assert_eq!(
        open_now,
        hours.is_open_with(instant, SessionKind::Both),
        "is_open disagrees with is_open_with(Both) ({ctx})"
    );

    // Maintenance implies closed.
    let maintenance = hours.is_maintenance(instant);
    assert!(
        !(maintenance && open_now),
        "venue reported maintenance while open ({ctx})"
    );

    // Session bounds, when they exist, are ordered; an open venue always has
    // containing bounds.
    let bounds = session_bounds(hours, instant);
    if let Some((b_open, b_close)) = bounds {
        // Strict: a present session has positive width — `None` is the only
        // representation of absence, never a zero-width pair.
        assert!(
            b_close > b_open,
            "session close {b_close} does not exceed open {b_open} ({ctx})"
        );
    }
    if open_now {
        let (b_open, b_close) = bounds.expect("an open venue must report session bounds");
        assert!(
            b_open <= instant && instant < b_close,
            "open venue's bounds {b_open}..{b_close} do not contain {instant} ({ctx})"
        );
    }

    // The next session, when it exists, is never in the past and is ordered;
    // the open-only projection agrees with the full bounds.
    let next = next_session_after(hours, instant);
    if let Some((n_open, n_close)) = next {
        assert!(
            n_open >= instant,
            "next session open {n_open} precedes query instant {instant} ({ctx})"
        );
        assert!(
            n_close > n_open,
            "next session close {n_close} does not exceed open {n_open} ({ctx})"
        );
    }
    assert_eq!(
        next.map(|(open, _)| open),
        next_session_open_after(hours, instant),
        "next_session_open_after disagrees with next_session_after().open ({ctx})"
    );

    // Candle ends, when they exist, never precede the bar start; seconds is a
    // pure offset and always exists (the probed intervals are non-zero).
    for res in RESOLUTIONS {
        let end = candle_end(hours, instant, res);
        if let Some(end) = end {
            assert!(
                end >= instant,
                "candle_end({res:?})={end} precedes bar start {instant} ({ctx})"
            );
        }
        if let CalendarResolution::Seconds(secs) = res {
            assert_eq!(
                end,
                Some(instant + Duration::seconds(i64::from(secs))),
                "Seconds({secs}) candle is not a pure offset ({ctx})"
            );
        }
        // A bar's start exists exactly when its end does, and never after it.
        let start = exchange_hours::candle_start(hours, instant, res);
        assert_eq!(
            start.is_some(),
            end.is_some(),
            "candle_start and candle_end disagree on bar existence for {res:?} ({ctx})"
        );
        if let (Some(start), Some(end)) = (start, end) {
            assert!(
                start <= end,
                "candle_start {start} exceeds candle_end {end} for {res:?} ({ctx})"
            );
        }
    }

    // candle_end_with path coverage across every session kind.
    for kind in KINDS {
        if let Some(end) = candle_end_with(hours, instant, CalendarResolution::Daily, kind) {
            assert!(
                end >= instant,
                "candle_end_with(Daily, {kind:?})={end} precedes bar start {instant} ({ctx})"
            );
        }
    }

    // is_closed_all_day_on is total for the venue-local day and coherent with
    // always-open venues checked in a dedicated fixture below.
    let local_day = instant.with_timezone(&hours.tz).date_naive();
    let closed_all_day = hours.is_closed_all_day_on(local_day, SessionKind::Both);
    assert!(
        !(closed_all_day && open_now),
        "venue reported closed all day while open at instant ({ctx})"
    );
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn calendar_queries_are_total_and_consistent_over_random_instants() {
    const ITERATIONS: usize = 256;
    for seed in SEEDS {
        let mut state = seed;
        for iter in 0..ITERATIONS {
            let exch = PROPERTY_VENUES[bounded(&mut state, PROPERTY_VENUES.len())];
            let hours = hours_for_exchange(exch);
            let instant = random_instant(&mut state);
            let label = format!(
                "[total+consistency invariant] seed={seed:#018x} iter={iter}/{ITERATIONS} \
                 venues={} resolutions={} kinds={}",
                PROPERTY_VENUES.len(),
                RESOLUTIONS.len(),
                KINDS.len()
            );
            assert_instant_invariants(exch, &hours, instant, &label);

            // The point-in-time surface must satisfy the same invariants with
            // the profile the venue actually published at the sampled instant
            // — the ~14-year window crosses every recorded cutover, including
            // the pre-go-live no-session profiles.
            let historical = hours_for_exchange_as_of(exch, instant);
            let historical_label = format!("{label} [as-of profile]");
            assert_instant_invariants(exch, &historical, instant, &historical_label);
        }
    }
}

#[test]
fn next_session_after_walk_strictly_advances_opens() {
    const STEPS: usize = 160;
    const WALK_VENUES: [Exchange; 4] = [
        Exchange::Cme,
        Exchange::Eurex,
        Exchange::BinanceFutures,
        Exchange::Unknown,
    ];
    for seed in SEEDS {
        let mut state = seed;
        for exch in WALK_VENUES {
            let hours = hours_for_exchange(exch);
            let mut cursor = random_instant(&mut state);
            for step in 0..STEPS {
                let (open, close) = next_session_after(&hours, cursor)
                    .expect("walk venues run sessions every week");
                let ctx = format!(
                    "[strictly-advancing session walk] seed={seed:#018x} venue={exch:?} \
                     step={step}/{STEPS} cursor={cursor} open={open} close={close}"
                );
                // Advancing by the previous open guarantees strict progress: the
                // next session must open strictly after the cursor, so a stall
                // or backward move fails here.
                assert!(
                    open > cursor,
                    "session walk stalled or moved backward ({ctx})"
                );
                assert!(
                    close >= open,
                    "session close precedes open during walk ({ctx})"
                );
                cursor = open;
            }
        }
    }
}

#[test]
fn always_open_venues_stay_open_without_maintenance() {
    const ITERATIONS: usize = 128;
    const ALWAYS_OPEN: [Exchange; 2] = [Exchange::BinanceFutures, Exchange::Unknown];
    for seed in SEEDS {
        let mut state = seed;
        for exch in ALWAYS_OPEN {
            let hours = hours_for_exchange(exch);
            for iter in 0..ITERATIONS {
                let instant = random_instant(&mut state);
                let ctx = format!(
                    "[always-open invariant] seed={seed:#018x} venue={exch:?} \
                     iter={iter}/{ITERATIONS} instant={instant}"
                );
                assert!(
                    hours.is_open(instant),
                    "always-open venue reported closed ({ctx})"
                );
                assert!(
                    !hours.is_maintenance(instant),
                    "always-open venue reported maintenance ({ctx})"
                );
                assert!(
                    !hours.is_closed_all_day_on(
                        instant.with_timezone(&hours.tz).date_naive(),
                        SessionKind::Both,
                    ),
                    "always-open venue reported closed all day ({ctx})"
                );
            }
        }
    }
}

#[test]
fn dst_transition_queries_are_stable_and_total() {
    use chrono::TimeZone;

    let hours = hours_for_exchange(Exchange::Cme);
    // UTC instants bracketing the US 2025 spring-forward (Mar 9) and fall-back
    // (Nov 2) transitions, where local wall-clock mapping is a gap or a fold.
    let instants = [
        Utc.with_ymd_and_hms(2025, 3, 9, 6, 0, 0)
            .single()
            .expect("valid UTC instant"),
        Utc.with_ymd_and_hms(2025, 3, 9, 7, 0, 0)
            .single()
            .expect("valid UTC instant"),
        Utc.with_ymd_and_hms(2025, 3, 9, 8, 0, 0)
            .single()
            .expect("valid UTC instant"),
        Utc.with_ymd_and_hms(2025, 11, 2, 5, 0, 0)
            .single()
            .expect("valid UTC instant"),
        Utc.with_ymd_and_hms(2025, 11, 2, 6, 0, 0)
            .single()
            .expect("valid UTC instant"),
        Utc.with_ymd_and_hms(2025, 11, 2, 7, 0, 0)
            .single()
            .expect("valid UTC instant"),
    ];
    for instant in instants {
        assert_instant_invariants(
            Exchange::Cme,
            &hours,
            instant,
            "[pinned DST stability fixture]",
        );
    }
}

// ---------------------------------------------------------------------------
// Cross-query consistency fence
//
// `is_open_with` and `session_bounds_with` are two views of one fact: whether a
// session covers an instant. They are computed by separate code paths, so they
// can drift — and they did. A wrap rule enabled on a weekday used to contribute
// its *close* side that day as well as its open side, which reported venues
// whose wrap closes late in the day (COMEX/NYMEX 17:00→16:00 CT, ICE US
// 20:00→18:00 ET) open all Sunday morning while `session_bounds` correctly
// returned the Sunday-evening session. `is_open` said yes, the bounds said no.
//
// The invariant below is the fence: for every venue, every session kind, and
// every instant in the grid, `is_open_with(t, kind)` must equal
// "`session_bounds_with(hours, t, kind)` contains t". `None` from a venue with
// no rules contains nothing, which is the correct `false`.
// ---------------------------------------------------------------------------

/// Number of [`Exchange`] variants. `Exchange` is `#[non_exhaustive]`, so
/// this crate cannot hold the compile-time coverage fence any more — that
/// fence is the wildcard-free match in `Exchange::as_str` inside the library.
/// This count and the independent list below are the runtime companions:
/// `all_exchanges_matches_the_crates_own_list` cross-checks them against
/// `Exchange::ALL`, so the grids fail loudly instead of silently shrinking.
/// Bump all three together (`as_str`, `Exchange::ALL`, and this pair).
const EXCHANGE_VARIANT_COUNT: usize = 69;

/// Every [`Exchange`] variant, maintained by hand and on purpose
/// independently of `Exchange::ALL`: the two lists are compared element by
/// element below, so an omission in either is caught by the other.
const ALL_EXCHANGES: &[Exchange] = &[
    Exchange::Unknown,
    Exchange::Nasdaq,
    Exchange::NasdaqBx,
    Exchange::NasdaqPsx,
    Exchange::CboeBzx,
    Exchange::CboeByx,
    Exchange::CboeEdga,
    Exchange::CboeEdgx,
    Exchange::Nyse,
    Exchange::NyseArca,
    Exchange::NyseAmerican,
    Exchange::NyseNational,
    Exchange::NyseTexas,
    Exchange::MemxEq,
    Exchange::MiaxPearlEq,
    Exchange::Iex,
    Exchange::IntelligentcrossIqx,
    Exchange::BlueOceanAts,
    Exchange::FinraTrfCarteret,
    Exchange::FinraTrfChicago,
    Exchange::FinraTrfNyse,
    Exchange::CboeOptionsC1,
    Exchange::CboeC2Options,
    Exchange::CboeBzxOptions,
    Exchange::CboeEdgxOptions,
    Exchange::NyseArcaOptions,
    Exchange::NyseAmericanOptions,
    Exchange::NasdaqPhlx,
    Exchange::NasdaqIse,
    Exchange::NasdaqNom,
    Exchange::NasdaqMrx,
    Exchange::NasdaqGemx,
    Exchange::NasdaqBxOptions,
    Exchange::MiaxOptions,
    Exchange::MiaxEmeraldOptions,
    Exchange::MiaxPearlOptions,
    Exchange::MiaxSapphireOptions,
    Exchange::BoxOptions,
    Exchange::MemxOptions,
    Exchange::Cme,
    Exchange::Cbot,
    Exchange::Comex,
    Exchange::Nymex,
    Exchange::Cfe,
    Exchange::Eurex,
    Exchange::Eex,
    Exchange::Iceus,
    Exchange::Iceeu,
    Exchange::IceEuropeCommodities,
    Exchange::IceEuropeFinancials,
    Exchange::IceEndex,
    Exchange::IceAbuDhabi,
    Exchange::IceCanada,
    Exchange::Sgx,
    Exchange::Lse,
    Exchange::Xetra,
    Exchange::Six,
    Exchange::EuronextParis,
    Exchange::EuronextAmsterdam,
    Exchange::EuronextBrussels,
    Exchange::EuronextLisbon,
    Exchange::EuronextDublin,
    Exchange::EuronextMilan,
    Exchange::Bme,
    Exchange::NasdaqStockholm,
    Exchange::NasdaqHelsinki,
    Exchange::NasdaqCopenhagen,
    Exchange::Vienna,
    Exchange::BinanceFutures,
];

#[test]
fn all_exchanges_matches_the_crates_own_list() {
    // `Exchange` is `#[non_exhaustive]`, so this crate can no longer hold an
    // exhaustive match as the coverage fence — that fence is the wildcard-free
    // match in `Exchange::as_str` inside the library. What this test pins
    // instead: the hand-maintained list above agrees element-by-element (and
    // in order) with `Exchange::ALL`, and both carry the declared count. An
    // omission in either list is caught by the other; a variant missing from
    // *both* is caught by the library's own `as_str` fence at compile time.
    assert_eq!(
        ALL_EXCHANGES,
        Exchange::ALL,
        "the test's independent list and Exchange::ALL disagree; a new \
         variant was added to one but not the other"
    );
    assert_eq!(
        ALL_EXCHANGES.len(),
        EXCHANGE_VARIANT_COUNT,
        "ALL_EXCHANGES lost or gained an entry; update it together with \
         Exchange::ALL, Exchange::as_str, and EXCHANGE_VARIANT_COUNT or the \
         grids skip a venue"
    );
    assert!(
        Exchange::ALL.windows(2).all(|pair| pair[0] < pair[1]),
        "Exchange::ALL must stay in declaration (Ord) order with no duplicates"
    );
}

#[test]
fn every_exchange_name_round_trips_through_serde_display_and_from_str() {
    // One venue has exactly one snake_case name, shared by serde, `as_str`,
    // `Display`, and `FromStr`. `as_str` is a hand-written second copy of the
    // serde rename table, so every variant is checked both ways; `FromStr`
    // searches `Exchange::ALL` by `as_str`, so its agreement is implied but
    // pinned anyway.
    for &exchange in Exchange::ALL {
        let name = exchange.as_str();
        let serde_form = serde_json::to_value(exchange).expect("serializes");
        assert_eq!(
            serde_form,
            serde_json::Value::String(name.to_owned()),
            "{exchange:?}: as_str and the serde wire form disagree"
        );
        assert_eq!(
            exchange.to_string(),
            name,
            "{exchange:?}: Display and as_str disagree"
        );
        assert_eq!(
            name.parse::<Exchange>(),
            Ok(exchange),
            "{exchange:?}: FromStr does not round-trip its own name"
        );
    }
}

#[test]
fn from_str_rejects_unrecognized_names_instead_of_defaulting() {
    // A typo must surface as an error a caller can see — never silently
    // become `Exchange::Unknown`. `"unknown"` itself parses, because that is
    // the canonical name a caller uses to *choose* the fallback explicitly.
    for bad in ["", "CME", "nyse-arca", "cme ", "totally_made_up"] {
        let parsed = bad.parse::<Exchange>();
        assert!(parsed.is_err(), "{bad:?} must not parse, got {parsed:?}");
        let error = parsed.expect_err("checked above");
        assert_eq!(error.input(), bad, "the error must carry the bad input");
        assert!(
            !error.to_string().is_empty(),
            "the error must render a message"
        );
    }
    assert_eq!("unknown".parse::<Exchange>(), Ok(Exchange::Unknown));
}

/// Builds the UTC instant for `ssm` seconds after local midnight on `day` in
/// `tz`, biased earliest across a fall-back hour. Returns `None` for a
/// wall-clock that does not exist (spring-forward gap) or for the end-of-day
/// sentinel `86_400`, neither of which is a usable grid sample.
fn local_sample(tz: chrono_tz::Tz, day: NaiveDate, ssm: u32) -> Option<DateTime<Utc>> {
    if ssm >= 86_400 {
        return None;
    }
    tz.with_ymd_and_hms(
        day.year(),
        day.month(),
        day.day(),
        ssm / 3600,
        (ssm % 3600) / 60,
        ssm % 60,
    )
    .earliest()
    .map(|local| local.with_timezone(&Utc))
}

/// The instants each venue is probed at.
///
/// Three overlapping layers, because each catches a different failure shape:
/// an hourly sweep of a full reference week (weekends, overnight sessions, and
/// maintenance gaps); the exact open and close instant of every rule on every
/// weekday it is enabled, plus one second either side (off-by-one at a boundary,
/// and the wrap open/close sides); and hourly sweeps across four DST transitions
/// covering the US and EU shift dates, which fall on different days.
fn probe_instants(hours: &MarketHours) -> Vec<DateTime<Utc>> {
    // Sunday 2026-04-19 through Monday 2026-04-27, the reference week the
    // per-venue suite pins.
    let week_start = Utc
        .with_ymd_and_hms(2026, 4, 19, 0, 0, 0)
        .single()
        .expect("valid reference week start");

    let mut instants: Vec<DateTime<Utc>> = (0..8 * 24)
        .map(|hour| week_start + Duration::hours(hour))
        .collect();

    // Rule-derived boundaries across the reference week.
    let first_day = week_start.with_timezone(&hours.tz).date_naive();
    for offset in 0..9 {
        let Some(day) = first_day.checked_add_signed(Duration::days(offset)) else {
            continue;
        };
        for rule in hours.regular.iter().chain(hours.extended.iter()) {
            for ssm in [rule.open_ssm, rule.close_ssm] {
                // The end-exclusive `close_ssm == 86_400` sentinel is local
                // midnight of the next day; probe it there so full-day rules
                // get their close boundary +/-1s like every other rule.
                let (boundary_day, boundary_ssm) = if ssm == 86_400 {
                    let Some(next_day) = day.checked_add_signed(Duration::days(1)) else {
                        continue;
                    };
                    (next_day, 0)
                } else {
                    (day, ssm)
                };
                let Some(boundary) = local_sample(hours.tz, boundary_day, boundary_ssm) else {
                    continue;
                };
                instants.push(boundary - Duration::seconds(1));
                instants.push(boundary);
                instants.push(boundary + Duration::seconds(1));
            }
        }
    }

    // DST transitions: US spring-forward / fall-back and the EU equivalents,
    // which land on different dates. Swept hourly across the whole local day.
    for (year, month, day) in [(2026, 3, 8), (2026, 11, 1), (2026, 3, 29), (2026, 10, 25)] {
        let Some(midnight) = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).single() else {
            continue;
        };
        for hour in -6..30 {
            instants.push(midnight + Duration::hours(hour));
        }
    }

    instants
}

#[test]
fn is_open_agrees_with_session_bounds_for_every_venue_and_instant() {
    let kinds = [
        SessionKind::Regular,
        SessionKind::Extended,
        SessionKind::Both,
    ];
    let mut checked = 0_u32;

    for &exchange in ALL_EXCHANGES {
        let hours = hours_for_exchange(exchange);
        for instant in probe_instants(&hours) {
            for kind in kinds {
                let bounds = session_bounds_with(&hours, instant, kind);
                let contained =
                    bounds.is_some_and(|(open, close)| open <= instant && instant < close);
                assert_eq!(
                    hours.is_open_with(instant, kind),
                    contained,
                    "{exchange:?} / {kind:?}: is_open_with disagrees with \
                     session_bounds_with at {instant} (bounds {bounds:?}, \
                     venue-local {})",
                    instant.with_timezone(&hours.tz),
                );
                checked += 1;
            }
        }
    }

    assert!(
        checked > 50_000,
        "grid collapsed to {checked} checks; the probe set is no longer covering the venues"
    );
}

#[test]
fn closed_instants_agree_with_the_next_session_for_every_venue() {
    // The other half of the contract: when a venue is closed, the bounds it
    // reports are the *next* session, which must be exactly what
    // `next_session_after_with` reports and must not start in the past.
    for &exchange in ALL_EXCHANGES {
        let hours = hours_for_exchange(exchange);
        for instant in probe_instants(&hours) {
            for kind in [
                SessionKind::Regular,
                SessionKind::Extended,
                SessionKind::Both,
            ] {
                if hours.is_open_with(instant, kind) {
                    continue;
                }
                let bounds = session_bounds_with(&hours, instant, kind);
                let next = next_session_after_with(&hours, instant, kind);
                assert_eq!(
                    bounds.map(|(open, _)| open),
                    next.map(|(open, _)| open),
                    "{exchange:?} / {kind:?}: closed at {instant} but session_bounds_with \
                     and next_session_after_with disagree on the next open"
                );
                if let Some((next_open, _)) = next {
                    assert!(
                        next_open >= instant,
                        "{exchange:?} / {kind:?}: next session at {next_open} precedes {instant}"
                    );
                }
            }
        }
    }
}

#[test]
fn every_shipped_rule_table_satisfies_the_session_rule_domain() {
    use chrono::TimeZone;
    use exchange_hours::{MarketHoursKey, hours_for_exchange_as_of, session_profile};

    // Epochs on both sides of every recorded cutover, so the historical
    // profiles (including pre-go-live empties) are validated too.
    let epochs = [
        Utc.with_ymd_and_hms(2012, 6, 1, 12, 0, 0)
            .single()
            .expect("valid UTC instant"),
        Utc.with_ymd_and_hms(2015, 6, 1, 12, 0, 0)
            .single()
            .expect("valid UTC instant"),
        Utc.with_ymd_and_hms(2020, 6, 1, 12, 0, 0)
            .single()
            .expect("valid UTC instant"),
        Utc.with_ymd_and_hms(2026, 6, 1, 12, 0, 0)
            .single()
            .expect("valid UTC instant"),
    ];

    let validate_all = |hours: &MarketHours, source: &str| {
        for rule in hours.regular.iter().chain(hours.extended.iter()) {
            rule.validate().unwrap_or_else(|violation| {
                panic!("{source}: shipped rule {rule:?} violates the domain: {violation}")
            });
        }
    };

    for &exchange in ALL_EXCHANGES {
        validate_all(&hours_for_exchange(exchange), &format!("{exchange:?}"));
        for epoch in epochs {
            validate_all(
                &hours_for_exchange_as_of(exchange, epoch),
                &format!("{exchange:?} as of {epoch}"),
            );
        }
    }

    // The shared futures profiles addressed by key. `MarketHoursKey` is
    // non_exhaustive for downstream code; this in-repo list tracks the enum.
    for key in [
        MarketHoursKey::GlobexEquityIndex,
        MarketHoursKey::GlobexEnergy,
        MarketHoursKey::GlobexGrains,
        MarketHoursKey::GlobexFx,
        MarketHoursKey::CfeVix,
        MarketHoursKey::Eurex,
        MarketHoursKey::IceUs,
        MarketHoursKey::Sgx,
        MarketHoursKey::AlwaysOpen,
    ] {
        let profile = session_profile(key);
        for rule in profile.regular.iter().chain(profile.extended.iter()) {
            rule.validate().unwrap_or_else(|violation| {
                panic!("{key:?}: shipped rule {rule:?} violates the domain: {violation}")
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Point-in-time fence: the as-of surface
//
// `hours_for_exchange_as_of` selects between whole profiles, so the grid fence
// above (which sweeps `hours_for_exchange` only) never exercised the
// historical tables. Two properties close that gap: every historical profile
// must hold the same is_open/session_bounds agreement, and the cutover
// boundary itself must flip at the venue-local midnight of the effective date
// — never an hour early or late through a UTC comparison.
// ---------------------------------------------------------------------------

/// A calendar date as `(year, month, day)`.
type Ymd = (i32, u32, u32);

/// Every venue with a recorded cutover, with a UTC probe instant safely inside
/// each regime. CFE has three regimes; everything else has two.
const CUTOVER_EPOCHS: &[(Exchange, &[Ymd])] = &[
    (Exchange::CboeOptionsC1, &[(2024, 2, 1), (2025, 2, 1)]),
    (Exchange::Cme, &[(2015, 6, 1), (2017, 6, 1)]),
    (Exchange::Eurex, &[(2018, 6, 1), (2019, 6, 1)]),
    (Exchange::Iex, &[(2015, 2, 1), (2016, 2, 1)]),
    (Exchange::Cfe, &[(2013, 6, 1), (2018, 6, 1), (2022, 6, 1)]),
    (Exchange::NyseTexas, &[(2024, 6, 1), (2025, 6, 1)]),
    (Exchange::BlueOceanAts, &[(2021, 6, 1), (2022, 6, 1)]),
    (Exchange::Cbot, &[(2012, 6, 1), (2014, 6, 1)]),
    (Exchange::CboeEdgx, &[(2020, 6, 1), (2022, 6, 1)]),
    (Exchange::CboeBzx, &[(2024, 6, 1), (2026, 6, 1)]),
];

#[test]
fn historical_profiles_hold_the_cross_query_fence() {
    let kinds = [
        SessionKind::Regular,
        SessionKind::Extended,
        SessionKind::Both,
    ];
    let mut checked = 0_u32;

    for &(exchange, epochs) in CUTOVER_EPOCHS {
        for &(year, month, day) in epochs {
            let epoch = Utc
                .with_ymd_and_hms(year, month, day, 12, 0, 0)
                .single()
                .expect("valid UTC epoch");
            let hours = hours_for_exchange_as_of(exchange, epoch);
            for instant in probe_instants(&hours) {
                for kind in kinds {
                    let bounds = session_bounds_with(&hours, instant, kind);
                    let contained =
                        bounds.is_some_and(|(open, close)| open <= instant && instant < close);
                    assert_eq!(
                        hours.is_open_with(instant, kind),
                        contained,
                        "{exchange:?} as of {epoch} / {kind:?}: is_open_with disagrees with \
                         session_bounds_with at {instant} (bounds {bounds:?}, \
                         venue-local {})",
                        instant.with_timezone(&hours.tz),
                    );
                    checked += 1;
                }
            }
        }
    }

    // 21 regimes (10 venues; CFE has three) x ~450 probe instants x 3 kinds. The floor guards against the
    // probe set silently collapsing, not an exact count.
    assert!(
        checked > 15_000,
        "as-of grid collapsed to {checked} checks; the probe set no longer covers the regimes"
    );
}

#[test]
fn cutovers_flip_exactly_at_the_venue_local_midnight() {
    // (venue, effective local date, venue tz). The new profile applies from
    // the venue-local midnight of the effective date: one second before still
    // sees the old hours.
    let cutovers: &[(Exchange, Ymd, chrono_tz::Tz)] = &[
        (
            Exchange::CboeOptionsC1,
            (2024, 8, 26),
            chrono_tz::America::New_York,
        ),
        (Exchange::Cme, (2016, 3, 4), chrono_tz::US::Central),
        (Exchange::Eurex, (2018, 12, 10), chrono_tz::Europe::Berlin),
        (Exchange::Iex, (2015, 8, 21), chrono_tz::America::New_York),
        (Exchange::Cfe, (2014, 6, 1), chrono_tz::US::Central),
        (Exchange::Cfe, (2021, 12, 6), chrono_tz::US::Central),
        (
            Exchange::NyseTexas,
            (2025, 3, 31),
            chrono_tz::America::New_York,
        ),
        (
            Exchange::BlueOceanAts,
            (2021, 10, 5),
            chrono_tz::America::New_York,
        ),
        (Exchange::Cbot, (2013, 4, 8), chrono_tz::US::Central),
        (
            Exchange::CboeEdgx,
            (2021, 3, 8),
            chrono_tz::America::New_York,
        ),
        (
            Exchange::CboeBzx,
            (2025, 5, 1),
            chrono_tz::America::New_York,
        ),
    ];

    for &(exchange, (year, month, day), tz) in cutovers {
        let local_midnight = tz
            .with_ymd_and_hms(year, month, day, 0, 0, 0)
            .earliest()
            .expect("cutover midnight resolves in the venue zone");
        let at = local_midnight.with_timezone(&Utc);
        let just_before = at - Duration::seconds(1);

        let hours_before = hours_for_exchange_as_of(exchange, just_before);
        let hours_at = hours_for_exchange_as_of(exchange, at);

        assert_ne!(
            (hours_before.regular.clone(), hours_before.extended.clone()),
            (hours_at.regular.clone(), hours_at.extended.clone()),
            "{exchange:?}: the {year}-{month:02}-{day:02} cutover did not change the profile \
             at venue-local midnight"
        );
        // One second after the boundary must already be stable on the new side.
        assert_eq!(
            hours_at,
            hours_for_exchange_as_of(exchange, at + Duration::seconds(1)),
            "{exchange:?}: profile unstable immediately after the cutover instant"
        );
        // And well inside each regime the boundary profiles match the epochs.
        assert_eq!(
            hours_before,
            hours_for_exchange_as_of(exchange, just_before - Duration::days(30)),
            "{exchange:?}: pre-cutover profile differs from the deep pre-cutover epoch"
        );
    }
}
