// SPDX-License-Identifier: MIT-0

//! CBOT standard-size grain and oilseed futures schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// Standard-size CBOT grain and oilseed futures only. Mini-sized Corn, Soybean,
// Wheat and KC HRW Wheat diverged on 2012-09-16 and are owned by
// `mini_grains.rs`. See docs/evidence/cbot.md and
// docs/evidence/globex_grains.md.

// The `CBOT_*` rule tables below are `pub(crate)` because Rough Rice
// (`rough_rice.rs`) ran on this same grid until its 2018-01-21 divergence and
// borrows the pre-divergence eras rather than copying them. That key owns its
// own `StaticHoursProfile` values and its own timeline, so a future Rough
// Rice-specific finding repoints one of its eras instead of editing anything
// here. Nothing in this file may be changed on Rough Rice evidence.

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

// ORDER-ENTRY CLASSIFICATION. Only the matching windows can print a trade, so
// the Sunday evening queue, the weekday morning queue (07:15, later 08:00,
// briefly 08:15 from 2013-04-07, back to 08:00 from 2013-08-18) and the
// afternoon PCP are `order_entry`; the electronic session and the post-2012
// afternoon matching slice stay `extended`.
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

// Evidence: docs/evidence/cbot.md, docs/evidence/globex_grains.md
static REVISIONS: &[Revision] = revisions![
    // 2010-04-19 — T1 — CME Globex notice 20100405 — the afternoon PCP expands
    // to 13:15:30-16:00 CT.
    (2010, 4, 19, &FROM_2010_04_19, "CME Globex notice 20100405"),
    // 2011-12-27 — T1 — CFTC filing rul120711cbot001 — the weekday morning queue
    // moves to 08:00 CT.
    (
        2011,
        12,
        27,
        &FROM_2011_12_27,
        "CFTC filing rul120711cbot001"
    ),
    // 2012-05-20 — T1 — CME market-data advisory 20120518 — matching expands to
    // 17:00-14:00 CT.
    (
        2012,
        5,
        20,
        &FROM_2012_05_20,
        "CME market-data advisory 20120518"
    ),
    // 2013-04-07 — T1 — CME SER-6617 and GCC notice 2013-03-22 — 19:00-07:45 CT
    // around an 08:30-13:15 CT day session, with the full queue set.
    (
        2013,
        4,
        7,
        &FROM_2013_04_07,
        "CME SER-6617 and GCC notice 2013-03-22"
    ),
    // 2013-08-18 — T1 — CME market-data advisory 20130812 — the morning Pre-Open
    // widens from 08:15 to 08:00 CT.
    (
        2013,
        8,
        18,
        &FROM_2013_08_18,
        "CME market-data advisory 20130812"
    ),
    // 2015-07-05 — T1 — CME SER-7395R — the day-session close moves to 13:20 CT.
    (2015, 7, 5, &DATED_CURRENT, "CME SER-7395R"),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &AT_2010_FLOOR, REVISIONS)
}
