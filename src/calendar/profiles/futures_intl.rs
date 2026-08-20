// SPDX-License-Identifier: MIT-0

//! Non-US futures and energy venue tables: EUREX, the ICE complex, EEX, SGX.
//!
//! Two shapes recur here. The ICE venues wrap (20:00→18:00 ET, so a two-hour
//! daily break), while the ICE Europe / ENDEX / Abu Dhabi contracts run a long
//! same-day 01:00–23:00 window in their own local zone — the shared
//! [`REG_01_23`] table with three different time zones, which is why the local
//! zone, not the SSM values, is what distinguishes them.
//!
//! SGX is the one profile whose extended session wraps on a `MON_FRI` mask
//! rather than `SUN_PLUS_MON_THU`: its Friday T+1 session legitimately runs into
//! Saturday morning SGT, which the weekend tests pin.

use chrono_tz::{America, Europe};

use super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::rule::SUN_PLUS_MON_THU;

// EUREX: with/without Asian slice 01:00–08:00
pub(crate) static EUREX_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 22 * 3600,
}];
pub(crate) static EUREX_ASIAN: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3600,
    close_ssm: 8 * 3600,
}];
pub(crate) static EUREX_PROFILE_WITH_ASIAN: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_REGULAR,
    extended: EUREX_ASIAN,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static EUREX_PROFILE_NO_ASIAN: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_REGULAR,
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// ICE US/Canada: wrapped 20:00 → 18:00 ET
pub(crate) static ICE_WRAP_20_18_EXT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 20 * 3600,
    close_ssm: 18 * 3600,
}];
// ICE Futures U.S. shares this table through the `IceUs` futures profile rather
// than through a per-venue `StaticHoursProfile`.
pub(crate) static ICE_CANADA_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: &[],
    extended: ICE_WRAP_20_18_EXT,
    has_daily_close: true,
    has_weekend_close: true,
};

// ICE Europe and ENDEX: 01:00–23:00 local
static REG_01_23: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3600,
    close_ssm: 23 * 3600,
}];
pub(crate) static ICE_EU_LONDON_01_23_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::London,
    regular: REG_01_23,
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static ENDEX_01_23_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Amsterdam,
    regular: REG_01_23,
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static ABU_DHABI_01_23_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: chrono_tz::Asia::Dubai,
    regular: REG_01_23,
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// EEX: 08:00–18:00 CET/CEST
static REG_08_18: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 18 * 3600,
}];
pub(crate) static EEX_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: REG_08_18,
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// SGX: Day 07:10–20:00; T+1 wrap 20:00 → 05:15. SGX reaches these tables through
// the `Sgx` futures profile rather than through a per-venue `StaticHoursProfile`.
pub(crate) static SGX_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 10 * 60,
    close_ssm: 20 * 3600,
}];
pub(crate) static SGX_EXTENDED: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 20 * 3600,
    close_ssm: 5 * 3600 + 15 * 60,
}];
