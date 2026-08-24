// SPDX-License-Identifier: MIT-0

//! Stock Exchange of Thailand cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

const TUE_SAT: [bool; 7] = [false, true, true, true, true, true, false];

static SET_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 10 * 3600,
        close_ssm: 16 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 19 * 3600,
        close_ssm: 2 * 3600 + 45 * 60,
    },
];
static SET_REGULAR_POST_2024: &[SessionRule] = &[
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
static SET_REGULAR_PRE_2024: &[SessionRule] = &[
    SET_REGULAR_POST_2024[0],
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
    SessionRule {
        days: MON_FRI,
        open_ssm: 18 * 3600 + 45 * 60,
        close_ssm: 19 * 3600,
    },
    SessionRule {
        days: TUE_SAT,
        open_ssm: 2 * 3600 + 45 * 60,
        close_ssm: 3 * 3600,
    },
];
static SET_EXTENDED_TRANSITION: &[SessionRule] = &[
    SET_EXTENDED_CURRENT[0],
    SET_EXTENDED_CURRENT[1],
    SET_EXTENDED_CURRENT[2],
    SET_EXTENDED_CURRENT[3],
];
static SET_EXTENDED_POST_2024: &[SessionRule] = &[
    SET_EXTENDED_CURRENT[0],
    SET_EXTENDED_CURRENT[1],
    SET_EXTENDED_CURRENT[2],
];
static SET_EXTENDED_PRE_2024: &[SessionRule] = &[
    SET_EXTENDED_CURRENT[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600,
        close_ssm: 14 * 3600 + 30 * 60,
    },
    SET_EXTENDED_CURRENT[2],
];

// SET's venue-availability envelope includes eligible Europe/Americas DRs.
// From 2025-05-06 those DRs trade continuously through the ordinary-share
// lunch and in a 19:00–03:00 night session. The night pre-open begins 18:45,
// continuous trading ends 02:45, and the randomized closing auction ends no
// later than 03:00. The next-local-day tail belongs to the prior opening day's
// trade date. Not every listed security is eligible for every phase.
// https://www.set.or.th/en/market/information/trading-procedure/trading-hours
// https://www.set.or.th/en/market/news-and-alert/newsdetails?id=95921400&symbol=SET
pub(crate) static SET_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Bangkok,
    regular: SET_REGULAR_CURRENT,
    extended: SET_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
// The first night session opened on Tuesday 2025-05-06. This one-day profile
// prevents the generic Tuesday 02:45–03:00 tail from appearing before launch;
// the complete recurring week starts on the following local day.
pub(crate) static SET_PROFILE_2025_05_06: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Bangkok,
    regular: SET_REGULAR_CURRENT,
    extended: SET_EXTENDED_TRANSITION,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
// SET moved the ordinary afternoon session 30 minutes earlier on 2024-03-25.
// https://www.set.or.th/en/market/news-and-alert/newsdetails?id=86864800&symbol=SET
pub(crate) static SET_PROFILE_POST_2024_03_25: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Bangkok,
    regular: SET_REGULAR_POST_2024,
    extended: SET_EXTENDED_POST_2024,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static SET_PROFILE_PRE_2024_03_25: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Bangkok,
    regular: SET_REGULAR_PRE_2024,
    extended: SET_EXTENDED_PRE_2024,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &SET_PROFILE_CURRENT;

static REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2024, 3, 25),
        profile: &SET_PROFILE_POST_2024_03_25,
    },
    Revision {
        effective: effective_date(2025, 5, 6),
        profile: &SET_PROFILE_2025_05_06,
    },
    Revision {
        effective: effective_date(2025, 5, 7),
        profile: &SET_PROFILE_CURRENT,
    },
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &SET_PROFILE_PRE_2024_03_25,
        REVISIONS,
    )
}
