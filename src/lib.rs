// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! SHARUR-owned market-hours, session-boundary, and candle-boundary contracts.
//!
//! This crate models exchange trading hours at the **exchange level** and answers
//! three questions over a `chrono::DateTime<Utc>` instant: is the market open,
//! what are the bounds of the containing (or next) session, and where does a bar
//! of a given [`CalendarResolution`] start and end while respecting the
//! calendar. All inputs and outputs are UTC; an exchange's local time zone is
//! used only internally to interpret session rules and resolve DST transitions
//! deterministically.
//!
//! Key types (all re-exported from the private `calendar` module):
//! - [`Exchange`] — the venue identifier; [`hours_for_exchange`] and
//!   [`hours_for_exchange_as_of`] turn it into a [`MarketHours`] profile.
//! - [`MarketHours`] — a timezone-aware set of `regular` + `extended`
//!   [`SessionRule`]s plus the open/closed and boundary query methods.
//! - [`SessionRule`] / [`SessionKind`] — one weekday-masked open/close slice
//!   (seconds since local midnight, end-exclusive close) and the regular /
//!   extended / both selector.
//! - [`CalendarResolution`] — the bar interval consumed by [`candle_start`],
//!   [`candle_end`], and [`candle_end_with`].
//! - [`MarketHoursKey`] / [`FuturesSessionProfile`] / [`session_profile`] — the
//!   named, statically-shared futures profiles reused by several venues and by
//!   the `instrument-catalog` crate.
//!
//! Holiday and product-level calendars are intentionally out of scope; the
//! `is_holiday` hook is a stub today, so these are pure normal-week defaults.

#![forbid(unsafe_code)]

mod calendar;

pub use calendar::*;

#[cfg(test)]
#[path = "../tests/unit/market_hours.rs"]
mod tests;
