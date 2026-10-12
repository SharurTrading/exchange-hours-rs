// SPDX-License-Identifier: MIT-0

//! OANDA holiday data, 2024-02-21..2026-10-30. Evidence: `docs/evidence/oanda_brent.md`.

use super::fences::early_close;
use crate::calendar::ExceptionBlock;

use super::{
    EvidenceTier::T1,
    HolidayKind::{Closed, ReplacementBlocks, Unsourced},
    HolidayTable, holidays,
};

// Evidence: docs/evidence/oanda_brent.md
#[rustfmt::skip]
static WEEKDAY_REOPEN: [ExceptionBlock; 2] = [
    ExceptionBlock::regular(0, 3_600, 21 * 3_600 + 59 * 60),
    ExceptionBlock::regular(0, 22 * 3_600 + 5 * 60, 22 * 3_600 + 59 * 60),
];

#[rustfmt::skip]
static FRIDAY_REOPEN: [ExceptionBlock; 1] = [ExceptionBlock::regular(0, 3_600, 21 * 3_600 + 59 * 60)];

#[rustfmt::skip]
static THANKSGIVING_REOPEN: [ExceptionBlock; 1] = [ExceptionBlock::regular(0, 3_600, 20 * 3_600)];

/// The book's built-in holiday rows and the window they were audited over.
// Evidence: docs/evidence/oanda_brent.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2024, 2, 21) ..= (2026, 10, 30)],
    rows: [
        // 2024-03-29 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Good Friday
        (2024, 3, 29, Closed, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-04-01 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Easter Monday UK section names other books; Brent arrangement unstated (#332)
        (2024, 4, 1, Unsourced, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-05-27 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Memorial Day 13:30 ET close
        (2024, 5, 27, early_close(18 * 3_600 + 30 * 60), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-05-28 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Memorial Day Brent reopen unprinted; separate Commodities wording ambiguous (#332)
        (2024, 5, 28, Unsourced, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-06-19 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Juneteenth 13:30 ET close
        (2024, 6, 19, early_close(18 * 3_600 + 30 * 60), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-06-20 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Juneteenth Brent reopen unprinted; separate Commodities wording ambiguous (#332)
        (2024, 6, 20, Unsourced, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-07-03 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Independence eve section names only indices; Brent silent (#332)
        (2024, 7, 3, Unsourced, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-07-04 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Independence Day 13:30 ET close
        (2024, 7, 4, early_close(18 * 3_600 + 30 * 60), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-07-05 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Independence Day Brent reopen unprinted (#332)
        (2024, 7, 5, Unsourced, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-08-26 - T1 - OANDA-UK-CARTER-2025-WAYBACK - UK Summer Bank Holiday section names other books; Brent silent (#332)
        (2024, 8, 26, Unsourced, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-09-02 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Labor Day 13:30 ET close
        (2024, 9, 2, early_close(18 * 3_600 + 30 * 60), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-09-03 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Labor Day Brent reopen unprinted; separate Commodities wording ambiguous (#332)
        (2024, 9, 3, Unsourced, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-11-28 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Thanksgiving 13:30 ET close
        (2024, 11, 28, early_close(18 * 3_600 + 30 * 60), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-11-29 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Thanksgiving 20:00 ET reopen and 15:00 ET Friday close
        (2024, 11, 29, ReplacementBlocks(&THANKSGIVING_REOPEN), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-12-24 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Christmas Eve 13:59 ET close
        (2024, 12, 24, early_close(18 * 3_600 + 59 * 60), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-12-25 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Christmas Day
        (2024, 12, 25, Closed, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-12-31 - T1 - OANDA-UK-CARTER-2025-WAYBACK - New Years Eve 14:59 ET close
        (2024, 12, 31, early_close(19 * 3_600 + 59 * 60), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2025-01-01 - T1 - OANDA-UK-HOLIDAYS - New Year's Day: Brent Crude Oil is closed
        (2025, 1, 1, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-01-09 - T1 - OANDA-UK-HOLIDAYS - Carter's Day of Mourning: the section prints US index/bonds and
        // Commodities rows and nothing for Brent, whose regular-hours vocabulary makes silence a withholding
        (2025, 1, 9, Unsourced, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-04-17 - T1 - OANDA-UK-HOLIDAYS - Holy Thursday: the section prints US and Japan rows, Brent silent
        (2025, 4, 17, Unsourced, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-04-18 - T1 - OANDA-UK-HOLIDAYS - Good Friday: BCO is closed
        (2025, 4, 18, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-04-21 - T1 - OANDA-UK-HOLIDAYS - Easter Monday: other books named; Brent arrangement unstated (#332)
        (2025, 4, 21, Unsourced, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-05-26 - T1 - OANDA-UK-HOLIDAYS - Memorial Day: closes 1:30 PM ET (18:30 London), reopens 8:00 PM ET
        (2025, 5, 26, early_close(18 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-05-27 - T1 - OANDA-UK-HOLIDAYS - Previous holiday reopens 20:00 ET = 01:00 London
        (2025, 5, 27, ReplacementBlocks(&WEEKDAY_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-06-19 - T1 - OANDA-UK-HOLIDAYS - Juneteenth: closes 1:30 PM ET, reopens 8:00 PM ET
        (2025, 6, 19, early_close(18 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-06-20 - T1 - OANDA-UK-HOLIDAYS - Previous holiday reopens 20:00 ET = 01:00 London
        (2025, 6, 20, ReplacementBlocks(&FRIDAY_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-07-03 - T1 - OANDA-UK-HOLIDAYS - Independence Day eve: the section prints US index and Commodities
        // rows and nothing for Brent
        (2025, 7, 3, Unsourced, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-07-04 - T1 - OANDA-UK-HOLIDAYS - Independence Day: closes 1:30 pm ET
        (2025, 7, 4, early_close(18 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-08-25 - T1 - OANDA-UK-HOLIDAYS - UK Summer Bank Holiday: other books named; Brent arrangement unstated (#332)
        (2025, 8, 25, Unsourced, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-09-01 - T1 - OANDA-UK-HOLIDAYS - Labor Day: closes 1:30 PM ET, reopens 8:00 PM ET
        (2025, 9, 1, early_close(18 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-09-02 - T1 - OANDA-UK-HOLIDAYS - Previous holiday reopens 20:00 ET = 01:00 London
        (2025, 9, 2, ReplacementBlocks(&WEEKDAY_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-11-26 - T1 - OANDA-UK-HOLIDAYS - Thanksgiving eve: the section prints one Agricultural row, Brent silent
        (2025, 11, 26, Unsourced, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-11-27 - T1 - OANDA-UK-HOLIDAYS - Thanksgiving: closes 1:30 PM ET, reopens 8:00 PM ET
        (2025, 11, 27, early_close(18 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-11-28 - T1 - OANDA-UK-HOLIDAYS - Previous holiday reopens 20:00 ET = 01:00 London; printed Friday close retained
        (2025, 11, 28, ReplacementBlocks(&THANKSGIVING_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
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
        // 2026-04-06 - T1 - OANDA-UK-HOLIDAYS - Easter Monday: other books named; Brent arrangement unstated (#332)
        (2026, 4, 6, Unsourced, T1, "OANDA-UK-HOLIDAYS"),
        // 2026-05-01 - T1 - OANDA-UK-HOLIDAYS - Labour Day: other books named; Brent arrangement unstated (#332)
        (2026, 5, 1, Unsourced, T1, "OANDA-UK-HOLIDAYS"),
        // 2026-05-04 - T1 - OANDA-UK-HOLIDAYS - Early May Bank Holiday: other books named; Brent arrangement unstated (#332)
        (2026, 5, 4, Unsourced, T1, "OANDA-UK-HOLIDAYS"),
        // 2026-05-25 - T1 - OANDA-UK-HOLIDAYS - Memorial Day: closes 13:29 ET (18:29 London), reopens 20:01 ET
        (2026, 5, 25, early_close(18 * 3_600 + 29 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-06-19 - T1 - OANDA-UK-HOLIDAYS - Juneteenth: closes 13:29 ET
        (2026, 6, 19, early_close(18 * 3_600 + 29 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-07-03 - T1 - OANDA-UK-HOLIDAYS - Independence Day: closes 13:29 ET
        (2026, 7, 3, early_close(18 * 3_600 + 29 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-08-31 - T1 - OANDA-UK-HOLIDAYS - UK Summer Bank Holiday: other books named; Brent arrangement unstated (#332)
        (2026, 8, 31, Unsourced, T1, "OANDA-UK-HOLIDAYS"),
        // 2026-09-07 - T1 - OANDA-UK-HOLIDAYS - Labor Day: closes 13:29 ET
        (2026, 9, 7, early_close(18 * 3_600 + 29 * 60), T1, "OANDA-UK-HOLIDAYS"),
    ],
};
