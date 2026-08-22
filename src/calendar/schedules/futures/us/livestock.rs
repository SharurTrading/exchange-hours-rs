// SPDX-License-Identifier: MIT-0

//! CME Live Cattle, Feeder Cattle, and Lean Hog futures schedules.

use chrono_tz::US;

use crate::calendar::rule::{FRI, MON_FRI, MON_ONLY};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};
use crate::calendar::{FuturesSessionProfile, SessionRule};

const MON_WED: [bool; 7] = [true, true, true, false, false, false, false];
const THU_ONLY: [bool; 7] = [false, false, false, true, false, false, false];
const TUE_THU: [bool; 7] = [false, true, true, true, false, false, false];

// CME's 2007 launch announcement establishes the Monday 09:05 CT weekly open
// and 16:00-17:00 daily halts; Q2008-215 moved the Friday close to 13:55 before
// the audit floor, and the 2014 reduction report confirms that complete old
// grid. SER-7194 removed the evening sessions effective Monday 2014-10-27.
// SER-7591 then set the current 08:30-13:05 CT weekday session for LE, GF, and
// HE effective Monday 2016-02-29.
// https://www.cmegroup.com/media-room/press-releases/2007/3/07/cme_to_offer_around-the-clocktradingofcommodityproductsoncmeglob.html
// https://www.cmegroup.com/tools-information/lookups/advisories/market-data/Q2008-215.html
// https://www.cmegroup.com/tools-information/lookups/advisories/ser/files/SER-7194.pdf
// https://www.cmegroup.com/market-regulation/files/14-408.pdf
// https://www.cmegroup.com/tools-information/lookups/advisories/ser/files/SER-7591.pdf
// https://www.cmegroup.com/education/lessons/live-cattle-product-overview
static REGULAR_AT_2010_FLOOR: &[SessionRule] = &[
    SessionRule {
        days: MON_ONLY,
        open_ssm: 9 * 3600 + 5 * 60,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: MON_WED,
        open_ssm: 17 * 3600,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: THU_ONLY,
        open_ssm: 17 * 3600,
        close_ssm: 13 * 3600 + 55 * 60,
    },
];

static REGULAR_2014_10_27: &[SessionRule] = &[
    SessionRule {
        days: MON_ONLY,
        open_ssm: 9 * 3600 + 5 * 60,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: TUE_THU,
        open_ssm: 8 * 3600,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: FRI,
        open_ssm: 8 * 3600,
        close_ssm: 13 * 3600 + 55 * 60,
    },
];

static REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 5 * 60,
}];

pub(crate) static CURRENT_FUTURES_PROFILE: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: REGULAR_CURRENT,
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_AT_2010_FLOOR: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: REGULAR_AT_2010_FLOOR,
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_2014_10_27: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: REGULAR_2014_10_27,
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: REGULAR_CURRENT,
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2014, 10, 27),
        profile: &PROFILE_2014_10_27,
    },
    Revision {
        effective: effective_date(2016, 2, 29),
        profile: &PROFILE_CURRENT,
    },
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, US::Central),
        &PROFILE_AT_2010_FLOOR,
        REVISIONS,
    )
}
