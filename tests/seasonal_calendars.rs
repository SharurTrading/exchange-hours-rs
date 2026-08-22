// SPDX-License-Identifier: MIT-0

//! Date-aware calendar coverage, including the B3/BMV reference-zone grids.

#![expect(
    clippy::expect_used,
    reason = "fixture constructors assert literals that must fail the test if invalid"
)]

#[path = "seasonal_calendars/mod.rs"]
mod suite;
mod support;
