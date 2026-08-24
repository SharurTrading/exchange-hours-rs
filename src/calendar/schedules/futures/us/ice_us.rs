// SPDX-License-Identifier: MIT-0

//! ICE Futures U.S. NYSE FANG+ Index futures schedules.

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY, TUE_ONLY};
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};
use crate::calendar::schedules::{CLOSED_NEW_YORK, StaticHoursProfile};

// The `iceus` default is the NYSE FANG+ Index futures family, not a venue-wide
// clock. ICE launched it for trade date 2017-11-08 with 20:00-18:00 ET hours
// and an exceptional Sunday 18:00 open; the current product page and ICE's
// June-2026 master table retain that grid. The launch notice starts Pre-Open
// 30 minutes before each executable session, and the current product page
// separately publishes the 17:30 Sunday and 19:30 weekday queue starts.
//
// Equal SessionRule endpoints encode one complete local-day span, so the
// exceptional Sunday session remains continuous through Monday 18:00.
// https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_FANG%2BFuture_20170926.pdf
// https://www.ice.com/products/66380320/NYSE-FANG-Index-Future
// https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
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
// ORDER ENTRY, NOT TRADING. The 17:30 Sunday and 19:30 weekday phases are the
// Pre-Open queues the launch notice and product page describe: the platform
// accepts, amends and cancels orders for the coming session, and nothing
// matches until the 18:00 / 20:00 open. They are therefore classified as
// order-entry phases rather than tradeable extended sessions. FANG publishes no
// tradeable phase outside its executable session, so the extended slice is
// empty.
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

// The launch notice says trading began at the start of trade date 2017-11-08;
// its 20:00 prior-day trading rule and 30-minute Pre-Open therefore pin the
// first order-entry phase to Tuesday 2017-11-07 at 19:30 ET. This one-evening
// profile avoids pretending the product accepted orders earlier that day.
static ICE_US_FANG_LAUNCH_EVE_REGULAR: &[SessionRule] = &[SessionRule {
    days: TUE_ONLY,
    open_ssm: 20 * 3600,
    close_ssm: 18 * 3600,
}];
// Same Pre-Open queue as the current profile, so the same classification: this
// is the launch evening's order-entry phase, not a tradeable session.
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

static ICE_US_FANG_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2017, 11, 7),
        profile: &ICE_US_FANG_LAUNCH_EVE,
    },
    Revision {
        effective: effective_date(2017, 11, 8),
        profile: &ICE_US_FANG_CURRENT,
    },
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
