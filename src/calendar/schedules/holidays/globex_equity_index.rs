// SPDX-License-Identifier: MIT-0

//! `globex_equity_index` holiday rows, venue-local trade dates
//! 2010-01-01 .. 2012-12-31, 2016-01-01 .. 2018-12-31 and
//! 2025-01-01 .. 2027-12-31 (LAW-HOLIDAY-SCOPE).
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
use super::HolidayKind::Closed;
use super::fences::{early_close, late_open};
use super::{HolidayTable, holidays};

/// 12:00 CT, the Monday/Thursday-holiday and Independence-Day final close.
const NOON: u32 = 12 * 3_600;
/// 12:15 CT, the Christmas-Eve and day-after-Thanksgiving final close.
const QUARTER_PAST_NOON: u32 = 12 * 3_600 + 15 * 60;
/// 08:15 CT, the Good Friday 2026 equity-index final close.
const QUARTER_PAST_EIGHT: u32 = 8 * 3_600 + 15 * 60;

/// The family's built-in holiday rows and the windows they were audited over.
///
/// Three audited eras: 2010-2012 at T1, 2016-2018 at T1 and 2025-2027 at T2.
/// The 2013-2015 and 2019-2024 intervals between them are audited by no wave
/// and lie outside every window, so `holiday_on` has no answer there rather
/// than reporting a normal date. Coverage ends at the operator's published
/// future: CME's trading-hours service answers through New Year 2028, and
/// LAW-NO-FABRICATED-DATES permits encoding an unconditional, fully sourced
/// future ahead of its effective day. Inside a window a date with no row was
/// audited and found normal.
// Evidence: docs/evidence/globex_equity_index.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2012, 12, 31), (2016, 1, 1) ..= (2018, 12, 31), (2025, 1, 1) ..= (2027, 12, 31)],
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
        // 2026-07-03 — T2 — CME-SVC-2026-07-03 — Independence Day observed:
        // 12:00 CT.
        (2026, 7, 3, early_close(NOON), T2, "CME-SVC-2026-07-03"),
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
