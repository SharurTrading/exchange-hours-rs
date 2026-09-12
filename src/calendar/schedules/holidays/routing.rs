// SPDX-License-Identifier: MIT-0

//! The identity-to-table match, with no catch-all arm.
//!
//! This mirrors `hours_for_market_hours_key` and `hours_for_exchange`: there is
//! no wildcard, so adding an `Exchange` or a `MarketHoursKey` is a compile
//! error until someone decides whether that identity has a holiday table.
//! LAW-SERVICE-TIERS makes that a decision, not a default — a served identity
//! owes a table, a dormant one owes best effort — and a wildcard would hand it
//! silently to whichever answer the wildcard happened to give.
//!
//! **Every arm is `None` in this wave.** Wave 0 ships the engine, the row
//! types, the macro, the gate, the public accessors and the fences, and merges
//! with zero rows and therefore zero behaviour change; the first family tables
//! land beside this file as `holidays/<owner>.rs` modules in Wave 1.

use super::HolidayTable;
use crate::calendar::{CalendarSource, Exchange, MarketHoursKey};

/// Returns the built-in holiday table for `source`, if one ships.
///
/// The result is resolved once per query and is `Copy`-cheap: a table is a
/// `&'static` borrow of a static slice, so an identity with a table costs one
/// pointer and an identity without one costs nothing at all.
pub(crate) const fn table_for(source: CalendarSource) -> Option<&'static HolidayTable> {
    match source {
        CalendarSource::Exchange(exchange) => for_exchange(exchange),
        CalendarSource::MarketHoursKey(key) => for_market_hours_key(key),
    }
}

/// The venue half of the match. A venue calendar's table is the intersection of
/// the families that route to it, so a date on which two families disagree
/// ships no row and is a declared gap in the venue's evidence file.
#[expect(
    clippy::match_same_arms,
    reason = "the per-variant arms are the fence: one arm per identity is what a \
              reviewer edits when that venue's table lands, and collapsing them \
              would let the next venue inherit an answer nobody decided"
)]
const fn for_exchange(exchange: Exchange) -> Option<&'static HolidayTable> {
    match exchange {
        Exchange::Unknown => None,
        Exchange::Nasdaq => None,
        Exchange::NasdaqBx => None,
        Exchange::NasdaqPsx => None,
        Exchange::CboeBzx => None,
        Exchange::CboeByx => None,
        Exchange::CboeEdga => None,
        Exchange::CboeEdgx => None,
        Exchange::Nyse => None,
        Exchange::NyseArca => None,
        Exchange::NyseAmerican => None,
        Exchange::NyseNational => None,
        Exchange::NyseTexas => None,
        Exchange::MemxEq => None,
        Exchange::MiaxPearlEq => None,
        Exchange::Iex => None,
        Exchange::Ltse => None,
        Exchange::TwentyFourX => None,
        Exchange::Txse => None,
        Exchange::BlueOceanAts => None,
        Exchange::FinraTrfCarteret => None,
        Exchange::FinraTrfChicago => None,
        Exchange::FinraTrfNyse => None,
        Exchange::CboeOptionsC1 => None,
        Exchange::CboeC2Options => None,
        Exchange::CboeBzxOptions => None,
        Exchange::CboeEdgxOptions => None,
        Exchange::NyseArcaOptions => None,
        Exchange::NyseAmericanOptions => None,
        Exchange::NasdaqPhlx => None,
        Exchange::NasdaqIse => None,
        Exchange::NasdaqNom => None,
        Exchange::NasdaqMrx => None,
        Exchange::NasdaqGemx => None,
        Exchange::NasdaqBxOptions => None,
        Exchange::MiaxOptions => None,
        Exchange::MiaxEmeraldOptions => None,
        Exchange::MiaxPearlOptions => None,
        Exchange::MiaxSapphireOptions => None,
        Exchange::BoxOptions => None,
        Exchange::MemxOptions => None,
        Exchange::Cme => None,
        Exchange::Cbot => None,
        Exchange::Comex => None,
        Exchange::Nymex => None,
        Exchange::Cfe => None,
        Exchange::CoinbaseDerivatives => None,
        Exchange::Smfe => None,
        Exchange::Eurex => None,
        Exchange::Eex => None,
        Exchange::Iceus => None,
        Exchange::Iceeu => None,
        Exchange::IceEuropeCommodities => None,
        Exchange::IceEuropeFinancials => None,
        Exchange::IceEndex => None,
        Exchange::IceAbuDhabi => None,
        Exchange::IceCanada => None,
        Exchange::Sgx => None,
        Exchange::Asx => None,
        Exchange::TmxAustralia => None,
        Exchange::Nzx => None,
        Exchange::Tse => None,
        Exchange::NseIndia => None,
        Exchange::BseIndia => None,
        Exchange::Hkex => None,
        Exchange::SgxSecurities => None,
        Exchange::BursaMalaysia => None,
        Exchange::SetThailand => None,
        Exchange::Idx => None,
        Exchange::Pse => None,
        Exchange::Hose => None,
        Exchange::Sse => None,
        Exchange::Szse => None,
        Exchange::Krx => None,
        Exchange::Twse => None,
        Exchange::Lse => None,
        Exchange::Xetra => None,
        Exchange::Six => None,
        Exchange::EuronextParis => None,
        Exchange::EuronextAmsterdam => None,
        Exchange::EuronextBrussels => None,
        Exchange::EuronextLisbon => None,
        Exchange::EuronextDublin => None,
        Exchange::EuronextMilan => None,
        Exchange::Bme => None,
        Exchange::NasdaqStockholm => None,
        Exchange::NasdaqHelsinki => None,
        Exchange::NasdaqCopenhagen => None,
        Exchange::Vienna => None,
        Exchange::BorsaIstanbul => None,
        Exchange::Tsx => None,
        Exchange::Jse => None,
        Exchange::Tadawul => None,
        Exchange::B3 => None,
        Exchange::Bmv => None,
        Exchange::BinanceFutures => None,
    }
}

/// The product-family half of the match. A family owns its own calendar, so
/// this is where a served key's table attaches.
#[expect(
    clippy::match_same_arms,
    reason = "the per-variant arms are the fence: one arm per family is what a \
              reviewer edits when that family's table lands, and collapsing them \
              would let the next key inherit an answer nobody decided"
)]
const fn for_market_hours_key(key: MarketHoursKey) -> Option<&'static HolidayTable> {
    match key {
        MarketHoursKey::GlobexEquityIndex => None,
        MarketHoursKey::GlobexEnergy => None,
        MarketHoursKey::GlobexGrains => None,
        MarketHoursKey::GlobexMiniGrains => None,
        MarketHoursKey::GlobexFx => None,
        MarketHoursKey::GlobexInterestRates => None,
        MarketHoursKey::GlobexLivestock => None,
        MarketHoursKey::GlobexCryptocurrency => None,
        MarketHoursKey::CfeVix => None,
        MarketHoursKey::Eurex => None,
        MarketHoursKey::IceUs => None,
        MarketHoursKey::IceUsSugar => None,
        MarketHoursKey::IceUsCoffee => None,
        MarketHoursKey::IceUsCocoa => None,
        MarketHoursKey::IceUsCotton => None,
        MarketHoursKey::IceUsOrangeJuice => None,
        MarketHoursKey::IceUsDollarIndex => None,
        MarketHoursKey::GlobexNikkei225Dollar => None,
        MarketHoursKey::EurexFixedIncome => None,
        MarketHoursKey::SgxEquityIndexJapan => None,
        MarketHoursKey::SgxEquityIndexChina => None,
        MarketHoursKey::SgxEquityIndexSingapore => None,
        MarketHoursKey::SgxEquityIndexTaiwan => None,
        MarketHoursKey::SgxEquityIndexNtrUsd => None,
        MarketHoursKey::GlobexRoughRice => None,
        MarketHoursKey::GlobexWeather => None,
        MarketHoursKey::GlobexSpotQuoted => None,
        MarketHoursKey::GlobexEventContracts => None,
        MarketHoursKey::GlobexEventContractsBtc => None,
        MarketHoursKey::GlobexGoldTas => None,
        MarketHoursKey::GlobexSilverTas => None,
        MarketHoursKey::GlobexCopperTas => None,
        MarketHoursKey::GlobexPlatinumTas => None,
        MarketHoursKey::GlobexPalladiumTas => None,
        MarketHoursKey::Sgx => None,
        MarketHoursKey::AlwaysOpen => None,
    }
}
