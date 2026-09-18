// SPDX-License-Identifier: MIT-0

//! CME Live Cattle, Feeder Cattle and Lean Hog holiday rows, 2010-2012,
//! 2013-2015, 2019-2021, 2022-2024 and 2025-2027 (LAW-HOLIDAY-SCOPE).
//!
//! Keyed by the crate's own venue-local trade date in `America/Chicago`
//! (design memo D1). The conversion is the identity for this family and is the
//! reason the module is short: from 2016-02-29 the grid is a flat Monday-Friday
//! 08:30-13:05 CT regular session with an 08:00-08:30 CT Pre-Open and a
//! 14:30-16:00 CT Post-Close queue, all inside one local day, so no session
//! wraps a midnight and every occurrence's trade date is its own civil date.
//! CME's event-date records therefore key one row each, and the operator's own
//! printed trade date corroborates every one of them.
//!
//! The 2022-2024 era mixes T1 and T2; the 2025-2027 rows below are T2.
//!
//! **2010-2012.** CME's own holiday-calendar PDFs are the T1 source. This
//! family's era grid is a wrapping 17:00 -> 16:00 CT block, and its rows are
//! sixteen **full closures** (New Year's Day, Christmas Day, both Good Fridays,
//! 2011's Good Friday and the ten Monday and Thursday holidays CME kept the
//! family closed on), **early closes** at 13:55 CT on the Good Fridays of 2011
//! and 2012 and at 12:00/12:15 CT on the year-end half-days, and **late opens**
//! at 09:05 CT on 2011-12-27, 2012-01-03, 2012-07-05 and 2012-12-26.
//!
//! **2022-2024.** This family's flat grid keeps every trade date on its own
//! civil date, so each row below is stated on the occurrence's own date.
//! Thirty-three rows: twenty-six closures, four
//! early closes — 12:05 CT on 2022-11-25, 2023-11-24 and 2024-11-29, and
//! 12:15 CT on 2024-12-24 — and three `Unsourced` rows. The 2022 rows and the
//! 2023 rows CME published a holiday schedule for are **T1**; the three 2023
//! dates it published nothing for and all of 2024 are **T2**. The three
//! `Unsourced` dates — 2023-01-16, 2023-02-20 and 2023-04-07 — mean the
//! operator published nothing this crate could read, not that no holiday fell
//! on them; an operator document stating each date in session language would
//! close them.
//!
//! The 2025-2027 rows come from CME's trading-hours service — the endpoint
//! `cmegroup.com/trading-hours.html` itself calls — read as bytes and saved,
//! so those rows are **T2** under LAW-PRIMARY-SOURCES. CME publishes no T1
//! per-asset-class rendering for them; that, the eight 2025 windows that
//! survive only in a pre-holiday capture, and the two Saturday sessions this
//! family never had are recorded as gaps in
//! [`docs/evidence/globex_livestock.md`](../../../../../docs/evidence/globex_livestock.md).
//!
//! Two shapes only: `Closed` on a full Globex closure, and `EarlyClose` on the
//! half-days CME publishes for the family. The 2022-2024 and 2025-2027 windows
//! have no late open, and no row whose internal phase topology the scalar
//! vocabulary cannot state.

use super::fences::{early_close, late_open, late_open_and_early_close};
use super::{
    EvidenceTier::{T1, T2},
    HolidayKind::{Closed, Unsourced},
    HolidayTable, holidays,
};

/// The family's built-in holiday rows and the windows they were audited over.
///
///  Five audited eras: 2010-2012 at T1, 2013-2015 at T1, 2019-2021 at T1,
/// 2022-2024 at T1/T2 and 2025-2027 at T2.
///  The 2016-2018 interval between them is audited by no wave and lies
/// outside every window, so `holiday_on` has no answer there rather than
/// reporting a normal date.
///
/// Coverage ends at 2027-12-31, the end of
/// the operator's published future, and CME's 2028-01-01 record sits outside
/// every window and ships no row. Inside a window a date with no row is audited
/// normal, except where an `Unsourced` row marks the operator's silence
/// instead.
// Evidence: docs/evidence/globex_livestock.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2012, 12, 31), (2013, 1, 1) ..= (2015, 12, 31), (2019, 1, 1) ..= (2021, 12, 31), (2022, 1, 1) ..= (2024, 12, 31), (2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2010-01-01 - T1 - 2010-new-years.pdf - closed: new year's day 2010.
        (2010, 1, 1, Closed, T1, "2010-new-years.pdf @2010-02-15T05:16:52Z"),
        // 2010-04-02 - T1 - 2010-good-friday.pdf - closed: good friday 2010.
        (2010, 4, 2, Closed, T1, "2010-good-friday.pdf @2010-06-01T11:19:16Z"),
        (2010, 11, 26, early_close(12 * 3_600), T1, "2010-thanksgiving.pdf @2010-11-22T09:40:12Z"),
        // 2010-12-24 - T1 - 2010-christmas.pdf - closed: christmas day 2010 observed.
        (2010, 12, 24, Closed, T1, "2010-christmas.pdf @2010-12-14T06:12:38Z"),
        (2010, 12, 31, early_close(12 * 3_600 + 15 * 60), T1, "2011-new-years.pdf @2011-11-01T14:39:45Z"),
        (2011, 4, 21, early_close(13 * 3_600 + 55 * 60), T1, "2011-good-friday.pdf @2011-10-28T02:37:07Z"),
        // 2011-04-22 - T1 - 2011-good-friday.pdf - closed: good friday 2011.
        (2011, 4, 22, Closed, T1, "2011-good-friday.pdf @2011-10-28T02:37:07Z"),
        // 2011-05-30 - T1 - 2011-memorial-day.pdf - closed: memorial day 2011.
        (2011, 5, 30, Closed, T1, "2011-memorial-day.pdf @2013-09-30T10:56:52Z"),
        // 2011-07-04 - T1 - 2011-4th-of-july.pdf - closed: independence day 2011.
        (2011, 7, 4, Closed, T1, "2011-4th-of-july.pdf @2011-11-01T14:40:54Z"),
        // 2011-09-05 - T1 - 2011-labor-day.pdf - closed: labor day 2011.
        (2011, 9, 5, Closed, T1, "2011-labor-day.pdf @2011-11-01T14:43:45Z"),
        (2011, 11, 25, early_close(12 * 3_600 + 15 * 60), T1, "2011-thanksgiving.pdf @2011-11-24T18:52:46Z"),
        // 2011-12-26 - T1 - 2011-christmas.pdf - closed: christmas day 2011 observed.
        (2011, 12, 26, Closed, T1, "2011-christmas.pdf @2012-01-25T02:05:48Z"),
        (2011, 12, 27, late_open(9 * 3_600 + 5 * 60), T1, "2011-christmas.pdf @2012-01-25T02:05:48Z"),
        // 2012-01-02 - T1 - 2012-new-years.pdf - closed: new year's day 2012 observed.
        (2012, 1, 2, Closed, T1, "2012-new-years.pdf @2012-01-25T02:54:30Z"),
        (2012, 1, 3, late_open(9 * 3_600 + 5 * 60), T1, "2012-new-years.pdf @2012-01-25T02:54:30Z"),
        // 2012-01-16 - T1 - 2012-martin-luther-king.pdf - closed: martin luther king jr. day 2012.
        (2012, 1, 16, Closed, T1, "2012-martin-luther-king.pdf @2012-05-05T16:15:26Z"),
        // 2012-02-20 - T1 - 2012-presidents-day.pdf - closed: presidents' day 2012.
        (2012, 2, 20, Closed, T1, "2012-presidents-day.pdf @2012-05-05T16:15:39Z"),
        (2012, 4, 5, early_close(13 * 3_600 + 55 * 60), T1, "2012-good-friday.pdf @2012-04-17T00:42:47Z"),
        // 2012-04-06 - T1 - 2012-good-friday.pdf - closed: good friday 2012.
        (2012, 4, 6, Closed, T1, "2012-good-friday.pdf @2012-04-17T00:42:47Z"),
        // 2012-05-28 - T1 - 2012-memorial-day.pdf - closed: memorial day 2012.
        (2012, 5, 28, Closed, T1, "2012-memorial-day.pdf @2012-09-15T00:37:14Z"),
        // 2012-07-04 - T1 - 2012-4th-of-july.pdf - closed: independence day 2012.
        (2012, 7, 4, Closed, T1, "2012-4th-of-july.pdf @2012-09-15T00:39:23Z"),
        (2012, 7, 5, late_open(9 * 3_600 + 5 * 60), T1, "2012-4th-of-july.pdf @2012-09-15T00:39:23Z"),
        // 2012-09-03 - T1 - 2012-labor-day.pdf - closed: labor day 2012.
        (2012, 9, 3, Closed, T1, "2012-labor-day.pdf @2012-09-15T00:34:37Z"),
        (2012, 11, 23, early_close(12 * 3_600 + 15 * 60), T1, "2012-thanksgiving.pdf @2013-01-27T22:39:01Z"),
        (2012, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        // 2012-12-25 - T1 - 2012-christmas.pdf - closed: christmas day 2012.
        (2012, 12, 25, Closed, T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        (2012, 12, 26, late_open(9 * 3_600 + 5 * 60), T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        // 2013-01-01 - T1 - 2013-new-years.pdf @2013-04-14T19:41:46Z - CME prints no session running through this date.
        (2013, 1, 1, Closed, T1, "2013-new-years.pdf @2013-04-14T19:41:46Z"),
        // 2013-01-02 - T1 - 2013-new-years.pdf @2013-04-14T19:41:46Z - the trade date's first open is 9:05 CT: the evening leg that would have opened earlier did not run.
        (2013, 1, 2, late_open(9 * 3_600 + 5 * 60), T1, "2013-new-years.pdf @2013-04-14T19:41:46Z"),
        // 2013-01-21 - T1 - 2013-martin-luther-king.pdf @2012-11-19T00:16:09Z - CME prints no session running through this date.
        (2013, 1, 21, Closed, T1, "2013-martin-luther-king.pdf @2012-11-19T00:16:09Z"),
        // 2013-02-18 - T1 - 2013-presidents-day.pdf @2013-03-09T11:53:37Z - CME prints no session running through this date.
        (2013, 2, 18, Closed, T1, "2013-presidents-day.pdf @2013-03-09T11:53:37Z"),
        // 2013-03-28 - T1 - 2013-good-friday.pdf @2013-06-23T19:59:25Z - the printed final close 13:55 CT is earlier than the family's ordinary 16:00 CT.
        (2013, 3, 28, early_close(13 * 3_600 + 55 * 60), T1, "2013-good-friday.pdf @2013-06-23T19:59:25Z"),
        // 2013-03-29 - T1 - 2013-good-friday.pdf @2013-06-23T19:59:25Z - CME prints no session running through this date.
        (2013, 3, 29, Closed, T1, "2013-good-friday.pdf @2013-06-23T19:59:25Z"),
        // 2013-05-27 - T1 - 2013-memorial-day.pdf @2013-06-23T20:36:04Z - CME prints no session running through this date.
        (2013, 5, 27, Closed, T1, "2013-memorial-day.pdf @2013-06-23T20:36:04Z"),
        // 2013-07-03 - T1 - 2013-4th-of-july-done.pdf @2013-07-17T05:03:33Z - the printed final close 12:15 CT for the Livestock Futures & Options line is earlier than the family's ordinary 16:00 CT; the same revision's 12:00 CT is the Dairy and Lumber line, which this key does not model.
        (2013, 7, 3, early_close(12 * 3_600 + 15 * 60), T1, "2013-4th-of-july-done.pdf @2013-07-17T05:03:33Z"),
        // 2013-07-04 - T1 - 2013-4th-of-july.pdf @2013-06-23T20:58:25Z - CME prints no session running through this date.
        (2013, 7, 4, Closed, T1, "2013-4th-of-july.pdf @2013-06-23T20:58:25Z"),
        // 2013-09-02 - T1 - 2013-labor-day.pdf @2013-09-02T17:08:41Z - CME prints no session running through this date.
        (2013, 9, 2, Closed, T1, "2013-labor-day.pdf @2013-09-02T17:08:41Z"),
        // 2013-11-28 - T1 - 2013-thanksgiving.pdf @2014-02-14T06:28:36Z - CME prints no session running through this date.
        (2013, 11, 28, Closed, T1, "2013-thanksgiving.pdf @2014-02-14T06:28:36Z"),
        // 2013-11-29 - T1 - 2013-thanksgiving.pdf @2014-02-14T06:28:36Z - the trade date's first open is 9:05 CT: the evening leg that would have opened earlier did not run; the printed final close 12:15 CT is earlier than the family's ordinary 13:55 CT.
        (2013, 11, 29, late_open_and_early_close(9 * 3_600 + 5 * 60, 12 * 3_600 + 15 * 60), T1, "2013-thanksgiving.pdf @2014-02-14T06:28:36Z"),
        // 2013-12-24 - T1 - 2013-christmas.pdf @2014-04-12T06:24:28Z - the printed final close 12:15 CT is earlier than the family's ordinary 16:00 CT.
        (2013, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "2013-christmas.pdf @2014-04-12T06:24:28Z"),
        // 2013-12-25 - T1 - 2013-christmas.pdf @2014-04-12T06:24:28Z - CME prints no session running through this date.
        (2013, 12, 25, Closed, T1, "2013-christmas.pdf @2014-04-12T06:24:28Z"),
        // 2013-12-26 - T1 - 2013-christmas.pdf @2014-04-12T06:24:28Z - the trade date's first open is 9:05 CT: the evening leg that would have opened earlier did not run.
        (2013, 12, 26, late_open(9 * 3_600 + 5 * 60), T1, "2013-christmas.pdf @2014-04-12T06:24:28Z"),
        // 2014-01-01 - T1 - 2014-new-years.pdf @2013-10-07T20:58:00Z - CME prints no session running through this date.
        (2014, 1, 1, Closed, T1, "2014-new-years.pdf @2013-10-07T20:58:00Z"),
        // 2014-01-02 - T1 - 2014-new-years.pdf @2013-10-07T20:58:00Z - the trade date's first open is 9:05 CT: the evening leg that would have opened earlier did not run.
        (2014, 1, 2, late_open(9 * 3_600 + 5 * 60), T1, "2014-new-years.pdf @2013-10-07T20:58:00Z"),
        // 2014-01-20 - T1 - 2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z - CME prints no session running through this date.
        (2014, 1, 20, Closed, T1, "2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z"),
        // 2014-02-17 - T1 - 2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z - CME prints no session running through this date.
        (2014, 2, 17, Closed, T1, "2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z"),
        // 2014-04-17 - T1 - 2014-good-friday-holiday-schedule.pdf @2014-03-26T15:27:35Z - the printed final close 13:55 CT is earlier than the family's ordinary 16:00 CT.
        (2014, 4, 17, early_close(13 * 3_600 + 55 * 60), T1, "2014-good-friday-holiday-schedule.pdf @2014-03-26T15:27:35Z"),
        // 2014-04-18 - T1 - 2014-good-friday-holiday-schedule.pdf @2014-03-26T15:27:35Z - CME prints no session running through this date.
        (2014, 4, 18, Closed, T1, "2014-good-friday-holiday-schedule.pdf @2014-03-26T15:27:35Z"),
        // 2014-05-26 - T1 - 2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z - CME prints no session running through this date.
        (2014, 5, 26, Closed, T1, "2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z"),
        // 2014-07-03 - T1 - 2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z - the printed final close 12:15 CT is earlier than the family's ordinary 16:00 CT.
        (2014, 7, 3, early_close(12 * 3_600 + 15 * 60), T1, "2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z"),
        // 2014-07-04 - T1 - 2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z - CME prints no session running through this date.
        (2014, 7, 4, Closed, T1, "2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z"),
        // 2014-09-01 - T1 - 2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z - CME prints no session running through this date.
        (2014, 9, 1, Closed, T1, "2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z"),
        // 2014-11-27 - T1 - 2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z - CME prints no session running through this date.
        (2014, 11, 27, Closed, T1, "2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z"),
        // 2014-11-28 - T1 - 2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z - the printed final close 12:15 CT is earlier than the family's ordinary 13:55 CT.
        (2014, 11, 28, early_close(12 * 3_600 + 15 * 60), T1, "2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z"),
        // 2014-12-24 - T1 - 2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z - the printed final close 12:15 CT is earlier than the family's ordinary 16:00 CT.
        (2014, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z"),
        // 2014-12-25 - T1 - 2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z - CME prints no session running through this date.
        (2014, 12, 25, Closed, T1, "2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z"),
        // 2015-01-01 - T1 - 2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z - CME prints no session running through this date.
        (2015, 1, 1, Closed, T1, "2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z"),
        // 2015-01-19 - T1 - 2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z - CME prints no session running through this date.
        (2015, 1, 19, Closed, T1, "2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z"),
        // 2015-02-16 - T1 - 2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z - CME prints no session running through this date.
        (2015, 2, 16, Closed, T1, "2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z"),
        // 2015-04-02 - T1 - 2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z - the printed final close 13:55 CT is earlier than the family's ordinary 16:00 CT.
        (2015, 4, 2, early_close(13 * 3_600 + 55 * 60), T1, "2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z"),
        // 2015-04-03 - T1 - 2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z - CME prints no session running through this date.
        (2015, 4, 3, Closed, T1, "2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z"),
        // 2015-05-25 - T1 - 2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z - CME prints no session running through this date.
        (2015, 5, 25, Closed, T1, "2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z"),
        // 2015-07-02 - T1 - 2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z - the printed final close 12:15 CT is earlier than the family's ordinary 16:00 CT.
        (2015, 7, 2, early_close(12 * 3_600 + 15 * 60), T1, "2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z"),
        // 2015-07-03 - T1 - 2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z - CME prints no session running through this date.
        (2015, 7, 3, Closed, T1, "2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z"),
        // 2015-09-07 - T1 - 2015-labor-day-holiday-schedule.pdf @2015-08-24T02:30:39Z - CME prints no session running through this date.
        (2015, 9, 7, Closed, T1, "2015-labor-day-holiday-schedule.pdf @2015-08-24T02:30:39Z"),
        // 2015-11-26 - T1 - 2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z - CME prints no session running through this date.
        (2015, 11, 26, Closed, T1, "2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z"),
        // 2015-11-27 - T1 - 2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z - the printed final close 12:15 CT is earlier than the family's ordinary 13:55 CT.
        (2015, 11, 27, early_close(12 * 3_600 + 15 * 60), T1, "2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z"),
        // 2015-12-24 - T1 - 2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z - the printed final close 12:15 CT is earlier than the family's ordinary 16:00 CT.
        (2015, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z"),
        // 2015-12-25 - T1 - 2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z - CME prints no session running through this date.
        (2015, 12, 25, Closed, T1, "2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z"),
        // 2015-12-31 - T1 - 2016-new-years-holiday-schedule.pdf @2016-01-08 - the printed final close 13:55 CT is earlier than the family's ordinary 16:00 CT.
        (2015, 12, 31, early_close(13 * 3_600 + 55 * 60), T1, "2016-new-years-holiday-schedule.pdf @2016-01-08"),
        // 2019-01-01 - T1 - 2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z - CME prints no session running through this date.
        (2019, 1, 1, Closed, T1, "2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z"),
        // 2019-01-21 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-martin-luther-king-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - CME prints no session running through this date.
        (2019, 1, 21, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-martin-luther-king-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-02-18 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-presidents-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - CME prints no session running through this date.
        (2019, 2, 18, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-presidents-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-04-19 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-good-friday-holiday-compact.xls @2021-01-26T09:48:37Z - CME prints no session running through this date.
        (2019, 4, 19, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-good-friday-holiday-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-05-27 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-memorial-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - CME prints no session running through this date.
        (2019, 5, 27, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-memorial-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-06-19 - T1 - 2019-holiday-calendars.zip @2021-01-26T09:48:37Z - .
        (2019, 6, 19, Unsourced, T1, "2019-holiday-calendars.zip @2021-01-26T09:48:37Z"),
        // 2019-07-03 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - the printed final close 12:15 CT is earlier than the ordinary 13:05 CT close.
        (2019, 7, 3, early_close(12 * 3_600 + 15 * 60), T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-07-04 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - CME prints no session running through this date.
        (2019, 7, 4, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-09-02 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-labor-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - CME prints no session running through this date.
        (2019, 9, 2, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-labor-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-11-28 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - CME prints no session running through this date.
        (2019, 11, 28, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-11-29 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - the printed final close 12:15 CT is earlier than the ordinary 13:05 CT close.
        (2019, 11, 29, early_close(12 * 3_600 + 15 * 60), T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-12-24 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - the printed final close 12:15 CT is earlier than the ordinary 13:05 CT close.
        (2019, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-12-25 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - CME prints no session running through this date.
        (2019, 12, 25, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2020-01-01 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - CME prints no session running through this date.
        (2020, 1, 1, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2020-01-20 - T1 - 2020-holiday-calendars.zip#2020-martin-luther-king-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - CME prints no session running through this date.
        (2020, 1, 20, Closed, T1, "2020-holiday-calendars.zip#2020-martin-luther-king-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-02-17 - T1 - 2020-holiday-calendars.zip#2020-presidents-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - CME prints no session running through this date.
        (2020, 2, 17, Closed, T1, "2020-holiday-calendars.zip#2020-presidents-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-04-10 - T1 - 2020-holiday-calendars.zip#2020-good-friday-holiday-compact.xls @2026-07-30T11:18:34Z - CME prints no session running through this date.
        (2020, 4, 10, Closed, T1, "2020-holiday-calendars.zip#2020-good-friday-holiday-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-05-25 - T1 - 2020-holiday-calendars.zip#2020-memorial-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - CME prints no session running through this date.
        (2020, 5, 25, Closed, T1, "2020-holiday-calendars.zip#2020-memorial-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-06-19 - T1 - 2020-holiday-calendars.zip @2026-07-30T11:18:34Z - .
        (2020, 6, 19, Unsourced, T1, "2020-holiday-calendars.zip @2026-07-30T11:18:34Z"),
        // 2020-07-02 - T1 - 2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - the printed final close 12:15 CT is earlier than the ordinary 13:05 CT close.
        (2020, 7, 2, early_close(12 * 3_600 + 15 * 60), T1, "2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-07-03 - T1 - 2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - CME prints no session running through this date.
        (2020, 7, 3, Closed, T1, "2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-09-07 - T1 - 2020-holiday-calendars.zip#2020-labor-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - CME prints no session running through this date.
        (2020, 9, 7, Closed, T1, "2020-holiday-calendars.zip#2020-labor-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-11-26 - T1 - 2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - CME prints no session running through this date.
        (2020, 11, 26, Closed, T1, "2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-11-27 - T1 - 2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - the printed final close 12:05 CT is earlier than the ordinary 13:05 CT close.
        (2020, 11, 27, early_close(12 * 3_600 + 5 * 60), T1, "2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-12-24 - T1 - 2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - the printed final close 12:05 CT is earlier than the ordinary 13:05 CT close.
        (2020, 12, 24, early_close(12 * 3_600 + 5 * 60), T1, "2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-12-25 - T1 - 2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - CME prints no session running through this date.
        (2020, 12, 25, Closed, T1, "2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2021-01-01 - T1 - 2020-holiday-calendars.zip#2021-new-years-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - CME prints no session running through this date.
        (2021, 1, 1, Closed, T1, "2020-holiday-calendars.zip#2021-new-years-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2021-01-18 - T1 - 2021-holiday-calendars.zip#2021-mlk-day-schedule-compact.xls @2026-08-30T10:03:27Z - CME prints no session running through this date.
        (2021, 1, 18, Closed, T1, "2021-holiday-calendars.zip#2021-mlk-day-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-02-15 - T1 - 2021-holiday-calendars.zip#2021-presidents-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - CME prints no session running through this date.
        (2021, 2, 15, Closed, T1, "2021-holiday-calendars.zip#2021-presidents-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-04-02 - T1 - 2021-holiday-calendars.zip#2021-good-friday-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - CME prints no session running through this date.
        (2021, 4, 2, Closed, T1, "2021-holiday-calendars.zip#2021-good-friday-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-05-31 - T1 - 2021-holiday-calendars.zip#2021-memorial-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - CME prints no session running through this date.
        (2021, 5, 31, Closed, T1, "2021-holiday-calendars.zip#2021-memorial-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-06-19 - T1 - 2021-holiday-calendars.zip @2026-08-30T10:03:27Z - .
        (2021, 6, 19, Unsourced, T1, "2021-holiday-calendars.zip @2026-08-30T10:03:27Z"),
        // 2021-07-05 - T1 - 2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - CME prints no session running through this date.
        (2021, 7, 5, Closed, T1, "2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-09-06 - T1 - 2021-holiday-calendars.zip#2021-labor-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - CME prints no session running through this date.
        (2021, 9, 6, Closed, T1, "2021-holiday-calendars.zip#2021-labor-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-11-25 - T1 - 2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - CME prints no session running through this date.
        (2021, 11, 25, Closed, T1, "2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-11-26 - T1 - 2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - the printed final close 12:05 CT is earlier than the ordinary 13:05 CT close.
        (2021, 11, 26, early_close(12 * 3_600 + 5 * 60), T1, "2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-12-24 - T1 - 2021-holiday-calendars.zip#2021-christmas-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - CME prints no session running through this date.
        (2021, 12, 24, Closed, T1, "2021-holiday-calendars.zip#2021-christmas-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2022-01-17 - T1 - 2022-mlk-day-holiday-schedule.xls @2022-01-17T21:22:30Z - closed: no trade date.
        (2022, 1, 17, Closed, T1, "2022-mlk-day-holiday-schedule.xls @2022-01-17T21:22:30Z"),
        // 2022-02-21 - T1 - 2022-presidents-day-holiday-schedule.xls @2022-07-04T07:38:10Z - closed: no trade date.
        (2022, 2, 21, Closed, T1, "2022-presidents-day-holiday-schedule.xls @2022-07-04T07:38:10Z"),
        // 2022-04-15 - T1 - 2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z - closed: no trade date.
        (2022, 4, 15, Closed, T1, "2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z"),
        // 2022-05-30 - T1 - 2022-memorial-day-holiday-schedule.xls @2022-07-04T06:54:38Z - closed: no trade date.
        (2022, 5, 30, Closed, T1, "2022-memorial-day-holiday-schedule.xls @2022-07-04T06:54:38Z"),
        // 2022-06-20 - T1 - 2022-juneteenth-holiday-schedule.xls @2022-06-20T20:02:10Z - closed: no trade date.
        (2022, 6, 20, Closed, T1, "2022-juneteenth-holiday-schedule.xls @2022-06-20T20:02:10Z"),
        // 2022-07-04 - T1 - 2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z - closed: no trade date.
        (2022, 7, 4, Closed, T1, "2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z"),
        // 2022-09-05 - T1 - 2022-labor-day-holiday-schedule.xls @2022-07-04T06:54:41Z - closed: no trade date.
        (2022, 9, 5, Closed, T1, "2022-labor-day-holiday-schedule.xls @2022-07-04T06:54:41Z"),
        // 2022-11-24 - T1 - 2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z - closed: no trade date.
        (2022, 11, 24, Closed, T1, "2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z"),
        // 2022-11-25 - T1 - 2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z - early close 12:05 CT.
        (2022, 11, 25, early_close(12 * 3_600 + 5 * 60), T1, "2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z"),
        // 2022-12-26 - T1 - 2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z - closed: no trade date.
        (2022, 12, 26, Closed, T1, "2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z"),
        // 2023-01-02 - T1 - 2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z - closed: no trade date.
        (2023, 1, 2, Closed, T1, "2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z"),
        // 2023-01-16 - T2 - CME-SVC-2023-01-15 - unsourced: no operator document covers this date.
        (2023, 1, 16, Unsourced, T2, "CME-SVC-2023-01-15"),
        // 2023-02-20 - T2 - CME-SVC-2023-02-19 - unsourced: no operator document covers this date.
        (2023, 2, 20, Unsourced, T2, "CME-SVC-2023-02-19"),
        // 2023-04-07 - T2 - CME-SVC-2023-04-06 - unsourced: no operator document covers this date.
        (2023, 4, 7, Unsourced, T2, "CME-SVC-2023-04-06"),
        // 2023-05-29 - T1 - memorial-day-2023.pdf @2023-04-20T22:40:18Z - closed: no trade date.
        (2023, 5, 29, Closed, T1, "memorial-day-2023.pdf @2023-04-20T22:40:18Z"),
        // 2023-06-19 - T1 - juneteenth-2023.pdf @2023-06-13T18:59:49Z - closed: no trade date.
        (2023, 6, 19, Closed, T1, "juneteenth-2023.pdf @2023-06-13T18:59:49Z"),
        // 2023-07-04 - T1 - 4th-of-july-2023.pdf @2023-06-27T12:50:57Z - closed: no trade date.
        (2023, 7, 4, Closed, T1, "4th-of-july-2023.pdf @2023-06-27T12:50:57Z"),
        // 2023-09-04 - T1 - labor-day-2023.pdf @2023-08-02T19:24:46Z - closed: no trade date.
        (2023, 9, 4, Closed, T1, "labor-day-2023.pdf @2023-08-02T19:24:46Z"),
        // 2023-11-23 - T1 - thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z - closed: no trade date.
        (2023, 11, 23, Closed, T1, "thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z"),
        // 2023-11-24 - T1 - thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z - early close 12:05 CT.
        (2023, 11, 24, early_close(12 * 3_600 + 5 * 60), T1, "thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z"),
        // 2023-12-25 - T1 - christmas-day-2023.pdf @2026-07-19T09:52:48Z - closed: no trade date.
        (2023, 12, 25, Closed, T1, "christmas-day-2023.pdf @2026-07-19T09:52:48Z"),
        // 2024-01-01 - T1 - new-years-day-2024.pdf @2026-08-11T16:57:16Z - closed: no trade date.
        (2024, 1, 1, Closed, T1, "new-years-day-2024.pdf @2026-08-11T16:57:16Z"),
        // 2024-01-15 - T2 - CME-SVC-2024-01-14 - closed: no trade date.
        (2024, 1, 15, Closed, T2, "CME-SVC-2024-01-14"),
        // 2024-02-19 - T2 - CME-SVC-2024-02-18 - closed: no trade date.
        (2024, 2, 19, Closed, T2, "CME-SVC-2024-02-18"),
        // 2024-03-29 - T2 - CME-SVC-2024-03-28 - closed: no trade date.
        (2024, 3, 29, Closed, T2, "CME-SVC-2024-03-28"),
        // 2024-05-27 - T2 - CME-SVC-2024-05-26 - closed: no trade date.
        (2024, 5, 27, Closed, T2, "CME-SVC-2024-05-26"),
        // 2024-06-19 - T2 - CME-SVC-2024-06-18 - closed: no trade date.
        (2024, 6, 19, Closed, T2, "CME-SVC-2024-06-18"),
        // 2024-07-04 - T2 - CME-SVC-2024-07-03 - closed: no trade date.
        (2024, 7, 4, Closed, T2, "CME-SVC-2024-07-03"),
        // 2024-09-02 - T2 - CME-SVC-2024-09-01 - closed: no trade date.
        (2024, 9, 2, Closed, T2, "CME-SVC-2024-09-01"),
        // 2024-11-28 - T2 - CME-SVC-2024-11-27 - closed: no trade date.
        (2024, 11, 28, Closed, T2, "CME-SVC-2024-11-27"),
        // 2024-11-29 - T2 - CME-SVC-2024-11-27 - early close 12:05 CT.
        (2024, 11, 29, early_close(12 * 3_600 + 5 * 60), T2, "CME-SVC-2024-11-27"),
        // 2024-12-24 - T2 - CME-SVC-2024-12-24 - early close 12:15 CT.
        (2024, 12, 24, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-2024-12-24"),
        // 2024-12-25 - T2 - CME-SVC-2024-12-24 - closed: no trade date.
        (2024, 12, 25, Closed, T2, "CME-SVC-2024-12-24"),
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 - T2 - CME-SVC-2025-01-19 - Martin Luther King Jr. Day, no events.
        (2025, 1, 20, Closed, T2, "CME-SVC-2025-01-19"),
        // 2025-02-17 - T2 - CME-SVC-2025-02-16 - Presidents' Day, no events published.
        (2025, 2, 17, Closed, T2, "CME-SVC-2025-02-16"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-17 - Good Friday, no events published.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 - T2 - CME-SVC-2025-05-25 - Memorial Day, no events published.
        (2025, 5, 26, Closed, T2, "CME-SVC-2025-05-25"),
        // 2025-06-19 - T2 - CME-SVC-2025-06-18 - Juneteenth, no events published.
        (2025, 6, 19, Closed, T2, "CME-SVC-2025-06-18"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-03 - Independence Day, no events published.
        (2025, 7, 4, Closed, T2, "CME-SVC-2025-07-03"),
        // 2025-09-01 - T2 - CME-SVC-2025-08-31 - Labor Day, no events published.
        (2025, 9, 1, Closed, T2, "CME-SVC-2025-08-31"),
        // 2025-11-27 - T2 - CME-SVC-2025-11-26 - Thanksgiving, no events published.
        (2025, 11, 27, Closed, T2, "CME-SVC-2025-11-26"),
        // 2025-11-28 - T2 - CME-SVC-2025-11-26 - day after Thanksgiving, 12:05 CT close.
        (2025, 11, 28, early_close(12 * 3_600 + 5 * 60), T2, "CME-SVC-2025-11-26"),
        // 2025-11-29 - T2 - CME-SVC-2025-11-26-SAT - Thanksgiving Saturday, no events.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-12-24 - T2 - CME-SVC-2025-12-24 - Christmas Eve, 12:15 CT close.
        (2025, 12, 24, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-2025-12-24"),
        // 2025-12-25 - T2 - CME-SVC-2025-12-24 - Christmas Day, no events published.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-24"),
        // 2026-01-01 - T2 - CME-SVC-2025-12-31 - New Year's Day, no events published.
        (2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-01-19 - T2 - CME-SVC-2026-01-18 - Martin Luther King Jr. Day, no events.
        (2026, 1, 19, Closed, T2, "CME-SVC-2026-01-18"),
        // 2026-02-16 - T2 - CME-SVC-2026-02-15 - Presidents' Day, no events published.
        (2026, 2, 16, Closed, T2, "CME-SVC-2026-02-15"),
        // 2026-04-03 - T2 - CME-SVC-2026-04-01 - Good Friday, no events published.
        (2026, 4, 3, Closed, T2, "CME-SVC-2026-04-01"),
        // 2026-05-25 - T2 - CME-SVC-2026-05-24 - Memorial Day, no events published.
        (2026, 5, 25, Closed, T2, "CME-SVC-2026-05-24"),
        // 2026-06-19 - T2 - CME-SVC-2026-06-18 - Juneteenth, no events published.
        (2026, 6, 19, Closed, T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - Independence Day observed, no events.
        (2026, 7, 3, Closed, T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 - T2 - CME-SVC-2026-09-06 - Labor Day, no events published.
        (2026, 9, 7, Closed, T2, "CME-SVC-2026-09-06"),
        // 2026-11-26 - T2 - CME-SVC-2026-11-25 - Thanksgiving, no events published.
        (2026, 11, 26, Closed, T2, "CME-SVC-2026-11-25"),
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - day after Thanksgiving, 12:05 CT close.
        (2026, 11, 27, early_close(12 * 3_600 + 5 * 60), T2, "CME-SVC-2026-11-25"),
        // 2026-12-24 - T2 - CME-SVC-2026-12-22 - Christmas Eve, 12:05 CT close.
        (2026, 12, 24, early_close(12 * 3_600 + 5 * 60), T2, "CME-SVC-2026-12-22"),
        // 2026-12-25 - T2 - CME-SVC-2026-12-24 - Christmas Day, no events published.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 - T2 - CME-SVC-2026-12-31 - New Year's Day, no events published.
        (2027, 1, 1, Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-01-18 - T2 - CME-SVC-2027-01-17 - Martin Luther King Jr. Day, no events.
        (2027, 1, 18, Closed, T2, "CME-SVC-2027-01-17"),
        // 2027-02-15 - T2 - CME-SVC-2027-02-14 - Presidents' Day, no events published.
        (2027, 2, 15, Closed, T2, "CME-SVC-2027-02-14"),
        // 2027-03-26 - T2 - CME-SVC-2027-03-25 - Good Friday, no events published.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 - T2 - CME-SVC-2027-05-30 - Memorial Day, no events published.
        (2027, 5, 31, Closed, T2, "CME-SVC-2027-05-30"),
        // 2027-06-18 - T2 - CME-SVC-2027-06-17 - Juneteenth observed, no events.
        (2027, 6, 18, Closed, T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 - T2 - CME-SVC-2027-07-04 - Independence Day observed, no events.
        (2027, 7, 5, Closed, T2, "CME-SVC-2027-07-04"),
        // 2027-09-06 - T2 - CME-SVC-2027-09-05 - Labor Day, no events published.
        (2027, 9, 6, Closed, T2, "CME-SVC-2027-09-05"),
        // 2027-11-25 - T2 - CME-SVC-2027-11-24 - Thanksgiving, no events published.
        (2027, 11, 25, Closed, T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - day after Thanksgiving, 12:05 CT close.
        (2027, 11, 26, early_close(12 * 3_600 + 5 * 60), T2, "CME-SVC-2027-11-24"),
        // 2027-12-24 - T2 - CME-SVC-2027-12-22 - Christmas Friday closure, no events.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
