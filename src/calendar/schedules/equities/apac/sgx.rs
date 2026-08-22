// SPDX-License-Identifier: MIT-0

//! Singapore Exchange securities market.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static SGX_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 12 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600,
        close_ssm: 17 * 3600,
    },
];
static SGX_REGULAR_CONTINUOUS: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 17 * 3600,
}];
static SGX_REGULAR_PRE_2011: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 12 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600,
        close_ssm: 17 * 3600,
    },
];
static SGX_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600,
        close_ssm: 13 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600,
        close_ssm: 17 * 3600 + 16 * 60,
    },
];
static SGX_EXTENDED_PRE_TAC: &[SessionRule] = &[
    SGX_EXTENDED_CURRENT[0],
    SGX_EXTENDED_CURRENT[1],
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600,
        close_ssm: 17 * 3600 + 6 * 60,
    },
];
static SGX_EXTENDED_NO_LUNCH: &[SessionRule] = &[
    SGX_EXTENDED_CURRENT[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600,
        close_ssm: 17 * 3600 + 6 * 60,
    },
];
static SGX_EXTENDED_PRE_2011: &[SessionRule] = &[
    SGX_EXTENDED_CURRENT[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600 + 30 * 60,
        close_ssm: 14 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600,
        close_ssm: 17 * 3600 + 6 * 60,
    },
];

// SGX-ST current phases: opening routine 08:30–09:00; regular
// 09:00–12:00/13:00–17:00; order-entry/auction lunch routine 12:00–13:00;
// closing routine to 17:06 and TAC to 17:16.
// https://rulebook.sgx.com/rulebook/regulatory-notice-821-trading-hours-market-phases-application-market-phases-and-principles
pub(crate) static SGX_SEC_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_REGULAR_CURRENT,
    extended: SGX_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Continuous all-day trading began 2011-08-01; the lunch break returned
// 2017-11-13; TAC began 2019-06-03.
// https://rulebook.sgx.com/sites/default/files/net_file_store/SGX_ST_Rules_August_1_2011.pdf
// https://links.sgx.com/1.0.0/corporate-announcements/AYXNAX3DG8RCFZT7/20170718_SGX_to_adjust_equities_market_structure_after_supportive_feedback.pdf
// https://links.sgx.com/1.0.0/corporate-announcements/46OQY4VBYIHO4ARN/20190514_SGX_to_launch_securities_market_trade_at_close_session_on_3_June.pdf
pub(crate) static SGX_SEC_PROFILE_POST_2017_11_13: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_REGULAR_CURRENT,
    extended: SGX_EXTENDED_PRE_TAC,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static SGX_SEC_PROFILE_POST_2011_08_01: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_REGULAR_CONTINUOUS,
    extended: SGX_EXTENDED_NO_LUNCH,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static SGX_SEC_PROFILE_PRE_2011_08_01: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_REGULAR_PRE_2011,
    extended: SGX_EXTENDED_PRE_2011,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &SGX_SEC_PROFILE_CURRENT;

static REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2011, 8, 1),
        profile: &SGX_SEC_PROFILE_POST_2011_08_01,
    },
    Revision {
        effective: effective_date(2017, 11, 13),
        profile: &SGX_SEC_PROFILE_POST_2017_11_13,
    },
    Revision {
        effective: effective_date(2019, 6, 3),
        profile: &SGX_SEC_PROFILE_CURRENT,
    },
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &SGX_SEC_PROFILE_PRE_2011_08_01,
        REVISIONS,
    )
}
