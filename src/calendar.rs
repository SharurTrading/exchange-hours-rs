// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

#![forbid(unsafe_code)]

//!
//! Market hours: exchange trading calendars, sessions, and bar boundary helpers.
//!
//! This module models exchange trading hours at the exchange level (per [`Exchange`])
//! and exposes utilities to query whether a market is open at a given UTC instant,
//! compute session bounds, and find candle end boundaries that respect trading hours.
//!
//! Key concepts:
//! - Time semantics: All helpers take and return `chrono::DateTime<Utc>`. The
//!   exchange's local time zone (`tz`) is used only internally to interpret session
//!   rules and handle DST transitions correctly.
//! - Session rules: Each exchange is described by a set of [`SessionRule`]s for
//!   "regular" and/or "extended" trading. A rule activates on specific weekdays and
//!   defines an open and close time as seconds since local midnight (SSM). If
//!   `open_ssm <= close_ssm`, the session runs within the same local day; if
//!   `open_ssm > close_ssm`, the session wraps past midnight and closes the next
//!   local day.
//! - Weekday indexing: Weekdays are indexed Monday=0 through Sunday=6. The `days`
//!   mask in [`SessionRule`] follows this convention throughout the module.
//! - Holidays: If an exchange-local calendar date is marked as a holiday, no
//!   sessions occur on that local date. Wrapped sessions from "yesterday" do not
//!   bleed into a holiday; i.e., a previous-day wrap is ignored if the previous
//!   local date is a holiday.
//! - Maintenance windows: A convenience heuristic [`MarketHours::is_maintenance`]
//!   returns true when the market is currently closed but the next session starts
//!   within ~90 minutes (useful for CME’s daily maintenance break).
//!
//! What you can do with this module:
//! - Check open/closed state at an instant: [`MarketHours::is_open`],
//!   [`MarketHours::is_open_with`], [`MarketHours::is_open_regular`],
//!   [`MarketHours::is_open_extended`].
//! - Find the trading session that contains a time (or the next one):
//!   [`session_bounds`], [`session_bounds_with`], [`next_session_after`],
//!   [`next_session_after_with`], [`next_session_open_after`].
//! - Compute bar start/end times that align with the market calendar:
//!   [`candle_start`] and [`candle_end`] (alias [`time_end_of_day`]). Intraday
//!   bars step to the next permitted boundary skipping closed periods;
//!   Daily/Weekly/Monthly bars use canonical session-period boundaries.
//!
//! Exchange presets:
//! - [`hours_for_exchange`] provides pragmatic default hours for major venues
//!   (CME, CBOT, COMEX, NYMEX, EUREX, ICEUS, ICEEU, SGX, CFE). These are exchange-
//!   level defaults for common product profiles (e.g., CME equity-index futures):
//!   Sun 17:00 – Fri 16:00 Central, with a daily break 16:00–17:00, RTH 08:30–15:15,
//!   and a brief 15:30–16:00 window. Always verify specific contract rules when
//!   needed; product-level calendars can be layered later if required.
//!
//! DST and ambiguous times:
//! - Local times are resolved with bias-aware helpers: opens prefer the **earliest**
//!   mapping, closes (end-exclusive) prefer the **latest** mapping. If a local time
//!   is skipped (spring-forward gap), we pick the earliest valid instant *after* the
//!   gap to ensure a valid instant is produced.
//! - Internal comparisons that determine “next open after T” are performed on
//!   **instants**, not just SSM, to avoid fall-back anomalies.
//!
//! Invariants and nuances:
//! - All comparisons against session close are end-exclusive: the instant equal to
//!   `close_ssm` is considered closed. This avoids overlapping sessions.
//! - "Wrapped" sessions are represented by `open_ssm > close_ssm`. When inside a
//!   wrap, times before `close_ssm` on the next day are still considered inside the
//!   same session, unless the next day is a holiday (no wrap may bleed into a holiday).
//! - The calendar-wide checks (`is_closed_all_day_*`) interpret a "day" in the
//!   specified calendar TZ and correctly handle DST while checking for any session
//!   overlap within that window.
//!
//! Test coverage for the normal-week futures baseline lives in
//! `market_hours/tests.rs`.

use std::borrow::Cow;
use std::collections::BTreeMap;

use chrono::{
    DateTime, Datelike, Duration, LocalResult, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc,
};
use chrono_tz::UTC;
use chrono_tz::{America, Asia, Europe, Tz, US};
use serde::{Deserialize, Serialize};
use std::sync::Once;
use tracing::warn;

const SECONDS_PER_NORMAL_DAY: u64 = 86_400;
const SECONDS_PER_NORMAL_WEEK: u64 = 7 * SECONDS_PER_NORMAL_DAY;

// -----------------------------------------------------------------------------
// Core types
// -----------------------------------------------------------------------------

/// Identifies an exchange or trading venue.
///
/// Variants are grouped by product family (US equities, US options, US futures,
/// European futures/energy, Asia-Pacific futures, EU equities, and always-open
/// crypto venues). The enum is exhaustive, so adding a venue forces the match in
/// [`hours_for_exchange`] to be updated and keeps the calendar surface complete.
///
/// Holidays and product-level calendar variations are deliberately not modeled
/// here: this enum drives only normal-week, exchange-level session defaults.
/// [`Exchange::Unknown`] maps to a 24×7 UTC fallback. Variants serialize as
/// `snake_case` strings (e.g. `Exchange::NasdaqBx` ↔ `"nasdaq_bx"`); that wire
/// form is asserted by the `exchange_serde_snake_case_*` tests and must stay
/// stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Exchange {
    /// Unrecognized or unset venue; [`hours_for_exchange`] returns a 24×7 UTC
    /// fallback and logs a one-shot warning.
    Unknown,

    // US Equities (ET)
    Nasdaq,
    NasdaqBx,
    NasdaqPsx,
    CboeBzx,
    CboeByx,
    CboeEdga,
    CboeEdgx,
    Nyse,
    NyseArca,
    NyseAmerican,
    NyseNational,
    NyseTexas,
    MemxEq,
    MiaxPearlEq,
    Iex,
    IntelligentcrossIqx,
    BlueOceanAts,
    FinraTrfCarteret,
    FinraTrfChicago,
    FinraTrfNyse,

    // US Options (ET)
    CboeOptionsC1,
    CboeC2Options,
    CboeBzxOptions,
    CboeEdgxOptions,
    NyseArcaOptions,
    NyseAmericanOptions,
    NasdaqPhlx,
    NasdaqIse,
    NasdaqNom,
    NasdaqMrx,
    NasdaqGemx,
    NasdaqBxOptions,
    MiaxOptions,
    MiaxEmeraldOptions,
    MiaxPearlOptions,
    MiaxSapphireOptions,
    BoxOptions,
    MemxOptions,

    // US Futures
    Cme,
    Cbot,
    Comex,
    Nymex,
    Cfe,

    // European Futures / Energy
    Eurex,
    Eex,
    Iceus,
    Iceeu,
    IceEuropeCommodities,
    IceEuropeFinancials,
    IceEndex,
    IceAbuDhabi,
    IceCanada,

    // Asia-Pacific Futures
    Sgx,

    // EU Equities
    Lse,
    Xetra,
    Six,
    EuronextParis,
    EuronextAmsterdam,
    EuronextBrussels,
    EuronextLisbon,
    EuronextDublin,
    EuronextMilan,
    Bme,
    NasdaqStockholm,
    NasdaqHelsinki,
    NasdaqCopenhagen,
    Vienna,

    // Crypto / always-open venues
    //
    // These venues trade continuously without daily maintenance breaks.
    // They are modeled as 24×7 UTC sessions and must remain explicitly
    // separated from futures-session venues that have daily and weekend
    // boundaries. See `default_24x7` for the profile and the
    // `always_open_*` tests for the behavioral contract.
    BinanceFutures,
}

/// Bar resolution / time interval for candle-boundary computations.
///
/// Intraday variants carry a positive interval count; [`candle_end`] steps by
/// that interval, clamped to the enclosing session and snapped past maintenance
/// gaps. [`CalendarResolution::Daily`], [`CalendarResolution::Weekly`], and
/// [`CalendarResolution::Monthly`] resolve to canonical session-period
/// boundaries instead of a fixed grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CalendarResolution {
    /// Fixed-grid bar spanning the given number of seconds.
    Seconds(u32),
    /// Fixed-grid bar spanning the given number of minutes.
    Minutes(u32),
    /// Fixed-grid bar spanning the given number of hours.
    Hours(u32),
    /// One bar per trading day, ending at the day's session close.
    Daily,
    /// One bar per ISO week, ending at the last daily close in that week.
    Weekly,
    /// One bar per calendar month, ending at the last daily close in that month.
    Monthly,
}

const US_EQUITY_EXCHANGES: &[Exchange] = &[
    Exchange::Nasdaq,
    Exchange::NasdaqBx,
    Exchange::NasdaqPsx,
    Exchange::CboeBzx,
    Exchange::CboeByx,
    Exchange::CboeEdga,
    Exchange::CboeEdgx,
    Exchange::Nyse,
    Exchange::NyseArca,
    Exchange::NyseAmerican,
    Exchange::NyseNational,
    Exchange::NyseTexas,
    Exchange::MemxEq,
    Exchange::MiaxPearlEq,
    Exchange::Iex,
    Exchange::IntelligentcrossIqx,
    Exchange::BlueOceanAts,
];

const EU_EQUITY_EXCHANGES: &[Exchange] = &[
    Exchange::Lse,
    Exchange::Xetra,
    Exchange::Six,
    Exchange::EuronextParis,
    Exchange::EuronextAmsterdam,
    Exchange::EuronextBrussels,
    Exchange::EuronextLisbon,
    Exchange::EuronextDublin,
    Exchange::EuronextMilan,
    Exchange::Bme,
    Exchange::NasdaqStockholm,
    Exchange::NasdaqHelsinki,
    Exchange::NasdaqCopenhagen,
    Exchange::Vienna,
];

// Common weekday masks (Mon=0 .. Sun=6) used across presets.
// Monday–Friday open days (no weekend trading)
const MON_FRI: [bool; 7] = [true, true, true, true, true, false, false];
// Open on Sunday and Mon–Thu (i.e., Sunday evening open, no Friday overnight open)
const SUN_PLUS_MON_THU: [bool; 7] = [true, true, true, true, false, false, true];
// Open on all seven weekdays.
const ALL_DAYS: [bool; 7] = [true, true, true, true, true, true, true];

// -----------------------------------------------------------------------------
// Static session rules for common US equity patterns (borrowed at runtime)
// -----------------------------------------------------------------------------

/// Reg NMS regular-hours rule set: 09:30–16:00 ET, Monday–Friday.
///
/// Shared by the US-equities profiles; callers can borrow it into a
/// [`MarketHours`] without allocating.
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

// -----------------------------------------------------------------------------
// Static Market Hours Profiles (shared by many venues)
// -----------------------------------------------------------------------------
#[derive(Clone, Copy)]
struct StaticHoursProfile {
    tz: Tz,
    regular: &'static [SessionRule],
    extended: &'static [SessionRule],
    has_daily_close: bool,
    has_weekend_close: bool,
}

#[inline]
fn from_profile(exchange: Exchange, p: &'static StaticHoursProfile) -> MarketHours {
    MarketHours {
        exchange,
        tz: p.tz,
        regular: Cow::Borrowed(p.regular),
        extended: Cow::Borrowed(p.extended),
        has_daily_close: p.has_daily_close,
        has_weekend_close: p.has_weekend_close,
    }
}

// US equities (Reg NMS): RTH + pre/post
static US_EQUITIES_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: US_EQUITY_REGULAR,
    extended: US_EQUITY_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

// NYSE Texas adds opening+late windows to the US equities regular
static NYSE_TEXAS_PROFILE: StaticHoursProfile = StaticHoursProfile {
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
static FINRA_TRF_PROFILE: StaticHoursProfile = StaticHoursProfile {
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
static IQX_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: US_EQUITY_REGULAR,
    extended: IQX_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

// Blue Ocean ATS (overnight only)
static BLUE_OCEAN_PROFILE: StaticHoursProfile = StaticHoursProfile {
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
static US_OPTIONS_DEFAULT_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: US_OPTIONS_REGULAR,
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// CFE (VIX): RTH + curb + overnight
static CFE_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 15 * 3600 + 15 * 60,
}];
static CFE_EXTENDED: &[SessionRule] = &[
    // curb
    SessionRule {
        days: [true, true, true, true, false, false, false],
        open_ssm: 15 * 3600 + 30 * 60,
        close_ssm: 16 * 3600,
    },
    // overnight wrap
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
];
static CFE_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CFE_REGULAR,
    extended: CFE_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

// CME (Equity index): pre-2016 uses 15:30–16:15, post-2016 uses 15:30–16:00; both have 17:00–08:30 wrap
static CME_REGULAR: &[SessionRule] = &[SessionRule {
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
static CME_EXT_POST2016: &[SessionRule] = &[
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
static CME_PROFILE_PRE2016: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CME_REGULAR,
    extended: CME_EXT_PRE2016,
    has_daily_close: true,
    has_weekend_close: true,
};
static CME_PROFILE_POST2016: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CME_REGULAR,
    extended: CME_EXT_POST2016,
    has_daily_close: true,
    has_weekend_close: true,
};

// CBOT (Grains/Oilseeds): post-2013 day 08:30–13:20, overnight 19:00–07:45; pre-2013 overnight 17:00–07:45 and day 08:30–13:15
static CBOT_REGULAR_POST2013: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 20 * 60,
}];
static CBOT_EXT_POST2013: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 19 * 3600,
    close_ssm: 7 * 3600 + 45 * 60,
}];
static CBOT_PROFILE_POST2013: StaticHoursProfile = StaticHoursProfile {
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
static CBOT_PROFILE_PRE2013: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CBOT_REGULAR_PRE2013,
    extended: CBOT_EXT_PRE2013,
    has_daily_close: true,
    has_weekend_close: true,
};

// EUREX: with/without Asian slice 01:00–08:00
static EUREX_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 22 * 3600,
}];
static EUREX_ASIAN: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3600,
    close_ssm: 8 * 3600,
}];
static EUREX_PROFILE_WITH_ASIAN: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_REGULAR,
    extended: EUREX_ASIAN,
    has_daily_close: true,
    has_weekend_close: true,
};
static EUREX_PROFILE_NO_ASIAN: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_REGULAR,
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
static IEX_PROFILE_PRE2015: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: US_EQUITY_REGULAR,
    extended: IEX_PRE_EXT,
    has_daily_close: true,
    has_weekend_close: true,
};
static IEX_PROFILE_POST2015: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: US_EQUITY_REGULAR,
    extended: IEX_POST_EXT,
    has_daily_close: true,
    has_weekend_close: true,
};

// COMEX / NYMEX: 17:00–16:00 wrap (maintenance 16:00–17:00), no Fri overnight
static MAINT_17_16_EXT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];
#[allow(
    dead_code,
    reason = "historical GUI-local profile; COMEX now delegates to instrument-catalog"
)]
static COMEX_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: MAINT_17_16_EXT,
    has_daily_close: true,
    has_weekend_close: true,
};
#[allow(
    dead_code,
    reason = "historical GUI-local profile; NYMEX now delegates to instrument-catalog"
)]
static NYMEX_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: MAINT_17_16_EXT,
    has_daily_close: true,
    has_weekend_close: true,
};

// ICE US/Canada: wrapped 20:00 → 18:00 ET
static ICE_WRAP_20_18_EXT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 20 * 3600,
    close_ssm: 18 * 3600,
}];
#[allow(
    dead_code,
    reason = "historical GUI-local profile; ICEUS now delegates to instrument-catalog"
)]
static ICEUS_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: &[],
    extended: ICE_WRAP_20_18_EXT,
    has_daily_close: true,
    has_weekend_close: true,
};
static ICE_CANADA_PROFILE: StaticHoursProfile = StaticHoursProfile {
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
static ICE_EU_LONDON_01_23_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::London,
    regular: REG_01_23,
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
static ENDEX_01_23_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Amsterdam,
    regular: REG_01_23,
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
static ABU_DHABI_01_23_PROFILE: StaticHoursProfile = StaticHoursProfile {
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
static EEX_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: REG_08_18,
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// SGX: Day 07:10–20:00; T+1 wrap 20:00 → 05:15
static SGX_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 10 * 60,
    close_ssm: 20 * 3600,
}];
static SGX_EXTENDED: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 20 * 3600,
    close_ssm: 5 * 3600 + 15 * 60,
}];
#[allow(
    dead_code,
    reason = "historical GUI-local profile; SGX now delegates to instrument-catalog"
)]
static SGX_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_REGULAR,
    extended: SGX_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

// EU Equities profiles
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
static LSE_PROFILE: StaticHoursProfile = StaticHoursProfile {
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
static XETRA_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: REG_0900_1730,
    extended: AU_0850_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};
static SIX_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Zurich,
    regular: REG_0900_1730,
    extended: AU_0850_0900_1730_1735,
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
static EURONEXT_PARIS_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Paris,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};
static EURONEXT_AMS_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Amsterdam,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};
static EURONEXT_BRU_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Brussels,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};
static EURONEXT_LIS_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Lisbon,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};
static EURONEXT_DUB_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Dublin,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};
static EURONEXT_MIL_PROFILE: StaticHoursProfile = StaticHoursProfile {
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
static BME_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Madrid,
    regular: REG_0900_1730,
    extended: AU_0830_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};

// Nasdaq Nordics: 09:00–17:30; auctions 08:45–09:00 and 17:30–17:35
static NASDAQ_STO_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Stockholm,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};
static NASDAQ_HEL_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Helsinki,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};
static NASDAQ_CPH_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Copenhagen,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
    has_daily_close: true,
    has_weekend_close: true,
};

// Vienna: 09:00–17:30; auctions 08:45–09:00 and 17:30–17:35
static VIENNA_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: REG_0900_1730,
    extended: AU_0845_0900_1730_1735,
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
static C1_PROFILE_PRE_2024_08_26: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: C1_REGULAR,
    extended: C1_EXT_PRE_2024_08_26,
    has_daily_close: true,
    has_weekend_close: true,
};
static C1_PROFILE_POST_2024_08_26: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: C1_REGULAR,
    extended: C1_EXT_POST_2024_08_26,
    has_daily_close: true,
    has_weekend_close: true,
};

// CFE pre-2014: no overnight; RTH + 15:30–16:00 curb
static CFE_EXT_PRE2014: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 15 * 3600 + 30 * 60,
    close_ssm: 16 * 3600,
}];
static CFE_PROFILE_PRE2014: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: CFE_REGULAR,
    extended: CFE_EXT_PRE2014,
    has_daily_close: true,
    has_weekend_close: true,
};

/// One schedule slice for a market session.
///
/// - `days`: Monday=0 .. Sunday=6 mask; `true` enables the rule on that weekday.
/// - `open_ssm` / `close_ssm`: seconds since local midnight in the exchange time zone.
///   If `open_ssm <= close_ssm` the session is same-day; if `open_ssm > close_ssm`
///   the session wraps into the next local day and closes at `close_ssm` there.
/// - Close comparisons are end-exclusive: an instant exactly equal to `close_ssm`
///   is considered closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionRule {
    /// Weekday activation mask (Mon=0 .. Sun=6).
    pub days: [bool; 7],
    /// Open time in seconds since local midnight.
    pub open_ssm: u32,
    /// Close time in seconds since local midnight, end-exclusive.
    pub close_ssm: u32,
}

/// A timezone-aware set of normal-week futures session rules.
///
/// `regular` carries primary trading sessions; `extended` carries electronic,
/// overnight, and other non-regular sessions. Holiday and special-session
/// overlays are deliberately modeled outside this normal-week profile.
pub struct FuturesSessionProfile {
    /// Exchange local timezone used to interpret `SessionRule` SSM values.
    pub tz: Tz,
    /// Primary trading sessions.
    pub regular: &'static [SessionRule],
    /// Electronic, overnight, and other non-regular sessions.
    pub extended: &'static [SessionRule],
    /// True when the venue has a distinct daily close.
    pub has_daily_close: bool,
    /// True when the venue has a true weekend close.
    pub has_weekend_close: bool,
}

impl FuturesSessionProfile {
    /// Returns `true` when any regular or extended normal-week session is active.
    #[must_use]
    pub fn is_open(&self, t: DateTime<Utc>) -> bool {
        let hours = MarketHours {
            exchange: Exchange::Unknown,
            tz: self.tz,
            regular: Cow::Borrowed(self.regular),
            extended: Cow::Borrowed(self.extended),
            has_daily_close: self.has_daily_close,
            has_weekend_close: self.has_weekend_close,
        };
        hours.is_open(t)
    }
}

/// Names a SHARUR-owned normal-week market-hours profile.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketHoursKey {
    /// CME equity-index Globex hours.
    GlobexEquityIndex,
    /// Shared 23-hour Globex hours for NYMEX/COMEX energy and metals, CBOT rates, and CME FX.
    GlobexEnergy,
    /// CBOT grains/oilseeds Globex hours.
    GlobexGrains,
    /// CME FX Globex hours.
    GlobexFx,
    /// CFE VIX futures hours.
    CfeVix,
    /// EUREX index and interest-rate futures hours.
    Eurex,
    /// ICE Futures U.S. common profile.
    IceUs,
    /// SGX derivatives generic profile.
    Sgx,
    /// Continuous 24x7 UTC profile.
    AlwaysOpen,
}

static ALWAYS_OPEN_RULE: &[SessionRule] = &[SessionRule {
    days: ALL_DAYS,
    open_ssm: 0,
    close_ssm: 24 * 3600,
}];

static FUTURES_GLOBEX_EQUITY_INDEX: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: CME_REGULAR,
    extended: CME_EXT_POST2016,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_GLOBEX_ENERGY: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: MAINT_17_16_EXT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_GLOBEX_GRAINS: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: CBOT_REGULAR_POST2013,
    extended: CBOT_EXT_POST2013,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_CFE_VIX: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: CFE_REGULAR,
    extended: CFE_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_EUREX: FuturesSessionProfile = FuturesSessionProfile {
    tz: Europe::Berlin,
    regular: EUREX_REGULAR,
    extended: EUREX_ASIAN,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_ICE_US: FuturesSessionProfile = FuturesSessionProfile {
    tz: America::New_York,
    regular: &[],
    extended: ICE_WRAP_20_18_EXT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_SGX: FuturesSessionProfile = FuturesSessionProfile {
    tz: Asia::Singapore,
    regular: SGX_REGULAR,
    extended: SGX_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_ALWAYS_OPEN: FuturesSessionProfile = FuturesSessionProfile {
    tz: UTC,
    regular: ALWAYS_OPEN_RULE,
    extended: &[],
    has_daily_close: false,
    has_weekend_close: false,
};

/// Returns the normal-week futures session profile for a well-known key.
#[must_use]
pub fn session_profile(key: MarketHoursKey) -> &'static FuturesSessionProfile {
    match key {
        MarketHoursKey::GlobexEquityIndex => &FUTURES_GLOBEX_EQUITY_INDEX,
        MarketHoursKey::GlobexEnergy | MarketHoursKey::GlobexFx => &FUTURES_GLOBEX_ENERGY,
        MarketHoursKey::GlobexGrains => &FUTURES_GLOBEX_GRAINS,
        MarketHoursKey::CfeVix => &FUTURES_CFE_VIX,
        MarketHoursKey::Eurex => &FUTURES_EUREX,
        MarketHoursKey::IceUs => &FUTURES_ICE_US,
        MarketHoursKey::Sgx => &FUTURES_SGX,
        MarketHoursKey::AlwaysOpen => &FUTURES_ALWAYS_OPEN,
    }
}

/// Resolves a [`MarketHoursKey`] to the [`MarketHours`] value the calendar query
/// surface ([`candle_end`], [`session_bounds`], …) consumes.
///
/// This is a field copy of the same static [`FuturesSessionProfile`] table
/// [`session_profile`] returns — not a second source of truth — mirroring the
/// inline construction in [`FuturesSessionProfile::is_open`]. The `exchange` tag
/// is [`Exchange::Unknown`] because the key, not a venue enum, identifies these
/// shared futures profiles.
#[must_use]
pub fn hours_for_market_hours_key(key: MarketHoursKey) -> MarketHours {
    let profile = session_profile(key);
    MarketHours {
        exchange: Exchange::Unknown,
        tz: profile.tz,
        regular: Cow::Borrowed(profile.regular),
        extended: Cow::Borrowed(profile.extended),
        has_daily_close: profile.has_daily_close,
        has_weekend_close: profile.has_weekend_close,
    }
}

/// Exchange-level trading hours definition.
///
/// This is a pragmatic calendar describing typical hours for an exchange. It may not
/// capture product-specific exceptions; consult contract specs for exact rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketHours {
    /// Which exchange these hours represent.
    #[allow(
        dead_code,
        reason = "retained in the test-only market-hours harness; tests inspect session rules, not the tag"
    )]
    pub exchange: Exchange,
    /// Exchange’s local time zone (used to interpret `SessionRule`s and handle DST).
    pub tz: Tz,
    /// Primary/pit ("regular") trading sessions (e.g., RTH for equities/futures).
    /// Uses Cow so callers can borrow from static tables without allocation.
    pub regular: Cow<'static, [SessionRule]>,
    /// Electronic/overnight and other non-regular sessions.
    /// Uses Cow so callers can borrow from static tables without allocation.
    pub extended: Cow<'static, [SessionRule]>,
    /// True if there is a distinct daily close (used for daily candle boundaries).
    pub has_daily_close: bool,
    /// True if the exchange has a true weekend close (used for weekly boundaries).
    pub has_weekend_close: bool,
}

/// Selects which session set a query consults.
///
/// `MarketHours` keeps `regular` and `extended` rule sets separately; every
/// open/closed and boundary query takes a `SessionKind` to choose between them.
#[derive(Debug, Clone, Copy)]
pub enum SessionKind {
    /// Consult only primary/RTH (`regular`) sessions.
    Regular,
    /// Consult only electronic/overnight/auction (`extended`) sessions.
    Extended,
    /// Consult regular and extended sessions together.
    Both,
}

impl MarketHours {
    #[inline]
    fn iter_rules(&self, kind: SessionKind) -> impl Iterator<Item = &SessionRule> + '_ {
        let reg = self.regular.iter();
        let ext = self.extended.iter();
        match kind {
            SessionKind::Regular => reg.take(usize::MAX).chain(ext.take(0)),
            SessionKind::Extended => reg.take(0).chain(ext.take(usize::MAX)),
            SessionKind::Both => reg.take(usize::MAX).chain(ext.take(usize::MAX)),
        }
    }

    /// Returns the number of distinct scheduled open seconds in this profile's
    /// normal week, consulting regular and extended sessions together.
    ///
    /// Overlapping and adjacent rules are unioned before their durations are
    /// summed, and rules that wrap across the Sunday/Monday boundary are split
    /// at that boundary. Holiday and daylight-saving overlays are intentionally
    /// excluded because [`MarketHours`] is a normal-week profile.
    #[must_use]
    pub fn normal_week_open_seconds(&self) -> u64 {
        let mut intervals = self
            .iter_rules(SessionKind::Both)
            .flat_map(normal_week_rule_intervals)
            .collect::<Vec<_>>();
        intervals.sort_unstable();

        let mut merged = Vec::<(u64, u64)>::new();
        for (start, end) in intervals {
            if let Some((_, previous_end)) = merged.last_mut()
                && start <= *previous_end
            {
                *previous_end = (*previous_end).max(end);
            } else {
                merged.push((start, end));
            }
        }
        merged
            .into_iter()
            .map(|(start, end)| end.saturating_sub(start))
            .sum::<u64>()
            .min(SECONDS_PER_NORMAL_WEEK)
    }

    /// True if **any** (regular or extended) session is open at `t`.
    #[must_use]
    pub fn is_open(&self, t: DateTime<Utc>) -> bool {
        self.is_open_with(t, SessionKind::Both)
    }

    /// True if a session of the requested kind is open at `t`.
    #[must_use]
    pub fn is_open_with(&self, t: DateTime<Utc>, kind: SessionKind) -> bool {
        let local = t.with_timezone(&self.tz);
        // Holidays are modeled via status; exchange-level defaults do not gate by holidays.
        let w_today = local.weekday().num_days_from_monday() as usize;
        let ssm = local.num_seconds_from_midnight();

        // Direct rule evaluation (no exchange-specific hard gates; breaks are modeled as rules)
        if self.iter_rules(kind).any(|r| {
            if !r.days[w_today] {
                return false;
            }
            if r.open_ssm <= r.close_ssm {
                ssm >= r.open_ssm && ssm < r.close_ssm
            } else {
                // wrap (open today, close next day)
                ssm >= r.open_ssm || ssm < r.close_ssm
            }
        }) {
            return true;
        }

        // Yesterday's wrap may spill into today unless yesterday was a holiday.
        let yday_date = local.date_naive() - Duration::days(1);
        let yday = yday_date.weekday().num_days_from_monday() as usize;
        self.iter_rules(kind).any(|r| {
            if r.open_ssm <= r.close_ssm {
                return false;
            }
            if !r.days[yday] {
                return false;
            }
            ssm < r.close_ssm
        })
    }

    /// True if a **regular** (primary/RTH) session is open at `t`.
    ///
    /// Shorthand for [`is_open_with`](Self::is_open_with) with
    /// [`SessionKind::Regular`]; extended/overnight sessions are ignored.
    #[must_use]
    pub fn is_open_regular(&self, t: DateTime<Utc>) -> bool {
        self.is_open_with(t, SessionKind::Regular)
    }

    /// True if an **extended** (electronic/overnight/auction) session is open at `t`.
    ///
    /// Shorthand for [`is_open_with`](Self::is_open_with) with
    /// [`SessionKind::Extended`]; regular sessions are ignored.
    #[must_use]
    pub fn is_open_extended(&self, t: DateTime<Utc>) -> bool {
        self.is_open_with(t, SessionKind::Extended)
    }

    /// True if `t` falls in a short pre-open "maintenance" gap.
    ///
    /// Heuristic: the market is currently closed **and** the next session opens
    /// within 90 minutes (across regular and extended sessions). This captures
    /// daily maintenance breaks such as CME's 16:00–17:00 CT window without
    /// modeling them as explicit rules. Always-open venues are never in
    /// maintenance because they are never closed.
    #[must_use]
    pub fn is_maintenance(&self, t: DateTime<Utc>) -> bool {
        // “Maintenance” := closed now but next session is within ~90 minutes.
        if self.is_open(t) {
            return false;
        }
        let (open, _close) = next_session_after(self, t);
        (open - t) <= chrono::Duration::minutes(90)
    }

    /// Return true iff the market is closed for the entire **calendar day** `day`
    /// interpreted in `calendar_tz`. This checks whether *any* session (of `kind`)
    /// intersects the `[day@00:00, next_day@00:00)` window.
    ///
    /// # Panics
    ///
    /// Panics only if Chrono cannot resolve the fallback local time used for an
    /// exceptional midnight DST gap, or if `day` is the maximum representable
    /// `NaiveDate` and therefore has no successor.
    #[must_use]
    pub fn is_closed_all_day_in_calendar(
        &self,
        day: NaiveDate,
        calendar_tz: Tz,
        kind: SessionKind,
    ) -> bool {
        // Convert the calendar day's bounds to UTC, DST-safe.
        fn midnight_utc_for(tz: Tz, d: NaiveDate) -> DateTime<Utc> {
            match tz.with_ymd_and_hms(d.year(), d.month(), d.day(), 0, 0, 0) {
                LocalResult::Single(dt) => dt.with_timezone(&Utc),
                LocalResult::Ambiguous(a, b) => a.min(b).with_timezone(&Utc),
                LocalResult::None => {
                    // Extremely rare at midnight; fallback to 01:00 local.
                    tz.with_ymd_and_hms(d.year(), d.month(), d.day(), 1, 0, 0)
                        .single()
                        .expect("resolvable local time")
                        .with_timezone(&Utc)
                }
            }
        }

        let start_utc = midnight_utc_for(calendar_tz, day);
        let end_utc = midnight_utc_for(calendar_tz, day.succ_opt().expect("valid next day"));

        // NOTE: Do not short-circuit on exchange-local holidays here; a calendar
        // day in another TZ can still overlap valid trading. Decide strictly by overlap.

        // If any session is active at window start → not closed all day.
        if self.is_open_with(start_utc, kind) {
            return false;
        }

        // Otherwise, check the *next* session after the window start; if it opens
        // before the window ends, the day is not fully closed.
        let (next_open, _next_close) = next_session_after_with(kind, self, start_utc);
        next_open >= end_utc
    }

    /// Convenience: interpret the date in the **exchange TZ** (what your old
    /// `is_closed_all_day_on` did).
    #[inline]
    #[must_use]
    pub fn is_closed_all_day_on(&self, local_day: NaiveDate, kind: SessionKind) -> bool {
        self.is_closed_all_day_in_calendar(local_day, self.tz, kind)
    }

    /// Convenience: starting from a UTC timestamp, use a chosen calendar TZ to define “the day”.
    #[inline]
    #[allow(
        dead_code,
        reason = "retained in the test-only market-hours harness for scheduled calendar checks"
    )]
    #[must_use]
    pub fn is_closed_all_day_at(
        &self,
        ts_utc: DateTime<Utc>,
        calendar_tz: Tz,
        kind: SessionKind,
    ) -> bool {
        let day = ts_utc.with_timezone(&calendar_tz).date_naive();
        self.is_closed_all_day_in_calendar(day, calendar_tz, kind)
    }
}

fn normal_week_rule_intervals(rule: &SessionRule) -> Vec<(u64, u64)> {
    let mut intervals = Vec::with_capacity(8);
    for (weekday, enabled) in (0_u64..).zip(rule.days) {
        if !enabled {
            continue;
        }
        let start = weekday
            .saturating_mul(SECONDS_PER_NORMAL_DAY)
            .saturating_add(u64::from(rule.open_ssm));
        let duration = if rule.open_ssm <= rule.close_ssm {
            u64::from(rule.close_ssm - rule.open_ssm)
        } else {
            SECONDS_PER_NORMAL_DAY
                .saturating_sub(u64::from(rule.open_ssm))
                .saturating_add(u64::from(rule.close_ssm))
        };
        let end = start.saturating_add(duration);
        if end <= SECONDS_PER_NORMAL_WEEK {
            intervals.push((start, end));
        } else {
            intervals.push((start, SECONDS_PER_NORMAL_WEEK));
            intervals.push((0, end.saturating_sub(SECONDS_PER_NORMAL_WEEK)));
        }
    }
    intervals
}

/// Build default futures trading hours per exchange.
/// NOTE: These are exchange-level defaults. Product-level variations may differ.
#[expect(
    clippy::too_many_lines,
    reason = "exhaustive match over all Exchange variants; splitting would add indirection without clarity"
)]
pub fn hours_for_exchange(exch: Exchange) -> MarketHours {
    match exch {
        Exchange::Unknown => {
            static ONCE: Once = Once::new();
            ONCE.call_once(|| {
                warn!("hours_for_exchange: Unknown exchange encountered; using 24x7 UTC fallback");
            });
            default_24x7(exch)
        }
        // ==============================
        // US EQUITIES (ET)
        // ==============================
        Exchange::Nasdaq
        | Exchange::NasdaqBx
        | Exchange::NasdaqPsx
        | Exchange::CboeBzx
        | Exchange::CboeByx
        | Exchange::CboeEdga
        | Exchange::CboeEdgx
        | Exchange::Nyse
        | Exchange::NyseArca
        | Exchange::NyseAmerican
        | Exchange::NyseNational
        | Exchange::MemxEq
        | Exchange::MiaxPearlEq => from_profile(exch, &US_EQUITIES_PROFILE),

        // IEX (executes only RTH; keep pre for uniformity if you prefer)
        Exchange::Iex => from_profile(Exchange::Iex, &US_EQUITIES_PROFILE),

        // NYSE Texas — opening 07:00–09:30, core 09:30–16:00, late 16:00–20:00 ET.
        Exchange::NyseTexas => from_profile(Exchange::NyseTexas, &NYSE_TEXAS_PROFILE),

        // FINRA TRFs — facility open 08:00–20:00 ET.
        Exchange::FinraTrfCarteret | Exchange::FinraTrfChicago | Exchange::FinraTrfNyse => {
            from_profile(exch, &FINRA_TRF_PROFILE)
        }

        // IntelligentCross — executes RTH 09:30–16:00; accepts orders from 09:00.
        Exchange::IntelligentcrossIqx => from_profile(Exchange::IntelligentcrossIqx, &IQX_PROFILE),

        // Blue Ocean ATS — 20:00–04:00 ET Sun + Mon–Thu (overnight).
        Exchange::BlueOceanAts => from_profile(Exchange::BlueOceanAts, &BLUE_OCEAN_PROFILE),

        // ==============================
        // US OPTIONS (ET)
        // ==============================
        // Cboe Options (C1) — handled in as_of cutover below
        Exchange::CboeOptionsC1 => {
            from_profile(Exchange::CboeOptionsC1, &C1_PROFILE_POST_2024_08_26)
        }

        // Other options venues — pragmatic default RTH 09:30–16:00 ET.
        Exchange::CboeC2Options
        | Exchange::CboeBzxOptions
        | Exchange::CboeEdgxOptions
        | Exchange::NyseArcaOptions
        | Exchange::NyseAmericanOptions
        | Exchange::NasdaqPhlx
        | Exchange::NasdaqIse
        | Exchange::NasdaqNom
        | Exchange::NasdaqMrx
        | Exchange::NasdaqGemx
        | Exchange::NasdaqBxOptions
        | Exchange::MiaxOptions
        | Exchange::MiaxEmeraldOptions
        | Exchange::MiaxPearlOptions
        | Exchange::MiaxSapphireOptions
        | Exchange::BoxOptions
        | Exchange::MemxOptions => from_profile(exch, &US_OPTIONS_DEFAULT_PROFILE),

        // ==============================
        // FUTURES / ENERGY (EU/INTL)
        // ==============================
        // ICE Europe Commodities — many contracts 01:00–23:00 London
        Exchange::IceEuropeCommodities => {
            from_profile(Exchange::IceEuropeCommodities, &ICE_EU_LONDON_01_23_PROFILE)
        }
        // ICE Europe Financials — same broad window. (Individual products vary; override if needed.)
        Exchange::IceEuropeFinancials => {
            from_profile(Exchange::IceEuropeFinancials, &ICE_EU_LONDON_01_23_PROFILE)
        }
        // ICE Endex (Amsterdam) — pragmatic default 01:00–23:00 local.
        Exchange::IceEndex => from_profile(Exchange::IceEndex, &ENDEX_01_23_PROFILE),
        // ICE Abu Dhabi (IFAD) — nearly 24×5 (22–24h); default to 01:00–23:00 GST.
        Exchange::IceAbuDhabi => from_profile(Exchange::IceAbuDhabi, &ABU_DHABI_01_23_PROFILE),
        // ICE Canada — default to 20:00→18:00 ET wrap (ICE pattern).
        Exchange::IceCanada => from_profile(Exchange::IceCanada, &ICE_CANADA_PROFILE),
        // EEX — exchange trading generally 08:00–18:00 CET.
        Exchange::Eex => from_profile(Exchange::Eex, &EEX_PROFILE),

        // ----------------------- EU Equities -----------------------
        // LSE (UK): 08:00–16:30; pre-open auction 07:50–08:00; closing auction 16:30–16:35
        Exchange::Lse => from_profile(Exchange::Lse, &LSE_PROFILE),

        // Xetra / Frankfurt: 09:00–17:30; auctions 08:50–09:00 and 17:30–17:35
        Exchange::Xetra => from_profile(Exchange::Xetra, &XETRA_PROFILE),

        // SIX Swiss: 09:00–17:30; small auctions modeled similarly
        Exchange::Six => from_profile(Exchange::Six, &SIX_PROFILE),

        // Euronext venues (typical): 09:00–17:30, auctions 08:45–09:00 & 17:30–17:35
        Exchange::EuronextParis => from_profile(Exchange::EuronextParis, &EURONEXT_PARIS_PROFILE),
        Exchange::EuronextAmsterdam => {
            from_profile(Exchange::EuronextAmsterdam, &EURONEXT_AMS_PROFILE)
        }
        Exchange::EuronextBrussels => {
            from_profile(Exchange::EuronextBrussels, &EURONEXT_BRU_PROFILE)
        }
        Exchange::EuronextLisbon => from_profile(Exchange::EuronextLisbon, &EURONEXT_LIS_PROFILE),
        Exchange::EuronextDublin => from_profile(Exchange::EuronextDublin, &EURONEXT_DUB_PROFILE),
        Exchange::EuronextMilan => from_profile(Exchange::EuronextMilan, &EURONEXT_MIL_PROFILE),

        // Spain (BME): 09:00–17:30; pre-open 08:30–09:00; closing auction 17:30–17:35
        Exchange::Bme => from_profile(Exchange::Bme, &BME_PROFILE),

        // Nasdaq Nordic (stockholm/helsinki/copenhagen): 09:00–17:30; auctions 08:45–09:00 & 17:30–17:35
        Exchange::NasdaqStockholm => from_profile(Exchange::NasdaqStockholm, &NASDAQ_STO_PROFILE),
        Exchange::NasdaqHelsinki => from_profile(Exchange::NasdaqHelsinki, &NASDAQ_HEL_PROFILE),
        Exchange::NasdaqCopenhagen => from_profile(Exchange::NasdaqCopenhagen, &NASDAQ_CPH_PROFILE),

        // Vienna: 09:00–17:30; auctions like Euronext pattern
        Exchange::Vienna => from_profile(Exchange::Vienna, &VIENNA_PROFILE),
        // ------------------------------------------------------------
        // CME (CME Globex, Equity Index default)
        // Shared GlobexEquityIndex profile (see `session_profile`).
        // Sun 17:00 – Fri 16:00 CT with daily 60-min break at 16:00;
        // RTH 08:30–15:15 CT; short window 15:30–16:00 CT; no Fri overnight.
        // ------------------------------------------------------------
        Exchange::Cme => {
            let p = session_profile(MarketHoursKey::GlobexEquityIndex);
            MarketHours {
                exchange: Exchange::Cme,
                tz: p.tz,
                regular: Cow::Borrowed(p.regular),
                extended: Cow::Borrowed(p.extended),
                has_daily_close: p.has_daily_close,
                has_weekend_close: p.has_weekend_close,
            }
        }

        // ------------------------------------------------------------
        // CBOT (Grains/Oilseeds default)
        // Shared GlobexGrains profile (see `session_profile`).
        // Sun–Thu overnight 19:00–07:45 CT, day 08:30–13:20 CT; no Fri overnight.
        // ------------------------------------------------------------
        Exchange::Cbot => {
            let p = session_profile(MarketHoursKey::GlobexGrains);
            MarketHours {
                exchange: Exchange::Cbot,
                tz: p.tz,
                regular: Cow::Borrowed(p.regular),
                extended: Cow::Borrowed(p.extended),
                has_daily_close: p.has_daily_close,
                has_weekend_close: p.has_weekend_close,
            }
        }

        // ------------------------------------------------------------
        // COMEX (Metals default)
        // Shared GlobexEnergy profile (see `session_profile`).
        // 17:00–16:00 CT, daily maintenance 16:00–17:00; no Fri overnight.
        // ------------------------------------------------------------
        Exchange::Comex => {
            let p = session_profile(MarketHoursKey::GlobexEnergy);
            MarketHours {
                exchange: Exchange::Comex,
                tz: p.tz,
                regular: Cow::Borrowed(p.regular),
                extended: Cow::Borrowed(p.extended),
                has_daily_close: p.has_daily_close,
                has_weekend_close: p.has_weekend_close,
            }
        }

        // ------------------------------------------------------------
        // NYMEX (Energy default)
        // Shared GlobexEnergy profile (see `session_profile`); identical to COMEX.
        // Same “17:00–16:00 CT, daily break at 16:00–17:00”; no Fri overnight.
        // ------------------------------------------------------------
        Exchange::Nymex => {
            let p = session_profile(MarketHoursKey::GlobexEnergy);
            MarketHours {
                exchange: Exchange::Nymex,
                tz: p.tz,
                regular: Cow::Borrowed(p.regular),
                extended: Cow::Borrowed(p.extended),
                has_daily_close: p.has_daily_close,
                has_weekend_close: p.has_weekend_close,
            }
        }

        // ------------------------------------------------------------
        // EUREX (generic index/IR default)
        // Shared Eurex profile (see `session_profile`).
        // Asian hours 01:00–08:00 CET/CEST, then regular 08:00–22:00 CET/CEST (Mon–Fri).
        // ------------------------------------------------------------
        Exchange::Eurex => {
            let p = session_profile(MarketHoursKey::Eurex);
            MarketHours {
                exchange: Exchange::Eurex,
                tz: p.tz,
                regular: Cow::Borrowed(p.regular),
                extended: Cow::Borrowed(p.extended),
                has_daily_close: p.has_daily_close,
                has_weekend_close: p.has_weekend_close,
            }
        }

        // ------------------------------------------------------------
        // ICE Futures U.S. (common profile)
        // Shared IceUs profile (see `session_profile`).
        // Many contracts follow ~20:00–18:00 ET (22h) with daily 2h break; no Fri overnight.
        // ------------------------------------------------------------
        Exchange::Iceus => {
            let p = session_profile(MarketHoursKey::IceUs);
            MarketHours {
                exchange: Exchange::Iceus,
                tz: p.tz,
                regular: Cow::Borrowed(p.regular),
                extended: Cow::Borrowed(p.extended),
                has_daily_close: p.has_daily_close,
                has_weekend_close: p.has_weekend_close,
            }
        }

        // ------------------------------------------------------------
        // ICE Futures Europe (common profile)
        // Typical 01:00–23:00 London (Mon–Fri). Equity-only profile remains GUI-local.
        // ------------------------------------------------------------
        Exchange::Iceeu => from_profile(Exchange::Iceeu, &ICE_EU_LONDON_01_23_PROFILE),

        // ------------------------------------------------------------
        // SGX Derivatives (generic)
        // Shared Sgx profile (see `session_profile`).
        // T session ~07:10–20:00 SGT, T+1 ~20:00–05:15 SGT (Mon–Fri).
        // ------------------------------------------------------------
        Exchange::Sgx => {
            let p = session_profile(MarketHoursKey::Sgx);
            MarketHours {
                exchange: Exchange::Sgx,
                tz: p.tz,
                regular: Cow::Borrowed(p.regular),
                extended: Cow::Borrowed(p.extended),
                has_daily_close: p.has_daily_close,
                has_weekend_close: p.has_weekend_close,
            }
        }

        // ------------------------------------------------------------
        // CFE (Cboe Futures – VIX default profile)
        // Shared CfeVix profile (see `session_profile`).
        // RTH 08:30–15:15; curb 15:30–16:00; overnight wrap Sun+Mon–Thu 17:00→08:30.
        // ------------------------------------------------------------
        Exchange::Cfe => {
            let p = session_profile(MarketHoursKey::CfeVix);
            MarketHours {
                exchange: Exchange::Cfe,
                tz: p.tz,
                regular: Cow::Borrowed(p.regular),
                extended: Cow::Borrowed(p.extended),
                has_daily_close: p.has_daily_close,
                has_weekend_close: p.has_weekend_close,
            }
        }

        // ------------------------------------------------------------
        // Crypto / always-open venues
        //
        // Shared AlwaysOpen profile (see `session_profile`).
        // 24×7 UTC, no maintenance break, no weekend close, midnight UTC boundaries.
        // Deliberately NOT modeled as futures-session venues; the always-open
        // contract is kept explicit and separate from CME-style daily-break calendars.
        //
        // Note: Product-level nuances such as Binance quarterly expiry pauses are
        // deferred to a later product-level calendar layer.
        // ------------------------------------------------------------
        Exchange::BinanceFutures => {
            let p = session_profile(MarketHoursKey::AlwaysOpen);
            MarketHours {
                exchange: Exchange::BinanceFutures,
                tz: p.tz,
                regular: Cow::Borrowed(p.regular),
                extended: Cow::Borrowed(p.extended),
                has_daily_close: p.has_daily_close,
                has_weekend_close: p.has_weekend_close,
            }
        } // All known exchanges covered; no default arm.
    }
}

/// Time-aware market hours: returns the exchange-level `MarketHours` profile appropriate
/// for the provided `as_of` timestamp. Defaults to the latest profile if no historical
/// revision is defined for the exchange.
///
/// # Panics
///
/// Panics only if one of the hard-coded historical cutover dates in this
/// function is invalid.
#[must_use]
pub fn hours_for_exchange_as_of(exch: Exchange, as_of: DateTime<Utc>) -> MarketHours {
    match exch {
        // Cboe Options (C1): GTH end extended from 09:15 → 09:25 ET on 2024-08-26.
        Exchange::CboeOptionsC1 => {
            let d = as_of.with_timezone(&America::New_York).date_naive();
            if d < chrono::NaiveDate::from_ymd_opt(2024, 8, 26).unwrap() {
                from_profile(Exchange::CboeOptionsC1, &C1_PROFILE_PRE_2024_08_26)
            } else {
                from_profile(Exchange::CboeOptionsC1, &C1_PROFILE_POST_2024_08_26)
            }
        }

        // CME equity index defaults:
        //  - < 2016-03-04: RTH 08:30–15:15; short window 15:30–16:15
        //  - >= 2016-03-04: RTH 08:30–15:15; short window 15:30–16:00
        Exchange::Cme => {
            let d = as_of.with_timezone(&US::Central).date_naive();
            if d < chrono::NaiveDate::from_ymd_opt(2016, 3, 4).unwrap() {
                from_profile(Exchange::Cme, &CME_PROFILE_PRE2016)
            } else {
                from_profile(Exchange::Cme, &CME_PROFILE_POST2016)
            }
        }

        // EUREX: Asian hours added 2018-12-10. Before that, omit the 01:00–08:00 slice.
        Exchange::Eurex => {
            let d = as_of.with_timezone(&Europe::Berlin).date_naive();
            let asian = d >= chrono::NaiveDate::from_ymd_opt(2018, 12, 10).unwrap();
            if asian {
                from_profile(Exchange::Eurex, &EUREX_PROFILE_WITH_ASIAN)
            } else {
                from_profile(Exchange::Eurex, &EUREX_PROFILE_NO_ASIAN)
            }
        }

        // IEX: Pre 08:00–09:30 and Post 16:00–17:00 begin 2015-08-21; before then, use RTH only.
        Exchange::Iex => {
            let d = as_of.with_timezone(&America::New_York).date_naive();
            let has_ext = d >= chrono::NaiveDate::from_ymd_opt(2015, 8, 21).unwrap();
            if has_ext {
                from_profile(Exchange::Iex, &IEX_PROFILE_POST2015)
            } else {
                from_profile(Exchange::Iex, &IEX_PROFILE_PRE2015)
            }
        }

        // CFE (VIX): before 2014-06-01, approximate as RTH + curb only (no overnight).
        Exchange::Cfe => {
            let d = as_of.with_timezone(&US::Central).date_naive();
            if d < chrono::NaiveDate::from_ymd_opt(2014, 6, 1).unwrap() {
                from_profile(Exchange::Cfe, &CFE_PROFILE_PRE2014)
            } else {
                from_profile(Exchange::Cfe, &CFE_PROFILE)
            }
        }

        // NYSE Texas: go-live 2025-03-31; before that, no trading sessions.
        Exchange::NyseTexas => {
            let d = as_of.with_timezone(&America::New_York).date_naive();
            if d < chrono::NaiveDate::from_ymd_opt(2025, 3, 31).unwrap() {
                MarketHours {
                    exchange: Exchange::NyseTexas,
                    tz: America::New_York,
                    regular: Vec::new().into(),
                    extended: Vec::new().into(),
                    has_daily_close: true,
                    has_weekend_close: true,
                }
            } else {
                from_profile(Exchange::NyseTexas, &NYSE_TEXAS_PROFILE)
            }
        }

        // Blue Ocean ATS: assume go-live ~2021-01-01; prior to that, no sessions.
        Exchange::BlueOceanAts => {
            let d = as_of.with_timezone(&America::New_York).date_naive();
            if d < chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap() {
                MarketHours {
                    exchange: Exchange::BlueOceanAts,
                    tz: America::New_York,
                    regular: Vec::new().into(),
                    extended: Vec::new().into(),
                    has_daily_close: true,
                    has_weekend_close: true,
                }
            } else {
                from_profile(Exchange::BlueOceanAts, &BLUE_OCEAN_PROFILE)
            }
        }

        // CBOT: reduced hours effective 2013-04-08; before that, use 17:00–07:45 overnight and 08:30–13:15 day.
        Exchange::Cbot => {
            let d = as_of.with_timezone(&US::Central).date_naive();
            if d < chrono::NaiveDate::from_ymd_opt(2013, 4, 8).unwrap() {
                from_profile(Exchange::Cbot, &CBOT_PROFILE_PRE2013)
            } else {
                from_profile(Exchange::Cbot, &CBOT_PROFILE_POST2013)
            }
        }

        // Default: return the current exchange profile.
        _ => hours_for_exchange(exch),
    }
}

// ---------------------------
// Session helpers
// ---------------------------

/// Returns the `[open, close)` UTC bounds of the session of `kind` that contains `t`.
///
/// When `t` lies inside a session, that session's bounds are returned. Otherwise
/// the search falls back, in order, to: (1) a previous-day wrap session that
/// spills into `t`'s local day (unless the open day is a holiday), then (2) the
/// next session strictly after `t` via [`next_session_after_with`]. Bounds are
/// end-exclusive on the close; opens resolve to the earliest DST mapping and
/// closes to the latest.
///
/// # Panics
///
/// Panics only on a malformed calendar — if resolving a session's local
/// open/close instant exhausts the bounded DST-gap search (see
/// [`next_session_after_with`]).
#[must_use]
pub fn session_bounds_with(
    kind: SessionKind,
    hours: &MarketHours,
    t: DateTime<Utc>,
) -> (DateTime<Utc>, DateTime<Utc>) {
    let local = t.with_timezone(&hours.tz);
    let day = local.date_naive();
    let ssm = i64::from(local.num_seconds_from_midnight());

    let w_today = day.weekday().num_days_from_monday() as usize;
    let yday_date = day - chrono::Duration::days(1);
    let yday = yday_date.weekday().num_days_from_monday() as usize;

    // 1) Try all of TODAY's rules.
    for r in hours.iter_rules(kind).filter(|r| r.days[w_today]) {
        if (i64::from(r.open_ssm)) <= (i64::from(r.close_ssm)) {
            // same-day
            if ssm >= i64::from(r.open_ssm) && ssm < i64::from(r.close_ssm) {
                let open = mk_local_open(hours.tz, day, r.open_ssm);
                let close = mk_local_close(hours.tz, day, r.close_ssm);
                return (open.with_timezone(&Utc), close.with_timezone(&Utc));
            }
        } else {
            // wrap (open today, close tomorrow)
            if ssm >= i64::from(r.open_ssm) {
                // ensure tomorrow isn't a holiday (no wrap bleed into holiday)
                if is_holiday(hours, day + chrono::Duration::days(1)) {
                    continue;
                }
                let open = mk_local_open(hours.tz, day, r.open_ssm);
                let close = mk_local_close(hours.tz, day + chrono::Duration::days(1), r.close_ssm);
                return (open.with_timezone(&Utc), close.with_timezone(&Utc));
            }
        }
    }

    // 2) Try all of YESTERDAY's WRAP rules that spill into today (unless yesterday was a holiday).
    if !is_holiday(hours, yday_date) {
        for r in hours.iter_rules(kind).filter(|r| r.days[yday]) {
            if r.open_ssm > r.close_ssm {
                // yesterday had a wrap; if we're before today's wrap close, we are inside it
                if ssm < i64::from(r.close_ssm) {
                    let open = mk_local_open(hours.tz, day - chrono::Duration::days(1), r.open_ssm);
                    let close = mk_local_close(hours.tz, day, r.close_ssm);
                    return (open.with_timezone(&Utc), close.with_timezone(&Utc));
                }
            }
        }
    }

    // 3) Otherwise, fall forward to the next session after t.
    next_session_after_with(kind, hours, t)
}

/// Returns [`session_bounds_with`] over [`SessionKind::Both`] (regular + extended).
///
/// # Panics
///
/// Panics only on a malformed calendar; see [`session_bounds_with`].
#[must_use]
pub fn session_bounds(hours: &MarketHours, t: DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
    session_bounds_with(SessionKind::Both, hours, t)
}

/// Returns the `[open, close)` UTC bounds of the next session of `kind` that
/// opens strictly after `end_excl`.
///
/// Scans up to 14 local days forward, skipping holidays and refusing wrap
/// sessions whose close day is a holiday; within a day it picks the earliest
/// qualifying open. If no session is found in that horizon it returns the
/// degenerate `(end_excl, end_excl)`.
///
/// # Panics
///
/// Panics only on a malformed calendar — if resolving a session's local
/// open/close instant exhausts the bounded DST-gap search in the internal
/// local-time resolver.
#[must_use]
pub fn next_session_after_with(
    kind: SessionKind,
    hours: &MarketHours,
    end_excl: DateTime<Utc>,
) -> (DateTime<Utc>, DateTime<Utc>) {
    let local = end_excl.with_timezone(&hours.tz);
    let base_day = local.date_naive();

    for dd in 0..14 {
        let d = base_day + Duration::days(dd);
        if is_holiday(hours, d) {
            continue;
        }

        let mut best_open: Option<(DateTime<Utc>, DateTime<Utc>)> = None;
        let w = d.weekday().num_days_from_monday() as usize; // Mon=0..Sun=6

        for r in hours.iter_rules(kind) {
            if !r.days[w] {
                continue;
            }

            if r.open_ssm <= r.close_ssm {
                // same-day
                let open_l = mk_local_open(hours.tz, d, r.open_ssm);
                if open_l <= local {
                    continue; // strictly after end_excl
                }
                let close_l = mk_local_close(hours.tz, d, r.close_ssm);
                let cand = (open_l.with_timezone(&Utc), close_l.with_timezone(&Utc));
                best_open = match best_open {
                    None => Some(cand),
                    Some(cur) => Some(if cand.0 < cur.0 { cand } else { cur }),
                };
            } else {
                // wrap (open d, close d+1) — only valid if next day is not a holiday
                if is_holiday(hours, d + Duration::days(1)) {
                    continue;
                }
                let open_l = mk_local_open(hours.tz, d, r.open_ssm);
                if open_l <= local {
                    continue;
                }
                let close_l = mk_local_close(hours.tz, d + Duration::days(1), r.close_ssm);
                let cand = (open_l.with_timezone(&Utc), close_l.with_timezone(&Utc));
                best_open = match best_open {
                    None => Some(cand),
                    Some(cur) => Some(if cand.0 < cur.0 { cand } else { cur }),
                };
            }
        }

        if let Some(b) = best_open {
            return b;
        }
    }
    // Fallback: if nothing found within 14 days, return a degenerate bound.
    (end_excl, end_excl)
}

/// Returns [`next_session_after_with`] over [`SessionKind::Both`] (regular + extended).
///
/// # Panics
///
/// Panics only on a malformed calendar; see [`next_session_after_with`].
#[must_use]
pub fn next_session_after(
    hours: &MarketHours,
    end_excl: DateTime<Utc>,
) -> (DateTime<Utc>, DateTime<Utc>) {
    next_session_after_with(SessionKind::Both, hours, end_excl)
}

// ---------------------------
// Internals
// ---------------------------

fn is_holiday(_hours: &MarketHours, _d: chrono::NaiveDate) -> bool {
    false
}

#[allow(dead_code)]
fn rule_for_date_in(
    hours: &MarketHours,
    d: chrono::NaiveDate,
    kind: SessionKind,
) -> Option<&SessionRule> {
    if is_holiday(hours, d) {
        return None;
    }
    let w = d.weekday().num_days_from_monday() as usize;
    hours.iter_rules(kind).find(|r| r.days[w])
}

#[derive(Clone, Copy)]
enum AmbigBias {
    Earliest,
    Latest,
}

/// Resolve a local wall-clock (day + SSM) into a concrete `DateTime<Tz>`,
/// choosing a deterministic mapping across DST transitions.
/// - Ambiguous: pick earliest/latest according to `bias`.
/// - Skipped (spring forward): pick the earliest valid instant *after* the gap.
fn mk_local_biased(
    tz: Tz,
    day: chrono::NaiveDate,
    ssm: u32,
    bias: AmbigBias,
) -> chrono::DateTime<Tz> {
    let base: NaiveDateTime = day.and_hms_opt(0, 0, 0).unwrap() + Duration::seconds(i64::from(ssm));
    match tz.from_local_datetime(&base) {
        LocalResult::Single(dt) => dt,
        LocalResult::Ambiguous(a, b) => match bias {
            AmbigBias::Earliest => a.min(b),
            AmbigBias::Latest => a.max(b),
        },
        LocalResult::None => {
            // Step forward until we land on a representable instant (bounded).
            // Typical DST gaps are 60 minutes; 180 provides ample headroom.
            let mut trial = base + Duration::minutes(1);
            for _ in 0..180 {
                match tz.from_local_datetime(&trial) {
                    LocalResult::Single(dt) => return dt,
                    LocalResult::Ambiguous(a, b) => {
                        return match bias {
                            AmbigBias::Earliest => a.min(b),
                            AmbigBias::Latest => a.max(b),
                        };
                    }
                    LocalResult::None => {
                        trial += Duration::minutes(1);
                    }
                }
            }
            // If still unresolved (extremely unlikely), assert—calendars should be well-formed.
            panic!("unresolvable local time after DST gap search");
        }
    }
}

#[inline]
fn mk_local_open(tz: Tz, day: chrono::NaiveDate, ssm: u32) -> chrono::DateTime<Tz> {
    mk_local_biased(tz, day, ssm, AmbigBias::Earliest)
}

#[inline]
fn mk_local_close(tz: Tz, day: chrono::NaiveDate, ssm: u32) -> chrono::DateTime<Tz> {
    mk_local_biased(tz, day, ssm, AmbigBias::Latest)
}

/// Returns only the open instant of the next session after `after_utc`.
///
/// Thin projection of [`next_session_after`], returning only its `.0` field.
///
/// # Panics
///
/// Panics only on a malformed calendar; see [`next_session_after_with`].
#[inline]
#[allow(
    dead_code,
    reason = "retained in the test-only market-hours harness; no test exercises this entry point directly"
)]
#[must_use]
pub fn next_session_open_after(mh: &MarketHours, after_utc: DateTime<Utc>) -> DateTime<Utc> {
    next_session_after(mh, after_utc).0
}

/// Returns the session-aware bar close strictly **after** `t` for `res` and `kind`.
///
/// - `Seconds`: a pure `t + interval` step (no session clamping).
/// - `Minutes`/`Hours`: steps from `max(t, session_open)` by the interval,
///   clamped to the enclosing session close. If that close coincides with the
///   venue's daily close and a maintenance gap follows, the end snaps forward to
///   the next session open so bars do not terminate at the maintenance start.
/// - `Daily`: the next day-close after `t` (latest close on a local calendar day).
/// - `Weekly`: the last day-close in `t`'s ISO week.
/// - `Monthly`: the last day-close in `t`'s exchange-local calendar month.
///
/// # Panics
///
/// Panics only on a malformed calendar; see [`session_bounds_with`].
#[must_use]
pub fn candle_end_with(
    hours: &MarketHours,
    t: DateTime<Utc>,
    res: CalendarResolution,
    kind: SessionKind,
) -> DateTime<Utc> {
    match res {
        CalendarResolution::Seconds(s) => t + Duration::seconds(i64::from(s)),
        CalendarResolution::Minutes(m) => {
            let step = Duration::minutes(i64::from(m));
            let (open, close) = session_bounds_with(kind, hours, t);
            let anchor = if t < open { open } else { t };
            let mut end = (anchor + step).min(close);

            // If the computed end lands exactly on the session close and that close is the
            // exchange's daily close (e.g., CME 16:00 CT), skip short “maintenance” gaps by
            // snapping to the next session open. This prevents intraday bars from closing
            // at the maintenance start and instead closes them after the break.
            if end == close {
                let close_day = close.with_timezone(&hours.tz).date_naive();
                if let Some(dc) = daily_close_for_local_day(hours, close_day, kind)
                    && dc == close
                    && hours.is_maintenance(close)
                {
                    let (next_open, _next_close) = next_session_after_with(kind, hours, close);
                    end = next_open;
                }
            }

            end
        }
        CalendarResolution::Hours(h) => {
            let step = Duration::hours(i64::from(h));
            let (open, close) = session_bounds_with(kind, hours, t);
            let anchor = if t < open { open } else { t };
            let mut end = (anchor + step).min(close);

            // Apply the same maintenance-gap skip logic as for minute bars.
            if end == close {
                let close_day = close.with_timezone(&hours.tz).date_naive();
                if let Some(dc) = daily_close_for_local_day(hours, close_day, kind)
                    && dc == close
                    && hours.is_maintenance(close)
                {
                    let (next_open, _next_close) = next_session_after_with(kind, hours, close);
                    end = next_open;
                }
            }

            end
        }
        CalendarResolution::Daily => next_daily_close_after_with(hours, t, kind),
        CalendarResolution::Weekly => next_weekly_close_after_with(hours, t, kind),
        CalendarResolution::Monthly => next_monthly_close_after_with(hours, t, kind),
    }
}

/// Returns [`candle_end_with`] over [`SessionKind::Both`] (regular + extended).
///
/// # Panics
///
/// Panics only on a malformed calendar; see [`candle_end_with`].
#[inline]
#[must_use]
pub fn candle_end(
    hours: &MarketHours,
    t: DateTime<Utc>,
    interval: CalendarResolution,
) -> DateTime<Utc> {
    candle_end_with(hours, t, interval, SessionKind::Both)
}

/// Returns the opening instant paired with [`candle_end`] for `t` and `interval`.
///
/// - `Seconds`: `t`, matching the interval stepped directly from `t`.
/// - `Minutes`/`Hours`: `max(t, session_open)`, matching the anchor used by
///   [`candle_end`].
/// - `Daily`: the first session open after the preceding trading-day close.
/// - `Weekly`: the first session open whose daily close belongs to the target
///   ISO week.
/// - `Monthly`: the first session open whose daily close belongs to the target
///   exchange-local month.
///
/// Calendar starts can fall on the preceding civil day, month, or year. For
/// example, the CME trading day that closes Monday at 16:00 CT starts Sunday at
/// 17:00 CT. Callers converting an end-exclusive provider close marker should
/// query with an instant immediately before that marker.
///
/// # Panics
///
/// Panics only when `hours` is malformed such that [`candle_end`] finds a
/// calendar close but no preceding daily close or subsequent session open can
/// be found within the calendar's bounded search horizons.
#[must_use]
pub fn candle_start(
    hours: &MarketHours,
    t: DateTime<Utc>,
    interval: CalendarResolution,
) -> DateTime<Utc> {
    match interval {
        CalendarResolution::Seconds(_) => t,
        CalendarResolution::Minutes(_) | CalendarResolution::Hours(_) => {
            let (session_open, _) = session_bounds(hours, t);
            t.max(session_open)
        }
        CalendarResolution::Daily | CalendarResolution::Weekly | CalendarResolution::Monthly => {
            calendar_period_start(hours, t, interval)
        }
    }
}

fn calendar_period_start(
    hours: &MarketHours,
    t: DateTime<Utc>,
    interval: CalendarResolution,
) -> DateTime<Utc> {
    let period_end = candle_end(hours, t, interval);
    let end_local = period_end.with_timezone(&hours.tz);
    let end_day = end_local.date_naive();
    let end_iso_week = end_local.iso_week();
    let end_month = (end_local.year(), end_local.month());

    let mut first_close = period_end;
    if interval != CalendarResolution::Daily {
        let mut day = end_day - Duration::days(1);
        for _ in 0..31 {
            let same_period = match interval {
                CalendarResolution::Weekly => day.iso_week() == end_iso_week,
                CalendarResolution::Monthly => (day.year(), day.month()) == end_month,
                CalendarResolution::Daily
                | CalendarResolution::Seconds(_)
                | CalendarResolution::Minutes(_)
                | CalendarResolution::Hours(_) => false,
            };
            if !same_period {
                break;
            }
            if let Some(close) = daily_close_for_local_day(hours, day, SessionKind::Both) {
                first_close = close;
            }
            day -= Duration::days(1);
        }
    }

    let mut day = first_close.with_timezone(&hours.tz).date_naive() - Duration::days(1);
    let mut previous_close = None;
    for _ in 0..21 {
        if let Some(close) = daily_close_for_local_day(hours, day, SessionKind::Both)
            && close < first_close
        {
            previous_close = Some(close);
            break;
        }
        day -= Duration::days(1);
    }
    let Some(previous_close) = previous_close else {
        panic!("calendar period has no preceding daily close");
    };

    // Probe just before the previous close so a continuous session whose next
    // open equals that close (for example a 24×7 UTC profile) is included.
    let Some(open_probe) = previous_close.checked_sub_signed(Duration::nanoseconds(1)) else {
        panic!("calendar period's preceding close cannot be probed");
    };
    let period_start = next_session_after(hours, open_probe).0;
    if period_start < first_close {
        period_start
    } else {
        panic!("calendar period has no session open before its first daily close");
    }
}

/// Returns the next trading-day close after `t` ("end of day").
///
/// Alias for [`candle_end`] with [`CalendarResolution::Daily`].
///
/// # Panics
///
/// Panics only on a malformed calendar; see [`candle_end_with`].
#[inline]
#[allow(
    dead_code,
    reason = "retained in the test-only market-hours harness as a compatibility alias"
)]
#[must_use]
pub fn time_end_of_day(hours: &MarketHours, t: DateTime<Utc>) -> DateTime<Utc> {
    candle_end(hours, t, CalendarResolution::Daily)
}

// ---------- Daily / Weekly ----------

/// Next "day close" strictly after `t` for this market and session kind.
/// A "day close" is the **latest session close that occurs on a local calendar day**.
/// - Same-day sessions contribute closes on that day.
/// - Wrap sessions contribute a close on the **next** local day, but only if the
///   previous day (the open day) is not a holiday (no wrap across holidays).
fn next_daily_close_after_with(
    hours: &MarketHours,
    t: DateTime<Utc>,
    kind: SessionKind,
) -> DateTime<Utc> {
    let tz = hours.tz;
    let mut day = t.with_timezone(&tz).date_naive();

    for _ in 0..21 {
        if let Some(close) = daily_close_for_local_day(hours, day, kind)
            && close > t
        {
            return close;
        }
        day += Duration::days(1);
    }

    // Fallback (shouldn't happen with well-formed calendars)
    next_session_after_with(kind, hours, t).1
}

/// Compute the last close **that occurs on** `day` (exchange-local day).
fn daily_close_for_local_day(
    hours: &MarketHours,
    day: NaiveDate,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    // If the close occurs on `day`, we may have two sources:
    //  (A) same-day sessions on `day`
    //  (B) wrap sessions opened on `day-1` that close on `day` (only if `day-1` not a holiday)
    let tz = hours.tz;

    if is_holiday(hours, day) {
        return None;
    }

    let w_today = day.weekday().num_days_from_monday() as usize;

    let mut best: Option<DateTime<Utc>> = None;

    for r in hours.iter_rules(kind) {
        if r.open_ssm <= r.close_ssm {
            // same-day close occurs on `day` if rule is active on `day`
            if r.days[w_today] {
                let close = mk_local_close(tz, day, r.close_ssm).with_timezone(&Utc);
                best = Some(best.map_or(close, |b| b.max(close)));
            }
        } else {
            // wrap: close occurs on `day` if:
            // - `day-1` is active in rule
            // - `day-1` is NOT a holiday (no wrap bleed into holiday)
            let yday = day - Duration::days(1);
            if is_holiday(hours, yday) {
                continue;
            }
            let w_yday = yday.weekday().num_days_from_monday() as usize;
            if r.days[w_yday] {
                let close = mk_local_close(tz, day, r.close_ssm).with_timezone(&Utc);
                best = Some(best.map_or(close, |b| b.max(close)));
            }
        }
    }

    best
}

/// Next weekly close strictly after `t`: find the **last** day-close
/// that belongs to its ISO week (in exchange TZ).
fn next_weekly_close_after_with(
    hours: &MarketHours,
    t: DateTime<Utc>,
    kind: SessionKind,
) -> DateTime<Utc> {
    let tz = hours.tz;

    // First candidate daily close after t
    let mut c = next_daily_close_after_with(hours, t, kind);

    // Keep advancing daily closes while they are still in the same ISO week
    // as the candidate `c`. The first close whose **next** daily close is in a
    // different ISO week is the weekly boundary.
    loop {
        let c_local = c.with_timezone(&tz);
        let c_week = (c_local.iso_week().year(), c_local.iso_week().week());

        // Tiny epsilon to move strictly past `c`
        let c_next = next_daily_close_after_with(hours, c + Duration::nanoseconds(1), kind);
        let n_local = c_next.with_timezone(&tz);
        let n_week = (n_local.iso_week().year(), n_local.iso_week().week());

        if n_week != c_week {
            return c; // `c` is the last close of its week
        }

        c = c_next;
    }
}

/// Next monthly close strictly after `t`: find the **last** day-close that
/// belongs to its exchange-local calendar month. Mirrors
/// [`next_weekly_close_after_with`] but groups by `(year, month)` instead of ISO
/// week, so the boundary handles month lengths and year rollover by construction.
fn next_monthly_close_after_with(
    hours: &MarketHours,
    t: DateTime<Utc>,
    kind: SessionKind,
) -> DateTime<Utc> {
    let tz = hours.tz;

    // First candidate daily close after `t`.
    let mut c = next_daily_close_after_with(hours, t, kind);

    // Keep advancing daily closes while the next close is still in the same
    // calendar month as the candidate `c`. The first close whose **next** daily
    // close lands in a different month is the monthly boundary.
    loop {
        let c_local = c.with_timezone(&tz);
        let c_month = (c_local.year(), c_local.month());

        // Tiny epsilon to move strictly past `c`.
        let c_next = next_daily_close_after_with(hours, c + Duration::nanoseconds(1), kind);
        let n_local = c_next.with_timezone(&tz);
        let n_month = (n_local.year(), n_local.month());

        if n_month != c_month {
            return c; // `c` is the last close of its month
        }

        c = c_next;
    }
}

// Bulk builders

/// Builds the current [`MarketHours`] for each exchange, preserving input order.
#[must_use]
pub fn hours_for_all(exchanges: &[Exchange]) -> Vec<MarketHours> {
    exchanges.iter().map(|&e| hours_for_exchange(e)).collect()
}

/// Builds the current [`MarketHours`] for the built-in US-equities exchange set.
#[allow(
    dead_code,
    reason = "retained in the test-only market-hours harness as a bulk-hours builder"
)]
#[must_use]
pub fn hours_for_us_equities() -> Vec<MarketHours> {
    hours_for_all(US_EQUITY_EXCHANGES)
}

/// Builds the current [`MarketHours`] for the built-in EU-equities exchange set.
#[allow(
    dead_code,
    reason = "retained in the test-only market-hours harness as a bulk-hours builder"
)]
#[must_use]
pub fn hours_for_eu_equities() -> Vec<MarketHours> {
    hours_for_all(EU_EQUITY_EXCHANGES)
}

/// Builds a deterministic [`Exchange`]-keyed map of current [`MarketHours`].
///
/// Backed by a [`BTreeMap`] so iteration order is the [`Exchange`] `Ord` order,
/// not insertion order.
#[must_use]
pub fn hours_map_for(exchanges: &[Exchange]) -> BTreeMap<Exchange, MarketHours> {
    let mut m = BTreeMap::new();
    for &e in exchanges {
        m.insert(e, hours_for_exchange(e));
    }
    m
}

/// Builds an [`Exchange`]-keyed [`BTreeMap`] for the built-in US-equities set.
#[allow(
    dead_code,
    reason = "retained in the test-only market-hours harness as a bulk-hours builder"
)]
#[must_use]
pub fn hours_map_us_equities() -> BTreeMap<Exchange, MarketHours> {
    hours_map_for(US_EQUITY_EXCHANGES)
}

/// Build a 24×7 UTC profile for always-open venues (crypto perpetuals, etc.).
///
/// This profile models a market that never closes: every day of the week is
/// active, the session spans the full 24-hour day, and the daily boundary
/// falls at midnight UTC.  It is intentionally **not** used for futures venues
/// that have daily maintenance breaks (CME, COMEX, NYMEX) or weekend gaps;
/// those use venue-specific session rules instead.
///
/// The always-open contract is kept explicit and separate from futures-session
/// rules so that callers can distinguish the two categories by inspecting the
/// session rules (an always-open venue has a single `0..86400` same-day rule
/// active on all seven days, while futures venues use multi-rule wrap patterns).
fn default_24x7(ex: Exchange) -> MarketHours {
    let days_all = [true, true, true, true, true, true, true];
    let full_day = SessionRule {
        days: days_all,
        open_ssm: 0,
        close_ssm: 24 * 3600,
    };
    MarketHours {
        exchange: ex,
        tz: UTC,
        regular: vec![full_day].into(),
        extended: Vec::new().into(),
        has_daily_close: false,
        has_weekend_close: false,
    }
}
/// Builds an [`Exchange`]-keyed [`BTreeMap`] for the built-in EU-equities set.
#[allow(
    dead_code,
    reason = "retained in the test-only market-hours harness as a bulk-hours builder"
)]
#[must_use]
pub fn hours_map_eu_equities() -> BTreeMap<Exchange, MarketHours> {
    hours_map_for(EU_EQUITY_EXCHANGES)
}

// -----------------------------------------------------------------------------
// Equity presets helpers
// -----------------------------------------------------------------------------

#[allow(dead_code)]
fn us_equity_regular() -> SessionRule {
    // 09:30–16:00 ET (Mon–Fri)
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 30 * 60,
        close_ssm: 16 * 3600,
    }
}

#[allow(dead_code)]
fn us_equity_extended_pre() -> SessionRule {
    // 04:00–09:30 ET premarket
    SessionRule {
        days: MON_FRI,
        open_ssm: 4 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    }
}

#[allow(dead_code)]
fn us_equity_extended_post() -> SessionRule {
    // 16:00–20:00 ET postmarket
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 20 * 3600,
    }
}

// EU venue helpers (simple builders for common patterns)
// Note: EU builder helpers removed; profiles directly encode continuous and auction slices.
