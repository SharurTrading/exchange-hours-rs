// SPDX-License-Identifier: MIT-0

//! CBOT standard-size grain and oilseed futures schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// This profile is deliberately limited to standard-size CBOT grain and oilseed
// futures. Mini-sized Corn, Soybean, and Wheat diverged on 2012-09-16 and are
// not represented by this key.
//
// At the January-2010 audit floor, matching ran 18:00-07:15 around the
// 09:30-13:15 RTH. The operator's March-2010 market-state table supplies the
// then-live 16:15-18:00 Sunday, 07:15-09:30 weekday, and 14:30-16:00 PCP
// phases. On 2010-04-19 PCP expanded to 13:15:30-16:00. The CFTC filing makes
// the weekday morning queue's move to 08:00 effective Tuesday 2011-12-27.
// A later generic Globex notice broadly names CBOT in an afternoon queue
// change, but it does not enumerate this family and conflicts with the complete
// family-specific state table. No separate evening queue is inferred from it.
// https://www.cmegroup.com/media-room/press-releases/2009/6/05/cme_group_announcesadditionalagricultureethanolelectronictrading.html
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100315.html
// https://www.cmegroup.com/tools-information/lookups/advisories/market-data/Q2010-62.html
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100405.html
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
// https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul120711cbot001.pdf
//
// Matching expanded to 17:00-14:00 on 2012-05-20. SER-6617 then established
// 19:00-07:45 and 08:30-13:15 effective Sunday 2013-04-07. CME expanded the
// exact morning Pre-Open to 08:00-08:30 on 2013-08-18, and SER-7395R moved the
// RTH close to 13:20 on 2015-07-05.
// https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html
// https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120904.html
// https://www.cmegroup.com/rulebook/files/ser_6617_cbot_grain_oilseed_hours_2013_final.pdf
// https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20130812.html
// https://www.cmegroup.com/tools-information/lookups/advisories/ser/SER-7395R.html
//
// Current primary material additionally establishes Sunday 16:00-19:00,
// Monday-Thursday 16:45-19:00, and PCP 14:30-16:00. It does not state the
// unconditional days on which those phases entered the post-May-2012 schedule.
// The fixed-current profile includes them. Dated profiles conservatively omit
// those unresolved phases from 2012-05-20 onward instead of inventing onset
// selectors; matching and the exact 2013 morning-queue change remain complete.
// https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf
// https://www.cmegroup.com/trading-hours/files/memorial-day-2023.pdf

static REGULAR_0930_1315: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 15 * 60,
}];
static REGULAR_0830_1315: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 15 * 60,
}];
pub(crate) static CBOT_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 20 * 60,
}];

static EXTENDED_AT_2010_FLOOR: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 18 * 3600,
        close_ssm: 7 * 3600 + 15 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 15 * 60,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 30 * 60,
        close_ssm: 16 * 3600,
    },
];
static EXTENDED_2010_04_19: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 18 * 3600,
        close_ssm: 7 * 3600 + 15 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 15 * 60,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 15 * 60 + 30,
        close_ssm: 16 * 3600,
    },
];
static EXTENDED_2011_12_27: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 18 * 3600,
        close_ssm: 7 * 3600 + 15 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 15 * 60 + 30,
        close_ssm: 16 * 3600,
    },
];
static EXTENDED_2012_05_20: &[SessionRule] = &[
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
static EXTENDED_2013_04_07: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 19 * 3600,
    close_ssm: 7 * 3600 + 45 * 60,
}];
static EXTENDED_DATED_2013_08_18: &[SessionRule] = &[
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 19 * 3600,
        close_ssm: 7 * 3600 + 45 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
];
pub(crate) static CBOT_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600,
        close_ssm: 19 * 3600,
    },
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 19 * 3600,
        close_ssm: 7 * 3600 + 45 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 30 * 60,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 19 * 3600,
    },
];

const fn profile(
    regular: &'static [SessionRule],
    extended: &'static [SessionRule],
) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: US::Central,
        regular,
        extended,
        order_entry: &[],
        has_daily_close: true,
        has_weekend_close: true,
    }
}

static AT_2010_FLOOR: StaticHoursProfile = profile(REGULAR_0930_1315, EXTENDED_AT_2010_FLOOR);
static FROM_2010_04_19: StaticHoursProfile = profile(REGULAR_0930_1315, EXTENDED_2010_04_19);
static FROM_2011_12_27: StaticHoursProfile = profile(REGULAR_0930_1315, EXTENDED_2011_12_27);
static FROM_2012_05_20: StaticHoursProfile = profile(REGULAR_0930_1315, EXTENDED_2012_05_20);
static FROM_2013_04_07: StaticHoursProfile = profile(REGULAR_0830_1315, EXTENDED_2013_04_07);
static FROM_2013_08_18: StaticHoursProfile = profile(REGULAR_0830_1315, EXTENDED_DATED_2013_08_18);
static DATED_CURRENT: StaticHoursProfile = profile(CBOT_REGULAR_CURRENT, EXTENDED_DATED_2013_08_18);

static REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2010, 4, 19),
        profile: &FROM_2010_04_19,
    },
    Revision {
        effective: effective_date(2011, 12, 27),
        profile: &FROM_2011_12_27,
    },
    Revision {
        effective: effective_date(2012, 5, 20),
        profile: &FROM_2012_05_20,
    },
    Revision {
        effective: effective_date(2013, 4, 7),
        profile: &FROM_2013_04_07,
    },
    Revision {
        effective: effective_date(2013, 8, 18),
        profile: &FROM_2013_08_18,
    },
    Revision {
        effective: effective_date(2015, 7, 5),
        profile: &DATED_CURRENT,
    },
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &AT_2010_FLOOR, REVISIONS)
}
