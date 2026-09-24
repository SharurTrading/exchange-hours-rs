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
        CalendarQueryError, CalendarResolution, Exchange, ExchangeCalendar, MarketHours,
        MarketHoursKey, SessionKind, calendar_for_exchange, calendar_for_market_hours_key,
        candle_end, candle_end_with, candle_start, candle_start_with, hours_for_exchange,
        hours_for_market_hours_key, next_session_after, next_session_after_with, session_bounds,
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

        // The identity may refuse this instant's venue-local date outright
        // (Stage 2B). Where it does, the parity claim is unavailable and the
        // refusal is what must be asserted; where it does not, every answer must
        // still equal the fixed snapshot's own.
        let venue_day = instant.with_timezone(&calendar.tz()).date_naive();

        // Whether a given query answers or refuses is a **per-query** fact, not
        // a property of the date: an instant-addressed session query reads only
        // the day it lands on, while a period scan (`candle_start`/`candle_end`)
        // needs earlier days to establish its bound, and a declared phase gap can
        // withhold those. So the parity claim is conditional and both branches
        // are asserted: an answer must equal the fixed snapshot's own, and a
        // refusal must be a coverage refusal rather than a fabricated closure.
        macro_rules! parity {
            ($answer:expr, $snapshot:expr, $label:expr) => {{
                // Two admissible outcomes, and the weaker one is deliberate.
                // An answer must equal the fixed snapshot's own, because the
                // snapshot is the surface that states the schedule. A refusal is
                // allowed — a date-aware query may have no sourced answer — but
                // it must be a **coverage** refusal, never a fabricated closure
                // and never a different error.
                //
                // Which of the two a given query produces is a per-query fact
                // and cannot be read off `coverage_on`: a declared phase-level
                // gap makes the whole date incomplete, yet an instant-addressed
                // session query answers across it while a period scan
                // (`candle_start`/`candle_end`) refuses because it needs the
                // earlier days the gap withholds. Asserting one uniform outcome
                // here would encode a rule the crate does not have.
                match $answer {
                    Ok(answer) => assert_eq!(answer, $snapshot, "{}", $label),
                    Err(error) => assert!(
                        matches!(
                            error,
                            CalendarQueryError::BeforeSupportFloor { .. }
                                | CalendarQueryError::OutsideCoveredRange { .. }
                                | CalendarQueryError::UnresolvedGap { .. }
                                | CalendarQueryError::SearchExhausted { .. }
                        ),
                        "{}: only a coverage refusal may stand in for the fixed \
                         snapshot's answer, got {error:?}",
                        $label
                    ),
                }
            }};
        }

        assert_eq!(calendar.exchange(), Some(exchange));
        assert_eq!(calendar.tz(), fixed.tz);
        assert_eq!(calendar.hours_at(instant), fixed);

        parity!(
            calendar.normal_week_open_seconds_containing(instant),
            fixed.normal_week_open_seconds(),
            "normal_week_open_seconds_containing"
        );
        parity!(calendar.is_open(instant), fixed.is_open(instant), "is_open");
        parity!(
            calendar.is_open_regular(instant),
            fixed.is_open_regular(instant),
            "is_open_regular"
        );
        parity!(
            calendar.is_open_extended(instant),
            fixed.is_open_extended(instant),
            "is_open_extended"
        );
        parity!(
            calendar.is_maintenance(instant),
            fixed.is_maintenance(instant),
            "is_maintenance"
        );
        parity!(
            calendar.session_bounds(instant),
            session_bounds(&fixed, instant),
            "session_bounds"
        );
        parity!(
            calendar.next_session_after(instant),
            next_session_after(&fixed, instant),
            "next_session_after"
        );

        for kind in [
            SessionKind::Regular,
            SessionKind::Extended,
            SessionKind::Both,
        ] {
            parity!(
                calendar.is_open_with(instant, kind),
                fixed.is_open_with(instant, kind),
                "is_open_with"
            );
            parity!(
                calendar.session_bounds_with(instant, kind),
                session_bounds_with(&fixed, instant, kind),
                "session_bounds_with"
            );
            parity!(
                calendar.next_session_after_with(instant, kind),
                next_session_after_with(&fixed, instant, kind),
                "next_session_after_with"
            );
            parity!(
                calendar.candle_start_with(instant, CalendarResolution::Minutes(30), kind),
                candle_start_with(&fixed, instant, CalendarResolution::Minutes(30), kind),
                "candle_start_with"
            );
            parity!(
                calendar.candle_end_with(instant, CalendarResolution::Minutes(30), kind),
                candle_end_with(&fixed, instant, CalendarResolution::Minutes(30), kind),
                "candle_end_with"
            );
            parity!(
                calendar.is_closed_all_day_on(venue_day, kind),
                fixed.is_closed_all_day_on(venue_day, kind),
                "is_closed_all_day_on"
            );
        }

        for resolution in [
            CalendarResolution::Daily,
            CalendarResolution::Weekly,
            CalendarResolution::Monthly,
        ] {
            parity!(
                calendar.candle_start(instant, resolution),
                candle_start(&fixed, instant, resolution),
                "candle_start"
            );
            parity!(
                calendar.candle_end(instant, resolution),
                candle_end(&fixed, instant, resolution),
                "candle_end"
            );
        }

        parity!(
            calendar.time_end_of_day(instant),
            time_end_of_day(&fixed, instant),
            "time_end_of_day"
        );
    }
}
