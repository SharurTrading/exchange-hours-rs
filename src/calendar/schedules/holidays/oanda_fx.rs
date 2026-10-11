// SPDX-License-Identifier: MIT-0

//! OANDA FX-book holidays, 2024-02-21..2026-10-30.
//! Evidence: `docs/evidence/oanda_fx.md`.

use super::{EvidenceTier::T1, HolidayKind::Closed, HolidayTable, holidays};

/// The book's built-in holiday rows and the window they were audited over.
// Evidence: docs/evidence/oanda_fx.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2024, 2, 21) ..= (2026, 10, 30)],
    rows: [
        // 2024-12-25 - T1 - OANDA-UK-CARTER-2025-WAYBACK - FX reopens at 17:05
        (2024, 12, 25, Closed, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
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
