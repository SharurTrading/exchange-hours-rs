// SPDX-License-Identifier: MIT-0

//! CBOT mini-sized grain and oilseed futures schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

use super::grains::{
    CBOT_EXTENDED_AT_2010_FLOOR, CBOT_EXTENDED_CURRENT, CBOT_ORDER_ENTRY_2013_04_07,
    CBOT_ORDER_ENTRY_CURRENT,
};

// CBOT mini-sized grain and oilseed futures in America/Chicago: Mini-Sized
// Corn `XC`, Soybean `XK`, Wheat `XW` and KC HRW Wheat `MKC`, one family on
// one grid. Excludes the standard-size ZC/ZS/ZW/KE contracts (`grains.rs`),
// Rough Rice (`rough_rice.rs`) and the Micro Ag futures. Not foldable into
// `globex_grains`: the minis carried a permanent 30-minute-later day close
// until 2022-10-02. Narrative: docs/evidence/globex_mini_grains.md

static MINI_REGULAR_0930_1345: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 45 * 60,
}];
static MINI_REGULAR_0830_1345: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 45 * 60,
}];

// ORDER-ENTRY CLASSIFICATION. Only the matching windows can print a trade, so
// the Sunday evening queue, the weekday morning queue (07:15, then 08:00 from
// 2011-12-27, briefly 08:15 from 2013-04-07, back to 08:00 from 2013-08-18,
// up to the day-session open), and the afternoon PCP are `order_entry`; the
// overnight leg and the continuous session's afternoon slice stay `extended`.
static MINI_ORDER_ENTRY_AT_2010_FLOOR: &[SessionRule] = &[
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
// 2010-04-19: the PCP re-anchors to 30 seconds after the mini 13:45 close.
// The matching grid is unchanged, so the revision reuses
// `CBOT_EXTENDED_AT_2010_FLOOR`.
static MINI_ORDER_ENTRY_2010_04_19: &[SessionRule] = &[
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
        open_ssm: 13 * 3600 + 45 * 60 + 30,
        close_ssm: 16 * 3600,
    },
];
// 2011-12-27: the morning queue's start moves to 08:00.
static MINI_ORDER_ENTRY_2011_12_27: &[SessionRule] = &[
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
        open_ssm: 13 * 3600 + 45 * 60 + 30,
        close_ssm: 16 * 3600,
    },
];
// 2012-09-16: the only queue this era's advisory dates is the PCP, from
// 14:40 to the 16:00 end CME's same-day trading-hours page shows.
static MINI_ORDER_ENTRY_2012_09_16: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 14 * 3600 + 40 * 60,
    close_ssm: 16 * 3600,
}];

// The 21-hour continuous regimes. The wrap reaches the 09:30 day open and the
// afternoon slice resumes at the 13:45 mini day open, so the envelope runs
// 17:00 to 14:00 (then 14:30) continuously while the open-outcry window
// inside it stays regular.
static MINI_EXTENDED_2012_05_20: &[SessionRule] = &[
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 45 * 60,
        close_ssm: 14 * 3600,
    },
];
static MINI_EXTENDED_2012_09_16: &[SessionRule] = &[
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 45 * 60,
        close_ssm: 14 * 3600 + 30 * 60,
    },
];

/// The overnight leg SER-9049 and the 2022 notices carry unchanged:
/// Sunday-Friday 19:00-07:45 CT, wrapping local midnight.
pub(crate) use super::grains::CBOT_EXTENDED_CURRENT as MINI_EXTENDED_CURRENT;
/// The queue set SER-9049 lists as "(unchanged)" for all four minis: Sunday
/// 16:00-19:00, Monday-Friday 08:00-08:30, Monday-Friday 14:30-16:00, and
/// Monday-Thursday 16:45-19:00 CT.
pub(crate) use super::grains::CBOT_ORDER_ENTRY_CURRENT as MINI_ORDER_ENTRY_CURRENT;
/// From the 2022-10-02 convergence the mini day session is the standard
/// grains' 08:30-13:20 CT window, stated for all four products by SER-9049
/// and the October-2022 Globex notices. The table is shared with `grains.rs`
/// because the grids genuinely coincide from that day.
pub(crate) use super::grains::CBOT_REGULAR_CURRENT as MINI_REGULAR_CURRENT;

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
    MINI_REGULAR_0930_1345,
    CBOT_EXTENDED_AT_2010_FLOOR,
    MINI_ORDER_ENTRY_AT_2010_FLOOR,
);
static FROM_2010_04_19: StaticHoursProfile = profile(
    MINI_REGULAR_0930_1345,
    CBOT_EXTENDED_AT_2010_FLOOR,
    MINI_ORDER_ENTRY_2010_04_19,
);
static FROM_2011_12_27: StaticHoursProfile = profile(
    MINI_REGULAR_0930_1345,
    CBOT_EXTENDED_AT_2010_FLOOR,
    MINI_ORDER_ENTRY_2011_12_27,
);
static FROM_2012_05_20: StaticHoursProfile =
    profile(MINI_REGULAR_0930_1345, MINI_EXTENDED_2012_05_20, &[]);
static FROM_2012_09_16: StaticHoursProfile = profile(
    MINI_REGULAR_0930_1345,
    MINI_EXTENDED_2012_09_16,
    MINI_ORDER_ENTRY_2012_09_16,
);
static FROM_2013_04_07: StaticHoursProfile = profile(
    MINI_REGULAR_0830_1345,
    CBOT_EXTENDED_CURRENT,
    CBOT_ORDER_ENTRY_2013_04_07,
);
static FROM_2013_08_18: StaticHoursProfile = profile(
    MINI_REGULAR_0830_1345,
    CBOT_EXTENDED_CURRENT,
    CBOT_ORDER_ENTRY_CURRENT,
);
static DATED_CURRENT: StaticHoursProfile = profile(
    MINI_REGULAR_CURRENT,
    MINI_EXTENDED_CURRENT,
    MINI_ORDER_ENTRY_CURRENT,
);

// Every revision row below is T1; each row's effective day and citation
// literal are its own fields, and the document, quotation and URL behind each
// are in the evidence file.
// Evidence: docs/evidence/globex_mini_grains.md
static REVISIONS: &[Revision] = revisions![
    (
        2010,
        4,
        19,
        &FROM_2010_04_19,
        "CME Globex notice 20100405, mini-sized block"
    ),
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
        2012,
        9,
        16,
        &FROM_2012_09_16,
        "CME market-data advisory 20120904"
    ),
    (
        2013,
        4,
        7,
        &FROM_2013_04_07,
        "CME GCC notice 2013-03-22 and CBOT Submission 13-092"
    ),
    (
        2013,
        8,
        18,
        &FROM_2013_08_18,
        "CME market-data advisory 20130812"
    ),
    (
        2022,
        10,
        2,
        &DATED_CURRENT,
        "CME SER-9049 and Globex notice 20220905"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &AT_2010_FLOOR, REVISIONS)
}
