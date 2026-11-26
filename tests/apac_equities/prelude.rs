// SPDX-License-Identifier: MIT-0

//! Shared imports and fixture helpers for this suite.

use crate::support::PROFILE_INSTANT;
pub(super) use exchange_hours::{
    CalendarQueryError, CalendarResolution, CalendarSource, Exchange, MarketHours, SessionKind,
    SessionState, calendar_for_exchange, candle_end, candle_start, hours_for_apac_equities,
    hours_for_exchange, hours_map_apac_equities,
};
pub(super) use jiff::SignedDuration;
pub(super) use jiff::tz::TimeZone;

pub(super) use crate::support::local;

pub(super) fn cutover_sides(
    exchange: Exchange,
    tz: &TimeZone,
    date: (i16, i8, i8),
) -> (MarketHours, MarketHours) {
    let midnight = local(tz, date, (0, 0, 0));
    (
        hours_for_exchange(exchange, midnight - SignedDuration::from_nanos(1)),
        hours_for_exchange(exchange, midnight),
    )
}

pub(super) fn assert_weekend_closed(exchange: Exchange, tz: &TimeZone) {
    let saturday = local(tz, (2026, 8, 22), (11, 0, 0));
    assert!(!hours_for_exchange(exchange, PROFILE_INSTANT).is_open(saturday));
}
