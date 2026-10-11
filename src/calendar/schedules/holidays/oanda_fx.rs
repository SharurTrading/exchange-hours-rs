// SPDX-License-Identifier: MIT-0

//! OANDA FX-book holiday rows, from the operator's UK holiday page's dated
//! sections 2025-01-01 through 2026-10-30.
//!
//! Keyed by the crate's own venue-local trade date in `America/New_York`
//! (design memo D1). The FX book's holiday surface is Christmas and New
//! Year only: the page prints `Forex: open at 17:05` on Christmas Day and
//! New Year's Day and nothing else on any other dated section. "Open at
//! 17:05" on the holiday names the reopen instant, which is the **next**
//! session's normal open — the holiday trade date's own session (which
//! normally runs from the prior evening's 17:05 to that day's 16:59) does
//! not trade, so the row is `Closed`; 2025-12-31's `close at 16:59` states
//! the matching no-reopen on New Year's Eve and needs no row of its own
//! because 16:59 is already the normal daily close. Every other date in the
//! window has no FX row, which is the operator's statement that the book
//! runs its normal grid there.
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
