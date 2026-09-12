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

use super::fences::early_close;
use super::{EvidenceTier::T2, HolidayKind::Closed, HolidayTable, holidays};

/// The family's built-in holiday rows and the window they were audited over.
///
/// Coverage runs to 2027-12-31, the end of the operator's published future;
/// CME's 2028-01-01 record sits outside it and ships no row.
// Evidence: docs/evidence/globex_fx.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: (2025, 1, 1) ..= (2027, 12, 31),
    rows: [
        // 2025-01-01 - T2 - CME-SVC-2025-01-01 - New Year's Day; no day session,
        // only the 16:00 CT queue and 17:00 CT open for trade date 2025-01-02.
        (2025, 1, 1, Closed, T2, "CME-SVC-2025-01-01"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-18 - Good Friday, no events published.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-18"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-04 - Independence Day, 12:00 CT close.
        (2025, 7, 4, early_close(12 * 3_600), T2, "CME-SVC-2025-07-04"),
        // 2025-11-28 - T2 - CME-SVC-2025-11-28 - day after Thanksgiving, 13:45 CT close.
        (
            2025,
            11,
            28,
            early_close(13 * 3_600 + 45 * 60),
            T2,
            "CME-SVC-2025-11-28"
        ),
        // 2025-11-29 - T2 - CME-SVC-2025-11-29 - Thanksgiving Saturday, no events.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-29"),
        // 2025-12-24 - T2 - CME-SVC-2025-12-24 - Christmas Eve, 12:45 CT close.
        (
            2025,
            12,
            24,
            early_close(12 * 3_600 + 45 * 60),
            T2,
            "CME-SVC-2025-12-24"
        ),
        // 2025-12-25 - T2 - CME-SVC-2025-12-25 - Christmas Day; no day session,
        // only the 16:00 CT queue and 17:00 CT open for trade date 2025-12-26.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-25"),
        // 2026-01-01 - T2 - CME-SVC-2026-01-01 - New Year's Day; no day session,
        // only the 16:00 CT queue and 17:00 CT open for trade date 2026-01-02.
        (2026, 1, 1, Closed, T2, "CME-SVC-2026-01-01"),
        // 2026-04-03 - T2 - CME-SVC-2026-04-03 - Good Friday, 10:15 CT close; CME's
        // own page names FX as one of the four groups that traded that morning.
        (
            2026,
            4,
            3,
            early_close(10 * 3_600 + 15 * 60),
            T2,
            "CME-SVC-2026-04-03"
        ),
        // 2026-06-19 - T2 - CME-SVC-2026-06-19 - Juneteenth, 12:00 CT close.
        (2026, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-2026-06-19"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - Independence Day observed, 12:00 CT close.
        (2026, 7, 3, early_close(12 * 3_600), T2, "CME-SVC-2026-07-03"),
        // 2026-11-27 - T2 - CME-SVC-2026-11-27 - day after Thanksgiving, 13:45 CT close.
        (
            2026,
            11,
            27,
            early_close(13 * 3_600 + 45 * 60),
            T2,
            "CME-SVC-2026-11-27"
        ),
        // 2026-12-24 - T2 - CME-SVC-2026-12-24 - Christmas Eve, 12:45 CT close.
        (
            2026,
            12,
            24,
            early_close(12 * 3_600 + 45 * 60),
            T2,
            "CME-SVC-2026-12-24"
        ),
        // 2026-12-25 - T2 - CME-SVC-2026-12-25 - Christmas Day, no events published.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-25"),
        // 2027-01-01 - T2 - CME-SVC-2027-01-01 - New Year's Day, no events published.
        (2027, 1, 1, Closed, T2, "CME-SVC-2027-01-01"),
        // 2027-03-26 - T2 - CME-SVC-2027-03-26 - Good Friday, no events published.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-26"),
        // 2027-06-18 - T2 - CME-SVC-2027-06-18 - Juneteenth observed, 12:00 CT close.
        (2027, 6, 18, early_close(12 * 3_600), T2, "CME-SVC-2027-06-18"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-26 - day after Thanksgiving, 13:45 CT close.
        (
            2027,
            11,
            26,
            early_close(13 * 3_600 + 45 * 60),
            T2,
            "CME-SVC-2027-11-26"
        ),
        // 2027-12-24 - T2 - CME-SVC-2027-12-24 - Christmas Friday closure, no events.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-24"),
    ],
};
