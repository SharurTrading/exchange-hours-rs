// SPDX-License-Identifier: MIT-0

//! Fixed-current profile values behind [`super::session_profile`].

use chrono_tz::{America, Asia, Europe, US, UTC};

use super::{FuturesSessionProfile, MarketHoursKey};
use crate::calendar::SessionRule;
use crate::calendar::rule::ALL_DAYS;
use crate::calendar::schedules::futures::international::{
    EUREX_CURRENT_EXTENDED, EUREX_CURRENT_REGULAR, SGX_CURRENT_EXTENDED, SGX_CURRENT_REGULAR,
};
use crate::calendar::schedules::futures::us::{
    CBOT_EXTENDED_CURRENT, CBOT_REGULAR_CURRENT, CFE_EXTENDED, CFE_REGULAR, CME_EXTENDED_CURRENT,
    CME_REGULAR, CRYPTOCURRENCY_CURRENT, ENERGY_METALS_EXTENDED_CURRENT, FX_CURRENT,
    ICE_US_FANG_EXTENDED_CURRENT, ICE_US_FANG_REGULAR_CURRENT, INTEREST_RATES_CURRENT,
    LIVESTOCK_CURRENT,
};

static ALWAYS_OPEN_RULE: &[SessionRule] = &[SessionRule {
    days: ALL_DAYS,
    open_ssm: 0,
    close_ssm: 24 * 3600,
}];

static FUTURES_GLOBEX_EQUITY_INDEX: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: CME_REGULAR,
    extended: CME_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_GLOBEX_ENERGY: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: ENERGY_METALS_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_GLOBEX_GRAINS: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: CBOT_REGULAR_CURRENT,
    extended: CBOT_EXTENDED_CURRENT,
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
    regular: EUREX_CURRENT_REGULAR,
    extended: EUREX_CURRENT_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_ICE_US: FuturesSessionProfile = FuturesSessionProfile {
    tz: America::New_York,
    regular: ICE_US_FANG_REGULAR_CURRENT,
    extended: ICE_US_FANG_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_SGX: FuturesSessionProfile = FuturesSessionProfile {
    tz: Asia::Singapore,
    regular: SGX_CURRENT_REGULAR,
    extended: SGX_CURRENT_EXTENDED,
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

/// Returns the fixed-current normal-week futures session profile for `key`.
///
/// This function does not select historical revisions. Use
/// [`super::hours_for_market_hours_key_as_of`] for a dated key snapshot.
/// Callers that scan across a transition should use
/// [`crate::calendar::calendar_for_market_hours_key`], which reselects the
/// product-family profile for every candidate opening day.
#[must_use]
pub fn session_profile(key: MarketHoursKey) -> &'static FuturesSessionProfile {
    match key {
        MarketHoursKey::GlobexEquityIndex => &FUTURES_GLOBEX_EQUITY_INDEX,
        MarketHoursKey::GlobexEnergy => &FUTURES_GLOBEX_ENERGY,
        MarketHoursKey::GlobexGrains => &FUTURES_GLOBEX_GRAINS,
        MarketHoursKey::GlobexFx => &FX_CURRENT,
        MarketHoursKey::GlobexInterestRates => &INTEREST_RATES_CURRENT,
        MarketHoursKey::GlobexLivestock => &LIVESTOCK_CURRENT,
        MarketHoursKey::GlobexCryptocurrency => &CRYPTOCURRENCY_CURRENT,
        MarketHoursKey::CfeVix => &FUTURES_CFE_VIX,
        MarketHoursKey::Eurex => &FUTURES_EUREX,
        MarketHoursKey::IceUs => &FUTURES_ICE_US,
        MarketHoursKey::Sgx => &FUTURES_SGX,
        MarketHoursKey::AlwaysOpen => &FUTURES_ALWAYS_OPEN,
    }
}
