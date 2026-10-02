// SPDX-License-Identifier: MIT-0

//! Tokyo Stock Exchange cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// JPX publishes the current 09:00–11:30 and 12:30–15:30 auction-trading
// sessions, with order acceptance from 08:00 and 12:05. Arrowhead continuous
// matching ends at 15:25; the final five minutes are the closing call.
// Sources:
// https://www.jpx.co.jp/english/equities/trading/domestic/01.html
// https://www.jpx.co.jp/english/systems/equities-trading/01.html
static TSE_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 11 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600 + 30 * 60,
        close_ssm: 15 * 3600 + 25 * 60,
    },
];
static TSE_REGULAR_POST_2011: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 11 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600 + 30 * 60,
        close_ssm: 15 * 3600,
    },
];
static TSE_REGULAR_PRE_2011: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 11 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600 + 30 * 60,
        close_ssm: 15 * 3600,
    },
];
// TSE cash products trade on both the arrowhead auction system and the off-auction ToSTNeT system. Narrative:
// docs/evidence/tse.md.
static TSE_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 20 * 60,
    close_ssm: 18 * 3600,
}];
static TSE_EXTENDED_PRE_2024: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 20 * 60,
    close_ssm: 17 * 3600 + 30 * 60,
}];
// Arrowhead order acceptance ahead of the 08:20 ToSTNeT open. Orders may be
// entered, amended and cancelled; no matching engine runs, so no trade prints.
static TSE_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 8 * 3600 + 20 * 60,
}];

// JPX's official trading-hours transition table dates the arrowhead morning extension to 2011-11-21. Narrative:
// docs/evidence/tse.md.
pub(crate) static TSE_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Tokyo,
    regular: TSE_REGULAR_CURRENT,
    extended: TSE_EXTENDED_CURRENT,
    order_entry: TSE_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static TSE_PROFILE_POST_2011_11_21: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Tokyo,
    regular: TSE_REGULAR_POST_2011,
    extended: TSE_EXTENDED_PRE_2024,
    order_entry: TSE_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static TSE_PROFILE_PRE_2011_11_21: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Tokyo,
    regular: TSE_REGULAR_PRE_2011,
    extended: TSE_EXTENDED_PRE_2024,
    order_entry: TSE_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &TSE_PROFILE_CURRENT;

// Evidence: docs/evidence/tse.md
static REVISIONS: &[Revision] = revisions![
    (
        2011,
        11,
        21,
        &TSE_PROFILE_POST_2011_11_21,
        "JPX trading-hours transition table"
    ),
    (
        2024,
        11,
        5,
        &TSE_PROFILE_CURRENT,
        "JPX news release 20241103-01"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &TSE_PROFILE_PRE_2011_11_21,
        REVISIONS,
    )
}
