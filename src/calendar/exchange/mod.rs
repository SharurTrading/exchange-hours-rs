// SPDX-License-Identifier: MIT-0

//! Venue identity. One table generates [`Exchange`], [`Exchange::ALL`], and
//! [`Exchange::as_str`]; exhaustive routing catches incomplete additions.

use serde::{Deserialize, Serialize};

mod define;
mod name;

use define::exchanges;

pub use name::ParseExchangeError;

exchanges! {
    /// Identifies an exchange or trading venue.
    ///
    /// Variants are grouped by product family. Inside this crate the matches
    /// stay exhaustive, so adding a venue forces the match in
    /// [`hours_for_exchange`](super::hours_for_exchange) to be updated and
    /// keeps the calendar surface complete.
    ///
    /// The enum is `#[non_exhaustive]`, mirroring
    /// [`MarketHoursKey`](super::MarketHoursKey): venue coverage grows over
    /// time, and adding a variant must not be a breaking change for
    /// dependents. Match it with a wildcard arm; [`Exchange::ALL`] enumerates
    /// the variants of the version you compiled against.
    ///
    /// Holidays and product-level calendar variations are deliberately not
    /// modeled here: this enum drives only normal-week, exchange-level
    /// session defaults. [`Exchange::Unknown`] maps to a 24×7 UTC fallback.
    /// Variants have one canonical `snake_case` name (e.g.
    /// `Exchange::NasdaqBx` ↔ `"nasdaq_bx"`) used identically by serde,
    /// [`Exchange::as_str`], [`std::fmt::Display`], and
    /// [`std::str::FromStr`]; that wire form is asserted variant-by-variant
    /// in the test suite and must stay stable.
    #[non_exhaustive]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum Exchange {
        /// Unrecognized or unset venue; [`hours_for_exchange`](super::hours_for_exchange)
        /// returns a 24×7 UTC fallback. Reaching this variant is always a
        /// deliberate act: neither `FromStr` nor serde will produce it from an
        /// unrecognized name, and it is not a `Default`.
        Unknown => "unknown",
        // US Equities (ET)
        /// The Nasdaq Stock Market — the primary Nasdaq listing venue; early
        /// session from 04:00 ET.
        Nasdaq => "nasdaq",
        /// Nasdaq Texas (the stable Nasdaq BX wire identity); early session
        /// from 07:00 ET since 2011-04-18.
        NasdaqBx => "nasdaq_bx",
        /// Nasdaq PSX, the former Philadelphia Stock Exchange equities book;
        /// early session from 04:00 ET.
        NasdaqPsx => "nasdaq_psx",
        /// Cboe BZX Equities — early session from 04:00 ET (07:00 before
        /// 2025-05-01).
        CboeBzx => "cboe_bzx",
        /// Cboe BYX Equities — early session from 07:00 ET.
        CboeByx => "cboe_byx",
        /// Cboe EDGA Equities — early session from 07:00 ET.
        CboeEdga => "cboe_edga",
        /// Cboe EDGX Equities — early session from 04:00 ET (07:00 before
        /// 2021-03-08).
        CboeEdgx => "cboe_edgx",
        /// New York Stock Exchange — the core session only; NYSE Group
        /// extended trading happens on Arca, American, National, and Texas.
        Nyse => "nyse",
        /// NYSE Arca — early session from 04:00 ET, unlike the NYSE core book.
        NyseArca => "nyse_arca",
        /// NYSE American, formerly the American Stock Exchange — early
        /// session from 07:00 ET.
        NyseAmerican => "nyse_american",
        /// NYSE National — early session from 07:00 ET.
        NyseNational => "nyse_national",
        /// NYSE Texas, formerly NYSE Chicago — no sessions before its
        /// 2025-03-31 go-live.
        NyseTexas => "nyse_texas",
        /// MEMX, the Members Exchange — early session from 04:00 ET since
        /// 2025-05-19.
        MemxEq => "memx_eq",
        /// MIAX Pearl Equities — early and late sessions since 2025-02-20.
        MiaxPearlEq => "miax_pearl_eq",
        /// IEX, the Investors Exchange — System Hours 08:00–17:00 ET,
        /// narrower than the Reg NMS extended default at both ends.
        Iex => "iex",
        /// `IntelligentCross`, an ATS whose IQX book executes during regular
        /// hours.
        IntelligentcrossIqx => "intelligentcross_iqx",
        /// Blue Ocean ATS, an overnight-only US equities ATS.
        BlueOceanAts => "blue_ocean_ats",
        /// FINRA/Nasdaq Trade Reporting Facility, Carteret — an off-exchange
        /// reporting facility rather than a matching venue.
        FinraTrfCarteret => "finra_trf_carteret",
        /// FINRA/Nasdaq Trade Reporting Facility, Chicago.
        FinraTrfChicago => "finra_trf_chicago",
        /// FINRA/NYSE Trade Reporting Facility.
        FinraTrfNyse => "finra_trf_nyse",
        // US Options (ET)
        /// Cboe Options Exchange (C1) — ordinary individual-stock options'
        /// executable regular session.
        CboeOptionsC1 => "cboe_options_c1",
        /// Cboe C2 — ordinary individual-stock options' regular session.
        CboeC2Options => "cboe_c2_options",
        /// Cboe BZX — ordinary individual-stock options' regular session.
        CboeBzxOptions => "cboe_bzx_options",
        /// Cboe EDGX — ordinary individual-stock options' regular session.
        CboeEdgxOptions => "cboe_edgx_options",
        /// NYSE Arca — ordinary individual-stock options' regular session.
        NyseArcaOptions => "nyse_arca_options",
        /// NYSE American — ordinary individual-stock options' regular session.
        NyseAmericanOptions => "nyse_american_options",
        /// Nasdaq PHLX — ordinary individual-stock options' regular session.
        NasdaqPhlx => "nasdaq_phlx",
        /// Nasdaq ISE — ordinary individual-stock options' regular session.
        NasdaqIse => "nasdaq_ise",
        /// Nasdaq Options Market — ordinary individual-stock options' regular
        /// session.
        NasdaqNom => "nasdaq_nom",
        /// Nasdaq MRX — ordinary individual-stock options' regular session.
        NasdaqMrx => "nasdaq_mrx",
        /// Nasdaq GEMX — ordinary individual-stock options' regular session.
        NasdaqGemx => "nasdaq_gemx",
        /// Nasdaq Texas Options (stable BX wire name) — ordinary
        /// individual-stock options' regular session.
        NasdaqBxOptions => "nasdaq_bx_options",
        /// MIAX Options — ordinary individual-stock options' regular session.
        MiaxOptions => "miax_options",
        /// MIAX Emerald — ordinary individual-stock options' regular session.
        MiaxEmeraldOptions => "miax_emerald_options",
        /// MIAX Pearl — ordinary individual-stock options' regular session.
        MiaxPearlOptions => "miax_pearl_options",
        /// MIAX Sapphire — ordinary individual-stock options' regular session.
        MiaxSapphireOptions => "miax_sapphire_options",
        /// BOX — ordinary individual-stock options' regular session.
        BoxOptions => "box_options",
        /// MEMX — ordinary individual-stock options' regular session.
        MemxOptions => "memx_options",
        // US Futures
        /// CME, the Chicago Mercantile Exchange; the venue default is the
        /// Globex equity-index profile.
        Cme => "cme",
        /// CBOT, the Chicago Board of Trade; the venue default is the Globex
        /// grains profile, which keeps its own day session.
        Cbot => "cbot",
        /// COMEX metals, sharing the Globex energy/metals profile.
        Comex => "comex",
        /// NYMEX energy, sharing the Globex energy/metals profile.
        Nymex => "nymex",
        /// CFE, the Cboe Futures Exchange; the venue default is the VIX
        /// profile.
        Cfe => "cfe",
        // European Futures / Energy
        /// Eurex FESX/FDAX/FDXM benchmark-index futures default.
        Eurex => "eurex",
        /// EEX Nordic Zonal Power Futures.
        Eex => "eex",
        /// ICE Futures U.S.
        Iceus => "iceus",
        /// ICE Futures Europe Brent Crude Futures default.
        Iceeu => "iceeu",
        /// ICE Futures Europe Brent Crude Futures commodities identity.
        IceEuropeCommodities => "ice_europe_commodities",
        /// ICE Futures Europe FTSE 100 Index Futures.
        IceEuropeFinancials => "ice_europe_financials",
        /// ICE Endex Dutch TTF Natural Gas Futures.
        IceEndex => "ice_endex",
        /// ICE Futures Abu Dhabi Murban Crude Oil Futures.
        IceAbuDhabi => "ice_abu_dhabi",
        /// Legacy ICE Futures Canada, which ceased after the modeled 2018-07-30 migration.
        IceCanada => "ice_canada",
        // Asia-Pacific Futures
        /// SGX Three-Month SORA Futures.
        Sgx => "sgx",
        // Asia-Pacific Equities
        /// Australian Securities Exchange cash market.
        Asx => "asx",
        /// TMX Australia, formerly Cboe Australia and Chi-X Australia.
        TmxAustralia => "tmx_australia",
        /// New Zealand Exchange Main Board.
        Nzx => "nzx",
        /// Tokyo Stock Exchange cash equities, operated by Japan Exchange Group.
        Tse => "tse",
        /// National Stock Exchange of India cash equities.
        NseIndia => "nse_india",
        /// BSE (formerly Bombay Stock Exchange) cash equities.
        BseIndia => "bse_india",
        /// Hong Kong Exchanges and Clearing securities market.
        Hkex => "hkex",
        /// Singapore Exchange securities market; distinct from SGX derivatives.
        SgxSecurities => "sgx_securities",
        /// Bursa Malaysia securities market.
        BursaMalaysia => "bursa_malaysia",
        /// Stock Exchange of Thailand cash equities.
        SetThailand => "set_thailand",
        /// Indonesia Stock Exchange cash equities.
        Idx => "idx",
        /// Philippine Stock Exchange cash equities.
        Pse => "pse",
        /// Ho Chi Minh Stock Exchange cash equities.
        Hose => "hose",
        /// Shanghai Stock Exchange cash equities.
        Sse => "sse",
        /// Shenzhen Stock Exchange cash equities.
        Szse => "szse",
        /// Korea Exchange KOSPI/KOSDAQ cash-equity default.
        Krx => "krx",
        /// Taiwan Stock Exchange listed-equity market.
        Twse => "twse",
        // EU Equities
        /// London Stock Exchange (SETS).
        Lse => "lse",
        /// Xetra DAX-share venue envelope, including Extended Retail phases.
        Xetra => "xetra",
        /// SIX Swiss Exchange.
        Six => "six",
        /// Euronext Paris.
        EuronextParis => "euronext_paris",
        /// Euronext Amsterdam.
        EuronextAmsterdam => "euronext_amsterdam",
        /// Euronext Brussels.
        EuronextBrussels => "euronext_brussels",
        /// Euronext Lisbon.
        EuronextLisbon => "euronext_lisbon",
        /// Euronext Dublin.
        EuronextDublin => "euronext_dublin",
        /// Euronext Milan, formerly Borsa Italiana.
        EuronextMilan => "euronext_milan",
        /// BME, the Spanish exchange operator (Bolsa de Madrid).
        Bme => "bme",
        /// Nasdaq Stockholm.
        NasdaqStockholm => "nasdaq_stockholm",
        /// Nasdaq Helsinki — the one Nordic book quoted in EET rather than
        /// CET.
        NasdaqHelsinki => "nasdaq_helsinki",
        /// Nasdaq Copenhagen.
        NasdaqCopenhagen => "nasdaq_copenhagen",
        /// Wiener Börse, the Vienna Stock Exchange.
        Vienna => "vienna",
        // Other Global Equities
        /// Borsa Istanbul Equity Market.
        BorsaIstanbul => "borsa_istanbul",
        /// Toronto Stock Exchange cash equities.
        Tsx => "tsx",
        /// Johannesburg Stock Exchange main-board cash equities.
        Jse => "jse",
        /// Saudi Exchange (Tadawul) Main Market cash equities.
        Tadawul => "tadawul",
        /// B3 (Brasil, Bolsa, Balcão) cash equities.
        B3 => "b3",
        /// Bolsa Mexicana de Valores cash equities.
        Bmv => "bmv",
        // Crypto / always-open venues
        // These trade continuously without daily maintenance breaks and stay
        // separate from futures venues with daily and weekend boundaries.
        /// Binance USDⓈ-M perpetual futures normal 24×7 availability.
        BinanceFutures => "binance_futures",
    }
}
