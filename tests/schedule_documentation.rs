// SPDX-License-Identifier: MIT-0

//! Documentation fences for the schedule-review ledger.

#![expect(
    clippy::expect_used,
    reason = "malformed repository-owned documentation must fail this contract test"
)]

#[path = "schedule_documentation/mod.rs"]
mod suite;
