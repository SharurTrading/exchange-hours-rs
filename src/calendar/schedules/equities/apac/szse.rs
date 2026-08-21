// SPDX-License-Identifier: MIT-0

//! Shenzhen Stock Exchange cash equities.

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
];
static CHINA_EXTENDED_CURRENT: &[SessionRule] = &[
    CHINA_EXTENDED_CORE[0],
    CHINA_EXTENDED_CORE[1],
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600,
        close_ssm: 15 * 3600 + 30 * 60,
    },
];
static SZSE_EXTENDED_PRE_2016: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 15 * 60,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 57 * 60,
        close_ssm: 15 * 3600,
    },
];
// Effective 2026-07-06, both exchanges extended generic after-hours
// fixed-price order acceptance through 15:30 (matching begins 15:05). The
// 15:00–15:05 order-only slice is extended by convention. Product-specific
// STAR/ChiNext predecessors are excluded.
// SSE rule and notice:
// https://www.sse.com.cn/lawandrules/sselawsrules2025/stocks/exchange/c/c_20260424_10816482.shtml
// SZSE rule and notice:
// https://www.szse.cn/lawrules/rule/trade/current/t20260424_620190.html
pub(crate) static SZSE_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Shanghai,
    regular: CHINA_REGULAR_WITH_CLOSE_CALL,
    extended: CHINA_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};
// SZSE stopped accepting orders in its 09:25–09:30 blocking interval on
// 2016-05-09; the session intervals otherwise stayed unchanged.
// https://www.szse.cn/aboutus/trends/news/t20160930_518722.html
pub(crate) static SZSE_PROFILE_POST_2016_05_09: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Shanghai,
    regular: CHINA_REGULAR_WITH_CLOSE_CALL,
    extended: CHINA_EXTENDED_CORE,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static SZSE_PROFILE_PRE_2016_05_09: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Shanghai,
    regular: CHINA_REGULAR_WITH_CLOSE_CALL,
    extended: SZSE_EXTENDED_PRE_2016,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &SZSE_PROFILE_CURRENT;

static REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2016, 5, 9),
        profile: &SZSE_PROFILE_POST_2016_05_09,
    },
    Revision {
        effective: effective_date(2026, 7, 6),
        profile: &SZSE_PROFILE_CURRENT,
    },
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &SZSE_PROFILE_PRE_2016_05_09,
        REVISIONS,
    )
}
