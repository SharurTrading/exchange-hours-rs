// SPDX-License-Identifier: MIT-0

//! Copenhagen principal-share holidays, 2026–2027.
//! Evidence: `docs/evidence/nasdaq_copenhagen.md`.

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::{HolidayTable, holidays};

// Evidence: docs/evidence/nasdaq_copenhagen.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2026, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2026-01-01 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2026, 1, 1, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-04-02 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2026, 4, 2, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-04-03 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2026, 4, 3, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-04-06 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2026, 4, 6, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-05-14 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2026, 5, 14, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-05-15 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2026, 5, 15, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-05-25 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2026, 5, 25, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-06-05 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2026, 6, 5, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-12-24 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2026, 12, 24, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-12-25 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2026, 12, 25, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-12-31 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2026, 12, 31, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-01-01 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2027, 1, 1, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-03-25 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2027, 3, 25, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-03-26 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2027, 3, 26, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-03-29 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2027, 3, 29, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-05-06 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2027, 5, 6, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-05-07 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2027, 5, 7, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-05-17 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2027, 5, 17, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-12-24 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2027, 12, 24, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-12-31 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Copenhagen cash calendar.
        (2027, 12, 31, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
    ],
};
