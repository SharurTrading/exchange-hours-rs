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
    CalendarQueryError, CalendarResolution, DateCoverage, DayOverride, Exchange, ExchangeCalendar,
    MarketHoursKey, SessionState, StaticDayPolicy, calendar_for_exchange,
    calendar_for_market_hours_key, hours_for_exchange, hours_for_market_hours_key,
};

/// Asserts a refusal is exactly the one `calendar` publishes for the day the
/// query named, so a query verdict and the identity's own coverage metadata can
/// never disagree.
///
/// Driving the expectation from
/// [`exchange_hours::CalendarCoverage::coverage_on`] keeps a many-date sweep
/// honest: the sweep states the verdict the shipped data declares rather than a
/// hand-copied list of dates. `SearchExhausted` is the one refusal that is not a
/// per-date verdict — a bounded walk ran out — so it is checked for the identity
/// and for the documented shape of its report (`date` is the day the walk
/// stopped on, and `bound` equals it).
fn assert_published_refusal(error: CalendarQueryError, calendar: ExchangeCalendar, label: &str) {
    assert_eq!(
        error.source(),
        calendar.source(),
        "{label}: the refusal names the wrong identity"
    );
    let date = error.date();
    let verdict = calendar.coverage().coverage_on(date);
    // `DateCoverage` is `#[non_exhaustive]`. A verdict this file does not know
    // cannot be mapped to a refusal, and comparing the error with itself would
    // fence nothing, so it fails here, loudly, before the match below.
    assert!(
        matches!(
            verdict,
            DateCoverage::Covered
                | DateCoverage::NormalWeekOnly
                | DateCoverage::BeforeSupportFloor
                | DateCoverage::OutsideCoveredRange
                | DateCoverage::UnresolvedGap
        ),
        "{label}: unrecognised coverage verdict {verdict:?} for {date}, got {error:?}"
    );
    let declared = match verdict {
        DateCoverage::OutsideCoveredRange => Some(CalendarQueryError::OutsideCoveredRange {
            source: calendar.source(),
            date,
        }),
        DateCoverage::UnresolvedGap => Some(CalendarQueryError::UnresolvedGap {
            source: calendar.source(),
            date,
        }),
        DateCoverage::BeforeSupportFloor => Some(CalendarQueryError::BeforeSupportFloor {
            source: calendar.source(),
            date,
        }),
        DateCoverage::Covered | DateCoverage::NormalWeekOnly | _ => None,
    };
    match declared {
        Some(expected) => assert_eq!(
            error, expected,
            "{label}: the query must state the coverage verdict its identity publishes"
        ),
        None => assert!(
            matches!(
                error,
                CalendarQueryError::SearchExhausted { date: stopped, bound, .. } if stopped == bound
            ),
            "{label}: only a bounded search may refuse a date the identity declares              complete, and it reports the day it stopped on as its bound, got {error:?}"
        ),
    }
}

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
        let hours = hours_for_exchange(
            *exchange,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
        );
        for instant in week_samples() {
            assert!(
                !(hours.is_open(instant) && hours.is_order_entry_only(instant)),
                "{}: reported both open and order-entry-only at {instant}",
                exchange.as_str()
            );
        }
    }
    for key in MarketHoursKey::ALL {
        let hours = hours_for_market_hours_key(
            *key,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
        );
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
    for exchange in Exchange::ALL {
        let hours = hours_for_exchange(
            *exchange,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
        );
        for instant in week_samples() {
            if hours.is_open(instant) {
                assert!(
                    hours.is_accepting_orders(instant),
                    "{}: open but not accepting orders at {instant}",
                    exchange.as_str()
                );
            }
        }
    }
    for key in MarketHoursKey::ALL {
        let hours = hours_for_market_hours_key(
            *key,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
        );
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
            let label = format!("{} at {instant}", key.as_str());
            // The split is the coverage line (LAW-COVERAGE): where the identity
            // answers the instant, the separation this fence is about is
            // claimable; where it declares no answer for a day the instant needs
            // — a scope with no holiday layer, or the withheld Sunday queue
            // before its knowledge-bound era (#79) — the query refuses, and the
            // refusal is asserted rather than read as a state.
            let state = match calendar.session_state(instant) {
                Ok(state) => state,
                Err(error) => {
                    assert_published_refusal(error, calendar, &label);
                    continue;
                }
            };
            if state == SessionState::OrderEntry {
                assert!(
                    !calendar
                        .is_open(instant)
                        .expect("a state that answered answers is_open too"),
                    "{label}: OrderEntry state but is_open is true"
                );
                // `candle_start` is forward-looking for any closed instant -
                // it reports the next bar, exactly as it does during
                // Maintenance. The phantom-bar defect was not that it returned
                // something, but that an order-entry window used to be treated
                // as a session, so a bar STARTED inside it. Assert that no bar
                // begins within the window.
                if let Some(start) = calendar
                    .candle_start(instant, CalendarResolution::Hours(1))
                    .expect("a state that answered answers the bar too")
                {
                    assert!(
                        start > instant,
                        "{label}: an hourly candle starts at or before {instant}, inside \
                         an order-entry window where no trade can print"
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

    // The audited holiday window reached 2017-04-10 on 2026-09-29 (UTC), so
    // these 2018 instants answer through the identity itself: each order-entry
    // probe is answered by the profile owning the opening day, not by the
    // instant's civil date, which is the reselection this fixture pins.
    //
    // 22:10/22:20 UTC are 16:10/16:20 CT (CST). The prior regime's queue
    // starts 16:15.
    assert_eq!(
        calendar.is_order_entry_only(utc(2018, 2, 18, 22, 10)),
        Ok(false),
        "the prior regime's queue, before its 16:15 onset"
    );
    assert_eq!(
        calendar.is_order_entry_only(utc(2018, 2, 18, 22, 20)),
        Ok(true),
        "the prior regime's queue, after its 16:15 onset"
    );

    // Sunday 2018-02-25, 22:01 UTC = 16:01 CT: the new regime already queues.
    assert_eq!(
        calendar.is_order_entry_only(utc(2018, 2, 25, 22, 1)),
        Ok(true),
        "the new regime's queue"
    );
    // Monday 2018-02-26, 14:00 UTC = 08:00 CT: the wrapped session opened
    // Sunday under the new profile and is still trading.
    assert_eq!(
        calendar.is_open(utc(2018, 2, 26, 14, 0)),
        Ok(true),
        "the wrapped Monday session"
    );
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
        calendar
            .session_state(sunday_queue)
            .expect("the coverage contract must answer a covered date"),
        SessionState::OrderEntry
    );

    let closed = calendar.with_day_policy(&policy);
    assert_eq!(
        closed
            .session_state(sunday_queue)
            .expect("the coverage contract must answer a covered date"),
        SessionState::Closed
    );
}

#[test]
fn the_sunday_evening_queue_answers_through_every_entry_point_it_describes() {
    // Issue #132: the order-entry gate used to judge a Sunday-evening queue by
    // its **opening day** — a Sunday, outside every trade-date-keyed coverage
    // range — and refuse `is_order_entry_only` and `is_accepting_orders` for
    // exactly the families whose normal week carries a Sunday queue, while
    // `is_open` answered the same instant from the tradeable scan. The
    // date-scoped phase declarations (#172) key the withholding to the dates
    // whose queue resolves, so the Sunday queue of a covered trade date
    // answers through the identity like any other sourced phase.
    //
    // The fence is the issue's own: the four families whose normal week
    // carries a Sunday order-entry rule must answer inside the queue, and the
    // two answers must agree with `is_open` at the same instant.
    // 2026-06-21 is a Sunday; 21:30Z is 16:30 CT, inside every CME Sunday
    // Pre-Open (16:00-17:00 CT); 21:50Z is a second sample past its midpoint.
    for key in [
        MarketHoursKey::GlobexEnergy,
        MarketHoursKey::GlobexEquityIndex,
        MarketHoursKey::GlobexInterestRates,
        MarketHoursKey::GlobexFx,
    ] {
        let calendar = calendar_for_market_hours_key(key);
        for (hour, minute) in [(21, 30), (21, 50)] {
            let instant = utc(2026, 6, 21, hour, minute);
            let label = format!("{key:?} {instant}");
            assert_eq!(
                calendar.is_order_entry_only(instant),
                Ok(true),
                "{label}: the Sunday queue is order-entry only"
            );
            assert_eq!(
                calendar.is_accepting_orders(instant),
                Ok(true),
                "{label}: the Sunday queue accepts orders"
            );
            assert_eq!(
                calendar.is_open(instant),
                Ok(false),
                "{label}: no trade can print in the queue"
            );
        }
    }
    // The families without a Sunday queue answer too — refusal-free, which is
    // the regression the issue records: grains and livestock answered before,
    // but only because their scans happened to find a weekday occurrence.
    for key in [MarketHoursKey::GlobexGrains, MarketHoursKey::GlobexLivestock] {
        let calendar = calendar_for_market_hours_key(key);
        let instant = utc(2026, 6, 21, 21, 30);
        assert!(
            calendar.is_order_entry_only(instant).is_ok(),
            "{key:?}: a Sunday instant outside the queue answers, it does not refuse"
        );
        assert!(
            calendar.is_accepting_orders(instant).is_ok(),
            "{key:?}: a Sunday instant outside the queue answers, it does not refuse"
        );
    }
}

#[test]
fn the_calendar_never_accepts_orders_the_fixed_profile_rejects() {
    // Equality does NOT hold here and must not be asserted: 24 of 96 exchanges
    // legitimately diverge, because a dated timeline omits phases whose onset
    // day cannot be sourced rather than inventing a cutover. Measured on a
    // current week, every one of those divergences is in the safe direction.
    //
    // Containment is the property that matters. The calendar may report fewer
    // order-accepting minutes than the current snapshot; it must never report
    // more, or a consumer would be told it can work an order the venue would
    // reject.
    for exchange in Exchange::ALL {
        let hours = hours_for_exchange(
            *exchange,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
        );
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
