// SPDX-License-Identifier: MIT-0

//! Witnessed TRY sessions within OANDA's published Forex holiday scope.

mod support;

use crate::support::date_opt;
use exchange_hours::{ExchangeCalendar, MarketHoursKey, SessionKind, SessionState};
use jiff::{SignedDuration, Timestamp};

fn utc(value: &str) -> Timestamp {
    value.parse().expect("valid UTC fixture")
}

#[test]
fn current_try_queries_answer_the_daily_grid_across_the_european_clock_change() {
    let calendar = ExchangeCalendar::for_market_hours_key(MarketHoursKey::OandaTryFx);
    for (day, open, close) in [
        (12, "2026-10-12T06:00:00Z", "2026-10-12T15:00:00Z"),
        (23, "2026-10-23T06:00:00Z", "2026-10-23T15:00:00Z"),
        (26, "2026-10-26T07:00:00Z", "2026-10-26T16:00:00Z"),
        (30, "2026-10-30T07:00:00Z", "2026-10-30T16:00:00Z"),
    ] {
        let date = date_opt(2026, 10, day).expect("valid date");
        assert_eq!(calendar.is_open(utc(open)), Ok(true));
        assert_eq!(
            calendar.session_bounds(utc(open)),
            Ok(Some((utc(open), utc(close))))
        );
        assert_eq!(calendar.trade_date(utc(open)), Ok(Some(date)));
        assert_eq!(
            calendar.is_closed_trade_date(date, SessionKind::Both),
            Ok(false)
        );
        assert_eq!(
            calendar.is_open(utc(close) - SignedDuration::from_secs(1)),
            Ok(true)
        );
        if day == 30 {
            // Classifying the final gap consults the unpublished next date.
            assert!(calendar.session_state(utc(close)).is_err());
        } else {
            assert_eq!(calendar.session_state(utc(close)), Ok(SessionState::Closed));
            assert_eq!(calendar.is_maintenance(utc(close)), Ok(false));
        }
    }
}

#[test]
fn current_try_next_open_answers_without_turning_its_overnight_gap_into_maintenance() {
    let calendar = ExchangeCalendar::for_market_hours_key(MarketHoursKey::OandaTryFx);
    let before_open = utc("2026-10-12T00:00:00Z");
    assert_eq!(calendar.is_open(before_open), Ok(false));
    assert_eq!(
        calendar.next_session_open_after(before_open),
        Ok(Some(utc("2026-10-12T06:00:00Z")))
    );
    let overnight = utc("2026-10-12T18:00:00Z");
    assert_eq!(calendar.is_open(overnight), Ok(false));
    assert_eq!(calendar.session_state(overnight), Ok(SessionState::Closed));
    assert_eq!(
        calendar.next_session_open_after(overnight),
        Ok(Some(utc("2026-10-13T06:00:00Z")))
    );
}

#[test]
fn try_coverage_refuses_dates_before_the_witness_and_after_publication() {
    let calendar = ExchangeCalendar::for_market_hours_key(MarketHoursKey::OandaTryFx);
    for date in ["2025-11-14T12:00:00Z", "2026-11-02T12:00:00Z"] {
        assert!(calendar.is_open(utc(date)).is_err());
    }
    assert!(
        calendar
            .next_session_open_after(utc("2026-10-30T17:00:00Z"))
            .is_err()
    );
}

#[test]
fn archived_try_grid_answers_from_the_first_witnessed_opening_day() {
    let calendar = ExchangeCalendar::for_market_hours_key(MarketHoursKey::OandaTryFx);
    assert_eq!(
        calendar.session_bounds(utc("2025-11-17T07:00:00Z")),
        Ok(Some((
            utc("2025-11-17T07:00:00Z"),
            utc("2025-11-17T16:00:00Z")
        )))
    );
}

#[test]
fn try_forex_holiday_reopens_leave_no_session_on_christmas_or_new_years_day() {
    let calendar = ExchangeCalendar::for_market_hours_key(MarketHoursKey::OandaTryFx);
    for (eve, holiday, midday, next_open) in [
        (
            (2025, 12, 24),
            (2025, 12, 25),
            "2025-12-25T12:00:00Z",
            "2025-12-26T07:00:00Z",
        ),
        (
            (2025, 12, 31),
            (2026, 1, 1),
            "2026-01-01T12:00:00Z",
            "2026-01-02T07:00:00Z",
        ),
    ] {
        let (year, month, day) = eve;
        assert_eq!(
            calendar.is_closed_trade_date(date_opt(year, month, day).unwrap(), SessionKind::Both),
            Ok(false)
        );
        let (year, month, day) = holiday;
        assert_eq!(
            calendar.is_closed_trade_date(date_opt(year, month, day).unwrap(), SessionKind::Both),
            Ok(true)
        );
        assert_eq!(calendar.is_open(utc(midday)), Ok(false));
        assert_eq!(
            calendar.next_session_open_after(utc(midday)),
            Ok(Some(utc(next_open)))
        );
    }
}
