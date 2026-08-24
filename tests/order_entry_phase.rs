// SPDX-License-Identifier: MIT-0

//! Contracts for the order-entry phase.
//!
//! `extended` holds genuinely tradeable electronic and overnight sessions.
//! `order_entry` holds pre-open queues and post-close order windows in which
//! orders may be entered, amended or cancelled but **no trade can match**.
//! Conflating the two made `is_open` answer true for untradeable windows and
//! made the candle machinery emit bars for markets with no price.
//!
//! These tests pin the separation itself, so it cannot quietly erode as rules
//! are reclassified venue by venue.

use chrono::{DateTime, TimeZone as _, Utc};
use exchange_hours::{
    CalendarResolution, Exchange, MarketHoursKey, SessionState, calendar_for_exchange,
    calendar_for_market_hours_key, hours_for_exchange, hours_for_market_hours_key,
};

fn utc(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .unwrap_or(DateTime::UNIX_EPOCH)
}

/// A recent Monday-to-Sunday week, sampled every fifteen minutes.
///
/// Deliberately a CURRENT week. Sampling a historical one would compare a dated
/// profile from that era against today's fixed snapshot, which is a comparison
/// between two different schedules rather than between two views of the same
/// one - NSE India, for instance, genuinely moved its post-close window during
/// 2026, so the two legitimately disagree in June and agree in August.
fn week_samples() -> impl Iterator<Item = DateTime<Utc>> {
    (17..=23u32).flat_map(|day| {
        (0..24u32).flat_map(move |hour| {
            (0..60u32)
                .step_by(15)
                .map(move |minute| utc(2026, 8, day, hour, minute))
        })
    })
}

#[test]
fn open_and_order_entry_only_are_mutually_exclusive() {
    // A caller must be able to branch on these without ordering care.
    for exchange in Exchange::ALL {
        let hours = hours_for_exchange(*exchange);
        for instant in week_samples() {
            assert!(
                !(hours.is_open(instant) && hours.is_order_entry_only(instant)),
                "{}: reported both open and order-entry-only at {instant}",
                exchange.as_str()
            );
        }
    }
    for key in MarketHoursKey::ALL {
        let hours = hours_for_market_hours_key(*key);
        for instant in week_samples() {
            assert!(
                !(hours.is_open(instant) && hours.is_order_entry_only(instant)),
                "{}: reported both open and order-entry-only at {instant}",
                key.as_str()
            );
        }
    }
}

#[test]
fn accepting_orders_is_a_superset_of_open() {
    // Anything tradeable is necessarily order-accepting. The reverse does not
    // hold, which is the entire point of the phase.
    for key in MarketHoursKey::ALL {
        let hours = hours_for_market_hours_key(*key);
        for instant in week_samples() {
            if hours.is_open(instant) {
                assert!(
                    hours.is_accepting_orders(instant),
                    "{}: open but not accepting orders at {instant}",
                    key.as_str()
                );
            }
        }
    }
}

#[test]
fn an_order_entry_window_is_never_reported_as_an_open_session() {
    // The failure this phase exists to prevent: a window where nothing can
    // match reporting as OpenExtended, and emitting a candle for a market with
    // no price.
    for key in MarketHoursKey::ALL {
        let calendar = calendar_for_market_hours_key(*key);
        for instant in week_samples() {
            if calendar.session_state(instant) == SessionState::OrderEntry {
                assert!(
                    !calendar.is_open(instant),
                    "{}: OrderEntry state but is_open is true at {instant}",
                    key.as_str()
                );
                assert!(
                    calendar
                        .candle_start(instant, CalendarResolution::Hours(1))
                        .is_none(),
                    "{}: emitted an hourly candle during an order-entry window at \
                     {instant}; there is no price in that window",
                    key.as_str()
                );
            }
        }
    }
}

#[test]
fn the_calendar_never_accepts_orders_the_fixed_profile_rejects() {
    // Equality does NOT hold here and must not be asserted: 24 of 94 exchanges
    // legitimately diverge, because a dated timeline omits phases whose onset
    // day cannot be sourced rather than inventing a cutover. Measured on a
    // current week, every one of those divergences is in the safe direction.
    //
    // Containment is the property that matters. The calendar may report fewer
    // order-accepting minutes than the current snapshot; it must never report
    // more, or a consumer would be told it can work an order the venue would
    // reject.
    for exchange in Exchange::ALL {
        let hours = hours_for_exchange(*exchange);
        let calendar = calendar_for_exchange(*exchange);
        for instant in week_samples() {
            if calendar.hours_at(instant).is_accepting_orders(instant) {
                assert!(
                    hours.is_accepting_orders(instant),
                    "{}: the dated calendar accepts orders at {instant} but the current \
                     sourced profile does not. Divergence must only ever drop phases.",
                    exchange.as_str()
                );
            }
        }
    }
}
