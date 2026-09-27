// SPDX-License-Identifier: MIT-0

//! Built-in holiday rows for `globex_interest_rates`, venue-local trade dates
//! 2025-01-01 .. 2027-12-31 (LAW-HOLIDAY-SCOPE).
//!
//! Every row is keyed by the crate's own venue-local trade date in
//! `America/Chicago`, converted from the operator's event date: CME publishes
//! a Monday holiday's noon halt on the Monday, and the trading day it shortens
//! is the one that opened at 17:00 CT the previous evening, so the row lands on
//! the trade date the clip has to be stated on.
//!
//! **2025-2027.** Those rows are the CME Group trading-hours service's Interest
//! Rates line — the ZN 10-Year T-Note schedule CME itself uses to render that
//! group — at tier **T2**. The family's grid in that window is one wrapping leg
//! per trade date, Sunday to Thursday 17:00 CT into a 16:00 CT close the next
//! local day, so a full closure deletes the previous evening's wrap and an
//! early close clips a session that opened the evening before. Both follow from
//! the trade-date key; neither needs a mechanism of its own.
//!
//! Quotations, document URLs, capture timestamps, the event-date to trade-date
//! conversion and the declared gaps live in
//! `docs/evidence/globex_interest_rates.md`.

use super::EvidenceTier::T2;
use super::HolidayKind::{Closed, ReplacementBlocks};
use super::fences::early_close;
use super::{HolidayTable, holidays};
/// The complete trading day of a merged trade date whose holiday closes at
/// `12:00` CT and whose `-2` day is a Sunday.
///
/// The holiday publishes no final close for its own trade date, so the span from
/// Sunday evening carries the next business day's date — but it still ends where
/// the crate's own holiday row ends it, `12:00`, and the queue opens there.
///
/// Evidence: `docs/evidence/globex_interest_rates.md`.
pub(crate) static MERGED_SESSION_BLOCKS: [ExceptionBlock; 4] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 12 * 3_600),
    ExceptionBlock::order_entry(-1, 12 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 16 * 3_600),
];

/// The same, for the 2027-07-06 trade date only: its holiday is the July 4
/// observed Monday, where CME publishes `13:30` rather than the `12:00` every
/// other Monday holiday carries. Read the value off the shipped holiday row, not
/// off any per-family summary.
///
/// Evidence: `docs/evidence/globex_interest_rates.md`.
pub(crate) static MERGED_SESSION_JULY4_BLOCKS: [ExceptionBlock; 4] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 13 * 3_600 + 30 * 60),
    ExceptionBlock::order_entry(-1, 13 * 3_600 + 30 * 60, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 16 * 3_600),
];

/// The same merged day where the `-2` day is an ordinary weekday rather than a
/// Sunday, so its queue opens at the family's weekday `16:45` CT. The Juneteenth
/// Thursday 2025-06-19 merge is the case.
///
/// Evidence: `docs/evidence/globex_interest_rates.md`.
pub(crate) static MERGED_SESSION_AFTER_WEEKDAY_BLOCKS: [ExceptionBlock; 4] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 12 * 3_600),
    ExceptionBlock::order_entry(-1, 12 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 16 * 3_600),
];

/// The same merged day, for the one trade date where the operator publishes a
/// second Pre-Open: the day-after-Thanksgiving Friday 2025-11-28.
///
/// The finalised publication states `07:00 preopen`, `07:30 open` and
/// `12:15 closed` on eventDate 2025-11-28 with trade date 2025-11-28, so the
/// trade date's matching runs in two pieces with a 30-minute order-entry queue
/// between them. Carried as one continuous `extended` run the queue answered
/// `is_open = true`, a window CME defines as "No order matching", so the queue
/// is an `order_entry` block and matching resumes at its `07:30 open`.
///
/// The 2026 and 2027 Thanksgiving Fridays publish the `12:15 closed` line alone
/// and keep [`MERGED_SESSION_EARLY_CLOSE_BLOCKS`], which this row would
/// otherwise overstate.
///
/// - offset `-2`, 16:45-17:00 CT: the Wednesday Pre-Open queue.
/// - offset `-2`, 17:00 CT to offset `-1` 12:00 CT: the matching session.
/// - offset `-1`, 12:00-17:00 CT: the holiday's own Pre-Open queue.
/// - offset `-1`, 17:00 CT to the trade date's 12:15 CT close: the matching run,
///   split at local midnight into the two pieces below so the trade date's own
///   07:00 queue can be stated without overlapping it. The block list is
///   non-decreasing by opening day then open time, which the table fence
///   enforces at compile time.
/// - offset `0`, 00:00-07:00 CT: the tail of that overnight run. It restates the
///   wrapped block's own span, which is what keeps the run's envelope whole.
/// - offset `0`, 07:00-07:30 CT: the trade date's morning Pre-Open queue.
/// - offset `0`, 07:30 CT to the trade date's 12:15 CT close: the day session.
///
/// Evidence: `docs/evidence/globex_interest_rates.md`.
// `0 * 3_600 + 0 * 60` is midnight, written in the table fence's own grammar
// (`h * 3_600 + m * 60`); a bare `0` is rejected by
// `tests/schedule_documentation/evidence_files.rs`, so the zero product is
// deliberate and the lint is wrong here.
#[expect(
    clippy::erasing_op,
    clippy::identity_op,
    reason = "the table fence reads every block instant as `h * 3_600 + m * 60`; \
              midnight has to be written in that grammar"
)]
pub(crate) static MERGED_SESSION_EARLY_CLOSE_BLOCKS_2025_11_28: [ExceptionBlock; 7] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 12 * 3_600),
    ExceptionBlock::order_entry(-1, 12 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 24 * 3_600),
    ExceptionBlock::extended(0, 0 * 3_600 + 0 * 60, 7 * 3_600),
    ExceptionBlock::order_entry(0, 7 * 3_600, 7 * 3_600 + 30 * 60),
    ExceptionBlock::extended(0, 7 * 3_600 + 30 * 60, 12 * 3_600 + 15 * 60),
];

/// The same merged day when the trade date is a day-after-Thanksgiving Friday,
/// whose session this family ends at `12:15` CT.
///
/// Evidence: `docs/evidence/globex_interest_rates.md`.
pub(crate) static MERGED_SESSION_EARLY_CLOSE_BLOCKS: [ExceptionBlock; 4] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 12 * 3_600),
    ExceptionBlock::order_entry(-1, 12 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 12 * 3_600 + 15 * 60),
];
use crate::calendar::exceptions::ExceptionBlock;
/// The complete trading day of the 2026-06-22, 2026-07-06 and 2027-06-21 trade
/// dates, which CME states a Saturday session on.
///
/// CME published all five of the trading day's phases for each of these dates,
/// so the row states the day rather than the Saturday alone. Stating only the
/// Saturday would delete the Thursday-evening and Sunday-evening sessions that
/// belong to the same trade date, because a replacement row replaces the
/// **complete** trade date.
///
/// - offset `-4`, Thursday 16:45-17:00 CT: the Pre-Open queue.
/// - offset `-4`, Thursday 17:00 CT to Friday 12:00 CT: the session ending at
///   the Friday holiday's early close.
/// - offset `-2`, Saturday 05:00-17:00 CT: the session itself.
/// - offset `-1`, Sunday 16:00-17:00 CT: the Pre-Open queue, which no trade
///   matches. The operator publishes 16:00 for these dates; the normal week's
///   own Sunday queue is what the profile states, and the row states what was
///   published.
/// - offset `-1`, Sunday 17:00 CT to Monday 16:00 CT: the matching session,
///   wrapping one local midnight.
///
/// Evidence: `docs/evidence/globex_interest_rates.md`.
pub(crate) static SATURDAY_SESSION_BLOCKS: [ExceptionBlock; 5] = [
    ExceptionBlock::order_entry(-4, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-4, 17 * 3_600, 12 * 3_600),
    ExceptionBlock::extended(-2, 5 * 3_600, 17 * 3_600),
    ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 16 * 3_600),
];

/// The family's built-in holiday rows and the windows they were audited over.
///
///  One audited era: 2025-2027 at T2, whose window opens at the crate's
/// permanent 2025 support floor (LAW-COVERAGE). The eras before it left in
/// Stage 5 of the release plan (#117), so nothing before 2025-01-01 has a
/// row or a window: that span lies outside the window, so
/// `holiday_on` has no answer there rather than reporting a normal date.
///
/// Coverage ends at
/// 2027-12-31, the end of the operator's published future, and CME's 2028-01-01
/// record sits outside every window and ships no row. Inside a window a date
/// with no row is audited normal, except where an `Unsourced` row marks the
/// operator's silence instead.
// Evidence: docs/evidence/globex_interest_rates.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 — T2 — CME-SVC-2025-01-19 — Martin Luther King Jr. Day:
        // matching halts 12:00 CT.
        (2025, 1, 20, early_close(12 * 3_600), T2, "CME-SVC-2025-01-19"),
        // 2025-01-21 - T2 - CME-SVC-2025-01-19 - MLK Day; the holiday publishes no final close, so the span from Sunday evening carries this trade date.
        (
            2025,
            1,
            21,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-01-19"
        ),
        // 2025-02-17 — T2 — CME-SVC-2025-02-16 — Presidents' Day: 12:00 CT.
        (2025, 2, 17, early_close(12 * 3_600), T2, "CME-SVC-2025-02-16"),
        // 2025-02-18 - T2 - CME-SVC-2025-02-16 - Presidents Day; as 2025-01-21.
        (
            2025,
            2,
            18,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-02-16"
        ),
        // 2025-04-18 — T2 — CME-SVC-2025-04-17 — Good Friday: no events published.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 — T2 — CME-SVC-2025-05-25 — Memorial Day: 12:00 CT.
        (2025, 5, 26, early_close(12 * 3_600), T2, "CME-SVC-2025-05-25"),
        // 2025-05-27 - T2 - CME-SVC-2025-05-25 - Memorial Day; as 2025-01-21.
        (
            2025,
            5,
            27,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-05-25"
        ),
        // 2025-06-19 — T2 — CME-SVC-2025-06-18 — Juneteenth: 12:00 CT.
        (2025, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-2025-06-18"),
        // 2025-06-20 - T2 - CME-SVC-2025-06-18 - Juneteenth falls on the Thursday, so the span opens Wednesday evening and its -2 queue is the weekday 16:45.
        (
            2025,
            6,
            20,
            ReplacementBlocks(&MERGED_SESSION_AFTER_WEEKDAY_BLOCKS),
            T2,
            "CME-SVC-2025-06-18"
        ),
        // 2025-07-04 — T2 — CME-SVC-2025-07-03 — Independence Day: 12:00 CT final
        // close on its own trade date.
        (2025, 7, 4, early_close(12 * 3_600), T2, "CME-SVC-2025-07-03"),
        // 2025-09-01 — T2 — CME-SVC-2025-08-31 — Labor Day: 12:00 CT.
        (2025, 9, 1, early_close(12 * 3_600), T2, "CME-SVC-2025-08-31"),
        // 2025-09-02 - T2 - CME-SVC-2025-08-31 - Labor Day; as 2025-01-21.
        (
            2025,
            9,
            2,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-08-31"
        ),
        // 2025-11-27 — T2 — CME-SVC-2025-11-26 — Thanksgiving: 12:00 CT.
        (2025, 11, 27, early_close(12 * 3_600), T2, "CME-SVC-2025-11-26"),
        // 2025-11-28 — T2 — CME-SVC-2025-11-26 — day after Thanksgiving: 12:15 CT
        // final close on its own trade date.
        // 2025-11-28 - T2 - CME-SVC-2025-11-26 - day after Thanksgiving. The Thursday holiday publishes no final close, so this trade date owns the span from Wednesday evening and ends at this family's 12:15 CT close.
        (
            2025,
            11,
            28,
            ReplacementBlocks(&MERGED_SESSION_EARLY_CLOSE_BLOCKS_2025_11_28),
            T2,
            "CME-SVC-2025-11-26"
        ),
        // 2025-11-29 — T2 — CME-SVC-2025-11-26-SAT — Thanksgiving Saturday: no events
        // published, and the normal week has no Saturday session either.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-12-24 — T2 — CME-SVC-2025-12-24 — Christmas Eve: 12:15 CT final
        // close, and no evening re-open because 2025-12-25 is closed.
        (2025, 12, 24, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-2025-12-24"),
        // 2025-12-25 — T2 — CME-SVC-2025-12-24 — Christmas Day: only a 16:00 CT
        // pre-open and a 17:00 CT open, both carrying trade date 2025-12-26.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-24"),
        // 2026-01-01 — T2 — CME-SVC-2025-12-31 — New Year's Day: only a 16:00 CT
        // pre-open and a 17:00 CT open, both carrying trade date 2026-01-02.
        (2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-01-19 — T2 — CME-SVC-2026-01-18 — Martin Luther King Jr. Day: 12:00 CT.
        (2026, 1, 19, early_close(12 * 3_600), T2, "CME-SVC-2026-01-18"),
        // 2026-01-20 - T2 - CME-SVC-2026-01-18 - MLK Day; as 2025-01-21.
        (
            2026,
            1,
            20,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-01-18"
        ),
        // 2026-02-16 — T2 — CME-SVC-2026-02-15 — Presidents' Day: 12:00 CT.
        (2026, 2, 16, early_close(12 * 3_600), T2, "CME-SVC-2026-02-15"),
        // 2026-02-17 - T2 - CME-SVC-2026-02-15 - Presidents Day; as 2025-01-21.
        (
            2026,
            2,
            17,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-02-15"
        ),
        // 2026-04-03 — T2 — CME-SVC-2026-04-01 — Good Friday, the year CME keeps
        // rates trading for the employment release: 10:15 CT final close.
        (2026, 4, 3, early_close(10 * 3_600 + 15 * 60), T2, "CME-SVC-2026-04-01"),
        // 2026-05-25 — T2 — CME-SVC-2026-05-24 — Memorial Day: 12:00 CT.
        (2026, 5, 25, early_close(12 * 3_600), T2, "CME-SVC-2026-05-24"),
        // 2026-05-26 - T2 - CME-SVC-2026-05-24 - Memorial Day; as 2025-01-21.
        (
            2026,
            5,
            26,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-05-24"
        ),
        // 2026-06-19 — T2 — CME-SVC-2026-06-18 — Juneteenth: 12:00 CT final close.
        (2026, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-2026-06-18"),
        (2026, 6, 22, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 — T2 — CME-SVC-2026-07-03 — Independence Day observed: 12:00 CT
        // final close.
        (2026, 7, 3, early_close(12 * 3_600), T2, "CME-SVC-2026-07-03"),
        (2026, 7, 6, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 — T2 — CME-SVC-2026-09-06 — Labor Day: 12:00 CT.
        (2026, 9, 7, early_close(12 * 3_600), T2, "CME-SVC-2026-09-06"),
        // 2026-09-08 - T2 - CME-SVC-2026-09-06 - Labor Day; as 2025-01-21.
        (
            2026,
            9,
            8,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-09-06"
        ),
        // 2026-11-26 — T2 — CME-SVC-2026-11-25 — Thanksgiving: 12:00 CT.
        (2026, 11, 26, early_close(12 * 3_600), T2, "CME-SVC-2026-11-25"),
        // 2026-11-27 — T2 — CME-SVC-2026-11-25 — day after Thanksgiving: 12:15 CT.
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - day after Thanksgiving; as 2025-11-28.
        (
            2026,
            11,
            27,
            ReplacementBlocks(&MERGED_SESSION_EARLY_CLOSE_BLOCKS),
            T2,
            "CME-SVC-2026-11-25"
        ),
        // 2026-12-24 — T2 — CME-SVC-2026-12-22 — Christmas Eve: 12:15 CT final
        // close, and no evening re-open because 2026-12-25 is closed.
        (2026, 12, 24, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-2026-12-22"),
        // 2026-12-25 — T2 — CME-SVC-2026-12-24 — Christmas Day: no events published.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 — T2 — CME-SVC-2026-12-31 — New Year's Day: no events published.
        (2027, 1, 1, Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-01-18 — T2 — CME-SVC-2027-01-17 — Martin Luther King Jr. Day: 12:00 CT.
        (2027, 1, 18, early_close(12 * 3_600), T2, "CME-SVC-2027-01-17"),
        // 2027-01-19 - T2 - CME-SVC-2027-01-17 - MLK Day; as 2025-01-21.
        (
            2027,
            1,
            19,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-01-17"
        ),
        // 2027-02-15 — T2 — CME-SVC-2027-02-14 — Presidents' Day: 12:00 CT.
        (2027, 2, 15, early_close(12 * 3_600), T2, "CME-SVC-2027-02-14"),
        // 2027-02-16 - T2 - CME-SVC-2027-02-14 - Presidents Day; as 2025-01-21.
        (
            2027,
            2,
            16,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-02-14"
        ),
        // 2027-03-26 — T2 — CME-SVC-2027-03-25 — Good Friday: no events published.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 — T2 — CME-SVC-2027-05-30 — Memorial Day: 12:00 CT.
        (2027, 5, 31, early_close(12 * 3_600), T2, "CME-SVC-2027-05-30"),
        // 2027-06-01 - T2 - CME-SVC-2027-05-30 - Memorial Day; as 2025-01-21.
        (
            2027,
            6,
            1,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-05-30"
        ),
        // 2027-06-18 — T2 — CME-SVC-2027-06-17 — Juneteenth observed: 12:00 CT
        // final close.
        (2027, 6, 18, early_close(12 * 3_600), T2, "CME-SVC-2027-06-17"),
        (2027, 6, 21, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 — T2 — CME-SVC-2027-07-04 — Independence Day observed:
        // matching halts 13:30 CT.
        (2027, 7, 5, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-07-04"),
        // 2027-07-06 - T2 - CME-SVC-2027-07-04 - Independence Day observed on the Monday; its holiday close is 13:30, not the 12:00 the other Monday holidays carry.
        (
            2027,
            7,
            6,
            ReplacementBlocks(&MERGED_SESSION_JULY4_BLOCKS),
            T2,
            "CME-SVC-2027-07-04"
        ),
        // 2027-09-06 — T2 — CME-SVC-2027-09-05 — Labor Day: 12:00 CT.
        (2027, 9, 6, early_close(12 * 3_600), T2, "CME-SVC-2027-09-05"),
        // 2027-09-07 - T2 - CME-SVC-2027-09-05 - Labor Day; as 2025-01-21.
        (
            2027,
            9,
            7,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-09-05"
        ),
        // 2027-11-25 — T2 — CME-SVC-2027-11-24 — Thanksgiving: 12:00 CT.
        (2027, 11, 25, early_close(12 * 3_600), T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 — T2 — CME-SVC-2027-11-24 — day after Thanksgiving: 12:15 CT.
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - day after Thanksgiving; as 2025-11-28.
        (
            2027,
            11,
            26,
            ReplacementBlocks(&MERGED_SESSION_EARLY_CLOSE_BLOCKS),
            T2,
            "CME-SVC-2027-11-24"
        ),
        // 2027-12-24 — T2 — CME-SVC-2027-12-22 — Globex closed for the Christmas
        // holiday: no events published.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
