// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! EU equities venue tables.
//!
//! The continental default is a 09:00–17:30 local continuous session
//! ([`REG_0900_1730`]), but the venues genuinely diverge around it: London
//! runs 08:00–16:30, SIX ends continuous trading at 17:20, Euronext Dublin at
//! 17:28, Nasdaq Stockholm at 17:25, Copenhagen at 16:55, and Helsinki trades
//! 10:00–18:25 in its own EET zone (the Nordic books synchronise on CET).
//! Post-close price-formation windows differ too: Euronext, BME, Xetra,
//! Vienna, and SIX run a trading-at-last/trade-at-close session after the
//! closing auction, LSE runs its Closing Price Crossing, and the Nordics do
//! not (except Copenhagen's optional trade-at-close).
//!
//! Auction windows are modeled as **extended** sessions rather than as part of
//! the regular session: they are periods when the venue is doing something but
//! continuous trading is not open, which is exactly the regular/extended split.
//!
//! Where a venue is not individually cited, hours are verified against FESE,
//! "Regular Trading Hours CET in 2025"
//! (<https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf>), which
//! tabulates continuous-trading and auction windows for every European
//! exchange in local time.

use chrono_tz::Europe;

use super::StaticHoursProfile;
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

// The shared continental continuous session, 09:00–17:30 local.
static REG_0900_1730: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 17 * 3600 + 30 * 60,
}];
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

// Euronext (Paris/Amsterdam/Brussels/Lisbon/Milan): 09:00–17:30 continuous;
// pre-open call 08:45–09:00; closing auction call 17:30–17:35 with the uncross
// at ~17:35; then Trading-at-Last — executions at the closing price — to
// 17:40. Source: FESE 2025 hours table, Euronext rows — "17:30 - 17:35
// Pre-closing; 17:35 Closing Auction; 17:35 - 17:40 Trading at Last";
// corroborated by Euronext's cash-market documentation ("Uncrossing is
// performed randomly … between 17:35:00 and 17:35:30, followed by the
// Trading-at-Last phase until 17:40:00 CET"). All times are local, which for
// Lisbon means WET — the shared SSM values hold in each venue's own zone.
static EURONEXT_EXTENDED: &[SessionRule] = &[
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
    // Trading-at-Last 17:35–17:40.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 35 * 60,
        close_ssm: 17 * 3600 + 40 * 60,
    },
];

// Euronext Dublin is the outlier of the family: continuous trading ends
// 17:28, the closing auction call runs 17:28–17:30 with the uncross at 17:30,
// and Trading-at-Last runs 17:30–17:40. Source: FESE 2025 hours table,
// Euronext Dublin row — "09:00 (WET) Opening Auction; 17:28 - 17:30 (WET)
// Pre-closing; 17:30 (WET) Closing auction; 17:30 - 17:40 (WET) Trading at
// Last".
static DUBLIN_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 17 * 3600 + 28 * 60,
}];
static DUBLIN_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 45 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 28 * 60,
        close_ssm: 17 * 3600 + 30 * 60,
    },
    // Trading-at-Last 17:30–17:40.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 40 * 60,
    },
];
pub(crate) static EURONEXT_PARIS_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Paris,
    regular: REG_0900_1730,
    extended: EURONEXT_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static EURONEXT_AMS_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Amsterdam,
    regular: REG_0900_1730,
    extended: EURONEXT_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static EURONEXT_BRU_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Brussels,
    regular: REG_0900_1730,
    extended: EURONEXT_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static EURONEXT_LIS_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Lisbon,
    regular: REG_0900_1730,
    extended: EURONEXT_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static EURONEXT_DUB_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Dublin,
    regular: DUBLIN_REGULAR,
    extended: DUBLIN_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static EURONEXT_MIL_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Rome,
    regular: REG_0900_1730,
    extended: EURONEXT_EXTENDED,
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

// Nasdaq Nordics. The three books synchronise on CET but publish local
// hours, and their closes differ. Source: FESE 2025 hours table —
// Stockholm "08:00 - 9:00 Pre-opening; ~9:00 Opening Auction; 17:25 - ~17:30
// Pre-closing; ~17:30 Closing Auction"; Helsinki "09:00 - 10:00 (EET)
// Pre-opening; ~10:00 (EET) Opening Auction; 18:25 - ~18:30 (EET)
// Pre-closing; ~18:30 (EET) Closing Auction"; Copenhagen "08:00 - 9:00
// Pre-opening; ~9:00 Opening Auction; 16:55 - ~17:00 Pre-closing; ~17:00
// Closing Auction; 17:00 - 17:10 Trade at close (optional)".

// Stockholm: continuous 09:00–17:25 CET; pre-open 08:00–09:00; closing
// auction call 17:25–17:30. No post-close trading-at-last.
static STO_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 17 * 3600 + 25 * 60,
}];
static STO_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 25 * 60,
        close_ssm: 17 * 3600 + 30 * 60,
    },
];
pub(crate) static NASDAQ_STO_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Stockholm,
    regular: STO_REGULAR,
    extended: STO_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

// Helsinki trades in EET: continuous 10:00–18:25 local; pre-open 09:00–10:00;
// closing auction call 18:25–18:30. The SSM values are one hour later than
// Stockholm's because the zone is one hour ahead — the books align on CET.
static HEL_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 10 * 3600,
    close_ssm: 18 * 3600 + 25 * 60,
}];
static HEL_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 10 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 18 * 3600 + 25 * 60,
        close_ssm: 18 * 3600 + 30 * 60,
    },
];
pub(crate) static NASDAQ_HEL_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Helsinki,
    regular: HEL_REGULAR,
    extended: HEL_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

// Copenhagen: continuous 09:00–16:55 CET; pre-open 08:00–09:00; closing
// auction call 16:55–17:00; optional Trade-at-Close 17:00–17:10.
static CPH_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 16 * 3600 + 55 * 60,
}];
static CPH_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 55 * 60,
        close_ssm: 17 * 3600,
    },
    // Trade-at-Close 17:00–17:10.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600,
        close_ssm: 17 * 3600 + 10 * 60,
    },
];
pub(crate) static NASDAQ_CPH_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Copenhagen,
    regular: CPH_REGULAR,
    extended: CPH_EXTENDED,
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
