// SPDX-License-Identifier: MIT-0

//! Euronext Lisbon cash-equity holidays published for 2026 and 2027. The
//! 2027 rows are the Lisbon column of the operator's 06 November 2026
//! holiday-calendar notice: `Closed` on 1 January, Good Friday 26 March and
//! Easter Monday 29 March, `*Half Trading Day` on both December eves with the
//! hours deferred to the end-of-year appendix (`Unsourced`), and `Full
//! Trading Day` on every other printed row.
//! Evidence: `docs/evidence/euronext_lisbon.md`.

use super::EvidenceTier::T1;
use super::HolidayKind::{Closed, Unsourced};
use super::{HolidayTable, holidays};

/// Lisbon column of the operator cash-market calendar.
// Evidence: docs/evidence/euronext_lisbon.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2026, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2026-01-01 - T1 - EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12 - Lisbon cash calendar.
        (2026, 1, 1, Closed, T1, "EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12"),
        // 2026-04-03 - T1 - EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12 - Lisbon cash calendar.
        (2026, 4, 3, Closed, T1, "EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12"),
        // 2026-04-06 - T1 - EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12 - Lisbon cash calendar.
        (2026, 4, 6, Closed, T1, "EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12"),
        // 2026-05-01 - T1 - EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12 - Lisbon cash calendar.
        (2026, 5, 1, Closed, T1, "EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12"),
        // 2026-12-24 - T1 - EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12 - Lisbon cash calendar.
        (2026, 12, 24, Unsourced, T1, "EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12"),
        // 2026-12-25 - T1 - EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12 - Lisbon cash calendar.
        (2026, 12, 25, Closed, T1, "EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12"),
        // 2026-12-31 - T1 - EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12 - Lisbon cash calendar.
        (2026, 12, 31, Unsourced, T1, "EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12"),
        // 2027-01-01 - T1 - EURONEXT-IF-2027 - Lisbon cash calendar.
        (2027, 1, 1, Closed, T1, "EURONEXT-IF-2027"),
        // 2027-03-26 - T1 - EURONEXT-IF-2027 - Lisbon cash calendar.
        (2027, 3, 26, Closed, T1, "EURONEXT-IF-2027"),
        // 2027-03-29 - T1 - EURONEXT-IF-2027 - Lisbon cash calendar.
        (2027, 3, 29, Closed, T1, "EURONEXT-IF-2027"),
        // 2027-12-24 - T1 - EURONEXT-IF-2027 - Lisbon prints `*Half
        // Trading Day` and the appendix hours are announced but unstated.
        (2027, 12, 24, Unsourced, T1, "EURONEXT-IF-2027"),
        // 2027-12-31 - T1 - EURONEXT-IF-2027 - the same Lisbon half-day
        // cell and unstated appendix instants.
        (2027, 12, 31, Unsourced, T1, "EURONEXT-IF-2027"),
    ],
};
