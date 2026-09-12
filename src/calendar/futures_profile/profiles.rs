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
    BITCOIN_EVENT_CONTRACTS_EXTENDED_CURRENT, BITCOIN_EVENT_CONTRACTS_ORDER_ENTRY_CURRENT,
    CBOT_EXTENDED_CURRENT, CBOT_ORDER_ENTRY_CURRENT, CBOT_REGULAR_CURRENT, CFE_EXTENDED,
    CFE_ORDER_ENTRY, CFE_REGULAR, CME_EXTENDED_CURRENT, CME_ORDER_ENTRY_CURRENT, CME_REGULAR,
    COCOA_EXTENDED_CURRENT, COCOA_ORDER_ENTRY_CURRENT, COCOA_REGULAR_CURRENT,
    COFFEE_EXTENDED_CURRENT, COFFEE_ORDER_ENTRY_CURRENT, COFFEE_REGULAR_CURRENT,
    COPPER_TAS_EXTENDED_CURRENT, COTTON_EXTENDED_CURRENT, COTTON_ORDER_ENTRY_CURRENT,
    COTTON_REGULAR_CURRENT, CRYPTOCURRENCY_CURRENT, ENERGY_METALS_EXTENDED_CURRENT,
    ENERGY_METALS_ORDER_ENTRY_CURRENT, EVENT_CONTRACTS_EXTENDED_CURRENT,
    EVENT_CONTRACTS_ORDER_ENTRY_CURRENT, FCOJ_EXTENDED_CURRENT, FCOJ_ORDER_ENTRY_CURRENT,
    FCOJ_REGULAR_CURRENT, FX_CURRENT, GOLD_TAS_EXTENDED_CURRENT, ICE_US_FANG_EXTENDED_CURRENT,
    ICE_US_FANG_ORDER_ENTRY_CURRENT, ICE_US_FANG_REGULAR_CURRENT, ICE_USDX_EXTENDED_CURRENT,
    ICE_USDX_ORDER_ENTRY_CURRENT, ICE_USDX_REGULAR_CURRENT, INTEREST_RATES_CURRENT,
    LIVESTOCK_CURRENT, METALS_TAS_ORDER_ENTRY_CURRENT, MINI_EXTENDED_CURRENT,
    MINI_ORDER_ENTRY_CURRENT, MINI_REGULAR_CURRENT, NKD_EXTENDED_CURRENT, NKD_REGULAR_CURRENT,
    PALLADIUM_TAS_EXTENDED_CURRENT, PGM_TAS_ORDER_ENTRY_CURRENT, PLATINUM_TAS_EXTENDED_CURRENT,
    ROUGH_RICE_EXTENDED_CURRENT, ROUGH_RICE_ORDER_ENTRY_CURRENT, ROUGH_RICE_REGULAR_CURRENT,
    SILVER_TAS_EXTENDED_CURRENT, SPOT_QUOTED_EXTENDED_CURRENT, SPOT_QUOTED_ORDER_ENTRY_CURRENT,
    SUGAR_EXTENDED_CURRENT, SUGAR_ORDER_ENTRY_CURRENT, SUGAR_REGULAR_CURRENT,
    WEATHER_EXTENDED_CURRENT, WEATHER_ORDER_ENTRY_CURRENT,
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

// The mini grid converged with the standard grain grid on 2022-10-02, so the
// tables are the shared ones under mini names; `mini_grains.rs` holds the
// diverged history that makes the separate key necessary.
static FUTURES_GLOBEX_MINI_GRAINS: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: MINI_REGULAR_CURRENT,
    extended: MINI_EXTENDED_CURRENT,
    order_entry: MINI_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// No regular session: weather futures have never been pit-eligible, so the
// whole executable envelope is the CME Globex wrap. `weather.rs` holds the
// 15:15 CT close this family ran on until SER-9519, which is what makes the
// key necessary despite an envelope shared with FX and energy today.
static FUTURES_GLOBEX_WEATHER: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: WEATHER_EXTENDED_CURRENT,
    order_entry: WEATHER_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// No regular session either: no CME document classifies any part of the
// spot-quoted session as RTH, so the whole Globex wrap is extended.
// `spot_quoted.rs` holds the 2025-06-29 launch and the closed era before it,
// which is what makes this key distinct from `globex_weather` despite an
// identical current envelope.
static FUTURES_GLOBEX_SPOT_QUOTED: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: SPOT_QUOTED_EXTENDED_CURRENT,
    order_entry: SPOT_QUOTED_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// No regular session either: no CME document about this family splits its
// session into RTH and ETH, so the whole Globex wrap is extended. The 16:00 CT
// close is CME's published daily maintenance boundary, not any contract's
// termination-of-trading time — those are stated expiration times and vary by
// root. `event_contracts.rs` holds the 2022-09-18 launch, the closed era
// before it, and `ECBTC`'s 2026-05-29 departure from scope.
static FUTURES_GLOBEX_EVENT_CONTRACTS: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: EVENT_CONTRACTS_EXTENDED_CURRENT,
    order_entry: EVENT_CONTRACTS_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// `ECBTC` alone: 24/7 on the sourced intersection of CME's two primaries, so
// the weekday executable leg ends at 15:00 CT and resumes at 16:02, with the
// 16:01 and Saturday 03:45 Pre-Opens as order-entry phases. The Saturday
// 02:00-04:00 window is agreed. `bitcoin_event_contracts.rs` holds the 2023
// listing, the shared era, the 2026-05-29 transition day and the three
// notice-dated Saturday extensions.
static FUTURES_GLOBEX_EVENT_CONTRACTS_BTC: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: BITCOIN_EVENT_CONTRACTS_EXTENDED_CURRENT,
    order_entry: BITCOIN_EVENT_CONTRACTS_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: false,
};

// The five metals Trading at Settlement books. `regular` is empty on all of
// them: the TAS shape is Globex-only in CME's own Rule 524 route enumeration.
// Each opens 17:00 CT Sunday through Thursday and wraps to its own close with
// no Friday-evening reopen, so every inter-trade-date gap exceeds four hours
// and is `Closed`. COMEX gold, silver and copper share one queue in
// `metals_tas.rs`; `pgm_tas.rs` holds NYMEX platinum and palladium.
static FUTURES_GLOBEX_GOLD_TAS: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: GOLD_TAS_EXTENDED_CURRENT,
    order_entry: METALS_TAS_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_GLOBEX_SILVER_TAS: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: SILVER_TAS_EXTENDED_CURRENT,
    order_entry: METALS_TAS_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_GLOBEX_COPPER_TAS: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: COPPER_TAS_EXTENDED_CURRENT,
    order_entry: METALS_TAS_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_GLOBEX_PLATINUM_TAS: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: PLATINUM_TAS_EXTENDED_CURRENT,
    order_entry: PGM_TAS_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// No 16:00-17:00 CT break: the specification clause that once implied one is
// inherited boilerplate from the outright row, and CME deleted it in 2020.
static FUTURES_GLOBEX_PALLADIUM_TAS: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: PALLADIUM_TAS_EXTENDED_CURRENT,
    order_entry: PGM_TAS_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_GLOBEX_ROUGH_RICE: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: ROUGH_RICE_REGULAR_CURRENT,
    extended: ROUGH_RICE_EXTENDED_CURRENT,
    order_entry: ROUGH_RICE_ORDER_ENTRY_CURRENT,
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
        MarketHoursKey::GlobexMiniGrains => &FUTURES_GLOBEX_MINI_GRAINS,
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
        MarketHoursKey::GlobexRoughRice => &FUTURES_GLOBEX_ROUGH_RICE,
        MarketHoursKey::GlobexWeather => &FUTURES_GLOBEX_WEATHER,
        MarketHoursKey::GlobexSpotQuoted => &FUTURES_GLOBEX_SPOT_QUOTED,
        MarketHoursKey::GlobexEventContracts => &FUTURES_GLOBEX_EVENT_CONTRACTS,
        MarketHoursKey::GlobexEventContractsBtc => &FUTURES_GLOBEX_EVENT_CONTRACTS_BTC,
        MarketHoursKey::GlobexGoldTas => &FUTURES_GLOBEX_GOLD_TAS,
        MarketHoursKey::GlobexSilverTas => &FUTURES_GLOBEX_SILVER_TAS,
        MarketHoursKey::GlobexCopperTas => &FUTURES_GLOBEX_COPPER_TAS,
        MarketHoursKey::GlobexPlatinumTas => &FUTURES_GLOBEX_PLATINUM_TAS,
        MarketHoursKey::GlobexPalladiumTas => &FUTURES_GLOBEX_PALLADIUM_TAS,
        MarketHoursKey::Sgx => &FUTURES_SGX,
        MarketHoursKey::AlwaysOpen => &FUTURES_ALWAYS_OPEN,
    }
}
