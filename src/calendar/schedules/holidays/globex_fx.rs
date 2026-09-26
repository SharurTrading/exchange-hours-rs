// SPDX-License-Identifier: MIT-0

//! CME standard-grid FX futures holiday rows, 2010-2012, 2013-2015, 2016-2018,
//! 2019-2021, 2022-2024 and 2025-2027 (LAW-HOLIDAY-SCOPE).
//!
//! Keyed by the crate's own venue-local trade date in `America/Chicago`
//! (design memo D1). The conversion is **not** the identity for this family:
//! the grid is a single wrapping Sunday-to-Thursday 17:00→16:00 CT matching
//! block, so every trading day opens on the previous local evening and CME's
//! event-date records have to be read against the trade date CME itself prints
//! beside them. An eve record is evidence for the holiday's own row, never a
//! row of its own, unless the eve carries an early close of its *own* trade
//! date — which is exactly what Christmas Eve does.
//!
//! Of the 2022-2027 rows below, the 2022-2024 era mixes T1 and T2 and every
//! 2025-2027 row is T2.
//!
//! **2010-2012.** CME's own holiday-calendar PDFs (`2010-martin-luther-king.pdf`
//! and its siblings) are the T1 source for the era, whose grid is the same
//! wrapping 17:00 -> 16:00 CT block. Three shapes: a **full closure** (New
//! Year's Day, Christmas Day, 2011's Good Friday and the observed days around
//! them); an **early close** at 15:15 CT on a Friday-holiday eve, at 12:00 CT
//! on the Monday and Thursday holidays, at 10:15 CT on the Good Fridays of 2010
//! and 2012, and at 12:15 CT on the year-end half-days; and a **late open** on
//! 2011-12-27, 2012-01-03 and 2012-12-26, where the closure removed the
//! prior-evening leg and CME states `0500 CT - CME Globex open for trade date
//! ...`, so that trade date's first open is 05:00 CT on the trade date itself
//! instead of the ordinary 17:00 CT on the eve — twelve hours later. A stated
//! re-open that lands exactly on the family's ordinary 17:00 CT evening open
//! moves no answer and ships no row, and the evidence file lists those dates.
//! (The operator prints that re-open above the next trade date's name.)
//!
//! **2016-2018.** CME's own published Globex holiday schedules are the T1
//! source for this era too: per-holiday PDFs inside the 2016 annual bundle, a
//! standalone 2016 New Year's PDF, and per-product-group sheets for 2017 and
//! 2018, whose annual bundles carry CME's final revisions. The grid is the same
//! wrapping `17:00 -> 16:00 CT` block as the 2010-2012 era's, so an early close
//! is the printed final close stated on the trade date it ends, a closure
//! removes the trade date with its prior-evening leg, and every stated re-open
//! at the ordinary 17:00 CT evening open ships no row. Thirty-one rows: nine
//! closures and twenty-two early closes — 12:00 CT on the eighteen Monday and
//! Thursday holidays and 12:15 CT on the three Thanksgiving Fridays and
//! 2018-12-24 — with no late open, because every re-open after a closure is
//! that ordinary evening open.
//!
//! **2022-2024.** The grid is the same wrapping `17:00 -> 16:00 CT` block, and
//! the conversions are the same. Fourteen rows: seven closures, four early
//! closes — 12:15 CT on 2022-11-25, 2023-11-24 and 2024-11-29, and 12:45 CT on
//! 2024-12-24 — no late open, and three `Unsourced` rows. The 2022 rows
//! and the 2023 rows CME published a holiday schedule for are **T1**; the
//! three 2023 dates it published nothing for and all of 2024 are **T2**. The
//! three `Unsourced` dates — 2023-01-16, 2023-02-20 and 2023-04-07 — mean the
//! operator published nothing this crate could read, not that no holiday fell
//! on them; an operator document stating each date in session language would
//! close them.
//!
//! The 2025-2027 rows come from CME's trading-hours service — the endpoint
//! `cmegroup.com/trading-hours.html` itself calls — read as bytes and saved,
//! so those rows are **T2** under LAW-PRIMARY-SOURCES. CME publishes no T1
//! per-asset-class rendering for them; that, the eight 2025 windows that
//! survive only in a pre-holiday capture, the three Saturday sessions CME
//! states in this window — each a `ReplacementBlocks` row carrying the
//! following Monday's trade date — the holiday spans whose topology no shipped
//! row states, and the sixteen dates on which CME merges the holiday into the
//! next business day's trade date are recorded in
//! [`docs/evidence/globex_fx.md`](../../../../../docs/evidence/globex_fx.md).
//!
//! Three shapes: `Closed` on a full Globex closure, `EarlyClose` on the
//! half-days CME publishes for the family, and `ReplacementBlocks` on the three
//! trade dates of 2026-06-22, 2026-07-06 and 2027-06-21, whose operator day
//! carries a Saturday session. The 2025-2027 window has **no** late open — CME
//! never reopens this family after a closure there other than at its normal
//! 17:00 CT — and it carries no `Unsourced` row, because every date inside its
//! own coverage is answered by CME's own published schedule.
//!
//! # What does not ship a row
//!
//! On Monday and Thursday holidays CME publishes `16:00 preopen; 17:00 open`
//! for this family instead of the normal `16:00 closed; 16:45 preopen;
//! 17:00 open`. Matching still stops at 16:00 CT and still resumes at 17:00 CT,
//! so **no executable phase moves**; what changes is that the holiday has no
//! final close of its own and the whole span carries the next business day's
//! trade date, and that the queue opens 45 minutes early. No shipped row
//! states either, so both are declared gaps rather than rows — the design
//! memo's §1.6 triage, applied to its own §1.1 worked example. The three
//! Saturday-session trade dates below are the one place this window states a
//! queue time of its own: the operator's Sunday windows print `16:00 preopen`
//! for them where the dated profile in force carries 16:15 CT for the two 2026
//! dates.

use super::fences::{early_close, late_open};
use crate::calendar::exceptions::ExceptionBlock;

/// The complete trading day of the 2026-06-22, 2026-07-06 and 2027-06-21 trade
/// dates, which CME states a Saturday session on.
///
/// CME published all three of the trading day's phases for each of these dates,
/// so the row states the day rather than the Saturday alone. Stating only the
/// Saturday would delete the Sunday-evening session that belongs to the same
/// trade date, because a replacement row replaces the **complete** trade date.
///
/// - offset `-2`, Saturday 05:00-17:00 CT: the session itself.
/// - offset `-1`, Sunday 16:00-17:00 CT: the Pre-Open queue, which no trade
///   matches. The operator's Sunday window prints `16:00 preopen` for all three
///   dates, so the row states that value and not the 16:15 CT the dated profile
///   in force carries for the two 2026 dates.
/// - offset `-1`, Sunday 17:00 CT to Monday 16:00 CT: the matching session,
///   wrapping one local midnight.
///
/// Evidence: `docs/evidence/globex_fx.md`.
pub(crate) static SATURDAY_SESSION_BLOCKS: [ExceptionBlock; 3] = [
    ExceptionBlock::extended(-2, 5 * 3_600, 17 * 3_600),
    ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 16 * 3_600),
];
use super::{
    EvidenceTier::{T1, T2},
    HolidayKind::{Closed, ReplacementBlocks, Unsourced},
    HolidayTable, holidays,
};

/// The family's built-in holiday rows and the windows they were audited over.
///
///  Six audited eras: 2010-2012 at T1, 2013-2015 at T1, 2016-2018 at T1,
/// 2019-2021 at T1, 2022-2024 at T1/T2 and 2025-2027 at T2.
///  Every audited interval is contiguous, and nothing before 2010-01-01
/// has a table at all: that span lies outside every window, so
/// `holiday_on` has no answer there rather than reporting a normal date.
///
/// Coverage ends at
/// 2027-12-31, the end of the operator's published future, and CME's 2028-01-01
/// record sits outside every window and ships no row. Inside a window a date
/// with no row is audited normal, except where an `Unsourced` row marks the
/// operator's silence instead.
// Evidence: docs/evidence/globex_fx.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2012, 12, 31), (2013, 1, 1) ..= (2015, 12, 31), (2016, 1, 1) ..= (2018, 12, 31), (2019, 1, 1) ..= (2021, 12, 31), (2022, 1, 1) ..= (2024, 12, 31), (2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // --- 2010-2012, CME Group holiday calendars, tier T1 ---
        // 2010-01-15 - T1 - 2010-martin-luther-king.pdf - early close.
        // 2010-01-01 - T1 - 2010-new-years.pdf - closed: new year's day 2010.
        (2010, 1, 1, Closed, T1, "2010-new-years.pdf @2010-02-15T05:16:52Z"),
        (2010, 1, 15, early_close(15 * 3_600 + 15 * 60), T1, "2010-martin-luther-king.pdf @2010-03-31T06:42:26Z"),
        // 2010-01-18 - T1 - 2010-martin-luther-king.pdf - early close.
        (2010, 1, 18, early_close(12 * 3_600), T1, "2010-martin-luther-king.pdf @2010-03-31T06:42:26Z"),
        // 2010-02-12 - T1 - 2010-presidents-day.pdf - early close.
        (2010, 2, 12, early_close(15 * 3_600 + 15 * 60), T1, "2010-presidents-day.pdf @2010-02-15T06:46:41Z"),
        // 2010-02-15 - T1 - 2010-presidents-day.pdf - early close.
        (2010, 2, 15, early_close(12 * 3_600), T1, "2010-presidents-day.pdf @2010-02-15T06:46:41Z"),
        // 2010-04-02 - T1 - 2010-good-friday.pdf - early close.
        (2010, 4, 2, early_close(10 * 3_600 + 15 * 60), T1, "2010-good-friday.pdf @2010-06-01T11:19:16Z"),
        // 2010-05-28 - T1 - 2010-memorial-day.pdf - early close.
        (2010, 5, 28, early_close(15 * 3_600 + 15 * 60), T1, "2010-memorial-day.pdf @2010-06-01T09:42:25Z"),
        // 2010-05-31 - T1 - 2010-memorial-day.pdf - early close.
        (2010, 5, 31, early_close(12 * 3_600), T1, "2010-memorial-day.pdf @2010-06-01T09:42:25Z"),
        // 2010-07-02 - T1 - 2010-4th-of-july.pdf - early close.
        (2010, 7, 2, early_close(15 * 3_600 + 15 * 60), T1, "2010-4th-of-july.pdf @2010-06-02T00:56:37Z"),
        // 2010-07-05 - T1 - 2010-4th-of-july.pdf - early close.
        (2010, 7, 5, early_close(12 * 3_600), T1, "2010-4th-of-july.pdf @2010-06-02T00:56:37Z"),
        // 2010-09-03 - T1 - 2010-labor-day.pdf - early close.
        (2010, 9, 3, early_close(15 * 3_600 + 15 * 60), T1, "2010-labor-day.pdf @2010-06-02T00:56:41Z"),
        // 2010-09-06 - T1 - 2010-labor-day.pdf - early close.
        (2010, 9, 6, early_close(12 * 3_600), T1, "2010-labor-day.pdf @2010-06-02T00:56:41Z"),
        // 2010-10-08 - T1 - 2010-columbus-day.pdf - early close.
        (2010, 10, 8, early_close(15 * 3_600 + 15 * 60), T1, "2010-columbus-day.pdf @2010-08-21T13:31:22Z"),
        // 2010-11-25 - T1 - 2010-thanksgiving.pdf - early close.
        (2010, 11, 25, early_close(12 * 3_600), T1, "2010-thanksgiving.pdf @2010-11-22T09:40:12Z"),
        // 2010-11-26 - T1 - 2010-thanksgiving.pdf - early close.
        (2010, 11, 26, early_close(12 * 3_600 + 15 * 60), T1, "2010-thanksgiving.pdf @2010-11-22T09:40:12Z"),
        // 2010-12-31 - T1 - 2011-new-years.pdf - early close.
        // 2010-12-24 - T1 - 2010-christmas.pdf - closed: christmas day 2010 observed.
        (2010, 12, 24, Closed, T1, "2010-christmas.pdf @2010-12-14T06:12:38Z"),
        (2010, 12, 31, early_close(12 * 3_600 + 15 * 60), T1, "2011-new-years.pdf @2011-11-01T14:39:45Z"),
        // 2011-01-14 - T1 - 2011-martin-luther-king.pdf - early close.
        (2011, 1, 14, early_close(15 * 3_600 + 15 * 60), T1, "2011-martin-luther-king.pdf @2011-10-28T02:34:29Z"),
        // 2011-01-17 - T1 - 2011-martin-luther-king.pdf - early close.
        (2011, 1, 17, early_close(12 * 3_600), T1, "2011-martin-luther-king.pdf @2011-10-28T02:34:29Z"),
        // 2011-02-18 - T1 - 2011-presidents-day.pdf - early close.
        (2011, 2, 18, early_close(15 * 3_600 + 15 * 60), T1, "2011-presidents-day.pdf @2011-10-28T02:35:16Z"),
        // 2011-02-21 - T1 - 2011-presidents-day.pdf - early close.
        (2011, 2, 21, early_close(12 * 3_600), T1, "2011-presidents-day.pdf @2011-10-28T02:35:16Z"),
        // 2011-05-27 - T1 - 2011-memorial-day.pdf - early close.
        // 2011-04-22 - T1 - 2011-good-friday.pdf - closed: good friday 2011.
        (2011, 4, 22, Closed, T1, "2011-good-friday.pdf @2011-10-28T02:37:07Z"),
        (2011, 5, 27, early_close(15 * 3_600 + 15 * 60), T1, "2011-memorial-day.pdf @2013-09-30T10:56:52Z"),
        // 2011-05-30 - T1 - 2011-memorial-day.pdf - early close.
        (2011, 5, 30, early_close(12 * 3_600), T1, "2011-memorial-day.pdf @2013-09-30T10:56:52Z"),
        // 2011-07-01 - T1 - 2011-4th-of-july.pdf - early close.
        (2011, 7, 1, early_close(15 * 3_600 + 15 * 60), T1, "2011-4th-of-july.pdf @2011-11-01T14:40:54Z"),
        // 2011-07-04 - T1 - 2011-4th-of-july.pdf - early close.
        (2011, 7, 4, early_close(12 * 3_600), T1, "2011-4th-of-july.pdf @2011-11-01T14:40:54Z"),
        // 2011-09-02 - T1 - 2011-labor-day.pdf - early close.
        (2011, 9, 2, early_close(15 * 3_600 + 15 * 60), T1, "2011-labor-day.pdf @2011-11-01T14:43:45Z"),
        // 2011-09-05 - T1 - 2011-labor-day.pdf - early close.
        (2011, 9, 5, early_close(12 * 3_600), T1, "2011-labor-day.pdf @2011-11-01T14:43:45Z"),
        // 2011-10-07 - T1 - 2011-columbus-day.pdf - early close.
        (2011, 10, 7, early_close(15 * 3_600 + 15 * 60), T1, "2011-columbus-day.pdf @2011-11-01T14:39:16Z"),
        // 2011-11-24 - T1 - 2011-thanksgiving.pdf - early close.
        (2011, 11, 24, early_close(12 * 3_600), T1, "2011-thanksgiving.pdf @2011-11-24T18:52:46Z"),
        // 2011-11-25 - T1 - 2011-thanksgiving.pdf - early close.
        (2011, 11, 25, early_close(12 * 3_600 + 15 * 60), T1, "2011-thanksgiving.pdf @2011-11-24T18:52:46Z"),
        // 2011-12-27 - T1 - 2011-christmas.pdf - late open.
        // 2011-12-26 - T1 - 2011-christmas.pdf - closed: christmas day 2011 observed.
        (2011, 12, 26, Closed, T1, "2011-christmas.pdf @2012-01-25T02:05:48Z"),
        (2011, 12, 27, late_open(5 * 3_600), T1, "2011-christmas.pdf @2012-01-25T02:05:48Z"),
        // 2012-01-03 - T1 - 2012-new-years.pdf - late open.
        // 2012-01-02 - T1 - 2012-new-years.pdf - closed: new year's day 2012 observed.
        (2012, 1, 2, Closed, T1, "2012-new-years.pdf @2012-01-25T02:54:30Z"),
        (2012, 1, 3, late_open(5 * 3_600), T1, "2012-new-years.pdf @2012-01-25T02:54:30Z"),
        // 2012-01-13 - T1 - 2012-martin-luther-king.pdf - early close.
        (2012, 1, 13, early_close(15 * 3_600 + 15 * 60), T1, "2012-martin-luther-king.pdf @2012-05-05T16:15:26Z"),
        // 2012-01-16 - T1 - 2012-martin-luther-king.pdf - early close.
        (2012, 1, 16, early_close(12 * 3_600), T1, "2012-martin-luther-king.pdf @2012-05-05T16:15:26Z"),
        // 2012-02-17 - T1 - 2012-presidents-day.pdf - early close.
        (2012, 2, 17, early_close(15 * 3_600 + 15 * 60), T1, "2012-presidents-day.pdf @2012-05-05T16:15:39Z"),
        // 2012-02-20 - T1 - 2012-presidents-day.pdf - early close.
        (2012, 2, 20, early_close(12 * 3_600), T1, "2012-presidents-day.pdf @2012-05-05T16:15:39Z"),
        // 2012-04-06 - T1 - 2012-good-friday.pdf - early close.
        (2012, 4, 6, early_close(10 * 3_600 + 15 * 60), T1, "2012-good-friday.pdf @2012-04-17T00:42:47Z"),
        // 2012-05-25 - T1 - 2012-memorial-day.pdf - early close.
        (2012, 5, 25, early_close(15 * 3_600 + 15 * 60), T1, "2012-memorial-day.pdf @2012-09-15T00:37:14Z"),
        // 2012-05-28 - T1 - 2012-memorial-day.pdf - early close.
        (2012, 5, 28, early_close(12 * 3_600), T1, "2012-memorial-day.pdf @2012-09-15T00:37:14Z"),
        // 2012-07-04 - T1 - 2012-4th-of-july.pdf - early close.
        (2012, 7, 4, early_close(12 * 3_600), T1, "2012-4th-of-july.pdf @2012-09-15T00:39:23Z"),
        // 2012-08-31 - T1 - 2012-labor-day.pdf - early close.
        (2012, 8, 31, early_close(15 * 3_600 + 15 * 60), T1, "2012-labor-day.pdf @2012-09-15T00:34:37Z"),
        // 2012-09-03 - T1 - 2012-labor-day.pdf - early close.
        (2012, 9, 3, early_close(12 * 3_600), T1, "2012-labor-day.pdf @2012-09-15T00:34:37Z"),
        // 2012-10-05 - T1 - 2012-columbus-day.pdf - early close.
        (2012, 10, 5, early_close(15 * 3_600 + 15 * 60), T1, "2012-columbus-day.pdf @2012-09-15T00:15:14Z"),
        // 2012-11-22 - T1 - 2012-thanksgiving.pdf - early close.
        (2012, 11, 22, early_close(12 * 3_600), T1, "2012-thanksgiving.pdf @2013-01-27T22:39:01Z"),
        // 2012-11-23 - T1 - 2012-thanksgiving.pdf - early close.
        (2012, 11, 23, early_close(12 * 3_600 + 15 * 60), T1, "2012-thanksgiving.pdf @2013-01-27T22:39:01Z"),
        // 2012-12-24 - T1 - 2012-christmas.pdf - early close.
        (2012, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        // 2012-12-26 - T1 - 2012-christmas.pdf - late open.
        // 2012-12-25 - T1 - 2012-christmas.pdf - closed: christmas day 2012.
        (2012, 12, 25, Closed, T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        (2012, 12, 26, late_open(5 * 3_600), T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        // 2013-01-01 - T1 - 2013-new-years.pdf @2013-04-14T19:41:46Z - CME prints no session running through this date.
        (2013, 1, 1, Closed, T1, "2013-new-years.pdf @2013-04-14T19:41:46Z"),
        // 2013-01-02 - T1 - 2013-new-years.pdf @2013-04-14T19:41:46Z - the trade date's first open is 5:00 CT: the evening leg that would have opened earlier did not run.
        (2013, 1, 2, late_open(5 * 3_600), T1, "2013-new-years.pdf @2013-04-14T19:41:46Z"),
        // 2013-01-18 - T1 - 2013-martin-luther-king.pdf @2012-11-19T00:16:09Z - the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT.
        (2013, 1, 18, early_close(15 * 3_600 + 15 * 60), T1, "2013-martin-luther-king.pdf @2012-11-19T00:16:09Z"),
        // 2013-01-21 - T1 - 2013-martin-luther-king.pdf @2012-11-19T00:16:09Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT.
        (2013, 1, 21, early_close(12 * 3_600), T1, "2013-martin-luther-king.pdf @2012-11-19T00:16:09Z"),
        // 2013-02-15 - T1 - 2013-presidents-day.pdf @2013-03-09T11:53:37Z - the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT.
        (2013, 2, 15, early_close(15 * 3_600 + 15 * 60), T1, "2013-presidents-day.pdf @2013-03-09T11:53:37Z"),
        // 2013-02-18 - T1 - 2013-presidents-day.pdf @2013-03-09T11:53:37Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT.
        (2013, 2, 18, early_close(12 * 3_600), T1, "2013-presidents-day.pdf @2013-03-09T11:53:37Z"),
        // 2013-03-29 - T1 - 2013-good-friday.pdf @2013-06-23T19:59:25Z - CME prints no session running through this date.
        (2013, 3, 29, Closed, T1, "2013-good-friday.pdf @2013-06-23T19:59:25Z"),
        // 2013-05-24 - T1 - 2013-memorial-day.pdf @2013-06-23T20:36:04Z - the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT.
        (2013, 5, 24, early_close(15 * 3_600 + 15 * 60), T1, "2013-memorial-day.pdf @2013-06-23T20:36:04Z"),
        // 2013-05-27 - T1 - 2013-memorial-day.pdf @2013-06-23T20:36:04Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT.
        (2013, 5, 27, early_close(12 * 3_600), T1, "2013-memorial-day.pdf @2013-06-23T20:36:04Z"),
        // 2013-07-04 - T1 - 2013-4th-of-july.pdf @2013-06-23T20:58:25Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT.
        (2013, 7, 4, early_close(12 * 3_600), T1, "2013-4th-of-july.pdf @2013-06-23T20:58:25Z"),
        // 2013-08-30 - T1 - 2013-labor-day.pdf @2013-09-02T17:08:41Z - the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT.
        (2013, 8, 30, early_close(15 * 3_600 + 15 * 60), T1, "2013-labor-day.pdf @2013-09-02T17:08:41Z"),
        // 2013-09-02 - T1 - 2013-labor-day.pdf @2013-09-02T17:08:41Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT.
        (2013, 9, 2, early_close(12 * 3_600), T1, "2013-labor-day.pdf @2013-09-02T17:08:41Z"),
        // 2013-11-28 - T1 - 2013-thanksgiving.pdf @2014-02-14T06:28:36Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT.
        (2013, 11, 28, early_close(12 * 3_600), T1, "2013-thanksgiving.pdf @2014-02-14T06:28:36Z"),
        // 2013-11-29 - T1 - 2013-thanksgiving.pdf @2014-02-14T06:28:36Z - the printed final close 12:15 CT is earlier than the family's ordinary 16:00 CT.
        (2013, 11, 29, early_close(12 * 3_600 + 15 * 60), T1, "2013-thanksgiving.pdf @2014-02-14T06:28:36Z"),
        // 2013-12-24 - T1 - 2013-christmas.pdf @2014-04-12T06:24:28Z - the printed final close 12:15 CT is earlier than the family's ordinary 16:00 CT.
        (2013, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "2013-christmas.pdf @2014-04-12T06:24:28Z"),
        // 2013-12-25 - T1 - 2013-christmas.pdf @2014-04-12T06:24:28Z - CME prints no session running through this date.
        (2013, 12, 25, Closed, T1, "2013-christmas.pdf @2014-04-12T06:24:28Z"),
        // 2013-12-26 - T1 - 2013-christmas.pdf @2014-04-12T06:24:28Z - the trade date's first open is 5:00 CT: the evening leg that would have opened earlier did not run.
        (2013, 12, 26, late_open(5 * 3_600), T1, "2013-christmas.pdf @2014-04-12T06:24:28Z"),
        // 2014-01-01 - T1 - 2014-new-years.pdf @2013-10-07T20:58:00Z - CME prints no session running through this date.
        (2014, 1, 1, Closed, T1, "2014-new-years.pdf @2013-10-07T20:58:00Z"),
        // 2014-01-02 - T1 - 2014-new-years.pdf @2013-10-07T20:58:00Z - the trade date's first open is 5:00 CT: the evening leg that would have opened earlier did not run.
        (2014, 1, 2, late_open(5 * 3_600), T1, "2014-new-years.pdf @2013-10-07T20:58:00Z"),
        // 2014-01-17 - T1 - 2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z - the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT.
        (2014, 1, 17, early_close(15 * 3_600 + 15 * 60), T1, "2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z"),
        // 2014-01-20 - T1 - 2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT.
        (2014, 1, 20, early_close(12 * 3_600), T1, "2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z"),
        // 2014-02-14 - T1 - 2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z - the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT.
        (2014, 2, 14, early_close(15 * 3_600 + 15 * 60), T1, "2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z"),
        // 2014-02-17 - T1 - 2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT.
        (2014, 2, 17, early_close(12 * 3_600), T1, "2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z"),
        // 2014-04-18 - T1 - 2014-good-friday-holiday-schedule.pdf @2014-03-26T15:27:35Z - CME prints no session running through this date.
        (2014, 4, 18, Closed, T1, "2014-good-friday-holiday-schedule.pdf @2014-03-26T15:27:35Z"),
        // 2014-05-23 - T1 - 2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z - the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT.
        (2014, 5, 23, early_close(15 * 3_600 + 15 * 60), T1, "2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z"),
        // 2014-05-26 - T1 - 2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT.
        (2014, 5, 26, early_close(12 * 3_600), T1, "2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z"),
        // 2014-07-04 - T1 - 2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT.
        (2014, 7, 4, early_close(12 * 3_600), T1, "2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z"),
        // 2014-08-29 - T1 - 2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z - the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT.
        (2014, 8, 29, early_close(15 * 3_600 + 15 * 60), T1, "2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z"),
        // 2014-09-01 - T1 - 2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT.
        (2014, 9, 1, early_close(12 * 3_600), T1, "2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z"),
        // 2014-11-27 - T1 - 2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT.
        (2014, 11, 27, early_close(12 * 3_600), T1, "2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z"),
        // 2014-11-28 - T1 - 2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z - the printed final close 12:15 CT is earlier than the family's ordinary 16:00 CT.
        (2014, 11, 28, early_close(12 * 3_600 + 15 * 60), T1, "2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z"),
        // 2014-12-24 - T1 - 2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z - the printed final close 12:15 CT is earlier than the family's ordinary 16:00 CT.
        (2014, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z"),
        // 2014-12-25 - T1 - 2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z - CME prints no session running through this date.
        (2014, 12, 25, Closed, T1, "2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z"),
        // 2015-01-01 - T1 - 2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z - CME prints no session running through this date.
        (2015, 1, 1, Closed, T1, "2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z"),
        // 2015-01-16 - T1 - 2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z - the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT.
        (2015, 1, 16, early_close(15 * 3_600 + 15 * 60), T1, "2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z"),
        // 2015-01-19 - T1 - 2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT.
        (2015, 1, 19, early_close(12 * 3_600), T1, "2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z"),
        // 2015-02-13 - T1 - 2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z - the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT.
        (2015, 2, 13, early_close(15 * 3_600 + 15 * 60), T1, "2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z"),
        // 2015-02-16 - T1 - 2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT.
        (2015, 2, 16, early_close(12 * 3_600), T1, "2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z"),
        // 2015-04-03 - T1 - 2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z - the printed final close 10:15 CT is earlier than the family's ordinary 16:00 CT.
        (2015, 4, 3, early_close(10 * 3_600 + 15 * 60), T1, "2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z"),
        // 2015-05-22 - T1 - 2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z - the printed final close 15:15 CT is earlier than the family's ordinary 16:00 CT.
        (2015, 5, 22, early_close(15 * 3_600 + 15 * 60), T1, "2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z"),
        // 2015-05-25 - T1 - 2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT.
        (2015, 5, 25, early_close(12 * 3_600), T1, "2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z"),
        // 2015-07-03 - T1 - 2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT.
        (2015, 7, 3, early_close(12 * 3_600), T1, "2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z"),
        // 2015-09-07 - T1 - 2015-labor-day-holiday-schedule.pdf @2015-08-24T02:30:39Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT.
        (2015, 9, 7, early_close(12 * 3_600), T1, "2015-labor-day-holiday-schedule.pdf @2015-08-24T02:30:39Z"),
        // 2015-11-26 - T1 - 2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:00 CT.
        (2015, 11, 26, early_close(12 * 3_600), T1, "2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z"),
        // 2015-11-27 - T1 - 2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z - the printed final close 12:15 CT is earlier than the family's ordinary 16:00 CT.
        (2015, 11, 27, early_close(12 * 3_600 + 15 * 60), T1, "2015-thanksgiving-holiday-schedule.pdf @2016-02-05T16:25:19Z"),
        // 2015-12-24 - T1 - 2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z - the printed final close 12:15 CT is earlier than the family's ordinary 16:00 CT.
        (2015, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z"),
        // 2015-12-25 - T1 - 2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z - CME prints no session running through this date.
        (2015, 12, 25, Closed, T1, "2015-christmas-holiday-schedule.pdf @2015-11-23T06:15:20Z"),
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
        // 2019-01-01 - T1 - 2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z - CME prints no session running through this date.
        (2019, 1, 1, Closed, T1, "2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z"),
        // 2019-01-21 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-martin-luther-king-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - the printed final close 12:00 CT is earlier than the ordinary 16:00 CT close.
        (2019, 1, 21, early_close(12 * 3_600), T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-martin-luther-king-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-02-18 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-presidents-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - the printed final close 12:00 CT is earlier than the ordinary 16:00 CT close.
        (2019, 2, 18, early_close(12 * 3_600), T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-presidents-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-04-19 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-good-friday-holiday-compact.xls @2021-01-26T09:48:37Z - CME prints no session running through this date.
        (2019, 4, 19, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-good-friday-holiday-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-05-27 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-memorial-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - the printed final close 12:00 CT is earlier than the ordinary 16:00 CT close.
        (2019, 5, 27, early_close(12 * 3_600), T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-memorial-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-06-19 - T1 - 2019-holiday-calendars.zip @2021-01-26T09:48:37Z - .
        (2019, 6, 19, Unsourced, T1, "2019-holiday-calendars.zip @2021-01-26T09:48:37Z"),
        // 2019-07-04 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - the printed final close 12:00 CT is earlier than the ordinary 16:00 CT close.
        (2019, 7, 4, early_close(12 * 3_600), T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-09-02 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-labor-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - the printed final close 12:00 CT is earlier than the ordinary 16:00 CT close.
        (2019, 9, 2, early_close(12 * 3_600), T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-labor-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-11-28 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - the printed final close 12:00 CT is earlier than the ordinary 16:00 CT close.
        (2019, 11, 28, early_close(12 * 3_600), T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-11-29 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - the printed final close 12:15 CT is earlier than the ordinary 16:00 CT close.
        (2019, 11, 29, early_close(12 * 3_600 + 15 * 60), T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-12-24 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - the printed final close 12:15 CT is earlier than the ordinary 16:00 CT close.
        (2019, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2019-12-25 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - CME prints no session running through this date.
        (2019, 12, 25, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2020-01-01 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - CME prints no session running through this date.
        (2020, 1, 1, Closed, T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
        // 2020-01-20 - T1 - 2020-holiday-calendars.zip#2020-martin-luther-king-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - the printed final close 12:00 CT is earlier than the ordinary 16:00 CT close.
        (2020, 1, 20, early_close(12 * 3_600), T1, "2020-holiday-calendars.zip#2020-martin-luther-king-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-02-17 - T1 - 2020-holiday-calendars.zip#2020-presidents-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - the printed final close 12:00 CT is earlier than the ordinary 16:00 CT close.
        (2020, 2, 17, early_close(12 * 3_600), T1, "2020-holiday-calendars.zip#2020-presidents-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-04-10 - T1 - 2020-holiday-calendars.zip#2020-good-friday-holiday-compact.xls @2026-07-30T11:18:34Z - CME prints no session running through this date.
        (2020, 4, 10, Closed, T1, "2020-holiday-calendars.zip#2020-good-friday-holiday-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-05-25 - T1 - 2020-holiday-calendars.zip#2020-memorial-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - the printed final close 12:00 CT is earlier than the ordinary 16:00 CT close.
        (2020, 5, 25, early_close(12 * 3_600), T1, "2020-holiday-calendars.zip#2020-memorial-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-06-19 - T1 - 2020-holiday-calendars.zip @2026-07-30T11:18:34Z - .
        (2020, 6, 19, Unsourced, T1, "2020-holiday-calendars.zip @2026-07-30T11:18:34Z"),
        // 2020-07-03 - T1 - 2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - the printed final close 12:00 CT is earlier than the ordinary 16:00 CT close.
        (2020, 7, 3, early_close(12 * 3_600), T1, "2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-09-07 - T1 - 2020-holiday-calendars.zip#2020-labor-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - the printed final close 12:00 CT is earlier than the ordinary 16:00 CT close.
        (2020, 9, 7, early_close(12 * 3_600), T1, "2020-holiday-calendars.zip#2020-labor-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-11-26 - T1 - 2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - the printed final close 12:00 CT is earlier than the ordinary 16:00 CT close.
        (2020, 11, 26, early_close(12 * 3_600), T1, "2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-11-27 - T1 - 2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - the printed final close 12:15 CT is earlier than the ordinary 16:00 CT close.
        (2020, 11, 27, early_close(12 * 3_600 + 15 * 60), T1, "2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-12-24 - T1 - 2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - the printed final close 12:15 CT is earlier than the ordinary 16:00 CT close.
        (2020, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2020-12-25 - T1 - 2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - CME prints no session running through this date.
        (2020, 12, 25, Closed, T1, "2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2021-01-01 - T1 - 2020-holiday-calendars.zip#2021-new-years-holiday-schedule-compact.xls @2026-07-30T11:18:34Z - CME prints no session running through this date.
        (2021, 1, 1, Closed, T1, "2020-holiday-calendars.zip#2021-new-years-holiday-schedule-compact.xls @2026-07-30T11:18:34Z"),
        // 2021-01-18 - T1 - 2021-holiday-calendars.zip#2021-mlk-day-schedule-compact.xls @2026-08-30T10:03:27Z - the printed final close 12:00 CT is earlier than the ordinary 16:00 CT close.
        (2021, 1, 18, early_close(12 * 3_600), T1, "2021-holiday-calendars.zip#2021-mlk-day-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-02-15 - T1 - 2021-holiday-calendars.zip#2021-presidents-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - the printed final close 12:00 CT is earlier than the ordinary 16:00 CT close.
        (2021, 2, 15, early_close(12 * 3_600), T1, "2021-holiday-calendars.zip#2021-presidents-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-04-02 - T1 - 2021-holiday-calendars.zip#2021-good-friday-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - the printed final close 10:15 CT is earlier than the ordinary 16:00 CT close.
        (2021, 4, 2, early_close(10 * 3_600 + 15 * 60), T1, "2021-holiday-calendars.zip#2021-good-friday-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-05-31 - T1 - 2021-holiday-calendars.zip#2021-memorial-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - the printed final close 12:00 CT is earlier than the ordinary 16:00 CT close.
        (2021, 5, 31, early_close(12 * 3_600), T1, "2021-holiday-calendars.zip#2021-memorial-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-06-19 - T1 - 2021-holiday-calendars.zip @2026-08-30T10:03:27Z - .
        (2021, 6, 19, Unsourced, T1, "2021-holiday-calendars.zip @2026-08-30T10:03:27Z"),
        // 2021-07-05 - T1 - 2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - the printed final close 12:00 CT is earlier than the ordinary 16:00 CT close.
        (2021, 7, 5, early_close(12 * 3_600), T1, "2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-09-06 - T1 - 2021-holiday-calendars.zip#2021-labor-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - the printed final close 12:00 CT is earlier than the ordinary 16:00 CT close.
        (2021, 9, 6, early_close(12 * 3_600), T1, "2021-holiday-calendars.zip#2021-labor-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-11-25 - T1 - 2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - the printed final close 12:00 CT is earlier than the ordinary 16:00 CT close.
        (2021, 11, 25, early_close(12 * 3_600), T1, "2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-11-26 - T1 - 2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - the printed final close 12:15 CT is earlier than the ordinary 16:00 CT close.
        (2021, 11, 26, early_close(12 * 3_600 + 15 * 60), T1, "2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-12-24 - T1 - 2021-holiday-calendars.zip#2021-christmas-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - CME prints no session running through this date.
        (2021, 12, 24, Closed, T1, "2021-holiday-calendars.zip#2021-christmas-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2022-04-15 - T1 - 2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z - closed: no trade date.
        (2022, 4, 15, Closed, T1, "2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z"),
        // 2022-11-25 - T1 - 2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z - early close 12:15 CT.
        (2022, 11, 25, early_close(12 * 3_600 + 15 * 60), T1, "2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z"),
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
        // 2023-11-24 - T1 - thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z - early close 12:15 CT.
        (2023, 11, 24, early_close(12 * 3_600 + 15 * 60), T1, "thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z"),
        // 2023-12-25 - T1 - christmas-day-2023.pdf @2026-07-19T09:52:48Z - closed: no trade date.
        (2023, 12, 25, Closed, T1, "christmas-day-2023.pdf @2026-07-19T09:52:48Z"),
        // 2024-01-01 - T1 - new-years-day-2024.pdf @2026-08-11T16:57:16Z - closed: no trade date.
        (2024, 1, 1, Closed, T1, "new-years-day-2024.pdf @2026-08-11T16:57:16Z"),
        // 2024-03-29 - T2 - CME-SVC-2024-03-28 - closed: no trade date.
        (2024, 3, 29, Closed, T2, "CME-SVC-2024-03-28"),
        // 2024-11-29 - T2 - CME-SVC-2024-11-27 - early close 12:15 CT.
        (2024, 11, 29, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-2024-11-27"),
        // 2024-12-24 - T2 - CME-SVC-2024-12-24 - early close 12:45 CT.
        (2024, 12, 24, early_close(12 * 3_600 + 45 * 60), T2, "CME-SVC-2024-12-24"),
        // 2024-12-25 - T2 - CME-SVC-2024-12-24 - closed: no trade date.
        (2024, 12, 25, Closed, T2, "CME-SVC-2024-12-24"),
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-17 - Good Friday, no events published.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-03 - Independence Day, 12:00 CT close.
        (2025, 7, 4, early_close(12 * 3_600), T2, "CME-SVC-2025-07-03"),
        // 2025-11-28 - T2 - CME-SVC-2025-11-26 - day after Thanksgiving, 13:45 CT close.
        (
            2025,
            11,
            28,
            early_close(13 * 3_600 + 45 * 60),
            T2,
            "CME-SVC-2025-11-26"
        ),
        // 2025-11-29 - T2 - CME-SVC-2025-11-26-SAT - Thanksgiving Saturday, no events.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-12-24 - T2 - CME-SVC-2025-12-24 - Christmas Eve, 12:45 CT close.
        (
            2025,
            12,
            24,
            early_close(12 * 3_600 + 45 * 60),
            T2,
            "CME-SVC-2025-12-24"
        ),
        // 2025-12-25 - T2 - CME-SVC-2025-12-24 - Christmas Day; no day session,
        // only the 16:00 CT queue and 17:00 CT open for trade date 2025-12-26.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-24"),
        // 2026-01-01 - T2 - CME-SVC-2025-12-31 - New Year's Day; no day session,
        // only the 16:00 CT queue and 17:00 CT open for trade date 2026-01-02.
        (2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-04-03 - T2 - CME-SVC-2026-04-01 - Good Friday, 10:15 CT close; CME's
        // own page names FX as one of the four groups that traded that morning.
        (
            2026,
            4,
            3,
            early_close(10 * 3_600 + 15 * 60),
            T2,
            "CME-SVC-2026-04-01"
        ),
        // 2026-06-19 - T2 - CME-SVC-2026-06-18 - Juneteenth, 12:00 CT close.
        (2026, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-2026-06-18"),
        (2026, 6, 22, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - Independence Day observed, 12:00 CT close.
        (2026, 7, 3, early_close(12 * 3_600), T2, "CME-SVC-2026-07-03"),
        (2026, 7, 6, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2026-07-03"),
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - day after Thanksgiving, 13:45 CT close.
        (
            2026,
            11,
            27,
            early_close(13 * 3_600 + 45 * 60),
            T2,
            "CME-SVC-2026-11-25"
        ),
        // 2026-12-24 - T2 - CME-SVC-2026-12-22 - Christmas Eve, 12:45 CT close.
        (
            2026,
            12,
            24,
            early_close(12 * 3_600 + 45 * 60),
            T2,
            "CME-SVC-2026-12-22"
        ),
        // 2026-12-25 - T2 - CME-SVC-2026-12-24 - Christmas Day, no events published.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 - T2 - CME-SVC-2026-12-31 - New Year's Day, no events published.
        (2027, 1, 1, Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-03-26 - T2 - CME-SVC-2027-03-25 - Good Friday, no events published.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-06-18 - T2 - CME-SVC-2027-06-17 - Juneteenth observed, 12:00 CT close.
        (2027, 6, 18, early_close(12 * 3_600), T2, "CME-SVC-2027-06-17"),
        (2027, 6, 21, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2027-06-17"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - day after Thanksgiving, 13:45 CT close.
        (
            2027,
            11,
            26,
            early_close(13 * 3_600 + 45 * 60),
            T2,
            "CME-SVC-2027-11-24"
        ),
        // 2027-12-24 - T2 - CME-SVC-2027-12-22 - Christmas Friday closure, no events.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
