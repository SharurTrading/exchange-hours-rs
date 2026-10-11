// SPDX-License-Identifier: MIT-0

//! OANDA US index and bond book holiday rows, from the operator's UK
//! holiday page's dated sections 2025-01-01 through 2026-10-30.
//!
//! Keyed by the crate's own venue-local trade date in `America/New_York`
//! (design memo D1). This book is the page's `US indices and bonds` group —
//! including its split `US Indices` / `US Bonds` rows — plus the `JP225`
//! rows the page prints separately, which state the US indices' own
//! arrangement on every date they appear (Christmas Eve 13:14, Christmas
//! Day and New Year reopen 18:01, New Year's Eve 16:59), and the `US index
//! futures` / `US bonds` wording of the 2025 archive. Two withheld dates
//! ship `Unsourced` rather than a guess: 2025-01-09, whose `US index close
//! early at 9:30` carries no meridiem and matches no other printed instant
//! (the same section dates the bonds' 13:15 close), and nothing else — the
//! Good Friday 2026 split instants ship as the group's narrowest window
//! (indices 9:14; the bonds' later 11:14 is recorded in the evidence file)
//! per the sourced-intersection rule.
//!
//! Every row is **T1**, the operator's own holiday page. All of it is
//! recorded in
//! [`docs/evidence/oanda_us_index_cfd.md`](../../../../../docs/evidence/oanda_us_index_cfd.md).

use super::fences::early_close;
use super::{
    EvidenceTier::T1,
    HolidayKind::{Closed, Unsourced},
    HolidayTable, holidays,
};

/// The book's built-in holiday rows and the window they were audited over.
// Evidence: docs/evidence/oanda_us_index_cfd.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2026, 10, 30)],
    rows: [
        // 2025-01-01 - T1 - OANDA-UK-HOLIDAYS - New Year's Day: US indices and US bonds open late 18:04
        (2025, 1, 1, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-01-09 - T1 - OANDA-UK-HOLIDAYS - Carter's Day of Mourning: "US index close early at 9:30" prints no
        // meridiem and matches no other printed instant; the bonds' 13:15 close is recorded in the evidence file
        (2025, 1, 9, Unsourced, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-02-17 - T1 - OANDA-UK-HOLIDAYS - Presidents Day: closes 1 pm, reopens 6 pm
        (2025, 2, 17, early_close(13 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-04-17 - T1 - OANDA-UK-HOLIDAYS - Holy Thursday: closes 17:00 ET
        (2025, 4, 17, early_close(17 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-04-18 - T1 - OANDA-UK-HOLIDAYS - Good Friday: closed
        (2025, 4, 18, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-05-26 - T1 - OANDA-UK-HOLIDAYS - Memorial Day: closes 1:00 PM, reopens 6:00 PM
        (2025, 5, 26, early_close(13 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-06-19 - T1 - OANDA-UK-HOLIDAYS - Juneteenth: closes 1:00 PM, reopens 6:00 PM
        (2025, 6, 19, early_close(13 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-07-03 - T1 - OANDA-UK-HOLIDAYS - Independence Day eve: closes 1:15 pm, reopens 6 pm
        (2025, 7, 3, early_close(13 * 3_600 + 15 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-07-04 - T1 - OANDA-UK-HOLIDAYS - Independence Day: closes 1 pm
        (2025, 7, 4, early_close(13 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-09-01 - T1 - OANDA-UK-HOLIDAYS - Labor Day: closes 1:00 PM, reopens 6:00 PM
        (2025, 9, 1, early_close(13 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-11-27 - T1 - OANDA-UK-HOLIDAYS - Thanksgiving: closes 1:00 PM, reopens 6:00 PM
        (2025, 11, 27, early_close(13 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-11-28 - T1 - OANDA-UK-HOLIDAYS - Thanksgiving Friday: closes 1:15 PM
        (2025, 11, 28, early_close(13 * 3_600 + 15 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-12-24 - T1 - OANDA-UK-HOLIDAYS - Christmas Eve: JP225 and US indices close 13:14, US bonds close 13:14
        (2025, 12, 24, early_close(13 * 3_600 + 14 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-12-25 - T1 - OANDA-UK-HOLIDAYS - Christmas Day: JP225, US indices and US bonds open 18:01
        (2025, 12, 25, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2026-01-01 - T1 - OANDA-UK-HOLIDAYS - New Year's Day: JP225, US indices and US bonds open 18:01
        (2026, 1, 1, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2026-01-19 - T1 - OANDA-UK-HOLIDAYS - MLK Day: closes 12:59, reopens 18:01
        (2026, 1, 19, early_close(12 * 3_600 + 59 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-02-16 - T1 - OANDA-UK-HOLIDAYS - Presidents Day: closes 12:59, reopens 18:01
        (2026, 2, 16, early_close(12 * 3_600 + 59 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-04-03 - T1 - OANDA-UK-HOLIDAYS - Good Friday: US Indices close 9:14, US Bonds close 11:14 — the
        // group ships the narrowest sourced window and the evidence file records the bonds' later close
        (2026, 4, 3, early_close(9 * 3_600 + 14 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-05-25 - T1 - OANDA-UK-HOLIDAYS - Memorial Day: closes 12:59, reopens 18:01
        (2026, 5, 25, early_close(12 * 3_600 + 59 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-06-19 - T1 - OANDA-UK-HOLIDAYS - Juneteenth: closes 12:59
        (2026, 6, 19, early_close(12 * 3_600 + 59 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-07-03 - T1 - OANDA-UK-HOLIDAYS - Independence Day: closes 12:59
        (2026, 7, 3, early_close(12 * 3_600 + 59 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-09-07 - T1 - OANDA-UK-HOLIDAYS - Labor Day: closes 12:59, reopens 18:01
        (2026, 9, 7, early_close(12 * 3_600 + 59 * 60), T1, "OANDA-UK-HOLIDAYS"),
    ],
};
