// SPDX-License-Identifier: MIT-0

//! OANDA FX-book holiday rows, from the operator's UK holiday page's dated
//! sections 2025-01-01 through 2026-10-30.
//!
//! Keyed by the crate's own venue-local trade date in `America/New_York`
//! (design memo D1). The FX book's holiday surface is Christmas and New
//! Year, stated on five dated sections: three reopen statements — `open
//! late 17:05` (New Year's Day 2025) and `open at 17:05` (Christmas Day
//! 2025, New Year's Day 2026) — and two no-reopen statements — `close at
//! 16:59` (Christmas Eve and New Year's Eve 2025). An open statement names
//! the reopen instant, which is the **next** session's normal open: the
//! holiday trade date's own session (which normally runs from the prior
//! evening's 17:05 to that day's 16:59) does not trade, so the row is
//! `Closed`. A close-at-16:59 statement names the normal daily close — the
//! no-reopen — and needs no row of its own. Every other date in the window
//! has no FX row, which is the operator's statement that the book runs its
//! normal grid there.
//!
//! All rows are **T1**, the operator's own holiday page. All of it is
//! recorded in [`docs/evidence/oanda_fx.md`](../../../../../docs/evidence/oanda_fx.md).

use super::{EvidenceTier::T1, HolidayKind::Closed, HolidayTable, holidays};

/// The book's built-in holiday rows and the window they were audited over.
// Evidence: docs/evidence/oanda_fx.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2026, 10, 30)],
    rows: [
        // 2025-01-01 - T1 - OANDA-UK-HOLIDAYS - New Year's Day: FX open late 17:05, so the
        // holiday trade date's session does not trade
        (2025, 1, 1, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-12-25 - T1 - OANDA-UK-HOLIDAYS - Christmas Day: FX open at 17:05
        (2025, 12, 25, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2026-01-01 - T1 - OANDA-UK-HOLIDAYS - New Year's Day: FX open at 17:05; the page's
        // 2025-12-31 "close at 16:59" states the no-reopen
        (2026, 1, 1, Closed, T1, "OANDA-UK-HOLIDAYS"),
    ],
};
