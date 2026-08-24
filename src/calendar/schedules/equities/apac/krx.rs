// SPDX-License-Identifier: MIT-0

//! Korea Exchange cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static KRX_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 15 * 3600 + 20 * 60,
}];
static KRX_REGULAR_PRE_2016: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 14 * 3600 + 50 * 60,
}];
static KRX_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 20 * 60,
        close_ssm: 18 * 3600,
    },
];
static KRX_EXTENDED_POST_2016: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 30 * 60,
        close_ssm: 9 * 3600,
    },
    KRX_EXTENDED_CURRENT[1],
];
static KRX_EXTENDED_PRE_2016: &[SessionRule] = &[
    KRX_EXTENDED_POST_2016[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 50 * 60,
        close_ssm: 18 * 3600,
    },
];

// KRX continuous trading is 09:00–15:20, followed by a ten-minute closing
// call and order/trading phases through 18:00. Opening-call orders begin 08:30,
// while executable off-hours block and basket trading begins at 08:00.
// https://global.krx.co.kr/contents/GLB/06/0602/0602020204/GLB0602020204T1.jsp
pub(crate) static KRX_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Seoul,
    regular: KRX_REGULAR_CURRENT,
    extended: KRX_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// The regular close extended by 30 minutes on 2016-08-01. On 2019-04-29,
// pre-market block/basket trading moved 07:30 -> 08:00, prior-close trading
// moved 07:30–08:30 -> 08:30–08:40, and opening-call order reception moved
// 08:00 -> 08:30. The profile uses the earliest executable/accepted edge.
// https://global.krx.co.kr/contents/GLB/01/0107/0107010000/20170630_eng_brochure.pdf
// https://www.fsc.go.kr/po010106/73613
// https://law.krx.co.kr/las/LawBon.jsp?lawid=000111
pub(crate) static KRX_PROFILE_POST_2016_08_01: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Seoul,
    regular: KRX_REGULAR_CURRENT,
    extended: KRX_EXTENDED_POST_2016,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static KRX_PROFILE_PRE_2016_08_01: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Seoul,
    regular: KRX_REGULAR_PRE_2016,
    extended: KRX_EXTENDED_PRE_2016,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &KRX_PROFILE_CURRENT;

static REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2016, 8, 1),
        profile: &KRX_PROFILE_POST_2016_08_01,
    },
    Revision {
        effective: effective_date(2019, 4, 29),
        profile: &KRX_PROFILE_CURRENT,
    },
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &KRX_PROFILE_PRE_2016_08_01,
        REVISIONS,
    )
}
