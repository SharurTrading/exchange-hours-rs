// SPDX-License-Identifier: MIT-0

//! CME NYMEX energy and COMEX metals holiday rows, 2025-2027
//! (LAW-HOLIDAY-SCOPE).
//!
//! Keyed by the crate's own venue-local trade date in `America/Chicago`
//! (design memo D1), never by CME's event date. The family's grid wraps: from
//! the 2015 revision one occurrence opens 17:00 CT on the previous local day
//! and closes 16:00 CT on the trade date, Sunday through Thursday, so a trade
//! date's session begins the evening before and CME's event-date records must
//! be converted before they can key a row.
//!
//! The conversion has three shapes in the 2025-2027 window, and each one is
//! recorded per year in the evidence file:
//!
//! - CME publishes a final close on the date at an instant earlier than 16:00
//!   CT — `12:00`, `12:45`, `13:45` — which is an `EarlyClose` on that trade
//!   date. It clips the occurrence that opened the previous evening, because
//!   the clip is stated on the trade date rather than on a civil day.
//! - CME publishes no final close on the date and only a pre-open and a 17:00
//!   CT open carrying the **next** business day's trade date, or publishes no
//!   events at all. No session belongs to the date, so the row is `Closed` and
//!   the previous evening's 17:00 CT leg disappears with it.
//! - CME publishes only a pre-open on the date, at 13:30 CT, followed by the
//!   ordinary 17:00 CT open. A pre-open is order entry with no matching by
//!   CME's own legend, so matching ended at 13:30 CT: that is the trade date's
//!   final close and the row is an `EarlyClose` at 13:30 CT.
//!
//! On the three Friday holidays of 2026 and 2027 CME prints the early close
//! but dates it to the following Monday. The crate assigns a session to the
//! venue-local date of its own final close, so the row stays on the Friday and
//! the divergence is recorded as an interpretive step rather than modelled; a
//! `Closed` row there would delete roughly nineteen hours of sourced trading,
//! because this family has a weekend close and no following-business-day roll.
//!
//! Every 2025-2027 row is **T2** under LAW-PRIMARY-SOURCES: CME's
//! trading-hours service, the endpoint `cmegroup.com/trading-hours.html` itself
//! calls, read as bytes and saved. No T1 per-asset-class rendering exists for
//! those years. That,
//! the Saturday sessions after the Friday holidays, the 2025-11-28 morning
//! re-open pair, the order-entry deviations and the eight 2025 windows that
//! survive only in a pre-holiday capture are gaps recorded in
//! [`docs/evidence/globex_energy.md`](../../../../../docs/evidence/globex_energy.md).
//!
//! Energy and metals are one key and CME prints them as one product-group row
//! on every date either table audits, so the memo's D17 intersection rule is
//! never reached: the two halves agree everywhere.

use super::fences::early_close;
use super::{
    EvidenceTier::T2,
    HolidayKind::{Closed, ReplacementBlocks},
    HolidayTable, holidays,
};
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
/// The 2026-06-22 and 2027-06-21 trade dates read two windows, because the
/// window that carries their Saturday stops there and prints no Sunday entry at
/// all: `CME-SVC-2026-06-18` and `CME-SVC-2027-06-17` carry the Saturday
/// session, and `CME-SVC-2026-06-21` and `CME-SVC-2027-06-20` carry the Sunday
/// and Monday phases. The 2026-07-06 trade date needs one window only:
/// `CME-SVC-2026-07-03` runs through its Sunday and prints both legs.
///
/// - offset `-4`, Thursday 16:45-17:00 CT: the Pre-Open queue.
/// - offset `-4`, Thursday 17:00 CT to Friday 12:00 CT: the session ending at
///   the Friday holiday's early close.
/// - offset `-2`, Saturday 05:00-17:00 CT: the session itself.
/// - offset `-1`, Sunday 16:00-17:00 CT: the ordinary Sunday Pre-Open queue,
///   which no trade matches.
/// - offset `-1`, Sunday 17:00 CT to Monday 16:00 CT: the ordinary electronic
///   session, wrapping one local midnight.
///
/// The Thursday and Sunday queue and evening phases are the family's own
/// normal-week phases, quoted here because a replacement states the whole day;
/// the Thursday session ends at the Friday holiday's early close rather than at
/// 16:00 CT. Evidence: `docs/evidence/globex_energy.md`.
pub(crate) static SATURDAY_SESSION_BLOCKS: [ExceptionBlock; 5] = [
    ExceptionBlock::order_entry(-4, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-4, 17 * 3_600, 12 * 3_600),
    ExceptionBlock::extended(-2, 5 * 3_600, 17 * 3_600),
    ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 16 * 3_600),
];

/// The complete trading day of a merged trade date whose holiday closes at
/// `13:30` CT and whose `-2` day is a Sunday.
///
/// The holiday publishes no final close for its own trade date, so the span from
/// Sunday evening carries the next business day's date — and it still ends where
/// the crate's own holiday row ends it, `13:30`, with the queue opening there.
///
/// Evidence: `docs/evidence/globex_energy.md`.
pub(crate) static MERGED_SESSION_BLOCKS: [ExceptionBlock; 4] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 13 * 3_600 + 30 * 60),
    ExceptionBlock::order_entry(-1, 13 * 3_600 + 30 * 60, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 16 * 3_600),
];

/// The same, where the `-2` day is an ordinary weekday rather than a Sunday, so
/// its queue opens at the family's weekday `16:45` CT. The Juneteenth Thursday
/// 2025-06-19 merge is the case.
///
/// Evidence: `docs/evidence/globex_energy.md`.
pub(crate) static MERGED_SESSION_AFTER_WEEKDAY_BLOCKS: [ExceptionBlock; 4] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 13 * 3_600 + 30 * 60),
    ExceptionBlock::order_entry(-1, 13 * 3_600 + 30 * 60, 17 * 3_600),
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
/// - offset `-2`, 17:00 CT to offset `-1` 13:30 CT: the matching session.
/// - offset `-1`, 13:30-17:00 CT: the holiday's own Pre-Open queue.
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
/// Evidence: `docs/evidence/globex_energy.md`.
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
    ExceptionBlock::extended(-2, 17 * 3_600, 13 * 3_600 + 30 * 60),
    ExceptionBlock::order_entry(-1, 13 * 3_600 + 30 * 60, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 24 * 3_600),
    ExceptionBlock::extended(0, 0 * 3_600 + 0 * 60, 7 * 3_600),
    ExceptionBlock::order_entry(0, 7 * 3_600, 7 * 3_600 + 30 * 60),
    ExceptionBlock::extended(0, 7 * 3_600 + 30 * 60, 13 * 3_600 + 45 * 60),
];

/// The same, when the trade date is a day-after-Thanksgiving Friday whose
/// session the operator ends at `13:45` CT.
///
/// Evidence: `docs/evidence/globex_energy.md`.
pub(crate) static MERGED_SESSION_EARLY_CLOSE_BLOCKS: [ExceptionBlock; 4] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 13 * 3_600 + 30 * 60),
    ExceptionBlock::order_entry(-1, 13 * 3_600 + 30 * 60, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 13 * 3_600 + 45 * 60),
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
/// with no row is audited normal, including Columbus Day and Veterans Day,
/// which are not CME Globex holidays at all, except where an `Unsourced` row
/// marks the operator's silence instead.
// Evidence: docs/evidence/globex_energy.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 - T2 - CME-SVC-2025-01-19 - Martin Luther King Jr. Day, 13:30 CT close.
        (2025, 1, 20, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-01-19"),
        // 2025-01-21 - T2 - CME-SVC-2025-01-19 - MLK Day; the holiday publishes no final close, so the span from Sunday evening carries this trade date, and it still ends at the holiday's 13:30 CT close.
        (
            2025,
            1,
            21,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-01-19"
        ),
        // 2025-02-17 - T2 - CME-SVC-2025-02-16 - Presidents' Day, 13:30 CT close.
        (2025, 2, 17, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-02-16"),
        // 2025-02-18 - T2 - CME-SVC-2025-02-16 - Presidents Day; as 2025-01-21.
        (
            2025,
            2,
            18,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-02-16"
        ),
        // 2025-04-18 - T2 - CME-SVC-2025-04-17 - Good Friday; no events published.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 - T2 - CME-SVC-2025-05-25 - Memorial Day, 13:30 CT close.
        (2025, 5, 26, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-05-25"),
        // 2025-05-27 - T2 - CME-SVC-2025-05-25 - Memorial Day; as 2025-01-21.
        (
            2025,
            5,
            27,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-05-25"
        ),
        // 2025-06-19 - T2 - CME-SVC-2025-06-18 - Juneteenth, 13:30 CT close.
        (2025, 6, 19, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-06-18"),
        // 2025-06-20 - T2 - CME-SVC-2025-06-18 - Juneteenth falls on the Thursday, so the span opens Wednesday evening and its -2 queue is the weekday 16:45.
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
        // 2025-09-01 - T2 - CME-SVC-2025-08-31 - Labor Day, 13:30 CT close.
        (2025, 9, 1, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-08-31"),
        // 2025-09-02 - T2 - CME-SVC-2025-08-31 - Labor Day; as 2025-01-21.
        (
            2025,
            9,
            2,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-08-31"
        ),
        // 2025-11-27 - T2 - CME-SVC-2025-11-26 - Thanksgiving, 13:30 CT close.
        (2025, 11, 27, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-11-26"),
        // 2025-11-28 - T2 - CME-SVC-2025-11-26 - day after Thanksgiving, 13:45 CT close.
        // 2025-11-28 - T2 - CME-SVC-2025-11-26 - day after Thanksgiving; the Thursday holiday publishes no final close, so this trade date owns the span from Wednesday evening and ends at the operator's 13:45 CT close.
        (
            2025,
            11,
            28,
            ReplacementBlocks(&MERGED_SESSION_EARLY_CLOSE_BLOCKS_2025_11_28),
            T2,
            "CME-SVC-2025-11-26"
        ),
        // 2025-11-29 - T2 - CME-SVC-2025-11-26-SAT - Thanksgiving Saturday; no events published.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-12-24 - T2 - CME-SVC-2025-12-24 - Christmas Eve, 12:45 CT close.
        (2025, 12, 24, early_close(12 * 3_600 + 45 * 60), T2, "CME-SVC-2025-12-24"),
        // 2025-12-25 - T2 - CME-SVC-2025-12-24 - Christmas Day; no trade date of its own.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-24"),
        // 2026-01-01 - T2 - CME-SVC-2025-12-31 - New Year's Day; no trade date of its own.
        (2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-01-19 - T2 - CME-SVC-2026-01-18 - Martin Luther King Jr. Day, 13:30 CT close.
        (2026, 1, 19, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-01-18"),
        // 2026-01-20 - T2 - CME-SVC-2026-01-18 - MLK Day; as 2025-01-21.
        (
            2026,
            1,
            20,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-01-18"
        ),
        // 2026-02-16 - T2 - CME-SVC-2026-02-15 - Presidents' Day, 13:30 CT close.
        (2026, 2, 16, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-02-15"),
        // 2026-02-17 - T2 - CME-SVC-2026-02-15 - Presidents Day; as 2025-01-21.
        (
            2026,
            2,
            17,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-02-15"
        ),
        // 2026-04-03 - T2 - CME-SVC-2026-04-01 - Good Friday; no events published.
        (2026, 4, 3, Closed, T2, "CME-SVC-2026-04-01"),
        // 2026-05-25 - T2 - CME-SVC-2026-05-24 - Memorial Day, 13:30 CT close.
        (2026, 5, 25, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-05-24"),
        // 2026-05-26 - T2 - CME-SVC-2026-05-24 - Memorial Day; as 2025-01-21.
        (
            2026,
            5,
            26,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-05-24"
        ),
        // 2026-06-19 - T2 - CME-SVC-2026-06-18 - Juneteenth, 12:00 CT close dated to 06-22.
        (2026, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-2026-06-18"),
        // 2026-06-22 - T2 - CME-SVC-2026-06-18 - Saturday 2026-06-20 05:00-17:00 CT from that
        // window, and the Sunday 2026-06-21 Pre-Open plus Sunday-17:00-to-Monday-16:00
        // session from CME-SVC-2026-06-21, all carrying this trade date; the complete
        // trading day is stated.
        (2026, 6, 22, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - Independence Day observed, 12:00 CT close.
        (2026, 7, 3, early_close(12 * 3_600), T2, "CME-SVC-2026-07-03"),
        // 2026-07-06 - T2 - CME-SVC-2026-07-03 - Saturday 2026-07-04 05:00-17:00 CT and the Sunday
        // 2026-07-05 Pre-Open plus Sunday-17:00-to-Monday-16:00 session, all
        // carrying this trade date; the complete trading day is stated.
        (2026, 7, 6, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 - T2 - CME-SVC-2026-09-06 - Labor Day, 13:30 CT close.
        (2026, 9, 7, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-09-06"),
        // 2026-09-08 - T2 - CME-SVC-2026-09-06 - Labor Day; as 2025-01-21.
        (
            2026,
            9,
            8,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-09-06"
        ),
        // 2026-11-26 - T2 - CME-SVC-2026-11-25 - Thanksgiving, 13:30 CT close.
        (2026, 11, 26, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-11-25"),
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - day after Thanksgiving, 13:45 CT close.
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - day after Thanksgiving; as 2025-11-28.
        (
            2026,
            11,
            27,
            ReplacementBlocks(&MERGED_SESSION_EARLY_CLOSE_BLOCKS),
            T2,
            "CME-SVC-2026-11-25"
        ),
        // 2026-12-24 - T2 - CME-SVC-2026-12-22 - Christmas Eve, 12:45 CT close.
        (2026, 12, 24, early_close(12 * 3_600 + 45 * 60), T2, "CME-SVC-2026-12-22"),
        // 2026-12-25 - T2 - CME-SVC-2026-12-24 - Christmas Day; no events published.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 - T2 - CME-SVC-2026-12-31 - New Year's Day; no events published.
        (2027, 1, 1, Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-01-18 - T2 - CME-SVC-2027-01-17 - Martin Luther King Jr. Day, 13:30 CT close.
        (2027, 1, 18, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-01-17"),
        // 2027-01-19 - T2 - CME-SVC-2027-01-17 - MLK Day; as 2025-01-21.
        (
            2027,
            1,
            19,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-01-17"
        ),
        // 2027-02-15 - T2 - CME-SVC-2027-02-14 - Presidents' Day, 13:30 CT close.
        (2027, 2, 15, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-02-14"),
        // 2027-02-16 - T2 - CME-SVC-2027-02-14 - Presidents Day; as 2025-01-21.
        (
            2027,
            2,
            16,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-02-14"
        ),
        // 2027-03-26 - T2 - CME-SVC-2027-03-25 - Good Friday; no events published.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 - T2 - CME-SVC-2027-05-30 - Memorial Day, 13:30 CT close.
        (2027, 5, 31, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-05-30"),
        // 2027-06-01 - T2 - CME-SVC-2027-05-30 - Memorial Day; as 2025-01-21.
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
        // 2027-06-21 - T2 - CME-SVC-2027-06-17 - Saturday 2027-06-19 05:00-17:00 CT from that
        // window, and the Sunday 2027-06-20 Pre-Open plus Sunday-17:00-to-Monday-16:00
        // session from CME-SVC-2027-06-20, all carrying this trade date; the complete
        // trading day is stated.
        (2027, 6, 21, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 - T2 - CME-SVC-2027-07-04 - Independence Day observed, 13:30 CT close.
        (2027, 7, 5, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-07-04"),
        // 2027-07-06 - T2 - CME-SVC-2027-07-04 - Independence Day observed; as 2025-01-21.
        (
            2027,
            7,
            6,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-07-04"
        ),
        // 2027-09-06 - T2 - CME-SVC-2027-09-05 - Labor Day, 13:30 CT close.
        (2027, 9, 6, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-09-05"),
        // 2027-09-07 - T2 - CME-SVC-2027-09-05 - Labor Day; as 2025-01-21.
        (
            2027,
            9,
            7,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-09-05"
        ),
        // 2027-11-25 - T2 - CME-SVC-2027-11-24 - Thanksgiving, 13:30 CT close.
        (2027, 11, 25, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - day after Thanksgiving, 13:45 CT close.
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - day after Thanksgiving; as 2025-11-28.
        (
            2027,
            11,
            26,
            ReplacementBlocks(&MERGED_SESSION_EARLY_CLOSE_BLOCKS),
            T2,
            "CME-SVC-2027-11-24"
        ),
        // 2027-12-24 - T2 - CME-SVC-2027-12-22 - Christmas Friday; no events published.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
