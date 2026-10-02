// SPDX-License-Identifier: MIT-0

//! National Stock Exchange of India cash equities.

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

static NSE_EXTENDED_PRE_2010_10_18: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 15 * 3600 + 50 * 60,
    close_ssm: 16 * 3600,
}];

// The 09:00–09:15 pre-open is two phases, not one. Narrative:
// docs/evidence/nse_india.md.
static INDIA_ORDER_ENTRY_PREOPEN: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 9 * 3600 + 7 * 60,
}];
static INDIA_PREOPEN_MATCH: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 7 * 60,
    close_ssm: 9 * 3600 + 15 * 60,
};

static NSE_EXTENDED_POST_2010_10_18: &[SessionRule] = &[
    INDIA_PREOPEN_MATCH,
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 50 * 60,
        close_ssm: 16 * 3600,
    },
];
static INDIA_EXTENDED_PRE_CAS: &[SessionRule] = &[
    INDIA_PREOPEN_MATCH,
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 40 * 60,
        close_ssm: 16 * 3600,
    },
];
static INDIA_EXTENDED_CURRENT: &[SessionRule] = &[
    INDIA_PREOPEN_MATCH,
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
// Current NSE/BSE venue envelope. Narrative:
// docs/evidence/nse_india.md.
pub(crate) static NSE_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Kolkata,
    regular: REGULAR_0915_1530,
    extended: INDIA_EXTENDED_CURRENT,
    order_entry: INDIA_ORDER_ENTRY_PREOPEN,
    has_daily_close: true,
    has_weekend_close: true,
};
// NSE introduced pre-open 09:00–09:15 on 2010-10-18, then moved its
// post-close start from 15:50 to 15:40 on 2011-10-03.
// Sources: NSE circular NSE/CMTR/15981 and NSE/CMTR/19013.
// https://nsearchives.nseindia.com/global/content/about_us/NSEIL_Annual_Report_2011.pdf
// https://nsearchives.nseindia.com/content/circulars/cmtr19013.pdf
pub(crate) static NSE_PROFILE_POST_2011_10_03: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Kolkata,
    regular: REGULAR_0915_1530,
    extended: INDIA_EXTENDED_PRE_CAS,
    order_entry: INDIA_ORDER_ENTRY_PREOPEN,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static NSE_PROFILE_POST_2010_10_18: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Kolkata,
    regular: REGULAR_0915_1530,
    extended: NSE_EXTENDED_POST_2010_10_18,
    order_entry: INDIA_ORDER_ENTRY_PREOPEN,
    has_daily_close: true,
    has_weekend_close: true,
};

// NSE and BSE jointly moved the continuous open 09:55 -> 09:00 on 2010-01-04. Narrative:
// docs/evidence/nse_india.md.
pub(crate) static NSE_PROFILE_POST_2010_01_04: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Kolkata,
    regular: REGULAR_0900_1530,
    extended: NSE_EXTENDED_PRE_2010_10_18,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static NSE_PROFILE_PRE_2010_01_04: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Kolkata,
    regular: REGULAR_0955_1530,
    extended: NSE_EXTENDED_PRE_2010_10_18,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &NSE_PROFILE_CURRENT;

// Evidence: docs/evidence/nse_india.md
static REVISIONS: &[Revision] = revisions![
    (
        2010,
        1,
        4,
        &NSE_PROFILE_POST_2010_01_04,
        "NSE press release 17122009"
    ),
    (
        2010,
        10,
        18,
        &NSE_PROFILE_POST_2010_10_18,
        "NSE circular NSE/CMTR/15981"
    ),
    (
        2011,
        10,
        3,
        &NSE_PROFILE_POST_2011_10_03,
        "NSE circular NSE/CMTR/19013"
    ),
    (2026, 8, 3, &NSE_PROFILE_CURRENT, "SEBI circular 99122"),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &NSE_PROFILE_PRE_2010_01_04,
        REVISIONS,
    )
}
