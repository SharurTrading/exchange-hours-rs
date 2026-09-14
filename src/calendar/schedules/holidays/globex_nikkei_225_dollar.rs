// SPDX-License-Identifier: MIT-0

//! CME Nikkei 225 Dollar (`NKD`) holiday and early-close rows, 2016-2018 and
//! 2025-2027.
//!
//! Keyed by the crate's own venue-local trade date in `America/Chicago`
//! (design memo D1). The conversion is never the identity here: the family
//! runs one wrapped envelope per trade date — 17:00 CT on the previous evening
//! to 16:00 CT on the trade date — so CME's event-date records for a holiday
//! and its eve collapse into one row keyed to the day the session's final
//! close falls on, and the eve record is evidence for that row rather than a
//! row of its own.
//!
//! The rows come from CME's trading-hours service — the endpoint
//! `cmegroup.com/trading-hours.html` itself calls — read as bytes and saved,
//! so the 2025-2027 block is **T2** under LAW-PRIMARY-SOURCES. From Thanksgiving
//! 2025 the service publishes `NKD` and `NIY` as their own line; through Labor
//! Day 2025 it does not, and those nine rows are taken from the Equity Index
//! line of the same capture. That interpretive step, the two Saturday sessions
//! the scalar vocabulary cannot state, and the one sourced intraday-topology
//! day are recorded in
//! [`docs/evidence/globex_nikkei_225_dollar.md`](../../../../../docs/evidence/globex_nikkei_225_dollar.md).
//!
//! **2016-2018.** The era's rows are **T1**, CME's own published Globex holiday
//! schedules, and the grid is the wrapped `17:00 CT -> 16:00 CT` leg: an early
//! close is the printed final close on the trade date it ends, a closure
//! removes the trade date with its prior-evening leg, and every stated re-open
//! at the ordinary 17:00 CT evening open ships no row. The thirty-four rows
//! carry the same shape and instants as `globex_equity_index`'s own 2016-2018
//! window: nine closures, twenty-four early closes — 12:00 CT on the eighteen
//! Monday and Thursday holidays and 12:15 CT on the three Thanksgiving Fridays
//! and three year-end half-days — and one late open.
//!
//! Three shapes for the family: `Closed` on a full Globex closure,
//! `EarlyClose` on the half-days, and — in 2016-2018 alone — a **late open** at
//! 15:30 CT on 2018-12-26. That one is the CME Christmas sheet's own printed
//! pair, `Pre-opening 15:15; Open 15:30`, which is the era's routine
//! extended-session grid printed as one pair rather than a disagreement between
//! sources. The 2025-2027 window has no late open, because every CME re-open
//! after a closure there is the grid's own 17:00 CT evening open.

use super::fences::{early_close, late_open};
use super::{
    EvidenceTier::{T1, T2},
    HolidayKind::Closed,
    HolidayTable, holidays,
};

/// The family's built-in holiday rows and the windows they were audited over.
///
/// Two audited eras: 2016-2018 at T1 and 2025-2027 at T2, with the 2013-2015
/// and 2019-2024 intervals audited by no wave and outside every window. No
/// Nikkei table was ever built for 2010-2012, so that era is not a window here.
/// Coverage runs to 2027-12-31, the end of the operator's published future;
/// CME's 2028-01-01 record is a Saturday outside every window and ships no row.
// Evidence: docs/evidence/globex_nikkei_225_dollar.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2016, 1, 1) ..= (2018, 12, 31), (2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2016-01-01 - T1 - 2016-new-years-holiday-schedule.pdf @2016-01-08 - closed: no trade date.
        (2016, 1, 1, Closed, T1, "2016-new-years-holiday-schedule.pdf @2016-01-08"),
        // 2016-01-18 - T1 - 2016-holiday-calendars.zip#2016-martin-luther-king-holiday-schedule.pdf @2017-06-28 - early close 12:00 CT.
        (2016, 1, 18, early_close(12 * 3_600), T1, "2016-holiday-calendars.zip#2016-martin-luther-king-holiday-schedule.pdf @2017-06-28"),
        // 2016-02-15 - T1 - 2016-holiday-calendars.zip#2016-presidents-day-holiday-schedule.pdf @2017-06-28 - early close 12:00 CT.
        (2016, 2, 15, early_close(12 * 3_600), T1, "2016-holiday-calendars.zip#2016-presidents-day-holiday-schedule.pdf @2017-06-28"),
        // 2016-03-25 - T1 - 2016-holiday-calendars.zip#2016-good-friday-holiday-schedule.pdf @2017-06-28 - closed: no trade date.
        (2016, 3, 25, Closed, T1, "2016-holiday-calendars.zip#2016-good-friday-holiday-schedule.pdf @2017-06-28"),
        // 2016-05-30 - T1 - 2016-holiday-calendars.zip#2016-memorial-day-holiday-schedule.pdf @2017-06-28 - early close 12:00 CT.
        (2016, 5, 30, early_close(12 * 3_600), T1, "2016-holiday-calendars.zip#2016-memorial-day-holiday-schedule.pdf @2017-06-28"),
        // 2016-07-04 - T1 - 2016-holiday-calendars.zip#2016-4th-of-july-holiday-schedule.pdf @2017-06-28 - early close 12:00 CT.
        (2016, 7, 4, early_close(12 * 3_600), T1, "2016-holiday-calendars.zip#2016-4th-of-july-holiday-schedule.pdf @2017-06-28"),
        // 2016-09-05 - T1 - 2016-holiday-calendars.zip#2016-labor-day-holiday-schedule.pdf @2017-06-28 - early close 12:00 CT.
        (2016, 9, 5, early_close(12 * 3_600), T1, "2016-holiday-calendars.zip#2016-labor-day-holiday-schedule.pdf @2017-06-28"),
        // 2016-11-24 - T1 - 2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28 - early close 12:00 CT.
        (2016, 11, 24, early_close(12 * 3_600), T1, "2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28"),
        // 2016-11-25 - T1 - 2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28 - early close 12:15 CT.
        (2016, 11, 25, early_close(12 * 3_600 + 15 * 60), T1, "2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28"),
        // 2016-12-26 - T1 - 2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28 - closed: no trade date.
        (2016, 12, 26, Closed, T1, "2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28"),
        // 2017-01-02 - T1 - 2016-holiday-calendars.zip#2017-new-years-holiday-schedule.pdf @2017-06-28 - closed: no trade date.
        (2017, 1, 2, Closed, T1, "2016-holiday-calendars.zip#2017-new-years-holiday-schedule.pdf @2017-06-28"),
        // 2017-01-16 - T1 - 2017-martin-luther-king-holiday-schedule.xls @2017-06-28 - early close 12:00 CT.
        (2017, 1, 16, early_close(12 * 3_600), T1, "2017-martin-luther-king-holiday-schedule.xls @2017-06-28"),
        // 2017-02-20 - T1 - 2017-presidents-day-holiday-schedule.xls @2017-06-28 - early close 12:00 CT.
        (2017, 2, 20, early_close(12 * 3_600), T1, "2017-presidents-day-holiday-schedule.xls @2017-06-28"),
        // 2017-04-14 - T1 - 2017-good-friday-holiday-schedule.xls @2017-05-05 - closed: no trade date.
        (2017, 4, 14, Closed, T1, "2017-good-friday-holiday-schedule.xls @2017-05-05"),
        // 2017-05-29 - T1 - 2017-memorial-day-holiday-schedule.xls @2017-10-25 - early close 12:00 CT.
        (2017, 5, 29, early_close(12 * 3_600), T1, "2017-memorial-day-holiday-schedule.xls @2017-10-25"),
        // 2017-07-03 - T1 - 2017-4th-of-july-holiday-schedule.xls @2017-10-25 - early close 12:15 CT.
        (2017, 7, 3, early_close(12 * 3_600 + 15 * 60), T1, "2017-4th-of-july-holiday-schedule.xls @2017-10-25"),
        // 2017-07-04 - T1 - 2017-4th-of-july-holiday-schedule.xls @2017-10-25 - early close 12:00 CT.
        (2017, 7, 4, early_close(12 * 3_600), T1, "2017-4th-of-july-holiday-schedule.xls @2017-10-25"),
        // 2017-09-04 - T1 - 2017-labor-day-holiday-schedule.xls @2017-10-25 - early close 12:00 CT.
        (2017, 9, 4, early_close(12 * 3_600), T1, "2017-labor-day-holiday-schedule.xls @2017-10-25"),
        // 2017-11-23 - T1 - 2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26 - early close 12:00 CT.
        (2017, 11, 23, early_close(12 * 3_600), T1, "2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26"),
        // 2017-11-24 - T1 - 2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26 - early close 12:15 CT.
        (2017, 11, 24, early_close(12 * 3_600 + 15 * 60), T1, "2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26"),
        // 2017-12-25 - T1 - 2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26 - closed: no trade date.
        (2017, 12, 25, Closed, T1, "2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26"),
        // 2018-01-01 - T1 - 2018-new-years-holiday-schedule.xls @2018-01-06 - closed: no trade date.
        (2018, 1, 1, Closed, T1, "2018-new-years-holiday-schedule.xls @2018-01-06"),
        // 2018-01-15 - T1 - 2018-martin-luther-king-holiday-schedule.xls @2018-05-08 - early close 12:00 CT.
        (2018, 1, 15, early_close(12 * 3_600), T1, "2018-martin-luther-king-holiday-schedule.xls @2018-05-08"),
        // 2018-02-19 - T1 - 2018-presidents-day-holiday-schedule.xls @2018-05-08 - early close 12:00 CT.
        (2018, 2, 19, early_close(12 * 3_600), T1, "2018-presidents-day-holiday-schedule.xls @2018-05-08"),
        // 2018-03-30 - T1 - 2018-holiday-calendars.zip#2018-good-friday-holiday-schedule.xls @2026-08-30 - closed: no trade date.
        (2018, 3, 30, Closed, T1, "2018-holiday-calendars.zip#2018-good-friday-holiday-schedule.xls @2026-08-30"),
        // 2018-05-28 - T1 - 2018-holiday-calendars.zip#2018-memorial-day-holiday-schedule.xls @2026-08-30 - early close 12:00 CT.
        (2018, 5, 28, early_close(12 * 3_600), T1, "2018-holiday-calendars.zip#2018-memorial-day-holiday-schedule.xls @2026-08-30"),
        // 2018-07-03 - T1 - 2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30 - early close 12:15 CT.
        (2018, 7, 3, early_close(12 * 3_600 + 15 * 60), T1, "2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30"),
        // 2018-07-04 - T1 - 2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30 - early close 12:00 CT.
        (2018, 7, 4, early_close(12 * 3_600), T1, "2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30"),
        // 2018-09-03 - T1 - 2018-holiday-calendars.zip#2018-labor-day-holiday-schedule.xls @2026-08-30 - early close 12:00 CT.
        (2018, 9, 3, early_close(12 * 3_600), T1, "2018-holiday-calendars.zip#2018-labor-day-holiday-schedule.xls @2026-08-30"),
        // 2018-11-22 - T1 - 2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30 - early close 12:00 CT.
        (2018, 11, 22, early_close(12 * 3_600), T1, "2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30"),
        // 2018-11-23 - T1 - 2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30 - early close 12:15 CT.
        (2018, 11, 23, early_close(12 * 3_600 + 15 * 60), T1, "2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30"),
        // 2018-12-24 - T1 - 2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30 - early close 12:15 CT.
        (2018, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30"),
        // 2018-12-25 - T1 - 2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30 - closed: no trade date.
        (2018, 12, 25, Closed, T1, "2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30"),
        // 2018-12-26 - T1 - 2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30 - late open 15:30 CT: no prior-evening leg.
        (2018, 12, 26, late_open(15 * 3_600 + 30 * 60), T1, "2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30"),
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 - T2 - CME-SVC-2025-01-19 - Martin Luther King Jr. Day, 12:00 CT close.
        (2025, 1, 20, early_close(12 * 3_600), T2, "CME-SVC-2025-01-19"),
        // 2025-02-17 - T2 - CME-SVC-2025-02-16 - Presidents' Day, 12:00 CT close.
        (2025, 2, 17, early_close(12 * 3_600), T2, "CME-SVC-2025-02-16"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-17 - Good Friday, no events published.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 - T2 - CME-SVC-2025-05-25 - Memorial Day, 12:00 CT close.
        (2025, 5, 26, early_close(12 * 3_600), T2, "CME-SVC-2025-05-25"),
        // 2025-06-19 - T2 - CME-SVC-2025-06-18 - Juneteenth, 12:00 CT close.
        (2025, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-2025-06-18"),
        // 2025-07-03 - T2 - CME-SVC-2025-07-03 - Independence Day eve, 12:15 CT close.
        (2025, 7, 3, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-2025-07-03"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-03 - Independence Day, 12:00 CT close.
        (2025, 7, 4, early_close(12 * 3_600), T2, "CME-SVC-2025-07-03"),
        // 2025-09-01 - T2 - CME-SVC-2025-08-31 - Labor Day, 12:00 CT close.
        (2025, 9, 1, early_close(12 * 3_600), T2, "CME-SVC-2025-08-31"),
        // 2025-11-27 - T2 - CME-SVC-B-2025-11-26 - Thanksgiving, 12:00 CT close.
        (2025, 11, 27, early_close(12 * 3_600), T2, "CME-SVC-B-2025-11-26"),
        // 2025-11-28 - T2 - CME-SVC-B-2025-11-26 - day after Thanksgiving, 12:15 CT close.
        (2025, 11, 28, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-B-2025-11-26"),
        // 2025-11-29 - T2 - CME-SVC-B-2025-11-26 - Thanksgiving Saturday; NKD and
        // NIY publish no events, and CME's 2025 Globex table states the period
        // as "27 - 29 November 2025".
        (2025, 11, 29, Closed, T2, "CME-SVC-B-2025-11-26"),
        // 2025-12-24 - T2 - CME-SVC-B-2025-12-24 - Christmas Eve, 12:15 CT close.
        (2025, 12, 24, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-B-2025-12-24"),
        // 2025-12-25 - T2 - CME-SVC-B-2025-12-24 - Christmas Day, no trade date of its own.
        (2025, 12, 25, Closed, T2, "CME-SVC-B-2025-12-24"),
        // 2026-01-01 - T2 - CME-SVC-B-2025-12-31 - New Year's Day, no trade date of its own.
        (2026, 1, 1, Closed, T2, "CME-SVC-B-2025-12-31"),
        // 2026-01-19 - T2 - CME-SVC-B-2026-01-18 - Martin Luther King Jr. Day, 12:00 CT close.
        (2026, 1, 19, early_close(12 * 3_600), T2, "CME-SVC-B-2026-01-18"),
        // 2026-02-16 - T2 - CME-SVC-B-2026-02-15 - Presidents' Day, 12:00 CT close.
        (2026, 2, 16, early_close(12 * 3_600), T2, "CME-SVC-B-2026-02-15"),
        // 2026-04-03 - T2 - CME-SVC-B-2026-04-01 - Good Friday, 08:15 CT close with Equity Index.
        (2026, 4, 3, early_close(8 * 3_600 + 15 * 60), T2, "CME-SVC-B-2026-04-01"),
        // 2026-05-25 - T2 - CME-SVC-B-2026-05-24 - Memorial Day, 12:00 CT close.
        (2026, 5, 25, early_close(12 * 3_600), T2, "CME-SVC-B-2026-05-24"),
        // 2026-06-19 - T2 - CME-SVC-B-2026-06-18 - Juneteenth, 12:00 CT close.
        (2026, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-B-2026-06-18"),
        // 2026-07-03 - T2 - CME-SVC-B-2026-07-03 - Independence Day observed, 12:00 CT close.
        (2026, 7, 3, early_close(12 * 3_600), T2, "CME-SVC-B-2026-07-03"),
        // 2026-09-07 - T2 - CME-SVC-B-2026-09-06 - Labor Day, 12:00 CT close.
        (2026, 9, 7, early_close(12 * 3_600), T2, "CME-SVC-B-2026-09-06"),
        // 2026-11-26 - T2 - CME-SVC-B-2026-11-25 - Thanksgiving, 12:00 CT close.
        (2026, 11, 26, early_close(12 * 3_600), T2, "CME-SVC-B-2026-11-25"),
        // 2026-11-27 - T2 - CME-SVC-B-2026-11-25 - day after Thanksgiving, 12:15 CT close.
        (2026, 11, 27, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-B-2026-11-25"),
        // 2026-12-24 - T2 - CME-SVC-B-2026-12-24 - Christmas Eve, 12:15 CT close.
        (2026, 12, 24, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-B-2026-12-24"),
        // 2026-12-25 - T2 - CME-SVC-B-2026-12-24 - Christmas Day, no events published.
        (2026, 12, 25, Closed, T2, "CME-SVC-B-2026-12-24"),
        // 2027-01-01 - T2 - CME-SVC-B-2026-12-31 - New Year's Day, no events published.
        (2027, 1, 1, Closed, T2, "CME-SVC-B-2026-12-31"),
        // 2027-01-18 - T2 - CME-SVC-B-2027-01-17 - Martin Luther King Jr. Day, 12:00 CT close.
        (2027, 1, 18, early_close(12 * 3_600), T2, "CME-SVC-B-2027-01-17"),
        // 2027-02-15 - T2 - CME-SVC-B-2027-02-14 - Presidents' Day, 12:00 CT close.
        (2027, 2, 15, early_close(12 * 3_600), T2, "CME-SVC-B-2027-02-14"),
        // 2027-03-26 - T2 - CME-SVC-B-2027-03-25 - Good Friday, no events published.
        (2027, 3, 26, Closed, T2, "CME-SVC-B-2027-03-25"),
        // 2027-05-31 - T2 - CME-SVC-B-2027-05-30 - Memorial Day, 12:00 CT close.
        (2027, 5, 31, early_close(12 * 3_600), T2, "CME-SVC-B-2027-05-30"),
        // 2027-06-18 - T2 - CME-SVC-B-2027-06-17 - Juneteenth observed, 12:00 CT close.
        (2027, 6, 18, early_close(12 * 3_600), T2, "CME-SVC-B-2027-06-17"),
        // 2027-07-05 - T2 - CME-SVC-B-2027-07-04 - Independence Day observed, 12:00 CT close.
        (2027, 7, 5, early_close(12 * 3_600), T2, "CME-SVC-B-2027-07-04"),
        // 2027-09-06 - T2 - CME-SVC-B-2027-09-05 - Labor Day, 12:00 CT close.
        (2027, 9, 6, early_close(12 * 3_600), T2, "CME-SVC-B-2027-09-05"),
        // 2027-11-25 - T2 - CME-SVC-B-2027-11-24 - Thanksgiving, 12:00 CT close.
        (2027, 11, 25, early_close(12 * 3_600), T2, "CME-SVC-B-2027-11-24"),
        // 2027-11-26 - T2 - CME-SVC-B-2027-11-24 - day after Thanksgiving, 12:15 CT close.
        (2027, 11, 26, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-B-2027-11-24"),
        // 2027-12-24 - T2 - CME-SVC-B-2027-12-22 - Christmas Friday closure, no events.
        (2027, 12, 24, Closed, T2, "CME-SVC-B-2027-12-22"),
    ],
};
