// SPDX-License-Identifier: MIT-0

//! Dated-boundary contracts for the product-family keys added in 1.0.
//!
//! Each assertion probes an instant that falls on one side of a sourced cutover
//! and would flip if the revision were mis-keyed or the profile mis-encoded. A
//! 15-minute encoding slip is otherwise invisible to the rest of the suite.
//!
//! This root is a harness; the fixtures live in `futures_family_boundaries/`,
//! split by venue family.

#![expect(
    clippy::expect_used,
    reason = "fixture constructors assert their own literals; a bad literal must fail the test"
)]

#[path = "futures_family_boundaries/mod.rs"]
mod suite;
