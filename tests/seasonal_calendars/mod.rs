// SPDX-License-Identifier: MIT-0

//! Date-aware integration suite, organized by venue and query behavior.

mod b3;
mod bmv;
mod candles_and_weekends;
mod chrono_edges;
mod contracts;
mod international;
mod transition_scans;

mod prelude {
    pub(super) use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Utc};
    pub(super) use chrono_tz::{America, Asia, Europe, US};
    pub(super) use exchange_hours::{
        CalendarResolution, Exchange, ExchangeCalendar, MarketHours, SessionKind,
        calendar_for_exchange, candle_end, candle_end_with, candle_start, candle_start_with,
        hours_for_exchange, next_session_after, next_session_after_with, session_bounds,
        session_bounds_with, time_end_of_day,
    };

    pub(super) use crate::support::local;

    pub(super) fn day(date: (i32, u32, u32)) -> NaiveDate {
        NaiveDate::from_ymd_opt(date.0, date.1, date.2).expect("valid date fixture")
    }

    pub(super) fn regular_window(hours: &MarketHours) -> (u32, u32) {
        let rule = hours
            .regular
            .first()
            .expect("cash venue has a regular rule");
        (rule.open_ssm, rule.close_ssm)
    }

    pub(super) fn assert_fixed_calendar_parity(exchange: Exchange, instant: DateTime<Utc>) {
        let calendar = calendar_for_exchange(exchange);
        let fixed = hours_for_exchange(exchange, instant);

        assert_eq!(calendar.exchange(), Some(exchange));
        assert_eq!(calendar.tz(), fixed.tz);
        assert_eq!(calendar.hours_at(instant), fixed);
        assert_eq!(
            calendar.normal_week_open_seconds_containing(instant),
            fixed.normal_week_open_seconds()
        );
        assert_eq!(calendar.is_open(instant), fixed.is_open(instant));
        assert_eq!(
            calendar.is_open_regular(instant),
            fixed.is_open_regular(instant)
        );
        assert_eq!(
            calendar.is_open_extended(instant),
            fixed.is_open_extended(instant)
        );
        assert_eq!(
            calendar.is_maintenance(instant),
            fixed.is_maintenance(instant)
        );
        assert_eq!(
            calendar.session_bounds(instant),
            session_bounds(&fixed, instant)
        );
        assert_eq!(
            calendar.next_session_after(instant),
            next_session_after(&fixed, instant)
        );

        for kind in [
            SessionKind::Regular,
            SessionKind::Extended,
            SessionKind::Both,
        ] {
            assert_eq!(
                calendar.is_open_with(instant, kind),
                fixed.is_open_with(instant, kind)
            );
            assert_eq!(
                calendar.session_bounds_with(instant, kind),
                session_bounds_with(&fixed, instant, kind)
            );
            assert_eq!(
                calendar.next_session_after_with(instant, kind),
                next_session_after_with(&fixed, instant, kind)
            );
            assert_eq!(
                calendar.candle_start_with(instant, CalendarResolution::Minutes(30), kind),
                candle_start_with(&fixed, instant, CalendarResolution::Minutes(30), kind)
            );
            assert_eq!(
                calendar.candle_end_with(instant, CalendarResolution::Minutes(30), kind),
                candle_end_with(&fixed, instant, CalendarResolution::Minutes(30), kind)
            );
        }

        for resolution in [
            CalendarResolution::Daily,
            CalendarResolution::Weekly,
            CalendarResolution::Monthly,
        ] {
            assert_eq!(
                calendar.candle_start(instant, resolution),
                candle_start(&fixed, instant, resolution)
            );
            assert_eq!(
                calendar.candle_end(instant, resolution),
                candle_end(&fixed, instant, resolution)
            );
        }
        assert_eq!(
            calendar.time_end_of_day(instant),
            time_end_of_day(&fixed, instant)
        );
        let venue_day = instant.with_timezone(&calendar.tz()).date_naive();
        for kind in [
            SessionKind::Regular,
            SessionKind::Extended,
            SessionKind::Both,
        ] {
            assert_eq!(
                calendar.is_closed_all_day_on(venue_day, kind),
                fixed.is_closed_all_day_on(venue_day, kind)
            );
        }
    }
}
