// SPDX-License-Identifier: MIT-0

//! SIX holiday rows, 2025-2027: the operator's per-year Trading Calendar PDFs,
//! closures only.

use super::prelude::*;

fn ch(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    local(Europe::Zurich, date, time)
}

fn day(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("fixture must be a valid date")
}

fn calendar() -> ExchangeCalendar {
    calendar_for_exchange(Exchange::Six)
}

#[test]
fn coverage_opens_at_the_2012_backfill_and_stops_at_the_2027_schedule() {
    let calendar = calendar();
    let coverage = calendar
        .holiday_coverage()
        .expect("six ships a built-in table");
    assert_eq!(coverage.first(), day(2012, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    let windows = coverage.windows();
    assert_eq!(
        windows,
        vec![
            (day(2012, 1, 1), day(2017, 12, 31)),
            (day(2020, 1, 1), day(2024, 12, 31)),
            (day(2025, 1, 1), day(2027, 12, 31)),
        ],
        "three audited windows; 2010-2011 and 2018-2019 are the unaudited spans (#212)"
    );
    // Inside the unaudited spans the table has no answer at all.
    assert_eq!(calendar.holiday_on(day(2011, 1, 2)), None);
    assert_eq!(calendar.holiday_on(day(2018, 8, 1)), None);
    assert_eq!(calendar.holiday_on(day(2019, 12, 25)), None);
    // Outside the audited history entirely: the same.
    assert_eq!(calendar.holiday_on(day(2009, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2028, 1, 1)), None);
}

#[test]
fn every_printed_2012_cell_ships() {
    let calendar = calendar();
    assert_closed(
        "six",
        calendar,
        Europe::Zurich,
        &[
            day(2012, 1, 2),
            day(2012, 4, 6),
            day(2012, 4, 9),
            day(2012, 5, 1),
            day(2012, 5, 17),
            day(2012, 5, 28),
            day(2012, 8, 1),
            day(2012, 12, 24),
            day(2012, 12, 25),
            day(2012, 12, 26),
            day(2012, 12, 31),
        ],
    );
    // New Year 2012 fell on a Sunday: the grid marks no cell and no row ships.
    assert_eq!(calendar.holiday_on(day(2012, 1, 1)), None);
    // The red cells cite the year's own PDF.
    assert_eq!(
        calendar
            .holiday_on(day(2012, 5, 17))
            .expect("2012-05-17 ships a row")
            .document_id(),
        "SIX-TC-2012"
    );
}

#[test]
fn the_2016_grid_marks_seven_weekday_holidays_and_no_weekend_fall() {
    let calendar = calendar();
    assert_closed(
        "six",
        calendar,
        Europe::Zurich,
        &[
            day(2016, 1, 1),
            day(2016, 3, 25),
            day(2016, 3, 28),
            day(2016, 5, 5),
            day(2016, 5, 16),
            day(2016, 8, 1),
            day(2016, 12, 26),
        ],
    );
    // Christmas Day 2016 fell on a Sunday: the Sunday shading deletes it and
    // no weekday row ships.
    assert_eq!(calendar.holiday_on(day(2016, 12, 25)), None);
}

#[test]
fn the_2022_grid_marks_six_weekday_holidays() {
    let calendar = calendar();
    assert_closed(
        "six",
        calendar,
        Europe::Zurich,
        &[
            day(2022, 4, 15),
            day(2022, 4, 18),
            day(2022, 5, 26),
            day(2022, 6, 6),
            day(2022, 8, 1),
            day(2022, 12, 26),
        ],
    );
    // New Year 2022 (Saturday), Christmas Eve (Saturday), Christmas Day
    // (Sunday) and New Year's Eve (Saturday) shade away.
    for weekend_fall in [(2022, 1, 1), (2022, 12, 24), (2022, 12, 25), (2022, 12, 31)] {
        assert_eq!(
            calendar.holiday_on(day(weekend_fall.0, weekend_fall.1, weekend_fall.2)),
            None,
            "{weekend_fall:?} keys no weekday row"
        );
    }
    assert_eq!(
        calendar
            .holiday_on(day(2022, 8, 1))
            .expect("2022-08-01 ships a row")
            .document_id(),
        "SIX-TC-2022"
    );
}

#[test]
fn every_printed_2024_cell_ships() {
    let calendar = calendar();
    assert_closed(
        "six",
        calendar,
        Europe::Zurich,
        &[
            day(2024, 1, 1),
            day(2024, 1, 2),
            day(2024, 3, 29),
            day(2024, 4, 1),
            day(2024, 5, 1),
            day(2024, 5, 9),
            day(2024, 5, 20),
            day(2024, 8, 1),
            day(2024, 12, 24),
            day(2024, 12, 25),
            day(2024, 12, 26),
            day(2024, 12, 31),
        ],
    );
}

#[test]
fn a_pre_2025_ordinary_weekday_trades_and_the_unaudited_spans_refuse() {
    let calendar = calendar();
    // Wednesday 2015-07-08: inside the 2012-2017 window, no row, and the
    // session layer answers the ordinary day end-exclusively.
    assert_eq!(calendar.holiday_on(day(2015, 7, 8)), None);
    assert!(
        calendar
            .is_open(ch((2015, 7, 8), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "an ordinary pre-2025 midday is open"
    );
    assert!(
        !calendar
            .is_open(ch((2015, 7, 8), (17, 40, 0)))
            .expect("the coverage contract must answer a covered date"),
        "17:40 is the end-exclusive Trading-At-Last close"
    );
    // The unaudited spans refuse with the coverage contract: the operator's
    // trading calendar for them is archived on no channel (#212).
    for (label, instant) in [
        ("2011-05-12", ch((2011, 5, 12), (12, 0, 0))),
        ("2018-06-06", ch((2018, 6, 6), (12, 0, 0))),
        ("2019-10-03", ch((2019, 10, 3), (12, 0, 0))),
    ] {
        let error = calendar
            .is_open(instant)
            .expect_err("an unaudited-span query must refuse");
        assert!(
            matches!(error, CalendarQueryError::OutsideCoveredRange { .. }),
            "{label} must refuse with OutsideCoveredRange, got {error:?}"
        );
    }
}

#[test]
fn every_printed_2025_cell_ships() {
    let calendar = calendar();
    assert_closed(
        "six",
        calendar,
        Europe::Zurich,
        &[
            day(2025, 1, 1),
            day(2025, 1, 2),
            day(2025, 4, 18),
            day(2025, 4, 21),
            day(2025, 5, 1),
            day(2025, 5, 29),
            day(2025, 6, 9),
            day(2025, 8, 1),
            day(2025, 12, 24),
            day(2025, 12, 25),
            day(2025, 12, 26),
            day(2025, 12, 31),
        ],
    );
    // Every closure cites its own year's calendar PDF.
    assert_eq!(
        calendar
            .holiday_on(day(2025, 8, 1))
            .expect("2025-08-01 ships a row")
            .document_id(),
        "SIX-TC-2025"
    );
}

#[test]
fn every_printed_2026_cell_ships() {
    let calendar = calendar();
    assert_closed(
        "six",
        calendar,
        Europe::Zurich,
        &[
            day(2026, 1, 1),
            day(2026, 1, 2),
            day(2026, 4, 3),
            day(2026, 4, 6),
            day(2026, 5, 1),
            day(2026, 5, 14),
            day(2026, 5, 25),
            day(2026, 12, 24),
            day(2026, 12, 25),
            day(2026, 12, 31),
        ],
    );
    // The Swiss National Day 2026-08-01 and St. Stephen's Day 2026-12-26 fall
    // on Saturdays: the PDF shades them as weekend days and they key no row.
    assert_eq!(calendar.holiday_on(day(2026, 8, 1)), None);
    assert_eq!(calendar.holiday_on(day(2026, 12, 26)), None);
    assert!(
        !calendar
            .is_open(ch((2026, 8, 1), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "a Saturday is closed by the normal week, not by a holiday row"
    );
}

#[test]
fn every_printed_2027_cell_ships() {
    let calendar = calendar();
    assert_closed(
        "six",
        calendar,
        Europe::Zurich,
        &[
            day(2027, 1, 1),
            day(2027, 3, 26),
            day(2027, 3, 29),
            day(2027, 5, 6),
            day(2027, 5, 17),
            day(2027, 12, 24),
            day(2027, 12, 31),
        ],
    );
    // St. Berchtold Day 2027 (Saturday), Labour Day 2027 (Saturday), Swiss
    // National Day 2027 (Sunday), Christmas Day 2027 (Saturday) and St.
    // Stephen's Day 2027 (Sunday) are weekend shading, not rows.
    for weekend_holiday in [
        (2027, 1, 2),
        (2027, 5, 1),
        (2027, 8, 1),
        (2027, 12, 25),
        (2027, 12, 26),
    ] {
        assert_eq!(
            calendar.holiday_on(day(weekend_holiday.0, weekend_holiday.1, weekend_holiday.2)),
            None,
            "a weekend-falling holiday keys no weekday row: {weekend_holiday:?}"
        );
    }
}

#[test]
fn an_ordinary_weekday_trades_to_the_end_exclusive_close() {
    let calendar = calendar();
    // Wednesday 2026-07-08 has no row — and nor does Corpus Christi
    // 2026-06-04, which closes Xetra but not SIX, so the two venues' tables
    // disagree by design.
    assert_eq!(calendar.holiday_on(day(2026, 7, 8)), None);
    assert_eq!(calendar.holiday_on(day(2026, 6, 4)), None);
    assert!(
        calendar
            .is_open(ch((2026, 6, 4), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "Corpus Christi is a SIX trading day"
    );
    assert!(
        calendar
            .is_open(ch((2026, 7, 8), (17, 39, 59)))
            .expect("the coverage contract must answer a covered date"),
        "17:39:59 is inside Trading-At-Last, the last executable phase"
    );
    assert!(
        !calendar
            .is_open(ch((2026, 7, 8), (17, 40, 0)))
            .expect("the coverage contract must answer a covered date"),
        "17:40:00 is the end-exclusive close; the 17:40-22:00 tail is order entry"
    );
    assert!(
        !calendar
            .is_open(ch((2026, 7, 8), (21, 59, 59)))
            .expect("the coverage contract must answer a covered date"),
        "the post-trading tail accepts orders only, so it never reads open"
    );
}

#[test]
fn the_christmas_block_reopens_on_the_first_open_day() {
    let calendar = calendar();
    // 2025-12-24, 25 and 26 are closures and 27-28 the weekend, so trading
    // resumes Monday 2025-12-29.
    let next = calendar
        .next_session_after(ch((2025, 12, 24), (12, 0, 0)))
        .expect("the coverage contract must answer a covered date")
        .expect("a reopening must exist inside the bounded search");
    assert_eq!(
        next.0.with_timezone(&Europe::Zurich).date_naive(),
        day(2025, 12, 29),
        "six must reopen on 29 December 2025"
    );
}

#[test]
fn queries_before_the_2010_floor_are_refused() {
    let calendar = calendar();
    let error = calendar
        .is_open(ch((2009, 12, 31), (12, 0, 0)))
        .expect_err("a pre-floor query must refuse");
    assert!(
        (super::prelude::PRE_FLOOR_REFUSAL)(&error),
        "six must refuse 2009-12-31 with BeforeSupportFloor, got {error:?}"
    );
}

#[test]
fn mutating_a_shipped_row_fails_a_test() {
    let calendar = calendar();
    // Every row in this table is a `Closed`, so the kind check is the mutation
    // fence for the kind, and the noon probe inside `assert_closed` is the
    // behavioural one. This test adds the per-year document split: a row that
    // stops citing its own year's PDF fails.
    for (date, document) in [
        (day(2025, 4, 18), "SIX-TC-2025"),
        (day(2026, 5, 14), "SIX-TC-2026"),
        (day(2027, 5, 6), "SIX-TC-2027"),
    ] {
        let holiday = calendar
            .holiday_on(date)
            .unwrap_or_else(|| panic!("{date} ships a row"));
        assert_eq!(holiday.kind(), HolidayKind::Closed, "{date}");
        assert_eq!(holiday.tier(), EvidenceTier::T1, "{date}");
        assert_eq!(holiday.document_id(), document, "{date}");
    }
}
