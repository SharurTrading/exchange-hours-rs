// SPDX-License-Identifier: MIT-0

//! CME Nikkei 225 Dollar (`NKD`) futures schedules.
//!
//! CME Globex NKD (clearing/ClearPort `NK`, BTIC `NKT`, CME product id 168,
//! CME Rulebook Chapter 352) is quoted by CME in both Eastern and Central time;
//! CME's trading-hours page states that hours are U.S. Central unless otherwise
//! noted, so Central is the zone modelled here. `US::Central` is the IANA link
//! for `America/Chicago` and is the zone constant every other CME/CBOT module
//! in this crate already uses.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, SUN_PLUS_MON_THU};
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

// CME publishes no normal-week pre-open or order-entry start time for NKD, and
// BTIC (`NKT`) is separately scheduled on its own published hours, so no
// extended phase is asserted. The 16:00-17:00 CT daily break is a maintenance
// period, not an order-entry phase. See docs/evidence/globex_nikkei_225_dollar.md.
pub(crate) static NKD_EXTENDED_CURRENT: &[SessionRule] = &[];

pub(crate) static NKD_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: NKD_REGULAR_CURRENT,
    extended: NKD_EXTENDED_CURRENT,
    order_entry: &[],
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

// THE 2011 GRID, AND WHY IT IS NOT CARRIED TO THE FLOOR. The 2010 grid was
// materially different — daytime-anchored, DST-dependent, with no Sunday
// session at all in CST — and its changeover day is undated, so neither grid may
// be carried across it and dates below 2011-01-12 are modelled sessionless. See
// docs/evidence/globex_nikkei_225_dollar.md.
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

/// Sessionless profile for dates before the first sourced appearance of the
/// 2011 grid. The 2010 grid is sourced but structurally different and its
/// changeover day is undated; see the note above.
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
    // 2011-01-12 — T1 — first sourced CME trading-hours capture of this grid —
    // knowledge boundary: 17:00-15:15 CT with the 15:30-16:30 CT post-halt
    // segment. Dates below this row resolve to the sessionless profile.
    (
        2011,
        1,
        12,
        &NKD_2011,
        "first sourced CME trading-hours capture of this grid"
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
/// Dates before 2011-01-12 — the first sourced appearance of the 2011 grid —
/// resolve to a sessionless profile. The 2010 grid was materially different and
/// its changeover day is undated, so neither grid may be carried across it.
pub(crate) fn nkd_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &NKD_CLOSED, NKD_REVISIONS)
}
