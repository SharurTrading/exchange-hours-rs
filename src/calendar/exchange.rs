// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! Venue identity for the calendar.
//!
//! [`Exchange`] names a venue and nothing else — it carries no hours. The
//! mapping from a variant to a schedule lives in [`super::presets`], and the
//! match there is exhaustive on purpose so a new variant cannot silently inherit
//! a default. The two region lists below are the built-in inputs for the bulk
//! builders in [`super::bulk`].

use serde::{Deserialize, Serialize};

/// Identifies an exchange or trading venue.
///
/// Variants are grouped by product family (US equities, US options, US futures,
/// European futures/energy, Asia-Pacific futures, EU equities, and always-open
/// crypto venues). The enum is exhaustive, so adding a venue forces the match in
/// [`hours_for_exchange`](super::hours_for_exchange) to be updated and keeps the
/// calendar surface complete.
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
    /// Unrecognized or unset venue; [`hours_for_exchange`](super::hours_for_exchange)
    /// returns a 24×7 UTC fallback and logs a one-shot warning.
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

pub(crate) const US_EQUITY_EXCHANGES: &[Exchange] = &[
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

pub(crate) const EU_EQUITY_EXCHANGES: &[Exchange] = &[
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
