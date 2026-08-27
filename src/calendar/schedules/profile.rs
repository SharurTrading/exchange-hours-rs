// SPDX-License-Identifier: MIT-0

//! Allocation-free normal-week schedule storage.

use std::borrow::Cow;

use chrono_tz::{America, Tz, UTC};

use crate::calendar::exchange_calendar::CalendarSource;
use crate::calendar::rule::ALL_DAYS;
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
        tz: profile.tz,
        regular: Cow::Borrowed(profile.regular),
        extended: Cow::Borrowed(profile.extended),
        order_entry: Cow::Borrowed(profile.order_entry),
        has_daily_close: profile.has_daily_close,
        has_weekend_close: profile.has_weekend_close,
    }
}
