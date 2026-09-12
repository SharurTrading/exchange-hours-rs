// SPDX-License-Identifier: MIT-0

//! Coinbase Derivatives holiday rows, 2026 through Labor Day.
//!
//! Keyed by the crate's own venue-local trade date in `America/Chicago`
//! (design memo D1). CDE's Market Notices are already keyed by **trade date**
//! and print the open, close and roll instants of the neighbouring dates
//! beside each one, so the conversion is the identity and the operator's own
//! table corroborates it row for row.
//!
//! The venue default this table attaches to is CDE's recurring 23x5 futures
//! grid — Sunday to Friday, 17:00-16:00 CT with the daily 16:00-17:00 break —
//! so every row states what the notices state for the 23x5 tier. The 24x7
//! tier, which trades through each of these dates, is out of scope for the
//! venue profile and claims no key, and Gold and Silver moved into that tier on
//! trade date 2026-06-15; both are recorded in
//! [`docs/evidence/coinbase_derivatives.md`](../../../../../docs/evidence/coinbase_derivatives.md).
//!
//! Every row is **T1**, a numbered CDE Market Notice, and every row is a full
//! closure: CDE publishes no early close or late open for the 23x5 tier in this
//! window. Coverage stops at 2026-09-07 because the Thanksgiving and Christmas
//! notices had not issued at retrieval, and CDE has published nothing at all
//! for 2027.

use super::{EvidenceTier::T1, HolidayKind::Closed, HolidayTable, holidays};

/// The venue's built-in holiday rows and the window they were audited over.
// Evidence: docs/evidence/coinbase_derivatives.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: (2026, 1, 1) ..= (2026, 9, 7),
    rows: [
        // 2026-01-01 - T1 - CDE-MN-25-42 - New Year's Day, Energy & Metal and
        // Equity closed for holiday; trade date 1/2 opens 1/1 17:00 CT.
        (2026, 1, 1, Closed, T1, "CDE-MN-25-42"),
        // 2026-01-19 - T1 - CDE-MN-26-01 - Martin Luther King Jr. Day.
        (2026, 1, 19, Closed, T1, "CDE-MN-26-01"),
        // 2026-02-16 - T1 - CDE-MN-26-05 - Presidents' Day.
        (2026, 2, 16, Closed, T1, "CDE-MN-26-05"),
        // 2026-04-03 - T1 - CDE-MN-26-12 - Good Friday.
        (2026, 4, 3, Closed, T1, "CDE-MN-26-12"),
        // 2026-05-25 - T1 - CDE-MN-26-23 - Memorial Day.
        (2026, 5, 25, Closed, T1, "CDE-MN-26-23"),
        // 2026-06-19 - T1 - CDE-MN-26-27.1 - Juneteenth; the 23x5 and 24x5
        // tiers close while Gold and Silver stay open with the 24x7 tier.
        (2026, 6, 19, Closed, T1, "CDE-MN-26-27.1"),
        // 2026-07-03 - T1 - CDE-MN-26-29 - Independence Day observed.
        (2026, 7, 3, Closed, T1, "CDE-MN-26-29"),
        // 2026-09-07 - T1 - CDE-MN-26-36 - Labor Day.
        (2026, 9, 7, Closed, T1, "CDE-MN-26-36"),
    ],
};
