// SPDX-License-Identifier: MIT-0

//! The canonical name of every [`Exchange`] variant, and the list of all of
//! them.
//!
//! One venue has exactly one `snake_case` name, shared by serde,
//! [`Exchange::as_str`], `Display`, and `FromStr` — callers holding a string
//! (a config key, a wire message, a CLI argument) parse it directly instead of
//! deserializing through a format crate or pattern-matching by hand.
//!
//! [`Exchange::as_str`] is the crate's compile-time fence for variant
//! coverage: its match has no wildcard arm, so a new variant does not compile
//! until it is named here — and the doc on that match lists the other tables
//! the same edit must reach. `FromStr` searches [`Exchange::ALL`] for a
//! matching name rather than repeating the table, so the two cannot disagree;
//! the test suite closes the remaining gap by checking `ALL` against an
//! independently maintained list, in order, and round-tripping every entry
//! through serde and `FromStr`.

use super::Exchange;

impl Exchange {
    /// Every variant, in declaration (and therefore `Ord`) order.
    ///
    /// This is the version-you-compiled-against enumeration: the enum is
    /// `#[non_exhaustive]`, so new venues appear here in later releases.
    /// Kept complete by the test suite's variant-count and ordering fences.
    pub const ALL: &'static [Exchange] = &[
        Exchange::Unknown,
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
        Exchange::FinraTrfCarteret,
        Exchange::FinraTrfChicago,
        Exchange::FinraTrfNyse,
        Exchange::CboeOptionsC1,
        Exchange::CboeC2Options,
        Exchange::CboeBzxOptions,
        Exchange::CboeEdgxOptions,
        Exchange::NyseArcaOptions,
        Exchange::NyseAmericanOptions,
        Exchange::NasdaqPhlx,
        Exchange::NasdaqIse,
        Exchange::NasdaqNom,
        Exchange::NasdaqMrx,
        Exchange::NasdaqGemx,
        Exchange::NasdaqBxOptions,
        Exchange::MiaxOptions,
        Exchange::MiaxEmeraldOptions,
        Exchange::MiaxPearlOptions,
        Exchange::MiaxSapphireOptions,
        Exchange::BoxOptions,
        Exchange::MemxOptions,
        Exchange::Cme,
        Exchange::Cbot,
        Exchange::Comex,
        Exchange::Nymex,
        Exchange::Cfe,
        Exchange::Eurex,
        Exchange::Eex,
        Exchange::Iceus,
        Exchange::Iceeu,
        Exchange::IceEuropeCommodities,
        Exchange::IceEuropeFinancials,
        Exchange::IceEndex,
        Exchange::IceAbuDhabi,
        Exchange::IceCanada,
        Exchange::Sgx,
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
        Exchange::BinanceFutures,
    ];

    /// The variant's canonical `snake_case` name — the same string serde
    /// writes and [`FromStr`](std::str::FromStr) accepts.
    ///
    /// No wildcard arm on purpose: this match is the in-crate compile fence
    /// for a new variant. When it forces you here, also add the variant to
    /// [`Exchange::ALL`] above, to the region list in `exchange.rs` if a bulk
    /// builder covers it, and to the test suite's independent list.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Exchange::Unknown => "unknown",
            Exchange::Nasdaq => "nasdaq",
            Exchange::NasdaqBx => "nasdaq_bx",
            Exchange::NasdaqPsx => "nasdaq_psx",
            Exchange::CboeBzx => "cboe_bzx",
            Exchange::CboeByx => "cboe_byx",
            Exchange::CboeEdga => "cboe_edga",
            Exchange::CboeEdgx => "cboe_edgx",
            Exchange::Nyse => "nyse",
            Exchange::NyseArca => "nyse_arca",
            Exchange::NyseAmerican => "nyse_american",
            Exchange::NyseNational => "nyse_national",
            Exchange::NyseTexas => "nyse_texas",
            Exchange::MemxEq => "memx_eq",
            Exchange::MiaxPearlEq => "miax_pearl_eq",
            Exchange::Iex => "iex",
            Exchange::IntelligentcrossIqx => "intelligentcross_iqx",
            Exchange::BlueOceanAts => "blue_ocean_ats",
            Exchange::FinraTrfCarteret => "finra_trf_carteret",
            Exchange::FinraTrfChicago => "finra_trf_chicago",
            Exchange::FinraTrfNyse => "finra_trf_nyse",
            Exchange::CboeOptionsC1 => "cboe_options_c1",
            Exchange::CboeC2Options => "cboe_c2_options",
            Exchange::CboeBzxOptions => "cboe_bzx_options",
            Exchange::CboeEdgxOptions => "cboe_edgx_options",
            Exchange::NyseArcaOptions => "nyse_arca_options",
            Exchange::NyseAmericanOptions => "nyse_american_options",
            Exchange::NasdaqPhlx => "nasdaq_phlx",
            Exchange::NasdaqIse => "nasdaq_ise",
            Exchange::NasdaqNom => "nasdaq_nom",
            Exchange::NasdaqMrx => "nasdaq_mrx",
            Exchange::NasdaqGemx => "nasdaq_gemx",
            Exchange::NasdaqBxOptions => "nasdaq_bx_options",
            Exchange::MiaxOptions => "miax_options",
            Exchange::MiaxEmeraldOptions => "miax_emerald_options",
            Exchange::MiaxPearlOptions => "miax_pearl_options",
            Exchange::MiaxSapphireOptions => "miax_sapphire_options",
            Exchange::BoxOptions => "box_options",
            Exchange::MemxOptions => "memx_options",
            Exchange::Cme => "cme",
            Exchange::Cbot => "cbot",
            Exchange::Comex => "comex",
            Exchange::Nymex => "nymex",
            Exchange::Cfe => "cfe",
            Exchange::Eurex => "eurex",
            Exchange::Eex => "eex",
            Exchange::Iceus => "iceus",
            Exchange::Iceeu => "iceeu",
            Exchange::IceEuropeCommodities => "ice_europe_commodities",
            Exchange::IceEuropeFinancials => "ice_europe_financials",
            Exchange::IceEndex => "ice_endex",
            Exchange::IceAbuDhabi => "ice_abu_dhabi",
            Exchange::IceCanada => "ice_canada",
            Exchange::Sgx => "sgx",
            Exchange::Lse => "lse",
            Exchange::Xetra => "xetra",
            Exchange::Six => "six",
            Exchange::EuronextParis => "euronext_paris",
            Exchange::EuronextAmsterdam => "euronext_amsterdam",
            Exchange::EuronextBrussels => "euronext_brussels",
            Exchange::EuronextLisbon => "euronext_lisbon",
            Exchange::EuronextDublin => "euronext_dublin",
            Exchange::EuronextMilan => "euronext_milan",
            Exchange::Bme => "bme",
            Exchange::NasdaqStockholm => "nasdaq_stockholm",
            Exchange::NasdaqHelsinki => "nasdaq_helsinki",
            Exchange::NasdaqCopenhagen => "nasdaq_copenhagen",
            Exchange::Vienna => "vienna",
            Exchange::BinanceFutures => "binance_futures",
        }
    }
}

impl core::fmt::Display for Exchange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl core::str::FromStr for Exchange {
    type Err = ParseExchangeError;

    /// Parses a canonical `snake_case` venue name (`"nyse_arca"`,
    /// `"binance_futures"`, …) — exactly the strings [`Exchange::as_str`]
    /// returns and serde writes, matched case-sensitively.
    ///
    /// An unrecognized name is an error, **not** [`Exchange::Unknown`]:
    /// `Unknown` is a value a caller chooses deliberately (its name
    /// `"unknown"` parses like any other), never a silent default for a typo.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Exchange::ALL
            .iter()
            .copied()
            .find(|exchange| exchange.as_str() == s)
            .ok_or_else(|| ParseExchangeError { input: s.into() })
    }
}

/// The error [`Exchange`]'s [`FromStr`](core::str::FromStr) returns for a
/// string that is not a canonical venue name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseExchangeError {
    input: Box<str>,
}

impl ParseExchangeError {
    /// The string that failed to parse.
    #[must_use]
    pub fn input(&self) -> &str {
        &self.input
    }
}

impl core::fmt::Display for ParseExchangeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:?} is not a known exchange name; expected a snake_case name such as \"cme\" or \"nyse_arca\"",
            self.input
        )
    }
}

impl std::error::Error for ParseExchangeError {}
