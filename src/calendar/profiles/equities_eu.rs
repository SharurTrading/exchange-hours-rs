// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! EU equities venue tables.
//!
//! Almost every continental venue runs the same 09:00–17:30 local continuous
//! session, so [`REG_0900_1730`] is shared and the venues differ only by time
//! zone and by when their opening auction starts (08:30 in Madrid, 08:45 across
//! Euronext and the Nordics, 08:50 on Xetra and SIX). London is the odd one out
//! at 08:00–16:30.
//!
//! Auction windows are modeled as **extended** sessions rather than as part of
//! the regular session: they are periods when the venue is doing something but
//! continuous trading is not open, which is exactly the regular/extended split.

use chrono_tz::Europe;

use super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// LSE: 08:00–16:30; auctions 07:50–08:00 and 16:30–16:35
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
];
pub(crate) static LSE_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::London,
    regular: LSE_REGULAR,
    extended: AU_0750_0800_1630_1635,
    has_daily_close: true,
    has_weekend_close: true,
};

// Xetra/SIX: 09:00–17:30; auctions 08:50–09:00 and 17:30–17:35
static REG_0900_1730: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 17 * 3600 + 30 * 60,
}];
static AU_0850_0900_1730_1735: &[SessionRule] = &[
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
];
pub(crate) static XETRA_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: REG_0900_1730,
    extended: AU_0850_0900_1730_1735,
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

// Euronext: 09:00–17:30; auctions 08:45–09:00 and 17:30–17:35
static AU_0845_0900_1730_1735: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 45 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 35 * 60,
    },
];
pub(crate) static EURONEXT_PARIS_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Paris,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static EURONEXT_AMS_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Amsterdam,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static EURONEXT_BRU_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Brussels,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static EURONEXT_LIS_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Lisbon,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static EURONEXT_DUB_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Dublin,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static EURONEXT_MIL_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Rome,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};

// Spain BME: 09:00–17:30; auctions 08:30–09:00 and 17:30–17:35
static AU_0830_0900_1730_1735: &[SessionRule] = &[
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
];
pub(crate) static BME_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Madrid,
    regular: REG_0900_1730,
    extended: AU_0830_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};

// Nasdaq Nordics: 09:00–17:30; auctions 08:45–09:00 and 17:30–17:35
pub(crate) static NASDAQ_STO_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Stockholm,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static NASDAQ_HEL_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Helsinki,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static NASDAQ_CPH_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Copenhagen,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};

// Vienna: 09:00–17:30; auctions 08:45–09:00 and 17:30–17:35
pub(crate) static VIENNA_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};
