// SPDX-License-Identifier: MIT-0

//! HKEX holiday rows, 2025-2027: the operator's own holiday schedule, with the
//! Lunar New Year, Christmas and New Year eves as 12:10 early closes.

use super::prelude::*;

fn hk(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    local(Asia::Hong_Kong, date, time)
}

fn day(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("fixture must be a valid date")
}

fn calendar() -> ExchangeCalendar {
    calendar_for_exchange(Exchange::Hkex)
}

#[test]
fn coverage_opens_at_the_2025_floor_and_stops_at_the_2027_schedule() {
    let calendar = calendar();
    let coverage = calendar
        .holiday_coverage()
        .expect("hkex ships a built-in table");
    assert_eq!(coverage.first(), day(2025, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    // Outside the window the table has no answer at all.
    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2028, 1, 1)), None);
}

#[test]
fn every_printed_2025_row_ships() {
    let calendar = calendar();
    assert_closed(
        "hkex",
        calendar,
        Asia::Hong_Kong,
        &[
            day(2025, 1, 1),
            day(2025, 1, 29),
            day(2025, 1, 30),
            day(2025, 1, 31),
            day(2025, 4, 4),
            day(2025, 4, 18),
            day(2025, 4, 21),
            day(2025, 5, 1),
            day(2025, 5, 5),
            day(2025, 7, 1),
            day(2025, 10, 1),
            day(2025, 10, 7),
            day(2025, 10, 29),
            day(2025, 12, 25),
            day(2025, 12, 26),
        ],
    );
    // The three eves are early closes at the printed half-day CAS edge.
    for eve in [(2025, 1, 28), (2025, 12, 24), (2025, 12, 31)] {
        assert_early_close(
            "hkex",
            calendar,
            Asia::Hong_Kong,
            eve,
            12 * 3_600 + 10 * 60,
            "HKEX-TC-2025",
        );
    }
    // Every other date the schedule leaves alone is audited normal.
    assert_eq!(calendar.holiday_on(day(2025, 6, 2)), None);
}

#[test]
fn every_printed_2026_row_ships() {
    let calendar = calendar();
    assert_closed(
        "hkex",
        calendar,
        Asia::Hong_Kong,
        &[
            day(2026, 1, 1),
            day(2026, 2, 17),
            day(2026, 2, 18),
            day(2026, 2, 19),
            day(2026, 4, 3),
            day(2026, 4, 6),
            day(2026, 4, 7),
            day(2026, 5, 1),
            day(2026, 5, 25),
            day(2026, 6, 19),
            day(2026, 7, 1),
            day(2026, 10, 1),
            day(2026, 10, 19),
            day(2026, 12, 25),
        ],
    );
    for eve in [(2026, 2, 16), (2026, 12, 24), (2026, 12, 31)] {
        assert_early_close(
            "hkex",
            calendar,
            Asia::Hong_Kong,
            eve,
            12 * 3_600 + 10 * 60,
            "HKEX-TC-2026-2027",
        );
    }
    // The operator's three editions print no 2026-12-28 holiday, so the day
    // answers as an ordinary Monday: verified against the general-holiday
    // list (no conflict — it names Saturday 2026-12-26); prose, never a row.
    assert_eq!(calendar.holiday_on(day(2026, 12, 28)), None);
    assert!(
        calendar
            .is_open(hk((2026, 12, 28), (11, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "2026-12-28 answers as the trading day the operator's schedule prints"
    );
}

#[test]
fn every_printed_2027_row_ships() {
    let calendar = calendar();
    assert_closed(
        "hkex",
        calendar,
        Asia::Hong_Kong,
        &[
            day(2027, 1, 1),
            day(2027, 2, 8),
            day(2027, 2, 9),
            day(2027, 3, 26),
            day(2027, 3, 29),
            day(2027, 4, 5),
            day(2027, 5, 13),
            day(2027, 6, 9),
            day(2027, 7, 1),
            day(2027, 9, 16),
            day(2027, 10, 1),
            day(2027, 10, 8),
            day(2027, 12, 27),
        ],
    );
    for eve in [(2027, 2, 5), (2027, 12, 24), (2027, 12, 31)] {
        assert_early_close(
            "hkex",
            calendar,
            Asia::Hong_Kong,
            eve,
            12 * 3_600 + 10 * 60,
            "HKEX-TC-2026-2027",
        );
    }
    assert_eq!(calendar.holiday_on(day(2027, 12, 28)), None);
}

#[test]
fn an_ordinary_weekday_trades_to_the_end_exclusive_cas_edge() {
    let calendar = calendar();
    // Wednesday 2026-07-08: inside the window, no row.
    assert_eq!(calendar.holiday_on(day(2026, 7, 8)), None);
    assert!(
        calendar
            .is_open(hk((2026, 7, 8), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "an ordinary midday is open"
    );
    // The 16:10 close is end-exclusive: one second inside is open, the close
    // itself is closed.
    assert!(
        calendar
            .is_open(hk((2026, 7, 8), (16, 9, 59)))
            .expect("the coverage contract must answer a covered date"),
        "16:09:59 is inside the closing-auction tail"
    );
    assert!(
        !calendar
            .is_open(hk((2026, 7, 8), (16, 10, 0)))
            .expect("the coverage contract must answer a covered date"),
        "16:10:00 is the end-exclusive close"
    );
}

#[test]
fn the_lunar_new_year_block_reopens_on_the_first_open_day() {
    let calendar = calendar();
    // 2026-02-16 is a half day, 17-19 closed, so the next full session opens
    // on Friday 2026-02-20 at the 09:00 pre-opening edge.
    let next = calendar
        .next_session_after(hk((2026, 2, 16), (12, 30, 0)))
        .expect("the coverage contract must answer a covered date")
        .expect("a reopening must exist inside the bounded search");
    assert_eq!(
        next.0.with_timezone(&Asia::Hong_Kong).date_naive(),
        day(2026, 2, 20),
        "hkex must reopen on 20 February 2026"
    );
}

#[test]
fn queries_before_the_2010_floor_are_refused() {
    let calendar = calendar();
    let error = calendar
        .is_open(hk((2009, 12, 31), (12, 0, 0)))
        .expect_err("a pre-floor query must refuse");
    assert!(
        (super::prelude::PRE_FLOOR_REFUSAL)(&error),
        "hkex must refuse 2009-12-31 with BeforeSupportFloor, got {error:?}"
    );
    assert!(
        (super::prelude::PRE_FLOOR_REFUSAL)(
            &calendar
                .is_open(hk((2004, 1, 22), (12, 0, 0)))
                .expect_err("a pre-floor query must refuse")
        ),
        "the refusal applies across the whole pre-floor range"
    );
}

#[test]
fn mutating_a_shipped_row_fails_a_test() {
    // The two row kinds are fenced value by value: an `Closed` flipped to an
    // `EarlyClose` fails `assert_closed`'s kind check, an `EarlyClose` with a
    // moved instant fails `assert_early_close`'s end-exclusive probes. This
    // test fences the third leg a row carries — its document id — so a row
    // whose id stops resolving to the cited edition fails too.
    let calendar = calendar();
    let closed = calendar
        .holiday_on(day(2026, 4, 3))
        .expect("2026-04-03 ships a row");
    assert_eq!(
        closed.kind(),
        HolidayKind::Closed,
        "mutation check: the closure kind is what the module ships"
    );
    assert_eq!(closed.document_id(), "HKEX-TC-2026-2027");
    let eve = calendar
        .holiday_on(day(2026, 12, 24))
        .expect("2026-12-24 ships a row");
    assert_eq!(
        eve.kind(),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 10 * 60
        },
        "mutation check: the half-day close is the printed 12:10 edge"
    );
    // The half-day morning is untouched and the afternoon is gone.
    assert!(
        calendar
            .is_open(hk((2026, 12, 24), (11, 59, 59)))
            .expect("the coverage contract must answer a covered date"),
        "the half-day morning session trades"
    );
    assert!(
        !calendar
            .is_open(hk((2026, 12, 24), (14, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the half-day afternoon is deleted"
    );
}
