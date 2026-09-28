// SPDX-License-Identifier: MIT-0

//! NZX Main Board holiday rows, 2025 through the operator's published horizon
//! (2027-01-04).
//!
//! Keyed by the crate's venue-local trade date in `Pacific/Auckland` (design
//! memo D1). NZX sessions do not wrap past local midnight, so every holiday
//! row lands on its own civil date. NZX prints no consolidated year sheets;
//! the operator's `NZX Market Holidays & Trading Hours` page carries a rolling
//! table of roughly the next thirteen months, so the rows are read from the
//! page's own captures: the 2025 block and the first two 2026 dates from the
//! December 2025 / January 2025 replays, and Waitangi Day 2026 onward from the
//! February 2026 replay, which the live page (retrieved 2026-09-28) still
//! prints row for row. NZX has published nothing past 2027-01-04, so coverage
//! stops there and the operator's next table refresh extends it; the per-row
//! derivation is recorded in
//! [`docs/evidence/nzx.md`](../../../../../docs/evidence/nzx.md).
//!
//! The four abbreviated-trading dates (the business day prior to Christmas Day
//! and to New Year's Day in 2025 and 2026) are **not** scalar early closes.
//! The operator's own abbreviated grid keeps a tradeable closing auction after
//! the shortened Normal Trading window: Pre-Close runs 12:45-13:00 and the
//! closing uncross randomises within 30 seconds either side of 13:00, exactly
//! as it does around 17:00 on a full day. An `EarlyClose` clip cannot state
//! that day — clipped at 12:45 it deletes the auction prints, and clipped at
//! 13:00:30 it drags the order-entry-only Pre-Close queue inside `is_open` —
//! so each abbreviated day ships as a replacement block set restating the
//! operator's own grid, the same conclusion the CFE evidence reached for its
//! 2025 mourning day.

use super::EvidenceTier::T1;
use super::HolidayKind::{Closed, ReplacementBlocks};
use super::{HolidayTable, holidays};
use crate::calendar::exceptions::ExceptionBlock;

/// The operator's abbreviated-trading day, relative to that trade date.
///
/// Read from the abbreviated column of the operator's own Main Board table:
/// Pre-open 08:30-10:00 (tradeable — off-market reports print), Normal Trading
/// 10:00-12:45, Pre-Close 12:45-13:00 (order entry; the slice stops at
/// 12:59:30 so the randomised uncross stays out of the order-entry block),
/// and the closing uncross envelope 12:59:30-13:00:30, using the same ±30
/// second randomisation envelope the normal-week profile carries around 17:00
/// from the operator's anatomy-of-a-trading-day statement. Enquiry and Adjust
/// accept no matched orders and state no blocks.
///
/// Evidence: `docs/evidence/nzx.md`.
#[rustfmt::skip]
static ABBREVIATED_DAY_BLOCKS: [ExceptionBlock; 4] = [
    ExceptionBlock::extended(0, 8 * 3_600 + 30 * 60, 10 * 3_600),
    ExceptionBlock::regular(0, 10 * 3_600, 12 * 3_600 + 45 * 60),
    ExceptionBlock::order_entry(0, 12 * 3_600 + 45 * 60, 12 * 3_600 + 59 * 60 + 30),
    ExceptionBlock::extended(0, 12 * 3_600 + 59 * 60 + 30, 13 * 3_600 + 30),
];

/// NZX's built-in holiday rows and the window they were audited over.
///
/// Every date inside the window with no row is audited normal. 2025 rows cite
/// the operator's page as replayed 2025-01-23 (New Year's Day and the day
/// after, which that replay had already rotated out, cite the 2024-12-16
/// replay that still prints them), and Waitangi Day 2026 onward cite the
/// 2026-02-03 replay.
// Evidence: docs/evidence/nzx.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2027, 1, 4)],
    rows: [
        // 2025-01-01 - T1 - NZX-TH-2024-12-16 - New Year's Day, `Closed`.
        (2025, 1, 1, Closed, T1, "NZX-TH-2024-12-16"),
        // 2025-01-02 - T1 - NZX-TH-2024-12-16 - Day after New Year's Day, `Closed`.
        (2025, 1, 2, Closed, T1, "NZX-TH-2024-12-16"),
        // 2025-02-06 - T1 - NZX-TH-2025-01-23 - Waitangi Day, `Closed`.
        (2025, 2, 6, Closed, T1, "NZX-TH-2025-01-23"),
        // 2025-04-18 - T1 - NZX-TH-2025-01-23 - Good Friday, `Closed`.
        (2025, 4, 18, Closed, T1, "NZX-TH-2025-01-23"),
        // 2025-04-21 - T1 - NZX-TH-2025-01-23 - Easter Monday, `Closed`.
        (2025, 4, 21, Closed, T1, "NZX-TH-2025-01-23"),
        // 2025-04-25 - T1 - NZX-TH-2025-01-23 - ANZAC Day, `Closed`.
        (2025, 4, 25, Closed, T1, "NZX-TH-2025-01-23"),
        // 2025-06-02 - T1 - NZX-TH-2025-01-23 - King's Birthday, `Closed`.
        (2025, 6, 2, Closed, T1, "NZX-TH-2025-01-23"),
        // 2025-06-20 - T1 - NZX-TH-2025-01-23 - Matariki, `Closed`.
        (2025, 6, 20, Closed, T1, "NZX-TH-2025-01-23"),
        // 2025-10-27 - T1 - NZX-TH-2025-01-23 - Labour Day, `Closed`.
        (2025, 10, 27, Closed, T1, "NZX-TH-2025-01-23"),
        // 2025-12-24 - T1 - NZX-TH-2025-01-23 - Business Day Prior to Christmas
        // Day, `Abbreviated Trading`: the operator's own abbreviated grid as
        // one replacement day.
        (2025, 12, 24, ReplacementBlocks(&ABBREVIATED_DAY_BLOCKS), T1, "NZX-TH-2025-01-23"),
        // 2025-12-25 - T1 - NZX-TH-2025-01-23 - Christmas Day, `Closed`.
        (2025, 12, 25, Closed, T1, "NZX-TH-2025-01-23"),
        // 2025-12-26 - T1 - NZX-TH-2025-01-23 - Boxing Day, `Closed`.
        (2025, 12, 26, Closed, T1, "NZX-TH-2025-01-23"),
        // 2025-12-31 - T1 - NZX-TH-2025-01-23 - Business Day Prior to New
        // Year's Day, `Abbreviated Trading`.
        (2025, 12, 31, ReplacementBlocks(&ABBREVIATED_DAY_BLOCKS), T1, "NZX-TH-2025-01-23"),
        // 2026-01-01 - T1 - NZX-TH-2025-01-23 - New Year's Day, `Closed`.
        (2026, 1, 1, Closed, T1, "NZX-TH-2025-01-23"),
        // 2026-01-02 - T1 - NZX-TH-2025-01-23 - Day after New Year's Day,
        // `Closed`.
        (2026, 1, 2, Closed, T1, "NZX-TH-2025-01-23"),
        // 2026-02-06 - T1 - NZX-TH-2026-02-03 - Waitangi Day, `Closed`.
        (2026, 2, 6, Closed, T1, "NZX-TH-2026-02-03"),
        // 2026-04-03 - T1 - NZX-TH-2026-02-03 - Good Friday, `Closed`.
        (2026, 4, 3, Closed, T1, "NZX-TH-2026-02-03"),
        // 2026-04-06 - T1 - NZX-TH-2026-02-03 - Easter Monday, `Closed`.
        (2026, 4, 6, Closed, T1, "NZX-TH-2026-02-03"),
        // 2026-04-27 - T1 - NZX-TH-2026-02-03 - ANZAC Day, `Closed`: the sheet
        // mondayises the Saturday to Monday 27 April.
        (2026, 4, 27, Closed, T1, "NZX-TH-2026-02-03"),
        // 2026-06-01 - T1 - NZX-TH-2026-02-03 - King's Birthday, `Closed`.
        (2026, 6, 1, Closed, T1, "NZX-TH-2026-02-03"),
        // 2026-07-10 - T1 - NZX-TH-2026-02-03 - Matariki, `Closed`.
        (2026, 7, 10, Closed, T1, "NZX-TH-2026-02-03"),
        // 2026-10-26 - T1 - NZX-TH-2026-02-03 - Labour Day, `Closed`.
        (2026, 10, 26, Closed, T1, "NZX-TH-2026-02-03"),
        // 2026-12-24 - T1 - NZX-TH-2026-02-03 - Business Day Prior to Christmas
        // Day, `Abbreviated Trading`.
        (2026, 12, 24, ReplacementBlocks(&ABBREVIATED_DAY_BLOCKS), T1, "NZX-TH-2026-02-03"),
        // 2026-12-25 - T1 - NZX-TH-2026-02-03 - Christmas Day, `Closed`.
        (2026, 12, 25, Closed, T1, "NZX-TH-2026-02-03"),
        // 2026-12-28 - T1 - NZX-TH-2026-02-03 - Boxing Day, `Closed`: the
        // sheet mondayises the Saturday to Monday 28 December.
        (2026, 12, 28, Closed, T1, "NZX-TH-2026-02-03"),
        // 2026-12-31 - T1 - NZX-TH-2026-02-03 - Business Day Prior to New
        // Year's Day, `Abbreviated Trading`.
        (2026, 12, 31, ReplacementBlocks(&ABBREVIATED_DAY_BLOCKS), T1, "NZX-TH-2026-02-03"),
        // 2027-01-01 - T1 - NZX-TH-2026-02-03 - New Year's Day, `Closed`, the
        // last year the operator's table reaches.
        (2027, 1, 1, Closed, T1, "NZX-TH-2026-02-03"),
        // 2027-01-04 - T1 - NZX-TH-2026-02-03 - Day after New Year's Day,
        // `Closed`, the operator's published horizon.
        (2027, 1, 4, Closed, T1, "NZX-TH-2026-02-03"),
    ],
};
