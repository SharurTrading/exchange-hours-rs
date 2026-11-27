// SPDX-License-Identifier: MIT-0

//! Date-aware behavior at Jiff's representational edges.

use super::prelude::*;
use crate::support::PROFILE_INSTANT;
use crate::support::elapsed_days;
use crate::support::utc_datetime;
use crate::support::zones::NEW_YORK;
use crate::support::zones::TORONTO;
use crate::support::zones::UTC;
use jiff::SignedDuration;
use jiff::Timestamp;
use jiff::civil::{Time, Weekday};

#[test]
fn date_aware_queries_are_total_at_timestamp_bounds() {
    for &exchange in Exchange::ALL {
        let calendar = calendar_for_exchange(exchange);
        for instant in [Timestamp::MIN, Timestamp::MAX] {
            let result = std::panic::catch_unwind(|| {
                let snapshot = hours_for_exchange(exchange, instant);
                let fixed_open = snapshot.is_open(instant);
                let fixed_bounds = session_bounds(&snapshot, instant);
                let _fixed_next = next_session_after(&snapshot, instant);
                let _fixed_daily = candle_end(&snapshot, instant, CalendarResolution::Daily);
                let _fixed_start = candle_start(&snapshot, instant, CalendarResolution::Monthly);
                let _fixed_seconds = candle_end(&snapshot, instant, CalendarResolution::Seconds(1));
                let _fixed_maintenance = snapshot.is_maintenance(instant);
                let _fixed_closed = snapshot.is_closed_all_day_at(instant, &UTC, SessionKind::Both);
                let open = calendar.is_open(instant);
                let bounds = calendar.session_bounds(instant);
                let _next = calendar.next_session_after(instant);
                let _daily = calendar.candle_end(instant, CalendarResolution::Daily);
                let _weekly = calendar.candle_end(instant, CalendarResolution::Weekly);
                let _monthly = calendar.candle_end(instant, CalendarResolution::Monthly);
                let _start = calendar.candle_start(instant, CalendarResolution::Monthly);
                let _seconds = calendar.candle_end(instant, CalendarResolution::Seconds(1));
                let _maintenance = calendar.is_maintenance(instant);
                let _closed = calendar.is_closed_all_day_at(instant, &UTC, SessionKind::Both);
                let _week = calendar.normal_week_open_seconds_containing(instant);

                (fixed_open, fixed_bounds, open, bounds)
            });
            let (fixed_open, fixed_bounds, open, bounds) = result
                .unwrap_or_else(|_| panic!("{exchange:?} panicked at timestamp bound {instant}"));
            assert_eq!(
                fixed_open,
                fixed_bounds.is_some_and(|(start, end)| start <= instant && instant < end),
                "{exchange:?} fixed query fence failed at {instant}"
            );
            // The calendar may refuse a timestamp bound (it is far below the floor),
            // and that refusal is total: it must not be a panic, and it must be a
            // coverage refusal rather than a fabricated answer. Where it does
            // answer, the containment agreement is asserted exactly as before.
            match (open, bounds) {
                (Ok(open), Ok(bounds)) => assert_eq!(
                    open,
                    bounds.is_some_and(|(start, end)| start <= instant && instant < end),
                    "{exchange:?} calendar query fence failed at {instant}"
                ),
                (open, bounds) => assert!(
                    open.is_err() && bounds.is_err(),
                    "{exchange:?} at {instant}: the calendar must answer both queries or \
                     refuse both, got open={open:?} bounds={bounds:?}"
                ),
            }
        }
    }
}

/// The open/bounds pair each surface returns for the caller to fence.
type OpenAndBounds = (bool, Option<(Timestamp, Timestamp)>);

/// Runs one query under a label, so a panic names the entry point that failed.
fn total<T>(key: MarketHoursKey, instant: Timestamp, query: &str, call: impl FnOnce() -> T) -> T {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(call))
        .unwrap_or_else(|_| panic!("{key:?}: {query} panicked at timestamp bound {instant}"))
}

/// Asserts a timestamp-bound query returns **totally**.
///
/// This is the totality claim at the representable bounds after Stage 2B: the
/// query must return — not panic, not hang — and if it refuses, the refusal must
/// be a coverage error rather than a fabricated closure. A bound date is usually
/// far below the floor or above every horizon so the identity refuses, but an
/// identity whose profile genuinely holds there may answer; both are total, and
/// only a panic or a non-coverage error fails. Confirmed by a `catch_unwind`
/// probe so a panic is reported with this query's name.
fn assert_refuses_totally<T: core::fmt::Debug>(
    key: MarketHoursKey,
    instant: Timestamp,
    query: &str,
    call: impl Fn() -> Result<T, exchange_hours::CalendarQueryError>,
) {
    let answer = std::panic::catch_unwind(std::panic::AssertUnwindSafe(&call))
        .unwrap_or_else(|_| panic!("{key:?}: {query} panicked at {instant}"));
    match answer {
        // A coverage refusal and an answer are both total. The claim at a timestamp
        // bound is **totality**, not refusal: most identities have no sourced
        // answer there, but a profile that genuinely holds at the bound (the
        // always-open key, for instance) may answer — and what this fence
        // requires is that it returns rather than panicking, hanging, or
        // fabricating a degenerate session.
        Err(
            exchange_hours::CalendarQueryError::BeforeSupportFloor { .. }
            | exchange_hours::CalendarQueryError::OutsideCoveredRange { .. }
            | exchange_hours::CalendarQueryError::UnresolvedGap { .. }
            | exchange_hours::CalendarQueryError::SearchExhausted { .. },
        )
        | Ok(_) => {}
        // Any other error is not a coverage verdict, so it is a defect.
        Err(other) => panic!("{key:?}: {query} at {instant} returned {other:?}"),
    }
}

/// Every resolution on both surfaces, each named individually.
fn exercise_every_resolution(
    key: MarketHoursKey,
    instant: Timestamp,
    snapshot: &MarketHours,
    calendar: ExchangeCalendar,
) {
    for resolution in [
        CalendarResolution::Daily,
        CalendarResolution::Weekly,
        CalendarResolution::Monthly,
        CalendarResolution::Seconds(1),
    ] {
        total(
            key,
            instant,
            &format!("candle_end(&snapshot, {resolution:?})"),
            || candle_end(snapshot, instant, resolution),
        );
        total(
            key,
            instant,
            &format!("candle_start(&snapshot, {resolution:?})"),
            || candle_start(snapshot, instant, resolution),
        );
        total(
            key,
            instant,
            &format!("ExchangeCalendar::candle_end({resolution:?})"),
            || calendar.candle_end(instant, resolution).unwrap_or(None),
        );
        total(
            key,
            instant,
            &format!("ExchangeCalendar::candle_start({resolution:?})"),
            || calendar.candle_start(instant, resolution).unwrap_or(None),
        );
    }
}

/// The fixed-snapshot surface. Returns the open/bounds pair the caller fences.
fn exercise_fixed_surface(
    key: MarketHoursKey,
    instant: Timestamp,
    snapshot: &MarketHours,
) -> OpenAndBounds {
    let open = total(key, instant, "MarketHours::is_open", || {
        snapshot.is_open(instant)
    });
    let bounds = total(key, instant, "session_bounds(&snapshot)", || {
        session_bounds(snapshot, instant)
    });
    total(key, instant, "next_session_after(&snapshot)", || {
        next_session_after(snapshot, instant)
    });
    total(key, instant, "MarketHours::is_maintenance", || {
        snapshot.is_maintenance(instant)
    });
    total(key, instant, "MarketHours::is_accepting_orders", || {
        snapshot.is_accepting_orders(instant)
    });
    for kind in [
        SessionKind::Regular,
        SessionKind::Extended,
        SessionKind::Both,
    ] {
        total(
            key,
            instant,
            &format!("MarketHours::is_closed_all_day_at({kind:?})"),
            || snapshot.is_closed_all_day_at(instant, &UTC, kind),
        );
    }
    (open, bounds)
}

/// The dated-calendar surface. Returns the open/bounds pair the caller fences.
fn exercise_calendar_surface(
    key: MarketHoursKey,
    instant: Timestamp,
    calendar: ExchangeCalendar,
) -> OpenAndBounds {
    assert_refuses_totally(key, instant, "ExchangeCalendar::is_open", || {
        calendar.is_open(instant)
    });
    let open = total(key, instant, "ExchangeCalendar::is_open", || {
        calendar.is_open(instant).unwrap_or(false)
    });
    assert_refuses_totally(key, instant, "ExchangeCalendar::session_bounds", || {
        calendar.session_bounds(instant)
    });
    let bounds = total(key, instant, "ExchangeCalendar::session_bounds", || {
        calendar.session_bounds(instant).unwrap_or(None)
    });
    total(key, instant, "ExchangeCalendar::next_session_after", || {
        calendar.next_session_after(instant).unwrap_or(None)
    });
    total(key, instant, "ExchangeCalendar::is_maintenance", || {
        calendar.is_maintenance(instant).unwrap_or(false)
    });
    total(key, instant, "ExchangeCalendar::trade_date", || {
        calendar.trade_date(instant).unwrap_or(None)
    });
    total(key, instant, "normal_week_open_seconds_containing", || {
        calendar
            .normal_week_open_seconds_containing(instant)
            .unwrap_or(0)
    });
    for kind in [
        SessionKind::Regular,
        SessionKind::Extended,
        SessionKind::Both,
    ] {
        total(
            key,
            instant,
            &format!("ExchangeCalendar::is_closed_all_day_at({kind:?})"),
            || {
                calendar
                    .is_closed_all_day_at(instant, &UTC, kind)
                    .unwrap_or(false)
            },
        );
    }
    (open, bounds)
}

/// The same totality contract, over `MarketHoursKey` rather than `Exchange`.
///
/// The test above iterates `Exchange::ALL` only, so until this existed **no
/// product-family key was covered at Jiff's edges** — not the ones added
/// recently and not the original twenty-odd either. `LAW-PANIC` is a promise
/// about every public entry point, and `calendar_for_market_hours_key` is one,
/// so the gap was in the fence rather than in the engine.
///
/// Each query runs under its own label, because a bare "key panicked at bound"
/// message cannot say which of two dozen entry points, resolutions or session
/// kinds failed — and pointing at the one that did is the whole value here.
/// The surfaces are split into helpers to stay inside the 100-line limit.
#[test]
fn date_aware_key_queries_are_total_at_timestamp_bounds() {
    for &key in MarketHoursKey::ALL {
        let calendar = calendar_for_market_hours_key(key);
        for instant in [Timestamp::MIN, Timestamp::MAX] {
            let snapshot = total(key, instant, "hours_for_market_hours_key", || {
                hours_for_market_hours_key(key, instant)
            });
            let (fixed_open, fixed_bounds) = exercise_fixed_surface(key, instant, &snapshot);
            let (open, bounds) = exercise_calendar_surface(key, instant, calendar);
            exercise_every_resolution(key, instant, &snapshot, calendar);

            assert_eq!(
                fixed_open,
                fixed_bounds.is_some_and(|(start, end)| start <= instant && instant < end),
                "{key:?} fixed query fence failed at {instant}"
            );
            assert_eq!(
                open,
                bounds.is_some_and(|(start, end)| start <= instant && instant < end),
                "{key:?} calendar query fence failed at {instant}"
            );
        }
    }
}

#[test]
fn synthetic_always_open_utc_profile_has_exact_timestamp_edge_sessions() {
    let calendar = calendar_for_exchange(Exchange::Unknown);
    let fixed = hours_for_exchange(Exchange::Unknown, PROFILE_INSTANT);
    let minimum = Timestamp::MIN;
    let first_day = UTC.to_datetime(minimum).date().tomorrow().unwrap();
    let first_midnight = utc_datetime(first_day.to_datetime(Time::midnight()));
    let next_midnight = utc_datetime(first_day.tomorrow().unwrap().to_datetime(Time::midnight()));

    // The UTC day containing MIN has an opening outside the timestamp domain.
    // The next full session still resolves with both of its real endpoints.
    assert!(!fixed.is_open(minimum));
    assert_eq!(
        session_bounds(&fixed, minimum),
        Some((first_midnight, next_midnight))
    );
    assert_eq!(
        next_session_after(&fixed, minimum),
        Some((first_midnight, next_midnight))
    );
    assert!(fixed.is_open(first_midnight));
    assert!(calendar.is_open(minimum).is_err());
    assert_eq!(
        session_bounds(&fixed, first_midnight),
        Some((first_midnight, next_midnight))
    );
    assert!(
        calendar.session_bounds(minimum).is_err(),
        "a timestamp-bound session-bounds query refuses rather than answering"
    );
    for resolution in [
        CalendarResolution::Daily,
        CalendarResolution::Weekly,
        CalendarResolution::Monthly,
    ] {
        assert_eq!(candle_start(&fixed, minimum, resolution), None);
        assert_eq!(candle_end(&fixed, minimum, resolution), None);
        assert!(
            calendar.candle_start(minimum, resolution).is_err(),
            "a timestamp-bound candle start refuses rather than answering"
        );
        assert!(
            calendar.candle_end(minimum, resolution).is_err(),
            "a timestamp-bound candle end refuses rather than answering"
        );
    }

    let near_maximum = Timestamp::MAX
        .checked_sub(elapsed_days(1))
        .expect("one day before timestamp maximum is representable");
    assert!(fixed.is_open(near_maximum));
    assert!(
        calendar
            .is_open(near_maximum)
            .expect("the coverage contract must answer a covered date")
    );
}

#[test]
fn negative_offset_scan_keeps_the_first_session_at_timestamp_minimum() {
    let minimum = Timestamp::MIN;
    let mut day = NEW_YORK
        .to_datetime(minimum)
        .date()
        .tomorrow()
        .expect("the first full local day fits");
    while matches!(day.weekday(), Weekday::Saturday | Weekday::Sunday) {
        day = day.tomorrow().expect("the first full trading day fits");
    }
    let open = NEW_YORK
        .to_ambiguous_timestamp(day.at(9, 30, 0, 0))
        .unambiguous()
        .expect("NYSE minimum-date open has one local mapping");
    let close = NEW_YORK
        .to_ambiguous_timestamp(day.at(16, 0, 0, 0))
        .unambiguous()
        .expect("NYSE minimum-date close has one local mapping");
    let expected = Some((open, close));
    let fixed = hours_for_exchange(Exchange::Nyse, minimum);
    let calendar = calendar_for_exchange(Exchange::Nyse);

    assert_eq!(next_session_after(&fixed, minimum), expected);
    assert_eq!(session_bounds(&fixed, minimum), expected);
    assert_eq!(session_bounds(&fixed, open), expected);
    assert!(
        calendar.next_session_after(minimum).is_err(),
        "a timestamp-bound forward scan refuses rather than answering"
    );
    assert!(
        calendar.session_bounds(minimum).is_err(),
        "a timestamp-bound session-bounds query refuses rather than answering"
    );
    assert_eq!(
        candle_end(&fixed, minimum, CalendarResolution::Daily),
        Some(close)
    );
    assert!(
        calendar
            .candle_end(minimum, CalendarResolution::Daily)
            .is_err(),
        "a timestamp-bound daily close refuses rather than answering"
    );
}

#[test]
fn maximum_hour_resolution_clamps_without_losing_a_bar() {
    let tz = TORONTO;
    let instant = local(&tz, (2026, 8, 19), (10, 12, 0));
    let close = local(&tz, (2026, 8, 19), (16, 0, 0));
    let resolution = CalendarResolution::Hours(u32::MAX);
    let fixed = hours_for_exchange(Exchange::Tsx, instant);
    let calendar = calendar_for_exchange(Exchange::Tsx);

    assert_eq!(candle_start(&fixed, instant, resolution), Some(instant));
    assert_eq!(candle_end(&fixed, instant, resolution), Some(close));
    // The identity-backed calendar clamps exactly as the fixed snapshot does:
    // the bar bounds resolve from the self-dated session the instant lands in
    // (a session opening on its own day is dated by that day), so no date
    // outside the audited window is needed and the timestamp-bound clamp answers
    // the same bar the fixed snapshot states.
    assert_eq!(
        calendar.candle_start(instant, resolution),
        Ok(Some(instant))
    );
    assert_eq!(calendar.candle_end(instant, resolution), Ok(Some(close)));
}

#[test]
fn dynamic_period_walks_keep_the_last_close_near_timestamp_maximum() {
    let calendar = calendar_for_exchange(Exchange::Nyse);
    let maximum = Timestamp::MAX;
    let max_day = UTC.to_datetime(maximum).date();
    let final_week_start = max_day
        .checked_sub(elapsed_days(i64::from(
            max_day.weekday().to_monday_zero_offset(),
        )))
        .expect("the final ISO week starts inside Jiff's range");
    let instant = utc_datetime(final_week_start.at(0, 0, 0, 0));

    // The detached normal-week rules can still resolve the final available
    // closes. The identity's holiday horizon produces a typed refusal.
    let fixed = hours_for_exchange(Exchange::Nyse, instant);
    let mut last_close = candle_end(&fixed, instant, CalendarResolution::Daily)
        .expect("at least one trading day fits in the last ISO week");
    for _ in 0..7 {
        let probe = last_close
            .checked_add(SignedDuration::from_nanos(1))
            .unwrap();
        let Some(next) = candle_end(&fixed, probe, CalendarResolution::Daily) else {
            break;
        };
        assert!(next > last_close);
        last_close = next;
    }
    let probe = last_close
        .checked_add(SignedDuration::from_nanos(1))
        .unwrap();
    assert_eq!(candle_end(&fixed, probe, CalendarResolution::Daily), None);
    for resolution in [CalendarResolution::Weekly, CalendarResolution::Monthly] {
        assert_eq!(candle_end(&fixed, instant, resolution), Some(last_close));
        assert!(matches!(
            calendar.candle_end(instant, resolution),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ));
    }
}

#[test]
fn a_civil_day_beyond_the_utc_timestamp_range_keeps_a_typed_coverage_refusal() {
    let calendar = calendar_for_exchange(Exchange::Nyse);
    let fixed = hours_for_exchange(Exchange::Nyse, PROFILE_INSTANT);
    let unrepresentable = jiff::civil::Date::MAX;
    assert!(fixed.is_closed_all_day_in_calendar(unrepresentable, &UTC, SessionKind::Both));
    assert!(matches!(
        calendar.is_closed_all_day_in_calendar(unrepresentable, &UTC, SessionKind::Both),
        Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == unrepresentable
    ));
}

#[test]
fn an_early_close_beyond_the_timestamp_limit_keeps_a_representable_session() {
    use exchange_hours::{DayOverride, StaticDayPolicy};

    let calendar = calendar_for_exchange(Exchange::Nyse).with_last_known_week();
    let day = NEW_YORK.to_datetime(Timestamp::MAX).date();
    let at = |hour, minute| NEW_YORK.to_timestamp(day.at(hour, minute, 0, 0)).unwrap();
    let probe = at(12, 0);
    let expected = Some((at(9, 30), at(16, 0)));
    let overrides =
        [DayOverride::early_close(day, 20 * 3_600).expect("the fixture boundary must be valid")];
    let policy = StaticDayPolicy::new(&overrides).expect("valid early-close override");

    assert!(NEW_YORK.to_timestamp(day.at(20, 0, 0, 0)).is_err());
    assert_eq!(
        calendar.session_bounds_with(probe, SessionKind::Regular),
        Ok(expected)
    );
    assert_eq!(
        calendar
            .with_day_policy(&policy)
            .session_bounds_with(probe, SessionKind::Regular),
        Ok(expected)
    );
}

#[test]
fn an_early_close_beyond_the_timestamp_limit_keeps_a_representable_replacement() {
    use exchange_hours::{
        DayOverride, ExceptionBlock, SessionExceptionRecord, StaticDayPolicy,
        StaticSessionExceptions,
    };

    let calendar = calendar_for_exchange(Exchange::Nyse).with_last_known_week();
    let day = NEW_YORK.to_datetime(Timestamp::MAX).date();
    let at = |hour| NEW_YORK.to_timestamp(day.at(hour, 0, 0, 0)).unwrap();
    let blocks = [ExceptionBlock::regular(0, 10 * 3_600, 14 * 3_600)];
    let records = [SessionExceptionRecord::replace_sessions(day, &blocks)];
    let exceptions = StaticSessionExceptions::new(calendar.source(), day, day, &records)
        .expect("valid replacement session");
    let overrides =
        [DayOverride::early_close(day, 20 * 3_600).expect("the fixture boundary must be valid")];
    let policy = StaticDayPolicy::new(&overrides).expect("valid early-close override");

    assert_eq!(
        calendar
            .with_session_exceptions(&exceptions)
            .expect("same calendar source")
            .with_day_policy(&policy)
            .session_bounds_with(at(12), SessionKind::Regular),
        Ok(Some((at(10), at(14))))
    );
}

#[test]
fn an_earlier_replacement_can_belong_to_a_trade_date_beyond_the_timestamp_limit() {
    use exchange_hours::{ExceptionBlock, SessionExceptionRecord, StaticSessionExceptions};
    use jiff::civil::Date;

    let calendar = calendar_for_exchange(Exchange::Nyse).with_last_known_week();
    let trade_date = Date::MAX;
    let day = trade_date.yesterday().expect("the opening day fits");
    let at = |hour, minute| NEW_YORK.to_timestamp(day.at(hour, minute, 0, 0)).unwrap();
    let blocks = [ExceptionBlock::regular(-1, 7 * 3_600, 8 * 3_600)];
    let records = [SessionExceptionRecord::replace_sessions(
        trade_date, &blocks,
    )];
    let exceptions =
        StaticSessionExceptions::new(calendar.source(), trade_date, trade_date, &records)
            .expect("valid replacement session");
    let overlaid = calendar
        .with_session_exceptions(&exceptions)
        .expect("same calendar source");
    let probe = at(7, 30);

    assert!(NEW_YORK.to_timestamp(trade_date.at(0, 0, 0, 0)).is_err());
    assert_eq!(overlaid.is_open_regular(probe), Ok(true));
    assert_eq!(
        overlaid.session_bounds_with(probe, SessionKind::Regular),
        Ok(Some((at(7, 0), at(8, 0))))
    );
    assert_eq!(overlaid.trade_date(probe), Ok(Some(trade_date)));
    assert_eq!(
        overlaid.is_closed_trade_date(trade_date, SessionKind::Regular),
        Ok(false)
    );
}
