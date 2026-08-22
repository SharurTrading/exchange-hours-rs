// SPDX-License-Identifier: MIT-0

//! CME Group futures schedules.
//!
//! Wrapped rules encode the prior-evening Globex open and next-afternoon
//! close. Omitting Friday from their open-day mask produces the weekend close.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, MON_THU, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// U.S.-grid CME and CBOT equity-index futures, including CBOT YM/MYM but not
// CME Nikkei 225 Dollar (NKD), whose historical grid differs. CME's
// October-2009 product guide supplies the complete grid at the audit floor:
// Sunday 17:00–Monday 15:15, then Monday–Thursday 17:00–15:15 and
// 15:30–16:30, with 16:30–17:00 maintenance. The 2012 notice changed the
// trade-date boundary and the post-halt slice to 15:30–16:15, including
// Fridays, effective Sunday 2012-11-18. CME Globex then moved that close
// 15 minutes earlier to 16:00 CT effective Sunday 2015-09-20 for trade date
// Monday 2015-09-21. Revisions are keyed by the local session-opening day.
// https://www.cmegroup.com/content/dam/cmegroup/education/modules/files/EQ240_EQ_for_AIT.pdf
// https://www.cmegroup.com/education/files/eq-trading-hours.pdf
// https://www.cmegroup.com/tools-information/lookups/advisories/clearing/Chadv12-423.html
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20121022.html
// https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20121015.html
// https://www.cmegroup.com/notices/clearing/2019/06/Chadv19-182.pdf
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150914.html
pub(crate) static CME_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 15 * 3600 + 15 * 60,
}];
static CME_EXT_PRE_2015_09_20: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 15 * 60,
    },
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
];
static CME_EXT_PRE_2012_11_18: &[SessionRule] = &[
    SessionRule {
        days: MON_THU,
        open_ssm: 15 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 30 * 60,
    },
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
];
pub(crate) static CME_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 30 * 60,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
];
static CME_PROFILE_PRE_2012_11_18: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CME_REGULAR,
    extended: CME_EXT_PRE_2012_11_18,
    has_daily_close: true,
    has_weekend_close: true,
};
static CME_PROFILE_2012_11_18: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CME_REGULAR,
    extended: CME_EXT_PRE_2015_09_20,
    has_daily_close: true,
    has_weekend_close: true,
};
static CME_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CME_REGULAR,
    extended: CME_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// CBOT grains/oilseeds. At the January 2010 audit floor, electronic trading ran
// 18:00–07:15 CT around the 09:30–13:15 day session. CME expanded electronic
// hours to a continuous 17:00–14:00 effective Sunday 2012-05-20. SER-6617 then
// established 19:00–07:45 and 08:30–13:15 effective Sunday 2013-04-07 for
// Monday's trade date. SER-7395R moved only the day close to 13:20 effective
// Sunday 2015-07-05 for trade date Monday 2015-07-06.
// https://www.cmegroup.com/media-room/press-releases/2009/6/05/cme_group_announcesadditionalagricultureethanolelectronictrading.html
// https://www.cmegroup.com/media-room/press-releases/2012/5/18/cme_group_to_startexpandedcbotgrainandoilseedtradinghoursmay20.html
// https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html
// https://www.cmegroup.com/rulebook/files/ser_6617_cbot_grain_oilseed_hours_2013_final.pdf
// https://www.cmegroup.com/tools-information/lookups/advisories/ser/SER-7395R.html
pub(crate) static CBOT_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 20 * 60,
}];
pub(crate) static CBOT_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 19 * 3600,
    close_ssm: 7 * 3600 + 45 * 60,
}];
static CBOT_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CBOT_REGULAR_CURRENT,
    extended: CBOT_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};
static CBOT_REGULAR_2013_04_07: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 15 * 60,
}];
static CBOT_EXTENDED_AT_2010_FLOOR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 18 * 3600,
    close_ssm: 7 * 3600 + 15 * 60,
}];
static CBOT_REGULAR_AT_2010_FLOOR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 15 * 60,
}];
static CBOT_PROFILE_AT_2010_FLOOR: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CBOT_REGULAR_AT_2010_FLOOR,
    extended: CBOT_EXTENDED_AT_2010_FLOOR,
    has_daily_close: true,
    has_weekend_close: true,
};
static CBOT_EXTENDED_2012_05_20: &[SessionRule] = &[
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 15 * 60,
        close_ssm: 14 * 3600,
    },
];
static CBOT_PROFILE_2012_05_20: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CBOT_REGULAR_AT_2010_FLOOR,
    extended: CBOT_EXTENDED_2012_05_20,
    has_daily_close: true,
    has_weekend_close: true,
};
static CBOT_PROFILE_2013_04_07: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CBOT_REGULAR_2013_04_07,
    extended: CBOT_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static CME_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2012, 11, 18),
        profile: &CME_PROFILE_2012_11_18,
    },
    Revision {
        effective: effective_date(2015, 9, 20),
        profile: &CME_PROFILE_CURRENT,
    },
];

pub(crate) fn cme_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, US::Central),
        &CME_PROFILE_PRE_2012_11_18,
        CME_REVISIONS,
    )
}

static CBOT_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2012, 5, 20),
        profile: &CBOT_PROFILE_2012_05_20,
    },
    Revision {
        effective: effective_date(2013, 4, 7),
        profile: &CBOT_PROFILE_2013_04_07,
    },
    Revision {
        // SER-7395R's official effective date is Sunday 2015-07-05; the
        // 13:20 daytime close first occurs on trade date Monday 2015-07-06.
        effective: effective_date(2015, 7, 5),
        profile: &CBOT_PROFILE_CURRENT,
    },
];

pub(crate) fn cbot_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, US::Central),
        &CBOT_PROFILE_AT_2010_FLOOR,
        CBOT_REVISIONS,
    )
}
