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
//! Each family that has a table owns a `holidays/<owner>.rs` module beside
//! this file; an identity with no table answers `None` and is unaffected by
//! the holiday layer entirely. The four `Exchange` venues CME routes to —
//! `Cme`, `Cbot`, `Comex`, `Nymex` — share `holidays/venues.rs`, because their
//! tables are not retrieved at all: each is the **intersection** of the
//! families that route to the venue (design memo D17), so the four are one
//! derivation over one corpus rather than four independent bodies of evidence.

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
        Exchange::Cme => Some(super::venues::CME),
        Exchange::Cbot => Some(super::venues::CBOT),
        Exchange::Comex => Some(super::venues::COMEX),
        Exchange::Nymex => Some(super::venues::NYMEX),
        Exchange::Cfe => Some(super::cfe::TABLE),
        Exchange::CoinbaseDerivatives => Some(super::coinbase_derivatives::TABLE),
        Exchange::Smfe => None,
        Exchange::Eurex => Some(super::eurex::TABLE),
        Exchange::Eex => None,
        Exchange::Iceus => Some(super::ice_us::VENUE),
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
        MarketHoursKey::GlobexEquityIndex => Some(super::globex_equity_index::TABLE),
        MarketHoursKey::GlobexEnergy => Some(super::globex_energy::TABLE),
        MarketHoursKey::GlobexGrains => Some(super::globex_grains::TABLE),
        MarketHoursKey::GlobexMiniGrains => None,
        MarketHoursKey::GlobexFx => Some(super::globex_fx::TABLE),
        MarketHoursKey::GlobexInterestRates => Some(super::globex_interest_rates::TABLE),
        MarketHoursKey::GlobexLivestock => Some(super::globex_livestock::TABLE),
        MarketHoursKey::GlobexCryptocurrency => Some(super::globex_cryptocurrency::TABLE),
        MarketHoursKey::CfeVix => Some(super::cfe::TABLE),
        MarketHoursKey::Eurex => Some(super::eurex::TABLE),
        MarketHoursKey::IceUs => Some(super::ice_us::FANG),
        MarketHoursKey::IceUsSugar => Some(super::ice_us::SUGAR_COFFEE_COCOA),
        MarketHoursKey::IceUsCoffee => Some(super::ice_us::SUGAR_COFFEE_COCOA),
        MarketHoursKey::IceUsCocoa => Some(super::ice_us::SUGAR_COFFEE_COCOA),
        MarketHoursKey::IceUsCotton => Some(super::ice_us::COTTON),
        MarketHoursKey::IceUsOrangeJuice => Some(super::ice_us::ORANGE_JUICE),
        MarketHoursKey::IceUsDollarIndex => Some(super::ice_us::DOLLAR_INDEX),
        MarketHoursKey::GlobexNikkei225Dollar => Some(super::globex_nikkei_225_dollar::TABLE),
        MarketHoursKey::EurexFixedIncome => Some(super::eurex::TABLE),
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
