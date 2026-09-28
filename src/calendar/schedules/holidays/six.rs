// SPDX-License-Identifier: MIT-0

//! SIX Swiss Exchange holiday rows, 2025-2027.
//!
//! Keyed by the crate's own venue-local trade date in `Europe/Zurich`. SIX
//! publishes one `Trading Calendar` PDF per year — an operator document in the
//! Trading Guide set — whose month grids mark every non-trading day: light
//! shades for Saturday and Sunday and a dark cell for `Market Holiday —
//! Market Closed`. The whole block is **T1**; each year's rows cite that
//! year's PDF.
//!
//! The calendars print closures only: no half day, no late open and no
//! intraday instant anywhere in the three years, so `Closed` is the only kind
//! the operator's own statement supports and none other is invented. Holidays
//! that fall on a weekend are not marked (the Saturday/Sunday shading already
//! deletes them) and key no weekday row; the 2026-08-01 and 2026-12-26 and
//! the 2027-01-02, 2027-05-01, 2027-08-01, 2027-12-25 and 2027-12-26 holidays
//! are those cases. The per-cell derivation is recorded in
//! [`docs/evidence/six.md`](../../../../../docs/evidence/six.md).

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::{HolidayTable, holidays};

/// SIX's built-in holiday rows and the window they were audited over.
///
/// Every row is one dark `Market Holiday — Market Closed` cell of the year's
/// own `Trading Calendar` PDF. A date inside the window with no row is audited
/// normal.
// Evidence: docs/evidence/six.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2025-01-01 - T1 - SIX-TC-2025 - New Year's Day.
        (2025, 1, 1, Closed, T1, "SIX-TC-2025"),
        // 2025-01-02 - T1 - SIX-TC-2025 - St. Berchtold Day.
        (2025, 1, 2, Closed, T1, "SIX-TC-2025"),
        // 2025-04-18 - T1 - SIX-TC-2025 - Good Friday.
        (2025, 4, 18, Closed, T1, "SIX-TC-2025"),
        // 2025-04-21 - T1 - SIX-TC-2025 - Easter Monday.
        (2025, 4, 21, Closed, T1, "SIX-TC-2025"),
        // 2025-05-01 - T1 - SIX-TC-2025 - Labour Day.
        (2025, 5, 1, Closed, T1, "SIX-TC-2025"),
        // 2025-05-29 - T1 - SIX-TC-2025 - Ascension Day.
        (2025, 5, 29, Closed, T1, "SIX-TC-2025"),
        // 2025-06-09 - T1 - SIX-TC-2025 - Whit Monday.
        (2025, 6, 9, Closed, T1, "SIX-TC-2025"),
        // 2025-08-01 - T1 - SIX-TC-2025 - Swiss National Day.
        (2025, 8, 1, Closed, T1, "SIX-TC-2025"),
        // 2025-12-24 - T1 - SIX-TC-2025 - Christmas Eve.
        (2025, 12, 24, Closed, T1, "SIX-TC-2025"),
        // 2025-12-25 - T1 - SIX-TC-2025 - Christmas Day.
        (2025, 12, 25, Closed, T1, "SIX-TC-2025"),
        // 2025-12-26 - T1 - SIX-TC-2025 - St. Stephen's Day.
        (2025, 12, 26, Closed, T1, "SIX-TC-2025"),
        // 2025-12-31 - T1 - SIX-TC-2025 - New Year's Eve.
        (2025, 12, 31, Closed, T1, "SIX-TC-2025"),
        // 2026-01-01 - T1 - SIX-TC-2026 - New Year's Day.
        (2026, 1, 1, Closed, T1, "SIX-TC-2026"),
        // 2026-01-02 - T1 - SIX-TC-2026 - St. Berchtold Day.
        (2026, 1, 2, Closed, T1, "SIX-TC-2026"),
        // 2026-04-03 - T1 - SIX-TC-2026 - Good Friday.
        (2026, 4, 3, Closed, T1, "SIX-TC-2026"),
        // 2026-04-06 - T1 - SIX-TC-2026 - Easter Monday.
        (2026, 4, 6, Closed, T1, "SIX-TC-2026"),
        // 2026-05-01 - T1 - SIX-TC-2026 - Labour Day.
        (2026, 5, 1, Closed, T1, "SIX-TC-2026"),
        // 2026-05-14 - T1 - SIX-TC-2026 - Ascension Day.
        (2026, 5, 14, Closed, T1, "SIX-TC-2026"),
        // 2026-05-25 - T1 - SIX-TC-2026 - Whit Monday.
        (2026, 5, 25, Closed, T1, "SIX-TC-2026"),
        // 2026-12-24 - T1 - SIX-TC-2026 - Christmas Eve.
        (2026, 12, 24, Closed, T1, "SIX-TC-2026"),
        // 2026-12-25 - T1 - SIX-TC-2026 - Christmas Day.
        (2026, 12, 25, Closed, T1, "SIX-TC-2026"),
        // 2026-12-31 - T1 - SIX-TC-2026 - New Year's Eve. The calendar marks
        // no weekday holiday for Swiss National Day 2026 (Saturday) or St.
        // Stephen's Day 2026 (Saturday).
        (2026, 12, 31, Closed, T1, "SIX-TC-2026"),
        // 2027-01-01 - T1 - SIX-TC-2027 - New Year's Day. St. Berchtold Day
        // 2027 falls on Saturday and is marked as a weekend.
        (2027, 1, 1, Closed, T1, "SIX-TC-2027"),
        // 2027-03-26 - T1 - SIX-TC-2027 - Good Friday.
        (2027, 3, 26, Closed, T1, "SIX-TC-2027"),
        // 2027-03-29 - T1 - SIX-TC-2027 - Easter Monday.
        (2027, 3, 29, Closed, T1, "SIX-TC-2027"),
        // 2027-05-06 - T1 - SIX-TC-2027 - Ascension Day.
        (2027, 5, 6, Closed, T1, "SIX-TC-2027"),
        // 2027-05-17 - T1 - SIX-TC-2027 - Whit Monday. Labour Day 2027 falls
        // on Saturday and is marked as a weekend.
        (2027, 5, 17, Closed, T1, "SIX-TC-2027"),
        // 2027-12-24 - T1 - SIX-TC-2027 - Christmas Eve. Christmas Day 2027
        // (Saturday) and St. Stephen's Day 2027 (Sunday) are marked as a
        // weekend, and Swiss National Day 2027 (Sunday) likewise.
        (2027, 12, 24, Closed, T1, "SIX-TC-2027"),
        // 2027-12-31 - T1 - SIX-TC-2027 - New Year's Eve.
        (2027, 12, 31, Closed, T1, "SIX-TC-2027"),
    ],
};
