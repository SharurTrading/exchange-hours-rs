// SPDX-License-Identifier: MIT-0

//! Current published hours for every [`Exchange`] variant.
//!
//! The bulk of this module is one exhaustive `match` over `Exchange`, which is
//! load-bearing: there is no catch-all arm, so adding a venue is a compile
//! error until someone decides its hours. Splitting the match across modules
//! would replace that guarantee with a runtime fallthrough, so the arms stay
//! together. The data and its history live in venue-owned modules under
//! [`super::super::schedules`].

use crate::calendar::schedules::equities::{africa_middle_east, americas, apac, europe, us};
use crate::calendar::schedules::from_profile;
use crate::calendar::schedules::futures::{international, us as futures_us};
use crate::calendar::{Exchange, MarketHours, MarketHoursKey, session_profile};

/// Builds the default fixed trading-hours snapshot for an exchange.
///
/// Product-level variations may differ. This date-free function necessarily
/// returns one compatibility snapshot for every venue, including profiles with
/// recurring date selection: B3 and BMV follow New York offset relationships,
/// Vienna has a third-Friday settlement grid, Eurex's Asian open is fixed in
/// UTC, and ICE Endex / ICE Abu Dhabi follow New York reference clocks. Use
/// [`hours_for_exchange_as_of`](crate::hours_for_exchange_as_of) for one dated
/// snapshot and [`calendar_for_exchange`](crate::calendar_for_exchange) for a
/// query or scan that can cross a transition. The compatibility choices are
/// B3's short grid, BMV's early grid, Vienna's ordinary non-settlement grid,
/// and the current-season reference-clock grids for the three derivatives
/// profiles.
#[expect(
    clippy::too_many_lines,
    reason = "one exhaustive match is the compile-time fence for every Exchange variant"
)]
#[must_use]
pub fn hours_for_exchange(exch: Exchange) -> MarketHours {
    match exch {
        Exchange::Unknown => default_24x7(exch),
        // US cash equities. Distinct profile names keep venue amendments local.
        Exchange::Nasdaq => from_profile(exch, &us::NASDAQ_PROFILE),
        Exchange::NasdaqBx => from_profile(exch, &us::NASDAQ_BX_PROFILE),
        Exchange::NasdaqPsx => from_profile(exch, &us::NASDAQ_PSX_PROFILE),
        Exchange::CboeBzx => from_profile(exch, &us::CBOE_BZX_PROFILE),
        Exchange::CboeByx => from_profile(exch, &us::CBOE_BYX_PROFILE),
        Exchange::CboeEdga => from_profile(exch, &us::CBOE_EDGA_PROFILE),
        Exchange::CboeEdgx => from_profile(exch, &us::CBOE_EDGX_PROFILE),
        Exchange::Nyse => from_profile(exch, &us::NYSE_PROFILE),
        Exchange::NyseArca => from_profile(exch, &us::NYSE_ARCA_PROFILE),
        Exchange::NyseAmerican => from_profile(exch, &us::NYSE_AMERICAN_PROFILE),
        Exchange::NyseNational => from_profile(exch, &us::NYSE_NATIONAL_PROFILE),
        Exchange::NyseTexas => from_profile(exch, &us::NYSE_TEXAS_PROFILE),
        Exchange::MemxEq => from_profile(exch, &us::MEMX_EQ_PROFILE),
        Exchange::MiaxPearlEq => from_profile(exch, &us::MIAX_PEARL_EQ_PROFILE),
        Exchange::Iex => from_profile(exch, &us::IEX_PROFILE),
        Exchange::Ltse => from_profile(exch, &us::LTSE_PROFILE),
        Exchange::TwentyFourX => from_profile(exch, &us::TWENTY_FOUR_X_PROFILE),
        Exchange::Txse => from_profile(exch, &us::TXSE_PROFILE),
        Exchange::BlueOceanAts => from_profile(exch, &us::BLUE_OCEAN_PROFILE),
        Exchange::FinraTrfCarteret => from_profile(exch, &us::FINRA_TRF_CARTERET_PROFILE),
        Exchange::FinraTrfChicago => from_profile(exch, &us::FINRA_TRF_CHICAGO_PROFILE),
        Exchange::FinraTrfNyse => from_profile(exch, &us::FINRA_TRF_NYSE_PROFILE),

        // US options. The rule slices may coincide; the venue profiles do not.
        Exchange::CboeOptionsC1 => from_profile(exch, &us::CBOE_OPTIONS_C1_PROFILE),
        Exchange::CboeC2Options => from_profile(exch, &us::CBOE_C2_OPTIONS_PROFILE),
        Exchange::CboeBzxOptions => from_profile(exch, &us::CBOE_BZX_OPTIONS_PROFILE),
        Exchange::CboeEdgxOptions => from_profile(exch, &us::CBOE_EDGX_OPTIONS_PROFILE),
        Exchange::NyseArcaOptions => from_profile(exch, &us::NYSE_ARCA_OPTIONS_PROFILE),
        Exchange::NyseAmericanOptions => from_profile(exch, &us::NYSE_AMERICAN_OPTIONS_PROFILE),
        Exchange::NasdaqPhlx => from_profile(exch, &us::NASDAQ_PHLX_OPTIONS_PROFILE),
        Exchange::NasdaqIse => from_profile(exch, &us::NASDAQ_ISE_OPTIONS_PROFILE),
        Exchange::NasdaqNom => from_profile(exch, &us::NASDAQ_NOM_OPTIONS_PROFILE),
        Exchange::NasdaqMrx => from_profile(exch, &us::NASDAQ_MRX_OPTIONS_PROFILE),
        Exchange::NasdaqGemx => from_profile(exch, &us::NASDAQ_GEMX_OPTIONS_PROFILE),
        Exchange::NasdaqBxOptions => from_profile(exch, &us::NASDAQ_BX_OPTIONS_PROFILE),
        Exchange::MiaxOptions => from_profile(exch, &us::MIAX_OPTIONS_PROFILE),
        Exchange::MiaxEmeraldOptions => from_profile(exch, &us::MIAX_EMERALD_OPTIONS_PROFILE),
        Exchange::MiaxPearlOptions => from_profile(exch, &us::MIAX_PEARL_OPTIONS_PROFILE),
        Exchange::MiaxSapphireOptions => from_profile(exch, &us::MIAX_SAPPHIRE_OPTIONS_PROFILE),
        Exchange::BoxOptions => from_profile(exch, &us::BOX_OPTIONS_PROFILE),
        Exchange::MemxOptions => from_profile(exch, &us::MEMX_OPTIONS_PROFILE),

        // Venue-owned international futures and energy profiles.
        Exchange::IceEuropeCommodities => {
            from_profile(exch, &international::ICE_EUROPE_COMMODITIES_CURRENT)
        }
        Exchange::IceEuropeFinancials => {
            from_profile(exch, &international::ICE_EUROPE_FINANCIALS_CURRENT)
        }
        Exchange::IceEndex => from_profile(exch, &international::ICE_ENDEX_CURRENT),
        Exchange::IceAbuDhabi => from_profile(exch, &international::ICE_ABU_DHABI_CURRENT),
        Exchange::IceCanada => from_profile(exch, &international::ICE_CANADA_PROFILE),
        Exchange::Eex => from_profile(exch, &international::EEX_PROFILE),

        // Asia-Pacific equities.
        Exchange::Asx => from_profile(exch, apac::asx::CURRENT),
        Exchange::TmxAustralia => from_profile(exch, apac::tmx_australia::CURRENT),
        Exchange::Nzx => from_profile(exch, apac::nzx::CURRENT),
        Exchange::Tse => from_profile(exch, apac::tse::CURRENT),
        Exchange::NseIndia => from_profile(exch, apac::nse::CURRENT),
        Exchange::BseIndia => from_profile(exch, apac::bse::CURRENT),
        Exchange::Hkex => from_profile(exch, apac::hkex::CURRENT),
        Exchange::SgxSecurities => from_profile(exch, apac::sgx::CURRENT),
        Exchange::BursaMalaysia => from_profile(exch, apac::bursa::CURRENT),
        Exchange::SetThailand => from_profile(exch, apac::set::CURRENT),
        Exchange::Idx => from_profile(exch, apac::idx::CURRENT),
        Exchange::Pse => from_profile(exch, apac::pse::CURRENT),
        Exchange::Hose => from_profile(exch, apac::hose::CURRENT),
        Exchange::Sse => from_profile(exch, apac::sse::CURRENT),
        Exchange::Szse => from_profile(exch, apac::szse::CURRENT),
        Exchange::Krx => from_profile(exch, apac::krx::CURRENT),
        Exchange::Twse => from_profile(exch, apac::twse::CURRENT),

        // European equities.
        Exchange::Lse => from_profile(exch, &europe::lse::LSE_PROFILE),
        Exchange::Xetra => from_profile(exch, &europe::xetra::XETRA_PROFILE),
        Exchange::Six => from_profile(exch, &europe::six::SIX_PROFILE),
        Exchange::EuronextParis => from_profile(exch, &europe::euronext::EURONEXT_PARIS_PROFILE),
        Exchange::EuronextAmsterdam => from_profile(exch, &europe::euronext::EURONEXT_AMS_PROFILE),
        Exchange::EuronextBrussels => from_profile(exch, &europe::euronext::EURONEXT_BRU_PROFILE),
        Exchange::EuronextLisbon => from_profile(exch, &europe::euronext::EURONEXT_LIS_PROFILE),
        Exchange::EuronextDublin => from_profile(exch, &europe::euronext::EURONEXT_DUB_PROFILE),
        Exchange::EuronextMilan => from_profile(exch, &europe::euronext::EURONEXT_MIL_PROFILE),
        Exchange::Bme => from_profile(exch, &europe::bme::BME_PROFILE),
        Exchange::NasdaqStockholm => {
            from_profile(exch, &europe::nasdaq_nordics::NASDAQ_STO_PROFILE)
        }
        Exchange::NasdaqHelsinki => from_profile(exch, &europe::nasdaq_nordics::NASDAQ_HEL_PROFILE),
        Exchange::NasdaqCopenhagen => {
            from_profile(exch, &europe::nasdaq_nordics::NASDAQ_CPH_PROFILE)
        }
        // Date-free compatibility snapshot uses an ordinary (non-settlement)
        // ATX day. Use `calendar_for_exchange` across multiple dates so every
        // third Friday selects Vienna's published settlement grid.
        Exchange::Vienna => from_profile(exch, &europe::vienna::VIENNA_PROFILE),

        // Other major global equities.
        Exchange::BorsaIstanbul => from_profile(exch, europe::bist::CURRENT),
        Exchange::Tsx => from_profile(exch, americas::tsx::CURRENT),
        Exchange::Jse => from_profile(exch, africa_middle_east::jse::CURRENT),
        Exchange::Tadawul => from_profile(exch, africa_middle_east::tadawul::CURRENT),
        // Date-free compatibility snapshots. B3/BMV users scanning more than
        // one date should use `calendar_for_exchange`, which reselects the
        // published grid at every candidate trading day.
        Exchange::B3 => from_profile(exch, americas::b3::CURRENT),
        Exchange::Bmv => from_profile(exch, americas::bmv::CURRENT),

        // Futures and crypto venue defaults. Some reuse the matching named
        // key; the remainder route directly to their sourced product-family
        // profile.
        Exchange::Cme => session_profile(MarketHoursKey::GlobexEquityIndex).to_market_hours(exch),
        Exchange::Cbot => session_profile(MarketHoursKey::GlobexGrains).to_market_hours(exch),
        Exchange::Comex | Exchange::Nymex => from_profile(exch, &futures_us::ENERGY_METALS_CURRENT),
        Exchange::Eurex => session_profile(MarketHoursKey::Eurex).to_market_hours(exch),
        Exchange::Iceus => from_profile(exch, &futures_us::ICE_US_FANG_CURRENT),
        Exchange::Iceeu => from_profile(exch, &international::ICEEU_CURRENT),
        Exchange::Sgx => session_profile(MarketHoursKey::Sgx).to_market_hours(exch),
        Exchange::Cfe => session_profile(MarketHoursKey::CfeVix).to_market_hours(exch),
        Exchange::BinanceFutures => from_profile(exch, &international::BINANCE_CURRENT),
        // All known exchanges covered; no default arm.
    }
}

/// Tags the shared 24×7 UTC profile with an always-open venue.
fn default_24x7(ex: Exchange) -> MarketHours {
    session_profile(MarketHoursKey::AlwaysOpen).to_market_hours(ex)
}
