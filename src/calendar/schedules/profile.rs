// SPDX-License-Identifier: MIT-0

//! Allocation-free normal-week schedule storage.

use std::borrow::Cow;

use chrono_tz::{America, Tz};

use crate::calendar::exchange_calendar::CalendarSource;
use crate::calendar::{MarketHours, SessionRule};

/// A venue's normal-week schedule in fully static form.
///
/// Rule slices stay borrowed when converted to [`MarketHours`], so resolving a
/// current or historical venue schedule never allocates.
#[derive(Clone, Copy)]
pub(crate) struct StaticHoursProfile {
    pub(crate) tz: Tz,
    pub(crate) regular: &'static [SessionRule],
    pub(crate) extended: &'static [SessionRule],
    /// Order-entry-only phases in which no trade can match.
    pub(crate) order_entry: &'static [SessionRule],
    pub(crate) has_daily_close: bool,
    pub(crate) has_weekend_close: bool,
}

pub(in crate::calendar::schedules) static CLOSED_NEW_YORK: StaticHoursProfile =
    StaticHoursProfile {
        tz: America::New_York,
        regular: &[],
        extended: &[],
        order_entry: &[],
        has_daily_close: true,
        has_weekend_close: true,
    };

/// Tags a static schedule with its venue without cloning its rule slices.
#[inline]
pub(crate) fn from_profile(
    source: impl Into<CalendarSource>,
    profile: &'static StaticHoursProfile,
) -> MarketHours {
    MarketHours {
        source: source.into(),
        tz: profile.tz,
        regular: Cow::Borrowed(profile.regular),
        extended: Cow::Borrowed(profile.extended),
        order_entry: Cow::Borrowed(profile.order_entry),
        has_daily_close: profile.has_daily_close,
        has_weekend_close: profile.has_weekend_close,
    }
}
