// SPDX-License-Identifier: MIT-0

//! Built-in holiday rows for the three served cash-equity venues whose tables
//! shipped with the 2025-2027 wave: `b3`, `tadawul` and `borsa_istanbul`.
//!
//! Every case below goes through the public identity-backed calendar, and each
//! venue's section fences its own kinds: closures per year, the half-day and
//! late-open instants at second granularity so a row that moves fails, an
//! ordinary weekday inside the window, the end-exclusive close, the coverage
//! endpoints, the pre-floor refusal, and a per-kind census so a row cannot
//! change kind silently.
//!
//! The Islamic-calendar rows (Tadawul) and the Carnival rows (B3) are fenced
//! against the dates the operators printed, not against a computed calendar:
//! each entry names its own printed day, so a row keyed to a computed rather
//! than printed date fails here.

use super::prelude::*;

use chrono::TimeZone as _;
use chrono_tz::{America, Asia, Europe};
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
    if date == (2025, 1, 1) {
        assert!(
            calendar
                .is_closed_trade_date(day(date.0, date.1, date.2), SessionKind::Both)
                .is_err_and(|error| matches!(
                    error,
                    CalendarQueryError::OutsideCoveredRange { .. }
                )),
            "{label}: the derivation behind {date:?} reads unaudited 2024-12-31"
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
                .is_closed_trade_date(day(date.0, date.1, date.2), SessionKind::Both)
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

// ---------------------------------------------------------------------------
// b3 — B3 (Brasil, Bolsa, Balcão), 2025-2026, America/Sao_Paulo.
// ---------------------------------------------------------------------------

#[test]
fn b3_closures_per_year_match_the_operators_printed_calendars() {
    let calendar = calendar_for(Exchange::B3);
    // 2025: the circular's Quadro A, equities leg. The weekend holidays
    // (07 September, 12 October, 02 November, 15 November) print no session
    // change and ship no row.
    for date in [
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
    ] {
        assert_closed(calendar, date, "B3 2025 closure", &|d, time| {
            sao_paulo(d, time)
        });
    }
    // 2026: the operator's 2026 article, including the two year-end days it
    // states separately have no trading session.
    for date in [
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
    ] {
        assert_closed(calendar, date, "B3 2026 closure", &|d, time| {
            sao_paulo(d, time)
        });
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

/// Ash Wednesday is a **late open**: the operator states special hours with
/// equities trading beginning at 13:00 on the long grid, whose 17:55 close is
/// unchanged. The probes are at second granularity so a row that takes any
/// other instant fails.
#[test]
fn b3_ash_wednesday_opens_late_at_1300_sao_paulo() {
    let calendar = calendar_for(Exchange::B3);

    for date in [(2025, 3, 5), (2026, 2, 18)] {
        assert_eq!(
            calendar
                .holiday_on(day(date.0, date.1, date.2))
                .map(Holiday::kind),
            Some(HolidayKind::LateOpen {
                open_ssm: 13 * 3_600
            }),
            "{date:?} carries the printed late open"
        );
        // The printed 12:45-13:00 pre-opening accepts no orders that match, and
        // continuous trading begins exactly at 13:00.
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
        // The long grid's close is unchanged and stays end-exclusive.
        assert!(
            calendar
                .is_open(sao_paulo(date, (17, 54, 59)))
                .expect("a covered date answers"),
            "{date:?} still trades through the long-grid afternoon"
        );
        // The regular session hands into the unchanged 17:55-18:00 closing
        // call; the final close stays end-exclusive at 18:00.
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
    let coverage = calendar
        .holiday_coverage()
        .expect("B3 ships a built-in table");
    assert_eq!(coverage.first(), day(2025, 1, 1));
    assert_eq!(coverage.last(), day(2026, 12, 31));
    assert_eq!(
        calendar.holiday_on(coverage.first().pred_opt().expect("representable")),
        None,
        "no answer below the window"
    );
    // 2027-01-04 is inside 2027 but outside the audited window: the identity
    // refuses the date rather than claiming a normal Monday.
    let outside = sao_paulo((2027, 1, 4), (11, 0, 0));
    assert!(matches!(
        calendar.is_open(outside),
        Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == day(2027, 1, 4)
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
                panic!("{date} ships a kind B3's 2025-2026 calendars do not state: {other:?}")
            }
        }
        date = date.succ_opt().expect("the window stays representable");
    }
    assert_eq!(
        (closed, late_opens),
        (25, 2),
        "closures and late opens, B3 2025-2026"
    );
}

// ---------------------------------------------------------------------------
// tadawul — Saudi Exchange Main Market, 2025-2027, Asia/Riyadh (Sun-Thu).
// ---------------------------------------------------------------------------

#[test]
fn tadawul_closures_per_year_match_the_operators_printed_entries() {
    let calendar = calendar_for(Exchange::Tadawul);
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
    // them.
    assert_eq!(
        calendar.holiday_on(day(2025, 3, 28)),
        None,
        "a Friday leg needs no row in a Sun-Thu week"
    );
    assert_eq!(
        calendar.holiday_on(day(2025, 3, 29)),
        None,
        "a Saturday leg needs no row in a Sun-Thu week"
    );
}

#[test]
fn tadawul_trading_resumes_on_the_printed_days() {
    let calendar = calendar_for(Exchange::Tadawul);
    // The entries state resume days: 03/04/2025, 11/06/2025, 24/03/2026,
    // 31/05/2026, 14/03/2027, 23/05/2027 — each open with the ordinary
    // 09:30 order entry and 10:00 regular open.
    for date in [
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
    assert_eq!(coverage.first(), day(2025, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert_eq!(
        calendar.holiday_on(coverage.first().pred_opt().expect("representable")),
        None
    );
    assert_eq!(
        calendar.holiday_on(coverage.last().succ_opt().expect("representable")),
        None
    );
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
    assert_eq!(closed, 34, "closures, Tadawul 2025-2027");
}

// ---------------------------------------------------------------------------
// borsa_istanbul — Borsa İstanbul Equity Market, 2025-2026, Europe/Istanbul.
// ---------------------------------------------------------------------------

#[test]
fn borsa_istanbul_closures_per_year_match_the_operators_printed_tables() {
    let calendar = calendar_for(Exchange::BorsaIstanbul);
    // 2025: the EK-3 annex's no-session dates that land on a weekday.
    for date in [
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
    ] {
        assert_closed(calendar, date, "BIST 2025 closure", &|d, time| {
            istanbul(d, time)
        });
    }
    // 2026.
    for date in [
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
    ] {
        assert_closed(calendar, date, "BIST 2026 closure", &|d, time| {
            istanbul(d, time)
        });
    }
    // Weekend legs (Zafer Bayramı 2025-08-30 and 2026-08-30, Ramazan Arefesi
    // 2025-03-29) change no trade date and ship no row.
    assert_eq!(
        calendar.holiday_on(day(2025, 8, 30)),
        None,
        "a Saturday holiday needs no row in a Mon-Fri week"
    );
}

/// The four half days keep their session and end at the printed 13:00. The
/// probes sit at second granularity, so a row that takes any other instant —
/// or copies a neighbouring holiday's — fails.
#[test]
fn each_half_day_closes_at_the_printed_1300_istanbul() {
    let calendar = calendar_for(Exchange::BorsaIstanbul);
    for date in [
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
    let coverage = calendar
        .holiday_coverage()
        .expect("Borsa Istanbul ships a built-in table");
    assert_eq!(coverage.first(), day(2025, 1, 1));
    assert_eq!(coverage.last(), day(2026, 12, 31));
    assert_eq!(
        calendar.holiday_on(coverage.last().succ_opt().expect("representable")),
        None,
        "no answer past the window: 2027 is unpublished"
    );
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
            Some(other) => panic!(
                "{date} ships a kind Borsa Istanbul's 2025-2026 tables do not state: {other:?}"
            ),
        }
        date = date.succ_opt().expect("the window stays representable");
    }
    assert_eq!(
        (closed, half_days),
        (20, 5),
        "closures and 13:00 half days, Borsa Istanbul 2025-2026"
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
