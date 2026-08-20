// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! The bar interval a candle-boundary query is asked about.
//!
//! Two families live in one enum because callers select between them at
//! runtime: fixed-grid intraday intervals, whose boundaries are arithmetic
//! clamped to the enclosing session, and calendar periods, whose boundaries come
//! from the venue's session closes rather than from any fixed grid. See
//! [`super::candle`] for which family each variant takes.

use serde::{Deserialize, Serialize};

/// Bar resolution / time interval for candle-boundary computations.
///
/// Intraday variants carry a positive interval count; [`candle_end`](super::candle_end)
/// steps by that interval, clamped to the enclosing session and snapped past
/// maintenance gaps. [`CalendarResolution::Daily`], [`CalendarResolution::Weekly`],
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
