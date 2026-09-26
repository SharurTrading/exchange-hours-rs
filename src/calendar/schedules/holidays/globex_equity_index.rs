// SPDX-License-Identifier: MIT-0

//! `globex_equity_index` holiday rows, venue-local trade dates
//! 2010-01-01 .. 2012-12-31, 2013-01-01 .. 2015-12-31,
//! 2016-01-01 .. 2018-12-31, 2019-01-01 .. 2021-12-31,
//! 2022-01-01 .. 2024-12-31 and 2025-01-01 .. 2027-12-31
//! (LAW-HOLIDAY-SCOPE).
//!
//! **2010-2012.** CME published one holiday-calendar PDF per holiday
//! (`2010-martin-luther-king.pdf` and its siblings), retrieved from the
//! Internet Archive at **T1**. The grid of the era is the family's ordinary
//! 08:30-15:15 CT day session inside a wrapped 17:00 CT evening leg, whose
//! final close is 15:15 CT. Three shapes appear, all keyed to the trade date
//! CME prints:
//!
//! * a **full closure** — New Year's Day, Christmas Day, 2011's Good Friday and
//!   the observed days around them — which removes the trading day and the
//!   prior-evening leg that fed it;
//! * a **holiday morning session that halts early**: every Monday holiday from
//!   2010 through 2012, plus Wednesday 2012-07-04, prints a `1030 CT` halt, so
//!   the row is an early close at 10:30 CT; Good Friday 2010 and 2012 close
//!   their day sessions at 08:15 CT, and the Friday-holiday eves at 12:15 CT;
//! * a **late open** on the trade date after a closure — 2011-12-27, 2012-01-03
//!   and 2012-12-26 print `0500 CT - CME Globex open for trade date ...`, so
//!   the trade date's first open is 05:00 CT on the trade date itself rather
//!   than the ordinary 17:00 CT on the eve, twelve hours later.
//!
//! **2016-2018.** CME's own published Globex holiday schedules are the T1
//! source for the era: per-holiday PDFs inside the 2016 annual bundle, a
//! standalone 2016 New Year's PDF, and per-product-group sheets for 2017 and
//! 2018, whose annual bundles carry CME's final revisions. The era's grid is
//! the wrapped `17:00 CT -> 16:00 CT` leg, so an early close
//! is the printed final close stated on the trade date it ends and a closure
//! removes the trade date with its prior-evening leg; every stated re-open at
//! the ordinary 17:00 CT evening open ships no row. Thirty-four rows: nine
//! closures, twenty-four early closes — 12:00 CT on the nine Monday and
//! Thursday holidays and 12:15 CT on the three Thanksgiving Fridays and three
//! year-end half-days — and one late open.
//!
//! **2022-2024.** The grid is one wrapped `17:00 CT -> 16:00 CT` leg per trade
//! date, the same shape the 2016-2018 era states, and its two conversions are
//! the same. Thirty-five rows: seven closures, twenty-five early closes — 12:00
//! CT on the nineteen Monday and Thursday holidays and 12:15 CT on 2022-11-25,
//! 2023-07-03, 2023-11-24, 2024-07-03, 2024-11-29 and 2024-12-24 — no late open,
//! and three `Unsourced` rows. The 2022 rows and the 2023 rows CME published a holiday
//! schedule for are **T1**; the three 2023 dates it published nothing for and
//! all of 2024 are **T2**. The three `Unsourced` dates — 2023-01-16,
//! 2023-02-20 and 2023-04-07 — mean the operator published nothing this crate
//! could read, not that no holiday fell on them; an operator document stating
//! each date in session language would close them.
//!
//! **2025-2027.** The family's grid in this window is the one CME Globex
//! notice 20210621 put in force on 2021-06-27: a trading day for venue-local
//! trade date `D` opens 17:00 CT on the preceding business evening, runs
//! continuously through the 08:30-15:15 CT regular session and the
//! 15:15-16:00 CT extended leg, and ends at its 16:00 CT final close on `D`.
//! Every row is therefore stated on the day the trading day *closes*, never on
//! the operator's event date, so a session that opened the previous evening is
//! clipped on the correct civil day. These rows are **T2**, from the operator's
//! own trading-hours service.
//!
//! Two conversions are worth naming because every row in that window depends
//! on them.
//!
//! An **early close** replaces that 16:00 CT final close. On the Monday and
//! Thursday holidays CME publishes the instant as a `preopen` rather than a
//! `closed` event and carries the whole span under the following business
//! day's trade date; matching still stops at the printed instant, and on the
//! crate's close-date key that instant is this trade date's final close.
//!
//! A **closure** removes the complete trading day, the prior-evening wrap
//! included, and leaves the leg that opens on the holiday evening for the next
//! trade date alone — which is exactly what CME publishes on those dates
//! (`16:00 preopen; 17:00 open`, carrying the next trade date).
//!
//! The evidence, the operator's printed instants, the event-date-to-trade-date
//! conversion behind each row and these windows' declared gaps are in
//! [`docs/evidence/globex_equity_index.md`](../../../../../docs/evidence/globex_equity_index.md).
//!
//! The era's one late open is 2018-12-26, whose CME Christmas sheet prints the
//! pair `Pre-opening 15:15; Open 15:30`: that is the era's routine
//! extended-session grid, printed as one pair rather than a disagreement
//! between sources, and it is encoded as a late open at 15:30 CT.

use super::EvidenceTier::{T1, T2};
use super::HolidayKind::{Closed, ReplacementBlocks, Unsourced};
use super::fences::{early_close, late_open};
use super::{HolidayTable, holidays};
use crate::calendar::exceptions::ExceptionBlock;

/// The complete trading day of the 2026-06-22, 2026-07-06 and 2027-06-21 trade
/// dates, which CME states a Saturday session on.
///
/// The equity-index day is one continuous 17:00-16:00 CT matching envelope, but
/// it carries a **regular** session inside it: 08:30-15:15 CT is `regular` and
/// the rest of the envelope is `extended`. The set therefore splits each
/// matching envelope at the regular boundaries — extended, regular, extended —
/// so that `is_open_regular`, `session_state` and the regular bounds keep
/// answering on these dates exactly as they do on an ordinary week. Two
/// envelopes carry a split below: the Thursday-evening-to-Friday one, whose
/// regular session the Friday early close cuts short at 12:00 CT, and the
/// Sunday-evening-to-Monday one.
///
/// Stating the envelope as one `extended` block instead would answer
/// `OpenExtended` at 10:00 CT where the normal week answers `OpenRegular`, and
/// would move the regular session's bounds to the next trade date; a
/// replacement replaces the complete trade date, and the scan selects blocks by
/// kind. An independent review caught that, which is why the split is explicit
/// and pinned by a test.
///
/// The blocks state each date completely: the Thursday Pre-Open queue and the
/// session it opens into at offset `-4`, the Saturday session at offset `-2`,
/// the Sunday Pre-Open queue at offset `-1`, and the
/// Sunday-17:00-through-Monday-16:00 envelope at offset `-1`. Evidence:
/// `docs/evidence/globex_equity_index.md`.
///
/// The Thursday leg is stated because the operator assigns it to this trade
/// date: its own window prints the Friday early close as an event carrying this
/// date's trade date, so the session opening Thursday 17:00 CT and ending at
/// that close belongs here and not to the holiday. The **Friday evening** after
/// that close is deliberately not stated: CME publishes no Friday-evening open
/// for these dates, so a block there would put an open on the clock at an
/// instant no operator document states.
pub(crate) static SATURDAY_SESSION_BLOCKS: [ExceptionBlock; 8] = [
    ExceptionBlock::order_entry(-4, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-4, 17 * 3_600, 8 * 3_600 + 30 * 60),
    ExceptionBlock::regular(-3, 8 * 3_600 + 30 * 60, 12 * 3_600),
    ExceptionBlock::extended(-2, 5 * 3_600, 17 * 3_600),
    ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 8 * 3_600 + 30 * 60),
    ExceptionBlock::regular(0, 8 * 3_600 + 30 * 60, 15 * 3_600 + 15 * 60),
    ExceptionBlock::extended(0, 15 * 3_600 + 15 * 60, 16 * 3_600),
];

/// 12:00 CT, the Monday/Thursday-holiday and Independence-Day final close.
const NOON: u32 = 12 * 3_600;
/// 12:15 CT, the Christmas-Eve and day-after-Thanksgiving final close.
const QUARTER_PAST_NOON: u32 = 12 * 3_600 + 15 * 60;
/// 08:15 CT, the Good Friday 2026 equity-index final close.
const QUARTER_PAST_EIGHT: u32 = 8 * 3_600 + 15 * 60;

/// The family's built-in holiday rows and the windows they were audited over.
///
///  Six audited eras: 2010-2012 at T1, 2013-2015 at T1, 2016-2018 at T1,
/// 2019-2021 at T1, 2022-2024 at T1/T2 and 2025-2027 at T2.
///  Every audited interval is contiguous, and nothing before 2010-01-01
/// has a table at all: that span lies outside every window, so
/// `holiday_on` has no answer there rather than reporting a normal date.
///
/// Coverage ends at the
/// operator's published future: CME's trading-hours service answers through New
/// Year 2028, and LAW-NO-FABRICATED-DATES permits encoding an unconditional,
/// fully sourced future ahead of its effective day. Inside a window a date with
/// no row was audited and found normal, except where an `Unsourced` row marks
/// the operator's silence instead.
// Evidence: docs/evidence/globex_equity_index.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2012, 12, 31), (2013, 1, 1) ..= (2015, 12, 31), (2016, 1, 1) ..= (2018, 12, 31), (2019, 1, 1) ..= (2021, 12, 31), (2022, 1, 1) ..= (2024, 12, 31), (2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2010-01-01 - T1 - 2010-new-years.pdf - closed: new year's day 2010.
        (2010, 1, 1, Closed, T1, "2010-new-years.pdf @2010-02-15T05:16:52Z"),
        (2010, 1, 18, early_close(10 * 3_600 + 30 * 60), T1, "2010-martin-luther-king.pdf @2010-03-31T06:42:26Z"),
        (2010, 2, 15, early_close(10 * 3_600 + 30 * 60), T1, "2010-presidents-day.pdf @2010-02-15T06:46:41Z"),
        (2010, 4, 2, early_close(8 * 3_600 + 15 * 60), T1, "2010-good-friday.pdf @2010-06-01T11:19:16Z"),
        (2010, 5, 31, early_close(10 * 3_600 + 30 * 60), T1, "2010-memorial-day.pdf @2010-06-01T09:42:25Z"),
        (2010, 7, 5, early_close(10 * 3_600 + 30 * 60), T1, "2010-4th-of-july.pdf @2010-06-02T00:56:37Z"),
        (2010, 9, 6, early_close(10 * 3_600 + 30 * 60), T1, "2010-labor-day.pdf @2010-06-02T00:56:41Z"),
        (2010, 11, 25, early_close(10 * 3_600 + 30 * 60), T1, "2010-thanksgiving.pdf @2010-11-22T09:40:12Z"),
        (2010, 11, 26, early_close(12 * 3_600 + 15 * 60), T1, "2010-thanksgiving.pdf @2010-11-22T09:40:12Z"),
        // 2010-12-24 - T1 - 2010-christmas.pdf - closed: christmas day 2010 observed.
        (2010, 12, 24, Closed, T1, "2010-christmas.pdf @2010-12-14T06:12:38Z"),
        (2011, 1, 17, early_close(10 * 3_600 + 30 * 60), T1, "2011-martin-luther-king.pdf @2011-10-28T02:34:29Z"),
        (2011, 2, 21, early_close(10 * 3_600 + 30 * 60), T1, "2011-presidents-day.pdf @2011-10-28T02:35:16Z"),
        // 2011-04-22 - T1 - 2011-good-friday.pdf - closed: good friday 2011.
        (2011, 4, 22, Closed, T1, "2011-good-friday.pdf @2011-10-28T02:37:07Z"),
        (2011, 5, 30, early_close(10 * 3_600 + 30 * 60), T1, "2011-memorial-day.pdf @2013-09-30T10:56:52Z"),
        (2011, 7, 4, early_close(10 * 3_600 + 30 * 60), T1, "2011-4th-of-july.pdf @2011-11-01T14:40:54Z"),
        (2011, 9, 5, early_close(10 * 3_600 + 30 * 60), T1, "2011-labor-day.pdf @2011-11-01T14:43:45Z"),
        (2011, 11, 24, early_close(10 * 3_600 + 30 * 60), T1, "2011-thanksgiving.pdf @2011-11-24T18:52:46Z"),
        (2011, 11, 25, early_close(12 * 3_600 + 15 * 60), T1, "2011-thanksgiving.pdf @2011-11-24T18:52:46Z"),
        // 2011-12-26 - T1 - 2011-christmas.pdf - closed: christmas day 2011 observed.
        (2011, 12, 26, Closed, T1, "2011-christmas.pdf @2012-01-25T02:05:48Z"),
        (2011, 12, 27, late_open(5 * 3_600), T1, "2011-christmas.pdf @2012-01-25T02:05:48Z"),
        // 2012-01-02 - T1 - 2012-new-years.pdf - closed: new year's day 2012 observed.
        (2012, 1, 2, Closed, T1, "2012-new-years.pdf @2012-01-25T02:54:30Z"),
        (2012, 1, 3, late_open(5 * 3_600), T1, "2012-new-years.pdf @2012-01-25T02:54:30Z"),
        (2012, 1, 16, early_close(10 * 3_600 + 30 * 60), T1, "2012-martin-luther-king.pdf @2012-05-05T16:15:26Z"),
        (2012, 2, 20, early_close(10 * 3_600 + 30 * 60), T1, "2012-presidents-day.pdf @2012-05-05T16:15:39Z"),
        (2012, 4, 6, early_close(8 * 3_600 + 15 * 60), T1, "2012-good-friday.pdf @2012-04-17T00:42:47Z"),
        (2012, 5, 28, early_close(10 * 3_600 + 30 * 60), T1, "2012-memorial-day.pdf @2012-09-15T00:37:14Z"),
        (2012, 7, 3, early_close(12 * 3_600 + 15 * 60), T1, "2012-4th-of-july.pdf @2012-09-15T00:39:23Z"),
        (2012, 7, 4, early_close(10 * 3_600 + 30 * 60), T1, "2012-4th-of-july.pdf @2012-09-15T00:39:23Z"),
        (2012, 9, 3, early_close(10 * 3_600 + 30 * 60), T1, "2012-labor-day.pdf @2012-09-15T00:34:37Z"),
        (2012, 11, 22, early_close(10 * 3_600 + 30 * 60), T1, "2012-thanksgiving.pdf @2013-01-27T22:39:01Z"),
        (2012, 11, 23, early_close(12 * 3_600 + 15 * 60), T1, "2012-thanksgiving.pdf @2013-01-27T22:39:01Z"),
        (2012, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        // 2012-12-25 - T1 - 2012-christmas.pdf - closed: christmas day 2012.
        (2012, 12, 25, Closed, T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        (2012, 12, 26, late_open(5 * 3_600), T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        // 2013-01-01 - T1 - 2013-new-years.pdf @2013-04-14T19:41:46Z - CME prints no session running through this date.
        (2013, 1, 1, Closed, T1, "2013-new-years.pdf @2013-04-14T19:41:46Z"),
        // 2013-01-02 - T1 - 2013-new-years.pdf @2013-04-14T19:41:46Z - the trade date's first open is 5:00 CT: the evening leg that would have opened earlier did not run.
        (2013, 1, 2, late_open(5 * 3_600), T1, "2013-new-years.pdf @2013-04-14T19:41:46Z"),
        // 2013-01-21 - T1 - 2013-martin-luther-king.pdf @2012-11-19T00:16:09Z - the printed final close 10:30 CT is earlier than the family's ordinary 16:15 CT.
        (2013, 1, 21, early_close(10 * 3_600 + 30 * 60), T1, "2013-martin-luther-king.pdf @2012-11-19T00:16:09Z"),
        // 2013-02-18 - T1 - 2013-presidents-day.pdf @2013-03-09T11:53:37Z - the printed final close 10:30 CT is earlier than the family's ordinary 16:15 CT.
        (2013, 2, 18, early_close(10 * 3_600 + 30 * 60), T1, "2013-presidents-day.pdf @2013-03-09T11:53:37Z"),
        // 2013-03-29 - T1 - 2013-good-friday.pdf @2013-06-23T19:59:25Z - CME prints no session running through this date.
        (2013, 3, 29, Closed, T1, "2013-good-friday.pdf @2013-06-23T19:59:25Z"),
        // 2013-05-27 - T1 - 2013-memorial-day.pdf @2013-06-23T20:36:04Z - the printed final close 10:30 CT is earlier than the family's ordinary 16:15 CT.
        (2013, 5, 27, early_close(10 * 3_600 + 30 * 60), T1, "2013-memorial-day.pdf @2013-06-23T20:36:04Z"),
        // 2013-07-03 - T1 - 2013-4th-of-july.pdf @2013-06-23T20:58:25Z - the printed final close 12:15 CT is earlier than the family's ordinary 16:15 CT.
        (2013, 7, 3, early_close(12 * 3_600 + 15 * 60), T1, "2013-4th-of-july.pdf @2013-06-23T20:58:25Z"),
        // 2013-07-04 - T1 - 2013-4th-of-july.pdf @2013-06-23T20:58:25Z - the printed final close 10:30 CT is earlier than the family's ordinary 16:15 CT.
        (2013, 7, 4, early_close(10 * 3_600 + 30 * 60), T1, "2013-4th-of-july.pdf @2013-06-23T20:58:25Z"),
        // 2013-09-02 - T1 - 2013-labor-day.pdf @2013-09-02T17:08:41Z - the printed final close 10:30 CT is earlier than the family's ordinary 16:15 CT.
        (2013, 9, 2, early_close(10 * 3_600 + 30 * 60), T1, "2013-labor-day.pdf @2013-09-02T17:08:41Z"),
        // 2013-11-28 - T1 - 2013-thanksgiving.pdf @2014-02-14T06:28:36Z - the printed final close 10:30 CT is earlier than the family's ordinary 16:15 CT.
        (2013, 11, 28, early_close(10 * 3_600 + 30 * 60), T1, "2013-thanksgiving.pdf @2014-02-14T06:28:36Z"),
        // 2013-11-29 - T1 - 2013-thanksgiving.pdf @2014-02-14T06:28:36Z - the printed final close 12:15 CT is earlier than the family's ordinary 16:15 CT.
        (2013, 11, 29, early_close(12 * 3_600 + 15 * 60), T1, "2013-thanksgiving.pdf @2014-02-14T06:28:36Z"),
        // 2013-12-24 - T1 - 2013-christmas.pdf @2014-04-12T06:24:28Z - the printed final close 12:15 CT is earlier than the family's ordinary 16:15 CT.
        (2013, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "2013-christmas.pdf @2014-04-12T06:24:28Z"),
        // 2013-12-25 - T1 - 2013-christmas.pdf @2014-04-12T06:24:28Z - CME prints no session running through this date.
        (2013, 12, 25, Closed, T1, "2013-christmas.pdf @2014-04-12T06:24:28Z"),
        // 2013-12-26 - T1 - 2013-christmas.pdf @2014-04-12T06:24:28Z - the trade date's first open is 5:00 CT: the evening leg that would have opened earlier did not run.
        (2013, 12, 26, late_open(5 * 3_600), T1, "2013-christmas.pdf @2014-04-12T06:24:28Z"),
        // 2014-01-01 - T1 - 2014-new-years.pdf @2013-10-07T20:58:00Z - CME prints no session running through this date.
        (2014, 1, 1, Closed, T1, "2014-new-years.pdf @2013-10-07T20:58:00Z"),
        // 2014-01-02 - T1 - 2014-new-years.pdf @2013-10-07T20:58:00Z - the trade date's first open is 5:00 CT: the evening leg that would have opened earlier did not run.
        (2014, 1, 2, late_open(5 * 3_600), T1, "2014-new-years.pdf @2013-10-07T20:58:00Z"),
        // 2014-01-20 - T1 - 2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z - the printed final close 10:30 CT is earlier than the family's ordinary 16:15 CT.
        (2014, 1, 20, early_close(10 * 3_600 + 30 * 60), T1, "2014-martin-luther-king-holiday-schedule.pdf @2014-03-26T16:02:15Z"),
        // 2014-02-17 - T1 - 2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z - the printed final close 10:30 CT is earlier than the family's ordinary 16:15 CT.
        (2014, 2, 17, early_close(10 * 3_600 + 30 * 60), T1, "2014-presidents-day-holiday-schedule.pdf @2014-02-14T19:23:32Z"),
        // 2014-04-18 - T1 - 2014-good-friday-holiday-schedule.pdf @2014-03-26T15:27:35Z - CME prints no session running through this date.
        (2014, 4, 18, Closed, T1, "2014-good-friday-holiday-schedule.pdf @2014-03-26T15:27:35Z"),
        // 2014-05-26 - T1 - 2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:15 CT.
        (2014, 5, 26, early_close(12 * 3_600), T1, "2014-memorial-day-holiday-schedule.pdf @2014-07-08T02:01:55Z"),
        // 2014-07-03 - T1 - 2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z - the printed final close 12:15 CT is earlier than the family's ordinary 16:15 CT.
        (2014, 7, 3, early_close(12 * 3_600 + 15 * 60), T1, "2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z"),
        // 2014-07-04 - T1 - 2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:15 CT.
        (2014, 7, 4, early_close(12 * 3_600), T1, "2014-4th-of-july-holiday-schedule.pdf @2014-07-08T01:57:36Z"),
        // 2014-09-01 - T1 - 2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:15 CT.
        (2014, 9, 1, early_close(12 * 3_600), T1, "2014-labor-day-holiday-schedule.pdf @2014-09-12T07:16:08Z"),
        // 2014-11-27 - T1 - 2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:15 CT.
        (2014, 11, 27, early_close(12 * 3_600), T1, "2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z"),
        // 2014-11-28 - T1 - 2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z - the printed final close 12:15 CT is earlier than the family's ordinary 16:15 CT.
        (2014, 11, 28, early_close(12 * 3_600 + 15 * 60), T1, "2014-thanksgiving-holiday-schedule.pdf @2015-01-21T14:54:56Z"),
        // 2014-12-24 - T1 - 2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z - the printed final close 12:15 CT is earlier than the family's ordinary 16:15 CT.
        (2014, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z"),
        // 2014-12-25 - T1 - 2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z - CME prints no session running through this date.
        (2014, 12, 25, Closed, T1, "2014-christmas-holiday-schedule.pdf @2015-01-21T14:10:00Z"),
        // 2015-01-01 - T1 - 2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z - CME prints no session running through this date.
        (2015, 1, 1, Closed, T1, "2015-new-years-holiday-schedule.pdf @2015-01-21T14:10:43Z"),
        // 2015-01-19 - T1 - 2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:15 CT.
        (2015, 1, 19, early_close(12 * 3_600), T1, "2015-martin-luther-king-holiday-schedule.pdf @2015-01-21T14:10:12Z"),
        // 2015-02-16 - T1 - 2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:15 CT.
        (2015, 2, 16, early_close(12 * 3_600), T1, "2015-presidents-day-holiday-schedule.pdf @2015-01-21T19:24:01Z"),
        // 2015-04-03 - T1 - 2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z - the printed final close 8:15 CT is earlier than the family's ordinary 16:15 CT.
        (2015, 4, 3, early_close(8 * 3_600 + 15 * 60), T1, "2015-good-friday-holiday-schedule.pdf @2015-09-05T22:32:30Z"),
        // 2015-05-25 - T1 - 2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:15 CT.
        (2015, 5, 25, early_close(12 * 3_600), T1, "2015-memorial-day-holiday-schedule.pdf @2015-03-26T11:39:38Z"),
        // 2015-07-03 - T1 - 2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:15 CT.
        (2015, 7, 3, early_close(12 * 3_600), T1, "2015-4th-of-july-holiday-schedule.pdf @2015-09-05T22:27:33Z"),
        // 2015-09-07 - T1 - 2015-labor-day-holiday-schedule.pdf @2015-08-24T02:30:39Z - the printed final close 12:00 CT is earlier than the family's ordinary 16:15 CT.
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
        // 2019-07-03 - T1 - 2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z - the printed final close 12:15 CT is earlier than the ordinary 16:00 CT close.
        (2019, 7, 3, early_close(12 * 3_600 + 15 * 60), T1, "2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z"),
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
        // 2021-04-02 - T1 - 2021-holiday-calendars.zip#2021-good-friday-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - the printed final close 08:15 CT is earlier than the ordinary 16:00 CT close.
        (2021, 4, 2, early_close(8 * 3_600 + 15 * 60), T1, "2021-holiday-calendars.zip#2021-good-friday-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
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
        // 2022-01-17 - T1 - 2022-mlk-day-holiday-schedule.xls @2022-01-17T21:22:30Z - early close 12:00 CT.
        (2022, 1, 17, early_close(12 * 3_600), T1, "2022-mlk-day-holiday-schedule.xls @2022-01-17T21:22:30Z"),
        // 2022-02-21 - T1 - 2022-presidents-day-holiday-schedule.xls @2022-07-04T07:38:10Z - early close 12:00 CT.
        (2022, 2, 21, early_close(12 * 3_600), T1, "2022-presidents-day-holiday-schedule.xls @2022-07-04T07:38:10Z"),
        // 2022-04-15 - T1 - 2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z - closed: no trade date.
        (2022, 4, 15, Closed, T1, "2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z"),
        // 2022-05-30 - T1 - 2022-memorial-day-holiday-schedule.xls @2022-07-04T06:54:38Z - early close 12:00 CT.
        (2022, 5, 30, early_close(12 * 3_600), T1, "2022-memorial-day-holiday-schedule.xls @2022-07-04T06:54:38Z"),
        // 2022-06-20 - T1 - 2022-juneteenth-holiday-schedule.xls @2022-06-20T20:02:10Z - early close 12:00 CT.
        (2022, 6, 20, early_close(12 * 3_600), T1, "2022-juneteenth-holiday-schedule.xls @2022-06-20T20:02:10Z"),
        // 2022-07-04 - T1 - 2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z - early close 12:00 CT.
        (2022, 7, 4, early_close(12 * 3_600), T1, "2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z"),
        // 2022-09-05 - T1 - 2022-labor-day-holiday-schedule.xls @2022-07-04T06:54:41Z - early close 12:00 CT.
        (2022, 9, 5, early_close(12 * 3_600), T1, "2022-labor-day-holiday-schedule.xls @2022-07-04T06:54:41Z"),
        // 2022-11-24 - T1 - 2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z - early close 12:00 CT.
        (2022, 11, 24, early_close(12 * 3_600), T1, "2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z"),
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
        // 2023-05-29 - T1 - memorial-day-2023.pdf @2023-04-20T22:40:18Z - early close 12:00 CT.
        (2023, 5, 29, early_close(12 * 3_600), T1, "memorial-day-2023.pdf @2023-04-20T22:40:18Z"),
        // 2023-06-19 - T1 - juneteenth-2023.pdf @2023-06-13T18:59:49Z - early close 12:00 CT.
        (2023, 6, 19, early_close(12 * 3_600), T1, "juneteenth-2023.pdf @2023-06-13T18:59:49Z"),
        // 2023-07-03 - T1 - 4th-of-july-2023.pdf @2023-06-27T12:50:57Z - early close 12:15 CT.
        (2023, 7, 3, early_close(12 * 3_600 + 15 * 60), T1, "4th-of-july-2023.pdf @2023-06-27T12:50:57Z"),
        // 2023-07-04 - T1 - 4th-of-july-2023.pdf @2023-06-27T12:50:57Z - early close 12:00 CT.
        (2023, 7, 4, early_close(12 * 3_600), T1, "4th-of-july-2023.pdf @2023-06-27T12:50:57Z"),
        // 2023-09-04 - T1 - labor-day-2023.pdf @2023-08-02T19:24:46Z - early close 12:00 CT.
        (2023, 9, 4, early_close(12 * 3_600), T1, "labor-day-2023.pdf @2023-08-02T19:24:46Z"),
        // 2023-11-23 - T1 - thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z - early close 12:00 CT.
        (2023, 11, 23, early_close(12 * 3_600), T1, "thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z"),
        // 2023-11-24 - T1 - thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z - early close 12:15 CT.
        (2023, 11, 24, early_close(12 * 3_600 + 15 * 60), T1, "thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z"),
        // 2023-12-25 - T1 - christmas-day-2023.pdf @2026-07-19T09:52:48Z - closed: no trade date.
        (2023, 12, 25, Closed, T1, "christmas-day-2023.pdf @2026-07-19T09:52:48Z"),
        // 2024-01-01 - T1 - new-years-day-2024.pdf @2026-08-11T16:57:16Z - closed: no trade date.
        (2024, 1, 1, Closed, T1, "new-years-day-2024.pdf @2026-08-11T16:57:16Z"),
        // 2024-01-15 - T2 - CME-SVC-2024-01-14 - early close 12:00 CT.
        (2024, 1, 15, early_close(12 * 3_600), T2, "CME-SVC-2024-01-14"),
        // 2024-02-19 - T2 - CME-SVC-2024-02-18 - early close 12:00 CT.
        (2024, 2, 19, early_close(12 * 3_600), T2, "CME-SVC-2024-02-18"),
        // 2024-03-29 - T2 - CME-SVC-2024-03-28 - closed: no trade date.
        (2024, 3, 29, Closed, T2, "CME-SVC-2024-03-28"),
        // 2024-05-27 - T2 - CME-SVC-2024-05-26 - early close 12:00 CT.
        (2024, 5, 27, early_close(12 * 3_600), T2, "CME-SVC-2024-05-26"),
        // 2024-06-19 - T2 - CME-SVC-2024-06-18 - early close 12:00 CT.
        (2024, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-2024-06-18"),
        // 2024-07-03 - T2 - CME-SVC-2024-07-03 - early close 12:15 CT.
        (2024, 7, 3, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-2024-07-03"),
        // 2024-07-04 - T2 - CME-SVC-2024-07-03 - early close 12:00 CT.
        (2024, 7, 4, early_close(12 * 3_600), T2, "CME-SVC-2024-07-03"),
        // 2024-09-02 - T2 - CME-SVC-2024-09-01 - early close 12:00 CT.
        (2024, 9, 2, early_close(12 * 3_600), T2, "CME-SVC-2024-09-01"),
        // 2024-11-28 - T2 - CME-SVC-2024-11-27 - early close 12:00 CT.
        (2024, 11, 28, early_close(12 * 3_600), T2, "CME-SVC-2024-11-27"),
        // 2024-11-29 - T2 - CME-SVC-2024-11-27 - early close 12:15 CT.
        (2024, 11, 29, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-2024-11-27"),
        // 2024-12-24 - T2 - CME-SVC-2024-12-24 - early close 12:15 CT.
        (2024, 12, 24, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-2024-12-24"),
        // 2024-12-25 - T2 - CME-SVC-2024-12-24 - closed: no trade date.
        (2024, 12, 25, Closed, T2, "CME-SVC-2024-12-24"),
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 — T2 — CME-SVC-2025-01-19 — Martin Luther King Jr. Day:
        // matching stops 12:00 CT, published as a preopen.
        (2025, 1, 20, early_close(NOON), T2, "CME-SVC-2025-01-19"),
        // 2025-02-17 — T2 — CME-SVC-2025-02-16 — Presidents' Day: 12:00 CT.
        (2025, 2, 17, early_close(NOON), T2, "CME-SVC-2025-02-16"),
        // 2025-04-18 — T2 — CME-SVC-2025-04-17 — Good Friday: full Globex
        // closure, no events published for any family.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 — T2 — CME-SVC-2025-05-25 — Memorial Day: 12:00 CT.
        (2025, 5, 26, early_close(NOON), T2, "CME-SVC-2025-05-25"),
        // 2025-06-19 — T2 — CME-SVC-2025-06-18 — Juneteenth: 12:00 CT.
        (2025, 6, 19, early_close(NOON), T2, "CME-SVC-2025-06-18"),
        // 2025-07-03 — T2 — CME-SVC-2025-07-03 — Independence Day eve: equity
        // index alone closes 12:15 CT; the evening leg then runs normally.
        (2025, 7, 3, early_close(QUARTER_PAST_NOON), T2, "CME-SVC-2025-07-03"),
        // 2025-07-04 — T2 — CME-SVC-2025-07-03 — Independence Day: 12:00 CT.
        (2025, 7, 4, early_close(NOON), T2, "CME-SVC-2025-07-03"),
        // 2025-09-01 — T2 — CME-SVC-2025-08-31 — Labor Day: 12:00 CT.
        (2025, 9, 1, early_close(NOON), T2, "CME-SVC-2025-08-31"),
        // 2025-11-27 — T2 — CME-SVC-2025-11-26 — Thanksgiving: 12:00 CT.
        (2025, 11, 27, early_close(NOON), T2, "CME-SVC-2025-11-26"),
        // 2025-11-28 — T2 — CME-SVC-2025-11-26 — day after Thanksgiving:
        // 12:15 CT, from CME's finalised post-holiday publication.
        (
            2025,
            11,
            28,
            early_close(QUARTER_PAST_NOON),
            T2,
            "CME-SVC-2025-11-26"
        ),
        // 2025-11-29 — T2 — CME-SVC-2025-11-26-SAT — Thanksgiving Saturday: the
        // service publishes an empty schedule for all ten products, and CME's
        // 2025 Globex table states the period as "27 - 29 November 2025".
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-12-24 — T2 — CME-SVC-2025-12-24 — Christmas Eve: 12:15 CT, and
        // no evening re-open because 2025-12-25 is closed.
        (
            2025,
            12,
            24,
            early_close(QUARTER_PAST_NOON),
            T2,
            "CME-SVC-2025-12-24"
        ),
        // 2025-12-25 — T2 — CME-SVC-2025-12-24 — Christmas Day: no trading day
        // of its own; 16:00 preopen and 17:00 open carry trade date 2025-12-26.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-24"),
        // 2026-01-01 — T2 — CME-SVC-2025-12-31 — New Year's Day: no trading day
        // of its own; 16:00 preopen and 17:00 open carry trade date 2026-01-02.
        (2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-01-19 — T2 — CME-SVC-2026-01-18 — Martin Luther King Jr. Day:
        // 12:00 CT.
        (2026, 1, 19, early_close(NOON), T2, "CME-SVC-2026-01-18"),
        // 2026-02-16 — T2 — CME-SVC-2026-02-15 — Presidents' Day: 12:00 CT.
        (2026, 2, 16, early_close(NOON), T2, "CME-SVC-2026-02-15"),
        // 2026-04-03 — T2 — CME-SVC-2026-04-01 — Good Friday: the exception CME
        // itself flags for the employment release; equity index closes 08:15 CT.
        (
            2026,
            4,
            3,
            early_close(QUARTER_PAST_EIGHT),
            T2,
            "CME-SVC-2026-04-01"
        ),
        // 2026-05-25 — T2 — CME-SVC-2026-05-24 — Memorial Day: 12:00 CT.
        (2026, 5, 25, early_close(NOON), T2, "CME-SVC-2026-05-24"),
        // 2026-06-19 — T2 — CME-SVC-2026-06-18 — Juneteenth: 12:00 CT.
        (2026, 6, 19, early_close(NOON), T2, "CME-SVC-2026-06-18"),
        // 2026-06-22 — T2 — CME-SVC-2026-06-18 — Saturday session 05:00-17:00 CT
        // carrying this trade date; the complete trading day is stated.
        (2026, 6, 22, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 — T2 — CME-SVC-2026-07-03 — Independence Day observed:
        // 12:00 CT.
        (2026, 7, 3, early_close(NOON), T2, "CME-SVC-2026-07-03"),
        // 2026-07-06 — T2 — CME-SVC-2026-07-03 — Saturday session 05:00-17:00 CT
        // carrying this trade date; the complete trading day is stated.
        (2026, 7, 6, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 — T2 — CME-SVC-2026-09-06 — Labor Day: 12:00 CT.
        (2026, 9, 7, early_close(NOON), T2, "CME-SVC-2026-09-06"),
        // 2026-11-26 — T2 — CME-SVC-2026-11-25 — Thanksgiving: 12:00 CT.
        (2026, 11, 26, early_close(NOON), T2, "CME-SVC-2026-11-25"),
        // 2026-11-27 — T2 — CME-SVC-2026-11-25 — day after Thanksgiving:
        // 12:15 CT.
        (
            2026,
            11,
            27,
            early_close(QUARTER_PAST_NOON),
            T2,
            "CME-SVC-2026-11-25"
        ),
        // 2026-12-24 — T2 — CME-SVC-2026-12-22 — Christmas Eve: 12:15 CT, and
        // no evening re-open because 2026-12-25 is closed.
        (
            2026,
            12,
            24,
            early_close(QUARTER_PAST_NOON),
            T2,
            "CME-SVC-2026-12-22"
        ),
        // 2026-12-25 — T2 — CME-SVC-2026-12-24 — Christmas Day: full Globex
        // closure, no events published.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 — T2 — CME-SVC-2026-12-31 — New Year's Day: full Globex
        // closure, no events published.
        (2027, 1, 1, Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-01-18 — T2 — CME-SVC-2027-01-17 — Martin Luther King Jr. Day:
        // 12:00 CT.
        (2027, 1, 18, early_close(NOON), T2, "CME-SVC-2027-01-17"),
        // 2027-02-15 — T2 — CME-SVC-2027-02-14 — Presidents' Day: 12:00 CT.
        (2027, 2, 15, early_close(NOON), T2, "CME-SVC-2027-02-14"),
        // 2027-03-26 — T2 — CME-SVC-2027-03-25 — Good Friday: full Globex
        // closure, no events published.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 — T2 — CME-SVC-2027-05-30 — Memorial Day: 12:00 CT.
        (2027, 5, 31, early_close(NOON), T2, "CME-SVC-2027-05-30"),
        // 2027-06-18 — T2 — CME-SVC-2027-06-17 — Juneteenth observed: 12:00 CT.
        (2027, 6, 18, early_close(NOON), T2, "CME-SVC-2027-06-17"),
        // 2027-06-21 — T2 — CME-SVC-2027-06-17 — Saturday session 05:00-17:00 CT
        // carrying this trade date; the complete trading day is stated.
        (2027, 6, 21, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 — T2 — CME-SVC-2027-07-04 — Independence Day observed:
        // 12:00 CT.
        (2027, 7, 5, early_close(NOON), T2, "CME-SVC-2027-07-04"),
        // 2027-09-06 — T2 — CME-SVC-2027-09-05 — Labor Day: 12:00 CT.
        (2027, 9, 6, early_close(NOON), T2, "CME-SVC-2027-09-05"),
        // 2027-11-25 — T2 — CME-SVC-2027-11-24 — Thanksgiving: 12:00 CT.
        (2027, 11, 25, early_close(NOON), T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 — T2 — CME-SVC-2027-11-24 — day after Thanksgiving:
        // 12:15 CT.
        (
            2027,
            11,
            26,
            early_close(QUARTER_PAST_NOON),
            T2,
            "CME-SVC-2027-11-24"
        ),
        // 2027-12-24 — T2 — CME-SVC-2027-12-22 — Christmas 2027, which CME keys
        // to Thursday 2027-12-23: full Globex closure on the Friday.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
