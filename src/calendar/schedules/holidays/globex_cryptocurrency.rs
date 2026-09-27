// SPDX-License-Identifier: MIT-0

//! CME cryptocurrency holiday rows, venue-local trade dates 2025-01-01 to
//! 2027-12-31 (LAW-HOLIDAY-SCOPE).
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
//!   Monday's trade date. Where it is *printed* — the era's Friday holidays —
//!   the ordinary week already states the whole block and the `Closed` row is
//!   the only row the date needs. Where it is **omitted** — the era's Monday and
//!   Thursday holidays — the holiday's own 16:00-16:01 CT minute carried no
//!   matching break, so the following trade date's complete day ships as a
//!   `ReplacementBlocks` row too: [`MERGED_24X7_AFTER_MONDAY_HOLIDAY_BLOCKS`]
//!   for a Monday holiday and
//!   [`MERGED_24X7_AFTER_THURSDAY_HOLIDAY_BLOCKS`] for a Thursday one.
//!
//! The 2025-2027 block is **T2**: CME's own trading-hours service, the channel
//! the operator's trading-hours page calls to render its per-asset-class holiday
//! table, read as bytes and saved. It states seventeen merged trade dates as
//! `ReplacementBlocks` rows — the nine five-day-era dates and the eight 24/7-era
//! ones — and one six-block day-after-Thanksgiving row. What stays open is the
//! five-day era's undated Pre-Open onset (#123). The quotations, capture times,
//! the event-date-to-trade-date conversion and that gap are in
//! `docs/evidence/globex_cryptocurrency.md`.

use super::fences::early_close;
use super::{
    EvidenceTier::T2, HolidayKind, HolidayKind::ReplacementBlocks, HolidayTable, holidays,
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

/// The complete 24/7-era trading day a Monday holiday merges into the following
/// Tuesday: Friday 16:02 CT through Tuesday 16:00 CT.
///
/// The holiday is not a trade date — every event the operator prints from the
/// pre-holiday Friday evening to the Tuesday final close carries the Tuesday —
/// so the Tuesday's trading day opens four local days before its close, and the
/// `Closed` row on the holiday alone would delete the whole span. Stating the
/// day means restating the ordinary 24/7 grid it contains, with one difference:
/// the holiday's 16:00-16:01 CT minute is not a matching break. The operator
/// prints no `closed` event at 16:00 on the holiday, only the 16:01 `preopen`
/// and the 16:02 `open`, so matching runs across the instant the ordinary week
/// stops at.
///
/// - offset `-4`, 16:01-16:02 CT: the pre-holiday Friday's Pre-Open queue.
/// - offset `-4`, 16:02 CT to offset `-3` 00:00 CT: the Friday evening session.
/// - offset `-3`, 00:00-02:00 CT: the ordinary Saturday session.
/// - offset `-3`, 03:45-04:00 CT: the Saturday Pre-Open queue, inside the
///   02:00-04:00 CT maintenance window the operator does print.
/// - offset `-3`, 04:00 CT to offset `-2` 00:00 CT: the Saturday session after
///   that window.
/// - offset `-2`, 00:00-24:00 CT: the Sunday session, which prints no events.
/// - offset `-1`, 00:00-16:01 CT: the holiday's own session, running through
///   the 16:00 CT instant the ordinary week breaks at.
/// - offset `-1`, 16:01-16:02 CT: the holiday's Pre-Open queue.
/// - offset `-1`, 16:02 CT to offset `0` 00:00 CT: the holiday's evening
///   session.
/// - offset `0`, 00:00-16:00 CT: the trade date's own session, ending at the
///   ordinary 16:00 CT final close.
///
/// Evidence: `docs/evidence/globex_cryptocurrency.md`.
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
pub(crate) static MERGED_24X7_AFTER_MONDAY_HOLIDAY_BLOCKS: [ExceptionBlock; 10] = [
    ExceptionBlock::order_entry(-4, 16 * 3_600 + 1 * 60, 16 * 3_600 + 2 * 60),
    ExceptionBlock::extended(-4, 16 * 3_600 + 2 * 60, 24 * 3_600),
    ExceptionBlock::extended(-3, 0 * 3_600 + 0 * 60, 2 * 3_600),
    ExceptionBlock::order_entry(-3, 3 * 3_600 + 45 * 60, 4 * 3_600),
    ExceptionBlock::extended(-3, 4 * 3_600, 24 * 3_600),
    ExceptionBlock::extended(-2, 0 * 3_600 + 0 * 60, 24 * 3_600),
    ExceptionBlock::extended(-1, 0 * 3_600 + 0 * 60, 16 * 3_600 + 1 * 60),
    ExceptionBlock::order_entry(-1, 16 * 3_600 + 1 * 60, 16 * 3_600 + 2 * 60),
    ExceptionBlock::extended(-1, 16 * 3_600 + 2 * 60, 24 * 3_600),
    ExceptionBlock::extended(0, 0 * 3_600 + 0 * 60, 16 * 3_600),
];

/// The complete 24/7-era trading day a Thursday holiday merges into the
/// following Friday: Wednesday 16:02 CT through Friday 16:00 CT.
///
/// As the Monday shape above, one local day shorter because the merged span
/// opens on the Wednesday evening rather than the pre-holiday Friday. The
/// Wednesday's own daytime session is not restated: it carries the Wednesday's
/// trade date, so no `Closed` row removes it.
///
/// - offset `-2`, 16:01-16:02 CT: the pre-holiday Wednesday's Pre-Open queue.
/// - offset `-2`, 16:02 CT to offset `-1` 00:00 CT: the Wednesday evening
///   session.
/// - offset `-1`, 00:00-16:01 CT: the holiday's own session, running through
///   the 16:00 CT instant the ordinary week breaks at.
/// - offset `-1`, 16:01-16:02 CT: the holiday's Pre-Open queue.
/// - offset `-1`, 16:02 CT to offset `0` 00:00 CT: the holiday's evening
///   session.
/// - offset `0`, 00:00-16:00 CT: the trade date's own session, ending at the
///   ordinary 16:00 CT final close.
///
/// Evidence: `docs/evidence/globex_cryptocurrency.md`.
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
pub(crate) static MERGED_24X7_AFTER_THURSDAY_HOLIDAY_BLOCKS: [ExceptionBlock; 6] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600 + 1 * 60, 16 * 3_600 + 2 * 60),
    ExceptionBlock::extended(-2, 16 * 3_600 + 2 * 60, 24 * 3_600),
    ExceptionBlock::extended(-1, 0 * 3_600 + 0 * 60, 16 * 3_600 + 1 * 60),
    ExceptionBlock::order_entry(-1, 16 * 3_600 + 1 * 60, 16 * 3_600 + 2 * 60),
    ExceptionBlock::extended(-1, 16 * 3_600 + 2 * 60, 24 * 3_600),
    ExceptionBlock::extended(0, 0 * 3_600 + 0 * 60, 16 * 3_600),
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
/// One audited era: 2025-2027 at T2, whose window opens at the crate's
/// permanent 2025 support floor (LAW-COVERAGE). The eras before it left in
/// Stage 5 of the release plan (#117), so nothing before 2025-01-01 has a
/// row or a window: that interval lies outside the window, so `holiday_on`
/// has no answer there rather than reporting a normal date.
///
/// Coverage ends at 2027-12-31, the
/// end of the operator's published future. Inside a window a date with no row
/// is audited normal, except where an `Unsourced` row marks the operator's
/// silence instead.
// Evidence: docs/evidence/globex_cryptocurrency.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2025-01-01 — T2 — CME-SVC-2024-12-31 — New Year's Day; only a 16:00 CT
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
        // 2026-09-08 — T2 — CME-SVC-2026-09-06 — the trade date the merged
        // Labor Day span carries: Friday 16:02 CT through Tuesday 16:00 CT.
        (
            2026,
            9,
            8,
            ReplacementBlocks(&MERGED_24X7_AFTER_MONDAY_HOLIDAY_BLOCKS),
            T2,
            "CME-SVC-2026-09-06"
        ),
        // 2026-11-26 — T2 — CME-SVC-2026-11-25 — Thanksgiving; the 16:00 CT close
        // is omitted and the re-open carries trade date 2026-11-27.
        (2026, 11, 26, HolidayKind::Closed, T2, "CME-SVC-2026-11-25"),
        // 2026-11-27 — T2 — CME-SVC-2026-11-25 — the trade date the merged
        // Thanksgiving span carries: Wednesday 16:02 CT through Friday 16:00 CT.
        (
            2026,
            11,
            27,
            ReplacementBlocks(&MERGED_24X7_AFTER_THURSDAY_HOLIDAY_BLOCKS),
            T2,
            "CME-SVC-2026-11-25"
        ),
        // 2026-12-25 — T2 — CME-SVC-2026-12-24 — Christmas Day; the 16:00 CT close
        // carries trade date 2026-12-28.
        (2026, 12, 25, HolidayKind::Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 — T2 — CME-SVC-2026-12-31 — New Year's Day; the 16:00 CT close
        // carries trade date 2027-01-04.
        (2027, 1, 1, HolidayKind::Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-01-18 — T2 — CME-SVC-2027-01-17 — Martin Luther King Jr. Day; the
        // 16:00 CT close is omitted and the re-open carries trade date 2027-01-19.
        (2027, 1, 18, HolidayKind::Closed, T2, "CME-SVC-2027-01-17"),
        // 2027-01-19 — T2 — CME-SVC-2027-01-17 — the trade date the merged
        // Martin Luther King Jr. Day span carries; as 2026-09-08.
        (
            2027,
            1,
            19,
            ReplacementBlocks(&MERGED_24X7_AFTER_MONDAY_HOLIDAY_BLOCKS),
            T2,
            "CME-SVC-2027-01-17"
        ),
        // 2027-02-15 — T2 — CME-SVC-2027-02-14 — Presidents' Day; the 16:00 CT
        // close is omitted and the re-open carries trade date 2027-02-16.
        (2027, 2, 15, HolidayKind::Closed, T2, "CME-SVC-2027-02-14"),
        // 2027-02-16 — T2 — CME-SVC-2027-02-14 — the trade date the merged
        // Presidents' Day span carries; as 2026-09-08.
        (
            2027,
            2,
            16,
            ReplacementBlocks(&MERGED_24X7_AFTER_MONDAY_HOLIDAY_BLOCKS),
            T2,
            "CME-SVC-2027-02-14"
        ),
        // 2027-03-26 — T2 — CME-SVC-2027-03-25 — Good Friday; the 16:00 CT close
        // carries trade date 2027-03-29.
        (2027, 3, 26, HolidayKind::Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 — T2 — CME-SVC-2027-05-30 — Memorial Day; the 16:00 CT close
        // is omitted and the re-open carries trade date 2027-06-01.
        (2027, 5, 31, HolidayKind::Closed, T2, "CME-SVC-2027-05-30"),
        // 2027-06-01 — T2 — CME-SVC-2027-05-30 — the trade date the merged
        // Memorial Day span carries; as 2026-09-08.
        (
            2027,
            6,
            1,
            ReplacementBlocks(&MERGED_24X7_AFTER_MONDAY_HOLIDAY_BLOCKS),
            T2,
            "CME-SVC-2027-05-30"
        ),
        // 2027-06-18 — T2 — CME-SVC-2027-06-17 — Juneteenth observed; the 16:00 CT
        // close carries trade date 2027-06-21.
        (2027, 6, 18, HolidayKind::Closed, T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 — T2 — CME-SVC-2027-07-04 — Independence Day observed; the
        // 16:00 CT close is omitted and the re-open carries trade date 2027-07-06.
        (2027, 7, 5, HolidayKind::Closed, T2, "CME-SVC-2027-07-04"),
        // 2027-07-06 — T2 — CME-SVC-2027-07-04 — the trade date the merged
        // Independence Day span carries; as 2026-09-08.
        (
            2027,
            7,
            6,
            ReplacementBlocks(&MERGED_24X7_AFTER_MONDAY_HOLIDAY_BLOCKS),
            T2,
            "CME-SVC-2027-07-04"
        ),
        // 2027-09-06 — T2 — CME-SVC-2027-09-05 — Labor Day; the 16:00 CT close is
        // omitted and the re-open carries trade date 2027-09-07.
        (2027, 9, 6, HolidayKind::Closed, T2, "CME-SVC-2027-09-05"),
        // 2027-09-07 — T2 — CME-SVC-2027-09-05 — the trade date the merged
        // Labor Day span carries; as 2026-09-08.
        (
            2027,
            9,
            7,
            ReplacementBlocks(&MERGED_24X7_AFTER_MONDAY_HOLIDAY_BLOCKS),
            T2,
            "CME-SVC-2027-09-05"
        ),
        // 2027-11-25 — T2 — CME-SVC-2027-11-24 — Thanksgiving; the 16:00 CT close
        // is omitted and the re-open carries trade date 2027-11-26.
        (2027, 11, 25, HolidayKind::Closed, T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 — T2 — CME-SVC-2027-11-24 — the trade date the merged
        // Thanksgiving span carries; as 2026-11-27.
        (
            2027,
            11,
            26,
            ReplacementBlocks(&MERGED_24X7_AFTER_THURSDAY_HOLIDAY_BLOCKS),
            T2,
            "CME-SVC-2027-11-24"
        ),
        // 2027-12-24 — T2 — CME-SVC-2027-12-22 — Globex closed for Christmas; the
        // 2027-12-23 re-open carries trade date 2027-12-27, skipping this date.
        (2027, 12, 24, HolidayKind::Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
