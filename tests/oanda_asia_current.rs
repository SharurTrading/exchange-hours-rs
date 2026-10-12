// SPDX-License-Identifier: MIT-0

//! Broker-native Asia CFD boundaries and current publication coverage.
use chrono::{DateTime, Duration, NaiveDate, Utc};
use exchange_hours::{
    CalendarQueryError, CalendarResolution, DateCoverage, ExchangeCalendar, MarketHoursKey as K,
    SessionState, session_profile,
};

#[expect(
    clippy::expect_used,
    reason = "validated UTC fixtures must fail loudly"
)]
fn utc(s: &str) -> DateTime<Utc> {
    s.parse().expect("valid UTC fixture")
}

#[test]
fn broker_day_night_boundaries_and_friday_wrap_are_exact() {
    for (key, day_open, day_close, night_open, night_close) in [
        (
            K::OandaChina50Cfd,
            "2026-10-12T01:01:00Z",
            "2026-10-12T08:29:00Z",
            "2026-10-12T09:01:00Z",
            "2026-10-12T20:44:00Z",
        ),
        (
            K::OandaSingapore30Cfd,
            "2026-10-12T00:31:00Z",
            "2026-10-12T09:19:00Z",
            "2026-10-12T09:51:00Z",
            "2026-10-12T21:14:00Z",
        ),
    ] {
        let c = ExchangeCalendar::for_market_hours_key(key);
        assert_eq!(session_profile(key).tz, chrono_tz::Asia::Singapore);
        assert!(session_profile(key).regular.is_empty());
        assert!(session_profile(key).order_entry.is_empty());
        for (open, close) in [
            (utc(day_open), utc(day_close)),
            (utc(night_open), utc(night_close)),
        ] {
            assert_eq!(c.is_open(open - Duration::seconds(1)), Ok(false));
            assert_eq!(c.is_open(open), Ok(true));
            assert_eq!(c.is_open_regular(open), Ok(false));
            assert_eq!(c.is_open_extended(open), Ok(true));
            assert_eq!(
                c.candle_end(open, CalendarResolution::Minutes(1)),
                Ok(Some(open + Duration::minutes(1)))
            );
            assert_eq!(c.session_state(open), Ok(SessionState::OpenExtended));
            assert_eq!(c.is_order_entry_only(open), Ok(false));
            assert_eq!(c.is_open(close - Duration::seconds(1)), Ok(true));
            assert_eq!(c.is_open(close), Ok(false));
        }
        let friday_open = utc(night_open) + Duration::days(4);
        let saturday_close = utc(night_close) + Duration::days(4);
        assert_eq!(c.is_open(friday_open), Ok(true));
        assert_eq!(
            c.trade_date(friday_open),
            Ok(Some(
                NaiveDate::from_ymd_opt(2026, 10, 17).expect("Saturday")
            ))
        );
        assert_eq!(
            c.session_bounds(friday_open),
            Ok(Some((friday_open, saturday_close)))
        );
        assert_eq!(c.is_open(saturday_close), Ok(false));
        assert_eq!(c.is_open(utc("2026-10-17T10:00:00Z")), Ok(false));
        assert_eq!(c.is_open(utc("2026-10-18T10:00:00Z")), Ok(false));
        assert_eq!(
            c.next_session_open_after(saturday_close),
            Ok(Some(utc(day_open) + Duration::days(7)))
        );
        assert_eq!(key.as_str().parse::<K>(), Ok(key));
        assert_eq!(
            serde_json::from_str::<K>(&serde_json::to_string(&key).expect("serialize key"))
                .expect("deserialize key"),
            key
        );
    }
}

#[test]
fn night_and_next_day_have_closing_civil_trade_date_but_separate_bounds() {
    for (key, open, close) in [
        (
            K::OandaChina50Cfd,
            "2026-10-12T09:01:00Z",
            "2026-10-12T20:44:00Z",
        ),
        (
            K::OandaSingapore30Cfd,
            "2026-10-12T09:51:00Z",
            "2026-10-12T21:14:00Z",
        ),
    ] {
        let c = ExchangeCalendar::for_market_hours_key(key);
        assert_eq!(
            c.trade_date(utc(open)),
            Ok(Some(
                NaiveDate::from_ymd_opt(2026, 10, 13).expect("Tuesday")
            ))
        );
        assert_eq!(
            c.session_bounds(utc(open)),
            Ok(Some((utc(open), utc(close))))
        );
    }
}

#[test]
fn only_the_witnessed_current_publication_window_claims_coverage() {
    for key in [K::OandaChina50Cfd, K::OandaSingapore30Cfd] {
        let c = ExchangeCalendar::for_market_hours_key(key);
        assert_eq!(
            c.coverage()
                .coverage_on(NaiveDate::from_ymd_opt(2026, 10, 12).expect("current date")),
            DateCoverage::Covered
        );
        assert!(matches!(
            c.is_open(utc("2026-10-09T02:00:00Z")),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ));
        assert!(matches!(
            c.is_open(utc("2026-11-02T02:00:00Z")),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ));
        assert!(
            c.holiday_on(NaiveDate::from_ymd_opt(2026, 10, 19).expect("HK-only holiday"))
                .is_none()
        );
    }
}
