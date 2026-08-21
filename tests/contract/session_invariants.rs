// SPDX-License-Identifier: MIT-0

//! Deterministic property + workload coverage for the `exchange-hours` public
//! schedule contract.
//!
//! `exchange-hours` is a pure, stateless schedule-arithmetic library, so the
//! correct validation class is property/deterministic-fixture rather than a
//! stateful workload. These tests assert structural invariants that must hold
//! for every venue profile and every instant, using a self-contained
//! deterministic PRNG over a fixed seed set so any failure is exactly
//! reproducible.
//!
//! # Invariants proved
//!
//! - **Totality + cross-query consistency.** Every public query (`is_open`, `is_open_with`,
//!   `is_maintenance`, `session_bounds`, `next_session_after`,
//!   `next_session_open_after`, `candle_end`, `candle_end_with`,
//!   `is_closed_all_day_on`) is panic-free and agrees with the public semantic
//!   relationships below.
//! - **`is_open` equals `is_open_with(Both)`** for every instant.
//! - **Maintenance implies closed:** `is_maintenance(t)` is never true while
//!   `is_open(t)` is true.
//! - **Session bounds are ordered:** `session_bounds` and `next_session_after`
//!   never return a close before its open, and the next session never opens in
//!   the past.
//! - **Candle ends never precede the bar start;** `Seconds(s)` is a pure
//!   `t + s` offset.
//! - **Strictly-advancing session walk (deterministic workload).** Repeatedly
//!   advancing by `next_session_after(..).open` yields a strictly increasing,
//!   progress-making sequence of opens — the function never stalls or moves
//!   backward and sessions do not overlap in open order.
//! - **Always-open venues never close** and are never in maintenance.
//! - **DST stability (pinned fixture):** queries are total and ordered across
//!   spring-forward and fall-back transition instants.
//!
//! # Reproducibility
//!
//! Failures print the seed, iteration/step index, the enabled venue/resolution
//! operation space, the venue, and the offending UTC instant
//! (`TEST-DETERMINISM-01`).

#![expect(
    clippy::expect_used,
    reason = "fixture constructors assert their own literals; a bad literal must fail the test"
)]

#[path = "session_invariants/mod.rs"]
mod suite;
