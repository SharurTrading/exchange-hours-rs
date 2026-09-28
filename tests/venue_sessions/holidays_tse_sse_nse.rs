// SPDX-License-Identifier: MIT-0

//! Built-in holiday rows for the three APAC cash-equity venues: `tse`,
//! `nse_india` and `sse`.
//!
//! Each venue's fence is written against the operator's own printed list, not
//! against the module: every asserted kind, date, tier and document id is
//! spelled out here independently, so a mutation of any shipped row fails on
//! the row it moves rather than on a count that copied the module.
//!
//! The three tables are closures only, plus two `Unsourced` Muhurat-Trading
//! dates for NSE whose special-session instants the operator has not
//! published; the fence pins both the withheld shape (the date refuses rather
//! than answers) and the closure shape (the day answers shut, end-exclusively
//! around the sessions that do run).
//!
//! Coverage horizons are the operators' own: JPX publishes the current and
//! next year (2025-2027 audited), SSE its annual December notice (2025-2026),
//! and NSE its annual list (2025-2026). The fences hold each table to its
//! window: a date outside it refuses rather than answers from the normal week.

use chrono::{Days, TimeDelta};
use chrono_tz::Asia;
use exchange_hours::{
    CalendarQueryError, DateCoverage, EvidenceTier, Exchange, ExchangeCalendar, Holiday,
    HolidayKind, SessionKind, calendar_for_exchange,
};

use super::prelude::zoned;

fn day(year: i32, month: u32, date: u32) -> chrono::NaiveDate {
    chrono::NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
}

/// Asserts the row `date` ships is a `Closed` row citing `document` at T1,
/// and that the whole trade date answers shut. The one exception is the
/// window's own first date: settling that trade date reads the day before it,
/// which sits outside every audited window, so the query refuses there — the
/// same shape the CFE fence documents for its New Year row.
#[expect(
    clippy::panic,
    reason = "a shared test helper is not itself a #[test], so the \
              allow-panic-in-tests switch does not see it; aborting on a \
              broken fixture is the test's job"
)]
fn assert_closure(calendar: ExchangeCalendar, date: (i32, u32, u32), document: &str, label: &str) {
    let (year, month, day_of_month) = date;
    let trade_date = day(year, month, day_of_month);
    let row = calendar
        .holiday_on(trade_date)
        .unwrap_or_else(|| panic!("{label}: {trade_date} must ship a row"));
    assert_eq!(
        row.kind(),
        HolidayKind::Closed,
        "{label}: {trade_date} kind"
    );
    assert_eq!(row.tier(), EvidenceTier::T1, "{label}: {trade_date} tier");
    assert_eq!(
        row.document_id(),
        document,
        "{label}: {trade_date} document id"
    );
    // Settling a trade-date question scans backward to the previous session;
    // around New Year that scan walks off the window's first day (2025-01-01,
    // 01-02 and 01-03 are all rows and the prior session sits in 2024), and
    // beside a withheld Muhurat date it walks onto that withheld day. The
    // contract is to refuse naming the day the scan needed: accepted only when
    // the error names a day before the window or a day the table withholds.
    // Everywhere else the date must answer shut.
    match calendar.is_closed_trade_date(trade_date, SessionKind::Both) {
        Ok(closed) => assert!(
            closed,
            "{label}: {trade_date} has no session in either phase"
        ),
        Err(CalendarQueryError::OutsideCoveredRange { date, .. }) => {
            let first = calendar
                .holiday_coverage()
                .map(exchange_hours::HolidayCoverage::first)
                .expect("each of these venues ships a table");
            assert!(
                date < first,
                "{label}: {trade_date} refused naming in-window {date}; only a scan that \
                 exits the window start may refuse"
            );
        }
        Err(CalendarQueryError::UnresolvedGap { date, .. }) => {
            let withheld = calendar
                .holiday_on(date)
                .map(Holiday::kind)
                .is_some_and(|kind| kind == HolidayKind::Unsourced);
            assert!(
                withheld,
                "{label}: {trade_date} refused naming {date}, which the table does not withhold"
            );
        }
        Err(other) => panic!(
            "{label}: {trade_date} must answer shut or refuse naming the day its scan needed, \
             got {other:?}"
        ),
    }
}

/// Asserts the date refuses as an `Unsourced` gap: the row exists, the metadata
/// names the date unresolved, and the date-aware queries refuse rather than
/// answer from the normal week.
#[expect(
    clippy::panic,
    reason = "a shared test helper is not itself a #[test], so the \
              allow-panic-in-tests switch does not see it; aborting on a \
              broken fixture is the test's job"
)]
fn assert_unsourced(
    calendar: ExchangeCalendar,
    date: (i32, u32, u32),
    document: &str,
    label: &str,
) {
    let (year, month, day_of_month) = date;
    let trade_date = day(year, month, day_of_month);
    let row = calendar
        .holiday_on(trade_date)
        .unwrap_or_else(|| panic!("{label}: {trade_date} must ship a row"));
    assert_eq!(
        row.kind(),
        HolidayKind::Unsourced,
        "{label}: {trade_date} kind"
    );
    assert_eq!(row.tier(), EvidenceTier::T1, "{label}: {trade_date} tier");
    assert_eq!(
        row.document_id(),
        document,
        "{label}: {trade_date} document id"
    );
    assert_eq!(
        calendar.coverage().coverage_on(trade_date),
        DateCoverage::UnresolvedGap,
        "{label}: {trade_date} metadata verdict"
    );
    assert!(
        matches!(
            calendar.is_closed_trade_date(trade_date, SessionKind::Both),
            Err(CalendarQueryError::UnresolvedGap { .. })
        ),
        "{label}: {trade_date} must refuse, never answer a closure or an open day"
    );
}

/// Asserts one ordinary weekday trades: open mid-morning, open the nanosecond
/// before the final close, closed at the close itself (closes are
/// end-exclusive), and the trade date resolves to the venue-local day.
fn assert_ordinary_weekday(
    calendar: ExchangeCalendar,
    date: (i32, u32, u32),
    final_close: (u32, u32),
    label: &str,
) {
    let close = zoned(calendar.tz(), date, (final_close.0, final_close.1, 0));
    let mid_session = zoned(calendar.tz(), date, (10, 0, 0));
    assert!(
        calendar
            .is_open(mid_session)
            .expect("{label}: the coverage contract must answer a covered date"),
        "{label}: an ordinary weekday is open mid-session"
    );
    assert!(
        calendar
            .is_open(close - TimeDelta::nanoseconds(1))
            .expect("{label}: the coverage contract must answer a covered date"),
        "{label}: the instant before the final close is open"
    );
    assert!(
        !calendar
            .is_open(close)
            .expect("{label}: the coverage contract must answer a covered date"),
        "{label}: the final close is end-exclusive"
    );
    assert_eq!(
        calendar
            .trade_date(mid_session)
            .expect("{label}: the coverage contract must answer a covered date"),
        Some(day(date.0, date.1, date.2)),
        "{label}: an ordinary weekday's trade date is its own local day"
    );
}

/// Asserts every TSE closure row, per year, against the operator's printed
/// tables. The date list is spelled here independently of the module: a row
/// that moves, loses its clip or changes its citation fails here.
#[test]
fn tse_printed_closures_ship_a_row_per_year() {
    let tse = calendar_for_exchange(Exchange::Tse);
    for (date, document, label) in [
        // 2025, from the operator's 2025 table.
        ((2025, 1, 1), "JPX-HOL-2025", "New Year's Day"),
        ((2025, 1, 2), "JPX-HOL-2025", "Market Holiday"),
        ((2025, 1, 3), "JPX-HOL-2025", "Market Holiday"),
        ((2025, 1, 13), "JPX-HOL-2025", "Coming of Age Day"),
        ((2025, 2, 11), "JPX-HOL-2025", "National Foundation Day"),
        ((2025, 2, 24), "JPX-HOL-2025", "Emperor's Birthday observed"),
        ((2025, 3, 20), "JPX-HOL-2025", "Vernal Equinox"),
        ((2025, 4, 29), "JPX-HOL-2025", "Showa Day"),
        ((2025, 5, 5), "JPX-HOL-2025", "Children's Day"),
        ((2025, 5, 6), "JPX-HOL-2025", "Greenery Day observed"),
        ((2025, 7, 21), "JPX-HOL-2025", "Marine Day"),
        ((2025, 8, 11), "JPX-HOL-2025", "Mountain Day"),
        ((2025, 9, 15), "JPX-HOL-2025", "Respect for the Aged Day"),
        ((2025, 9, 23), "JPX-HOL-2025", "Autumnal Equinox"),
        ((2025, 10, 13), "JPX-HOL-2025", "Sports Day"),
        ((2025, 11, 3), "JPX-HOL-2025", "Culture Day"),
        (
            (2025, 11, 24),
            "JPX-HOL-2025",
            "Labor Thanksgiving Day observed",
        ),
        ((2025, 12, 31), "JPX-HOL-2025", "Market Holiday"),
        // 2026, from the operator's 2026 table.
        ((2026, 1, 1), "JPX-HOL-2026-2027", "New Year's Day"),
        ((2026, 1, 2), "JPX-HOL-2026-2027", "Market Holiday"),
        ((2026, 1, 12), "JPX-HOL-2026-2027", "Coming of Age Day"),
        (
            (2026, 2, 11),
            "JPX-HOL-2026-2027",
            "National Foundation Day",
        ),
        ((2026, 2, 23), "JPX-HOL-2026-2027", "Emperor's Birthday"),
        ((2026, 3, 20), "JPX-HOL-2026-2027", "Vernal Equinox"),
        ((2026, 4, 29), "JPX-HOL-2026-2027", "Showa Day"),
        ((2026, 5, 4), "JPX-HOL-2026-2027", "Greenery Day"),
        ((2026, 5, 5), "JPX-HOL-2026-2027", "Children's Day"),
        (
            (2026, 5, 6),
            "JPX-HOL-2026-2027",
            "Constitution Memorial Day observed",
        ),
        ((2026, 7, 20), "JPX-HOL-2026-2027", "Marine Day"),
        ((2026, 8, 11), "JPX-HOL-2026-2027", "Mountain Day"),
        (
            (2026, 9, 21),
            "JPX-HOL-2026-2027",
            "Respect for the Aged Day",
        ),
        ((2026, 9, 22), "JPX-HOL-2026-2027", "citizen's holiday"),
        ((2026, 9, 23), "JPX-HOL-2026-2027", "Autumnal Equinox"),
        ((2026, 10, 12), "JPX-HOL-2026-2027", "Sports Day"),
        ((2026, 11, 3), "JPX-HOL-2026-2027", "Culture Day"),
        (
            (2026, 11, 23),
            "JPX-HOL-2026-2027",
            "Labor Thanksgiving Day",
        ),
        ((2026, 12, 31), "JPX-HOL-2026-2027", "Market Holiday"),
        // 2027, from the operator's 2027 table.
        ((2027, 1, 1), "JPX-HOL-2026-2027", "New Year's Day"),
        ((2027, 1, 11), "JPX-HOL-2026-2027", "Coming of Age Day"),
        (
            (2027, 2, 11),
            "JPX-HOL-2026-2027",
            "National Foundation Day",
        ),
        ((2027, 2, 23), "JPX-HOL-2026-2027", "Emperor's Birthday"),
        (
            (2027, 3, 22),
            "JPX-HOL-2026-2027",
            "Vernal Equinox observed",
        ),
        ((2027, 4, 29), "JPX-HOL-2026-2027", "Showa Day"),
        (
            (2027, 5, 3),
            "JPX-HOL-2026-2027",
            "Constitution Memorial Day",
        ),
        ((2027, 5, 4), "JPX-HOL-2026-2027", "Greenery Day"),
        ((2027, 5, 5), "JPX-HOL-2026-2027", "Children's Day"),
        ((2027, 7, 19), "JPX-HOL-2026-2027", "Marine Day"),
        ((2027, 8, 11), "JPX-HOL-2026-2027", "Mountain Day"),
        (
            (2027, 9, 20),
            "JPX-HOL-2026-2027",
            "Respect for the Aged Day",
        ),
        ((2027, 9, 23), "JPX-HOL-2026-2027", "Autumnal Equinox"),
        ((2027, 10, 11), "JPX-HOL-2026-2027", "Sports Day"),
        ((2027, 11, 3), "JPX-HOL-2026-2027", "Culture Day"),
        (
            (2027, 11, 23),
            "JPX-HOL-2026-2027",
            "Labor Thanksgiving Day",
        ),
        ((2027, 12, 31), "JPX-HOL-2026-2027", "Market Holiday"),
    ] {
        assert_closure(tse, date, document, &format!("TSE {label}"));
    }
}

/// Asserts every SSE closure row, per year, against the operator's printed
/// notices. The date list is spelled here independently of the module: a row
/// that moves, loses its clip or changes its citation fails here.
#[test]
fn sse_printed_closures_ship_a_row_per_year() {
    let sse = calendar_for_exchange(Exchange::Sse);
    for (date, document, label) in [
        // 2025, from 上证公告〔2024〕38号.
        ((2025, 1, 1), "SSE-NOTICE-2024-38", "New Year's Day"),
        ((2025, 1, 28), "SSE-NOTICE-2024-38", "Spring Festival"),
        ((2025, 1, 29), "SSE-NOTICE-2024-38", "Spring Festival"),
        ((2025, 1, 30), "SSE-NOTICE-2024-38", "Spring Festival"),
        ((2025, 1, 31), "SSE-NOTICE-2024-38", "Spring Festival"),
        ((2025, 2, 3), "SSE-NOTICE-2024-38", "Spring Festival"),
        ((2025, 2, 4), "SSE-NOTICE-2024-38", "Spring Festival"),
        ((2025, 4, 4), "SSE-NOTICE-2024-38", "Qingming"),
        ((2025, 5, 1), "SSE-NOTICE-2024-38", "Labour Day"),
        ((2025, 5, 2), "SSE-NOTICE-2024-38", "Labour Day"),
        ((2025, 5, 5), "SSE-NOTICE-2024-38", "Labour Day"),
        ((2025, 6, 2), "SSE-NOTICE-2024-38", "Dragon Boat"),
        (
            (2025, 10, 1),
            "SSE-NOTICE-2024-38",
            "National Day + Mid-Autumn",
        ),
        (
            (2025, 10, 2),
            "SSE-NOTICE-2024-38",
            "National Day + Mid-Autumn",
        ),
        (
            (2025, 10, 3),
            "SSE-NOTICE-2024-38",
            "National Day + Mid-Autumn",
        ),
        (
            (2025, 10, 6),
            "SSE-NOTICE-2024-38",
            "National Day + Mid-Autumn",
        ),
        (
            (2025, 10, 7),
            "SSE-NOTICE-2024-38",
            "National Day + Mid-Autumn",
        ),
        (
            (2025, 10, 8),
            "SSE-NOTICE-2024-38",
            "National Day + Mid-Autumn",
        ),
        // 2026, from 上证公告〔2025〕45号.
        ((2026, 1, 1), "SSE-NOTICE-2025-45", "New Year's Day"),
        ((2026, 1, 2), "SSE-NOTICE-2025-45", "New Year's Day"),
        ((2026, 2, 16), "SSE-NOTICE-2025-45", "Spring Festival"),
        ((2026, 2, 17), "SSE-NOTICE-2025-45", "Spring Festival"),
        ((2026, 2, 18), "SSE-NOTICE-2025-45", "Spring Festival"),
        ((2026, 2, 19), "SSE-NOTICE-2025-45", "Spring Festival"),
        ((2026, 2, 20), "SSE-NOTICE-2025-45", "Spring Festival"),
        ((2026, 2, 23), "SSE-NOTICE-2025-45", "Spring Festival"),
        ((2026, 4, 6), "SSE-NOTICE-2025-45", "Qingming"),
        ((2026, 5, 1), "SSE-NOTICE-2025-45", "Labour Day"),
        ((2026, 5, 4), "SSE-NOTICE-2025-45", "Labour Day"),
        ((2026, 5, 5), "SSE-NOTICE-2025-45", "Labour Day"),
        ((2026, 6, 19), "SSE-NOTICE-2025-45", "Dragon Boat"),
        ((2026, 9, 25), "SSE-NOTICE-2025-45", "Mid-Autumn"),
        ((2026, 10, 1), "SSE-NOTICE-2025-45", "National Day"),
        ((2026, 10, 2), "SSE-NOTICE-2025-45", "National Day"),
        ((2026, 10, 5), "SSE-NOTICE-2025-45", "National Day"),
        ((2026, 10, 6), "SSE-NOTICE-2025-45", "National Day"),
        ((2026, 10, 7), "SSE-NOTICE-2025-45", "National Day"),
    ] {
        assert_closure(sse, date, document, &format!("SSE {label}"));
    }
}

/// Asserts every NSE closure row, per year, against the operator's printed
/// lists. The date list is spelled here independently of the module: a row
/// that moves, loses its clip or changes its citation fails here.
#[test]
fn nse_printed_closures_ship_a_row_per_year() {
    let nse = calendar_for_exchange(Exchange::NseIndia);
    for (date, document, label) in [
        // 2025, from the operator's 2025 list.
        ((2025, 2, 26), "NSE-HOL-2025", "Mahashivratri"),
        ((2025, 3, 14), "NSE-HOL-2025", "Holi"),
        ((2025, 3, 31), "NSE-HOL-2025", "Ramadan Eid"),
        ((2025, 4, 10), "NSE-HOL-2025", "Mahavir Jayanti"),
        ((2025, 4, 14), "NSE-HOL-2025", "Ambedkar Jayanti"),
        ((2025, 4, 18), "NSE-HOL-2025", "Good Friday"),
        ((2025, 5, 1), "NSE-HOL-2025", "Maharashtra Day"),
        ((2025, 8, 15), "NSE-HOL-2025", "Independence Day"),
        ((2025, 8, 27), "NSE-HOL-2025", "Ganesh Chaturthi"),
        ((2025, 10, 2), "NSE-HOL-2025", "Gandhi Jayanti / Dussehra"),
        ((2025, 10, 22), "NSE-HOL-2025", "Diwali Balipratipada"),
        ((2025, 11, 5), "NSE-HOL-2025", "Gurunanak Jayanti"),
        ((2025, 12, 25), "NSE-HOL-2025", "Christmas"),
        // 2026, from the operator's 2026 list.
        ((2026, 1, 26), "NSE-HOL-2026", "Republic Day"),
        ((2026, 3, 3), "NSE-HOL-2026", "Holi"),
        ((2026, 3, 26), "NSE-HOL-2026", "Ram Navami"),
        ((2026, 3, 31), "NSE-HOL-2026", "Mahavir Jayanti"),
        ((2026, 4, 3), "NSE-HOL-2026", "Good Friday"),
        ((2026, 4, 14), "NSE-HOL-2026", "Ambedkar Jayanti"),
        ((2026, 5, 1), "NSE-HOL-2026", "Maharashtra Day"),
        ((2026, 5, 28), "NSE-HOL-2026", "Bakri Id"),
        ((2026, 6, 26), "NSE-HOL-2026", "Muharram"),
        ((2026, 9, 14), "NSE-HOL-2026", "Ganesh Chaturthi"),
        ((2026, 10, 2), "NSE-HOL-2026", "Gandhi Jayanti"),
        ((2026, 10, 20), "NSE-HOL-2026", "Dussehra"),
        ((2026, 11, 10), "NSE-HOL-2026", "Diwali-Balipratipada"),
        ((2026, 11, 24), "NSE-HOL-2026", "Gurunanak Jayanti"),
        ((2026, 12, 25), "NSE-HOL-2026", "Christmas"),
    ] {
        assert_closure(nse, date, document, &format!("NSE {label}"));
    }
}

/// The two Muhurat-Trading dates are withheld, not closed: the operator
/// announces a session there whose instants it has not published.
#[test]
fn the_muhurat_dates_are_unsourced_neither_closed_nor_normal() {
    let nse = calendar_for_exchange(Exchange::NseIndia);
    assert_unsourced(
        nse,
        (2025, 10, 21),
        "NSE-HOL-2025",
        "Diwali Laxmi Pujan + Muhurat",
    );
    assert_unsourced(nse, (2026, 11, 8), "NSE-HOL-2026", "Sunday Muhurat");

    // A probe inside the Sunday Muhurat day refuses too, rather than reading
    // the normal weekend. The withholding shadows its own Monday as well: a
    // query on 2026-11-09 scans the day before it, touches the withheld
    // 2026-11-08 row, and refuses naming that day — the crate's honest
    // failure mode for a scan that reaches an unresolved date. The Saturday
    // before is untouched by the scan and answers as an ordinary weekend.
    assert!(matches!(
        nse.is_open(zoned(Asia::Kolkata, (2026, 11, 8), (18, 0, 0))),
        Err(CalendarQueryError::UnresolvedGap { .. })
    ));
    assert!(matches!(
        nse.is_open(zoned(Asia::Kolkata, (2026, 11, 9), (10, 0, 0))),
        Err(CalendarQueryError::UnresolvedGap { date, .. }) if date == day(2026, 11, 8)
    ));
    assert!(
        !nse.is_open(zoned(Asia::Kolkata, (2026, 11, 7), (10, 0, 0)))
            .expect("2026-11-07 is an audited weekend and must answer"),
        "the Saturday before the Muhurat Sunday is an ordinary weekend"
    );
    // 2025-10-21 is withheld as a whole; the listed closures on both sides
    // answer shut.
    let diwali_eve = day(2025, 10, 20);
    assert_eq!(
        nse.holiday_on(diwali_eve),
        None,
        "2025-10-20 is audited normal"
    );
    // 2025-10-22 answers shut; settling its trade date scans back onto the
    // withheld 2025-10-21 and may refuse naming that day, which is the
    // withholding's own shadow rather than an answer about 2025-10-22.
    match nse.is_closed_trade_date(day(2025, 10, 22), SessionKind::Both) {
        Ok(closed) => assert!(closed, "Diwali Balipratipada answers shut"),
        Err(CalendarQueryError::UnresolvedGap { date, .. }) => {
            assert_eq!(date, day(2025, 10, 21), "the scan named the withheld day");
        }
        Err(other) => panic!("unexpected refusal for 2025-10-22: {other:?}"),
    }
}

/// The counts per year, read back from the shipped tables by walking them:
/// 18 + 19 + 17 TSE closures, 18 + 19 SSE closures, and 13 + 1 plus 15 + 1
/// NSE rows (the one per year being the withheld Muhurat date). A row added,
/// moved across a year or dropped breaks the count; the per-row fence above
/// pins where.
#[test]
fn the_window_counts_are_the_printed_lists_counts() {
    fn count_by_year(calendar: ExchangeCalendar) -> Vec<(i32, usize, usize)> {
        let coverage = calendar
            .holiday_coverage()
            .expect("each of these venues ships a table");
        let mut out: Vec<(i32, usize, usize)> = Vec::new();
        let mut date = coverage.first();
        while date <= coverage.last() {
            let year = date.year();
            let entry = match out.last_mut() {
                Some((years, _, _)) if *years == year => out.last_mut().expect("just checked"),
                _ => {
                    out.push((year, 0, 0));
                    out.last_mut().expect("just pushed")
                }
            };
            match calendar.holiday_on(date).map(Holiday::kind) {
                Some(HolidayKind::Closed) => entry.1 += 1,
                Some(HolidayKind::Unsourced) => entry.2 += 1,
                Some(other) => {
                    panic!("{date} ships a kind these venues do not state: {other:?}")
                }
                None => {}
            }
            date = date
                .succ_opt()
                .expect("the window stays inside the representable calendar");
        }
        out
    }

    use chrono::Datelike as _;

    assert_eq!(
        count_by_year(calendar_for_exchange(Exchange::Tse)),
        [(2025, 18, 0), (2026, 19, 0), (2027, 17, 0)],
        "TSE closures per year, from the operator's tables"
    );
    assert_eq!(
        count_by_year(calendar_for_exchange(Exchange::Sse)),
        [(2025, 18, 0), (2026, 19, 0)],
        "SSE closures per year, from the operator's notices"
    );
    assert_eq!(
        count_by_year(calendar_for_exchange(Exchange::NseIndia)),
        [(2025, 13, 1), (2026, 15, 1)],
        "NSE closures and withheld Muhurat dates per year, from the operator's lists"
    );
}

/// An ordinary mid-window weekday trades with end-exclusive bounds on each
/// venue's own envelope close.
#[test]
fn ordinary_weekdays_inside_the_window_trade_end_exclusively() {
    // TSE: the venue union runs to the 18:00 ToSTNeT close today.
    assert_ordinary_weekday(
        calendar_for_exchange(Exchange::Tse),
        (2026, 2, 24),
        (18, 0),
        "TSE Tuesday after the Emperor's Birthday closure",
    );
    // SSE: the venue union runs to the 15:30 block/fixed-price close.
    assert_ordinary_weekday(
        calendar_for_exchange(Exchange::Sse),
        (2025, 10, 9),
        (15, 30),
        "SSE Thursday after the National Day week",
    );
    // NSE: the venue union runs to the 16:00 post-close close.
    assert_ordinary_weekday(
        calendar_for_exchange(Exchange::NseIndia),
        (2026, 4, 6),
        (16, 0),
        "NSE Monday after Good Friday",
    );
}

/// Each table answers only inside its own window: the first row-bearing date's
/// day before and the last date's day after carry no row, the metadata dates
/// the window, and a query outside it refuses rather than reading the normal
/// week. Pre-floor instants refuse as before the floor.
#[test]
fn coverage_runs_exactly_over_each_operators_published_window() {
    // TSE: 2025-01-01 .. 2027-12-31.
    let tse = calendar_for_exchange(Exchange::Tse);
    let tse_coverage = tse.holiday_coverage().expect("TSE ships a table");
    assert_eq!(tse_coverage.first(), day(2025, 1, 1));
    assert_eq!(tse_coverage.last(), day(2027, 12, 31));
    assert_eq!(
        tse.holiday_on(day(2024, 12, 31)),
        None,
        "the day before the window carries no row"
    );
    assert_eq!(
        tse.holiday_on(day(2028, 1, 3)),
        None,
        "the day after the window carries no row"
    );
    assert!(
        matches!(
            tse.is_open(zoned(Asia::Tokyo, (2024, 12, 31), (10, 0, 0))),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ),
        "a 2024 weekday is inside no audited window and must be refused"
    );
    assert!(
        matches!(
            tse.is_open(zoned(Asia::Tokyo, (2028, 1, 4), (10, 0, 0))),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ),
        "a 2028 weekday is past the published schedule and must be refused"
    );

    // SSE: 2025-01-01 .. 2026-12-31; the 2027 notice is not published.
    let sse = calendar_for_exchange(Exchange::Sse);
    let sse_coverage = sse.holiday_coverage().expect("SSE ships a table");
    assert_eq!(sse_coverage.first(), day(2025, 1, 1));
    assert_eq!(sse_coverage.last(), day(2026, 12, 31));
    assert!(
        matches!(
            sse.is_open(zoned(Asia::Shanghai, (2027, 1, 4), (10, 0, 0))),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ),
        "2027 is unpublished by SSE and the identity must refuse it outright"
    );

    // NSE: 2025-01-01 .. 2026-12-31; the 2027 list is not published.
    let nse = calendar_for_exchange(Exchange::NseIndia);
    let nse_coverage = nse.holiday_coverage().expect("NSE ships a table");
    assert_eq!(nse_coverage.first(), day(2025, 1, 1));
    assert_eq!(nse_coverage.last(), day(2026, 12, 31));
    assert!(
        matches!(
            nse.is_open(zoned(Asia::Kolkata, (2027, 1, 4), (10, 0, 0))),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ),
        "2027 is unpublished by NSE and the identity must refuse it outright"
    );

    // Pre-floor instants refuse naming their venue-local pre-2010 day, on all
    // three, whatever the window does.
    for (calendar, tz) in [
        (&tse, Asia::Tokyo),
        (&sse, Asia::Shanghai),
        (&nse, Asia::Kolkata),
    ] {
        let instant = zoned(tz, (2009, 12, 31), (10, 0, 0));
        assert!(
            matches!(
                calendar.is_open(instant),
                Err(CalendarQueryError::BeforeSupportFloor { date, .. })
                    if date == day(2009, 12, 31)
            ),
            "{:?}: the pre-floor weekday must refuse naming its local day",
            calendar.source()
        );
    }
}

/// The weekend shape survives inside the audited window: a Saturday or Sunday
/// that the normal week closes but no printed holiday touches answers shut as
/// an ordinary weekend, and NSE's Sunday Muhurat date is the one weekend the
/// table withholds.
#[test]
fn weekends_inside_the_window_stay_weekends_except_where_withheld() {
    let sse = calendar_for_exchange(Exchange::Sse);
    let saturday = zoned(Asia::Shanghai, (2025, 10, 4), (10, 0, 0));
    assert!(
        !sse.is_open(saturday)
            .expect("the coverage contract must answer a covered date"),
        "the Saturday inside the National Day range is shut"
    );

    let tse = calendar_for_exchange(Exchange::Tse);
    let sunday = zoned(Asia::Tokyo, (2026, 5, 3), (10, 0, 0));
    assert_eq!(
        tse.holiday_on(day(2026, 5, 3)),
        None,
        "the printed Sunday holiday ships no row"
    );
    assert!(
        !tse.is_open(sunday)
            .expect("the coverage contract must answer a covered date"),
        "a printed Sunday holiday is still just a weekend"
    );
}

/// Detaching the table restores the pure normal week and claims no coverage.
#[test]
fn detaching_the_table_restores_the_normal_week() {
    for exchange in [Exchange::Tse, Exchange::Sse, Exchange::NseIndia] {
        let calendar = calendar_for_exchange(exchange);
        let detached = calendar.without_holidays();
        assert_eq!(detached.holiday_coverage(), None);
        // A detached snapshot answers the pure normal week where the built-in
        // table would have clipped or refused.
        assert!(
            detached
                .is_open(zoned(calendar.tz(), (2026, 2, 24), (10, 0, 0)))
                .expect("a detached snapshot claims no coverage and answers the normal week"),
            "the detached Tuesday trades on the pure normal week"
        );
        assert_eq!(
            detached.coverage().coverage_on(day(2026, 2, 23)),
            DateCoverage::NormalWeekOnly,
            "the detached verdict is the normal-week-only one"
        );
    }
}

/// The per-year counts fence above is a mutation fence only together with a
/// check that no row changed its *kind* within its year: a `Closed` flipped to
/// `Unsourced` (or back) keeps the totals and must fail here. Walk the tables
/// and assert the exact dates that carry the withheld kind.
#[test]
fn the_only_unsourced_rows_are_the_two_muhurat_dates() {
    for exchange in [Exchange::Tse, Exchange::Sse, Exchange::NseIndia] {
        let calendar = calendar_for_exchange(exchange);
        let coverage = calendar.holiday_coverage().expect("each ships a table");
        let mut date = coverage.first();
        while date <= coverage.last() {
            if let Some(row) = calendar.holiday_on(date)
                && row.kind() == HolidayKind::Unsourced
            {
                assert_eq!(
                    exchange,
                    Exchange::NseIndia,
                    "{date}: only NSE ships withheld rows"
                );
                assert!(
                    date == day(2025, 10, 21) || date == day(2026, 11, 8),
                    "{date}: the only withheld dates are the two Muhurat dates"
                );
            }
            date = date
                .checked_add_days(Days::new(1))
                .expect("the window stays inside the representable calendar");
        }
    }
}

/// The venue and its `parse`/`Display` wire form agree, so the consumer's
/// routing strings resolve to the calendars these rows attach to.
#[test]
fn the_wire_names_resolve_to_the_three_calendars() {
    for (wire, exchange) in [
        ("tse", Exchange::Tse),
        ("nse_india", Exchange::NseIndia),
        ("sse", Exchange::Sse),
    ] {
        let parsed: Exchange = wire.parse().expect("the wire name parses");
        assert_eq!(parsed, exchange);
        assert_eq!(parsed.to_string(), wire);
        assert!(calendar_for_exchange(exchange).holiday_coverage().is_some());
    }
}
