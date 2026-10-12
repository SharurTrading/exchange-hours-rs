// SPDX-License-Identifier: MIT-0

//! OANDA holiday data, 2024-01-01..2024-12-31 and 2025-01-01..2026-10-30. Evidence: `docs/evidence/oanda_us_bond_cfd.md`.

use super::fences::{early_close, late_open};
use crate::calendar::ExceptionBlock;

use super::{
    EvidenceTier::T1,
    HolidayKind::{Closed, ReplacementBlocks, Unsourced},
    HolidayTable, holidays,
};

// Evidence: docs/evidence/oanda_us_bond_cfd.md
#[rustfmt::skip]
static GROUP_1800_REOPEN: [ExceptionBlock; 1] = [ExceptionBlock::regular(-1, 18 * 3_600, 16 * 3_600 + 59 * 60)];

#[rustfmt::skip]
static HOLY_THURSDAY: [ExceptionBlock; 1] = [ExceptionBlock::regular(-1, 18 * 3_600 + 60, 17 * 3_600)];

#[rustfmt::skip]
static THANKSGIVING_REOPEN: [ExceptionBlock; 1] = [ExceptionBlock::regular(-1, 18 * 3_600, 13 * 3_600 + 15 * 60)];

/// The book's built-in holiday rows and the window they were audited over.
// Evidence: docs/evidence/oanda_us_bond_cfd.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2024, 1, 1) ..= (2024, 12, 31), (2025, 1, 1) ..= (2026, 10, 30)],
    rows: [
        // 2024-01-01 - T1 - OANDA-UK-CARTER-2025-WAYBACK - New Year: the holiday closes before the evening reopen
        (2024, 1, 1, Closed, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-01-02 - T1 - OANDA-UK-CARTER-2025-WAYBACK - New Year 18:00 ET reopen known; pre-witness final close unknown (#332)
        (2024, 1, 2, Unsourced, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-01-15 - T1 - OANDA-UK-CARTER-2025-WAYBACK - MLK Day closes 13:00 ET
        (2024, 1, 15, early_close(13 * 3_600), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-01-16 - T1 - OANDA-UK-CARTER-2025-WAYBACK - MLK 18:00 ET reopen known; pre-witness final close unknown (#332)
        (2024, 1, 16, Unsourced, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-02-19 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Presidents Day closes 13:00 ET
        (2024, 2, 19, early_close(13 * 3_600), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-02-20 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Presidents Day 18:00 ET reopen known; pre-witness final close unknown (#332)
        (2024, 2, 20, Unsourced, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-03-29 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Good Friday: US indices and US bonds closed
        (2024, 3, 29, Closed, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-05-27 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Memorial Day closes 13:00 ET
        (2024, 5, 27, early_close(13 * 3_600), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-05-28 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Memorial evening reopen is literal 18:00 ET
        (2024, 5, 28, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-06-19 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Juneteenth closes 13:00 ET
        (2024, 6, 19, early_close(13 * 3_600), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-06-20 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Juneteenth evening reopen is literal 18:00 ET
        (2024, 6, 20, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-07-04 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Independence Day: bonds close 13:00 ET; July 3 names only indices
        (2024, 7, 4, early_close(13 * 3_600), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-07-05 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Independence evening reopen is literal 18:00 ET
        (2024, 7, 5, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-09-02 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Labor Day closes 13:00 ET
        (2024, 9, 2, early_close(13 * 3_600), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-09-03 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Labor Day evening reopen is literal 18:00 ET
        (2024, 9, 3, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-11-28 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Thanksgiving Thursday closes 13:00 ET
        (2024, 11, 28, early_close(13 * 3_600), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-11-29 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Thanksgiving Friday retains prior 18:00 reopen and printed 13:15 close
        (2024, 11, 29, ReplacementBlocks(&THANKSGIVING_REOPEN), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-12-24 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Christmas Eve: US-bond arrangement omitted; exact session withheld (#332)
        (2024, 12, 24, Unsourced, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-12-25 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Christmas: holiday closed before evening 18:04 ET reopen
        (2024, 12, 25, Closed, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-12-26 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Previous holiday reopens 18:04 ET
        (2024, 12, 26, late_open(18 * 3_600 + 4 * 60), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2025-01-01 - T1 - OANDA-UK-HOLIDAYS - New Year's Day: US indices and US bonds open late 18:04
        (2025, 1, 1, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-01-02 - T1 - OANDA-UK-HOLIDAYS - New Year's Day evening reopen belongs to the following trade date
        (2025, 1, 2, late_open(18 * 3_600 + 4 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-01-09 - T1 - OANDA-UK-HOLIDAYS - Carter's Day of Mourning: bonds close 13:15 ET
        (2025, 1, 9, early_close(13 * 3_600 + 15 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-01-10 - T1 - OANDA-UK-HOLIDAYS - Carter's Day evening reopen 18:00 ET
        (2025, 1, 10, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-01-20 - T1 - OANDA-UK-HOLIDAYS - MLK Day: printed group close
        (2025, 1, 20, early_close(13 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-01-21 - T1 - OANDA-UK-HOLIDAYS - Previous holiday reopens 18:00 ET
        (2025, 1, 21, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-02-17 - T1 - OANDA-UK-HOLIDAYS - Presidents Day: closes 1 pm, reopens 6 pm
        (2025, 2, 17, early_close(13 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-02-18 - T1 - OANDA-UK-HOLIDAYS - Previous holiday reopens 18:00 ET
        (2025, 2, 18, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-04-17 - T1 - OANDA-UK-HOLIDAYS - Holy Thursday: closes 17:00 ET
        (2025, 4, 17, ReplacementBlocks(&HOLY_THURSDAY), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-04-18 - T1 - OANDA-UK-HOLIDAYS - Good Friday: closed
        (2025, 4, 18, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-05-26 - T1 - OANDA-UK-HOLIDAYS - Memorial Day: closes 1:00 PM, reopens 6:00 PM
        (2025, 5, 26, early_close(13 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-05-27 - T1 - OANDA-UK-HOLIDAYS - Previous holiday reopens 18:00 ET
        (2025, 5, 27, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-06-19 - T1 - OANDA-UK-HOLIDAYS - Juneteenth: closes 1:00 PM, reopens 6:00 PM
        (2025, 6, 19, early_close(13 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-06-20 - T1 - OANDA-UK-HOLIDAYS - Previous holiday reopens 18:00 ET
        (2025, 6, 20, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-07-04 - T1 - OANDA-UK-HOLIDAYS - Independence Day: bonds close 1 pm; July 3 states only an index arrangement
        (2025, 7, 4, early_close(13 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-09-01 - T1 - OANDA-UK-HOLIDAYS - Labor Day: closes 1:00 PM, reopens 6:00 PM
        (2025, 9, 1, early_close(13 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-09-02 - T1 - OANDA-UK-HOLIDAYS - Previous holiday reopens 18:00 ET
        (2025, 9, 2, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-11-27 - T1 - OANDA-UK-HOLIDAYS - Thanksgiving: closes 1:00 PM, reopens 6:00 PM
        (2025, 11, 27, early_close(13 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-11-28 - T1 - OANDA-UK-HOLIDAYS - Previous holiday reopens 18:00 ET; retain this trade date's printed close
        (2025, 11, 28, ReplacementBlocks(&THANKSGIVING_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
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
        // 2026-04-03 - T1 - OANDA-UK-HOLIDAYS - Good Friday: bonds close 11:14 ET
        (2026, 4, 3, early_close(11 * 3_600 + 14 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-05-25 - T1 - OANDA-UK-HOLIDAYS - Memorial Day: closes 12:59; source prints incomplete reopen 18:0 (evidence records the residual)
        (2026, 5, 25, early_close(12 * 3_600 + 59 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-05-26 - T1 - OANDA-UK-HOLIDAYS - Previous evening reopen is incomplete (18:0); exact session withheld (#332)
        (2026, 5, 26, Unsourced, T1, "OANDA-UK-HOLIDAYS"),
        // 2026-06-19 - T1 - OANDA-UK-HOLIDAYS - Juneteenth: closes 12:59
        (2026, 6, 19, early_close(12 * 3_600 + 59 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-07-03 - T1 - OANDA-UK-HOLIDAYS - Independence Day: closes 12:59
        (2026, 7, 3, early_close(12 * 3_600 + 59 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-09-07 - T1 - OANDA-UK-HOLIDAYS - Labor Day: closes 12:59, reopens 18:01
        (2026, 9, 7, early_close(12 * 3_600 + 59 * 60), T1, "OANDA-UK-HOLIDAYS"),
    ],
};
