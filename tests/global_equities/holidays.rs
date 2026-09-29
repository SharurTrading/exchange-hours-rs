// SPDX-License-Identifier: MIT-0

//! Built-in holiday rows for the served cash-equity venues whose tables
//! shipped with the 2025-2027 wave: `b3` (2025-2026 extended back to 2011,
//! the 2010 span unaudited), `tadawul` (2021-2027, the earlier span
//! unaudited) and `borsa_istanbul` (2012-03-02..2026-12-31), the APAC venues `nzx` (2010-2024 backfilled beside the
//! operator's 2025-2027-01-04 rolling horizon, with the 2016-2017 capture gap
//! refusing), `asx` (2010-2024 backfilled from each year's own operator
//! sheet, beside 2025-2027) and `sgx_securities` (2014-2020-01-01 backfilled
//! beside 2025-2026, the 2010-2013 and 2020-2024 capture gaps refusing), and
//! the European/American venues `lse` (2010-2015 and 2020-2027, the
//! rolling table's archived states, with five 2025 dates withheld and the two
//! capture gaps refusing), `euronext_paris` (2025-2026; the 2026 half-day
//! hours are announced but unstated) and `tsx` (2025-2026).
//!
//! Every case below goes through the public identity-backed calendar, the
//! same surface the consumer routes through. Each venue's section fences its
//! own kinds: closures per year, the half-day and late-open instants at
//! second granularity so a row that moves fails, an ordinary weekday inside
//! the window, the end-exclusive close, the coverage endpoints, the pre-floor
//! refusal, and a per-kind census so a row cannot change kind silently.
//!
//! The Islamic-calendar rows (Tadawul) and the Carnival rows (B3) are fenced
//! against the dates the operators printed, not against a computed calendar:
//! each entry names its own printed day, so a row keyed to a computed rather
//! than printed date fails here.
//!
//! The APAC modules below walk every shipped row of their windows and tally
//! the kinds per year against the operators' sheets — which is the mutation
//! fence: flipping any one shipped row (its date, its kind or its instant)
//! fails that walk.

#![expect(
    clippy::panic,
    reason = "the shared walk helpers below are plain functions rather than #[test] \
              functions, so the test switches in clippy.toml do not reach them; a \
              kind an operator does not print must fail the walk loudly"
)]

use super::prelude::*;

use chrono::{Datelike as _, TimeDelta, TimeZone as _, Utc};
use chrono_tz::{America, Asia, Australia, Europe, Pacific};
use exchange_hours::{
    CalendarQueryError, CalendarResolution, Exchange, ExchangeCalendar, Holiday, HolidayKind,
    SessionKind, calendar_for_exchange,
};

fn day(year: i32, month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
}

fn sao_paulo(date: (i32, u32, u32), time: (u32, u32, u32)) -> chrono::DateTime<chrono::Utc> {
    local(America::Sao_Paulo, date, time)
}

fn riyadh(date: (i32, u32, u32), time: (u32, u32, u32)) -> chrono::DateTime<chrono::Utc> {
    Asia::Riyadh
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous Riyadh instant")
        .with_timezone(&chrono::Utc)
}

fn istanbul(date: (i32, u32, u32), time: (u32, u32, u32)) -> chrono::DateTime<chrono::Utc> {
    local(Europe::Istanbul, date, time)
}

fn calendar_for(exchange: Exchange) -> ExchangeCalendar {
    calendar_for_exchange(exchange)
}

/// The instant constructor one section uses, threaded through the helpers so
/// every probe stays in the venue's own zone.
type At = dyn Fn((i32, u32, u32), (u32, u32, u32)) -> chrono::DateTime<chrono::Utc>;

/// Asserts a closure row on `date`: the kind, no session in either phase, and
/// the whole venue-local day shut.
///
/// The window's first closure is special exactly as the CFE suite states it:
/// the derivation reads behind the query's bounds, and for 2025-01-01 that
/// reads 2024-12-31, outside every audited window, so every identity-backed
/// probe on the day refuses rather than answers.
fn assert_closed(calendar: ExchangeCalendar, date: (i32, u32, u32), label: &str, at: &At) {
    assert_eq!(
        calendar
            .holiday_on(day(date.0, date.1, date.2))
            .map(Holiday::kind),
        Some(HolidayKind::Closed),
        "{label}: {date:?} carries a closure row"
    );
    let covered = day(date.0, date.1, date.2);
    let coverage = calendar
        .holiday_coverage()
        .expect("these venues ship built-in tables");
    if coverage.first() == covered {
        // The window's first closure is special exactly as the CFE suite
        // states it: the derivation reads behind the query's bounds — the day
        // before the window, outside every audited window — so the
        // identity-backed probes on the day refuse rather than answer.
        assert!(
            calendar
                .is_closed_trade_date(covered, SessionKind::Both)
                .is_err_and(|error| matches!(
                    error,
                    CalendarQueryError::OutsideCoveredRange { .. }
                )),
            "{label}: the derivation behind {date:?} reads outside the audited windows"
        );
        assert!(
            calendar
                .is_open(at(date, (11, 0, 0)))
                .is_err_and(|error| matches!(
                    error,
                    CalendarQueryError::OutsideCoveredRange { .. }
                )),
            "{label}: probes on {date:?} refuse while the derivation reaches behind the window"
        );
    } else {
        assert!(
            calendar
                .is_closed_trade_date(covered, SessionKind::Both)
                .expect("the coverage contract must answer a covered date"),
            "{label}: {date:?} has no session in either phase"
        );
        assert!(
            !calendar
                .is_open(at(date, (11, 0, 0)))
                .expect("a covered date answers"),
            "{label}: {date:?} is shut midday"
        );
    }
}

/// Asserts an ordinary weekday: no row, the regular session open midday, and
/// the end-exclusive close both ways.
fn assert_ordinary_weekday(
    calendar: ExchangeCalendar,
    date: (i32, u32, u32),
    open: (u32, u32),
    close: (u32, u32),
    instant_of: &At,
) {
    assert_eq!(
        calendar.holiday_on(day(date.0, date.1, date.2)),
        None,
        "{date:?} is audited normal and carries no row"
    );
    let open_at = instant_of(date, (open.0, open.1, 0));
    let close_at = instant_of(date, (close.0, close.1, 0));
    assert!(
        calendar.is_open(open_at).expect("a covered date answers"),
        "{date:?} is open at its regular open"
    );
    assert!(
        calendar
            .is_open(close_at - chrono::TimeDelta::nanoseconds(1))
            .expect("a covered date answers"),
        "{date:?} is open one nanosecond before its close"
    );
    assert!(
        !calendar.is_open(close_at).expect("a covered date answers"),
        "{date:?} closes are end-exclusive"
    );
}

/// One nanosecond before a venue-local wall-clock instant, for end-exclusive
/// bounds.
fn before(
    tz: chrono_tz::Tz,
    date: (i32, u32, u32),
    time: (u32, u32, u32),
) -> chrono::DateTime<Utc> {
    tz.with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous local instant")
        .with_timezone(&Utc)
        - TimeDelta::nanoseconds(1)
}

/// One walked row: its venue-local date as a triple, its kind label, and the
/// scalar instant the kind states when it states one.
type WalkedRow = ((i32, u32, u32), String, Option<u32>);

/// Asserts `date` is a printed closure: the row kind, no session in either
/// phase, and a shut envelope at intraday probes.
fn assert_closure(calendar: ExchangeCalendar, date: (i32, u32, u32), label: &str) {
    let d = day(date.0, date.1, date.2);
    assert_eq!(
        calendar.holiday_on(d).map(Holiday::kind),
        Some(HolidayKind::Closed),
        "{label}: {date:?} carries a printed closure"
    );
    assert!(
        calendar
            .is_closed_trade_date(d, SessionKind::Both)
            .expect("the coverage contract must answer a covered date"),
        "{label}: {date:?} has no session in either phase"
    );
}

/// Counts every row in the table's window by kind, per year, and returns the
/// per-year tallies so each venue's walk can compare them against the sheets.
fn rows_per_year(calendar: ExchangeCalendar) -> Vec<WalkedRow> {
    let coverage = calendar
        .holiday_coverage()
        .expect("these venues ship built-in tables");
    let mut rows = Vec::new();
    let mut date = coverage.first();
    while date <= coverage.last() {
        if let Some(holiday) = calendar.holiday_on(date) {
            // `HolidayKind` is `#[non_exhaustive]`: a kind these operators do
            // not print fails the walk loudly instead of tallying silently.
            let instant = match holiday.kind() {
                HolidayKind::Closed
                | HolidayKind::Unsourced
                | HolidayKind::ReplacementBlocks(_) => None,
                HolidayKind::EarlyClose { close_ssm } => Some(close_ssm),
                other @ (HolidayKind::LateOpen { .. }
                | HolidayKind::LateOpenAndEarlyClose { .. }) => {
                    panic!("{date} ships a kind these operators do not print: {other:?}")
                }
                other => panic!("{date} ships an unknown kind: {other:?}"),
            };
            let kind = match holiday.kind() {
                HolidayKind::Closed => "closed".to_owned(),
                HolidayKind::ReplacementBlocks(_) => "replacement".to_owned(),
                HolidayKind::EarlyClose { .. } => "early close".to_owned(),
                HolidayKind::Unsourced => "unsourced".to_owned(),
                other => panic!("{date} ships an unknown kind: {other:?}"),
            };
            rows.push(((date.year(), date.month(), date.day()), kind, instant));
        }
        date = date
            .succ_opt()
            .expect("the windows stay inside the representable calendar");
    }
    rows
}

fn tally(rows: &[WalkedRow], year: i32) -> (usize, usize, usize) {
    let mut closed = 0;
    let mut early = 0;
    let mut replacement = 0;
    for (date, kind, _) in rows {
        if date.0 != year {
            continue;
        }
        match kind.as_str() {
            "closed" => closed += 1,
            "early close" => early += 1,
            "replacement" => replacement += 1,
            other => panic!("{year} ships an unexpected kind: {other}"),
        }
    }
    (closed, early, replacement)
}

/// The four-way tally for venues whose sheets also carry dates the crate
/// withholds: an operator era can leave a date inside its window printed by
/// nothing (`Unsourced`), and a walk that panicked on that kind could not
/// fence a table that ships it.
fn census(rows: &[WalkedRow], year: i32) -> (usize, usize, usize, usize) {
    let mut closed = 0;
    let mut early = 0;
    let mut replacement = 0;
    let mut unsourced = 0;
    for (date, kind, _) in rows {
        if date.0 != year {
            continue;
        }
        match kind.as_str() {
            "closed" => closed += 1,
            "early close" => early += 1,
            "replacement" => replacement += 1,
            "unsourced" => unsourced += 1,
            other => panic!("{year} ships an unexpected kind: {other}"),
        }
    }
    (closed, early, replacement, unsourced)
}

// ---------------------------------------------------------------------------
// b3 — B3 (Brasil, Bolsa, Balcão), 2011-2024 backfill plus 2025-2026,
// America/Sao_Paulo.
// ---------------------------------------------------------------------------

/// One operator year's weekday closure list and the label its probes carry.
type B3YearClosures<'a> = (&'a [(i32, u32, u32)], &'a str);

/// The weekday closure rows per calendar year, exactly as the operator's own
/// artifacts print them: the yearly announcements (2011, 2012, 2015), the
/// `Calendário do Mercado` pages (2013, 2014), the PUMA `Feriados` captures
/// (2016-2024) and the 2025 circular and 2026 article. A date that changes or
/// disappears breaks the per-year walk.
const B3_CLOSURES_PER_YEAR: &[B3YearClosures<'_>] = &[
    (
        &[
            (2011, 1, 25),
            (2011, 3, 7),
            (2011, 3, 8),
            (2011, 4, 21),
            (2011, 4, 22),
            (2011, 6, 23),
            (2011, 9, 7),
            (2011, 10, 12),
            (2011, 11, 2),
            (2011, 11, 15),
            (2011, 12, 30),
        ],
        "B3 2011",
    ),
    (
        &[
            (2012, 1, 25),
            (2012, 2, 20),
            (2012, 2, 21),
            (2012, 4, 6),
            (2012, 5, 1),
            (2012, 6, 7),
            (2012, 7, 9),
            (2012, 9, 7),
            (2012, 10, 12),
            (2012, 11, 2),
            (2012, 11, 15),
            (2012, 11, 20),
            (2012, 12, 24),
            (2012, 12, 25),
            (2012, 12, 31),
        ],
        "B3 2012",
    ),
    (
        &[
            (2013, 1, 1),
            (2013, 1, 25),
            (2013, 2, 11),
            (2013, 2, 12),
            (2013, 3, 29),
            (2013, 5, 1),
            (2013, 5, 30),
            (2013, 7, 9),
            (2013, 11, 15),
            (2013, 11, 20),
            (2013, 12, 24),
            (2013, 12, 25),
            (2013, 12, 31),
        ],
        "B3 2013",
    ),
    (
        &[
            (2014, 1, 1),
            (2014, 3, 3),
            (2014, 3, 4),
            (2014, 4, 18),
            (2014, 4, 21),
            (2014, 5, 1),
            (2014, 6, 19),
            (2014, 7, 9),
            (2014, 11, 20),
            (2014, 12, 24),
            (2014, 12, 25),
            (2014, 12, 31),
        ],
        "B3 2014",
    ),
    (
        &[
            (2015, 1, 1),
            (2015, 2, 16),
            (2015, 2, 17),
            (2015, 4, 3),
            (2015, 4, 21),
            (2015, 5, 1),
            (2015, 6, 4),
            (2015, 7, 9),
            (2015, 9, 7),
            (2015, 10, 12),
            (2015, 11, 2),
            (2015, 11, 20),
            (2015, 12, 24),
            (2015, 12, 25),
            (2015, 12, 31),
        ],
        "B3 2015",
    ),
    (
        &[
            (2016, 1, 1),
            (2016, 1, 25),
            (2016, 2, 8),
            (2016, 2, 9),
            (2016, 3, 25),
            (2016, 4, 21),
            (2016, 5, 26),
            (2016, 9, 7),
            (2016, 10, 12),
            (2016, 11, 2),
            (2016, 11, 15),
            (2016, 12, 30),
        ],
        "B3 2016",
    ),
    (
        &[
            (2017, 1, 25),
            (2017, 2, 27),
            (2017, 2, 28),
            (2017, 4, 14),
            (2017, 4, 21),
            (2017, 5, 1),
            (2017, 6, 15),
            (2017, 9, 7),
            (2017, 10, 12),
            (2017, 11, 2),
            (2017, 11, 15),
            (2017, 11, 20),
            (2017, 12, 25),
            (2017, 12, 29),
        ],
        "B3 2017",
    ),
    (
        &[
            (2018, 1, 1),
            (2018, 1, 25),
            (2018, 2, 12),
            (2018, 2, 13),
            (2018, 3, 30),
            (2018, 5, 1),
            (2018, 5, 31),
            (2018, 7, 9),
            (2018, 9, 7),
            (2018, 10, 12),
            (2018, 11, 2),
            (2018, 11, 15),
            (2018, 11, 20),
            (2018, 12, 24),
            (2018, 12, 25),
            (2018, 12, 31),
        ],
        "B3 2018",
    ),
    (
        &[
            (2019, 1, 1),
            (2019, 1, 25),
            (2019, 3, 4),
            (2019, 3, 5),
            (2019, 4, 19),
            (2019, 5, 1),
            (2019, 6, 20),
            (2019, 7, 9),
            (2019, 11, 15),
            (2019, 11, 20),
            (2019, 12, 24),
            (2019, 12, 25),
            (2019, 12, 31),
        ],
        "B3 2019",
    ),
    (
        &[
            (2020, 1, 1),
            (2020, 2, 24),
            (2020, 2, 25),
            (2020, 4, 10),
            (2020, 4, 21),
            (2020, 5, 1),
            (2020, 6, 11),
            (2020, 7, 9),
            (2020, 9, 7),
            (2020, 10, 12),
            (2020, 11, 2),
            (2020, 11, 20),
            (2020, 12, 24),
            (2020, 12, 25),
            (2020, 12, 31),
        ],
        "B3 2020",
    ),
    (
        &[
            (2021, 1, 1),
            (2021, 1, 25),
            (2021, 2, 15),
            (2021, 2, 16),
            (2021, 4, 2),
            (2021, 4, 21),
            (2021, 6, 3),
            (2021, 7, 9),
            (2021, 9, 7),
            (2021, 10, 12),
            (2021, 11, 2),
            (2021, 11, 15),
            (2021, 12, 24),
            (2021, 12, 31),
        ],
        "B3 2021",
    ),
    (
        &[
            (2022, 2, 28),
            (2022, 3, 1),
            (2022, 4, 15),
            (2022, 4, 21),
            (2022, 6, 16),
            (2022, 9, 7),
            (2022, 10, 12),
            (2022, 11, 2),
            (2022, 11, 15),
            (2022, 12, 30),
        ],
        "B3 2022",
    ),
    (
        &[
            (2023, 2, 20),
            (2023, 2, 21),
            (2023, 4, 7),
            (2023, 4, 21),
            (2023, 5, 1),
            (2023, 6, 8),
            (2023, 9, 7),
            (2023, 10, 12),
            (2023, 11, 2),
            (2023, 11, 15),
            (2023, 12, 25),
            (2023, 12, 29),
        ],
        "B3 2023",
    ),
    (
        &[
            (2024, 1, 1),
            (2024, 2, 12),
            (2024, 2, 13),
            (2024, 3, 29),
            (2024, 5, 1),
            (2024, 5, 30),
            (2024, 11, 15),
            (2024, 11, 20),
            (2024, 12, 24),
            (2024, 12, 25),
            (2024, 12, 31),
        ],
        "B3 2024",
    ),
    (
        &[
            (2025, 1, 1),
            (2025, 3, 3),
            (2025, 3, 4),
            (2025, 4, 18),
            (2025, 4, 21),
            (2025, 5, 1),
            (2025, 6, 19),
            (2025, 11, 20),
            (2025, 12, 24),
            (2025, 12, 25),
            (2025, 12, 31),
        ],
        "B3 2025",
    ),
    (
        &[
            (2026, 1, 1),
            (2026, 2, 16),
            (2026, 2, 17),
            (2026, 4, 3),
            (2026, 4, 21),
            (2026, 5, 1),
            (2026, 6, 4),
            (2026, 9, 7),
            (2026, 10, 12),
            (2026, 11, 2),
            (2026, 11, 20),
            (2026, 12, 24),
            (2026, 12, 25),
            (2026, 12, 31),
        ],
        "B3 2026",
    ),
];

#[test]
fn b3_closures_per_year_match_the_operators_printed_calendars() {
    let calendar = calendar_for(Exchange::B3);
    // Every weekday closure of the operator's yearly lists, per year: the
    // announcement articles (2011, 2012, 2015), the `Calendário do Mercado`
    // pages (2013, 2014) and the PUMA `Feriados` captures (2016-2024), then
    // the 2025 circular and the 2026 article. Weekend legs are fenced
    // separately below.
    for (dates, label) in B3_CLOSURES_PER_YEAR {
        for date in *dates {
            assert_closed(calendar, *date, label, &|d, time| sao_paulo(d, time));
        }
    }
    // The Christmas Eve closure deletes the day itself; the 2025-12-24 row is
    // what removes trading, and Christmas Day stays shut beside it.
    assert!(
        !calendar
            .is_open(sao_paulo((2025, 12, 24), (11, 0, 0)))
            .expect("a covered date answers"),
        "Christmas Eve 2025 has no equities session at all"
    );
}

/// Weekend legs and the operator's own "trades anyway" dates print no session
/// change and ship no row: a weekend leg changes no Monday-Friday trade date,
/// and the 2022-2024 São Paulo-holiday rows state `Haverá negociação nos
/// mercados de renda variável`.
#[test]
fn b3_weekend_legs_and_sao_paulo_holidays_are_audited_normal() {
    let calendar = calendar_for(Exchange::B3);
    // Weekend legs: some printed by the operator (2019-04-21, 2019-09-07,
    // 2022-01-01), some unprinted; all carry no row either way.
    for date in [
        (2011, 1, 1),
        (2014, 1, 25),
        (2019, 4, 21),
        (2019, 9, 7),
        (2019, 11, 2),
        (2020, 1, 25),
        (2021, 11, 20),
        (2022, 1, 1),
        (2022, 11, 20),
        (2023, 12, 24),
        (2024, 4, 21),
    ] {
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "{date:?} is a weekend leg and carries no row"
        );
        assert!(
            calendar
                .is_closed_trade_date(day(date.0, date.1, date.2), SessionKind::Both)
                .expect("the coverage contract must answer a covered date"),
            "{date:?} is shut by the normal week"
        );
    }
    // The São Paulo holidays the operator's own calendars state trade: no
    // row, and an ordinary midday open (both grids open at 10:00).
    for date in [
        (2022, 1, 25),
        (2023, 1, 25),
        (2023, 11, 20),
        (2024, 1, 25),
        (2024, 7, 9),
    ] {
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "{date:?} prints `Haverá negociação nos mercados de renda variável`"
        );
        assert!(
            calendar
                .is_open(sao_paulo(date, (11, 0, 0)))
                .expect("a covered date answers"),
            "{date:?} trades on the operator's own statement"
        );
    }
}

/// Ash Wednesday is a **late open**: the operator states special hours with
/// equities trading beginning at 13:00 and no changed close. The probes are
/// at second granularity so a row that takes any other instant fails.
#[test]
fn b3_ash_wednesday_opens_late_at_1300_sao_paulo() {
    let calendar = calendar_for(Exchange::B3);

    for date in [
        (2011, 3, 9),
        (2012, 2, 22),
        (2013, 2, 13),
        (2014, 3, 5),
        (2015, 2, 18),
        (2016, 2, 10),
        (2017, 3, 1),
        (2018, 2, 14),
        (2019, 3, 6),
        (2020, 2, 26),
        (2021, 2, 17),
        (2022, 3, 2),
        (2023, 2, 22),
        (2024, 2, 14),
        (2025, 3, 5),
        (2026, 2, 18),
    ] {
        assert_eq!(
            calendar
                .holiday_on(day(date.0, date.1, date.2))
                .map(Holiday::kind),
            Some(HolidayKind::LateOpen {
                open_ssm: 13 * 3_600
            }),
            "{date:?} carries the printed late open"
        );
        // The printed 12:45-13:00 pre-opening accepts no orders that match,
        // and continuous trading begins exactly at 13:00.
        assert!(
            !calendar
                .is_open(sao_paulo(date, (12, 59, 59)))
                .expect("a covered date answers"),
            "{date:?} is closed at 12:59:59"
        );
        assert!(
            calendar
                .is_open(sao_paulo(date, (13, 0, 0)))
                .expect("a covered date answers"),
            "{date:?} opens at 13:00"
        );
    }

    // On the 2025-2026 long-grid days the close is the unchanged 17:55 with
    // the closing call to 18:00, end-exclusive.
    for date in [(2025, 3, 5), (2026, 2, 18)] {
        assert!(
            calendar
                .is_open(sao_paulo(date, (17, 54, 59)))
                .expect("a covered date answers"),
            "{date:?} still trades through the long-grid afternoon"
        );
        assert!(
            calendar
                .is_open(sao_paulo(date, (17, 55, 0)))
                .expect("a covered date answers"),
            "{date:?} keeps the closing call at 17:55"
        );
        assert!(
            !calendar
                .is_open(sao_paulo(date, (18, 0, 0)))
                .expect("a covered date answers"),
            "{date:?} closes at the normal 18:00"
        );
        assert_eq!(
            calendar
                .session_bounds(sao_paulo(date, (14, 0, 0)))
                .expect("a covered date answers"),
            Some((sao_paulo(date, (13, 0, 0)), sao_paulo(date, (17, 55, 0)))),
            "{date:?} regular session is the printed 13:00-17:55"
        );
    }
    // The late open deletes the ordinary morning: a mutated row that clipped
    // nothing would leave the 10:00 open standing.
    assert!(
        !calendar
            .is_open(sao_paulo((2025, 3, 5), (10, 0, 0)))
            .expect("a covered date answers"),
        "the ordinary 10:00 open is gone on Ash Wednesday"
    );
    // Trading dates either side are ordinary long-grid days.
    assert!(
        calendar
            .is_open(sao_paulo((2025, 3, 6), (10, 0, 0)))
            .expect("a covered date answers"),
        "the day after Ash Wednesday opens normally"
    );
}

#[test]
fn b3_window_ordinary_weekdays_and_coverage_endpoints() {
    let calendar = calendar_for(Exchange::B3);
    // An ordinary weekday inside the window: June is US daylight time, so the
    // short grid is in force - regular 10:00-16:55 and a final 18:00 close
    // after the closing call and after-market envelope, end-exclusive.
    assert_ordinary_weekday(calendar, (2025, 6, 17), (10, 0), (18, 0), &sao_paulo);
    // The short grid's regular session ends at 16:55 and hands straight into
    // the closing call: the venue keeps trading, the regular session does not.
    let (_, regular_close) = calendar
        .session_bounds(sao_paulo((2025, 6, 17), (16, 0, 0)))
        .expect("a covered date answers")
        .expect("a midday probe sits inside the regular session");
    assert_eq!(
        regular_close,
        sao_paulo((2025, 6, 17), (16, 55, 0)),
        "the short grid's regular close"
    );
    assert!(
        calendar
            .is_open_extended(sao_paulo((2025, 6, 17), (16, 56, 0)))
            .expect("a covered date answers"),
        "the closing call after the regular close is extended"
    );
    // An ordinary weekday inside the backfilled era: 2016 sits on the fixed
    // short grid, and its 16:55 regular close hands into the extended call.
    assert_ordinary_weekday(calendar, (2016, 6, 15), (10, 0), (17, 0), &sao_paulo);
    let coverage = calendar
        .holiday_coverage()
        .expect("B3 ships a built-in table");
    assert_eq!(coverage.first(), day(2011, 1, 1));
    assert_eq!(coverage.last(), day(2026, 12, 31));
    assert_eq!(
        calendar.holiday_on(coverage.first().pred_opt().expect("representable")),
        None,
        "no answer below the window: 2010 is the unaudited span"
    );
    // 2010 sits between the support floor and the audited window: the
    // identity refuses the date rather than claiming a normal day.
    let unaudited = sao_paulo((2010, 6, 2), (11, 0, 0));
    assert!(matches!(
        calendar.is_open(unaudited),
        Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == day(2010, 6, 2)
    ));
    // 2027-01-05 is inside 2027 but outside the audited window: the identity
    // refuses the date rather than claiming a normal Monday.
    let outside = sao_paulo((2027, 1, 5), (11, 0, 0));
    assert!(matches!(
        calendar.is_open(outside),
        Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == day(2027, 1, 5)
    ));
    // Pre-floor refusal.
    let ancient = sao_paulo((2009, 12, 31), (11, 0, 0));
    assert!(matches!(
        calendar.is_open(ancient),
        Err(CalendarQueryError::BeforeSupportFloor { .. })
    ));
}

#[test]
fn b3_window_ships_only_closures_and_one_late_open_shape() {
    let calendar = calendar_for(Exchange::B3);
    let coverage = calendar
        .holiday_coverage()
        .expect("B3 ships a built-in table");
    let mut closed = 0_usize;
    let mut late_opens = 0_usize;
    let mut date = coverage.first();
    while date <= coverage.last() {
        match calendar.holiday_on(date).map(Holiday::kind) {
            None => {}
            Some(HolidayKind::Closed) => closed += 1,
            Some(HolidayKind::LateOpen { open_ssm }) if open_ssm == 13 * 3_600 => late_opens += 1,
            Some(other) => {
                panic!("{date} ships a kind B3's calendars do not state: {other:?}")
            }
        }
        date = date.succ_opt().expect("the window stays representable");
    }
    assert_eq!(
        (closed, late_opens),
        (208, 16),
        "closures and late opens, B3 2011-2026"
    );
    // Every late open is the year's Ash Wednesday: one per calendar year of
    // the audited windows, so a duplicated or shifted row breaks this.
    let mut late_dates = Vec::new();
    let mut date = coverage.first();
    while date <= coverage.last() {
        if matches!(
            calendar.holiday_on(date).map(Holiday::kind),
            Some(HolidayKind::LateOpen { .. })
        ) {
            late_dates.push((date.year(), date.month(), date.day()));
        }
        date = date.succ_opt().expect("the window stays representable");
    }
    assert_eq!(
        late_dates,
        vec![
            (2011, 3, 9),
            (2012, 2, 22),
            (2013, 2, 13),
            (2014, 3, 5),
            (2015, 2, 18),
            (2016, 2, 10),
            (2017, 3, 1),
            (2018, 2, 14),
            (2019, 3, 6),
            (2020, 2, 26),
            (2021, 2, 17),
            (2022, 3, 2),
            (2023, 2, 22),
            (2024, 2, 14),
            (2025, 3, 5),
            (2026, 2, 18),
        ],
        "the late opens are exactly each year's printed Ash Wednesday"
    );
}

// ---------------------------------------------------------------------------
// tadawul — Saudi Exchange Main Market, 2021-2027, Asia/Riyadh (Sun-Thu).
// ---------------------------------------------------------------------------

#[test]
fn tadawul_closures_per_year_match_the_operators_printed_entries() {
    let calendar = calendar_for(Exchange::Tadawul);
    // 2021: the Eid Al Fiter first/last-day entry (13 and 16 May), the Eid Al
    // Adha legs between the printed 15/07 discontinue and 25/07 resume, and
    // National Day.
    for date in [
        (2021, 5, 13),
        (2021, 5, 16),
        (2021, 7, 18),
        (2021, 7, 19),
        (2021, 7, 20),
        (2021, 7, 21),
        (2021, 7, 22),
        (2021, 9, 23),
    ] {
        assert_closed(calendar, date, "Tadawul 2021 closure", &|d, time| {
            riyadh(d, time)
        });
    }
    // 2022: Founding Day, the Eid Al Fiter first/last-day entry's weekday
    // range, Eid Al Adha between 06/07 and 13/07, National Day.
    for date in [
        (2022, 2, 22),
        (2022, 4, 28),
        (2022, 5, 1),
        (2022, 5, 2),
        (2022, 5, 3),
        (2022, 5, 4),
        (2022, 5, 5),
        (2022, 7, 7),
        (2022, 7, 10),
        (2022, 7, 11),
        (2022, 7, 12),
        (2022, 9, 22),
    ] {
        assert_closed(calendar, date, "Tadawul 2022 closure", &|d, time| {
            riyadh(d, time)
        });
    }
    // 2023: Founding Day, Eid ranges, and the Sunday the National Day
    // arrangement removes between the printed 21/09 discontinue and 25/09
    // resume.
    for date in [
        (2023, 2, 22),
        (2023, 4, 18),
        (2023, 4, 19),
        (2023, 4, 20),
        (2023, 4, 23),
        (2023, 4, 24),
        (2023, 6, 25),
        (2023, 6, 26),
        (2023, 6, 27),
        (2023, 6, 28),
        (2023, 6, 29),
        (2023, 9, 24),
    ] {
        assert_closed(calendar, date, "Tadawul 2023 closure", &|d, time| {
            riyadh(d, time)
        });
    }
    // 2024: Founding Day, Eid ranges, National Day.
    for date in [
        (2024, 2, 22),
        (2024, 4, 7),
        (2024, 4, 8),
        (2024, 4, 9),
        (2024, 4, 10),
        (2024, 4, 11),
        (2024, 6, 16),
        (2024, 6, 17),
        (2024, 6, 18),
        (2024, 6, 19),
        (2024, 6, 20),
        (2024, 9, 23),
    ] {
        assert_closed(calendar, date, "Tadawul 2024 closure", &|d, time| {
            riyadh(d, time)
        });
    }
    // 2025: Founding Day observed Sunday 23 February (the operator's printed
    // day, not the civil 22nd), both Eid ranges' Sunday-Thursday legs, and
    // National Day.
    for date in [
        (2025, 2, 23),
        (2025, 3, 30),
        (2025, 3, 31),
        (2025, 4, 1),
        (2025, 4, 2),
        (2025, 6, 5),
        (2025, 6, 8),
        (2025, 6, 9),
        (2025, 6, 10),
        (2025, 9, 23),
    ] {
        assert_closed(calendar, date, "Tadawul 2025 closure", &|d, time| {
            riyadh(d, time)
        });
    }
    // 2026.
    for date in [
        (2026, 2, 22),
        (2026, 3, 17),
        (2026, 3, 18),
        (2026, 3, 19),
        (2026, 3, 22),
        (2026, 3, 23),
        (2026, 5, 24),
        (2026, 5, 25),
        (2026, 5, 26),
        (2026, 5, 27),
        (2026, 5, 28),
        (2026, 9, 23),
    ] {
        assert_closed(calendar, date, "Tadawul 2026 closure", &|d, time| {
            riyadh(d, time)
        });
    }
    // 2027: the operator published the full year, Eid ranges included.
    for date in [
        (2027, 2, 22),
        (2027, 3, 7),
        (2027, 3, 8),
        (2027, 3, 9),
        (2027, 3, 10),
        (2027, 3, 11),
        (2027, 5, 16),
        (2027, 5, 17),
        (2027, 5, 18),
        (2027, 5, 19),
        (2027, 5, 20),
        (2027, 9, 23),
    ] {
        assert_closed(calendar, date, "Tadawul 2027 closure", &|d, time| {
            riyadh(d, time)
        });
    }
    // The printed weekend legs — Friday 2025-03-28 and Saturday 2025-03-29 —
    // change no trade date and carry no row; the normal week already closes
    // them. The same reading holds in the backfilled years: the 2023 Eid Al
    // Adha range prints 23-24 June and the 2024 Eid Al Adha range prints
    // 21-22 June, both weekends.
    for date in [
        (2023, 6, 23),
        (2023, 6, 24),
        (2024, 6, 21),
        (2024, 6, 22),
        (2025, 3, 28),
        (2025, 3, 29),
    ] {
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "a Friday or Saturday leg needs no row in a Sun-Thu week: {date:?}"
        );
    }
}

#[test]
fn tadawul_trading_resumes_on_the_printed_days() {
    let calendar = calendar_for(Exchange::Tadawul);
    // The entries state resume days: 25/07/2021, 13/07/2022, 25/04/2023,
    // 02/07/2023, 25/09/2023, 14/04/2024, 23/06/2024, 03/04/2025, 11/06/2025,
    // 24/03/2026, 31/05/2026, 14/03/2027, 23/05/2027 — each open with the
    // ordinary 10:00 regular open.
    for date in [
        (2021, 7, 25),
        (2022, 7, 13),
        (2023, 4, 25),
        (2023, 7, 2),
        (2023, 9, 25),
        (2024, 4, 14),
        (2024, 6, 23),
        (2025, 4, 3),
        (2025, 6, 11),
        (2026, 3, 24),
        (2026, 5, 31),
        (2027, 3, 14),
        (2027, 5, 23),
    ] {
        assert!(
            calendar
                .is_open(riyadh(date, (10, 0, 0)))
                .expect("a covered date answers"),
            "{date:?} is the printed resume day and opens normally"
        );
        assert!(
            calendar
                .is_open(riyadh(date, (14, 59, 59)))
                .expect("a covered date answers"),
            "{date:?} trades through the regular close"
        );
        assert!(
            !calendar
                .is_open(riyadh(date, (15, 20, 0)))
                .expect("a covered date answers"),
            "{date:?} closes end-exclusive after trade at last"
        );
    }
}

#[test]
fn tadawul_window_ordinary_weekday_and_coverage_endpoints() {
    let calendar = calendar_for(Exchange::Tadawul);
    // An ordinary Tuesday inside the window: regular 10:00-15:00, extended to
    // 15:20, closes end-exclusive.
    assert_ordinary_weekday(calendar, (2025, 7, 8), (10, 0), (15, 20), &riyadh);
    let coverage = calendar
        .holiday_coverage()
        .expect("Tadawul ships a built-in table");
    assert_eq!(coverage.first(), day(2021, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert_eq!(
        calendar.holiday_on(coverage.first().pred_opt().expect("representable")),
        None,
        "no answer below the window: 2013-2020 is the unaudited span"
    );
    assert_eq!(
        calendar.holiday_on(coverage.last().succ_opt().expect("representable")),
        None
    );
    // A 2020 probe sits in the unaudited span: the identity refuses rather
    // than answering.
    let unaudited = riyadh((2020, 12, 15), (11, 0, 0));
    assert!(matches!(
        calendar.is_open(unaudited),
        Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == day(2020, 12, 15)
    ));
    // Pre-floor refusal.
    let ancient = riyadh((2009, 12, 31), (11, 0, 0));
    assert!(matches!(
        calendar.is_open(ancient),
        Err(CalendarQueryError::BeforeSupportFloor { .. })
    ));
}

#[test]
fn tadawul_window_ships_only_closures() {
    let calendar = calendar_for(Exchange::Tadawul);
    let coverage = calendar
        .holiday_coverage()
        .expect("Tadawul ships a built-in table");
    let mut closed = 0_usize;
    let mut date = coverage.first();
    while date <= coverage.last() {
        match calendar.holiday_on(date).map(Holiday::kind) {
            None => {}
            Some(HolidayKind::Closed) => closed += 1,
            Some(kind) => panic!("{date} ships a kind Tadawul's calendar does not state: {kind:?}"),
        }
        date = date.succ_opt().expect("the window stays representable");
    }
    assert_eq!(closed, 78, "closures, Tadawul 2021-2027");
}

// ---------------------------------------------------------------------------
// borsa_istanbul — Borsa İstanbul Equity Market, 2012-2026, Europe/Istanbul.
// ---------------------------------------------------------------------------

/// One operator year's weekday closure list and the label its probes carry.
type BistYearClosures<'a> = (&'a [(i32, u32, u32)], &'a str);

/// The weekday closure rows per calendar year, exactly as the operator's
/// `Resmi Tatil Günleri` page (2025-2026: with the annex) prints them. A date
/// that changes or disappears breaks the per-year walk.
const BIST_CLOSURES_PER_YEAR: &[BistYearClosures<'_>] = &[
    (
        &[
            (2012, 4, 23),
            (2012, 5, 1),
            (2012, 8, 20),
            (2012, 8, 21),
            (2012, 8, 30),
            (2012, 10, 25),
            (2012, 10, 26),
            (2012, 10, 29),
        ],
        "BIST 2012",
    ),
    (
        &[
            (2013, 1, 1),
            (2013, 4, 23),
            (2013, 5, 1),
            (2013, 8, 8),
            (2013, 8, 9),
            (2013, 8, 30),
            (2013, 10, 15),
            (2013, 10, 16),
            (2013, 10, 17),
            (2013, 10, 18),
            (2013, 10, 29),
        ],
        "BIST 2013",
    ),
    (
        &[
            (2014, 1, 1),
            (2014, 4, 23),
            (2014, 5, 1),
            (2014, 5, 19),
            (2014, 7, 28),
            (2014, 7, 29),
            (2014, 7, 30),
            (2014, 10, 6),
            (2014, 10, 7),
            (2014, 10, 29),
        ],
        "BIST 2014",
    ),
    (
        &[
            (2015, 1, 1),
            (2015, 4, 23),
            (2015, 5, 1),
            (2015, 5, 19),
            (2015, 7, 17),
            (2015, 9, 24),
            (2015, 9, 25),
            (2015, 10, 29),
        ],
        "BIST 2015",
    ),
    (
        &[
            (2016, 1, 1),
            (2016, 5, 19),
            (2016, 7, 5),
            (2016, 7, 6),
            (2016, 7, 7),
            (2016, 8, 30),
            (2016, 9, 12),
            (2016, 9, 13),
            (2016, 9, 14),
            (2016, 9, 15),
        ],
        "BIST 2016",
    ),
    (
        &[
            (2017, 5, 1),
            (2017, 5, 19),
            (2017, 6, 26),
            (2017, 6, 27),
            (2017, 8, 30),
            (2017, 9, 1),
            (2017, 9, 4),
        ],
        "BIST 2017",
    ),
    (
        &[
            (2018, 1, 1),
            (2018, 4, 23),
            (2018, 5, 1),
            (2018, 6, 15),
            (2018, 8, 21),
            (2018, 8, 22),
            (2018, 8, 23),
            (2018, 8, 24),
            (2018, 8, 30),
            (2018, 10, 29),
        ],
        "BIST 2018",
    ),
    (
        &[
            (2019, 1, 1),
            (2019, 4, 23),
            (2019, 5, 1),
            (2019, 6, 4),
            (2019, 6, 5),
            (2019, 6, 6),
            (2019, 7, 15),
            (2019, 8, 12),
            (2019, 8, 13),
            (2019, 8, 14),
            (2019, 8, 30),
            (2019, 10, 29),
        ],
        "BIST 2019",
    ),
    (
        &[
            (2020, 1, 1),
            (2020, 4, 23),
            (2020, 5, 1),
            (2020, 5, 19),
            (2020, 5, 25),
            (2020, 5, 26),
            (2020, 7, 15),
            (2020, 7, 31),
            (2020, 8, 3),
            (2020, 10, 29),
        ],
        "BIST 2020",
    ),
    (
        &[
            (2021, 1, 1),
            (2021, 4, 23),
            (2021, 5, 13),
            (2021, 5, 14),
            (2021, 5, 19),
            (2021, 7, 15),
            (2021, 7, 20),
            (2021, 7, 21),
            (2021, 7, 22),
            (2021, 7, 23),
            (2021, 8, 30),
            (2021, 10, 29),
        ],
        "BIST 2021",
    ),
    (
        &[
            (2022, 5, 2),
            (2022, 5, 3),
            (2022, 5, 4),
            (2022, 5, 19),
            (2022, 7, 11),
            (2022, 7, 12),
            (2022, 7, 15),
            (2022, 8, 30),
        ],
        "BIST 2022",
    ),
    (
        &[
            (2023, 4, 21),
            (2023, 5, 1),
            (2023, 5, 19),
            (2023, 6, 28),
            (2023, 6, 29),
            (2023, 6, 30),
            (2023, 8, 30),
        ],
        "BIST 2023",
    ),
    (
        &[
            (2024, 1, 1),
            (2024, 4, 10),
            (2024, 4, 11),
            (2024, 4, 12),
            (2024, 4, 23),
            (2024, 5, 1),
            (2024, 6, 17),
            (2024, 6, 18),
            (2024, 6, 19),
            (2024, 7, 15),
            (2024, 8, 30),
            (2024, 10, 29),
        ],
        "BIST 2024",
    ),
    (
        &[
            (2025, 1, 1),
            (2025, 3, 31),
            (2025, 4, 1),
            (2025, 4, 23),
            (2025, 5, 1),
            (2025, 5, 19),
            (2025, 6, 6),
            (2025, 6, 9),
            (2025, 7, 15),
            (2025, 10, 29),
        ],
        "BIST 2025",
    ),
    (
        &[
            (2026, 1, 1),
            (2026, 3, 20),
            (2026, 4, 23),
            (2026, 5, 1),
            (2026, 5, 19),
            (2026, 5, 27),
            (2026, 5, 28),
            (2026, 5, 29),
            (2026, 7, 15),
            (2026, 10, 29),
        ],
        "BIST 2026",
    ),
];

#[test]
fn borsa_istanbul_closures_per_year_match_the_operators_printed_tables() {
    let calendar = calendar_for(Exchange::BorsaIstanbul);
    // Every weekday closure of the operator's yearly tables, per year: the
    // `Resmi Tatil Günleri` page's year tabs (2012-2024) and the annex's
    // no-session dates (2025-2026). Weekend legs are fenced below.
    for (dates, label) in BIST_CLOSURES_PER_YEAR {
        for date in *dates {
            assert_closed(calendar, *date, label, &|d, time| istanbul(d, time));
        }
    }
    // Weekend legs (Zafer Bayramı 2025-08-30 and 2026-08-30, Ramazan Arefesi
    // 2025-03-29, and the backfilled years' weekend `Kapalı` legs) change no
    // trade date and ship no row.
    for date in [
        (2012, 1, 1),
        (2016, 4, 23),
        (2016, 10, 29),
        (2017, 1, 1),
        (2017, 4, 23),
        (2017, 10, 28),
        (2020, 5, 23),
        (2022, 1, 1),
        (2023, 4, 23),
        (2025, 3, 29),
        (2025, 8, 30),
        (2026, 8, 30),
    ] {
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "a Saturday or Sunday holiday needs no row in a Mon-Fri week: {date:?}"
        );
        assert!(
            calendar
                .is_closed_trade_date(day(date.0, date.1, date.2), SessionKind::Both)
                .expect("the coverage contract must answer a covered date"),
            "{date:?} is shut by the normal week"
        );
    }
}

/// The half days keep their session and end at the printed 13:00. The probes
/// sit at second granularity, so a row that takes any other instant — or
/// copies a neighbouring holiday's — fails.
#[test]
fn each_half_day_closes_at_the_printed_1300_istanbul() {
    let calendar = calendar_for(Exchange::BorsaIstanbul);
    // Half days before the 2015-11-30 midday-call change: the normal day's
    // morning session runs to 12:30 and the afternoon started at 14:00, so the
    // half day is the morning plus the 12:30-13:00 tail and the 13:00 clip
    // deletes the afternoon entirely.
    for date in [
        (2012, 10, 24),
        (2013, 8, 7),
        (2013, 10, 14),
        (2013, 10, 28),
        (2014, 10, 3),
        (2014, 10, 28),
        (2015, 7, 16),
        (2015, 9, 23),
        (2015, 10, 28),
    ] {
        assert_eq!(
            calendar
                .holiday_on(day(date.0, date.1, date.2))
                .map(Holiday::kind),
            Some(HolidayKind::EarlyClose {
                close_ssm: 13 * 3_600
            }),
            "{date:?} carries the printed half-day close"
        );
        assert!(
            calendar
                .is_open(istanbul(date, (12, 29, 59)))
                .expect("a covered date answers"),
            "{date:?} trades to the end of the morning session"
        );
        assert!(
            !calendar
                .is_open(istanbul(date, (12, 45, 0)))
                .expect("a covered date answers"),
            "{date:?} has no lunch-hour session"
        );
        assert!(
            !calendar
                .is_open(istanbul(date, (13, 0, 0)))
                .expect("a covered date answers"),
            "{date:?} closes at 13:00, end-exclusive"
        );
        assert!(
            !calendar
                .is_open(istanbul(date, (14, 0, 0)))
                .expect("a covered date answers"),
            "{date:?} afternoon is gone"
        );
        // The candle ends at the morning session's own 12:30 end: the printed
        // 13:00 is the day's formal close bound, and the executable morning
        // ends half an hour before it.
        assert_eq!(
            calendar
                .candle_end(istanbul(date, (11, 0, 0)), CalendarResolution::Daily)
                .expect("a covered date answers"),
            Some(istanbul(date, (12, 30, 0))),
            "{date:?} candle edge is the morning session's end"
        );
    }
    // Half days from 2016 on: the midday call session spans 13:00, so the day
    // trades to one second before the printed close.
    for date in [
        (2016, 7, 4),
        (2016, 10, 28),
        (2017, 8, 31),
        (2018, 6, 14),
        (2018, 8, 20),
        (2019, 6, 3),
        (2019, 10, 28),
        (2020, 7, 30),
        (2020, 10, 28),
        (2021, 5, 12),
        (2021, 7, 19),
        (2021, 10, 28),
        (2022, 7, 8),
        (2022, 10, 28),
        (2023, 4, 20),
        (2023, 6, 27),
        (2024, 4, 9),
        (2024, 10, 28),
        (2025, 6, 5),
        (2025, 10, 28),
        (2026, 3, 19),
        (2026, 5, 26),
        (2026, 10, 28),
    ] {
        assert_eq!(
            calendar
                .holiday_on(day(date.0, date.1, date.2))
                .map(Holiday::kind),
            Some(HolidayKind::EarlyClose {
                close_ssm: 13 * 3_600
            }),
            "{date:?} carries the printed half-day close"
        );
        assert!(
            calendar
                .is_open(istanbul(date, (12, 59, 59)))
                .expect("a covered date answers"),
            "{date:?} is open one second before the half-day close"
        );
        assert!(
            !calendar
                .is_open(istanbul(date, (13, 0, 0)))
                .expect("a covered date answers"),
            "{date:?} closes at 13:00, end-exclusive"
        );
        assert_eq!(
            calendar
                .candle_end(istanbul(date, (11, 0, 0)), CalendarResolution::Daily)
                .expect("a covered date answers"),
            Some(istanbul(date, (13, 0, 0))),
            "{date:?} candle edge is the printed close"
        );
    }
}

#[test]
fn borsa_istanbul_window_ordinary_weekday_and_coverage_endpoints() {
    let calendar = calendar_for(Exchange::BorsaIstanbul);
    // An ordinary weekday inside the window: continuous 10:00-18:00, closing
    // envelope to 18:10, closes end-exclusive.
    assert_ordinary_weekday(calendar, (2025, 6, 17), (10, 0), (18, 10), &|date, time| {
        istanbul(date, time)
    });
    // An ordinary weekday inside the backfilled era: after the 2016-11-14
    // extended day the continuous sessions run 10:00-13:00 and 14:00-18:00 and
    // the closing envelope ends at 18:10.
    assert_ordinary_weekday(calendar, (2017, 3, 16), (11, 0), (18, 10), &|date, time| {
        istanbul(date, time)
    });
    let coverage = calendar
        .holiday_coverage()
        .expect("Borsa Istanbul ships a built-in table");
    assert_eq!(coverage.first(), day(2012, 3, 2));
    assert_eq!(coverage.last(), day(2026, 12, 31));
    assert_eq!(
        calendar.holiday_on(coverage.last().succ_opt().expect("representable")),
        None,
        "no answer past the window: 2027 is unpublished"
    );
    // A 2012 probe below the window start (the floor and 2012-03-01 sit in
    // the carried pre-baseline era) refuses.
    let unaudited = istanbul((2012, 2, 20), (11, 0, 0));
    assert!(matches!(
        calendar.is_open(unaudited),
        Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == day(2012, 2, 20)
    ));
    // 2027-01-02 is inside 2027 and outside the audited window.
    let outside = istanbul((2027, 1, 4), (11, 0, 0));
    assert!(matches!(
        calendar.is_open(outside),
        Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == day(2027, 1, 4)
    ));
    // Pre-floor refusal.
    let ancient = istanbul((2009, 12, 31), (11, 0, 0));
    assert!(matches!(
        calendar.is_open(ancient),
        Err(CalendarQueryError::BeforeSupportFloor { .. })
    ));
}

#[test]
fn borsa_istanbul_window_ships_only_closures_and_half_days() {
    let calendar = calendar_for(Exchange::BorsaIstanbul);
    let coverage = calendar
        .holiday_coverage()
        .expect("Borsa Istanbul ships a built-in table");
    let mut closed = 0_usize;
    let mut half_days = 0_usize;
    let mut date = coverage.first();
    while date <= coverage.last() {
        match calendar.holiday_on(date).map(Holiday::kind) {
            None => {}
            Some(HolidayKind::Closed) => closed += 1,
            Some(HolidayKind::EarlyClose { close_ssm }) if close_ssm == 13 * 3_600 => {
                half_days += 1;
            }
            Some(other) => {
                panic!("{date} ships a kind Borsa Istanbul's tables do not state: {other:?}")
            }
        }
        date = date.succ_opt().expect("the window stays representable");
    }
    assert_eq!(
        (closed, half_days),
        (145, 32),
        "closures and 13:00 half days, Borsa Istanbul 2012-2026"
    );
}

/// Detaching the built-in table restores the pure normal week on dates the
/// table covers and refuses nothing.
#[test]
fn every_table_detaches_to_the_normal_week() {
    for (exchange, probe) in [
        (Exchange::B3, &sao_paulo as &At),
        (Exchange::Tadawul, &riyadh as &At),
        (Exchange::BorsaIstanbul, &istanbul as &At),
    ] {
        let calendar = calendar_for(exchange);
        let detached = calendar.without_holidays();
        assert_eq!(detached.holiday_coverage(), None, "{exchange:?}");
        // On B3's Ash Wednesday the detached snapshot answers the ordinary
        // long grid where the built-in table states the late open; the other
        // two venues have an ordinary Wednesday there.
        assert!(
            detached
                .is_open(probe((2025, 3, 5), (11, 0, 0)))
                .expect("a detached snapshot claims no coverage"),
            "{exchange:?}: the detached snapshot answers the normal week"
        );
        assert_eq!(
            detached.holiday_on(day(2025, 3, 5)),
            None,
            "{exchange:?}: the detached snapshot carries no row"
        );
    }
}
mod nzx {
    use super::*;

    fn nzx() -> ExchangeCalendar {
        calendar_for_exchange(Exchange::Nzx)
    }

    fn akl(date: (i32, u32, u32), time: (u32, u32, u32)) -> chrono::DateTime<Utc> {
        Pacific::Auckland
            .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
            .single()
            .expect("fixture must be an unambiguous Auckland instant")
            .with_timezone(&Utc)
    }

    #[test]
    fn a_representative_closure_answers_in_each_published_year() {
        let calendar = nzx();
        for date in [(2025, 2, 6), (2026, 4, 27), (2027, 1, 4)] {
            assert_closure(calendar, date, "nzx");
        }
        // 2020-04-27 sits above the 2020-04-06 ledger horizon, so the closure
        // answers through the identity-backed surface.
        assert_closure(calendar, (2020, 4, 27), "nzx");
        // Below the horizon the holiday rows answer while the session queries
        // refuse as carried: 2011-04-25 is the sheet's own Easter Monday and
        // ANZAC Day on one date.
        assert_eq!(
            calendar.holiday_on(day(2011, 4, 25)).map(Holiday::kind),
            Some(HolidayKind::Closed)
        );
        assert!(matches!(
            calendar.is_closed_trade_date(day(2011, 4, 25), SessionKind::Both),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ));
        assert!(matches!(
            calendar.is_open(akl((2011, 4, 25), (11, 0, 0))),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ));
        // ANZAC Day 2026: the sheet mondayises the Saturday to its own printed
        // Monday. The Saturday itself is closed by the normal week and ships
        // no row.
        assert_eq!(
            calendar.holiday_on(day(2026, 4, 25)).map(Holiday::kind),
            None
        );
        assert!(
            calendar
                .is_closed_trade_date(day(2026, 4, 25), SessionKind::Both)
                .expect("the coverage contract must answer a covered date")
        );
        assert!(
            !calendar
                .is_open(akl((2026, 4, 27), (11, 0, 0)))
                .expect("covered")
        );
    }

    #[test]
    fn an_abbreviated_day_restates_the_operators_own_grid() {
        let calendar = nzx();
        let d = day(2025, 12, 24);
        let Some(HolidayKind::ReplacementBlocks(blocks)) =
            calendar.holiday_on(d).map(Holiday::kind)
        else {
            panic!("2025-12-24 must ship a replacement block set");
        };
        assert_eq!(
            blocks
                .iter()
                .map(|block| (
                    block.kind(),
                    block.open_day_offset(),
                    block.open_ssm(),
                    block.close_ssm()
                ))
                .collect::<Vec<_>>(),
            vec![
                (
                    exchange_hours::ExceptionBlockKind::Extended,
                    0,
                    30_600,
                    36_000
                ),
                (
                    exchange_hours::ExceptionBlockKind::Regular,
                    0,
                    36_000,
                    45_900
                ),
                (
                    exchange_hours::ExceptionBlockKind::OrderEntry,
                    0,
                    45_900,
                    46_770
                ),
                (
                    exchange_hours::ExceptionBlockKind::Extended,
                    0,
                    46_770,
                    46_830
                ),
            ],
            "the blocks are the operator's abbreviated column, phase for phase"
        );
        // Pre-open prints (reported off-market trades) and the shortened
        // regular session answer open ...
        assert!(
            calendar
                .is_open(akl((2025, 12, 24), (9, 0, 0)))
                .expect("covered")
        );
        assert!(
            calendar
                .is_open(akl((2025, 12, 24), (11, 0, 0)))
                .expect("covered")
        );
        // ... the Pre-Close queue stays out of `is_open` ...
        assert!(
            !calendar
                .is_open(akl((2025, 12, 24), (12, 50, 0)))
                .expect("covered")
        );
        // ... the closing-uncross envelope is tradeable to its 13:00:30 end,
        // end-exclusive ...
        assert!(
            calendar
                .is_open(akl((2025, 12, 24), (12, 59, 45)))
                .expect("covered")
        );
        assert!(
            !calendar
                .is_open(akl((2025, 12, 24), (13, 0, 30)))
                .expect("covered")
        );
        // ... and nothing answers after the day's own close, not even the
        // ordinary 17:00 envelope.
        assert!(
            !calendar
                .is_open(akl((2025, 12, 24), (16, 0, 0)))
                .expect("covered")
        );
        assert!(
            !calendar
                .is_open(akl((2025, 12, 24), (17, 0, 0)))
                .expect("covered")
        );
        // The trade date still resolves to the abbreviated day itself.
        assert_eq!(
            calendar
                .trade_date(akl((2025, 12, 24), (11, 0, 0)))
                .expect("covered"),
            Some(d)
        );
    }

    /// The three abbreviated-day eras restate their own operator grid, phase
    /// for phase: the 2010-2012 grid (9:00 pre-open, 15:45 normal close), the
    /// 2013-2020 grid (9:00 pre-open, 12:45 normal close) and the 2021+ grid
    /// (8:30 pre-open, 12:45 normal close).
    #[test]
    fn the_pre_2025_abbreviated_days_restate_their_era_grid() {
        let calendar = nzx();
        let blocks_of = |date: (i32, u32, u32)| match calendar
            .holiday_on(day(date.0, date.1, date.2))
            .map(Holiday::kind)
        {
            Some(HolidayKind::ReplacementBlocks(blocks)) => blocks
                .iter()
                .map(|block| (block.kind(), block.open_ssm(), block.close_ssm()))
                .collect::<Vec<_>>(),
            other => panic!("{date:?} must ship a replacement block set, got {other:?}"),
        };
        let grid = |pre_open: u32, regular_end: u32| {
            vec![
                (
                    exchange_hours::ExceptionBlockKind::Extended,
                    pre_open,
                    36_000,
                ),
                (
                    exchange_hours::ExceptionBlockKind::Regular,
                    36_000,
                    regular_end,
                ),
                (
                    exchange_hours::ExceptionBlockKind::OrderEntry,
                    regular_end,
                    regular_end + 870,
                ),
                (
                    exchange_hours::ExceptionBlockKind::Extended,
                    regular_end + 870,
                    regular_end + 930,
                ),
            ]
        };
        // 2010-2012: Normal Trading 10:00-15:45, Pre-Close 15:45-16:00.
        for date in [(2010, 4, 1), (2011, 12, 23), (2012, 12, 31)] {
            assert_eq!(blocks_of(date), grid(32_400, 56_700), "{date:?}");
        }
        // 2013-2020: Normal Trading 10:00-12:45, Pre-Close 12:45-13:00, 9:00
        // Pre-open — including the two 2020 days that hold the 9:00 Pre-open
        // at its narrowest sourced value.
        for date in [(2013, 12, 24), (2018, 12, 31), (2020, 12, 31)] {
            assert_eq!(blocks_of(date), grid(32_400, 45_900), "{date:?}");
        }
        // 2021 onward: the sourced 8:30 Pre-open.
        for date in [(2021, 12, 24), (2024, 12, 31)] {
            assert_eq!(blocks_of(date), grid(30_600, 45_900), "{date:?}");
        }
    }

    /// 2020-12-24 sits above the ledger horizon, so the replacement day's own
    /// blocks answer session queries: the disputed 8:30-9:00 hour stays out,
    /// the 9:00 Pre-open prints, the Pre-Close queue stays out of `is_open`
    /// and the closing uncross envelope trades to its 13:00:30 end.
    #[test]
    fn the_2020_abbreviated_day_answers_with_the_900_pre_open() {
        let calendar = nzx();
        assert!(
            !calendar
                .is_open(akl((2020, 12, 24), (8, 45, 0)))
                .expect("covered"),
            "the disputed 8:30-9:00 Pre-open hour is withheld"
        );
        assert!(
            calendar
                .is_open(akl((2020, 12, 24), (9, 30, 0)))
                .expect("covered"),
            "the 9:00 Pre-open prints"
        );
        assert!(
            calendar
                .is_open(akl((2020, 12, 24), (11, 0, 0)))
                .expect("covered"),
            "the shortened regular session trades"
        );
        assert!(
            !calendar
                .is_open(akl((2020, 12, 24), (12, 50, 0)))
                .expect("covered"),
            "the Pre-Close queue stays out of is_open"
        );
        assert!(
            calendar
                .is_open(akl((2020, 12, 24), (12, 59, 45)))
                .expect("covered"),
            "the closing uncross envelope trades"
        );
        assert!(
            !calendar
                .is_open(akl((2020, 12, 24), (13, 0, 30)))
                .expect("covered"),
            "closes are end-exclusive at the abbreviated envelope end"
        );
        assert!(
            !calendar
                .is_open(akl((2020, 12, 24), (17, 0, 0)))
                .expect("covered"),
            "nothing answers after the day's own close"
        );
    }

    #[test]
    fn an_ordinary_weekday_answers_and_closes_end_exclusively() {
        let calendar = nzx();
        // Tuesday 2025-07-15: inside the window, no row, normal envelope.
        assert_eq!(calendar.holiday_on(day(2025, 7, 15)), None);
        assert!(
            calendar
                .is_open(akl((2025, 7, 15), (11, 0, 0)))
                .expect("covered")
        );
        assert_eq!(
            calendar
                .session_bounds(akl((2025, 7, 15), (11, 0, 0)))
                .expect("covered"),
            Some((
                akl((2025, 7, 15), (10, 0, 0)),
                akl((2025, 7, 15), (16, 45, 0))
            ))
        );
        // The closing-uncross envelope runs to 17:00:30, end-exclusive.
        assert!(
            calendar
                .is_open(akl((2025, 7, 15), (17, 0, 0)))
                .expect("covered")
        );
        assert!(
            calendar
                .is_open(before(Pacific::Auckland, (2025, 7, 15), (17, 0, 30)))
                .expect("covered")
        );
        assert!(
            !calendar
                .is_open(akl((2025, 7, 15), (17, 0, 30)))
                .expect("covered")
        );
    }

    #[test]
    fn the_window_refuses_on_both_sides_of_the_operators_horizon() {
        let calendar = nzx();
        let coverage = calendar.holiday_coverage().expect("nzx ships a table");
        assert_eq!(coverage.first(), day(2010, 1, 1));
        assert_eq!(coverage.last(), day(2027, 1, 4));
        // The 2016-2017 capture gap: no operator artifact prints the span, so
        // it sits between two audited windows and the identity refuses it.
        let gap = akl((2016, 6, 8), (11, 0, 0));
        assert!(matches!(
            calendar.is_open(gap),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ));
        assert_eq!(calendar.holiday_on(day(2016, 6, 8)), None);
        // Before the support floor.
        assert!(matches!(
            calendar.is_open(akl((2009, 12, 31), (11, 0, 0))),
            Err(CalendarQueryError::BeforeSupportFloor { .. })
        ));
        // Past the operator's horizon: 2027-01-05 is a Tuesday NZX has not
        // published, so the identity refuses rather than claiming normal.
        assert!(matches!(
            calendar.is_open(akl((2027, 1, 5), (11, 0, 0))),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ));
        // 2027-01-04 itself is the horizon day and is a printed closure.
        assert!(
            !calendar
                .is_open(akl((2027, 1, 4), (11, 0, 0)))
                .expect("covered")
        );
        // Detaching the table restores the pure normal week everywhere.
        assert_eq!(calendar.without_holidays().holiday_coverage(), None);
        assert!(
            calendar
                .without_holidays()
                .is_open(akl((2027, 1, 5), (11, 0, 0)))
                .expect("a detached snapshot claims no coverage")
        );
    }

    #[test]
    fn every_shipped_row_matches_the_sheets_per_year() {
        let rows = rows_per_year(nzx());
        assert_eq!(
            rows.len(),
            203,
            "144 pre-2025 closures + 31 pre-2025 abbreviated days + 24 closures \
             and 4 abbreviated days across 2025-2027"
        );
        // 2010-2024, read off the operator's own pages per year.
        assert_eq!(tally(&rows, 2010), (10, 0, 3), "2010");
        assert_eq!(tally(&rows, 2011), (9, 0, 3), "2011");
        assert_eq!(tally(&rows, 2012), (10, 0, 3), "2012");
        assert_eq!(tally(&rows, 2013), (10, 0, 2), "2013");
        assert_eq!(tally(&rows, 2014), (10, 0, 2), "2014");
        assert_eq!(tally(&rows, 2015), (11, 0, 2), "2015");
        assert_eq!(
            tally(&rows, 2016),
            (7, 0, 0),
            "2016: only the January-April sheet dates the capture gap leaves"
        );
        assert_eq!(
            tally(&rows, 2017),
            (3, 0, 2),
            "2017: only the October-December sheet dates the capture gap leaves"
        );
        assert_eq!(tally(&rows, 2018), (10, 0, 2), "2018");
        assert_eq!(tally(&rows, 2019), (10, 0, 2), "2019");
        assert_eq!(tally(&rows, 2020), (10, 0, 2), "2020");
        assert_eq!(tally(&rows, 2021), (10, 0, 2), "2021");
        assert_eq!(
            tally(&rows, 2022),
            (12, 0, 2),
            "2022: the Queen Elizabeth II Memorial Day closure included"
        );
        assert_eq!(tally(&rows, 2023), (11, 0, 2), "2023");
        assert_eq!(tally(&rows, 2024), (11, 0, 2), "2024");
        assert_eq!(
            tally(&rows, 2025),
            (11, 0, 2),
            "2025: eleven closures, two abbreviated days"
        );
        assert_eq!(
            tally(&rows, 2026),
            (11, 0, 2),
            "2026: eleven closures, two abbreviated days"
        );
        assert_eq!(
            tally(&rows, 2027),
            (2, 0, 0),
            "2027: the two New Year closures only"
        );
        // The abbreviated days carry no scalar instant and sit only on the
        // sheet's own dates: flipping one to an early close, or moving a
        // closure to a neighbour date, breaks the tallies above or the shapes
        // here.
        for (date, kind, instant) in &rows {
            if kind == "replacement" {
                assert!(
                    matches!(
                        date,
                        (2010, 4, 1)
                            | (2011, 4, 21)
                            | (2012, 4, 5)
                            // December abbreviated days, the sheets' own
                            // dates: 24|31 December except where the sheets
                            // printed 22/23/29/30.
                            | (2010 | 2012 | 2013 | 2014 | 2015 | 2018 | 2019
                                | 2020 | 2021 | 2024 | 2025 | 2026, 12, 24 | 31)
                            | (2011 | 2022, 12, 23 | 30)
                            | (2017 | 2023, 12, 22 | 29)
                    ),
                    "the abbreviated days are exactly the sheets' own: {date:?}"
                );
            }
            assert_eq!(
                *instant, None,
                "no nzx row states a scalar instant: {date:?}"
            );
        }
    }
}

mod asx {
    use super::*;

    fn asx() -> ExchangeCalendar {
        calendar_for_exchange(Exchange::Asx)
    }

    fn syd(date: (i32, u32, u32), time: (u32, u32, u32)) -> chrono::DateTime<Utc> {
        Australia::Sydney
            .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
            .single()
            .expect("fixture must be an unambiguous Sydney instant")
            .with_timezone(&Utc)
    }

    #[test]
    fn a_representative_closure_answers_in_each_published_year() {
        let calendar = asx();
        for date in [(2025, 12, 25), (2026, 4, 6), (2027, 12, 27)] {
            assert_closure(calendar, date, "asx");
        }
        // Australia Day 2025 sits below the ledger horizon: the SR15-era
        // normal week is sourced only from 2025-06-23, so LAW-COVERAGE has
        // the identity refuse the date even though the holiday row ships.
        // `holiday_on` still reports the printed row.
        assert_eq!(
            calendar.holiday_on(day(2025, 1, 27)).map(Holiday::kind),
            Some(HolidayKind::Closed)
        );
        assert!(matches!(
            calendar.is_closed_trade_date(day(2025, 1, 27), SessionKind::Both),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ));
        // The 2027 Christmas row is the sheet's own substitute date.
        assert!(
            !calendar
                .is_open(syd((2027, 12, 27), (11, 0, 0)))
                .expect("covered"),
            "the substitute Monday is closed"
        );
        // Pre-2025 closure rows answer through `holiday_on` even below the
        // carried horizon: 2010-04-26 is the sheet's own observed ANZAC
        // Monday and 2013-01-28 its observed Australia Day.
        for date in [(2010, 4, 26), (2013, 1, 28)] {
            assert_eq!(
                calendar
                    .holiday_on(day(date.0, date.1, date.2))
                    .map(Holiday::kind),
                Some(HolidayKind::Closed),
                "{date:?}"
            );
        }
        // ANZAC Day 2027 prints `OPEN` on the sheet: audited normal, no row.
        assert_eq!(
            calendar.holiday_on(day(2027, 4, 26)).map(Holiday::kind),
            None
        );
        assert!(
            calendar
                .is_open(syd((2027, 4, 26), (11, 0, 0)))
                .expect("covered")
        );
        // ANZAC Day 2026 prints `CLOSED` against the Saturday; the row
        // restates it and the answer matches the normal week.
        assert_eq!(
            calendar.holiday_on(day(2026, 4, 25)).map(Holiday::kind),
            Some(HolidayKind::Closed)
        );
    }

    #[test]
    fn the_close_early_days_clip_at_the_sheets_own_1410() {
        let calendar = asx();
        let cutoff = syd((2025, 12, 24), (14, 10, 0));
        assert_eq!(
            calendar.holiday_on(day(2025, 12, 24)).map(Holiday::kind),
            Some(HolidayKind::EarlyClose {
                close_ssm: 14 * 3_600 + 10 * 60
            })
        );
        // Open to the last instant before the printed cessation ...
        assert!(
            calendar
                .is_open(before(Australia::Sydney, (2025, 12, 24), (14, 10, 0)))
                .expect("covered")
        );
        // ... and closed at it: closes are end-exclusive.
        assert!(!calendar.is_open(cutoff).expect("covered"));
        assert_eq!(
            calendar
                .session_bounds(syd((2025, 12, 24), (13, 0, 0)))
                .expect("covered"),
            Some((syd((2025, 12, 24), (10, 0, 0)), cutoff))
        );
        assert_eq!(
            calendar
                .candle_end(
                    syd((2025, 12, 24), (13, 0, 0)),
                    exchange_hours::CalendarResolution::Daily
                )
                .expect("covered"),
            Some(cutoff)
        );
        // The afternoon blocks start after the clip and are gone with it.
        assert!(
            !calendar
                .is_open(syd((2025, 12, 24), (16, 15, 0)))
                .expect("covered")
        );
        assert_eq!(
            calendar
                .trade_date(syd((2025, 12, 24), (13, 0, 0)))
                .expect("covered"),
            Some(day(2025, 12, 24)),
            "the shortened day keeps its trade date"
        );
    }

    #[test]
    fn an_ordinary_weekday_answers_and_closes_end_exclusively() {
        let calendar = asx();
        assert_eq!(calendar.holiday_on(day(2025, 7, 15)), None);
        assert!(
            calendar
                .is_open(syd((2025, 7, 15), (11, 0, 0)))
                .expect("covered")
        );
        assert!(
            calendar
                .is_open(syd((2025, 7, 15), (12, 30, 0)))
                .expect("covered")
        );
        // The envelope's last tradeable instant is the 16:21:30 Post Close
        // end; closes are end-exclusive.
        assert!(
            calendar
                .is_open(syd((2025, 7, 15), (16, 15, 0)))
                .expect("covered")
        );
        assert!(
            calendar
                .is_open(before(Australia::Sydney, (2025, 7, 15), (16, 21, 30)))
                .expect("covered")
        );
        assert!(
            !calendar
                .is_open(syd((2025, 7, 15), (16, 21, 30)))
                .expect("covered")
        );
    }

    #[test]
    fn the_window_refuses_before_the_floor_and_the_sheet_has_no_forward_gap() {
        let calendar = asx();
        let coverage = calendar.holiday_coverage().expect("asx ships a table");
        assert_eq!(coverage.first(), day(2010, 1, 1));
        assert_eq!(coverage.last(), day(2027, 12, 31));
        assert!(matches!(
            calendar.is_open(syd((2009, 12, 31), (11, 0, 0))),
            Err(CalendarQueryError::BeforeSupportFloor { .. })
        ));
        // ASX publishes the full 2027 sheet, so the last trading day of the
        // window — a half day, `CLOSE EARLY` — answers rather than refuses.
        assert!(
            calendar
                .is_open(syd((2027, 12, 31), (11, 0, 0)))
                .expect("the operator publishes 2027")
        );
        assert!(
            !calendar
                .is_open(syd((2027, 12, 31), (14, 10, 0)))
                .expect("covered")
        );
        assert_eq!(calendar.without_holidays().holiday_coverage(), None);
    }

    #[test]
    fn every_shipped_row_matches_the_sheets_per_year() {
        let rows = rows_per_year(asx());
        assert_eq!(
            rows.len(),
            174,
            "121 pre-2025 closures + 24 pre-2025 14:10 early closes, 23 closures \
             and six 14:10 early closes across 2025-2027"
        );
        // 2010-2024, read off each year's own operator sheet.
        assert_eq!(tally(&rows, 2010), (8, 2, 0), "2010");
        assert_eq!(
            tally(&rows, 2011),
            (8, 2, 0),
            "2011: the Easter Tuesday one-off included"
        );
        assert_eq!(tally(&rows, 2012), (8, 2, 0), "2012");
        assert_eq!(tally(&rows, 2013), (8, 2, 0), "2013");
        assert_eq!(tally(&rows, 2014), (8, 2, 0), "2014");
        assert_eq!(
            tally(&rows, 2015),
            (8, 2, 0),
            "2015: the Saturday ANZAC included"
        );
        assert_eq!(tally(&rows, 2016), (8, 2, 0), "2016");
        assert_eq!(
            tally(&rows, 2017),
            (8, 0, 0),
            "2017: the sheet prints no Last Business Day rows, so no early close"
        );
        assert_eq!(tally(&rows, 2018), (8, 2, 0), "2018");
        assert_eq!(tally(&rows, 2019), (8, 2, 0), "2019");
        assert_eq!(
            tally(&rows, 2020),
            (8, 2, 0),
            "2020: the Saturday ANZAC (no substitute) included"
        );
        assert_eq!(
            tally(&rows, 2021),
            (8, 2, 0),
            "2021: the Sunday ANZAC included"
        );
        assert_eq!(
            tally(&rows, 2022),
            (9, 0, 0),
            "2022: the National Day of Mourning included; both year-end rows print OPEN"
        );
        assert_eq!(
            tally(&rows, 2023),
            (8, 0, 0),
            "2023: both year-end rows print OPEN, so no early close"
        );
        assert_eq!(tally(&rows, 2024), (8, 2, 0), "2024");
        assert_eq!(
            tally(&rows, 2025),
            (8, 2, 0),
            "2025: eight closures, two half days"
        );
        assert_eq!(
            tally(&rows, 2026),
            (8, 2, 0),
            "2026: eight closures (the Saturday ANZAC included), two half days"
        );
        assert_eq!(
            tally(&rows, 2027),
            (7, 2, 0),
            "2027: seven closures (ANZAC prints OPEN), two half days"
        );
        for (date, _, instant) in &rows {
            if instant.is_some() {
                assert_eq!(*instant, Some(14 * 3_600 + 10 * 60), "{date:?}");
            }
        }
        // The scalar early closes sit only on the sheets' own dates: adding
        // one to 2017/2022/2023 (whose sheets print none) or moving a closure
        // breaks the tallies above.
        for (date, _, instant) in &rows {
            if instant.is_some() {
                assert!(
                    matches!(
                        date,
                        (
                            2010 | 2012
                                | 2013
                                | 2014
                                | 2015
                                | 2018
                                | 2019
                                | 2020
                                | 2021
                                | 2024
                                | 2025
                                | 2026
                                | 2027,
                            12,
                            24 | 31
                        ) | (2011 | 2016, 12, 23 | 30)
                    ),
                    "the early closes are exactly the sheets' own: {date:?}"
                );
            }
        }
    }

    /// Below the 2025-06-23 SR15 horizon the session queries refuse as
    /// carried, but the holiday rows answer: 2011-04-26 is the sheet's own
    /// one-off Easter Tuesday congruence holiday, and 2022-09-22 is the
    /// National Day of Mourning the sheet gained between replays.
    #[test]
    fn the_pre_2025_rows_answer_while_carried_dates_refuse() {
        let calendar = asx();
        for date in [(2011, 4, 26), (2022, 9, 22), (2010, 12, 24), (2023, 12, 25)] {
            assert!(
                calendar.holiday_on(day(date.0, date.1, date.2)).is_some(),
                "{date:?} ships a row"
            );
            assert!(matches!(
                calendar.is_open(syd(date, (11, 0, 0))),
                Err(CalendarQueryError::OutsideCoveredRange { .. })
            ));
        }
    }
}

mod sgx_securities {
    use super::*;

    fn sgx() -> ExchangeCalendar {
        calendar_for_exchange(Exchange::SgxSecurities)
    }

    fn sgt(date: (i32, u32, u32), time: (u32, u32, u32)) -> chrono::DateTime<Utc> {
        Asia::Singapore
            .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
            .single()
            .expect("fixture must be an unambiguous Singapore instant")
            .with_timezone(&Utc)
    }

    #[test]
    fn a_representative_closure_answers_in_each_published_year() {
        let calendar = sgx();
        for date in [(2025, 1, 29), (2026, 6, 1)] {
            assert_closure(calendar, date, "sgx_securities");
        }
        // The 2026 Vesak closure is MOM's own Sunday substitution moved to
        // its Monday; the Sunday itself is closed by the normal week and
        // ships no row.
        assert_eq!(
            calendar.holiday_on(day(2026, 5, 31)).map(Holiday::kind),
            None
        );
        assert!(
            !calendar
                .is_open(sgt((2026, 6, 1), (10, 0, 0)))
                .expect("covered")
        );
        // Hari Raya Puasa 2026 falls on a Saturday: MOM prints no
        // substitution, so no row and no weekday answer changes.
        assert_eq!(
            calendar.holiday_on(day(2026, 3, 21)).map(Holiday::kind),
            None
        );
    }

    #[test]
    fn the_half_days_restate_the_operators_printed_grid() {
        let calendar = sgx();
        let d = day(2026, 2, 16);
        let Some(HolidayKind::ReplacementBlocks(blocks)) =
            calendar.holiday_on(d).map(Holiday::kind)
        else {
            panic!("2026-02-16 must ship a replacement block set");
        };
        assert_eq!(
            blocks
                .iter()
                .map(|block| (
                    block.kind(),
                    block.open_day_offset(),
                    block.open_ssm(),
                    block.close_ssm()
                ))
                .collect::<Vec<_>>(),
            vec![
                (
                    exchange_hours::ExceptionBlockKind::OrderEntry,
                    0,
                    30_600,
                    32_280
                ),
                (
                    exchange_hours::ExceptionBlockKind::Extended,
                    0,
                    32_280,
                    32_400
                ),
                (
                    exchange_hours::ExceptionBlockKind::Regular,
                    0,
                    32_400,
                    43_200
                ),
                (
                    exchange_hours::ExceptionBlockKind::OrderEntry,
                    0,
                    43_200,
                    43_440
                ),
                (
                    exchange_hours::ExceptionBlockKind::Extended,
                    0,
                    43_440,
                    44_160
                ),
            ],
            "the blocks are the operator's half-day column, phase for phase"
        );
        // The morning trades ...
        assert!(
            calendar
                .is_open(sgt((2026, 2, 16), (9, 30, 0)))
                .expect("covered")
        );
        assert!(
            calendar
                .is_open(sgt((2026, 2, 16), (11, 0, 0)))
                .expect("covered")
        );
        // ... the Pre-Close queue stays out of `is_open` ...
        assert!(
            !calendar
                .is_open(sgt((2026, 2, 16), (12, 2, 0)))
                .expect("covered")
        );
        // ... the closing Non-Cancel and Trade at Close print to the printed
        // 12:16 close, end-exclusive ...
        assert!(
            calendar
                .is_open(sgt((2026, 2, 16), (12, 5, 0)))
                .expect("covered")
        );
        assert!(
            calendar
                .is_open(sgt((2026, 2, 16), (12, 10, 0)))
                .expect("covered")
        );
        assert!(
            calendar
                .is_open(before(Asia::Singapore, (2026, 2, 16), (12, 16, 0)))
                .expect("covered")
        );
        assert!(
            !calendar
                .is_open(sgt((2026, 2, 16), (12, 16, 0)))
                .expect("covered")
        );
        // ... the trade date still resolves to the half day itself ...
        assert_eq!(
            calendar
                .trade_date(sgt((2026, 2, 16), (11, 0, 0)))
                .expect("covered"),
            Some(d)
        );
        // ... and the afternoon is gone.
        assert!(
            !calendar
                .is_open(sgt((2026, 2, 16), (14, 0, 0)))
                .expect("covered")
        );
        assert!(
            !calendar
                .is_open(sgt((2026, 2, 16), (17, 10, 0)))
                .expect("covered")
        );
    }

    #[test]
    fn an_ordinary_weekday_answers_with_its_lunch_gap_and_end_exclusive_close() {
        let calendar = sgx();
        assert_eq!(calendar.holiday_on(day(2025, 7, 15)), None);
        assert!(
            calendar
                .is_open(sgt((2025, 7, 15), (10, 0, 0)))
                .expect("covered")
        );
        assert!(
            !calendar
                .is_open(sgt((2025, 7, 15), (12, 30, 0)))
                .expect("covered")
        );
        assert!(
            calendar
                .is_open(sgt((2025, 7, 15), (14, 0, 0)))
                .expect("covered")
        );
        // The envelope's last tradeable instant is the 17:16 Trade-at-Close
        // end; closes are end-exclusive.
        assert!(
            calendar
                .is_open(sgt((2025, 7, 15), (17, 10, 0)))
                .expect("covered")
        );
        assert!(
            calendar
                .is_open(before(Asia::Singapore, (2025, 7, 15), (17, 16, 0)))
                .expect("covered")
        );
        assert!(
            !calendar
                .is_open(sgt((2025, 7, 15), (17, 16, 0)))
                .expect("covered")
        );
    }

    #[test]
    fn the_window_refuses_past_the_operators_published_horizon() {
        let calendar = sgx();
        let coverage = calendar.holiday_coverage().expect("sgx ships a table");
        assert_eq!(coverage.first(), day(2014, 1, 1));
        assert_eq!(coverage.last(), day(2026, 12, 31));
        assert!(matches!(
            calendar.is_open(sgt((2009, 12, 31), (10, 0, 0))),
            Err(CalendarQueryError::BeforeSupportFloor { .. })
        ));
        // The 2020-2024 capture gap sits between two audited windows and the
        // identity refuses it: no operator artifact prints those closures.
        let gap = sgt((2022, 6, 8), (10, 0, 0));
        assert!(matches!(
            calendar.is_open(gap),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ));
        assert_eq!(calendar.holiday_on(day(2022, 6, 8)), None);
        // The one 2020 date the 2019 sheet prints answers as a row, and the
        // session queries answer with it: the market is closed all day.
        assert_eq!(
            calendar.holiday_on(day(2020, 1, 1)).map(Holiday::kind),
            Some(HolidayKind::Closed)
        );
        assert!(
            !calendar
                .is_open(sgt((2020, 1, 1), (10, 0, 0)))
                .expect("the date is inside an audited window")
        );
        // 2027-01-01 is an SGX holiday in fact, but the operator has
        // published no 2027 half-day schedule, so the identity must refuse
        // the date outright rather than answer from a normal week it has not
        // audited. The detached snapshot still answers the pure normal week.
        let outside = sgt((2027, 1, 4), (10, 0, 0));
        assert!(matches!(
            calendar.is_open(outside),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ));
        assert!(
            calendar
                .without_holidays()
                .is_open(outside)
                .expect("a detached snapshot claims no coverage")
        );
        assert_eq!(calendar.without_holidays().holiday_coverage(), None);
    }

    #[test]
    fn every_shipped_row_matches_the_sheets_per_year() {
        let rows = rows_per_year(sgx());
        assert_eq!(
            rows.len(),
            95,
            "63 pre-2025 closures + 7 pre-2025 half-day grids + 19 closures and \
             six printed half-day grids across 2025-2026"
        );
        // 2014-2020, read off the operator's own sheets per year.
        assert_eq!(tally(&rows, 2014), (9, 0, 0), "2014");
        assert_eq!(
            tally(&rows, 2015),
            (13, 0, 0),
            "2015: SG50 and Polling Day included"
        );
        assert_eq!(
            tally(&rows, 2016),
            (9, 0, 0),
            "2016: three Sunday substitutions keyed by the operator's own *"
        );
        assert_eq!(
            tally(&rows, 2017),
            (10, 0, 1),
            "2017: the CNY-eve half day the sheet's # names"
        );
        assert_eq!(tally(&rows, 2018), (10, 0, 3), "2018");
        assert_eq!(tally(&rows, 2019), (11, 0, 3), "2019");
        assert_eq!(
            tally(&rows, 2020),
            (1, 0, 0),
            "2020: only the one date the 2019 sheet's 2020 table prints"
        );
        assert_eq!(
            tally(&rows, 2025),
            (9, 0, 3),
            "2025: nine closures, three half days"
        );
        assert_eq!(
            tally(&rows, 2026),
            (10, 0, 3),
            "2026: ten closures (three Sunday substitutions), three half days"
        );
        // The half days are exactly the operator's printed dates: the seven
        // pre-2025 `#` markers and the six 2025-2026 sheet dates, and no row
        // anywhere in the window states a scalar instant.
        for (date, kind, instant) in &rows {
            assert_eq!(*instant, None, "{date:?}");
            if kind == "replacement" {
                // The 24|31 December arms mix the two eras (2018 and 2019 are
                // the operator's pre-2025 `#` markers; 2025 and 2026 the
                // current sheet's printed dates); 2018-02-15, 2019-02-04,
                // 2017-01-27, 2025-01-28 and 2026-02-16 are the remaining
                // Chinese New Year eves.
                assert!(
                    matches!(
                        date,
                        (2017, 1, 27)
                            | (2018, 2, 15)
                            | (2018 | 2019 | 2025 | 2026, 12, 24 | 31)
                            | (2019, 2, 4)
                            | (2025, 1, 28)
                            | (2026, 2, 16)
                    ),
                    "the half days are exactly the sheets' own: {date:?}"
                );
            }
        }
    }

    /// The 2017-2019 half days restate the grid the operator's own page
    /// prints — Trading 09:00-12:30, Pre-Close 12:30-12:34, Non-Cancel to
    /// the printed 12:36 close — not the current 12:00/12:16 grid.
    #[test]
    fn the_pre_2025_half_days_restate_the_printed_1236_grid() {
        let calendar = sgx();
        for date in [(2017, 1, 27), (2018, 12, 24), (2019, 12, 31)] {
            let kind = calendar
                .holiday_on(day(date.0, date.1, date.2))
                .map(Holiday::kind);
            let Some(HolidayKind::ReplacementBlocks(blocks)) = kind else {
                panic!("{date:?} must ship a replacement block set, got {kind:?}");
            };
            let shapes = blocks
                .iter()
                .map(|block| (block.kind(), block.open_ssm(), block.close_ssm()))
                .collect::<Vec<_>>();
            assert_eq!(
                shapes,
                vec![
                    (
                        exchange_hours::ExceptionBlockKind::OrderEntry,
                        30_600,
                        32_280
                    ),
                    (exchange_hours::ExceptionBlockKind::Extended, 32_280, 32_400),
                    (exchange_hours::ExceptionBlockKind::Regular, 32_400, 45_000),
                    (
                        exchange_hours::ExceptionBlockKind::OrderEntry,
                        45_000,
                        45_240
                    ),
                    (exchange_hours::ExceptionBlockKind::Extended, 45_240, 45_360),
                ],
                "{date:?} must restate the printed 09:00-12:30 / 12:36 grid"
            );
        }
    }

    /// The pre-2025 rows answer through the identity-backed surface: the
    /// normal week is sourced from 2011-08-01, so 2014-2019 dates are above
    /// the horizon and the holiday rows close them outright. 2015-08-07 is
    /// the operator's own SG50 one-off, 2019-10-28 its own Deepavali
    /// substitution Monday and 2017-01-27 its printed half day.
    #[test]
    fn the_pre_2025_rows_answer_the_session_queries() {
        let calendar = sgx();
        for date in [(2015, 8, 7), (2019, 10, 28), (2014, 10, 6)] {
            assert_eq!(
                calendar
                    .holiday_on(day(date.0, date.1, date.2))
                    .map(Holiday::kind),
                Some(HolidayKind::Closed),
                "{date:?}"
            );
            assert!(
                !calendar
                    .is_open(sgt(date, (10, 0, 0)))
                    .expect("the date is inside an audited window"),
                "{date:?} is a sourced closure"
            );
        }
        // The half day trades its printed grid: open at 09:30, order-entry
        // only in the 12:30 Pre-Close, tradeable through the 12:35:45
        // Non-Cancel, closed at the end-exclusive 12:36.
        let half = (2017, 1, 27);
        assert!(
            calendar.is_open(sgt(half, (9, 30, 0))).expect("covered"),
            "the half day's trading session is open"
        );
        assert!(
            !calendar.is_open(sgt(half, (12, 32, 0))).expect("covered"),
            "the Pre-Close queue stays out of is_open"
        );
        assert!(
            calendar.is_open(sgt(half, (12, 35, 45))).expect("covered"),
            "the closing Non-Cancel prints"
        );
        assert!(
            !calendar.is_open(sgt(half, (12, 36, 0))).expect("covered"),
            "the printed 12:36 close is end-exclusive"
        );
    }
}

mod lse {
    use super::*;

    fn lse() -> ExchangeCalendar {
        calendar_for_exchange(Exchange::Lse)
    }

    fn london(date: (i32, u32, u32), time: (u32, u32, u32)) -> chrono::DateTime<Utc> {
        Europe::London
            .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
            .single()
            .expect("fixture must be an unambiguous London instant")
            .with_timezone(&Utc)
    }

    #[test]
    fn closures_per_year_match_the_operators_printed_table() {
        let calendar = lse();
        // Every `NON-trading day.` row of the operator's business-days table,
        // per year: 2025-12-25/26 and the whole 2026 and 2027 sets are printed
        // by the 2025-12-18 capture.
        let closed = [
            (2025, 12, 25),
            (2025, 12, 26),
            (2026, 1, 1),
            (2026, 4, 3),
            (2026, 4, 6),
            (2026, 5, 4),
            (2026, 5, 25),
            (2026, 8, 31),
            (2026, 12, 25),
            (2026, 12, 28),
            (2027, 1, 1),
            (2027, 3, 26),
            (2027, 3, 29),
            (2027, 5, 3),
            (2027, 5, 31),
            (2027, 8, 30),
            (2027, 12, 27),
            (2027, 12, 28),
        ];
        for date in closed {
            assert_closed(calendar, date, "lse", &london);
        }
        // 2025-01-01, printed by the 2024-02-07 capture: with the second
        // audited window reaching back over 2024, its derivation reads the
        // covered 2024-12-31 and answers end to end (the special behind-window
        // refusal the single-window tables show no longer applies).
        assert_eq!(
            calendar.holiday_on(day(2025, 1, 1)).map(Holiday::kind),
            Some(HolidayKind::Closed)
        );
        assert!(
            calendar
                .is_closed_trade_date(day(2025, 1, 1), SessionKind::Both)
                .expect("the derivation reads covered 2024-12-31"),
            "lse 2025-01-01 has no session in either phase"
        );
        assert!(!calendar.is_open(london((2025, 1, 1), (11, 0, 0))).expect("covered"));
    }

    #[test]
    fn the_pre_2025_eras_answer_through_the_identity_backed_surface() {
        let calendar = lse();
        // One representative closure per pre-2025 audited year, each the
        // operator's own printed row: the 2010 substitute Christmas pair, the
        // 2011 Royal Wedding, the 2012 Diamond Jubilee, the 2013-2014 spring
        // bank holidays, 2020's substitute Boxing Day, the 2021 Christmas
        // substitute, 2022's Platinum Jubilee, 2023's Coronation and 2024's
        // Good Friday.
        let closed = [
            (2010, 12, 28),
            (2011, 4, 29),
            (2012, 6, 5),
            (2013, 5, 27),
            (2014, 8, 25),
            (2020, 12, 28),
            (2021, 12, 27),
            (2022, 6, 3),
            (2023, 5, 8),
            (2024, 3, 29),
        ];
        for date in closed {
            assert_closure(calendar, date, "lse");
            assert!(
                !calendar.is_open(london(date, (12, 0, 0))).expect("covered"),
                "lse {date:?} is shut midday"
            );
        }
        // The first row of the first window answers end to end: the engine's
        // behind-derivation reads 2009-12-31 and the support floor carries the
        // baseline, so the closure resolves through the full surface.
        assert_eq!(
            calendar.holiday_on(day(2010, 1, 1)).map(Holiday::kind),
            Some(HolidayKind::Closed)
        );
        assert!(
            calendar
                .is_closed_trade_date(day(2010, 1, 1), SessionKind::Both)
                .expect("the floor carries the baseline behind the first row"),
            "lse 2010-01-01 has no session in either phase"
        );
        assert!(!calendar.is_open(london((2010, 1, 1), (12, 0, 0))).expect("covered"));
        // The last row of the first window answers end to end: its derivation
        // reads 2014-12-31, inside the window.
        assert_closure(calendar, (2015, 1, 1), "lse");
        assert!(
            !calendar
                .is_open(london((2015, 1, 1), (12, 0, 0)))
                .expect("covered")
        );
    }

    #[test]
    fn the_pre_2025_half_days_close_at_the_printed_1230_london_time() {
        let calendar = lse();
        // 2012's Christmas Eve (page-level sentence), 2011's printed Friday
        // 23 December and 2022's printed Friday 30 December: every pre-2025
        // half-day shape, each closing at the operator's 12:30 instant,
        // end-exclusive.
        for date in [(2011, 12, 23), (2012, 12, 24), (2022, 12, 30), (2024, 12, 24)] {
            assert_eq!(
                calendar
                    .holiday_on(day(date.0, date.1, date.2))
                    .map(Holiday::kind),
                Some(HolidayKind::EarlyClose { close_ssm: 45_000 }),
                "lse {date:?} carries the printed 12:30 half-day close"
            );
            assert!(
                calendar
                    .is_open(london(date, (12, 29, 59)))
                    .expect("covered"),
                "lse {date:?} still trades at 12:29:59"
            );
            assert!(
                !calendar
                    .is_open(london(date, (12, 30, 0)))
                    .expect("covered"),
                "lse {date:?} is closed at the 12:30 close (end-exclusive)"
            );
            assert!(
                !calendar.is_open(london(date, (14, 0, 0))).expect("covered"),
                "lse {date:?} has no afternoon session"
            );
            assert!(
                calendar.is_open(london(date, (9, 0, 0))).expect("covered"),
                "lse {date:?} trades the morning"
            );
        }
    }

    #[test]
    fn the_two_coverage_gaps_refuse_rather_than_answer() {
        let calendar = lse();
        // 2015-01-02..2019-12-31 and 2020-01-01..2020-08-30 survive in no
        // operator capture, so they sit outside every window: the holiday
        // layer has no answer and the session queries refuse.
        for date in [(2016, 6, 13), (2018, 12, 24), (2020, 3, 30), (2020, 8, 28)] {
            assert_eq!(
                calendar.holiday_on(day(date.0, date.1, date.2)),
                None,
                "lse {date:?} is inside a coverage gap and carries no row"
            );
            assert!(
                calendar
                    .is_open(london(date, (12, 0, 0)))
                    .is_err_and(|error| matches!(
                        error,
                        CalendarQueryError::OutsideCoveredRange { .. }
                    )),
                "lse {date:?}: the query must refuse inside the gap"
            );
        }
        // The second window opens on its own first row.
        assert_eq!(
            calendar.holiday_on(day(2020, 8, 31)).map(Holiday::kind),
            Some(HolidayKind::Closed)
        );
        // ... whose session probes read 2020-08-28, inside the gap, so they
        // refuse; the first fully answering date is the next trading day.
        assert!(matches!(
            calendar.is_open(london((2020, 8, 31), (12, 0, 0))),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ));
        assert!(
            calendar
                .is_closed_trade_date(day(2020, 8, 31), SessionKind::Both)
                .is_err_and(|error| matches!(
                    error,
                    CalendarQueryError::OutsideCoveredRange { .. }
                ))
        );
        assert!(
            calendar
                .is_open(london((2020, 9, 1), (12, 0, 0)))
                .expect("covered"),
            "lse 2020-09-01 answers as the window's first fully derived date"
        );
    }

    #[test]
    fn the_half_days_close_at_the_printed_1230_london_time() {
        let calendar = lse();
        // The sheet states `Markets closing process commences from 12:30
        // London time.` for every Christmas Eve and New Year's Eve in the
        // window; closes are end-exclusive.
        for date in [
            (2025, 12, 24),
            (2025, 12, 31),
            (2026, 12, 24),
            (2026, 12, 31),
            (2027, 12, 24),
            (2027, 12, 31),
        ] {
            assert_eq!(
                calendar
                    .holiday_on(day(date.0, date.1, date.2))
                    .map(Holiday::kind),
                Some(HolidayKind::EarlyClose { close_ssm: 45_000 }),
                "lse {date:?} carries the printed 12:30 half-day close"
            );
            assert!(
                calendar
                    .is_open(london(date, (12, 29, 59)))
                    .expect("covered"),
                "lse {date:?} still trades at 12:29:59"
            );
            assert!(
                !calendar
                    .is_open(london(date, (12, 30, 0)))
                    .expect("covered"),
                "lse {date:?} is closed at the 12:30 close (end-exclusive)"
            );
            // No afternoon session survives the half-day close, and the
            // morning runs normally.
            assert!(
                !calendar.is_open(london(date, (14, 0, 0))).expect("covered"),
                "lse {date:?} has no afternoon session"
            );
            assert!(
                calendar.is_open(london(date, (9, 0, 0))).expect("covered"),
                "lse {date:?} trades the morning"
            );
        }
    }

    #[test]
    fn the_rolling_table_gap_dates_claim_nothing() {
        let calendar = lse();
        // The operator's table is rolling and the Wayback index holds no
        // capture of the 2025-01-02..2025-12-17 gap, so these five dates ship
        // `Unsourced` and every derivation that reads them refuses rather
        // than claims.
        for date in [
            (2025, 4, 18),
            (2025, 4, 21),
            (2025, 5, 5),
            (2025, 5, 26),
            (2025, 8, 25),
        ] {
            assert_eq!(
                calendar
                    .holiday_on(day(date.0, date.1, date.2))
                    .map(Holiday::kind),
                Some(HolidayKind::Unsourced),
                "lse {date:?} must ship an Unsourced row, not a claimed closure"
            );
            assert!(
                calendar
                    .is_open(london(date, (12, 0, 0)))
                    .is_err_and(|error| matches!(error, CalendarQueryError::UnresolvedGap { .. })),
                "lse {date:?}: is_open must refuse rather than claim"
            );
            assert!(
                calendar
                    .is_closed_trade_date(day(date.0, date.1, date.2), SessionKind::Both)
                    .is_err_and(|error| matches!(error, CalendarQueryError::UnresolvedGap { .. })),
                "lse {date:?}: the trade-date closure question must refuse"
            );
        }
    }

    #[test]
    fn an_ordinary_weekday_answers_with_the_end_exclusive_close() {
        let calendar = lse();
        // Wednesday 2026-06-10: inside the window, no row, the normal SETS
        // envelope.
        assert_eq!(calendar.holiday_on(day(2026, 6, 10)), None);
        assert!(
            calendar
                .is_open(london((2026, 6, 10), (12, 0, 0)))
                .expect("covered")
        );
        // The closing-uncross envelope runs to the 16:40 CPX end, end-exclusive.
        assert!(
            calendar
                .is_open(before(Europe::London, (2026, 6, 10), (16, 40, 0)))
                .expect("covered")
        );
        assert!(
            !calendar
                .is_open(london((2026, 6, 10), (16, 40, 0)))
                .expect("covered")
        );
    }

    #[test]
    fn the_window_bounds_and_the_pre_floor_refusal() {
        let calendar = lse();
        let coverage = calendar.holiday_coverage().expect("lse ships a table");
        assert_eq!(coverage.first(), day(2010, 1, 1));
        assert_eq!(coverage.last(), day(2027, 12, 31));
        // Outside the audited windows the table has no answer at all: a gap
        // year and a date past the live table's print.
        assert_eq!(calendar.holiday_on(day(2017, 6, 13)), None);
        assert_eq!(calendar.holiday_on(day(2028, 1, 3)), None);
        // Before the support floor.
        assert!(matches!(
            calendar.is_open(london((2009, 12, 31), (12, 0, 0))),
            Err(CalendarQueryError::BeforeSupportFloor { .. })
        ));
        // Past the audited window: 2028-01-03 is inside the live table's own
        // print but outside this table's audited window, so the identity
        // refuses rather than answering from an unaudited normal week. The
        // detached snapshot still answers the pure normal week.
        let outside = london((2028, 1, 4), (12, 0, 0));
        assert!(matches!(
            calendar.is_open(outside),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ));
        assert!(
            calendar
                .without_holidays()
                .is_open(outside)
                .expect("a detached snapshot claims no coverage")
        );
        assert_eq!(calendar.without_holidays().holiday_coverage(), None);
    }

    #[test]
    fn every_shipped_row_matches_the_sheet_per_year() {
        let rows = rows_per_year(lse());
        assert_eq!(
            rows.len(),
            130,
            "99 closures, 26 half days, five rolling-table gaps — the gap years carry no rows"
        );
        let expected: [(i32, (usize, usize, usize, usize)); 13] = [
            (2010, (8, 2, 0, 0)),
            (2011, (9, 2, 0, 0)),
            (2012, (9, 2, 0, 0)),
            (2013, (8, 2, 0, 0)),
            (2014, (8, 2, 0, 0)),
            (2015, (1, 0, 0, 0)),
            (2020, (3, 2, 0, 0)),
            (2021, (8, 2, 0, 0)),
            (2022, (9, 2, 0, 0)),
            (2023, (9, 2, 0, 0)),
            (2024, (8, 2, 0, 0)),
            (2025, (3, 2, 0, 5)),
            (2027, (8, 2, 0, 0)),
        ];
        for (year, tally) in expected {
            assert_eq!(census(&rows, year), tally, "{year} census");
        }
        for year in [2016, 2017, 2018, 2019] {
            assert_eq!(
                census(&rows, year),
                (0, 0, 0, 0),
                "{year} is a gap year and ships no row"
            );
        }
        // The half days are exactly the sheet's printed 12:30 dates — 24/31
        // December every year except 2011 and 2022 (Friday 23/30), 2023
        // (22/29), and none in the gap years — each stating the printed
        // instant; flipping a row's date, kind or instant breaks the walk.
        for (date, kind, instant) in &rows {
            if kind == "early close" {
                assert_eq!(*instant, Some(45_000), "{date:?} prints 12:30");
                assert!(
                    matches!(date.1, 12) && matches!(date.2, 22 | 23 | 24 | 29 | 30 | 31),
                    "the half days are the sheet's December dates: {date:?}"
                );
            } else {
                assert_eq!(*instant, None, "{date:?} states no scalar instant");
            }
        }
        let half_days: Vec<(i32, u32, u32)> = rows
            .iter()
            .filter(|(_, kind, _)| kind == "early close")
            .map(|(date, _, _)| *date)
            .collect();
        assert_eq!(
            half_days,
            vec![
                (2010, 12, 24),
                (2010, 12, 31),
                (2011, 12, 23),
                (2011, 12, 30),
                (2012, 12, 24),
                (2012, 12, 31),
                (2013, 12, 24),
                (2013, 12, 31),
                (2014, 12, 24),
                (2014, 12, 31),
                (2020, 12, 24),
                (2020, 12, 31),
                (2021, 12, 24),
                (2021, 12, 31),
                (2022, 12, 23),
                (2022, 12, 30),
                (2023, 12, 22),
                (2023, 12, 29),
                (2024, 12, 24),
                (2024, 12, 31),
                (2025, 12, 24),
                (2025, 12, 31),
                (2026, 12, 24),
                (2026, 12, 31),
                (2027, 12, 24),
                (2027, 12, 31),
            ],
            "the sheet's half days, year by year"
        );
    }
}

mod euronext_paris {
    use super::*;

    fn paris() -> ExchangeCalendar {
        calendar_for_exchange(Exchange::EuronextParis)
    }

    fn paris_time(date: (i32, u32, u32), time: (u32, u32, u32)) -> chrono::DateTime<Utc> {
        Europe::Paris
            .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
            .single()
            .expect("fixture must be an unambiguous Paris instant")
            .with_timezone(&Utc)
    }

    #[test]
    fn closures_per_year_match_the_operators_printed_columns() {
        let calendar = paris();
        // Every Paris `Closed` cell: the 2025 column of the 2025-12-06 page
        // capture and the 2026 column of the live page and its INFO-FLASH.
        // 2026-12-25 is fenced separately: its derivation reads 2026-12-24,
        // the announced-but-unstated eve, so the closure question refuses.
        let closed = [
            (2025, 1, 1),
            (2025, 4, 18),
            (2025, 4, 21),
            (2025, 5, 1),
            (2025, 12, 25),
            (2025, 12, 26),
            (2026, 1, 1),
            (2026, 4, 3),
            (2026, 4, 6),
            (2026, 5, 1),
        ];
        for date in closed {
            assert_closed(calendar, date, "euronext_paris", &paris_time);
        }
    }

    #[test]
    fn the_unresolved_eve_shadows_the_following_christmas_derivation() {
        let calendar = paris();
        // The row for 2026-12-25 itself still answers Closed.
        assert_eq!(
            calendar.holiday_on(day(2026, 12, 25)).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "2026-12-25 carries the printed closure row"
        );
        // But the trade-date and intraday questions read behind the query's
        // bounds — 2026-12-24, whose half-day hours are announced and
        // unstated — so both refuse as an unresolved gap rather than claim.
        assert!(
            calendar
                .is_closed_trade_date(day(2026, 12, 25), SessionKind::Both)
                .is_err_and(|error| matches!(error, CalendarQueryError::UnresolvedGap { .. })),
            "the 2026-12-25 closure question must refuse on the unresolved eve"
        );
        assert!(
            calendar
                .is_open(paris_time((2026, 12, 25), (12, 0, 0)))
                .is_err_and(|error| matches!(error, CalendarQueryError::UnresolvedGap { .. })),
            "the 2026-12-25 intraday question must refuse on the unresolved eve"
        );
    }

    #[test]
    fn the_2025_half_days_close_at_the_appendix_printed_1405_cet() {
        let calendar = paris();
        // The 2025 end-of-year appendix prints the Paris equity segments'
        // closing auction at 14:00 and TAL to 14:05 on 24 and 31 December, so
        // the envelope close is 14:05 CET, end-exclusive.
        for date in [(2025, 12, 24), (2025, 12, 31)] {
            assert_eq!(
                calendar
                    .holiday_on(day(date.0, date.1, date.2))
                    .map(Holiday::kind),
                Some(HolidayKind::EarlyClose { close_ssm: 50_700 }),
                "paris {date:?} carries the appendix's 14:05 close"
            );
            assert!(
                calendar
                    .is_open(paris_time(date, (14, 4, 59)))
                    .expect("covered"),
                "paris {date:?} still trades at 14:04:59"
            );
            assert!(
                !calendar
                    .is_open(paris_time(date, (14, 5, 0)))
                    .expect("covered"),
                "paris {date:?} is closed at the 14:05 close (end-exclusive)"
            );
            assert!(
                calendar
                    .is_open(paris_time(date, (9, 30, 0)))
                    .expect("covered"),
                "paris {date:?} trades the morning"
            );
        }
    }

    #[test]
    fn the_2026_half_days_are_announced_but_unstated() {
        let calendar = paris();
        // The 2026 table prints `**Half Trading Day` for both December eves,
        // but the hours live in the end-of-year appendix, which is announced
        // and unpublished — so the rows claim nothing.
        for date in [(2026, 12, 24), (2026, 12, 31)] {
            assert_eq!(
                calendar
                    .holiday_on(day(date.0, date.1, date.2))
                    .map(Holiday::kind),
                Some(HolidayKind::Unsourced),
                "paris {date:?}: the half day is announced, its hours are not"
            );
            assert!(
                calendar
                    .is_open(paris_time(date, (12, 0, 0)))
                    .is_err_and(|error| matches!(error, CalendarQueryError::UnresolvedGap { .. })),
                "paris {date:?}: is_open must refuse rather than claim"
            );
            assert!(
                calendar
                    .is_closed_trade_date(day(date.0, date.1, date.2), SessionKind::Both)
                    .is_err_and(|error| matches!(error, CalendarQueryError::UnresolvedGap { .. })),
                "paris {date:?}: the trade-date closure question must refuse"
            );
        }
    }

    #[test]
    fn other_markets_rows_never_clip_paris() {
        let calendar = paris();
        // The "Wednesday before Easter" half day is Oslo-only, the Irish May
        // Bank Holiday is Dublin-only, and Ascension, Whit Monday, Ferragosto
        // and the substitutes print `Full Trading Day` for Paris: no row and
        // no clipped answer.
        for date in [
            (2025, 4, 16),
            (2025, 5, 5),
            (2025, 5, 29),
            (2025, 6, 9),
            (2025, 8, 15),
            (2026, 4, 1),
            (2026, 1, 2),
            (2026, 5, 4),
            (2026, 12, 28),
        ] {
            assert_eq!(
                calendar.holiday_on(day(date.0, date.1, date.2)),
                None,
                "{date:?} is not a Paris row"
            );
            assert!(
                calendar
                    .is_open(paris_time(date, (14, 0, 0)))
                    .expect("covered"),
                "{date:?}: Paris trades through another market's arrangement"
            );
        }
    }

    #[test]
    fn an_ordinary_weekday_answers_with_the_end_exclusive_close() {
        let calendar = paris();
        assert_eq!(calendar.holiday_on(day(2026, 6, 10)), None);
        assert!(
            calendar
                .is_open(paris_time((2026, 6, 10), (12, 0, 0)))
                .expect("covered")
        );
        // The envelope's last tradeable instant is the 17:40 Trading-at-Last
        // end; closes are end-exclusive.
        assert!(
            calendar
                .is_open(before(Europe::Paris, (2026, 6, 10), (17, 40, 0)))
                .expect("covered")
        );
        assert!(
            !calendar
                .is_open(paris_time((2026, 6, 10), (17, 40, 0)))
                .expect("covered")
        );
    }

    #[test]
    fn the_window_bounds_and_the_pre_floor_refusal() {
        let calendar = paris();
        let coverage = calendar.holiday_coverage().expect("paris ships a table");
        assert_eq!(coverage.first(), day(2025, 1, 1));
        assert_eq!(coverage.last(), day(2026, 12, 31));
        assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);
        assert_eq!(calendar.holiday_on(day(2027, 1, 1)), None);
        assert!(matches!(
            calendar.is_open(paris_time((2009, 12, 31), (12, 0, 0))),
            Err(CalendarQueryError::BeforeSupportFloor { .. })
        ));
        // 2027 is not published anywhere on the operator's site, so the year
        // the wave scopes is deliberately absent and queries past 2026-12-31
        // refuse. The detached snapshot still answers the pure normal week.
        let outside = paris_time((2027, 1, 4), (12, 0, 0));
        assert!(matches!(
            calendar.is_open(outside),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ));
        assert!(
            calendar
                .without_holidays()
                .is_open(outside)
                .expect("a detached snapshot claims no coverage")
        );
        assert_eq!(calendar.without_holidays().holiday_coverage(), None);
    }

    #[test]
    fn every_shipped_row_matches_the_sheets_per_year() {
        let rows = rows_per_year(paris());
        assert_eq!(rows.len(), 15, "eleven closures, two half days, two gaps");
        assert_eq!(
            census(&rows, 2025),
            (6, 2, 0, 0),
            "2025: six closures and two appendix-dated half days"
        );
        assert_eq!(
            census(&rows, 2026),
            (5, 0, 0, 2),
            "2026: five closures; the two half days are announced but unstated"
        );
        // The half days are exactly the sheet's four December eves, and only
        // 2025's state their appendix instant; the 2026 pair ship Unsourced.
        for (date, kind, instant) in &rows {
            if kind == "early close" {
                assert!(
                    matches!(date, (2025, 12, 24 | 31)),
                    "2025's half days are exactly the appendix's two: {date:?}"
                );
                assert_eq!(*instant, Some(50_700), "{date:?} prints 14:05");
            } else if kind == "unsourced" {
                assert!(
                    matches!(date, (2026, 12, 24 | 31)),
                    "the gaps are exactly the announced 2026 eves: {date:?}"
                );
            } else {
                assert_eq!(*instant, None, "{date:?} states no scalar instant");
            }
        }
    }
}

mod tsx {
    use super::*;

    fn tsx() -> ExchangeCalendar {
        calendar_for_exchange(Exchange::Tsx)
    }

    fn toronto(date: (i32, u32, u32), time: (u32, u32, u32)) -> chrono::DateTime<Utc> {
        America::Toronto
            .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
            .single()
            .expect("fixture must be an unambiguous Toronto instant")
            .with_timezone(&Utc)
    }

    #[test]
    fn closures_per_year_match_the_operators_printed_lists() {
        let calendar = tsx();
        // Every entry of the operator's "Canadian Holidays" lists: the 2025
        // and 2026 "Stock Markets Closed" sections of the live page.
        let closed = [
            (2025, 1, 1),
            (2025, 2, 17),
            (2025, 4, 18),
            (2025, 5, 19),
            (2025, 7, 1),
            (2025, 8, 4),
            (2025, 9, 1),
            (2025, 10, 13),
            (2025, 12, 25),
            (2025, 12, 26),
            (2026, 1, 1),
            (2026, 2, 16),
            (2026, 4, 3),
            (2026, 5, 18),
            (2026, 7, 1),
            (2026, 8, 3),
            (2026, 9, 7),
            (2026, 10, 12),
            (2026, 12, 25),
            (2026, 12, 28),
        ];
        for date in closed {
            assert_closed(calendar, date, "tsx", &toronto);
        }
    }

    #[test]
    fn christmas_eve_closes_at_the_printed_1300_toronto_time() {
        let calendar = tsx();
        // The calendar's footnote: `* Closing at 1:00 PM (TSX/TSXV) and 1:30
        // (ALPHA/ALPHA X/DRK)`. This identity's scope is the TSX/TSXV close;
        // the 1:30 book-system half is outside it.
        for date in [(2025, 12, 24), (2026, 12, 24)] {
            assert_eq!(
                calendar
                    .holiday_on(day(date.0, date.1, date.2))
                    .map(Holiday::kind),
                Some(HolidayKind::EarlyClose { close_ssm: 46_800 }),
                "tsx {date:?} carries the printed 1:00 PM close"
            );
            assert!(
                calendar
                    .is_open(toronto(date, (12, 59, 59)))
                    .expect("covered"),
                "tsx {date:?} still trades at 12:59:59"
            );
            assert!(
                !calendar
                    .is_open(toronto(date, (13, 0, 0)))
                    .expect("covered"),
                "tsx {date:?} is closed at the 1:00 PM close (end-exclusive)"
            );
            // The 16:15 extended-trading session the normal week carries does
            // not survive a 13:00 close.
            assert!(
                !calendar
                    .is_open(toronto(date, (16, 20, 0)))
                    .expect("covered"),
                "tsx {date:?} has no extended session after the early close"
            );
            assert!(
                calendar
                    .is_open(toronto(date, (10, 0, 0)))
                    .expect("covered"),
                "tsx {date:?} trades the morning"
            );
        }
    }

    #[test]
    fn the_us_holiday_list_is_settlement_only_and_never_clips() {
        let calendar = tsx();
        // The same page lists U.S. holidays under a separate heading footnoted
        // `** U.S. Holidays with Special Settlement for Issues trading in USD`
        // — a settlement arrangement, not a trading closure
        // (LAW-SESSION-NOT-EXPIRY).
        for date in [
            (2025, 1, 20),
            (2025, 5, 26),
            (2025, 6, 19),
            (2025, 7, 4),
            (2025, 11, 27),
            (2026, 1, 19),
            (2026, 5, 25),
            (2026, 6, 19),
            (2026, 7, 3),
            (2026, 11, 26),
        ] {
            assert_eq!(
                calendar.holiday_on(day(date.0, date.1, date.2)),
                None,
                "tsx {date:?} is an ordinary trading day, not a settlement-only listing"
            );
            assert!(
                calendar
                    .is_open(toronto(date, (12, 0, 0)))
                    .expect("covered"),
                "tsx {date:?} trades through the U.S. holiday"
            );
        }
    }

    #[test]
    fn an_ordinary_weekday_answers_with_the_end_exclusive_close() {
        let calendar = tsx();
        assert_eq!(calendar.holiday_on(day(2026, 6, 10)), None);
        assert!(
            calendar
                .is_open(toronto((2026, 6, 10), (12, 0, 0)))
                .expect("covered")
        );
        // The envelope's last tradeable instant is the 17:00 extended-trading
        // end; closes are end-exclusive.
        assert!(
            calendar
                .is_open(before(America::Toronto, (2026, 6, 10), (17, 0, 0)))
                .expect("covered")
        );
        assert!(
            !calendar
                .is_open(toronto((2026, 6, 10), (17, 0, 0)))
                .expect("covered")
        );
    }

    #[test]
    fn the_window_bounds_and_the_pre_floor_refusal() {
        let calendar = tsx();
        let coverage = calendar.holiday_coverage().expect("tsx ships a table");
        assert_eq!(coverage.first(), day(2025, 1, 1));
        assert_eq!(coverage.last(), day(2026, 12, 31));
        assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);
        assert_eq!(calendar.holiday_on(day(2027, 1, 1)), None);
        assert!(matches!(
            calendar.is_open(toronto((2009, 12, 31), (12, 0, 0))),
            Err(CalendarQueryError::BeforeSupportFloor { .. })
        ));
        // 2027 is not published: TMX adds the next year in Q4, so queries past
        // 2026-12-31 refuse rather than answer from an unaudited normal week.
        let outside = toronto((2027, 1, 4), (12, 0, 0));
        assert!(matches!(
            calendar.is_open(outside),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ));
        assert!(
            calendar
                .without_holidays()
                .is_open(outside)
                .expect("a detached snapshot claims no coverage")
        );
        assert_eq!(calendar.without_holidays().holiday_coverage(), None);
    }

    #[test]
    fn every_shipped_row_matches_the_sheet_per_year() {
        let rows = rows_per_year(tsx());
        assert_eq!(rows.len(), 22, "twenty closures and two Christmas Eves");
        assert_eq!(
            census(&rows, 2025),
            (10, 1, 0, 0),
            "2025: ten closures and the printed Christmas Eve close"
        );
        assert_eq!(
            census(&rows, 2026),
            (10, 1, 0, 0),
            "2026: ten closures (the Boxing Day substitute included) and the \
             printed Christmas Eve close"
        );
        // The early closes are exactly the two printed Christmas Eves, each
        // stating the 1:00 PM instant.
        for (date, kind, instant) in &rows {
            if kind == "early close" {
                assert!(
                    matches!(date, (2025 | 2026, 12, 24)),
                    "the early closes are exactly the sheet's two: {date:?}"
                );
                assert_eq!(*instant, Some(46_800), "{date:?} prints 1:00 PM");
            } else {
                assert_eq!(*instant, None, "{date:?} states no scalar instant");
            }
        }
    }
}
