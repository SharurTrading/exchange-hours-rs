//! The `Exchange::Cbot` table holiday rows — the `globex_grains` ∩ `globex_interest_rates`.
//!
//! Derived, not retrieved: the rows are the intersection of the families that
//! route to this venue, by the rule and routing recorded in
//! [`super`](index.html). A row ships only where every routed family states
//! it; a disagreement ships [`HolidayKind::Unsourced`], which clips nothing
//! and tells the caller the date is special without inventing an instant.
//!
//! Coverage is the single audited era **2025-2027**, whose rows are T2 — CME's
//! own `trading-hours-by-product` service — and whose window opens at the
//! crate's permanent 2025 support floor (LAW-COVERAGE). Seventy rows: nine
//! state a status — the Globex full closures — and sixty-one carry
//! [`HolidayKind::Unsourced`], each a date on which the two families state
//! different rows, or one states a row while the other has audited the date
//! normal. The per-date disagreements are named in the rows' comments and in
//! the venue's evidence file.
//!
//! The derivation, the instant disagreements and every dropped date are in the
//! venue's own evidence file, and the per-family rows are in the family files.

use super::super::{
    EvidenceTier::T2, HolidayKind::Closed, HolidayKind::Unsourced, HolidayTable, holidays,
};

/// The `Exchange::Cbot` table: `globex_grains` ∩ `globex_interest_rates`.
///
/// The two families trade the same building around different sessions, and the
/// day session is where they touch: every one of the nine full closures below
/// keeps both closed, while a holiday early close moves the two by a different
/// amount — the grain day session and the rate leg halt at different minutes —
/// so those dates ship `Unsourced`. 70 rows over the single audited window
/// `2025-01-01..2027-12-31`: nine stated and 61 `Unsourced`. The window is
/// declared as the coverage clause, and outside it the table reports no answer
/// rather than a normal one.
// Evidence: docs/evidence/cbot.md
pub(crate) static CBOT: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2025-01-01 - T2 - CME-SVC-2024-12-31 - closed: no trade date.
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-02 - T2 - CME-SVC-2024-12-31 - grains late open 08:30 CT;
        // no row in interest rates.
        (2025, 1, 2, Unsourced, T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 - T2 - CME-SVC-2025-01-19 - grains closed, interest rates
        // early close 12:00 CT.
        (2025, 1, 20, Unsourced, T2, "CME-SVC-2025-01-19"),
        // 2025-01-21 - T2 - CME-SVC-2025-01-19 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2025, 1, 21, Unsourced, T2, "CME-SVC-2025-01-19"),
        // 2025-02-17 - T2 - CME-SVC-2025-02-16 - as 2025-01-20.
        (2025, 2, 17, Unsourced, T2, "CME-SVC-2025-02-16"),
        // 2025-02-18 - T2 - CME-SVC-2025-02-16 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2025, 2, 18, Unsourced, T2, "CME-SVC-2025-02-16"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-17 - both families closed.
        // 2025-04-17 - T2 - CME-SVC-2025-04-17 - grains states the closure eve's replacement row; interest rates audited the date normal, so the two routed families disagree.
        (2025, 4, 17, Unsourced, T2, "CME-SVC-2025-04-17"),
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 - T2 - CME-SVC-2025-05-25 - as 2025-01-20.
        (2025, 5, 26, Unsourced, T2, "CME-SVC-2025-05-25"),
        // 2025-05-27 - T2 - CME-SVC-2025-05-25 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2025, 5, 27, Unsourced, T2, "CME-SVC-2025-05-25"),
        // 2025-06-19 - T2 - CME-SVC-2025-06-18 - as 2025-01-20.
        // 2025-06-18 - T2 - CME-SVC-2025-06-18 - grains states the closure eve's replacement row; interest rates audited the date normal, so the two routed families disagree.
        (2025, 6, 18, Unsourced, T2, "CME-SVC-2025-06-18"),
        (2025, 6, 19, Unsourced, T2, "CME-SVC-2025-06-18"),
        // 2025-06-20 - T2 - CME-SVC-2025-06-18 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2025, 6, 20, Unsourced, T2, "CME-SVC-2025-06-18"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-03 - as 2025-01-20.
        // 2025-07-03 - T2 - CME-SVC-2025-07-03 - grains states the closure eve's replacement row; interest rates audited the date normal, so the two routed families disagree.
        (2025, 7, 3, Unsourced, T2, "CME-SVC-2025-07-03"),
        (2025, 7, 4, Unsourced, T2, "CME-SVC-2025-07-03"),
        // 2025-09-01 - T2 - CME-SVC-2025-08-31 - as 2025-01-20.
        (2025, 9, 1, Unsourced, T2, "CME-SVC-2025-08-31"),
        // 2025-09-02 - T2 - CME-SVC-2025-08-31 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2025, 9, 2, Unsourced, T2, "CME-SVC-2025-08-31"),
        // 2025-11-27 - T2 - CME-SVC-2025-11-26-SAT - as 2025-01-20.
        // 2025-11-26 - T2 - CME-SVC-2025-11-26-SAT - grains states the closure eve's replacement row; interest rates audited the date normal, so the two routed families disagree.
        (2025, 11, 26, Unsourced, T2, "CME-SVC-2025-11-26-SAT"),
        (2025, 11, 27, Unsourced, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-11-28 - T2 - CME-SVC-2025-11-26 - grains 08:30 CT late open
        // into a 12:05 CT close, interest rates 12:15 CT early close.
        (2025, 11, 28, Unsourced, T2, "CME-SVC-2025-11-26"),
        // 2025-11-29 - T2 - CME-SVC-2025-11-26-SAT - both families closed.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-12-24 - T2 - CME-SVC-2025-12-24 - grains 12:05 CT, interest
        // rates 12:15 CT.
        (2025, 12, 24, Unsourced, T2, "CME-SVC-2025-12-24"),
        // 2025-12-25 - T2 - CME-SVC-2025-12-24 - both families closed.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-24"),
        // 2025-12-26 - T2 - CME-SVC-2025-12-24 - grains late open 08:30 CT;
        // no row in interest rates.
        (2025, 12, 26, Unsourced, T2, "CME-SVC-2025-12-24"),
        // 2026-01-01 - T2 - CME-SVC-2025-12-31 - both families closed.
        // 2025-12-31 - T2 - CME-SVC-2025-12-31 - grains states the closure eve's replacement row; interest rates audited the date normal, so the two routed families disagree.
        (2025, 12, 31, Unsourced, T2, "CME-SVC-2025-12-31"),
        (2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-01-02 - T2 - CME-SVC-2025-12-31 - grains late open 08:30 CT;
        // no row in interest rates.
        (2026, 1, 2, Unsourced, T2, "CME-SVC-2025-12-31"),
        // 2026-01-19 - T2 - CME-SVC-2026-01-18 - as 2025-01-20.
        (2026, 1, 19, Unsourced, T2, "CME-SVC-2026-01-18"),
        // 2026-01-20 - T2 - CME-SVC-2026-01-18 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2026, 1, 20, Unsourced, T2, "CME-SVC-2026-01-18"),
        // 2026-02-16 - T2 - CME-SVC-2026-02-15 - as 2025-01-20.
        (2026, 2, 16, Unsourced, T2, "CME-SVC-2026-02-15"),
        // 2026-02-17 - T2 - CME-SVC-2026-02-15 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2026, 2, 17, Unsourced, T2, "CME-SVC-2026-02-15"),
        // 2026-04-03 - T2 - CME-SVC-2026-04-01 - grains closed, interest rates
        // early close 10:15 CT.
        // 2026-04-02 - T2 - CME-SVC-2026-04-01 - grains states the closure eve's replacement row; interest rates audited the date normal, so the two routed families disagree.
        (2026, 4, 2, Unsourced, T2, "CME-SVC-2026-04-01"),
        (2026, 4, 3, Unsourced, T2, "CME-SVC-2026-04-01"),
        // 2026-05-25 - T2 - CME-SVC-2026-05-24 - as 2025-01-20.
        (2026, 5, 25, Unsourced, T2, "CME-SVC-2026-05-24"),
        // 2026-05-26 - T2 - CME-SVC-2026-05-24 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2026, 5, 26, Unsourced, T2, "CME-SVC-2026-05-24"),
        // 2026-06-19 - T2 - CME-SVC-2026-06-18 - as 2025-01-20.
        // 2026-06-18 - T2 - CME-SVC-2026-06-18 - grains states the closure eve's replacement row; interest rates audited the date normal, so the two routed families disagree.
        (2026, 6, 18, Unsourced, T2, "CME-SVC-2026-06-18"),
        (2026, 6, 19, Unsourced, T2, "CME-SVC-2026-06-18"),
        // 2026, 6, 22 - T2 - CME-SVC-2026-06-18 - interest rates states a Saturday-session replacement and grains states nothing, so the two routed families disagree.
        (2026, 6, 22, Unsourced, T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - as 2025-01-20.
        // 2026-07-02 - T2 - CME-SVC-2026-07-02 - grains states the closure eve's replacement row; interest rates audited the date normal, so the two routed families disagree.
        (2026, 7, 2, Unsourced, T2, "CME-SVC-2026-07-02"),
        (2026, 7, 3, Unsourced, T2, "CME-SVC-2026-07-03"),
        // 2026, 7, 6 - T2 - CME-SVC-2026-07-03 - interest rates states a Saturday-session replacement and grains states nothing, so the two routed families disagree.
        (2026, 7, 6, Unsourced, T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 - T2 - CME-SVC-2026-09-06 - as 2025-01-20.
        (2026, 9, 7, Unsourced, T2, "CME-SVC-2026-09-06"),
        // 2026-09-08 - T2 - CME-SVC-2026-09-06 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2026, 9, 8, Unsourced, T2, "CME-SVC-2026-09-06"),
        // 2026-11-26 - T2 - CME-SVC-2026-11-25 - as 2025-01-20.
        // 2026-11-25 - T2 - CME-SVC-2026-11-25 - grains states the closure eve's replacement row; interest rates audited the date normal, so the two routed families disagree.
        (2026, 11, 25, Unsourced, T2, "CME-SVC-2026-11-25"),
        (2026, 11, 26, Unsourced, T2, "CME-SVC-2026-11-25"),
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - as 2025-11-28.
        (2026, 11, 27, Unsourced, T2, "CME-SVC-2026-11-25"),
        // 2026-12-24 - T2 - CME-SVC-2026-12-22 - grains 12:05 CT, interest
        // rates 12:15 CT.
        (2026, 12, 24, Unsourced, T2, "CME-SVC-2026-12-22"),
        // 2026-12-25 - T2 - CME-SVC-2026-12-24 - both families closed.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 - T2 - CME-SVC-2026-12-31 - both families closed.
        // 2026-12-31 - T2 - CME-SVC-2026-12-31 - grains states the closure eve's replacement row; interest rates audited the date normal, so the two routed families disagree.
        (2026, 12, 31, Unsourced, T2, "CME-SVC-2026-12-31"),
        (2027, 1, 1, Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-01-18 - T2 - CME-SVC-2027-01-17 - as 2025-01-20.
        (2027, 1, 18, Unsourced, T2, "CME-SVC-2027-01-17"),
        // 2027-01-19 - T2 - CME-SVC-2027-01-17 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2027, 1, 19, Unsourced, T2, "CME-SVC-2027-01-17"),
        // 2027-02-15 - T2 - CME-SVC-2027-02-14 - as 2025-01-20.
        (2027, 2, 15, Unsourced, T2, "CME-SVC-2027-02-14"),
        // 2027-02-16 - T2 - CME-SVC-2027-02-14 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2027, 2, 16, Unsourced, T2, "CME-SVC-2027-02-14"),
        // 2027-03-26 - T2 - CME-SVC-2027-03-25 - both families closed.
        // 2027-03-25 - T2 - CME-SVC-2027-03-25 - grains states the closure eve's replacement row; interest rates audited the date normal, so the two routed families disagree.
        (2027, 3, 25, Unsourced, T2, "CME-SVC-2027-03-25"),
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 - T2 - CME-SVC-2027-05-30 - as 2025-01-20.
        (2027, 5, 31, Unsourced, T2, "CME-SVC-2027-05-30"),
        // 2027-06-01 - T2 - CME-SVC-2027-05-30 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2027, 6, 1, Unsourced, T2, "CME-SVC-2027-05-30"),
        // 2027-06-18 - T2 - CME-SVC-2027-06-17 - as 2025-01-20.
        // 2027-06-17 - T2 - CME-SVC-2027-06-17 - grains states the closure eve's replacement row; interest rates audited the date normal, so the two routed families disagree.
        (2027, 6, 17, Unsourced, T2, "CME-SVC-2027-06-17"),
        (2027, 6, 18, Unsourced, T2, "CME-SVC-2027-06-17"),
        // 2027, 6, 21 - T2 - CME-SVC-2027-06-17 - interest rates states a Saturday-session replacement and grains states nothing, so the two routed families disagree.
        (2027, 6, 21, Unsourced, T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 - T2 - CME-SVC-2027-07-04 - grains closed, interest rates
        // early close 13:30 CT.
        (2027, 7, 5, Unsourced, T2, "CME-SVC-2027-07-04"),
        // 2027-07-06 - T2 - CME-SVC-2027-07-04 - grains late open 08:30 CT;
        // no row in interest rates.
        (2027, 7, 6, Unsourced, T2, "CME-SVC-2027-07-04"),
        // 2027-09-06 - T2 - CME-SVC-2027-09-05 - as 2025-01-20.
        (2027, 9, 6, Unsourced, T2, "CME-SVC-2027-09-05"),
        // 2027-09-07 - T2 - CME-SVC-2027-09-05 - interest rates states the merged trade date; the other routed families state no row, so the venue cannot support either answer.
        (2027, 9, 7, Unsourced, T2, "CME-SVC-2027-09-05"),
        // 2027-11-25 - T2 - CME-SVC-2027-11-24 - as 2025-01-20.
        // 2027-11-24 - T2 - CME-SVC-2027-11-24 - grains states the closure eve's replacement row; interest rates audited the date normal, so the two routed families disagree.
        (2027, 11, 24, Unsourced, T2, "CME-SVC-2027-11-24"),
        (2027, 11, 25, Unsourced, T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - as 2025-11-28.
        (2027, 11, 26, Unsourced, T2, "CME-SVC-2027-11-24"),
        // 2027-12-24 - T2 - CME-SVC-2027-12-22 - both families closed.
        // 2027-12-23 - T2 - CME-SVC-2027-12-22 - grains states the closure eve's replacement row; interest rates audited the date normal, so the two routed families disagree.
        (2027, 12, 23, Unsourced, T2, "CME-SVC-2027-12-22"),
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
