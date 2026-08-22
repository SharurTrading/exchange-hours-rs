// SPDX-License-Identifier: MIT-0

//! Venue-owned normal-week schedules and their point-in-time revisions.
//!
//! A leaf module owns the literal session tables, adjacent primary-source
//! citations, and historical selector for one venue or a genuinely coupled
//! operator family. The central presets route exchanges only; they do not own
//! schedule data.

pub(crate) mod equities;
pub(crate) mod futures;
mod profile;
pub(crate) mod timeline;

pub use equities::us::{
    BLUE_OCEAN_EXTENDED, NYSE_TEXAS_EXTENDED, US_EQUITY_EXTENDED, US_EQUITY_REGULAR,
};
pub(in crate::calendar::schedules) use profile::CLOSED_NEW_YORK;
pub(crate) use profile::{StaticHoursProfile, from_profile};
