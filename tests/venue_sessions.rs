// SPDX-License-Identifier: MIT-0

//! Normal-week baseline tests for futures venues: open/closed flags, session
//! bounds, weekend/holiday-window checks, and serde wire-format stability.
//!
//! Deliberately not covered here (and out of scope for the normal-week model):
//!
//!   - product-level hour variations (e.g. CME wheat vs equity index)
//!   - holiday calendars (the normal-week holiday policy returns false)
//!   - early-close / half-day schedules
//!   - DST-transition edge cases (covered separately by the bias-aware helpers)
//!   - Binance quarterly-expiry pauses

#![expect(
    clippy::expect_used,
    reason = "fixture constructors assert their own literals; a bad literal must fail the test"
)]

#[path = "venue_sessions/mod.rs"]
mod suite;
