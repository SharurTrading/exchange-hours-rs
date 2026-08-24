// SPDX-License-Identifier: MIT-0

//! Saudi Exchange (Tadawul) Main Market cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;

const SUN_THU: [bool; 7] = [true, true, true, true, false, false, true];
const SAT_WED: [bool; 7] = [true, true, true, false, false, true, true];

static REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_THU,
    open_ssm: 10 * 3600,
    close_ssm: 15 * 3600,
}];
static EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_THU,
        open_ssm: 9 * 3600 + 30 * 60,
        close_ssm: 10 * 3600,
    },
    SessionRule {
        days: SUN_THU,
        open_ssm: 15 * 3600,
        close_ssm: 15 * 3600 + 20 * 60,
    },
];
static EXTENDED_POST_2018: &[SessionRule] = &[
    EXTENDED_CURRENT[0],
    SessionRule {
        days: SUN_THU,
        open_ssm: 15 * 3600,
        close_ssm: 15 * 3600 + 10 * 60,
    },
];
static EXTENDED_POST_2016: &[SessionRule] = &[EXTENDED_CURRENT[0]];
static REGULAR_OLD_SUN_THU: &[SessionRule] = &[SessionRule {
    days: SUN_THU,
    open_ssm: 11 * 3600,
    close_ssm: 15 * 3600 + 30 * 60,
}];
static EXTENDED_OLD_SUN_THU: &[SessionRule] = &[SessionRule {
    days: SUN_THU,
    open_ssm: 10 * 3600,
    close_ssm: 11 * 3600,
}];
static REGULAR_OLD_SAT_WED: &[SessionRule] = &[SessionRule {
    days: SAT_WED,
    open_ssm: 11 * 3600,
    close_ssm: 15 * 3600 + 30 * 60,
}];
static EXTENDED_OLD_SAT_WED: &[SessionRule] = &[SessionRule {
    days: SAT_WED,
    open_ssm: 10 * 3600,
    close_ssm: 11 * 3600,
}];
static REGULAR_PANDEMIC: &[SessionRule] = &[SessionRule {
    days: SUN_THU,
    open_ssm: 10 * 3600,
    close_ssm: 13 * 3600,
}];
static EXTENDED_PANDEMIC: &[SessionRule] = &[
    EXTENDED_CURRENT[0],
    SessionRule {
        days: SUN_THU,
        open_ssm: 13 * 3600,
        close_ssm: 13 * 3600 + 20 * 60,
    },
];

// Current Main Market phases: opening-auction orders 09:30–10:00,
// continuous trading 10:00–15:00, closing auction 15:00–15:10, and trade at
// last 15:10–15:20. Auction uncrosses can be randomized by up to 30 seconds;
// the static profile uses the published nominal boundaries.
// https://www.saudiexchange.sa/wps/portal/saudiexchange/rules-guidance/capital-market-overview/trading-cycle-and-times?locale=en
pub(crate) static TADAWUL_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Riyadh,
    regular: REGULAR_CURRENT,
    extended: EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// Session changes since 2010: the workweek changed effective 2013-06-29;
// hours moved to 10:00–15:00 effective 2016-04-03; closing auction arrived
// 2018-05-27; and trade at last arrived 2019-05-12.
// https://www.spa.gov.sa/7e453de27d
// https://www.spa.gov.sa/1484000?lang=en&newsid=1484000
// https://www.saudiexchange.sa/wps/wcm/connect/24ca438e-86a0-47d0-b8f4-65b4cbfebcdd/Saudi%2BStock%2BExchange%2B-Tadawul-%2CStatistical%2BReport%2B%E2%80%93%2BFirst%2BHalf%2B2018%2B-%2BUpdated.pdf
// https://www.saudiexchange.sa/wps/wcm/connect/4657c15f-ef37-45c8-8423-09e2a5055ab7/Saudi%2BStock%2BExchange%2B%28Tadawul%29%2CStatistical%2BReport%2B%E2%80%93%2B%2B2019-%2BEn.pdf
pub(crate) static TADAWUL_PROFILE_POST_2018_05_27: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Riyadh,
    regular: REGULAR_CURRENT,
    extended: EXTENDED_POST_2018,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static TADAWUL_PROFILE_POST_2016_04_03: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Riyadh,
    regular: REGULAR_CURRENT,
    extended: EXTENDED_POST_2016,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static TADAWUL_PROFILE_POST_2013_06_29: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Riyadh,
    regular: REGULAR_OLD_SUN_THU,
    extended: EXTENDED_OLD_SUN_THU,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static TADAWUL_PROFILE_PRE_2013_06_29: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Riyadh,
    regular: REGULAR_OLD_SAT_WED,
    extended: EXTENDED_OLD_SAT_WED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// Temporary shortened hours applied 2020-03-26 through 2020-05-30.
// https://www.saudiexchange.sa/wps/portal/saudiexchange/newsandreports/issuer-news/news-detail-wcm/?locale=en&newsId=6262
// https://www.saudiexchange.sa/wps/portal/saudiexchange/newsandreports/issuer-news/news-detail-wcm/saudiexchangecontent/issuernews/issuernewsdetails/saudiexchange-announces-resumption-of-normal-trading-hours?locale=en
pub(crate) static TADAWUL_PROFILE_PANDEMIC: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Riyadh,
    regular: REGULAR_PANDEMIC,
    extended: EXTENDED_PANDEMIC,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &TADAWUL_PROFILE_CURRENT;

static REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2013, 6, 29),
        profile: &TADAWUL_PROFILE_POST_2013_06_29,
    },
    Revision {
        effective: effective_date(2016, 4, 3),
        profile: &TADAWUL_PROFILE_POST_2016_04_03,
    },
    Revision {
        effective: effective_date(2018, 5, 27),
        profile: &TADAWUL_PROFILE_POST_2018_05_27,
    },
    Revision {
        effective: effective_date(2019, 5, 12),
        profile: &TADAWUL_PROFILE_CURRENT,
    },
    Revision {
        effective: effective_date(2020, 3, 26),
        profile: &TADAWUL_PROFILE_PANDEMIC,
    },
    Revision {
        effective: effective_date(2020, 5, 31),
        profile: &TADAWUL_PROFILE_CURRENT,
    },
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &TADAWUL_PROFILE_PRE_2013_06_29,
        REVISIONS,
    )
}
