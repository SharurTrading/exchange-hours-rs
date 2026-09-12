// SPDX-License-Identifier: MIT-0

//! CME FX futures on the standard 17:00-16:00 CT Globex grid.

use chrono_tz::US;

use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};
use crate::calendar::{FuturesSessionProfile, SessionRule};

// CME FX futures on the standard 17:00-16:00 CT Globex matching grid. This
// family is not a promise for eFix, BTIC, TAS, options, or any product whose
// own specification publishes a different grid.
// See docs/evidence/globex_fx.md.

// ORDER-ENTRY CLASSIFICATION. The 17:00-16:00 window is the matching grid the
// 2010 product guide publishes. Every other phase here is a Globex queue: the
// weekday "Pre-Open" the comment above names (16:50, then 16:45, to 17:00) and
// the Sunday 16:00-17:00 queue accept, amend, and cancel orders while the
// matching engine is stopped, so no trade can print until 17:00. They are
// `order_entry`.
static MATCHING_GRID: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];
// SUNDAY QUEUE, CARRIED BACK AS THE SOURCED INTERSECTION. The 16:15-17:00
// window is order-entry under every sourced state, so carrying it from the
// January-2010 floor asserts no cutover; the knowledge-bound row below adds the
// disputed 16:00-16:15 quarter-hour. See docs/evidence/globex_fx.md.
static ORDER_ENTRY_AT_2010_FLOOR: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 50 * 60,
        close_ssm: 17 * 3600,
    },
];
static ORDER_ENTRY_DATED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600,
    },
];
pub(crate) static ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600,
    },
];

pub(crate) static CURRENT_FUTURES_PROFILE: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: MATCHING_GRID,
    order_entry: ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static AT_2010_FLOOR: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: MATCHING_GRID,
    order_entry: ORDER_ENTRY_AT_2010_FLOOR,
    has_daily_close: true,
    has_weekend_close: true,
};
static DATED_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: MATCHING_GRID,
    order_entry: ORDER_ENTRY_DATED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};
// Verified-current grid: identical to DATED_CURRENT except the Sunday
// 16:00–17:00 queue, whose onset day no reviewed primary source states.
static FX_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: MATCHING_GRID,
    order_entry: ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Evidence: docs/evidence/globex_fx.md
static REVISIONS: &[Revision] = revisions![
    // 2010-11-15 — T1 — CME Globex notice 20101025 — Monday-Thursday Pre-Open
    // moves from 16:50 to 16:45 CT.
    (2010, 11, 15, &DATED_CURRENT, "CME Globex notice 20101025"),
    // 2026-08-22 — T1 — 2026-08-22 review: verified current, onset undated —
    // knowledge-bound row widening the Sunday queue by the disputed 16:00-16:15
    // CT quarter-hour, the only part depending on the undated 2012 move. A
    // sourced onset day replaces this row.
    (
        2026,
        8,
        22,
        &FX_CURRENT,
        "2026-08-22 review: verified current, onset undated"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &AT_2010_FLOOR, REVISIONS)
}
