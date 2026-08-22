// SPDX-License-Identifier: MIT-0

//! Session bounds: which `[open, close)` window contains an instant, and which
//! one comes next.
//!
//! Searches check the instant's venue-local opening day, then a previous-day
//! wrap that may still be open, then scan forward. The close is exclusive: it
//! is the first instant outside the session. Forward scans are bounded through
//! 14 local days ahead, so a profile with no matching rule returns `None`
//! instead of a fabricated interval.
//!
//! These public fixed-profile adapters and the date-aware calendar use the
//! same private engine, preventing their session semantics from drifting.

use chrono::{DateTime, Utc};

use super::query::{QueryContext, sessions};
use super::{MarketHours, SessionKind};

/// Returns the `[open, close)` UTC bounds of the session of `kind` containing
/// `instant`, or the next session when `instant` is closed.
///
/// A previous-day wrap is considered before the forward scan. Opens use the
/// earliest valid DST mapping and closes the latest. `None` means no matching
/// session exists in the bounded horizon.
#[must_use]
pub fn session_bounds_with(
    hours: &MarketHours,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    sessions::session_bounds_with(&QueryContext::fixed(hours), instant, kind)
}

/// Returns [`session_bounds_with`] over regular and extended sessions.
#[must_use]
pub fn session_bounds(
    hours: &MarketHours,
    instant: DateTime<Utc>,
) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    session_bounds_with(hours, instant, SessionKind::Both)
}

/// Returns the first session of `kind` opening strictly after `instant`.
///
/// The search scans through 14 venue-local days ahead, skips unavailable or
/// civil-time-collapsed occurrences, rejects wraps whose close day is
/// unavailable, and picks the earliest qualifying open on the first matching
/// day.
#[must_use]
pub fn next_session_after_with(
    hours: &MarketHours,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    sessions::next_session_after_with(&QueryContext::fixed(hours), instant, kind)
}

/// Returns [`next_session_after_with`] over regular and extended sessions.
#[must_use]
pub fn next_session_after(
    hours: &MarketHours,
    instant: DateTime<Utc>,
) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    next_session_after_with(hours, instant, SessionKind::Both)
}

/// Returns only the next regular-or-extended session open after `instant`.
///
/// This is a projection of [`next_session_after`]; `None` has the same bounded
/// no-session meaning.
#[must_use]
pub fn next_session_open_after(
    hours: &MarketHours,
    instant: DateTime<Utc>,
) -> Option<DateTime<Utc>> {
    next_session_after(hours, instant).map(|(open, _close)| open)
}
