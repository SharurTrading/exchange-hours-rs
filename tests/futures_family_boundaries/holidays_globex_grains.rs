// SPDX-License-Identifier: MIT-0

//! `globex_grains` built-in holiday rows, 2025-2027 (LAW-HOLIDAY-SCOPE).
//!
//! Every probe is stated in America/Chicago wall clock and converted, so a DST
//! slip in either direction fails rather than passing on a coincidence. The
//! family's sourced grid wraps — an evening leg at 19:00 CT running to 07:45 CT
//! the next morning, then an 08:30-13:20 CT day session whose close is the
//! trading day's final close — so each case below is chosen to exercise the
//! clip on the civil day the operator states it, not on the day the session
//! opened.
//!
//! The dates are CME's own: Christmas 2025 (Thursday closure, 12:05 CT
//! Christmas Eve, no Friday evening leg), the day after Thanksgiving 2025
//! (08:30-12:05 CT and nothing else) and Martin Luther King Jr. Day 2025 (a
//! Monday whose trade date does not exist, while the Monday 19:00 CT open that
//! begins Tuesday's trade date runs normally).

use chrono::{DateTime, Duration, NaiveDate, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, Holiday, HolidayKind, MarketHoursKey, SessionKind,
    calendar_for_market_hours_key, hours_for_market_hours_key,
};

const ZC: MarketHoursKey = MarketHoursKey::GlobexGrains;

/// A probe instant stated in the venue's own wall clock.
fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous Central instant")
        .with_timezone(&Utc)
}

fn day(date: (i32, u32, u32)) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.0, date.1, date.2).expect("fixture must be a valid date")
}

fn open_at(instant: DateTime<Utc>) -> bool {
    calendar_for_market_hours_key(ZC).is_open(instant)
}

/// Case 1 — a closed trade date removes its whole trading day, including the
/// leg that opened the previous evening, and leaves the evening leg that begins
/// the *next* trade date alone.
#[test]
fn christmas_2025_closes_its_trade_date_and_the_prior_evening_leg() {
    let calendar = calendar_for_market_hours_key(ZC);
    let closed = day((2025, 12, 25));

    assert_eq!(
        calendar.holiday_on(closed).map(Holiday::kind),
        Some(HolidayKind::Closed),
    );
    assert!(calendar.is_closed_trade_date(closed, SessionKind::Both));

    for probe in [(2, 0, 0), (9, 0, 0), (12, 0, 0), (20, 0, 0)] {
        assert!(
            !open_at(ct((2025, 12, 25), probe)),
            "no session belongs to trade date 2025-12-25 at {probe:?} CT"
        );
    }

    // The Wednesday-evening leg would have carried trade date 2025-12-25.
    assert!(!open_at(ct((2025, 12, 24), (19, 0, 0))));
    assert!(!open_at(ct((2025, 12, 24), (23, 30, 0))));
}

/// Case 1, the converse — the Monday holiday's own trade date is gone, but the
/// Monday 19:00 CT open that begins Tuesday's trade date is untouched.
#[test]
fn mlk_2025_removes_the_trade_date_not_the_evening_leg_that_begins_the_next() {
    let calendar = calendar_for_market_hours_key(ZC);

    assert_eq!(
        calendar.holiday_on(day((2025, 1, 20))).map(Holiday::kind),
        Some(HolidayKind::Closed),
    );
    // The Sunday-evening leg fed trade date 2025-01-20 and is deleted with it.
    assert!(!open_at(ct((2025, 1, 19), (19, 30, 0))));
    assert!(!open_at(ct((2025, 1, 20), (9, 0, 0))));
    assert!(!open_at(ct((2025, 1, 20), (12, 0, 0))));
    // The Monday-evening leg carries trade date 2025-01-21 and runs normally.
    assert!(open_at(ct((2025, 1, 20), (19, 30, 0))));
    assert!(open_at(ct((2025, 1, 21), (9, 0, 0))));
}

/// Case 2 — the instant before an early close is still open.
#[test]
fn christmas_eve_2025_is_open_one_second_before_the_early_close() {
    assert!(open_at(
        ct((2025, 12, 24), (12, 5, 0)) - Duration::seconds(1)
    ));
    assert!(open_at(ct((2025, 12, 24), (10, 0, 0))));
}

/// Case 3 — the early close is end-exclusive, ends the session and the daily
/// bar, and deletes what the normal week would have run after it.
#[test]
fn christmas_eve_2025_closes_at_1205_central() {
    let calendar = calendar_for_market_hours_key(ZC);
    let cutoff = ct((2025, 12, 24), (12, 5, 0));
    let probe = ct((2025, 12, 24), (10, 0, 0));

    assert!(!open_at(cutoff));
    assert_eq!(
        calendar.session_bounds(probe),
        Some((ct((2025, 12, 24), (8, 30, 0)), cutoff)),
    );
    assert_eq!(
        calendar.candle_end(probe, CalendarResolution::Daily),
        Some(cutoff),
    );
    // The normal week runs the day session to 13:20 CT; the row removes it.
    assert!(!open_at(ct((2025, 12, 24), (13, 0, 0))));
}

/// Case 4 — a late open stated at 08:30 CT, which is earlier than the trading
/// day's normal 19:00 CT first open, so the cutoff lands on the trade date
/// itself and deletes the prior-evening leg rather than the day session.
#[test]
fn boxing_day_2025_opens_late_at_0830_central() {
    let cutoff = ct((2025, 12, 26), (8, 30, 0));

    // Thursday 19:00 CT would normally have opened trade date 2025-12-26.
    assert!(!open_at(ct((2025, 12, 25), (19, 30, 0))));
    assert!(!open_at(ct((2025, 12, 26), (6, 0, 0))));
    assert!(!open_at(cutoff - Duration::seconds(1)));
    assert!(open_at(cutoff));
    assert_eq!(
        calendar_for_market_hours_key(ZC).session_bounds(cutoff),
        Some((cutoff, ct((2025, 12, 26), (13, 20, 0)))),
    );
}

/// Case 4 again, on the row that moves both boundaries: the day after
/// Thanksgiving 2025 is 08:30-12:05 CT and nothing else.
#[test]
fn day_after_thanksgiving_2025_is_a_single_0830_to_1205_block() {
    let calendar = calendar_for_market_hours_key(ZC);
    let open = ct((2025, 11, 28), (8, 30, 0));
    let close = ct((2025, 11, 28), (12, 5, 0));

    assert_eq!(
        calendar.holiday_on(day((2025, 11, 28))).map(Holiday::kind),
        Some(HolidayKind::LateOpenAndEarlyClose {
            open_ssm: 8 * 3_600 + 30 * 60,
            close_ssm: 12 * 3_600 + 5 * 60,
        }),
    );
    assert!(!open_at(ct((2025, 11, 27), (19, 30, 0))));
    assert!(!open_at(open - Duration::seconds(1)));
    assert!(open_at(open));
    assert!(open_at(close - Duration::seconds(1)));
    assert!(!open_at(close));
    assert_eq!(calendar.session_bounds(open), Some((open, close)));
}

/// Case 5 — the removed wrap, read through the query a consumer would use:
/// after Christmas Eve's 12:05 CT close the next grain session is Friday's
/// 08:30 CT day open, because both intervening evening legs are gone.
#[test]
fn next_session_after_christmas_eve_2025_is_the_friday_day_open() {
    let calendar = calendar_for_market_hours_key(ZC);

    assert_eq!(
        calendar.next_session_open_after(ct((2025, 12, 24), (12, 10, 0))),
        Some(ct((2025, 12, 26), (8, 30, 0))),
    );
    assert_eq!(
        calendar.next_session_after(ct((2025, 12, 24), (12, 10, 0))),
        Some((
            ct((2025, 12, 26), (8, 30, 0)),
            ct((2025, 12, 26), (13, 20, 0))
        )),
    );
}

/// Case 6 — the trade-date consequence. A shortened day keeps its own trade
/// date; the holiday evening's open carries the post-holiday date; and the
/// evening leg that fed a closed date has no trade date at all because it has
/// no session.
#[test]
fn holiday_rows_move_the_trade_date_the_way_cme_prints_it() {
    let calendar = calendar_for_market_hours_key(ZC);

    assert_eq!(
        calendar.trade_date(ct((2025, 12, 24), (10, 0, 0))),
        Some(day((2025, 12, 24))),
    );
    assert_eq!(
        calendar.trade_date(ct((2025, 11, 28), (10, 0, 0))),
        Some(day((2025, 11, 28))),
    );
    // CME prints "19:00 open (trade date 2025-01-21)" on MLK Monday.
    assert_eq!(
        calendar.trade_date(ct((2025, 1, 20), (19, 30, 0))),
        Some(day((2025, 1, 21))),
    );
    // The Sunday leg that would have fed the closed Monday is gone entirely.
    assert_eq!(calendar.trade_date(ct((2025, 1, 19), (19, 30, 0))), None);
}

/// Case 7 — both edges of the declared coverage window. Inside it a date with
/// no row is audited normal; outside it the table has no answer and the
/// normal week is served unmodified, so a holiday one day below the window and
/// a holiday well above it are both ignored.
#[test]
fn coverage_window_is_declared_and_does_not_extend() {
    let calendar = calendar_for_market_hours_key(ZC);
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_grains ships a holiday table");

    assert_eq!(coverage.first(), day((2025, 1, 1)));
    assert_eq!(coverage.last(), day((2027, 12, 31)));
    assert!(coverage.contains(day((2025, 1, 1))));
    assert!(coverage.contains(day((2027, 12, 31))));
    assert!(!coverage.contains(day((2024, 12, 31))));
    assert!(!coverage.contains(day((2028, 1, 1))));

    assert_eq!(calendar.holiday_on(day((2024, 12, 31))), None);
    assert_eq!(calendar.holiday_on(day((2028, 1, 1))), None);

    // Christmas 2024 is a real CME closure one week below the window; the table
    // says nothing about it and the normal week answers.
    assert_eq!(calendar.holiday_on(day((2024, 12, 25))), None);
    assert!(open_at(ct((2024, 12, 25), (9, 0, 0))));

    // Martin Luther King Jr. Day 2028 is above the window; the table does not
    // extrapolate the pattern it holds for 2025, 2026 and 2027.
    assert_eq!(calendar.holiday_on(day((2028, 1, 17))), None);
    assert!(open_at(ct((2028, 1, 17), (9, 0, 0))));
}

/// Case 7's inner half — a date inside coverage with no row is audited normal.
/// Good Friday eve 2026 is one of the eleven dates on which CME publishes the
/// whole day session and then withholds the evening leg; the leg belongs to the
/// closed Friday's trade date and the `Closed` row alone removes it.
#[test]
fn good_friday_eve_2026_is_audited_normal_and_ships_no_row() {
    let calendar = calendar_for_market_hours_key(ZC);

    assert_eq!(calendar.holiday_on(day((2026, 4, 2))), None);
    assert!(open_at(ct((2026, 4, 2), (9, 0, 0))));
    assert!(open_at(ct((2026, 4, 2), (13, 19, 0))));
    assert!(!open_at(ct((2026, 4, 2), (13, 20, 0))));
    // The 19:00 CT leg would have carried trade date 2026-04-03.
    assert!(!open_at(ct((2026, 4, 2), (19, 30, 0))));
    assert!(calendar.is_closed_trade_date(day((2026, 4, 3)), SessionKind::Both));
}

/// `without_holidays` restores exactly the pre-table answer, over a dense
/// instant grid across the whole Christmas 2025 week and at each shipped row's
/// own probes. This is the consumer's escape hatch and the benchmark control.
#[test]
fn without_holidays_reproduces_the_normal_week() {
    let detached = calendar_for_market_hours_key(ZC).without_holidays();

    assert_eq!(detached.holiday_coverage(), None);
    assert_eq!(detached.holiday_on(day((2025, 12, 25))), None);
    assert!(detached.is_open(ct((2025, 12, 25), (9, 0, 0))));
    assert!(detached.is_open(ct((2025, 12, 24), (13, 0, 0))));
    assert!(detached.is_open(ct((2025, 12, 25), (19, 30, 0))));
    assert!(detached.is_open(ct((2025, 1, 20), (9, 0, 0))));
    assert!(detached.is_open(ct((2025, 11, 28), (13, 0, 0))));

    let mut probe = ct((2025, 12, 21), (0, 0, 0));
    let end = ct((2025, 12, 29), (0, 0, 0));
    while probe < end {
        assert_eq!(
            detached.is_open(probe),
            hours_for_market_hours_key(ZC, probe).is_open(probe),
            "detached calendar must equal the identity-erased snapshot at {probe}"
        );
        probe += Duration::minutes(5);
    }
}
