// SPDX-License-Identifier: MIT-0

//! Bar boundaries that respect the trading calendar.
//!
//! Minute/hour bars step from `max(instant, session_open)` and clamp to the
//! enclosing half-open session close, so they never claim closed time. A
//! day's last bar therefore ends at the close itself, not at a later reopen.
//! [`CalendarResolution::Seconds`] is the
//! one unclamped variant: it is a pure checked offset.
//!
//! Daily, weekly, and monthly bars derive their boundaries from session
//! closes. Their starts can consequently fall on the preceding civil day,
//! month, or year; for example, a CME trading day closing Monday can open
//! Sunday evening. Probe an end-exclusive provider close marker at
//! `close - 1ns`, not at `close`.
//!
//! Every boundary is optional. `None` means no advancing bar exists: the
//! selected session kind has no rule in the bounded horizon, the interval is
//! zero, or Chrono has no later representable instant. The same private engine
//! serves fixed [`MarketHours`] values and date-aware
//! [`ExchangeCalendar`](crate::ExchangeCalendar) values.

use chrono::{DateTime, Utc};

use super::query::{QueryContext, candles};
use super::{CalendarResolution, MarketHours, SessionKind};

/// Returns the session-aware bar close after `instant` for `resolution` and
/// `kind`, or `None` when no advancing bar exists.
///
/// - `Seconds`: a pure checked `instant + interval` offset.
/// - `Minutes`/`Hours`: starts at `max(instant, session_open)` and clamps to
///   the enclosing session close; a closed instant anchors at the next open.
/// - `Daily`: the next venue-local trading-day close when the profile has one.
/// - `Weekly`: the last trading-day close in the target ISO week when the
///   profile has a true weekend close.
/// - `Monthly`: the last close in the target venue-local calendar month when
///   the profile has daily closes.
///
/// A returned close is strictly after `instant`. Zero intraday intervals, a
/// sessionless profile, or a calendar resolution whose corresponding
/// `has_daily_close` / `has_weekend_close` flag is false returns `None`.
#[must_use]
pub fn candle_end_with(
    hours: &MarketHours,
    instant: DateTime<Utc>,
    resolution: CalendarResolution,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    candles::candle_end_with(&QueryContext::fixed(hours), instant, resolution, kind)
}

/// Returns [`candle_end_with`] over regular and extended sessions.
#[must_use]
pub fn candle_end(
    hours: &MarketHours,
    instant: DateTime<Utc>,
    resolution: CalendarResolution,
) -> Option<DateTime<Utc>> {
    candle_end_with(hours, instant, resolution, SessionKind::Both)
}

/// Returns the bar open paired with [`candle_end_with`] for `kind`.
///
/// Seconds bars start at `instant`; minute/hour bars start at
/// `max(instant, session_open)`; calendar bars start at the first session open
/// in their daily, ISO-weekly, or venue-local monthly period. `None` exactly
/// mirrors the paired end query.
#[must_use]
pub fn candle_start_with(
    hours: &MarketHours,
    instant: DateTime<Utc>,
    resolution: CalendarResolution,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    candles::candle_start_with(&QueryContext::fixed(hours), instant, resolution, kind)
}

/// Returns [`candle_start_with`] over regular and extended sessions.
///
/// Calendar starts may precede the civil period named by their close because
/// overnight sessions belong to their opening day.
#[must_use]
pub fn candle_start(
    hours: &MarketHours,
    instant: DateTime<Utc>,
    resolution: CalendarResolution,
) -> Option<DateTime<Utc>> {
    candle_start_with(hours, instant, resolution, SessionKind::Both)
}

/// Returns the next trading-day close after `instant`.
///
/// This is an alias for [`candle_end`] with
/// [`CalendarResolution::Daily`].
#[must_use]
pub fn time_end_of_day(hours: &MarketHours, instant: DateTime<Utc>) -> Option<DateTime<Utc>> {
    candle_end(hours, instant, CalendarResolution::Daily)
}
