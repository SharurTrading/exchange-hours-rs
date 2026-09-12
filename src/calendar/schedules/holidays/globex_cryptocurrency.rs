// SPDX-License-Identifier: MIT-0

//! CME cryptocurrency holiday rows, venue-local trade dates 2025-01-01 to
//! 2027-12-31 (LAW-HOLIDAY-SCOPE).
//!
//! Every row is keyed by the crate's own America/Chicago trade date, never by
//! CME's event date: the operator publishes a holiday as an event list on a
//! civil day and prints the trade date each event carries, and it is that
//! printed trade date the rows below are built from.
//!
//! The family's [`HolidayKind::Closed`] rows mean "there is no such trade
//! date", not "trading stopped". Which of the two consequences follows is a
//! property of the era, not of the row:
//!
//! - in the five-day 17:00-16:00 CT era, which closes over the weekend, a
//!   closed trade date deletes its complete trading day including the session
//!   that opened the previous evening;
//! - in the 24/7 era from trade date 2026-05-30, where the family assigns a
//!   block to the following open business date, a closed trade date is skipped
//!   by that roll and the connected block survives, carrying the next business
//!   date instead. The operator's own data says exactly this: on a holiday its
//!   16:00 CT final close is either omitted or printed with the following
//!   Monday's trade date.
//!
//! Quotations, capture times, the event-date-to-trade-date conversion and the
//! declared gaps: `docs/evidence/globex_cryptocurrency.md`.

use super::fences::early_close;
use super::{EvidenceTier::T2, HolidayKind, HolidayTable, holidays};

/// The family's audited trade-date window and its rows.
///
/// Source: CME's own trading-hours service, the channel the operator's
/// trading-hours page calls to render its per-asset-class holiday table, read
/// as bytes and saved — T2 under LAW-PRIMARY-SOURCES. No T1 per-asset-class
/// rendering exists for these years; that is a declared gap, not a reason to
/// withhold the rows.
// Evidence: docs/evidence/globex_cryptocurrency.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: (2025, 1, 1) ..= (2027, 12, 31),
    rows: [
        // 2025-01-01 — T2 — CME-SVC-2025-01-01 — New Year's Day; only a 16:00 CT
        // pre-open and a 17:00 CT open, both carrying trade date 2025-01-02.
        (2025, 1, 1, HolidayKind::Closed, T2, "CME-SVC-2025-01-01"),
        // 2025-01-20 — T2 — CME-SVC-2025-01-20 — Martin Luther King Jr. Day; the
        // whole Sunday-evening span carries trade date 2025-01-21.
        (2025, 1, 20, HolidayKind::Closed, T2, "CME-SVC-2025-01-20"),
        // 2025-02-17 — T2 — CME-SVC-2025-02-17 — Presidents' Day; trade date
        // 2025-02-18 throughout.
        (2025, 2, 17, HolidayKind::Closed, T2, "CME-SVC-2025-02-17"),
        // 2025-04-18 — T2 — CME-SVC-2025-04-18 — Good Friday; no events published.
        (2025, 4, 18, HolidayKind::Closed, T2, "CME-SVC-2025-04-18"),
        // 2025-05-26 — T2 — CME-SVC-2025-05-26 — Memorial Day; trade date
        // 2025-05-27 throughout.
        (2025, 5, 26, HolidayKind::Closed, T2, "CME-SVC-2025-05-26"),
        // 2025-06-19 — T2 — CME-SVC-2025-06-19 — Juneteenth; trade date 2025-06-20
        // throughout.
        (2025, 6, 19, HolidayKind::Closed, T2, "CME-SVC-2025-06-19"),
        // 2025-07-04 — T2 — CME-SVC-2025-07-04 — Independence Day; 12:00 CT final
        // close on its own trade date.
        (2025, 7, 4, early_close(12 * 3_600), T2, "CME-SVC-2025-07-04"),
        // 2025-09-01 — T2 — CME-SVC-2025-09-01 — Labor Day; trade date 2025-09-02
        // throughout.
        (2025, 9, 1, HolidayKind::Closed, T2, "CME-SVC-2025-09-01"),
        // 2025-11-27 — T2 — CME-SVC-2025-11-27 — Thanksgiving; trade date
        // 2025-11-28 throughout.
        (2025, 11, 27, HolidayKind::Closed, T2, "CME-SVC-2025-11-27"),
        // 2025-11-28 — T2 — CME-SVC-2025-11-27 — day after Thanksgiving; 13:45 CT
        // final close on its own trade date.
        (
            2025,
            11,
            28,
            early_close(13 * 3_600 + 45 * 60),
            T2,
            "CME-SVC-2025-11-27"
        ),
        // 2025-12-24 — T2 — CME-SVC-2025-12-25 — Christmas Eve; 12:45 CT final
        // close on its own trade date and no evening re-open.
        (
            2025,
            12,
            24,
            early_close(12 * 3_600 + 45 * 60),
            T2,
            "CME-SVC-2025-12-25"
        ),
        // 2025-12-25 — T2 — CME-SVC-2025-12-25 — Christmas Day; only a 16:00 CT
        // pre-open and a 17:00 CT open, both carrying trade date 2025-12-26.
        (2025, 12, 25, HolidayKind::Closed, T2, "CME-SVC-2025-12-25"),
        // 2026-01-01 — T2 — CME-SVC-2026-01-01 — New Year's Day; trade date
        // 2026-01-02 throughout.
        (2026, 1, 1, HolidayKind::Closed, T2, "CME-SVC-2026-01-01"),
        // 2026-01-19 — T2 — CME-SVC-2026-01-19 — Martin Luther King Jr. Day; trade
        // date 2026-01-20 throughout.
        (2026, 1, 19, HolidayKind::Closed, T2, "CME-SVC-2026-01-19"),
        // 2026-02-16 — T2 — CME-SVC-2026-02-16 — Presidents' Day; trade date
        // 2026-02-17 throughout.
        (2026, 2, 16, HolidayKind::Closed, T2, "CME-SVC-2026-02-16"),
        // 2026-04-03 — T2 — CME-SVC-2026-04-03 — Good Friday, the employment-report
        // exception; 10:15 CT final close on its own trade date.
        (
            2026,
            4,
            3,
            early_close(10 * 3_600 + 15 * 60),
            T2,
            "CME-SVC-2026-04-03"
        ),
        // 2026-05-25 — T2 — CME-SVC-2026-05-25 — Memorial Day, the last holiday of
        // the five-day era; trade date 2026-05-26 throughout.
        (2026, 5, 25, HolidayKind::Closed, T2, "CME-SVC-2026-05-25"),
        // 2026-06-19 — T2 — CME-SVC-2026-06-19 — Juneteenth, first of the 24/7-era
        // Friday holidays; the 16:00 CT close carries trade date 2026-06-22.
        (2026, 6, 19, HolidayKind::Closed, T2, "CME-SVC-2026-06-19"),
        // 2026-07-03 — T2 — CME-SVC-2026-07-03 — Independence Day observed; the
        // 16:00 CT close carries trade date 2026-07-06.
        (2026, 7, 3, HolidayKind::Closed, T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 — T2 — CME-SVC-2026-09-07 — Labor Day; the 16:00 CT close is
        // omitted and the re-open carries trade date 2026-09-08.
        (2026, 9, 7, HolidayKind::Closed, T2, "CME-SVC-2026-09-07"),
        // 2026-11-26 — T2 — CME-SVC-2026-11-26 — Thanksgiving; the 16:00 CT close
        // is omitted and the re-open carries trade date 2026-11-27.
        (2026, 11, 26, HolidayKind::Closed, T2, "CME-SVC-2026-11-26"),
        // 2026-12-25 — T2 — CME-SVC-2026-12-25 — Christmas Day; the 16:00 CT close
        // carries trade date 2026-12-28.
        (2026, 12, 25, HolidayKind::Closed, T2, "CME-SVC-2026-12-25"),
        // 2027-01-01 — T2 — CME-SVC-2027-01-01 — New Year's Day; the 16:00 CT close
        // carries trade date 2027-01-04.
        (2027, 1, 1, HolidayKind::Closed, T2, "CME-SVC-2027-01-01"),
        // 2027-01-18 — T2 — CME-SVC-2027-01-18 — Martin Luther King Jr. Day; the
        // 16:00 CT close is omitted and the re-open carries trade date 2027-01-19.
        (2027, 1, 18, HolidayKind::Closed, T2, "CME-SVC-2027-01-18"),
        // 2027-02-15 — T2 — CME-SVC-2027-02-15 — Presidents' Day; the re-open
        // carries trade date 2027-02-16.
        (2027, 2, 15, HolidayKind::Closed, T2, "CME-SVC-2027-02-15"),
        // 2027-03-26 — T2 — CME-SVC-2027-03-26 — Good Friday; the 16:00 CT close
        // carries trade date 2027-03-29.
        (2027, 3, 26, HolidayKind::Closed, T2, "CME-SVC-2027-03-26"),
        // 2027-05-31 — T2 — CME-SVC-2027-05-31 — Memorial Day; the re-open carries
        // trade date 2027-06-01.
        (2027, 5, 31, HolidayKind::Closed, T2, "CME-SVC-2027-05-31"),
        // 2027-06-18 — T2 — CME-SVC-2027-06-18 — Juneteenth observed; the 16:00 CT
        // close carries trade date 2027-06-21.
        (2027, 6, 18, HolidayKind::Closed, T2, "CME-SVC-2027-06-18"),
        // 2027-07-05 — T2 — CME-SVC-2027-07-05 — Independence Day observed; the
        // re-open carries trade date 2027-07-06.
        (2027, 7, 5, HolidayKind::Closed, T2, "CME-SVC-2027-07-05"),
        // 2027-09-06 — T2 — CME-SVC-2027-09-06 — Labor Day; the re-open carries
        // trade date 2027-09-07.
        (2027, 9, 6, HolidayKind::Closed, T2, "CME-SVC-2027-09-06"),
        // 2027-11-25 — T2 — CME-SVC-2027-11-25 — Thanksgiving; the re-open carries
        // trade date 2027-11-26.
        (2027, 11, 25, HolidayKind::Closed, T2, "CME-SVC-2027-11-25"),
        // 2027-12-24 — T2 — CME-SVC-2027-12-23 — Globex closed for Christmas; the
        // 2027-12-23 re-open carries trade date 2027-12-27, skipping this date.
        (2027, 12, 24, HolidayKind::Closed, T2, "CME-SVC-2027-12-23"),
    ],
};
