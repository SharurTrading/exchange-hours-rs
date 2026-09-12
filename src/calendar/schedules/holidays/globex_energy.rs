// SPDX-License-Identifier: MIT-0

//! CME NYMEX energy and COMEX metals holiday rows, 2025-2027.
//!
//! Keyed by the crate's own venue-local trade date in `America/Chicago`
//! (design memo D1), never by CME's event date. The family's grid wraps: from
//! the 2015 revision one occurrence opens 17:00 CT on the previous local day
//! and closes 16:00 CT on the trade date, Sunday through Thursday, so a trade
//! date's session begins the evening before and CME's event-date records must
//! be converted before they can key a row.
//!
//! The conversion has three shapes in this window, and each one is recorded
//! per year in the evidence file:
//!
//! - CME publishes a final close on the date at an instant earlier than 16:00
//!   CT — `12:00`, `12:45`, `13:45` — which is an `EarlyClose` on that trade
//!   date. It clips the occurrence that opened the previous evening, because
//!   the clip is stated on the trade date rather than on a civil day.
//! - CME publishes no final close on the date and only a pre-open and a 17:00
//!   CT open carrying the **next** business day's trade date, or publishes no
//!   events at all. No session belongs to the date, so the row is `Closed` and
//!   the previous evening's 17:00 CT leg disappears with it.
//! - CME publishes only a pre-open on the date, at 13:30 CT, followed by the
//!   ordinary 17:00 CT open. A pre-open is order entry with no matching by
//!   CME's own legend, so matching ended at 13:30 CT: that is the trade date's
//!   final close and the row is an `EarlyClose` at 13:30 CT.
//!
//! On the three Friday holidays of 2026 and 2027 CME prints the early close
//! but dates it to the following Monday. The crate assigns a session to the
//! venue-local date of its own final close, so the row stays on the Friday and
//! the divergence is recorded as an interpretive step rather than modelled; a
//! `Closed` row there would delete roughly nineteen hours of sourced trading,
//! because this family has a weekend close and no following-business-day roll.
//!
//! Every row is **T2** under LAW-PRIMARY-SOURCES: CME's trading-hours service,
//! the endpoint `cmegroup.com/trading-hours.html` itself calls, read as bytes
//! and saved. No T1 per-asset-class rendering exists for these years. That,
//! the Saturday sessions after the Friday holidays, the 2025-11-28 morning
//! re-open pair, the order-entry deviations and the eight 2025 windows that
//! survive only in a pre-holiday capture are gaps recorded in
//! [`docs/evidence/globex_energy.md`](../../../../../docs/evidence/globex_energy.md).
//!
//! Energy and metals are one key and CME prints them as one product-group row
//! on every date in this window, so the memo's D17 intersection rule is never
//! reached: the two halves agree everywhere.

use super::fences::early_close;
use super::{EvidenceTier::T2, HolidayKind::Closed, HolidayTable, holidays};

/// The family's built-in holiday rows and the window they were audited over.
///
/// Coverage runs to 2027-12-31, the end of the operator's published future;
/// CME's 2028-01-01 record sits outside it and ships no row. Inside the window
/// a date with no row is audited normal, including Columbus Day and Veterans
/// Day, which are not CME Globex holidays at all.
// Evidence: docs/evidence/globex_energy.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: (2025, 1, 1) ..= (2027, 12, 31),
    rows: [
        // 2025-01-01 - T2 - CME-SVC-2025-01-01 - New Year's Day; no trade date of its own.
        (2025, 1, 1, Closed, T2, "CME-SVC-2025-01-01"),
        // 2025-01-20 - T2 - CME-SVC-2025-01-20 - Martin Luther King Jr. Day, 13:30 CT close.
        (2025, 1, 20, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-01-20"),
        // 2025-02-17 - T2 - CME-SVC-2025-02-17 - Presidents' Day, 13:30 CT close.
        (2025, 2, 17, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-02-17"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-18 - Good Friday; no events published.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-18"),
        // 2025-05-26 - T2 - CME-SVC-2025-05-26 - Memorial Day, 13:30 CT close.
        (2025, 5, 26, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-05-26"),
        // 2025-06-19 - T2 - CME-SVC-2025-06-19 - Juneteenth, 13:30 CT close.
        (2025, 6, 19, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-06-19"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-04 - Independence Day, 12:00 CT close.
        (2025, 7, 4, early_close(12 * 3_600), T2, "CME-SVC-2025-07-04"),
        // 2025-09-01 - T2 - CME-SVC-2025-09-01 - Labor Day, 13:30 CT close.
        (2025, 9, 1, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-09-01"),
        // 2025-11-27 - T2 - CME-SVC-2025-11-27 - Thanksgiving, 13:30 CT close.
        (2025, 11, 27, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-11-27"),
        // 2025-11-28 - T2 - CME-SVC-2025-11-28 - day after Thanksgiving, 13:45 CT close.
        (2025, 11, 28, early_close(13 * 3_600 + 45 * 60), T2, "CME-SVC-2025-11-28"),
        // 2025-11-29 - T2 - CME-SVC-2025-11-29 - Thanksgiving Saturday; no events published.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-29"),
        // 2025-12-24 - T2 - CME-SVC-2025-12-24 - Christmas Eve, 12:45 CT close.
        (2025, 12, 24, early_close(12 * 3_600 + 45 * 60), T2, "CME-SVC-2025-12-24"),
        // 2025-12-25 - T2 - CME-SVC-2025-12-25 - Christmas Day; no trade date of its own.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-25"),
        // 2026-01-01 - T2 - CME-SVC-2026-01-01 - New Year's Day; no trade date of its own.
        (2026, 1, 1, Closed, T2, "CME-SVC-2026-01-01"),
        // 2026-01-19 - T2 - CME-SVC-2026-01-19 - Martin Luther King Jr. Day, 13:30 CT close.
        (2026, 1, 19, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-01-19"),
        // 2026-02-16 - T2 - CME-SVC-2026-02-16 - Presidents' Day, 13:30 CT close.
        (2026, 2, 16, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-02-16"),
        // 2026-04-03 - T2 - CME-SVC-2026-04-03 - Good Friday; no events published.
        (2026, 4, 3, Closed, T2, "CME-SVC-2026-04-03"),
        // 2026-05-25 - T2 - CME-SVC-2026-05-25 - Memorial Day, 13:30 CT close.
        (2026, 5, 25, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-05-25"),
        // 2026-06-19 - T2 - CME-SVC-2026-06-19 - Juneteenth, 12:00 CT close dated to 06-22.
        (2026, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-2026-06-19"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - Independence Day observed, 12:00 CT close.
        (2026, 7, 3, early_close(12 * 3_600), T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 - T2 - CME-SVC-2026-09-07 - Labor Day, 13:30 CT close.
        (2026, 9, 7, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-09-07"),
        // 2026-11-26 - T2 - CME-SVC-2026-11-26 - Thanksgiving, 13:30 CT close.
        (2026, 11, 26, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-11-26"),
        // 2026-11-27 - T2 - CME-SVC-2026-11-27 - day after Thanksgiving, 13:45 CT close.
        (2026, 11, 27, early_close(13 * 3_600 + 45 * 60), T2, "CME-SVC-2026-11-27"),
        // 2026-12-24 - T2 - CME-SVC-2026-12-24 - Christmas Eve, 12:45 CT close.
        (2026, 12, 24, early_close(12 * 3_600 + 45 * 60), T2, "CME-SVC-2026-12-24"),
        // 2026-12-25 - T2 - CME-SVC-2026-12-25 - Christmas Day; no events published.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-25"),
        // 2027-01-01 - T2 - CME-SVC-2027-01-01 - New Year's Day; no events published.
        (2027, 1, 1, Closed, T2, "CME-SVC-2027-01-01"),
        // 2027-01-18 - T2 - CME-SVC-2027-01-18 - Martin Luther King Jr. Day, 13:30 CT close.
        (2027, 1, 18, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-01-18"),
        // 2027-02-15 - T2 - CME-SVC-2027-02-15 - Presidents' Day, 13:30 CT close.
        (2027, 2, 15, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-02-15"),
        // 2027-03-26 - T2 - CME-SVC-2027-03-26 - Good Friday; no events published.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-26"),
        // 2027-05-31 - T2 - CME-SVC-2027-05-31 - Memorial Day, 13:30 CT close.
        (2027, 5, 31, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-05-31"),
        // 2027-06-18 - T2 - CME-SVC-2027-06-18 - Juneteenth observed, 12:00 CT close.
        (2027, 6, 18, early_close(12 * 3_600), T2, "CME-SVC-2027-06-18"),
        // 2027-07-05 - T2 - CME-SVC-2027-07-05 - Independence Day observed, 13:30 CT close.
        (2027, 7, 5, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-07-05"),
        // 2027-09-06 - T2 - CME-SVC-2027-09-06 - Labor Day, 13:30 CT close.
        (2027, 9, 6, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-09-06"),
        // 2027-11-25 - T2 - CME-SVC-2027-11-25 - Thanksgiving, 13:30 CT close.
        (2027, 11, 25, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-11-25"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-26 - day after Thanksgiving, 13:45 CT close.
        (2027, 11, 26, early_close(13 * 3_600 + 45 * 60), T2, "CME-SVC-2027-11-26"),
        // 2027-12-24 - T2 - CME-SVC-2027-12-24 - Christmas Friday; no events published.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-24"),
    ],
};
