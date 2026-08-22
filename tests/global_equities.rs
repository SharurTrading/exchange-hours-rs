// SPDX-License-Identifier: MIT-0

//! Baselines and dated amendments for other major global cash-equity venues.

#![expect(
    clippy::expect_used,
    reason = "fixture constructors assert literals that must fail the test if invalid"
)]

#[path = "global_equities/mod.rs"]
mod suite;
mod support;
