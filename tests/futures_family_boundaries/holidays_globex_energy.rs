// SPDX-License-Identifier: MIT-0

//! `globex_energy`'s built-in holiday rows, 2025-2027, through the public
//! surface only.
//!
//! The family's trading day wraps: one occurrence opens 17:00 CT on the
//! previous local day and closes 16:00 CT on the trade date. That is what
//! makes these cases worth fencing — a row is stated on the **trade date**, so
//! an early close has to land on a session that opened the evening before, a
//! closure has to delete that evening leg rather than the holiday's own
//! daytime, and the eve's own trade date has to survive both.
//!
//! Every probe is stated in `America/Chicago` wall clock and converted, so a
//! DST slip in either direction fails rather than passing on a coincidence.
//! The seven cases of the design memo's §4.1 are covered in order, with the
//! late-open case answered by proving the family ships no late open in this
//! window rather than by inventing one.

use chrono::{DateTime, Days, NaiveDate, TimeDelta, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, ExchangeCalendar, HolidayKind, MarketHoursKey, SessionKind,
    calendar_for_market_hours_key,
};

const KEY: MarketHoursKey = MarketHoursKey::GlobexEnergy;

/// A venue-local calendar date stated as `(year, month, day)`.
type Ymd = (i32, u32, u32);

fn calendar() -> ExchangeCalendar {
    calendar_for_market_hours_key(KEY)
}

/// A probe instant stated in the venue's own wall clock.
fn ct(date: Ymd, hour: u32, minute: u32) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, hour, minute, 0)
        .single()
        .expect("fixture must be an unambiguous Central instant")
        .with_timezone(&Utc)
}

fn day(date: Ymd) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.0, date.1, date.2).expect("fixture must be a valid date")
}

fn one_second_before(instant: DateTime<Utc>) -> DateTime<Utc> {
    instant - TimeDelta::seconds(1)
}

/// Case 1 — a closed day. Christmas 2025 falls on a Thursday, so its trade
/// date is removed together with the Wednesday-evening leg that fed it.
#[test]
fn christmas_2025_removes_the_whole_trade_date() {
    let calendar = calendar();
    let christmas = day((2025, 12, 25));

    assert!(calendar.is_closed_trade_date(christmas, SessionKind::Both));
    assert_eq!(
        calendar
            .holiday_on(christmas)
            .map(exchange_hours::Holiday::kind),
        Some(HolidayKind::Closed)
    );
    for (hour, minute) in [(0_u32, 30_u32), (10, 0), (15, 30)] {
        assert!(
            !calendar.is_open(ct((2025, 12, 25), hour, minute)),
            "the market is closed all of Christmas Day at {hour:02}:{minute:02} CT"
        );
    }
}

/// Case 5 — the wrap a closure removes, and where the next session actually
/// opens. Christmas Eve keeps its own trade date and closes early at 12:45 CT;
/// the 17:00 CT leg that would have opened trade date 2025-12-25 is gone, and
/// the next open is 17:00 CT on Christmas Day itself, for trade date 12-26.
#[test]
fn the_christmas_eve_evening_leg_is_removed_and_the_next_open_is_the_holiday_evening() {
    let calendar = calendar();

    assert!(!calendar.is_open(ct((2025, 12, 24), 17, 30)));
    assert!(!calendar.is_open(ct((2025, 12, 24), 23, 0)));

    let (open, close) = calendar
        .next_session_after(ct((2025, 12, 24), 13, 0))
        .expect("a session opens after the shortened Christmas Eve");
    assert_eq!(open, ct((2025, 12, 25), 17, 0));
    assert_eq!(close, ct((2025, 12, 26), 16, 0));
}

/// Cases 2 and 3 — the instant before an early close and the instant at it.
/// The day after Thanksgiving 2025 closes 13:45 CT; the session it ends opened
/// at 17:00 CT the evening before, on a trade date that is itself an early
/// close at 13:30 CT and does not clip this one.
#[test]
fn the_2025_black_friday_early_close_is_end_exclusive_on_its_own_trade_date() {
    let calendar = calendar();
    let cutoff = ct((2025, 11, 28), 13, 45);

    assert!(calendar.is_open(one_second_before(cutoff)));
    assert!(!calendar.is_open(cutoff));

    let (open, close) = calendar
        .session_bounds(ct((2025, 11, 28), 10, 0))
        .expect("the shortened Friday session is the containing session");
    assert_eq!(open, ct((2025, 11, 27), 17, 0));
    assert_eq!(close, cutoff);

    assert_eq!(
        calendar.candle_end(ct((2025, 11, 28), 10, 0), CalendarResolution::Daily),
        Some(cutoff)
    );
    assert!(!calendar.is_open(ct((2025, 11, 28), 15, 0)));
}

/// The Thanksgiving pre-open shape CME publishes instead of a close: matching
/// stops at 13:30 CT and the ordinary 17:00 CT open still starts the next
/// trade date, so one civil day carries the end of one trading day and the
/// start of the next.
#[test]
fn the_2025_thanksgiving_early_close_stops_matching_at_13_30_and_reopens_at_17_00() {
    let calendar = calendar();
    let cutoff = ct((2025, 11, 27), 13, 30);

    assert!(calendar.is_open(one_second_before(cutoff)));
    assert!(!calendar.is_open(cutoff));
    assert!(!calendar.is_open(ct((2025, 11, 27), 16, 59)));
    assert!(calendar.is_open(ct((2025, 11, 27), 17, 0)));
}

/// The Friday holidays of 2026 and 2027, where CME prints the early close but
/// dates it to the following Monday. The crate keys the session to the
/// venue-local date of its own final close, so the row stays on the Friday and
/// the Thursday-evening leg it clips is kept rather than deleted.
#[test]
fn juneteenth_2026_clips_the_friday_and_keeps_the_thursday_evening_leg() {
    let calendar = calendar();
    let cutoff = ct((2026, 6, 19), 12, 0);

    assert!(calendar.is_open(ct((2026, 6, 18), 17, 30)));
    assert!(calendar.is_open(one_second_before(cutoff)));
    assert!(!calendar.is_open(cutoff));
    assert_eq!(
        calendar.session_bounds(ct((2026, 6, 19), 9, 0)),
        Some((ct((2026, 6, 18), 17, 0), cutoff))
    );
    assert_eq!(
        calendar.trade_date(ct((2026, 6, 19), 9, 0)),
        Some(day((2026, 6, 19)))
    );
}

/// Case 4 — a late open. This family has none in the published window: CME
/// moves the pre-open on a holiday but never the 17:00 CT open itself, so
/// every deviation is a close or a closure. The absent case is fenced rather
/// than skipped, because a row of the wrong kind would otherwise ship unseen.
#[test]
fn no_late_open_row_ships_in_the_published_window() {
    let calendar = calendar();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_energy ships a holiday table");

    let mut date = coverage.first();
    while date <= coverage.last() {
        if let Some(holiday) = calendar.holiday_on(date) {
            assert!(
                matches!(
                    holiday.kind(),
                    HolidayKind::Closed | HolidayKind::EarlyClose { .. }
                ),
                "globex_energy ships only closures and early closes: {date} is {:?}",
                holiday.kind()
            );
        }
        date = date
            .succ_opt()
            .expect("the coverage window ends well inside the calendar range");
    }
}

/// Case 6 — the trade-date consequence. A shortened day keeps its own trade
/// date, the evening leg of an early-close day already carries the next one,
/// and the evening of a closed day carries the post-holiday date.
#[test]
fn trade_dates_follow_the_rows_rather_than_the_civil_day() {
    let calendar = calendar();

    assert_eq!(
        calendar.trade_date(ct((2025, 11, 28), 12, 0)),
        Some(day((2025, 11, 28)))
    );
    assert_eq!(
        calendar.trade_date(ct((2025, 11, 27), 18, 0)),
        Some(day((2025, 11, 28)))
    );
    assert_eq!(
        calendar.trade_date(ct((2025, 12, 25), 18, 0)),
        Some(day((2025, 12, 26)))
    );
    assert_eq!(calendar.trade_date(ct((2025, 12, 25), 10, 0)), None);
}

/// Case 7 — both edges of the coverage window. Inside it a date with no row is
/// audited normal; outside it the table has no answer at all and must not
/// silently extend to a holiday it never audited.
#[test]
fn the_coverage_window_bounds_every_answer_the_table_gives() {
    let calendar = calendar();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_energy ships a holiday table");

    assert_eq!(coverage.first(), day((2025, 1, 1)));
    assert_eq!(coverage.last(), day((2027, 12, 31)));
    assert!(coverage.contains(day((2025, 1, 1))));
    assert!(coverage.contains(day((2027, 12, 31))));

    assert_eq!(
        calendar
            .holiday_on(day((2025, 1, 1)))
            .map(exchange_hours::Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(calendar.holiday_on(day((2024, 12, 31))).is_none());
    assert!(calendar.holiday_on(day((2028, 1, 1))).is_none());

    // Christmas 2024 and Christmas 2028 are holidays CME observes and this
    // table never audited. Both must answer exactly as the normal week does.
    for probe in [ct((2024, 12, 25), 10, 0), ct((2028, 12, 25), 10, 0)] {
        assert!(calendar.holiday_on(probe.date_naive()).is_none());
        assert_eq!(
            calendar.is_open(probe),
            calendar.without_holidays().is_open(probe)
        );
    }
}

/// `without_holidays` is the exact undo: it restores the normal-week answer on
/// a holiday week and changes nothing on an ordinary one.
#[test]
fn without_holidays_restores_the_normal_week_answer() {
    let calendar = calendar();
    let detached = calendar.without_holidays();

    assert!(detached.holiday_on(day((2025, 12, 25))).is_none());
    assert!(detached.holiday_coverage().is_none());
    assert!(detached.is_open(ct((2025, 12, 25), 10, 0)));
    assert!(detached.is_open(ct((2025, 11, 28), 15, 0)));

    // A dense grid over the 2025 Christmas week: the two answers differ only
    // where a row says so, and an ordinary week later agrees at every probe.
    let mut differences = 0_u32;
    for offset in 0..7_u64 {
        let date = day((2025, 12, 22))
            .checked_add_days(Days::new(offset))
            .expect("the fixture week is inside the calendar range");
        for hour in 0..24_u32 {
            let probe = US::Central
                .from_local_datetime(
                    &date
                        .and_hms_opt(hour, 0, 0)
                        .expect("a whole hour is a valid local time"),
                )
                .single()
                .expect("Central has no ambiguous hour in the fixture week")
                .with_timezone(&Utc);
            if calendar.is_open(probe) != detached.is_open(probe) {
                differences = differences.saturating_add(1);
            }
        }
    }
    assert!(
        differences > 0,
        "the Christmas week must differ once the table is attached"
    );

    for offset in 0..7_u64 {
        let date = day((2026, 10, 19))
            .checked_add_days(Days::new(offset))
            .expect("the reference week is inside the calendar range");
        for hour in 0..24_u32 {
            let probe = US::Central
                .from_local_datetime(
                    &date
                        .and_hms_opt(hour, 0, 0)
                        .expect("a whole hour is a valid local time"),
                )
                .single()
                .expect("Central has no ambiguous hour in the reference week")
                .with_timezone(&Utc);
            assert_eq!(
                calendar.is_open(probe),
                detached.is_open(probe),
                "an audited-normal week must answer identically at {probe}"
            );
        }
    }
}
