// SPDX-License-Identifier: MIT-0

//! Product-family schedules for non-US futures and energy markets.
//!
//! The exchanges in this family do not publish one venue-wide clock. Each
//! public exchange default is therefore scoped to a named liquid contract
//! family in its owner module rather than sharing a generic venue envelope.

mod binance;
mod bme_ibex;
mod europe;
mod ice_abu_dhabi;
mod ice_canada;
mod ice_endex;
mod ice_europe;
mod ice_europe_gilts;
mod oanda;
mod oanda_asia;
mod sgx;

mod eurex_fixed_income;
mod eurex_smi;
mod euronext_index_futures;

mod sgx_equity_index;

mod sgx_equity_index_more;

pub(crate) use binance::profile_at as binance_profile_at;
pub(crate) use bme_ibex::bme_ibex_profile_at;
pub(crate) use bme_ibex::{BME_IBEX_ORDER_ENTRY_CURRENT, BME_IBEX_REGULAR_CURRENT};
pub(crate) use eurex_fixed_income::EUREX_FIXED_INCOME_ORDER_ENTRY_CURRENT;
pub(crate) use eurex_fixed_income::{
    EUREX_FIXED_INCOME_EXTENDED_CURRENT, EUREX_FIXED_INCOME_REGULAR_CURRENT,
    eurex_fixed_income_profile_at,
};
pub(crate) use eurex_smi::{
    EUREX_SMI_ORDER_ENTRY_CURRENT, EUREX_SMI_REGULAR_CURRENT, eurex_smi_profile_at,
};
pub(crate) use euronext_index_futures::{
    EURONEXT_INDEX_FUTURES_REGULAR, euronext_index_futures_profile_at,
};
pub(crate) use europe::EUREX_CURRENT_ORDER_ENTRY;
pub(crate) use europe::{
    EUREX_CURRENT_EXTENDED, EUREX_CURRENT_REGULAR, eex_profile_at, eurex_profile_at,
};
pub(crate) use ice_abu_dhabi::ice_abu_dhabi_profile_at;
pub(crate) use ice_canada::ice_canada_profile_at;
pub(crate) use ice_endex::profile_at as ice_endex_profile_at;
pub(crate) use ice_europe::{
    ice_europe_commodities_profile_at, ice_europe_financials_profile_at, iceeu_profile_at,
};
pub(crate) use ice_europe_gilts::ice_europe_gilts_profile_at;
pub(crate) use ice_europe_gilts::{
    ICE_EUROPE_GILTS_ORDER_ENTRY_CURRENT, ICE_EUROPE_GILTS_REGULAR_CURRENT,
};
pub(crate) use oanda::{
    OANDA_BRENT_REGULAR, OANDA_FX_REGULAR, OANDA_METALS_REGULAR, OANDA_TRY_FX_REGULAR,
    OANDA_US_CONTINUOUS_REGULAR, oanda_brent_profile_at, oanda_fx_profile_at,
    oanda_metals_profile_at, oanda_try_fx_profile_at, oanda_us_commodity_profile_at,
    oanda_us_index_profile_at,
};
pub(crate) use oanda_asia::{
    OANDA_CHINA50_EXTENDED, OANDA_SINGAPORE30_EXTENDED, oanda_china50_profile_at,
    oanda_singapore30_profile_at,
};
pub(crate) use sgx::SGX_CURRENT_ORDER_ENTRY;
pub(crate) use sgx::{SGX_CURRENT_EXTENDED, SGX_CURRENT_REGULAR, sgx_profile_at};
pub(crate) use sgx_equity_index::SGX_EQUITY_INDEX_CHINA_ORDER_ENTRY_CURRENT;
pub(crate) use sgx_equity_index::SGX_EQUITY_INDEX_JAPAN_ORDER_ENTRY_CURRENT;
pub(crate) use sgx_equity_index::SGX_EQUITY_INDEX_SINGAPORE_ORDER_ENTRY_CURRENT;
pub(crate) use sgx_equity_index::{
    SGX_EQUITY_INDEX_CHINA_EXTENDED_CURRENT, SGX_EQUITY_INDEX_CHINA_REGULAR_CURRENT,
    SGX_EQUITY_INDEX_JAPAN_EXTENDED_CURRENT, SGX_EQUITY_INDEX_JAPAN_REGULAR_CURRENT,
    SGX_EQUITY_INDEX_SINGAPORE_EXTENDED_CURRENT, SGX_EQUITY_INDEX_SINGAPORE_REGULAR_CURRENT,
    sgx_equity_index_china_profile_at, sgx_equity_index_japan_profile_at,
    sgx_equity_index_singapore_profile_at,
};
pub(crate) use sgx_equity_index_more::SGX_EQUITY_INDEX_NTR_USD_ORDER_ENTRY_CURRENT;
pub(crate) use sgx_equity_index_more::SGX_EQUITY_INDEX_TAIWAN_ORDER_ENTRY_CURRENT;
pub(crate) use sgx_equity_index_more::{
    SGX_EQUITY_INDEX_NTR_USD_EXTENDED_CURRENT, SGX_EQUITY_INDEX_NTR_USD_REGULAR_CURRENT,
    SGX_EQUITY_INDEX_TAIWAN_EXTENDED_CURRENT, SGX_EQUITY_INDEX_TAIWAN_REGULAR_CURRENT,
    sgx_equity_index_ntr_usd_profile_at, sgx_equity_index_taiwan_profile_at,
};
