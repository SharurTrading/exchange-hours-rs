// SPDX-License-Identifier: MIT-0

//! Fixed-current profile values behind [`super::session_profile`].

use chrono_tz::{America, Asia, Europe, US, UTC};

use super::{FuturesSessionProfile, MarketHoursKey};
use crate::calendar::schedules::ALWAYS_OPEN_RULE;
use crate::calendar::schedules::futures::international::{
    EUREX_CURRENT_EXTENDED, EUREX_CURRENT_ORDER_ENTRY, EUREX_CURRENT_REGULAR,
    EUREX_FIXED_INCOME_EXTENDED_CURRENT, EUREX_FIXED_INCOME_ORDER_ENTRY_CURRENT,
    EUREX_FIXED_INCOME_REGULAR_CURRENT, SGX_CURRENT_EXTENDED, SGX_CURRENT_ORDER_ENTRY,
    SGX_CURRENT_REGULAR, SGX_EQUITY_INDEX_CHINA_EXTENDED_CURRENT,
    SGX_EQUITY_INDEX_CHINA_ORDER_ENTRY_CURRENT, SGX_EQUITY_INDEX_CHINA_REGULAR_CURRENT,
    SGX_EQUITY_INDEX_JAPAN_EXTENDED_CURRENT, SGX_EQUITY_INDEX_JAPAN_ORDER_ENTRY_CURRENT,
    SGX_EQUITY_INDEX_JAPAN_REGULAR_CURRENT, SGX_EQUITY_INDEX_NTR_USD_EXTENDED_CURRENT,
    SGX_EQUITY_INDEX_NTR_USD_ORDER_ENTRY_CURRENT, SGX_EQUITY_INDEX_NTR_USD_REGULAR_CURRENT,
    SGX_EQUITY_INDEX_SINGAPORE_EXTENDED_CURRENT, SGX_EQUITY_INDEX_SINGAPORE_ORDER_ENTRY_CURRENT,
    SGX_EQUITY_INDEX_SINGAPORE_REGULAR_CURRENT, SGX_EQUITY_INDEX_TAIWAN_EXTENDED_CURRENT,
    SGX_EQUITY_INDEX_TAIWAN_ORDER_ENTRY_CURRENT, SGX_EQUITY_INDEX_TAIWAN_REGULAR_CURRENT,
};
use crate::calendar::schedules::futures::us::{
    CBOT_EXTENDED_CURRENT, CBOT_ORDER_ENTRY_CURRENT, CBOT_REGULAR_CURRENT, CFE_EXTENDED,
    CFE_ORDER_ENTRY, CFE_REGULAR, CME_EXTENDED_CURRENT, CME_ORDER_ENTRY_CURRENT, CME_REGULAR,
    COCOA_EXTENDED_CURRENT, COCOA_ORDER_ENTRY_CURRENT, COCOA_REGULAR_CURRENT,
    COFFEE_EXTENDED_CURRENT, COFFEE_ORDER_ENTRY_CURRENT, COFFEE_REGULAR_CURRENT,
    COTTON_EXTENDED_CURRENT, COTTON_ORDER_ENTRY_CURRENT, COTTON_REGULAR_CURRENT,
    CRYPTOCURRENCY_CURRENT, ENERGY_METALS_EXTENDED_CURRENT, ENERGY_METALS_ORDER_ENTRY_CURRENT,
    FCOJ_EXTENDED_CURRENT, FCOJ_ORDER_ENTRY_CURRENT, FCOJ_REGULAR_CURRENT, FX_CURRENT,
    ICE_US_FANG_EXTENDED_CURRENT, ICE_US_FANG_ORDER_ENTRY_CURRENT, ICE_US_FANG_REGULAR_CURRENT,
    ICE_USDX_EXTENDED_CURRENT, ICE_USDX_ORDER_ENTRY_CURRENT, ICE_USDX_REGULAR_CURRENT,
    INTEREST_RATES_CURRENT, LIVESTOCK_CURRENT, NKD_EXTENDED_CURRENT, NKD_REGULAR_CURRENT,
    SUGAR_EXTENDED_CURRENT, SUGAR_ORDER_ENTRY_CURRENT, SUGAR_REGULAR_CURRENT,
};

static FUTURES_GLOBEX_EQUITY_INDEX: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: CME_REGULAR,
    extended: CME_EXTENDED_CURRENT,
    order_entry: CME_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_GLOBEX_ENERGY: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: ENERGY_METALS_EXTENDED_CURRENT,
    order_entry: ENERGY_METALS_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_GLOBEX_GRAINS: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: CBOT_REGULAR_CURRENT,
    extended: CBOT_EXTENDED_CURRENT,
    order_entry: CBOT_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_CFE_VIX: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: CFE_REGULAR,
    extended: CFE_EXTENDED,
    order_entry: CFE_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_EUREX: FuturesSessionProfile = FuturesSessionProfile {
    tz: Europe::Berlin,
    regular: EUREX_CURRENT_REGULAR,
    extended: EUREX_CURRENT_EXTENDED,
    order_entry: EUREX_CURRENT_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_ICE_US: FuturesSessionProfile = FuturesSessionProfile {
    tz: America::New_York,
    regular: ICE_US_FANG_REGULAR_CURRENT,
    extended: ICE_US_FANG_EXTENDED_CURRENT,
    order_entry: ICE_US_FANG_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_ICE_US_SUGAR: FuturesSessionProfile = FuturesSessionProfile {
    tz: America::New_York,
    regular: SUGAR_REGULAR_CURRENT,
    extended: SUGAR_EXTENDED_CURRENT,
    order_entry: SUGAR_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_COFFEE: FuturesSessionProfile = FuturesSessionProfile {
    tz: America::New_York,
    regular: COFFEE_REGULAR_CURRENT,
    extended: COFFEE_EXTENDED_CURRENT,
    order_entry: COFFEE_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_COCOA: FuturesSessionProfile = FuturesSessionProfile {
    tz: America::New_York,
    regular: COCOA_REGULAR_CURRENT,
    extended: COCOA_EXTENDED_CURRENT,
    order_entry: COCOA_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_COTTON: FuturesSessionProfile = FuturesSessionProfile {
    tz: America::New_York,
    regular: COTTON_REGULAR_CURRENT,
    extended: COTTON_EXTENDED_CURRENT,
    order_entry: COTTON_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_FCOJ: FuturesSessionProfile = FuturesSessionProfile {
    tz: America::New_York,
    regular: FCOJ_REGULAR_CURRENT,
    extended: FCOJ_EXTENDED_CURRENT,
    order_entry: FCOJ_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_ICE_USDX: FuturesSessionProfile = FuturesSessionProfile {
    tz: America::New_York,
    regular: ICE_USDX_REGULAR_CURRENT,
    extended: ICE_USDX_EXTENDED_CURRENT,
    order_entry: ICE_USDX_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_NKD: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: NKD_REGULAR_CURRENT,
    extended: NKD_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_EUREX_FIXED_INCOME: FuturesSessionProfile = FuturesSessionProfile {
    tz: Europe::Berlin,
    regular: EUREX_FIXED_INCOME_REGULAR_CURRENT,
    extended: EUREX_FIXED_INCOME_EXTENDED_CURRENT,
    order_entry: EUREX_FIXED_INCOME_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_SGX_EQUITY_INDEX_JAPAN: FuturesSessionProfile = FuturesSessionProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_JAPAN_REGULAR_CURRENT,
    extended: SGX_EQUITY_INDEX_JAPAN_EXTENDED_CURRENT,
    order_entry: SGX_EQUITY_INDEX_JAPAN_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_SGX_EQUITY_INDEX_CHINA: FuturesSessionProfile = FuturesSessionProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_CHINA_REGULAR_CURRENT,
    extended: SGX_EQUITY_INDEX_CHINA_EXTENDED_CURRENT,
    order_entry: SGX_EQUITY_INDEX_CHINA_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_SGX_EQUITY_INDEX_SINGAPORE: FuturesSessionProfile = FuturesSessionProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_SINGAPORE_REGULAR_CURRENT,
    extended: SGX_EQUITY_INDEX_SINGAPORE_EXTENDED_CURRENT,
    order_entry: SGX_EQUITY_INDEX_SINGAPORE_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_SGX_EQUITY_INDEX_TAIWAN: FuturesSessionProfile = FuturesSessionProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_TAIWAN_REGULAR_CURRENT,
    extended: SGX_EQUITY_INDEX_TAIWAN_EXTENDED_CURRENT,
    order_entry: SGX_EQUITY_INDEX_TAIWAN_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_SGX_EQUITY_INDEX_NTR_USD: FuturesSessionProfile = FuturesSessionProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_NTR_USD_REGULAR_CURRENT,
    extended: SGX_EQUITY_INDEX_NTR_USD_EXTENDED_CURRENT,
    order_entry: SGX_EQUITY_INDEX_NTR_USD_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_SGX: FuturesSessionProfile = FuturesSessionProfile {
    tz: Asia::Singapore,
    regular: SGX_CURRENT_REGULAR,
    extended: SGX_CURRENT_EXTENDED,
    order_entry: SGX_CURRENT_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_ALWAYS_OPEN: FuturesSessionProfile = FuturesSessionProfile {
    tz: UTC,
    regular: ALWAYS_OPEN_RULE,
    extended: &[],
    order_entry: &[],
    has_daily_close: false,
    has_weekend_close: false,
};
/// Returns the fixed-current normal-week futures session profile for `key`.
///
/// This is the static current table, not a time selection: it equals the
/// revision timeline's selection at any instant on or after the family's
/// knowledge-bound row (the 2026-08-22 repository review for families whose
/// current order-entry queues have no sourced onset day). Use
/// [`super::hours_for_market_hours_key`] to resolve the family at a caller's
/// instant, and
/// [`crate::calendar::calendar_for_market_hours_key`], which reselects the
/// product-family profile for every candidate opening day, for scans that
/// cross a transition.
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
        MarketHoursKey::IceUsSugar => &FUTURES_ICE_US_SUGAR,
        MarketHoursKey::IceUsCoffee => &FUTURES_COFFEE,
        MarketHoursKey::IceUsCocoa => &FUTURES_COCOA,
        MarketHoursKey::IceUsCotton => &FUTURES_COTTON,
        MarketHoursKey::IceUsOrangeJuice => &FUTURES_FCOJ,
        MarketHoursKey::IceUsDollarIndex => &FUTURES_ICE_USDX,
        MarketHoursKey::GlobexNikkei225Dollar => &FUTURES_NKD,
        MarketHoursKey::EurexFixedIncome => &FUTURES_EUREX_FIXED_INCOME,
        MarketHoursKey::SgxEquityIndexJapan => &FUTURES_SGX_EQUITY_INDEX_JAPAN,
        MarketHoursKey::SgxEquityIndexChina => &FUTURES_SGX_EQUITY_INDEX_CHINA,
        MarketHoursKey::SgxEquityIndexSingapore => &FUTURES_SGX_EQUITY_INDEX_SINGAPORE,
        MarketHoursKey::SgxEquityIndexTaiwan => &FUTURES_SGX_EQUITY_INDEX_TAIWAN,
        MarketHoursKey::SgxEquityIndexNtrUsd => &FUTURES_SGX_EQUITY_INDEX_NTR_USD,
        MarketHoursKey::Sgx => &FUTURES_SGX,
        MarketHoursKey::AlwaysOpen => &FUTURES_ALWAYS_OPEN,
    }
}
