// SPDX-License-Identifier: MIT-0

//! Shared query engine for fixed and date-aware schedules.

pub(in crate::calendar) mod candles;
pub(in crate::calendar) mod periods;
mod schedule;
pub(in crate::calendar) mod sessions;
pub(in crate::calendar) mod status;
pub(in crate::calendar) mod week;

pub(in crate::calendar) use schedule::QueryContext;
