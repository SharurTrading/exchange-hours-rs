// SPDX-License-Identifier: MIT-0

//! OANDA spot-metals book holiday rows, from the operator's UK holiday
//! page's dated sections 2025-01-01 through 2026-10-30.
//!
//! Keyed by the crate's own venue-local trade date in `America/New_York`
//! (design memo D1). The page's "US energies and metals" group covers the
//! gold and silver books for closes — its Memorial Day 2026 row prints the
//! structure outright, `Close at 14:29; reopen at 18:01 (Gold, Silver and
//! their crosses reopen at 18:05)` — so each group close ships here at the
//! group's printed instant, and the reopen is always the book's own normal
//! 18:05 open, which no row states and no row needs. The standalone
//! `Metals:` rows the page prints on reopen-only dates (`open at 18:01`,
//! `open late 18:04`) belong to the commodity-metals listings of
//! [`oanda_us_commodity_cfd`](super::oanda_us_commodity_cfd): 18:04 is late
//! against that book's 18:01 open and impossible against this book's 18:05.
//! Full-closure dates ship `Closed`, the trade date's session not trading
//! until the evening reopen.
//!
//! Every row is **T1**, the operator's own holiday page. All of it is
//! recorded in [`docs/evidence/oanda_metals.md`](../../../../../docs/evidence/oanda_metals.md).

use super::fences::early_close;
use super::{EvidenceTier::T1, HolidayKind::Closed, HolidayTable, holidays};

/// The book's built-in holiday rows and the window they were audited over.
// Evidence: docs/evidence/oanda_metals.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2026, 10, 30)],
    rows: [
        // 2025-01-01 - T1 - OANDA-UK-HOLIDAYS - New Year's Day: US holiday groups reopen at the evening open
        (2025, 1, 1, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-02-17 - T1 - OANDA-UK-HOLIDAYS - Presidents Day: group closes 2.30pm ET
        (2025, 2, 17, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-04-17 - T1 - OANDA-UK-HOLIDAYS - Holy Thursday: group closes 17:00 ET
        (2025, 4, 17, early_close(17 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-04-18 - T1 - OANDA-UK-HOLIDAYS - Good Friday: group closed
        (2025, 4, 18, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-05-26 - T1 - OANDA-UK-HOLIDAYS - Memorial Day: group closes 2:30 PM ET
        (2025, 5, 26, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-06-19 - T1 - OANDA-UK-HOLIDAYS - Juneteenth: group closes 2:30 PM ET
        (2025, 6, 19, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-07-04 - T1 - OANDA-UK-HOLIDAYS - Independence Day: group closes 1 pm ET
        (2025, 7, 4, early_close(13 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-09-01 - T1 - OANDA-UK-HOLIDAYS - Labor Day: group closes 2:30 PM ET
        (2025, 9, 1, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-11-27 - T1 - OANDA-UK-HOLIDAYS - Thanksgiving: group closes 2:30 PM ET
        (2025, 11, 27, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-11-28 - T1 - OANDA-UK-HOLIDAYS - Thanksgiving Friday: group closes 2:45 PM ET
        (2025, 11, 28, early_close(14 * 3_600 + 45 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-12-24 - T1 - OANDA-UK-HOLIDAYS - Christmas Eve: Metals close at 13:44
        (2025, 12, 24, early_close(13 * 3_600 + 44 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-12-25 - T1 - OANDA-UK-HOLIDAYS - Christmas Day: closed until the evening open
        (2025, 12, 25, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2026-01-01 - T1 - OANDA-UK-HOLIDAYS - New Year's Day: closed until the evening open
        (2026, 1, 1, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2026-01-19 - T1 - OANDA-UK-HOLIDAYS - MLK Day: group closes 14:29
        (2026, 1, 19, early_close(14 * 3_600 + 29 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-02-16 - T1 - OANDA-UK-HOLIDAYS - Presidents Day: group closes 14:29
        (2026, 2, 16, early_close(14 * 3_600 + 29 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-04-03 - T1 - OANDA-UK-HOLIDAYS - Good Friday: US Energies and metals Closed
        (2026, 4, 3, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2026-05-25 - T1 - OANDA-UK-HOLIDAYS - Memorial Day: the page's own gold/silver callout, close 14:29,
        // reopen at the book's own 18:05
        (2026, 5, 25, early_close(14 * 3_600 + 29 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-06-19 - T1 - OANDA-UK-HOLIDAYS - Juneteenth: group closes 12:59
        (2026, 6, 19, early_close(12 * 3_600 + 59 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-07-03 - T1 - OANDA-UK-HOLIDAYS - Independence Day: group closes 12:59
        (2026, 7, 3, early_close(12 * 3_600 + 59 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-09-07 - T1 - OANDA-UK-HOLIDAYS - Labor Day: group closes 14:29
        (2026, 9, 7, early_close(14 * 3_600 + 29 * 60), T1, "OANDA-UK-HOLIDAYS"),
    ],
};
