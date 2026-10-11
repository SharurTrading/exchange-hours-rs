// SPDX-License-Identifier: MIT-0

//! OANDA Brent-book holiday rows, from the operator's UK holiday page's
//! dated sections 2025-01-01 through 2026-10-30.
//!
//! Keyed by the crate's own venue-local trade date in `Europe/London`
//! (design memo D1). The page prints every instant in Eastern Time and this
//! book keeps London time, so each printed instant is converted at the
//! ET-to-London offset in force on its own date — five hours on every row
//! here, verified date by date: none of these holidays falls inside the
//! two-three-week windows where `America/New_York` and `Europe/London` disagree
//! (which in 2025-2026 run roughly March 9-29 and October 26-November 2).
//! Four dates ship `Unsourced` rather than a guess: the page prints other
//! groups' arrangements on them but is silent on Brent — and unlike the FX
//! and US groups, this page has an explicit "Brent Crude Oil: Regular
//! trading hours" vocabulary it uses when the book runs normal, so its
//! silence there withholds rather than affirms (MLK and Presidents Day
//! 2025/2026 print that regular-hours row and ship no row). Early closes
//! ship at the converted instant; the stated reopens are the book's own
//! normal 01:01 evening open.
//!
//! Every row is **T1**, the operator's own holiday page. All of it is
//! recorded in [`docs/evidence/oanda_brent.md`](../../../../../docs/evidence/oanda_brent.md).

use super::fences::early_close;
use super::{
    EvidenceTier::T1,
    HolidayKind::{Closed, Unsourced},
    HolidayTable, holidays,
};

/// The book's built-in holiday rows and the window they were audited over.
// Evidence: docs/evidence/oanda_brent.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2026, 10, 30)],
    rows: [
        // 2025-01-01 - T1 - OANDA-UK-HOLIDAYS - New Year's Day: Brent Crude Oil is closed
        (2025, 1, 1, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-01-09 - T1 - OANDA-UK-HOLIDAYS - Carter's Day of Mourning: the section prints US index/bonds and
        // Commodities rows and nothing for Brent, whose regular-hours vocabulary makes silence a withholding
        (2025, 1, 9, Unsourced, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-04-17 - T1 - OANDA-UK-HOLIDAYS - Holy Thursday: the section prints US and Japan rows, Brent silent
        (2025, 4, 17, Unsourced, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-04-18 - T1 - OANDA-UK-HOLIDAYS - Good Friday: BCO is closed
        (2025, 4, 18, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-05-26 - T1 - OANDA-UK-HOLIDAYS - Memorial Day: closes 1:30 PM ET (18:30 London), reopens 8:00 PM ET
        (2025, 5, 26, early_close(18 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-06-19 - T1 - OANDA-UK-HOLIDAYS - Juneteenth: closes 1:30 PM ET, reopens 8:00 PM ET
        (2025, 6, 19, early_close(18 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-07-03 - T1 - OANDA-UK-HOLIDAYS - Independence Day eve: the section prints US index and Commodities
        // rows and nothing for Brent
        (2025, 7, 3, Unsourced, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-07-04 - T1 - OANDA-UK-HOLIDAYS - Independence Day: closes 1:30 pm ET
        (2025, 7, 4, early_close(18 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-09-01 - T1 - OANDA-UK-HOLIDAYS - Labor Day: closes 1:30 PM ET, reopens 8:00 PM ET
        (2025, 9, 1, early_close(18 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-11-26 - T1 - OANDA-UK-HOLIDAYS - Thanksgiving eve: the section prints one Agricultural row, Brent silent
        (2025, 11, 26, Unsourced, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-11-27 - T1 - OANDA-UK-HOLIDAYS - Thanksgiving: closes 1:30 PM ET, reopens 8:00 PM ET
        (2025, 11, 27, early_close(18 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-11-28 - T1 - OANDA-UK-HOLIDAYS - Thanksgiving Friday: closes 3:00 PM ET (20:00 London)
        (2025, 11, 28, early_close(20 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-12-24 - T1 - OANDA-UK-HOLIDAYS - Christmas Eve: closes 13:59 ET (18:59 London)
        (2025, 12, 24, early_close(18 * 3_600 + 59 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-12-25 - T1 - OANDA-UK-HOLIDAYS - Christmas Day: closed
        (2025, 12, 25, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-12-31 - T1 - OANDA-UK-HOLIDAYS - New Year's Eve: closes 14:59 ET (19:59 London)
        (2025, 12, 31, early_close(19 * 3_600 + 59 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-01-01 - T1 - OANDA-UK-HOLIDAYS - New Year's Day: closed
        (2026, 1, 1, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2026-04-03 - T1 - OANDA-UK-HOLIDAYS - Good Friday: Brent Crude Oil Closed
        (2026, 4, 3, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2026-05-25 - T1 - OANDA-UK-HOLIDAYS - Memorial Day: closes 13:29 ET (18:29 London), reopens 20:01 ET
        (2026, 5, 25, early_close(18 * 3_600 + 29 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-06-19 - T1 - OANDA-UK-HOLIDAYS - Juneteenth: closes 13:29 ET
        (2026, 6, 19, early_close(18 * 3_600 + 29 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-07-03 - T1 - OANDA-UK-HOLIDAYS - Independence Day: closes 13:29 ET
        (2026, 7, 3, early_close(18 * 3_600 + 29 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-09-07 - T1 - OANDA-UK-HOLIDAYS - Labor Day: closes 13:29 ET, reopens 20:01 ET
        (2026, 9, 7, early_close(18 * 3_600 + 29 * 60), T1, "OANDA-UK-HOLIDAYS"),
    ],
};
