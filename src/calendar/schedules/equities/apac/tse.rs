// SPDX-License-Identifier: MIT-0

//! Tokyo Stock Exchange cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// JPX publishes the current 09:00–11:30 and 12:30–15:30 auction-trading
// sessions, with order acceptance from 08:00 and 12:05. Arrowhead continuous
// matching ends at 15:25; the final five minutes are the closing call.
// Sources:
// https://www.jpx.co.jp/english/equities/trading/domestic/01.html
// https://www.jpx.co.jp/english/systems/equities-trading/01.html
static TSE_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 11 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600 + 30 * 60,
        close_ssm: 15 * 3600 + 25 * 60,
    },
];
static TSE_REGULAR_POST_2011: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 11 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600 + 30 * 60,
        close_ssm: 15 * 3600,
    },
];
static TSE_REGULAR_PRE_2011: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 11 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600 + 30 * 60,
        close_ssm: 15 * 3600,
    },
];
static TSE_PREOPEN: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600 + 5 * 60,
        close_ssm: 12 * 3600 + 30 * 60,
    },
];
static TSE_EXTENDED_CURRENT: &[SessionRule] = &[
    TSE_PREOPEN[0],
    TSE_PREOPEN[1],
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 25 * 60,
        close_ssm: 15 * 3600 + 30 * 60,
    },
];

// JPX's official trading-hours transition table dates the morning extension
// to 2011-11-21 and the afternoon extension to 2024-11-05.
// https://www.jpx.co.jp/english/equities/trading/domestic/tvdivq0000006blj-att/tradinghours_eg.pdf
pub(crate) static TSE_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Tokyo,
    regular: TSE_REGULAR_CURRENT,
    extended: TSE_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static TSE_PROFILE_POST_2011_11_21: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Tokyo,
    regular: TSE_REGULAR_POST_2011,
    extended: TSE_PREOPEN,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static TSE_PROFILE_PRE_2011_11_21: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Tokyo,
    regular: TSE_REGULAR_PRE_2011,
    extended: TSE_PREOPEN,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &TSE_PROFILE_CURRENT;

static REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2011, 11, 21),
        profile: &TSE_PROFILE_POST_2011_11_21,
    },
    Revision {
        effective: effective_date(2024, 11, 5),
        profile: &TSE_PROFILE_CURRENT,
    },
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &TSE_PROFILE_PRE_2011_11_21,
        REVISIONS,
    )
}
