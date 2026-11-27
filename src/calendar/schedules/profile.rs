// SPDX-License-Identifier: MIT-0

//! Allocation-free normal-week schedule storage.

use std::borrow::Cow;

use jiff::tz::TimeZone;

use crate::calendar::time_zones::{America, UTC};

use crate::calendar::exchange_calendar::CalendarSource;
use crate::calendar::rule::ALL_DAYS;
use crate::calendar::{MarketHours, SessionRule};

/// A normal-week profile borrowed from a built-in table or a caller's snapshot.
#[derive(Clone, Copy)]
pub(crate) struct HoursProfile<'a> {
    pub(crate) tz: &'a TimeZone,
    pub(crate) regular: &'a [SessionRule],
    pub(crate) extended: &'a [SessionRule],
    /// Order-entry-only phases in which no trade can match.
    pub(crate) order_entry: &'a [SessionRule],
    pub(crate) has_daily_close: bool,
    pub(crate) has_weekend_close: bool,
}

/// The stored profile for a built-in schedule.
pub(crate) type StaticHoursProfile = HoursProfile<'static>;

impl<'a> From<&'a MarketHours> for HoursProfile<'a> {
    fn from(hours: &'a MarketHours) -> Self {
        Self {
            tz: &hours.tz,
            regular: &hours.regular,
            extended: &hours.extended,
            order_entry: &hours.order_entry,
            has_daily_close: hours.has_daily_close,
            has_weekend_close: hours.has_weekend_close,
        }
    }
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

/// One shared static rule slice for continuous 24×7 profiles (the synthetic
/// `Exchange::Unknown` fallback and `MarketHoursKey::AlwaysOpen`), so the two
/// identities cannot drift apart.
pub(crate) static ALWAYS_OPEN_RULE: &[SessionRule] = &[SessionRule {
    days: ALL_DAYS,
    open_ssm: 0,
    close_ssm: 24 * 3600,
}];

/// The continuous 24×7 UTC profile. It has no final daily close, so
/// date-aware trade-date queries return `None` at every instant.
pub(crate) static ALWAYS_OPEN_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: UTC,
    regular: ALWAYS_OPEN_RULE,
    extended: &[],
    order_entry: &[],
    has_daily_close: false,
    has_weekend_close: false,
};

/// Tags a static schedule with its venue without cloning its rule slices.
#[inline]
pub(crate) fn from_profile(
    source: impl Into<CalendarSource>,
    profile: &'static StaticHoursProfile,
) -> MarketHours {
    MarketHours {
        source: source.into(),
        tz: profile.tz.clone(),
        regular: Cow::Borrowed(profile.regular),
        extended: Cow::Borrowed(profile.extended),
        order_entry: Cow::Borrowed(profile.order_entry),
        has_daily_close: profile.has_daily_close,
        has_weekend_close: profile.has_weekend_close,
    }
}
