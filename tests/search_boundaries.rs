// SPDX-License-Identifier: MIT-0

//! Calendar searches across civil days that split or inherit an opening.

use std::borrow::Cow;

use exchange_hours::{
    DayOverride, ExceptionBlock, Exchange, ExchangeCalendar, MarketHours, MarketHoursKey,
    SessionExceptionRecord, SessionKind, SessionRule, StaticDayPolicy, StaticSessionExceptions,
};
use jiff::{Timestamp, civil::date, tz::TimeZone};

fn at(value: &str) -> Timestamp {
    value.parse().unwrap()
}

#[test]
fn civil_day_classification_uses_both_pieces_of_a_midnight_rollback() {
    let goose_bay = jiff::tz::get!("America/Goose_Bay");
    let saturday = date(2010, 11, 6);
    let sunday = date(2010, 11, 7);
    let open = at("2010-11-07T03:30Z");
    assert_eq!(goose_bay.to_datetime(open).date(), saturday);

    let sunday_utc = [false, false, false, false, false, false, true];
    let hours = MarketHours::new(
        Exchange::Unknown,
        TimeZone::UTC,
        Cow::Owned(vec![
            SessionRule::new(sunday_utc, 3 * 3_600 + 30 * 60, 3 * 3_600 + 45 * 60).unwrap(),
        ]),
        Cow::Borrowed(&[]),
        Cow::Borrowed(&[]),
        true,
        true,
    );

    assert!(!hours.is_closed_all_day_in_calendar(saturday, &goose_bay, SessionKind::Regular));
    assert!(hours.is_closed_all_day_in_calendar(sunday, &goose_bay, SessionKind::Regular));
    assert!(!hours.is_closed_all_day_at(open, &goose_bay, SessionKind::Regular));
    assert!(hours.is_closed_all_day_at(at("2010-11-07T03:00Z"), &goose_bay, SessionKind::Regular));
}

#[test]
fn cotton_forward_search_keeps_the_shifted_friday_open_after_midnight() {
    let calendar =
        ExchangeCalendar::for_market_hours_key(MarketHoursKey::IceUsCotton).with_last_known_week();
    for day in ["2025-11-28", "2026-11-27"] {
        let open = at(&format!("{day}T13:00Z"));
        let close = at(&format!("{day}T18:30Z"));
        let midnight = at(&format!("{day}T05:00Z"));
        for instant in [
            midnight
                .checked_sub(jiff::SignedDuration::from_nanos(1))
                .unwrap(),
            midnight,
            at(&format!("{day}T12:59:59.999999999Z")),
        ] {
            for kind in [SessionKind::Both, SessionKind::Regular] {
                assert_eq!(
                    calendar.next_session_after_with(instant, kind),
                    Ok(Some((open, close)))
                );
                assert_eq!(
                    calendar.session_bounds_with(instant, kind),
                    Ok(Some((open, close)))
                );
            }
            assert_eq!(calendar.next_session_open_after(instant), Ok(Some(open)));
        }
        assert_eq!(calendar.session_bounds(open), Ok(Some((open, close))));
        assert_ne!(calendar.next_session_after(open), Ok(Some((open, close))));
        assert_eq!(calendar.is_open_regular(close), Ok(false));
    }
}

#[test]
fn forward_search_considers_a_shifted_occurrence_from_a_caller_policy() {
    let records = [DayOverride::late_open(date(2026, 4, 21), 8 * 3_600).unwrap()];
    let policy = StaticDayPolicy::new(&records).unwrap();
    let calendar = ExchangeCalendar::for_market_hours_key(MarketHoursKey::IceUsCotton)
        .without_holidays()
        .with_day_policy(&policy);
    let midnight = at("2026-04-21T04:00Z");
    let open = at("2026-04-21T12:00Z");
    let close = at("2026-04-21T18:20Z");

    assert_eq!(
        calendar.next_session_after(midnight),
        Ok(Some((open, close)))
    );
    assert_eq!(calendar.session_bounds(midnight), Ok(Some((open, close))));
    assert_eq!(
        calendar.is_closed_all_day_on(date(2026, 4, 21), SessionKind::Regular),
        Ok(false)
    );
}

#[test]
fn a_session_opening_before_the_rollback_remains_open_when_the_date_goes_back() {
    let zone = jiff::tz::get!("America/Goose_Bay");
    let sunday = [false, false, false, false, false, false, true];
    let hours = MarketHours::new(
        Exchange::Unknown,
        zone.clone(),
        Cow::Owned(vec![SessionRule::new(sunday, 0, 45 * 60).unwrap()]),
        Cow::Borrowed(&[]),
        Cow::Borrowed(&[]),
        true,
        true,
    );
    let saturday_again = at("2010-11-07T03:30Z");
    let bounds = (at("2010-11-07T03:00Z"), at("2010-11-07T04:45Z"));
    let saturday = date(2010, 11, 6);
    assert_eq!(zone.to_datetime(saturday_again).date(), saturday);
    assert!(hours.is_open_regular(saturday_again));
    assert_eq!(
        exchange_hours::session_bounds_with(&hours, saturday_again, SessionKind::Regular),
        Some(bounds)
    );
    assert!(!hours.is_closed_all_day_on(saturday, SessionKind::Regular));
}

#[test]
fn identity_backed_classification_checks_the_repeated_date_in_an_external_zone() {
    let zone = jiff::tz::get!("America/Goose_Bay");
    let saturday = date(2010, 11, 6);
    let sunday = date(2010, 11, 7);
    let calendar = ExchangeCalendar::new(Exchange::Nyse).without_holidays();
    let blocks = [ExceptionBlock::regular(
        -1,
        23 * 3_600 + 30 * 60,
        23 * 3_600 + 45 * 60,
    )];
    let records = [SessionExceptionRecord::replace_sessions(sunday, &blocks)];
    let exceptions =
        StaticSessionExceptions::new(calendar.source(), sunday, sunday, &records).unwrap();
    let calendar = calendar.with_session_exceptions(&exceptions).unwrap();
    assert_eq!(
        calendar.is_closed_all_day_in_calendar(saturday, &zone, SessionKind::Regular),
        Ok(false)
    );
    assert_eq!(
        calendar.is_closed_all_day_in_calendar(sunday, &zone, SessionKind::Regular),
        Ok(true)
    );
}

#[test]
fn a_shifted_open_does_not_preempt_an_earlier_open_on_the_following_day() {
    let tuesday = date(2026, 4, 21);
    let wednesday = date(2026, 4, 22);
    let calendar = ExchangeCalendar::new(Exchange::Nyse).without_holidays();
    let overnight = [ExceptionBlock::regular(
        -1,
        21 * 3_600,
        14 * 3_600 + 20 * 60,
    )];
    let morning = [ExceptionBlock::regular(-1, 7 * 3_600, 7 * 3_600 + 30 * 60)];
    let exceptions = [
        SessionExceptionRecord::replace_sessions(tuesday, &overnight),
        SessionExceptionRecord::replace_sessions(wednesday, &morning),
    ];
    let exceptions =
        StaticSessionExceptions::new(calendar.source(), tuesday, wednesday, &exceptions).unwrap();
    let overrides = [DayOverride::late_open(tuesday, 8 * 3_600).unwrap()];
    let policy = StaticDayPolicy::new(&overrides).unwrap();
    let calendar = calendar
        .with_session_exceptions(&exceptions)
        .unwrap()
        .with_day_policy(&policy);
    let monday_evening = at("2026-04-21T00:00Z");
    let tuesday_morning = at("2026-04-21T04:00Z");
    let earlier = (at("2026-04-21T11:00Z"), at("2026-04-21T11:30Z"));
    let shifted = (at("2026-04-21T12:00Z"), at("2026-04-21T18:20Z"));
    assert_eq!(
        calendar.next_session_after(monday_evening),
        Ok(Some(earlier))
    );
    assert_eq!(
        calendar.next_session_after(tuesday_morning),
        Ok(Some(earlier))
    );
    assert_eq!(calendar.next_session_after(earlier.1), Ok(Some(shifted)));
}

#[test]
fn a_proven_midnight_open_does_not_read_the_following_unaudited_opening_day() {
    let calendar = ExchangeCalendar::new(Exchange::Nyse);
    let unaudited = calendar
        .holiday_coverage()
        .unwrap()
        .last()
        .tomorrow()
        .unwrap();
    assert_eq!(unaudited, date(2029, 1, 1));
    let blocks = [ExceptionBlock::regular(
        -1,
        21 * 3_600,
        14 * 3_600 + 20 * 60,
    )];
    let exceptions = [SessionExceptionRecord::replace_sessions(unaudited, &blocks)];
    let exceptions =
        StaticSessionExceptions::new(calendar.source(), unaudited, unaudited, &exceptions).unwrap();
    let overrides = [DayOverride::late_open(unaudited, 0).unwrap()];
    let policy = StaticDayPolicy::new(&overrides).unwrap();
    let calendar = calendar
        .with_session_exceptions(&exceptions)
        .unwrap()
        .with_day_policy(&policy);
    let open = at("2029-01-01T05:00Z");
    let close = at("2029-01-01T19:20Z");
    let earlier = at("2029-01-01T04:59:59.999999999Z");
    assert_eq!(
        calendar.next_session_after(earlier),
        Ok(Some((open, close)))
    );
    assert!(matches!(
        calendar.next_session_after(open),
        Err(error) if matches!(error.kind(), exchange_hours::CalendarQueryErrorKind::OutsideCoveredRange { date, .. } if date == unaudited)
    ));
}

#[test]
fn a_late_open_masked_by_an_explicit_closure_cannot_refuse_the_forward_search() {
    let calendar = ExchangeCalendar::new(Exchange::Iceus);
    let trade_date = date(2025, 1, 22);
    let records = [SessionExceptionRecord::closed(trade_date)];
    let exceptions =
        StaticSessionExceptions::new(calendar.source(), trade_date, trade_date, &records).unwrap();
    let overrides = [DayOverride::late_open(trade_date, 8 * 3_600).unwrap()];
    let policy = StaticDayPolicy::new(&overrides).unwrap();
    let closed = calendar.with_session_exceptions(&exceptions).unwrap();
    let instant = at("2025-01-22T05:00Z");
    let expected = Ok(Some((at("2025-01-23T01:00Z"), at("2025-01-23T23:00Z"))));

    assert_eq!(closed.next_session_after(instant), expected);
    assert_eq!(
        closed.with_day_policy(&policy).next_session_after(instant),
        expected
    );
}

#[test]
fn an_evening_late_open_does_not_require_the_withheld_preceding_holiday() {
    let calendar = ExchangeCalendar::for_market_hours_key(MarketHoursKey::OandaUsBondCfd);
    let next = Some((at("2024-12-26T23:01Z"), at("2024-12-27T21:59Z")));
    let after = Some((at("2024-12-29T23:01Z"), at("2024-12-30T21:59Z")));
    for calendar in [calendar, calendar.with_last_known_week()] {
        for instant in [at("2024-12-26T05:00Z"), at("2024-12-26T22:59:59Z")] {
            assert_eq!(calendar.next_session_after(instant), Ok(next));
            assert_eq!(
                calendar.next_session_after_with(instant, SessionKind::Regular),
                Ok(next)
            );
        }
        assert_eq!(
            calendar.next_session_after(at("2024-12-26T23:04Z")),
            Ok(after)
        );
    }
}

#[test]
fn an_actual_morning_shift_still_requires_its_withheld_history() {
    let calendar = ExchangeCalendar::new(Exchange::Iceus);
    let day = date(2025, 1, 22);
    let records = [DayOverride::late_open(day, 8 * 3_600).unwrap()];
    let policy = StaticDayPolicy::new(&records).unwrap();
    let instant = at("2025-01-22T05:00Z");
    assert!(matches!(
        calendar.with_day_policy(&policy).next_session_after(instant),
        Err(error) if matches!(
            error.kind(),
            exchange_hours::CalendarQueryErrorKind::UnresolvedGap { date: refused, .. }
                if refused == date(2025, 1, 20)
        )
    ));
    assert_eq!(
        calendar
            .with_last_known_week()
            .with_day_policy(&policy)
            .next_session_after(instant),
        Ok(Some((at("2025-01-22T13:00Z"), at("2025-01-22T23:00Z"))))
    );
}
