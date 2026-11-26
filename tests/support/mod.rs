// SPDX-License-Identifier: MIT-0

//! Public-surface-only fixtures shared by integration-test targets.

#[expect(
    dead_code,
    reason = "each integration target uses only the fixture constructors it needs"
)]
mod local;
#[expect(
    dead_code,
    reason = "each integration target uses only the IANA zones it exercises"
)]
pub(crate) mod zones;

#[expect(
    unused_imports,
    reason = "each integration target imports only the fixture constructors it needs"
)]
pub(crate) use local::{
    PROFILE_INSTANT, date_opt, elapsed_days, local, local_datetime, local_earliest, local_opt,
    utc_datetime,
};
