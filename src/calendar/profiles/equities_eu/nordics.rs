// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! The Nasdaq Nordic books: Stockholm, Helsinki, Copenhagen.
//!
//! The three books synchronise on CET but publish local hours and close at
//! different times, so each has its own profile — sharing the continental
//! 09:00–17:30 table here would put Helsinki an hour off all day.

use chrono_tz::Europe;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

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
