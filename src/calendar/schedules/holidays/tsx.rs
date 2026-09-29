// SPDX-License-Identifier: MIT-0

//! Toronto Stock Exchange (TSX) holiday rows, 2025-2026.
//!
//! Keyed by the crate's own venue-local trade date in `America/Toronto`. TSX
//! runs no overnight session, so an event date and its trade date are one
//! civil day and the conversion is the identity.
//!
//! The whole block is **T1**: TMX Group's own "Stock Market Holidays - Stock
//! Markets Closed" calendar at
//! `tsx.com/en/trading/calendars-and-trading-hours/calendar`, one server-
//! rendered page carrying the 2025 and 2026 lists. TSX has published no 2027
//! calendar yet (the page's newest section is 2026; the operator adds the next
//! year in Q4), so coverage stops at 2026-12-31; that and the per-row
//! derivation are recorded in
//! [`docs/evidence/tsx.md`](../../../../../docs/evidence/tsx.md).
//!
//! The one early close is the operator's own footnote: Christmas Eve closes at
//! `1:00 PM (TSX/TSXV)` — the 1:30 PM half of that footnote applies to the
//! ALPHA/ALPHA X/DRK book systems and is outside this venue's scope. The U.S.
//! holidays the same page lists under a separate heading are footnoted as
//! **special-settlement** days for USD issues, not trading closures, so none
//! of them is encoded (LAW-SESSION-NOT-EXPIRY).

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::fences::early_close;
use super::{HolidayTable, holidays};

/// TSX's stated Christmas Eve close, 1:00 PM Toronto time.
///
/// The calendar's footnote reads `* Closing at 1:00 PM (TSX/TSXV) and 1:30
/// (ALPHA/ALPHA X/DRK)`; this identity's scope is the Toronto Stock Exchange
/// cash-equity market, whose close is 13:00.
const HALF_DAY_13_00: u32 = 13 * 3_600;

/// TSX's built-in holiday rows and the window they were audited over.
///
/// Every row is one line of the operator's 2025 or 2026 "Stock Markets Closed"
/// list at the live retrieval of 2026-09-28. A date inside the window with no
/// row is audited normal.
// Evidence: docs/evidence/tsx.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2026, 12, 31)],
    rows: [
        // 2025-01-01 - T1 - TSX-CAL-2026-09-28 - New Year's Day.
        (2025, 1, 1, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-02-17 - T1 - TSX-CAL-2026-09-28 - Family Day.
        (2025, 2, 17, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-04-18 - T1 - TSX-CAL-2026-09-28 - Good Friday.
        (2025, 4, 18, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-05-19 - T1 - TSX-CAL-2026-09-28 - Victoria Day.
        (2025, 5, 19, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-07-01 - T1 - TSX-CAL-2026-09-28 - Canada Day.
        (2025, 7, 1, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-08-04 - T1 - TSX-CAL-2026-09-28 - Civic Holiday.
        (2025, 8, 4, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-09-01 - T1 - TSX-CAL-2026-09-28 - Labour Day.
        (2025, 9, 1, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-10-13 - T1 - TSX-CAL-2026-09-28 - Thanksgiving Day.
        (2025, 10, 13, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-12-24 - T1 - TSX-CAL-2026-09-28 - Christmas Eve: closing at
        // 1:00 PM (TSX/TSXV).
        (2025, 12, 24, early_close(HALF_DAY_13_00), T1, "TSX-CAL-2026-09-28"),
        // 2025-12-25 - T1 - TSX-CAL-2026-09-28 - Christmas Day.
        (2025, 12, 25, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-12-26 - T1 - TSX-CAL-2026-09-28 - Boxing Day.
        (2025, 12, 26, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-01-01 - T1 - TSX-CAL-2026-09-28 - New Year's Day.
        (2026, 1, 1, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-02-16 - T1 - TSX-CAL-2026-09-28 - Family Day.
        (2026, 2, 16, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-04-03 - T1 - TSX-CAL-2026-09-28 - Good Friday.
        (2026, 4, 3, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-05-18 - T1 - TSX-CAL-2026-09-28 - Victoria Day.
        (2026, 5, 18, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-07-01 - T1 - TSX-CAL-2026-09-28 - Canada Day.
        (2026, 7, 1, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-08-03 - T1 - TSX-CAL-2026-09-28 - Civic Holiday.
        (2026, 8, 3, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-09-07 - T1 - TSX-CAL-2026-09-28 - Labour Day.
        (2026, 9, 7, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-10-12 - T1 - TSX-CAL-2026-09-28 - Thanksgiving Day.
        (2026, 10, 12, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-12-24 - T1 - TSX-CAL-2026-09-28 - Christmas Eve: closing at
        // 1:00 PM (TSX/TSXV).
        (2026, 12, 24, early_close(HALF_DAY_13_00), T1, "TSX-CAL-2026-09-28"),
        // 2026-12-25 - T1 - TSX-CAL-2026-09-28 - Christmas Day.
        (2026, 12, 25, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-12-28 - T1 - TSX-CAL-2026-09-28 - In Lieu of Boxing Day.
        (2026, 12, 28, Closed, T1, "TSX-CAL-2026-09-28"),
    ],
};
