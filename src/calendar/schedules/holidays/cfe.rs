// SPDX-License-Identifier: MIT-0

//! Cboe Futures Exchange holiday rows, 2025-2026.
//!
//! Keyed by the crate's own venue-local trade date in `US/Central` (design
//! memo D1). CFE's trading day wraps: the session for trade date `D` opens at
//! 17:00 CT on `D-1`, runs through the 08:30-15:00 CT regular session and ends
//! with the 15:00-16:00 CT extended window on `D`. Cboe's holiday table is
//! keyed by civil date and prints two cells per row, `Regular Trading Hours`
//! and `Extended Trading Hours`, so a row whose regular cell reads `None` and
//! whose extended cell still names a block is the overnight leg of that trade
//! date stopping early — an **early close**, not a closure.
//!
//! The whole block is **T1**: Cboe's own per-holiday notices under
//! `cdn.cboe.com/resources/schedule_update/<publication year>/`, which are the
//! only CFE artifacts that state session instants for 2025, plus the operator's
//! `Hours & Holidays` page and CSV export for 2026. Cboe has published no 2027
//! schedule, so coverage stops at 2026-12-31; that and the per-row derivation
//! are recorded in [`docs/evidence/cfe.md`](../../../../../docs/evidence/cfe.md)
//! and [`docs/evidence/cfe_vix.md`](../../../../../docs/evidence/cfe_vix.md).
//!
//! Two 2025 rows are not the 2026 shapes. Good Friday 2025-04-18 prints no
//! Friday close and no Friday trade date, so it is a **closure** where the 2026
//! date is an early close at the regular open. The National Day of Mourning
//! 2025-01-09 is a **trading day with no regular hours at all**: the only
//! session is extended trading from 17:00 CT on 2025-01-08 to 08:30 CT on
//! 2025-01-09, which no scalar clip can state, so it ships as a
//! `ReplacementBlocks` row.
//!
//! One table serves both the `cfe` venue and the `cfe_vix` key: Cboe publishes
//! one schedule for all CFE futures, and VIX futures are the only family the
//! crate routes to the venue, so the design memo's venue intersection (D17) is
//! that one family's own table.

use super::EvidenceTier::T1;
use super::HolidayKind::{Closed, ReplacementBlocks};
use super::fences::early_close;
use super::{HolidayTable, holidays};
use crate::calendar::exceptions::ExceptionBlock;

/// The complete 2025-01-09 trading day, relative to that trade date.
///
/// Cboe states it as one extended session — `5:00 p.m. (Wednesday, January 8,
/// 2025) to 8:30 a.m. (Extended Trading Hours) for VX, VXM, VXT, and VXMT` —
/// and `No Regular Trading Hours for VX, VXM, VXT, and VXMT`. So the day is one
/// wrapping extended block and nothing else: no regular block, and no
/// 15:00-16:00 CT extended window, because the session has already ended.
///
/// Cboe also states that `The Thursday, January 9, 2025, business day will end
/// at 8:30 a.m. CT on January 9, 2025, and the daily settlement prices of VX,
/// VXM, IBHY, IBIG, and IEMD futures will be determined at that time.` That
/// settlement instant coincides with the close and is therefore **not** a
/// boundary of its own (LAW-SESSION-NOT-EXPIRY); nothing is modelled for it.
///
/// Evidence: `docs/evidence/cfe.md`, `docs/evidence/cfe_vix.md`.
// The row's bounds are read from this declaration by
// `a_replacement_rows_printed_instants_are_the_block_bounds_it_ships`, which
// parses one `ExceptionBlock::<kind>(offset, open, close)` call per line, so the
// constructor stays on one line.
#[rustfmt::skip]
static MOURNING_2025_01_09_BLOCKS: [ExceptionBlock; 1] =
    [ExceptionBlock::extended(-1, 17 * 3_600, 8 * 3_600 + 30 * 60)];

/// CFE's built-in holiday rows and the window they were audited over.
///
/// Every row is one line of a Cboe CFE holiday notice (2025) or of Cboe's
/// `2026 Futures Holiday Schedule` (2026). A date inside the window with no row
/// is audited normal; 2025-11-28, 2025-12-24, 2026-11-27 and 2026-12-24 carry
/// rows because Cboe prints them as half days, not because they are holidays.
// Evidence: docs/evidence/cfe.md, docs/evidence/cfe_vix.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2026, 12, 31)],
    rows: [
        // 2025-01-01 - T1 - CBOE-SU-2025-NEW-YEAR - New Year's Day: the notice
        // prints the Tuesday 2024-12-31 close and the Thursday 2025-01-02
        // evening reopen, and no block at all for the holiday's own day.
        (2025, 1, 1, Closed, T1, "CBOE-SU-2025-NEW-YEAR"),
        // 2025-01-09 - T1 - CBOE-SU-2025-MOURNING - National Day of Mourning:
        // one extended block, 17:00 CT on 2025-01-08 to 08:30 CT, and no regular
        // session at all.
        (2025, 1, 9, ReplacementBlocks(&MOURNING_2025_01_09_BLOCKS), T1, "CBOE-SU-2025-MOURNING"),
        // 2025-01-20 - T1 - CBOE-SU-2025-MLK - MLK Jr. Day, 10:30 CT.
        (2025, 1, 20, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2025-MLK"),
        // 2025-02-17 - T1 - CBOE-SU-2025-PRESIDENTS - Presidents' Day, 10:30 CT.
        (2025, 2, 17, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2025-PRESIDENTS"),
        // 2025-04-18 - T1 - CBOE-SU-2025-GOOD-FRIDAY - Good Friday: the notice
        // prints no Friday close and no Friday trade date, so the day is deleted
        // outright rather than clipped at the regular open.
        (2025, 4, 18, Closed, T1, "CBOE-SU-2025-GOOD-FRIDAY"),
        // 2025-05-26 - T1 - CBOE-SU-2025-MEMORIAL - Memorial Day, 10:30 CT.
        (2025, 5, 26, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2025-MEMORIAL"),
        // 2025-06-19 - T1 - CBOE-SU-2025-JUNETEENTH - Juneteenth, 10:30 CT.
        (2025, 6, 19, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2025-JUNETEENTH"),
        // 2025-07-03 - T1 - CBOE-SU-2025-INDEPENDENCE - Independence Day
        // observed Friday 2025-07-04, so its own day is deleted and the half day
        // falls on the Thursday trade date.
        (2025, 7, 3, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2025-INDEPENDENCE"),
        // 2025-07-04 - T1 - CBOE-SU-2025-INDEPENDENCE - Independence Day: the
        // notice prints no block for that column and no trade date for it.
        (2025, 7, 4, Closed, T1, "CBOE-SU-2025-INDEPENDENCE"),
        // 2025-09-01 - T1 - CBOE-SU-2025-LABOR - Labor Day, 10:30 CT.
        (2025, 9, 1, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2025-LABOR"),
        // 2025-11-27 - T1 - CBOE-SU-2025-THANKSGIVING - Thanksgiving Day, 10:30 CT.
        (2025, 11, 27, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-SU-2025-THANKSGIVING"),
        // 2025-11-28 - T1 - CBOE-SU-2025-THANKSGIVING - Thanksgiving half day,
        // 12:15 CT; the Thursday-evening leg belongs to the 2025-11-27 row.
        (2025, 11, 28, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2025-THANKSGIVING"),
        // 2025-12-24 - T1 - CBOE-SU-2025-CHRISTMAS - Christmas Eve, 12:15 CT;
        // the missing Thursday-evening leg belongs to the 2025-12-25 row.
        (2025, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-SU-2025-CHRISTMAS"),
        // 2025-12-25 - T1 - CBOE-SU-2025-CHRISTMAS - Christmas Day: the notice
        // prints no RTH start or close and no ETH start for that column.
        (2025, 12, 25, Closed, T1, "CBOE-SU-2025-CHRISTMAS"),
        // 2026-01-01 - T1 - CBOE-HOURS-USFUT-2026 - New Year's Day: regular
        // `None`, and the only extended block printed is the Thursday-evening
        // leg of trade date 2026-01-02.
        (2026, 1, 1, Closed, T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-01-19 - T1 - CBOE-HOURS-USFUT-2026 - MLK Jr. Day, 10:30 CT.
        (2026, 1, 19, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-02-16 - T1 - CBOE-HOURS-USFUT-2026 - Presidents' Day, 10:30 CT.
        (2026, 2, 16, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-04-03 - T1 - CBOE-HOURS-USFUT-2026 - Good Friday, 08:30 CT: the
        // overnight leg ends at the regular open and no regular session runs.
        (2026, 4, 3, early_close(8 * 3_600 + 30 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-05-25 - T1 - CBOE-HOURS-USFUT-2026 - Memorial Day, 10:30 CT.
        (2026, 5, 25, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-06-19 - T1 - CBOE-HOURS-USFUT-2026 - Juneteenth, 10:30 CT.
        (2026, 6, 19, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-07-03 - T1 - CBOE-HOURS-USFUT-2026 - Independence Day observed, 10:30 CT.
        (2026, 7, 3, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-09-07 - T1 - CBOE-HOURS-USFUT-2026 - Labor Day, 10:30 CT.
        (2026, 9, 7, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-11-26 - T1 - CBOE-HOURS-USFUT-2026 - Thanksgiving Day, 10:30 CT.
        (2026, 11, 26, early_close(10 * 3_600 + 30 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-11-27 - T1 - CBOE-HOURS-USFUT-2026 - Thanksgiving half day, 12:15 CT.
        (2026, 11, 27, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-12-24 - T1 - CBOE-HOURS-USFUT-2026 - Christmas Eve, 12:15 CT;
        // the missing Thursday-evening leg belongs to the 2026-12-25 row.
        (2026, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "CBOE-HOURS-USFUT-2026"),
        // 2026-12-25 - T1 - CBOE-HOURS-USFUT-2026 - Christmas Day, both cells `None`.
        (2026, 12, 25, Closed, T1, "CBOE-HOURS-USFUT-2026"),
    ],
};
