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
    CalendarQueryError, CalendarResolution, CalendarSource, DateCoverage, DayOverride, Exchange,
    ExchangeCalendar, MarketHoursKey, SessionState, StaticDayPolicy, calendar_for_exchange,
    calendar_for_market_hours_key, hours_for_exchange, hours_for_market_hours_key,
};

/// Asserts an identity-backed query returns exactly `expected`, the coverage
/// error the shipped data declares. The variant, identity and date are all part
/// of the contract: a refusal that named the wrong day would be as wrong as an
/// answer.
fn assert_refusal<T>(
    answer: Result<T, CalendarQueryError>,
    expected: CalendarQueryError,
    label: &str,
) {
    assert_eq!(
        answer.err(),
        Some(expected),
        "{label}: the query must state the refusal its identity declares"
    );
}

/// Asserts a query refuses `date` because the venue-local day precedes the
/// permanent 2025 support floor (LAW-COVERAGE).
fn assert_before_floor<T: std::fmt::Debug>(
    answer: Result<T, CalendarQueryError>,
    source: CalendarSource,
    date: NaiveDate,
    label: &str,
) {
    assert_refusal(
        answer,
        CalendarQueryError::BeforeSupportFloor { source, date },
        label,
    );
}

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

    // Every probe here is a venue-local 2018 date, before the permanent 2025
    // floor, so the identity refuses each of them: which profile owns an opening
    // day is no longer observable across this revision, and a refusal is not an
    // order-entry answer (LAW-COVERAGE). The refusal names the instant's own
    // local day, which is the date the query could not source.
    let cfe = CalendarSource::MarketHoursKey(MarketHoursKey::CfeVix);

    // 22:10/22:20 UTC are 16:10/16:20 CT (CST). The prior regime's queue
    // starts 16:15.
    assert_before_floor(
        calendar.is_order_entry_only(utc(2018, 2, 18, 22, 10)),
        cfe,
        NaiveDate::from_ymd_opt(2018, 2, 18).expect("fixture date"),
        "the prior regime's queue, before its 16:15 onset",
    );
    assert_before_floor(
        calendar.is_order_entry_only(utc(2018, 2, 18, 22, 20)),
        cfe,
        NaiveDate::from_ymd_opt(2018, 2, 18).expect("fixture date"),
        "the prior regime's queue, after its 16:15 onset",
    );

    // Sunday 2018-02-25, 22:01 UTC = 16:01 CT: the new regime already queues.
    assert_before_floor(
        calendar.is_order_entry_only(utc(2018, 2, 25, 22, 1)),
        cfe,
        NaiveDate::from_ymd_opt(2018, 2, 25).expect("fixture date"),
        "the new regime's queue",
    );
    // Monday 2018-02-26, 14:00 UTC = 08:00 CT: the wrapped session opened
    // Sunday under the new profile and is still trading.
    assert_before_floor(
        calendar.is_open(utc(2018, 2, 26, 14, 0)),
        cfe,
        NaiveDate::from_ymd_opt(2018, 2, 26).expect("fixture date"),
        "the wrapped Monday session",
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
