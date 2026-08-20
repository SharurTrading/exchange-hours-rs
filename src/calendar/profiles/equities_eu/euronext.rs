// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! The Euronext cash markets: Paris, Amsterdam, Brussels, Lisbon, Dublin,
//! Milan.
//!
//! Five of the six share one schedule stated in their own local zones —
//! 09:00–17:30 continuous, closing auction call to ~17:35, Trading-at-Last to
//! 17:40. Dublin is the family outlier: continuous ends 17:28 and its
//! Trading-at-Last runs 17:30–17:40.

use chrono_tz::Europe;

use super::super::StaticHoursProfile;
use super::REG_0900_1730;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

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
