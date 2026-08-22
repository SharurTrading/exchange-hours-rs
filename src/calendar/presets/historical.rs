// SPDX-License-Identifier: MIT-0

//! Point-in-time venue schedule routing.
//!
//! Literal tables, effective dates, source citations, and selection logic live
//! together in the owning module under [`super::super::schedules`]. This file
//! only preserves the public `Exchange` dispatch and the default-to-current
//! behavior for venues with no recorded revision.
//!
//! **Evidence rule.** A selector is added only when the owning venue module has
//! primary evidence for an unconditional, day-level boundary
//! (LAW-NO-FABRICATED-DATES). The current verification ledger records no
//! modeled-history gap within any row's stated scope. Undated changes are
//! disclosed, and conditional future plans remain unselected, rather than this
//! router inventing or prematurely activating a cutover.

use chrono::{DateTime, Utc};

use crate::calendar::local_time::bounded_utc;
use crate::calendar::schedules::equities::{africa_middle_east, americas, apac, europe, us};
use crate::calendar::schedules::from_profile;
use crate::calendar::schedules::futures::{international, us as futures_us};
use crate::calendar::{Exchange, MarketHours};

use super::hours_for_exchange;

/// Returns the fixed schedule snapshot in effect at `as_of`.
///
/// Date-only changes are interpreted at venue-local midnight on the session's
/// opening day. A source-stated intraday boundary is preserved at its exact UTC
/// instant. Recurring selectors choose the applicable B3/BMV New York-offset
/// grid, Vienna third-Friday grid, Eurex fixed-UTC Asian open, or ICE Endex /
/// ICE Abu Dhabi New York-reference grid for the venue-local day containing
/// `as_of`. A caller scanning across later schedule transitions should use
/// [`calendar_for_exchange`](crate::calendar_for_exchange), which reselects at
/// every candidate opening day.
///
/// Venues without recorded historical revisions return their current profile.
/// See the module documentation for the evidence rule applied to revisions.
#[must_use]
pub fn hours_for_exchange_as_of(exch: Exchange, as_of: DateTime<Utc>) -> MarketHours {
    let current = hours_for_exchange(exch);
    let as_of = bounded_utc(as_of, current.tz);
    let profile = match exch {
        Exchange::Nasdaq => us::nasdaq_profile_at(as_of),
        Exchange::NasdaqBx => us::nasdaq_bx_profile_at(as_of),
        Exchange::NasdaqPsx => us::nasdaq_psx_profile_at(as_of),
        Exchange::CboeEdgx => us::edgx_profile_at(as_of),
        Exchange::CboeBzx => us::bzx_profile_at(as_of),
        Exchange::CboeByx => us::byx_profile_at(as_of),
        Exchange::CboeEdga => us::edga_profile_at(as_of),
        Exchange::CboeOptionsC1 => us::c1_profile_at(as_of),
        Exchange::CboeC2Options => us::c2_options_profile_at(as_of),
        Exchange::CboeBzxOptions => us::bzx_options_profile_at(as_of),
        Exchange::CboeEdgxOptions => us::edgx_options_profile_at(as_of),
        Exchange::NasdaqMrx => us::nasdaq_mrx_profile_at(as_of),
        Exchange::NasdaqGemx => us::nasdaq_gemx_profile_at(as_of),
        Exchange::NasdaqBxOptions => us::nasdaq_bx_options_profile_at(as_of),
        Exchange::MiaxOptions => us::miax_options_profile_at(as_of),
        Exchange::MiaxEmeraldOptions => us::miax_emerald_options_profile_at(as_of),
        Exchange::MiaxPearlOptions => us::miax_pearl_options_profile_at(as_of),
        Exchange::MiaxSapphireOptions => us::miax_sapphire_options_profile_at(as_of),
        Exchange::MemxOptions => us::memx_options_profile_at(as_of),
        Exchange::Iex => us::iex_profile_at(as_of),
        Exchange::NyseAmerican => us::nyse_american_profile_at(as_of),
        Exchange::NyseNational => us::nyse_national_profile_at(as_of),
        Exchange::NyseTexas => us::nyse_texas_profile_at(as_of),
        Exchange::BlueOceanAts => us::blue_ocean_profile_at(as_of),
        Exchange::MemxEq => us::memx_profile_at(as_of),
        Exchange::MiaxPearlEq => us::miax_pearl_profile_at(as_of),
        Exchange::FinraTrfCarteret => us::finra_trf_carteret_profile_at(as_of),
        Exchange::FinraTrfChicago => us::finra_trf_chicago_profile_at(as_of),
        Exchange::FinraTrfNyse => us::finra_trf_nyse_profile_at(as_of),
        Exchange::Cme => futures_us::cme_profile_at(as_of),
        Exchange::Cfe => futures_us::cfe_profile_at(as_of),
        Exchange::Cbot => futures_us::cbot_profile_at(as_of),
        Exchange::Comex | Exchange::Nymex => futures_us::energy_metals_profile_at(as_of),
        Exchange::Iceus => futures_us::ice_us_fang_profile_at(as_of),
        Exchange::Eurex => international::eurex_profile_at(as_of),
        Exchange::Eex => international::eex_profile_at(as_of),
        Exchange::Iceeu => international::iceeu_profile_at(as_of),
        Exchange::IceEuropeCommodities => international::ice_europe_commodities_profile_at(as_of),
        Exchange::IceEuropeFinancials => international::ice_europe_financials_profile_at(as_of),
        Exchange::IceEndex => international::ice_endex_profile_at(as_of),
        Exchange::IceAbuDhabi => international::ice_abu_dhabi_profile_at(as_of),
        Exchange::IceCanada => international::ice_canada_profile_at(as_of),
        Exchange::Sgx => international::sgx_profile_at(as_of),
        Exchange::BinanceFutures => international::binance_profile_at(as_of),
        Exchange::Asx => apac::asx::profile_at(as_of),
        Exchange::TmxAustralia => apac::tmx_australia::profile_at(as_of),
        Exchange::Nzx => apac::nzx::profile_at(as_of),
        Exchange::Tse => apac::tse::profile_at(as_of),
        Exchange::NseIndia => apac::nse::profile_at(as_of),
        Exchange::BseIndia => apac::bse::profile_at(as_of),
        Exchange::Hkex => apac::hkex::profile_at(as_of),
        Exchange::SgxSecurities => apac::sgx::profile_at(as_of),
        Exchange::SetThailand => apac::set::profile_at(as_of),
        Exchange::Idx => apac::idx::profile_at(as_of),
        Exchange::Pse => apac::pse::profile_at(as_of),
        Exchange::Hose => apac::hose::profile_at(as_of),
        Exchange::Sse => apac::sse::profile_at(as_of),
        Exchange::Szse => apac::szse::profile_at(as_of),
        Exchange::Krx => apac::krx::profile_at(as_of),
        Exchange::Twse => apac::twse::profile_at(as_of),
        Exchange::Lse => europe::lse::profile_at(as_of),
        Exchange::Xetra => europe::xetra::profile_at(as_of),
        Exchange::Six => europe::six::profile_at(as_of),
        Exchange::EuronextParis => europe::euronext::paris_profile_at(as_of),
        Exchange::EuronextAmsterdam => europe::euronext::amsterdam_profile_at(as_of),
        Exchange::EuronextBrussels => europe::euronext::brussels_profile_at(as_of),
        Exchange::EuronextLisbon => europe::euronext::lisbon_profile_at(as_of),
        Exchange::EuronextDublin => europe::euronext::dublin_profile_at(as_of),
        Exchange::EuronextMilan => europe::euronext::milan_profile_at(as_of),
        Exchange::Bme => europe::bme::profile_at(as_of),
        Exchange::NasdaqStockholm => europe::nasdaq_nordics::stockholm_profile_at(as_of),
        Exchange::NasdaqHelsinki => europe::nasdaq_nordics::helsinki_profile_at(as_of),
        Exchange::NasdaqCopenhagen => europe::nasdaq_nordics::copenhagen_profile_at(as_of),
        Exchange::Vienna => europe::vienna::profile_at(as_of),
        Exchange::BorsaIstanbul => europe::bist::profile_at(as_of),
        Exchange::Jse => africa_middle_east::jse::profile_at(as_of),
        Exchange::Tadawul => africa_middle_east::tadawul::profile_at(as_of),
        Exchange::B3 => americas::b3::profile_at(as_of),
        Exchange::Bmv => americas::bmv::profile_at(as_of),
        _ => return current,
    };
    from_profile(exch, profile)
}
