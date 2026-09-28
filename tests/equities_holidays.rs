// SPDX-License-Identifier: MIT-0

//! Built-in holiday tables for the three served cash-equity venues the
//! consumer routes by venue: `hkex`, `xetra` and `six`.

#![expect(
    clippy::expect_used,
    reason = "fixture constructors assert literals that must fail the test if invalid"
)]
#![expect(
    clippy::panic,
    reason = "a fence helper outside a #[test] body reports an unfenced holiday row by \
              panicking, which is the only way a contract test can fail"
)]

#[path = "equities_holidays/mod.rs"]
mod suite;
mod support;
