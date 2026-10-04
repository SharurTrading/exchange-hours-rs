// SPDX-License-Identifier: MIT-0

//! CME Nikkei 225 Dollar (`NKD`) futures schedules.
//!
//! CME Globex NKD (clearing/ClearPort `NK`, BTIC `NKT`, CME product id 168,
//! CME Rulebook Chapter 352) is quoted by CME in both Eastern and Central time;
//! CME's trading-hours page states that hours are U.S. Central unless otherwise
//! noted, so Central is the zone modelled here. `US::Central` is the IANA link
//! for `America/Chicago` and is the zone constant every other CME/CBOT module
//! in this crate already uses.
//!
//! One continuous 17:00-16:00 CT envelope per trade date, and the 16:00-17:00 CT
//! break between consecutive trade dates carries the operator's Pre-Open queue:
//! `16:45 preopen` Monday-Thursday and a Sunday onset that moved from 16:15 to
//! 16:00 CT without a day-level statement, both handing over to the 17:00 CT
//! open. A queue is `order_entry`, so it accepts orders without reporting a
//! session. The disputed Sunday quarter-hour's undated move is a disclosed
//! residual beside this family's served onset (#79, retired 2026-10-04), and
//! the 2011/2012/2013 eras ship no order-entry phase; both records are in
//! [`docs/evidence/globex_nikkei_225_dollar.md`](../../../../../docs/evidence/globex_nikkei_225_dollar.md).
//!
//! Below the served grid sits the old 2010 grid, modelled from the operator's
//! own equities-hours page (captures 2009-04-06 and 2010-04-02 print it
//! identically): daytime-anchored sessions per DST regime, keyed at the Chicago
//! DST transitions because the page states the hours per regime. The 2010-04-05
//! Globex notice ends that era at the Sunday 2010-04-11 session-opening day.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// NKD outrights run one continuous Globex envelope per trade date: the session
// opens 17:00 CT on the previous calendar evening and closes 16:00 CT on the
// trade date, with a 60-minute 16:00-17:00 CT break between consecutive trade
// dates. Friday is absent from the opening-day mask because a Friday-evening
// open would belong to a Saturday trade date, which does not exist; that
// omission is what produces the Friday 16:00 CT weekly wrap.
pub(crate) static NKD_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];

// No extended phase is asserted: BTIC (`NKT`) is separately scheduled on its
// own published hours, so it is not a phase of this outright order book.
pub(crate) static NKD_EXTENDED_CURRENT: &[SessionRule] = &[];

/// The Pre-Open the operator publishes ahead of the 17:00 CT open:
/// `16:45 preopen` Monday-Thursday and `16:00 preopen` on the Sunday that opens
/// the week.
///
/// `preopen` is the operator's own event type — "Order Entry, modification, and
/// cancel are allowed. No order matching." — so the window is an order-entry
/// phase, never a session, and it stays out of `is_open`.
///
/// The phase is carried from the 2015-09-20 revision, the grid the cited
/// normal-week capture witnesses, and not into the 2011/2012/2013 eras: a later
/// observation is not carry-back, and those eras' captures state a different
/// Pre-Open onset. Both records — the pre-2015 omission and the Sunday onset's
/// undated 16:15-to-16:00 move — are in the evidence file.
///
/// Evidence: `docs/evidence/globex_nikkei_225_dollar.md`.
pub(crate) static NKD_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3_600,
        close_ssm: 17 * 3_600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3_600 + 45 * 60,
        close_ssm: 17 * 3_600,
    },
];

pub(crate) static NKD_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: NKD_REGULAR_CURRENT,
    extended: NKD_EXTENDED_CURRENT,
    order_entry: NKD_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Revisions are keyed by the local session-opening day, matching `cme_group`:
// the first close at 16:00 CT is trade date Monday 2015-09-21, whose session
// opened Sunday 2015-09-20.

// 2013-03-03 through 2015-09-19: the halt is gone, the close is still 16:15 CT.
static NKD_REGULAR_2013: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600 + 15 * 60,
}];

static NKD_2013: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: NKD_REGULAR_2013,
    extended: NKD_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// 2012-11-18 through 2013-03-02: close extended to 16:15 CT with a 15-minute
// electronic halt at 15:15-15:30 CT, so the day is two rules. The first rule
// carries the opening days (Sunday–Thursday evenings); the post-halt
// continuation runs on the closing local day of those wrapped sessions,
// which is Monday–Friday — a Sunday-afternoon instance never existed, and
// Friday's post-halt segment belongs to the Thursday-evening session.
static NKD_REGULAR_2012: &[SessionRule] = &[
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 15 * 3600 + 15 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 15 * 60,
    },
];

static NKD_2012: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: NKD_REGULAR_2012,
    extended: NKD_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

/// The old grid's standard-time weekdays, verbatim from the operator's
/// equities-hours page: `CST: 02:00-15:15; reopens 15:30-16:30; closes 16:30`.
/// Both runs open and close on their own local day — there is no evening leg —
/// and the 15:15-15:30 CT gap between them is a gap, not a rule.
///
/// Evidence: `docs/evidence/globex_nikkei_225_dollar.md`.
static NKD_REGULAR_2010_CST: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 2 * 3_600,
        close_ssm: 15 * 3_600 + 15 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3_600 + 30 * 60,
        close_ssm: 16 * 3_600 + 30 * 60,
    },
];

static NKD_2010_CST: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: NKD_REGULAR_2010_CST,
    extended: NKD_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

/// The old grid's daylight-time weekdays and Sunday, verbatim from the same
/// page: `CDT: 03:00-15:15 | reopens 15:30-16:30; closes 16:30-17:00; reopens
/// 17:00-18:00`, and the Sunday column's `CDT: Opens 17:00-18:00`. The daily
/// 16:30-17:00 CT maintenance halt is the gap between the second and third
/// rules, and the 17:00-18:00 CT tail runs on Sunday evenings too — the
/// operator prints those same instants in both columns.
///
/// Evidence: `docs/evidence/globex_nikkei_225_dollar.md`.
static NKD_REGULAR_2010_CDT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 3 * 3_600,
        close_ssm: 15 * 3_600 + 15 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3_600 + 30 * 60,
        close_ssm: 16 * 3_600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3_600,
        close_ssm: 18 * 3_600,
    },
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 17 * 3_600,
        close_ssm: 18 * 3_600,
    },
];

static NKD_2010_CDT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: NKD_REGULAR_2010_CDT,
    extended: NKD_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// THE SERVED GRID'S START IS DATED by the operator's own Globex notice of
// 2010-04-05, which expands the Nikkei 225 (Dollar) hours effective Sunday,
// April 11, 2010 to exactly this grid, so the revision is keyed to that
// Sunday session-opening day. The old grid's last session opened Friday
// 2010-04-09; Saturday 2010-04-10 carries no session on either grid.
// Evidence: docs/evidence/globex_nikkei_225_dollar.md.
static NKD_REGULAR_2011: &[SessionRule] = &[
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 15 * 3600 + 15 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 30 * 60,
    },
];

static NKD_2011: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: NKD_REGULAR_2011,
    extended: NKD_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

/// Sessionless fallback for dates before the timeline's first row. The support
/// floor is 2010-01-01 and the old grid's own row keys there, so only a query
/// below the floor reaches this profile, and the coverage layer refuses those
/// dates before the profile is asked.
static NKD_CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// Evidence: docs/evidence/globex_nikkei_225_dollar.md
pub(crate) static NKD_REVISIONS: &[Revision] = revisions![
    // 2010-01-01 — T1 — the operator's own equities-hours page, archived
    // captures 2009-04-06 and 2010-04-02 printing the same grid: the old grid's
    // standard-time weekdays `CST: 02:00-15:15; reopens 15:30-16:30; closes
    // 16:30` with `CST: No Sunday Hours`. Keyed at the support floor, the era's
    // first local opening day.
    (
        2010,
        1,
        1,
        &NKD_2010_CST,
        "CME equities-hours page (capture 2010-04-02), CST spelling"
    ),
    // 2010-03-14 — T1 — the same page's daylight-time spelling, `CDT: 03:00-
    // 15:15 | reopens 15:30-16:30; closes 16:30-17:00; reopens 17:00-18:00`
    // with the Sunday column's `CDT: Opens 17:00-18:00`, printed beside the CST
    // spelling on every capture; which spelling governs a date is the Chicago
    // DST calendar's question, so the row keys the regime's first local
    // opening day, the Sunday DST began.
    (
        2010,
        3,
        14,
        &NKD_2010_CDT,
        "CME equities-hours page (capture 2010-04-02), CDT spelling"
    ),
    // 2010-04-11 — T1 — CME Globex notice 20100405 — the Nikkei 225 (Dollar)
    // expanded hours take effect this Sunday: 17:00 CT Sunday open to 15:15 CT,
    // Monday-Friday 15:30 CT to 15:15 CT the next day with a 16:30-17:00 CT
    // maintenance shutdown. The 2011-01-12 trading-hours capture corroborates
    // the same grid. Dates below this row answer from the old-grid profiles.
    (
        2010,
        4,
        11,
        &NKD_2011,
        "CME Globex notice 20100405 (Nikkei 225 Dollar expanded hours)"
    ),
    // 2012-11-18 — T1 — CME SER-6465 — the close is extended to 16:15 CT with a
    // 15:15-15:30 CT electronic halt.
    (2012, 11, 18, &NKD_2012, "CME SER-6465"),
    // 2013-03-03 — T1 — CME SER-6554R — the 15:15-15:30 CT halt is removed for
    // International Equity Index futures, naming NKD explicitly.
    (2013, 3, 3, &NKD_2013, "CME SER-6554R"),
    // 2015-09-20 — T1 — CME Globex notice 20150817 — the CME Equity close moves
    // to 16:00 CT for trade date Monday 2015-09-21.
    (2015, 9, 20, &NKD_CURRENT, "CME Globex notice 20150817"),
];

/// Selects the CME Nikkei 225 Dollar profile in force on `as_of`'s Chicago day.
///
/// Dates from the 2010-01-01 floor through 2010-04-10 answer from the old
/// daytime-anchored grid the operator's equities-hours page states, split at
/// the Chicago DST transitions into its standard-time and daylight-time
/// spellings; from 2010-04-11 — the day the operator's own Globex notice of
/// 2010-04-05 dates the served grid's first Sunday session — the served grid
/// answers.
pub(crate) fn nkd_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &NKD_CLOSED, NKD_REVISIONS)
}
