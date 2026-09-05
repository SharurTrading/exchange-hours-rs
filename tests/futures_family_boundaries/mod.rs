// SPDX-License-Identifier: MIT-0

//! Dated-boundary fixtures organized by venue family.

mod cme_families;
mod cme_mini_grains;
mod cme_nikkei;
mod cme_rough_rice;
mod cme_weather;
mod ice;
mod sgx_equity_index;

mod prelude {
    pub(super) use chrono::{DateTime, Duration, TimeZone as _, Utc};
    pub(super) use exchange_hours::{MarketHoursKey, SessionState, hours_for_market_hours_key};

    /// Builds a UTC probe instant from literals. UTC has no ambiguous local
    /// times, so `single()` always resolves here; the epoch fallback keeps the
    /// helper total without a panic path, and any probe that somehow reached it
    /// would fail its assertion loudly rather than pass silently.
    pub(super) fn utc(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, hour, minute, 0)
            .single()
            .unwrap_or(DateTime::UNIX_EPOCH)
    }

    /// The instant one second before `utc(..)`, for fencing an exact open.
    pub(super) fn just_before(instant: DateTime<Utc>) -> DateTime<Utc> {
        instant - Duration::seconds(1)
    }

    pub(super) fn open_at(key: MarketHoursKey, instant: DateTime<Utc>) -> bool {
        hours_for_market_hours_key(key, instant).is_open(instant)
    }

    /// Regular session only. The ICE families run order-entry phases outside the
    /// executable session, so `is_open` would answer true during a pre-open.
    pub(super) fn open_regular_at(key: MarketHoursKey, instant: DateTime<Utc>) -> bool {
        hours_for_market_hours_key(key, instant).session_state(instant) == SessionState::OpenRegular
    }
}
