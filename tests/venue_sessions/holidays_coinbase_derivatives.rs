// SPDX-License-Identifier: MIT-0

//! Built-in holiday rows for Coinbase Derivatives, 2026 through Labor Day.
//!
//! Every row is a full closure of the 23x5 grid the venue profile models, so
//! §4.1 cases 2, 3 and 4 have nothing to exercise here: CDE publishes no early
//! close and no late open for that tier in this window. Coverage stops at
//! 2026-09-07 because the Thanksgiving and Christmas notices had not issued at
//! retrieval, and the last test is the fence on that.

use chrono::{Days, NaiveDate, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    Exchange, ExchangeCalendar, Holiday, HolidayKind, SessionKind, calendar_for_exchange,
};

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

#[test]
fn the_venue_ships_the_eight_notices_coinbase_had_published() {
    let calendar = cde();
    let closures = [
        (day(2026, 1, 1), "CDE-MN-25-42"),
        (day(2026, 1, 19), "CDE-MN-26-01"),
        (day(2026, 2, 16), "CDE-MN-26-05"),
        (day(2026, 4, 3), "CDE-MN-26-12"),
        (day(2026, 5, 25), "CDE-MN-26-23"),
        (day(2026, 6, 19), "CDE-MN-26-27.1"),
        (day(2026, 7, 3), "CDE-MN-26-29"),
        (day(2026, 9, 7), "CDE-MN-26-36"),
    ];
    for (date, document) in closures {
        let row = calendar
            .holiday_on(date)
            .unwrap_or_else(|| panic!("{date} must carry a row"));
        assert_eq!(row.kind(), HolidayKind::Closed, "{date}");
        assert_eq!(row.document_id(), document, "{date}");
        assert!(
            calendar.is_closed_trade_date(date, SessionKind::Both),
            "{date}"
        );
    }
}

#[test]
fn martin_luther_king_day_removes_the_whole_trading_day() {
    let calendar = cde();

    for probe in [(0, 30, 0), (10, 0, 0), (15, 30, 0)] {
        assert!(
            !calendar.is_open(ct((2026, 1, 19), probe)),
            "CDE must be shut at {probe:?} on MLK Day"
        );
    }
    // The Sunday-evening leg belongs to trade date 2026-01-19, so it is gone.
    assert!(!calendar.is_open(ct((2026, 1, 18), (18, 0, 0))));
    // The Monday-evening leg belongs to trade date 2026-01-20, so it is not.
    assert!(calendar.is_open(ct((2026, 1, 19), (18, 0, 0))));
    assert_eq!(
        calendar.trade_date(ct((2026, 1, 19), (18, 0, 0))),
        Some(day(2026, 1, 20))
    );
}

#[test]
fn the_session_after_the_friday_before_a_monday_holiday_is_that_mondays_evening() {
    let calendar = cde();
    let next = calendar
        .next_session_after(ct((2026, 1, 16), (16, 30, 0)))
        .expect("a reopening must exist inside the bounded search");

    assert_eq!(next.0, ct((2026, 1, 19), (17, 0, 0)));
}

#[test]
fn a_friday_holiday_removes_the_thursday_evening_leg() {
    let calendar = cde();

    assert!(calendar.is_closed_trade_date(day(2026, 4, 3), SessionKind::Both));
    // Thursday 17:00 CT feeds trade date Good Friday: gone.
    assert!(!calendar.is_open(ct((2026, 4, 2), (18, 0, 0))));
    // Thursday's own day session, trade date 2026-04-02, is untouched.
    assert!(calendar.is_open(ct((2026, 4, 2), (10, 0, 0))));
}

#[test]
fn coverage_stops_at_the_last_notice_coinbase_had_issued() {
    let calendar = cde();
    let coverage = calendar
        .holiday_coverage()
        .expect("Coinbase Derivatives ships a built-in table");

    assert_eq!(coverage.first(), day(2026, 1, 1));
    assert_eq!(coverage.last(), day(2026, 9, 7));
    assert_eq!(
        calendar.holiday_on(
            coverage
                .first()
                .checked_sub_days(Days::new(1))
                .expect("a representable date")
        ),
        None
    );
    assert_eq!(
        calendar.holiday_on(
            coverage
                .last()
                .checked_add_days(Days::new(1))
                .expect("a representable date")
        ),
        None
    );
    // Thanksgiving 2026 is a holiday CDE will observe, but its notice had not
    // issued at retrieval, so the table must not reach it.
    let outside = ct((2026, 11, 26), (10, 0, 0));
    assert_eq!(calendar.holiday_on(day(2026, 11, 26)), None);
    assert_eq!(
        calendar.is_open(outside),
        calendar.without_holidays().is_open(outside)
    );
    assert!(calendar.is_open(outside));
}

#[test]
fn detaching_the_table_restores_the_normal_week() {
    let calendar = cde();
    let labor_day = ct((2026, 9, 7), (10, 0, 0));

    assert!(!calendar.is_open(labor_day));
    assert!(calendar.without_holidays().is_open(labor_day));
    assert_eq!(calendar.without_holidays().holiday_coverage(), None);
    assert_eq!(
        calendar.without_holidays().holiday_on(day(2026, 9, 7)),
        None
    );
}

/// The module's claim, fenced over the whole coverage window: every row is a
/// full closure and there are exactly the eight the notices published, so an
/// early close or a ninth row on a date no test names fails here.
#[test]
fn every_row_in_the_window_is_a_full_closure() {
    let calendar = cde();
    let coverage = calendar
        .holiday_coverage()
        .expect("Coinbase Derivatives ships a built-in table");
    let mut closed = 0_usize;
    let mut date = coverage.first();
    while date <= coverage.last() {
        match calendar.holiday_on(date).map(Holiday::kind) {
            None => {}
            Some(HolidayKind::Closed) => closed += 1,
            Some(other) => panic!("{date} ships a kind other than a full closure: {other:?}"),
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the coverage window stays inside the representable calendar");
    }
    assert_eq!(
        closed, 8,
        "the eight published closures, 2026-01-01..2026-09-07"
    );
}
