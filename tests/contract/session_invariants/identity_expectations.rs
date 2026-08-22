// SPDX-License-Identifier: MIT-0

//! Handwritten expectations kept independent of the production exchange table.

use super::prelude::*;

/// Number of [`Exchange`] variants. `Exchange`, `Exchange::ALL`, and
/// `Exchange::as_str` are generated from one table by the library's
/// `exchanges!` macro, so `ALL` is complete by construction; this count and
/// the independent list below are the *expectation* side:
/// `all_exchanges_matches_the_crates_own_list` compares them against the
/// generated `ALL`, so a row accidentally dropped from (or mis-ordered in)
/// the library's table fails here instead of silently shrinking the grids.
/// A new venue bumps this count and adds one entry below.
pub(super) const EXCHANGE_VARIANT_COUNT: usize = 91;

/// Every [`Exchange`] variant, maintained by hand and on purpose
/// independently of the generated `Exchange::ALL`: this list is the test's
/// own expectation of what the library's table contains, compared element by
/// element below.
pub(super) const ALL_EXCHANGES: &[Exchange] = &[
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
    Exchange::Asx,
    Exchange::TmxAustralia,
    Exchange::Nzx,
    Exchange::Tse,
    Exchange::NseIndia,
    Exchange::BseIndia,
    Exchange::Hkex,
    Exchange::SgxSecurities,
    Exchange::BursaMalaysia,
    Exchange::SetThailand,
    Exchange::Idx,
    Exchange::Pse,
    Exchange::Hose,
    Exchange::Sse,
    Exchange::Szse,
    Exchange::Krx,
    Exchange::Twse,
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
    Exchange::BorsaIstanbul,
    Exchange::Tsx,
    Exchange::Jse,
    Exchange::Tadawul,
    Exchange::B3,
    Exchange::Bmv,
    Exchange::BinanceFutures,
];
