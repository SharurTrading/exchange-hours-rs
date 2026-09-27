// SPDX-License-Identifier: MIT-0

//! CME standard-grid FX futures holiday rows, 2025-2027 (LAW-HOLIDAY-SCOPE).
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
//! The 2025-2027 rows come from CME's trading-hours service — the endpoint
//! `cmegroup.com/trading-hours.html` itself calls — read as bytes and saved,
//! so those rows are **T2** under LAW-PRIMARY-SOURCES. CME publishes no T1
//! per-asset-class rendering for them; that, the eight 2025 windows that
//! survive only in a pre-holiday capture, the three Saturday sessions CME
//! states in this window — each a `ReplacementBlocks` row carrying the
//! following Monday's trade date — and the seventeen dates on which CME merges
//! the holiday into the next business day's trade date, each also stated by a
//! `ReplacementBlocks` row, are recorded in
//! [`docs/evidence/globex_fx.md`](../../../../../docs/evidence/globex_fx.md).
//!
//! Three shapes: `Closed` on a full Globex closure, `EarlyClose` on the
//! half-days CME publishes for the family, and `ReplacementBlocks` on the three
//! trade dates of 2026-06-22, 2026-07-06 and 2027-06-21, whose operator day
//! carries a Saturday session, and on the seventeen merged trade dates the
//! evidence file's year tables list. The 2025-2027 window has **no** late
//! open — CME never reopens this family after a closure there other than at its
//! normal 17:00 CT — and it carries no `Unsourced` row, because every date
//! inside its own coverage is answered by CME's own published schedule.
//!
//! # What does not ship a row
//!
//! A Monday or Thursday holiday whose span merges ships no row on its own
//! date: CME publishes `16:00 preopen; 17:00 open` for this family instead of
//! the normal `16:00 closed; 16:45 preopen; 17:00 open`, with both events
//! carrying the next business day's trade date, so the following trade date's
//! `ReplacementBlocks` row states it. Matching still stops at 16:00 CT and
//! still resumes at 17:00 CT, so **no executable phase moves**; what changes
//! is that the holiday has no final close of its own and the whole span
//! carries the next business day's trade date, and that the queue opens 45
//! minutes early. The merged rows above state both, their `-1` order-entry
//! block being that `16:00`-`17:00` CT queue; the three Saturday-session trade
//! dates state the operator's own Sunday queue value too (`16:00 preopen`
//! where the dated profile in force carries 16:15 CT for the two 2026 dates).

use super::fences::early_close;
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
/// - offset `-4`, Thursday 17:00 CT to Friday 12:00 CT: the session that ends
///   at the Friday holiday's early close.
/// - offset `-2`, Saturday 05:00-17:00 CT: the session itself.
/// - offset `-1`, Sunday 16:00-17:00 CT: the Pre-Open queue, which no trade
///   matches. The operator's Sunday window prints `16:00 preopen` for all three
///   dates, so the row states that value and not the 16:15 CT the dated profile
///   in force carries for the two 2026 dates.
/// - offset `-1`, Sunday 17:00 CT to Monday 16:00 CT: the matching session,
///   wrapping one local midnight.
///
/// Evidence: `docs/evidence/globex_fx.md`.
pub(crate) static SATURDAY_SESSION_BLOCKS: [ExceptionBlock; 5] = [
    ExceptionBlock::order_entry(-4, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-4, 17 * 3_600, 12 * 3_600),
    ExceptionBlock::extended(-2, 5 * 3_600, 17 * 3_600),
    ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 16 * 3_600),
];

/// The complete trading day of the trade dates CME merges with the session
/// before them, on a Monday or Thursday holiday.
///
/// The holiday publishes no final close of its own: the queue and evening
/// session that would carry it are printed against the next business day
/// instead, so the whole span Sunday-evening-or-eve through the following
/// afternoon carries one trade date. A `Closed` row would instead delete a full
/// evening and day of trading the operator ran, which is why these dates ship no
/// row at all before this one.
///
/// - offset `-2`, 16:00-17:00 CT: the Pre-Open queue. `16:00` because the `-2`
///   day is a Sunday, which is this family's published Sunday onset.
/// - offset `-2`, 17:00 CT to offset `-1` 16:00 CT: the matching session.
/// - offset `-1`, 16:00-17:00 CT: the holiday's own Pre-Open queue. CME
///   publishes `16:00 preopen` for this family on the holiday, not the ordinary
///   weekday `16:45`.
/// - offset `-1`, 17:00 CT to trade date 16:00 CT: the matching session.
///
/// Evidence: `docs/evidence/globex_fx.md`.
pub(crate) static MERGED_SESSION_BLOCKS: [ExceptionBlock; 4] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 16 * 3_600),
    ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 16 * 3_600),
];

/// The same merged day where the `-2` day is an ordinary weekday rather than a
/// Sunday, so its queue opens at the family's weekday `16:45` CT.
///
/// The Juneteenth Thursday 2025-06-19 merge is the 2025 case: the span runs from
/// Wednesday evening, not Sunday evening.
///
/// Evidence: `docs/evidence/globex_fx.md`.
pub(crate) static MERGED_SESSION_AFTER_WEEKDAY_BLOCKS: [ExceptionBlock; 4] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 16 * 3_600),
    ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 16 * 3_600),
];

/// The same merged day, for the one trade date where the operator publishes a
/// second Pre-Open: the day-after-Thanksgiving Friday 2025-11-28.
///
/// The finalised publication states `07:00 preopen`, `07:30 open` and
/// `13:45 closed` on eventDate 2025-11-28 with trade date 2025-11-28, so the
/// trade date's matching runs in two pieces with a 30-minute order-entry queue
/// between them. Carried as one continuous `extended` run the queue answered
/// `is_open = true`, a window CME defines as "No order matching", so the queue
/// is an `order_entry` block and matching resumes at its `07:30 open`.
///
/// The 2026 and 2027 Thanksgiving Fridays publish the `13:45 closed` line alone
/// and keep [`MERGED_SESSION_EARLY_CLOSE_BLOCKS`], which this row would
/// otherwise overstate.
///
/// - offset `-2`, 16:45-17:00 CT: the Wednesday Pre-Open queue.
/// - offset `-2`, 17:00 CT to offset `-1` 16:00 CT: the matching session.
/// - offset `-1`, 16:00-17:00 CT: the holiday's own Pre-Open queue.
/// - offset `-1`, 17:00 CT to the trade date's 13:45 CT close: the matching run,
///   split at local midnight into the two pieces below so the trade date's own
///   07:00 queue can be stated without overlapping it. The block list is
///   non-decreasing by opening day then open time, which the table fence
///   enforces at compile time.
/// - offset `0`, 00:00-07:00 CT: the tail of that overnight run. It restates the
///   wrapped block's own span, which is what keeps the run's envelope whole.
/// - offset `0`, 07:00-07:30 CT: the trade date's morning Pre-Open queue.
/// - offset `0`, 07:30 CT to the trade date's 13:45 CT close: the day session.
///
/// Evidence: `docs/evidence/globex_fx.md`.
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
    ExceptionBlock::extended(-2, 17 * 3_600, 16 * 3_600),
    ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 24 * 3_600),
    ExceptionBlock::extended(0, 0 * 3_600 + 0 * 60, 7 * 3_600),
    ExceptionBlock::order_entry(0, 7 * 3_600, 7 * 3_600 + 30 * 60),
    ExceptionBlock::extended(0, 7 * 3_600 + 30 * 60, 13 * 3_600 + 45 * 60),
];

/// The same merged day when the trade date itself is a day-after-Thanksgiving
/// Friday, whose session the operator ends at `13:45` CT.
///
/// Evidence: `docs/evidence/globex_fx.md`.
pub(crate) static MERGED_SESSION_EARLY_CLOSE_BLOCKS: [ExceptionBlock; 4] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 16 * 3_600),
    ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 13 * 3_600 + 45 * 60),
];
use super::{
    EvidenceTier::T2,
    HolidayKind::{Closed, ReplacementBlocks},
    HolidayTable, holidays,
};

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
// Evidence: docs/evidence/globex_fx.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-21 - T2 - CME-SVC-2025-01-19 - Martin Luther King Day; the
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
        // 2025-02-18 - T2 - CME-SVC-2025-02-16 - Presidents Day; as 2025-01-21.
        (
            2025,
            2,
            18,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-02-16"
        ),
        // 2025-04-18 - T2 - CME-SVC-2025-04-17 - Good Friday, no events published.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-27 - T2 - CME-SVC-2025-05-25 - Memorial Day; as 2025-01-21.
        (
            2025,
            5,
            27,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-05-25"
        ),
        // 2025-06-20 - T2 - CME-SVC-2025-06-18 - Juneteenth observed on the
        // Thursday, so the merged span opens on Wednesday evening and this row
        // carries the family's ordinary weekday 16:45 CT queue at offset -2.
        (
            2025,
            6,
            20,
            ReplacementBlocks(&MERGED_SESSION_AFTER_WEEKDAY_BLOCKS),
            T2,
            "CME-SVC-2025-06-18"
        ),
        // 2025-07-04 - T2 - CME-SVC-2025-07-03 - Independence Day, 12:00 CT close.
        (2025, 7, 4, early_close(12 * 3_600), T2, "CME-SVC-2025-07-03"),
        // 2025-09-02 - T2 - CME-SVC-2025-08-31 - Labor Day; as 2025-01-21.
        (
            2025,
            9,
            2,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-08-31"
        ),
        // 2025-11-28 - T2 - CME-SVC-2025-11-26 - day after Thanksgiving. The
        // Thursday holiday publishes no final close, so this trade date owns the
        // span from Wednesday evening and ends at the operator's 13:45 CT close.
        (
            2025,
            11,
            28,
            ReplacementBlocks(&MERGED_SESSION_EARLY_CLOSE_BLOCKS_2025_11_28),
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
        // 2026-01-20 - T2 - CME-SVC-2026-01-18 - Martin Luther King Day; the
        // holiday publishes no final close of its own, so the span from Sunday
        // evening through Tuesday 16:00 CT carries this trade date.
        (
            2026,
            1,
            20,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-01-18"
        ),
        // 2026-02-17 - T2 - CME-SVC-2026-02-15 - Presidents Day; as 2026-01-20.
        (
            2026,
            2,
            17,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-02-15"
        ),
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
        // 2026-05-26 - T2 - CME-SVC-2026-05-24 - Memorial Day; as 2026-01-20.
        (
            2026,
            5,
            26,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-05-24"
        ),
        // 2026-06-19 - T2 - CME-SVC-2026-06-18 - Juneteenth, 12:00 CT close.
        (2026, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-2026-06-18"),
        (2026, 6, 22, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - Independence Day observed, 12:00 CT close.
        (2026, 7, 3, early_close(12 * 3_600), T2, "CME-SVC-2026-07-03"),
        (2026, 7, 6, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2026-07-03"),
        // 2026-09-08 - T2 - CME-SVC-2026-09-06 - Labor Day; as 2026-01-20.
        (
            2026,
            9,
            8,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-09-06"
        ),
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - day after Thanksgiving. The
        // Thursday holiday publishes no final close, so this trade date owns the
        // span from Wednesday evening and ends at the operator's 13:45 CT close.
        (
            2026,
            11,
            27,
            ReplacementBlocks(&MERGED_SESSION_EARLY_CLOSE_BLOCKS),
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
        // 2027-01-19 - T2 - CME-SVC-2027-01-17 - Martin Luther King Day; the
        // holiday publishes no final close of its own, so the span from Sunday
        // evening through Tuesday 16:00 CT carries this trade date.
        (
            2027,
            1,
            19,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-01-17"
        ),
        // 2027-02-16 - T2 - CME-SVC-2027-02-14 - Presidents Day; as 2027-01-19.
        (
            2027,
            2,
            16,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-02-14"
        ),
        // 2027-03-26 - T2 - CME-SVC-2027-03-25 - Good Friday, no events published.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-06-01 - T2 - CME-SVC-2027-05-30 - Memorial Day; as 2027-01-19.
        (
            2027,
            6,
            1,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-05-30"
        ),
        // 2027-06-18 - T2 - CME-SVC-2027-06-17 - Juneteenth observed, 12:00 CT close.
        (2027, 6, 18, early_close(12 * 3_600), T2, "CME-SVC-2027-06-17"),
        (2027, 6, 21, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2027-06-17"),
        // 2027-07-06 - T2 - CME-SVC-2027-07-04 - Independence Day observed on the
        // Monday; as 2027-01-19.
        (
            2027,
            7,
            6,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-07-04"
        ),
        // 2027-09-07 - T2 - CME-SVC-2027-09-05 - Labor Day; as 2027-01-19.
        (
            2027,
            9,
            7,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-09-05"
        ),
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - day after Thanksgiving. The
        // Thursday holiday publishes no final close, so this trade date owns the
        // span from Wednesday evening and ends at the operator's 13:45 CT close.
        (
            2027,
            11,
            26,
            ReplacementBlocks(&MERGED_SESSION_EARLY_CLOSE_BLOCKS),
            T2,
            "CME-SVC-2027-11-24"
        ),
        // 2027-12-24 - T2 - CME-SVC-2027-12-22 - Christmas Friday closure, no events.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
