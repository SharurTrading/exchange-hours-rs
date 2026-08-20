// SPDX-License-Identifier: MIT-0

//! Venue → hours. The two entry points that turn an [`Exchange`](super::Exchange)
//! into a [`MarketHours`](super::MarketHours).
//!
//! Split by *when* rather than by venue, because the two functions answer
//! different questions: [`current`] gives today's published hours for every
//! venue, [`historical`] gives the hours that were in effect at an instant.
//! Backtests need the second one — a 2015 CME session queried with 2026 hours
//! silently mislabels bars.

mod current;
mod historical;

pub use current::hours_for_exchange;
pub use historical::hours_for_exchange_as_of;
