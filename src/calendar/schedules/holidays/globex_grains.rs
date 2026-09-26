// SPDX-License-Identifier: MIT-0

//! CBOT standard-size grain and oilseed futures holiday rows, 2010-2012,
//! 2013-2015, 2016-2018, 2019-2021, 2022-2024 and 2025-2027
//! (LAW-HOLIDAY-SCOPE).
//!
//! Keyed by the crate's own venue-local trade date in `America/Chicago`
//! (design memo D1), never by CME's event date. That conversion is not the
//! identity for this family: the sourced grid is a wrapping one — an evening
//! leg at 19:00 CT that runs to 07:45 CT the next morning, then an 08:30-13:20
//! CT day session whose close is the trading day's final close — so the
//! trading day for trade date `D` opens on `D - 1`. CME's own printed trade
//! date corroborates every row below, and it is what settles the conversion
//! wherever an event-date record could be read two ways.
//!
//! Three consequences of that shape, worked out once here so the rows read
//! plainly:
//!
//! * A holiday eve on which CME publishes the whole day session and then no
//!   `16:45 preopen` / `19:00 open` pair loses its evening leg to the
//!   neighbouring `Closed` row, which owns the holiday's trade date (memo D6,
//!   §1.6 case 3). It is nonetheless a row of its own from 2025-04-17 on: CME
//!   prints that day's `14:30 pcp` carrying the **eve's** trade date, and the
//!   crate keys one `ReplacementBlocks` row per eve to restate the complete
//!   day, so the queue survives the closure that follows it.
//! * A day after a mid-week closure, on which CME publishes a `06:00 preopen`
//!   in place of the usual `07:45 paused` / `08:00 preopen` pair, has lost its
//!   prior-evening leg. A scalar `late_open` at the day session's own 08:30 CT
//!   open states where matching begins — `08:30` is earlier than the trading
//!   day's normal 19:00 CT first open, so the cutoff lands on the trade date
//!   itself rather than the preceding local date (memo D7) — but it cannot
//!   state the `06:00` order-entry window beside it, so from 2025-01-02 the
//!   four such dates carry the whole day as a `ReplacementBlocks` row instead.
//! * The three days after Thanksgiving carry both — no prior-evening leg and a
//!   12:05 CT final close — and are the only `LateOpenAndEarlyClose` rows of
//!   the 2016-2018 era.
//!
//! Of the 2022-2027 rows below, the 2022-2024 era mixes T1 and T2 and every
//! 2025-2027 row is T2.
//!
//! **2016-2018.** CME's own published Globex holiday schedules are the T1
//! source for this era: per-holiday PDFs inside the 2016 annual bundle, a
//! standalone 2016 New Year's PDF, and per-product-group sheets for 2017 and
//! 2018, whose annual bundles carry CME's final revisions. The grid is a 19:00
//! CT evening leg into the 08:30-13:20 CT day session, whose close is the
//! trading day's final close, so an early close is the printed final close
//! stated on the trade date it ends and a closure removes the trade date with
//! its prior-evening leg. Thirty-six rows: twenty-seven closures, five early
//! closes at 12:05 CT, three **late-open-and-early-close** rows on the day
//! after Thanksgiving in 2016, 2017 and 2018, where CME withdrew the
//! prior-evening leg and printed an 08:30 CT open beside the 12:05 CT close,
//! and one late open alone, 2018-12-26 at 08:30 CT. The dates
//! are the thirty-four the financial families state plus 2016-12-23 and
//! 2017-12-22.
//!
//! **2022-2024.** The era's grid is the same 19:00 CT evening leg into the
//! 08:30-13:20 CT day session. Thirty-nine rows: twenty-six closures, the
//! three `LateOpenAndEarlyClose` days after Thanksgiving, whose 08:30 CT open is
//! the same late open a removed leg produces, one early close at 12:05 CT on
//! 2024-12-24, the six late opens on 2022-07-05, 2023-07-05, 2023-12-26,
//! 2024-01-02, 2024-07-05 and 2024-12-26, and the three `Unsourced` dates below
//! that the operator published nothing for. The 2022 rows, the 2023 rows CME
//! published a holiday schedule for, and the 2024 New Year's Day row are
//! **T1**; the three 2023 dates it published nothing for and the rest of 2024
//! are **T2**.
//!
//! **2010-2012.** CME's own holiday-calendar PDFs are the T1 source. The era's
//! grid is an 18:00 CT evening leg (17:00 from 2012-05-20) into a 09:30-13:15 CT
//! day session. Rows are ten **full closures** (New Year's Day, Christmas Day,
//! both Good Fridays, 2011's Good Friday, Independence Day and Thanksgiving
//! 2012, and the observed days around them), **early closes** at 12:00 CT on
//! the four year-end half-days and 2012-07-03, and **late opens** where CME
//! states one: 09:30 CT on 2011-12-27, 2012-01-03, 2012-07-05 and 2012-12-26,
//! and 19:00 CT on 2012-05-28 and 2012-09-03 after the grid moved. The 2012 day
//! after Thanksgiving carries both. Dates on which the stated re-open is the
//! family's ordinary evening open ship no row and are listed in the evidence
//! file.
//! Every row is **T2**: CME's trading-hours service, the endpoint
//! `cmegroup.com/trading-hours.html` itself calls, read as bytes and saved.
//! CME publishes no T1 per-asset-class rendering of these instants; that gap,
//! the eight 2025 windows that survive only in a pre-holiday capture, the
//! order-entry deviations no shipped row states, and the audited
//! weekend dates that ship no row are all recorded in
//! [`docs/evidence/globex_grains.md`](../../../../../docs/evidence/globex_grains.md).
//!
//! Scope is the standard-size CBOT grain and oilseed complex — corn, soybeans
//! and wheat. CME's service answers for `ZC`, `ZS` and `ZW` separately and the
//! three agree on every date the table audits, so the family takes one table.
//! Mini grains are a separate key and a separate table.

use super::fences::{early_close, late_open, late_open_and_early_close};
use super::{
    EvidenceTier::{T1, T2},
    HolidayKind::{Closed, ReplacementBlocks, Unsourced},
    HolidayTable, holidays,
};
use crate::calendar::exceptions::ExceptionBlock;

/// 08:30 CT, the day session's own open and the late-open instant every
/// evening-leg-less trade date in this block falls back to.
const DAY_OPEN: u32 = 8 * 3_600 + 30 * 60;

/// 12:05 CT, the only early final close CME publishes for this family in this
/// window.
const HALF_DAY_CLOSE: u32 = 12 * 3_600 + 5 * 60;

/// The complete trading day of a closure eve, keyed to the eve's own trade
/// date.
///
/// CME withholds the eve's `16:45 preopen` / `19:00 open` pair — that leg
/// belongs to the following, closed trade date — and prints the eve's own
/// `14:30 pcp` carrying the **eve's** trade date. The crate dates an
/// order-entry occurrence by the session it feeds, so the ordinary week puts
/// that queue on the holiday and the neighbouring `Closed` row deletes it. A
/// row keyed to the eve states the day whole: the ordinary prior-evening queue
/// and leg, the morning queue, the day session, and the post-close queue CME
/// publishes.
///
/// The ordinary phases are the family's own normal-week values, quoted here
/// because a replacement states the complete day, and both queues are order
/// entry: no trade matches in them, and none is claimed. Because a replacement
/// carries its own trade-date assignment, the eve's queue reads with the eve's
/// trade date — the operator's own label. On every other date the ordinary
/// assignment stands and the queue still reads with the trade date it feeds;
/// that divergence is recorded in
/// [`docs/evidence/globex_grains.md`](../../../../../docs/evidence/globex_grains.md).
pub(crate) static CLOSURE_EVE_BLOCKS: [ExceptionBlock; 5] = [
    ExceptionBlock::order_entry(-1, 16 * 3_600 + 45 * 60, 19 * 3_600),
    ExceptionBlock::extended(-1, 19 * 3_600, 7 * 3_600 + 45 * 60),
    ExceptionBlock::order_entry(0, 8 * 3_600, 8 * 3_600 + 30 * 60),
    ExceptionBlock::regular(0, 8 * 3_600 + 30 * 60, 13 * 3_600 + 20 * 60),
    ExceptionBlock::order_entry(0, 14 * 3_600 + 30 * 60, 16 * 3_600),
];

/// The complete trading day of the four 2025-2027 trade dates that follow a
/// mid-week closure.
///
/// The closure removes the prior-evening leg, so the day opens with the
/// `06:00 preopen` CME prints in place of the ordinary `07:45 paused` /
/// `08:00 preopen` pair, and then runs the ordinary day session and post-close
/// queue. A scalar `late_open` row cannot state the queue beside the open, so
/// the trade date ships as a replacement instead.
///
/// Evidence: `docs/evidence/globex_grains.md`.
pub(crate) static LATE_OPEN_BLOCKS: [ExceptionBlock; 3] = [
    ExceptionBlock::order_entry(0, 6 * 3_600, 8 * 3_600 + 30 * 60),
    ExceptionBlock::regular(0, 8 * 3_600 + 30 * 60, 13 * 3_600 + 20 * 60),
    ExceptionBlock::order_entry(0, 14 * 3_600 + 30 * 60, 16 * 3_600),
];

/// The family's built-in holiday rows and the windows they were audited over.
///
///  Six audited eras: 2010-2012 at T1, 2013-2015 at T1, 2016-2018 at T1,
/// 2019-2021 at T1, 2022-2024 at T1/T2 and 2025-2027 at T2.
///  Every audited interval is contiguous, and nothing before 2010-01-01
/// has a table at all: that span lies outside every window, so
/// `holiday_on` has no answer there rather than reporting a normal date.
///
/// Coverage ends at
/// 2027-12-31, the end of the operator's published future
/// (LAW-NO-FABRICATED-DATES permits encoding it ahead of its effective day);
/// CME's 2028-01-01 record sits outside every window and ships no row.
// Evidence: docs/evidence/globex_grains.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2012, 12, 31), (2013, 1, 1) ..= (2015, 12, 31), (2016, 1, 1) ..= (2018, 12, 31), (2019, 1, 1) ..= (2021, 12, 31), (2022, 1, 1) ..= (2024, 12, 31), (2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2010-01-01 - T1 - 2010-new-years.pdf - closed: new year's day 2010.
        (2010, 1, 1, Closed, T1, "2010-new-years.pdf @2010-02-15T05:16:52Z"),
        // 2010-04-02 - T1 - 2010-good-friday.pdf - closed: good friday 2010.
        (2010, 4, 2, Closed, T1, "2010-good-friday.pdf @2010-06-01T11:19:16Z"),
        (2010, 11, 26, early_close(12 * 3_600), T1, "2010-thanksgiving.pdf @2010-11-22T09:40:12Z"),
        // 2010-12-24 - T1 - 2010-christmas.pdf - closed: christmas day 2010 observed.
        (2010, 12, 24, Closed, T1, "2010-christmas.pdf @2010-12-14T06:12:38Z"),
        (2010, 12, 31, early_close(12 * 3_600), T1, "2011-new-years.pdf @2011-11-01T14:39:45Z"),
        // 2011-04-22 - T1 - 2011-good-friday.pdf - closed: good friday 2011.
        (2011, 4, 22, Closed, T1, "2011-good-friday.pdf @2011-10-28T02:37:07Z"),
        (2011, 11, 25, early_close(12 * 3_600), T1, "2011-thanksgiving.pdf @2011-11-24T18:52:46Z"),
        // 2011-12-26 - T1 - 2011-christmas.pdf - closed: christmas day 2011 observed.
        (2011, 12, 26, Closed, T1, "2011-christmas.pdf @2012-01-25T02:05:48Z"),
        (2011, 12, 27, late_open(9 * 3_600 + 30 * 60), T1, "2011-christmas.pdf @2012-01-25T02:05:48Z"),
        // 2012-01-02 - T1 - 2012-new-years.pdf - closed: new year's day 2012 observed.
        (2012, 1, 2, Closed, T1, "2012-new-years.pdf @2012-01-25T02:54:30Z"),
        (2012, 1, 3, late_open(9 * 3_600 + 30 * 60), T1, "2012-new-years.pdf @2012-01-25T02:54:30Z"),
        // 2012-04-06 - T1 - 2012-good-friday.pdf - closed: good friday 2012.
        (2012, 4, 6, Closed, T1, "2012-good-friday.pdf @2012-04-17T00:42:47Z"),
        (2012, 5, 28, late_open(19 * 3_600), T1, "2012-memorial-day.pdf @2012-09-15T00:37:14Z"),
        (2012, 7, 3, early_close(12 * 3_600), T1, "2012-4th-of-july.pdf @2012-09-15T00:39:23Z"),
        // 2012-07-04 - T1 - 2012-4th-of-july.pdf - closed: independence day 2012.
        (2012, 7, 4, Closed, T1, "2012-4th-of-july.pdf @2012-09-15T00:39:23Z"),
        (2012, 7, 5, late_open(9 * 3_600 + 30 * 60), T1, "2012-4th-of-july.pdf @2012-09-15T00:39:23Z"),
        (2012, 9, 3, late_open(19 * 3_600), T1, "2012-labor-day.pdf @2012-09-15T00:34:37Z"),
        // 2012-11-22 - T1 - 2012-thanksgiving.pdf - closed: thanksgiving day 2012.
        (2012, 11, 22, Closed, T1, "2012-thanksgiving.pdf @2013-01-27T22:39:01Z"),
        (2012, 11, 23, late_open_and_early_close(9 * 3_600 + 30 * 60, 12 * 3_600), T1, "2012-thanksgiving.pdf @2013-01-27T22:39:01Z"),
        (2012, 12, 24, early_close(12 * 3_600), T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        // 2012-12-25 - T1 - 2012-christmas.pdf - closed: christmas day 2012.
        (2012, 12, 25, Closed, T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        (2012, 12, 26, late_open(9 * 3_600 + 30 * 60), T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        // 2013-01-01 - T1 - 2013-new-years.pdf @2013-04-14T19:41:46Z - CME prints no session running through this date.
        (2013, 1, 1, Closed, T1, "2013-new-years.pdf @2013-04-14T19:41:46Z"),
        // 2013-01-02 - T1 - 2013-new-years.pdf @2013-04-14T19:41:46Z - the trade date's first open is 9:30 CT: the evening leg that would have opened earlier did not run.
        (2013, 1, 2, late_open(9 * 3_600 + 30 * 60), T1, "2013-new-years.pdf @2013-04-14T19:41:46Z"),
        // 2013-01-21 - T1 - 2013-martin-luther-king.pdf @2012-11-19T00:16:09Z - CME prints no session running through this date.
        (2013, 1, 21, Closed, T1, "2013-martin-luther-king.pdf @2012-11-19T00:16:09Z"),
        // 2013-02-18 - T1 - 2013-presidents-day.pdf @2013-03-09T11:53:37Z - CME prints no session running through this date.
        (2013, 2, 18, Closed, T1, "2013-presidents-day.pdf @2013-03-09T11:53:37Z"),
        // 2013-03-29 - T1 - 2013-good-friday.pdf @2013-06-23T19:59:25Z - CME prints no session running through this date.
        (2013, 3, 29, Closed, T1, "2013-good-friday.pdf @2013-06-23T19:59:25Z"),
        // 2013-05-27 - T1 - 2013-memorial-day.pdf @2013-06-23T20:36:04Z - CME prints no session running through this date.
        (2013, 5, 27, Closed, T1, "2013-memorial-day.pdf @2013-06-23T20:36:04Z"),
        // 2013-07-03 - T1 - 2013-4th-of-july.pdf @2013-06-23T20:58:25Z - the printed final close 12:00 CT is earlier than the family's ordinary 13:15 CT.
        (2013, 7, 3, early_close(12 * 3_600), T1, "2013-4th-of-july.pdf @2013-06-23T20:58:25Z"),
        // 2013-07-04 - T1 - 2013-4th-of-july.pdf @2013-06-23T20:58:25Z - CME prints no session running through this date.
        (2013, 7, 4, Closed, T1, "2013-4th-of-july.pdf @2013-06-23T20:58:25Z"),
        // 2013-07-05 - T1 - 2013-4th-of-july.pdf @2013-06-23T20:58:25Z - the trade date's first open is 8:30 CT: the evening leg that would have opened earlier did not run.
        (2013, 7, 5, late_open(8 * 3_600 + 30 * 60), T1, "2013-4th-of-july.pdf @2013-06-23T20:58:25Z"),
        // 2013-09-02 - T1 - 2013-labor-day.pdf @2013-09-02T17:08:41Z - CME prints no session running through this date.
        (2013, 9, 2, Closed, T1, "2013-labor-day.pdf @2013-09-02T17:08:41Z"),
        // 2013-11-28 - T1 - 2013-thanksgiving.pdf @2014-02-14T06:28:36Z - CME prints no session running through this date.
        (2013, 11, 28, Closed, T1, "2013-thanksgiving.pdf @2014-02-14T06:28:36Z"),
        // 2013-11-29 - T1 - 2013-thanksgiving.pdf @2014-02-14T06:28:36Z - the trade date's first open is 8:30 CT: the evening leg that would have opened earlier did not run; the printed final close 12:00 CT is earlier than the family's ordinary 13:15 CT.
        (2013, 11, 29, late_open_and_early_close(8 * 3_600 + 30 * 60, 12 * 3_600), T1, "2013-thanksgiving.pdf @2014-02-14T06:28:36Z"),
        // 2013-12-24 - T1 - 2013-christmas.pdf @2014-04-12T06:24:28Z - the printed final close 12:00 CT is earlier than the family's ordinary 13:15 CT.
        (2013, 12, 24, early_close(12 * 3_600), T1, "2013-christmas.pdf @2014-04-12T06:24:28Z"),
        // 2013-12-25 - T1 - 2013-christmas.pdf @2014-04-12T06:24:28Z - CME prints no session running through this date.
        (2013, 12, 25, Closed, T1, "2013-christmas.pdf @2014-04-12T06:24:28Z"),
        // 2013-12-26 - T1 - 2013-christmas.pdf @2014-04-12T06:24:28Z - the trade date's first open is 8:30 CT: the evening leg that would have opened earlier did not run.
        (2013, 12, 26, late_open(8 * 3_600 + 30 * 60), T1, "2013-christmas.pdf @2014-04-12T06:24:28Z"),
        // 2014-01-01 - T1 - 2014-new-years.pdf @2013-10-07T20:58:00Z - CME prints no session running through this date.
        (2014, 1, 1, Closed, T1, "2014-new-years.pdf @2013-10-07T20:58:00Z"),
        // 2014-01-02 - T1 - 2014-new-years.pdf @2013-10-07T20:58:00Z - the trade date's first open is 8:30 CT: the evening leg that would have opened earlier did not run.
        (2014, 1, 2, late_open(8 * 3_600 + 30 * 60), T1, "2014-new-years.pdf @2013-10-07T20:58:00Z"),
        // 2014-01-20 - T1 - 2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z - CME prints no session running through this date.
        (2014, 1, 20, Closed, T1, "2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z"),
        // 2014-02-17 - T1 - 2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z - CME prints no session running through this date.
        (2014, 2, 17, Closed, T1, "2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z"),
        // 2014-04-18 - T1 - 2014-good-friday-holiday-schedule.pdf @2014-03-26T15:27:35Z - CME prints no session running through this date.
        (2014, 4, 18, Closed, T1, "2014-good-friday-holiday-schedule.pdf @2014-03-26T15:27:35Z"),
        // 2014-05-26 - T1 - 2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z - CME prints no session running through this date.
        (2014, 5, 26, Closed, T1, "2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z"),
        // 2014-07-03 - T1 - 2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z - the printed final close 12:00 CT is earlier than the family's ordinary 13:15 CT.
        (2014, 7, 3, early_close(12 * 3_600), T1, "2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z"),
        // 2014-07-04 - T1 - 2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z - CME prints no session running through this date.
        (2014, 7, 4, Closed, T1, "2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z"),
        // 2014-07-07 - T1 - 2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z - the trade date's first open is 8:30 CT: the evening leg that would have opened earlier did not run.
        (2014, 7, 7, late_open(8 * 3_600 + 30 * 60), T1, "2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z"),
        // 2014-09-01 - T1 - 2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z - CME prints no session running through this date.
        (2014, 9, 1, Closed, T1, "2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z"),
        // 2014-11-27 - T1 - 2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z - CME prints no session running through this date.
        (2014, 11, 27, Closed, T1, "2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z"),
        // 2014-11-28 - T1 - 2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z - the trade date's first open is 8:30 CT: the evening leg that would have opened earlier did not run; the printed final close 12:00 CT is earlier than the family's ordinary 13:15 CT.
        (2014, 11, 28, late_open_and_early_close(8 * 3_600 + 30 * 60, 12 * 3_600), T1, "2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z"),
        // 2014-12-24 - T1 - 2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z - the printed final close 12:00 CT is earlier than the family's ordinary 13:15 CT.
        (2014, 12, 24, early_close(12 * 3_600), T1, "2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z"),
        // 2014-12-25 - T1 - 2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z - CME prints no session running through this date.
        (2014, 12, 25, Closed, T1, "2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z"),
        // 2014-12-26 - T1 - 2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z - the trade date's first open is 8:30 CT: the evening leg that would have opened earlier did not run.
        (2014, 12, 26, late_open(8 * 3_600 + 30 * 60), T1, "2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z"),
        // 2015-01-01 - T1 - 2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z - CME prints no session running through this date.
        (2015, 1, 1, Closed, T1, "2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z"),
        // 2015-01-02 - T1 - 2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z - the trade date's first open is 8:30 CT: the evening leg that would have opened earlier did not run.
        (2015, 1, 2, late_open(8 * 3_600 + 30 * 60), T1, "2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z"),
        // 2015-01-19 - T1 - 2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z - CME prints no session running through this date.
        (2015, 1, 19, Closed, T1, "2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z"),
        // 2015-02-16 - T1 - 2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z - CME prints no session running through this date.
        (2015, 2, 16, Closed, T1, "2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z"),
        // 2015-04-03 - T1 - 2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z - CME prints no session running through this date.
        (2015, 4, 3, Closed, T1, "2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z"),
        // 2015-05-25 - T1 - 2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z - CME prints no session running through this date.
        (2015, 5, 25, Closed, T1, "2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z"),
        // 2015-07-02 - T1 - 2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z - the printed final close 12:00 CT is earlier than the family's ordinary 13:15 CT.
        (2015, 7, 2, early_close(12 * 3_600), T1, "2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z"),
        // 2015-07-03 - T1 - 2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z - CME prints no session running through this date.
        (2015, 7, 3, Closed, T1, "2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z"),
        // 2015-09-07 - T1 - 2015-labor-day-holiday-schedule.pdf @2015-08-24T02:30:39Z - CME prints no session running through this date.
        (2015, 9, 7, Closed, T1, "2015-labor-day-holiday-schedule.pdf @2015-08-24T02:30:39Z"),
        // 2015-11-26 - T1 - 2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z - CME prints no session running through this date.
        (2015, 11, 26, Closed, T1, "2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z"),
        // 2015-11-27 - T1 - 2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z - the trade date's first open is 8:30 CT: the evening leg that would have opened earlier did not run; the printed final close 12:05 CT is earlier than the family's ordinary 13:20 CT.
        (2015, 11, 27, late_open_and_early_close(8 * 3_600 + 30 * 60, 12 * 3_600 + 5 * 60), T1, "2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z"),
        // 2015-12-24 - T1 - 2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z - the printed final close 12:05 CT is earlier than the family's ordinary 13:20 CT.
        (2015, 12, 24, early_close(12 * 3_600 + 5 * 60), T1, "2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z"),
        // 2015-12-25 - T1 - 2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z - CME prints no session running through this date.
        (2015, 12, 25, Closed, T1, "2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z"),
        // 2016-01-01 - T1 - 2016-new-years-holiday-schedule.pdf @2016-01-08 - closed: no day session and no prior-evening leg.
        (2016, 1, 1, Closed, T1, "2016-new-years-holiday-schedule.pdf @2016-01-08"),
        // 2016-01-18 - T1 - 2016-holiday-calendars.zip#2016-martin-luther-king-holiday-schedule.pdf @2017-06-28 - closed: no day session and no prior-evening leg.
        (2016, 1, 18, Closed, T1, "2016-holiday-calendars.zip#2016-martin-luther-king-holiday-schedule.pdf @2017-06-28"),
        // 2016-02-15 - T1 - 2016-holiday-calendars.zip#2016-presidents-day-holiday-schedule.pdf @2017-06-28 - closed: no day session and no prior-evening leg.
        (2016, 2, 15, Closed, T1, "2016-holiday-calendars.zip#2016-presidents-day-holiday-schedule.pdf @2017-06-28"),
        // 2016-03-25 - T1 - 2016-holiday-calendars.zip#2016-good-friday-holiday-schedule.pdf @2017-06-28 - closed: no day session and no prior-evening leg.
        (2016, 3, 25, Closed, T1, "2016-holiday-calendars.zip#2016-good-friday-holiday-schedule.pdf @2017-06-28"),
        // 2016-05-30 - T1 - 2016-holiday-calendars.zip#2016-memorial-day-holiday-schedule.pdf @2017-06-28 - closed: no day session and no prior-evening leg.
        (2016, 5, 30, Closed, T1, "2016-holiday-calendars.zip#2016-memorial-day-holiday-schedule.pdf @2017-06-28"),
        // 2016-07-04 - T1 - 2016-holiday-calendars.zip#2016-4th-of-july-holiday-schedule.pdf @2017-06-28 - closed: no day session and no prior-evening leg.
        (2016, 7, 4, Closed, T1, "2016-holiday-calendars.zip#2016-4th-of-july-holiday-schedule.pdf @2017-06-28"),
        // 2016-09-05 - T1 - 2016-holiday-calendars.zip#2016-labor-day-holiday-schedule.pdf @2017-06-28 - closed: no day session and no prior-evening leg.
        (2016, 9, 5, Closed, T1, "2016-holiday-calendars.zip#2016-labor-day-holiday-schedule.pdf @2017-06-28"),
        // 2016-11-24 - T1 - 2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28 - closed: no day session and no prior-evening leg.
        (2016, 11, 24, Closed, T1, "2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28"),
        // 2016-11-25 - T1 - 2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28 - no prior-evening leg, then the early close 12:05 CT.
        (2016, 11, 25, late_open_and_early_close(8 * 3_600 + 30 * 60, 12 * 3_600 + 5 * 60), T1, "2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28"),
        // 2016-12-23 - T1 - 2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28 - early close 12:05 CT.
        (2016, 12, 23, early_close(12 * 3_600 + 5 * 60), T1, "2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28"),
        // 2016-12-26 - T1 - 2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28 - closed: no day session and no prior-evening leg.
        (2016, 12, 26, Closed, T1, "2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28"),
        // 2017-01-02 - T1 - 2016-holiday-calendars.zip#2017-new-years-holiday-schedule.pdf @2017-06-28 - closed: no day session and no prior-evening leg.
        (2017, 1, 2, Closed, T1, "2016-holiday-calendars.zip#2017-new-years-holiday-schedule.pdf @2017-06-28"),
        // 2017-01-16 - T1 - 2017-martin-luther-king-holiday-schedule.xls @2017-06-28 - closed: no day session and no prior-evening leg.
        (2017, 1, 16, Closed, T1, "2017-martin-luther-king-holiday-schedule.xls @2017-06-28"),
        // 2017-02-20 - T1 - 2017-presidents-day-holiday-schedule.xls @2017-06-28 - closed: no day session and no prior-evening leg.
        (2017, 2, 20, Closed, T1, "2017-presidents-day-holiday-schedule.xls @2017-06-28"),
        // 2017-04-14 - T1 - 2017-good-friday-holiday-schedule.xls @2017-05-05 - closed: no day session and no prior-evening leg.
        (2017, 4, 14, Closed, T1, "2017-good-friday-holiday-schedule.xls @2017-05-05"),
        // 2017-05-29 - T1 - 2017-memorial-day-holiday-schedule.xls @2017-10-25 - closed: no day session and no prior-evening leg.
        (2017, 5, 29, Closed, T1, "2017-memorial-day-holiday-schedule.xls @2017-10-25"),
        // 2017-07-03 - T1 - 2017-4th-of-july-holiday-schedule.xls @2017-10-25 - early close 12:05 CT.
        (2017, 7, 3, early_close(12 * 3_600 + 5 * 60), T1, "2017-4th-of-july-holiday-schedule.xls @2017-10-25"),
        // 2017-07-04 - T1 - 2017-4th-of-july-holiday-schedule.xls @2017-10-25 - closed: no day session and no prior-evening leg.
        (2017, 7, 4, Closed, T1, "2017-4th-of-july-holiday-schedule.xls @2017-10-25"),
        // 2017-09-04 - T1 - 2017-labor-day-holiday-schedule.xls @2017-10-25 - closed: no day session and no prior-evening leg.
        (2017, 9, 4, Closed, T1, "2017-labor-day-holiday-schedule.xls @2017-10-25"),
        // 2017-11-23 - T1 - 2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26 - closed: no day session and no prior-evening leg.
        (2017, 11, 23, Closed, T1, "2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26"),
        // 2017-11-24 - T1 - 2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26 - no prior-evening leg, then the early close 12:05 CT.
        (2017, 11, 24, late_open_and_early_close(8 * 3_600 + 30 * 60, 12 * 3_600 + 5 * 60), T1, "2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26"),
        // 2017-12-22 - T1 - 2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26 - early close 12:05 CT.
        (2017, 12, 22, early_close(12 * 3_600 + 5 * 60), T1, "2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26"),
        // 2017-12-25 - T1 - 2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26 - closed: no day session and no prior-evening leg.
        (2017, 12, 25, Closed, T1, "2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26"),
        // 2018-01-01 - T1 - 2018-new-years-holiday-schedule.xls @2018-01-06 - closed: no day session and no prior-evening leg.
        (2018, 1, 1, Closed, T1, "2018-new-years-holiday-schedule.xls @2018-01-06"),
        // 2018-01-15 - T1 - 2018-martin-luther-king-holiday-schedule.xls @2018-05-08 - closed: no day session and no prior-evening leg.
        (2018, 1, 15, Closed, T1, "2018-martin-luther-king-holiday-schedule.xls @2018-05-08"),
        // 2018-02-19 - T1 - 2018-presidents-day-holiday-schedule.xls @2018-05-08 - closed: no day session and no prior-evening leg.
        (2018, 2, 19, Closed, T1, "2018-presidents-day-holiday-schedule.xls @2018-05-08"),
        // 2018-03-30 - T1 - 2018-holiday-calendars.zip#2018-good-friday-holiday-schedule.xls @2026-08-30 - closed: no day session and no prior-evening leg.
        (2018, 3, 30, Closed, T1, "2018-holiday-calendars.zip#2018-good-friday-holiday-schedule.xls @2026-08-30"),
        // 2018-05-28 - T1 - 2018-holiday-calendars.zip#2018-memorial-day-holiday-schedule.xls @2026-08-30 - closed: no day session and no prior-evening leg.
        (2018, 5, 28, Closed, T1, "2018-holiday-calendars.zip#2018-memorial-day-holiday-schedule.xls @2026-08-30"),
        // 2018-07-03 - T1 - 2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30 - early close 12:05 CT.
        (2018, 7, 3, early_close(12 * 3_600 + 5 * 60), T1, "2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30"),
        // 2018-07-04 - T1 - 2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30 - closed: no day session and no prior-evening leg.
        (2018, 7, 4, Closed, T1, "2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30"),
        // 2018-09-03 - T1 - 2018-holiday-calendars.zip#2018-labor-day-holiday-schedule.xls @2026-08-30 - closed: no day session and no prior-evening leg.
        (2018, 9, 3, Closed, T1, "2018-holiday-calendars.zip#2018-labor-day-holiday-schedule.xls @2026-08-30"),
        // 2018-11-22 - T1 - 2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30 - closed: no day session and no prior-evening leg.
        (2018, 11, 22, Closed, T1, "2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30"),
        // 2018-11-23 - T1 - 2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30 - no prior-evening leg, then the early close 12:05 CT.
        (2018, 11, 23, late_open_and_early_close(8 * 3_600 + 30 * 60, 12 * 3_600 + 5 * 60), T1, "2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30"),
        // 2018-12-24 - T1 - 2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30 - early close 12:05 CT.
        (2018, 12, 24, early_close(12 * 3_600 + 5 * 60), T1, "2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30"),
        // 2018-12-25 - T1 - 2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30 - closed: no day session and no prior-evening leg.
        (2018, 12, 25, Closed, T1, "2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30"),
        // 2018-12-26 - T1 - 2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30 - late open 08:30 CT: no prior-evening leg.
        (2018, 12, 26, late_open(8 * 3_600 + 30 * 60), T1, "2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30"),
        // 2019-01-01 - T1 - 2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z - CME prints no session running through this date.
        (2019, 1, 1, Closed, T1, "2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z"),
        // 2019-01-02 - T1 - 2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z - the prior-evening leg of trade date 2019-01-02 did not run: the operator prints the day session's own 08:30 CT open, later than the ordinary 19:00 CT first open.
        (2019, 1, 2, late_open(8 * 3_600 + 30 * 60), T1, "2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z"),
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
        // 2019-07-03 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - the printed final close 12:05 CT is earlier than the ordinary 13:20 CT close.
        (2019, 7, 3, early_close(12 * 3_600 + 5 * 60), T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-07-04 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - CME prints no session running through this date.
        (2019, 7, 4, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-07-05 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - the prior-evening leg of trade date 2019-07-05 did not run: the operator prints the day session's own 08:30 CT open, later than the ordinary 19:00 CT first open.
        (2019, 7, 5, late_open(8 * 3_600 + 30 * 60), T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-09-02 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-labor-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - CME prints no session running through this date.
        (2019, 9, 2, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-labor-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-11-28 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - CME prints no session running through this date.
        (2019, 11, 28, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-11-29 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - the operator's reopen cell names the day session as regular on trade date 2019-11-29, so the first trade there is the ordinary 08:30 CT day-session open — later than the family's 19:00 CT first open, which falls on the preceding local date; the printed final close 12:05 CT is earlier than the ordinary 13:20 CT close.
        (2019, 11, 29, late_open_and_early_close(8 * 3_600 + 30 * 60, 12 * 3_600 + 5 * 60), T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-12-24 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - the printed final close 12:05 CT is earlier than the ordinary 13:20 CT close.
        (2019, 12, 24, early_close(12 * 3_600 + 5 * 60), T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-12-25 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - CME prints no session running through this date.
        (2019, 12, 25, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-12-26 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - the prior-evening leg of trade date 2019-12-26 did not run: the operator prints the day session's own 08:30 CT open, later than the ordinary 19:00 CT first open.
        (2019, 12, 26, late_open(8 * 3_600 + 30 * 60), T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2020-01-01 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - CME prints no session running through this date.
        (2020, 1, 1, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2020-01-02 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - the prior-evening leg of trade date 2020-01-02 did not run: the operator prints the day session's own 08:30 CT open, later than the ordinary 19:00 CT first open.
        (2020, 1, 2, late_open(8 * 3_600 + 30 * 60), T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
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
        // 2020-07-02 - T1 - 2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - the printed final close 12:05 CT is earlier than the ordinary 13:20 CT close.
        (2020, 7, 2, early_close(12 * 3_600 + 5 * 60), T1, "2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-07-03 - T1 - 2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - CME prints no session running through this date.
        (2020, 7, 3, Closed, T1, "2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-09-07 - T1 - 2020-holiday-calendars.zip#2020-labor-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - CME prints no session running through this date.
        (2020, 9, 7, Closed, T1, "2020-holiday-calendars.zip#2020-labor-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-11-26 - T1 - 2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - CME prints no session running through this date.
        (2020, 11, 26, Closed, T1, "2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-11-27 - T1 - 2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - the operator's reopen cell names the day session as regular on trade date 2020-11-27, so the first trade there is the ordinary 08:30 CT day-session open — later than the family's 19:00 CT first open, which falls on the preceding local date; the printed final close 12:05 CT is earlier than the ordinary 13:20 CT close.
        (2020, 11, 27, late_open_and_early_close(8 * 3_600 + 30 * 60, 12 * 3_600 + 5 * 60), T1, "2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-12-24 - T1 - 2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - the printed final close 12:05 CT is earlier than the ordinary 13:20 CT close.
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
        // 2021-07-06 - T1 - 2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - the prior-evening leg of trade date 2021-07-06 did not run: the operator prints the day session's own 08:30 CT open, later than the ordinary 19:00 CT first open.
        (2021, 7, 6, late_open(8 * 3_600 + 30 * 60), T1, "2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-09-06 - T1 - 2021-holiday-calendars.zip#2021-labor-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - CME prints no session running through this date.
        (2021, 9, 6, Closed, T1, "2021-holiday-calendars.zip#2021-labor-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-11-25 - T1 - 2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - CME prints no session running through this date.
        (2021, 11, 25, Closed, T1, "2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-11-26 - T1 - 2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - the prior-evening leg of trade date 2021-11-26 did not run: the operator prints the day session's own 08:30 CT open, later than the ordinary 19:00 CT first open; the printed final close 12:05 CT is earlier than the ordinary 13:20 CT close.
        (2021, 11, 26, late_open_and_early_close(8 * 3_600 + 30 * 60, 12 * 3_600 + 5 * 60), T1, "2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
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
        // 2022-07-05 - T1 - 2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z - late open 08:30 CT.
        (2022, 7, 5, late_open(8 * 3_600 + 30 * 60), T1, "2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z"),
        // 2022-09-05 - T1 - 2022-labor-day-holiday-schedule.xls @2022-07-04T06:54:41Z - closed: no trade date.
        (2022, 9, 5, Closed, T1, "2022-labor-day-holiday-schedule.xls @2022-07-04T06:54:41Z"),
        // 2022-11-24 - T1 - 2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z - closed: no trade date.
        (2022, 11, 24, Closed, T1, "2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z"),
        // 2022-11-25 - T1 - 2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z - late open 08:30 CT and early close 12:05 CT.
        (2022, 11, 25, late_open_and_early_close(8 * 3_600 + 30 * 60, 12 * 3_600 + 5 * 60), T1, "2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z"),
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
        // 2023-07-05 - T1 - 4th-of-july-2023.pdf @2023-06-27T12:50:57Z - late open 08:30 CT.
        (2023, 7, 5, late_open(8 * 3_600 + 30 * 60), T1, "4th-of-july-2023.pdf @2023-06-27T12:50:57Z"),
        // 2023-09-04 - T1 - labor-day-2023.pdf @2023-08-02T19:24:46Z - closed: no trade date.
        (2023, 9, 4, Closed, T1, "labor-day-2023.pdf @2023-08-02T19:24:46Z"),
        // 2023-11-23 - T1 - thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z - closed: no trade date.
        (2023, 11, 23, Closed, T1, "thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z"),
        // 2023-11-24 - T1 - thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z - late open 08:30 CT and early close 12:05 CT.
        (2023, 11, 24, late_open_and_early_close(8 * 3_600 + 30 * 60, 12 * 3_600 + 5 * 60), T1, "thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z"),
        // 2023-12-25 - T1 - christmas-day-2023.pdf @2026-07-19T09:52:48Z - closed: no trade date.
        (2023, 12, 25, Closed, T1, "christmas-day-2023.pdf @2026-07-19T09:52:48Z"),
        // 2023-12-26 - T1 - christmas-day-2023.pdf @2026-07-19T09:52:48Z - late open 08:30 CT.
        (2023, 12, 26, late_open(8 * 3_600 + 30 * 60), T1, "christmas-day-2023.pdf @2026-07-19T09:52:48Z"),
        // 2024-01-01 - T1 - new-years-day-2024.pdf @2026-08-11T16:57:16Z - closed: no trade date.
        (2024, 1, 1, Closed, T1, "new-years-day-2024.pdf @2026-08-11T16:57:16Z"),
        // 2024-01-02 - T1 - new-years-day-2024.pdf @2026-08-11T16:57:16Z - late open 08:30 CT.
        (2024, 1, 2, late_open(8 * 3_600 + 30 * 60), T1, "new-years-day-2024.pdf @2026-08-11T16:57:16Z"),
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
        // 2024-07-05 - T2 - CME-SVC-2024-07-03 - late open 08:30 CT.
        (2024, 7, 5, late_open(8 * 3_600 + 30 * 60), T2, "CME-SVC-2024-07-03"),
        // 2024-09-02 - T2 - CME-SVC-2024-09-01 - closed: no trade date.
        (2024, 9, 2, Closed, T2, "CME-SVC-2024-09-01"),
        // 2024-11-28 - T2 - CME-SVC-2024-11-27 - closed: no trade date.
        (2024, 11, 28, Closed, T2, "CME-SVC-2024-11-27"),
        // 2024-11-29 - T2 - CME-SVC-2024-11-27 - late open 08:30 CT and early close 12:05 CT.
        (2024, 11, 29, late_open_and_early_close(8 * 3_600 + 30 * 60, 12 * 3_600 + 5 * 60), T2, "CME-SVC-2024-11-27"),
        // 2024-12-24 - T2 - CME-SVC-2024-12-24 - early close 12:05 CT.
        (2024, 12, 24, early_close(12 * 3_600 + 5 * 60), T2, "CME-SVC-2024-12-24"),
        // 2024-12-25 - T2 - CME-SVC-2024-12-24 - closed: no trade date.
        (2024, 12, 25, Closed, T2, "CME-SVC-2024-12-24"),
        // 2024-12-26 - T2 - CME-SVC-2024-12-24 - late open 08:30 CT.
        (2024, 12, 26, late_open(8 * 3_600 + 30 * 60), T2, "CME-SVC-2024-12-24"),
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-02 - T2 - CME-SVC-2024-12-31 - no prior-evening leg; the day opens with CME's 06:00 CT pre-open, then the ordinary session and post-close queue.
        (2025, 1, 2, ReplacementBlocks(&LATE_OPEN_BLOCKS), T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 - T2 - CME-SVC-2025-01-19 - MLK Day, only a 19:00 CT open for trade date 01-21.
        (2025, 1, 20, Closed, T2, "CME-SVC-2025-01-19"),
        // 2025-02-17 - T2 - CME-SVC-2025-02-16 - Presidents' Day, only a 19:00 CT open for 02-18.
        (2025, 2, 17, Closed, T2, "CME-SVC-2025-02-16"),
        // 2025-04-17 - T2 - CME-SVC-2025-04-17 - Good Friday eve: the complete day, whose 14:30-16:00 CT queue CME dates to this trade date.
        (2025, 4, 17, ReplacementBlocks(&CLOSURE_EVE_BLOCKS), T2, "CME-SVC-2025-04-17"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-17 - Good Friday, no events published.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 - T2 - CME-SVC-2025-05-25 - Memorial Day, only a 19:00 CT open for 05-27.
        (2025, 5, 26, Closed, T2, "CME-SVC-2025-05-25"),
        // 2025-06-18 - T2 - CME-SVC-2025-06-18 - Juneteenth eve: the complete day, whose 14:30-16:00 CT queue CME dates to this trade date.
        (2025, 6, 18, ReplacementBlocks(&CLOSURE_EVE_BLOCKS), T2, "CME-SVC-2025-06-18"),
        // 2025-06-19 - T2 - CME-SVC-2025-06-18 - Juneteenth, only a 19:00 CT open for 06-20.
        (2025, 6, 19, Closed, T2, "CME-SVC-2025-06-18"),
        // 2025-07-03 - T2 - CME-SVC-2025-07-03 - Independence Day eve: the complete day, whose 14:30-16:00 CT queue CME dates to this trade date.
        (2025, 7, 3, ReplacementBlocks(&CLOSURE_EVE_BLOCKS), T2, "CME-SVC-2025-07-03"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-03 - Independence Day, no events published.
        (2025, 7, 4, Closed, T2, "CME-SVC-2025-07-03"),
        // 2025-09-01 - T2 - CME-SVC-2025-08-31 - Labor Day, only a 19:00 CT open for 09-02.
        (2025, 9, 1, Closed, T2, "CME-SVC-2025-08-31"),
        // 2025-11-26 - T2 - CME-SVC-2025-11-26-SAT - Thanksgiving eve: the complete day, whose 14:30-16:00 CT queue CME dates to this trade date.
        (2025, 11, 26, ReplacementBlocks(&CLOSURE_EVE_BLOCKS), T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-11-27 - T2 - CME-SVC-2025-11-26-SAT - Thanksgiving, no events published.
        (2025, 11, 27, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-11-28 - T2 - CME-SVC-2025-11-26-SAT - day after Thanksgiving, 08:30-12:05 CT only.
        (
            2025,
            11,
            28,
            late_open_and_early_close(DAY_OPEN, HALF_DAY_CLOSE),
            T2,
            "CME-SVC-2025-11-26-SAT"
        ),
        // 2025-11-29 - T2 - CME-SVC-2025-11-26-SAT - Thanksgiving Saturday, no events published.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-12-24 - T2 - CME-SVC-2025-12-24 - Christmas Eve, 12:05 CT final close.
        (2025, 12, 24, early_close(HALF_DAY_CLOSE), T2, "CME-SVC-2025-12-24"),
        // 2025-12-25 - T2 - CME-SVC-2025-12-24 - Christmas Day, no events published.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-24"),
        // 2025-12-26 - T2 - CME-SVC-2025-12-24 - no prior-evening leg; the day opens with CME's 06:00 CT pre-open, then the ordinary session and post-close queue.
        (2025, 12, 26, ReplacementBlocks(&LATE_OPEN_BLOCKS), T2, "CME-SVC-2025-12-24"),
        // 2025-12-31 - T2 - CME-SVC-2025-12-31 - New Year's Day eve: the complete day, whose 14:30-16:00 CT queue CME dates to this trade date.
        (2025, 12, 31, ReplacementBlocks(&CLOSURE_EVE_BLOCKS), T2, "CME-SVC-2025-12-31"),
        // 2026-01-01 - T2 - CME-SVC-2025-12-31 - New Year's Day, no events published.
        (2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-01-02 - T2 - CME-SVC-2025-12-31 - no prior-evening leg; the day opens with CME's 06:00 CT pre-open, then the ordinary session and post-close queue.
        (2026, 1, 2, ReplacementBlocks(&LATE_OPEN_BLOCKS), T2, "CME-SVC-2025-12-31"),
        // 2026-01-19 - T2 - CME-SVC-2026-01-18 - MLK Day, only a 19:00 CT open for 01-20.
        (2026, 1, 19, Closed, T2, "CME-SVC-2026-01-18"),
        // 2026-02-16 - T2 - CME-SVC-2026-02-15 - Presidents' Day, only a 19:00 CT open for 02-17.
        (2026, 2, 16, Closed, T2, "CME-SVC-2026-02-15"),
        // 2026-04-02 - T2 - CME-SVC-2026-04-01 - Good Friday eve: the complete day, whose 14:30-16:00 CT queue CME dates to this trade date.
        (2026, 4, 2, ReplacementBlocks(&CLOSURE_EVE_BLOCKS), T2, "CME-SVC-2026-04-01"),
        // 2026-04-03 - T2 - CME-SVC-2026-04-01 - Good Friday, no events published.
        (2026, 4, 3, Closed, T2, "CME-SVC-2026-04-01"),
        // 2026-05-25 - T2 - CME-SVC-2026-05-24 - Memorial Day, only a 19:00 CT open for 05-26.
        (2026, 5, 25, Closed, T2, "CME-SVC-2026-05-24"),
        // 2026-06-18 - T2 - CME-SVC-2026-06-18 - Juneteenth eve: the complete day, whose 14:30-16:00 CT queue CME dates to this trade date.
        (2026, 6, 18, ReplacementBlocks(&CLOSURE_EVE_BLOCKS), T2, "CME-SVC-2026-06-18"),
        // 2026-06-19 - T2 - CME-SVC-2026-06-18 - Juneteenth, no events published.
        (2026, 6, 19, Closed, T2, "CME-SVC-2026-06-18"),
        // 2026-07-02 - T2 - CME-SVC-2026-07-02 - Independence Day eve: the complete day, whose 14:30-16:00 CT queue CME dates to this trade date.
        (2026, 7, 2, ReplacementBlocks(&CLOSURE_EVE_BLOCKS), T2, "CME-SVC-2026-07-02"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - Independence Day observed, no events published.
        (2026, 7, 3, Closed, T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 - T2 - CME-SVC-2026-09-06 - Labor Day, only a 19:00 CT open for 09-08.
        (2026, 9, 7, Closed, T2, "CME-SVC-2026-09-06"),
        // 2026-11-25 - T2 - CME-SVC-2026-11-25 - Thanksgiving eve: the complete day, whose 14:30-16:00 CT queue CME dates to this trade date.
        (2026, 11, 25, ReplacementBlocks(&CLOSURE_EVE_BLOCKS), T2, "CME-SVC-2026-11-25"),
        // 2026-11-26 - T2 - CME-SVC-2026-11-25 - Thanksgiving, no events published.
        (2026, 11, 26, Closed, T2, "CME-SVC-2026-11-25"),
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - day after Thanksgiving, 08:30-12:05 CT only.
        (
            2026,
            11,
            27,
            late_open_and_early_close(DAY_OPEN, HALF_DAY_CLOSE),
            T2,
            "CME-SVC-2026-11-25"
        ),
        // 2026-12-24 - T2 - CME-SVC-2026-12-24 - Christmas Eve, 12:05 CT final close.
        (2026, 12, 24, early_close(HALF_DAY_CLOSE), T2, "CME-SVC-2026-12-24"),
        // 2026-12-25 - T2 - CME-SVC-2026-12-24 - Christmas Day, no events published.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24"),
        // 2026-12-31 - T2 - CME-SVC-2026-12-31 - New Year's Day eve: the complete day, whose 14:30-16:00 CT queue CME dates to this trade date.
        (2026, 12, 31, ReplacementBlocks(&CLOSURE_EVE_BLOCKS), T2, "CME-SVC-2026-12-31"),
        // 2027-01-01 - T2 - CME-SVC-2026-12-31 - New Year's Day, no events published.
        (2027, 1, 1, Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-01-18 - T2 - CME-SVC-2027-01-17 - MLK Day, only a 19:00 CT open for 01-19.
        (2027, 1, 18, Closed, T2, "CME-SVC-2027-01-17"),
        // 2027-02-15 - T2 - CME-SVC-2027-02-14 - Presidents' Day, only a 19:00 CT open for 02-16.
        (2027, 2, 15, Closed, T2, "CME-SVC-2027-02-14"),
        // 2027-03-25 - T2 - CME-SVC-2027-03-25 - Good Friday eve: the complete day, whose 14:30-16:00 CT queue CME dates to this trade date.
        (2027, 3, 25, ReplacementBlocks(&CLOSURE_EVE_BLOCKS), T2, "CME-SVC-2027-03-25"),
        // 2027-03-26 - T2 - CME-SVC-2027-03-25 - Good Friday, no events published.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 - T2 - CME-SVC-2027-05-30 - Memorial Day, only a 19:00 CT open for 06-01.
        (2027, 5, 31, Closed, T2, "CME-SVC-2027-05-30"),
        // 2027-06-17 - T2 - CME-SVC-2027-06-17 - Juneteenth eve: the complete day, whose 14:30-16:00 CT queue CME dates to this trade date.
        (2027, 6, 17, ReplacementBlocks(&CLOSURE_EVE_BLOCKS), T2, "CME-SVC-2027-06-17"),
        // 2027-06-18 - T2 - CME-SVC-2027-06-17 - Juneteenth observed, no events published.
        (2027, 6, 18, Closed, T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 - T2 - CME-SVC-2027-07-04 - Independence Day observed, no events published.
        (2027, 7, 5, Closed, T2, "CME-SVC-2027-07-04"),
        // 2027-07-06 - T2 - CME-SVC-2027-07-04 - no prior-evening leg; the day opens with CME's 06:00 CT pre-open, then the ordinary session and post-close queue.
        (2027, 7, 6, ReplacementBlocks(&LATE_OPEN_BLOCKS), T2, "CME-SVC-2027-07-04"),
        // 2027-09-06 - T2 - CME-SVC-2027-09-05 - Labor Day, only a 19:00 CT open for 09-07.
        (2027, 9, 6, Closed, T2, "CME-SVC-2027-09-05"),
        // 2027-11-24 - T2 - CME-SVC-2027-11-24 - Thanksgiving eve: the complete day, whose 14:30-16:00 CT queue CME dates to this trade date.
        (2027, 11, 24, ReplacementBlocks(&CLOSURE_EVE_BLOCKS), T2, "CME-SVC-2027-11-24"),
        // 2027-11-25 - T2 - CME-SVC-2027-11-24 - Thanksgiving, no events published.
        (2027, 11, 25, Closed, T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - day after Thanksgiving, 08:30-12:05 CT only.
        (
            2027,
            11,
            26,
            late_open_and_early_close(DAY_OPEN, HALF_DAY_CLOSE),
            T2,
            "CME-SVC-2027-11-24"
        ),
        // 2027-12-23 - T2 - CME-SVC-2027-12-22 - Christmas Friday eve: the complete day, whose 14:30-16:00 CT queue CME dates to this trade date.
        (2027, 12, 23, ReplacementBlocks(&CLOSURE_EVE_BLOCKS), T2, "CME-SVC-2027-12-22"),
        // 2027-12-24 - T2 - CME-SVC-2027-12-22 - Christmas Friday closure, no events published.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
