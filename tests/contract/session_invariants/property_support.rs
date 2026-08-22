// SPDX-License-Identifier: MIT-0

//! Deterministic generators and per-instant property assertions.

use super::prelude::*;

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
pub(super) fn bounded(state: &mut u64, n: usize) -> usize {
    let modulus = u64::try_from(n).unwrap_or(1).max(1);
    usize::try_from(splitmix64(state) % modulus).unwrap_or(0)
}

// ---------------------------------------------------------------------------
// Operation space
// ---------------------------------------------------------------------------

/// Fixed seed set; every randomized assertion is reproducible from these.
pub(super) const SEEDS: [u64; 3] = [
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
pub(super) const PROPERTY_VENUES: [Exchange; 19] = [
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
    Exchange::Asx,
    Exchange::NseIndia,
    Exchange::Hkex,
    Exchange::Lse,
    Exchange::Xetra,
    Exchange::BinanceFutures,
];

/// Resolutions exercised by `candle_end`: fixed-grid intraday plus the
/// session-aware day/week/month boundaries.
pub(super) const RESOLUTIONS: [CalendarResolution; 9] = [
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
pub(super) const KINDS: [SessionKind; 3] = [
    SessionKind::Regular,
    SessionKind::Extended,
    SessionKind::Both,
];

/// Draws a UTC instant uniformly from the sampling window.
pub(super) fn random_instant(state: &mut u64) -> DateTime<Utc> {
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
pub(super) fn assert_instant_invariants(
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
