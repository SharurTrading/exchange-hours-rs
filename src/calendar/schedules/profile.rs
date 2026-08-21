// SPDX-License-Identifier: MIT-0

//! Allocation-free normal-week schedule storage.

use std::borrow::Cow;

use chrono_tz::Tz;

use crate::calendar::{Exchange, MarketHours, SessionRule};

/// A venue's normal-week schedule in fully static form.
///
/// Rule slices stay borrowed when converted to [`MarketHours`], so resolving a
/// current or historical venue schedule never allocates.
#[derive(Clone, Copy)]
pub(crate) struct StaticHoursProfile {
    pub(crate) tz: Tz,
    pub(crate) regular: &'static [SessionRule],
    pub(crate) extended: &'static [SessionRule],
    pub(crate) has_daily_close: bool,
    pub(crate) has_weekend_close: bool,
}

/// Tags a static schedule with its venue without cloning its rule slices.
#[inline]
pub(crate) fn from_profile(
    exchange: Exchange,
    profile: &'static StaticHoursProfile,
) -> MarketHours {
    MarketHours {
        exchange,
        tz: profile.tz,
        regular: Cow::Borrowed(profile.regular),
        extended: Cow::Borrowed(profile.extended),
        has_daily_close: profile.has_daily_close,
        has_weekend_close: profile.has_weekend_close,
    }
}
