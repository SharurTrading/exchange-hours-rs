// SPDX-License-Identifier: MIT-0

//! Current-session and point-in-time amendment tests for APAC cash equities.

#![expect(
    clippy::expect_used,
    reason = "fixture constructors assert literals that must fail the test if invalid"
)]

#[path = "apac_equities/mod.rs"]
mod suite;
mod support;
