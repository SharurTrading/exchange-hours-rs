// SPDX-License-Identifier: MIT-0

//! Shanghai Stock Exchange cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static CHINA_REGULAR_WITH_CLOSE_CALL: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 30 * 60,
        close_ssm: 11 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600,
        close_ssm: 14 * 3600 + 57 * 60,
    },
];
static CHINA_REGULAR_WITHOUT_CLOSE_CALL: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 30 * 60,
        close_ssm: 11 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600,
        close_ssm: 15 * 3600,
    },
];
static CHINA_EXTENDED_CORE: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 15 * 60,
        close_ssm: 9 * 3600 + 25 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 57 * 60,
        close_ssm: 15 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600,
        close_ssm: 15 * 3600 + 30 * 60,
    },
];
static SSE_EXTENDED_PRE_2018: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 15 * 60,
        close_ssm: 9 * 3600 + 25 * 60,
    },
    CHINA_EXTENDED_CORE[2],
];
// SSE has accepted and confirmed block-trading declarations through 15:30
// since before the January-2010 audit floor. The 2026-07-06 generic fixed-price
// expansion changed eligibility inside that existing venue envelope, not the
// exchange-level close. Block and fixed-price phases are extended by
// convention; not every security is eligible for them.
// SSE rule and notice:
// https://www.sse.com.cn/lawandrules/sselawsrules2025/stocks/exchange/c/c_20260424_10816482.shtml
// Historical block-trading rule:
// https://www.sse.com.cn/lawandrules/sselawsrules2025/repeal/rules/c/c_20120918_10785158.shtml
pub(crate) static SSE_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Shanghai,
    regular: CHINA_REGULAR_WITH_CLOSE_CALL,
    extended: CHINA_EXTENDED_CORE,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
// SSE added its 14:57–15:00 closing call on 2018-08-20.
// https://english.sse.com.cn/news/newsrelease/c/4947833.shtml
pub(crate) static SSE_PROFILE_POST_2018_08_20: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Shanghai,
    regular: CHINA_REGULAR_WITH_CLOSE_CALL,
    extended: CHINA_EXTENDED_CORE,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static SSE_PROFILE_PRE_2018_08_20: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Shanghai,
    regular: CHINA_REGULAR_WITHOUT_CLOSE_CALL,
    extended: SSE_EXTENDED_PRE_2018,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &SSE_PROFILE_CURRENT;

static REVISIONS: &[Revision] = revisions![(
    2018,
    8,
    20,
    &SSE_PROFILE_POST_2018_08_20,
    "SSE news release 4947833"
),];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &SSE_PROFILE_PRE_2018_08_20,
        REVISIONS,
    )
}
