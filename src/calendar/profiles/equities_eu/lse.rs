// SPDX-License-Identifier: MIT-0

//! London Stock Exchange (SETS).
//!
//! The UK venue stands apart from the continental default: 08:00–16:30
//! continuous, and a Closing Price Crossing session — real executions at the
//! closing auction price — after the closing auction.

use chrono_tz::Europe;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// LSE (SETS): 08:00–16:30 continuous; opening auction call 07:50–08:00,
// closing auction call 16:30–16:35, then the Closing Price Crossing session —
// real electronic executions at the closing auction price — to 16:40.
// Source: LSE "Guide to the Trading System" (MIT201): Closing Auction Call
// 16:30–16:35, "Closing Price Crossing Session" 16:35–16:40, during which
// orders are matched at that day's closing price.
static LSE_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 16 * 3600 + 30 * 60,
}];
static AU_0750_0800_1630_1635: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 50 * 60,
        close_ssm: 8 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 35 * 60,
    },
    // Closing Price Crossing 16:35–16:40.
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 35 * 60,
        close_ssm: 16 * 3600 + 40 * 60,
    },
];
pub(crate) static LSE_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::London,
    regular: LSE_REGULAR,
    extended: AU_0750_0800_1630_1635,
    has_daily_close: true,
    has_weekend_close: true,
};
