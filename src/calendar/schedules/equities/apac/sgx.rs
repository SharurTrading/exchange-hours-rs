// SPDX-License-Identifier: MIT-0

//! Singapore Exchange securities market.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static SGX_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 12 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600,
        close_ssm: 17 * 3600,
    },
];
static SGX_REGULAR_CONTINUOUS: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 17 * 3600,
}];
static SGX_REGULAR_PRE_2011: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 12 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600,
        close_ssm: 17 * 3600,
    },
];
// Regulatory Notice 8.2.1 splits every routine into two documented phases: a
// Pre-Open/Pre-Close Phase that "allows order entry, order modification and
// withdrawal of orders but no matching of orders", and a Non-Cancel Phase in
// which "all existing orders that can be matched are matched at a single price".
// Only the Non-Cancel Phase can print, so each routine is split at the earliest
// possible Non-Cancel start. Trade at Close matches at the Equilibrium Price and
// is tradeable throughout.
// https://rulebook.sgx.com/rulebook/regulatory-notice-821-trading-hours-market-phases-application-market-phases-and-principles

// Order-entry-only Pre-Open/Pre-Close phases (current randomized boundaries:
// Pre-Open ends 08:58–08:59 and 12:58–12:59, Pre-Close ends 17:04–17:05; the
// slices stop at the earliest possible end so no matching time is claimed).
const SGX_PRE_OPEN_MORNING: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 8 * 3600 + 58 * 60,
};
const SGX_PRE_OPEN_MIDDAY: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 12 * 3600,
    close_ssm: 12 * 3600 + 58 * 60,
};
const SGX_PRE_CLOSE: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 17 * 3600,
    close_ssm: 17 * 3600 + 4 * 60,
};
// Pre-2017 boundaries come from Practice Note 8.2.1 as amended 2011-08-01:
// Pre-Open 08:30–08:59, lunch-break Adjust 12:30–13:59 ("no matching of
// orders"), Pre-Close 17:00–17:05.
const SGX_PRE_OPEN_MORNING_PRE_2017: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 8 * 3600 + 59 * 60,
};
const SGX_LUNCH_ADJUST_PRE_2011: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 12 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 59 * 60,
};
const SGX_PRE_CLOSE_PRE_2017: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 17 * 3600,
    close_ssm: 17 * 3600 + 5 * 60,
};

// Matching windows: Non-Cancel phases, and the Trade at Close tail.
const SGX_NON_CANCEL_MORNING: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 58 * 60,
    close_ssm: 9 * 3600,
};
const SGX_NON_CANCEL_MIDDAY: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 12 * 3600 + 58 * 60,
    close_ssm: 13 * 3600,
};
const SGX_NON_CANCEL_CLOSE_AND_TAC: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 17 * 3600 + 4 * 60,
    close_ssm: 17 * 3600 + 16 * 60,
};
const SGX_NON_CANCEL_CLOSE: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 17 * 3600 + 4 * 60,
    close_ssm: 17 * 3600 + 6 * 60,
};
const SGX_NON_CANCEL_MORNING_PRE_2017: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 59 * 60,
    close_ssm: 9 * 3600,
};
const SGX_NON_CANCEL_LUNCH_PRE_2011: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 13 * 3600 + 59 * 60,
    close_ssm: 14 * 3600,
};
const SGX_NON_CANCEL_CLOSE_PRE_2017: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 17 * 3600 + 5 * 60,
    close_ssm: 17 * 3600 + 6 * 60,
};

static SGX_EXTENDED_CURRENT: &[SessionRule] = &[
    SGX_NON_CANCEL_MORNING,
    SGX_NON_CANCEL_MIDDAY,
    SGX_NON_CANCEL_CLOSE_AND_TAC,
];
static SGX_ORDER_ENTRY_CURRENT: &[SessionRule] =
    &[SGX_PRE_OPEN_MORNING, SGX_PRE_OPEN_MIDDAY, SGX_PRE_CLOSE];
static SGX_EXTENDED_PRE_TAC: &[SessionRule] = &[
    SGX_NON_CANCEL_MORNING,
    SGX_NON_CANCEL_MIDDAY,
    SGX_NON_CANCEL_CLOSE,
];
static SGX_ORDER_ENTRY_PRE_TAC: &[SessionRule] = SGX_ORDER_ENTRY_CURRENT;
static SGX_EXTENDED_NO_LUNCH: &[SessionRule] = &[
    SGX_NON_CANCEL_MORNING_PRE_2017,
    SGX_NON_CANCEL_CLOSE_PRE_2017,
];
static SGX_ORDER_ENTRY_NO_LUNCH: &[SessionRule] =
    &[SGX_PRE_OPEN_MORNING_PRE_2017, SGX_PRE_CLOSE_PRE_2017];
static SGX_EXTENDED_PRE_2011: &[SessionRule] = &[
    SGX_NON_CANCEL_MORNING_PRE_2017,
    SGX_NON_CANCEL_LUNCH_PRE_2011,
    SGX_NON_CANCEL_CLOSE_PRE_2017,
];
static SGX_ORDER_ENTRY_PRE_2011: &[SessionRule] = &[
    SGX_PRE_OPEN_MORNING_PRE_2017,
    SGX_LUNCH_ADJUST_PRE_2011,
    SGX_PRE_CLOSE_PRE_2017,
];

// SGX-ST current phases: opening routine 08:30–09:00; regular
// 09:00–12:00/13:00–17:00; order-entry/auction lunch routine 12:00–13:00;
// closing routine to 17:06 and TAC to 17:16. Each routine's Pre-Open/Pre-Close
// leg is order entry only and its Non-Cancel leg is where the single-price
// match prints; see the phase constants above.
// https://rulebook.sgx.com/rulebook/regulatory-notice-821-trading-hours-market-phases-application-market-phases-and-principles
pub(crate) static SGX_SEC_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_REGULAR_CURRENT,
    extended: SGX_EXTENDED_CURRENT,
    order_entry: SGX_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Continuous all-day trading began 2011-08-01; the lunch break returned
// 2017-11-13; TAC began 2019-06-03.
// https://rulebook.sgx.com/sites/default/files/net_file_store/SGX_ST_Rules_August_1_2011.pdf
// https://links.sgx.com/1.0.0/corporate-announcements/AYXNAX3DG8RCFZT7/20170718_SGX_to_adjust_equities_market_structure_after_supportive_feedback.pdf
// https://links.sgx.com/1.0.0/corporate-announcements/46OQY4VBYIHO4ARN/20190514_SGX_to_launch_securities_market_trade_at_close_session_on_3_June.pdf
// The 2011-08-01 practice note carries the pre-2017 routine boundaries used by
// the two oldest profiles: Pre-Open 08:30–08:59 / Non-Cancel 08:59–09:00,
// lunch-break Adjust 12:30–13:59 with no matching and its 13:59–14:00 match,
// and Pre-Close 17:00–17:05 / Non-Cancel 17:05–17:06.
pub(crate) static SGX_SEC_PROFILE_POST_2017_11_13: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_REGULAR_CURRENT,
    extended: SGX_EXTENDED_PRE_TAC,
    order_entry: SGX_ORDER_ENTRY_PRE_TAC,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static SGX_SEC_PROFILE_POST_2011_08_01: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_REGULAR_CONTINUOUS,
    extended: SGX_EXTENDED_NO_LUNCH,
    order_entry: SGX_ORDER_ENTRY_NO_LUNCH,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static SGX_SEC_PROFILE_PRE_2011_08_01: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_REGULAR_PRE_2011,
    extended: SGX_EXTENDED_PRE_2011,
    order_entry: SGX_ORDER_ENTRY_PRE_2011,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &SGX_SEC_PROFILE_CURRENT;

static REVISIONS: &[Revision] = revisions![
    (
        2011,
        8,
        1,
        &SGX_SEC_PROFILE_POST_2011_08_01,
        "SGX-ST Rules 2011-08-01"
    ),
    (
        2017,
        11,
        13,
        &SGX_SEC_PROFILE_POST_2017_11_13,
        "SGX announcement 2017-07-18"
    ),
    (
        2019,
        6,
        3,
        &SGX_SEC_PROFILE_CURRENT,
        "SGX announcement 2019-05-14"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &SGX_SEC_PROFILE_PRE_2011_08_01,
        REVISIONS,
    )
}
