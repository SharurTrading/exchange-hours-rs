// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! Bulk builders for callers that need many venues at once.
//!
//! Two shapes, and the difference matters: the `Vec` builders preserve the
//! caller's input order, while the map builders are backed by a [`BTreeMap`] and
//! therefore iterate in [`Exchange`] `Ord` order regardless of insertion order.
//! Anything that feeds a deterministic pipeline should take the map form.
//!
//! Every builder resolves through [`hours_for_exchange`], so these return
//! *current* hours; a point-in-time sweep must call
//! [`hours_for_exchange_as_of`](super::hours_for_exchange_as_of) per venue.

use std::collections::BTreeMap;

use super::exchange::{EU_EQUITY_EXCHANGES, US_EQUITY_EXCHANGES};
use super::{Exchange, MarketHours, hours_for_exchange};

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

/// Builds the current [`MarketHours`] for the built-in EU-equities exchange set.
#[must_use]
pub fn hours_for_eu_equities() -> Vec<MarketHours> {
    hours_for_all(EU_EQUITY_EXCHANGES)
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
#[must_use]
pub fn hours_map_eu_equities() -> BTreeMap<Exchange, MarketHours> {
    hours_map_for(EU_EQUITY_EXCHANGES)
}
