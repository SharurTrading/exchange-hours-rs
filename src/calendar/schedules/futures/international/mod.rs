// SPDX-License-Identifier: MIT-0

//! Product-family schedules for non-US futures and energy markets.
//!
//! The exchanges in this family do not publish one venue-wide clock. Each
//! public exchange default is therefore scoped to a named liquid contract
//! family in its owner module rather than sharing a generic venue envelope.

mod binance;
mod europe;
mod ice_abu_dhabi;
mod ice_canada;
mod ice_endex;
mod ice_europe;
mod sgx;

mod eurex_fixed_income;

mod sgx_equity_index;

mod sgx_equity_index_more;

pub(crate) use binance::profile_at as binance_profile_at;
pub(crate) use eurex_fixed_income::EUREX_FIXED_INCOME_ORDER_ENTRY_CURRENT;
pub(crate) use eurex_fixed_income::{
    EUREX_FIXED_INCOME_EXTENDED_CURRENT, EUREX_FIXED_INCOME_REGULAR_CURRENT,
    eurex_fixed_income_profile_at,
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
