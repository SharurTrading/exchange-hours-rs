// SPDX-License-Identifier: MIT-0

//! Built-in holiday rows for Coinbase Derivatives, from the venue's first
//! trade date 2021-06-28 through 2026-09-07.
//!
//! The venue profile is CDE's 23x5 grid, so every row states what the
//! operator's Market Notice states for the product groups on that grid. Fifty
//! rows are full closures, six are early closes where the groups on the grid
//! printed different instants and the row carries the earliest, and two are
//! `Unsourced`: the 2022 Thanksgiving notice is listed by the operator but its
//! PDF is unreachable, so those dates are not audited either way.
//!
//! `ERA_ROWS` is the hand-checked list, in order, the whole window is swept
//! against. The count sweep alone cannot see a row moved to another audited
//! date with the same kind, which is why the ordered table exists.

use chrono::{Days, NaiveDate, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    DateCoverage, Exchange, ExchangeCalendar, Holiday, HolidayKind, SessionKind,
    calendar_for_exchange,
};

use super::prelude::{assert_refused_variant, assert_refuses_before_floor};

fn cde() -> ExchangeCalendar {
    calendar_for_exchange(Exchange::CoinbaseDerivatives)
}

fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> chrono::DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous Chicago instant")
        .with_timezone(&Utc)
}

fn day(year: i32, month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
}

/// The early close instants the notices print, in venue-local seconds.
const P_12_15: u32 = 12 * 3_600 + 15 * 60;
const P_12_45: u32 = 12 * 3_600 + 45 * 60;
const P_13_45: u32 = 13 * 3_600 + 45 * 60;

/// Every row the crate ships, in order, as `(trade date, kind)`.
///
/// Read row by row off the operator's own Market Notices; the research store
/// keeps the bytes and `docs/evidence/coinbase_derivatives.md` quotes them.
#[rustfmt::skip]
const ERA_ROWS: [(i32, u32, u32, HolidayKind); 58] = [
    (2021,  7,  5, HolidayKind::Closed),
    (2021,  9,  6, HolidayKind::Closed),
    (2021, 11, 25, HolidayKind::Closed),
    (2021, 11, 26, HolidayKind::EarlyClose { close_ssm: P_12_15 }),
    (2021, 12, 24, HolidayKind::Closed),
    (2022,  1, 17, HolidayKind::Closed),
    (2022,  2, 21, HolidayKind::Closed),
    (2022,  4, 15, HolidayKind::Closed),
    (2022,  5, 30, HolidayKind::Closed),
    (2022,  6, 20, HolidayKind::Closed),
    (2022,  7,  4, HolidayKind::Closed),
    (2022,  9,  5, HolidayKind::Closed),
    (2022, 11, 24, HolidayKind::Unsourced),
    (2022, 11, 25, HolidayKind::Unsourced),
    (2022, 12, 26, HolidayKind::Closed),
    (2023,  1,  2, HolidayKind::Closed),
    (2023,  1, 16, HolidayKind::Closed),
    (2023,  2, 20, HolidayKind::Closed),
    (2023,  4,  7, HolidayKind::Closed),
    (2023,  5, 29, HolidayKind::Closed),
    (2023,  6, 19, HolidayKind::Closed),
    (2023,  7,  4, HolidayKind::Closed),
    (2023,  9,  4, HolidayKind::Closed),
    (2023, 11, 23, HolidayKind::Closed),
    (2023, 11, 24, HolidayKind::EarlyClose { close_ssm: P_12_15 }),
    (2023, 12, 25, HolidayKind::Closed),
    (2024,  1,  1, HolidayKind::Closed),
    (2024,  1, 15, HolidayKind::Closed),
    (2024,  2, 19, HolidayKind::Closed),
    (2024,  3, 29, HolidayKind::Closed),
    (2024,  5, 27, HolidayKind::Closed),
    (2024,  6, 19, HolidayKind::Closed),
    (2024,  7,  4, HolidayKind::Closed),
    (2024,  9,  2, HolidayKind::Closed),
    (2024, 11, 28, HolidayKind::Closed),
    (2024, 11, 29, HolidayKind::EarlyClose { close_ssm: P_13_45 }),
    (2024, 12, 24, HolidayKind::EarlyClose { close_ssm: P_12_45 }),
    (2024, 12, 25, HolidayKind::Closed),
    (2025,  1,  1, HolidayKind::Closed),
    (2025,  1, 20, HolidayKind::Closed),
    (2025,  2, 17, HolidayKind::Closed),
    (2025,  4, 18, HolidayKind::Closed),
    (2025,  5, 26, HolidayKind::Closed),
    (2025,  6, 19, HolidayKind::Closed),
    (2025,  7,  4, HolidayKind::Closed),
    (2025,  9,  1, HolidayKind::Closed),
    (2025, 11, 27, HolidayKind::Closed),
    (2025, 11, 28, HolidayKind::EarlyClose { close_ssm: P_12_15 }),
    (2025, 12, 24, HolidayKind::EarlyClose { close_ssm: P_12_15 }),
    (2025, 12, 25, HolidayKind::Closed),
    (2026,  1,  1, HolidayKind::Closed),
    (2026,  1, 19, HolidayKind::Closed),
    (2026,  2, 16, HolidayKind::Closed),
    (2026,  4,  3, HolidayKind::Closed),
    (2026,  5, 25, HolidayKind::Closed),
    (2026,  6, 19, HolidayKind::Closed),
    (2026,  7,  3, HolidayKind::Closed),
    (2026,  9,  7, HolidayKind::Closed),
];

/// Every date in the audited window that carries a row, in ascending order.
fn shipped_rows() -> Vec<(NaiveDate, HolidayKind)> {
    let calendar = cde();
    let coverage = calendar
        .holiday_coverage()
        .expect("Coinbase Derivatives ships a built-in table");
    let mut rows = Vec::new();
    let mut date = coverage.first();
    while date <= coverage.last() {
        if let Some(holiday) = calendar.holiday_on(date) {
            rows.push((date, holiday.kind()));
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the coverage window stays inside the representable calendar");
    }
    rows
}

#[test]
fn the_window_carries_exactly_the_notices_published_in_order() {
    let expected = ERA_ROWS
        .iter()
        .map(|(year, month, date, kind)| (day(*year, *month, *date), *kind))
        .collect::<Vec<_>>();

    assert_eq!(
        shipped_rows(),
        expected,
        "the shipped rows must be the notices' rows, in trade-date order"
    );
}

#[test]
fn coverage_runs_from_the_launch_day_to_the_last_notice() {
    let calendar = cde();
    let coverage = calendar
        .holiday_coverage()
        .expect("Coinbase Derivatives ships a built-in table");

    // The venue's first trade date is FairX's launch Monday; the table starts
    // there because no earlier trade date exists to carry a holiday.
    assert_eq!(coverage.first(), day(2021, 6, 28));
    assert_eq!(coverage.last(), day(2026, 9, 7));
    for (outside, instant) in [
        (day(2021, 6, 27), ct((2021, 6, 27), (10, 0, 0))),
        (day(2026, 9, 8), ct((2026, 9, 8), (10, 0, 0))),
        // Thanksgiving 2026 is a holiday CDE will observe, but its notice had
        // not issued at retrieval, so the table must not reach it.
        (day(2026, 11, 26), ct((2026, 11, 26), (10, 0, 0))),
    ] {
        assert_eq!(
            calendar.holiday_on(outside),
            None,
            "{outside} is outside the audited window"
        );
        // Nothing outside the audited window ships a row, so the row layer is
        // not what answers here — the coverage contract is. 2021-06-27 is
        // pre-floor and refuses as `BeforeSupportFloor`; the two 2026 dates are
        // past the table's last audited day and refuse as
        // `OutsideCoveredRange`. Above the floor the detached calendar is the
        // reference that still answers, and the neutrality claim is that it is
        // open there; below the floor the floor refuses it too, so the refusal
        // is the whole of what either surface can state.
        if outside < exchange_hours::SUPPORT_FLOOR {
            assert_refuses_before_floor(calendar.is_open(instant), calendar, instant);
            assert_refuses_before_floor(
                calendar.without_holidays().is_open(instant),
                calendar,
                instant,
            );
        } else {
            assert_refused_variant(
                &calendar.is_open(instant),
                DateCoverage::OutsideCoveredRange,
                &format!("{outside} is past the audited window"),
            );
            assert!(
                calendar
                    .without_holidays()
                    .is_open(instant)
                    .expect("the detached calendar answers the normal week"),
                "{outside} must clip nothing outside the window"
            );
        }
    }
}

#[test]
fn an_early_close_ends_the_trading_day_at_the_earliest_printed_instant() {
    let calendar = cde();

    // 2021-11-26: Equity printed 12:15 CT and Energy 12:45 CT, so the venue
    // row carries 12:15 and never reports the half hour only Energy traded.
    let row = calendar
        .holiday_on(day(2021, 11, 26))
        .expect("the Thanksgiving half day carries a row");
    assert_eq!(row.kind(), HolidayKind::EarlyClose { close_ssm: P_12_15 });
    assert_eq!(row.document_id(), "CDE-MN-21-06");

    // The session opens 2021-11-25 17:00 CT and closes at 12:15 CT. That whole
    // era is pre-floor, so the calendar states none of it: each probe below is
    // refused as `BeforeSupportFloor` rather than answered. What is no longer
    // claimable is an `is_open` value on the half day; the printed instant
    // itself is asserted from the row above, and the end-exclusive behaviour it
    // implies is fenced on covered rows by
    // `martin_luther_king_day_removes_the_whole_trading_day`.
    for probe in [
        ct((2021, 11, 25), (17, 0, 0)),
        ct((2021, 11, 26), (12, 14, 59)),
        ct((2021, 11, 26), (12, 15, 0)),
        ct((2021, 11, 26), (17, 30, 0)),
        ct((2021, 11, 28), (17, 0, 0)),
    ] {
        assert_refuses_before_floor(calendar.is_open(probe), calendar, probe);
    }
}

#[test]
fn a_later_closing_group_does_not_widen_the_venue_row() {
    let calendar = cde();

    // 2025-11-28: Equity printed 12:15 CT, Energy & Metal 13:45 CT. The venue
    // row is the intersection, so it is shut at 12:15 even though Energy and
    // Metal were still trading; the evidence file records the under-report.
    assert_eq!(
        calendar.holiday_on(day(2025, 11, 28)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose { close_ssm: P_12_15 })
    );
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (12, 15, 0)))
            .expect("the 2025 half day is inside the audited window"),
        "the venue is shut at the earliest printed instant"
    );
    assert!(
        calendar
            .is_open(ct((2025, 11, 28), (12, 14, 59)))
            .expect("the 2025 half day is inside the audited window"),
        "and still trading one second before it"
    );

    // 2024-11-29 is the one half day every group printed at 13:45 CT. Its era is
    // pre-floor, so the calendar refuses both sides rather than confirming the
    // 13:45 edge; the printed instant is asserted from the row above, and the
    // under-report the row carries is recorded in the evidence file.
    assert_eq!(
        calendar.holiday_on(day(2024, 11, 29)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose { close_ssm: P_13_45 })
    );
    assert_refuses_before_floor(
        calendar.is_open(ct((2024, 11, 29), (13, 44, 59))),
        calendar,
        ct((2024, 11, 29), (13, 44, 59)),
    );
    assert_refuses_before_floor(
        calendar.is_open(ct((2024, 11, 29), (13, 45, 0))),
        calendar,
        ct((2024, 11, 29), (13, 45, 0)),
    );
}

#[test]
fn the_two_unreadable_thanksgiving_dates_clip_nothing() {
    let calendar = cde();

    for (date, instant) in [
        (day(2022, 11, 24), ct((2022, 11, 24), (10, 0, 0))),
        (day(2022, 11, 25), ct((2022, 11, 25), (10, 0, 0))),
    ] {
        let row = calendar
            .holiday_on(date)
            .unwrap_or_else(|| panic!("{date} must carry an Unsourced row"));
        assert_eq!(row.kind(), HolidayKind::Unsourced, "{date}");
        assert_eq!(row.document_id(), "CDE-NOTICES-INDEX-2026-09-19", "{date}");
        // `Unsourced` changes no answer, and the 2022 dates are pre-floor, so
        // both surfaces refuse the probe as `BeforeSupportFloor`: the detached
        // calendar drops the layer but keeps the floor. The row's kind and
        // document id above are the statement; the ordinary 23x5 grid it must
        // leave alone is asserted on covered dates by
        // `martin_luther_king_day_removes_the_whole_trading_day`.
        assert_refuses_before_floor(calendar.is_open(instant), calendar, instant);
        assert_refuses_before_floor(
            calendar.without_holidays().is_open(instant),
            calendar,
            instant,
        );
    }
}

#[test]
fn a_normal_trading_day_notice_ships_no_row() {
    let calendar = cde();

    // Notice 24-27 states the 2025-01-09 National Day of Mourning session
    // "will observe a normal trading day".
    assert_eq!(calendar.holiday_on(day(2025, 1, 9)), None);
    assert!(
        calendar
            .is_open(ct((2025, 1, 9), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );

    // 2021-12-31 carries no notice: the operator's 2021 listing is complete and
    // New Year's Day 2022 fell on a Saturday, so it is audited normal. The date
    // is pre-floor, so the calendar refuses the probe rather than confirming
    // the ordinary session; "audited normal" is stated by the row's absence
    // above and by the fixed grid.
    assert_eq!(calendar.holiday_on(day(2021, 12, 31)), None);
    assert_refuses_before_floor(
        calendar.is_open(ct((2021, 12, 31), (10, 0, 0))),
        calendar,
        ct((2021, 12, 31), (10, 0, 0)),
    );
}

#[test]
fn martin_luther_king_day_removes_the_whole_trading_day() {
    let calendar = cde();

    for probe in [(0, 30, 0), (10, 0, 0), (15, 30, 0)] {
        assert!(
            !calendar
                .is_open(ct((2026, 1, 19), probe))
                .expect("the coverage contract must answer a covered date"),
            "CDE must be shut at {probe:?} on MLK Day"
        );
    }
    // The Sunday-evening leg belongs to trade date 2026-01-19, so it is gone.
    assert!(
        !calendar
            .is_open(ct((2026, 1, 18), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // The Monday-evening leg belongs to trade date 2026-01-20, so it is not.
    assert!(
        calendar
            .is_open(ct((2026, 1, 19), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .trade_date(ct((2026, 1, 19), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 1, 20))
    );
}

#[test]
fn the_earliest_row_in_the_window_removes_its_own_evening_leg() {
    let calendar = cde();

    // 2021-07-05 is the first row the table carries, and the whole launch era
    // is pre-floor: the calendar refuses every probe below as
    // `BeforeSupportFloor` rather than confirming the removal. What is no
    // longer claimable is a session observation on the era's dates; the row's
    // own existence and the day-removal mechanism are fenced above and in
    // `martin_luther_king_day_removes_the_whole_trading_day`.
    for probe in [
        ct((2021, 7, 4), (18, 0, 0)),
        ct((2021, 7, 2), (10, 0, 0)),
        ct((2021, 7, 5), (17, 30, 0)),
    ] {
        assert_refuses_before_floor(calendar.is_open(probe), calendar, probe);
    }
    assert_refuses_before_floor(
        calendar.is_closed_trade_date(day(2021, 7, 5), SessionKind::Both),
        calendar,
        ct((2021, 7, 5), (12, 0, 0)),
    );
}

#[test]
fn a_friday_holiday_removes_the_thursday_evening_leg() {
    let calendar = cde();

    assert!(
        calendar
            .is_closed_trade_date(day(2026, 4, 3), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    // Thursday 17:00 CT feeds trade date Good Friday: gone.
    assert!(
        !calendar
            .is_open(ct((2026, 4, 2), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // Thursday's own day session, trade date 2026-04-02, is untouched.
    assert!(
        calendar
            .is_open(ct((2026, 4, 2), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

#[test]
fn the_session_after_the_friday_before_a_monday_holiday_is_that_mondays_evening() {
    let calendar = cde();
    let next = calendar
        .next_session_after(ct((2026, 1, 16), (16, 30, 0)))
        .expect("the coverage contract must answer a covered date")
        .expect("a reopening must exist inside the bounded search");

    assert_eq!(next.0, ct((2026, 1, 19), (17, 0, 0)));
}

#[test]
fn detaching_the_table_restores_the_normal_week() {
    let calendar = cde();
    let labor_day = ct((2026, 9, 7), (10, 0, 0));
    let half_day = ct((2021, 11, 26), (13, 0, 0));

    // MLK Day 2026 stands in for the closure half of the claim, because it is
    // the window's last-era closure the calendar can still answer: Labor Day
    // 2026 is the table's last row and the query refuses it as
    // `OutsideCoveredRange` naming 2026-09-08, the day after it. The 2021 half
    // day is pre-floor. Both refusals are the coverage contract, never a
    // `false`.
    let mlk_day = ct((2026, 1, 19), (10, 0, 0));
    assert!(
        !calendar
            .is_open(mlk_day)
            .expect("MLK Day 2026 is inside the audited window"),
        "the holiday layer closes MLK Day"
    );
    assert_refused_variant(
        &calendar.is_open(labor_day),
        DateCoverage::OutsideCoveredRange,
        "the table's last closure is past the audited window it is read from",
    );
    assert_refuses_before_floor(calendar.is_open(half_day), calendar, half_day);
    // Detaching is what restores the normal week, and it is the only surface
    // that can state it here: with the layer gone MLK Day is an ordinary
    // session.
    assert!(
        calendar
            .without_holidays()
            .is_open(mlk_day)
            .expect("the detached calendar answers the normal week")
    );
    assert_eq!(calendar.without_holidays().holiday_coverage(), None);
    assert_eq!(
        calendar.without_holidays().holiday_on(day(2026, 9, 7)),
        None
    );
}
