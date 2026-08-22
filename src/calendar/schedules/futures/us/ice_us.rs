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
// June-2026 master table retain that grid. Order-only pre-open is excluded.
//
// Equal SessionRule endpoints encode one complete local-day span, so the
// exceptional Sunday session remains continuous through Monday 18:00.
// https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_FANG%2BFuture_20170926.pdf
// https://www.ice.com/products/66380320/NYSE-FANG-Index-Future
// https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
pub(crate) static ICE_US_FANG_EXTENDED_CURRENT: &[SessionRule] = &[
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

pub(crate) static ICE_US_FANG_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: &[],
    extended: ICE_US_FANG_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// The launch notice says trading began at the start of trade date 2017-11-08;
// its own 20:00 prior-day rule therefore pins the first opening to Tuesday
// 2017-11-07 at 20:00 ET. This one-evening profile avoids pretending the
// product traded earlier on its local launch-eve date.
static ICE_US_FANG_LAUNCH_EVE_EXTENDED: &[SessionRule] = &[SessionRule {
    days: TUE_ONLY,
    open_ssm: 20 * 3600,
    close_ssm: 18 * 3600,
}];
static ICE_US_FANG_LAUNCH_EVE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: &[],
    extended: ICE_US_FANG_LAUNCH_EVE_EXTENDED,
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
