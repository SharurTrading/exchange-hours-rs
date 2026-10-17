// SPDX-License-Identifier: MIT-0

//! Broker-native Asia CFD boundaries and current publication coverage.
use chrono::{DateTime, Duration, NaiveDate, Utc};
use exchange_hours::{
    CalendarQueryError, CalendarResolution, DateCoverage, ExchangeCalendar, MarketHoursKey as K,
    SessionKind, SessionState, session_profile,
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
        let unaudited = NaiveDate::from_ymd_opt(2026, 10, 9).expect("unaudited holiday date");
        assert_eq!(
            c.coverage().coverage_on(unaudited),
            DateCoverage::OutsideCoveredRange
        );
        assert_eq!(c.is_open(utc("2026-10-09T02:00:00Z")), Ok(true));
        assert!(matches!(
            c.is_closed_trade_date(unaudited, SessionKind::Both),
            Err(CalendarQueryError::UnresolvedGap { .. })
        ));
        assert!(matches!(
            c.is_open(utc("2021-02-15T02:00:00Z")),
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

#[test]
fn last_audited_friday_returns_complete_bounds_but_saturday_queries_refuse() {
    for (key, open, close) in [
        (
            K::OandaChina50Cfd,
            "2026-10-30T09:01:00Z",
            "2026-10-30T20:44:00Z",
        ),
        (
            K::OandaSingapore30Cfd,
            "2026-10-30T09:51:00Z",
            "2026-10-30T21:14:00Z",
        ),
    ] {
        let c = ExchangeCalendar::for_market_hours_key(key);
        let saturday = NaiveDate::from_ymd_opt(2026, 10, 31).expect("closing civil date");
        for instant in [utc(open), utc("2026-10-30T15:59:59Z")] {
            assert_eq!(c.is_open(instant), Ok(true));
            assert_eq!(c.session_bounds(instant), Ok(Some((utc(open), utc(close)))));
            assert!(
                matches!(c.trade_date(instant), Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == saturday)
            );
        }
        for instant in [
            utc("2026-10-30T16:00:00Z"),
            utc(close) - Duration::seconds(1),
            utc(close),
        ] {
            assert!(
                matches!(c.is_open(instant), Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == saturday)
            );
            assert!(
                matches!(c.session_bounds(instant), Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == saturday)
            );
        }
        assert!(
            matches!(c.is_open(utc(close)), Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == saturday)
        );
        assert!(
            matches!(c.session_bounds(utc(close)), Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == saturday)
        );
        assert!(
            matches!(c.candle_end(utc("2026-10-12T02:00:00Z"), CalendarResolution::Monthly), Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == saturday)
        );
    }
}

#[test]
fn witnessed_eras_answer_on_their_own_opening_days() {
    for (
        key,
        legacy,
        former_span_day_close,
        legacy_night_close,
        legacy_last_day,
        current_first_day,
        current_day_close,
    ) in [
        (
            K::OandaChina50Cfd,
            // Legacy `09:00 - 16:30; 17:00 - 04:45`, witnessed
            // 2021-03-03..2023-06-05 by the page prints and through
            // 2024-02-09 by the operator's machine channel.
            ("2022-06-07T01:00:00Z", "2022-06-07T08:30:00Z"),
            ("2023-08-09T08:29:59Z", "2023-08-09T08:30:00Z"),
            "2023-08-08T20:45:00Z",
            ("2024-02-09T01:00:00Z", "2024-02-09T08:30:00Z"),
            ("2024-02-12T01:00:00Z", "2024-02-12T01:01:00Z"),
            ("2024-02-12T08:28:59Z", "2024-02-12T08:29:00Z"),
        ),
        (
            K::OandaSingapore30Cfd,
            // Legacy `08:30 - 17:20; 17:50 - 04:45`.
            ("2022-06-07T00:30:00Z", "2022-06-07T09:20:00Z"),
            ("2023-08-09T09:19:59Z", "2023-08-09T09:20:00Z"),
            // The night close stays at the legacy 04:45 on a former-span
            // day (04:45 SGT on Aug 9 = 20:45 UTC on Aug 8).
            "2023-08-08T20:45:00Z",
            ("2024-02-09T00:30:00Z", "2024-02-09T09:20:00Z"),
            ("2024-02-12T00:30:00Z", "2024-02-12T00:31:00Z"),
            ("2024-02-12T09:18:59Z", "2024-02-12T09:19:00Z"),
        ),
    ] {
        let c = ExchangeCalendar::for_market_hours_key(key);
        let (legacy_open, legacy_close) = legacy;
        assert_eq!(
            c.is_open(utc(legacy_open)),
            Ok(true),
            "{key:?}: the legacy day open instant"
        );
        assert_eq!(
            c.is_open(utc(legacy_close)),
            Ok(false),
            "{key:?}: the legacy day close is end-exclusive"
        );
        // The former undated span (2023-06-06..2024-02-11 opening days) is
        // dated now and serves the legacy grid: its day close keeps the
        // legacy instant.
        let (span_last_trading, span_close) = former_span_day_close;
        assert_eq!(
            c.is_open(utc(span_last_trading)),
            Ok(true),
            "{key:?}: a former-span date trades to the legacy close"
        );
        assert_eq!(
            c.is_open(utc(span_close)),
            Ok(false),
            "{key:?}: the legacy day close is end-exclusive on a former-span date"
        );
        assert_eq!(
            c.is_open(utc(legacy_night_close)),
            Ok(false),
            "{key:?}: the legacy night close instant stays closed"
        );
        // The dated boundary: Friday 2024-02-09 is the legacy grid's last
        // opening day; Monday 2024-02-12 already runs the current grid.
        let (last_open, last_close) = legacy_last_day;
        assert_eq!(
            c.is_open(utc(last_open)),
            Ok(true),
            "{key:?}: the legacy grid's last opening day opens on its grid"
        );
        assert_eq!(
            c.is_open(utc(last_close)),
            Ok(false),
            "{key:?}: the legacy grid's final day close is end-exclusive"
        );
        let (first_pre_open, first_open) = current_first_day;
        assert_eq!(
            c.is_open(utc(first_pre_open)),
            Ok(false),
            "{key:?}: one second before the current day open"
        );
        assert_eq!(
            c.is_open(utc(first_open)),
            Ok(true),
            "{key:?}: the current day open instant"
        );
        let (cur_last_trading, cur_close) = current_day_close;
        assert_eq!(
            c.is_open(utc(cur_last_trading)),
            Ok(true),
            "{key:?}: the current era's first day trades to its close"
        );
        assert_eq!(
            c.is_open(utc(cur_close)),
            Ok(false),
            "{key:?}: the current day close is end-exclusive"
        );
        // The witnessed span sits below the first audited window, so the
        // holiday classification refuses typed while sessions answer.
        let legacy_day = NaiveDate::from_ymd_opt(2022, 6, 8).expect("valid date");
        assert!(matches!(
            c.is_closed_trade_date(legacy_day, SessionKind::Both),
            Err(CalendarQueryError::UnresolvedGap { .. })
        ));
    }
}

#[test]
fn books_refuse_below_their_2021_witness() {
    for key in [K::OandaChina50Cfd, K::OandaSingapore30Cfd] {
        let c = ExchangeCalendar::for_market_hours_key(key);
        let carried = NaiveDate::from_ymd_opt(2021, 2, 15).expect("valid date");
        assert!(matches!(
            c.is_closed_trade_date(carried, SessionKind::Both),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ));
    }
}
