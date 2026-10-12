// SPDX-License-Identifier: MIT-0

//! Euronext Brussels cash-equity holidays published for 2026.
//! Evidence: `docs/evidence/euronext_brussels.md`.

use super::EvidenceTier::T1;
use super::HolidayKind::{Closed, Unsourced};
use super::{HolidayTable, holidays};

/// Brussels column of the operator cash-market calendar.
// Evidence: docs/evidence/euronext_brussels.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2026, 1, 1) ..= (2026, 12, 31)],
    rows: [
        // 2026-01-01 - T1 - EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12 - Brussels cash calendar.
        (2026, 1, 1, Closed, T1, "EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12"),
        // 2026-04-03 - T1 - EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12 - Brussels cash calendar.
        (2026, 4, 3, Closed, T1, "EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12"),
        // 2026-04-06 - T1 - EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12 - Brussels cash calendar.
        (2026, 4, 6, Closed, T1, "EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12"),
        // 2026-05-01 - T1 - EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12 - Brussels cash calendar.
        (2026, 5, 1, Closed, T1, "EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12"),
        // 2026-12-24 - T1 - EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12 - Brussels cash calendar.
        (2026, 12, 24, Unsourced, T1, "EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12"),
        // 2026-12-25 - T1 - EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12 - Brussels cash calendar.
        (2026, 12, 25, Closed, T1, "EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12"),
        // 2026-12-31 - T1 - EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12 - Brussels cash calendar.
        (2026, 12, 31, Unsourced, T1, "EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12"),
    ],
};
