// SPDX-License-Identifier: MIT-0

//! Venue → hours. The one entry point that turns an [`Exchange`](super::Exchange)
//! into a [`MarketHours`](super::MarketHours).
//!
//! There is exactly one selection path and it always carries the caller's
//! instant: [`historical::hours_for_exchange`] routes every venue through its
//! owned revision timeline. A backtest and a live query therefore run
//! identical code, and a pre-coded future revision rolls over with no release
//! in between.

mod historical;

pub use historical::hours_for_exchange;
