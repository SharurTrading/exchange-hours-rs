// SPDX-License-Identifier: MIT-0

//! SGX-ST securities-market holiday rows, 2025-2026.
//!
//! Keyed by the crate's venue-local trade date in `Asia/Singapore` (design
//! memo D1). SGX sessions do not wrap past local midnight, so every holiday
//! row lands on its own civil date. The operator's
//! `Stock Exchange — Trading` page (read as bytes from SGX's own content API,
//! the feed the live page renders) prints the half-day phase grid and the
//! half-day date table for 2025 & 2026, and designates the closure calendar:
//! `SGX follows the Singapore holiday calendar available on the Ministry of
//! Manpower website`. The closure rows are that designation resolved against
//! MOM's printed gazetted list (saved beside the operator artifact in the
//! research store): a gazetted holiday falling on a weekday is a `Closed`
//! row, and MOM's own Sunday-substitution sentence moves Vesak Day, National
//! Day and Deepavali 2026 to their Mondays. The per-row derivation is
//! recorded in
//! [`docs/evidence/sgx_securities.md`](../../../../../docs/evidence/sgx_securities.md).
//!
//! The window ends at 2026-12-31 because the operator's own sheet does: the
//! half-day table is printed for 2025 & 2026 only, and the 2027 treatment of
//! the Chinese New Year, Christmas and New Year eves is unpublished, so a
//! 2027 date without a row could not be audited normal. The operator's next
//! annual schedule extends the window.
//!
//! The six half-day dates are **not** scalar early closes. The operator's own
//! half-day sheet keeps executable phases after the 12:00 trading close: the
//! closing Non-Cancel matches to 12:06 and Trade at Close matches to the
//! printed 12:16 close. An `EarlyClose` clip at 12:16 would drag the
//! order-entry-only Pre-Close into `is_open`, and one at 12:00 would delete
//! those prints, so each half day ships as a replacement block set restating
//! the operator's own printed grid, phase for phase.

use super::EvidenceTier::T2;
use super::HolidayKind::{Closed, ReplacementBlocks};
use super::{HolidayTable, holidays};
use crate::calendar::exceptions::ExceptionBlock;

/// The operator's half-day trading day, relative to that trade date.
///
/// Read from the half-day column of the operator's own `Trading Schedules`
/// table: Pre-Open 08:30-08:58/08:59 (order entry; the slice stops at the
/// earliest possible Non-Cancel start, exactly as the normal-week profile
/// does), morning Non-Cancel to 09:00, Trading 09:00-12:00, Pre-Close
/// 12:00-12:04/12:05 (order entry), and the closing Non-Cancel plus Trade at
/// Close to the printed `Close: 12:16pm`. The two closing phases are both
/// tradeable and ship as one `extended` block.
///
/// Evidence: `docs/evidence/sgx_securities.md`.
#[rustfmt::skip]
static HALF_DAY_BLOCKS: [ExceptionBlock; 5] = [
    ExceptionBlock::order_entry(0, 8 * 3_600 + 30 * 60, 8 * 3_600 + 58 * 60),
    ExceptionBlock::extended(0, 8 * 3_600 + 58 * 60, 9 * 3_600),
    ExceptionBlock::regular(0, 9 * 3_600, 12 * 3_600),
    ExceptionBlock::order_entry(0, 12 * 3_600, 12 * 3_600 + 4 * 60),
    ExceptionBlock::extended(0, 12 * 3_600 + 4 * 60, 12 * 3_600 + 16 * 60),
];

/// SGX-ST's built-in holiday rows and the window they were audited over.
///
/// Every date inside the window with no row is audited normal. All rows cite
/// the operator's own page read as bytes (`SGX-ST-SCHED`); the closure dates
/// themselves are printed on the MOM page the operator designates, and the
/// evidence file quotes MOM's print beside each closure row.
// Evidence: docs/evidence/sgx_securities.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2026, 12, 31)],
    rows: [
        // 2025-01-01 - T2 - SGX-ST-SCHED - New Year's Day, gazetted weekday
        // holiday (MOM: Wednesday), SGX-ST closed.
        (2025, 1, 1, Closed, T2, "SGX-ST-SCHED"),
        // 2025-01-28 - T2 - SGX-ST-SCHED - Eve of Chinese New Year, half-day
        // trading, `Close: 12:16pm`.
        (2025, 1, 28, ReplacementBlocks(&HALF_DAY_BLOCKS), T2, "SGX-ST-SCHED"),
        // 2025-01-29 - T2 - SGX-ST-SCHED - Chinese New Year, gazetted weekday
        // holiday (MOM: Wednesday).
        (2025, 1, 29, Closed, T2, "SGX-ST-SCHED"),
        // 2025-01-30 - T2 - SGX-ST-SCHED - Chinese New Year, gazetted weekday
        // holiday (MOM: Thursday).
        (2025, 1, 30, Closed, T2, "SGX-ST-SCHED"),
        // 2025-03-31 - T2 - SGX-ST-SCHED - Hari Raya Puasa, gazetted weekday
        // holiday (MOM: Monday).
        (2025, 3, 31, Closed, T2, "SGX-ST-SCHED"),
        // 2025-04-18 - T2 - SGX-ST-SCHED - Good Friday, gazetted weekday
        // holiday (MOM: Friday).
        (2025, 4, 18, Closed, T2, "SGX-ST-SCHED"),
        // 2025-05-01 - T2 - SGX-ST-SCHED - Labour Day, gazetted weekday
        // holiday (MOM: Thursday).
        (2025, 5, 1, Closed, T2, "SGX-ST-SCHED"),
        // 2025-05-12 - T2 - SGX-ST-SCHED - Vesak Day, gazetted weekday
        // holiday (MOM: Monday).
        (2025, 5, 12, Closed, T2, "SGX-ST-SCHED"),
        // 2025-10-20 - T2 - SGX-ST-SCHED - Deepavali, gazetted weekday
        // holiday (MOM: Monday).
        (2025, 10, 20, Closed, T2, "SGX-ST-SCHED"),
        // 2025-12-24 - T2 - SGX-ST-SCHED - Eve of Christmas, half-day trading,
        // `Close: 12:16pm`.
        (2025, 12, 24, ReplacementBlocks(&HALF_DAY_BLOCKS), T2, "SGX-ST-SCHED"),
        // 2025-12-25 - T2 - SGX-ST-SCHED - Christmas Day, gazetted weekday
        // holiday (MOM: Thursday).
        (2025, 12, 25, Closed, T2, "SGX-ST-SCHED"),
        // 2025-12-31 - T2 - SGX-ST-SCHED - Eve of New Year, half-day trading,
        // `Close: 12:16pm`.
        (2025, 12, 31, ReplacementBlocks(&HALF_DAY_BLOCKS), T2, "SGX-ST-SCHED"),
        // 2026-01-01 - T2 - SGX-ST-SCHED - New Year's Day, gazetted weekday
        // holiday (MOM: Thursday).
        (2026, 1, 1, Closed, T2, "SGX-ST-SCHED"),
        // 2026-02-16 - T2 - SGX-ST-SCHED - Eve of Chinese New Year, half-day
        // trading, `Close: 12:16pm`.
        (2026, 2, 16, ReplacementBlocks(&HALF_DAY_BLOCKS), T2, "SGX-ST-SCHED"),
        // 2026-02-17 - T2 - SGX-ST-SCHED - Chinese New Year, gazetted weekday
        // holiday (MOM: Tuesday).
        (2026, 2, 17, Closed, T2, "SGX-ST-SCHED"),
        // 2026-02-18 - T2 - SGX-ST-SCHED - Chinese New Year, gazetted weekday
        // holiday (MOM: Wednesday).
        (2026, 2, 18, Closed, T2, "SGX-ST-SCHED"),
        // 2026-04-03 - T2 - SGX-ST-SCHED - Good Friday, gazetted weekday
        // holiday (MOM: Friday).
        (2026, 4, 3, Closed, T2, "SGX-ST-SCHED"),
        // 2026-05-01 - T2 - SGX-ST-SCHED - Labour Day, gazetted weekday
        // holiday (MOM: Friday).
        (2026, 5, 1, Closed, T2, "SGX-ST-SCHED"),
        // 2026-05-27 - T2 - SGX-ST-SCHED - Hari Raya Haji, gazetted weekday
        // holiday (MOM: Wednesday).
        (2026, 5, 27, Closed, T2, "SGX-ST-SCHED"),
        // 2026-06-01 - T2 - SGX-ST-SCHED - Vesak Day substitution: MOM prints
        // Sunday 31 May and states `Monday, 1 June 2026, will be a public
        // holiday`.
        (2026, 6, 1, Closed, T2, "SGX-ST-SCHED"),
        // 2026-08-10 - T2 - SGX-ST-SCHED - National Day substitution: MOM
        // prints Sunday 9 August and states `Monday, 10 August 2026, will be a
        // public holiday`.
        (2026, 8, 10, Closed, T2, "SGX-ST-SCHED"),
        // 2026-11-09 - T2 - SGX-ST-SCHED - Deepavali substitution: MOM prints
        // Sunday 8 November and states `Monday, 9 November 2026, will be a
        // public holiday`.
        (2026, 11, 9, Closed, T2, "SGX-ST-SCHED"),
        // 2026-12-24 - T2 - SGX-ST-SCHED - Eve of Christmas, half-day trading,
        // `Close: 12:16pm`.
        (2026, 12, 24, ReplacementBlocks(&HALF_DAY_BLOCKS), T2, "SGX-ST-SCHED"),
        // 2026-12-25 - T2 - SGX-ST-SCHED - Christmas Day, gazetted weekday
        // holiday (MOM: Friday).
        (2026, 12, 25, Closed, T2, "SGX-ST-SCHED"),
        // 2026-12-31 - T2 - SGX-ST-SCHED - Eve of New Year, half-day trading,
        // `Close: 12:16pm`.
        (2026, 12, 31, ReplacementBlocks(&HALF_DAY_BLOCKS), T2, "SGX-ST-SCHED"),
    ],
};
