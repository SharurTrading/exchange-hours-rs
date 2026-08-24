// SPDX-License-Identifier: MIT-0

//! CME Group futures schedules.
//!
//! Wrapped rules encode the prior-evening Globex open and next-afternoon
//! close. Omitting Friday from their open-day mask produces the weekend close.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
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
// Monday 2015-09-21. CME then removed the 15:15-15:30 halt for the scoped
// contracts effective Sunday 2021-06-27, producing the current continuous
// 17:00-16:00 ETH envelope around the unchanged 08:30-15:15 RTH.
//
// The exact Monday-Thursday Pre-Open changed from 16:50 to 16:45 on
// 2010-11-15. Current primary material also establishes Sunday 16:00-17:00,
// but calls it a long-term practice without giving the day when the earlier
// 16:15 start moved. The fixed-current table includes that sourced current
// queue. Dated profiles deliberately omit only the Sunday queue rather than
// inventing its cutover; their executable trading and weekday queues remain
// exact. Revisions are keyed by the local session-opening day.
// https://www.cmegroup.com/content/dam/cmegroup/education/modules/files/EQ240_EQ_for_AIT.pdf
// https://www.cmegroup.com/education/files/eq-trading-hours.pdf
// https://www.cmegroup.com/tools-information/lookups/advisories/clearing/Chadv12-423.html
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20121022.html
// https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20121015.html
// https://www.cmegroup.com/notices/clearing/2019/06/Chadv19-182.pdf
// The 2015-09-20 revision's original announcement, CME Globex Notice #20150817
// of 17 August 2015: "Effective Monday, September 21, the daily CME Globex
// maintenance period will begin 15 minutes earlier Monday through Thursday from
// 16:00 until 16:45 Central Time (CT). ... the closing times for the following
// markets will now occur 15 minutes earlier Monday through Friday at 16:00 CT.
// CME Equity / CBOT Equity / COMEX / NYMEX / DME. All other CME Globex markets
// trading hours remain unchanged." The #20150914 repeat below carries the same
// article with "Effective this Monday" wording.
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150817.html
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150914.html
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
// https://www.cmegroup.com/notices/electronic-trading/2021/06/20210621.html
// https://www.cmegroup.com/market-regulation/rule-filings/2021/6/21-244R_2.pdf
// https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf
pub(crate) static CME_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 15 * 3600 + 15 * 60,
}];
// ORDER-ENTRY CLASSIFICATION. The evening phases the citations above name as
// the "Pre-Open" (Monday-Thursday 16:50, later 16:45, to the 17:00 Globex open)
// and the Sunday 16:00-17:00 queue are Globex pre-open states: the book accepts,
// amends, and cancels orders, but the matching engine is not running and no
// trade can print until the 17:00 open. They are therefore `order_entry`, not
// `extended`. Everything that remains in the extended slices below is a
// matching phase: the post-halt afternoon slice and the 17:00 electronic
// session both print trades.
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
static CME_EXT_2015_09_20: &[SessionRule] = &[
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
// One matching grid serves both pre-2012-11-18 profiles: the 16:50 -> 16:45
// Pre-Open move of 2010-11-15 changed only the queue, which now lives in the
// `order_entry` slices below.
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
        open_ssm: 15 * 3600 + 15 * 60,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
];
static CME_EXT_DATED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 15 * 60,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
];

// Pre-Open queues. No trade can match in any of these windows.
static CME_ORDER_ENTRY_1650: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 16 * 3600 + 50 * 60,
    close_ssm: 17 * 3600,
}];
static CME_ORDER_ENTRY_1645: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 16 * 3600 + 45 * 60,
    close_ssm: 17 * 3600,
}];
/// Fixed-current queues: the sourced Sunday 16:00-17:00 phase plus the
/// Monday-Thursday 16:45-17:00 Pre-Open.
pub(crate) static CME_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600,
    },
];
static CME_PROFILE_PRE_2012_11_18: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CME_REGULAR,
    extended: CME_EXT_PRE_2012_11_18,
    order_entry: CME_ORDER_ENTRY_1645,
    has_daily_close: true,
    has_weekend_close: true,
};
static CME_PROFILE_PRE_2010_11_15: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CME_REGULAR,
    extended: CME_EXT_PRE_2012_11_18,
    order_entry: CME_ORDER_ENTRY_1650,
    has_daily_close: true,
    has_weekend_close: true,
};
static CME_PROFILE_2012_11_18: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CME_REGULAR,
    extended: CME_EXT_PRE_2015_09_20,
    order_entry: CME_ORDER_ENTRY_1645,
    has_daily_close: true,
    has_weekend_close: true,
};
static CME_PROFILE_2015_09_20: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CME_REGULAR,
    extended: CME_EXT_2015_09_20,
    order_entry: CME_ORDER_ENTRY_1645,
    has_daily_close: true,
    has_weekend_close: true,
};
static CME_PROFILE_DATED_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CME_REGULAR,
    extended: CME_EXT_DATED_CURRENT,
    order_entry: CME_ORDER_ENTRY_1645,
    has_daily_close: true,
    has_weekend_close: true,
};

static CME_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2010, 11, 15),
        profile: &CME_PROFILE_PRE_2012_11_18,
    },
    Revision {
        effective: effective_date(2012, 11, 18),
        profile: &CME_PROFILE_2012_11_18,
    },
    Revision {
        effective: effective_date(2015, 9, 20),
        profile: &CME_PROFILE_2015_09_20,
    },
    Revision {
        effective: effective_date(2021, 6, 27),
        profile: &CME_PROFILE_DATED_CURRENT,
    },
];

pub(crate) fn cme_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, US::Central),
        &CME_PROFILE_PRE_2010_11_15,
        CME_REVISIONS,
    )
}
