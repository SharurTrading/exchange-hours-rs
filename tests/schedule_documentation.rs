// SPDX-License-Identifier: MIT-0

//! Documentation fences for the schedule-review ledger.

#![expect(
    clippy::expect_used,
    reason = "malformed repository-owned documentation must fail this contract test"
)]
#![expect(
    clippy::panic,
    reason = "a fence helper outside a #[test] body reports an unreadable ledger cell by \
              panicking, which is the only way a contract test can fail"
)]

#[path = "schedule_documentation/mod.rs"]
mod suite;
