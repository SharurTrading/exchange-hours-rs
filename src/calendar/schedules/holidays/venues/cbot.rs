//! The `Exchange::Cbot` table holiday rows — the `globex_grains` ∩ `globex_interest_rates`.
//!
//! Derived, not retrieved: the rows are the intersection of the families that
//! route to this venue, by the rule and routing recorded in
//! [`super`](index.html). A row ships only where every routed family states
//! it; a disagreement ships [`HolidayKind::Unsourced`], which clips nothing
//! and tells the caller the date is special without inventing an instant.
//!
//! Coverage is 6 audited eras: 2010-2012 and 2013-2015, whose rows are T1
//! (CME's own holiday-calendar PDFs and .xls workbooks); 2016-2018, 2019-2021
//! and 2022-2024, whose rows are
//! the D17 intersection of the routed families' T1 rows from CME's own
//! published Globex holiday schedules — all of 2019-2021 at T1, and 2022-2024
//! except for the three 2023 markers and the 2024 dates the trading-hours
//! service answers; and 2025-2027, whose rows are T2 (that service).
//!
//! On the 2016-2018 era's thirty-six dates this table states nine `Closed`
//! rows and withholds the other twenty-seven as [`HolidayKind::Unsourced`].
//! Four are the three Thanksgiving Fridays and 2018-12-24, where the grain day
//! session closes at 12:05 CT and the rate leg at 12:15 CT. The other
//! twenty-three split two ways: the eighteen Monday and Thursday holidays, on
//! which `globex_grains` is shut outright while `globex_interest_rates` states an
//! early close at 12:00 CT — different answers, so the date is disputed — and
//! 2016-12-23, 2017-07-03, 2017-12-22, 2018-07-03 and 2018-12-26, the
//! agricultural half-days and the day after Christmas, on which `globex_grains`
//! states a row and the rate leg audited the date normal. Both routed families
//! cover the era, so neither abstains.
//!
//! On the 2019-2021 era's **forty-two dates** this table states eight `Closed`
//! rows and withholds the other thirty-four as [`HolidayKind::Unsourced`].
//! Eighteen are the Monday and Thursday holidays, on which grains is shut while
//! the rate leg halts at 12:00 CT; five are dates grains states a late open at
//! 08:30 CT on and the rate leg audited normal (2019-01-02, 2019-07-05,
//! 2019-12-26, 2020-01-02 and 2021-07-06); eight are the two families' own
//! disagreements — the three Thanksgiving Fridays, on which grains reopens at
//! 08:30 CT and closes at 12:05 CT while the rate leg halts at 12:15 CT; the
//! two Christmas Eves, on which grains closes at 12:05 CT and the rate leg at
//! 12:15 CT; 2019-07-03 and 2020-07-02, on which grains closes at 12:05 CT and
//! the rate leg audited the date normal; and 2021-04-02, on which grains is
//! shut and the rate leg closes at 10:15 CT; and three are 2019-06-19,
//! 2020-06-19 and 2021-06-19, where both routed families state `Unsourced` —
//! the wave did not work those dates up — so the venue ships the families' own
//! marker rather than a dispute. Both routed families cover the era, so
//! neither abstains.
//!
//! On the 2022-2024 era's **thirty-nine dates** this table states seven `Closed`
//! rows and withholds the other thirty-two as [`HolidayKind::Unsourced`]. Four
//! are the three Thanksgiving Fridays and 2024-12-24, where the grain day
//! session closes at 12:05 CT and the rate leg at 12:15 CT; nineteen are the
//! Monday and Thursday holidays, on which grains is shut while the rate leg
//! halts at 12:00 CT; six are dates grains states a late open on and the rate
//! leg audited normal (2022-07-05, 2023-07-05, 2023-12-26, 2024-01-02,
//! 2024-07-05 and 2024-12-26); and three are 2023-01-16, 2023-02-20 and
//! 2023-04-07, where both routed families state `Unsourced` — the wave did not
//! work those dates up — so the venue ships the families' own marker rather
//! than a dispute.
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
/// day session is where they touch: every one of the forty-seven full closures
/// below keeps both closed, while a holiday early close moves the two by a
/// different amount — the grain day session ends at 12:05 CT or 12:00 CT while
/// the rate leg halts at 15:15, 12:00, 10:15 or 13:30 CT by date — so those
/// dates ship `Unsourced`. 266 rows over six audited eras: forty-seven
/// stated and 219 `Unsourced` (33 in 2010-2012, 46 in 2013-2015,
/// 27 in 2016-2018, 34 in 2019-2021, 32 in 2022-2024 and 47 in 2025-2027).
// Evidence: docs/evidence/cbot.md
pub(crate) static CBOT: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2012, 12, 31), (2013, 1, 1) ..= (2015, 12, 31), (2016, 1, 1) ..= (2018, 12, 31), (2019, 1, 1) ..= (2021, 12, 31), (2022, 1, 1) ..= (2024, 12, 31), (2025, 1, 1) ..= (2027, 12, 31)],
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
        // 2013-01-01 - T1 - 2013-new-years.pdf @2013-04-14T19:41:46Z - closed: no trade date.
        (2013, 1, 1, Closed, T1, "2013-new-years.pdf @2013-04-14T19:41:46Z"),
        // 2013-01-02 - T1 - 2013-new-years.pdf @2013-04-14T19:41:46Z - disagreement: grains late open 09:30 CT; interest rates late open 05:00 CT.
        (2013, 1, 2, Unsourced, T1, "2013-new-years.pdf @2013-04-14T19:41:46Z"),
        // 2013-01-18 - T1 - 2013-martin-luther-king.pdf @2012-11-19T00:16:09Z - disagreement: grains no row; interest rates early close 15:15 CT.
        (2013, 1, 18, Unsourced, T1, "2013-martin-luther-king.pdf @2012-11-19T00:16:09Z"),
        // 2013-01-21 - T1 - 2013-martin-luther-king.pdf @2012-11-19T00:16:09Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2013, 1, 21, Unsourced, T1, "2013-martin-luther-king.pdf @2012-11-19T00:16:09Z"),
        // 2013-02-15 - T1 - 2013-presidents-day.pdf @2013-03-09T11:53:37Z - disagreement: grains no row; interest rates early close 15:15 CT.
        (2013, 2, 15, Unsourced, T1, "2013-presidents-day.pdf @2013-03-09T11:53:37Z"),
        // 2013-02-18 - T1 - 2013-presidents-day.pdf @2013-03-09T11:53:37Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2013, 2, 18, Unsourced, T1, "2013-presidents-day.pdf @2013-03-09T11:53:37Z"),
        // 2013-03-29 - T1 - 2013-good-friday.pdf @2013-06-23T19:59:25Z - closed: no trade date.
        (2013, 3, 29, Closed, T1, "2013-good-friday.pdf @2013-06-23T19:59:25Z"),
        // 2013-05-24 - T1 - 2013-memorial-day.pdf @2013-06-23T20:36:04Z - disagreement: grains no row; interest rates early close 15:15 CT.
        (2013, 5, 24, Unsourced, T1, "2013-memorial-day.pdf @2013-06-23T20:36:04Z"),
        // 2013-05-27 - T1 - 2013-memorial-day.pdf @2013-06-23T20:36:04Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2013, 5, 27, Unsourced, T1, "2013-memorial-day.pdf @2013-06-23T20:36:04Z"),
        // 2013-07-03 - T1 - 2013-4th-of-july.pdf @2013-06-23T20:58:25Z - disagreement: grains early close 12:00 CT; interest rates no row.
        (2013, 7, 3, Unsourced, T1, "2013-4th-of-july.pdf @2013-06-23T20:58:25Z"),
        // 2013-07-04 - T1 - 2013-4th-of-july.pdf @2013-06-23T20:58:25Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2013, 7, 4, Unsourced, T1, "2013-4th-of-july.pdf @2013-06-23T20:58:25Z"),
        // 2013-07-05 - T1 - 2013-4th-of-july.pdf @2013-06-23T20:58:25Z - disagreement: grains late open 08:30 CT; interest rates no row.
        (2013, 7, 5, Unsourced, T1, "2013-4th-of-july.pdf @2013-06-23T20:58:25Z"),
        // 2013-08-30 - T1 - 2013-labor-day.pdf @2013-09-02T17:08:41Z - disagreement: grains no row; interest rates early close 15:15 CT.
        (2013, 8, 30, Unsourced, T1, "2013-labor-day.pdf @2013-09-02T17:08:41Z"),
        // 2013-09-02 - T1 - 2013-labor-day.pdf @2013-09-02T17:08:41Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2013, 9, 2, Unsourced, T1, "2013-labor-day.pdf @2013-09-02T17:08:41Z"),
        // 2013-11-28 - T1 - 2013-thanksgiving.pdf @2014-02-14T06:28:36Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2013, 11, 28, Unsourced, T1, "2013-thanksgiving.pdf @2014-02-14T06:28:36Z"),
        // 2013-11-29 - T1 - 2013-thanksgiving.pdf @2014-02-14T06:28:36Z - disagreement: grains late open 08:30 CT and early close 12:00 CT; interest rates early close 12:15 CT.
        (2013, 11, 29, Unsourced, T1, "2013-thanksgiving.pdf @2014-02-14T06:28:36Z"),
        // 2013-12-24 - T1 - 2013-christmas.pdf @2014-04-12T06:24:28Z - disagreement: grains early close 12:00 CT; interest rates early close 12:15 CT.
        (2013, 12, 24, Unsourced, T1, "2013-christmas.pdf @2014-04-12T06:24:28Z"),
        // 2013-12-25 - T1 - 2013-christmas.pdf @2014-04-12T06:24:28Z - closed: no trade date.
        (2013, 12, 25, Closed, T1, "2013-christmas.pdf @2014-04-12T06:24:28Z"),
        // 2013-12-26 - T1 - 2013-christmas.pdf @2014-04-12T06:24:28Z - disagreement: grains late open 08:30 CT; interest rates late open 05:00 CT.
        (2013, 12, 26, Unsourced, T1, "2013-christmas.pdf @2014-04-12T06:24:28Z"),
        // 2014-01-01 - T1 - 2014-new-years.pdf @2013-10-07T20:58:00Z - closed: no trade date.
        (2014, 1, 1, Closed, T1, "2014-new-years.pdf @2013-10-07T20:58:00Z"),
        // 2014-01-02 - T1 - 2014-new-years.pdf @2013-10-07T20:58:00Z - disagreement: grains late open 08:30 CT; interest rates late open 05:00 CT.
        (2014, 1, 2, Unsourced, T1, "2014-new-years.pdf @2013-10-07T20:58:00Z"),
        // 2014-01-17 - T1 - 2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z - disagreement: grains no row; interest rates early close 15:15 CT.
        (2014, 1, 17, Unsourced, T1, "2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z"),
        // 2014-01-20 - T1 - 2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2014, 1, 20, Unsourced, T1, "2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z"),
        // 2014-02-14 - T1 - 2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z - disagreement: grains no row; interest rates early close 15:15 CT.
        (2014, 2, 14, Unsourced, T1, "2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z"),
        // 2014-02-17 - T1 - 2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2014, 2, 17, Unsourced, T1, "2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z"),
        // 2014-04-18 - T1 - 2014-good-friday-holiday-schedule.pdf @2014-03-26T15:27:35Z - closed: no trade date.
        (2014, 4, 18, Closed, T1, "2014-good-friday-holiday-schedule.pdf @2014-03-26T15:27:35Z"),
        // 2014-05-23 - T1 - 2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z - disagreement: grains no row; interest rates early close 15:15 CT.
        (2014, 5, 23, Unsourced, T1, "2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z"),
        // 2014-05-26 - T1 - 2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2014, 5, 26, Unsourced, T1, "2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z"),
        // 2014-07-03 - T1 - 2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z - disagreement: grains early close 12:00 CT; interest rates no row.
        (2014, 7, 3, Unsourced, T1, "2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z"),
        // 2014-07-04 - T1 - 2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2014, 7, 4, Unsourced, T1, "2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z"),
        // 2014-07-07 - T1 - 2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z - disagreement: grains late open 08:30 CT; interest rates no row.
        (2014, 7, 7, Unsourced, T1, "2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z"),
        // 2014-08-29 - T1 - 2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z - disagreement: grains no row; interest rates early close 15:15 CT.
        (2014, 8, 29, Unsourced, T1, "2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z"),
        // 2014-09-01 - T1 - 2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2014, 9, 1, Unsourced, T1, "2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z"),
        // 2014-11-27 - T1 - 2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2014, 11, 27, Unsourced, T1, "2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z"),
        // 2014-11-28 - T1 - 2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z - disagreement: grains late open 08:30 CT and early close 12:00 CT; interest rates early close 12:15 CT.
        (2014, 11, 28, Unsourced, T1, "2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z"),
        // 2014-12-24 - T1 - 2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z - disagreement: grains early close 12:00 CT; interest rates early close 12:15 CT.
        (2014, 12, 24, Unsourced, T1, "2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z"),
        // 2014-12-25 - T1 - 2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z - closed: no trade date.
        (2014, 12, 25, Closed, T1, "2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z"),
        // 2014-12-26 - T1 - 2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z - disagreement: grains late open 08:30 CT; interest rates no row.
        (2014, 12, 26, Unsourced, T1, "2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z"),
        // 2015-01-01 - T1 - 2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z - closed: no trade date.
        (2015, 1, 1, Closed, T1, "2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z"),
        // 2015-01-02 - T1 - 2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z - disagreement: grains late open 08:30 CT; interest rates no row.
        (2015, 1, 2, Unsourced, T1, "2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z"),
        // 2015-01-16 - T1 - 2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z - disagreement: grains no row; interest rates early close 15:15 CT.
        (2015, 1, 16, Unsourced, T1, "2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z"),
        // 2015-01-19 - T1 - 2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2015, 1, 19, Unsourced, T1, "2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z"),
        // 2015-02-13 - T1 - 2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z - disagreement: grains no row; interest rates early close 15:15 CT.
        (2015, 2, 13, Unsourced, T1, "2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z"),
        // 2015-02-16 - T1 - 2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2015, 2, 16, Unsourced, T1, "2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z"),
        // 2015-04-03 - T1 - 2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z - disagreement: grains closed; interest rates early close 10:15 CT.
        (2015, 4, 3, Unsourced, T1, "2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z"),
        // 2015-05-22 - T1 - 2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z - disagreement: grains no row; interest rates early close 15:15 CT.
        (2015, 5, 22, Unsourced, T1, "2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z"),
        // 2015-05-25 - T1 - 2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2015, 5, 25, Unsourced, T1, "2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z"),
        // 2015-07-02 - T1 - 2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z - disagreement: grains early close 12:00 CT; interest rates no row.
        (2015, 7, 2, Unsourced, T1, "2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z"),
        // 2015-07-03 - T1 - 2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2015, 7, 3, Unsourced, T1, "2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z"),
        // 2015-09-07 - T1 - 2015-labor-day-holiday-schedule.pdf @2015-08-24T02:30:39Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2015, 9, 7, Unsourced, T1, "2015-labor-day-holiday-schedule.pdf @2015-08-24T02:30:39Z"),
        // 2015-11-26 - T1 - 2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2015, 11, 26, Unsourced, T1, "2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z"),
        // 2015-11-27 - T1 - 2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z - disagreement: grains late open 08:30 CT and early close 12:05 CT; interest rates early close 12:15 CT.
        (2015, 11, 27, Unsourced, T1, "2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z"),
        // 2015-12-24 - T1 - 2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z - disagreement: grains early close 12:05 CT; interest rates early close 12:15 CT.
        (2015, 12, 24, Unsourced, T1, "2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z"),
        // 2015-12-25 - T1 - 2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z - closed: no trade date.
        (2015, 12, 25, Closed, T1, "2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z"),
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
        // 2019-01-01 - T1 - 2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z - closed: no trade date.
        (2019, 1, 1, Closed, T1, "2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z"),
        // 2019-01-02 - T1 - 2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z - disagreement: grains late open 08:30 CT; interest rates no row.
        (2019, 1, 2, Unsourced, T1, "2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z"),
        // 2019-01-21 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-martin-luther-king-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2019, 1, 21, Unsourced, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-martin-luther-king-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-02-18 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-presidents-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2019, 2, 18, Unsourced, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-presidents-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-04-19 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-good-friday-holiday-compact.xls @2021-01-26T09:48:37Z - closed: no trade date.
        (2019, 4, 19, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-good-friday-holiday-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-05-27 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-memorial-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2019, 5, 27, Unsourced, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-memorial-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-06-19 - T1 - 2019-holiday-calendars.zip @2021-01-26T09:48:37Z - unsourced: the routed families state the date is not worked up.
        (2019, 6, 19, Unsourced, T1, "2019-holiday-calendars.zip @2021-01-26T09:48:37Z"),
        // 2019-07-03 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - disagreement: grains early close 12:05 CT; interest rates no row.
        (2019, 7, 3, Unsourced, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-07-04 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2019, 7, 4, Unsourced, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-07-05 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - disagreement: grains late open 08:30 CT; interest rates no row.
        (2019, 7, 5, Unsourced, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-09-02 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-labor-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2019, 9, 2, Unsourced, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-labor-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-11-28 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2019, 11, 28, Unsourced, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-11-29 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - disagreement: grains late open 08:30 CT and early close 12:05 CT; interest rates early close 12:15 CT.
        (2019, 11, 29, Unsourced, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-12-24 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - disagreement: grains early close 12:05 CT; interest rates early close 12:15 CT.
        (2019, 12, 24, Unsourced, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-12-25 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - closed: no trade date.
        (2019, 12, 25, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-12-26 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - disagreement: grains late open 08:30 CT; interest rates no row.
        (2019, 12, 26, Unsourced, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2020-01-01 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - closed: no trade date.
        (2020, 1, 1, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2020-01-02 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - disagreement: grains late open 08:30 CT; interest rates no row.
        (2020, 1, 2, Unsourced, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2020-01-20 - T1 - 2020-holiday-calendars.zip#2020-martin-luther-king-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2020, 1, 20, Unsourced, T1, "2020-holiday-calendars.zip#2020-martin-luther-king-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-02-17 - T1 - 2020-holiday-calendars.zip#2020-presidents-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2020, 2, 17, Unsourced, T1, "2020-holiday-calendars.zip#2020-presidents-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-04-10 - T1 - 2020-holiday-calendars.zip#2020-good-friday-holiday-compact.xls @2026-07-30T11:18:34Z - closed: no trade date.
        (2020, 4, 10, Closed, T1, "2020-holiday-calendars.zip#2020-good-friday-holiday-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-05-25 - T1 - 2020-holiday-calendars.zip#2020-memorial-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2020, 5, 25, Unsourced, T1, "2020-holiday-calendars.zip#2020-memorial-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-06-19 - T1 - 2020-holiday-calendars.zip @2026-07-30T11:18:34Z - unsourced: the routed families state the date is not worked up.
        (2020, 6, 19, Unsourced, T1, "2020-holiday-calendars.zip @2026-07-30T11:18:34Z"),
        // 2020-07-02 - T1 - 2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - disagreement: grains early close 12:05 CT; interest rates no row.
        (2020, 7, 2, Unsourced, T1, "2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-07-03 - T1 - 2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2020, 7, 3, Unsourced, T1, "2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-09-07 - T1 - 2020-holiday-calendars.zip#2020-labor-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2020, 9, 7, Unsourced, T1, "2020-holiday-calendars.zip#2020-labor-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-11-26 - T1 - 2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2020, 11, 26, Unsourced, T1, "2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-11-27 - T1 - 2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - disagreement: grains late open 08:30 CT and early close 12:05 CT; interest rates early close 12:15 CT.
        (2020, 11, 27, Unsourced, T1, "2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-12-24 - T1 - 2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - disagreement: grains early close 12:05 CT; interest rates early close 12:15 CT.
        (2020, 12, 24, Unsourced, T1, "2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-12-25 - T1 - 2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - closed: no trade date.
        (2020, 12, 25, Closed, T1, "2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2021-01-01 - T1 - 2020-holiday-calendars.zip#2021-new-years-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - closed: no trade date.
        (2021, 1, 1, Closed, T1, "2020-holiday-calendars.zip#2021-new-years-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2021-01-18 - T1 - 2021-holiday-calendars.zip#2021-mlk-day-schedule-compact.xls @2026-08-30T10:03:27Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2021, 1, 18, Unsourced, T1, "2021-holiday-calendars.zip#2021-mlk-day-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-02-15 - T1 - 2021-holiday-calendars.zip#2021-presidents-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2021, 2, 15, Unsourced, T1, "2021-holiday-calendars.zip#2021-presidents-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-04-02 - T1 - 2021-holiday-calendars.zip#2021-good-friday-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - disagreement: grains closed; interest rates early close 10:15 CT.
        (2021, 4, 2, Unsourced, T1, "2021-holiday-calendars.zip#2021-good-friday-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-05-31 - T1 - 2021-holiday-calendars.zip#2021-memorial-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2021, 5, 31, Unsourced, T1, "2021-holiday-calendars.zip#2021-memorial-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-06-19 - T1 - 2021-holiday-calendars.zip @2026-08-30T10:03:27Z - unsourced: the routed families state the date is not worked up.
        (2021, 6, 19, Unsourced, T1, "2021-holiday-calendars.zip @2026-08-30T10:03:27Z"),
        // 2021-07-05 - T1 - 2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2021, 7, 5, Unsourced, T1, "2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-07-06 - T1 - 2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - disagreement: grains late open 08:30 CT; interest rates no row.
        (2021, 7, 6, Unsourced, T1, "2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-09-06 - T1 - 2021-holiday-calendars.zip#2021-labor-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2021, 9, 6, Unsourced, T1, "2021-holiday-calendars.zip#2021-labor-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-11-25 - T1 - 2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2021, 11, 25, Unsourced, T1, "2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-11-26 - T1 - 2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - disagreement: grains late open 08:30 CT and early close 12:05 CT; interest rates early close 12:15 CT.
        (2021, 11, 26, Unsourced, T1, "2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-12-24 - T1 - 2021-holiday-calendars.zip#2021-christmas-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - closed: no trade date.
        (2021, 12, 24, Closed, T1, "2021-holiday-calendars.zip#2021-christmas-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2022-01-17 - T1 - 2022-mlk-day-holiday-schedule.xls @2022-01-17T21:22:30Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2022, 1, 17, Unsourced, T1, "2022-mlk-day-holiday-schedule.xls @2022-01-17T21:22:30Z"),
        // 2022-02-21 - T1 - 2022-presidents-day-holiday-schedule.xls @2022-07-04T07:38:10Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2022, 2, 21, Unsourced, T1, "2022-presidents-day-holiday-schedule.xls @2022-07-04T07:38:10Z"),
        // 2022-04-15 - T1 - 2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z - closed: no trade date.
        (2022, 4, 15, Closed, T1, "2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z"),
        // 2022-05-30 - T1 - 2022-memorial-day-holiday-schedule.xls @2022-07-04T06:54:38Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2022, 5, 30, Unsourced, T1, "2022-memorial-day-holiday-schedule.xls @2022-07-04T06:54:38Z"),
        // 2022-06-20 - T1 - 2022-juneteenth-holiday-schedule.xls @2022-06-20T20:02:10Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2022, 6, 20, Unsourced, T1, "2022-juneteenth-holiday-schedule.xls @2022-06-20T20:02:10Z"),
        // 2022-07-04 - T1 - 2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2022, 7, 4, Unsourced, T1, "2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z"),
        // 2022-07-05 - T1 - 2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z - disagreement: grains late open 08:30 CT; interest rates no row.
        (2022, 7, 5, Unsourced, T1, "2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z"),
        // 2022-09-05 - T1 - 2022-labor-day-holiday-schedule.xls @2022-07-04T06:54:41Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2022, 9, 5, Unsourced, T1, "2022-labor-day-holiday-schedule.xls @2022-07-04T06:54:41Z"),
        // 2022-11-24 - T1 - 2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2022, 11, 24, Unsourced, T1, "2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z"),
        // 2022-11-25 - T1 - 2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z - disagreement: grains late open 08:30 CT and early close 12:05 CT; interest rates early close 12:15 CT.
        (2022, 11, 25, Unsourced, T1, "2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z"),
        // 2022-12-26 - T1 - 2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z - closed: no trade date.
        (2022, 12, 26, Closed, T1, "2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z"),
        // 2023-01-02 - T1 - 2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z - closed: no trade date.
        (2023, 1, 2, Closed, T1, "2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z"),
        // 2023-01-16 - T2 - CME-SVC-2023-01-15 - unsourced: the routed families state the date is not worked up.
        (2023, 1, 16, Unsourced, T2, "CME-SVC-2023-01-15"),
        // 2023-02-20 - T2 - CME-SVC-2023-02-19 - unsourced: the routed families state the date is not worked up.
        (2023, 2, 20, Unsourced, T2, "CME-SVC-2023-02-19"),
        // 2023-04-07 - T2 - CME-SVC-2023-04-06 - unsourced: the routed families state the date is not worked up.
        (2023, 4, 7, Unsourced, T2, "CME-SVC-2023-04-06"),
        // 2023-05-29 - T1 - memorial-day-2023.pdf @2023-04-20T22:40:18Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2023, 5, 29, Unsourced, T1, "memorial-day-2023.pdf @2023-04-20T22:40:18Z"),
        // 2023-06-19 - T1 - juneteenth-2023.pdf @2023-06-13T18:59:49Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2023, 6, 19, Unsourced, T1, "juneteenth-2023.pdf @2023-06-13T18:59:49Z"),
        // 2023-07-04 - T1 - 4th-of-july-2023.pdf @2023-06-27T12:50:57Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2023, 7, 4, Unsourced, T1, "4th-of-july-2023.pdf @2023-06-27T12:50:57Z"),
        // 2023-07-05 - T1 - 4th-of-july-2023.pdf @2023-06-27T12:50:57Z - disagreement: grains late open 08:30 CT; interest rates no row.
        (2023, 7, 5, Unsourced, T1, "4th-of-july-2023.pdf @2023-06-27T12:50:57Z"),
        // 2023-09-04 - T1 - labor-day-2023.pdf @2023-08-02T19:24:46Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2023, 9, 4, Unsourced, T1, "labor-day-2023.pdf @2023-08-02T19:24:46Z"),
        // 2023-11-23 - T1 - thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z - disagreement: grains closed; interest rates early close 12:00 CT.
        (2023, 11, 23, Unsourced, T1, "thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z"),
        // 2023-11-24 - T1 - thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z - disagreement: grains late open 08:30 CT and early close 12:05 CT; interest rates early close 12:15 CT.
        (2023, 11, 24, Unsourced, T1, "thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z"),
        // 2023-12-25 - T1 - christmas-day-2023.pdf @2026-07-19T09:52:48Z - closed: no trade date.
        (2023, 12, 25, Closed, T1, "christmas-day-2023.pdf @2026-07-19T09:52:48Z"),
        // 2023-12-26 - T1 - christmas-day-2023.pdf @2026-07-19T09:52:48Z - disagreement: grains late open 08:30 CT; interest rates no row.
        (2023, 12, 26, Unsourced, T1, "christmas-day-2023.pdf @2026-07-19T09:52:48Z"),
        // 2024-01-01 - T1 - new-years-day-2024.pdf @2026-08-11T16:57:16Z - closed: no trade date.
        (2024, 1, 1, Closed, T1, "new-years-day-2024.pdf @2026-08-11T16:57:16Z"),
        // 2024-01-02 - T1 - new-years-day-2024.pdf @2026-08-11T16:57:16Z - disagreement: grains late open 08:30 CT; interest rates no row.
        (2024, 1, 2, Unsourced, T1, "new-years-day-2024.pdf @2026-08-11T16:57:16Z"),
        // 2024-01-15 - T2 - CME-SVC-2024-01-14 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2024, 1, 15, Unsourced, T2, "CME-SVC-2024-01-14"),
        // 2024-02-19 - T2 - CME-SVC-2024-02-18 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2024, 2, 19, Unsourced, T2, "CME-SVC-2024-02-18"),
        // 2024-03-29 - T2 - CME-SVC-2024-03-28 - closed: no trade date.
        (2024, 3, 29, Closed, T2, "CME-SVC-2024-03-28"),
        // 2024-05-27 - T2 - CME-SVC-2024-05-26 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2024, 5, 27, Unsourced, T2, "CME-SVC-2024-05-26"),
        // 2024-06-19 - T2 - CME-SVC-2024-06-18 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2024, 6, 19, Unsourced, T2, "CME-SVC-2024-06-18"),
        // 2024-07-04 - T2 - CME-SVC-2024-07-03 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2024, 7, 4, Unsourced, T2, "CME-SVC-2024-07-03"),
        // 2024-07-05 - T2 - CME-SVC-2024-07-03 - disagreement: grains late open 08:30 CT; interest rates no row.
        (2024, 7, 5, Unsourced, T2, "CME-SVC-2024-07-03"),
        // 2024-09-02 - T2 - CME-SVC-2024-09-01 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2024, 9, 2, Unsourced, T2, "CME-SVC-2024-09-01"),
        // 2024-11-28 - T2 - CME-SVC-2024-11-27 - disagreement: grains closed; interest rates early close 12:00 CT.
        (2024, 11, 28, Unsourced, T2, "CME-SVC-2024-11-27"),
        // 2024-11-29 - T2 - CME-SVC-2024-11-27 - disagreement: grains late open 08:30 CT and early close 12:05 CT; interest rates early close 12:15 CT.
        (2024, 11, 29, Unsourced, T2, "CME-SVC-2024-11-27"),
        // 2024-12-24 - T2 - CME-SVC-2024-12-24 - disagreement: grains early close 12:05 CT; interest rates early close 12:15 CT.
        (2024, 12, 24, Unsourced, T2, "CME-SVC-2024-12-24"),
        // 2024-12-25 - T2 - CME-SVC-2024-12-24 - closed: no trade date.
        (2024, 12, 25, Closed, T2, "CME-SVC-2024-12-24"),
        // 2024-12-26 - T2 - CME-SVC-2024-12-24 - disagreement: grains late open 08:30 CT; interest rates no row.
        (2024, 12, 26, Unsourced, T2, "CME-SVC-2024-12-24"),
        // 2025-01-01 - T2 - CME-SVC-2024-12-31 - closed: no trade date.
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-02 - T2 - CME-SVC-2024-12-31 - grains late open 08:30 CT;
        // no row in interest rates.
        (2025, 1, 2, Unsourced, T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 - T2 - CME-SVC-2025-01-19 - grains closed, interest rates
        // early close 12:00 CT.
        (2025, 1, 20, Unsourced, T2, "CME-SVC-2025-01-19"),
        // 2025-01-21 - T2 - CME-SVC-2025-01-19 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2025, 1, 21, Unsourced, T2, "CME-SVC-2025-01-19"),
        // 2025-02-17 - T2 - CME-SVC-2025-02-16 - as 2025-01-20.
        (2025, 2, 17, Unsourced, T2, "CME-SVC-2025-02-16"),
        // 2025-02-18 - T2 - CME-SVC-2025-02-16 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2025, 2, 18, Unsourced, T2, "CME-SVC-2025-02-16"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-17 - both families closed.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 - T2 - CME-SVC-2025-05-25 - as 2025-01-20.
        (2025, 5, 26, Unsourced, T2, "CME-SVC-2025-05-25"),
        // 2025-05-27 - T2 - CME-SVC-2025-05-25 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2025, 5, 27, Unsourced, T2, "CME-SVC-2025-05-25"),
        // 2025-06-19 - T2 - CME-SVC-2025-06-18 - as 2025-01-20.
        (2025, 6, 19, Unsourced, T2, "CME-SVC-2025-06-18"),
        // 2025-06-20 - T2 - CME-SVC-2025-06-18 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2025, 6, 20, Unsourced, T2, "CME-SVC-2025-06-18"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-03 - as 2025-01-20.
        (2025, 7, 4, Unsourced, T2, "CME-SVC-2025-07-03"),
        // 2025-09-01 - T2 - CME-SVC-2025-08-31 - as 2025-01-20.
        (2025, 9, 1, Unsourced, T2, "CME-SVC-2025-08-31"),
        // 2025-09-02 - T2 - CME-SVC-2025-08-31 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2025, 9, 2, Unsourced, T2, "CME-SVC-2025-08-31"),
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
        // 2026-01-20 - T2 - CME-SVC-2026-01-18 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2026, 1, 20, Unsourced, T2, "CME-SVC-2026-01-18"),
        // 2026-02-16 - T2 - CME-SVC-2026-02-15 - as 2025-01-20.
        (2026, 2, 16, Unsourced, T2, "CME-SVC-2026-02-15"),
        // 2026-02-17 - T2 - CME-SVC-2026-02-15 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2026, 2, 17, Unsourced, T2, "CME-SVC-2026-02-15"),
        // 2026-04-03 - T2 - CME-SVC-2026-04-01 - grains closed, interest rates
        // early close 10:15 CT.
        (2026, 4, 3, Unsourced, T2, "CME-SVC-2026-04-01"),
        // 2026-05-25 - T2 - CME-SVC-2026-05-24 - as 2025-01-20.
        (2026, 5, 25, Unsourced, T2, "CME-SVC-2026-05-24"),
        // 2026-05-26 - T2 - CME-SVC-2026-05-24 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2026, 5, 26, Unsourced, T2, "CME-SVC-2026-05-24"),
        // 2026-06-19 - T2 - CME-SVC-2026-06-18 - as 2025-01-20.
        (2026, 6, 19, Unsourced, T2, "CME-SVC-2026-06-18"),
        // 2026, 6, 22 - T2 - CME-SVC-2026-06-18 - interest rates states a Saturday-session replacement and grains states nothing, so the two routed families disagree.
        (2026, 6, 22, Unsourced, T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - as 2025-01-20.
        (2026, 7, 3, Unsourced, T2, "CME-SVC-2026-07-03"),
        // 2026, 7, 6 - T2 - CME-SVC-2026-07-03 - interest rates states a Saturday-session replacement and grains states nothing, so the two routed families disagree.
        (2026, 7, 6, Unsourced, T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 - T2 - CME-SVC-2026-09-06 - as 2025-01-20.
        (2026, 9, 7, Unsourced, T2, "CME-SVC-2026-09-06"),
        // 2026-09-08 - T2 - CME-SVC-2026-09-06 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2026, 9, 8, Unsourced, T2, "CME-SVC-2026-09-06"),
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
        // 2027-01-19 - T2 - CME-SVC-2027-01-17 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2027, 1, 19, Unsourced, T2, "CME-SVC-2027-01-17"),
        // 2027-02-15 - T2 - CME-SVC-2027-02-14 - as 2025-01-20.
        (2027, 2, 15, Unsourced, T2, "CME-SVC-2027-02-14"),
        // 2027-02-16 - T2 - CME-SVC-2027-02-14 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2027, 2, 16, Unsourced, T2, "CME-SVC-2027-02-14"),
        // 2027-03-26 - T2 - CME-SVC-2027-03-25 - both families closed.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 - T2 - CME-SVC-2027-05-30 - as 2025-01-20.
        (2027, 5, 31, Unsourced, T2, "CME-SVC-2027-05-30"),
        // 2027-06-01 - T2 - CME-SVC-2027-05-30 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2027, 6, 1, Unsourced, T2, "CME-SVC-2027-05-30"),
        // 2027-06-18 - T2 - CME-SVC-2027-06-17 - as 2025-01-20.
        (2027, 6, 18, Unsourced, T2, "CME-SVC-2027-06-17"),
        // 2027, 6, 21 - T2 - CME-SVC-2027-06-17 - interest rates states a Saturday-session replacement and grains states nothing, so the two routed families disagree.
        (2027, 6, 21, Unsourced, T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 - T2 - CME-SVC-2027-07-04 - grains closed, interest rates
        // early close 13:30 CT.
        (2027, 7, 5, Unsourced, T2, "CME-SVC-2027-07-04"),
        // 2027-07-06 - T2 - CME-SVC-2027-07-04 - grains late open 08:30 CT;
        // no row in interest rates.
        (2027, 7, 6, Unsourced, T2, "CME-SVC-2027-07-04"),
        // 2027-09-06 - T2 - CME-SVC-2027-09-05 - as 2025-01-20.
        (2027, 9, 6, Unsourced, T2, "CME-SVC-2027-09-05"),
        // 2027-09-07 - T2 - CME-SVC-2027-09-05 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2027, 9, 7, Unsourced, T2, "CME-SVC-2027-09-05"),
        // 2027-11-25 - T2 - CME-SVC-2027-11-24 - as 2025-01-20.
        (2027, 11, 25, Unsourced, T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - as 2025-11-28.
        (2027, 11, 26, Unsourced, T2, "CME-SVC-2027-11-24"),
        // 2027-12-24 - T2 - CME-SVC-2027-12-22 - both families closed.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
