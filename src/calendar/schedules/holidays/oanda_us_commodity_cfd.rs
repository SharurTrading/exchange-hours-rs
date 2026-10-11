// SPDX-License-Identifier: MIT-0

//! OANDA US energy and commodity-metals book holiday rows, from the
//! operator's UK holiday page's dated sections 2025-01-01 through
//! 2026-10-30.
//!
//! Keyed by the crate's own venue-local trade date in `America/New_York`
//! (design memo D1). This book is the page's own `US energies and metals`
//! group — Natural Gas, West Texas Oil, Platinum, Palladium and Copper —
//! plus the standalone `US energies`/`Metals` rows the page prints on the
//! Christmas and New Year sections, which name this book (18:01 and 18:04
//! reopens are late against its 18:01 normal open and meaningless against
//! the spot-metals book's 18:05). Early closes ship at the printed instant;
//! the stated reopens are the book's own normal evening open; full-closure
//! dates ship `Closed`.
//!
//! Every row is **T1**, the operator's own holiday page. All of it is
//! recorded in
//! [`docs/evidence/oanda_us_commodity_cfd.md`](../../../../../docs/evidence/oanda_us_commodity_cfd.md).

use super::fences::early_close;
use super::{EvidenceTier::T1, HolidayKind::Closed, HolidayTable, holidays};

/// The book's built-in holiday rows and the window they were audited over.
// Evidence: docs/evidence/oanda_us_commodity_cfd.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2026, 10, 30)],
    rows: [
        // 2025-01-01 - T1 - OANDA-UK-HOLIDAYS - New Year's Day: US energies open late 18:04, Metals open late 18:04
        (2025, 1, 1, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-02-17 - T1 - OANDA-UK-HOLIDAYS - Presidents Day: closes 2.30 pm, reopens 6 pm
        (2025, 2, 17, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-04-17 - T1 - OANDA-UK-HOLIDAYS - Holy Thursday: closes 17:00 ET
        (2025, 4, 17, early_close(17 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-04-18 - T1 - OANDA-UK-HOLIDAYS - Good Friday: closed
        (2025, 4, 18, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-05-26 - T1 - OANDA-UK-HOLIDAYS - Memorial Day: closes 2:30 PM, reopens 6:00 PM
        (2025, 5, 26, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-06-19 - T1 - OANDA-UK-HOLIDAYS - Juneteenth: closes 2:30 PM, reopens 6:00 PM
        (2025, 6, 19, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-07-04 - T1 - OANDA-UK-HOLIDAYS - Independence Day: closes 1 pm
        (2025, 7, 4, early_close(13 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-09-01 - T1 - OANDA-UK-HOLIDAYS - Labor Day: closes 2:30 PM, reopens 6:00 PM
        (2025, 9, 1, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-11-27 - T1 - OANDA-UK-HOLIDAYS - Thanksgiving: closes 2:30 PM, reopens 6:00 PM
        (2025, 11, 27, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-11-28 - T1 - OANDA-UK-HOLIDAYS - Thanksgiving Friday: closes 2:45 PM
        (2025, 11, 28, early_close(14 * 3_600 + 45 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-12-24 - T1 - OANDA-UK-HOLIDAYS - Christmas Eve: US energies close 13:44, Metals close 13:44
        (2025, 12, 24, early_close(13 * 3_600 + 44 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-12-25 - T1 - OANDA-UK-HOLIDAYS - Christmas Day: US energies open 18:01, Metals open 18:01
        (2025, 12, 25, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2026-01-01 - T1 - OANDA-UK-HOLIDAYS - New Year's Day: US energies open 18:01, Metals open 18:01
        (2026, 1, 1, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2026-01-19 - T1 - OANDA-UK-HOLIDAYS - MLK Day: closes 14:29, reopens 18:01
        (2026, 1, 19, early_close(14 * 3_600 + 29 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-02-16 - T1 - OANDA-UK-HOLIDAYS - Presidents Day: closes 14:29, reopens 18:01
        (2026, 2, 16, early_close(14 * 3_600 + 29 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-04-03 - T1 - OANDA-UK-HOLIDAYS - Good Friday: US Energies and metals Closed
        (2026, 4, 3, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2026-05-25 - T1 - OANDA-UK-HOLIDAYS - Memorial Day: closes 14:29, reopens 18:01
        (2026, 5, 25, early_close(14 * 3_600 + 29 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-06-19 - T1 - OANDA-UK-HOLIDAYS - Juneteenth: closes 12:59
        (2026, 6, 19, early_close(12 * 3_600 + 59 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-07-03 - T1 - OANDA-UK-HOLIDAYS - Independence Day: closes 12:59
        (2026, 7, 3, early_close(12 * 3_600 + 59 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-09-07 - T1 - OANDA-UK-HOLIDAYS - Labor Day: closes 14:29, reopens 18:01
        (2026, 9, 7, early_close(14 * 3_600 + 29 * 60), T1, "OANDA-UK-HOLIDAYS"),
    ],
};
