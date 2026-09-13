// SPDX-License-Identifier: MIT-0

//! Cboe Futures Exchange holiday rows, 2026.
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
//! The whole block is **T1**: Cboe's own `Hours & Holidays` page for U.S.
//! futures and its CSV export. Cboe has published no 2027 schedule, so
//! coverage stops at 2026-12-31; that and the per-row derivation are recorded
//! in [`docs/evidence/cfe.md`](../../../../../docs/evidence/cfe.md) and
//! [`docs/evidence/cfe_vix.md`](../../../../../docs/evidence/cfe_vix.md).
//!
//! One table serves both the `cfe` venue and the `cfe_vix` key: Cboe publishes
//! one schedule for all CFE futures, and VIX futures are the only family the
//! crate routes to the venue, so the design memo's venue intersection (D17) is
//! that one family's own table.

use super::fences::early_close;
use super::{EvidenceTier::T1, HolidayKind::Closed, HolidayTable, holidays};

/// CFE's built-in holiday rows and the window they were audited over.
///
/// Every row is one line of Cboe's `2026 Futures Holiday Schedule`. A date
/// inside the window with no row is audited normal; 2026-11-27 and 2026-12-24
/// carry rows because Cboe prints them as half days, not because they are
/// holidays.
// Evidence: docs/evidence/cfe.md, docs/evidence/cfe_vix.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2026, 1, 1) ..= (2026, 12, 31)],
    rows: [
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
