// SPDX-License-Identifier: MIT-0

//! Saudi Exchange (Tadawul) Main Market holiday rows, 2021-2027.
//!
//! Keyed by the venue-local trade date in `Asia/Riyadh`. The Main Market
//! trades Sunday-Thursday, so an operator holiday that falls on a Friday or
//! Saturday changes no trade date and ships no row; every row below is a
//! Sunday-Thursday date the operator's own calendar removes from trading.
//!
//! The whole block is **T1**: the operator's own
//! `Saudi Exchange Holiday Calendar` page (Exchange Media Centre), whose
//! entries state the arrangement in session language — most Eid entries print
//! `Trading will discontinue at the end of trading day <date>. Trading will
//! resume after the holiday on <date>.`, the 2021-2022 Eid Al Fiter entries
//! print the holiday's first and last day, and each Founding Day / National
//! Day entry prints the single day observed. The Eid dates carry the
//! operator's own annotation `* According to the UMM AL-QURA calendar`: the
//! dates are read from the operator's printed calendar, never computed. The
//! 2025 Founding Day entry is the operator's own observance statement — Sunday
//! 23 February 2025, not Saturday the 22nd — so the row keys to the printed
//! day. All five years in the window were on the page at its
//! 2026-09-28 retrieval, so coverage reaches 2027-12-31; the page's oldest
//! complete entry is the 2020 Eid Al Fiter, whose year cannot stand alone
//! because the same page states no Eid Al Adha 2020, so coverage begins at
//! 2021-01-01 and 2013-06-29..2020-12-31 is an unaudited span with the
//! closing condition recorded in
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
    coverage: [(2021, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2021-05-13 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter: the
        // entry states `First day of Eid Al Fiter is 13/5/2021. Last day of
        // Eid Al Fiter is 16/5/2021.`, so Thursday 13/05 and Sunday 16/05 are
        // removed (14-15 May are the weekend).
        (2021, 5, 13, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2021-05-16 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter, Sunday.
        (2021, 5, 16, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2021-07-18 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha: trading
        // discontinue end of Thursday 15/07, resume Sunday 25/07; the 16th-17th
        // printed legs are the weekend and 18/07 is the first removed trade
        // date.
        (2021, 7, 18, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2021-07-19 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Monday.
        (2021, 7, 19, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2021-07-20 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Tuesday.
        (2021, 7, 20, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2021-07-21 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha,
        // Wednesday.
        (2021, 7, 21, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2021-07-22 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Thursday.
        (2021, 7, 22, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2021-09-23 - T1 - TADAWUL-HOLCAL-2026-09-28 - National Day:
        // `National Day of Saudi Arabia is on 23/9/2021`, a Thursday.
        (2021, 9, 23, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2022-02-22 - T1 - TADAWUL-HOLCAL-2026-09-28 - Founding Day:
        // `Founding Day of Saudi Arabia is on 22/02/2022`, a Tuesday.
        (2022, 2, 22, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2022-04-28 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter: the
        // entry states `First day of Eid Al Fiter is 28/4/2022. Last day of
        // Eid Al Fiter is 8/5/2022.`, so the Thursday 28/04 and the
        // Sunday-Thursday dates inside the range are removed.
        (2022, 4, 28, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2022-05-01 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter, Sunday.
        (2022, 5, 1, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2022-05-02 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter, Monday.
        (2022, 5, 2, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2022-05-03 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter, Tuesday.
        (2022, 5, 3, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2022-05-04 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter,
        // Wednesday.
        (2022, 5, 4, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2022-05-05 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter,
        // Thursday.
        (2022, 5, 5, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2022-07-07 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha: trading
        // discontinue end of Wednesday 06/07, resume Wednesday 13/07; Thursday
        // 07/07 is the first removed trade date.
        (2022, 7, 7, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2022-07-10 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Sunday
        // (the 8th-9th printed legs are the weekend).
        (2022, 7, 10, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2022-07-11 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Monday.
        (2022, 7, 11, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2022-07-12 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Tuesday.
        (2022, 7, 12, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2022-09-22 - T1 - TADAWUL-HOLCAL-2026-09-28 - National Day:
        // `National Day of Saudi Arabia is on 22/9/2022`, a Thursday.
        (2022, 9, 22, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2023-02-22 - T1 - TADAWUL-HOLCAL-2026-09-28 - Founding Day:
        // `Founding Day of Saudi Arabia is on 22/02/2023`, a Wednesday.
        (2023, 2, 22, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2023-04-18 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter: trading
        // discontinue end of Monday 17/04, resume Tuesday 25/04; the 21st-22nd
        // printed legs are the weekend.
        (2023, 4, 18, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2023-04-19 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter,
        // Wednesday.
        (2023, 4, 19, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2023-04-20 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter,
        // Thursday.
        (2023, 4, 20, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2023-04-23 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter, Sunday.
        (2023, 4, 23, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2023-04-24 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter, Monday.
        (2023, 4, 24, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2023-06-25 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha: trading
        // discontinue end of Thursday 22/06, resume Sunday 02/07; the 23rd-24th
        // printed legs are the weekend and 25/06 is the first removed trade
        // date.
        (2023, 6, 25, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2023-06-26 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Monday.
        (2023, 6, 26, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2023-06-27 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Tuesday.
        (2023, 6, 27, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2023-06-28 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha,
        // Wednesday.
        (2023, 6, 28, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2023-06-29 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Thursday.
        (2023, 6, 29, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2023-09-24 - T1 - TADAWUL-HOLCAL-2026-09-28 - National Day: trading
        // discontinue end of Thursday 21/09, resume Monday 25/09, so Sunday
        // 24/09 is the removed trade date (the 22nd-23rd are the weekend).
        (2023, 9, 24, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2024-02-22 - T1 - TADAWUL-HOLCAL-2026-09-28 - Founding Day:
        // `Founding Day of Saudi Arabia is on 22/02/2024`, a Thursday.
        (2024, 2, 22, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2024-04-07 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter: trading
        // discontinue end of Thursday 04/04, resume Sunday 14/04; the 12th-13th
        // printed legs are the weekend.
        (2024, 4, 7, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2024-04-08 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter, Monday.
        (2024, 4, 8, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2024-04-09 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter, Tuesday.
        (2024, 4, 9, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2024-04-10 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter,
        // Wednesday.
        (2024, 4, 10, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2024-04-11 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Fiter,
        // Thursday.
        (2024, 4, 11, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2024-06-16 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha: trading
        // discontinue end of Thursday 13/06, resume Sunday 23/06; the 14th-15th
        // and 21st-22nd printed legs are the weekends.
        (2024, 6, 16, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2024-06-17 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Monday.
        (2024, 6, 17, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2024-06-18 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Tuesday.
        (2024, 6, 18, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2024-06-19 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha,
        // Wednesday.
        (2024, 6, 19, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2024-06-20 - T1 - TADAWUL-HOLCAL-2026-09-28 - Eid Al Adha, Thursday.
        (2024, 6, 20, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
        // 2024-09-23 - T1 - TADAWUL-HOLCAL-2026-09-28 - National Day:
        // `National Day of Saudi Arabia is on 23/09/2024`, a Monday.
        (2024, 9, 23, Closed, T1, "TADAWUL-HOLCAL-2026-09-28"),
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
