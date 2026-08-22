// SPDX-License-Identifier: MIT-0

//! The bar interval a candle-boundary query is asked about.
//!
//! Two families live in one enum because callers select between them at
//! runtime: arithmetic intraday intervals and calendar periods, whose boundaries
//! come from the venue's session closes rather than from any fixed grid.
//! Minute/hour bars are clamped to the enclosing session; second bars are a
//! pure checked offset. See [`super::candle`] for the exact behavior.

use serde::{Deserialize, Serialize};

/// Bar resolution / time interval for candle-boundary computations.
///
/// Intraday variants carry a positive interval count. Minute/hour bars returned
/// by [`candle_end`](super::candle_end) are clamped to the enclosing session;
/// second bars are a pure checked offset and do not consult market hours.
/// [`CalendarResolution::Daily`], [`CalendarResolution::Weekly`],
/// and [`CalendarResolution::Monthly`] resolve to canonical session-period
/// boundaries instead of a fixed grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CalendarResolution {
    /// Fixed-grid bar spanning the given number of seconds.
    Seconds(u32),
    /// Fixed-grid bar spanning the given number of minutes.
    Minutes(u32),
    /// Fixed-grid bar spanning the given number of hours.
    Hours(u32),
    /// One bar per trading day, ending at the day's session close.
    Daily,
    /// One bar per ISO week, ending at the last daily close in that week.
    Weekly,
    /// One bar per calendar month, ending at the last daily close in that month.
    Monthly,
}
