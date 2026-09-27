//! The `Exchange::Comex` table holiday rows — the COMEX metals half of `globex_energy`.
//!
//! Derived, not retrieved: the rows are the intersection of the families that
//! route to this venue, by the rule and routing recorded in
//! [`super`](index.html). A row ships only where every routed family states
//! it; a disagreement ships [`HolidayKind::Unsourced`], which clips nothing
//! and tells the caller the date is special without inventing an instant.
//!
//! Coverage is the single audited era **2025-2027**, whose rows are
//! `globex_energy`'s own at T2 — CME's `trading-hours-by-product` service —
//! and whose window opens at the crate's permanent 2025 support floor
//! (LAW-COVERAGE). The venue carries the family's 53 rows unchanged: ten
//! closures — including the energy complex's own Good Friday closure of
//! 2026-04-03 — and forty-three rows that state the family's own instant.
//!
//! The derivation, the instant disagreements and every dropped date are in the
//! venue's own evidence file, and the per-family rows are in the family files.

use super::super::{
    EvidenceTier::T2,
    HolidayKind::Closed,
    HolidayKind::ReplacementBlocks,
    HolidayTable,
    fences::early_close,
    globex_energy::{
        MERGED_SESSION_AFTER_WEEKDAY_BLOCKS, MERGED_SESSION_BLOCKS,
        MERGED_SESSION_EARLY_CLOSE_BLOCKS, MERGED_SESSION_EARLY_CLOSE_BLOCKS_2025_11_28,
        SATURDAY_SESSION_BLOCKS,
    },
    holidays,
};

/// The `Exchange::Comex` table: the COMEX metals half of `globex_energy`.
///
/// Metals and energy are one key and the operator prints them as one product
/// row on every date the table audits, so the intersection is total: the venue
/// carries the family's 53 rows unchanged over the single audited window
/// `2025-01-01..2027-12-31` — ten closures and forty-three rows that state the
/// family's own instant — and drops no date. The window is declared as the
/// coverage clause, and outside it the table reports no answer rather than a
/// normal one.
// Evidence: docs/evidence/comex.md
pub(crate) static COMEX: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2025-01-01 - T2 - CME-SVC-2024-12-31 - closed: no trade date.
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 - T2 - CME-SVC-2025-01-19 - metals early close 13:30 CT.
        (2025, 1, 20, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-01-19"),
        // 2025-01-21 - T2 - CME-SVC-2025-01-19 - MLK Day; the holiday publishes no final close, so the span from Sunday evening carries this trade date, and it still ends at the holiday's 13:30 CT close.
        (
            2025,
            1,
            21,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-01-19"
        ),
        // 2025-02-17 - T2 - CME-SVC-2025-02-16 - metals early close 13:30 CT.
        (2025, 2, 17, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-02-16"),
        // 2025-02-18 - T2 - CME-SVC-2025-02-16 - Presidents Day; as 2025-01-21.
        (
            2025,
            2,
            18,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-02-16"
        ),
        // 2025-04-18 - T2 - CME-SVC-2025-04-17 - metals closed.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 - T2 - CME-SVC-2025-05-25 - metals early close 13:30 CT.
        (2025, 5, 26, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-05-25"),
        // 2025-05-27 - T2 - CME-SVC-2025-05-25 - Memorial Day; as 2025-01-21.
        (
            2025,
            5,
            27,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-05-25"
        ),
        // 2025-06-19 - T2 - CME-SVC-2025-06-18 - metals early close 13:30 CT.
        (2025, 6, 19, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-06-18"),
        // 2025-06-20 - T2 - CME-SVC-2025-06-18 - Juneteenth falls on the Thursday, so the span opens Wednesday evening and its -2 queue is the weekday 16:45.
        (
            2025,
            6,
            20,
            ReplacementBlocks(&MERGED_SESSION_AFTER_WEEKDAY_BLOCKS),
            T2,
            "CME-SVC-2025-06-18"
        ),
        // 2025-07-04 - T2 - CME-SVC-2025-07-03 - metals early close 12:00 CT.
        (2025, 7, 4, early_close(12 * 3_600), T2, "CME-SVC-2025-07-03"),
        // 2025-09-01 - T2 - CME-SVC-2025-08-31 - metals early close 13:30 CT.
        (2025, 9, 1, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-08-31"),
        // 2025-09-02 - T2 - CME-SVC-2025-08-31 - Labor Day; as 2025-01-21.
        (
            2025,
            9,
            2,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2025-08-31"
        ),
        // 2025-11-27 - T2 - CME-SVC-2025-11-26 - metals early close 13:30 CT.
        (2025, 11, 27, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-11-26"),
        // 2025-11-28 - T2 - CME-SVC-2025-11-26 - metals early close 13:45 CT.
        // 2025-11-28 - T2 - CME-SVC-2025-11-26 - day after Thanksgiving; the Thursday holiday publishes no final close, so this trade date owns the span from Wednesday evening and ends at the operator's 13:45 CT close.
        (
            2025,
            11,
            28,
            ReplacementBlocks(&MERGED_SESSION_EARLY_CLOSE_BLOCKS_2025_11_28),
            T2,
            "CME-SVC-2025-11-26"
        ),
        // 2025-11-29 - T2 - CME-SVC-2025-11-26-SAT - metals closed.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-12-24 - T2 - CME-SVC-2025-12-24 - metals early close 12:45 CT.
        (2025, 12, 24, early_close(12 * 3_600 + 45 * 60), T2, "CME-SVC-2025-12-24"),
        // 2025-12-25 - T2 - CME-SVC-2025-12-24 - metals closed.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-24"),
        // 2026-01-01 - T2 - CME-SVC-2025-12-31 - metals closed.
        (2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-01-19 - T2 - CME-SVC-2026-01-18 - metals early close 13:30 CT.
        (2026, 1, 19, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-01-18"),
        // 2026-01-20 - T2 - CME-SVC-2026-01-18 - MLK Day; as 2025-01-21.
        (
            2026,
            1,
            20,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-01-18"
        ),
        // 2026-02-16 - T2 - CME-SVC-2026-02-15 - metals early close 13:30 CT.
        (2026, 2, 16, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-02-15"),
        // 2026-02-17 - T2 - CME-SVC-2026-02-15 - Presidents Day; as 2025-01-21.
        (
            2026,
            2,
            17,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-02-15"
        ),
        // 2026-04-03 - T2 - CME-SVC-2026-04-01 - metals closed.
        (2026, 4, 3, Closed, T2, "CME-SVC-2026-04-01"),
        // 2026-05-25 - T2 - CME-SVC-2026-05-24 - metals early close 13:30 CT.
        (2026, 5, 25, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-05-24"),
        // 2026-05-26 - T2 - CME-SVC-2026-05-24 - Memorial Day; as 2025-01-21.
        (
            2026,
            5,
            26,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-05-24"
        ),
        // 2026-06-19 - T2 - CME-SVC-2026-06-18 - metals early close 12:00 CT.
        (2026, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-2026-06-18"),
        // 2026-06-22 - T2 - CME-SVC-2026-06-18 - the energy family's own
        // Saturday-session replacement; one routed family cannot disagree.
        (2026, 6, 22, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - metals early close 12:00 CT.
        (2026, 7, 3, early_close(12 * 3_600), T2, "CME-SVC-2026-07-03"),
        // 2026-07-06 - T2 - CME-SVC-2026-07-03 - as 2026-06-22.
        (2026, 7, 6, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 - T2 - CME-SVC-2026-09-06 - metals early close 13:30 CT.
        (2026, 9, 7, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-09-06"),
        // 2026-09-08 - T2 - CME-SVC-2026-09-06 - Labor Day; as 2025-01-21.
        (
            2026,
            9,
            8,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2026-09-06"
        ),
        // 2026-11-26 - T2 - CME-SVC-2026-11-25 - metals early close 13:30 CT.
        (2026, 11, 26, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-11-25"),
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - metals early close 13:45 CT.
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - day after Thanksgiving; as 2025-11-28.
        (
            2026,
            11,
            27,
            ReplacementBlocks(&MERGED_SESSION_EARLY_CLOSE_BLOCKS),
            T2,
            "CME-SVC-2026-11-25"
        ),
        // 2026-12-24 - T2 - CME-SVC-2026-12-22 - metals early close 12:45 CT.
        (2026, 12, 24, early_close(12 * 3_600 + 45 * 60), T2, "CME-SVC-2026-12-22"),
        // 2026-12-25 - T2 - CME-SVC-2026-12-24 - metals closed.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 - T2 - CME-SVC-2026-12-31 - metals closed.
        (2027, 1, 1, Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-01-18 - T2 - CME-SVC-2027-01-17 - metals early close 13:30 CT.
        (2027, 1, 18, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-01-17"),
        // 2027-01-19 - T2 - CME-SVC-2027-01-17 - MLK Day; as 2025-01-21.
        (
            2027,
            1,
            19,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-01-17"
        ),
        // 2027-02-15 - T2 - CME-SVC-2027-02-14 - metals early close 13:30 CT.
        (2027, 2, 15, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-02-14"),
        // 2027-02-16 - T2 - CME-SVC-2027-02-14 - Presidents Day; as 2025-01-21.
        (
            2027,
            2,
            16,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-02-14"
        ),
        // 2027-03-26 - T2 - CME-SVC-2027-03-25 - metals closed.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 - T2 - CME-SVC-2027-05-30 - metals early close 13:30 CT.
        (2027, 5, 31, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-05-30"),
        // 2027-06-01 - T2 - CME-SVC-2027-05-30 - Memorial Day; as 2025-01-21.
        (
            2027,
            6,
            1,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-05-30"
        ),
        // 2027-06-18 - T2 - CME-SVC-2027-06-17 - metals early close 12:00 CT.
        (2027, 6, 18, early_close(12 * 3_600), T2, "CME-SVC-2027-06-17"),
        // 2027-06-21 - T2 - CME-SVC-2027-06-17 - as 2026-06-22.
        (2027, 6, 21, ReplacementBlocks(&SATURDAY_SESSION_BLOCKS), T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 - T2 - CME-SVC-2027-07-04 - metals early close 13:30 CT.
        (2027, 7, 5, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-07-04"),
        // 2027-07-06 - T2 - CME-SVC-2027-07-04 - Independence Day observed; as 2025-01-21.
        (
            2027,
            7,
            6,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-07-04"
        ),
        // 2027-09-06 - T2 - CME-SVC-2027-09-05 - metals early close 13:30 CT.
        (2027, 9, 6, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-09-05"),
        // 2027-09-07 - T2 - CME-SVC-2027-09-05 - Labor Day; as 2025-01-21.
        (
            2027,
            9,
            7,
            ReplacementBlocks(&MERGED_SESSION_BLOCKS),
            T2,
            "CME-SVC-2027-09-05"
        ),
        // 2027-11-25 - T2 - CME-SVC-2027-11-24 - metals early close 13:30 CT.
        (2027, 11, 25, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - metals early close 13:45 CT.
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - day after Thanksgiving; as 2025-11-28.
        (
            2027,
            11,
            26,
            ReplacementBlocks(&MERGED_SESSION_EARLY_CLOSE_BLOCKS),
            T2,
            "CME-SVC-2027-11-24"
        ),
        // 2027-12-24 - T2 - CME-SVC-2027-12-22 - metals closed.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
