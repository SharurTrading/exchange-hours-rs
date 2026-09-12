// SPDX-License-Identifier: MIT-0

//! CBOT U.S. Treasury/Fed Funds and CME SOFR futures schedules.

use chrono_tz::US;

use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};
use crate::calendar::{FuturesSessionProfile, SessionRule};

// CBOT U.S. Treasury and micro-Treasury, 30-Day Fed Funds and CME SOFR futures
// on one product-neutral family clock. Individual contract launch dates — SOFR
// joined this already-live family in May 2018 — are catalog facts, not
// revisions of the family clock.
// See docs/evidence/globex_interest_rates.md.

// ORDER-ENTRY CLASSIFICATION. The comment above calls the Sunday 16:15 (later
// 16:00) and weekday 16:50 (later 16:45) phases "queues": Globex accepts,
// amends, and cancels orders in them while the matching engine is stopped, and
// nothing can print until the 17:30 (later 17:00) open. They are `order_entry`.
// The matching windows below stay in `extended`.
static EXTENDED_1730_1600: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600 + 30 * 60,
    close_ssm: 16 * 3600,
}];
static EXTENDED_1700_1600: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];
static ORDER_ENTRY_AT_2010_FLOOR: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 50 * 60,
        close_ssm: 17 * 3600 + 30 * 60,
    },
];
static ORDER_ENTRY_2010_11_15: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600 + 30 * 60,
    },
];
// SUNDAY QUEUE, CARRIED BACK AS THE SOURCED INTERSECTION. The 16:15-17:00
// window is order-entry under every sourced state, so carrying it from the
// January-2010 floor asserts no cutover; the knowledge-bound row below adds the
// disputed 16:00-16:15 quarter-hour. See
// docs/evidence/globex_interest_rates.md.
static ORDER_ENTRY_DATED_2011_10_02: &[SessionRule] = &[
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
pub(crate) static ORDER_ENTRY_CURRENT: &[SessionRule] = &[
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

pub(crate) static CURRENT_FUTURES_PROFILE: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_1700_1600,
    order_entry: ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_AT_2010_FLOOR: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_1730_1600,
    order_entry: ORDER_ENTRY_AT_2010_FLOOR,
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_2010_11_15: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_1730_1600,
    order_entry: ORDER_ENTRY_2010_11_15,
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_2011_10_02: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_1700_1600,
    order_entry: ORDER_ENTRY_DATED_2011_10_02,
    has_daily_close: true,
    has_weekend_close: true,
};

// Verified-current grid: identical to PROFILE_2011_10_02 except the Sunday
// 16:00–17:00 queue, whose onset day no reviewed primary source states.
static PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_1700_1600,
    order_entry: ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Evidence: docs/evidence/globex_interest_rates.md
static REVISIONS: &[Revision] = revisions![
    // 2010-11-15 — T1 — CME Globex notice 20101025 — Monday-Thursday Pre-Open
    // moves from 16:50 to 16:45 CT.
    (
        2010,
        11,
        15,
        &PROFILE_2010_11_15,
        "CME Globex notice 20101025"
    ),
    // 2011-10-02 — T1 — CME Globex notice 20110926 — every legacy CBOT
    // interest-rate open moves to 17:00 CT for trade date Monday 2011-10-03.
    (
        2011,
        10,
        2,
        &PROFILE_2011_10_02,
        "CME Globex notice 20110926"
    ),
    // 2026-08-22 — T1 — 2026-08-22 review: verified current, onset undated —
    // knowledge-bound row widening the Sunday queue by the disputed 16:00-16:15
    // CT quarter-hour, the only part depending on the undated 2012 move. A
    // sourced onset day replaces this row.
    (
        2026,
        8,
        22,
        &PROFILE_CURRENT,
        "2026-08-22 review: verified current, onset undated"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, US::Central),
        &PROFILE_AT_2010_FLOOR,
        REVISIONS,
    )
}
