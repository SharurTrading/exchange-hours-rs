// SPDX-License-Identifier: MIT-0

//! U.S. futures venue tables in each operator's published local zone.
//!
//! The venue modules own their session tables, primary-source citations, and
//! point-in-time selectors. Current product-family slices are re-exported for
//! the public [`MarketHoursKey`](crate::MarketHoursKey) profiles.

mod bitcoin_event_contracts;
mod cfe;
mod cme_group;
mod coinbase_derivatives;
mod cryptocurrency;
mod energy_metals;
mod event_contracts;
mod fx;
mod grains;
mod ice_sugar;
mod ice_us;
mod interest_rates;
mod livestock;
mod metals_tas;
mod mini_grains;
mod pgm_tas;
mod rough_rice;
mod small_exchange;
mod spot_quoted;
mod weather;

mod cme_nikkei;

mod ice_cocoa;

mod ice_coffee;

mod ice_cotton;

mod ice_fcoj;

mod ice_usdx;

pub(crate) use bitcoin_event_contracts::{
    BITCOIN_EVENT_CONTRACTS_EXTENDED_CURRENT, BITCOIN_EVENT_CONTRACTS_ORDER_ENTRY_CURRENT,
    profile_at as bitcoin_event_contracts_profile_at,
};
pub(crate) use cfe::CFE_ORDER_ENTRY;
pub(crate) use cfe::{CFE_EXTENDED, CFE_REGULAR, cfe_profile_at};
pub(crate) use cme_group::CME_ORDER_ENTRY_CURRENT;
pub(crate) use cme_group::{CME_EXTENDED_CURRENT, CME_REGULAR, cme_profile_at};
pub(crate) use cme_nikkei::{NKD_EXTENDED_CURRENT, NKD_REGULAR_CURRENT, nkd_profile_at};
pub(crate) use coinbase_derivatives::profile_at as coinbase_derivatives_profile_at;
pub(crate) use cryptocurrency::{
    CURRENT_FUTURES_PROFILE as CRYPTOCURRENCY_CURRENT, profile_at as cryptocurrency_profile_at,
};
pub(crate) use energy_metals::ENERGY_METALS_ORDER_ENTRY_CURRENT;
pub(crate) use energy_metals::{ENERGY_METALS_EXTENDED_CURRENT, energy_metals_profile_at};
pub(crate) use event_contracts::{
    EVENT_CONTRACTS_EXTENDED_CURRENT, EVENT_CONTRACTS_ORDER_ENTRY_CURRENT,
    profile_at as event_contracts_profile_at,
};
pub(crate) use fx::{CURRENT_FUTURES_PROFILE as FX_CURRENT, profile_at as fx_profile_at};
pub(crate) use grains::CBOT_ORDER_ENTRY_CURRENT;
pub(crate) use grains::{
    CBOT_EXTENDED_CURRENT, CBOT_REGULAR_CURRENT, profile_at as cbot_profile_at,
};
pub(crate) use ice_cocoa::COCOA_ORDER_ENTRY_CURRENT;
pub(crate) use ice_cocoa::{COCOA_EXTENDED_CURRENT, COCOA_REGULAR_CURRENT, cocoa_profile_at};
pub(crate) use ice_coffee::COFFEE_ORDER_ENTRY_CURRENT;
pub(crate) use ice_coffee::{COFFEE_EXTENDED_CURRENT, COFFEE_REGULAR_CURRENT, coffee_profile_at};
pub(crate) use ice_cotton::COTTON_ORDER_ENTRY_CURRENT;
pub(crate) use ice_cotton::{COTTON_EXTENDED_CURRENT, COTTON_REGULAR_CURRENT, cotton_profile_at};
pub(crate) use ice_fcoj::FCOJ_ORDER_ENTRY_CURRENT;
pub(crate) use ice_fcoj::{FCOJ_EXTENDED_CURRENT, FCOJ_REGULAR_CURRENT, fcoj_profile_at};
pub(crate) use ice_sugar::SUGAR_ORDER_ENTRY_CURRENT;
pub(crate) use ice_sugar::{SUGAR_EXTENDED_CURRENT, SUGAR_REGULAR_CURRENT, sugar_profile_at};
pub(crate) use ice_us::ICE_US_FANG_ORDER_ENTRY_CURRENT;
pub(crate) use ice_us::{
    ICE_US_FANG_EXTENDED_CURRENT, ICE_US_FANG_REGULAR_CURRENT, ice_us_fang_profile_at,
};
pub(crate) use ice_usdx::ICE_USDX_ORDER_ENTRY_CURRENT;
pub(crate) use ice_usdx::{
    ICE_USDX_EXTENDED_CURRENT, ICE_USDX_REGULAR_CURRENT, ice_usdx_profile_at,
};
pub(crate) use interest_rates::{
    CURRENT_FUTURES_PROFILE as INTEREST_RATES_CURRENT, profile_at as interest_rates_profile_at,
};
pub(crate) use livestock::{
    CURRENT_FUTURES_PROFILE as LIVESTOCK_CURRENT, profile_at as livestock_profile_at,
};
pub(crate) use metals_tas::{
    COPPER_TAS_EXTENDED_CURRENT, GOLD_TAS_EXTENDED_CURRENT, METALS_TAS_ORDER_ENTRY_CURRENT,
    SILVER_TAS_EXTENDED_CURRENT, copper_profile_at as copper_tas_profile_at,
    gold_profile_at as gold_tas_profile_at, silver_profile_at as silver_tas_profile_at,
};
pub(crate) use mini_grains::{
    MINI_EXTENDED_CURRENT, MINI_ORDER_ENTRY_CURRENT, MINI_REGULAR_CURRENT,
    profile_at as mini_grains_profile_at,
};
pub(crate) use pgm_tas::{
    PALLADIUM_TAS_EXTENDED_CURRENT, PGM_TAS_ORDER_ENTRY_CURRENT, PLATINUM_TAS_EXTENDED_CURRENT,
    palladium_profile_at as palladium_tas_profile_at,
    platinum_profile_at as platinum_tas_profile_at,
};
pub(crate) use rough_rice::{
    ROUGH_RICE_EXTENDED_CURRENT, ROUGH_RICE_ORDER_ENTRY_CURRENT, ROUGH_RICE_REGULAR_CURRENT,
    profile_at as rough_rice_profile_at,
};
pub(crate) use small_exchange::profile_at as small_exchange_profile_at;
pub(crate) use spot_quoted::{
    SPOT_QUOTED_EXTENDED_CURRENT, SPOT_QUOTED_ORDER_ENTRY_CURRENT,
    profile_at as spot_quoted_profile_at,
};
pub(crate) use weather::{
    WEATHER_EXTENDED_CURRENT, WEATHER_ORDER_ENTRY_CURRENT, profile_at as weather_profile_at,
};
