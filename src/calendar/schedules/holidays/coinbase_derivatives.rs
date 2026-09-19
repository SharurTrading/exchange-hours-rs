// SPDX-License-Identifier: MIT-0

//! Coinbase Derivatives holiday rows, from the venue's first trade date
//! 2021-06-28 through 2026-09-07.
//!
//! Keyed by the crate's own venue-local trade date in `America/Chicago`
//! (design memo D1). CDE's Market Notices are already keyed by **trade date**
//! and print the open, close and roll instants of the neighbouring dates
//! beside each one, so the conversion is the identity and the operator's own
//! table corroborates it row for row.
//!
//! The venue default this table attaches to is CDE's recurring 23x5 futures
//! grid - Sunday to Friday, 17:00-16:00 CT with the daily 16:00-17:00 break -
//! so every row states what the notices state for the product groups on that
//! grid. CDE lists other tiers on other clocks: the 24x7 crypto tier, which
//! trades through these dates, and, from 2026, a 24x5 equity-index PSF group.
//! Neither is out of scope by accident - the venue profile models the 23x5
//! grid and those tiers claim no key - and both are recorded in
//! [`docs/evidence/coinbase_derivatives.md`](../../../../../docs/evidence/coinbase_derivatives.md).
//!
//! On a half day the groups listed on that grid can print different instants.
//! A row then carries the **earliest** of them, so the venue never reports a
//! window in which no product can print; each group's own instant is in the
//! evidence file. Six dates carry an early close and two are `Unsourced`:
//! 2022-11-24 and 2022-11-25, whose notice 22-10 the operator lists but whose
//! PDF is unreachable, so the crate declines to claim those dates either way.
//! Trade date 2025-01-09 ships no row because notice 24-27 states it "will
//! observe a normal trading day", and notice 24-26 records an unplanned
//! technical early close rather than a published holiday schedule.
//!
//! Every row is **T1**, a numbered CDE Market Notice. Coverage runs from the
//! `FairX` launch day to 2026-09-07, the end of the window the first table
//! shipped; the notices for 2026-09-08 onward belong to the published-future
//! refresh, and CDE has published nothing for 2027.

use super::fences::early_close;
use super::{
    EvidenceTier::T1,
    HolidayKind::{Closed, Unsourced},
    HolidayTable, holidays,
};

/// The venue's built-in holiday rows and the window they were audited over.
// Evidence: docs/evidence/coinbase_derivatives.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2021, 6, 28) ..= (2026, 9, 7)],
    rows: [
        // 2021-07-05 - T1 - CDE-MN-21-03 - Independence Day observed
        (2021, 7, 5, Closed, T1, "CDE-MN-21-03"),
        // 2021-09-06 - T1 - CDE-MN-21-04 - Labor Day
        (2021, 9, 6, Closed, T1, "CDE-MN-21-04"),
        // 2021-11-25 - T1 - CDE-MN-21-06 - Thanksgiving Day
        (2021, 11, 25, Closed, T1, "CDE-MN-21-06"),
        // 2021-11-26 - T1 - CDE-MN-21-06 - Thanksgiving half day: Equity Products 12:15 CT, Energy Products 12:45 CT
        (2021, 11, 26, early_close(12 * 3_600 + 15 * 60), T1, "CDE-MN-21-06"),
        // 2021-12-24 - T1 - CDE-MN-21-07 - Christmas Day observed
        (2021, 12, 24, Closed, T1, "CDE-MN-21-07"),
        // 2022-01-17 - T1 - CDE-MN-22-01 - Martin Luther King Jr. Day
        (2022, 1, 17, Closed, T1, "CDE-MN-22-01"),
        // 2022-02-21 - T1 - CDE-MN-22-02 - Presidents' Day
        (2022, 2, 21, Closed, T1, "CDE-MN-22-02"),
        // 2022-04-15 - T1 - CDE-MN-22-04 - Good Friday
        (2022, 4, 15, Closed, T1, "CDE-MN-22-04"),
        // 2022-05-30 - T1 - CDE-MN-22-05 - Memorial Day
        (2022, 5, 30, Closed, T1, "CDE-MN-22-05"),
        // 2022-06-20 - T1 - CDE-MN-22-06 - Juneteenth
        (2022, 6, 20, Closed, T1, "CDE-MN-22-06"),
        // 2022-07-04 - T1 - CDE-MN-22-07 - Independence Day
        (2022, 7, 4, Closed, T1, "CDE-MN-22-07"),
        // 2022-09-05 - T1 - CDE-MN-22-08 - Labor Day
        (2022, 9, 5, Closed, T1, "CDE-MN-22-08"),
        // 2022-11-24 - T1 - CDE-NOTICES-INDEX-2026-09-19 - Thanksgiving Day; notice 22-10 is listed but its PDF is unreachable
        (2022, 11, 24, Unsourced, T1, "CDE-NOTICES-INDEX-2026-09-19"),
        // 2022-11-25 - T1 - CDE-NOTICES-INDEX-2026-09-19 - Thanksgiving half day; notice 22-10 is listed but its PDF is unreachable
        (2022, 11, 25, Unsourced, T1, "CDE-NOTICES-INDEX-2026-09-19"),
        // 2022-12-26 - T1 - CDE-MN-22-11 - Christmas Day observed
        (2022, 12, 26, Closed, T1, "CDE-MN-22-11"),
        // 2023-01-02 - T1 - CDE-MN-23-01 - New Year's Day observed
        (2023, 1, 2, Closed, T1, "CDE-MN-23-01"),
        // 2023-01-16 - T1 - CDE-MN-23-02 - Martin Luther King Jr. Day
        (2023, 1, 16, Closed, T1, "CDE-MN-23-02"),
        // 2023-02-20 - T1 - CDE-MN-23-03 - Presidents' Day
        (2023, 2, 20, Closed, T1, "CDE-MN-23-03"),
        // 2023-04-07 - T1 - CDE-MN-23-07 - Good Friday
        (2023, 4, 7, Closed, T1, "CDE-MN-23-07"),
        // 2023-05-29 - T1 - CDE-MN-23-09 - Memorial Day
        (2023, 5, 29, Closed, T1, "CDE-MN-23-09"),
        // 2023-06-19 - T1 - CDE-MN-23-10 - Juneteenth
        (2023, 6, 19, Closed, T1, "CDE-MN-23-10"),
        // 2023-07-04 - T1 - CDE-MN-23-11 - Independence Day
        (2023, 7, 4, Closed, T1, "CDE-MN-23-11"),
        // 2023-09-04 - T1 - CDE-MN-23-13 - Labor Day
        (2023, 9, 4, Closed, T1, "CDE-MN-23-13"),
        // 2023-11-23 - T1 - CDE-MN-23-16 - Thanksgiving Day
        (2023, 11, 23, Closed, T1, "CDE-MN-23-16"),
        // 2023-11-24 - T1 - CDE-MN-23-16 - Thanksgiving half day: Equity 12:15 CT, Energy 12:45 CT, 23x5 Crypto 12:45 CT
        (2023, 11, 24, early_close(12 * 3_600 + 15 * 60), T1, "CDE-MN-23-16"),
        // 2023-12-25 - T1 - CDE-MN-23-19 - Christmas Day
        (2023, 12, 25, Closed, T1, "CDE-MN-23-19"),
        // 2024-01-01 - T1 - CDE-MN-23-20 - New Year's Day
        (2024, 1, 1, Closed, T1, "CDE-MN-23-20"),
        // 2024-01-15 - T1 - CDE-MN-24-01 - Martin Luther King Jr. Day
        (2024, 1, 15, Closed, T1, "CDE-MN-24-01"),
        // 2024-02-19 - T1 - CDE-MN-24-02 - Presidents' Day
        (2024, 2, 19, Closed, T1, "CDE-MN-24-02"),
        // 2024-03-29 - T1 - CDE-MN-24-04 - Good Friday
        (2024, 3, 29, Closed, T1, "CDE-MN-24-04"),
        // 2024-05-27 - T1 - CDE-MN-24-09 - Memorial Day
        (2024, 5, 27, Closed, T1, "CDE-MN-24-09"),
        // 2024-06-19 - T1 - CDE-MN-24-12 - Juneteenth
        (2024, 6, 19, Closed, T1, "CDE-MN-24-12"),
        // 2024-07-04 - T1 - CDE-MN-24-13 - Independence Day
        (2024, 7, 4, Closed, T1, "CDE-MN-24-13"),
        // 2024-09-02 - T1 - CDE-MN-24-16 - Labor Day
        (2024, 9, 2, Closed, T1, "CDE-MN-24-16"),
        // 2024-11-28 - T1 - CDE-MN-24-21 - Thanksgiving Day
        (2024, 11, 28, Closed, T1, "CDE-MN-24-21"),
        // 2024-11-29 - T1 - CDE-MN-24-21 - Thanksgiving half day; Energy, Metal and Crypto each print 13:45 CT
        (2024, 11, 29, early_close(13 * 3_600 + 45 * 60), T1, "CDE-MN-24-21"),
        // 2024-12-24 - T1 - CDE-MN-24-23 - Christmas Eve half day: Energy and Metal 12:45 CT; the 23x5 Crypto group trades to 16:00 CT
        (2024, 12, 24, early_close(12 * 3_600 + 45 * 60), T1, "CDE-MN-24-23"),
        // 2024-12-25 - T1 - CDE-MN-24-23 - Christmas Day
        (2024, 12, 25, Closed, T1, "CDE-MN-24-23"),
        // 2025-01-01 - T1 - CDE-MN-24-25 - New Year's Day
        (2025, 1, 1, Closed, T1, "CDE-MN-24-25"),
        // 2025-01-20 - T1 - CDE-MN-25-01 - Martin Luther King Jr. Day
        (2025, 1, 20, Closed, T1, "CDE-MN-25-01"),
        // 2025-02-17 - T1 - CDE-MN-25-03 - Presidents' Day
        (2025, 2, 17, Closed, T1, "CDE-MN-25-03"),
        // 2025-04-18 - T1 - CDE-MN-25-15 - Good Friday
        (2025, 4, 18, Closed, T1, "CDE-MN-25-15"),
        // 2025-05-26 - T1 - CDE-MN-25-18 - Memorial Day
        (2025, 5, 26, Closed, T1, "CDE-MN-25-18"),
        // 2025-06-19 - T1 - CDE-MN-25-20 - Juneteenth
        (2025, 6, 19, Closed, T1, "CDE-MN-25-20"),
        // 2025-07-04 - T1 - CDE-MN-25-21 - Independence Day
        (2025, 7, 4, Closed, T1, "CDE-MN-25-21"),
        // 2025-09-01 - T1 - CDE-MN-25-29 - Labor Day
        (2025, 9, 1, Closed, T1, "CDE-MN-25-29"),
        // 2025-11-27 - T1 - CDE-MN-25-37 - Thanksgiving Day
        (2025, 11, 27, Closed, T1, "CDE-MN-25-37"),
        // 2025-11-28 - T1 - CDE-MN-25-37 - Thanksgiving half day: Equity 12:15 CT, Energy & Metal 13:45 CT
        (2025, 11, 28, early_close(12 * 3_600 + 15 * 60), T1, "CDE-MN-25-37"),
        // 2025-12-24 - T1 - CDE-MN-25-41 - Christmas Eve half day: Equity 12:15 CT, Energy & Metal 12:45 CT
        (2025, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "CDE-MN-25-41"),
        // 2025-12-25 - T1 - CDE-MN-25-41 - Christmas Day
        (2025, 12, 25, Closed, T1, "CDE-MN-25-41"),
        // 2026-01-01 - T1 - CDE-MN-25-42 - New Year's Day, Energy & Metal and Equity closed for holiday; trade date 1/2 opens 1/1 17:00 CT.
        (2026, 1, 1, Closed, T1, "CDE-MN-25-42"),
        // 2026-01-19 - T1 - CDE-MN-26-01 - Martin Luther King Jr. Day
        (2026, 1, 19, Closed, T1, "CDE-MN-26-01"),
        // 2026-02-16 - T1 - CDE-MN-26-05 - Presidents' Day
        (2026, 2, 16, Closed, T1, "CDE-MN-26-05"),
        // 2026-04-03 - T1 - CDE-MN-26-12 - Good Friday
        (2026, 4, 3, Closed, T1, "CDE-MN-26-12"),
        // 2026-05-25 - T1 - CDE-MN-26-23 - Memorial Day
        (2026, 5, 25, Closed, T1, "CDE-MN-26-23"),
        // 2026-06-19 - T1 - CDE-MN-26-27.1 - Juneteenth; the 23x5 and 24x5 tiers close while Gold and Silver stay open with the 24x7 tier.
        (2026, 6, 19, Closed, T1, "CDE-MN-26-27.1"),
        // 2026-07-03 - T1 - CDE-MN-26-29 - Independence Day observed
        (2026, 7, 3, Closed, T1, "CDE-MN-26-29"),
        // 2026-09-07 - T1 - CDE-MN-26-36 - Labor Day
        (2026, 9, 7, Closed, T1, "CDE-MN-26-36"),
    ],
};
