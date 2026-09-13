//! The `Exchange::Cbot` table holiday rows — the `globex_grains` ∩ `globex_interest_rates`.
//!
//! Derived, not retrieved: the rows are the intersection of the families that
//! route to this venue, by the rule and routing recorded in
//! [`super`](index.html). A row ships only where every routed family states
//! it; a disagreement ships [`HolidayKind::Unsourced`], which clips nothing
//! and tells the caller the date is special without inventing an instant.
//!
//! Coverage runs from the January-2010 floor; 2010-2012 rows are T1 (CME's own
//! holiday-calendar PDFs), 2025-2027 rows are T2 (the trading-hours service).
//! The derivation, the instant disagreements and every dropped date are in the
//! venue's own evidence file, and the per-family rows are in the family files.
//!
//! **What changed on 2026-09-13:** this module was one file holding four tables.
//! It is now one file per venue, because the 2010-2012 rows took the combined
//! file past the 500-line reviewability guard. No row moved between venues.

use super::super::{
    EvidenceTier::{T1, T2},
    HolidayKind::Closed,
    HolidayKind::Unsourced,
    HolidayTable, holidays,
};

/// The `Exchange::Cbot` table: `globex_grains` ∩ `globex_interest_rates`.
///
/// The two families trade the same building around different sessions, and the
/// day session is where they touch: every one of the nine full closures below
/// keeps both closed, while a holiday early close moves the two by a different
/// amount — the grain day session ends at 12:05 CT or 12:00 CT while the rate
/// leg halts at 15:15, 12:00, 10:15 or 13:30 CT by date — so those dates ship
/// `Unsourced`. Seventy-one rows over two audited eras: nine stated and
/// sixty-two `Unsourced` (thirty-one in 2025-2027 and thirty-one in
/// 2010-2012), with the 2013-2024 interval between the eras outside every
/// declared window.
// Evidence: docs/evidence/cbot.md
pub(crate) static CBOT: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2012, 12, 31), (2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2010-01-15 - T1 - 2010-martin-luther-king.pdf - disagreement: grains no row, interest rates early close 15:15 CT.
        (2010, 1, 15, Unsourced, T1, "2010-martin-luther-king.pdf @2010-03-31T06:42:26Z"),
        // 2010-02-12 - T1 - 2010-presidents-day.pdf - disagreement: grains no row, interest rates early close 15:15 CT.
        (2010, 2, 12, Unsourced, T1, "2010-presidents-day.pdf @2010-02-15T06:46:41Z"),
        // 2010-04-02 - T1 - 2010-good-friday.pdf - disagreement: grains no row, interest rates early close 10:15 CT.
        (2010, 4, 2, Unsourced, T1, "2010-good-friday.pdf @2010-06-01T11:19:16Z"),
        // 2010-05-28 - T1 - 2010-memorial-day.pdf - disagreement: grains no row, interest rates early close 15:15 CT.
        (2010, 5, 28, Unsourced, T1, "2010-memorial-day.pdf @2010-06-01T09:42:25Z"),
        // 2010-07-02 - T1 - 2010-4th-of-july.pdf - disagreement: grains no row, interest rates early close 15:15 CT.
        (2010, 7, 2, Unsourced, T1, "2010-4th-of-july.pdf @2010-06-02T00:56:37Z"),
        // 2010-09-03 - T1 - 2010-labor-day.pdf - disagreement: grains no row, interest rates early close 15:15 CT.
        (2010, 9, 3, Unsourced, T1, "2010-labor-day.pdf @2010-06-02T00:56:41Z"),
        // 2010-10-08 - T1 - 2010-columbus-day.pdf - disagreement: grains no row, interest rates early close 15:15 CT.
        (2010, 10, 8, Unsourced, T1, "2010-columbus-day.pdf @2010-08-21T13:31:22Z"),
        // 2010-11-26 - T1 - 2010-thanksgiving.pdf - disagreement: grains early close 12:00 CT, interest rates early close 12:15 CT.
        (2010, 11, 26, Unsourced, T1, "2010-thanksgiving.pdf @2010-11-22T09:40:12Z"),
        // 2010-12-31 - T1 - 2011-new-years.pdf - disagreement: grains early close 12:00 CT, interest rates early close 12:15 CT.
        (2010, 12, 31, Unsourced, T1, "2011-new-years.pdf @2011-11-01T14:39:45Z"),
        // 2011-01-14 - T1 - 2011-martin-luther-king.pdf - disagreement: grains no row, interest rates early close 15:15 CT.
        (2011, 1, 14, Unsourced, T1, "2011-martin-luther-king.pdf @2011-10-28T02:34:29Z"),
        // 2011-02-18 - T1 - 2011-presidents-day.pdf - disagreement: grains no row, interest rates early close 15:15 CT.
        (2011, 2, 18, Unsourced, T1, "2011-presidents-day.pdf @2011-10-28T02:35:16Z"),
        // 2011-05-27 - T1 - 2011-memorial-day.pdf - disagreement: grains no row, interest rates early close 15:15 CT.
        (2011, 5, 27, Unsourced, T1, "2011-memorial-day.pdf @2013-09-30T10:56:52Z"),
        // 2011-07-01 - T1 - 2011-4th-of-july.pdf - disagreement: grains no row, interest rates early close 15:15 CT.
        (2011, 7, 1, Unsourced, T1, "2011-4th-of-july.pdf @2011-11-01T14:40:54Z"),
        // 2011-09-02 - T1 - 2011-labor-day.pdf - disagreement: grains no row, interest rates early close 15:15 CT.
        (2011, 9, 2, Unsourced, T1, "2011-labor-day.pdf @2011-11-01T14:43:45Z"),
        // 2011-10-07 - T1 - 2011-columbus-day.pdf - disagreement: grains no row, interest rates early close 15:15 CT.
        (2011, 10, 7, Unsourced, T1, "2011-columbus-day.pdf @2011-11-01T14:39:16Z"),
        // 2011-11-25 - T1 - 2011-thanksgiving.pdf - disagreement: grains early close 12:00 CT, interest rates early close 12:15 CT.
        (2011, 11, 25, Unsourced, T1, "2011-thanksgiving.pdf @2011-11-24T18:52:46Z"),
        // 2011-12-27 - T1 - 2011-christmas.pdf - disagreement: grains late open 09:30 CT, interest rates late open 05:00 CT.
        (2011, 12, 27, Unsourced, T1, "2011-christmas.pdf @2012-01-25T02:05:48Z"),
        // 2012-01-03 - T1 - 2012-new-years.pdf - disagreement: grains late open 09:30 CT, interest rates late open 05:00 CT.
        (2012, 1, 3, Unsourced, T1, "2012-new-years.pdf @2012-01-25T02:54:30Z"),
        // 2012-01-13 - T1 - 2012-martin-luther-king.pdf - disagreement: grains no row, interest rates early close 15:15 CT.
        (2012, 1, 13, Unsourced, T1, "2012-martin-luther-king.pdf @2012-05-05T16:15:26Z"),
        // 2012-02-17 - T1 - 2012-presidents-day.pdf - disagreement: grains no row, interest rates early close 15:15 CT.
        (2012, 2, 17, Unsourced, T1, "2012-presidents-day.pdf @2012-05-05T16:15:39Z"),
        // 2012-04-06 - T1 - 2012-good-friday.pdf - disagreement: grains no row, interest rates early close 10:15 CT.
        (2012, 4, 6, Unsourced, T1, "2012-good-friday.pdf @2012-05-05T16:16:49Z"),
        // 2012-05-25 - T1 - 2012-memorial-day.pdf - disagreement: grains no row, interest rates early close 15:15 CT.
        (2012, 5, 25, Unsourced, T1, "2012-memorial-day.pdf @2012-09-15T00:37:14Z"),
        // 2012-05-28 - T1 - 2012-memorial-day.pdf - disagreement: grains late open 19:00 CT, interest rates no row.
        (2012, 5, 28, Unsourced, T1, "2012-memorial-day.pdf @2012-09-15T00:37:14Z"),
        // 2012-07-03 - T1 - 2012-4th-of-july.pdf - disagreement: grains early close 12:00 CT, interest rates no row.
        (2012, 7, 3, Unsourced, T1, "2012-4th-of-july.pdf @2012-09-15T00:39:23Z"),
        // 2012-07-05 - T1 - 2012-4th-of-july.pdf - disagreement: grains late open 09:30 CT, interest rates no row.
        (2012, 7, 5, Unsourced, T1, "2012-4th-of-july.pdf @2012-09-15T00:39:23Z"),
        // 2012-08-31 - T1 - 2012-labor-day.pdf - disagreement: grains no row, interest rates early close 15:15 CT.
        (2012, 8, 31, Unsourced, T1, "2012-labor-day.pdf @2012-09-15T00:34:37Z"),
        // 2012-09-03 - T1 - 2012-labor-day.pdf - disagreement: grains late open 19:00 CT, interest rates no row.
        (2012, 9, 3, Unsourced, T1, "2012-labor-day.pdf @2012-09-15T00:34:37Z"),
        // 2012-10-05 - T1 - 2012-columbus-day.pdf - disagreement: grains no row, interest rates early close 15:15 CT.
        (2012, 10, 5, Unsourced, T1, "2012-columbus-day.pdf @2012-09-15T00:15:14Z"),
        // 2012-11-23 - T1 - 2012-thanksgiving.pdf - disagreement: grains late open 09:30 CT and early close 12:00 CT, interest rates early close 12:15 CT.
        (2012, 11, 23, Unsourced, T1, "2012-thanksgiving.pdf @2013-01-27T22:39:01Z"),
        // 2012-12-24 - T1 - 2012-christmas.pdf - disagreement: grains early close 12:00 CT, interest rates early close 12:15 CT.
        (2012, 12, 24, Unsourced, T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        // 2012-12-26 - T1 - 2012-christmas.pdf - disagreement: grains late open 09:30 CT, interest rates late open 05:00 CT.
        (2012, 12, 26, Unsourced, T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-02 - T2 - CME-SVC-2024-12-31 - grains late open 08:30 CT;
        // no row in interest rates.
        (2025, 1, 2, Unsourced, T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 - T2 - CME-SVC-2025-01-19 - grains closed, interest rates
        // early close 12:00 CT.
        (2025, 1, 20, Unsourced, T2, "CME-SVC-2025-01-19"),
        // 2025-02-17 - T2 - CME-SVC-2025-02-16 - as 2025-01-20.
        (2025, 2, 17, Unsourced, T2, "CME-SVC-2025-02-16"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-17 - both families closed.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 - T2 - CME-SVC-2025-05-25 - as 2025-01-20.
        (2025, 5, 26, Unsourced, T2, "CME-SVC-2025-05-25"),
        // 2025-06-19 - T2 - CME-SVC-2025-06-18 - as 2025-01-20.
        (2025, 6, 19, Unsourced, T2, "CME-SVC-2025-06-18"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-03 - as 2025-01-20.
        (2025, 7, 4, Unsourced, T2, "CME-SVC-2025-07-03"),
        // 2025-09-01 - T2 - CME-SVC-2025-08-31 - as 2025-01-20.
        (2025, 9, 1, Unsourced, T2, "CME-SVC-2025-08-31"),
        // 2025-11-27 - T2 - CME-SVC-2025-11-26-SAT - as 2025-01-20.
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
        (2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-01-02 - T2 - CME-SVC-2025-12-31 - grains late open 08:30 CT;
        // no row in interest rates.
        (2026, 1, 2, Unsourced, T2, "CME-SVC-2025-12-31"),
        // 2026-01-19 - T2 - CME-SVC-2026-01-18 - as 2025-01-20.
        (2026, 1, 19, Unsourced, T2, "CME-SVC-2026-01-18"),
        // 2026-02-16 - T2 - CME-SVC-2026-02-15 - as 2025-01-20.
        (2026, 2, 16, Unsourced, T2, "CME-SVC-2026-02-15"),
        // 2026-04-03 - T2 - CME-SVC-2026-04-01 - grains closed, interest rates
        // early close 10:15 CT.
        (2026, 4, 3, Unsourced, T2, "CME-SVC-2026-04-01"),
        // 2026-05-25 - T2 - CME-SVC-2026-05-24 - as 2025-01-20.
        (2026, 5, 25, Unsourced, T2, "CME-SVC-2026-05-24"),
        // 2026-06-19 - T2 - CME-SVC-2026-06-18 - as 2025-01-20.
        (2026, 6, 19, Unsourced, T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - as 2025-01-20.
        (2026, 7, 3, Unsourced, T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 - T2 - CME-SVC-2026-09-06 - as 2025-01-20.
        (2026, 9, 7, Unsourced, T2, "CME-SVC-2026-09-06"),
        // 2026-11-26 - T2 - CME-SVC-2026-11-25 - as 2025-01-20.
        (2026, 11, 26, Unsourced, T2, "CME-SVC-2026-11-25"),
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - as 2025-11-28.
        (2026, 11, 27, Unsourced, T2, "CME-SVC-2026-11-25"),
        // 2026-12-24 - T2 - CME-SVC-2026-12-22 - grains 12:05 CT, interest
        // rates 12:15 CT.
        (2026, 12, 24, Unsourced, T2, "CME-SVC-2026-12-22"),
        // 2026-12-25 - T2 - CME-SVC-2026-12-24 - both families closed.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 - T2 - CME-SVC-2026-12-31 - both families closed.
        (2027, 1, 1, Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-01-18 - T2 - CME-SVC-2027-01-17 - as 2025-01-20.
        (2027, 1, 18, Unsourced, T2, "CME-SVC-2027-01-17"),
        // 2027-02-15 - T2 - CME-SVC-2027-02-14 - as 2025-01-20.
        (2027, 2, 15, Unsourced, T2, "CME-SVC-2027-02-14"),
        // 2027-03-26 - T2 - CME-SVC-2027-03-25 - both families closed.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 - T2 - CME-SVC-2027-05-30 - as 2025-01-20.
        (2027, 5, 31, Unsourced, T2, "CME-SVC-2027-05-30"),
        // 2027-06-18 - T2 - CME-SVC-2027-06-17 - as 2025-01-20.
        (2027, 6, 18, Unsourced, T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 - T2 - CME-SVC-2027-07-04 - grains closed, interest rates
        // early close 13:30 CT.
        (2027, 7, 5, Unsourced, T2, "CME-SVC-2027-07-04"),
        // 2027-07-06 - T2 - CME-SVC-2027-07-04 - grains late open 08:30 CT;
        // no row in interest rates.
        (2027, 7, 6, Unsourced, T2, "CME-SVC-2027-07-04"),
        // 2027-09-06 - T2 - CME-SVC-2027-09-05 - as 2025-01-20.
        (2027, 9, 6, Unsourced, T2, "CME-SVC-2027-09-05"),
        // 2027-11-25 - T2 - CME-SVC-2027-11-24 - as 2025-01-20.
        (2027, 11, 25, Unsourced, T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - as 2025-11-28.
        (2027, 11, 26, Unsourced, T2, "CME-SVC-2027-11-24"),
        // 2027-12-24 - T2 - CME-SVC-2027-12-22 - both families closed.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
