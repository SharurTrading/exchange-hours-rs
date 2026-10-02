// SPDX-License-Identifier: MIT-0

//! Shared query engine for fixed and date-aware schedules.

pub(in crate::calendar) mod candles;
pub(in crate::calendar) mod gate;
mod identity;
pub(in crate::calendar) mod periods;
mod replacement;
pub(in crate::calendar) mod schedule;
pub(in crate::calendar) mod sessions;
pub(in crate::calendar) mod status;
pub(in crate::calendar) mod week;

pub(in crate::calendar) use schedule::QueryContext;
