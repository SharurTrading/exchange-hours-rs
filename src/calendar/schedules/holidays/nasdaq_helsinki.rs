// SPDX-License-Identifier: MIT-0

//! Helsinki principal-share holidays, 2026–2027.
//! Evidence: `docs/evidence/nasdaq_helsinki.md`.

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::{HolidayTable, holidays};

// Evidence: docs/evidence/nasdaq_helsinki.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2026, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2026-01-01 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2026, 1, 1, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-01-06 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2026, 1, 6, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-04-03 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2026, 4, 3, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-04-06 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2026, 4, 6, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-05-01 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2026, 5, 1, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-05-14 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2026, 5, 14, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-06-19 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2026, 6, 19, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-12-24 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2026, 12, 24, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-12-25 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2026, 12, 25, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2026-12-31 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2026, 12, 31, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-01-01 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2027, 1, 1, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-01-06 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2027, 1, 6, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-03-26 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2027, 3, 26, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-03-29 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2027, 3, 29, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-05-06 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2027, 5, 6, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-06-25 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2027, 6, 25, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-12-06 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2027, 12, 6, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-12-24 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2027, 12, 24, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
        // 2027-12-31 - T1 - NASDAQ-NORDIC-CASH-2026-2027 - Helsinki cash calendar.
        (2027, 12, 31, Closed, T1, "NASDAQ-NORDIC-CASH-2026-2027"),
    ],
};
