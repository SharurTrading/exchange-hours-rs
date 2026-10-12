// SPDX-License-Identifier: MIT-0

//! OANDA holiday data, 2024-02-21..2026-10-30. Evidence: `docs/evidence/oanda_metals.md`.

use super::fences::early_close;
use crate::calendar::ExceptionBlock;

use super::{
    EvidenceTier::T1,
    HolidayKind::{Closed, ReplacementBlocks},
    HolidayTable, holidays,
};

// Evidence: docs/evidence/oanda_metals.md
#[rustfmt::skip]
static HOLY_THURSDAY: [ExceptionBlock; 1] =
    [ExceptionBlock::regular(-1, 18 * 3_600 + 5 * 60, 17 * 3_600)];

#[rustfmt::skip]
static NEW_YEAR_REOPEN: [ExceptionBlock; 1] = [ExceptionBlock::regular(-1, 18 * 3_600 + 4 * 60, 16 * 3_600 + 59 * 60)];
#[rustfmt::skip]
static GROUP_1800_REOPEN: [ExceptionBlock; 1] = [ExceptionBlock::regular(-1, 18 * 3_600, 16 * 3_600 + 59 * 60)];
#[rustfmt::skip]
static THANKSGIVING_REOPEN: [ExceptionBlock; 1] = [ExceptionBlock::regular(-1, 18 * 3_600, 14 * 3_600 + 45 * 60)];
#[rustfmt::skip]
static GROUP_1801_REOPEN: [ExceptionBlock; 1] = [ExceptionBlock::regular(-1, 18 * 3_600 + 60, 16 * 3_600 + 59 * 60)];

/// The book's built-in holiday rows and the window they were audited over.
// Evidence: docs/evidence/oanda_metals.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2024, 2, 21) ..= (2026, 10, 30)],
    rows: [
        // 2024-03-29 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Good Friday
        (2024, 3, 29, Closed, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-05-27 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Memorial Day 14:30 close
        (2024, 5, 27, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-05-28 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Memorial Day 18:00 reopen
        (2024, 5, 28, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-06-19 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Juneteenth 14:30 close
        (2024, 6, 19, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-06-20 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Juneteenth 18:00 reopen
        (2024, 6, 20, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-07-04 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Independence Day 14:30 close
        (2024, 7, 4, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-07-05 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Independence Day 18:00 reopen
        (2024, 7, 5, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-09-02 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Labor Day 14:30 close
        (2024, 9, 2, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-09-03 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Labor Day 18:00 reopen
        (2024, 9, 3, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-11-28 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Thanksgiving 14:30 close
        (2024, 11, 28, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-11-29 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Thanksgiving 18:00 reopen and 14:45 close
        (2024, 11, 29, ReplacementBlocks(&THANKSGIVING_REOPEN), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-12-24 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Christmas Eve 13:44 close
        (2024, 12, 24, early_close(13 * 3_600 + 44 * 60), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-12-25 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Christmas Day until evening reopen
        (2024, 12, 25, Closed, T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2024-12-26 - T1 - OANDA-UK-CARTER-2025-WAYBACK - Christmas Day 18:04 reopen
        (2024, 12, 26, ReplacementBlocks(&NEW_YEAR_REOPEN), T1, "OANDA-UK-CARTER-2025-WAYBACK"),
        // 2025-01-01 - T1 - OANDA-UK-HOLIDAYS - New Year's Day: US holiday groups reopen at the evening open
        (2025, 1, 1, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-01-02 - T1 - OANDA-UK-HOLIDAYS - Previous holiday's literal metals group reopen
        (2025, 1, 2, ReplacementBlocks(&NEW_YEAR_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-01-20 - T1 - OANDA-UK-HOLIDAYS - MLK Day: printed group close
        (2025, 1, 20, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-01-21 - T1 - OANDA-UK-HOLIDAYS - Previous holiday's literal metals group reopen
        (2025, 1, 21, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-02-17 - T1 - OANDA-UK-HOLIDAYS - Presidents Day: group closes 2.30pm ET
        (2025, 2, 17, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-02-18 - T1 - OANDA-UK-HOLIDAYS - Previous holiday's literal metals group reopen
        (2025, 2, 18, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-04-17 - T1 - OANDA-UK-HOLIDAYS - Holy Thursday: group closes 17:00 ET
        (2025, 4, 17, ReplacementBlocks(&HOLY_THURSDAY), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-04-18 - T1 - OANDA-UK-HOLIDAYS - Good Friday: group closed
        (2025, 4, 18, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-05-26 - T1 - OANDA-UK-HOLIDAYS - Memorial Day: group closes 2:30 PM ET
        (2025, 5, 26, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-05-27 - T1 - OANDA-UK-HOLIDAYS - Previous holiday's literal metals group reopen
        (2025, 5, 27, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-06-19 - T1 - OANDA-UK-HOLIDAYS - Juneteenth: group closes 2:30 PM ET
        (2025, 6, 19, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-06-20 - T1 - OANDA-UK-HOLIDAYS - Previous holiday's literal metals group reopen
        (2025, 6, 20, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-07-04 - T1 - OANDA-UK-HOLIDAYS - Independence Day: group closes 1 pm ET
        (2025, 7, 4, early_close(13 * 3_600), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-09-01 - T1 - OANDA-UK-HOLIDAYS - Labor Day: group closes 2:30 PM ET
        (2025, 9, 1, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-09-02 - T1 - OANDA-UK-HOLIDAYS - Previous holiday's literal metals group reopen
        (2025, 9, 2, ReplacementBlocks(&GROUP_1800_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-11-27 - T1 - OANDA-UK-HOLIDAYS - Thanksgiving: group closes 2:30 PM ET
        (2025, 11, 27, early_close(14 * 3_600 + 30 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-11-28 - T1 - OANDA-UK-HOLIDAYS - Previous holiday's literal metals group reopen
        (2025, 11, 28, ReplacementBlocks(&THANKSGIVING_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-12-24 - T1 - OANDA-UK-HOLIDAYS - Christmas Eve: Metals close at 13:44
        (2025, 12, 24, early_close(13 * 3_600 + 44 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2025-12-25 - T1 - OANDA-UK-HOLIDAYS - Christmas Day: closed until the evening open
        (2025, 12, 25, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2025-12-26 - T1 - OANDA-UK-HOLIDAYS - Previous holiday's literal metals group reopen
        (2025, 12, 26, ReplacementBlocks(&GROUP_1801_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-01-01 - T1 - OANDA-UK-HOLIDAYS - New Year's Day: closed until the evening open
        (2026, 1, 1, Closed, T1, "OANDA-UK-HOLIDAYS"),
        // 2026-01-02 - T1 - OANDA-UK-HOLIDAYS - Previous holiday's literal metals group reopen
        (2026, 1, 2, ReplacementBlocks(&GROUP_1801_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-01-19 - T1 - OANDA-UK-HOLIDAYS - MLK Day: group closes 14:29
        (2026, 1, 19, early_close(14 * 3_600 + 29 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-01-20 - T1 - OANDA-UK-HOLIDAYS - Previous holiday's literal metals group reopen
        (2026, 1, 20, ReplacementBlocks(&GROUP_1801_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-02-16 - T1 - OANDA-UK-HOLIDAYS - Presidents Day: group closes 14:29
        (2026, 2, 16, early_close(14 * 3_600 + 29 * 60), T1, "OANDA-UK-HOLIDAYS"),
        // 2026-02-17 - T1 - OANDA-UK-HOLIDAYS - Previous holiday's literal metals group reopen
        (2026, 2, 17, ReplacementBlocks(&GROUP_1801_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
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
        // 2026-09-08 - T1 - OANDA-UK-HOLIDAYS - Previous holiday's literal metals group reopen
        (2026, 9, 8, ReplacementBlocks(&GROUP_1801_REOPEN), T1, "OANDA-UK-HOLIDAYS"),
    ],
};
