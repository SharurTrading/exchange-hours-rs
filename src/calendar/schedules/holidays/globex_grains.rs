// SPDX-License-Identifier: MIT-0

//! CBOT standard-size grain and oilseed futures holiday rows, 2025-2027.
//!
//! Keyed by the crate's own venue-local trade date in `America/Chicago`
//! (design memo D1), never by CME's event date. That conversion is not the
//! identity for this family: the sourced grid is a wrapping one — an evening
//! leg at 19:00 CT that runs to 07:45 CT the next morning, then an 08:30-13:20
//! CT day session whose close is the trading day's final close — so the
//! trading day for trade date `D` opens on `D - 1`. CME's own printed trade
//! date corroborates every row below, and it is what settles the conversion
//! wherever an event-date record could be read two ways.
//!
//! Three consequences of that shape, worked out once here so the rows read
//! plainly:
//!
//! * A holiday eve on which CME publishes the whole day session and then no
//!   `16:45 preopen` / `19:00 open` pair is **not** a row of its own. The
//!   evening leg CME withheld belongs to the holiday's trade date, and the
//!   neighbouring `Closed` row already deletes it (memo D6, §1.6 case 3). All
//!   eleven such dates in this block are audited normal and ship nothing.
//! * A day after a mid-week closure, on which CME publishes a `06:00 preopen`
//!   in place of the usual `07:45 paused` / `08:00 preopen` pair, has lost its
//!   prior-evening leg and nothing else. That is a **late open** at the day
//!   session's own 08:30 CT open, stated on the trade date: `08:30` is earlier
//!   than the trading day's normal 19:00 CT first open, so the cutoff lands on
//!   the trade date itself rather than the preceding local date (memo D7).
//! * The three days after Thanksgiving carry both — no prior-evening leg and a
//!   12:05 CT final close — and are the only `LateOpenAndEarlyClose` rows here.
//!
//! Every row is **T2**: CME's trading-hours service, the endpoint
//! `cmegroup.com/trading-hours.html` itself calls, read as bytes and saved.
//! CME publishes no T1 per-asset-class rendering of these instants; that gap,
//! the eight 2025 windows that survive only in a pre-holiday capture, the
//! order-entry deviations the scalar vocabulary cannot state, and the audited
//! weekend dates that ship no row are all recorded in
//! [`docs/evidence/globex_grains.md`](../../../../../docs/evidence/globex_grains.md).
//!
//! Scope is the standard-size CBOT grain and oilseed complex — corn, soybeans
//! and wheat. CME's service answers for `ZC`, `ZS` and `ZW` separately and the
//! three agree on every date in this window, so the family takes one table.
//! Mini grains are a separate key and a separate table.

use super::fences::{early_close, late_open, late_open_and_early_close};
use super::{EvidenceTier::T2, HolidayKind::Closed, HolidayTable, holidays};

/// 08:30 CT, the day session's own open and the late-open instant every
/// evening-leg-less trade date in this block falls back to.
const DAY_OPEN: u32 = 8 * 3_600 + 30 * 60;

/// 12:05 CT, the only early final close CME publishes for this family in this
/// window.
const HALF_DAY_CLOSE: u32 = 12 * 3_600 + 5 * 60;

/// The family's built-in holiday rows and the window they were audited over.
///
/// Coverage runs to 2027-12-31, the end of the operator's published future
/// (LAW-NO-FABRICATED-DATES permits encoding it ahead of its effective day);
/// CME's 2028-01-01 record sits outside the window and ships no row.
// Evidence: docs/evidence/globex_grains.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: (2025, 1, 1) ..= (2027, 12, 31),
    rows: [
        // 2025-01-01 - T2 - CME-SVC-2024-12-31 - New Year's Day, no events published.
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-02 - T2 - CME-SVC-2024-12-31 - no prior-evening leg; matching opens 08:30 CT.
        (2025, 1, 2, late_open(DAY_OPEN), T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 - T2 - CME-SVC-2025-01-19 - MLK Day, only a 19:00 CT open for trade date 01-21.
        (2025, 1, 20, Closed, T2, "CME-SVC-2025-01-19"),
        // 2025-02-17 - T2 - CME-SVC-2025-02-16 - Presidents' Day, only a 19:00 CT open for 02-18.
        (2025, 2, 17, Closed, T2, "CME-SVC-2025-02-16"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-17 - Good Friday, no events published.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 - T2 - CME-SVC-2025-05-25 - Memorial Day, only a 19:00 CT open for 05-27.
        (2025, 5, 26, Closed, T2, "CME-SVC-2025-05-25"),
        // 2025-06-19 - T2 - CME-SVC-2025-06-18 - Juneteenth, only a 19:00 CT open for 06-20.
        (2025, 6, 19, Closed, T2, "CME-SVC-2025-06-18"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-03 - Independence Day, no events published.
        (2025, 7, 4, Closed, T2, "CME-SVC-2025-07-03"),
        // 2025-09-01 - T2 - CME-SVC-2025-08-31 - Labor Day, only a 19:00 CT open for 09-02.
        (2025, 9, 1, Closed, T2, "CME-SVC-2025-08-31"),
        // 2025-11-27 - T2 - CME-SVC-2025-11-26-SAT - Thanksgiving, no events published.
        (2025, 11, 27, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-11-28 - T2 - CME-SVC-2025-11-26-SAT - day after Thanksgiving, 08:30-12:05 CT only.
        (
            2025,
            11,
            28,
            late_open_and_early_close(DAY_OPEN, HALF_DAY_CLOSE),
            T2,
            "CME-SVC-2025-11-26-SAT"
        ),
        // 2025-11-29 - T2 - CME-SVC-2025-11-26-SAT - Thanksgiving Saturday, no events published.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-12-24 - T2 - CME-SVC-2025-12-24 - Christmas Eve, 12:05 CT final close.
        (2025, 12, 24, early_close(HALF_DAY_CLOSE), T2, "CME-SVC-2025-12-24"),
        // 2025-12-25 - T2 - CME-SVC-2025-12-24 - Christmas Day, no events published.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-24"),
        // 2025-12-26 - T2 - CME-SVC-2025-12-24 - no prior-evening leg; matching opens 08:30 CT.
        (2025, 12, 26, late_open(DAY_OPEN), T2, "CME-SVC-2025-12-24"),
        // 2026-01-01 - T2 - CME-SVC-2025-12-31 - New Year's Day, no events published.
        (2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-01-02 - T2 - CME-SVC-2025-12-31 - no prior-evening leg; matching opens 08:30 CT.
        (2026, 1, 2, late_open(DAY_OPEN), T2, "CME-SVC-2025-12-31"),
        // 2026-01-19 - T2 - CME-SVC-2026-01-18 - MLK Day, only a 19:00 CT open for 01-20.
        (2026, 1, 19, Closed, T2, "CME-SVC-2026-01-18"),
        // 2026-02-16 - T2 - CME-SVC-2026-02-15 - Presidents' Day, only a 19:00 CT open for 02-17.
        (2026, 2, 16, Closed, T2, "CME-SVC-2026-02-15"),
        // 2026-04-03 - T2 - CME-SVC-2026-04-01 - Good Friday, no events published.
        (2026, 4, 3, Closed, T2, "CME-SVC-2026-04-01"),
        // 2026-05-25 - T2 - CME-SVC-2026-05-24 - Memorial Day, only a 19:00 CT open for 05-26.
        (2026, 5, 25, Closed, T2, "CME-SVC-2026-05-24"),
        // 2026-06-19 - T2 - CME-SVC-2026-06-18 - Juneteenth, no events published.
        (2026, 6, 19, Closed, T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - Independence Day observed, no events published.
        (2026, 7, 3, Closed, T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 - T2 - CME-SVC-2026-09-06 - Labor Day, only a 19:00 CT open for 09-08.
        (2026, 9, 7, Closed, T2, "CME-SVC-2026-09-06"),
        // 2026-11-26 - T2 - CME-SVC-2026-11-25 - Thanksgiving, no events published.
        (2026, 11, 26, Closed, T2, "CME-SVC-2026-11-25"),
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - day after Thanksgiving, 08:30-12:05 CT only.
        (
            2026,
            11,
            27,
            late_open_and_early_close(DAY_OPEN, HALF_DAY_CLOSE),
            T2,
            "CME-SVC-2026-11-25"
        ),
        // 2026-12-24 - T2 - CME-SVC-2026-12-24 - Christmas Eve, 12:05 CT final close.
        (2026, 12, 24, early_close(HALF_DAY_CLOSE), T2, "CME-SVC-2026-12-24"),
        // 2026-12-25 - T2 - CME-SVC-2026-12-24 - Christmas Day, no events published.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 - T2 - CME-SVC-2026-12-31 - New Year's Day, no events published.
        (2027, 1, 1, Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-01-18 - T2 - CME-SVC-2027-01-17 - MLK Day, only a 19:00 CT open for 01-19.
        (2027, 1, 18, Closed, T2, "CME-SVC-2027-01-17"),
        // 2027-02-15 - T2 - CME-SVC-2027-02-14 - Presidents' Day, only a 19:00 CT open for 02-16.
        (2027, 2, 15, Closed, T2, "CME-SVC-2027-02-14"),
        // 2027-03-26 - T2 - CME-SVC-2027-03-25 - Good Friday, no events published.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 - T2 - CME-SVC-2027-05-30 - Memorial Day, only a 19:00 CT open for 06-01.
        (2027, 5, 31, Closed, T2, "CME-SVC-2027-05-30"),
        // 2027-06-18 - T2 - CME-SVC-2027-06-17 - Juneteenth observed, no events published.
        (2027, 6, 18, Closed, T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 - T2 - CME-SVC-2027-07-04 - Independence Day observed, no events published.
        (2027, 7, 5, Closed, T2, "CME-SVC-2027-07-04"),
        // 2027-07-06 - T2 - CME-SVC-2027-07-04 - no prior-evening leg; matching opens 08:30 CT.
        (2027, 7, 6, late_open(DAY_OPEN), T2, "CME-SVC-2027-07-04"),
        // 2027-09-06 - T2 - CME-SVC-2027-09-05 - Labor Day, only a 19:00 CT open for 09-07.
        (2027, 9, 6, Closed, T2, "CME-SVC-2027-09-05"),
        // 2027-11-25 - T2 - CME-SVC-2027-11-24 - Thanksgiving, no events published.
        (2027, 11, 25, Closed, T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - day after Thanksgiving, 08:30-12:05 CT only.
        (
            2027,
            11,
            26,
            late_open_and_early_close(DAY_OPEN, HALF_DAY_CLOSE),
            T2,
            "CME-SVC-2027-11-24"
        ),
        // 2027-12-24 - T2 - CME-SVC-2027-12-22 - Christmas Friday closure, no events published.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
