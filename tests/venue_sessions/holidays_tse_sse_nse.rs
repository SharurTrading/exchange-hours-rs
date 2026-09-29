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
/// and that the whole trade date answers shut. The exceptions are the
/// window's own first dates: settling that trade date reads the day before
/// it, which sits outside the audited window (or below the 2010 support
/// floor), so the query refuses there — the same shape the CFE fence
/// documents for its New Year row.
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
/// 16+15+13+16+17+17+16+13+16+20+19+16+16+14+17 TSE closures across 2010-2024
/// plus 18 + 19 + 17 over 2025-2027, 18 + 19 SSE closures, and 13 + 1 plus
/// 15 + 1 NSE rows (the one per year being the withheld Muhurat date). A row
/// added, moved across a year or dropped breaks the count; the per-row fence
/// above pins where.
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
        [
            (2010, 16, 0),
            (2011, 15, 0),
            (2012, 13, 0),
            (2013, 16, 0),
            (2014, 17, 0),
            (2015, 17, 0),
            (2016, 16, 0),
            (2017, 13, 0),
            (2018, 16, 0),
            (2019, 20, 0),
            (2020, 19, 0),
            (2021, 16, 0),
            (2022, 16, 0),
            (2023, 14, 0),
            (2024, 17, 0),
            (2025, 18, 0),
            (2026, 19, 0),
            (2027, 17, 0),
        ],
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
