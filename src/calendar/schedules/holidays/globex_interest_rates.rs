// SPDX-License-Identifier: MIT-0

//! Built-in holiday rows for `globex_interest_rates` (LAW-HOLIDAY-SCOPE).
//!
//! Every row is keyed by the crate's own venue-local trade date in
//! `America/Chicago`, converted from the operator's event date: CME publishes
//! a Monday holiday's noon halt on the Monday, and the trading day it shortens
//! is the one that opened at 17:00 CT the previous evening, so the row lands on
//! the trade date the clip has to be stated on.
//!
//! The rows are the CME Group trading-hours service's Interest Rates line —
//! the ZN 10-Year T-Note schedule CME itself uses to render that group — at
//! tier T2. Quotations, document URLs, capture timestamps, the event-date to
//! trade-date conversion and the declared gaps live in
//! `docs/evidence/globex_interest_rates.md`.
//!
//! The family's 2025-2027 grid is one wrapping leg per trade date, Sunday to
//! Thursday 17:00 CT into a 16:00 CT close the next local day, so a full
//! closure deletes the previous evening's wrap and an early close clips a
//! session that opened the evening before. Both follow from the trade-date
//! key; neither needs a mechanism of its own.

use super::EvidenceTier::T2;
use super::HolidayKind::Closed;
use super::fences::early_close;
use super::{HolidayTable, holidays};

// Evidence: docs/evidence/globex_interest_rates.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: (2025, 1, 1) ..= (2027, 12, 31),
    rows: [
        // 2025-01-01 — T2 — CME-SVC-2024-12-31 — New Year's Day: only a 16:00 CT
        // pre-open and a 17:00 CT open, both carrying trade date 2025-01-02.
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 — T2 — CME-SVC-2025-01-19 — Martin Luther King Jr. Day:
        // matching halts 12:00 CT.
        (2025, 1, 20, early_close(12 * 3_600), T2, "CME-SVC-2025-01-19"),
        // 2025-02-17 — T2 — CME-SVC-2025-02-16 — Presidents' Day: 12:00 CT.
        (2025, 2, 17, early_close(12 * 3_600), T2, "CME-SVC-2025-02-16"),
        // 2025-04-18 — T2 — CME-SVC-2025-04-17 — Good Friday: no events published.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 — T2 — CME-SVC-2025-05-25 — Memorial Day: 12:00 CT.
        (2025, 5, 26, early_close(12 * 3_600), T2, "CME-SVC-2025-05-25"),
        // 2025-06-19 — T2 — CME-SVC-2025-06-18 — Juneteenth: 12:00 CT.
        (2025, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-2025-06-18"),
        // 2025-07-04 — T2 — CME-SVC-2025-07-03 — Independence Day: 12:00 CT final
        // close on its own trade date.
        (2025, 7, 4, early_close(12 * 3_600), T2, "CME-SVC-2025-07-03"),
        // 2025-09-01 — T2 — CME-SVC-2025-08-31 — Labor Day: 12:00 CT.
        (2025, 9, 1, early_close(12 * 3_600), T2, "CME-SVC-2025-08-31"),
        // 2025-11-27 — T2 — CME-SVC-2025-11-26 — Thanksgiving: 12:00 CT.
        (2025, 11, 27, early_close(12 * 3_600), T2, "CME-SVC-2025-11-26"),
        // 2025-11-28 — T2 — CME-SVC-2025-11-26 — day after Thanksgiving: 12:15 CT
        // final close on its own trade date.
        (2025, 11, 28, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-2025-11-26"),
        // 2025-11-29 — T2 — CME-SVC-2025-11-26-SAT — Thanksgiving Saturday: no events
        // published, and the normal week has no Saturday session either.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-12-24 — T2 — CME-SVC-2025-12-24 — Christmas Eve: 12:15 CT final
        // close, and no evening re-open because 2025-12-25 is closed.
        (2025, 12, 24, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-2025-12-24"),
        // 2025-12-25 — T2 — CME-SVC-2025-12-24 — Christmas Day: only a 16:00 CT
        // pre-open and a 17:00 CT open, both carrying trade date 2025-12-26.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-24"),
        // 2026-01-01 — T2 — CME-SVC-2025-12-31 — New Year's Day: only a 16:00 CT
        // pre-open and a 17:00 CT open, both carrying trade date 2026-01-02.
        (2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-01-19 — T2 — CME-SVC-2026-01-18 — Martin Luther King Jr. Day: 12:00 CT.
        (2026, 1, 19, early_close(12 * 3_600), T2, "CME-SVC-2026-01-18"),
        // 2026-02-16 — T2 — CME-SVC-2026-02-15 — Presidents' Day: 12:00 CT.
        (2026, 2, 16, early_close(12 * 3_600), T2, "CME-SVC-2026-02-15"),
        // 2026-04-03 — T2 — CME-SVC-2026-04-01 — Good Friday, the year CME keeps
        // rates trading for the employment release: 10:15 CT final close.
        (2026, 4, 3, early_close(10 * 3_600 + 15 * 60), T2, "CME-SVC-2026-04-01"),
        // 2026-05-25 — T2 — CME-SVC-2026-05-24 — Memorial Day: 12:00 CT.
        (2026, 5, 25, early_close(12 * 3_600), T2, "CME-SVC-2026-05-24"),
        // 2026-06-19 — T2 — CME-SVC-2026-06-18 — Juneteenth: 12:00 CT final close.
        (2026, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 — T2 — CME-SVC-2026-07-03 — Independence Day observed: 12:00 CT
        // final close.
        (2026, 7, 3, early_close(12 * 3_600), T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 — T2 — CME-SVC-2026-09-06 — Labor Day: 12:00 CT.
        (2026, 9, 7, early_close(12 * 3_600), T2, "CME-SVC-2026-09-06"),
        // 2026-11-26 — T2 — CME-SVC-2026-11-25 — Thanksgiving: 12:00 CT.
        (2026, 11, 26, early_close(12 * 3_600), T2, "CME-SVC-2026-11-25"),
        // 2026-11-27 — T2 — CME-SVC-2026-11-25 — day after Thanksgiving: 12:15 CT.
        (2026, 11, 27, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-2026-11-25"),
        // 2026-12-24 — T2 — CME-SVC-2026-12-22 — Christmas Eve: 12:15 CT final
        // close, and no evening re-open because 2026-12-25 is closed.
        (2026, 12, 24, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-2026-12-22"),
        // 2026-12-25 — T2 — CME-SVC-2026-12-24 — Christmas Day: no events published.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 — T2 — CME-SVC-2026-12-31 — New Year's Day: no events published.
        (2027, 1, 1, Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-01-18 — T2 — CME-SVC-2027-01-17 — Martin Luther King Jr. Day: 12:00 CT.
        (2027, 1, 18, early_close(12 * 3_600), T2, "CME-SVC-2027-01-17"),
        // 2027-02-15 — T2 — CME-SVC-2027-02-14 — Presidents' Day: 12:00 CT.
        (2027, 2, 15, early_close(12 * 3_600), T2, "CME-SVC-2027-02-14"),
        // 2027-03-26 — T2 — CME-SVC-2027-03-25 — Good Friday: no events published.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 — T2 — CME-SVC-2027-05-30 — Memorial Day: 12:00 CT.
        (2027, 5, 31, early_close(12 * 3_600), T2, "CME-SVC-2027-05-30"),
        // 2027-06-18 — T2 — CME-SVC-2027-06-17 — Juneteenth observed: 12:00 CT
        // final close.
        (2027, 6, 18, early_close(12 * 3_600), T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 — T2 — CME-SVC-2027-07-04 — Independence Day observed:
        // matching halts 13:30 CT.
        (2027, 7, 5, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-07-04"),
        // 2027-09-06 — T2 — CME-SVC-2027-09-05 — Labor Day: 12:00 CT.
        (2027, 9, 6, early_close(12 * 3_600), T2, "CME-SVC-2027-09-05"),
        // 2027-11-25 — T2 — CME-SVC-2027-11-24 — Thanksgiving: 12:00 CT.
        (2027, 11, 25, early_close(12 * 3_600), T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 — T2 — CME-SVC-2027-11-24 — day after Thanksgiving: 12:15 CT.
        (2027, 11, 26, early_close(12 * 3_600 + 15 * 60), T2, "CME-SVC-2027-11-24"),
        // 2027-12-24 — T2 — CME-SVC-2027-12-22 — Globex closed for the Christmas
        // holiday: no events published.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
