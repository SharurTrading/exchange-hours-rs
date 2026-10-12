// SPDX-License-Identifier: MIT-0

//! Stockholm principal-share holidays, 2026–2027.
//! Evidence: `docs/evidence/nasdaq_stockholm.md`.

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::HolidayKind::ReplacementBlocks;
use super::{HolidayTable, holidays};
use crate::calendar::ExceptionBlock;

// Evidence: docs/evidence/nasdaq_stockholm.md
static HALF_DAY: [ExceptionBlock; 4] = [
    ExceptionBlock::extended(0, 8 * 3_600, 9 * 3_600 + 5),
    ExceptionBlock::regular(0, 9 * 3_600 + 5, 12 * 3_600 + 55 * 60),
    ExceptionBlock::extended(0, 12 * 3_600 + 55 * 60, 13 * 3_600),
    ExceptionBlock::extended(0, 13 * 3_600, 13 * 3_600 + 30 * 60),
];

// Evidence: docs/evidence/nasdaq_stockholm.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2026, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2026-01-01 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2026, 1, 1, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-01-05 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2026, 1, 5, ReplacementBlocks(&HALF_DAY), T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-01-06 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2026, 1, 6, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-04-02 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2026, 4, 2, ReplacementBlocks(&HALF_DAY), T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-04-03 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2026, 4, 3, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-04-06 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2026, 4, 6, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-04-30 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2026, 4, 30, ReplacementBlocks(&HALF_DAY), T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-05-01 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2026, 5, 1, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-05-13 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2026, 5, 13, ReplacementBlocks(&HALF_DAY), T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-05-14 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2026, 5, 14, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-06-19 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2026, 6, 19, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-10-30 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2026, 10, 30, ReplacementBlocks(&HALF_DAY), T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-12-24 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2026, 12, 24, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-12-25 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2026, 12, 25, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-12-31 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2026, 12, 31, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-01-01 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2027, 1, 1, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-01-05 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2027, 1, 5, ReplacementBlocks(&HALF_DAY), T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-01-06 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2027, 1, 6, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-03-25 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2027, 3, 25, ReplacementBlocks(&HALF_DAY), T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-03-26 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2027, 3, 26, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-03-29 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2027, 3, 29, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-04-30 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2027, 4, 30, ReplacementBlocks(&HALF_DAY), T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-05-05 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2027, 5, 5, ReplacementBlocks(&HALF_DAY), T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-05-06 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2027, 5, 6, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-06-25 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2027, 6, 25, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-11-05 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2027, 11, 5, ReplacementBlocks(&HALF_DAY), T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-12-24 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2027, 12, 24, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-12-31 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Stockholm cash calendar.
        (2027, 12, 31, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
    ],
};
