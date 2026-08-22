// SPDX-License-Identifier: MIT-0

//! Taiwan Stock Exchange cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static TWSE_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 13 * 3600 + 25 * 60,
}];
static TWSE_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 25 * 60,
        close_ssm: 13 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600,
        close_ssm: 14 * 3600 + 30 * 60,
    },
];
static TWSE_EXTENDED_PRE_2020: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30 * 60,
        close_ssm: 13 * 3600 + 30 * 60,
    },
    TWSE_EXTENDED_CURRENT[2],
];

// TWSE continuous trading runs 09:00–13:25, bounded by opening and closing
// calls, with after-hours fixed-price order entry 14:00–14:30.
// https://www.twse.com.tw/en/products/system/trading.html
pub(crate) static TWSE_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Taipei,
    regular: TWSE_REGULAR_CURRENT,
    extended: TWSE_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Before continuous trading launched 2020-03-23, intraday matches were call
// auctions every five seconds, so the primary session is extended by the
// crate's auction convention rather than mislabeled as continuous/regular.
// https://www.twse.com.tw/en/about/company/history.html
pub(crate) static TWSE_PROFILE_PRE_2020_03_23: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Taipei,
    regular: &[],
    extended: TWSE_EXTENDED_PRE_2020,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &TWSE_PROFILE_CURRENT;

static REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2020, 3, 23),
    profile: &TWSE_PROFILE_CURRENT,
}];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &TWSE_PROFILE_PRE_2020_03_23,
        REVISIONS,
    )
}
