// SPDX-License-Identifier: MIT-0

//! Toronto Stock Exchange (TSX) holiday rows, 2024-2026.
//!
//! Keyed by the crate's own venue-local trade date in `America/Toronto`. TSX
//! runs no overnight session, so an event date and its trade date are one
//! civil day and the conversion is the identity.
//!
//! The whole block is **T1**: TMX Group's own "Stock Market Holidays - Stock
//! Markets Closed" calendar at
//! `tsx.com/en/trading/calendars-and-trading-hours/calendar`, one server-
//! rendered page carrying the current year's list in full and the next
//! year's from Q4. The 2025 and 2026 rows are read from the live page
//! (retrieved 2026-09-28); the 2024 rows are read from the archived
//! 2024-12-17 state, which carries the complete 2024 list including its
//! Christmas Eve line — the July 2024 states printed the list without that
//! row, so the Christmas Eve row keys to the December capture (a knowledge
//! boundary may only widen). The 2017-2023 rows are read from ten archived
//! states of the same page across its two paths: the `/trading/` relaunch
//! path (2018-2023, its first capture 2018-09-11, whose archive section also
//! restates the complete 2017 list) and the `/en/` path (2024). The
//! Christmas Eve half days of 2018-2021 ride the page's own sentence
//! `Markets will close at 1:00 PM ...`; the 2021 sentence was printed
//! `subject to Board Approval` in January 2021 and the 2022-01-28 capture's
//! archive section witnesses the discharged state, so that row keys to the
//! later artifact. **2010-2016** ships no rows: the Wayback index holds no
//! capture of any TSX holiday page for those years that this session could
//! retrieve (CDX was down for most of 2026-09-29; every reachable capture
//! set starts at 2018-09-11), so queries before 2017-01-01 refuse and the
//! span is tracked as an issue. TSX has published no 2027 calendar yet, so
//! coverage stops at 2026-12-31; all of this is recorded in
//! [`docs/evidence/tsx.md`](../../../../../docs/evidence/tsx.md).
//!
//! The one early close is the operator's own footnote: Christmas Eve closes at
//! `1:00 PM (TSX/TSXV)` — the 1:30 PM half of that footnote applies to the
//! ALPHA/ALPHA X/DRK book systems and is outside this venue's scope. The U.S.
//! holidays the same page lists under a separate heading are footnoted as
//! **special-settlement** days for USD issues, not trading closures, so none
//! of them is encoded (LAW-SESSION-NOT-EXPIRY).

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::fences::early_close;
use super::{HolidayTable, holidays};

/// TSX's stated Christmas Eve close, 1:00 PM Toronto time.
///
/// The calendar's footnote reads `* Closing at 1:00 PM (TSX/TSXV) and 1:30
/// (ALPHA/ALPHA X/DRK)`; this identity's scope is the Toronto Stock Exchange
/// cash-equity market, whose close is 13:00.
const HALF_DAY_13_00: u32 = 13 * 3_600;

/// TSX's built-in holiday rows and the window they were audited over.
///
/// Every row is one line of the operator's "Stock Markets Closed" list at
/// the earliest archived state that prints it — the live retrieval of
/// 2026-09-28 (2025 and 2026), the archived 2024-12-17 state (2024), or the
/// 2018-2023 captures of the `/trading/` path, whose 2018-09-11 archive
/// section restates 2017. A date inside the window with no row is audited
/// normal.
// Evidence: docs/evidence/tsx.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2017, 1, 1) ..= (2026, 12, 31)],
    rows: [
        // 2017-01-02 - T1 - TSX-CAL-2018-09-11 - New Year's Day (in lieu):
        // `Monday, January 2, 2017 * in lieu of New Years Day, Sunday January 1`.
        (2017, 1, 2, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2017-02-20 - T1 - TSX-CAL-2018-09-11 - Family Day.
        (2017, 2, 20, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2017-04-14 - T1 - TSX-CAL-2018-09-11 - Good Friday.
        (2017, 4, 14, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2017-05-22 - T1 - TSX-CAL-2018-09-11 - Victoria Day.
        (2017, 5, 22, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2017-07-03 - T1 - TSX-CAL-2018-09-11 - Canada Day (in lieu of the
        // Saturday).
        (2017, 7, 3, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2017-08-07 - T1 - TSX-CAL-2018-09-11 - Civic Holiday.
        (2017, 8, 7, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2017-09-04 - T1 - TSX-CAL-2018-09-11 - Labour Day.
        (2017, 9, 4, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2017-10-09 - T1 - TSX-CAL-2018-09-11 - Thanksgiving Day.
        (2017, 10, 9, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2017-12-25 - T1 - TSX-CAL-2018-09-11 - Christmas Day.
        (2017, 12, 25, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2017-12-26 - T1 - TSX-CAL-2018-09-11 - Boxing Day; the 2017 list
        // prints no Christmas Eve line.
        (2017, 12, 26, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-01-01 - T1 - TSX-CAL-2018-09-11 - New Year's Day.
        (2018, 1, 1, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-02-19 - T1 - TSX-CAL-2018-09-11 - Family Day.
        (2018, 2, 19, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-03-30 - T1 - TSX-CAL-2018-09-11 - Good Friday.
        (2018, 3, 30, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-05-21 - T1 - TSX-CAL-2018-09-11 - Victoria Day.
        (2018, 5, 21, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-07-02 - T1 - TSX-CAL-2018-09-11 - Canada Day (in lieu of the
        // Sunday).
        (2018, 7, 2, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-08-06 - T1 - TSX-CAL-2018-09-11 - Civic Holiday.
        (2018, 8, 6, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-09-03 - T1 - TSX-CAL-2018-09-11 - Labour Day.
        (2018, 9, 3, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-10-08 - T1 - TSX-CAL-2018-09-11 - Thanksgiving Day.
        (2018, 10, 8, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-12-24 - T1 - TSX-CAL-2018-09-11 - the page states `Markets
        // will close at 1:00 PM on December 24th, 2018.`; corroborated by the
        // 2019-08-20 capture's archive section.
        (2018, 12, 24, early_close(HALF_DAY_13_00), T1, "TSX-CAL-2018-09-11"),
        // 2018-12-25 - T1 - TSX-CAL-2018-09-11 - Christmas Day.
        (2018, 12, 25, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2018-12-26 - T1 - TSX-CAL-2018-09-11 - Boxing Day.
        (2018, 12, 26, Closed, T1, "TSX-CAL-2018-09-11"),
        // 2019-01-01 - T1 - TSX-CAL-2019-08-20 - New Year's Day.
        (2019, 1, 1, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2019-02-18 - T1 - TSX-CAL-2019-08-20 - Family Day.
        (2019, 2, 18, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2019-04-19 - T1 - TSX-CAL-2019-08-20 - Good Friday.
        (2019, 4, 19, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2019-05-20 - T1 - TSX-CAL-2019-08-20 - Victoria Day.
        (2019, 5, 20, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2019-07-01 - T1 - TSX-CAL-2019-08-20 - Canada Day.
        (2019, 7, 1, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2019-08-05 - T1 - TSX-CAL-2019-08-20 - Civic Holiday.
        (2019, 8, 5, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2019-09-02 - T1 - TSX-CAL-2019-08-20 - Labour Day.
        (2019, 9, 2, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2019-10-14 - T1 - TSX-CAL-2019-08-20 - Thanksgiving Day.
        (2019, 10, 14, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2019-12-24 - T1 - TSX-CAL-2019-08-20 - `Markets will close at
        // 1:00 PM on December 24th, 2019.`; corroborated by the 2020-03-29
        // capture.
        (2019, 12, 24, early_close(HALF_DAY_13_00), T1, "TSX-CAL-2019-08-20"),
        // 2019-12-25 - T1 - TSX-CAL-2019-08-20 - Christmas Day.
        (2019, 12, 25, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2019-12-26 - T1 - TSX-CAL-2019-08-20 - Boxing Day.
        (2019, 12, 26, Closed, T1, "TSX-CAL-2019-08-20"),
        // 2020-01-01 - T1 - TSX-CAL-2020-03-29 - New Year's Day.
        (2020, 1, 1, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2020-02-17 - T1 - TSX-CAL-2020-03-29 - Family Day.
        (2020, 2, 17, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2020-04-10 - T1 - TSX-CAL-2020-03-29 - Good Friday.
        (2020, 4, 10, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2020-05-18 - T1 - TSX-CAL-2020-03-29 - Victoria Day.
        (2020, 5, 18, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2020-07-01 - T1 - TSX-CAL-2020-03-29 - Canada Day.
        (2020, 7, 1, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2020-08-03 - T1 - TSX-CAL-2020-03-29 - Civic Holiday.
        (2020, 8, 3, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2020-09-07 - T1 - TSX-CAL-2020-03-29 - Labour Day.
        (2020, 9, 7, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2020-10-12 - T1 - TSX-CAL-2020-03-29 - Thanksgiving Day.
        (2020, 10, 12, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2020-12-24 - T1 - TSX-CAL-2020-03-29 - `Markets will close at
        // 1:00 PM on December 24th, 2020.`; restated by the 2021-01-25
        // capture.
        (2020, 12, 24, early_close(HALF_DAY_13_00), T1, "TSX-CAL-2020-03-29"),
        // 2020-12-25 - T1 - TSX-CAL-2020-03-29 - Christmas Day.
        (2020, 12, 25, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2020-12-28 - T1 - TSX-CAL-2020-03-29 - In Lieu of Boxing Day.
        (2020, 12, 28, Closed, T1, "TSX-CAL-2020-03-29"),
        // 2021-01-01 - T1 - TSX-CAL-2021-01-25 - New Year's Day.
        (2021, 1, 1, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2021-02-15 - T1 - TSX-CAL-2021-01-25 - Family Day.
        (2021, 2, 15, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2021-04-02 - T1 - TSX-CAL-2021-01-25 - Good Friday.
        (2021, 4, 2, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2021-05-24 - T1 - TSX-CAL-2021-01-25 - Victoria Day.
        (2021, 5, 24, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2021-07-01 - T1 - TSX-CAL-2021-01-25 - Canada Day.
        (2021, 7, 1, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2021-08-02 - T1 - TSX-CAL-2021-01-25 - Civic Holiday.
        (2021, 8, 2, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2021-09-06 - T1 - TSX-CAL-2021-01-25 - Labour Day.
        (2021, 9, 6, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2021-10-11 - T1 - TSX-CAL-2021-01-25 - Thanksgiving Day.
        (2021, 10, 11, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2021-12-24 - T1 - TSX-CAL-2022-01-28 - Christmas Eve: the January
        // capture printed the half day `subject to Board Approval`, and this
        // later state of the page witnesses the discharged condition —
        // `TSX and TSX Venture Exchange will close early at 1:00 p.m.`
        (2021, 12, 24, early_close(HALF_DAY_13_00), T1, "TSX-CAL-2022-01-28"),
        // 2021-12-27 - T1 - TSX-CAL-2021-01-25 - In Lieu of Christmas Day.
        (2021, 12, 27, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2021-12-28 - T1 - TSX-CAL-2021-01-25 - In Lieu of Boxing Day.
        (2021, 12, 28, Closed, T1, "TSX-CAL-2021-01-25"),
        // 2022-01-03 - T1 - TSX-CAL-2022-01-28 - In Lieu of New Year's Day.
        (2022, 1, 3, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2022-02-21 - T1 - TSX-CAL-2022-01-28 - Family Day.
        (2022, 2, 21, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2022-04-15 - T1 - TSX-CAL-2022-01-28 - Good Friday.
        (2022, 4, 15, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2022-05-23 - T1 - TSX-CAL-2022-01-28 - Victoria Day.
        (2022, 5, 23, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2022-07-01 - T1 - TSX-CAL-2022-01-28 - Canada Day.
        (2022, 7, 1, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2022-08-01 - T1 - TSX-CAL-2022-01-28 - Civic Holiday.
        (2022, 8, 1, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2022-09-05 - T1 - TSX-CAL-2022-01-28 - Labour Day.
        (2022, 9, 5, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2022-10-10 - T1 - TSX-CAL-2022-01-28 - Thanksgiving Day.
        (2022, 10, 10, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2022-12-26 - T1 - TSX-CAL-2022-01-28 - In Lieu of Christmas Day;
        // 24 December 2022 was a Saturday and the list prints no Christmas
        // Eve line.
        (2022, 12, 26, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2022-12-27 - T1 - TSX-CAL-2022-01-28 - In Lieu of Boxing Day.
        (2022, 12, 27, Closed, T1, "TSX-CAL-2022-01-28"),
        // 2023-01-02 - T1 - TSX-CAL-2023-01-16 - In Lieu of New Year's Day.
        (2023, 1, 2, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2023-02-20 - T1 - TSX-CAL-2023-01-16 - Family Day.
        (2023, 2, 20, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2023-04-07 - T1 - TSX-CAL-2023-01-16 - Good Friday.
        (2023, 4, 7, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2023-05-22 - T1 - TSX-CAL-2023-01-16 - Victoria Day.
        (2023, 5, 22, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2023-07-03 - T1 - TSX-CAL-2023-01-16 - Canada Day.
        (2023, 7, 3, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2023-08-07 - T1 - TSX-CAL-2023-01-16 - Civic Holiday.
        (2023, 8, 7, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2023-09-04 - T1 - TSX-CAL-2023-01-16 - Labour Day.
        (2023, 9, 4, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2023-10-09 - T1 - TSX-CAL-2023-01-16 - Thanksgiving Day.
        (2023, 10, 9, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2023-12-25 - T1 - TSX-CAL-2023-01-16 - Christmas Day; 24 December
        // 2023 was a Sunday and the list prints no Christmas Eve line.
        (2023, 12, 25, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2023-12-26 - T1 - TSX-CAL-2023-01-16 - Boxing Day.
        (2023, 12, 26, Closed, T1, "TSX-CAL-2023-01-16"),
        // 2024-01-01 - T1 - TSX-CAL-2024-12-17 - In Lieu of New Year's Day.
        (2024, 1, 1, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2024-02-19 - T1 - TSX-CAL-2024-12-17 - Family Day.
        (2024, 2, 19, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2024-03-29 - T1 - TSX-CAL-2024-12-17 - Good Friday.
        (2024, 3, 29, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2024-05-20 - T1 - TSX-CAL-2024-12-17 - Victoria Day.
        (2024, 5, 20, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2024-07-01 - T1 - TSX-CAL-2024-12-17 - Canada Day.
        (2024, 7, 1, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2024-08-05 - T1 - TSX-CAL-2024-12-17 - Civic Holiday.
        (2024, 8, 5, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2024-09-02 - T1 - TSX-CAL-2024-12-17 - Labour Day.
        (2024, 9, 2, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2024-10-14 - T1 - TSX-CAL-2024-12-17 - Thanksgiving Day.
        (2024, 10, 14, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2024-12-24 - T1 - TSX-CAL-2024-12-17 - Christmas Eve: closing at
        // 1:00 PM (TSX/TSXV); the operator added this line to the 2024 list
        // between the July and December states of the page.
        (2024, 12, 24, early_close(HALF_DAY_13_00), T1, "TSX-CAL-2024-12-17"),
        // 2024-12-25 - T1 - TSX-CAL-2024-12-17 - Christmas Day.
        (2024, 12, 25, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2024-12-26 - T1 - TSX-CAL-2024-12-17 - Boxing Day.
        (2024, 12, 26, Closed, T1, "TSX-CAL-2024-12-17"),
        // 2025-01-01 - T1 - TSX-CAL-2026-09-28 - New Year's Day.
        (2025, 1, 1, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-02-17 - T1 - TSX-CAL-2026-09-28 - Family Day.
        (2025, 2, 17, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-04-18 - T1 - TSX-CAL-2026-09-28 - Good Friday.
        (2025, 4, 18, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-05-19 - T1 - TSX-CAL-2026-09-28 - Victoria Day.
        (2025, 5, 19, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-07-01 - T1 - TSX-CAL-2026-09-28 - Canada Day.
        (2025, 7, 1, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-08-04 - T1 - TSX-CAL-2026-09-28 - Civic Holiday.
        (2025, 8, 4, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-09-01 - T1 - TSX-CAL-2026-09-28 - Labour Day.
        (2025, 9, 1, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-10-13 - T1 - TSX-CAL-2026-09-28 - Thanksgiving Day.
        (2025, 10, 13, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-12-24 - T1 - TSX-CAL-2026-09-28 - Christmas Eve: closing at
        // 1:00 PM (TSX/TSXV).
        (2025, 12, 24, early_close(HALF_DAY_13_00), T1, "TSX-CAL-2026-09-28"),
        // 2025-12-25 - T1 - TSX-CAL-2026-09-28 - Christmas Day.
        (2025, 12, 25, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2025-12-26 - T1 - TSX-CAL-2026-09-28 - Boxing Day.
        (2025, 12, 26, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-01-01 - T1 - TSX-CAL-2026-09-28 - New Year's Day.
        (2026, 1, 1, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-02-16 - T1 - TSX-CAL-2026-09-28 - Family Day.
        (2026, 2, 16, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-04-03 - T1 - TSX-CAL-2026-09-28 - Good Friday.
        (2026, 4, 3, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-05-18 - T1 - TSX-CAL-2026-09-28 - Victoria Day.
        (2026, 5, 18, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-07-01 - T1 - TSX-CAL-2026-09-28 - Canada Day.
        (2026, 7, 1, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-08-03 - T1 - TSX-CAL-2026-09-28 - Civic Holiday.
        (2026, 8, 3, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-09-07 - T1 - TSX-CAL-2026-09-28 - Labour Day.
        (2026, 9, 7, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-10-12 - T1 - TSX-CAL-2026-09-28 - Thanksgiving Day.
        (2026, 10, 12, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-12-24 - T1 - TSX-CAL-2026-09-28 - Christmas Eve: closing at
        // 1:00 PM (TSX/TSXV).
        (2026, 12, 24, early_close(HALF_DAY_13_00), T1, "TSX-CAL-2026-09-28"),
        // 2026-12-25 - T1 - TSX-CAL-2026-09-28 - Christmas Day.
        (2026, 12, 25, Closed, T1, "TSX-CAL-2026-09-28"),
        // 2026-12-28 - T1 - TSX-CAL-2026-09-28 - In Lieu of Boxing Day.
        (2026, 12, 28, Closed, T1, "TSX-CAL-2026-09-28"),
    ],
};
