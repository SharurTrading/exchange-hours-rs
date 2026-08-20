// SPDX-License-Identifier: MIT-0

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
/// crypto venues). Inside this crate the matches over it stay exhaustive, so
/// adding a venue forces the match in
/// [`hours_for_exchange`](super::hours_for_exchange) and the name table in
/// [`Exchange::as_str`] to be updated and keeps the calendar surface complete.
///
/// The enum is `#[non_exhaustive]`, mirroring
/// [`MarketHoursKey`](super::MarketHoursKey): venue coverage grows over time,
/// and adding a variant must not be a breaking change for dependents. Match it
/// with a wildcard arm; [`Exchange::ALL`] enumerates the variants of the
/// version you compiled against.
///
/// Holidays and product-level calendar variations are deliberately not modeled
/// here: this enum drives only normal-week, exchange-level session defaults.
/// [`Exchange::Unknown`] maps to a 24×7 UTC fallback. Variants have one
/// canonical `snake_case` name (e.g. `Exchange::NasdaqBx` ↔ `"nasdaq_bx"`)
/// used identically by serde, [`Exchange::as_str`], [`std::fmt::Display`], and
/// [`std::str::FromStr`]; that wire form is asserted variant-by-variant in the
/// test suite and must stay stable.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Exchange {
    /// Unrecognized or unset venue; [`hours_for_exchange`](super::hours_for_exchange)
    /// returns a 24×7 UTC fallback and logs a one-shot warning.
    Unknown,

    // US Equities (ET)
    /// The Nasdaq Stock Market — the primary Nasdaq listing venue; early
    /// session from 04:00 ET.
    Nasdaq,
    /// Nasdaq BX, the former Boston Stock Exchange book; early session from
    /// 04:00 ET.
    NasdaqBx,
    /// Nasdaq PSX, the former Philadelphia Stock Exchange equities book;
    /// early session from 04:00 ET.
    NasdaqPsx,
    /// Cboe BZX Equities — early session from 04:00 ET (07:00 before
    /// 2025-05-01).
    CboeBzx,
    /// Cboe BYX Equities — early session from 07:00 ET.
    CboeByx,
    /// Cboe EDGA Equities — early session from 07:00 ET.
    CboeEdga,
    /// Cboe EDGX Equities — early session from 04:00 ET (07:00 before
    /// 2021-03-08).
    CboeEdgx,
    /// New York Stock Exchange — the core session only; NYSE Group extended
    /// trading happens on Arca, American, National, and Texas.
    Nyse,
    /// NYSE Arca — early session from 04:00 ET, unlike the NYSE core book.
    NyseArca,
    /// NYSE American, formerly the American Stock Exchange — early session
    /// from 07:00 ET.
    NyseAmerican,
    /// NYSE National — early session from 07:00 ET.
    NyseNational,
    /// NYSE Texas, formerly NYSE Chicago — no sessions before its
    /// 2025-03-31 go-live.
    NyseTexas,
    /// MEMX, the Members Exchange — early session from 04:00 ET, with no
    /// recorded cutover (see `hours_for_exchange_as_of`).
    MemxEq,
    /// MIAX Pearl Equities — early session from 04:00 ET, with no recorded
    /// cutover (see `hours_for_exchange_as_of`).
    MiaxPearlEq,
    /// IEX, the Investors Exchange — System Hours 08:00–17:00 ET, narrower
    /// than the Reg NMS extended default at both ends.
    Iex,
    /// `IntelligentCross`, an ATS whose IQX book executes during regular hours.
    IntelligentcrossIqx,
    /// Blue Ocean ATS, an overnight-only US equities ATS.
    BlueOceanAts,
    /// FINRA/Nasdaq Trade Reporting Facility, Carteret — an off-exchange
    /// reporting facility rather than a matching venue.
    FinraTrfCarteret,
    /// FINRA/Nasdaq Trade Reporting Facility, Chicago.
    FinraTrfChicago,
    /// FINRA/NYSE Trade Reporting Facility.
    FinraTrfNyse,

    // US Options (ET)
    /// Cboe Options Exchange (C1) — the only US options venue here with a
    /// Global Trading Hours wrap and a post-close curb session.
    CboeOptionsC1,
    /// Cboe C2 Options Exchange.
    CboeC2Options,
    /// Cboe BZX Options Exchange.
    CboeBzxOptions,
    /// Cboe EDGX Options Exchange.
    CboeEdgxOptions,
    /// NYSE Arca Options.
    NyseArcaOptions,
    /// NYSE American Options.
    NyseAmericanOptions,
    /// Nasdaq PHLX, the former Philadelphia options exchange.
    NasdaqPhlx,
    /// Nasdaq ISE, the International Securities Exchange.
    NasdaqIse,
    /// The Nasdaq Options Market (NOM).
    NasdaqNom,
    /// Nasdaq MRX, formerly ISE Mercury.
    NasdaqMrx,
    /// Nasdaq GEMX, formerly ISE Gemini.
    NasdaqGemx,
    /// Nasdaq BX Options.
    NasdaqBxOptions,
    /// MIAX Options, the original Miami International Securities Exchange.
    MiaxOptions,
    /// MIAX Emerald Options.
    MiaxEmeraldOptions,
    /// MIAX Pearl Options.
    MiaxPearlOptions,
    /// MIAX Sapphire Options.
    MiaxSapphireOptions,
    /// BOX Options Exchange.
    BoxOptions,
    /// MEMX Options.
    MemxOptions,

    // US Futures
    /// CME, the Chicago Mercantile Exchange; the venue default is the Globex
    /// equity-index profile.
    Cme,
    /// CBOT, the Chicago Board of Trade; the venue default is the Globex
    /// grains profile, which keeps its own day session.
    Cbot,
    /// COMEX metals, sharing the Globex energy/metals profile.
    Comex,
    /// NYMEX energy, sharing the Globex energy/metals profile.
    Nymex,
    /// CFE, the Cboe Futures Exchange; the venue default is the VIX profile.
    Cfe,

    // European Futures / Energy
    /// Eurex, the European derivatives exchange.
    Eurex,
    /// EEX, the European Energy Exchange.
    Eex,
    /// ICE Futures U.S.
    Iceus,
    /// ICE Futures Europe.
    Iceeu,
    /// ICE Futures Europe, commodities products.
    IceEuropeCommodities,
    /// ICE Futures Europe, financial products.
    IceEuropeFinancials,
    /// ICE Endex, the Amsterdam gas and power market.
    IceEndex,
    /// ICE Futures Abu Dhabi.
    IceAbuDhabi,
    /// ICE Futures Canada.
    IceCanada,

    // Asia-Pacific Futures
    /// SGX, the Singapore Exchange — derivatives market.
    Sgx,

    // EU Equities
    /// London Stock Exchange (SETS).
    Lse,
    /// Xetra, Deutsche Börse's electronic order book.
    Xetra,
    /// SIX Swiss Exchange.
    Six,
    /// Euronext Paris.
    EuronextParis,
    /// Euronext Amsterdam.
    EuronextAmsterdam,
    /// Euronext Brussels.
    EuronextBrussels,
    /// Euronext Lisbon.
    EuronextLisbon,
    /// Euronext Dublin.
    EuronextDublin,
    /// Euronext Milan, formerly Borsa Italiana.
    EuronextMilan,
    /// BME, the Spanish exchange operator (Bolsa de Madrid).
    Bme,
    /// Nasdaq Stockholm.
    NasdaqStockholm,
    /// Nasdaq Helsinki — the one Nordic book quoted in EET rather than CET.
    NasdaqHelsinki,
    /// Nasdaq Copenhagen.
    NasdaqCopenhagen,
    /// Wiener Börse, the Vienna Stock Exchange.
    Vienna,

    // Crypto / always-open venues
    //
    // These venues trade continuously without daily maintenance breaks.
    // They are modeled as 24×7 UTC sessions and must remain explicitly
    // separated from futures-session venues that have daily and weekend
    // boundaries. See `default_24x7` for the profile and the
    // `always_open_*` tests for the behavioral contract.
    /// Binance futures — a 24×7 venue with no daily or weekend close.
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
