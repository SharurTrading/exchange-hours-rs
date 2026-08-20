// SPDX-License-Identifier: MIT-0

//! US equities and US options venue tables (all `America/New_York`).
//!
//! Reg NMS gives every lit US equities venue the same 09:30–16:00 ET regular
//! session, so [`US_EQUITY_REGULAR`] is shared and the venues differ only in
//! their extended windows — which genuinely differ: the Nasdaq markets, NYSE
//! Arca, Cboe BZX/EDGX, MEMX, and MIAX Pearl open their early session at
//! 04:00 ET, while NYSE American, NYSE National, NYSE Texas, and Cboe
//! BYX/EDGA open at 07:00, NYSE itself trades the core session only, IEX runs
//! System Hours 08:00–17:00, `IntelligentCross` accepts orders from 09:00
//! without executing, and Blue Ocean ATS is overnight-only — its single rule
//! wraps past midnight (`open_ssm > close_ssm`).
//!
//! `*_PRE*` / `*_POST*` pairs exist where a venue changed its published hours;
//! only [`super::super::hours_for_exchange_as_of`] selects between them, and
//! `hours_for_exchange` always names the current one.

use chrono_tz::America;

use super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::rule::SUN_PLUS_MON_THU;

/// Reg NMS regular-hours rule set: 09:30–16:00 ET, Monday–Friday.
///
/// Shared by the US-equities profiles; callers can borrow it into a
/// [`MarketHours`](crate::MarketHours) without allocating.
pub static US_EQUITY_REGULAR: &[SessionRule] = &[SessionRule {
    // 09:30–16:00 ET (Mon–Fri)
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 16 * 3600,
}];

/// Reg NMS extended-hours rule set: 04:00–09:30 ET pre-market and 16:00–20:00 ET
/// post-market, Monday–Friday.
pub static US_EQUITY_EXTENDED: &[SessionRule] = &[
    // 04:00–09:30 ET
    SessionRule {
        days: MON_FRI,
        open_ssm: 4 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    // 16:00–20:00 ET
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 20 * 3600,
    },
];

/// NYSE Texas extended-hours rule set: opening 07:00–09:30 ET and late
/// 16:00–20:00 ET slices that bracket the shared US-equities regular session.
pub static NYSE_TEXAS_EXTENDED: &[SessionRule] = &[
    // Opening 07:00–09:30 ET
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    // Late 16:00–20:00 ET
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 20 * 3600,
    },
];

/// Blue Ocean ATS overnight rule set: a single 20:00→04:00 ET wrap session on
/// Sunday plus Monday–Thursday (`open_ssm > close_ssm`, so it spills into the
/// next local day).
pub static BLUE_OCEAN_EXTENDED: &[SessionRule] = &[
    // 20:00–04:00 ET (overnight wrap) on Sun + Mon–Thu
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 20 * 3600, // 20:00 ET
        close_ssm: 4 * 3600, // 04:00 ET next day (wrap)
    },
];

// US equities, 04:00 ET early group: RTH + 04:00–09:30 / 16:00–20:00 extended.
// Applies to the venues whose early session actually starts 04:00 ET: the
// Nasdaq markets (Nasdaq/BX/PSX), NYSE Arca ("Early Trading Session 4:00 a.m.
// to 9:30 a.m. ET" — nyse.com, "Trading Hours"), Cboe BZX and EDGX ("Early
// Trading Session: 4:00 a.m. to 8:00 a.m. … Post-Market Session: 4:00 p.m. to
// 8:00 p.m." — cboe.com, "Hours & Holidays"; the 8:00–9:30 window is Cboe's
// separately named Pre-Market Session, also continuous trading), MEMX (memx.com
// insights: "MEMX began trading at 4:00 a.m. ET"), and MIAX Pearl Equities
// ("members are able to trade beginning at 4:00 AM ET … until 8:00 PM ET" —
// MIAX alert 2024-11-13 and Pearl Equities trade-hours page).
pub(crate) static US_EQUITIES_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: US_EQUITY_REGULAR,
    extended: US_EQUITY_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

/// Early-at-07:00 extended set: 07:00–09:30 ET early session and 16:00–20:00 ET
/// late session. NYSE American and NYSE National run "Early Trading Session
/// 7:00 a.m. to 9:30 a.m. ET / Late Trading Session 4:00 p.m. to 8:00 p.m. ET"
/// (nyse.com, "Trading Hours"); Cboe BYX and EDGA run "Early Trading Session:
/// 7:00 a.m. to 8:00 a.m." followed by the continuous Pre-Market Session to
/// 9:30 (cboe.com, "Hours & Holidays").
static EXT_EARLY_0700: &[SessionRule] = &[
    // Early 07:00–09:30 ET
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    // Late 16:00–20:00 ET
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 20 * 3600,
    },
];

// NYSE American / NYSE National / Cboe BYX / Cboe EDGA: early opens 07:00 ET.
pub(crate) static US_EQUITY_EARLY_0700_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: US_EQUITY_REGULAR,
    extended: EXT_EARLY_0700,
    has_daily_close: true,
    has_weekend_close: true,
};

// NYSE (XNYS) itself trades the core session only: "Core: 9:30 a.m. to
// 4:00 p.m. ET" with no early and no late session (nyse.com, "Trading
// Hours" — early/late trading on NYSE Group venues happens on Arca, American,
// National, and Texas). The 6:30 a.m. pre-opening is order queuing for the
// opening auction, not a trading session, so it is deliberately not modeled.
pub(crate) static NYSE_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: US_EQUITY_REGULAR,
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// NYSE Texas adds opening+late windows to the US equities regular
pub(crate) static NYSE_TEXAS_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: US_EQUITY_REGULAR,
    extended: NYSE_TEXAS_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

// FINRA TRF (08:00–20:00 ET) — no extended
static FINRA_TRF_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 20 * 3600,
}];
pub(crate) static FINRA_TRF_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: FINRA_TRF_REGULAR,
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// IntelligentCross: RTH + 09:00–09:30 window (orders accepted)
static IQX_EXTENDED: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 9 * 3600 + 30 * 60,
}];
pub(crate) static IQX_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: US_EQUITY_REGULAR,
    extended: IQX_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

// Blue Ocean ATS (overnight only)
pub(crate) static BLUE_OCEAN_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: &[],
    extended: BLUE_OCEAN_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

// US Options (default): 09:30–16:00 ET
static US_OPTIONS_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 16 * 3600,
}];
pub(crate) static US_OPTIONS_DEFAULT_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: US_OPTIONS_REGULAR,
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// IEX: RTH only (pre-2015), then add 08:00–09:30 and 16:00–17:00 (post-2015)
static IEX_PRE_EXT: &[SessionRule] = &[];
static IEX_POST_EXT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
];
pub(crate) static IEX_PROFILE_PRE2015: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: US_EQUITY_REGULAR,
    extended: IEX_PRE_EXT,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static IEX_PROFILE_POST2015: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: US_EQUITY_REGULAR,
    extended: IEX_POST_EXT,
    has_daily_close: true,
    has_weekend_close: true,
};

// US Options C1 (Cboe) — same RTH, GTH end changes 2024-08-26
static C1_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 16 * 3600 + 15 * 60,
}];
static C1_EXT_PRE_2024_08_26: &[SessionRule] = &[
    // GTH wrap Sun+Mon–Thu 20:15 → 09:15
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 20 * 3600 + 15 * 60,
        close_ssm: 9 * 3600 + 15 * 60,
    },
    // Curb 16:15–17:00
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600,
    },
];
static C1_EXT_POST_2024_08_26: &[SessionRule] = &[
    // GTH wrap Sun+Mon–Thu 20:15 → 09:25
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 20 * 3600 + 15 * 60,
        close_ssm: 9 * 3600 + 25 * 60,
    },
    // Curb 16:15–17:00
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600,
    },
];
pub(crate) static C1_PROFILE_PRE_2024_08_26: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: C1_REGULAR,
    extended: C1_EXT_PRE_2024_08_26,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static C1_PROFILE_POST_2024_08_26: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: C1_REGULAR,
    extended: C1_EXT_POST_2024_08_26,
    has_daily_close: true,
    has_weekend_close: true,
};
