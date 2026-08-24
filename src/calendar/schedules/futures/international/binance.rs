// SPDX-License-Identifier: MIT-0

//! Binance USDⓈ-M perpetual futures normal availability.

use chrono_tz::UTC;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::ALL_DAYS;

// Binance's archived official launch article states that Binance Futures went
// live at 2019-09-13 04:00 UTC. The operator's current USDⓈ-M perpetual launch
// specifications publish 24/7 trading. This normal-availability profile
// excludes contract-specific launch/delist windows, ad-hoc maintenance, and
// operational incidents.
// https://arquivo.pt/noFrame/replay/20200608065459id_/https://www.binance.com/en/support/articles/360033314152
// https://t.me/binance_announcements/799
// https://www.binance.com/en-TR/support/announcement/detail/2bfb6f8dccf447ada57165b7e6a4cf1b
static ALWAYS_OPEN: &[SessionRule] = &[SessionRule {
    days: ALL_DAYS,
    open_ssm: 0,
    close_ssm: 24 * 3600,
}];
static LAUNCH_DAY_RULE: &[SessionRule] = &[SessionRule {
    days: [false, false, false, false, true, false, false],
    open_ssm: 4 * 3600,
    close_ssm: 24 * 3600,
}];

pub(crate) static CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: UTC,
    regular: ALWAYS_OPEN,
    extended: &[],
    order_entry: &[],
    has_daily_close: false,
    has_weekend_close: false,
};
static CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: UTC,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
static LAUNCH_DAY: StaticHoursProfile = StaticHoursProfile {
    tz: UTC,
    regular: LAUNCH_DAY_RULE,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// 2019-09-13 04:00:00 UTC. An exact instant is required; this launch is not a
// venue-local-midnight revision.
const LAUNCH_UNIX_SECONDS: i64 = 1_568_347_200;
const LAUNCH_DAY_END_UNIX_SECONDS: i64 = 1_568_419_200;

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    if as_of.timestamp() < LAUNCH_UNIX_SECONDS {
        &CLOSED
    } else if as_of.timestamp() < LAUNCH_DAY_END_UNIX_SECONDS {
        &LAUNCH_DAY
    } else {
        &CURRENT
    }
}
