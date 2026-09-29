// SPDX-License-Identifier: MIT-0

//! London Stock Exchange (SETS) holiday rows, 2025-2027.
//!
//! Keyed by the crate's own venue-local trade date in `Europe/London`. LSE runs
//! no overnight session, so an event date and its trade date are one civil day
//! and the conversion is the identity.
//!
//! The whole block is **T1**: the operator's own "Bank holidays and their impact
//! on our trading services" table on its `Business days` page, delivered by the
//! content API behind `londonstockexchange.com`. That table is **rolling** — it
//! lists only business days from the present forward, to the next New Year — so
//! the audited years are pinned by three states of the same table: the capture
//! of 2024-02-07 (which reaches 2025-01-01), the capture of 2025-12-18 (which
//! covers 2025-12-24 through 2027-12-31) and the live table retrieved
//! 2026-09-28 (2026-08-31 onward, corroborating the two captures on every
//! overlapping row). A fourth capture, 2026-06-17, corroborates 2026-05-04 and
//! 2026-05-25. The per-row derivation and the five dates no surviving artifact
//! states are recorded in
//! [`docs/evidence/lse.md`](../../../../../docs/evidence/lse.md).
//!
//! Half days are **early closes at the operator's own instant**: the sheet
//! states `Markets closing process commences from 12:30 London time.` for the
//! Christmas Eve and New Year's Eve trade dates, so the day's final close is
//! 12:30 and the closing-auction phase that would follow disappears rather
//! than being invented. No other date carries an early close: LSE prints no
//! late opens and no half-day rows besides those two in any audited year.
//!
//! Five 2025 dates — 2025-04-18, 2025-04-21, 2025-05-05, 2025-05-26 and
//! 2025-08-25 — are inside the window but **not audited**: the rolling table
//! had already moved past them when the first 2025-era capture was taken
//! (2025-12-18 starts at 2025-12-24) and the Wayback index holds no capture of
//! the gap, so the rows ship as `Unsourced` and claim no closure.

use super::EvidenceTier::T1;
use super::HolidayKind::{Closed, Unsourced};
use super::fences::early_close;
use super::{HolidayTable, holidays};

/// CFE-style alias: LSE's stated half-day close, 12:30 London time.
///
/// The sheet's own words are `Markets closing process commences from 12:30
/// London time.` — the same boundary its normal week prints as 16:30 for the
/// closing-auction start, so the scalar is the day's final close.
const HALF_DAY_12_30: u32 = 12 * 3_600 + 30 * 60;

/// LSE's built-in holiday rows and the window they were audited over.
///
/// Every row is one line of the operator's business-days table at the capture
/// its document id names (2024-02-07, 2025-12-18) or in the live retrieval
/// (2026-09-28). A date inside the window with no row is audited normal;
/// 2025-04-18, 2025-04-21, 2025-05-05, 2025-05-26 and 2025-08-25 carry
/// `Unsourced` rows because no surviving artifact states them.
// Evidence: docs/evidence/lse.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2025-01-01 - T1 - LSE-BUSDAYS-2024-02-07 - New Year Day:
        // `NON-trading day.`
        (2025, 1, 1, Closed, T1, "LSE-BUSDAYS-2024-02-07"),
        // 2025-04-18 - T1 - LSE-BUSDAYS-2025-12-18 - Good Friday: the rolling
        // table had moved past this date before any surviving capture; no
        // status claimed.
        (2025, 4, 18, Unsourced, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2025-04-21 - T1 - LSE-BUSDAYS-2025-12-18 - Easter Monday: not stated
        // by any surviving artifact; no status claimed.
        (2025, 4, 21, Unsourced, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2025-05-05 - T1 - LSE-BUSDAYS-2025-12-18 - Early May Bank Holiday:
        // not stated by any surviving artifact; no status claimed.
        (2025, 5, 5, Unsourced, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2025-05-26 - T1 - LSE-BUSDAYS-2025-12-18 - Spring Bank Holiday: not
        // stated by any surviving artifact; no status claimed.
        (2025, 5, 26, Unsourced, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2025-08-25 - T1 - LSE-BUSDAYS-2025-12-18 - Summer Bank Holiday: not
        // stated by any surviving artifact; no status claimed.
        (2025, 8, 25, Unsourced, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2025-12-24 - T1 - LSE-BUSDAYS-2025-12-18 - Christmas Holiday half
        // day, closing process from 12:30 London time.
        (2025, 12, 24, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2025-12-18"),
        // 2025-12-25 - T1 - LSE-BUSDAYS-2025-12-18 - Christmas Day:
        // `NON-trading day.`
        (2025, 12, 25, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2025-12-26 - T1 - LSE-BUSDAYS-2025-12-18 - Boxing Day:
        // `NON-trading day.`
        (2025, 12, 26, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2025-12-31 - T1 - LSE-BUSDAYS-2025-12-18 - New Year's Holiday half
        // day, closing process from 12:30 London time.
        (2025, 12, 31, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-01-01 - T1 - LSE-BUSDAYS-2025-12-18 - New Year's Day:
        // `NON-trading day.`
        (2026, 1, 1, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-04-03 - T1 - LSE-BUSDAYS-2025-12-18 - Good Friday:
        // `NON-trading day.`
        (2026, 4, 3, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-04-06 - T1 - LSE-BUSDAYS-2025-12-18 - Easter Monday:
        // `NON-trading day.`
        (2026, 4, 6, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-05-04 - T1 - LSE-BUSDAYS-2025-12-18 - Early May Bank Holiday:
        // `NON-trading day.`
        (2026, 5, 4, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-05-25 - T1 - LSE-BUSDAYS-2025-12-18 - Spring Bank Holiday:
        // `NON-trading day.`
        (2026, 5, 25, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-08-31 - T1 - LSE-BUSDAYS-2025-12-18 - Summer Bank Holiday:
        // `NON-trading day.`
        (2026, 8, 31, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-12-24 - T1 - LSE-BUSDAYS-2025-12-18 - Christmas Holiday half
        // day, closing process from 12:30 London time.
        (2026, 12, 24, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-12-25 - T1 - LSE-BUSDAYS-2025-12-18 - Christmas Day:
        // `NON-trading day.`
        (2026, 12, 25, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-12-28 - T1 - LSE-BUSDAYS-2025-12-18 - Boxing Day (substitute):
        // `NON-trading day.`
        (2026, 12, 28, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2026-12-31 - T1 - LSE-BUSDAYS-2025-12-18 - New Year's Holiday half
        // day, closing process from 12:30 London time.
        (2026, 12, 31, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-01-01 - T1 - LSE-BUSDAYS-2025-12-18 - New Year's Day:
        // `NON-trading day.`
        (2027, 1, 1, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-03-26 - T1 - LSE-BUSDAYS-2025-12-18 - Good Friday:
        // `NON-trading day.`
        (2027, 3, 26, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-03-29 - T1 - LSE-BUSDAYS-2025-12-18 - Easter Monday:
        // `NON-trading day.`
        (2027, 3, 29, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-05-03 - T1 - LSE-BUSDAYS-2025-12-18 - Early May Bank Holiday:
        // `NON-trading day.`
        (2027, 5, 3, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-05-31 - T1 - LSE-BUSDAYS-2025-12-18 - Spring Bank Holiday:
        // `NON-trading day.`
        (2027, 5, 31, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-08-30 - T1 - LSE-BUSDAYS-2025-12-18 - Summer Bank Holiday:
        // `NON-trading day.`
        (2027, 8, 30, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-12-24 - T1 - LSE-BUSDAYS-2025-12-18 - Christmas Holiday half
        // day, closing process from 12:30 London time.
        (2027, 12, 24, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-12-27 - T1 - LSE-BUSDAYS-2025-12-18 - Christmas Day
        // (substitute): `NON-trading day.`
        (2027, 12, 27, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-12-28 - T1 - LSE-BUSDAYS-2025-12-18 - Boxing Day (substitute):
        // `NON-trading day.`
        (2027, 12, 28, Closed, T1, "LSE-BUSDAYS-2025-12-18"),
        // 2027-12-31 - T1 - LSE-BUSDAYS-2025-12-18 - New Year's Holiday half
        // day, closing process from 12:30 London time.
        (2027, 12, 31, early_close(HALF_DAY_12_30), T1, "LSE-BUSDAYS-2025-12-18"),
    ],
};
