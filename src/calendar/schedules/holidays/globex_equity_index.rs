// SPDX-License-Identifier: MIT-0

//! `globex_equity_index` holiday rows, venue-local trade dates
//! 2025-01-01 .. 2027-12-31 (LAW-HOLIDAY-SCOPE).
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

use super::EvidenceTier::T2;
use super::HolidayKind::{Closed, ReplacementBlocks};
use super::fences::early_close;
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

/// The complete trading day of a merged trade date whose holiday closes at
/// `12:00` CT and whose `-2` day is a Sunday.
///
/// The holiday publishes no final close for its own trade date, so the span from
/// Sunday evening carries the next business day's date — and it still ends where
/// the crate's own holiday row ends it, `12:00`, with the queue opening there.
/// Each envelope is split at the family's regular boundaries, so both the
/// holiday's clipped morning and the trade date's own session keep answering
/// `OpenRegular` where the ordinary week does.
///
/// Evidence: `docs/evidence/globex_equity_index.md`.
pub(crate) static MERGED_SESSION_BLOCKS: [ExceptionBlock; 7] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 8 * 3_600 + 30 * 60),
    ExceptionBlock::regular(-1, 8 * 3_600 + 30 * 60, 12 * 3_600),
    ExceptionBlock::order_entry(-1, 12 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 8 * 3_600 + 30 * 60),
    ExceptionBlock::regular(0, 8 * 3_600 + 30 * 60, 15 * 3_600 + 15 * 60),
    ExceptionBlock::extended(0, 15 * 3_600 + 15 * 60, 16 * 3_600),
];

/// The same, where the `-2` day is an ordinary weekday rather than a Sunday, so
/// its queue opens at the family's weekday `16:45` CT. The Juneteenth Thursday
/// 2025-06-19 merge is the case.
///
/// Evidence: `docs/evidence/globex_equity_index.md`.
pub(crate) static MERGED_SESSION_AFTER_WEEKDAY_BLOCKS: [ExceptionBlock; 7] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 8 * 3_600 + 30 * 60),
    ExceptionBlock::regular(-1, 8 * 3_600 + 30 * 60, 12 * 3_600),
    ExceptionBlock::order_entry(-1, 12 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 8 * 3_600 + 30 * 60),
    ExceptionBlock::regular(0, 8 * 3_600 + 30 * 60, 15 * 3_600 + 15 * 60),
    ExceptionBlock::extended(0, 15 * 3_600 + 15 * 60, 16 * 3_600),
];

/// The same, for the one trade date where the operator publishes a second
/// Pre-Open: the day-after-Thanksgiving Friday 2025-11-28.
///
/// The finalised publication states `07:00 preopen`, `07:30 open` and
/// `12:15 closed` on eventDate 2025-11-28 with trade date 2025-11-28. `07:30` is
/// earlier than the family's `08:30` regular open, so the queue is the overnight
/// session's own `order_entry` window rather than a late open, and matching runs
/// from 07:30 until the regular session takes over at 08:30. Carried as one
/// continuous `extended` run the queue answered `is_open = true`, a window CME
/// defines as "No order matching".
///
/// The 2026 and 2027 Thanksgiving Fridays publish the `12:15 closed` line alone
/// and keep [`MERGED_SESSION_EARLY_CLOSE_BLOCKS`], which this row would
/// otherwise overstate.
///
/// - offset `-2`, 16:45-17:00 CT: the Wednesday Pre-Open queue.
/// - offset `-2`, 17:00 CT to offset `-1` 08:30 CT: the overnight session.
/// - offset `-1`, 08:30-12:00 CT: the holiday's regular session.
/// - offset `-1`, 12:00-17:00 CT: the holiday's own Pre-Open queue.
/// - offset `-1`, 17:00 CT to the trade date's 12:15 CT close: the overnight
///   run, split at local midnight into the two pieces below so the trade date's
///   own 07:00 queue can be stated without overlapping it. The block list is
///   non-decreasing by opening day then open time, which the table fence
///   enforces at compile time.
/// - offset `0`, 00:00-07:00 CT: the tail of that overnight run. It restates the
///   wrapped block's own span, which is what keeps the run's envelope whole.
/// - offset `0`, 07:00-07:30 CT: the trade date's morning Pre-Open queue.
/// - offset `0`, 07:30-08:30 CT: matching, to the family's regular open.
/// - offset `0`, 08:30 CT to the trade date's 12:15 CT close: the regular
///   session that the early close cuts short.
///
/// Evidence: `docs/evidence/globex_equity_index.md`.
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
pub(crate) static MERGED_SESSION_EARLY_CLOSE_BLOCKS_2025_11_28: [ExceptionBlock; 9] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 8 * 3_600 + 30 * 60),
    ExceptionBlock::regular(-1, 8 * 3_600 + 30 * 60, 12 * 3_600),
    ExceptionBlock::order_entry(-1, 12 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 24 * 3_600),
    ExceptionBlock::extended(0, 0 * 3_600 + 0 * 60, 7 * 3_600),
    ExceptionBlock::order_entry(0, 7 * 3_600, 7 * 3_600 + 30 * 60),
    ExceptionBlock::extended(0, 7 * 3_600 + 30 * 60, 8 * 3_600 + 30 * 60),
    ExceptionBlock::regular(0, 8 * 3_600 + 30 * 60, 12 * 3_600 + 15 * 60),
];

/// The same, when the trade date is a day-after-Thanksgiving Friday whose
/// regular session the `12:15` CT close cuts short. Six blocks: the trade date's
/// own envelope ends at that close, so there is no trailing `extended` slice.
///
/// Evidence: `docs/evidence/globex_equity_index.md`.
pub(crate) static MERGED_SESSION_EARLY_CLOSE_BLOCKS: [ExceptionBlock; 6] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 8 * 3_600 + 30 * 60),
    ExceptionBlock::regular(-1, 8 * 3_600 + 30 * 60, 12 * 3_600),
    ExceptionBlock::order_entry(-1, 12 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 8 * 3_600 + 30 * 60),
    ExceptionBlock::regular(0, 8 * 3_600 + 30 * 60, 12 * 3_600 + 15 * 60),
];

/// 12:00 CT, the Monday/Thursday-holiday and Independence-Day final close.
const NOON: u32 = 12 * 3_600;
/// 12:15 CT, the Christmas-Eve and day-after-Thanksgiving final close.
const QUARTER_PAST_NOON: u32 = 12 * 3_600 + 15 * 60;
/// 08:15 CT, the Good Friday 2026 equity-index final close.
const QUARTER_PAST_EIGHT: u32 = 8 * 3_600 + 15 * 60;

/// The family's built-in holiday rows and the windows they were audited over.
///
///  One audited era: 2025-2027 at T2, whose window opens at the crate's
/// permanent 2025 support floor (LAW-COVERAGE). The eras before it left in
/// Stage 5 of the release plan (#117), so nothing before 2025-01-01 has a
/// row or a window: that span lies outside the window, so
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
    coverage: [(2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 — T2 — CME-SVC-2025-01-19 — Martin Luther King Jr. Day:
        // matching stops 12:00 CT, published as a preopen.
        (2025, 1, 20, early_close(NOON), T2, "CME-SVC-2025-01-19"),
        // 2025-01-21 - T2 - CME-SVC-2025-01-19 - MLK Day; the holiday publishes no final close, so the span from Sunday evening carries this trade date, and it still ends at the holiday's 12:00 CT close.
        (
            2025,
            1,
            21,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-01-19"
        ),
        // 2025-02-17 — T2 — CME-SVC-2025-02-16 — Presidents' Day: 12:00 CT.
        (2025, 2, 17, early_close(NOON), T2, "CME-SVC-2025-02-16"),
        // 2025-02-18 - T2 - CME-SVC-2025-02-16 - Presidents Day; as 2025-01-21.
        (
            2025,
            2,
            18,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-02-16"
        ),
        // 2025-04-18 — T2 — CME-SVC-2025-04-17 — Good Friday: full Globex
        // closure, no events published for any family.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 — T2 — CME-SVC-2025-05-25 — Memorial Day: 12:00 CT.
        (2025, 5, 26, early_close(NOON), T2, "CME-SVC-2025-05-25"),
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
        (2025, 6, 19, early_close(NOON), T2, "CME-SVC-2025-06-18"),
        // 2025-06-20 - T2 - CME-SVC-2025-06-18 - Juneteenth falls on the Thursday, so the span opens Wednesday evening and its -2 queue is the weekday 16:45.
        (
            2025,
            6,
            20,
            ReplacementBlocks(&MERGED_SESSION_AFTER_WEEKDAY_BLOCKS),
            T2,
            "CME-SVC-2025-06-18"
        ),
        // 2025-07-03 — T2 — CME-SVC-2025-07-03 — Independence Day eve: equity
        // index alone closes 12:15 CT; the evening leg then runs normally.
        (2025, 7, 3, early_close(QUARTER_PAST_NOON), T2, "CME-SVC-2025-07-03"),
        // 2025-07-04 — T2 — CME-SVC-2025-07-03 — Independence Day: 12:00 CT.
        (2025, 7, 4, early_close(NOON), T2, "CME-SVC-2025-07-03"),
        // 2025-09-01 — T2 — CME-SVC-2025-08-31 — Labor Day: 12:00 CT.
        (2025, 9, 1, early_close(NOON), T2, "CME-SVC-2025-08-31"),
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
        (2025, 11, 27, early_close(NOON), T2, "CME-SVC-2025-11-26"),
        // 2025-11-28 — T2 — CME-SVC-2025-11-26 — day after Thanksgiving:
        // 2025-11-28 - T2 - CME-SVC-2025-11-26 - day after Thanksgiving; the Thursday holiday publishes no final close, so this trade date owns the span from Wednesday evening and its own session is cut at 12:15 CT.
        (
            2025,
            11,
            28,
            ReplacementBlocks(&MERGED_SESSION_EARLY_CLOSE_BLOCKS_2025_11_28),
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
        (2026, 2, 16, early_close(NOON), T2, "CME-SVC-2026-02-15"),
        // 2026-02-17 - T2 - CME-SVC-2026-02-15 - Presidents Day; as 2025-01-21.
        (
            2026,
            2,
            17,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-02-15"
        ),
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
        // 2026-05-26 - T2 - CME-SVC-2026-05-24 - Memorial Day; as 2025-01-21.
        (
            2026,
            5,
            26,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-05-24"
        ),
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
        (2026, 11, 26, early_close(NOON), T2, "CME-SVC-2026-11-25"),
        // 2026-11-27 — T2 — CME-SVC-2026-11-25 — day after Thanksgiving:
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - day after Thanksgiving; as 2025-11-28.
        (
            2026,
            11,
            27,
            ReplacementBlocks(&MERGED_SESSION_EARLY_CLOSE_BLOCKS),
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
        (2027, 2, 15, early_close(NOON), T2, "CME-SVC-2027-02-14"),
        // 2027-02-16 - T2 - CME-SVC-2027-02-14 - Presidents Day; as 2025-01-21.
        (
            2027,
            2,
            16,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-02-14"
        ),
        // 2027-03-26 — T2 — CME-SVC-2027-03-25 — Good Friday: full Globex
        // closure, no events published.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 — T2 — CME-SVC-2027-05-30 — Memorial Day: 12:00 CT.
        (2027, 5, 31, early_close(NOON), T2, "CME-SVC-2027-05-30"),
        // 2027-06-01 - T2 - CME-SVC-2027-05-30 - Memorial Day; as 2025-01-21.
        (
            2027,
            6,
            1,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-05-30"
        ),
        // 2027-06-18 — T2 — CME-SVC-2027-06-17 — Juneteenth observed: 12:00 CT.
        (2027, 6, 18, early_close(NOON), T2, "CME-SVC-2027-06-17"),
        // 2027-06-21 — T2 — CME-SVC-2027-06-17 — Saturday session 05:00-17:00 CT
        // carrying this trade date; the complete trading day is stated.
        (2027, 6, 21, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 — T2 — CME-SVC-2027-07-04 — Independence Day observed:
        // 12:00 CT.
        (2027, 7, 5, early_close(NOON), T2, "CME-SVC-2027-07-04"),
        // 2027-07-06 - T2 - CME-SVC-2027-07-04 - Independence Day observed; as 2025-01-21.
        (
            2027,
            7,
            6,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-07-04"
        ),
        // 2027-09-06 — T2 — CME-SVC-2027-09-05 — Labor Day: 12:00 CT.
        (2027, 9, 6, early_close(NOON), T2, "CME-SVC-2027-09-05"),
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
        (2027, 11, 25, early_close(NOON), T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 — T2 — CME-SVC-2027-11-24 — day after Thanksgiving:
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - day after Thanksgiving; as 2025-11-28.
        (
            2027,
            11,
            26,
            ReplacementBlocks(&MERGED_SESSION_EARLY_CLOSE_BLOCKS),
            T2,
            "CME-SVC-2027-11-24"
        ),
        // 2027-12-24 — T2 — CME-SVC-2027-12-22 — Christmas 2027, which CME keys
        // to Thursday 2027-12-23: full Globex closure on the Friday.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
