// SPDX-License-Identifier: MIT-0

//! Xetra holiday rows, 2025-2027: the FWB non-trading-days closure set, with
//! 20:00 early closes only on the trading holidays the operator names.

use super::prelude::*;

fn de(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    local(Europe::Berlin, date, time)
}

fn day(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("fixture must be a valid date")
}

fn calendar() -> ExchangeCalendar {
    calendar_for_exchange(Exchange::Xetra)
}

#[test]
fn coverage_opens_at_the_2025_floor_and_stops_at_the_2027_schedule() {
    let calendar = calendar();
    let coverage = calendar
        .holiday_coverage()
        .expect("xetra ships a built-in table");
    assert_eq!(coverage.first(), day(2025, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2028, 1, 1)), None);
}

#[test]
fn every_printed_2025_closure_ships_and_no_early_close_is_invented() {
    let calendar = calendar();
    assert_closed(
        "xetra",
        calendar,
        Europe::Berlin,
        &[
            day(2025, 1, 1),
            day(2025, 4, 18),
            day(2025, 4, 21),
            day(2025, 5, 1),
            day(2025, 12, 24),
            day(2025, 12, 25),
            day(2025, 12, 26),
            day(2025, 12, 31),
        ],
    );
    // The 2025 page edition words the 20:00 note over Börse Frankfurt only, so
    // the 2025 trading holidays ship no Xetra early close and answer as
    // ordinary days: Corpus Christi 2025-06-19 has no row and its midday
    // trades.
    assert_eq!(calendar.holiday_on(day(2025, 6, 19)), None);
    assert!(
        calendar
            .is_open(de((2025, 6, 19), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "Corpus Christi 2025 trades as an ordinary Xetra day in this table"
    );
}

#[test]
fn the_2026_closures_and_the_three_named_trading_holidays_ship() {
    let calendar = calendar();
    assert_closed(
        "xetra",
        calendar,
        Europe::Berlin,
        &[
            day(2026, 1, 1),
            day(2026, 4, 3),
            day(2026, 4, 6),
            day(2026, 5, 1),
            day(2026, 12, 24),
            day(2026, 12, 25),
            day(2026, 12, 31),
        ],
    );
    // Ascension, Whit Monday and Corpus Christi: named 2026 trading holidays
    // with the printed 20:00 CET shares close.
    for holiday_date in [(2026, 5, 14), (2026, 5, 25), (2026, 6, 4)] {
        assert_early_close(
            "xetra",
            calendar,
            Europe::Berlin,
            holiday_date,
            20 * 3_600,
            "DB-TC-PAGE",
        );
    }
    // Christmas Eve and New Year's Eve are full closures — the operator's own
    // "** No trading but settlement is open" footnote — not half days.
    assert!(
        !calendar
            .is_open(de((2026, 12, 24), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "24 December is a closure, not an early close"
    );
}

#[test]
fn every_printed_2027_closure_ships() {
    let calendar = calendar();
    assert_closed(
        "xetra",
        calendar,
        Europe::Berlin,
        &[
            day(2027, 1, 1),
            day(2027, 3, 26),
            day(2027, 3, 29),
            day(2027, 12, 24),
            day(2027, 12, 31),
        ],
    );
    // The 2027 trading holidays are named by no retrieved artifact, so they
    // carry no row; Ascension 2027-05-06 answers as an ordinary Thursday.
    assert_eq!(calendar.holiday_on(day(2027, 5, 6)), None);
    assert!(
        calendar
            .is_open(de((2027, 5, 6), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the 2027 trading holidays answer as ordinary days in this table"
    );
}

#[test]
fn an_ordinary_weekday_trades_to_the_end_exclusive_late_retail_close() {
    let calendar = calendar();
    assert_eq!(calendar.holiday_on(day(2026, 7, 8)), None);
    assert!(
        calendar
            .is_open(de((2026, 7, 8), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "an ordinary midday is open"
    );
    assert!(
        calendar
            .is_open(de((2026, 7, 8), (21, 59, 59)))
            .expect("the coverage contract must answer a covered date"),
        "21:59:59 is inside the late retail window"
    );
    assert!(
        !calendar
            .is_open(de((2026, 7, 8), (22, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "22:00:00 is the end-exclusive close"
    );
}

#[test]
fn the_christmas_block_reopens_on_the_first_open_day() {
    let calendar = calendar();
    // 2026-12-24 and 25 are closures, 26-27 the weekend, so trading resumes
    // Monday 2026-12-28 at the 07:00 pre-trading open.
    let next = calendar
        .next_session_after(de((2026, 12, 24), (12, 0, 0)))
        .expect("the coverage contract must answer a covered date")
        .expect("a reopening must exist inside the bounded search");
    assert_eq!(
        next.0.with_timezone(&Europe::Berlin).date_naive(),
        day(2026, 12, 28),
        "xetra must reopen on 28 December 2026"
    );
}

#[test]
fn queries_before_the_2010_floor_are_refused() {
    let calendar = calendar();
    let error = calendar
        .is_open(de((2009, 12, 31), (12, 0, 0)))
        .expect_err("a pre-floor query must refuse");
    assert!(
        (super::prelude::PRE_FLOOR_REFUSAL)(&error),
        "xetra must refuse 2009-12-31 with BeforeSupportFloor, got {error:?}"
    );
}

#[test]
fn mutating_a_shipped_row_fails_a_test() {
    let calendar = calendar();
    // A closure flipped to an early close fails the noon probe inside
    // `assert_closed`; an early close moved from 20:00 fails the
    // end-exclusive probes inside `assert_early_close`. This test adds the
    // document-id leg and the 2026-vs-2025 document split, so a row whose id
    // stops naming the edition that states it fails too.
    let closure = calendar
        .holiday_on(day(2026, 12, 24))
        .expect("2026-12-24 ships a row");
    assert_eq!(closure.kind(), HolidayKind::Closed);
    assert_eq!(closure.document_id(), "DB-TC-PDF-2026");
    let early = calendar
        .holiday_on(day(2026, 5, 14))
        .expect("2026-05-14 ships a row");
    assert_eq!(
        early.kind(),
        HolidayKind::EarlyClose {
            close_ssm: 20 * 3_600
        }
    );
    assert_eq!(early.document_id(), "DB-TC-PAGE");
    // The 2027 closures cite the live page, not the 2026 PDF.
    let closure_2027 = calendar
        .holiday_on(day(2027, 3, 26))
        .expect("2027-03-26 ships a row");
    assert_eq!(closure_2027.document_id(), "DB-TC-PAGE");
}
