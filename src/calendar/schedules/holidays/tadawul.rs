// SPDX-License-Identifier: MIT-0

//! Saudi Exchange (Tadawul) Main Market holiday rows, 2025-2027.
//!
//! Keyed by the venue-local trade date in `Asia/Riyadh`. The Main Market
//! trades Sunday-Thursday, so an operator holiday that falls on a Friday or
//! Saturday changes no trade date and ships no row; every row below is a
//! Sunday-Thursday date the operator's own calendar removes from trading.
//!
//! The whole block is **T1**: the operator's own
//! `Saudi Exchange Holiday Calendar` page (Exchange Media Centre), whose
//! entries state the arrangement in session language — each Eid entry prints
//! `Trading will discontinue at the end of trading day <date>. Trading will
//! resume after the holiday on <date>.` and each Founding Day / National Day
//! entry prints the single day observed. The Eid dates carry the operator's
//! own annotation `* According to the UMM AL-QURA calendar`: the dates are
//! read from the operator's printed calendar, never computed. The 2025
//! Founding Day entry is the operator's own observance statement — Sunday
//! 23 February 2025, not Saturday the 22nd — so the row keys to the printed
//! day. All three years in the window were on the page at its
//! 2026-09-28 retrieval, so coverage reaches 2027-12-31; the per-row
//! derivation is recorded in
//! [`docs/evidence/tadawul.md`](../../../../../docs/evidence/tadawul.md).

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::{HolidayTable, holidays};

/// The Saudi Exchange Main Market's built-in holiday rows and the window they
/// were audited over.
///
/// Every row is one entry of the operator's holiday calendar page. A date
/// inside the window with no row is audited normal.
// Evidence: docs/evidence/tadawul.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2025-02-23 - T1 - TADAWUL-HOLCAL-2026-09-28 - Founding Day: the
        // entry states `Founding Day of Saudi Arabia is on 23/02/2025`, the
        // Sunday the exchange observed it (the 22nd was a Saturday).
        (2025, 2, 23, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2025-03-30 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter: trading
        // discontinue end of Thursday 27/03, resume 03/04; Sunday 30/03 is
        // the first removed trade date (27-28 March close the week and the
        // 28th-29th are the weekend).
        (2025, 3, 30, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2025-03-31 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter, Monday.
        (2025, 3, 31, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2025-04-01 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter, Tuesday.
        (2025, 4, 1, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2025-04-02 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter,
        // Wednesday; trading resumes Thursday 03/04.
        (2025, 4, 2, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2025-06-05 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha: trading
        // discontinue end of Wednesday 04/06, resume 11/06; Thursday 05/06 is
        // removed.
        (2025, 6, 5, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2025-06-08 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Sunday
        // (the 6th-7th printed legs are the weekend).
        (2025, 6, 8, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2025-06-09 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Monday.
        (2025, 6, 9, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2025-06-10 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Tuesday;
        // trading resumes Wednesday 11/06.
        (2025, 6, 10, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2025-09-23 - T1 - TADAWUL-HOLCAL-2026-09-28 - National Day, Tuesday.
        (2025, 9, 23, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2026-02-22 - T1 - TADAWUL-HOLCAL-2026-09-28 - Founding Day, Sunday.
        (2026, 2, 22, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2026-03-17 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter: trading
        // discontinue end of Monday 16/03, resume 24/03; Tuesday 17/03 is the
        // first removed trade date.
        (2026, 3, 17, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2026-03-18 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter,
        // Wednesday.
        (2026, 3, 18, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2026-03-19 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter,
        // Thursday.
        (2026, 3, 19, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2026-03-22 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter, Sunday
        // (the 20th-21st printed legs are the weekend).
        (2026, 3, 22, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2026-03-23 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter, Monday;
        // trading resumes Tuesday 24/03.
        (2026, 3, 23, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2026-05-24 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha: trading
        // discontinue end of Thursday 21/05, resume Sunday 31/05; the printed
        // range opens Sunday 24/05.
        (2026, 5, 24, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2026-05-25 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Monday.
        (2026, 5, 25, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2026-05-26 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Tuesday.
        (2026, 5, 26, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2026-05-27 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha,
        // Wednesday.
        (2026, 5, 27, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2026-05-28 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Thursday;
        // trading resumes Sunday 31/05.
        (2026, 5, 28, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2026-09-23 - T1 - TADAWUL-HOLCAL-2026-09-28 - National Day,
        // Wednesday.
        (2026, 9, 23, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2027-02-22 - T1 - TADAWUL-HOLCAL-2026-09-28 - Founding Day, Monday.
        (2027, 2, 22, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2027-03-07 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter: trading
        // discontinue end of Thursday 04/03, resume Sunday 14/03; the printed
        // range opens Sunday 07/03.
        (2027, 3, 7, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2027-03-08 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter, Monday.
        (2027, 3, 8, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2027-03-09 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter, Tuesday.
        (2027, 3, 9, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2027-03-10 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter,
        // Wednesday.
        (2027, 3, 10, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2027-03-11 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter,
        // Thursday; trading resumes Sunday 14/03.
        (2027, 3, 11, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2027-05-16 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha: trading
        // discontinue end of Thursday 13/05, resume Sunday 23/05; the printed
        // range opens Sunday 16/05.
        (2027, 5, 16, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2027-05-17 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Monday.
        (2027, 5, 17, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2027-05-18 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Tuesday.
        (2027, 5, 18, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2027-05-19 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha,
        // Wednesday.
        (2027, 5, 19, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2027-05-20 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Thursday;
        // trading resumes Sunday 23/05.
        (2027, 5, 20, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2027-09-23 - T1 - TADAWUL-HOLCAL-2026-09-28 - National Day,
        // Thursday.
        (2027, 9, 23, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
    ],
};
