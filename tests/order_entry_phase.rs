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

use chrono::{DateTime, NaiveDate, TimeZone as _, Utc};
use exchange_hours::{
    CalendarResolution, DayOverride, Exchange, MarketHoursKey, SessionState, StaticDayPolicy,
    calendar_for_exchange, calendar_for_market_hours_key, hours_for_exchange,
    hours_for_market_hours_key,
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
                // `candle_start` is forward-looking for any closed instant -
                // it reports the next bar, exactly as it does during
                // Maintenance. The phantom-bar defect was not that it returned
                // something, but that an order-entry window used to be treated
                // as a session, so a bar STARTED inside it. Assert that no bar
                // begins within the window.
                if let Some(start) = calendar.candle_start(instant, CalendarResolution::Hours(1)) {
                    assert!(
                        start > instant,
                        "{}: an hourly candle starts at or before {instant}, inside an \
                         order-entry window where no trade can print",
                        key.as_str()
                    );
                }
            }
        }
    }
}

#[test]
fn order_entry_queries_reselect_on_the_session_opening_day_across_a_revision() {
    // CFE's system migration completed Sunday 2018-02-25 for business date
    // Monday 2018-02-26. The Sunday pre-open queue moves from a 16:15 start to
    // the 16:00:03 conservative edge, and the wrapped Sunday 17:00 session it
    // feeds belongs to Monday's trade date. Order-entry queries must be
    // answered by the profile owning the opening day — Sunday — through the
    // same candidate-day reselection the open queries use, never by whichever
    // profile the instant's own civil date selects.
    let calendar = calendar_for_market_hours_key(MarketHoursKey::CfeVix);

    // 22:10/22:20 UTC are 16:10/16:20 CT (CST). The prior regime's queue
    // starts 16:15.
    assert!(!calendar.is_order_entry_only(utc(2018, 2, 18, 22, 10)));
    assert!(calendar.is_order_entry_only(utc(2018, 2, 18, 22, 20)));

    // Sunday 2018-02-25, 22:01 UTC = 16:01 CT: the new regime already queues.
    assert!(calendar.is_order_entry_only(utc(2018, 2, 25, 22, 1)));
    // Monday 2018-02-26, 14:00 UTC = 08:00 CT: the wrapped session opened
    // Sunday under the new profile and is still trading.
    assert!(calendar.is_open(utc(2018, 2, 26, 14, 0)));
}

#[test]
fn a_closed_trade_date_removes_the_queue_that_feeds_it() {
    // The Sunday 16:00-17:00 queue feeds Monday's trade date through the
    // wrapped Sunday session. A caller's policy closing that trade date must
    // remove the complete trading day - including the queue - exactly as it
    // removes the tradeable session itself, instead of reporting an
    // order-entry window for a day that will not trade.
    const MONDAY: NaiveDate =
        NaiveDate::from_ymd_opt(2026, 8, 24).expect("fixture must be a valid calendar date");
    static OVERRIDES: [DayOverride; 1] = [DayOverride::closed(MONDAY)];
    let policy =
        StaticDayPolicy::new(&OVERRIDES).expect("a single closed-date record must be valid");

    let calendar = calendar_for_market_hours_key(MarketHoursKey::CfeVix);
    // Sunday 2026-08-23, 21:30 UTC = 16:30 CT (CDT): inside the queue.
    let sunday_queue = utc(2026, 8, 23, 21, 30);
    assert_eq!(
        calendar.session_state(sunday_queue),
        SessionState::OrderEntry
    );

    let closed = calendar.with_day_policy(&policy);
    assert_eq!(closed.session_state(sunday_queue), SessionState::Closed);
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
