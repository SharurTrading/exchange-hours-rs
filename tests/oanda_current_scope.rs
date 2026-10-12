// SPDX-License-Identifier: MIT-0

//! Current TRY sessions within OANDA's published Forex holiday scope.

#![expect(
    clippy::expect_used,
    reason = "invalid fixture literals must fail loudly"
)]

use chrono::{DateTime, Duration, NaiveDate, Utc};
use exchange_hours::{ExchangeCalendar, MarketHoursKey, SessionKind, SessionState};

fn utc(value: &str) -> DateTime<Utc> {
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
        let date = NaiveDate::from_ymd_opt(2026, 10, day).expect("valid date");
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
            calendar.is_open(utc(close) - Duration::seconds(1)),
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
fn current_try_coverage_does_not_certify_history_or_unpublished_future() {
    let calendar = ExchangeCalendar::for_market_hours_key(MarketHoursKey::OandaTryFx);
    for date in ["2026-10-09T12:00:00Z", "2026-11-02T12:00:00Z"] {
        assert!(calendar.is_open(utc(date)).is_err());
    }
    assert!(
        calendar
            .next_session_open_after(utc("2026-10-30T17:00:00Z"))
            .is_err()
    );
}
