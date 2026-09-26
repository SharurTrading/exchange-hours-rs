// SPDX-License-Identifier: MIT-0

//! CME cryptocurrency holiday rows, venue-local trade dates 2019-01-01 to
//! 2021-12-31, 2022-01-01 to 2024-12-31 and 2025-01-01 to 2027-12-31
//! (LAW-HOLIDAY-SCOPE).
//!
//! Every row is keyed by the crate's own America/Chicago trade date, never by
//! CME's event date: the operator publishes a holiday as an event list on a
//! civil day and prints the trade date each event carries, and it is that
//! printed trade date the rows below are built from.
//!
//! The family's [`HolidayKind::Closed`] rows mean "there is no such trade
//! date", not "trading stopped". Which of the two consequences follows is a
//! property of the era, not of the row:
//!
//! - in the five-day 17:00-16:00 CT era, which closes over the weekend, a
//!   closed trade date deletes its complete trading day including the session
//!   that opened the previous evening, so only a date CME published with no
//!   trading at all carries a row. The Monday and Thursday holidays on which
//!   CME published a 16:00 CT pre-open instead of a 16:00 CT final close are
//!   not such dates: matching ran from the previous 17:00 CT to 16:00 CT as on
//!   a normal day and only the trade-date label merged. Such a holiday carries
//!   no row of its own, because the operator assigns the span the following
//!   business day's trade date: the span ships as a `ReplacementBlocks` row
//!   keyed to that trade date, and its Pre-Open queue is stated with it;
//! - in the 24/7 era from trade date 2026-05-30, where the family assigns a
//!   block to the following open business date, a closed trade date is skipped
//!   by that roll and the connected block survives, carrying the next business
//!   date instead. The operator's own data says exactly this: on a holiday its
//!   16:00 CT final close is either omitted or printed with the following
//!   Monday's trade date.
//!
//! **2022-2024.** The era sits inside the five-day 17:00-16:00 CT era, so each
//! of its `Closed` rows deletes a complete trading day. Fourteen rows: seven
//! closures, four early closes at 12:45 CT — on 2022-11-25, 2023-11-24,
//! 2024-11-29 and 2024-12-24 — and three `Unsourced` rows. The 2022 rows
//! and the 2023 rows CME published a holiday schedule for are **T1**; the
//! three 2023 dates it published nothing for and all of 2024 are **T2**. The
//! three `Unsourced` dates — 2023-01-16, 2023-02-20 and 2023-04-07 — mean the
//! operator published nothing this crate could read, not that no holiday fell
//! on them; an operator document stating each date in session language would
//! close them.
//!
//! The 2025-2027 block is **T2**: CME's own trading-hours service, the channel
//! the operator's trading-hours page calls to render its per-asset-class holiday
//! table, read as bytes and saved. It states nine merged trade dates as
//! `ReplacementBlocks` rows and one six-block day-after-Thanksgiving row. What
//! stays open is the 24/7 era's erased maintenance minute (#93) and the
//! five-day era's undated Pre-Open onset (#123). The quotations, capture times,
//! the event-date-to-trade-date conversion and those gaps are in
//! `docs/evidence/globex_cryptocurrency.md`.

use super::fences::early_close;
use super::{
    EvidenceTier::{T1, T2},
    HolidayKind,
    HolidayKind::{Closed, ReplacementBlocks, Unsourced},
    HolidayTable, holidays,
};
use crate::calendar::exceptions::ExceptionBlock;

/// The complete trading day of the trade dates CME merges with the session
/// before a Monday or Thursday holiday.
///
/// The holiday publishes no final close of its own: the queue and evening
/// session that would carry it are printed against the next business day
/// instead, so the whole span from Sunday evening through the following
/// afternoon carries one trade date. A `Closed` row on the holiday would
/// delete a full evening and day of trading the operator ran.
///
/// - offset `-2`, 16:00-17:00 CT: the Pre-Open queue, on the Sunday this
///   family's five-day week opens.
/// - offset `-2`, 17:00 CT to offset `-1` 16:00 CT: the matching session.
/// - offset `-1`, 16:00-17:00 CT: the holiday's own Pre-Open queue. CME
///   publishes `16:00 preopen` on the holiday.
/// - offset `-1`, 17:00 CT to trade date 16:00 CT: the matching session.
///
/// Evidence: `docs/evidence/globex_cryptocurrency.md`.
pub(crate) static MERGED_SESSION_BLOCKS: [ExceptionBlock; 4] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 16 * 3_600),
    ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 16 * 3_600),
];

/// The same merged day where the `-2` day is an ordinary weekday, so its queue
/// opens at the operator's printed `16:45` CT.
///
/// The Juneteenth Thursday 2025-06-19 merge is the 2025 case: the span opens on
/// the Wednesday evening, and the holiday's own queue still prints `16:00`.
///
/// Evidence: `docs/evidence/globex_cryptocurrency.md`.
pub(crate) static MERGED_SESSION_AFTER_WEEKDAY_BLOCKS: [ExceptionBlock; 4] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 16 * 3_600),
    ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 16 * 3_600),
];

/// The day after Thanksgiving, whose finalised publication adds a `07:00
/// preopen; 07:30 open` pause the pre-holiday capture lacks.
///
/// The Thursday holiday publishes no final close, so this trade date owns the
/// span from the Wednesday evening; the operator then ends matching at `07:00`
/// CT on the trade date, reopens it at `07:30` and closes the day at `13:45`.
///
/// - offset `-2`, 16:45-17:00 CT: the Wednesday Pre-Open queue.
/// - offset `-2`, 17:00 CT to offset `-1` 16:00 CT: the matching session.
/// - offset `-1`, 16:00-17:00 CT: the Thursday holiday's own queue.
/// - offset `-1`, 17:00 CT to trade date 07:00 CT: the matching session.
/// - offset `0`, 07:00-07:30 CT: the pause's Pre-Open queue.
/// - offset `0`, 07:30-13:45 CT: the final matching session.
///
/// Evidence: `docs/evidence/globex_cryptocurrency.md`.
pub(crate) static MERGED_THANKSGIVING_FRIDAY_BLOCKS: [ExceptionBlock; 6] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 16 * 3_600),
    ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 7 * 3_600),
    ExceptionBlock::order_entry(0, 7 * 3_600, 7 * 3_600 + 30 * 60),
    ExceptionBlock::extended(0, 7 * 3_600 + 30 * 60, 13 * 3_600 + 45 * 60),
];

/// The family's built-in holiday rows and the windows they were audited over.
///
/// Three audited eras: 2019-2021 at T1, 2022-2024 at T1/T2 and 2025-2027
/// at T2.
/// Nothing before 2019-01-01 has a table at all: that interval lies
/// outside every window, so `holiday_on` has no answer there rather than
/// reporting a normal date.
///
/// Coverage ends at 2027-12-31, the
/// end of the operator's published future. Inside a window a date with no row
/// is audited normal, except where an `Unsourced` row marks the operator's
/// silence instead.
// Evidence: docs/evidence/globex_cryptocurrency.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2019, 1, 1) ..= (2021, 12, 31), (2022, 1, 1) ..= (2024, 12, 31), (2025, 1, 1) ..= (2027, 12, 31)],
    rows: [

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
        // 2021-11-26 - T1 - 2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - the printed final close 12:45 CT is earlier than the ordinary 16:00 CT close.
        (2021, 11, 26, early_close(12 * 3_600 + 45 * 60), T1, "2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2021-12-24 - T1 - 2021-holiday-calendars.zip#2021-christmas-holiday-schedule-compact.xls @2026-08-30T10:03:27Z - CME prints no session running through this date.
        (2021, 12, 24, Closed, T1, "2021-holiday-calendars.zip#2021-christmas-holiday-schedule-compact.xls @2026-08-30T10:03:27Z"),
        // 2022-04-15 - T1 - 2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z - closed: no trade date.
        (2022, 4, 15, Closed, T1, "2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z"),
        // 2022-11-25 - T1 - 2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z - early close 12:45 CT.
        (2022, 11, 25, early_close(12 * 3_600 + 45 * 60), T1, "2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z"),
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
        // 2023-11-24 - T1 - thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z - early close 12:45 CT.
        (2023, 11, 24, early_close(12 * 3_600 + 45 * 60), T1, "thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z"),
        // 2023-12-25 - T1 - christmas-day-2023.pdf @2026-07-19T09:52:48Z - closed: no trade date.
        (2023, 12, 25, Closed, T1, "christmas-day-2023.pdf @2026-07-19T09:52:48Z"),
        // 2024-01-01 - T1 - new-years-day-2024.pdf @2026-08-11T16:57:16Z - closed: no trade date.
        (2024, 1, 1, Closed, T1, "new-years-day-2024.pdf @2026-08-11T16:57:16Z"),
        // 2024-03-29 - T2 - CME-SVC-2024-03-28 - closed: no trade date.
        (2024, 3, 29, Closed, T2, "CME-SVC-2024-03-28"),
        // 2024-11-29 - T2 - CME-SVC-2024-11-27 - early close 12:45 CT.
        (2024, 11, 29, early_close(12 * 3_600 + 45 * 60), T2, "CME-SVC-2024-11-27"),
        // 2024-12-24 - T2 - CME-SVC-2024-12-24 - early close 12:45 CT.
        (2024, 12, 24, early_close(12 * 3_600 + 45 * 60), T2, "CME-SVC-2024-12-24"),
        // 2024-12-25 - T2 - CME-SVC-2024-12-24 - closed: no trade date.
        (2024, 12, 25, Closed, T2, "CME-SVC-2024-12-24"),        // 2025-01-01 — T2 — CME-SVC-2024-12-31 — New Year's Day; only a 16:00 CT
        // pre-open and a 17:00 CT open, both carrying trade date 2025-01-02.
        (2025, 1, 1, HolidayKind::Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-21 — T2 — CME-SVC-2025-01-19 — Martin Luther King Jr. Day; the
        // holiday publishes no final close of its own, so the span from Sunday
        // evening through Tuesday 16:00 CT carries this trade date.
        (
            2025,
            1,
            21,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-01-19"
        ),
        // 2025-02-18 — T2 — CME-SVC-2025-02-16 — Presidents' Day; as 2025-01-21.
        (
            2025,
            2,
            18,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-02-16"
        ),
        // 2025-04-18 — T2 — CME-SVC-2025-04-17 — Good Friday; no events published.
        (2025, 4, 18, HolidayKind::Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-27 — T2 — CME-SVC-2025-05-25 — Memorial Day; as 2025-01-21.
        (
            2025,
            5,
            27,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-05-25"
        ),
        // 2025-06-20 — T2 — CME-SVC-2025-06-18 — Juneteenth observed on the
        // Thursday, so the merged span opens on the Wednesday evening and its
        // offset -2 queue prints the operator's ordinary 16:45 CT.
        (
            2025,
            6,
            20,
            ReplacementBlocks(&MERGED_SESSION_AFTER_WEEKDAY_BLOCKS),
            T2,
            "CME-SVC-2025-06-18"
        ),
        // 2025-07-04 — T2 — CME-SVC-2025-07-03 — Independence Day; 12:00 CT final
        // close on its own trade date.
        (2025, 7, 4, early_close(12 * 3_600), T2, "CME-SVC-2025-07-03"),
        // 2025-09-02 — T2 — CME-SVC-2025-08-31 — Labor Day; as 2025-01-21.
        (
            2025,
            9,
            2,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-08-31"
        ),
        // 2025-11-28 — T2 — CME-SVC-2025-11-26 — day after Thanksgiving; the
        // Thursday holiday publishes no final close, so this trade date owns the
        // span from Wednesday evening, pauses at 07:00 CT and ends at 13:45 CT.
        (
            2025,
            11,
            28,
            ReplacementBlocks(&MERGED_THANKSGIVING_FRIDAY_BLOCKS),
            T2,
            "CME-SVC-2025-11-26"
        ),
        // 2025-11-29 — T2 — CME-SVC-2025-11-26-SAT — Thanksgiving Saturday; the
        // service publishes an empty schedule for all ten products.
        (2025, 11, 29, HolidayKind::Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-12-24 — T2 — CME-SVC-2025-12-24 — Christmas Eve; 12:45 CT final
        // close on its own trade date and no evening re-open.
        (
            2025,
            12,
            24,
            early_close(12 * 3_600 + 45 * 60),
            T2,
            "CME-SVC-2025-12-24"
        ),
        // 2025-12-25 — T2 — CME-SVC-2025-12-24 — Christmas Day; only a 16:00 CT
        // pre-open and a 17:00 CT open, both carrying trade date 2025-12-26.
        (2025, 12, 25, HolidayKind::Closed, T2, "CME-SVC-2025-12-24"),
        // 2026-01-01 — T2 — CME-SVC-2025-12-31 — New Year's Day; trade date
        // 2026-01-02 throughout.
        (2026, 1, 1, HolidayKind::Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-01-20 — T2 — CME-SVC-2026-01-18 — Martin Luther King Jr. Day; as
        // 2025-01-21.
        (
            2026,
            1,
            20,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-01-18"
        ),
        // 2026-02-17 — T2 — CME-SVC-2026-02-15 — Presidents' Day; as 2025-01-21.
        (
            2026,
            2,
            17,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-02-15"
        ),
        // 2026-04-03 — T2 — CME-SVC-2026-04-01 — Good Friday, the employment-report
        // exception; 10:15 CT final close on its own trade date.
        (
            2026,
            4,
            3,
            early_close(10 * 3_600 + 15 * 60),
            T2,
            "CME-SVC-2026-04-01"
        ),
        // 2026-05-26 — T2 — CME-SVC-2026-05-24 — Memorial Day; as 2025-01-21.
        (
            2026,
            5,
            26,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-05-24"
        ),
        // 2026-06-19 — T2 — CME-SVC-2026-06-18 — Juneteenth, first of the 24/7-era
        // Friday holidays; the 16:00 CT close carries trade date 2026-06-22.
        (2026, 6, 19, HolidayKind::Closed, T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 — T2 — CME-SVC-2026-07-03 — Independence Day observed; the
        // 16:00 CT close carries trade date 2026-07-06.
        (2026, 7, 3, HolidayKind::Closed, T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 — T2 — CME-SVC-2026-09-06 — Labor Day; the 16:00 CT close is
        // omitted and the re-open carries trade date 2026-09-08.
        (2026, 9, 7, HolidayKind::Closed, T2, "CME-SVC-2026-09-06"),
        // 2026-11-26 — T2 — CME-SVC-2026-11-25 — Thanksgiving; the 16:00 CT close
        // is omitted and the re-open carries trade date 2026-11-27.
        (2026, 11, 26, HolidayKind::Closed, T2, "CME-SVC-2026-11-25"),
        // 2026-12-25 — T2 — CME-SVC-2026-12-24 — Christmas Day; the 16:00 CT close
        // carries trade date 2026-12-28.
        (2026, 12, 25, HolidayKind::Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 — T2 — CME-SVC-2026-12-31 — New Year's Day; the 16:00 CT close
        // carries trade date 2027-01-04.
        (2027, 1, 1, HolidayKind::Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-01-18 — T2 — CME-SVC-2027-01-17 — Martin Luther King Jr. Day; the
        // 16:00 CT close is omitted and the re-open carries trade date 2027-01-19.
        (2027, 1, 18, HolidayKind::Closed, T2, "CME-SVC-2027-01-17"),
        // 2027-02-15 — T2 — CME-SVC-2027-02-14 — Presidents' Day; the re-open
        // carries trade date 2027-02-16.
        (2027, 2, 15, HolidayKind::Closed, T2, "CME-SVC-2027-02-14"),
        // 2027-03-26 — T2 — CME-SVC-2027-03-25 — Good Friday; the 16:00 CT close
        // carries trade date 2027-03-29.
        (2027, 3, 26, HolidayKind::Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 — T2 — CME-SVC-2027-05-30 — Memorial Day; the re-open carries
        // trade date 2027-06-01.
        (2027, 5, 31, HolidayKind::Closed, T2, "CME-SVC-2027-05-30"),
        // 2027-06-18 — T2 — CME-SVC-2027-06-17 — Juneteenth observed; the 16:00 CT
        // close carries trade date 2027-06-21.
        (2027, 6, 18, HolidayKind::Closed, T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 — T2 — CME-SVC-2027-07-04 — Independence Day observed; the
        // re-open carries trade date 2027-07-06.
        (2027, 7, 5, HolidayKind::Closed, T2, "CME-SVC-2027-07-04"),
        // 2027-09-06 — T2 — CME-SVC-2027-09-05 — Labor Day; the re-open carries
        // trade date 2027-09-07.
        (2027, 9, 6, HolidayKind::Closed, T2, "CME-SVC-2027-09-05"),
        // 2027-11-25 — T2 — CME-SVC-2027-11-24 — Thanksgiving; the re-open carries
        // trade date 2027-11-26.
        (2027, 11, 25, HolidayKind::Closed, T2, "CME-SVC-2027-11-24"),
        // 2027-12-24 — T2 — CME-SVC-2027-12-22 — Globex closed for Christmas; the
        // 2027-12-23 re-open carries trade date 2027-12-27, skipping this date.
        (2027, 12, 24, HolidayKind::Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
