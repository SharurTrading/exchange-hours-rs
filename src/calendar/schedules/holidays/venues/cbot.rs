//! The `Exchange::Cbot` table holiday rows — the `globex_grains` ∩ `globex_interest_rates`.
//!
//! Derived, not retrieved: the rows are the intersection of the families that
//! route to this venue, by the rule and routing recorded in
//! [`super`](index.html). A row ships only where every routed family states
//! it; a disagreement ships [`HolidayKind::Unsourced`], which clips nothing
//! and tells the caller the date is special without inventing an instant.
//!
//! Coverage is three audited eras: 2010-2012, whose rows are T1 (CME's own
//! holiday-calendar PDFs); 2016-2018, whose rows are the D17 intersection of
//! the routed families' T1 rows from CME's own published Globex holiday
//! schedules; and 2025-2027, whose rows are T2 (the trading-hours service). The
//! 2013-2015 and 2019-2024 intervals between them are audited by no wave and
//! lie outside every declared window.
//!
//! On the 2016-2018 era's thirty-six dates this table states nine `Closed`
//! rows and withholds the other twenty-seven as [`HolidayKind::Unsourced`].
//! Four are the three Thanksgiving Fridays and 2018-12-24, where the grain day
//! session closes at 12:05 CT and the rate leg at 12:15 CT; the other
//! twenty-three are dates `globex_grains` states a row on and
//! `globex_interest_rates` audited normal — the eighteen Monday and Thursday
//! holidays, on which grains is shut and the rate leg trades its ordinary
//! hours, and 2016-12-23, 2017-07-03, 2017-12-22, 2018-07-03 and 2018-12-26,
//! the agricultural half-days and the day after Christmas. Both routed families
//! cover the era, so neither abstains.
//!
//! The derivation, the instant disagreements and every dropped date are in the
//! venue's own evidence file, and the per-family rows are in the family files.
//!
//! **What changed on 2026-09-13:** this module was one file holding four tables.
//! It is now one file per venue, because the 2010-2012 rows took the combined
//! file past the 500-line reviewability guard. No row moved between venues.

use super::super::{
    EvidenceTier::{T1, T2},
    HolidayKind::Closed,
    HolidayKind::Unsourced,
    HolidayTable, holidays,
};

/// The `Exchange::Cbot` table: `globex_grains` ∩ `globex_interest_rates`.
///
/// The two families trade the same building around different sessions, and the
/// day session is where they touch: every one of the nine full closures below
/// keeps both closed, while a holiday early close moves the two by a different
/// amount — the grain day session ends at 12:05 CT or 12:00 CT while the rate
/// leg halts at 15:15, 12:00, 10:15 or 13:30 CT by date — so those dates ship
/// `Unsourced`. 115 rows over three audited eras: twenty-four stated and 91
/// `Unsourced` (33 in 2010-2012, 27 in 2016-2018, and 31 in 2025-2027), with
/// the 2013-2015 and 2019-2024 intervals between the eras outside every
/// declared window.
// Evidence: docs/evidence/cbot.md
pub(crate) static CBOT: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2012, 12, 31), (2016, 1, 1) ..= (2018, 12, 31), (2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2010-01-01 - T1 - 2010-new-years.pdf - closed.
        (2010, 1, 1, Closed, T1, "2010-new-years.pdf @2010-02-15T05:16:52Z"),
        // 2010-01-15 - T1 - 2010-martin-luther-king.pdf - disagreement: grains no row; interest rates early close 15:15 CT.
        (2010, 1, 15, Unsourced, T1, "2010-martin-luther-king.pdf @2010-03-31T06:42:26Z"),
        // 2010-02-12 - T1 - 2010-presidents-day.pdf - disagreement: grains no row; interest rates early close 15:15 CT.
        (2010, 2, 12, Unsourced, T1, "2010-presidents-day.pdf @2010-02-15T06:46:41Z"),
        // 2010-04-02 - T1 - 2010-good-friday.pdf - disagreement: grains closed; interest rates early close 10:15 CT.
        (2010, 4, 2, Unsourced, T1, "2010-good-friday.pdf @2010-06-01T11:19:16Z"),
        // 2010-05-28 - T1 - 2010-memorial-day.pdf - disagreement: grains no row; interest rates early close 15:15 CT.
        (2010, 5, 28, Unsourced, T1, "2010-memorial-day.pdf @2010-06-01T09:42:25Z"),
        // 2010-07-02 - T1 - 2010-4th-of-july.pdf - disagreement: grains no row; interest rates early close 15:15 CT.
        (2010, 7, 2, Unsourced, T1, "2010-4th-of-july.pdf @2010-06-02T00:56:37Z"),
        // 2010-09-03 - T1 - 2010-labor-day.pdf - disagreement: grains no row; interest rates early close 15:15 CT.
        (2010, 9, 3, Unsourced, T1, "2010-labor-day.pdf @2010-06-02T00:56:41Z"),
        // 2010-10-08 - T1 - 2010-columbus-day.pdf - disagreement: grains no row; interest rates early close 15:15 CT.
        (2010, 10, 8, Unsourced, T1, "2010-columbus-day.pdf @2010-08-21T13:31:22Z"),
        // 2010-11-26 - T1 - 2010-thanksgiving.pdf - disagreement: grains early close 12:00 CT; interest rates early close 12:15 CT.
        (2010, 11, 26, Unsourced, T1, "2010-thanksgiving.pdf @2010-11-22T09:40:12Z"),
        // 2010-12-24 - T1 - 2010-christmas.pdf - closed.
        (2010, 12, 24, Closed, T1, "2010-christmas.pdf @2010-12-14T06:12:38Z"),
        // 2010-12-31 - T1 - 2011-new-years.pdf - disagreement: grains early close 12:00 CT; interest rates early close 12:15 CT.
        (2010, 12, 31, Unsourced, T1, "2011-new-years.pdf @2011-11-01T14:39:45Z"),
        // 2011-01-14 - T1 - 2011-martin-luther-king.pdf - disagreement: grains no row; interest rates early close 15:15 CT.
        (2011, 1, 14, Unsourced, T1, "2011-martin-luther-king.pdf @2011-10-28T02:34:29Z"),
        // 2011-02-18 - T1 - 2011-presidents-day.pdf - disagreement: grains no row; interest rates early close 15:15 CT.
        (2011, 2, 18, Unsourced, T1, "2011-presidents-day.pdf @2011-10-28T02:35:16Z"),
        // 2011-04-22 - T1 - 2011-good-friday.pdf - closed.
        (2011, 4, 22, Closed, T1, "2011-good-friday.pdf @2011-10-28T02:37:07Z"),
        // 2011-05-27 - T1 - 2011-memorial-day.pdf - disagreement: grains no row; interest rates early close 15:15 CT.
        (2011, 5, 27, Unsourced, T1, "2011-memorial-day.pdf @2013-09-30T10:56:52Z"),
        // 2011-07-01 - T1 - 2011-4th-of-july.pdf - disagreement: grains no row; interest rates early close 15:15 CT.
        (2011, 7, 1, Unsourced, T1, "2011-4th-of-july.pdf @2011-11-01T14:40:54Z"),
        // 2011-09-02 - T1 - 2011-labor-day.pdf - disagreement: grains no row; interest rates early close 15:15 CT.
        (2011, 9, 2, Unsourced, T1, "2011-labor-day.pdf @2011-11-01T14:43:45Z"),
        // 2011-10-07 - T1 - 2011-columbus-day.pdf - disagreement: grains no row; interest rates early close 15:15 CT.
        (2011, 10, 7, Unsourced, T1, "2011-columbus-day.pdf @2011-11-01T14:39:16Z"),
        // 2011-11-25 - T1 - 2011-thanksgiving.pdf - disagreement: grains early close 12:00 CT; interest rates early close 12:15 CT.
        (2011, 11, 25, Unsourced, T1, "2011-thanksgiving.pdf @2011-11-24T18:52:46Z"),
        // 2011-12-26 - T1 - 2011-christmas.pdf - closed.
        (2011, 12, 26, Closed, T1, "2011-christmas.pdf @2012-01-25T02:05:48Z"),
        // 2011-12-27 - T1 - 2011-christmas.pdf - disagreement: grains late open 09:30 CT; interest rates late open 05:00 CT.
        (2011, 12, 27, Unsourced, T1, "2011-christmas.pdf @2012-01-25T02:05:48Z"),
        // 2012-01-02 - T1 - 2012-new-years.pdf - closed.
        (2012, 1, 2, Closed, T1, "2012-new-years.pdf @2012-01-25T02:54:30Z"),
        // 2012-01-03 - T1 - 2012-new-years.pdf - disagreement: grains late open 09:30 CT; interest rates late open 05:00 CT.
        (2012, 1, 3, Unsourced, T1, "2012-new-years.pdf @2012-01-25T02:54:30Z"),
        // 2012-01-13 - T1 - 2012-martin-luther-king.pdf - disagreement: grains no row; interest rates early close 15:15 CT.
        (2012, 1, 13, Unsourced, T1, "2012-martin-luther-king.pdf @2012-05-05T16:15:26Z"),
        // 2012-02-17 - T1 - 2012-presidents-day.pdf - disagreement: grains no row; interest rates early close 15:15 CT.
        (2012, 2, 17, Unsourced, T1, "2012-presidents-day.pdf @2012-05-05T16:15:39Z"),
        // 2012-04-06 - T1 - 2012-good-friday.pdf - disagreement: grains closed; interest rates early close 10:15 CT.
        (2012, 4, 6, Unsourced, T1, "2012-good-friday.pdf @2012-04-17T00:42:47Z"),
        // 2012-05-25 - T1 - 2012-memorial-day.pdf - disagreement: grains no row; interest rates early close 15:15 CT.
        (2012, 5, 25, Unsourced, T1, "2012-memorial-day.pdf @2012-09-15T00:37:14Z"),
        // 2012-05-28 - T1 - 2012-memorial-day.pdf - disagreement: grains late open 19:00 CT; interest rates no row.
        (2012, 5, 28, Unsourced, T1, "2012-memorial-day.pdf @2012-09-15T00:37:14Z"),
        // 2012-07-03 - T1 - 2012-4th-of-july.pdf - disagreement: grains early close 12:00 CT; interest rates no row.
        (2012, 7, 3, Unsourced, T1, "2012-4th-of-july.pdf @2012-09-15T00:39:23Z"),
        // 2012-07-04 - T1 - 2012-4th-of-july.pdf - disagreement: grains closed; interest rates no row.
        (2012, 7, 4, Unsourced, T1, "2012-4th-of-july.pdf @2012-09-15T00:39:23Z"),
        // 2012-07-05 - T1 - 2012-4th-of-july.pdf - disagreement: grains late open 09:30 CT; interest rates no row.
        (2012, 7, 5, Unsourced, T1, "2012-4th-of-july.pdf @2012-09-15T00:39:23Z"),
        // 2012-08-31 - T1 - 2012-labor-day.pdf - disagreement: grains no row; interest rates early close 15:15 CT.
        (2012, 8, 31, Unsourced, T1, "2012-labor-day.pdf @2012-09-15T00:34:37Z"),
        // 2012-09-03 - T1 - 2012-labor-day.pdf - disagreement: grains late open 19:00 CT; interest rates no row.
        (2012, 9, 3, Unsourced, T1, "2012-labor-day.pdf @2012-09-15T00:34:37Z"),
        // 2012-10-05 - T1 - 2012-columbus-day.pdf - disagreement: grains no row; interest rates early close 15:15 CT.
        (2012, 10, 5, Unsourced, T1, "2012-columbus-day.pdf @2012-09-15T00:15:14Z"),
        // 2012-11-22 - T1 - 2012-thanksgiving.pdf - disagreement: grains closed; interest rates no row.
        (2012, 11, 22, Unsourced, T1, "2012-thanksgiving.pdf @2013-01-27T22:39:01Z"),
        // 2012-11-23 - T1 - 2012-thanksgiving.pdf - disagreement: grains late open 09:30 CT and early close 12:00 CT; interest rates early close 12:15 CT.
        (2012, 11, 23, Unsourced, T1, "2012-thanksgiving.pdf @2013-01-27T22:39:01Z"),
        // 2012-12-24 - T1 - 2012-christmas.pdf - disagreement: grains early close 12:00 CT; interest rates early close 12:15 CT.
        (2012, 12, 24, Unsourced, T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        // 2012-12-25 - T1 - 2012-christmas.pdf - closed.
        (2012, 12, 25, Closed, T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        // 2012-12-26 - T1 - 2012-christmas.pdf - disagreement: grains late open 09:30 CT; interest rates late open 05:00 CT.
        (2012, 12, 26, Unsourced, T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        // 2016-01-01 - T1 - 2016-new-years-holiday-schedule.pdf @2016-01-08 - closed: no trade date.
        (2016, 1, 1, Closed, T1, "2016-new-years-holiday-schedule.pdf @2016-01-08"),
        // 2016-01-18 - T1 - 2016-holiday-calendars.zip#2016-martin-luther-king-holiday-schedule.pdf @2017-06-28 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2016, 1, 18, Unsourced, T1, "2016-holiday-calendars.zip#2016-martin-luther-king-holiday-schedule.pdf @2017-06-28"),
        // 2016-02-15 - T1 - 2016-holiday-calendars.zip#2016-presidents-day-holiday-schedule.pdf @2017-06-28 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2016, 2, 15, Unsourced, T1, "2016-holiday-calendars.zip#2016-presidents-day-holiday-schedule.pdf @2017-06-28"),
        // 2016-03-25 - T1 - 2016-holiday-calendars.zip#2016-good-friday-holiday-schedule.pdf @2017-06-28 - closed: no trade date.
        (2016, 3, 25, Closed, T1, "2016-holiday-calendars.zip#2016-good-friday-holiday-schedule.pdf @2017-06-28"),
        // 2016-05-30 - T1 - 2016-holiday-calendars.zip#2016-memorial-day-holiday-schedule.pdf @2017-06-28 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2016, 5, 30, Unsourced, T1, "2016-holiday-calendars.zip#2016-memorial-day-holiday-schedule.pdf @2017-06-28"),
        // 2016-07-04 - T1 - 2016-holiday-calendars.zip#2016-4th-of-july-holiday-schedule.pdf @2017-06-28 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2016, 7, 4, Unsourced, T1, "2016-holiday-calendars.zip#2016-4th-of-july-holiday-schedule.pdf @2017-06-28"),
        // 2016-09-05 - T1 - 2016-holiday-calendars.zip#2016-labor-day-holiday-schedule.pdf @2017-06-28 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2016, 9, 5, Unsourced, T1, "2016-holiday-calendars.zip#2016-labor-day-holiday-schedule.pdf @2017-06-28"),
        // 2016-11-24 - T1 - 2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2016, 11, 24, Unsourced, T1, "2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28"),
        // 2016-11-25 - T1 - 2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28 - disagreement: grains late open and early close 12:05 CT; interest rates early close 12:15 CT.
        (2016, 11, 25, Unsourced, T1, "2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28"),
        // 2016-12-23 - T1 - 2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28 - disagreement: grains early close 12:05 CT; interest rates no row.
        (2016, 12, 23, Unsourced, T1, "2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28"),
        // 2016-12-26 - T1 - 2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28 - closed: no trade date.
        (2016, 12, 26, Closed, T1, "2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28"),
        // 2017-01-02 - T1 - 2016-holiday-calendars.zip#2017-new-years-holiday-schedule.pdf @2017-06-28 - closed: no trade date.
        (2017, 1, 2, Closed, T1, "2016-holiday-calendars.zip#2017-new-years-holiday-schedule.pdf @2017-06-28"),
        // 2017-01-16 - T1 - 2017-martin-luther-king-holiday-schedule.xls @2017-06-28 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2017, 1, 16, Unsourced, T1, "2017-martin-luther-king-holiday-schedule.xls @2017-06-28"),
        // 2017-02-20 - T1 - 2017-presidents-day-holiday-schedule.xls @2017-06-28 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2017, 2, 20, Unsourced, T1, "2017-presidents-day-holiday-schedule.xls @2017-06-28"),
        // 2017-04-14 - T1 - 2017-good-friday-holiday-schedule.xls @2017-05-05 - closed: no trade date.
        (2017, 4, 14, Closed, T1, "2017-good-friday-holiday-schedule.xls @2017-05-05"),
        // 2017-05-29 - T1 - 2017-memorial-day-holiday-schedule.xls @2017-10-25 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2017, 5, 29, Unsourced, T1, "2017-memorial-day-holiday-schedule.xls @2017-10-25"),
        // 2017-07-03 - T1 - 2017-4th-of-july-holiday-schedule.xls @2017-10-25 - disagreement: grains early close 12:05 CT; interest rates no row.
        (2017, 7, 3, Unsourced, T1, "2017-4th-of-july-holiday-schedule.xls @2017-10-25"),
        // 2017-07-04 - T1 - 2017-4th-of-july-holiday-schedule.xls @2017-10-25 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2017, 7, 4, Unsourced, T1, "2017-4th-of-july-holiday-schedule.xls @2017-10-25"),
        // 2017-09-04 - T1 - 2017-labor-day-holiday-schedule.xls @2017-10-25 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2017, 9, 4, Unsourced, T1, "2017-labor-day-holiday-schedule.xls @2017-10-25"),
        // 2017-11-23 - T1 - 2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2017, 11, 23, Unsourced, T1, "2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26"),
        // 2017-11-24 - T1 - 2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26 - disagreement: grains late open and early close 12:05 CT; interest rates early close 12:15 CT.
        (2017, 11, 24, Unsourced, T1, "2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26"),
        // 2017-12-22 - T1 - 2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26 - disagreement: grains early close 12:05 CT; interest rates no row.
        (2017, 12, 22, Unsourced, T1, "2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26"),
        // 2017-12-25 - T1 - 2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26 - closed: no trade date.
        (2017, 12, 25, Closed, T1, "2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26"),
        // 2018-01-01 - T1 - 2018-new-years-holiday-schedule.xls @2018-01-06 - closed: no trade date.
        (2018, 1, 1, Closed, T1, "2018-new-years-holiday-schedule.xls @2018-01-06"),
        // 2018-01-15 - T1 - 2018-martin-luther-king-holiday-schedule.xls @2018-05-08 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2018, 1, 15, Unsourced, T1, "2018-martin-luther-king-holiday-schedule.xls @2018-05-08"),
        // 2018-02-19 - T1 - 2018-presidents-day-holiday-schedule.xls @2018-05-08 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2018, 2, 19, Unsourced, T1, "2018-presidents-day-holiday-schedule.xls @2018-05-08"),
        // 2018-03-30 - T1 - 2018-holiday-calendars.zip#2018-good-friday-holiday-schedule.xls @2026-08-30 - closed: no trade date.
        (2018, 3, 30, Closed, T1, "2018-holiday-calendars.zip#2018-good-friday-holiday-schedule.xls @2026-08-30"),
        // 2018-05-28 - T1 - 2018-holiday-calendars.zip#2018-memorial-day-holiday-schedule.xls @2026-08-30 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2018, 5, 28, Unsourced, T1, "2018-holiday-calendars.zip#2018-memorial-day-holiday-schedule.xls @2026-08-30"),
        // 2018-07-03 - T1 - 2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30 - disagreement: grains early close 12:05 CT; interest rates no row.
        (2018, 7, 3, Unsourced, T1, "2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30"),
        // 2018-07-04 - T1 - 2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2018, 7, 4, Unsourced, T1, "2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30"),
        // 2018-09-03 - T1 - 2018-holiday-calendars.zip#2018-labor-day-holiday-schedule.xls @2026-08-30 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2018, 9, 3, Unsourced, T1, "2018-holiday-calendars.zip#2018-labor-day-holiday-schedule.xls @2026-08-30"),
        // 2018-11-22 - T1 - 2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2018, 11, 22, Unsourced, T1, "2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30"),
        // 2018-11-23 - T1 - 2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30 - disagreement: grains late open and early close 12:05 CT; interest rates early close 12:15 CT.
        (2018, 11, 23, Unsourced, T1, "2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30"),
        // 2018-12-24 - T1 - 2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30 - disagreement: grains early close 12:05 CT; interest rates early close 12:15 CT.
        (2018, 12, 24, Unsourced, T1, "2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30"),
        // 2018-12-25 - T1 - 2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30 - closed: no trade date.
        (2018, 12, 25, Closed, T1, "2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30"),
        // 2018-12-26 - T1 - 2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30 - disagreement: grains late open 08:30 CT; interest rates no row.
        (2018, 12, 26, Unsourced, T1, "2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30"),
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-02 - T2 - CME-SVC-2024-12-31 - grains late open 08:30 CT;
        // no row in interest rates.
        (2025, 1, 2, Unsourced, T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 - T2 - CME-SVC-2025-01-19 - grains closed, interest rates
        // early close 12:00 CT.
        (2025, 1, 20, Unsourced, T2, "CME-SVC-2025-01-19"),
        // 2025-02-17 - T2 - CME-SVC-2025-02-16 - as 2025-01-20.
        (2025, 2, 17, Unsourced, T2, "CME-SVC-2025-02-16"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-17 - both families closed.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 - T2 - CME-SVC-2025-05-25 - as 2025-01-20.
        (2025, 5, 26, Unsourced, T2, "CME-SVC-2025-05-25"),
        // 2025-06-19 - T2 - CME-SVC-2025-06-18 - as 2025-01-20.
        (2025, 6, 19, Unsourced, T2, "CME-SVC-2025-06-18"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-03 - as 2025-01-20.
        (2025, 7, 4, Unsourced, T2, "CME-SVC-2025-07-03"),
        // 2025-09-01 - T2 - CME-SVC-2025-08-31 - as 2025-01-20.
        (2025, 9, 1, Unsourced, T2, "CME-SVC-2025-08-31"),
        // 2025-11-27 - T2 - CME-SVC-2025-11-26-SAT - as 2025-01-20.
        (2025, 11, 27, Unsourced, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-11-28 - T2 - CME-SVC-2025-11-26 - grains 08:30 CT late open
        // into a 12:05 CT close, interest rates 12:15 CT early close.
        (2025, 11, 28, Unsourced, T2, "CME-SVC-2025-11-26"),
        // 2025-11-29 - T2 - CME-SVC-2025-11-26-SAT - both families closed.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-12-24 - T2 - CME-SVC-2025-12-24 - grains 12:05 CT, interest
        // rates 12:15 CT.
        (2025, 12, 24, Unsourced, T2, "CME-SVC-2025-12-24"),
        // 2025-12-25 - T2 - CME-SVC-2025-12-24 - both families closed.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-24"),
        // 2025-12-26 - T2 - CME-SVC-2025-12-24 - grains late open 08:30 CT;
        // no row in interest rates.
        (2025, 12, 26, Unsourced, T2, "CME-SVC-2025-12-24"),
        // 2026-01-01 - T2 - CME-SVC-2025-12-31 - both families closed.
        (2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-01-02 - T2 - CME-SVC-2025-12-31 - grains late open 08:30 CT;
        // no row in interest rates.
        (2026, 1, 2, Unsourced, T2, "CME-SVC-2025-12-31"),
        // 2026-01-19 - T2 - CME-SVC-2026-01-18 - as 2025-01-20.
        (2026, 1, 19, Unsourced, T2, "CME-SVC-2026-01-18"),
        // 2026-02-16 - T2 - CME-SVC-2026-02-15 - as 2025-01-20.
        (2026, 2, 16, Unsourced, T2, "CME-SVC-2026-02-15"),
        // 2026-04-03 - T2 - CME-SVC-2026-04-01 - grains closed, interest rates
        // early close 10:15 CT.
        (2026, 4, 3, Unsourced, T2, "CME-SVC-2026-04-01"),
        // 2026-05-25 - T2 - CME-SVC-2026-05-24 - as 2025-01-20.
        (2026, 5, 25, Unsourced, T2, "CME-SVC-2026-05-24"),
        // 2026-06-19 - T2 - CME-SVC-2026-06-18 - as 2025-01-20.
        (2026, 6, 19, Unsourced, T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - as 2025-01-20.
        (2026, 7, 3, Unsourced, T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 - T2 - CME-SVC-2026-09-06 - as 2025-01-20.
        (2026, 9, 7, Unsourced, T2, "CME-SVC-2026-09-06"),
        // 2026-11-26 - T2 - CME-SVC-2026-11-25 - as 2025-01-20.
        (2026, 11, 26, Unsourced, T2, "CME-SVC-2026-11-25"),
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - as 2025-11-28.
        (2026, 11, 27, Unsourced, T2, "CME-SVC-2026-11-25"),
        // 2026-12-24 - T2 - CME-SVC-2026-12-22 - grains 12:05 CT, interest
        // rates 12:15 CT.
        (2026, 12, 24, Unsourced, T2, "CME-SVC-2026-12-22"),
        // 2026-12-25 - T2 - CME-SVC-2026-12-24 - both families closed.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 - T2 - CME-SVC-2026-12-31 - both families closed.
        (2027, 1, 1, Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-01-18 - T2 - CME-SVC-2027-01-17 - as 2025-01-20.
        (2027, 1, 18, Unsourced, T2, "CME-SVC-2027-01-17"),
        // 2027-02-15 - T2 - CME-SVC-2027-02-14 - as 2025-01-20.
        (2027, 2, 15, Unsourced, T2, "CME-SVC-2027-02-14"),
        // 2027-03-26 - T2 - CME-SVC-2027-03-25 - both families closed.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 - T2 - CME-SVC-2027-05-30 - as 2025-01-20.
        (2027, 5, 31, Unsourced, T2, "CME-SVC-2027-05-30"),
        // 2027-06-18 - T2 - CME-SVC-2027-06-17 - as 2025-01-20.
        (2027, 6, 18, Unsourced, T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 - T2 - CME-SVC-2027-07-04 - grains closed, interest rates
        // early close 13:30 CT.
        (2027, 7, 5, Unsourced, T2, "CME-SVC-2027-07-04"),
        // 2027-07-06 - T2 - CME-SVC-2027-07-04 - grains late open 08:30 CT;
        // no row in interest rates.
        (2027, 7, 6, Unsourced, T2, "CME-SVC-2027-07-04"),
        // 2027-09-06 - T2 - CME-SVC-2027-09-05 - as 2025-01-20.
        (2027, 9, 6, Unsourced, T2, "CME-SVC-2027-09-05"),
        // 2027-11-25 - T2 - CME-SVC-2027-11-24 - as 2025-01-20.
        (2027, 11, 25, Unsourced, T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - as 2025-11-28.
        (2027, 11, 26, Unsourced, T2, "CME-SVC-2027-11-24"),
        // 2027-12-24 - T2 - CME-SVC-2027-12-22 - both families closed.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
