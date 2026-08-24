// SPDX-License-Identifier: MIT-0

//! BSE India cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static REGULAR_0955_1530: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 55 * 60,
    close_ssm: 15 * 3600 + 30 * 60,
}];
static REGULAR_0900_1530: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 15 * 3600 + 30 * 60,
}];
static REGULAR_0915_1530: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 15 * 60,
    close_ssm: 15 * 3600 + 30 * 60,
}];
static INDIA_EXTENDED_PRE_CAS: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 9 * 3600 + 15 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 40 * 60,
        close_ssm: 16 * 3600,
    },
];
static INDIA_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 9 * 3600 + 15 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 15 * 60,
        close_ssm: 15 * 3600 + 35 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 50 * 60,
        close_ssm: 16 * 3600,
    },
];
static BSE_EXTENDED_PRE_2010_10_18: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 15 * 3600 + 40 * 60,
    close_ssm: 16 * 3600,
}];
// Current NSE/BSE venue envelope. Effective 2026-08-03, derivative-eligible
// cash stocks enter CAS at 15:15 while non-CAS stocks continue normally to
// 15:30; the overlapping regular/extended rules preserve both venue-wide
// states. CAS ends 15:35, transition runs to 15:50, and post-close ends 16:00.
// Sources:
// https://www.nseindia.com/static/products-services/closing-auction-session
// https://www.sebi.gov.in/legal/circulars/jan-2026/introduction-of-closing-auction-session-cas-in-the-equity-cash-segment-and-certain-modifications-in-the-pre-open-auction-session_99122.html
// https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars?page=20260801-1
pub(crate) static BSE_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Kolkata,
    regular: REGULAR_0915_1530,
    extended: INDIA_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
// NSE and BSE jointly moved the continuous open 09:55 -> 09:00 on
// 2010-01-04. BSE's notice is 20091217-15; NSE's official release follows.
// https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars?page=20091217-15
// https://nsearchives.nseindia.com/content/press/17122009.htm
// BSE made pre-open 09:00–09:15 and continuous 09:15–15:30 effective
// 2010-10-18. Its 15:40 post-close start predates the audit floor (notice
// 20031205-4), so there is no fabricated 2011 BSE cutover.
// https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars?page=20101014-8
// https://api.bseindia.com/BseIndiaAPI/api/GetNoticesDownload_ng/w?Notice_no=20031205-4
pub(crate) static BSE_PROFILE_POST_2010_10_18: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Kolkata,
    regular: REGULAR_0915_1530,
    extended: INDIA_EXTENDED_PRE_CAS,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static BSE_PROFILE_POST_2010_01_04: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Kolkata,
    regular: REGULAR_0900_1530,
    extended: BSE_EXTENDED_PRE_2010_10_18,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static BSE_PROFILE_PRE_2010_01_04: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Kolkata,
    regular: REGULAR_0955_1530,
    extended: BSE_EXTENDED_PRE_2010_10_18,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &BSE_PROFILE_CURRENT;

static REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2010, 1, 4),
        profile: &BSE_PROFILE_POST_2010_01_04,
    },
    Revision {
        effective: effective_date(2010, 10, 18),
        profile: &BSE_PROFILE_POST_2010_10_18,
    },
    Revision {
        effective: effective_date(2026, 8, 3),
        profile: &BSE_PROFILE_CURRENT,
    },
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &BSE_PROFILE_PRE_2010_01_04,
        REVISIONS,
    )
}
