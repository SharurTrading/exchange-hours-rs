// SPDX-License-Identifier: MIT-0

//! ICE Futures U.S. NYSE FANG+ Index futures schedules.
//!
//! Narrative evidence, sources and residual risks: `docs/evidence/iceus.md` and
//! `docs/evidence/ice_us.md` (LAW-EVIDENCE-FILES).

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY, TUE_ONLY};
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};
use crate::calendar::schedules::{CLOSED_NEW_YORK, StaticHoursProfile};

// The `iceus` default is the NYSE FANG+ Index futures family, not a venue-wide
// clock: 20:00-18:00 ET hours with an exceptional Sunday 18:00 open. Equal
// `SessionRule` endpoints encode one complete local-day span, so that Sunday
// session remains continuous through Monday 18:00.
pub(crate) static ICE_US_FANG_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 18 * 3600,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 20 * 3600,
        close_ssm: 18 * 3600,
    },
];
// ORDER ENTRY, NOT TRADING. The 17:30 Sunday and 19:30 weekday phases are
// Pre-Open queues: orders rest, nothing matches until the 18:00 / 20:00 open.
// FANG+ publishes no tradeable phase outside its executable session, so the
// extended slice is empty.
pub(crate) static ICE_US_FANG_EXTENDED_CURRENT: &[SessionRule] = &[];
pub(crate) static ICE_US_FANG_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 19 * 3600 + 30 * 60,
        close_ssm: 20 * 3600,
    },
];

pub(crate) static ICE_US_FANG_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: ICE_US_FANG_REGULAR_CURRENT,
    extended: ICE_US_FANG_EXTENDED_CURRENT,
    order_entry: ICE_US_FANG_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Trading began at the start of trade date 2017-11-08, so the 20:00 prior-day
// rule and the 30-minute Pre-Open pin the first order-entry phase to Tuesday
// 2017-11-07 at 19:30 ET and nothing earlier that day.
static ICE_US_FANG_LAUNCH_EVE_REGULAR: &[SessionRule] = &[SessionRule {
    days: TUE_ONLY,
    open_ssm: 20 * 3600,
    close_ssm: 18 * 3600,
}];
// Same queue as the current profile, so the same classification.
static ICE_US_FANG_LAUNCH_EVE_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: TUE_ONLY,
    open_ssm: 19 * 3600 + 30 * 60,
    close_ssm: 20 * 3600,
}];
static ICE_US_FANG_LAUNCH_EVE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: ICE_US_FANG_LAUNCH_EVE_REGULAR,
    extended: &[],
    order_entry: ICE_US_FANG_LAUNCH_EVE_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// Evidence: docs/evidence/iceus.md, docs/evidence/ice_us.md
static ICE_US_FANG_REVISIONS: &[Revision] = revisions![
    // 2017-11-07 — T1 — ICE FANG+ launch notice 20170926 — launch-eve 19:30 ET
    // Pre-Open and 20:00 ET matching start.
    (
        2017,
        11,
        7,
        &ICE_US_FANG_LAUNCH_EVE,
        "ICE FANG+ launch notice 20170926"
    ),
    // 2017-11-08 — T1 — ICE FANG+ launch notice 20170926 — the full grid for
    // trade date 2017-11-08.
    (
        2017,
        11,
        8,
        &ICE_US_FANG_CURRENT,
        "ICE FANG+ launch notice 20170926"
    ),
];

pub(crate) fn ice_us_fang_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        ICE_US_FANG_REVISIONS,
    )
}
