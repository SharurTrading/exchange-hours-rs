// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! Deterministic property + workload coverage for the `market-hours` public
//! schedule contract.
//!
//! `market-hours` is a pure, stateless schedule-arithmetic library, so the
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

use chrono::{DateTime, Duration, Utc};
use exchange_hours::{
    CalendarResolution, Exchange, MarketHours, SessionKind, candle_end, candle_end_with,
    hours_for_exchange, next_session_after, next_session_open_after, session_bounds,
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

    // Session bounds are ordered.
    let bounds = session_bounds(hours, instant);
    let (b_open, b_close) = bounds;
    assert!(
        b_close >= b_open,
        "session close {b_close} precedes open {b_open} ({ctx})"
    );

    // Next session is never in the past and is ordered; the open-only
    // projection agrees with the full bounds.
    let next = next_session_after(hours, instant);
    let (n_open, n_close) = next;
    assert!(
        n_open >= instant,
        "next session open {n_open} precedes query instant {instant} ({ctx})"
    );
    assert!(
        n_close >= n_open,
        "next session close {n_close} precedes open {n_open} ({ctx})"
    );
    assert_eq!(
        n_open,
        next_session_open_after(hours, instant),
        "next_session_open_after disagrees with next_session_after().open ({ctx})"
    );

    // Candle ends never precede the bar start; seconds is a pure offset.
    for res in RESOLUTIONS {
        let end = candle_end(hours, instant, res);
        assert!(
            end >= instant,
            "candle_end({res:?})={end} precedes bar start {instant} ({ctx})"
        );
        if let CalendarResolution::Seconds(secs) = res {
            assert_eq!(
                end,
                instant + Duration::seconds(i64::from(secs)),
                "Seconds({secs}) candle is not a pure offset ({ctx})"
            );
        }
    }

    // candle_end_with path coverage across every session kind.
    for kind in KINDS {
        let end = candle_end_with(hours, instant, CalendarResolution::Daily, kind);
        assert!(
            end >= instant,
            "candle_end_with(Daily, {kind:?})={end} precedes bar start {instant} ({ctx})"
        );
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
                let (open, close) = next_session_after(&hours, cursor);
                let ctx = format!(
                    "[strictly-advancing session walk] seed={seed:#018x} venue={exch:?} \
                     step={step}/{STEPS} cursor={cursor} open={open} close={close}"
                );
                // Advancing by the previous open guarantees strict progress: the
                // next session must open strictly after the cursor, so a stall or
                // backward move (including the degenerate no-session fallback)
                // fails here.
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
