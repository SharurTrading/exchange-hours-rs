// SPDX-License-Identifier: MIT-0

//! Eurex holiday rows, 2025-2026.
//!
//! Keyed by the crate's own venue-local trade date in `Europe/Berlin` (design
//! memo D1). The conversion is the identity here: every Eurex phase the crate
//! models runs inside one Berlin civil day, so no session wraps a midnight and
//! an event date is its own trade date.
//!
//! The block is **T1**: Eurex's own `Holiday regulations` page, § 2025 as
//! captured on 2025-09-13 and § 2026, each corroborated by the matching Trading
//! Calendar PDF. Eurex publishes no early closes — 24 and 31 December are full
//! trading closures with clearing open — so every row is `Closed`.
//!
//! Coverage stops at 2026-12-31. Eurex's 2027-2036 calendars exist only "on a
//! preliminary and indicative basis … and are subject to change", which is not
//! an unconditional dated future, so LAW-NO-FABRICATED-DATES keeps them out.
//! That, and the unresolved "to be announced" line for additional German
//! closures in FDAX/FDXM, are recorded in
//! [`docs/evidence/eurex.md`](../../../../../docs/evidence/eurex.md).
//!
//! One table serves all three Eurex identities. The operator states the
//! closure for "all derivatives", which covers the index families behind the
//! `eurex` rows and the fixed-income families behind `eurex_fixed_income`
//! alike, so the venue intersection of design memo D17 is that same table.

use super::{EvidenceTier::T1, HolidayKind::Closed, HolidayTable, holidays};

/// Eurex's built-in holiday rows and the window they were audited over.
// Evidence: docs/evidence/eurex.md, docs/evidence/eurex_key.md,
// docs/evidence/eurex_fixed_income.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2026, 12, 31)],
    rows: [
        // 2025-01-01 - T1 - EUREX-HOLREG-2025 - New Year's Day, trading and clearing.
        (2025, 1, 1, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-04-18 - T1 - EUREX-HOLREG-2025 - Good Friday, trading and clearing.
        (2025, 4, 18, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-04-21 - T1 - EUREX-HOLREG-2025 - Easter Monday, trading and clearing.
        (2025, 4, 21, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-05-01 - T1 - EUREX-HOLREG-2025 - Labour Day, trading and clearing.
        (2025, 5, 1, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-12-24 - T1 - EUREX-HOLREG-2025 - Christmas Eve, a full trading closure.
        (2025, 12, 24, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-12-25 - T1 - EUREX-HOLREG-2025 - Christmas Day, trading and clearing.
        (2025, 12, 25, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-12-26 - T1 - EUREX-HOLREG-2025 - Boxing Day, trading and clearing.
        (2025, 12, 26, Closed, T1, "EUREX-HOLREG-2025"),
        // 2025-12-31 - T1 - EUREX-HOLREG-2025 - New Year's Eve, a full trading closure.
        (2025, 12, 31, Closed, T1, "EUREX-HOLREG-2025"),
        // 2026-01-01 - T1 - EUREX-HOLREG-2026 - New Year's Day, trading and clearing.
        (2026, 1, 1, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-04-03 - T1 - EUREX-HOLREG-2026 - Good Friday, trading and clearing.
        (2026, 4, 3, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-04-06 - T1 - EUREX-HOLREG-2026 - Easter Monday, trading and clearing.
        (2026, 4, 6, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-05-01 - T1 - EUREX-HOLREG-2026 - Labour Day, trading and clearing.
        (2026, 5, 1, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-12-24 - T1 - EUREX-HOLREG-2026 - Christmas Eve, a full trading closure.
        (2026, 12, 24, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-12-25 - T1 - EUREX-HOLREG-2026 - Christmas Day, trading and clearing.
        (2026, 12, 25, Closed, T1, "EUREX-HOLREG-2026"),
        // 2026-12-31 - T1 - EUREX-HOLREG-2026 - New Year's Eve, a full trading closure.
        (2026, 12, 31, Closed, T1, "EUREX-HOLREG-2026"),
    ],
};
