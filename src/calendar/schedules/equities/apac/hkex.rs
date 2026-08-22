// SPDX-License-Identifier: MIT-0

//! Hong Kong Exchanges and Clearing securities market.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static HKEX_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 30 * 60,
        close_ssm: 12 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600,
        close_ssm: 16 * 3600,
    },
];
static HKEX_REGULAR_PHASE_ONE: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 30 * 60,
        close_ssm: 12 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 30 * 60,
        close_ssm: 16 * 3600,
    },
];
static HKEX_REGULAR_PRE_2011: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 10 * 3600,
        close_ssm: 12 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 30 * 60,
        close_ssm: 16 * 3600,
    },
];
static HKEX_PREOPEN_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 9 * 3600 + 30 * 60,
}];
static HKEX_PREOPEN_OLD: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 10 * 3600,
}];
static HKEX_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 16 * 3600 + 10 * 60,
    },
];

// Current HKEX securities hours: POS 09:00–09:30, continuous
// 09:30–12:00/13:00–16:00, then CAS with a randomized 16:08–16:10 close. The
// static profile uses the maximum scheduled envelope. Extended Morning
// trading is product-specific and excluded.
// https://www.hkex.com.hk/Services/Trading-hours-and-Severe-Weather-Arrangements/Trading-Hours/Securities-Market?sc_lang=en
pub(crate) static HKEX_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Hong_Kong,
    regular: HKEX_REGULAR_CURRENT,
    extended: HKEX_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// HKEX Phase One took effect 2011-03-07 and Phase Two 2012-03-05. CAS became
// generic on 2019-10-08; the 2016/2017 phases covered subsets only.
// https://www.hkex.com.hk/News/News-Release/2011/110303news?sc_lang=en
// https://www.hkex.com.hk/News/Regulatory-Announcements/2012/120301news?sc_lang=en
// https://www.hkex.com.hk/News/Market-Communications/2019/1907053news?sc_lang=en
pub(crate) static HKEX_PROFILE_POST_2012_03_05: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Hong_Kong,
    regular: HKEX_REGULAR_CURRENT,
    extended: HKEX_PREOPEN_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static HKEX_PROFILE_POST_2011_03_07: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Hong_Kong,
    regular: HKEX_REGULAR_PHASE_ONE,
    extended: HKEX_PREOPEN_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static HKEX_PROFILE_PRE_2011_03_07: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Hong_Kong,
    regular: HKEX_REGULAR_PRE_2011,
    extended: HKEX_PREOPEN_OLD,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &HKEX_PROFILE_CURRENT;

static REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2011, 3, 7),
        profile: &HKEX_PROFILE_POST_2011_03_07,
    },
    Revision {
        effective: effective_date(2012, 3, 5),
        profile: &HKEX_PROFILE_POST_2012_03_05,
    },
    Revision {
        effective: effective_date(2019, 10, 8),
        profile: &HKEX_PROFILE_CURRENT,
    },
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &HKEX_PROFILE_PRE_2011_03_07,
        REVISIONS,
    )
}
