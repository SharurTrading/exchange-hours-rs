// SPDX-License-Identifier: MIT-0

//! SIX holiday rows, 2010-2027: the operator's per-year Trading Calendar PDFs
//! (the 28 May 2018 guide edition's own 2018-2019 grids, and the era's
//! Trading-and-Settlement-Calendar pages' own market-holiday marks for
//! 2010-2011), closures only.

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
fn coverage_runs_from_the_2010_floor_to_the_2027_schedule() {
    let calendar = calendar();
    let coverage = calendar
        .holiday_coverage()
        .expect("six ships a built-in table");
    assert_eq!(coverage.first(), day(2010, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    let windows = coverage.windows();
    assert_eq!(
        windows,
        vec![
            (day(2010, 1, 1), day(2011, 12, 31)),
            (day(2012, 1, 1), day(2017, 12, 31)),
            (day(2018, 1, 1), day(2019, 12, 31)),
            (day(2020, 1, 1), day(2024, 12, 31)),
            (day(2025, 1, 1), day(2027, 12, 31)),
        ],
        "five audited windows; the 2010-2011 half of #212 closed as data on \
         2026-10-05 UTC and no span between the floor and 2027-12-31 remains"
    );
    // Weekend-falling holidays of 2010-2011 key no weekday row either.
    assert_eq!(calendar.holiday_on(day(2011, 1, 2)), None);
    assert_eq!(calendar.holiday_on(day(2010, 8, 1)), None);
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
fn every_2010_and_2011_market_holiday_mark_ships() {
    let calendar = calendar();
    // The 2010 grid's `SIX Swiss Exchange Market holiday` shading, resolved
    // from the captures of 2010-01-31 and 2010-04-11: seven weekday marks.
    assert_closed(
        "six",
        calendar,
        Europe::Zurich,
        &[
            day(2010, 1, 1),
            day(2010, 4, 2),
            day(2010, 4, 5),
            day(2010, 5, 13),
            day(2010, 5, 24),
            day(2010, 12, 24),
            day(2010, 12, 31),
        ],
    );
    // The 2011 grid's marks (published 15 November 2010): six weekday marks.
    assert_closed(
        "six",
        calendar,
        Europe::Zurich,
        &[
            day(2011, 4, 22),
            day(2011, 4, 25),
            day(2011, 6, 2),
            day(2011, 6, 13),
            day(2011, 8, 1),
            day(2011, 12, 26),
        ],
    );
    // The holiday names the operator's calendar family uses key each row to
    // its own year's page.
    for (date, document) in [
        (day(2010, 1, 1), "SIX-TSC-2010"),
        (day(2010, 12, 31), "SIX-TSC-2010"),
        (day(2011, 8, 1), "SIX-TSC-2011"),
        (day(2011, 12, 26), "SIX-TSC-2011"),
    ] {
        let holiday = calendar
            .holiday_on(date)
            .unwrap_or_else(|| panic!("{date} ships a row"));
        assert_eq!(holiday.kind(), HolidayKind::Closed, "{date}");
        assert_eq!(holiday.tier(), EvidenceTier::T1, "{date}");
        assert_eq!(holiday.document_id(), document, "{date}");
    }
    // The weekend falls of 2010-2011 — St. Berchtold 2010-01-02 (Saturday),
    // Labour Day 2010-05-01 (Saturday), Swiss National Day 2010-08-01
    // (Sunday), Christmas 2010-12-25 and St. Stephen's 2010-12-26 (weekend),
    // New Year 2011-01-01 and St. Berchtold 2011-01-02 (weekend), Labour Day
    // 2011-05-01 (Sunday), Christmas Eve 2011-12-24, Christmas 2011-12-25 and
    // New Year's Eve 2011-12-31 (weekend) — shade away and key no row, the
    // grids' own convention.
    for weekend_fall in [
        (2010, 1, 2),
        (2010, 5, 1),
        (2010, 8, 1),
        (2010, 12, 25),
        (2010, 12, 26),
        (2011, 1, 1),
        (2011, 1, 2),
        (2011, 5, 1),
        (2011, 12, 24),
        (2011, 12, 25),
        (2011, 12, 31),
    ] {
        assert_eq!(
            calendar.holiday_on(day(weekend_fall.0, weekend_fall.1, weekend_fall.2)),
            None,
            "{weekend_fall:?} keys no weekday row"
        );
    }
    // The marks close behaviourally: Good Friday 2010 noon and Ascension 2011
    // noon are closed, and trading resumes on the next ordinary day.
    assert!(
        !calendar
            .is_open(ch((2010, 4, 2), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "Good Friday 2010 is a `Closed` row, not an ordinary day"
    );
    assert!(
        !calendar
            .is_open(ch((2011, 6, 2), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "Ascension Day 2011 is a `Closed` row, not an ordinary day"
    );
    // 2010-12-24 closes, the 25-26 weekend follows, the 27-30 weekdays are
    // ordinary, and 2010-12-31 closes again.
    let next = calendar
        .next_session_after(ch((2010, 12, 24), (12, 0, 0)))
        .expect("the coverage contract must answer a covered date")
        .expect("a reopening must exist inside the bounded search");
    assert_eq!(
        next.0.with_timezone(&Europe::Zurich).date_naive(),
        day(2010, 12, 27),
        "six must reopen on Monday 27 December 2010"
    );
    assert!(
        !calendar
            .is_open(ch((2010, 12, 31), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "New Year's Eve 2010 is a `Closed` row"
    );
}

#[test]
fn every_printed_2018_and_2019_cell_of_the_may_2018_guide_ships() {
    let calendar = calendar();
    // The "Trading Calendar 2018" section of the Trading Guide of 28 May
    // 2018: twelve dark `Market Holiday — Market Closed` cells.
    assert_closed(
        "six",
        calendar,
        Europe::Zurich,
        &[
            day(2018, 1, 1),
            day(2018, 1, 2),
            day(2018, 3, 30),
            day(2018, 4, 2),
            day(2018, 5, 1),
            day(2018, 5, 10),
            day(2018, 5, 21),
            day(2018, 8, 1),
            day(2018, 12, 24),
            day(2018, 12, 25),
            day(2018, 12, 26),
            day(2018, 12, 31),
        ],
    );
    // The same guide's "Trading Calendar 2019" section: twelve more.
    assert_closed(
        "six",
        calendar,
        Europe::Zurich,
        &[
            day(2019, 1, 1),
            day(2019, 1, 2),
            day(2019, 4, 19),
            day(2019, 4, 22),
            day(2019, 5, 1),
            day(2019, 5, 30),
            day(2019, 6, 10),
            day(2019, 8, 1),
            day(2019, 12, 24),
            day(2019, 12, 25),
            day(2019, 12, 26),
            day(2019, 12, 31),
        ],
    );
    // All 24 dark cells of these two grids fall on weekdays and no SIX
    // holiday of 2018-2019 falls on a weekend, so the grids arise no
    // weekend-fall exclusion; the weekend shading itself still keys no row.
    for weekend in [
        day(2018, 3, 31),
        day(2018, 4, 1), // Easter Sunday 2018, shaded as a weekend day
        day(2019, 6, 15),
        day(2019, 12, 28),
    ] {
        assert_eq!(
            calendar.holiday_on(weekend),
            None,
            "{weekend} is the calendar's own weekend shape, not a holiday row"
        );
    }
    // Both years' closures cite the guide edition that prints their grids.
    for probed in [day(2018, 5, 10), day(2019, 4, 19)] {
        let holiday = calendar
            .holiday_on(probed)
            .unwrap_or_else(|| panic!("{probed} ships a row"));
        assert_eq!(holiday.kind(), HolidayKind::Closed, "{probed}");
        assert_eq!(holiday.tier(), EvidenceTier::T1, "{probed}");
        assert_eq!(holiday.document_id(), "SIX-TG-2018", "{probed}");
    }
    // The 2018-2019 rows changed the refusal: an ordinary weekday inside the
    // formerly unaudited span answers, and the holiday closes it.
    assert!(
        calendar
            .is_open(ch((2018, 6, 6), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ch((2019, 4, 19), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "Good Friday 2019 is a `Closed` row, not an ordinary day"
    );
    // The window seam between the 2011 and 2012 tables is seamless coverage:
    // 2011-12-30 (Friday) is an ordinary day of the TSC-2011 window and
    // answers, while 2012-01-02, the first trade date of the 2012 table,
    // closes through its St. Berchtold row.
    assert!(
        calendar
            .is_open(ch((2011, 12, 30), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "2011-12-30 is an ordinary Friday once the TSC-2011 marks ship"
    );
    assert!(
        !calendar
            .is_open(ch((2012, 1, 2), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the first 2012 trade date closes through its St. Berchtold row"
    );
    // Past the last window the refusal contract still holds.
    let error = calendar
        .is_open(ch((2028, 1, 3), (12, 0, 0)))
        .expect_err("a query beyond the 2027 schedule must refuse");
    assert!(
        matches!(error, CalendarQueryError::OutsideCoveredRange { .. }),
        "2028-01-03 must refuse with OutsideCoveredRange, got {error:?}"
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
fn a_pre_2025_ordinary_weekday_trades_and_the_former_gap_answers() {
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
    // The formerly unaudited 2010-2011 span answers since 2026-10-05 UTC (the
    // era's Trading-and-Settlement-Calendar pages' own market-holiday marks):
    // Sunday 2010-06-06 closes by the normal week, and the ordinary weekdays
    // 2011-05-12 and 2011-12-30 trade.
    assert!(
        !calendar
            .is_open(ch((2010, 6, 6), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "2010-06-06 is a Sunday inside the covered window"
    );
    for (label, instant) in [
        ("2011-05-12", ch((2011, 5, 12), (12, 0, 0))),
        ("2011-12-30", ch((2011, 12, 30), (12, 0, 0))),
    ] {
        assert!(
            calendar
                .is_open(instant)
                .unwrap_or_else(|error| panic!("{label} must answer, got {error:?}")),
            "{label} is an ordinary covered weekday"
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
        (day(2010, 4, 2), "SIX-TSC-2010"),
        (day(2011, 6, 13), "SIX-TSC-2011"),
        (day(2018, 5, 10), "SIX-TG-2018"),
        (day(2019, 12, 31), "SIX-TG-2018"),
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
