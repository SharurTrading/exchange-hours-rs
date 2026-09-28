// SPDX-License-Identifier: MIT-0

//! National Stock Exchange of India holiday rows, 2025-2026.
//!
//! Keyed by the crate's own venue-local trade date in `Asia/Kolkata` (design
//! memo D1). NSE runs no overnight wrap and no weekend sessions, so a closed
//! trade date is one whose daytime sessions are absent outright.
//!
//! The whole block is **T1**: the operator's own published trading-holiday
//! lists — the "NSE Holiday List 2025" banner and the "Trading Holiday List
//! for Equity & Equity Derivatives, Calendar Year 2026" banner NSE serves from
//! its own `nsearchives.nseindia.com` asset host — read from the Wayback
//! `id_` replays the live site's bot wall makes necessary (the live channel
//! refused both the HTML pages and the `holiday-master` API on 2026-09-28).
//! The lists are the capital-market trading holidays, which is the envelope
//! the `nse_india` identity models; the derivation and per-row quotations are
//! recorded in
//! [`docs/evidence/nse_india.md`](../../../../../docs/evidence/nse_india.md).
//!
//! Two rows are **`Unsourced` rather than closures**, both for the same
//! reason: each banner also announces a Muhurat Trading special session whose
//! instants the operator had not published when the banner was captured
//! ("Timings of Muhurat Trading shall be notified ..."). 2025-10-21 is the
//! Diwali Laxmi Pujan holiday and the Muhurat day at once, so a `Closed` row
//! would deny a session the operator states exists there; 2026-11-08 is a
//! Sunday the normal week closes but on which the operator announces a
//! session, so silence would claim an audited-normal weekend. `Unsourced`
//! withholds the whole day in both cases and the evidence file records what
//! would close the gap.
//!
//! NSE publishes the next year's list each December; no 2027 list exists as of
//! the 2026-09-28 retrieval, so the window ends 2026-12-31.

use super::EvidenceTier::T1;
use super::HolidayKind::{Closed, Unsourced};
use super::{HolidayTable, holidays};

/// NSE's built-in holiday rows and the window they were audited over.
///
/// Every `Closed` row is one printed date of the operator's own holiday list:
/// `NSE-HOL-2025` for 2025 (banner captured 2024-12-23) and `NSE-HOL-2026`
/// for 2026 (banner captured 2026-01-08). A date inside the window with no
/// row is audited normal; the two `Unsourced` dates are not.
// Evidence: docs/evidence/nse_india.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2026, 12, 31)],
    rows: [
        // 2025-02-26 - T1 - NSE-HOL-2025 - Mahashivratri, Wednesday.
        (2025, 2, 26, Closed, T1, "NSE-HOL-2025"),
        // 2025-03-14 - T1 - NSE-HOL-2025 - Holi, Friday.
        (2025, 3, 14, Closed, T1, "NSE-HOL-2025"),
        // 2025-03-31 - T1 - NSE-HOL-2025 - Eid-ul-Fitr (Ramadan Eid), Monday.
        (2025, 3, 31, Closed, T1, "NSE-HOL-2025"),
        // 2025-04-10 - T1 - NSE-HOL-2025 - Shri Mahavir Jayanti, Thursday.
        (2025, 4, 10, Closed, T1, "NSE-HOL-2025"),
        // 2025-04-14 - T1 - NSE-HOL-2025 - Dr. Babasaheb Ambedkar Jayanti, Monday.
        (2025, 4, 14, Closed, T1, "NSE-HOL-2025"),
        // 2025-04-18 - T1 - NSE-HOL-2025 - Good Friday.
        (2025, 4, 18, Closed, T1, "NSE-HOL-2025"),
        // 2025-05-01 - T1 - NSE-HOL-2025 - Maharashtra Day, Thursday.
        (2025, 5, 1, Closed, T1, "NSE-HOL-2025"),
        // 2025-08-15 - T1 - NSE-HOL-2025 - Independence Day, Friday.
        (2025, 8, 15, Closed, T1, "NSE-HOL-2025"),
        // 2025-08-27 - T1 - NSE-HOL-2025 - Ganesh Chaturthi, Wednesday.
        (2025, 8, 27, Closed, T1, "NSE-HOL-2025"),
        // 2025-10-02 - T1 - NSE-HOL-2025 - Mahatma Gandhi Jayanti / Dussehra,
        // Thursday.
        (2025, 10, 2, Closed, T1, "NSE-HOL-2025"),
        // 2025-10-21 - T1 - NSE-HOL-2025 - Diwali Laxmi Pujan: the list prints the
        // closure and footnotes Muhurat Trading onto the same day with timings
        // "shall be notified in due course", so no complete statement of the day
        // exists and the date is withheld rather than closed.
        (2025, 10, 21, Unsourced, T1, "NSE-HOL-2025"),
        // 2025-10-22 - T1 - NSE-HOL-2025 - Diwali Balipratipada, Wednesday.
        (2025, 10, 22, Closed, T1, "NSE-HOL-2025"),
        // 2025-11-05 - T1 - NSE-HOL-2025 - Prakash Gurpurab Sri Guru Nanak Dev,
        // Wednesday.
        (2025, 11, 5, Closed, T1, "NSE-HOL-2025"),
        // 2025-12-25 - T1 - NSE-HOL-2025 - Christmas, Thursday.
        (2025, 12, 25, Closed, T1, "NSE-HOL-2025"),
        // 2026-01-26 - T1 - NSE-HOL-2026 - Republic Day, Monday.
        (2026, 1, 26, Closed, T1, "NSE-HOL-2026"),
        // 2026-03-03 - T1 - NSE-HOL-2026 - Holi, Tuesday.
        (2026, 3, 3, Closed, T1, "NSE-HOL-2026"),
        // 2026-03-26 - T1 - NSE-HOL-2026 - Shri Ram Navami, Thursday.
        (2026, 3, 26, Closed, T1, "NSE-HOL-2026"),
        // 2026-03-31 - T1 - NSE-HOL-2026 - Shri Mahavir Jayanti, Tuesday.
        (2026, 3, 31, Closed, T1, "NSE-HOL-2026"),
        // 2026-04-03 - T1 - NSE-HOL-2026 - Good Friday.
        (2026, 4, 3, Closed, T1, "NSE-HOL-2026"),
        // 2026-04-14 - T1 - NSE-HOL-2026 - Dr. Baba Saheb Ambedkar Jayanti, Tuesday.
        (2026, 4, 14, Closed, T1, "NSE-HOL-2026"),
        // 2026-05-01 - T1 - NSE-HOL-2026 - Maharashtra Day, Friday.
        (2026, 5, 1, Closed, T1, "NSE-HOL-2026"),
        // 2026-05-28 - T1 - NSE-HOL-2026 - Bakri Id, Thursday.
        (2026, 5, 28, Closed, T1, "NSE-HOL-2026"),
        // 2026-06-26 - T1 - NSE-HOL-2026 - Muharram, Friday.
        (2026, 6, 26, Closed, T1, "NSE-HOL-2026"),
        // 2026-09-14 - T1 - NSE-HOL-2026 - Ganesh Chaturthi, Monday.
        (2026, 9, 14, Closed, T1, "NSE-HOL-2026"),
        // 2026-10-02 - T1 - NSE-HOL-2026 - Mahatma Gandhi Jayanti, Friday.
        (2026, 10, 2, Closed, T1, "NSE-HOL-2026"),
        // 2026-10-20 - T1 - NSE-HOL-2026 - Dussehra, Tuesday.
        (2026, 10, 20, Closed, T1, "NSE-HOL-2026"),
        // 2026-11-08 - T1 - NSE-HOL-2026 - Muhurat Trading: the list footnotes a
        // special session onto this Sunday with timings "shall be notified
        // subsequently", so the day is not the audited-normal weekend closure the
        // normal week alone would claim and is withheld.
        (2026, 11, 8, Unsourced, T1, "NSE-HOL-2026"),
        // 2026-11-10 - T1 - NSE-HOL-2026 - Diwali-Balipratipada, Tuesday.
        (2026, 11, 10, Closed, T1, "NSE-HOL-2026"),
        // 2026-11-24 - T1 - NSE-HOL-2026 - Prakash Gurpurab Sri Guru Nanak Dev,
        // Tuesday.
        (2026, 11, 24, Closed, T1, "NSE-HOL-2026"),
        // 2026-12-25 - T1 - NSE-HOL-2026 - Christmas, Friday.
        (2026, 12, 25, Closed, T1, "NSE-HOL-2026"),
    ],
};
