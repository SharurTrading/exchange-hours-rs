// SPDX-License-Identifier: MIT-0

//! Public-surface contracts for [`ExchangeCalendar::with_next_known_week`]: the
//! caller-selected view for a consumer replaying history, such as a backtest, that
//! reads a date before an identity's coverage from the next week it knows, while
//! every answer the default calendar gives inside coverage stands (LAW-COVERAGE).

#![expect(
    clippy::expect_used,
    reason = "fixture constructors assert their own literals; a bad literal must fail the test"
)]

use chrono::{DateTime, Duration, NaiveDate, Utc};
use exchange_hours::{
    CalendarQueryError, CalendarResolution, DateCoverage, Exchange, ExchangeCalendar,
    MarketHoursKey, SUPPORT_FLOOR, SessionKind,
};

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("fixture must be a valid date")
}

fn utc(text: &str) -> DateTime<Utc> {
    text.parse().expect("fixture must be an RFC 3339 instant")
}

fn every_calendar() -> impl Iterator<Item = ExchangeCalendar> {
    Exchange::ALL
        .iter()
        .map(|exchange| ExchangeCalendar::new(*exchange))
        .chain(
            MarketHoursKey::ALL
                .iter()
                .map(|key| ExchangeCalendar::for_market_hours_key(*key)),
        )
}

fn nyse() -> ExchangeCalendar {
    ExchangeCalendar::new(Exchange::Nyse)
}

/// Every instant-addressed answer one calendar gives, rendered so two views can
/// be compared and a refusal stays visible as a refusal.
fn answers(
    calendar: ExchangeCalendar,
    at: DateTime<Utc>,
) -> [Result<String, CalendarQueryError>; 8] {
    fn text<T: core::fmt::Debug>(
        answer: Result<T, CalendarQueryError>,
    ) -> Result<String, CalendarQueryError> {
        answer.map(|value| format!("{value:?}"))
    }
    [
        text(calendar.is_open(at)),
        text(calendar.session_state(at)),
        text(calendar.trade_date(at)),
        text(calendar.session_bounds_with(at, SessionKind::Both)),
        text(calendar.next_session_open_after(at)),
        text(calendar.candle_start(at, CalendarResolution::Daily)),
        text(calendar.candle_start(at, CalendarResolution::Monthly)),
        text(calendar.candle_end(at, CalendarResolution::Monthly)),
    ]
}

#[test]
fn the_default_calendar_keeps_refusing_before_the_floor() {
    assert_eq!(
        nyse().is_open(utc("2005-03-01T16:00:00Z")),
        Err(CalendarQueryError::BeforeSupportFloor {
            source: nyse().source(),
            date: date(2005, 3, 1),
        })
    );
}

#[test]
fn a_date_before_the_floor_reads_the_next_known_week() {
    let view = nyse().with_next_known_week();
    assert_eq!(view.is_open(utc("2005-03-01T16:00:00Z")), Ok(true));
    assert_eq!(view.is_open_regular(utc("2005-03-05T16:00:00Z")), Ok(false));
    assert_eq!(
        view.session_bounds_with(utc("2005-03-01T16:00:00Z"), SessionKind::Regular),
        Ok(Some((
            utc("2005-03-01T14:30:00Z"),
            utc("2005-03-01T21:00:00Z")
        )))
    );
}

#[test]
fn the_next_known_week_follows_the_venue_clock_across_a_daylight_saving_change() {
    let view = nyse().with_next_known_week();
    assert_eq!(
        view.session_bounds_with(utc("2005-07-01T15:00:00Z"), SessionKind::Regular),
        Ok(Some((
            utc("2005-07-01T13:30:00Z"),
            utc("2005-07-01T20:00:00Z")
        )))
    );
}

#[test]
fn a_search_from_before_the_floor_reaches_audited_rows_inside_coverage() {
    // Friday 2010-01-01 is an audited NYSE closure, so the first session after
    // New Year's Eve 2009 opens on Monday 2010-01-04.
    let view = nyse().with_next_known_week();
    let bounds = view
        .session_bounds_with(utc("2009-12-31T21:30:00Z"), SessionKind::Regular)
        .expect("the view answers before the floor");
    assert_eq!(
        bounds,
        Some((utc("2010-01-04T14:30:00Z"), utc("2010-01-04T21:00:00Z")))
    );
}

#[test]
fn a_carried_era_reads_the_next_known_week() {
    let mut checked = 0;
    for calendar in every_calendar() {
        let Some(sourced_from) = calendar.coverage().normal_week_sourced_from() else {
            continue;
        };
        if sourced_from <= SUPPORT_FLOOR + Duration::days(3) {
            continue;
        }
        let carried = sourced_from - Duration::days(3);
        let at = carried
            .and_hms_opt(12, 0, 0)
            .expect("noon exists")
            .and_utc();
        let view = calendar.with_next_known_week();
        assert_eq!(
            view.coverage().coverage_on(carried),
            DateCoverage::NormalWeekOnly,
            "{:?}",
            view.source()
        );
        assert_eq!(
            view.is_open(at),
            Ok(view.hours_at(at).is_open(at)),
            "{:?}",
            view.source()
        );
        checked += 1;
    }
    assert!(checked > 0, "no identity carries an era to check");
}

#[test]
fn the_view_changes_nothing_after_coverage() {
    let view = nyse().with_next_known_week();
    let at = utc("2029-03-01T15:00:00Z");
    assert_eq!(view.is_open(at), nyse().is_open(at));
    assert!(view.is_open(at).is_err());
    let paris = ExchangeCalendar::new(Exchange::EuronextParis);
    let withheld = utc("2026-12-24T10:00:00Z");
    assert_eq!(
        paris.with_next_known_week().is_open(withheld),
        paris.is_open(withheld)
    );
    assert!(paris.is_open(withheld).is_err());
}

#[test]
fn the_view_agrees_with_the_default_wherever_the_default_answers() {
    for calendar in every_calendar() {
        let view = calendar.with_next_known_week();
        let mut at = utc("2010-01-20T11:00:00Z");
        while at < utc("2027-01-01T00:00:00Z") {
            for (bare, seen) in answers(calendar, at).iter().zip(answers(view, at).iter()) {
                if bare.is_ok() {
                    assert_eq!(bare, seen, "{:?} at {at}", calendar.source());
                }
            }
            at += Duration::hours(24 * 97 + 5);
        }
    }
}

#[test]
fn with_the_last_known_week_no_date_refuses() {
    for calendar in every_calendar() {
        let view = calendar.with_last_known_week().with_next_known_week();
        let mut at = utc("1995-01-16T11:00:00Z");
        while at < utc("2031-01-01T00:00:00Z") {
            for answer in answers(view, at) {
                assert!(
                    matches!(
                        answer,
                        Ok(_) | Err(CalendarQueryError::SearchExhausted { .. })
                    ),
                    "{:?} at {at}: {answer:?}",
                    view.source()
                );
            }
            at += Duration::hours(24 * 59 + 7);
        }
    }
}

#[test]
fn the_coverage_metadata_names_the_normal_week_contract_before_the_floor() {
    assert_eq!(
        nyse().coverage().coverage_on(date(2005, 3, 1)),
        DateCoverage::BeforeSupportFloor
    );
    assert_eq!(
        nyse()
            .with_next_known_week()
            .coverage()
            .coverage_on(date(2005, 3, 1)),
        DateCoverage::NormalWeekOnly
    );
}

#[test]
fn the_view_is_distinct_and_composes_in_either_order() {
    let calendar = nyse();
    let view = calendar.with_next_known_week();
    assert_ne!(view, calendar);
    assert_ne!(view.coverage(), calendar.coverage());
    assert_eq!(view.source(), calendar.source());
    assert_eq!(
        calendar.with_next_known_week().with_last_known_week(),
        calendar.with_last_known_week().with_next_known_week()
    );
    assert_eq!(
        calendar.with_next_known_week().without_holidays(),
        calendar.without_holidays().with_next_known_week()
    );
    assert_eq!(
        view.without_holidays().is_open(utc("2005-03-01T16:00:00Z")),
        Ok(true)
    );
}
