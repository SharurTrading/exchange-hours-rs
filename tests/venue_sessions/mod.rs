// SPDX-License-Identifier: MIT-0

//! Normal-week venue fixtures organized by product family and query behavior.

mod always_open_and_cross_venue;
mod bounds_and_serde;
mod candle_starts_and_profile_adapters;
mod cboe_options;
mod cde_smfe;
mod cme_cbot;
mod cme_families;
mod commodities;
mod equity_corrections;
mod eurex_ice;
mod finra_trfs;
mod holidays_nasdaq;
mod holidays_nyse;
mod international_products;
mod intraday_and_monthly_candles;
mod kind_aware_candles;
mod named_profiles;
mod nasdaq;
mod oanda_book;
mod oanda_books2;
mod sgx_cfe;
mod sunday_wraps;
mod us_equity_history;
mod us_independent_exchanges;
mod verified_corrections;

mod prelude;

mod holidays_cme_venues;
mod holidays_coinbase_derivatives;
mod holidays_tse_sse_nse;
