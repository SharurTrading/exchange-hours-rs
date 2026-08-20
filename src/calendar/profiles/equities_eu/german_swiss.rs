// SPDX-License-Identifier: MIT-0

//! The Deutsche Börse and SIX Group venues: Xetra, SIX Swiss Exchange,
//! Vienna (Wiener Börse, a Xetra/T7 market), and BME (Spain, SIX Group).
//!
//! All four run the continental 09:00–17:30 continuous default except SIX
//! (17:20), and all four run a post-close trade-at-last/trade-at-close
//! window — with different bounds each, which is why each has its own
//! extended table.

use chrono_tz::Europe;

use super::super::StaticHoursProfile;
use super::REG_0900_1730;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// Xetra: opening auction 08:50–09:00, closing auction 17:30–17:35, then
// Trade-at-Close to 17:45. Source: FESE 2025 hours table, Deutsche Börse
// (Xetra) row — "08:50 - 9:00 Opening Auction; 17:30 - ~17:35 Closing
// Auction; 17:35 - 17:45 Trading at Close".
static XETRA_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 50 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 35 * 60,
    },
    // Trade-at-Close 17:35–17:45.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 35 * 60,
        close_ssm: 17 * 3600 + 45 * 60,
    },
];
pub(crate) static XETRA_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: REG_0900_1730,
    extended: XETRA_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};
// SIX Swiss Exchange — shares segments (Blue Chip / Mid-/Small-Cap), which is
// what `Exchange::Six` denotes. SIX does NOT follow the Xetra pattern:
// continuous trading ends at 17:20, the closing auction runs 17:20–17:30, and
// Trading-At-Last then runs to 17:40. The 17:30–17:35 auction belongs to the
// ETF/ETP/Sponsored Funds segments only, which have no TAL.
//
// Trading Guide, Blue Chip Shares: "Trading Hours 09:00 - 17:30 CET /
// Continuous Trading 09:00 - 17:20 CET / Closing Auction 17:20 - 17:30 CET /
// Trading-At-Last Start: 17:30 - 17:32 CET End: 17:40 CET"; segment table row
// "Blue Chip Shares 06:00 09:00 17:20 17:30 17:30 17:40 22:00".
// Sources: SIX Group, "Trading hours"
// (https://www.six-group.com/en/products-services/the-swiss-stock-exchange/trading/trading-provisions/trading-hours.html)
// and the SIX Swiss Exchange Trading Guide.
//
// SIX labels its times "CET" year-round; they are local Zurich wall-clock, so
// `Europe::Zurich` (CET/CEST) is the correct zone, not a fixed offset.
static SIX_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 17 * 3600 + 20 * 60,
}];
static SIX_EXTENDED: &[SessionRule] = &[
    // Pre-opening: order entry with a theoretical opening price, from the start
    // of the business day until the opening auction uncrosses at 09:00. No
    // continuous trading happens in this window.
    SessionRule {
        days: MON_FRI,
        open_ssm: 6 * 3600,
        close_ssm: 9 * 3600,
    },
    // Closing auction.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 20 * 60,
        close_ssm: 17 * 3600 + 30 * 60,
    },
    // Trading-At-Last: execution at the closing price after the auction.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 40 * 60,
    },
];
pub(crate) static SIX_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Zurich,
    regular: SIX_REGULAR,
    extended: SIX_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

// Spain BME: 09:00–17:30 continuous; opening auction 08:30–09:00; closing
// auction 17:30–17:35; Trading-at-Last to 17:45. Source: FESE 2025 hours
// table, BME row — "08:30 - 09:00 Opening Auction; 17:30 - 17:35 Closing
// Auction; 17:35 - 17:45 Trading At Last".
static BME_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 35 * 60,
    },
    // Trading-at-Last 17:35–17:45.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 35 * 60,
        close_ssm: 17 * 3600 + 45 * 60,
    },
];
pub(crate) static BME_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Madrid,
    regular: REG_0900_1730,
    extended: BME_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

// Vienna: continuous 09:00–17:30 (the ~09:04 auction-uncross jitter is not
// representable in a normal-week SSM model, so the opening call is clipped at
// the nominal 09:00 continuous start); opening auction from 08:55; closing
// auction 17:30–17:35; Trade-at-Close to 17:45. Source: FESE 2025 hours
// table, Vienna Stock Exchange row — "08:55 - ~09:04 Opening Auction; 17:30 -
// 17:35 Closing Auction; 17:35 - 17:45 Trade at close".
static VIENNA_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 55 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 35 * 60,
    },
    // Trade-at-Close 17:35–17:45.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 35 * 60,
        close_ssm: 17 * 3600 + 45 * 60,
    },
];
pub(crate) static VIENNA_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: REG_0900_1730,
    extended: VIENNA_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};
