// SPDX-License-Identifier: MIT-0

//! ASX cash-market holiday rows, 2025-2027.
//!
//! Keyed by the crate's venue-local trade date in `Australia/Sydney` (design
//! memo D1). ASX sessions do not wrap past local midnight, so every holiday
//! row lands on its own civil date. The operator's `Trading calendar` page
//! under cash-market trading hours server-renders one sheet per year: the
//! 2025 rows are read from the page's 2025-04-16 replay and the 2026 and 2027
//! rows from the live page retrieved 2026-09-28, so all three published years
//! are audited and the window reaches 2027-12-31; the per-row derivation is
//! recorded in [`docs/evidence/asx.md`](../../../../../docs/evidence/asx.md).
//!
//! The two `CLOSE EARLY` dates per year state their own instant: the sheet's
//! footnote reads `Normal trading ceases at 14:10 (Sydney time)`, so the row
//! clips the envelope at 14:10 and the 16:10-16:21:30 Post Close block is gone
//! with it. The 2027 sheet prints ANZAC Day (Monday 2027-04-26) as `OPEN`
//! with the footnote `Substitute for Sunday 25 April`, so that Monday is an
//! audited-normal trading day and ships no row. ANZAC Day 2026 falls on
//! Saturday 25 April and the sheet prints it `CLOSED`; the row restates the
//! printed closure and changes no answer the Mon-Fri normal week has not
//! already made.

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::fences::early_close;
use super::{HolidayTable, holidays};

/// ASX's built-in holiday rows and the window they were audited over.
///
/// Every date inside the window with no row is audited normal. 2025 rows cite
/// the operator's page as replayed 2025-04-16 (`ASX-CAL-2025`); 2026 and 2027
/// rows cite the live page (`ASX-CAL-LIVE`), which renders both years' sheets.
// Evidence: docs/evidence/asx.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2025-01-01 - T1 - ASX-CAL-2025 - New Year's Day, `CLOSED`.
        (2025, 1, 1, Closed, T1, "ASX-CAL-2025"),
        // 2025-01-27 - T1 - ASX-CAL-2025 - Australia Day, `CLOSED` (the
        // Monday after the Sunday 26 January holiday).
        (2025, 1, 27, Closed, T1, "ASX-CAL-2025"),
        // 2025-04-18 - T1 - ASX-CAL-2025 - Good Friday, `CLOSED`.
        (2025, 4, 18, Closed, T1, "ASX-CAL-2025"),
        // 2025-04-21 - T1 - ASX-CAL-2025 - Easter Monday, `CLOSED`.
        (2025, 4, 21, Closed, T1, "ASX-CAL-2025"),
        // 2025-04-25 - T1 - ASX-CAL-2025 - ANZAC Day, `CLOSED`.
        (2025, 4, 25, Closed, T1, "ASX-CAL-2025"),
        // 2025-06-09 - T1 - ASX-CAL-2025 - King's Birthday, `CLOSED`.
        (2025, 6, 9, Closed, T1, "ASX-CAL-2025"),
        // 2025-12-24 - T1 - ASX-CAL-2025 - Last Business day before Christmas
        // Day, `CLOSE EARLY`: `Normal trading ceases at 14:10 (Sydney time)`.
        (2025, 12, 24, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2025"),
        // 2025-12-25 - T1 - ASX-CAL-2025 - Christmas Day, `CLOSED`.
        (2025, 12, 25, Closed, T1, "ASX-CAL-2025"),
        // 2025-12-26 - T1 - ASX-CAL-2025 - Boxing Day, `CLOSED`.
        (2025, 12, 26, Closed, T1, "ASX-CAL-2025"),
        // 2025-12-31 - T1 - ASX-CAL-2025 - Last Business day of the Year,
        // `CLOSE EARLY` at 14:10.
        (2025, 12, 31, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-2025"),
        // 2026-01-01 - T1 - ASX-CAL-LIVE - New Year's Day, `CLOSED`.
        (2026, 1, 1, Closed, T1, "ASX-CAL-LIVE"),
        // 2026-01-26 - T1 - ASX-CAL-LIVE - Australia Day, `CLOSED`.
        (2026, 1, 26, Closed, T1, "ASX-CAL-LIVE"),
        // 2026-04-03 - T1 - ASX-CAL-LIVE - Good Friday, `CLOSED`.
        (2026, 4, 3, Closed, T1, "ASX-CAL-LIVE"),
        // 2026-04-06 - T1 - ASX-CAL-LIVE - Easter Monday, `CLOSED`.
        (2026, 4, 6, Closed, T1, "ASX-CAL-LIVE"),
        // 2026-04-25 - T1 - ASX-CAL-LIVE - ANZAC Day, `CLOSED`: the sheet
        // prints the Saturday with no substitute, so it restates the normal
        // week's closure.
        (2026, 4, 25, Closed, T1, "ASX-CAL-LIVE"),
        // 2026-06-08 - T1 - ASX-CAL-LIVE - King's Birthday, `CLOSED`.
        (2026, 6, 8, Closed, T1, "ASX-CAL-LIVE"),
        // 2026-12-24 - T1 - ASX-CAL-LIVE - Last Business day before Christmas
        // Day, `CLOSE EARLY` at 14:10.
        (2026, 12, 24, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-LIVE"),
        // 2026-12-25 - T1 - ASX-CAL-LIVE - Christmas Day, `CLOSED`.
        (2026, 12, 25, Closed, T1, "ASX-CAL-LIVE"),
        // 2026-12-28 - T1 - ASX-CAL-LIVE - Boxing Day, `CLOSED` (Monday, the
        // sheet's own date for the Saturday 26 December holiday).
        (2026, 12, 28, Closed, T1, "ASX-CAL-LIVE"),
        // 2026-12-31 - T1 - ASX-CAL-LIVE - Last Business day of the Year,
        // `CLOSE EARLY` at 14:10.
        (2026, 12, 31, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-LIVE"),
        // 2027-01-01 - T1 - ASX-CAL-LIVE - New Year's Day, `CLOSED`.
        (2027, 1, 1, Closed, T1, "ASX-CAL-LIVE"),
        // 2027-01-26 - T1 - ASX-CAL-LIVE - Australia Day, `CLOSED`.
        (2027, 1, 26, Closed, T1, "ASX-CAL-LIVE"),
        // 2027-03-26 - T1 - ASX-CAL-LIVE - Good Friday, `CLOSED`.
        (2027, 3, 26, Closed, T1, "ASX-CAL-LIVE"),
        // 2027-03-29 - T1 - ASX-CAL-LIVE - Easter Monday, `CLOSED`.
        (2027, 3, 29, Closed, T1, "ASX-CAL-LIVE"),
        // 2027-06-14 - T1 - ASX-CAL-LIVE - King's Birthday, `CLOSED`.
        (2027, 6, 14, Closed, T1, "ASX-CAL-LIVE"),
        // 2027-12-24 - T1 - ASX-CAL-LIVE - Last Business day before Christmas
        // Day, `CLOSE EARLY` at 14:10.
        (2027, 12, 24, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-LIVE"),
        // 2027-12-27 - T1 - ASX-CAL-LIVE - Christmas Day, `CLOSED`: the
        // sheet's substitute for Saturday 25 December.
        (2027, 12, 27, Closed, T1, "ASX-CAL-LIVE"),
        // 2027-12-28 - T1 - ASX-CAL-LIVE - Boxing Day, `CLOSED`: the sheet's
        // substitute for Sunday 26 December.
        (2027, 12, 28, Closed, T1, "ASX-CAL-LIVE"),
        // 2027-12-31 - T1 - ASX-CAL-LIVE - Last Business day of the Year,
        // `CLOSE EARLY` at 14:10.
        (2027, 12, 31, early_close(14 * 3_600 + 10 * 60), T1, "ASX-CAL-LIVE"),
    ],
};
