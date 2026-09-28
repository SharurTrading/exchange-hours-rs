// SPDX-License-Identifier: MIT-0

//! Euronext Paris holiday rows, 2025-2026.
//!
//! Keyed by the crate's own venue-local trade date in `Europe/Paris`. Euronext
//! Paris runs no overnight session, so an event date and its trade date are one
//! civil day and the conversion is the identity.
//!
//! The whole block is **T1**: Euronext publishes one holiday calendar covering
//! its cash markets with a per-market column — Paris is the last column — on
//! its `Trading hours & Holidays` page, and a per-year INFO-FLASH PDF beside
//! it. The page keeps only the latest two years, so the 2025 rows are pinned by
//! the archived 2025-12-06 state of the page. December half days are stated
//! **only** by the operator's end-of-year appendix to the Euronext Instructions
//! 4-01/4-03 Trading Manuals: the 2025 appendix (an XLSX the December page
//! links) prints the Paris equity segments' 24/31 December 2025 schedule —
//! continuous trading to 13:55, closing uncross `14:00 Random`, TAL to
//! `14:00 - 14:05` — so the day's availability envelope ends at 14:05 and the
//! row is an early close at that operator-stated instant. The 2026 appendix is
//! announced but unpublished ("To be announced" on the page), so the two 2026
//! half days ship as `Unsourced`. This table encodes **Paris only**; the five
//! other Euronext cash markets' columns are out of this identity's scope, and
//! the follow-up for them is recorded in
//! [`docs/evidence/euronext_paris.md`](../../../../../docs/evidence/euronext_paris.md).
//!
//! The "half trading day, Wednesday before Easter" row in both years is **Oslo
//! only**: Paris prints `Full Trading Day` and no row is shipped for it. The
//! Dublin-only and Milan/Oslo-only rows likewise leave Paris untouched.

use super::EvidenceTier::T1;
use super::HolidayKind::{Closed, Unsourced};
use super::fences::early_close;
use super::{HolidayTable, holidays};

/// Euronext Paris's stated 2025 half-day envelope close, 14:05 CET.
///
/// The 2025 end-of-year appendix prints `14:00  -  14:05` for the TAL phase of
/// every Paris equity segment on `24th and 31st of December 2025`, so 14:05 is
/// the day's final close of the availability envelope.
const HALF_DAY_14_05: u32 = 14 * 3_600 + 5 * 60;

/// Euronext Paris's built-in holiday rows and the window they were audited
/// over.
///
/// Every row is the Paris cell of one line of the operator's calendar — the
/// 2025 table at its 2025-12-06 capture or the 2026 table on the live page and
/// its INFO-FLASH PDF — except the two early closes, which the end-of-year
/// appendix states. A date inside the window with no row is audited normal.
// Evidence: docs/evidence/euronext_paris.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2026, 12, 31)],
    rows: [
        // 2025-01-01 - T1 - EURONEXT-HH-2025-12-06 - New Year's Day: Paris
        // `Closed`.
        (2025, 1, 1, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-04-18 - T1 - EURONEXT-HH-2025-12-06 - Good Friday: Paris
        // `Closed`.
        (2025, 4, 18, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-04-21 - T1 - EURONEXT-HH-2025-12-06 - Easter Monday: Paris
        // `Closed`.
        (2025, 4, 21, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-05-01 - T1 - EURONEXT-HH-2025-12-06 - Labour Day: Paris
        // `Closed`.
        (2025, 5, 1, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-12-24 - T1 - EURONEXT-EOY-2025 - Christmas Eve half day: Paris
        // `**Half Trading Day`; the appendix states TAL to 14:05 CET.
        (2025, 12, 24, early_close(HALF_DAY_14_05), T1, "EURONEXT-EOY-2025"),
        // 2025-12-25 - T1 - EURONEXT-HH-2025-12-06 - Christmas: Paris
        // `Closed`.
        (2025, 12, 25, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-12-26 - T1 - EURONEXT-HH-2025-12-06 - St Stephen's Day / Boxing
        // Day: Paris `Closed`.
        (2025, 12, 26, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-12-31 - T1 - EURONEXT-EOY-2025 - New Year's Eve half day: Paris
        // `**Half Trading Day`; the appendix states TAL to 14:05 CET.
        (2025, 12, 31, early_close(HALF_DAY_14_05), T1, "EURONEXT-EOY-2025"),
        // 2026-01-01 - T1 - EURONEXT-IF-2026 - New Year's Day: Paris `Closed`.
        (2026, 1, 1, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-04-03 - T1 - EURONEXT-IF-2026 - Good Friday: Paris `Closed`.
        (2026, 4, 3, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-04-06 - T1 - EURONEXT-IF-2026 - Easter Monday: Paris `Closed`.
        (2026, 4, 6, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-05-01 - T1 - EURONEXT-IF-2026 - Labour Day: Paris `Closed`.
        (2026, 5, 1, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-12-24 - T1 - EURONEXT-IF-2026 - Christmas Eve: Paris prints
        // `**Half Trading Day` but the 2026 end-of-year hours are announced
        // and unstated ("To be announced"), so no instant is claimed.
        (2026, 12, 24, Unsourced, T1, "EURONEXT-IF-2026"),
        // 2026-12-25 - T1 - EURONEXT-IF-2026 - Christmas: Paris `Closed`.
        (2026, 12, 25, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-12-31 - T1 - EURONEXT-IF-2026 - New Year's Eve: Paris prints
        // `**Half Trading Day` but the 2026 end-of-year hours are announced
        // and unstated ("To be announced"), so no instant is claimed.
        (2026, 12, 31, Unsourced, T1, "EURONEXT-IF-2026"),
    ],
};
