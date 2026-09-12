// SPDX-License-Identifier: MIT-0

//! CME Group futures schedules.
//!
//! Wrapped rules encode the prior-evening Globex open and next-afternoon
//! close. Omitting Friday from their open-day mask produces the weekend close.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// U.S.-grid CME and CBOT equity-index futures, including CBOT YM/MYM but not
// CME Nikkei 225 Dollar (NKD), whose historical grid differs. Revisions are
// keyed by the local session-opening day. See docs/evidence/cme.md and
// docs/evidence/globex_equity_index.md.
pub(crate) static CME_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 15 * 3600 + 15 * 60,
}];
// ORDER-ENTRY CLASSIFICATION. The evening "Pre-Open" phases (Monday-Thursday
// 16:50, later 16:45, and Sunday) accept, amend and cancel orders while the
// matching engine is stopped, so they are `order_entry`. Everything in the
// extended slices below is a matching phase.
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

// Pre-Open queues; no trade can match in any of them. The Sunday queue is the
// sourced 16:15-17:00 intersection: CME's queue only widened inside the
// modelled window, so carrying it from the January-2010 floor asserts no
// cutover, and the knowledge-bound row below adds the disputed 16:00-16:15
// quarter-hour. See docs/evidence/cme.md.
static CME_ORDER_ENTRY_1650: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 50 * 60,
        close_ssm: 17 * 3600,
    },
];
static CME_ORDER_ENTRY_1645: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600,
    },
];
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
// Verified-current grid: identical to DATED_CURRENT except the Sunday
// 16:00–17:00 Pre-Open queue, whose onset day no reviewed primary source
// states.
static CME_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CME_REGULAR,
    extended: CME_EXTENDED_CURRENT,
    order_entry: CME_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Evidence: docs/evidence/cme.md, docs/evidence/globex_equity_index.md
static CME_REVISIONS: &[Revision] = revisions![
    // 2010-11-15 — T1 — CME Globex notice 20101025 — Monday-Thursday Pre-Open
    // moves from 16:50 to 16:45 CT.
    (
        2010,
        11,
        15,
        &CME_PROFILE_PRE_2012_11_18,
        "CME Globex notice 20101025"
    ),
    // 2012-11-18 — T1 — CME Globex notice 20121022 — new daily trading-hour
    // schedule; the post-halt slice becomes 15:30-16:15 CT including Fridays.
    (
        2012,
        11,
        18,
        &CME_PROFILE_2012_11_18,
        "CME Globex notice 20121022"
    ),
    // 2015-09-20 — T1 — CME Globex notice 20150817 — CME Equity and CBOT Equity
    // closes move 15 minutes earlier to 16:00 CT.
    (
        2015,
        9,
        20,
        &CME_PROFILE_2015_09_20,
        "CME Globex notice 20150817"
    ),
    // 2021-06-27 — T1 — CME Globex notice 20210621 — the 15:15-15:30 CT halt is
    // removed, producing the continuous 17:00-16:00 CT ETH envelope.
    (
        2021,
        6,
        27,
        &CME_PROFILE_DATED_CURRENT,
        "CME Globex notice 20210621"
    ),
    // 2026-08-22 — T1 — 2026-08-22 review: verified current, onset undated —
    // knowledge-bound row widening the Sunday queue to 16:00-17:00 CT. Only the
    // disputed quarter-hour depends on the undated 2012 move; the 16:15-17:00
    // remainder is already carried from the January-2010 floor above. A sourced
    // onset day replaces this row.
    (
        2026,
        8,
        22,
        &CME_PROFILE_CURRENT,
        "2026-08-22 review: verified current, onset undated"
    ),
];

pub(crate) fn cme_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, US::Central),
        &CME_PROFILE_PRE_2010_11_15,
        CME_REVISIONS,
    )
}
