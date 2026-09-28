// SPDX-License-Identifier: MIT-0

//! Xetra (Deutsche Börse cash market) holiday rows, 2025-2027.
//!
//! Keyed by the crate's own venue-local trade date in `Europe/Berlin`. The
//! operator's `Trading calendar and trading hours` page lists the non-trading
//! days of Frankfurter Wertpapierbörse — the closure set for Xetra and Börse
//! Frankfurt alike — and its per-year `Trading calendar` PDFs restate each
//! year's set in one sentence. The whole block is **T1**.
//!
//! Christmas Eve and New Year's Eve are full closures here, not early closes:
//! the operator's own `**` footnote reads "No trading but settlement is open",
//! which states that the market does not trade. Early closes appear only where
//! the operator names both the day and the instant: the live page names 2026's
//! trading holidays — Ascension Day (14 May 2026), Whit Monday (25 May 2026),
//! Corpus Christi (4 June 2026) — and states that "Trading of shares and
//! Exchange traded products on Frankfurt and Xetra ends on public holidays
//! ... at 20:00 CET". The 2025 edition of the same page words the 20:00 note
//! over Börse Frankfurt only, so no 2025 Xetra early close is sourced and
//! none ships; the 2027 trading holidays are named by no retrieved artifact,
//! so 2027 ships closures only. Both omissions are recorded as residual gaps
//! in [`docs/evidence/xetra.md`](../../../../../docs/evidence/xetra.md).
//!
//! The page's conditional note — "On December 30, 2026, deviating trading
//! hours may apply", echoed by the `*)` footnote on the 2026 PDF — keys no row
//! (LAW-NO-FABRICATED-DATES).

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::fences::early_close;
use super::{HolidayTable, holidays};

/// The trading-holiday close on shares and exchange traded products, `20:00`
/// venue-local, in seconds since midnight.
const HOLIDAY_CLOSE_SSM: u32 = 20 * 3_600;

/// Xetra's built-in holiday rows and the window they were audited over.
///
/// Every closure row is one line of the operator's per-year calendar sentence
/// ("there will be trading Mondays to Fridays in <year>, with the exception
/// of ...") or the equivalent cell of the page's non-trading-days table; the
/// three 2026 early closes are the page's own named 2026 trading holidays
/// under its 20:00 close rule. A date inside the window with no row is audited
/// normal.
// Evidence: docs/evidence/xetra.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2025-01-01 - T1 - DB-TC-PDF-2025 - New Year's Day.
        (2025, 1, 1, Closed, T1, "DB-TC-PDF-2025"),
        // 2025-04-18 - T1 - DB-TC-PDF-2025 - Good Friday.
        (2025, 4, 18, Closed, T1, "DB-TC-PDF-2025"),
        // 2025-04-21 - T1 - DB-TC-PDF-2025 - Easter Monday.
        (2025, 4, 21, Closed, T1, "DB-TC-PDF-2025"),
        // 2025-05-01 - T1 - DB-TC-PDF-2025 - Labour Day.
        (2025, 5, 1, Closed, T1, "DB-TC-PDF-2025"),
        // 2025-12-24 - T1 - DB-TC-PDF-2025 - Christmas Eve: "No trading but
        // settlement is open".
        (2025, 12, 24, Closed, T1, "DB-TC-PDF-2025"),
        // 2025-12-25 - T1 - DB-TC-PDF-2025 - Christmas Day.
        (2025, 12, 25, Closed, T1, "DB-TC-PDF-2025"),
        // 2025-12-26 - T1 - DB-TC-PDF-2025 - Boxing Day.
        (2025, 12, 26, Closed, T1, "DB-TC-PDF-2025"),
        // 2025-12-31 - T1 - DB-TC-PDF-2025 - New Year's Eve: "No trading but
        // settlement is open".
        (2025, 12, 31, Closed, T1, "DB-TC-PDF-2025"),
        // 2026-01-01 - T1 - DB-TC-PDF-2026 - New Year's Day.
        (2026, 1, 1, Closed, T1, "DB-TC-PDF-2026"),
        // 2026-04-03 - T1 - DB-TC-PDF-2026 - Good Friday.
        (2026, 4, 3, Closed, T1, "DB-TC-PDF-2026"),
        // 2026-04-06 - T1 - DB-TC-PDF-2026 - Easter Monday.
        (2026, 4, 6, Closed, T1, "DB-TC-PDF-2026"),
        // 2026-05-01 - T1 - DB-TC-PDF-2026 - Labour Day.
        (2026, 5, 1, Closed, T1, "DB-TC-PDF-2026"),
        // 2026-05-14 - T1 - DB-TC-PAGE - Ascension Day, a named 2026 trading
        // holiday; shares and ETPs end at 20:00 CET.
        (2026, 5, 14, early_close(HOLIDAY_CLOSE_SSM), T1, "DB-TC-PAGE"),
        // 2026-05-25 - T1 - DB-TC-PAGE - Whit Monday, a named 2026 trading
        // holiday; shares and ETPs end at 20:00 CET.
        (2026, 5, 25, early_close(HOLIDAY_CLOSE_SSM), T1, "DB-TC-PAGE"),
        // 2026-06-04 - T1 - DB-TC-PAGE - Corpus Christi, a named 2026 trading
        // holiday; shares and ETPs end at 20:00 CET.
        (2026, 6, 4, early_close(HOLIDAY_CLOSE_SSM), T1, "DB-TC-PAGE"),
        // 2026-12-24 - T1 - DB-TC-PDF-2026 - Christmas Eve: "No trading but
        // settlement is open".
        (2026, 12, 24, Closed, T1, "DB-TC-PDF-2026"),
        // 2026-12-25 - T1 - DB-TC-PDF-2026 - Christmas Day.
        (2026, 12, 25, Closed, T1, "DB-TC-PDF-2026"),
        // 2026-12-31 - T1 - DB-TC-PDF-2026 - New Year's Eve: "No trading but
        // settlement is open".
        (2026, 12, 31, Closed, T1, "DB-TC-PDF-2026"),
        // 2027-01-01 - T1 - DB-TC-PAGE - New Year's Day.
        (2027, 1, 1, Closed, T1, "DB-TC-PAGE"),
        // 2027-03-26 - T1 - DB-TC-PAGE - Good Friday.
        (2027, 3, 26, Closed, T1, "DB-TC-PAGE"),
        // 2027-03-29 - T1 - DB-TC-PAGE - Easter Monday.
        (2027, 3, 29, Closed, T1, "DB-TC-PAGE"),
        // 2027-12-24 - T1 - DB-TC-PAGE - Christmas Eve: "No trading but
        // settlement is open".
        (2027, 12, 24, Closed, T1, "DB-TC-PAGE"),
        // 2027-12-31 - T1 - DB-TC-PAGE - New Year's Eve: "No trading but
        // settlement is open".
        (2027, 12, 31, Closed, T1, "DB-TC-PAGE"),
    ],
};
