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
fn coverage_opens_at_the_2010_floor_and_stops_at_the_2027_schedule() {
    let calendar = calendar();
    let coverage = calendar
        .holiday_coverage()
        .expect("xetra ships a built-in table");
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
fn every_printed_2010_closure_ships_as_a_settlement_day_closure() {
    let calendar = calendar();
    assert_closed(
        "xetra",
        calendar,
        Europe::Berlin,
        &[
            day(2010, 1, 1),
            day(2010, 4, 2),
            day(2010, 4, 5),
            day(2010, 12, 24),
            day(2010, 12, 31),
        ],
    );
    // 24 and 31 December are the sentence's settlement days: no trading, so
    // the noon probe inside `assert_closed` must answer closed, not open.
    assert!(
        !calendar
            .is_open(de((2010, 12, 24), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "a settlement day is a closure, not a trading day"
    );
    // 1 May 2010 fell on a Saturday: the sentence's Mondays-to-Fridays premise
    // deletes it and no weekday row ships.
    assert_eq!(calendar.holiday_on(day(2010, 5, 1)), None);
}

#[test]
fn the_2011_sentence_names_three_closures_and_no_unity_day() {
    let calendar = calendar();
    assert_closed(
        "xetra",
        calendar,
        Europe::Berlin,
        &[day(2011, 4, 22), day(2011, 4, 25), day(2011, 12, 26)],
    );
    // New Year 2011 (Saturday), Labour Day (Sunday) and the Christmas/New Year
    // eves (Saturdays) are weekend deletions; no rows and no weekday answer.
    assert_eq!(calendar.holiday_on(day(2011, 1, 1)), None);
    assert_eq!(calendar.holiday_on(day(2011, 12, 24)), None);
    // The sentence names no 3 October — a Monday in 2011 — so the exchange
    // traded that German Unity Day as an ordinary day.
    assert_eq!(calendar.holiday_on(day(2011, 10, 3)), None);
    assert!(
        calendar
            .is_open(de((2011, 10, 3), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the 2011 sentence omits 3 October, so the Monday traded"
    );
}

#[test]
fn the_2014_german_edition_closes_unity_day_and_the_settlement_eves() {
    let calendar = calendar();
    assert_closed(
        "xetra",
        calendar,
        Europe::Berlin,
        &[
            day(2014, 1, 1),
            day(2014, 4, 18),
            day(2014, 4, 21),
            day(2014, 5, 1),
            day(2014, 10, 3),
            day(2014, 12, 24),
            day(2014, 12, 25),
            day(2014, 12, 26),
            day(2014, 12, 31),
        ],
    );
    let unity = calendar
        .holiday_on(day(2014, 10, 3))
        .expect("2014-10-03 ships a row");
    assert_eq!(unity.document_id(), "DB-TC-PDF-2014");
}

#[test]
fn the_2016_closures_ship_and_the_weekend_eves_key_no_row() {
    let calendar = calendar();
    assert_closed(
        "xetra",
        calendar,
        Europe::Berlin,
        &[
            day(2016, 1, 1),
            day(2016, 3, 25),
            day(2016, 3, 28),
            day(2016, 5, 16),
            day(2016, 10, 3),
            day(2016, 12, 26),
        ],
    );
    // 24, 25 and 31 December 2016 fell on weekends: the sentence's exception
    // dates delete themselves and key no weekday row.
    assert_eq!(calendar.holiday_on(day(2016, 12, 24)), None);
    assert_eq!(calendar.holiday_on(day(2016, 12, 25)), None);
    assert_eq!(calendar.holiday_on(day(2016, 12, 31)), None);
}

#[test]
fn from_2022_whit_monday_and_unity_day_trade_as_ordinary_days() {
    let calendar = calendar();
    // The 2022 sentence names only Good Friday, Easter Monday and Boxing Day.
    assert_closed(
        "xetra",
        calendar,
        Europe::Berlin,
        &[day(2022, 4, 15), day(2022, 4, 18), day(2022, 12, 26)],
    );
    // Whit Monday 2022-06-06 and German Unity Day 2022-10-03 are named by no
    // closure sentence from 2022 on: ordinary trading days.
    for traded in [(2022, 6, 6), (2022, 10, 3)] {
        assert_eq!(
            calendar.holiday_on(day(traded.0, traded.1, traded.2)),
            None,
            "{traded:?} ships no row"
        );
        assert!(
            calendar
                .is_open(de(traded, (12, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{traded:?} trades as an ordinary day from 2022 on"
        );
    }
    // 2013 names no 3 October either (a Thursday that year).
    assert_eq!(calendar.holiday_on(day(2013, 10, 3)), None);
    assert!(
        calendar
            .is_open(de((2013, 10, 3), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the 2013 sentence omits 3 October, so the Thursday traded"
    );
}

#[test]
fn a_pre_2025_ordinary_weekday_trades_to_the_end_exclusive_auction_edge() {
    let calendar = calendar();
    // Wednesday 2015-07-08: inside the window, no row.
    assert_eq!(calendar.holiday_on(day(2015, 7, 8)), None);
    assert!(
        calendar
            .is_open(de((2015, 7, 8), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "an ordinary midday is open"
    );
    // The era's closing auction ends 17:35:30; end-exclusive on both sides.
    assert!(
        calendar
            .is_open(de((2015, 7, 8), (17, 35, 29)))
            .expect("the coverage contract must answer a covered date"),
        "17:35:29 is inside the closing auction"
    );
    assert!(
        !calendar
            .is_open(de((2015, 7, 8), (17, 35, 30)))
            .expect("the coverage contract must answer a covered date"),
        "17:35:30 is the end-exclusive auction close"
    );
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
fn the_2026_final_session_replaces_retail_late_with_shortened_post_trading() {
    let calendar = calendar();
    let row = calendar
        .holiday_on(day(2026, 12, 30))
        .expect("the final session has a sourced replacement");
    assert!(matches!(row.kind(), HolidayKind::ReplacementBlocks(_)));
    assert_eq!(row.document_id(), "DB-FINAL-SESSION-2026");
    assert_eq!(row.tier(), EvidenceTier::T1);

    for (time, open) in [
        ((7, 59, 59), false),
        ((8, 0, 0), true),
        ((8, 55, 0), true),
        ((14, 0, 0), true),
        ((14, 5, 0), true),
        ((14, 9, 59), true),
        ((14, 10, 0), false),
        ((15, 0, 0), false),
        ((21, 0, 0), false),
    ] {
        assert_eq!(
            calendar
                .is_open(de((2026, 12, 30), time))
                .expect("the final trading date is covered"),
            open,
            "the final session's executable envelope at {time:?}"
        );
    }
    for (time, order_entry) in [
        ((6, 59, 59), false),
        ((7, 0, 0), true),
        ((8, 0, 0), false),
        ((14, 9, 59), false),
        ((14, 10, 0), true),
        ((14, 59, 59), true),
        ((15, 0, 0), false),
    ] {
        assert_eq!(
            calendar
                .is_order_entry_only(de((2026, 12, 30), time))
                .expect("the final trading date is covered"),
            order_entry,
            "pre-trading and post-trading remain order entry at {time:?}"
        );
    }
    for (time, regular) in [
        ((9, 0, 29), false),
        ((9, 0, 30), true),
        ((13, 0, 0), false),
        ((13, 2, 30), true),
        ((13, 59, 59), true),
        ((14, 0, 0), false),
    ] {
        assert_eq!(
            calendar
                .is_open_regular(de((2026, 12, 30), time))
                .expect("the final trading date is covered"),
            regular,
            "the final session's continuous phases at {time:?}"
        );
    }
    assert!(
        calendar
            .is_open(de((2026, 12, 29), (21, 0, 0)))
            .expect("the preceding ordinary date is covered")
    );
    let next = calendar
        .next_session_after(de((2026, 12, 30), (15, 0, 0)))
        .expect("the New Year interval is covered")
        .expect("the January reopening exists");
    assert_eq!(next.0, de((2027, 1, 4), (8, 0, 0)));
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
    // The 2027 PDF marks trading holidays only with the conditional
    // "Trading hours may differ" footnote and states no final hours, so no
    // early close is inferred; Ascension answers as an ordinary Thursday.
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
    // A historical closure cites the per-year calendar that prints its year.
    let closure_2019 = calendar
        .holiday_on(day(2019, 6, 10))
        .expect("2019-06-10 ships a row");
    assert_eq!(closure_2019.kind(), HolidayKind::Closed);
    assert_eq!(closure_2019.document_id(), "DB-TC-PDF-2019");
}
