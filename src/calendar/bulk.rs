// SPDX-License-Identifier: MIT-0

//! Bulk builders for callers that need many venues at once.
//!
//! Two shapes, and the difference matters: the `Vec` builders preserve the
//! caller's input order, while the map builders are backed by a [`BTreeMap`] and
//! therefore iterate in [`Exchange`] `Ord` order regardless of insertion order.
//! Anything that feeds a deterministic pipeline should take the map form.
//!
//! Every builder resolves through [`hours_for_exchange`], so these return its
//! fixed default snapshots. A point-in-time sweep must call
//! [`hours_for_exchange_as_of`](super::hours_for_exchange_as_of) per venue.
//! Scans that can cross a recurring selector transition—B3, BMV, Vienna,
//! Eurex, ICE Endex, or ICE Abu Dhabi—should use
//! [`calendar_for_exchange`](super::calendar_for_exchange).

use std::collections::BTreeMap;

use super::{Exchange, MarketHours, hours_for_exchange};

/// The lit US equities venues plus the equity ATSes with modeled hours —
/// the built-in input for the `*_us_equities` builders below. The FINRA TRFs
/// are reporting facilities, not matching venues, so they are not in the set.
const US_EQUITY_EXCHANGES: &[Exchange] = &[
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
    Exchange::Ltse,
    Exchange::TwentyFourX,
    Exchange::Txse,
    Exchange::BlueOceanAts,
];

/// The EU equities venues — the built-in input for the `*_eu_equities`
/// builders below.
const EU_EQUITY_EXCHANGES: &[Exchange] = &[
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

/// Asia-Pacific cash-equity venues in stable geographic/operator order.
const APAC_EQUITY_EXCHANGES: &[Exchange] = &[
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
];

/// Major standalone cash-equity venues outside the US/EU/APAC sets.
const GLOBAL_EQUITY_EXCHANGES: &[Exchange] = &[
    Exchange::BorsaIstanbul,
    Exchange::Tsx,
    Exchange::Jse,
    Exchange::Tadawul,
    Exchange::B3,
    Exchange::Bmv,
];

/// Builds the current [`MarketHours`] for each exchange, preserving input order.
#[must_use]
pub fn hours_for_all(exchanges: &[Exchange]) -> Vec<MarketHours> {
    exchanges.iter().map(|&e| hours_for_exchange(e)).collect()
}

/// Builds the current [`MarketHours`] for the built-in US-equities exchange set.
#[must_use]
pub fn hours_for_us_equities() -> Vec<MarketHours> {
    hours_for_all(US_EQUITY_EXCHANGES)
}

/// Builds default fixed [`MarketHours`] snapshots for the built-in EU-equities
/// set.
///
/// Vienna's entry is its ordinary non-settlement compatibility snapshot. Use
/// [`calendar_for_exchange`](super::calendar_for_exchange) for its recurring
/// third-Friday settlement grid.
#[must_use]
pub fn hours_for_eu_equities() -> Vec<MarketHours> {
    hours_for_all(EU_EQUITY_EXCHANGES)
}

/// Builds current [`MarketHours`] for the built-in Asia-Pacific equities set.
#[must_use]
pub fn hours_for_apac_equities() -> Vec<MarketHours> {
    hours_for_all(APAC_EQUITY_EXCHANGES)
}

/// Builds default fixed [`MarketHours`] snapshots for major-global equities.
///
/// B3 and BMV are date-dependent; use
/// [`calendar_for_exchange`](super::calendar_for_exchange) when scanning them
/// across dates.
#[must_use]
pub fn hours_for_global_equities() -> Vec<MarketHours> {
    hours_for_all(GLOBAL_EQUITY_EXCHANGES)
}

/// Builds a deterministic [`Exchange`]-keyed map of current [`MarketHours`].
///
/// Backed by a [`BTreeMap`] so iteration order is the [`Exchange`] `Ord` order,
/// not insertion order.
#[must_use]
pub fn hours_map_for(exchanges: &[Exchange]) -> BTreeMap<Exchange, MarketHours> {
    let mut m = BTreeMap::new();
    for &e in exchanges {
        m.insert(e, hours_for_exchange(e));
    }
    m
}

/// Builds an [`Exchange`]-keyed [`BTreeMap`] for the built-in US-equities set.
#[must_use]
pub fn hours_map_us_equities() -> BTreeMap<Exchange, MarketHours> {
    hours_map_for(US_EQUITY_EXCHANGES)
}

/// Builds an [`Exchange`]-keyed [`BTreeMap`] for the built-in EU-equities set.
///
/// Vienna's entry has the same recurring-grid caveat as
/// [`hours_for_eu_equities`].
#[must_use]
pub fn hours_map_eu_equities() -> BTreeMap<Exchange, MarketHours> {
    hours_map_for(EU_EQUITY_EXCHANGES)
}

/// Builds an [`Exchange`]-keyed map for the Asia-Pacific equities set.
#[must_use]
pub fn hours_map_apac_equities() -> BTreeMap<Exchange, MarketHours> {
    hours_map_for(APAC_EQUITY_EXCHANGES)
}

/// Builds a fixed-snapshot map for the other major-global equities set.
///
/// B3/BMV entries have the same date-dependent caveat as
/// [`hours_for_global_equities`].
#[must_use]
pub fn hours_map_global_equities() -> BTreeMap<Exchange, MarketHours> {
    hours_map_for(GLOBAL_EQUITY_EXCHANGES)
}
