// SPDX-License-Identifier: MIT-0

//! CME Live Cattle, Feeder Cattle and Lean Hog holiday rows, 2025-2027.
//!
//! Keyed by the crate's own venue-local trade date in `America/Chicago`
//! (design memo D1). The conversion is the identity for this family and is the
//! reason the module is short: from 2016-02-29 the grid is a flat Monday-Friday
//! 08:30-13:05 CT regular session with an 08:00-08:30 CT Pre-Open and a
//! 14:30-16:00 CT Post-Close queue, all inside one local day, so no session
//! wraps a midnight and every occurrence's trade date is its own civil date.
//! CME's event-date records therefore key one row each, and the operator's own
//! printed trade date corroborates every one of them.
//!
//! The rows come from CME's trading-hours service — the endpoint
//! `cmegroup.com/trading-hours.html` itself calls — read as bytes and saved,
//! so the whole block is **T2** under LAW-PRIMARY-SOURCES. CME publishes no T1
//! per-asset-class rendering for these years; that, the eight 2025 windows that
//! survive only in a pre-holiday capture, and the two Saturday sessions this
//! family never had are recorded as gaps in
//! [`docs/evidence/globex_livestock.md`](../../../../../docs/evidence/globex_livestock.md).
//!
//! Two shapes only: `Closed` on a full Globex closure, and `EarlyClose` on the
//! four half-days CME publishes for the family. There is no late open in this
//! window, and no row whose internal phase topology the scalar vocabulary
//! cannot state.

use super::fences::early_close;
use super::{EvidenceTier::T2, HolidayKind::Closed, HolidayTable, holidays};

/// The family's built-in holiday rows and the window they were audited over.
///
/// Coverage runs to 2027-12-31, the end of the operator's published future;
/// CME's 2028-01-01 record sits outside it and ships no row.
// Evidence: docs/evidence/globex_livestock.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: (2025, 1, 1) ..= (2027, 12, 31),
    rows: [
        // 2025-01-01 - T2 - CME-SVC-2025-01-01 - New Year's Day, no events published.
        (2025, 1, 1, Closed, T2, "CME-SVC-2025-01-01"),
        // 2025-01-20 - T2 - CME-SVC-2025-01-20 - Martin Luther King Jr. Day, no events.
        (2025, 1, 20, Closed, T2, "CME-SVC-2025-01-20"),
        // 2025-02-17 - T2 - CME-SVC-2025-02-17 - Presidents' Day, no events published.
        (2025, 2, 17, Closed, T2, "CME-SVC-2025-02-17"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-18 - Good Friday, no events published.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-18"),
        // 2025-05-26 - T2 - CME-SVC-2025-05-26 - Memorial Day, no events published.
        (2025, 5, 26, Closed, T2, "CME-SVC-2025-05-26"),
        // 2025-06-19 - T2 - CME-SVC-2025-06-19 - Juneteenth, no events published.
        (2025, 6, 19, Closed, T2, "CME-SVC-2025-06-19"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-04 - Independence Day, no events published.
        (2025, 7, 4, Closed, T2, "CME-SVC-2025-07-04"),
        // 2025-09-01 - T2 - CME-SVC-2025-09-01 - Labor Day, no events published.
        (2025, 9, 1, Closed, T2, "CME-SVC-2025-09-01"),
        // 2025-11-27 - T2 - CME-SVC-2025-11-27 - Thanksgiving, no events published.
        (2025, 11, 27, Closed, T2, "CME-SVC-2025-11-27"),
        // 2025-11-28 - T2 - CME-SVC-2025-11-28 - day after Thanksgiving, 12:05 CT close.
        (2025, 11, 28, early_close(12 * 3_600 + 5 * 60), T2, "CME-SVC-2025-11-28"),
        // 2025-11-29 - T2 - CME-SVC-2025-11-29 - Thanksgiving Saturday, no events.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-29"),
        // 2025-12-24 - T2 - CME-SVC-2025-12-24 - Christmas Eve, 12:15 CT close.
        (2025, 12, 24, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-2025-12-24"),
        // 2025-12-25 - T2 - CME-SVC-2025-12-25 - Christmas Day, no events published.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-25"),
        // 2026-01-01 - T2 - CME-SVC-2026-01-01 - New Year's Day, no events published.
        (2026, 1, 1, Closed, T2, "CME-SVC-2026-01-01"),
        // 2026-01-19 - T2 - CME-SVC-2026-01-19 - Martin Luther King Jr. Day, no events.
        (2026, 1, 19, Closed, T2, "CME-SVC-2026-01-19"),
        // 2026-02-16 - T2 - CME-SVC-2026-02-16 - Presidents' Day, no events published.
        (2026, 2, 16, Closed, T2, "CME-SVC-2026-02-16"),
        // 2026-04-03 - T2 - CME-SVC-2026-04-03 - Good Friday, no events published.
        (2026, 4, 3, Closed, T2, "CME-SVC-2026-04-03"),
        // 2026-05-25 - T2 - CME-SVC-2026-05-25 - Memorial Day, no events published.
        (2026, 5, 25, Closed, T2, "CME-SVC-2026-05-25"),
        // 2026-06-19 - T2 - CME-SVC-2026-06-19 - Juneteenth, no events published.
        (2026, 6, 19, Closed, T2, "CME-SVC-2026-06-19"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - Independence Day observed, no events.
        (2026, 7, 3, Closed, T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 - T2 - CME-SVC-2026-09-07 - Labor Day, no events published.
        (2026, 9, 7, Closed, T2, "CME-SVC-2026-09-07"),
        // 2026-11-26 - T2 - CME-SVC-2026-11-26 - Thanksgiving, no events published.
        (2026, 11, 26, Closed, T2, "CME-SVC-2026-11-26"),
        // 2026-11-27 - T2 - CME-SVC-2026-11-27 - day after Thanksgiving, 12:05 CT close.
        (2026, 11, 27, early_close(12 * 3_600 + 5 * 60), T2, "CME-SVC-2026-11-27"),
        // 2026-12-24 - T2 - CME-SVC-2026-12-24 - Christmas Eve, 12:05 CT close.
        (2026, 12, 24, early_close(12 * 3_600 + 5 * 60), T2, "CME-SVC-2026-12-24"),
        // 2026-12-25 - T2 - CME-SVC-2026-12-25 - Christmas Day, no events published.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-25"),
        // 2027-01-01 - T2 - CME-SVC-2027-01-01 - New Year's Day, no events published.
        (2027, 1, 1, Closed, T2, "CME-SVC-2027-01-01"),
        // 2027-01-18 - T2 - CME-SVC-2027-01-18 - Martin Luther King Jr. Day, no events.
        (2027, 1, 18, Closed, T2, "CME-SVC-2027-01-18"),
        // 2027-02-15 - T2 - CME-SVC-2027-02-15 - Presidents' Day, no events published.
        (2027, 2, 15, Closed, T2, "CME-SVC-2027-02-15"),
        // 2027-03-26 - T2 - CME-SVC-2027-03-26 - Good Friday, no events published.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-26"),
        // 2027-05-31 - T2 - CME-SVC-2027-05-31 - Memorial Day, no events published.
        (2027, 5, 31, Closed, T2, "CME-SVC-2027-05-31"),
        // 2027-06-18 - T2 - CME-SVC-2027-06-18 - Juneteenth observed, no events.
        (2027, 6, 18, Closed, T2, "CME-SVC-2027-06-18"),
        // 2027-07-05 - T2 - CME-SVC-2027-07-05 - Independence Day observed, no events.
        (2027, 7, 5, Closed, T2, "CME-SVC-2027-07-05"),
        // 2027-09-06 - T2 - CME-SVC-2027-09-06 - Labor Day, no events published.
        (2027, 9, 6, Closed, T2, "CME-SVC-2027-09-06"),
        // 2027-11-25 - T2 - CME-SVC-2027-11-25 - Thanksgiving, no events published.
        (2027, 11, 25, Closed, T2, "CME-SVC-2027-11-25"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-26 - day after Thanksgiving, 12:05 CT close.
        (2027, 11, 26, early_close(12 * 3_600 + 5 * 60), T2, "CME-SVC-2027-11-26"),
        // 2027-12-24 - T2 - CME-SVC-2027-12-24 - Christmas Friday closure, no events.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-24"),
    ],
};
