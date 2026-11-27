// SPDX-License-Identifier: MIT-0

//! OANDA TRY-pair holiday audit, 2025-11-17..2026-10-30.
//! Evidence: `docs/evidence/oanda_try_fx.md`.

use super::{EvidenceTier::T1, HolidayKind::Closed, HolidayTable, holidays};

/// The Forex category's published changes within the witnessed TRY grid.
// Evidence: docs/evidence/oanda_try_fx.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 11, 17) ..= (2026, 10, 30)],
    rows: [
        // 2025-12-25 - T1 - OANDA-UK-HOLIDAYS - Christmas Day Forex reopen after the TRY close
        (2025, 12, 25, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2026-01-01 - T1 - OANDA-UK-HOLIDAYS - New Year's Day Forex reopen after the TRY close
        (2026, 1, 1, Closed, T1, "OANDA-UK-HOLIDAYS"),
    ],
};
