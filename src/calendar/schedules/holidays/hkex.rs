// SPDX-License-Identifier: MIT-0

//! Hong Kong Exchanges and Clearing holiday rows, 2025-2027.
//!
//! Keyed by the crate's own venue-local trade date in `Asia/Hong_Kong`. HKEX's
//! securities market trades Monday to Friday excluding public holidays, and
//! its own holiday schedule — the `Trading Calendar and Holiday Schedule`
//! page's `Holiday Schedule` table — names every such day (`Holiday (no
//! trading)`) plus every shortened eve in its `Notes` block (`no afternoon and
//! after-hours trading session`). The whole block is **T1**.
//!
//! An eve row is an **early close**, not a closure: the securities hours page
//! gives the half-day shape its own times — continuous trading to 12:00 noon
//! and a Closing Auction Session to "a random closing between 12:08 p.m. and
//! 12:10 p.m." — so the profile's latest scheduled close edge, 12:10, replaces
//! the 16:10 normal close exactly as the normal day's 16:10 replaces it. The
//! 09:00-09:30 pre-opening phases and the 09:30-12:00 continuous session are
//! unchanged.
//!
//! Severe-weather arrangements (typhoon signals) are conditional and key no
//! row (LAW-NO-FABRICATED-DATES); the operator's schedule prints no such date.
//! The derivatives-only `#` footnotes (MSCI after-hours suspensions on UK/US
//! bank holidays) name no securities session and key no row either. Coverage
//! stops at 2027-12-31, the last trade year the operator's schedule names;
//! the per-row derivation is recorded in
//! [`docs/evidence/hkex.md`](../../../../../docs/evidence/hkex.md).

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::fences::early_close;
use super::{HolidayTable, holidays};

/// The half-day final close, `12:10` venue-local, in seconds since midnight.
///
/// The securities hours page prints the half-day Closing Auction Session as
/// `12:00 noon to a random closing between 12:08 p.m. and 12:10 p.m.`; the
/// profile states the latest scheduled CAS edge, exactly as the normal day's
/// `4:08 p.m. - 4:10 p.m.` close is stated at 16:10.
const HALF_DAY_CLOSE_SSM: u32 = 12 * 3_600 + 10 * 60;

/// HKEX's built-in holiday rows and the window they were audited over.
///
/// Every row is one line of the operator's `Holiday Schedule` table (a
/// `Holiday (no trading)` row) or one line of its `Notes` block (an eve with
/// `no afternoon and after-hours trading session`); the 12:10 instant comes
/// from the operator's securities-market half-day schedule. A date inside the
/// window with no row is audited normal.
// Evidence: docs/evidence/hkex.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2025-01-01 - T1 - HKEX-TC-2025 - The first day of January.
        (2025, 1, 1, Closed, T1, "HKEX-TC-2025"),
        // 2025-01-28 - T1 - HKEX-TC-2025 - Eve of Lunar New Year: no afternoon
        // session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2025, 1, 28, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2025"),
        // 2025-01-29 - T1 - HKEX-TC-2025 - Lunar New Year's Day.
        (2025, 1, 29, Closed, T1, "HKEX-TC-2025"),
        // 2025-01-30 - T1 - HKEX-TC-2025 - The second day of Lunar New Year.
        (2025, 1, 30, Closed, T1, "HKEX-TC-2025"),
        // 2025-01-31 - T1 - HKEX-TC-2025 - The third day of Lunar New Year.
        (2025, 1, 31, Closed, T1, "HKEX-TC-2025"),
        // 2025-04-04 - T1 - HKEX-TC-2025 - Ching Ming Festival.
        (2025, 4, 4, Closed, T1, "HKEX-TC-2025"),
        // 2025-04-18 - T1 - HKEX-TC-2025 - Good Friday.
        (2025, 4, 18, Closed, T1, "HKEX-TC-2025"),
        // 2025-04-21 - T1 - HKEX-TC-2025 - Easter Monday.
        (2025, 4, 21, Closed, T1, "HKEX-TC-2025"),
        // 2025-05-01 - T1 - HKEX-TC-2025 - Labour Day.
        (2025, 5, 1, Closed, T1, "HKEX-TC-2025"),
        // 2025-05-05 - T1 - HKEX-TC-2025 - The Birthday of the Buddha.
        (2025, 5, 5, Closed, T1, "HKEX-TC-2025"),
        // 2025-07-01 - T1 - HKEX-TC-2025 - HKSAR Establishment Day.
        (2025, 7, 1, Closed, T1, "HKEX-TC-2025"),
        // 2025-10-01 - T1 - HKEX-TC-2025 - National Day.
        (2025, 10, 1, Closed, T1, "HKEX-TC-2025"),
        // 2025-10-07 - T1 - HKEX-TC-2025 - The day following the Chinese
        // Mid-Autumn Festival.
        (2025, 10, 7, Closed, T1, "HKEX-TC-2025"),
        // 2025-10-29 - T1 - HKEX-TC-2025 - Chung Yeung Festival.
        (2025, 10, 29, Closed, T1, "HKEX-TC-2025"),
        // 2025-12-24 - T1 - HKEX-TC-2025 - Eve of Christmas Day: no afternoon
        // session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2025, 12, 24, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2025"),
        // 2025-12-25 - T1 - HKEX-TC-2025 - Christmas Day.
        (2025, 12, 25, Closed, T1, "HKEX-TC-2025"),
        // 2025-12-26 - T1 - HKEX-TC-2025 - The first weekday after Christmas Day.
        (2025, 12, 26, Closed, T1, "HKEX-TC-2025"),
        // 2025-12-31 - T1 - HKEX-TC-2025 - Eve of New Year: no afternoon
        // session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2025, 12, 31, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2025"),
        // 2026-01-01 - T1 - HKEX-TC-2026-2027 - The first day of January.
        (2026, 1, 1, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-02-16 - T1 - HKEX-TC-2026-2027 - Eve of Lunar New Year: no
        // afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2026, 2, 16, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2026-2027"),
        // 2026-02-17 - T1 - HKEX-TC-2026-2027 - Lunar New Year's Day.
        (2026, 2, 17, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-02-18 - T1 - HKEX-TC-2026-2027 - The second day of Lunar New Year.
        (2026, 2, 18, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-02-19 - T1 - HKEX-TC-2026-2027 - The third day of Lunar New Year.
        (2026, 2, 19, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-04-03 - T1 - HKEX-TC-2026-2027 - Good Friday.
        (2026, 4, 3, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-04-06 - T1 - HKEX-TC-2026-2027 - The day following Ching Ming
        // Festival.
        (2026, 4, 6, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-04-07 - T1 - HKEX-TC-2026-2027 - The day following Easter Monday.
        (2026, 4, 7, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-05-01 - T1 - HKEX-TC-2026-2027 - Labour Day.
        (2026, 5, 1, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-05-25 - T1 - HKEX-TC-2026-2027 - The day following the Birthday
        // of the Buddha.
        (2026, 5, 25, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-06-19 - T1 - HKEX-TC-2026-2027 - Tuen Ng Festival.
        (2026, 6, 19, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-07-01 - T1 - HKEX-TC-2026-2027 - HKSAR Establishment Day.
        (2026, 7, 1, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-10-01 - T1 - HKEX-TC-2026-2027 - National Day.
        (2026, 10, 1, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-10-19 - T1 - HKEX-TC-2026-2027 - The day following Chung Yeung
        // Festival.
        (2026, 10, 19, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-12-24 - T1 - HKEX-TC-2026-2027 - Eve of Christmas Day: no
        // afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2026, 12, 24, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2026-2027"),
        // 2026-12-25 - T1 - HKEX-TC-2026-2027 - Christmas Day.
        (2026, 12, 25, Closed, T1, "HKEX-TC-2026-2027"),
        // 2026-12-31 - T1 - HKEX-TC-2026-2027 - Eve of New Year: no afternoon
        // session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2026, 12, 31, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2026-2027"),
        // 2027-01-01 - T1 - HKEX-TC-2026-2027 - The first day of January.
        (2027, 1, 1, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-02-05 - T1 - HKEX-TC-2026-2027 - Eve of Lunar New Year: no
        // afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2027, 2, 5, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2026-2027"),
        // 2027-02-08 - T1 - HKEX-TC-2026-2027 - The third day of Lunar New Year.
        (2027, 2, 8, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-02-09 - T1 - HKEX-TC-2026-2027 - The fourth day of Lunar New Year.
        (2027, 2, 9, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-03-26 - T1 - HKEX-TC-2026-2027 - Good Friday.
        (2027, 3, 26, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-03-29 - T1 - HKEX-TC-2026-2027 - Easter Monday.
        (2027, 3, 29, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-04-05 - T1 - HKEX-TC-2026-2027 - Ching Ming Festival.
        (2027, 4, 5, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-05-13 - T1 - HKEX-TC-2026-2027 - The Birthday of the Buddha.
        (2027, 5, 13, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-06-09 - T1 - HKEX-TC-2026-2027 - Tuen Ng Festival.
        (2027, 6, 9, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-07-01 - T1 - HKEX-TC-2026-2027 - HKSAR Establishment Day.
        (2027, 7, 1, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-09-16 - T1 - HKEX-TC-2026-2027 - The day following the Chinese
        // Mid-Autumn Festival.
        (2027, 9, 16, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-10-01 - T1 - HKEX-TC-2026-2027 - National Day.
        (2027, 10, 1, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-10-08 - T1 - HKEX-TC-2026-2027 - Chung Yeung Festival.
        (2027, 10, 8, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-12-24 - T1 - HKEX-TC-2026-2027 - Eve of Christmas Day: no
        // afternoon session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2027, 12, 24, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2026-2027"),
        // 2027-12-27 - T1 - HKEX-TC-2026-2027 - The first weekday after
        // Christmas Day (Christmas Eve-day falls on the 2027 weekend).
        (2027, 12, 27, Closed, T1, "HKEX-TC-2026-2027"),
        // 2027-12-31 - T1 - HKEX-TC-2026-2027 - Eve of New Year: no afternoon
        // session; the half-day close is 12:10 (HKEX-HOURS-SEC).
        (2027, 12, 31, early_close(HALF_DAY_CLOSE_SSM), T1, "HKEX-TC-2026-2027"),
    ],
};
