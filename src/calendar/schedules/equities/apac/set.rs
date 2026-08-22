// SPDX-License-Identifier: MIT-0

//! Stock Exchange of Thailand cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static SET_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 10 * 3600,
        close_ssm: 12 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600,
        close_ssm: 16 * 3600 + 30 * 60,
    },
];
static SET_REGULAR_OLD: &[SessionRule] = &[
    SET_REGULAR_CURRENT[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 30 * 60,
    },
];
static SET_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 30 * 60,
        close_ssm: 10 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 30 * 60,
        close_ssm: 14 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 30 * 60,
        close_ssm: 17 * 3600,
    },
];
static SET_EXTENDED_OLD: &[SessionRule] = &[
    SET_EXTENDED_CURRENT[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600,
        close_ssm: 14 * 3600 + 30 * 60,
    },
    SET_EXTENDED_CURRENT[2],
];

// SET randomizes its actual auction uncrosses inside the published windows;
// the deterministic regular rules begin at their conservative latest edge.
// The afternoon session moved 30 minutes earlier on 2024-03-25.
// https://www.set.or.th/en/market/information/trading-procedure/trading-hours
// https://www.set.or.th/en/market/news-and-alert/newsdetails?id=86864800&symbol=SET
pub(crate) static SET_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Bangkok,
    regular: SET_REGULAR_CURRENT,
    extended: SET_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static SET_PROFILE_PRE_2024_03_25: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Bangkok,
    regular: SET_REGULAR_OLD,
    extended: SET_EXTENDED_OLD,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &SET_PROFILE_CURRENT;

static REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2024, 3, 25),
    profile: &SET_PROFILE_CURRENT,
}];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &SET_PROFILE_PRE_2024_03_25,
        REVISIONS,
    )
}
