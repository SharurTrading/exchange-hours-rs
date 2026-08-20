// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! US futures venue tables (CME Globex complex plus Cboe Futures), all
//! `US/Central`.
//!
//! These are the profiles where the wrap convention earns its keep: the Globex
//! trading day opens the *previous* evening and closes the next afternoon, so
//! every overnight rule has `open_ssm > close_ssm`. The Sunday-plus-Mon–Thu mask
//! is what encodes "no Friday-evening open" — the week ends at Friday's close
//! and does not reopen until Sunday evening.
//!
//! The daily 16:00–17:00 CT break is **not** a rule. It is the gap between one
//! wrap session's close and the next one's open, which is what makes
//! [`MarketHours::is_maintenance`](crate::MarketHours::is_maintenance) able to
//! recognise it generically instead of per venue.
//!
//! `*_PRE*` / `*_POST*` pairs record published hour changes and are reachable
//! only through [`hours_for_exchange_as_of`](crate::hours_for_exchange_as_of).

use chrono_tz::US;

use super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::rule::SUN_PLUS_MON_THU;

// CFE (VIX) — current schedule, effective 2021-12-06.
//
// "Following these changes, the trading hours for VX, VXM, and AMERIBOR futures
// on Monday through Friday will consist of ETH from 5:00 p.m. CT (prior day) to
// 8:30 a.m. CT, RTH from 8:30 a.m. CT to 3:00 p.m., and ETH from 3:00 p.m. CT to
// 4:00 p.m. CT … there will be no pause in trading at 3:00 p.m. CT and RTH will
// seamlessly transition to ETH at 3:00 p.m. CT."
// Source: Cboe notice C2021102603, "CFE to Update Trading Hours and Adjust
// Pre-Open Sequence for TAS and Non-TAS Contracts" (effective 2021-12-06); rule
// certification CFE-2021-028 (2021-11-04), "All times referenced in this
// submission are in Chicago time."
pub(crate) static CFE_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 15 * 3600,
}];
pub(crate) static CFE_EXTENDED: &[SessionRule] = &[
    // Post-settlement ETH, seamless from the 15:00 RTH close. Mon–Fri: the
    // 2021 change also removed the 15:15–15:30 queuing period that used to
    // separate the two.
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600,
        close_ssm: 16 * 3600,
    },
    // Overnight wrap: 17:00 CT the prior day → 08:30 CT.
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
];
pub(crate) static CFE_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CFE_REGULAR,
    extended: CFE_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

// CFE from 2014-06-01 until the 2021-12-06 change: RTH 08:30–15:15, a dead
// queuing period 15:15–15:30 in which no trading takes place, then ETH
// 15:30–16:00, plus the same overnight wrap.
//
// "These trading hours currently include extended trading hours from 5:00 p.m.
// the previous day to 8:30 a.m., regular trading hours from 8:30 a.m. to 3:15
// p.m., and extended trading hours from 3:30 p.m. to 4:00 p.m. There is
// currently a queuing period between 3:15 p.m. and 3:30 p.m. during which no
// trading takes place…" — CFE-2021-028.
static CFE_REGULAR_PRE_2021_12_06: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 15 * 3600 + 15 * 60,
}];
static CFE_EXT_PRE_2021_12_06: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 30 * 60,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
];
pub(crate) static CFE_PROFILE_PRE_2021_12_06: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CFE_REGULAR_PRE_2021_12_06,
    extended: CFE_EXT_PRE_2021_12_06,
    has_daily_close: true,
    has_weekend_close: true,
};

// CFE pre-2014: no overnight; RTH 08:30–15:15 + 15:30–16:00 curb.
static CFE_EXT_PRE2014: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 15 * 3600 + 30 * 60,
    close_ssm: 16 * 3600,
}];
pub(crate) static CFE_PROFILE_PRE2014: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CFE_REGULAR_PRE_2021_12_06,
    extended: CFE_EXT_PRE2014,
    has_daily_close: true,
    has_weekend_close: true,
};

// CME (Equity index): pre-2016 uses 15:30–16:15, post-2016 uses 15:30–16:00; both have 17:00–08:30 wrap
pub(crate) static CME_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 15 * 3600 + 15 * 60,
}];
static CME_EXT_PRE2016: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 15 * 60,
    },
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
];
pub(crate) static CME_EXT_POST2016: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 30 * 60,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
];
pub(crate) static CME_PROFILE_PRE2016: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CME_REGULAR,
    extended: CME_EXT_PRE2016,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static CME_PROFILE_POST2016: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CME_REGULAR,
    extended: CME_EXT_POST2016,
    has_daily_close: true,
    has_weekend_close: true,
};

// CBOT (Grains/Oilseeds): post-2013 day 08:30–13:20, overnight 19:00–07:45; pre-2013 overnight 17:00–07:45 and day 08:30–13:15
pub(crate) static CBOT_REGULAR_POST2013: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 20 * 60,
}];
pub(crate) static CBOT_EXT_POST2013: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 19 * 3600,
    close_ssm: 7 * 3600 + 45 * 60,
}];
pub(crate) static CBOT_PROFILE_POST2013: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CBOT_REGULAR_POST2013,
    extended: CBOT_EXT_POST2013,
    has_daily_close: true,
    has_weekend_close: true,
};
static CBOT_REGULAR_PRE2013: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 15 * 60,
}];
static CBOT_EXT_PRE2013: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 7 * 3600 + 45 * 60,
}];
pub(crate) static CBOT_PROFILE_PRE2013: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CBOT_REGULAR_PRE2013,
    extended: CBOT_EXT_PRE2013,
    has_daily_close: true,
    has_weekend_close: true,
};

// COMEX / NYMEX: 17:00–16:00 wrap (maintenance 16:00–17:00), no Fri overnight.
// Both venues reach this table through the shared `GlobexEnergy` futures profile
// rather than through a per-venue `StaticHoursProfile`.
pub(crate) static MAINT_17_16_EXT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];
