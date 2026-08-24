// SPDX-License-Identifier: MIT-0

//! Session-kind-aware candle starts.

use super::prelude::*;

// ---------------------------------------------------------------------------
// candle_start_with — kind-aware period starts
// ---------------------------------------------------------------------------

#[test]
fn candle_start_with_regular_kind_anchors_the_trading_day_at_rth() {
    let hours = hours_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let monday_mid_rth = ct((2026, 4, 20), (10, 0, 0));

    // Consulting only regular sessions, the Monday trading day runs
    // 08:30–15:15 CT; the overnight wrap belongs to the extended set.
    assert_eq!(
        candle_start_with(
            &hours,
            monday_mid_rth,
            CalendarResolution::Daily,
            SessionKind::Regular
        ),
        Some(ct((2026, 4, 20), (8, 30, 0))),
        "Regular-kind daily start is the RTH open, not the Globex overnight open"
    );
    assert_eq!(
        candle_end_with(
            &hours,
            monday_mid_rth,
            CalendarResolution::Daily,
            SessionKind::Regular
        ),
        Some(ct((2026, 4, 20), (15, 15, 0))),
        "Regular-kind daily end is the RTH close"
    );
}

#[test]
fn candle_start_with_extended_kind_anchors_the_trading_day_at_the_globex_open() {
    let hours = hours_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let monday_mid_rth = ct((2026, 4, 20), (10, 0, 0));

    // Consulting only extended sessions, the Monday trading day opened with
    // Sunday's 17:00 CT electronic open - the 16:00 Pre-Open is order entry,
    // not a session - and its last close is the 15:15-16:00 electronic window.
    assert_eq!(
        candle_start_with(
            &hours,
            monday_mid_rth,
            CalendarResolution::Daily,
            SessionKind::Extended
        ),
        Some(ct((2026, 4, 19), (17, 0, 0))),
        "Extended-kind daily start is the Sunday electronic open, not the Pre-Open"
    );
    assert_eq!(
        candle_end_with(
            &hours,
            monday_mid_rth,
            CalendarResolution::Daily,
            SessionKind::Extended
        ),
        Some(ct((2026, 4, 20), (16, 0, 0))),
        "Extended-kind daily end is the 16:00 CT extended close"
    );
}

#[test]
fn candle_start_with_both_matches_candle_start() {
    let hours = hours_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    for (res, t) in [
        (CalendarResolution::Daily, ct((2026, 1, 29), (18, 0, 0))),
        (
            CalendarResolution::Minutes(5),
            ct((2026, 4, 20), (10, 0, 0)),
        ),
        (CalendarResolution::Weekly, ct((2026, 4, 20), (10, 0, 0))),
    ] {
        assert_eq!(
            candle_start_with(&hours, t, res, SessionKind::Both),
            candle_start(&hours, t, res),
            "candle_start must be exactly candle_start_with over Both ({res:?})"
        );
    }
}

#[test]
fn adjacent_different_session_kinds_remain_distinct_bounds() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let before_handoff = ct((2026, 4, 20), (8, 29, 59));
    let at_handoff = ct((2026, 4, 20), (8, 30, 0));

    assert_eq!(
        calendar.session_bounds_with(before_handoff, SessionKind::Extended),
        Some((ct((2026, 4, 19), (17, 0, 0)), ct((2026, 4, 20), (8, 30, 0)),)),
    );
    assert_eq!(
        calendar.session_bounds(before_handoff),
        Some((ct((2026, 4, 19), (17, 0, 0)), ct((2026, 4, 20), (8, 30, 0)),)),
    );
    assert_eq!(
        calendar.session_bounds(at_handoff),
        Some((
            ct((2026, 4, 20), (8, 30, 0)),
            ct((2026, 4, 20), (15, 15, 0)),
        )),
    );
    assert_eq!(
        calendar.candle_end(
            ct((2026, 4, 20), (8, 28, 0)),
            CalendarResolution::Minutes(5),
        ),
        Some(at_handoff),
        "the natural 08:33 grid end is clamped at the session handoff",
    );
}
