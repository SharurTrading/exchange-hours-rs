// SPDX-License-Identifier: MIT-0

//! Shared imports and fixture helpers for this suite.

use crate::support::local_opt;
use crate::support::zones::UTC;
pub(super) use exchange_hours::{MarketHoursKey, SessionState, hours_for_market_hours_key};
pub(super) use jiff::{SignedDuration, Timestamp};

/// Builds a UTC probe instant from literals. UTC has no ambiguous local
/// times, so `single()` always resolves here; the epoch fallback keeps the
/// helper total without a panic path, and any probe that somehow reached it
/// would fail its assertion loudly rather than pass silently.
pub(super) fn utc(year: i16, month: i8, day: i8, hour: u32, minute: u32) -> Timestamp {
    local_opt(&UTC, (year, month, day), (hour, minute, 0)).unwrap_or(Timestamp::UNIX_EPOCH)
}

/// The instant one second before `utc(..)`, for fencing an exact open.
pub(super) fn just_before(instant: Timestamp) -> Timestamp {
    instant - SignedDuration::from_secs(1)
}

pub(super) fn open_at(key: MarketHoursKey, instant: Timestamp) -> bool {
    hours_for_market_hours_key(key, instant).is_open(instant)
}

/// Regular session only. The ICE families run order-entry phases outside the
/// executable session, so `is_open` would answer true during a pre-open.
pub(super) fn open_regular_at(key: MarketHoursKey, instant: Timestamp) -> bool {
    hours_for_market_hours_key(key, instant).session_state(instant) == SessionState::OpenRegular
}
