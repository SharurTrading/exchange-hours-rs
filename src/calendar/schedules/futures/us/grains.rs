// SPDX-License-Identifier: MIT-0

//! CBOT standard-size grain and oilseed futures schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// This profile is deliberately limited to standard-size CBOT grain and oilseed
// futures. Mini-sized Corn, Soybean, and Wheat diverged on 2012-09-16 and are
// not represented by this key.
//
// The `CBOT_*` rule tables below are `pub(crate)` because Rough Rice
// (`rough_rice.rs`) ran on this same grid until its 2018-01-21 divergence and
// borrows the pre-divergence eras rather than copying them. That key owns its
// own `StaticHoursProfile` values and its own timeline, so a future Rough
// Rice-specific finding repoints one of its eras instead of editing anything
// here. Nothing in this file may be changed on Rough Rice evidence.
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
// The 22 March 2013 Global Command Center client notice that carried the
// SER-6617 change also states every current queue's unconditional onset:
// "Effective Sunday, April 7, 2013 (trade date Monday, April 8) ... Pre-Opens
// (including MGEX): Sunday night: 16:00-19:00 CT / Monday-Thursday night:
// 16:45-19:00 CT / Monday-Friday morning: 08:15-08:30 CT. Post Close Pre-Open:
// Monday-Friday: 14:30-16:00 CT", with a 07:45-08:15 cancellation-only slice
// inside the break that no order-entry rule models. Only the 21-hour
// 2012-05-20..2013-04-06 regime's queue and PCP states remain undocumented, so
// 2026-08-31 grains-regime review — the states are now sourced, only their
// switch-on day is not. CME's own trading-hours pages inside the 21-hour
// regime publish, for Corn/Wheat/Soybean/Soybean Oil futures and options,
// Sunday Pre-Open 16:00, weekday Pre-Open "14:30-16:00, 16:45-17:00" (the PCP
// plus the evening queue) and ETH 17:00-14:00. The pre-expansion capture of
// 2012-05-11 shows the other side: Sunday Pre-Open 16:15, weekday
// "14:30-16:00 16:45 08:00" and ETH 18:00-07:15, 09:30-13:15. The switch is
// therefore bracketed to 2012-05-11..2012-05-28, which contains the sourced
// 2012-05-20 expansion — but Advisory #20120518 states only the new matching
// hours, never the queue times, so no queue revision is keyed to that day.
// Official origin http://www.cmegroup.com/trading_hours/ delivered via:
// https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities
// https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html
// https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/
// that dated profile conservatively omits them instead of inventing onsets.
// https://web.archive.org/web/20130423023212/http://www.cmegroup.com/globex/files/cmegroup_reduced_grain_and_oilseed_hours.pdf
// https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf

pub(crate) static CBOT_REGULAR_0930_1315: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 15 * 60,
}];
pub(crate) static CBOT_REGULAR_0830_1315: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 15 * 60,
}];
pub(crate) static CBOT_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 20 * 60,
}];

// ORDER-ENTRY CLASSIFICATION. The comment above distinguishes the matching
// windows from the market-state phases the operator's tables publish around
// them. Only the matching windows can print a trade, so the Sunday evening
// queue, the weekday morning queue (07:15, later 08:00, briefly 08:15 from
// 2013-04-07, back to 08:00 from 2013-08-18, up to the day-session open), and
// the afternoon PCP are `order_entry`; the electronic session and the
// post-2012 afternoon matching slice stay `extended`.
pub(crate) static CBOT_EXTENDED_AT_2010_FLOOR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 18 * 3600,
    close_ssm: 7 * 3600 + 15 * 60,
}];
pub(crate) static CBOT_ORDER_ENTRY_AT_2010_FLOOR: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 18 * 3600,
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
// 2010-04-19 and 2011-12-27 change only queue boundaries: PCP expands to start
// at 13:15:30, then the morning queue starts at 08:00. The matching grid is
// unchanged, so both revisions reuse `CBOT_EXTENDED_AT_2010_FLOOR`.
pub(crate) static CBOT_ORDER_ENTRY_2010_04_19: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 18 * 3600,
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
pub(crate) static CBOT_ORDER_ENTRY_2011_12_27: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 18 * 3600,
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
pub(crate) static CBOT_EXTENDED_2012_05_20: &[SessionRule] = &[
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
// Queues from the 2013-03-22 operator notice: the Sunday and Monday-Thursday
// evening pre-opens that run up to the 19:00 electronic open, the 08:15-08:30
// morning Pre-Open at go-live, and the 14:30-16:00 PCP. None can match a trade.
pub(crate) static CBOT_ORDER_ENTRY_2013_04_07: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600,
        close_ssm: 19 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 15 * 60,
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
pub(crate) static CBOT_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 19 * 3600,
    close_ssm: 7 * 3600 + 45 * 60,
}];
/// The queue set the 2013-03-22 notice established with the 19:00 open, with
/// the morning Pre-Open widened from 08:15 to 08:00 by the 2013-08-18 advisory:
/// the Sunday and Monday-Thursday evening pre-opens up to the 19:00 electronic
/// open, the 08:00-08:30 morning Pre-Open, and the 14:30-16:00 PCP. None of
/// them can match a trade.
pub(crate) static CBOT_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600,
        close_ssm: 19 * 3600,
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
    order_entry: &'static [SessionRule],
) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: US::Central,
        regular,
        extended,
        order_entry,
        has_daily_close: true,
        has_weekend_close: true,
    }
}

static AT_2010_FLOOR: StaticHoursProfile = profile(
    CBOT_REGULAR_0930_1315,
    CBOT_EXTENDED_AT_2010_FLOOR,
    CBOT_ORDER_ENTRY_AT_2010_FLOOR,
);
static FROM_2010_04_19: StaticHoursProfile = profile(
    CBOT_REGULAR_0930_1315,
    CBOT_EXTENDED_AT_2010_FLOOR,
    CBOT_ORDER_ENTRY_2010_04_19,
);
static FROM_2011_12_27: StaticHoursProfile = profile(
    CBOT_REGULAR_0930_1315,
    CBOT_EXTENDED_AT_2010_FLOOR,
    CBOT_ORDER_ENTRY_2011_12_27,
);
static FROM_2012_05_20: StaticHoursProfile =
    profile(CBOT_REGULAR_0930_1315, CBOT_EXTENDED_2012_05_20, &[]);
static FROM_2013_04_07: StaticHoursProfile = profile(
    CBOT_REGULAR_0830_1315,
    CBOT_EXTENDED_CURRENT,
    CBOT_ORDER_ENTRY_2013_04_07,
);
static FROM_2013_08_18: StaticHoursProfile = profile(
    CBOT_REGULAR_0830_1315,
    CBOT_EXTENDED_CURRENT,
    CBOT_ORDER_ENTRY_CURRENT,
);
static DATED_CURRENT: StaticHoursProfile = profile(
    CBOT_REGULAR_CURRENT,
    CBOT_EXTENDED_CURRENT,
    CBOT_ORDER_ENTRY_CURRENT,
);

// Revision evidence — each row's day-level effective date and the primary
// source that states it (full quotations sit in the blocks above):
//   2010-04-19 "CME Globex notice 20100405"
//     https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100405.html
//   2011-12-27 "CFTC filing rul120711cbot001"
//     https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul120711cbot001.pdf
//   2012-05-20 "CME market-data advisory 20120518"
//     https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html
//   2013-04-07 "CME SER-6617 and GCC notice 2013-03-22"
//     https://www.cmegroup.com/rulebook/files/ser_6617_cbot_grain_oilseed_hours_2013_final.pdf
//     https://web.archive.org/web/20130423023212/http://www.cmegroup.com/globex/files/cmegroup_reduced_grain_and_oilseed_hours.pdf
//   2013-08-18 "CME market-data advisory 20130812"
//     https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20130812.html
//   2015-07-05 "CME SER-7395R"
//     https://www.cmegroup.com/tools-information/lookups/advisories/ser/SER-7395R.html
static REVISIONS: &[Revision] = revisions![
    (2010, 4, 19, &FROM_2010_04_19, "CME Globex notice 20100405"),
    (
        2011,
        12,
        27,
        &FROM_2011_12_27,
        "CFTC filing rul120711cbot001"
    ),
    (
        2012,
        5,
        20,
        &FROM_2012_05_20,
        "CME market-data advisory 20120518"
    ),
    (
        2013,
        4,
        7,
        &FROM_2013_04_07,
        "CME SER-6617 and GCC notice 2013-03-22"
    ),
    (
        2013,
        8,
        18,
        &FROM_2013_08_18,
        "CME market-data advisory 20130812"
    ),
    (2015, 7, 5, &DATED_CURRENT, "CME SER-7395R"),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &AT_2010_FLOOR, REVISIONS)
}
