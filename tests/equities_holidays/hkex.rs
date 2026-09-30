// SPDX-License-Identifier: MIT-0

//! HKEX holiday rows, 2010-2027: the operator's own holiday schedule, with the
//! Lunar New Year, Christmas and New Year eves as early closes (12:30 in the
//! 2010-2011 era, 12:10 in the CAS era) and the 2012-2015 eves `Unsourced`.

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
fn coverage_opens_at_the_2010_floor_and_stops_at_the_2027_schedule() {
    let calendar = calendar();
    let coverage = calendar
        .holiday_coverage()
        .expect("hkex ships a built-in table");
    assert_eq!(coverage.first(), day(2010, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    let windows = coverage.windows();
    assert_eq!(
        windows,
        vec![
            (day(2010, 1, 1), day(2024, 12, 31)),
            (day(2025, 1, 1), day(2027, 12, 31)),
        ],
        "two audited windows, one per document wave"
    );
    // Outside the windows the table has no answer at all.
    assert_eq!(calendar.holiday_on(day(2009, 12, 31)), None);
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
fn every_printed_2010_row_ships_with_the_era_half_day_close() {
    let calendar = calendar();
    assert_closed(
        "hkex",
        calendar,
        Asia::Hong_Kong,
        &[
            day(2010, 1, 1),
            day(2010, 2, 15),
            day(2010, 2, 16),
            day(2010, 4, 2),
            day(2010, 4, 5),
            day(2010, 4, 6),
            day(2010, 5, 21),
            day(2010, 6, 16),
            day(2010, 7, 1),
            day(2010, 9, 23),
            day(2010, 10, 1),
            day(2010, 12, 27),
        ],
    );
    // The 2010 eves are the era's half days: 9:30am-12:30pm, no afternoon
    // session (HKEX-TN-2010), so the close is the 12:30 morning-session edge.
    // Both dates sit below the row's 2011-03-03 carried horizon, so the
    // identity-backed session layer refuses them and the rows are fenced at
    // row level.
    for eve in [(2010, 12, 24), (2010, 12, 31)] {
        assert_early_close_row("hkex", calendar, eve, 12 * 3_600 + 30 * 60, "HKEX-TC-2010");
    }
    // Weekend-falling holidays key no weekday row: 2010-12-25 was a Saturday.
    assert_eq!(calendar.holiday_on(day(2010, 12, 25)), None);
}

#[test]
fn every_printed_2011_row_ships_and_the_lunar_eve_is_the_era_half_day() {
    let calendar = calendar();
    assert_closed(
        "hkex",
        calendar,
        Asia::Hong_Kong,
        &[
            day(2011, 2, 3),
            day(2011, 2, 4),
            day(2011, 4, 5),
            day(2011, 4, 22),
            day(2011, 4, 25),
            day(2011, 5, 2),
            day(2011, 5, 10),
            day(2011, 6, 6),
            day(2011, 7, 1),
            day(2011, 9, 13),
            day(2011, 10, 5),
            day(2011, 12, 26),
            day(2011, 12, 27),
        ],
    );
    // 2 February 2011, the Lunar New Year eve, is a half-day trading day on
    // the pre-2011-03-07 grid: the close is the era's 12:30 edge. The date
    // sits below the carried horizon, so the row is fenced at row level.
    assert_early_close_row(
        "hkex",
        calendar,
        (2011, 2, 2),
        12 * 3_600 + 30 * 60,
        "HKEX-TC-2011",
    );
    // New Year's Day 2011 fell on a Saturday: the calendar's own Saturday
    // closure subsumes it and no weekday row ships.
    assert_eq!(calendar.holiday_on(day(2011, 1, 1)), None);
    assert_eq!(calendar.holiday_on(day(2011, 1, 3)), None);
}

#[test]
fn the_2012_2015_half_day_eves_ship_unsourced() {
    let calendar = calendar();
    // Each year's calendar names these eves half-day trading days, but no era
    // artifact states the 2012-2015 half-day close, so the rows withhold
    // (#208) instead of inventing an instant.
    for (year, month, day_of_month, document) in [
        (2012, 12, 24, "HKEX-TC-2012"),
        (2012, 12, 31, "HKEX-TC-2012"),
        (2013, 12, 24, "HKEX-TC-2013"),
        (2013, 12, 31, "HKEX-TC-2013"),
        (2014, 1, 30, "HKEX-TC-2014"),
        (2014, 12, 24, "HKEX-TC-2014"),
        (2014, 12, 31, "HKEX-TC-2014"),
        (2015, 2, 18, "HKEX-TC-2015"),
        (2015, 12, 24, "HKEX-TC-2015"),
        (2015, 12, 31, "HKEX-TC-2015"),
    ] {
        let date = day(year, month, day_of_month);
        let holiday = calendar
            .holiday_on(date)
            .unwrap_or_else(|| panic!("{date} ships a row"));
        assert_eq!(
            holiday.kind(),
            HolidayKind::Unsourced,
            "{date} must ship Unsourced"
        );
        assert_eq!(holiday.tier(), EvidenceTier::T1, "{date}");
        assert_eq!(holiday.document_id(), document, "{date}");
    }
}

#[test]
fn every_printed_2012_row_ships_including_labour_day() {
    let calendar = calendar();
    // The 2012 calendar's holiday list prints fourteen weekday closures, from
    // `2/1` through `26/12`: Labour Day (`1/5`) among them.
    assert_closed(
        "hkex",
        calendar,
        Asia::Hong_Kong,
        &[
            day(2012, 1, 2),
            day(2012, 1, 23),
            day(2012, 1, 24),
            day(2012, 1, 25),
            day(2012, 4, 4),
            day(2012, 4, 6),
            day(2012, 4, 9),
            day(2012, 5, 1),
            day(2012, 7, 2),
            day(2012, 10, 1),
            day(2012, 10, 2),
            day(2012, 10, 23),
            day(2012, 12, 25),
            day(2012, 12, 26),
        ],
    );
    // The year's two eves are the shared 2012-2015 `Unsourced` shape; they are
    // enumerated here too so the year's full row set is fenced in one place.
    for (month, day_of_month) in [(12, 24), (12, 31)] {
        let date = day(2012, month, day_of_month);
        let holiday = calendar
            .holiday_on(date)
            .unwrap_or_else(|| panic!("{date} ships a row"));
        assert_eq!(
            holiday.kind(),
            HolidayKind::Unsourced,
            "{date} must ship Unsourced"
        );
        assert_eq!(holiday.tier(), EvidenceTier::T1, "{date}");
        assert_eq!(holiday.document_id(), "HKEX-TC-2012", "{date}");
    }
    // The calendar's list also prints three holidays that fell on a Saturday —
    // the day following Good Friday (7 April), the Buddha's Birthday
    // (28 April) and Tuen Ng Festival (23 June) — and the calendar's own
    // Saturday closure subsumes them: no weekday row ships.
    for (month, day_of_month) in [(4, 7), (4, 28), (6, 23)] {
        assert_eq!(calendar.holiday_on(day(2012, month, day_of_month)), None);
    }
}

#[test]
fn every_printed_2016_closure_ships_and_2016_names_no_half_day() {
    let calendar = calendar();
    assert_closed(
        "hkex",
        calendar,
        Asia::Hong_Kong,
        &[
            day(2016, 1, 1),
            day(2016, 2, 8),
            day(2016, 2, 9),
            day(2016, 2, 10),
            day(2016, 3, 25),
            day(2016, 3, 28),
            day(2016, 4, 4),
            day(2016, 5, 2),
            day(2016, 6, 9),
            day(2016, 7, 1),
            day(2016, 9, 16),
            day(2016, 10, 10),
            day(2016, 12, 26),
            day(2016, 12, 27),
        ],
    );
    // The 2016 calendar's footer names no half-day trading day: the Lunar New
    // Year, Christmas and New Year eves all fell on weekends.
    assert_eq!(calendar.holiday_on(day(2016, 2, 5)), None);
}

#[test]
fn every_printed_2017_row_ships_with_the_cas_era_half_day() {
    let calendar = calendar();
    assert_closed(
        "hkex",
        calendar,
        Asia::Hong_Kong,
        &[
            day(2017, 1, 2),
            day(2017, 1, 30),
            day(2017, 1, 31),
            day(2017, 4, 4),
            day(2017, 4, 14),
            day(2017, 4, 17),
            day(2017, 5, 1),
            day(2017, 5, 3),
            day(2017, 5, 30),
            day(2017, 10, 2),
            day(2017, 10, 5),
            day(2017, 12, 25),
            day(2017, 12, 26),
        ],
    );
    // 27 January 2017, the Lunar New Year eve, closes at the CAS-era 12:10
    // edge.
    assert_early_close(
        "hkex",
        calendar,
        Asia::Hong_Kong,
        (2017, 1, 27),
        12 * 3_600 + 10 * 60,
        "HKEX-TC-2017",
    );
}

#[test]
fn the_2018_2024_page_editions_ship_their_rows() {
    let calendar = calendar();
    // One representative closure per page edition, each citing the edition
    // that prints the year.
    for (date, document) in [
        (day(2018, 2, 16), "HKEX-TC-PAGE-2018"),
        (day(2019, 5, 13), "HKEX-TC-PAGE-2019"),
        (day(2020, 10, 2), "HKEX-TC-PAGE-2020"),
        (day(2021, 4, 6), "HKEX-TC-PAGE-2021"),
        (day(2022, 9, 12), "HKEX-TC-PAGE-2022"),
        (day(2023, 5, 26), "HKEX-TC-PAGE-2023"),
        (day(2024, 9, 18), "HKEX-TC-PAGE-2024"),
    ] {
        let holiday = calendar
            .holiday_on(date)
            .unwrap_or_else(|| panic!("{date} ships a row"));
        assert_eq!(holiday.kind(), HolidayKind::Closed, "{date}");
        assert_eq!(holiday.tier(), EvidenceTier::T1, "{date}");
        assert_eq!(holiday.document_id(), document, "{date}");
    }
    // The CAS-era eves across the page era close at 12:10 end-exclusive.
    for eve in [
        (2018, 12, 24),
        (2019, 2, 4),
        (2020, 1, 24),
        (2021, 12, 31),
        (2022, 1, 31),
        (2024, 12, 31),
    ] {
        assert_early_close(
            "hkex",
            calendar,
            Asia::Hong_Kong,
            eve,
            12 * 3_600 + 10 * 60,
            match eve.0 {
                2018 => "HKEX-TC-PAGE-2018",
                2019 => "HKEX-TC-PAGE-2019",
                2020 => "HKEX-TC-PAGE-2020",
                2021 => "HKEX-TC-PAGE-2021",
                2022 => "HKEX-TC-PAGE-2022",
                _ => "HKEX-TC-PAGE-2024",
            },
        );
    }
    // 2023 names no securities eve at all: the Lunar New Year, Christmas and
    // New Year eves fell on weekends.
    assert_eq!(calendar.holiday_on(day(2023, 12, 29)), None);
}

#[test]
fn a_pre_2025_ordinary_weekday_trades_to_the_end_exclusive_close() {
    let calendar = calendar();
    // Wednesday 2019-07-10: inside the window, no row.
    assert_eq!(calendar.holiday_on(day(2019, 7, 10)), None);
    assert!(
        calendar
            .is_open(hk((2019, 7, 10), (16, 9, 59)))
            .expect("the coverage contract must answer a covered date"),
        "16:09:59 is inside the closing-auction tail"
    );
    assert!(
        !calendar
            .is_open(hk((2019, 7, 10), (16, 10, 0)))
            .expect("the coverage contract must answer a covered date"),
        "16:10:00 is the end-exclusive close"
    );
    // The 2010-12-24 half day used to sit below the carried horizon; since
    // 2026-09-30 UTC the operator's own Trading Hours page sources the grid to
    // the floor (docs/evidence/hkex.md), so the day answers: the sheet's
    // 9:30am-12:30pm half day is open at noon and closed from the end-exclusive
    // 12:30 close.
    let calendar_open = calendar
        .is_open(hk((2010, 12, 24), (12, 0, 0)))
        .expect("a sourced-era query must answer");
    assert!(calendar_open, "noon is inside the half day");
    assert!(
        !calendar
            .is_open(hk((2010, 12, 24), (12, 30, 0)))
            .expect("a sourced-era query must answer"),
        "12:30 is the end-exclusive half-day close"
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
