// SPDX-License-Identifier: MIT-0

//! CME Nikkei 225 Dollar (`NKD`) holiday and early-close rows, 2025-2027.
//!
//! Keyed by the crate's own venue-local trade date in `America/Chicago`
//! (design memo D1). The conversion is never the identity here: the family
//! runs one wrapped envelope per trade date — 17:00 CT on the previous evening
//! to 16:00 CT on the trade date — so CME's event-date records for a holiday
//! and its eve collapse into one row keyed to the day the session's final
//! close falls on, and the eve record is evidence for that row rather than a
//! row of its own.
//!
//! The rows come from CME's trading-hours service — the endpoint
//! `cmegroup.com/trading-hours.html` itself calls — read as bytes and saved,
//! so the whole block is **T2** under LAW-PRIMARY-SOURCES. From Thanksgiving
//! 2025 the service publishes `NKD` and `NIY` as their own line; through Labor
//! Day 2025 it does not, and those eight rows are taken from the Equity Index
//! line of the same capture. That interpretive step, the two Saturday sessions
//! the scalar vocabulary cannot state, and the one sourced intraday-topology
//! day are recorded in
//! [`docs/evidence/globex_nikkei_225_dollar.md`](../../../../../docs/evidence/globex_nikkei_225_dollar.md).
//!
//! Two shapes only: `Closed` on a full Globex closure, and `EarlyClose` on the
//! half-days. The family has no late open in this window, because every CME
//! re-open after a closure is the grid's own 17:00 CT evening open.

use super::fences::early_close;
use super::{EvidenceTier::T2, HolidayKind::Closed, HolidayTable, holidays};

/// The family's built-in holiday rows and the window they were audited over.
///
/// Coverage runs to 2027-12-31, the end of the operator's published future;
/// CME's 2028-01-01 record is a Saturday outside it and ships no row.
// Evidence: docs/evidence/globex_nikkei_225_dollar.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: (2025, 1, 1) ..= (2027, 12, 31),
    rows: [
        // 2025-01-01 - T2 - CME-SVC-2025-01-01 - New Year's Day, no trade date of its own.
        (2025, 1, 1, Closed, T2, "CME-SVC-2025-01-01"),
        // 2025-01-20 - T2 - CME-SVC-2025-01-20 - Martin Luther King Jr. Day, 12:00 CT close.
        (2025, 1, 20, early_close(12 * 3_600), T2, "CME-SVC-2025-01-20"),
        // 2025-02-17 - T2 - CME-SVC-2025-02-17 - Presidents' Day, 12:00 CT close.
        (2025, 2, 17, early_close(12 * 3_600), T2, "CME-SVC-2025-02-17"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-18 - Good Friday, no events published.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-18"),
        // 2025-05-26 - T2 - CME-SVC-2025-05-26 - Memorial Day, 12:00 CT close.
        (2025, 5, 26, early_close(12 * 3_600), T2, "CME-SVC-2025-05-26"),
        // 2025-06-19 - T2 - CME-SVC-2025-06-19 - Juneteenth, 12:00 CT close.
        (2025, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-2025-06-19"),
        // 2025-07-03 - T2 - CME-SVC-2025-07-03 - Independence Day eve, 12:15 CT close.
        (2025, 7, 3, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-2025-07-03"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-04 - Independence Day, 12:00 CT close.
        (2025, 7, 4, early_close(12 * 3_600), T2, "CME-SVC-2025-07-04"),
        // 2025-09-01 - T2 - CME-SVC-2025-09-01 - Labor Day, 12:00 CT close.
        (2025, 9, 1, early_close(12 * 3_600), T2, "CME-SVC-2025-09-01"),
        // 2025-11-27 - T2 - CME-SVC-B-2025-11-27 - Thanksgiving, 12:00 CT close.
        (2025, 11, 27, early_close(12 * 3_600), T2, "CME-SVC-B-2025-11-27"),
        // 2025-11-28 - T2 - CME-SVC-B-2025-11-28 - day after Thanksgiving, 12:15 CT close.
        (2025, 11, 28, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-B-2025-11-28"),
        // 2025-12-24 - T2 - CME-SVC-B-2025-12-24 - Christmas Eve, 12:15 CT close.
        (2025, 12, 24, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-B-2025-12-24"),
        // 2025-12-25 - T2 - CME-SVC-B-2025-12-25 - Christmas Day, no trade date of its own.
        (2025, 12, 25, Closed, T2, "CME-SVC-B-2025-12-25"),
        // 2026-01-01 - T2 - CME-SVC-B-2026-01-01 - New Year's Day, no trade date of its own.
        (2026, 1, 1, Closed, T2, "CME-SVC-B-2026-01-01"),
        // 2026-01-19 - T2 - CME-SVC-B-2026-01-19 - Martin Luther King Jr. Day, 12:00 CT close.
        (2026, 1, 19, early_close(12 * 3_600), T2, "CME-SVC-B-2026-01-19"),
        // 2026-02-16 - T2 - CME-SVC-B-2026-02-16 - Presidents' Day, 12:00 CT close.
        (2026, 2, 16, early_close(12 * 3_600), T2, "CME-SVC-B-2026-02-16"),
        // 2026-04-03 - T2 - CME-SVC-B-2026-04-03 - Good Friday, 08:15 CT close with Equity Index.
        (2026, 4, 3, early_close(8 * 3_600 + 15 * 60), T2, "CME-SVC-B-2026-04-03"),
        // 2026-05-25 - T2 - CME-SVC-B-2026-05-25 - Memorial Day, 12:00 CT close.
        (2026, 5, 25, early_close(12 * 3_600), T2, "CME-SVC-B-2026-05-25"),
        // 2026-06-19 - T2 - CME-SVC-B-2026-06-19 - Juneteenth, 12:00 CT close.
        (2026, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-B-2026-06-19"),
        // 2026-07-03 - T2 - CME-SVC-B-2026-07-03 - Independence Day observed, 12:00 CT close.
        (2026, 7, 3, early_close(12 * 3_600), T2, "CME-SVC-B-2026-07-03"),
        // 2026-09-07 - T2 - CME-SVC-B-2026-09-07 - Labor Day, 12:00 CT close.
        (2026, 9, 7, early_close(12 * 3_600), T2, "CME-SVC-B-2026-09-07"),
        // 2026-11-26 - T2 - CME-SVC-B-2026-11-26 - Thanksgiving, 12:00 CT close.
        (2026, 11, 26, early_close(12 * 3_600), T2, "CME-SVC-B-2026-11-26"),
        // 2026-11-27 - T2 - CME-SVC-B-2026-11-27 - day after Thanksgiving, 12:15 CT close.
        (2026, 11, 27, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-B-2026-11-27"),
        // 2026-12-24 - T2 - CME-SVC-B-2026-12-24 - Christmas Eve, 12:15 CT close.
        (2026, 12, 24, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-B-2026-12-24"),
        // 2026-12-25 - T2 - CME-SVC-B-2026-12-25 - Christmas Day, no events published.
        (2026, 12, 25, Closed, T2, "CME-SVC-B-2026-12-25"),
        // 2027-01-01 - T2 - CME-SVC-B-2027-01-01 - New Year's Day, no events published.
        (2027, 1, 1, Closed, T2, "CME-SVC-B-2027-01-01"),
        // 2027-01-18 - T2 - CME-SVC-B-2027-01-18 - Martin Luther King Jr. Day, 12:00 CT close.
        (2027, 1, 18, early_close(12 * 3_600), T2, "CME-SVC-B-2027-01-18"),
        // 2027-02-15 - T2 - CME-SVC-B-2027-02-15 - Presidents' Day, 12:00 CT close.
        (2027, 2, 15, early_close(12 * 3_600), T2, "CME-SVC-B-2027-02-15"),
        // 2027-03-26 - T2 - CME-SVC-B-2027-03-26 - Good Friday, no events published.
        (2027, 3, 26, Closed, T2, "CME-SVC-B-2027-03-26"),
        // 2027-05-31 - T2 - CME-SVC-B-2027-05-31 - Memorial Day, 12:00 CT close.
        (2027, 5, 31, early_close(12 * 3_600), T2, "CME-SVC-B-2027-05-31"),
        // 2027-06-18 - T2 - CME-SVC-B-2027-06-18 - Juneteenth observed, 12:00 CT close.
        (2027, 6, 18, early_close(12 * 3_600), T2, "CME-SVC-B-2027-06-18"),
        // 2027-07-05 - T2 - CME-SVC-B-2027-07-05 - Independence Day observed, 12:00 CT close.
        (2027, 7, 5, early_close(12 * 3_600), T2, "CME-SVC-B-2027-07-05"),
        // 2027-09-06 - T2 - CME-SVC-B-2027-09-06 - Labor Day, 12:00 CT close.
        (2027, 9, 6, early_close(12 * 3_600), T2, "CME-SVC-B-2027-09-06"),
        // 2027-11-25 - T2 - CME-SVC-B-2027-11-25 - Thanksgiving, 12:00 CT close.
        (2027, 11, 25, early_close(12 * 3_600), T2, "CME-SVC-B-2027-11-25"),
        // 2027-11-26 - T2 - CME-SVC-B-2027-11-26 - day after Thanksgiving, 12:15 CT close.
        (2027, 11, 26, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-B-2027-11-26"),
        // 2027-12-24 - T2 - CME-SVC-B-2027-12-24 - Christmas Friday closure, no events.
        (2027, 12, 24, Closed, T2, "CME-SVC-B-2027-12-24"),
    ],
};
