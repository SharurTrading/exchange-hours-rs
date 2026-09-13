//! The `Exchange::Nymex` table holiday rows — the the NYMEX energy half of `globex_energy`.
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
    HolidayTable,
    fences::early_close,
    holidays,
};

/// The `Exchange::Nymex` table: the NYMEX energy half of `globex_energy`.
///
/// The same rows as [`COMEX`], because the two venues route the same single
/// family: the operator publishes the metals and energy halves as one product
/// row on every date in this window. They stay separate tables rather than one
/// shared binding, matching the one-arm-per-identity rule the routing match
/// states — a venue's table is a decision about that venue, not an alias — and
/// `the_energy_venues_carry_the_family_table_unchanged` holds each of them
/// against the family's own answers.
// Evidence: docs/evidence/nymex.md
pub(crate) static NYMEX: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2012, 12, 31), (2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2010-01-15 - T1 - 2010-martin-luther-king.pdf - early close 15:15 CT.
        (2010, 1, 15, early_close(54900), T1, "2010-martin-luther-king.pdf @2010-03-31T06:42:26Z"),
        // 2010-01-18 - T1 - 2010-martin-luther-king.pdf - early close 12:15 CT.
        (2010, 1, 18, early_close(44100), T1, "2010-martin-luther-king.pdf @2010-03-31T06:42:26Z"),
        // 2010-02-12 - T1 - 2010-presidents-day.pdf - early close 15:15 CT.
        (2010, 2, 12, early_close(54900), T1, "2010-presidents-day.pdf @2010-02-15T06:46:41Z"),
        // 2010-02-15 - T1 - 2010-presidents-day.pdf - early close 12:15 CT.
        (2010, 2, 15, early_close(44100), T1, "2010-presidents-day.pdf @2010-02-15T06:46:41Z"),
        // 2010-05-28 - T1 - 2010-memorial-day.pdf - early close 15:15 CT.
        (2010, 5, 28, early_close(54900), T1, "2010-memorial-day.pdf @2010-06-01T09:42:25Z"),
        // 2010-05-31 - T1 - 2010-memorial-day.pdf - early close 12:15 CT.
        (2010, 5, 31, early_close(44100), T1, "2010-memorial-day.pdf @2010-06-01T09:42:25Z"),
        // 2010-07-02 - T1 - 2010-4th-of-july.pdf - early close 15:15 CT.
        (2010, 7, 2, early_close(54900), T1, "2010-4th-of-july.pdf @2010-06-02T00:56:37Z"),
        // 2010-07-05 - T1 - 2010-4th-of-july.pdf - early close 12:15 CT.
        (2010, 7, 5, early_close(44100), T1, "2010-4th-of-july.pdf @2010-06-02T00:56:37Z"),
        // 2010-09-03 - T1 - 2010-labor-day.pdf - early close 15:15 CT.
        (2010, 9, 3, early_close(54900), T1, "2010-labor-day.pdf @2010-06-02T00:56:41Z"),
        // 2010-09-06 - T1 - 2010-labor-day.pdf - early close 12:15 CT.
        (2010, 9, 6, early_close(44100), T1, "2010-labor-day.pdf @2010-06-02T00:56:41Z"),
        // 2010-10-08 - T1 - 2010-columbus-day.pdf - early close 15:15 CT.
        (2010, 10, 8, early_close(54900), T1, "2010-columbus-day.pdf @2010-08-21T13:31:22Z"),
        // 2010-11-25 - T1 - 2010-thanksgiving.pdf - early close 12:15 CT.
        (2010, 11, 25, early_close(44100), T1, "2010-thanksgiving.pdf @2010-11-22T09:40:12Z"),
        // 2010-11-26 - T1 - 2010-thanksgiving.pdf - early close 12:45 CT.
        (2010, 11, 26, early_close(45900), T1, "2010-thanksgiving.pdf @2010-11-22T09:40:12Z"),
        // 2010-12-31 - T1 - 2011-new-years.pdf - early close 15:15 CT.
        (2010, 12, 31, early_close(54900), T1, "2011-new-years.pdf @2011-11-01T14:39:45Z"),
        // 2011-01-14 - T1 - 2011-martin-luther-king.pdf - early close 15:15 CT.
        (2011, 1, 14, early_close(54900), T1, "2011-martin-luther-king.pdf @2011-10-28T02:34:29Z"),
        // 2011-01-17 - T1 - 2011-martin-luther-king.pdf - early close 12:15 CT.
        (2011, 1, 17, early_close(44100), T1, "2011-martin-luther-king.pdf @2011-10-28T02:34:29Z"),
        // 2011-02-21 - T1 - 2011-presidents-day.pdf - early close 12:15 CT.
        (2011, 2, 21, early_close(44100), T1, "2011-presidents-day.pdf @2011-10-28T02:35:16Z"),
        // 2011-05-30 - T1 - 2011-memorial-day.pdf - early close 12:15 CT.
        (2011, 5, 30, early_close(44100), T1, "2011-memorial-day.pdf @2013-09-30T10:56:52Z"),
        // 2011-07-04 - T1 - 2011-4th-of-july.pdf - early close 12:15 CT.
        (2011, 7, 4, early_close(44100), T1, "2011-4th-of-july.pdf @2011-11-01T14:40:54Z"),
        // 2011-09-05 - T1 - 2011-labor-day.pdf - early close 12:15 CT.
        (2011, 9, 5, early_close(44100), T1, "2011-labor-day.pdf @2011-11-01T14:43:45Z"),
        // 2011-11-24 - T1 - 2011-thanksgiving.pdf - early close 12:15 CT.
        (2011, 11, 24, early_close(44100), T1, "2011-thanksgiving.pdf @2011-11-24T18:52:46Z"),
        // 2011-11-25 - T1 - 2011-thanksgiving.pdf - early close 12:45 CT.
        (2011, 11, 25, early_close(45900), T1, "2011-thanksgiving.pdf @2011-11-24T18:52:46Z"),
        // 2012-01-16 - T1 - 2012-martin-luther-king.pdf - early close 12:15 CT.
        (2012, 1, 16, early_close(44100), T1, "2012-martin-luther-king.pdf @2012-05-05T16:15:26Z"),
        // 2012-02-20 - T1 - 2012-presidents-day.pdf - early close 12:15 CT.
        (2012, 2, 20, early_close(44100), T1, "2012-presidents-day.pdf @2012-05-05T16:15:39Z"),
        // 2012-05-28 - T1 - 2012-memorial-day.pdf - early close 12:15 CT.
        (2012, 5, 28, early_close(44100), T1, "2012-memorial-day.pdf @2012-09-15T00:37:14Z"),
        // 2012-07-04 - T1 - 2012-4th-of-july.pdf - early close 12:15 CT.
        (2012, 7, 4, early_close(44100), T1, "2012-4th-of-july.pdf @2012-09-15T00:39:23Z"),
        // 2012-09-03 - T1 - 2012-labor-day.pdf - early close 12:15 CT.
        (2012, 9, 3, early_close(44100), T1, "2012-labor-day.pdf @2012-09-15T00:34:37Z"),
        // 2012-11-22 - T1 - 2012-thanksgiving.pdf - early close 12:15 CT.
        (2012, 11, 22, early_close(44100), T1, "2012-thanksgiving.pdf @2013-01-27T22:39:01Z"),
        // 2012-11-23 - T1 - 2012-thanksgiving.pdf - early close 12:45 CT.
        (2012, 11, 23, early_close(45900), T1, "2012-thanksgiving.pdf @2013-01-27T22:39:01Z"),
        // 2012-12-24 - T1 - 2012-christmas.pdf - early close 12:45 CT.
        (2012, 12, 24, early_close(45900), T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 - T2 - CME-SVC-2025-01-19 - energy early close 13:30 CT.
        (2025, 1, 20, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-01-19"),
        // 2025-02-17 - T2 - CME-SVC-2025-02-16 - energy early close 13:30 CT.
        (2025, 2, 17, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-02-16"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-17 - energy closed.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 - T2 - CME-SVC-2025-05-25 - energy early close 13:30 CT.
        (2025, 5, 26, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-05-25"),
        // 2025-06-19 - T2 - CME-SVC-2025-06-18 - energy early close 13:30 CT.
        (2025, 6, 19, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-06-18"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-03 - energy early close 12:00 CT.
        (2025, 7, 4, early_close(12 * 3_600), T2, "CME-SVC-2025-07-03"),
        // 2025-09-01 - T2 - CME-SVC-2025-08-31 - energy early close 13:30 CT.
        (2025, 9, 1, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-08-31"),
        // 2025-11-27 - T2 - CME-SVC-2025-11-26 - energy early close 13:30 CT.
        (2025, 11, 27, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-11-26"),
        // 2025-11-28 - T2 - CME-SVC-2025-11-26 - energy early close 13:45 CT.
        (2025, 11, 28, early_close(13 * 3_600 + 45 * 60), T2, "CME-SVC-2025-11-26"),
        // 2025-11-29 - T2 - CME-SVC-2025-11-26-SAT - energy closed.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-12-24 - T2 - CME-SVC-2025-12-24 - energy early close 12:45 CT.
        (2025, 12, 24, early_close(12 * 3_600 + 45 * 60), T2, "CME-SVC-2025-12-24"),
        // 2025-12-25 - T2 - CME-SVC-2025-12-24 - energy closed.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-24"),
        // 2026-01-01 - T2 - CME-SVC-2025-12-31 - energy closed.
        (2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-01-19 - T2 - CME-SVC-2026-01-18 - energy early close 13:30 CT.
        (2026, 1, 19, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-01-18"),
        // 2026-02-16 - T2 - CME-SVC-2026-02-15 - energy early close 13:30 CT.
        (2026, 2, 16, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-02-15"),
        // 2026-04-03 - T2 - CME-SVC-2026-04-01 - energy closed.
        (2026, 4, 3, Closed, T2, "CME-SVC-2026-04-01"),
        // 2026-05-25 - T2 - CME-SVC-2026-05-24 - energy early close 13:30 CT.
        (2026, 5, 25, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-05-24"),
        // 2026-06-19 - T2 - CME-SVC-2026-06-18 - energy early close 12:00 CT.
        (2026, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - energy early close 12:00 CT.
        (2026, 7, 3, early_close(12 * 3_600), T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 - T2 - CME-SVC-2026-09-06 - energy early close 13:30 CT.
        (2026, 9, 7, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-09-06"),
        // 2026-11-26 - T2 - CME-SVC-2026-11-25 - energy early close 13:30 CT.
        (2026, 11, 26, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-11-25"),
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - energy early close 13:45 CT.
        (2026, 11, 27, early_close(13 * 3_600 + 45 * 60), T2, "CME-SVC-2026-11-25"),
        // 2026-12-24 - T2 - CME-SVC-2026-12-22 - energy early close 12:45 CT.
        (2026, 12, 24, early_close(12 * 3_600 + 45 * 60), T2, "CME-SVC-2026-12-22"),
        // 2026-12-25 - T2 - CME-SVC-2026-12-24 - energy closed.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 - T2 - CME-SVC-2026-12-31 - energy closed.
        (2027, 1, 1, Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-01-18 - T2 - CME-SVC-2027-01-17 - energy early close 13:30 CT.
        (2027, 1, 18, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-01-17"),
        // 2027-02-15 - T2 - CME-SVC-2027-02-14 - energy early close 13:30 CT.
        (2027, 2, 15, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-02-14"),
        // 2027-03-26 - T2 - CME-SVC-2027-03-25 - energy closed.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 - T2 - CME-SVC-2027-05-30 - energy early close 13:30 CT.
        (2027, 5, 31, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-05-30"),
        // 2027-06-18 - T2 - CME-SVC-2027-06-17 - energy early close 12:00 CT.
        (2027, 6, 18, early_close(12 * 3_600), T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 - T2 - CME-SVC-2027-07-04 - energy early close 13:30 CT.
        (2027, 7, 5, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-07-04"),
        // 2027-09-06 - T2 - CME-SVC-2027-09-05 - energy early close 13:30 CT.
        (2027, 9, 6, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-09-05"),
        // 2027-11-25 - T2 - CME-SVC-2027-11-24 - energy early close 13:30 CT.
        (2027, 11, 25, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - energy early close 13:45 CT.
        (2027, 11, 26, early_close(13 * 3_600 + 45 * 60), T2, "CME-SVC-2027-11-24"),
        // 2027-12-24 - T2 - CME-SVC-2027-12-22 - energy closed.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
