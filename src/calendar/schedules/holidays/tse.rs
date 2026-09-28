// SPDX-License-Identifier: MIT-0

//! Tokyo Stock Exchange holiday rows, 2025-2027.
//!
//! Keyed by the crate's own venue-local trade date in `Asia/Tokyo` (design
//! memo D1). TSE runs no overnight wrap and no Saturday sessions, so a closed
//! trade date is one whose daytime sessions are absent outright.
//!
//! The whole block is **T1**: the operator's own `Market Holidays` page on
//! `jpx.co.jp`, which prints the current and next year. The 2025 table came
//! from the page's Wayback `id_` replay (the live page rotates each January),
//! and the 2026 and 2027 tables from the live page retrieved 2026-09-28; the
//! 2026 table is identical in both except the live page's one extra weekend-printed row (Jan. 3 (Sat.) Market Holiday), which keys no row. JPX states the scope in one sentence:
//! "JPX markets are closed on Saturdays, Sundays, national holidays, and on
//! the dates indicated below", so the rows below are exactly the printed dates
//! that fall on a weekday — a printed holiday that lands on a Saturday or
//! Sunday removes no session and ships no row, which keeps every shipped row
//! an answer that differs from the normal week. The derivation, per-row
//! quotations and the national-holiday observance notes are recorded in
//! [`docs/evidence/tse.md`](../../../../../docs/evidence/tse.md).
//!
//! JPX publishes no holiday-time early closes or late opens for the cash
//! market: every printed date is a full closure, and the page's only
//! conditional note ("Exchange holidays are subject to change due to changes
//! to national holidays under Japan's Act on National Holidays") governs a
//! legislative change that has not occurred inside this window.

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::{HolidayTable, holidays};

/// TSE's built-in holiday rows and the window they were audited over.
///
/// Every row is one printed date of the operator's `Market Holidays` table:
/// `JPX-HOL-2025` for the 2025 table (Wayback replay of 2025-09-23, page
/// state "Update : Mar. 07, 2025") and `JPX-HOL-2026-2027` for the 2026 and
/// 2027 tables (live page, "Update : Feb. 06, 2026"). A date inside the
/// window with no row is audited normal.
// Evidence: docs/evidence/tse.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2027, 12, 31)],
    rows: [
        // 2025-01-01 - T1 - JPX-HOL-2025 - New Year's Day.
        (2025, 1, 1, Closed, T1, "JPX-HOL-2025"),
        // 2025-01-02 - T1 - JPX-HOL-2025 - Market Holiday.
        (2025, 1, 2, Closed, T1, "JPX-HOL-2025"),
        // 2025-01-03 - T1 - JPX-HOL-2025 - Market Holiday.
        (2025, 1, 3, Closed, T1, "JPX-HOL-2025"),
        // 2025-01-13 - T1 - JPX-HOL-2025 - Coming of Age Day.
        (2025, 1, 13, Closed, T1, "JPX-HOL-2025"),
        // 2025-02-11 - T1 - JPX-HOL-2025 - National Foundation Day.
        (2025, 2, 11, Closed, T1, "JPX-HOL-2025"),
        // 2025-02-24 - T1 - JPX-HOL-2025 - Emperor's Birthday (Feb. 23) observed:
        // Feb. 23 is a Sunday, so the observance is the row and Feb. 23 ships none.
        (2025, 2, 24, Closed, T1, "JPX-HOL-2025"),
        // 2025-03-20 - T1 - JPX-HOL-2025 - Vernal Equinox.
        (2025, 3, 20, Closed, T1, "JPX-HOL-2025"),
        // 2025-04-29 - T1 - JPX-HOL-2025 - Showa Day.
        (2025, 4, 29, Closed, T1, "JPX-HOL-2025"),
        // 2025-05-05 - T1 - JPX-HOL-2025 - Children's Day. May 3 (Constitution
        // Memorial Day, Saturday) and May 4 (Greenery Day, Sunday) ship no rows.
        (2025, 5, 5, Closed, T1, "JPX-HOL-2025"),
        // 2025-05-06 - T1 - JPX-HOL-2025 - Greenery Day (May 4) observed.
        (2025, 5, 6, Closed, T1, "JPX-HOL-2025"),
        // 2025-07-21 - T1 - JPX-HOL-2025 - Marine Day.
        (2025, 7, 21, Closed, T1, "JPX-HOL-2025"),
        // 2025-08-11 - T1 - JPX-HOL-2025 - Mountain Day.
        (2025, 8, 11, Closed, T1, "JPX-HOL-2025"),
        // 2025-09-15 - T1 - JPX-HOL-2025 - Respect for the Aged Day.
        (2025, 9, 15, Closed, T1, "JPX-HOL-2025"),
        // 2025-09-23 - T1 - JPX-HOL-2025 - Autumnal Equinox.
        (2025, 9, 23, Closed, T1, "JPX-HOL-2025"),
        // 2025-10-13 - T1 - JPX-HOL-2025 - Sports Day.
        (2025, 10, 13, Closed, T1, "JPX-HOL-2025"),
        // 2025-11-03 - T1 - JPX-HOL-2025 - Culture Day.
        (2025, 11, 3, Closed, T1, "JPX-HOL-2025"),
        // 2025-11-24 - T1 - JPX-HOL-2025 - Labor Thanksgiving Day (Nov. 23) observed:
        // Nov. 23 is a Sunday, so the observance is the row and Nov. 23 ships none.
        (2025, 11, 24, Closed, T1, "JPX-HOL-2025"),
        // 2025-12-31 - T1 - JPX-HOL-2025 - Market Holiday.
        (2025, 12, 31, Closed, T1, "JPX-HOL-2025"),
        // 2026-01-01 - T1 - JPX-HOL-2026-2027 - New Year's Day.
        (2026, 1, 1, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-01-02 - T1 - JPX-HOL-2026-2027 - Market Holiday. Jan. 3 is a
        // Saturday Market Holiday and ships no row.
        (2026, 1, 2, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-01-12 - T1 - JPX-HOL-2026-2027 - Coming of Age Day.
        (2026, 1, 12, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-02-11 - T1 - JPX-HOL-2026-2027 - National Foundation Day.
        (2026, 2, 11, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-02-23 - T1 - JPX-HOL-2026-2027 - Emperor's Birthday.
        (2026, 2, 23, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-03-20 - T1 - JPX-HOL-2026-2027 - Vernal Equinox.
        (2026, 3, 20, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-04-29 - T1 - JPX-HOL-2026-2027 - Showa Day.
        (2026, 4, 29, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-05-04 - T1 - JPX-HOL-2026-2027 - Greenery Day. May 3 is a Sunday
        // Constitution Memorial Day and ships no row.
        (2026, 5, 4, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-05-05 - T1 - JPX-HOL-2026-2027 - Children's Day.
        (2026, 5, 5, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-05-06 - T1 - JPX-HOL-2026-2027 - Constitution Memorial Day (May 3)
        // observed.
        (2026, 5, 6, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-07-20 - T1 - JPX-HOL-2026-2027 - Marine Day.
        (2026, 7, 20, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-08-11 - T1 - JPX-HOL-2026-2027 - Mountain Day.
        (2026, 8, 11, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-09-21 - T1 - JPX-HOL-2026-2027 - Respect for the Aged Day.
        (2026, 9, 21, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-09-22 - T1 - JPX-HOL-2026-2027 - Holiday: the page states
        // "September 22, 2026, is a holiday in accordance with Rule 3,
        // Paragraph 3 of Act on National Holidays" (the citizen's holiday
        // between two national holidays).
        (2026, 9, 22, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-09-23 - T1 - JPX-HOL-2026-2027 - Autumnal Equinox.
        (2026, 9, 23, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-10-12 - T1 - JPX-HOL-2026-2027 - Sports Day.
        (2026, 10, 12, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-11-03 - T1 - JPX-HOL-2026-2027 - Culture Day.
        (2026, 11, 3, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-11-23 - T1 - JPX-HOL-2026-2027 - Labor Thanksgiving Day.
        (2026, 11, 23, Closed, T1, "JPX-HOL-2026-2027"),
        // 2026-12-31 - T1 - JPX-HOL-2026-2027 - Market Holiday.
        (2026, 12, 31, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-01-01 - T1 - JPX-HOL-2026-2027 - New Year's Day. Jan. 2 (Saturday)
        // and Jan. 3 (Sunday) are printed Market Holidays on weekend days and
        // ship no rows.
        (2027, 1, 1, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-01-11 - T1 - JPX-HOL-2026-2027 - Coming of Age Day.
        (2027, 1, 11, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-02-11 - T1 - JPX-HOL-2026-2027 - National Foundation Day.
        (2027, 2, 11, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-02-23 - T1 - JPX-HOL-2026-2027 - Emperor's Birthday.
        (2027, 2, 23, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-03-22 - T1 - JPX-HOL-2026-2027 - Vernal Equinox (Mar. 21) observed:
        // Mar. 21 is a Sunday, so the observance is the row and Mar. 21 ships none.
        (2027, 3, 22, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-04-29 - T1 - JPX-HOL-2026-2027 - Showa Day.
        (2027, 4, 29, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-05-03 - T1 - JPX-HOL-2026-2027 - Constitution Memorial Day.
        (2027, 5, 3, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-05-04 - T1 - JPX-HOL-2026-2027 - Greenery Day.
        (2027, 5, 4, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-05-05 - T1 - JPX-HOL-2026-2027 - Children's Day.
        (2027, 5, 5, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-07-19 - T1 - JPX-HOL-2026-2027 - Marine Day.
        (2027, 7, 19, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-08-11 - T1 - JPX-HOL-2026-2027 - Mountain Day.
        (2027, 8, 11, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-09-20 - T1 - JPX-HOL-2026-2027 - Respect for the Aged Day.
        (2027, 9, 20, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-09-23 - T1 - JPX-HOL-2026-2027 - Autumnal Equinox.
        (2027, 9, 23, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-10-11 - T1 - JPX-HOL-2026-2027 - Sports Day.
        (2027, 10, 11, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-11-03 - T1 - JPX-HOL-2026-2027 - Culture Day.
        (2027, 11, 3, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-11-23 - T1 - JPX-HOL-2026-2027 - Labor Thanksgiving Day.
        (2027, 11, 23, Closed, T1, "JPX-HOL-2026-2027"),
        // 2027-12-31 - T1 - JPX-HOL-2026-2027 - Market Holiday.
        (2027, 12, 31, Closed, T1, "JPX-HOL-2026-2027"),
    ],
};
