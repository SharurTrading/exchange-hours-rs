// SPDX-License-Identifier: MIT-0

//! CME standard-grid FX futures holiday rows, 2025-2027.
//!
//! Keyed by the crate's own venue-local trade date in `America/Chicago`
//! (design memo D1). The conversion is **not** the identity for this family:
//! the grid is a single wrapping Sunday-to-Thursday 17:00→16:00 CT matching
//! block, so every trading day opens on the previous local evening and CME's
//! event-date records have to be read against the trade date CME itself prints
//! beside them. An eve record is evidence for the holiday's own row, never a
//! row of its own, unless the eve carries an early close of its *own* trade
//! date — which is exactly what Christmas Eve does.
//!
//! The rows come from CME's trading-hours service — the endpoint
//! `cmegroup.com/trading-hours.html` itself calls — read as bytes and saved,
//! so the whole block is **T2** under LAW-PRIMARY-SOURCES. CME publishes no T1
//! per-asset-class rendering for these years; that, the eight 2025 windows that
//! survive only in a pre-holiday capture, the six Saturday and holiday spans
//! whose topology the scalar vocabulary cannot state, and the sixteen dates on
//! which CME merges the holiday into the next business day's trade date are
//! recorded as gaps in
//! [`docs/evidence/globex_fx.md`](../../../../../docs/evidence/globex_fx.md).
//!
//! Two shapes only: `Closed` on a full Globex closure, and `EarlyClose` on the
//! half-days CME publishes for the family. There is **no** late open in this
//! window — CME never reopens this family later than its normal 17:00 CT — and
//! no `Unsourced` row: every date inside the coverage window is answered by the
//! operator's own channel.
//!
//! # What does not ship a row
//!
//! On Monday and Thursday holidays CME publishes `16:00 preopen; 17:00 open`
//! for this family instead of the normal `16:00 closed; 16:45 preopen;
//! 17:00 open`. Matching still stops at 16:00 CT and still resumes at 17:00 CT,
//! so **no executable phase moves**; what changes is that the holiday has no
//! final close of its own and the whole span carries the next business day's
//! trade date, and that the queue opens 45 minutes early. Neither is
//! representable by the scalar vocabulary this table shares with
//! [`DayPolicy`](crate::DayPolicy), so both are declared gaps rather than rows
//! — the design memo's §1.6 triage, applied to its own §1.1 worked example.

use super::fences::{early_close, late_open};
use super::{
    EvidenceTier::{T1, T2},
    HolidayKind::Closed,
    HolidayTable, holidays,
};

/// The family's built-in holiday rows and the window they were audited over.
///
/// Coverage runs to 2027-12-31, the end of the operator's published future;
/// CME's 2028-01-01 record sits outside it and ships no row.
// Evidence: docs/evidence/globex_fx.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: (2010, 1, 1) ..= (2027, 12, 31),
    rows: [
        // --- 2010-2012, CME Group holiday calendars, tier T1 ---
        // 2010-01-15 - T1 - 2010-martin-luther-king.pdf - early close.
        (2010, 1, 15, early_close(15 * 3_600 + 15 * 60), T1, "2010-martin-luther-king.pdf @2010-03-31T06:42:26Z"),
        // 2010-01-18 - T1 - 2010-martin-luther-king.pdf - early close.
        (2010, 1, 18, early_close(12 * 3_600), T1, "2010-martin-luther-king.pdf @2010-03-31T06:42:26Z"),
        // 2010-02-12 - T1 - 2010-presidents-day.pdf - early close.
        (2010, 2, 12, early_close(15 * 3_600 + 15 * 60), T1, "2010-presidents-day.pdf @2010-02-15T06:46:41Z"),
        // 2010-02-15 - T1 - 2010-presidents-day.pdf - early close.
        (2010, 2, 15, early_close(12 * 3_600), T1, "2010-presidents-day.pdf @2010-02-15T06:46:41Z"),
        // 2010-04-02 - T1 - 2010-good-friday.pdf - early close.
        (2010, 4, 2, early_close(10 * 3_600 + 15 * 60), T1, "2010-good-friday.pdf @2010-06-01T11:19:16Z"),
        // 2010-05-28 - T1 - 2010-memorial-day.pdf - early close.
        (2010, 5, 28, early_close(15 * 3_600 + 15 * 60), T1, "2010-memorial-day.pdf @2010-06-01T09:42:25Z"),
        // 2010-05-31 - T1 - 2010-memorial-day.pdf - early close.
        (2010, 5, 31, early_close(12 * 3_600), T1, "2010-memorial-day.pdf @2010-06-01T09:42:25Z"),
        // 2010-07-02 - T1 - 2010-4th-of-july.pdf - early close.
        (2010, 7, 2, early_close(15 * 3_600 + 15 * 60), T1, "2010-4th-of-july.pdf @2010-06-02T00:56:37Z"),
        // 2010-07-05 - T1 - 2010-4th-of-july.pdf - early close.
        (2010, 7, 5, early_close(12 * 3_600), T1, "2010-4th-of-july.pdf @2010-06-02T00:56:37Z"),
        // 2010-09-03 - T1 - 2010-labor-day.pdf - early close.
        (2010, 9, 3, early_close(15 * 3_600 + 15 * 60), T1, "2010-labor-day.pdf @2010-06-02T00:56:41Z"),
        // 2010-09-06 - T1 - 2010-labor-day.pdf - early close.
        (2010, 9, 6, early_close(12 * 3_600), T1, "2010-labor-day.pdf @2010-06-02T00:56:41Z"),
        // 2010-10-08 - T1 - 2010-columbus-day.pdf - early close.
        (2010, 10, 8, early_close(15 * 3_600 + 15 * 60), T1, "2010-columbus-day.pdf @2010-08-21T13:31:22Z"),
        // 2010-11-25 - T1 - 2010-thanksgiving.pdf - early close.
        (2010, 11, 25, early_close(12 * 3_600), T1, "2010-thanksgiving.pdf @2010-11-22T09:40:12Z"),
        // 2010-11-26 - T1 - 2010-thanksgiving.pdf - early close.
        (2010, 11, 26, early_close(12 * 3_600 + 15 * 60), T1, "2010-thanksgiving.pdf @2010-11-22T09:40:12Z"),
        // 2010-12-31 - T1 - 2011-new-years.pdf - early close.
        (2010, 12, 31, early_close(12 * 3_600 + 15 * 60), T1, "2011-new-years.pdf @2011-11-01T14:39:45Z"),
        // 2011-01-14 - T1 - 2011-martin-luther-king.pdf - early close.
        (2011, 1, 14, early_close(15 * 3_600 + 15 * 60), T1, "2011-martin-luther-king.pdf @2011-10-28T02:34:29Z"),
        // 2011-01-17 - T1 - 2011-martin-luther-king.pdf - early close.
        (2011, 1, 17, early_close(12 * 3_600), T1, "2011-martin-luther-king.pdf @2011-10-28T02:34:29Z"),
        // 2011-02-18 - T1 - 2011-presidents-day.pdf - early close.
        (2011, 2, 18, early_close(15 * 3_600 + 15 * 60), T1, "2011-presidents-day.pdf @2011-10-28T02:35:16Z"),
        // 2011-02-21 - T1 - 2011-presidents-day.pdf - early close.
        (2011, 2, 21, early_close(12 * 3_600), T1, "2011-presidents-day.pdf @2011-10-28T02:35:16Z"),
        // 2011-05-27 - T1 - 2011-memorial-day.pdf - early close.
        (2011, 5, 27, early_close(15 * 3_600 + 15 * 60), T1, "2011-memorial-day.pdf @2013-09-30T10:56:52Z"),
        // 2011-05-30 - T1 - 2011-memorial-day.pdf - early close.
        (2011, 5, 30, early_close(12 * 3_600), T1, "2011-memorial-day.pdf @2013-09-30T10:56:52Z"),
        // 2011-07-01 - T1 - 2011-4th-of-july.pdf - early close.
        (2011, 7, 1, early_close(15 * 3_600 + 15 * 60), T1, "2011-4th-of-july.pdf @2011-11-01T14:40:54Z"),
        // 2011-07-04 - T1 - 2011-4th-of-july.pdf - early close.
        (2011, 7, 4, early_close(12 * 3_600), T1, "2011-4th-of-july.pdf @2011-11-01T14:40:54Z"),
        // 2011-09-02 - T1 - 2011-labor-day.pdf - early close.
        (2011, 9, 2, early_close(15 * 3_600 + 15 * 60), T1, "2011-labor-day.pdf @2011-11-01T14:43:45Z"),
        // 2011-09-05 - T1 - 2011-labor-day.pdf - early close.
        (2011, 9, 5, early_close(12 * 3_600), T1, "2011-labor-day.pdf @2011-11-01T14:43:45Z"),
        // 2011-10-07 - T1 - 2011-columbus-day.pdf - early close.
        (2011, 10, 7, early_close(15 * 3_600 + 15 * 60), T1, "2011-columbus-day.pdf @2011-11-01T14:39:16Z"),
        // 2011-11-24 - T1 - 2011-thanksgiving.pdf - early close.
        (2011, 11, 24, early_close(12 * 3_600), T1, "2011-thanksgiving.pdf @2011-11-24T18:52:46Z"),
        // 2011-11-25 - T1 - 2011-thanksgiving.pdf - early close.
        (2011, 11, 25, early_close(12 * 3_600 + 15 * 60), T1, "2011-thanksgiving.pdf @2011-11-24T18:52:46Z"),
        // 2011-12-27 - T1 - 2011-christmas.pdf - late open.
        (2011, 12, 27, late_open(5 * 3_600), T1, "2011-christmas.pdf @2012-01-25T02:05:48Z"),
        // 2012-01-03 - T1 - 2012-new-years.pdf - late open.
        (2012, 1, 3, late_open(5 * 3_600), T1, "2012-new-years.pdf @2012-01-25T02:54:30Z"),
        // 2012-01-13 - T1 - 2012-martin-luther-king.pdf - early close.
        (2012, 1, 13, early_close(15 * 3_600 + 15 * 60), T1, "2012-martin-luther-king.pdf @2012-05-05T16:15:26Z"),
        // 2012-01-16 - T1 - 2012-martin-luther-king.pdf - early close.
        (2012, 1, 16, early_close(12 * 3_600), T1, "2012-martin-luther-king.pdf @2012-05-05T16:15:26Z"),
        // 2012-02-17 - T1 - 2012-presidents-day.pdf - early close.
        (2012, 2, 17, early_close(15 * 3_600 + 15 * 60), T1, "2012-presidents-day.pdf @2012-05-05T16:15:39Z"),
        // 2012-02-20 - T1 - 2012-presidents-day.pdf - early close.
        (2012, 2, 20, early_close(12 * 3_600), T1, "2012-presidents-day.pdf @2012-05-05T16:15:39Z"),
        // 2012-04-06 - T1 - 2012-good-friday.pdf - early close.
        (2012, 4, 6, early_close(10 * 3_600 + 15 * 60), T1, "2012-good-friday.pdf @2012-05-05T16:16:49Z"),
        // 2012-05-25 - T1 - 2012-memorial-day.pdf - early close.
        (2012, 5, 25, early_close(15 * 3_600 + 15 * 60), T1, "2012-memorial-day.pdf @2012-09-15T00:37:14Z"),
        // 2012-05-28 - T1 - 2012-memorial-day.pdf - early close.
        (2012, 5, 28, early_close(12 * 3_600), T1, "2012-memorial-day.pdf @2012-09-15T00:37:14Z"),
        // 2012-07-04 - T1 - 2012-4th-of-july.pdf - early close.
        (2012, 7, 4, early_close(12 * 3_600), T1, "2012-4th-of-july.pdf @2012-09-15T00:39:23Z"),
        // 2012-08-31 - T1 - 2012-labor-day.pdf - early close.
        (2012, 8, 31, early_close(15 * 3_600 + 15 * 60), T1, "2012-labor-day.pdf @2012-09-15T00:34:37Z"),
        // 2012-09-03 - T1 - 2012-labor-day.pdf - early close.
        (2012, 9, 3, early_close(12 * 3_600), T1, "2012-labor-day.pdf @2012-09-15T00:34:37Z"),
        // 2012-10-05 - T1 - 2012-columbus-day.pdf - early close.
        (2012, 10, 5, early_close(15 * 3_600 + 15 * 60), T1, "2012-columbus-day.pdf @2012-09-15T00:15:14Z"),
        // 2012-11-22 - T1 - 2012-thanksgiving.pdf - early close.
        (2012, 11, 22, early_close(12 * 3_600), T1, "2012-thanksgiving.pdf @2013-01-27T22:39:01Z"),
        // 2012-11-23 - T1 - 2012-thanksgiving.pdf - early close.
        (2012, 11, 23, early_close(12 * 3_600 + 15 * 60), T1, "2012-thanksgiving.pdf @2013-01-27T22:39:01Z"),
        // 2012-12-24 - T1 - 2012-christmas.pdf - early close.
        (2012, 12, 24, early_close(12 * 3_600 + 15 * 60), T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        // 2012-12-26 - T1 - 2012-christmas.pdf - late open.
        (2012, 12, 26, late_open(5 * 3_600), T1, "2012-christmas.pdf @2013-04-14T19:40:27Z"),
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-17 - Good Friday, no events published.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-03 - Independence Day, 12:00 CT close.
        (2025, 7, 4, early_close(12 * 3_600), T2, "CME-SVC-2025-07-03"),
        // 2025-11-28 - T2 - CME-SVC-2025-11-26 - day after Thanksgiving, 13:45 CT close.
        (
            2025,
            11,
            28,
            early_close(13 * 3_600 + 45 * 60),
            T2,
            "CME-SVC-2025-11-26"
        ),
        // 2025-11-29 - T2 - CME-SVC-2025-11-26-SAT - Thanksgiving Saturday, no events.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-12-24 - T2 - CME-SVC-2025-12-24 - Christmas Eve, 12:45 CT close.
        (
            2025,
            12,
            24,
            early_close(12 * 3_600 + 45 * 60),
            T2,
            "CME-SVC-2025-12-24"
        ),
        // 2025-12-25 - T2 - CME-SVC-2025-12-24 - Christmas Day; no day session,
        // only the 16:00 CT queue and 17:00 CT open for trade date 2025-12-26.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-24"),
        // 2026-01-01 - T2 - CME-SVC-2025-12-31 - New Year's Day; no day session,
        // only the 16:00 CT queue and 17:00 CT open for trade date 2026-01-02.
        (2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-04-03 - T2 - CME-SVC-2026-04-01 - Good Friday, 10:15 CT close; CME's
        // own page names FX as one of the four groups that traded that morning.
        (
            2026,
            4,
            3,
            early_close(10 * 3_600 + 15 * 60),
            T2,
            "CME-SVC-2026-04-01"
        ),
        // 2026-06-19 - T2 - CME-SVC-2026-06-18 - Juneteenth, 12:00 CT close.
        (2026, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - Independence Day observed, 12:00 CT close.
        (2026, 7, 3, early_close(12 * 3_600), T2, "CME-SVC-2026-07-03"),
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - day after Thanksgiving, 13:45 CT close.
        (
            2026,
            11,
            27,
            early_close(13 * 3_600 + 45 * 60),
            T2,
            "CME-SVC-2026-11-25"
        ),
        // 2026-12-24 - T2 - CME-SVC-2026-12-22 - Christmas Eve, 12:45 CT close.
        (
            2026,
            12,
            24,
            early_close(12 * 3_600 + 45 * 60),
            T2,
            "CME-SVC-2026-12-22"
        ),
        // 2026-12-25 - T2 - CME-SVC-2026-12-24 - Christmas Day, no events published.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 - T2 - CME-SVC-2026-12-31 - New Year's Day, no events published.
        (2027, 1, 1, Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-03-26 - T2 - CME-SVC-2027-03-25 - Good Friday, no events published.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-06-18 - T2 - CME-SVC-2027-06-17 - Juneteenth observed, 12:00 CT close.
        (2027, 6, 18, early_close(12 * 3_600), T2, "CME-SVC-2027-06-17"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - day after Thanksgiving, 13:45 CT close.
        (
            2027,
            11,
            26,
            early_close(13 * 3_600 + 45 * 60),
            T2,
            "CME-SVC-2027-11-24"
        ),
        // 2027-12-24 - T2 - CME-SVC-2027-12-22 - Christmas Friday closure, no events.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
