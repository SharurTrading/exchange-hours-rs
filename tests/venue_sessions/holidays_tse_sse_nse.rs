// SPDX-License-Identifier: MIT-0

//! Built-in holiday rows for the three APAC cash-equity venues: `tse`,
//! `nse_india` and `sse`.
//!
//! Each venue's fence is written against the operator's own printed list, not
//! against the module: every asserted kind, date, tier and document id is
//! spelled out here independently, so a mutation of any shipped row fails on
//! the row it moves rather than on a count that copied the module.
//!
//! The three tables are closures only, plus NSE's Muhurat-Trading dates: the
//! five whose circulars the operator published ship `ReplacementBlocks`
//! restating the printed special-session schedules, and the nine whose
//! instants it has not published are `Unsourced`; the fence pins both the
//! withheld shape (the date refuses rather than answers) and the replacement
//! shape (the day trades exactly inside the printed blocks), alongside the
//! closure shape (the day answers shut, end-exclusively around the sessions
//! that do run).
//!
//! Coverage horizons are the operators' own: JPX publishes the current and
//! next year (2025-2027 audited), SSE its annual December notice (2025-2026),
//! and NSE its annual list (2025-2026). The fences hold each table to its
//! window: a date outside it with only one window flank refuses, while a
//! span **between** two windows is the charter's bridged residual (#296) —
//! the session layer answers from the sourced normal week, the holiday layer
//! stays absent, and the metadata reports the residual.

use chrono::{Days, TimeDelta};
use chrono_tz::Asia;
use exchange_hours::{
    CalendarCoverage, CalendarQueryError, CoverageGapReason, DateCoverage, EvidenceTier, Exchange,
    ExchangeCalendar, Holiday, HolidayKind, SessionKind, calendar_for_exchange,
};

use super::prelude::zoned;

fn day(year: i32, month: u32, date: u32) -> chrono::NaiveDate {
    chrono::NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
}

/// The reason an identity's metadata reports for `date`, taken from the
/// reported gap spans.
fn gap_reason_on(coverage: CalendarCoverage, date: chrono::NaiveDate) -> Option<CoverageGapReason> {
    coverage
        .gaps()
        .find(|gap| gap.range().contains(date))
        .map(exchange_hours::CoverageGap::reason)
}

/// Asserts the row `date` ships is a `Closed` row citing `document` at T1,
/// and that the whole trade date answers shut. The exceptions are the
/// window's own first dates: settling that trade date reads the day before
/// it, which sits outside the audited window (or below the 2010 support
/// floor), so the query refuses there — the same shape the CFE fence
/// documents for its New Year row.
fn assert_closure(calendar: ExchangeCalendar, date: (i32, u32, u32), document: &str, label: &str) {
    assert_closure_at(calendar, date, document, EvidenceTier::T1, label);
}

/// The same fence at an explicit tier (the NSE 2023-2024 rows are T2).
#[expect(
    clippy::panic,
    reason = "a shared test helper is not itself a #[test], so the \
              allow-panic-in-tests switch does not see it; aborting on a \
              broken fixture is the test's job"
)]
fn assert_closure_at(
    calendar: ExchangeCalendar,
    date: (i32, u32, u32),
    document: &str,
    tier: EvidenceTier,
    label: &str,
) {
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
    assert_eq!(row.tier(), tier, "{label}: {trade_date} tier");
    assert_eq!(
        row.document_id(),
        document,
        "{label}: {trade_date} document id"
    );
    // Settling a trade-date question scans backward to the previous session;
    // around New Year that scan walks off the window's first day (2025-01-01,
    // 01-02 and 01-03 are all rows and the prior session sits in 2024, while
    // the new window start 2010-01-01..01-03 scans to pre-floor 2009-12-31),
    // and beside a withheld Muhurat date it walks onto that withheld day. The
    // contract is to refuse naming the day the scan needed: accepted only when
    // the error names a day before the window or the floor, or a day the table
    // withholds. Everywhere else the date must answer shut.
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
        Err(CalendarQueryError::BeforeSupportFloor { date, .. }) => {
            let first = calendar
                .holiday_coverage()
                .map(exchange_hours::HolidayCoverage::first)
                .expect("each of these venues ships a table");
            assert!(
                date < first,
                "{label}: {trade_date} refused naming in-window {date}; only a scan that                  exits the window start may refuse"
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
fn assert_unsourced(
    calendar: ExchangeCalendar,
    date: (i32, u32, u32),
    document: &str,
    label: &str,
) {
    assert_unsourced_at(calendar, date, document, EvidenceTier::T1, label);
}

/// The same withholding fence at an explicit tier (the NSE 2023-2024 rows are T2).
#[expect(
    clippy::panic,
    reason = "a shared test helper is not itself a #[test], so the \
              allow-panic-in-tests switch does not see it; aborting on a \
              broken fixture is the test's job"
)]
fn assert_unsourced_at(
    calendar: ExchangeCalendar,
    date: (i32, u32, u32),
    document: &str,
    tier: EvidenceTier,
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
    assert_eq!(row.tier(), tier, "{label}: {trade_date} tier");
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

/// Asserts the 2010-2014 TSE closure rows against the operator's own archived
/// holiday pages, one edition per calendar year (the TSE-era page `tse.or.jp/english/about/calendar.html`). The window-start rows scan back
/// below the 2010 floor and may refuse naming that day, which the helper accepts.
/// The date list is spelled here independently of the module: a row that
/// moves, loses its clip or changes its citation fails here.
#[test]
fn tse_printed_closures_2010_2014_ship_a_row_per_year() {
    let tse = calendar_for_exchange(Exchange::Tse);
    for (date, document, label) in [
        // 2010, from the operator's TSE-CAL-2010 edition.
        ((2010, 1, 1), "TSE-CAL-2010", "New Year's Day"),
        ((2010, 1, 11), "TSE-CAL-2010", "Coming of Age Day"),
        ((2010, 2, 11), "TSE-CAL-2010", "National Foundation Day"),
        ((2010, 3, 22), "TSE-CAL-2010", "Holiday"),
        ((2010, 4, 29), "TSE-CAL-2010", "Showa Day"),
        ((2010, 5, 3), "TSE-CAL-2010", "Constitution Memorial Day"),
        ((2010, 5, 4), "TSE-CAL-2010", "Greenery Day"),
        ((2010, 5, 5), "TSE-CAL-2010", "Children's Day"),
        ((2010, 7, 19), "TSE-CAL-2010", "Marine Day"),
        ((2010, 9, 20), "TSE-CAL-2010", "Respect for the Aged Day"),
        ((2010, 9, 23), "TSE-CAL-2010", "Autumnal equinox"),
        ((2010, 10, 11), "TSE-CAL-2010", "Health and Sports Day"),
        ((2010, 11, 3), "TSE-CAL-2010", "Culture Day"),
        ((2010, 11, 23), "TSE-CAL-2010", "Labor Thanksgiving Day"),
        ((2010, 12, 23), "TSE-CAL-2010", "Emperor's Birthday"),
        ((2010, 12, 31), "TSE-CAL-2010", "Market Holiday"),
        // 2011, from the operator's TSE-CAL-2011 edition.
        ((2011, 1, 3), "TSE-CAL-2011", "Market Holiday"),
        ((2011, 1, 10), "TSE-CAL-2011", "Coming of Age Day"),
        ((2011, 2, 11), "TSE-CAL-2011", "National Foundation Day"),
        ((2011, 3, 21), "TSE-CAL-2011", "Vernal Equinox"),
        ((2011, 4, 29), "TSE-CAL-2011", "Showa Day"),
        ((2011, 5, 3), "TSE-CAL-2011", "Constitution Memorial Day"),
        ((2011, 5, 4), "TSE-CAL-2011", "Greenery Day"),
        ((2011, 5, 5), "TSE-CAL-2011", "Children's Day"),
        ((2011, 7, 18), "TSE-CAL-2011", "Marine Day"),
        ((2011, 9, 19), "TSE-CAL-2011", "Respect for the Aged Day"),
        ((2011, 9, 23), "TSE-CAL-2011", "Autumnal equinox"),
        ((2011, 10, 10), "TSE-CAL-2011", "Health and Sports Day"),
        ((2011, 11, 3), "TSE-CAL-2011", "Culture Day"),
        ((2011, 11, 23), "TSE-CAL-2011", "Labor Thanksgiving Day"),
        ((2011, 12, 23), "TSE-CAL-2011", "Emperor's Birthday"),
        // 2012, from the operator's TSE-CAL-2012 edition.
        ((2012, 1, 2), "TSE-CAL-2012", "Holiday"),
        ((2012, 1, 3), "TSE-CAL-2012", "Exchange Holiday"),
        ((2012, 1, 9), "TSE-CAL-2012", "Coming of Age Day"),
        ((2012, 3, 20), "TSE-CAL-2012", "Vernal Equinox"),
        ((2012, 4, 30), "TSE-CAL-2012", "Holiday"),
        ((2012, 5, 3), "TSE-CAL-2012", "Constitution Memorial Day"),
        ((2012, 5, 4), "TSE-CAL-2012", "Greenery Day"),
        ((2012, 7, 16), "TSE-CAL-2012", "Marine Day"),
        ((2012, 9, 17), "TSE-CAL-2012", "Respect for the Aged Day"),
        ((2012, 10, 8), "TSE-CAL-2012", "Health and Sports Day"),
        ((2012, 11, 23), "TSE-CAL-2012", "Labor Thanksgiving Day"),
        ((2012, 12, 24), "TSE-CAL-2012", "Holiday"),
        ((2012, 12, 31), "TSE-CAL-2012", "Exchange Holiday"),
        // 2013, from the operator's TSE-CAL-2013 edition.
        ((2013, 1, 1), "TSE-CAL-2013", "New Year's Day"),
        ((2013, 1, 2), "TSE-CAL-2013", "Exchange Holiday"),
        ((2013, 1, 3), "TSE-CAL-2013", "Exchange Holiday"),
        ((2013, 1, 14), "TSE-CAL-2013", "Coming of Age Day"),
        ((2013, 2, 11), "TSE-CAL-2013", "National Foundation Day"),
        ((2013, 3, 20), "TSE-CAL-2013", "Vernal Equinox"),
        ((2013, 4, 29), "TSE-CAL-2013", "Showa Day"),
        ((2013, 5, 3), "TSE-CAL-2013", "Constitution Memorial Day"),
        ((2013, 5, 6), "TSE-CAL-2013", "Holiday"),
        ((2013, 7, 15), "TSE-CAL-2013", "Marine Day"),
        ((2013, 9, 16), "TSE-CAL-2013", "Respect for the Aged Day"),
        ((2013, 9, 23), "TSE-CAL-2013", "Autumnal equinox"),
        ((2013, 10, 14), "TSE-CAL-2013", "Health and Sports Day"),
        ((2013, 11, 4), "TSE-CAL-2013", "Holiday"),
        ((2013, 12, 23), "TSE-CAL-2013", "Emperor's Birthday"),
        ((2013, 12, 31), "TSE-CAL-2013", "Exchange Holiday"),
        // 2014, from the operator's TSE-CAL-2014 edition.
        ((2014, 1, 1), "TSE-CAL-2014", "New Year's Day"),
        ((2014, 1, 2), "TSE-CAL-2014", "Exchange Holiday"),
        ((2014, 1, 3), "TSE-CAL-2014", "Exchange Holiday"),
        ((2014, 1, 13), "TSE-CAL-2014", "Coming of Age Day"),
        ((2014, 2, 11), "TSE-CAL-2014", "National Foundation Day"),
        ((2014, 3, 21), "TSE-CAL-2014", "Vernal Equinox"),
        ((2014, 4, 29), "TSE-CAL-2014", "Showa Day"),
        ((2014, 5, 5), "TSE-CAL-2014", "Children's Day"),
        ((2014, 5, 6), "TSE-CAL-2014", "Holiday"),
        ((2014, 7, 21), "TSE-CAL-2014", "Marine Day"),
        ((2014, 9, 15), "TSE-CAL-2014", "Respect for the Aged Day"),
        ((2014, 9, 23), "TSE-CAL-2014", "Autumnal equinox"),
        ((2014, 10, 13), "TSE-CAL-2014", "Health and Sports Day"),
        ((2014, 11, 3), "TSE-CAL-2014", "Culture Day"),
        ((2014, 11, 24), "TSE-CAL-2014", "Holiday"),
        ((2014, 12, 23), "TSE-CAL-2014", "Emperor's Birthday"),
        ((2014, 12, 31), "TSE-CAL-2014", "Exchange Holiday"),
    ] {
        assert_closure(tse, date, document, &format!("TSE {label}"));
    }
}

/// Asserts the 2015-2019 TSE closure rows against the operator's own archived
/// holiday pages, one edition per calendar year (JPX's `english/corporate/calendar/` for 2015-2017 and today's `about-jpx/calendar/` page from 2018).
/// The date list is spelled here independently of the module: a row that
/// moves, loses its clip or changes its citation fails here.
#[test]
fn tse_printed_closures_2015_2019_ship_a_row_per_year() {
    let tse = calendar_for_exchange(Exchange::Tse);
    for (date, document, label) in [
        // 2015, from the operator's JPX-HOL-2015 edition.
        ((2015, 1, 1), "JPX-HOL-2015", "New Year's Day"),
        ((2015, 1, 2), "JPX-HOL-2015", "Exchange Holiday"),
        ((2015, 1, 12), "JPX-HOL-2015", "Coming of Age Day"),
        ((2015, 2, 11), "JPX-HOL-2015", "National Foundation Day"),
        ((2015, 4, 29), "JPX-HOL-2015", "Showa Day"),
        ((2015, 5, 4), "JPX-HOL-2015", "Greenery Day"),
        ((2015, 5, 5), "JPX-HOL-2015", "Children's Day"),
        ((2015, 5, 6), "JPX-HOL-2015", "Holiday"),
        ((2015, 7, 20), "JPX-HOL-2015", "Marine Day"),
        ((2015, 9, 21), "JPX-HOL-2015", "Respect for the Aged Day"),
        ((2015, 9, 22), "JPX-HOL-2015", "Holiday"),
        ((2015, 9, 23), "JPX-HOL-2015", "Autumnal equinox"),
        ((2015, 10, 12), "JPX-HOL-2015", "Health and Sports Day"),
        ((2015, 11, 3), "JPX-HOL-2015", "Culture Day"),
        ((2015, 11, 23), "JPX-HOL-2015", "Labor Thanksgiving Day"),
        ((2015, 12, 23), "JPX-HOL-2015", "Emperor's Birthday"),
        ((2015, 12, 31), "JPX-HOL-2015", "Exchange Holiday"),
        // 2016, from the operator's JPX-HOL-2016 edition.
        ((2016, 1, 1), "JPX-HOL-2016", "New Year's Day"),
        ((2016, 1, 11), "JPX-HOL-2016", "Coming of Age Day"),
        ((2016, 2, 11), "JPX-HOL-2016", "National Foundation Day"),
        ((2016, 3, 21), "JPX-HOL-2016", "Holiday"),
        ((2016, 4, 29), "JPX-HOL-2016", "Showa Day"),
        ((2016, 5, 3), "JPX-HOL-2016", "Constitution Memorial Day"),
        ((2016, 5, 4), "JPX-HOL-2016", "Greenery Day"),
        ((2016, 5, 5), "JPX-HOL-2016", "Children's Day"),
        ((2016, 7, 18), "JPX-HOL-2016", "Marine Day"),
        ((2016, 8, 11), "JPX-HOL-2016", "Mountain Day"),
        ((2016, 9, 19), "JPX-HOL-2016", "Respect for the Aged Day"),
        ((2016, 9, 22), "JPX-HOL-2016", "Autumnal equinox"),
        ((2016, 10, 10), "JPX-HOL-2016", "Health and Sports Day"),
        ((2016, 11, 3), "JPX-HOL-2016", "Culture Day"),
        ((2016, 11, 23), "JPX-HOL-2016", "Labor Thanksgiving Day"),
        ((2016, 12, 23), "JPX-HOL-2016", "Emperor's Birthday"),
        // 2017, from the operator's JPX-HOL-2017 edition.
        ((2017, 1, 2), "JPX-HOL-2017", "Holiday"),
        ((2017, 1, 3), "JPX-HOL-2017", "Exchange Holiday"),
        ((2017, 1, 9), "JPX-HOL-2017", "Coming of Age Day"),
        ((2017, 3, 20), "JPX-HOL-2017", "Vernal Equinox"),
        ((2017, 5, 3), "JPX-HOL-2017", "Constitution Memorial Day"),
        ((2017, 5, 4), "JPX-HOL-2017", "Greenery Day"),
        ((2017, 5, 5), "JPX-HOL-2017", "Children's Day"),
        ((2017, 7, 17), "JPX-HOL-2017", "Marine Day"),
        ((2017, 8, 11), "JPX-HOL-2017", "Mountain Day"),
        ((2017, 9, 18), "JPX-HOL-2017", "Respect for the Aged Day"),
        ((2017, 10, 9), "JPX-HOL-2017", "Health and Sports Day"),
        ((2017, 11, 3), "JPX-HOL-2017", "Culture Day"),
        ((2017, 11, 23), "JPX-HOL-2017", "Labor Thanksgiving Day"),
        // 2018, from the operator's JPX-HOL-2018 edition.
        ((2018, 1, 1), "JPX-HOL-2018", "New Year's Day"),
        ((2018, 1, 2), "JPX-HOL-2018", "Market Holiday"),
        ((2018, 1, 3), "JPX-HOL-2018", "Market Holiday"),
        ((2018, 1, 8), "JPX-HOL-2018", "Coming of Age Day"),
        ((2018, 2, 12), "JPX-HOL-2018", "National Foundation Day"),
        ((2018, 3, 21), "JPX-HOL-2018", "Vernal Equinox"),
        ((2018, 4, 30), "JPX-HOL-2018", "Showa Day"),
        ((2018, 5, 3), "JPX-HOL-2018", "Constitution Memorial Day"),
        ((2018, 5, 4), "JPX-HOL-2018", "Greenery Day"),
        ((2018, 7, 16), "JPX-HOL-2018", "Marine Day"),
        ((2018, 9, 17), "JPX-HOL-2018", "Respect for the Aged Day"),
        ((2018, 9, 24), "JPX-HOL-2018", "Autumnal Equinox"),
        ((2018, 10, 8), "JPX-HOL-2018", "Health and Sports Day"),
        ((2018, 11, 23), "JPX-HOL-2018", "Labor Thanksgiving Day"),
        ((2018, 12, 24), "JPX-HOL-2018", "Emperor's Birthday"),
        ((2018, 12, 31), "JPX-HOL-2018", "Market Holiday"),
        // 2019, from the operator's JPX-HOL-2019 edition.
        ((2019, 1, 1), "JPX-HOL-2019", "New Year's Day"),
        ((2019, 1, 2), "JPX-HOL-2019", "Market Holiday"),
        ((2019, 1, 3), "JPX-HOL-2019", "Market Holiday"),
        ((2019, 1, 14), "JPX-HOL-2019", "Coming of Age Day"),
        ((2019, 2, 11), "JPX-HOL-2019", "National Foundation Day"),
        ((2019, 3, 21), "JPX-HOL-2019", "Vernal Equinox"),
        ((2019, 4, 29), "JPX-HOL-2019", "Showa Day"),
        ((2019, 4, 30), "JPX-HOL-2019", "Abdication Day"),
        ((2019, 5, 1), "JPX-HOL-2019", "Accession Day"),
        ((2019, 5, 2), "JPX-HOL-2019", "National Holiday"),
        ((2019, 5, 3), "JPX-HOL-2019", "Constitution Memorial Day"),
        ((2019, 5, 6), "JPX-HOL-2019", "Children's Day"),
        ((2019, 7, 15), "JPX-HOL-2019", "Marine Day"),
        ((2019, 8, 12), "JPX-HOL-2019", "Mountain Day"),
        ((2019, 9, 16), "JPX-HOL-2019", "Respect for the Aged Day"),
        ((2019, 9, 23), "JPX-HOL-2019", "Autumnal Equinox"),
        ((2019, 10, 14), "JPX-HOL-2019", "Health and Sports Day"),
        ((2019, 10, 22), "JPX-HOL-2019", "Enthronement Ceremony Day"),
        ((2019, 11, 4), "JPX-HOL-2019", "Culture Day"),
        ((2019, 12, 31), "JPX-HOL-2019", "Market Holiday"),
    ] {
        assert_closure(tse, date, document, &format!("TSE {label}"));
    }
}

/// Asserts the 2020-2024 TSE closure rows against the operator's own archived
/// holiday pages, one edition per calendar year (JPX's `about-jpx/calendar/` page, the Tokyo-Olympics year included).
/// The date list is spelled here independently of the module: a row that
/// moves, loses its clip or changes its citation fails here.
#[test]
fn tse_printed_closures_2020_2024_ship_a_row_per_year() {
    let tse = calendar_for_exchange(Exchange::Tse);
    for (date, document, label) in [
        // 2020, from the operator's JPX-HOL-2020 edition.
        ((2020, 1, 1), "JPX-HOL-2020", "New Year's Day"),
        ((2020, 1, 2), "JPX-HOL-2020", "Market Holiday"),
        ((2020, 1, 3), "JPX-HOL-2020", "Market Holiday"),
        ((2020, 1, 13), "JPX-HOL-2020", "Coming of Age Day"),
        ((2020, 2, 11), "JPX-HOL-2020", "National Foundation Day"),
        ((2020, 2, 24), "JPX-HOL-2020", "Emperor's Birthday"),
        ((2020, 3, 20), "JPX-HOL-2020", "Vernal Equinox"),
        ((2020, 4, 29), "JPX-HOL-2020", "Showa Day"),
        ((2020, 5, 4), "JPX-HOL-2020", "Greenery Day"),
        ((2020, 5, 5), "JPX-HOL-2020", "Children's Day"),
        ((2020, 5, 6), "JPX-HOL-2020", "Constitution Memorial Day"),
        ((2020, 7, 23), "JPX-HOL-2020", "Marine Day"),
        ((2020, 7, 24), "JPX-HOL-2020", "Sports Day"),
        ((2020, 8, 10), "JPX-HOL-2020", "Mountain Day"),
        ((2020, 9, 21), "JPX-HOL-2020", "Respect for the Aged Day"),
        ((2020, 9, 22), "JPX-HOL-2020", "Autumnal Equinox"),
        ((2020, 11, 3), "JPX-HOL-2020", "Culture Day"),
        ((2020, 11, 23), "JPX-HOL-2020", "Labor Thanksgiving Day"),
        ((2020, 12, 31), "JPX-HOL-2020", "Market Holiday"),
        // 2021, from the operator's JPX-HOL-2021 edition.
        ((2021, 1, 1), "JPX-HOL-2021", "New Year's Day"),
        ((2021, 1, 11), "JPX-HOL-2021", "Coming of Age Day"),
        ((2021, 2, 11), "JPX-HOL-2021", "National Foundation Day"),
        ((2021, 2, 23), "JPX-HOL-2021", "Emperor's Birthday"),
        ((2021, 4, 29), "JPX-HOL-2021", "Showa Day"),
        ((2021, 5, 3), "JPX-HOL-2021", "Constitution Memorial Day"),
        ((2021, 5, 4), "JPX-HOL-2021", "Greenery Day"),
        ((2021, 5, 5), "JPX-HOL-2021", "Children's Day"),
        ((2021, 7, 22), "JPX-HOL-2021", "Marine Day"),
        ((2021, 7, 23), "JPX-HOL-2021", "Sports Day"),
        ((2021, 8, 9), "JPX-HOL-2021", "Mountain Day"),
        ((2021, 9, 20), "JPX-HOL-2021", "Respect for the Aged Day"),
        ((2021, 9, 23), "JPX-HOL-2021", "Autumnal Equinox"),
        ((2021, 11, 3), "JPX-HOL-2021", "Culture Day"),
        ((2021, 11, 23), "JPX-HOL-2021", "Labor Thanksgiving Day"),
        ((2021, 12, 31), "JPX-HOL-2021", "Market Holiday"),
        // 2022, from the operator's JPX-HOL-2022 edition.
        ((2022, 1, 3), "JPX-HOL-2022", "Market Holiday"),
        ((2022, 1, 10), "JPX-HOL-2022", "Coming of Age Day"),
        ((2022, 2, 11), "JPX-HOL-2022", "National Foundation Day"),
        ((2022, 2, 23), "JPX-HOL-2022", "Emperor's Birthday"),
        ((2022, 3, 21), "JPX-HOL-2022", "Vernal Equinox"),
        ((2022, 4, 29), "JPX-HOL-2022", "Showa Day"),
        ((2022, 5, 3), "JPX-HOL-2022", "Constitution Memorial Day"),
        ((2022, 5, 4), "JPX-HOL-2022", "Greenery Day"),
        ((2022, 5, 5), "JPX-HOL-2022", "Children's Day"),
        ((2022, 7, 18), "JPX-HOL-2022", "Marine Day"),
        ((2022, 8, 11), "JPX-HOL-2022", "Mountain Day"),
        ((2022, 9, 19), "JPX-HOL-2022", "Respect for the Aged Day"),
        ((2022, 9, 23), "JPX-HOL-2022", "Autumnal Equinox"),
        ((2022, 10, 10), "JPX-HOL-2022", "Sports Day"),
        ((2022, 11, 3), "JPX-HOL-2022", "Culture Day"),
        ((2022, 11, 23), "JPX-HOL-2022", "Labor Thanksgiving Day"),
        // 2023, from the operator's JPX-HOL-2023 edition.
        ((2023, 5, 3), "JPX-HOL-2023", "Constitution Memorial Day"),
        ((2023, 5, 4), "JPX-HOL-2023", "Greenery Day"),
        ((2023, 1, 2), "JPX-HOL-2023", "New Year's Day"),
        ((2023, 1, 3), "JPX-HOL-2023", "Market Holiday"),
        ((2023, 1, 9), "JPX-HOL-2023", "Coming of Age Day"),
        ((2023, 2, 23), "JPX-HOL-2023", "Emperor's Birthday"),
        ((2023, 3, 21), "JPX-HOL-2023", "Vernal Equinox"),
        ((2023, 5, 5), "JPX-HOL-2023", "Children's Day"),
        ((2023, 7, 17), "JPX-HOL-2023", "Marine Day"),
        ((2023, 8, 11), "JPX-HOL-2023", "Mountain Day"),
        ((2023, 9, 18), "JPX-HOL-2023", "Respect for the Aged Day"),
        ((2023, 10, 9), "JPX-HOL-2023", "Sports Day"),
        ((2023, 11, 3), "JPX-HOL-2023", "Culture Day"),
        ((2023, 11, 23), "JPX-HOL-2023", "Labor Thanksgiving Day"),
        // 2024, from the operator's JPX-HOL-2024 edition.
        ((2024, 1, 1), "JPX-HOL-2024", "New Year's Day"),
        ((2024, 1, 2), "JPX-HOL-2024", "Market Holiday"),
        ((2024, 1, 3), "JPX-HOL-2024", "Market Holiday"),
        ((2024, 1, 8), "JPX-HOL-2024", "Coming of Age Day"),
        ((2024, 2, 12), "JPX-HOL-2024", "National Foundation Day"),
        ((2024, 2, 23), "JPX-HOL-2024", "Emperor's Birthday"),
        ((2024, 3, 20), "JPX-HOL-2024", "Vernal Equinox"),
        ((2024, 4, 29), "JPX-HOL-2024", "Showa Day"),
        ((2024, 5, 3), "JPX-HOL-2024", "Constitution Memorial Day"),
        ((2024, 5, 6), "JPX-HOL-2024", "Children's Day"),
        ((2024, 7, 15), "JPX-HOL-2024", "Marine Day"),
        ((2024, 8, 12), "JPX-HOL-2024", "Mountain Day"),
        ((2024, 9, 16), "JPX-HOL-2024", "Respect for the Aged Day"),
        ((2024, 9, 23), "JPX-HOL-2024", "Autumnal Equinox"),
        ((2024, 10, 14), "JPX-HOL-2024", "Sports Day"),
        ((2024, 11, 4), "JPX-HOL-2024", "Culture Day"),
        ((2024, 12, 31), "JPX-HOL-2024", "Market Holiday"),
    ] {
        assert_closure(tse, date, document, &format!("TSE {label}"));
    }
}

/// Asserts every 2025-2027 TSE closure row, per year, against the operator's
/// printed tables. The date list is spelled here independently of the module:
/// a row that moves, loses its clip or changes its citation fails here.
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

/// Asserts the 2011-2014 SSE closure rows, per year, against the operator's own
/// annual closure-arrangement notices (the 2011-2013 notices live from the operator's media-center reprints, the 2014 notice from its Wayback replay). The date list is spelled here
/// independently of the module: a row that moves, loses its clip or changes
/// its citation fails here.
#[test]
fn sse_printed_closures_2011_2014_ship_a_row_per_year() {
    let sse = calendar_for_exchange(Exchange::Sse);
    for (date, document, label) in [
        // 2011, from the operator's 2011 arrangement notice.
        ((2011, 1, 3), "SSE-NOTICE-2011", "元旦"),
        ((2011, 2, 2), "SSE-NOTICE-2011", "春节"),
        ((2011, 2, 3), "SSE-NOTICE-2011", "春节"),
        ((2011, 2, 4), "SSE-NOTICE-2011", "春节"),
        ((2011, 2, 7), "SSE-NOTICE-2011", "春节"),
        ((2011, 2, 8), "SSE-NOTICE-2011", "春节"),
        ((2011, 4, 4), "SSE-NOTICE-2011", "清明节"),
        ((2011, 4, 5), "SSE-NOTICE-2011", "清明节"),
        ((2011, 5, 2), "SSE-NOTICE-2011", "劳动节"),
        ((2011, 6, 6), "SSE-NOTICE-2011", "端午节"),
        ((2011, 9, 12), "SSE-NOTICE-2011", "中秋节"),
        ((2011, 10, 3), "SSE-NOTICE-2011", "国庆节"),
        ((2011, 10, 4), "SSE-NOTICE-2011", "国庆节"),
        ((2011, 10, 5), "SSE-NOTICE-2011", "国庆节"),
        ((2011, 10, 6), "SSE-NOTICE-2011", "国庆节"),
        ((2011, 10, 7), "SSE-NOTICE-2011", "国庆节"),
        // 2012, from the operator's 2012 arrangement notice.
        ((2012, 1, 2), "SSE-NOTICE-2012", "元旦"),
        ((2012, 1, 3), "SSE-NOTICE-2012", "元旦"),
        ((2012, 1, 23), "SSE-NOTICE-2012", "春节"),
        ((2012, 1, 24), "SSE-NOTICE-2012", "春节"),
        ((2012, 1, 25), "SSE-NOTICE-2012", "春节"),
        ((2012, 1, 26), "SSE-NOTICE-2012", "春节"),
        ((2012, 1, 27), "SSE-NOTICE-2012", "春节"),
        ((2012, 4, 2), "SSE-NOTICE-2012", "清明节"),
        ((2012, 4, 3), "SSE-NOTICE-2012", "清明节"),
        ((2012, 4, 4), "SSE-NOTICE-2012", "清明节"),
        ((2012, 4, 30), "SSE-NOTICE-2012", "劳动节"),
        ((2012, 5, 1), "SSE-NOTICE-2012", "劳动节"),
        ((2012, 6, 22), "SSE-NOTICE-2012", "端午节"),
        ((2012, 10, 1), "SSE-NOTICE-2012", "中秋节、国庆节"),
        ((2012, 10, 2), "SSE-NOTICE-2012", "中秋节、国庆节"),
        ((2012, 10, 3), "SSE-NOTICE-2012", "中秋节、国庆节"),
        ((2012, 10, 4), "SSE-NOTICE-2012", "中秋节、国庆节"),
        ((2012, 10, 5), "SSE-NOTICE-2012", "中秋节、国庆节"),
        // 2013, from the operator's 2013 arrangement notice.
        ((2013, 1, 1), "SSE-NOTICE-2013", "元旦"),
        ((2013, 1, 2), "SSE-NOTICE-2013", "元旦"),
        ((2013, 1, 3), "SSE-NOTICE-2013", "元旦"),
        ((2013, 2, 11), "SSE-NOTICE-2013", "春节"),
        ((2013, 2, 12), "SSE-NOTICE-2013", "春节"),
        ((2013, 2, 13), "SSE-NOTICE-2013", "春节"),
        ((2013, 2, 14), "SSE-NOTICE-2013", "春节"),
        ((2013, 2, 15), "SSE-NOTICE-2013", "春节"),
        ((2013, 4, 4), "SSE-NOTICE-2013", "清明节"),
        ((2013, 4, 5), "SSE-NOTICE-2013", "清明节"),
        ((2013, 4, 29), "SSE-NOTICE-2013", "劳动节"),
        ((2013, 4, 30), "SSE-NOTICE-2013", "劳动节"),
        ((2013, 5, 1), "SSE-NOTICE-2013", "劳动节"),
        ((2013, 6, 10), "SSE-NOTICE-2013", "端午节"),
        ((2013, 6, 11), "SSE-NOTICE-2013", "端午节"),
        ((2013, 6, 12), "SSE-NOTICE-2013", "端午节"),
        ((2013, 9, 19), "SSE-NOTICE-2013", "中秋节"),
        ((2013, 9, 20), "SSE-NOTICE-2013", "中秋节"),
        ((2013, 10, 1), "SSE-NOTICE-2013", "国庆节"),
        ((2013, 10, 2), "SSE-NOTICE-2013", "国庆节"),
        ((2013, 10, 3), "SSE-NOTICE-2013", "国庆节"),
        ((2013, 10, 4), "SSE-NOTICE-2013", "国庆节"),
        ((2013, 10, 7), "SSE-NOTICE-2013", "国庆节"),
        // 2014, from the operator's 2014 arrangement notice.
        ((2014, 1, 1), "SSE-NOTICE-2014", "元旦"),
        ((2014, 1, 31), "SSE-NOTICE-2014", "春节"),
        ((2014, 2, 3), "SSE-NOTICE-2014", "春节"),
        ((2014, 2, 4), "SSE-NOTICE-2014", "春节"),
        ((2014, 2, 5), "SSE-NOTICE-2014", "春节"),
        ((2014, 2, 6), "SSE-NOTICE-2014", "春节"),
        ((2014, 4, 7), "SSE-NOTICE-2014", "清明节"),
        ((2014, 5, 1), "SSE-NOTICE-2014", "劳动节"),
        ((2014, 5, 2), "SSE-NOTICE-2014", "劳动节"),
        ((2014, 6, 2), "SSE-NOTICE-2014", "端午节"),
        ((2014, 9, 8), "SSE-NOTICE-2014", "中秋节"),
        ((2014, 10, 1), "SSE-NOTICE-2014", "国庆节"),
        ((2014, 10, 2), "SSE-NOTICE-2014", "国庆节"),
        ((2014, 10, 3), "SSE-NOTICE-2014", "国庆节"),
        ((2014, 10, 6), "SSE-NOTICE-2014", "国庆节"),
        ((2014, 10, 7), "SSE-NOTICE-2014", "国庆节"),
    ] {
        assert_closure(sse, date, document, &format!("SSE {label}"));
    }
}

/// Asserts the 2015-2019 SSE closure rows, per year, against the operator's own
/// annual closure-arrangement notices (the 上证公告-numbered notices, one per December, from Wayback replays). The date list is spelled here
/// independently of the module: a row that moves, loses its clip or changes
/// its citation fails here.
#[test]
fn sse_printed_closures_2015_2019_ship_a_row_per_year() {
    let sse = calendar_for_exchange(Exchange::Sse);
    for (date, document, label) in [
        // 2015, from the operator's 2015 arrangement notice.
        ((2015, 1, 1), "SSE-NOTICE-2014-15", "元旦"),
        ((2015, 1, 2), "SSE-NOTICE-2014-15", "元旦"),
        ((2015, 2, 18), "SSE-NOTICE-2014-15", "春节"),
        ((2015, 2, 19), "SSE-NOTICE-2014-15", "春节"),
        ((2015, 2, 20), "SSE-NOTICE-2014-15", "春节"),
        ((2015, 2, 23), "SSE-NOTICE-2014-15", "春节"),
        ((2015, 2, 24), "SSE-NOTICE-2014-15", "春节"),
        ((2015, 4, 6), "SSE-NOTICE-2014-15", "清明节"),
        ((2015, 5, 1), "SSE-NOTICE-2014-15", "劳动节"),
        ((2015, 6, 22), "SSE-NOTICE-2014-15", "端午节"),
        ((2015, 10, 1), "SSE-NOTICE-2014-15", "国庆节"),
        ((2015, 10, 2), "SSE-NOTICE-2014-15", "国庆节"),
        ((2015, 10, 5), "SSE-NOTICE-2014-15", "国庆节"),
        ((2015, 10, 6), "SSE-NOTICE-2014-15", "国庆节"),
        ((2015, 10, 7), "SSE-NOTICE-2014-15", "国庆节"),
        // 2016, from the operator's 2016 arrangement notice.
        ((2016, 1, 1), "SSE-NOTICE-2015-36", "元旦"),
        ((2016, 2, 8), "SSE-NOTICE-2015-36", "春节"),
        ((2016, 2, 9), "SSE-NOTICE-2015-36", "春节"),
        ((2016, 2, 10), "SSE-NOTICE-2015-36", "春节"),
        ((2016, 2, 11), "SSE-NOTICE-2015-36", "春节"),
        ((2016, 2, 12), "SSE-NOTICE-2015-36", "春节"),
        ((2016, 4, 4), "SSE-NOTICE-2015-36", "清明节"),
        ((2016, 5, 2), "SSE-NOTICE-2015-36", "劳动节"),
        ((2016, 6, 9), "SSE-NOTICE-2015-36", "端午节"),
        ((2016, 6, 10), "SSE-NOTICE-2015-36", "端午节"),
        ((2016, 9, 15), "SSE-NOTICE-2015-36", "中秋节"),
        ((2016, 9, 16), "SSE-NOTICE-2015-36", "中秋节"),
        ((2016, 10, 3), "SSE-NOTICE-2015-36", "国庆节"),
        ((2016, 10, 4), "SSE-NOTICE-2015-36", "国庆节"),
        ((2016, 10, 5), "SSE-NOTICE-2015-36", "国庆节"),
        ((2016, 10, 6), "SSE-NOTICE-2015-36", "国庆节"),
        ((2016, 10, 7), "SSE-NOTICE-2015-36", "国庆节"),
        // 2017, from the operator's 2017 arrangement notice.
        ((2017, 1, 2), "SSE-NOTICE-2016-25", "元旦"),
        ((2017, 1, 27), "SSE-NOTICE-2016-25", "春节"),
        ((2017, 1, 30), "SSE-NOTICE-2016-25", "春节"),
        ((2017, 1, 31), "SSE-NOTICE-2016-25", "春节"),
        ((2017, 2, 1), "SSE-NOTICE-2016-25", "春节"),
        ((2017, 2, 2), "SSE-NOTICE-2016-25", "春节"),
        ((2017, 4, 3), "SSE-NOTICE-2016-25", "清明节"),
        ((2017, 4, 4), "SSE-NOTICE-2016-25", "清明节"),
        ((2017, 5, 1), "SSE-NOTICE-2016-25", "劳动节"),
        ((2017, 5, 29), "SSE-NOTICE-2016-25", "端午节"),
        ((2017, 5, 30), "SSE-NOTICE-2016-25", "端午节"),
        ((2017, 10, 2), "SSE-NOTICE-2016-25", "中秋节、国庆节"),
        ((2017, 10, 3), "SSE-NOTICE-2016-25", "中秋节、国庆节"),
        ((2017, 10, 4), "SSE-NOTICE-2016-25", "中秋节、国庆节"),
        ((2017, 10, 5), "SSE-NOTICE-2016-25", "中秋节、国庆节"),
        ((2017, 10, 6), "SSE-NOTICE-2016-25", "中秋节、国庆节"),
        // 2018, from the operator's 2018 arrangement notice.
        ((2018, 1, 1), "SSE-NOTICE-2017-26", "元旦"),
        ((2018, 2, 15), "SSE-NOTICE-2017-26", "春节"),
        ((2018, 2, 16), "SSE-NOTICE-2017-26", "春节"),
        ((2018, 2, 19), "SSE-NOTICE-2017-26", "春节"),
        ((2018, 2, 20), "SSE-NOTICE-2017-26", "春节"),
        ((2018, 2, 21), "SSE-NOTICE-2017-26", "春节"),
        ((2018, 4, 5), "SSE-NOTICE-2017-26", "清明节"),
        ((2018, 4, 6), "SSE-NOTICE-2017-26", "清明节"),
        ((2018, 4, 30), "SSE-NOTICE-2017-26", "劳动节"),
        ((2018, 5, 1), "SSE-NOTICE-2017-26", "劳动节"),
        ((2018, 6, 18), "SSE-NOTICE-2017-26", "端午节"),
        ((2018, 9, 24), "SSE-NOTICE-2017-26", "中秋节"),
        ((2018, 10, 1), "SSE-NOTICE-2017-26", "国庆节"),
        ((2018, 10, 2), "SSE-NOTICE-2017-26", "国庆节"),
        ((2018, 10, 3), "SSE-NOTICE-2017-26", "国庆节"),
        ((2018, 10, 4), "SSE-NOTICE-2017-26", "国庆节"),
        ((2018, 10, 5), "SSE-NOTICE-2017-26", "国庆节"),
        ((2018, 12, 31), "SSE-NOTICE-2018-39", "元旦"),
        // 2019, from the operator's 2019 arrangement notice.
        ((2019, 1, 1), "SSE-NOTICE-2018-39", "元旦"),
        ((2019, 2, 4), "SSE-NOTICE-2018-39", "春节"),
        ((2019, 2, 5), "SSE-NOTICE-2018-39", "春节"),
        ((2019, 2, 6), "SSE-NOTICE-2018-39", "春节"),
        ((2019, 2, 7), "SSE-NOTICE-2018-39", "春节"),
        ((2019, 2, 8), "SSE-NOTICE-2018-39", "春节"),
        ((2019, 4, 5), "SSE-NOTICE-2018-39", "清明节"),
        ((2019, 5, 1), "SSE-NOTICE-2018-39", "劳动节"),
        ((2019, 6, 7), "SSE-NOTICE-2018-39", "端午节"),
        ((2019, 9, 13), "SSE-NOTICE-2018-39", "中秋节"),
        ((2019, 10, 1), "SSE-NOTICE-2018-39", "国庆节"),
        ((2019, 10, 2), "SSE-NOTICE-2018-39", "国庆节"),
        ((2019, 10, 3), "SSE-NOTICE-2018-39", "国庆节"),
        ((2019, 10, 4), "SSE-NOTICE-2018-39", "国庆节"),
        ((2019, 10, 7), "SSE-NOTICE-2018-39", "国庆节"),
    ] {
        assert_closure(sse, date, document, &format!("SSE {label}"));
    }
}

/// Asserts the 2020-2024 SSE closure rows, per year, against the operator's own
/// annual closure-arrangement notices (the 2020-2024 notices, one per December). The date list is spelled here
/// independently of the module: a row that moves, loses its clip or changes
/// its citation fails here.
#[test]
fn sse_printed_closures_2020_2024_ship_a_row_per_year() {
    let sse = calendar_for_exchange(Exchange::Sse);
    for (date, document, label) in [
        // 2020, from the operator's 2020 arrangement notice.
        ((2020, 1, 1), "SSE-NOTICE-2019-65", "元旦"),
        ((2020, 1, 24), "SSE-NOTICE-2019-65", "春节"),
        ((2020, 1, 27), "SSE-NOTICE-2019-65", "春节"),
        ((2020, 1, 28), "SSE-NOTICE-2019-65", "春节"),
        ((2020, 1, 29), "SSE-NOTICE-2019-65", "春节"),
        ((2020, 1, 30), "SSE-NOTICE-2019-65", "春节"),
        // 2020-01-31 closes by the COVID extension, not the annual notice.
        ((2020, 1, 31), "SSE-NOTICE-2020-6", "春节延长"),
        ((2020, 4, 6), "SSE-NOTICE-2019-65", "清明节"),
        ((2020, 5, 1), "SSE-NOTICE-2019-65", "劳动节"),
        ((2020, 5, 4), "SSE-NOTICE-2019-65", "劳动节"),
        ((2020, 5, 5), "SSE-NOTICE-2019-65", "劳动节"),
        ((2020, 6, 25), "SSE-NOTICE-2019-65", "端午节"),
        ((2020, 6, 26), "SSE-NOTICE-2019-65", "端午节"),
        ((2020, 10, 1), "SSE-NOTICE-2019-65", "国庆节、中秋节"),
        ((2020, 10, 2), "SSE-NOTICE-2019-65", "国庆节、中秋节"),
        ((2020, 10, 5), "SSE-NOTICE-2019-65", "国庆节、中秋节"),
        ((2020, 10, 6), "SSE-NOTICE-2019-65", "国庆节、中秋节"),
        ((2020, 10, 7), "SSE-NOTICE-2019-65", "国庆节、中秋节"),
        ((2020, 10, 8), "SSE-NOTICE-2019-65", "国庆节、中秋节"),
        // 2021, from the operator's 2021 arrangement notice.
        ((2021, 1, 1), "SSE-NOTICE-2020-48", "元旦"),
        ((2021, 2, 11), "SSE-NOTICE-2020-48", "春节"),
        ((2021, 2, 12), "SSE-NOTICE-2020-48", "春节"),
        ((2021, 2, 15), "SSE-NOTICE-2020-48", "春节"),
        ((2021, 2, 16), "SSE-NOTICE-2020-48", "春节"),
        ((2021, 2, 17), "SSE-NOTICE-2020-48", "春节"),
        ((2021, 4, 5), "SSE-NOTICE-2020-48", "清明节"),
        ((2021, 5, 3), "SSE-NOTICE-2020-48", "劳动节"),
        ((2021, 5, 4), "SSE-NOTICE-2020-48", "劳动节"),
        ((2021, 5, 5), "SSE-NOTICE-2020-48", "劳动节"),
        ((2021, 6, 14), "SSE-NOTICE-2020-48", "端午节"),
        ((2021, 9, 20), "SSE-NOTICE-2020-48", "中秋节"),
        ((2021, 9, 21), "SSE-NOTICE-2020-48", "中秋节"),
        ((2021, 10, 1), "SSE-NOTICE-2020-48", "国庆节"),
        ((2021, 10, 4), "SSE-NOTICE-2020-48", "国庆节"),
        ((2021, 10, 5), "SSE-NOTICE-2020-48", "国庆节"),
        ((2021, 10, 6), "SSE-NOTICE-2020-48", "国庆节"),
        ((2021, 10, 7), "SSE-NOTICE-2020-48", "国庆节"),
        // 2022, from the operator's 2022 arrangement notice.
        ((2022, 1, 3), "SSE-NOTICE-2021-37", "元旦"),
        ((2022, 1, 31), "SSE-NOTICE-2021-37", "春节"),
        ((2022, 2, 1), "SSE-NOTICE-2021-37", "春节"),
        ((2022, 2, 2), "SSE-NOTICE-2021-37", "春节"),
        ((2022, 2, 3), "SSE-NOTICE-2021-37", "春节"),
        ((2022, 2, 4), "SSE-NOTICE-2021-37", "春节"),
        ((2022, 4, 4), "SSE-NOTICE-2021-37", "清明节"),
        ((2022, 4, 5), "SSE-NOTICE-2021-37", "清明节"),
        ((2022, 5, 2), "SSE-NOTICE-2021-37", "劳动节"),
        ((2022, 5, 3), "SSE-NOTICE-2021-37", "劳动节"),
        ((2022, 5, 4), "SSE-NOTICE-2021-37", "劳动节"),
        ((2022, 6, 3), "SSE-NOTICE-2021-37", "端午节"),
        ((2022, 9, 12), "SSE-NOTICE-2021-37", "中秋节"),
        ((2022, 10, 3), "SSE-NOTICE-2021-37", "国庆节"),
        ((2022, 10, 4), "SSE-NOTICE-2021-37", "国庆节"),
        ((2022, 10, 5), "SSE-NOTICE-2021-37", "国庆节"),
        ((2022, 10, 6), "SSE-NOTICE-2021-37", "国庆节"),
        ((2022, 10, 7), "SSE-NOTICE-2021-37", "国庆节"),
        // 2023, from the operator's 2023 arrangement notice.
        ((2023, 1, 2), "SSE-NOTICE-2022-51", "元旦"),
        ((2023, 1, 23), "SSE-NOTICE-2022-51", "春节"),
        ((2023, 1, 24), "SSE-NOTICE-2022-51", "春节"),
        ((2023, 1, 25), "SSE-NOTICE-2022-51", "春节"),
        ((2023, 1, 26), "SSE-NOTICE-2022-51", "春节"),
        ((2023, 1, 27), "SSE-NOTICE-2022-51", "春节"),
        ((2023, 4, 5), "SSE-NOTICE-2022-51", "清明节"),
        ((2023, 5, 1), "SSE-NOTICE-2022-51", "劳动节"),
        ((2023, 5, 2), "SSE-NOTICE-2022-51", "劳动节"),
        ((2023, 5, 3), "SSE-NOTICE-2022-51", "劳动节"),
        ((2023, 6, 22), "SSE-NOTICE-2022-51", "端午节"),
        ((2023, 6, 23), "SSE-NOTICE-2022-51", "端午节"),
        ((2023, 9, 29), "SSE-NOTICE-2022-51", "中秋节、国庆节"),
        ((2023, 10, 2), "SSE-NOTICE-2022-51", "中秋节、国庆节"),
        ((2023, 10, 3), "SSE-NOTICE-2022-51", "中秋节、国庆节"),
        ((2023, 10, 4), "SSE-NOTICE-2022-51", "中秋节、国庆节"),
        ((2023, 10, 5), "SSE-NOTICE-2022-51", "中秋节、国庆节"),
        ((2023, 10, 6), "SSE-NOTICE-2022-51", "中秋节、国庆节"),
        // 2024, from the operator's 2024 arrangement notice.
        ((2024, 1, 1), "SSE-NOTICE-2023-47", "元旦"),
        ((2024, 2, 9), "SSE-NOTICE-2023-47", "春节"),
        ((2024, 2, 12), "SSE-NOTICE-2023-47", "春节"),
        ((2024, 2, 13), "SSE-NOTICE-2023-47", "春节"),
        ((2024, 2, 14), "SSE-NOTICE-2023-47", "春节"),
        ((2024, 2, 15), "SSE-NOTICE-2023-47", "春节"),
        ((2024, 2, 16), "SSE-NOTICE-2023-47", "春节"),
        ((2024, 4, 4), "SSE-NOTICE-2023-47", "清明节"),
        ((2024, 4, 5), "SSE-NOTICE-2023-47", "清明节"),
        ((2024, 5, 1), "SSE-NOTICE-2023-47", "劳动节"),
        ((2024, 5, 2), "SSE-NOTICE-2023-47", "劳动节"),
        ((2024, 5, 3), "SSE-NOTICE-2023-47", "劳动节"),
        ((2024, 6, 10), "SSE-NOTICE-2023-47", "端午节"),
        ((2024, 9, 16), "SSE-NOTICE-2023-47", "中秋节"),
        ((2024, 9, 17), "SSE-NOTICE-2023-47", "中秋节"),
        ((2024, 10, 1), "SSE-NOTICE-2023-47", "国庆节"),
        ((2024, 10, 2), "SSE-NOTICE-2023-47", "国庆节"),
        ((2024, 10, 3), "SSE-NOTICE-2023-47", "国庆节"),
        ((2024, 10, 4), "SSE-NOTICE-2023-47", "国庆节"),
        ((2024, 10, 7), "SSE-NOTICE-2023-47", "国庆节"),
    ] {
        assert_closure(sse, date, document, &format!("SSE {label}"));
    }
}

/// Asserts the 2025-2026 SSE closure rows against 上证公告〔2024〕38号 and
/// 上证公告〔2025〕45号. The date list is spelled here independently of the
/// module: a row that moves, loses its clip or changes its citation fails here.
#[test]
fn sse_printed_closures_2025_2026_ship_a_row_per_year() {
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

/// Asserts the 2010-2017 NSE closure rows, per year, against the operator's
/// own annual circulars and holiday pages. The date list is spelled here
/// independently of the module: a row that moves, loses its clip or changes
/// its citation fails here. 2012 is unrecovered and ships no rows (see the
/// refusal fence below).
/// Asserts the 2010-2017 NSE closure rows, per year, against the operator's
/// own annual holiday material. The date list is spelled here independently of
/// the module: a row that moves, loses its clip or changes its citation fails
/// here.
/// 2012 is unrecovered and ships no rows (see the refusal fence below).
#[test]
fn nse_printed_closures_2010_2017_ship_a_row_per_year() {
    let nse = calendar_for_exchange(Exchange::NseIndia);
    for (date, document, label) in [
        // 2010, from NSE-CIRC-2010-61.
        ((2010, 1, 1), "NSE-CIRC-2010-61", "NSE New Year"),
        ((2010, 1, 26), "NSE-CIRC-2010-61", "NSE Republic Day"),
        ((2010, 2, 12), "NSE-CIRC-2010-61", "NSE Mahashivratri"),
        ((2010, 3, 1), "NSE-CIRC-2010-61", "NSE Holi"),
        ((2010, 3, 24), "NSE-CIRC-2010-61", "NSE Ram Navmi"),
        ((2010, 4, 2), "NSE-CIRC-2010-61", "NSE Good Friday"),
        ((2010, 4, 14), "NSE-CIRC-2010-61", "NSE Ambedkar Jayanti"),
        ((2010, 9, 10), "NSE-CIRC-2010-61", "NSE Ramzan ID"),
        ((2010, 11, 17), "NSE-CIRC-2010-61", "NSE Bakri Id"),
        ((2010, 12, 17), "NSE-CIRC-2010-61", "NSE Moharum"),
        // 2011, from NSE-HOL-PAGE-2011.
        ((2011, 1, 26), "NSE-HOL-PAGE-2011", "NSE Republic Day"),
        ((2011, 3, 2), "NSE-HOL-PAGE-2011", "NSE Mahashivratri"),
        ((2011, 4, 12), "NSE-HOL-PAGE-2011", "NSE Ram Navmi"),
        ((2011, 4, 14), "NSE-HOL-PAGE-2011", "NSE Ambedkar Jayanti"),
        ((2011, 4, 22), "NSE-HOL-PAGE-2011", "NSE Good Friday"),
        ((2011, 8, 15), "NSE-HOL-PAGE-2011", "NSE Independence Day"),
        ((2011, 8, 31), "NSE-HOL-PAGE-2011", "NSE Ramzan ID"),
        ((2011, 9, 1), "NSE-HOL-PAGE-2011", "NSE Ganesh Chaturthi"),
        ((2011, 10, 6), "NSE-HOL-PAGE-2011", "NSE Dasara"),
        (
            (2011, 10, 27),
            "NSE-HOL-PAGE-2011",
            "NSE Diwali-Balipratipada",
        ),
        ((2011, 11, 7), "NSE-HOL-PAGE-2011", "NSE Bakri Id"),
        ((2011, 11, 10), "NSE-HOL-PAGE-2011", "NSE Gurunanak Jayanti"),
        ((2011, 12, 6), "NSE-HOL-PAGE-2011", "NSE Moharum"),
        // 2013, from NSE-CIRC-2012-79.
        ((2013, 3, 27), "NSE-CIRC-2012-79", "NSE Holi"),
        ((2013, 3, 29), "NSE-CIRC-2012-79", "NSE Good Friday"),
        ((2013, 4, 19), "NSE-CIRC-2012-79", "NSE Ram Navmi"),
        ((2013, 4, 24), "NSE-CIRC-2012-79", "NSE Mahavir Jayanti"),
        ((2013, 5, 1), "NSE-CIRC-2012-79", "NSE May Day"),
        ((2013, 8, 9), "NSE-CIRC-2012-79", "NSE Ramzan ID"),
        ((2013, 8, 15), "NSE-CIRC-2012-79", "NSE Independence Day"),
        ((2013, 9, 9), "NSE-CIRC-2012-79", "NSE Ganesh Chaturthi"),
        ((2013, 10, 2), "NSE-CIRC-2012-79", "NSE Gandhi Jayanti"),
        ((2013, 10, 16), "NSE-CIRC-2012-79", "NSE Bakri ID"),
        (
            (2013, 11, 4),
            "NSE-CIRC-2012-79",
            "NSE Diwali-Balipratipada",
        ),
        ((2013, 11, 14), "NSE-CIRC-2012-79", "NSE Moharram"),
        ((2013, 12, 25), "NSE-CIRC-2012-79", "NSE Christmas"),
        // 2014, from NSE-HOL-PAGE-2014.
        ((2014, 2, 27), "NSE-HOL-PAGE-2014", "NSE Mahashivratri"),
        ((2014, 3, 17), "NSE-HOL-PAGE-2014", "NSE Holi"),
        ((2014, 4, 8), "NSE-HOL-PAGE-2014", "NSE Ram Navmi"),
        ((2014, 4, 14), "NSE-HOL-PAGE-2014", "NSE Ambedkar Jayanti"),
        ((2014, 4, 18), "NSE-HOL-PAGE-2014", "NSE Good Friday"),
        ((2014, 5, 1), "NSE-HOL-PAGE-2014", "NSE May Day"),
        ((2014, 7, 29), "NSE-HOL-PAGE-2014", "NSE Ramzan ID"),
        ((2014, 8, 15), "NSE-HOL-PAGE-2014", "NSE Independence Day"),
        ((2014, 8, 29), "NSE-HOL-PAGE-2014", "NSE Ganesh Chaturthi"),
        ((2014, 10, 2), "NSE-HOL-PAGE-2014", "NSE Gandhi Jayanti"),
        ((2014, 10, 3), "NSE-HOL-PAGE-2014", "NSE Dasera"),
        ((2014, 10, 6), "NSE-HOL-PAGE-2014", "NSE Bakri ID"),
        (
            (2014, 10, 24),
            "NSE-HOL-PAGE-2014",
            "NSE Diwali-Balipratipada",
        ),
        ((2014, 11, 4), "NSE-HOL-PAGE-2014", "NSE Moharram"),
        ((2014, 11, 6), "NSE-HOL-PAGE-2014", "NSE Gurunank Jayanti"),
        ((2014, 12, 25), "NSE-HOL-PAGE-2014", "NSE Christmas"),
        // 2015, from NSE-HOL-PAGE-2015.
        ((2015, 1, 26), "NSE-HOL-PAGE-2015", "NSE Republic Day"),
        ((2015, 2, 17), "NSE-HOL-PAGE-2015", "NSE Mahashivratri"),
        ((2015, 3, 6), "NSE-HOL-PAGE-2015", "NSE Holi"),
        ((2015, 4, 2), "NSE-HOL-PAGE-2015", "NSE Mahavir Jayanti"),
        ((2015, 4, 3), "NSE-HOL-PAGE-2015", "NSE Good Friday"),
        ((2015, 4, 14), "NSE-HOL-PAGE-2015", "NSE Ambedkar Jayanti"),
        ((2015, 5, 1), "NSE-HOL-PAGE-2015", "NSE Maharashtra Day"),
        ((2015, 9, 17), "NSE-HOL-PAGE-2015", "NSE Ganesh Chaturthi"),
        ((2015, 9, 25), "NSE-HOL-PAGE-2015", "NSE Bakri ID"),
        ((2015, 10, 2), "NSE-HOL-PAGE-2015", "NSE Gandhi Jayanti"),
        ((2015, 10, 22), "NSE-HOL-PAGE-2015", "NSE Dussehra"),
        (
            (2015, 11, 12),
            "NSE-HOL-PAGE-2015",
            "NSE Diwali-Balipratipada",
        ),
        ((2015, 11, 25), "NSE-HOL-PAGE-2015", "NSE Gurunanak Jayanti"),
        ((2015, 12, 25), "NSE-HOL-PAGE-2015", "NSE Christmas"),
        // 2016, from NSE-HOL-PAGE-2016.
        ((2016, 1, 26), "NSE-HOL-PAGE-2016", "NSE Republic Day"),
        ((2016, 3, 7), "NSE-HOL-PAGE-2016", "NSE Mahashivratri"),
        ((2016, 3, 24), "NSE-HOL-PAGE-2016", "NSE Holi"),
        ((2016, 3, 25), "NSE-HOL-PAGE-2016", "NSE Good Friday"),
        ((2016, 4, 14), "NSE-HOL-PAGE-2016", "NSE Ambedkar Jayanti"),
        ((2016, 4, 15), "NSE-HOL-PAGE-2016", "NSE Ram Navami"),
        ((2016, 4, 19), "NSE-HOL-PAGE-2016", "NSE Mahavir Jayanti"),
        ((2016, 7, 6), "NSE-HOL-PAGE-2016", "NSE Ramzan ID"),
        ((2016, 8, 15), "NSE-HOL-PAGE-2016", "NSE Independence Day"),
        ((2016, 9, 5), "NSE-HOL-PAGE-2016", "NSE Ganesh Chaturthi"),
        ((2016, 9, 13), "NSE-HOL-PAGE-2016", "NSE Bakri ID"),
        ((2016, 10, 11), "NSE-HOL-PAGE-2016", "NSE Dasera"),
        ((2016, 10, 12), "NSE-HOL-PAGE-2016", "NSE Moharram"),
        (
            (2016, 10, 31),
            "NSE-HOL-PAGE-2016",
            "NSE Diwali-Balipratipada",
        ),
        ((2016, 11, 14), "NSE-HOL-PAGE-2016", "NSE Gurunanak Jayanti"),
        // 2017, from NSE-HOL-PAGE-2017.
        ((2017, 1, 26), "NSE-HOL-PAGE-2017", "NSE Republic Day"),
        ((2017, 2, 24), "NSE-HOL-PAGE-2017", "NSE Mahashivratri"),
        ((2017, 3, 13), "NSE-HOL-PAGE-2017", "NSE Holi"),
        ((2017, 4, 4), "NSE-HOL-PAGE-2017", "NSE Ram Navami"),
        (
            (2017, 4, 14),
            "NSE-HOL-PAGE-2017",
            "NSE Ambedkar Jayanti / Good Friday",
        ),
        ((2017, 5, 1), "NSE-HOL-PAGE-2017", "NSE Maharashtra Day"),
        ((2017, 6, 26), "NSE-HOL-PAGE-2017", "NSE Ramzan ID"),
        ((2017, 8, 15), "NSE-HOL-PAGE-2017", "NSE Independence Day"),
        ((2017, 8, 25), "NSE-HOL-PAGE-2017", "NSE Ganesh Chaturthi"),
        ((2017, 10, 2), "NSE-HOL-PAGE-2017", "NSE Gandhi Jayanti"),
        (
            (2017, 10, 20),
            "NSE-HOL-PAGE-2017",
            "NSE Diwali-Balipratipada",
        ),
        ((2017, 12, 25), "NSE-HOL-PAGE-2017", "NSE Christmas"),
    ] {
        assert_closure_at(
            nse,
            date,
            document,
            EvidenceTier::T1,
            &format!("NSE {label}"),
        );
    }
}

/// Asserts the 2019-2022 NSE closure rows, per year, against the operator's
/// own holiday pages. The date list is spelled here independently of the
/// module: a row that moves, loses its clip or changes its citation fails
/// here. 2018 is unrecovered and ships no rows (see the refusal fence below).
#[test]
fn nse_printed_closures_2019_2022_ship_a_row_per_year() {
    let nse = calendar_for_exchange(Exchange::NseIndia);
    for (date, document, label) in [
        // 2019, from NSE-HOL-PAGE-2019.
        ((2019, 3, 4), "NSE-HOL-PAGE-2019", "NSE Mahashivratri"),
        ((2019, 3, 21), "NSE-HOL-PAGE-2019", "NSE Holi"),
        ((2019, 4, 17), "NSE-HOL-PAGE-2019", "NSE Mahavir Jayanti"),
        ((2019, 4, 19), "NSE-HOL-PAGE-2019", "NSE Good Friday"),
        (
            (2019, 4, 29),
            "NSE-HOL-PAGE-2019",
            "NSE Parliamentary Elections",
        ),
        ((2019, 5, 1), "NSE-HOL-PAGE-2019", "NSE Maharashtra Day"),
        ((2019, 6, 5), "NSE-HOL-PAGE-2019", "NSE Ramzan ID"),
        ((2019, 8, 12), "NSE-HOL-PAGE-2019", "NSE Bakri Id"),
        ((2019, 8, 15), "NSE-HOL-PAGE-2019", "NSE Independence Day"),
        ((2019, 9, 2), "NSE-HOL-PAGE-2019", "NSE Ganesh Chaturthi"),
        ((2019, 9, 10), "NSE-HOL-PAGE-2019", "NSE Moharram"),
        ((2019, 10, 2), "NSE-HOL-PAGE-2019", "NSE Gandhi Jayanti"),
        ((2019, 10, 8), "NSE-HOL-PAGE-2019", "NSE Dasera"),
        (
            (2019, 10, 28),
            "NSE-HOL-PAGE-2019",
            "NSE Diwali-Balipratipada",
        ),
        ((2019, 11, 12), "NSE-HOL-PAGE-2019", "NSE Gurunanak Jayanti"),
        ((2019, 12, 25), "NSE-HOL-PAGE-2019", "NSE Christmas"),
        // 2020, from NSE-HOL-PAGE-2020.
        ((2020, 2, 21), "NSE-HOL-PAGE-2020", "NSE Mahashivratri"),
        ((2020, 3, 10), "NSE-HOL-PAGE-2020", "NSE Holi"),
        ((2020, 4, 2), "NSE-HOL-PAGE-2020", "NSE Ram Navami"),
        ((2020, 4, 6), "NSE-HOL-PAGE-2020", "NSE Mahavir Jayanti"),
        ((2020, 4, 10), "NSE-HOL-PAGE-2020", "NSE Good Friday"),
        ((2020, 4, 14), "NSE-HOL-PAGE-2020", "NSE Ambedkar Jayanti"),
        ((2020, 5, 1), "NSE-HOL-PAGE-2020", "NSE Maharashtra Day"),
        ((2020, 5, 25), "NSE-HOL-PAGE-2020", "NSE Ramzan ID"),
        ((2020, 10, 2), "NSE-HOL-PAGE-2020", "NSE Gandhi Jayanti"),
        (
            (2020, 11, 16),
            "NSE-HOL-PAGE-2020",
            "NSE Diwali-Balipratipada",
        ),
        ((2020, 11, 30), "NSE-HOL-PAGE-2020", "NSE Gurunanak Jayanti"),
        ((2020, 12, 25), "NSE-HOL-PAGE-2020", "NSE Christmas"),
        // 2021, from NSE-HOL-PAGE-2021.
        ((2021, 1, 26), "NSE-HOL-PAGE-2021", "NSE Republic Day"),
        ((2021, 3, 11), "NSE-HOL-PAGE-2021", "NSE Mahashivratri"),
        ((2021, 3, 29), "NSE-HOL-PAGE-2021", "NSE Holi"),
        ((2021, 4, 2), "NSE-HOL-PAGE-2021", "NSE Good Friday"),
        ((2021, 4, 14), "NSE-HOL-PAGE-2021", "NSE Ambedkar Jayanti"),
        ((2021, 4, 21), "NSE-HOL-PAGE-2021", "NSE Ram Navami"),
        ((2021, 5, 13), "NSE-HOL-PAGE-2021", "NSE Ramzan ID"),
        ((2021, 7, 21), "NSE-HOL-PAGE-2021", "NSE Bakri Id"),
        ((2021, 8, 19), "NSE-HOL-PAGE-2021", "NSE Moharram"),
        ((2021, 9, 10), "NSE-HOL-PAGE-2021", "NSE Ganesh Chaturthi"),
        ((2021, 10, 15), "NSE-HOL-PAGE-2021", "NSE Dussehra"),
        (
            (2021, 11, 5),
            "NSE-HOL-PAGE-2021",
            "NSE Diwali-Balipratipada",
        ),
        ((2021, 11, 19), "NSE-HOL-PAGE-2021", "NSE Gurunanak Jayanti"),
        // 2022, from NSE-HOL-PAGE-2022.
        ((2022, 1, 26), "NSE-HOL-PAGE-2022", "NSE Republic Day"),
        ((2022, 3, 1), "NSE-HOL-PAGE-2022", "NSE Mahashivratri"),
        ((2022, 3, 18), "NSE-HOL-PAGE-2022", "NSE Holi"),
        (
            (2022, 4, 14),
            "NSE-HOL-PAGE-2022",
            "NSE Ambedkar Jayanti / Mahavir Jayanti",
        ),
        ((2022, 4, 15), "NSE-HOL-PAGE-2022", "NSE Good Friday"),
        ((2022, 5, 3), "NSE-HOL-PAGE-2022", "NSE Ramzan ID"),
        ((2022, 8, 9), "NSE-HOL-PAGE-2022", "NSE Moharram"),
        ((2022, 8, 15), "NSE-HOL-PAGE-2022", "NSE Independence Day"),
        ((2022, 8, 31), "NSE-HOL-PAGE-2022", "NSE Ganesh Chaturthi"),
        ((2022, 10, 5), "NSE-HOL-PAGE-2022", "NSE Dussehra"),
        (
            (2022, 10, 26),
            "NSE-HOL-PAGE-2022",
            "NSE Diwali-Balipratipada",
        ),
        ((2022, 11, 8), "NSE-HOL-PAGE-2022", "NSE Gurunanak Jayanti"),
    ] {
        assert_closure_at(
            nse,
            date,
            document,
            EvidenceTier::T1,
            &format!("NSE {label}"),
        );
    }
}

/// Asserts the 2023-2024 NSE closure rows, per year, against the operator's
/// own holiday-master machine channel (T2 — the only T2 rows in the block).
/// The date list is spelled here independently of the module: a row that
/// moves, loses its clip or changes its citation fails here.
#[test]
fn nse_printed_closures_2023_2024_ship_a_row_per_year() {
    let nse = calendar_for_exchange(Exchange::NseIndia);
    for (date, document, label) in [
        // 2023, from NSE-HOLMASTER-2023.
        ((2023, 1, 26), "NSE-HOLMASTER-2023", "NSE Republic Day"),
        ((2023, 3, 7), "NSE-HOLMASTER-2023", "NSE Holi"),
        ((2023, 3, 30), "NSE-HOLMASTER-2023", "NSE Ram Navami"),
        ((2023, 4, 4), "NSE-HOLMASTER-2023", "NSE Mahavir Jayanti"),
        ((2023, 4, 7), "NSE-HOLMASTER-2023", "NSE Good Friday"),
        ((2023, 4, 14), "NSE-HOLMASTER-2023", "NSE Ambedkar Jayanti"),
        ((2023, 5, 1), "NSE-HOLMASTER-2023", "NSE Maharashtra Day"),
        ((2023, 6, 28), "NSE-HOLMASTER-2023", "NSE Bakri Id"),
        ((2023, 8, 15), "NSE-HOLMASTER-2023", "NSE Independence Day"),
        ((2023, 9, 19), "NSE-HOLMASTER-2023", "NSE Ganesh Chaturthi"),
        ((2023, 10, 2), "NSE-HOLMASTER-2023", "NSE Gandhi Jayanti"),
        ((2023, 10, 24), "NSE-HOLMASTER-2023", "NSE Dussehra"),
        (
            (2023, 11, 14),
            "NSE-HOLMASTER-2023",
            "NSE Diwali-Balipratipada",
        ),
        (
            (2023, 11, 27),
            "NSE-HOLMASTER-2023",
            "NSE Gurunanak Jayanti",
        ),
        ((2023, 12, 25), "NSE-HOLMASTER-2023", "NSE Christmas"),
        // 2024, from NSE-HOLMASTER-2024.
        ((2024, 1, 26), "NSE-HOLMASTER-2024", "NSE Republic Day"),
        ((2024, 3, 8), "NSE-HOLMASTER-2024", "NSE Mahashivratri"),
        ((2024, 3, 25), "NSE-HOLMASTER-2024", "NSE Holi"),
        ((2024, 3, 29), "NSE-HOLMASTER-2024", "NSE Good Friday"),
        ((2024, 4, 11), "NSE-HOLMASTER-2024", "NSE Ramadan Eid"),
        ((2024, 4, 17), "NSE-HOLMASTER-2024", "NSE Ram Navmi"),
        ((2024, 5, 1), "NSE-HOLMASTER-2024", "NSE Maharashtra Day"),
        ((2024, 6, 17), "NSE-HOLMASTER-2024", "NSE Bakri Id"),
        ((2024, 7, 17), "NSE-HOLMASTER-2024", "NSE Moharram"),
        ((2024, 8, 15), "NSE-HOLMASTER-2024", "NSE Independence Day"),
        ((2024, 10, 2), "NSE-HOLMASTER-2024", "NSE Gandhi Jayanti"),
        (
            (2024, 11, 15),
            "NSE-HOLMASTER-2024",
            "NSE Gurunanak Jayanti",
        ),
        ((2024, 12, 25), "NSE-HOLMASTER-2024", "NSE Christmas"),
    ] {
        assert_closure_at(
            nse,
            date,
            document,
            EvidenceTier::T2,
            &format!("NSE {label}"),
        );
    }
}

/// The unrecovered 2018 year is a **bridged residual** (issue #296; the
/// charter's 2026-10-06 convention): the span sits between two audited
/// windows, so the session layer answers from the sourced normal week while
/// the holiday layer stays honestly absent — no row, no audited normal, a
/// metadata verdict that withholds the complete claim, and the span reported
/// as `HolidayWindowsBridged` rather than refused. No closure is asserted
/// that no operator statement witnesses. 2012 sat in the same state until
/// 2026-10-06 UTC, when the 2012 list's own annual circular
/// (`NSE-CIRC-2011-66`, dated 2011-12-09) surfaced and the window merged:
/// today 2012 answers inside its audited window like every sourced year.
#[test]
fn the_unrecovered_2018_year_is_a_bridged_residual_and_2012_was_recovered() {
    let nse = calendar_for_exchange(Exchange::NseIndia);
    for date in [(2018, 3, 12), (2018, 11, 20)] {
        let trade_date = day(date.0, date.1, date.2);
        assert_eq!(
            nse.holiday_on(trade_date),
            None,
            "{trade_date} is inside no audited window, so the table ships no row"
        );
        assert_eq!(
            nse.coverage().coverage_on(trade_date),
            DateCoverage::OutsideCoveredRange,
            "{trade_date} withholds the complete-calendar claim: the holiday \
             layer is the bridged residual, not an audited normal"
        );
        assert_eq!(
            gap_reason_on(nse.coverage(), trade_date),
            Some(CoverageGapReason::HolidayWindowsBridged),
            "{trade_date} reports the bridged residual, not a one-flank gap"
        );
        // The session layer answers from the sourced normal week — the
        // disclosure lives in the metadata above, never in a fabricated
        // closure or a fabricated audited normal.
        assert_eq!(
            nse.is_closed_trade_date(trade_date, SessionKind::Both),
            Ok(false),
            "{trade_date} answers its normal-week session state beside the \
             disclosed residual"
        );
    }
    // The residual span is exactly the unrecovered year, and the one-flank
    // spans beside it are not bridged.
    assert_eq!(
        bridged_spans(nse.coverage()),
        vec![(day(2018, 1, 1), day(2018, 12, 31))],
        "the bridged residual is reported over the unrecovered year"
    );
    // 2012 was recovered the same day: its rows answer inside the merged
    // window (the printed closures of the annual circular), a date with no row
    // is audited complete there, and nothing reports a residual for it.
    assert_eq!(
        gap_reason_on(nse.coverage(), day(2012, 6, 15)),
        None,
        "recovered 2012 reports no residual"
    );
    assert!(nse.coverage().is_complete_on(day(2012, 6, 15)));
    assert_closure(
        nse,
        (2012, 8, 15),
        "NSE-CIRC-2011-66",
        "NSE 2012 Independence Day",
    );
}

/// Returns the **union** of the spans an identity's metadata reports as
/// `HolidayWindowsBridged`, merged where records abut.
///
/// The walk reports the residual over maximal verdict runs, and the static
/// boundary candidates may split one residual span into adjacent runs of the
/// same verdict; the union is the span the charter's convention names. The
/// per-date partition fence in `coverage_metadata.rs` holds every record to
/// the per-date accessor, so merging equal-verdict neighbours here cannot
/// hide a verdict change.
fn bridged_spans(coverage: CalendarCoverage) -> Vec<(chrono::NaiveDate, chrono::NaiveDate)> {
    let mut records: Vec<(chrono::NaiveDate, chrono::NaiveDate)> = coverage
        .gaps()
        .filter(|gap| gap.reason() == CoverageGapReason::HolidayWindowsBridged)
        .map(|gap| (gap.range().first(), gap.range().last()))
        .collect();
    records.sort();
    let mut merged: Vec<(chrono::NaiveDate, chrono::NaiveDate)> = Vec::new();
    for (first, last) in records {
        match merged.last_mut() {
            Some((_, open_last)) if first.pred_opt() == Some(*open_last) => *open_last = last,
            _ => merged.push((first, last)),
        }
    }
    merged
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

/// The remaining Muhurat-Trading dates are withheld, not closed: the operator
/// announces a session there whose instants it has not published. The eight
/// dates whose circulars were recovered ship replacement blocks and are
/// fenced by `the_published_muhurat_sessions_ship_as_replacement_blocks`
/// below.
#[test]
fn the_muhurat_dates_are_unsourced_neither_closed_nor_normal() {
    let nse = calendar_for_exchange(Exchange::NseIndia);
    assert_unsourced(nse, (2026, 11, 8), "NSE-HOL-2026", "Sunday Muhurat");
    // The same withholding across the backfilled years: an asterisked holiday
    // the operator footnotes a Muhurat session onto (T1 pages and circulars).
    // The 2012 footnote came with the recovered 2012 list itself
    // (`NSE-CIRC-2011-66`), so recovering that year reopened its window with
    // its own Muhurat date withheld like its sibling years'.
    assert_unsourced(nse, (2012, 11, 13), "NSE-CIRC-2011-66", "2012 Muhurat");
    assert_unsourced(nse, (2014, 10, 23), "NSE-HOL-PAGE-2014", "2014 Muhurat");

    // A probe inside the Sunday Muhurat day refuses too, rather than reading
    // the normal weekend. The Monday after does not: the Monday session opens
    // 09:15 IST on the sourced 2026-11-09, and no NSE session wraps, so
    // nothing from the withheld Sunday can reach it and noon answers inside
    // its own regular session. The Saturday before is likewise untouched and
    // answers as an ordinary weekend.
    assert!(matches!(
        nse.is_open(zoned(Asia::Kolkata, (2026, 11, 8), (18, 0, 0))),
        Err(CalendarQueryError::UnresolvedGap { .. })
    ));
    assert!(
        nse.is_open(zoned(Asia::Kolkata, (2026, 11, 9), (10, 0, 0)))
            .expect("the Monday session opens on the sourced day"),
        "the Monday after the Muhurat Sunday answers inside its own session"
    );
    assert!(
        !nse.is_open(zoned(Asia::Kolkata, (2026, 11, 7), (10, 0, 0)))
            .expect("2026-11-07 is an audited weekend and must answer"),
        "the Saturday before the Muhurat Sunday is an ordinary weekend"
    );
    // 2025-10-22 answers shut; the special session on the 2025-10-21 eve now
    // ships its own blocks, so settling the next day's trade date scans back
    // onto a sourced (if special) day and must not refuse.
    let diwali_eve = day(2025, 10, 20);
    assert_eq!(
        nse.holiday_on(diwali_eve),
        None,
        "2025-10-20 is audited normal"
    );
    assert!(
        nse.is_closed_trade_date(day(2025, 10, 22), SessionKind::Both)
            .expect("2025-10-22 settles against the sourced Muhurat eve"),
        "Diwali Balipratipada answers shut"
    );
}

/// The eight Muhurat dates whose instants the operator published ship as
/// replacement-block days restating the printed schedule: the row exists, is
/// keyed to the circular, the day trades exactly inside the printed blocks —
/// including the 2020 Saturday special session that has no normal-week
/// counterpart — and end-exclusive at every block close.
#[test]
fn the_published_muhurat_sessions_ship_as_replacement_blocks() {
    let nse = calendar_for_exchange(Exchange::NseIndia);

    // The row itself: kind, tier and the circular it keys to.
    for (date, document) in [
        ((2010, 11, 5), "NSE-CIRC-2010-121"),
        ((2015, 11, 11), "NSE-CIRC-2015-65"),
        ((2016, 10, 30), "NSE-CIRC-2016-56"),
        ((2020, 11, 14), "NSE-CIRC-2020-98"),
        ((2022, 10, 24), "NSE-CIRC-2022-124"),
        ((2023, 11, 12), "NSE-CIRC-2023-139"),
        ((2024, 11, 1), "NSE-CIRC-2024-147"),
        ((2025, 10, 21), "NSE-CIRC-2025-124"),
    ] {
        let row = nse
            .holiday_on(day(date.0, date.1, date.2))
            .unwrap_or_else(|| panic!("{date:?}: the Muhurat circular ships a row"));
        assert!(
            matches!(row.kind(), HolidayKind::ReplacementBlocks(_)),
            "{date:?} kind"
        );
        assert_eq!(row.tier(), EvidenceTier::T1, "{date:?} tier");
        assert_eq!(row.document_id(), document, "{date:?} document id");
    }

    // 2020-11-14 (a Saturday): block deal 17:45-18:00, pre-open order entry
    // 18:00-18:07 with the random-closure tail to the 18:15 open tradeable,
    // Normal Market 18:15-19:15, Closing 19:25-19:35. End-exclusive at the
    // printed closes; the daytime and the 19:15-19:25 gap answer shut.
    let sat = Asia::Kolkata;
    assert!(
        nse.is_open(zoned(sat, (2020, 11, 14), (17, 45, 0)))
            .expect("the block-deal session opens"),
        "the block-deal session trades from 17:45 on the Muhurat Saturday"
    );
    assert!(
        nse.is_order_entry_only(zoned(sat, (2020, 11, 14), (18, 2, 0)))
            .expect("the pre-open queue is order entry"),
        "18:02 sits in the pre-open order-entry leg"
    );
    assert!(
        !nse.is_order_entry_only(zoned(sat, (2020, 11, 14), (18, 8, 0)))
            .expect("the auction tail is tradeable"),
        "the random-closure tail to the open prints, so it is not order entry"
    );
    assert!(
        nse.is_open(zoned(sat, (2020, 11, 14), (19, 14, 59)))
            .expect("the close is end-exclusive"),
        "19:14:59 still trades inside the Normal Market block"
    );
    assert!(
        !nse.is_open(zoned(sat, (2020, 11, 14), (19, 15, 0)))
            .expect("the close answers"),
        "19:15 sharp is closed: closes are end-exclusive"
    );
    assert!(
        !nse.is_open(zoned(sat, (2020, 11, 14), (19, 20, 0)))
            .expect("the 19:15-19:25 gap answers"),
        "the gap between the Normal Market and the Closing Session is shut"
    );
    assert!(
        !nse.is_open(zoned(sat, (2020, 11, 14), (19, 35, 0)))
            .expect("the Closing Session close is end-exclusive"),
        "19:35 sharp is closed"
    );
    assert!(
        !nse.is_open(zoned(sat, (2020, 11, 14), (10, 0, 0)))
            .expect("the daytime answers"),
        "the Muhurat Saturday has no daytime session"
    );

    // 2024-11-01 (a Friday): the printed Pre Open bounds its order-entry
    // period at 17:45-17:53, so the leg stops at 17:52 and the tail to the
    // 18:00 open trades.
    assert!(
        nse.is_order_entry_only(zoned(sat, (2024, 11, 1), (17, 50, 0)))
            .expect("the 2024 pre-open queue is order entry"),
        "17:50 sits in the 17:45-17:52 order-entry leg"
    );
    assert!(
        nse.is_open(zoned(sat, (2024, 11, 1), (17, 55, 0)))
            .expect("the 2024 auction tail is tradeable"),
        "17:52-18:00 is the tradeable random-closure tail"
    );
    assert!(
        nse.is_open_regular(zoned(sat, (2024, 11, 1), (18, 30, 0)))
            .expect("the 2024 Normal Market answers"),
        "18:30 trades in the 18:00-19:00 Normal Market"
    );
    assert!(
        !nse.is_open(zoned(sat, (2024, 11, 1), (19, 20, 0)))
            .expect("the 2024 close is end-exclusive"),
        "19:20 sharp is closed"
    );

    // 2025-10-21 (a Tuesday): the whole day runs to the circular's printed
    // midday schedule; the ordinary daytime grid is replaced outright.
    assert!(
        nse.is_open(zoned(sat, (2025, 10, 21), (13, 15, 0)))
            .expect("the 2025 block-deal session opens"),
        "the block-deal session trades from 13:15"
    );
    assert!(
        nse.is_open(zoned(sat, (2025, 10, 21), (14, 44, 59)))
            .expect("the 2025 Normal Market close is end-exclusive"),
        "14:44:59 still trades"
    );
    assert!(
        !nse.is_open(zoned(sat, (2025, 10, 21), (14, 45, 0)))
            .expect("the 2025 close answers"),
        "14:45 sharp is closed: closes are end-exclusive"
    );
    assert!(
        !nse.is_open(zoned(sat, (2025, 10, 21), (9, 30, 0)))
            .expect("the ordinary daytime grid answers"),
        "the ordinary 09:15-15:30 grid is replaced: 09:30 is shut"
    );
    assert!(
        !nse.is_open(zoned(sat, (2025, 10, 21), (15, 5, 0)))
            .expect("the 2025 Closing Session close is end-exclusive"),
        "15:05 sharp is closed"
    );

    // 2010-11-05 (a Friday): pre-open order entry 18:00-18:07 with the
    // random-closure tail to the 18:15 open tradeable, Normal/RDM/Odd Lot
    // 18:15-19:00, Closing Session 19:20-19:30 (no block-deal session in
    // 2010). End-exclusive at the printed closes.
    the_recovered_2010_muhurat_schedule_probes(nse);

    // 2015-11-11 (a Wednesday): Pre Open 17:30-17:37, Normal Market/LPM
    // 17:45-18:45, Closing Session 18:55-19:05; 2016-10-30 (a Sunday the
    // operator announces a session on): Pre Open 18:15-18:22, Normal
    // Market/LPM 18:30-19:30, Closing Session 19:40-19:50.
    the_recovered_2015_and_2016_muhurat_schedule_probes(nse);

    // The special sessions produce no maintenance and settle their own trade
    // dates: a 2020 Saturday session means 2020-11-14 is not a closed trade
    // date even though the normal week would claim a weekend closure.
    assert!(
        !nse.is_closed_trade_date(day(2020, 11, 14), SessionKind::Both)
            .expect("the Muhurat Saturday answers"),
        "the Saturday special session is a session: the date does not settle closed"
    );
    assert!(
        !nse.is_closed_trade_date(day(2016, 10, 30), SessionKind::Both)
            .expect("the Muhurat Sunday answers"),
        "the announced Sunday session is a session: the date does not settle closed"
    );
    assert!(
        !nse.is_maintenance(zoned(sat, (2020, 11, 14), (18, 30, 0)))
            .expect("the Normal Market answers its state"),
        "inside the Muhurat Normal Market is a session, not maintenance"
    );
}

/// The 2010-11-05 printed schedule (a Friday): pre-open order entry
/// 18:00-18:07 with the random-closure tail to the 18:15 open tradeable,
/// Normal/RDM/Odd Lot 18:15-19:00, Closing Session 19:20-19:30 — no
/// block-deal session exists in 2010. End-exclusive at the printed closes.
fn the_recovered_2010_muhurat_schedule_probes(nse: ExchangeCalendar) {
    let ist = Asia::Kolkata;
    assert!(
        nse.is_order_entry_only(zoned(ist, (2010, 11, 5), (18, 2, 0)))
            .expect("the 2010 pre-open queue is order entry"),
        "18:02 sits in the 18:00-18:07 order-entry leg"
    );
    assert!(
        nse.is_open(zoned(ist, (2010, 11, 5), (18, 10, 0)))
            .expect("the 2010 auction tail is tradeable"),
        "18:07-18:15 is the tradeable random-closure tail"
    );
    assert!(
        nse.is_open_regular(zoned(ist, (2010, 11, 5), (18, 30, 0)))
            .expect("the 2010 Normal Market answers"),
        "18:30 trades in the 18:15-19:00 Normal Market"
    );
    assert!(
        !nse.is_open(zoned(ist, (2010, 11, 5), (19, 0, 0)))
            .expect("the 2010 Normal Market close answers"),
        "19:00 sharp is closed: closes are end-exclusive"
    );
    assert!(
        !nse.is_open(zoned(ist, (2010, 11, 5), (19, 10, 0)))
            .expect("the 2010 19:00-19:20 gap answers"),
        "the gap before the Closing Session is shut"
    );
    assert!(
        !nse.is_open(zoned(ist, (2010, 11, 5), (19, 30, 0)))
            .expect("the 2010 Closing Session close is end-exclusive"),
        "19:30 sharp is closed"
    );
    assert!(
        !nse.is_open(zoned(ist, (2010, 11, 5), (13, 0, 0)))
            .expect("the 2010 daytime answers"),
        "the ordinary 09:15-15:30 grid is replaced: 13:00 is shut"
    );
}

/// The 2015-11-11 printed schedule (a Wednesday): Pre Open 17:30-17:37,
/// Normal Market/LPM 17:45-18:45, Closing Session 18:55-19:05 — and the
/// 2016-10-30 printed schedule (a Sunday the operator announces a session
/// on): Pre Open 18:15-18:22, Normal Market/LPM 18:30-19:30, Closing Session
/// 19:40-19:50.
fn the_recovered_2015_and_2016_muhurat_schedule_probes(nse: ExchangeCalendar) {
    let ist = Asia::Kolkata;
    assert!(
        nse.is_order_entry_only(zoned(ist, (2015, 11, 11), (17, 33, 0)))
            .expect("the 2015 pre-open queue is order entry"),
        "17:33 sits in the 17:30-17:37 order-entry leg"
    );
    assert!(
        nse.is_open(zoned(ist, (2015, 11, 11), (17, 40, 0)))
            .expect("the 2015 auction tail is tradeable"),
        "17:37-17:45 is the tradeable random-closure tail"
    );
    assert!(
        nse.is_open_regular(zoned(ist, (2015, 11, 11), (18, 0, 0)))
            .expect("the 2015 Normal Market answers"),
        "18:00 trades in the 17:45-18:45 Normal Market"
    );
    assert!(
        !nse.is_open(zoned(ist, (2015, 11, 11), (18, 45, 0)))
            .expect("the 2015 Normal Market close is end-exclusive"),
        "18:45 sharp is closed"
    );
    assert!(
        !nse.is_open(zoned(ist, (2015, 11, 11), (18, 50, 0)))
            .expect("the 2015 18:45-18:55 gap answers"),
        "the gap before the Closing Session is shut"
    );
    assert!(
        nse.is_open(zoned(ist, (2015, 11, 11), (19, 4, 59)))
            .expect("the 2015 Closing Session still trades"),
        "19:04:59 still trades inside the Closing Session"
    );
    assert!(
        !nse.is_open(zoned(ist, (2015, 11, 11), (10, 0, 0)))
            .expect("the 2015 daytime answers"),
        "the ordinary 09:15-15:30 grid is replaced: 10:00 is shut"
    );
    assert!(
        nse.is_order_entry_only(zoned(ist, (2016, 10, 30), (18, 18, 0)))
            .expect("the 2016 pre-open queue is order entry"),
        "18:18 sits in the 18:15-18:22 order-entry leg"
    );
    assert!(
        nse.is_open(zoned(ist, (2016, 10, 30), (18, 26, 0)))
            .expect("the 2016 auction tail is tradeable"),
        "18:22-18:30 is the tradeable random-closure tail"
    );
    assert!(
        nse.is_open_regular(zoned(ist, (2016, 10, 30), (19, 0, 0)))
            .expect("the 2016 Normal Market answers"),
        "19:00 trades in the 18:30-19:30 Normal Market"
    );
    assert!(
        !nse.is_open(zoned(ist, (2016, 10, 30), (19, 30, 0)))
            .expect("the 2016 Normal Market close is end-exclusive"),
        "19:30 sharp is closed"
    );
    assert!(
        nse.is_open(zoned(ist, (2016, 10, 30), (19, 45, 0)))
            .expect("the 2016 Closing Session trades"),
        "19:45 trades inside the 19:40-19:50 Closing Session"
    );
    assert!(
        !nse.is_open(zoned(ist, (2016, 10, 30), (12, 0, 0)))
            .expect("the 2016 daytime answers"),
        "the ordinary weekend stays shut outside the announced session"
    );
}

/// The counts per year, read back from the shipped tables by walking them:
/// 16+15+13+16+17+17+16+13+16+20+19+16+16+14+17 TSE closures across 2010-2024
/// plus 18 + 19 + 17 over 2025-2027, 16+18+23+16+15+17+16+18+15+19+18+18+18+20
/// SSE closures across 2011-2024 plus 18 + 19 over 2025-2026, and the NSE rows
/// of 10+13+14+13+16+14+15+12+16+13+13+12+15+13 closures across 2010-2024
/// except 2018 (2018 unrecovered, shipping none) plus 13 + 1 and 15 + 1 over
/// 2025-2026 (the one per year being the withheld Muhurat date; 2012's own
/// withheld date came with its recovered list). NSE's eight published Muhurat
/// days ship `ReplacementBlocks` rows — 2010, 2015, 2016, 2020, 2022, 2023,
/// 2024 and 2025 carry one each in the last column. A row added, moved across
/// a year or dropped breaks the count; the per-row fence above pins where.
#[test]
fn the_window_counts_are_the_printed_lists_counts() {
    fn count_by_year(calendar: ExchangeCalendar) -> Vec<(i32, usize, usize, usize)> {
        let coverage = calendar
            .holiday_coverage()
            .expect("each of these venues ships a table");
        let mut out: Vec<(i32, usize, usize, usize)> = Vec::new();
        let mut date = coverage.first();
        while date <= coverage.last() {
            let year = date.year();
            let entry = match out.last_mut() {
                Some((years, ..)) if *years == year => out.last_mut().expect("just checked"),
                _ => {
                    out.push((year, 0, 0, 0));
                    out.last_mut().expect("just pushed")
                }
            };
            match calendar.holiday_on(date).map(Holiday::kind) {
                Some(HolidayKind::Closed) => entry.1 += 1,
                Some(HolidayKind::Unsourced) => entry.2 += 1,
                Some(HolidayKind::ReplacementBlocks(_)) => entry.3 += 1,
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
        [
            (2010, 16, 0, 0),
            (2011, 15, 0, 0),
            (2012, 13, 0, 0),
            (2013, 16, 0, 0),
            (2014, 17, 0, 0),
            (2015, 17, 0, 0),
            (2016, 16, 0, 0),
            (2017, 13, 0, 0),
            (2018, 16, 0, 0),
            (2019, 20, 0, 0),
            (2020, 19, 0, 0),
            (2021, 16, 0, 0),
            (2022, 16, 0, 0),
            (2023, 14, 0, 0),
            (2024, 17, 0, 0),
            (2025, 18, 0, 0),
            (2026, 19, 0, 0),
            (2027, 17, 0, 0),
        ],
        "TSE closures per year, from the operator's tables"
    );
    assert_eq!(
        count_by_year(calendar_for_exchange(Exchange::Sse)),
        [
            // 2018 carries 17 weekday legs of the 2018 notice plus the
            // 2019 notice's 12-31 元旦 leg; 2019 carries the 2019 notice's
            // 15 weekday legs (its own 元旦 leg is 2019-01-01); 2020 carries
            // the annual notice's 18 plus the COVID extension's 01-31.
            (2011, 16, 0, 0),
            (2012, 18, 0, 0),
            (2013, 23, 0, 0),
            (2014, 16, 0, 0),
            (2015, 15, 0, 0),
            (2016, 17, 0, 0),
            (2017, 16, 0, 0),
            (2018, 18, 0, 0),
            (2019, 15, 0, 0),
            (2020, 19, 0, 0),
            (2021, 18, 0, 0),
            (2022, 18, 0, 0),
            (2023, 18, 0, 0),
            (2024, 20, 0, 0),
            (2025, 18, 0, 0),
            (2026, 19, 0, 0),
        ],
        "SSE closures per year, from the operator's notices"
    );
    assert_eq!(
        count_by_year(calendar_for_exchange(Exchange::NseIndia)),
        [
            // 2018 is unrecovered (no operator artifact in the archive), so it
            // ships no rows and sits inside no window; every other year
            // carries its list's weekday legs — 2012's from the recovered
            // annual circular `NSE-CIRC-2011-66` (fourteen closures plus its
            // own withheld Muhurat date); the Muhurat dates whose circulars
            // were recovered ship replacement blocks (2010, 2015, 2016, 2020,
            // 2022, 2023, 2024 and 2025), the rest are withheld (2021's list
            // names no Muhurat date, so nothing is withheld there).
            (2010, 10, 0, 1),
            (2011, 13, 1, 0),
            (2012, 14, 1, 0),
            (2013, 13, 1, 0),
            (2014, 16, 1, 0),
            (2015, 14, 0, 1),
            (2016, 15, 0, 1),
            (2017, 12, 1, 0),
            (2018, 0, 0, 0),
            (2019, 16, 1, 0),
            (2020, 12, 0, 1),
            (2021, 13, 0, 0),
            (2022, 12, 0, 1),
            (2023, 15, 0, 1),
            (2024, 13, 0, 1),
            (2025, 13, 0, 1),
            (2026, 15, 1, 0),
        ],
        "NSE closures, withheld Muhurat dates and replacement-block Muhurat days per year, from the operator's lists"
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
    // TSE: 2010-01-01 .. 2027-12-31 — the operator's own pages state the
    // current and next year from 2010 on, so the window reaches the floor.
    let tse = calendar_for_exchange(Exchange::Tse);
    let tse_coverage = tse.holiday_coverage().expect("TSE ships a table");
    assert_eq!(tse_coverage.first(), day(2010, 1, 1));
    assert_eq!(tse_coverage.last(), day(2027, 12, 31));
    assert_eq!(
        tse.holiday_on(day(2009, 12, 31)),
        None,
        "the day before the window carries no row"
    );
    assert_eq!(
        tse.holiday_on(day(2028, 1, 3)),
        None,
        "the day after the window carries no row"
    );
    // 2024-12-31 is now inside the window and is one of the page's own rows.
    let year_end = tse
        .holiday_on(day(2024, 12, 31))
        .expect("2024-12-31 is inside the audited window");
    assert_eq!(
        year_end.kind(),
        HolidayKind::Closed,
        "2024-12-31 is the printed Dec. 31 Market Holiday of the 2024 page"
    );
    assert!(
        !tse.is_open(zoned(Asia::Tokyo, (2024, 12, 31), (10, 0, 0)))
            .expect("2024-12-31 is inside the audited window and must answer"),
        "the printed year-end holiday answers shut"
    );
    assert!(
        matches!(
            tse.is_open(zoned(Asia::Tokyo, (2028, 1, 4), (10, 0, 0))),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ),
        "a 2028 weekday is past the published schedule and must be refused"
    );

    // SSE: 2011-01-01 .. 2026-12-31; 2010 is the unrecovered-notice gap and
    // the 2027 notice is not published.
    let sse = calendar_for_exchange(Exchange::Sse);
    let sse_coverage = sse.holiday_coverage().expect("SSE ships a table");
    assert_eq!(sse_coverage.first(), day(2011, 1, 1));
    assert_eq!(sse_coverage.last(), day(2026, 12, 31));
    assert!(
        matches!(
            sse.is_open(zoned(Asia::Shanghai, (2027, 1, 4), (10, 0, 0))),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ),
        "2027 is unpublished by SSE and the identity must refuse it outright"
    );
    // 2010 is the unrecovered-notice gap, below the first audited window: the
    // session questions answer from the sourced normal week (the 2026-10-07
    // bridged-residual ruling, #296) while the holiday-table classification
    // refuses, because nothing witnesses the 2010 holiday layer.
    assert_eq!(
        sse.is_open(zoned(Asia::Shanghai, (2010, 12, 31), (10, 0, 0))),
        Ok(true),
        "the 2010-12-31 Thursday answers the sourced normal week beside the          one-flank bridged residual"
    );
    assert!(
        matches!(
            sse.is_closed_trade_date(day(2010, 12, 31), SessionKind::Both),
            Err(CalendarQueryError::UnresolvedGap { date, .. }) if date == day(2010, 12, 31)
        ),
        "the 2010 classification refuses: no audited window brackets the date"
    );

    // NSE: 2010-01-01 .. 2026-12-31 over two audited windows; 2018 is
    // unrecovered (no operator artifact in the archive) and the 2027 list is
    // not published. 2012 was unrecovered until 2026-10-06 UTC, when the
    // 2012 list's own annual circular (`NSE-CIRC-2011-66`, dated 2011-12-09)
    // surfaced and the window merged.
    let nse = calendar_for_exchange(Exchange::NseIndia);
    let nse_coverage = nse.holiday_coverage().expect("NSE ships a table");
    assert_eq!(nse_coverage.first(), day(2010, 1, 1));
    assert_eq!(nse_coverage.last(), day(2026, 12, 31));
    assert_eq!(
        nse_coverage.windows(),
        vec![
            (day(2010, 1, 1), day(2017, 12, 31)),
            (day(2019, 1, 1), day(2026, 12, 31)),
        ],
        "the two audited windows stop at the unrecovered year's edges"
    );
    assert!(
        matches!(
            nse.is_open(zoned(Asia::Kolkata, (2027, 1, 4), (10, 0, 0))),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ),
        "2027 is unpublished by NSE and the identity must refuse it outright"
    );
    // The unrecovered year sits between two audited windows, so the bridged
    // residual (#296) answers the session question from the sourced normal
    // week while the metadata reports the span as `HolidayWindowsBridged` —
    // never an audited normal, never a fabricated closure. The recovered 2012
    // beside it answers inside its own window.
    assert_eq!(
        nse.is_open(zoned(Asia::Kolkata, (2018, 6, 15), (10, 0, 0))),
        Ok(true),
        "2018-06-15 answers its normal-week session state beside the disclosed \
         bridged residual"
    );
    assert_eq!(
        gap_reason_on(nse.coverage(), day(2018, 6, 15)),
        Some(CoverageGapReason::HolidayWindowsBridged),
        "the unrecovered year reports the bridged residual"
    );
    assert_eq!(
        gap_reason_on(nse.coverage(), day(2012, 6, 15)),
        None,
        "the recovered 2012 reports no residual"
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
fn the_only_unsourced_rows_are_the_muhurat_dates() {
    use chrono::Datelike as _;

    const MUHURAT: [(i32, u32, u32); 7] = [
        (2011, 10, 26),
        (2012, 11, 13),
        (2013, 11, 3),
        (2014, 10, 23),
        (2017, 10, 19),
        (2019, 10, 27),
        (2026, 11, 8),
    ];
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
                    MUHURAT.contains(&(date.year(), date.month(), date.day())),
                    "{date}: the only withheld dates are the Muhurat dates"
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
