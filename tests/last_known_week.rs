// SPDX-License-Identifier: MIT-0

//! Public-surface contracts for [`ExchangeCalendar::with_last_known_week`]: the
//! caller-selected view that answers from the week the identity knows wherever
//! the default calendar would refuse, while every audited row keeps its
//! arrangement and only the support floor still refuses (LAW-COVERAGE).

#![expect(
    clippy::expect_used,
    reason = "fixture constructors assert their own literals; a bad literal must fail the test"
)]

use chrono::{DateTime, Duration, NaiveDate, Utc};
use exchange_hours::{
    CalendarQueryError, CalendarResolution, CoverageGapReason, DateCoverage, Exchange,
    ExchangeCalendar, HolidayContract, HolidayKind, MarketHoursKey, SUPPORT_FLOOR,
    SessionExceptionRecord, SessionKind, StaticDayPolicy, StaticSessionExceptions,
};
use exchange_hours::{DayOverride, ExceptionBlock};

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
) -> [Result<String, CalendarQueryError>; 12] {
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
        text(calendar.candle_end(at, CalendarResolution::Monthly)),
        text(calendar.candle_start(at, CalendarResolution::Weekly)),
        text(calendar.candle_start(at, CalendarResolution::Monthly)),
        text(calendar.is_accepting_orders(at)),
        text(calendar.is_order_entry_only(at)),
        text(calendar.is_closed_all_day_at(at, calendar.tz(), SessionKind::Both)),
    ]
}

#[test]
fn the_default_calendar_keeps_refusing_above_the_last_window() {
    let at = utc("2029-03-01T15:00:00Z");
    assert_eq!(
        nyse().is_open(at),
        Err(CalendarQueryError::OutsideCoveredRange {
            source: nyse().source(),
            date: date(2029, 3, 1),
        })
    );
}

#[test]
fn a_date_above_the_last_window_answers_the_sourced_normal_week() {
    let view = nyse().with_last_known_week();
    assert_eq!(view.is_open(utc("2029-03-01T15:00:00Z")), Ok(true));
    assert_eq!(
        view.session_bounds_with(utc("2029-03-01T15:00:00Z"), SessionKind::Regular),
        Ok(Some((
            utc("2029-03-01T14:30:00Z"),
            utc("2029-03-01T21:00:00Z")
        )))
    );
    assert_eq!(view.is_open_regular(utc("2029-03-03T15:00:00Z")), Ok(false));
}

#[test]
fn the_normal_week_follows_the_venue_clock_across_a_daylight_saving_change() {
    let view = nyse().with_last_known_week();
    assert_eq!(
        view.session_bounds_with(utc("2029-07-02T15:00:00Z"), SessionKind::Regular),
        Ok(Some((
            utc("2029-07-02T13:30:00Z"),
            utc("2029-07-02T20:00:00Z")
        )))
    );
}

#[test]
fn a_trade_date_above_the_last_window_classifies_from_the_normal_week() {
    let view = nyse().with_last_known_week();
    assert_eq!(
        view.is_closed_trade_date(date(2029, 3, 5), SessionKind::Both),
        Ok(false)
    );
    assert_eq!(
        view.is_closed_trade_date(date(2029, 3, 4), SessionKind::Both),
        Ok(true)
    );
}

#[test]
fn audited_rows_keep_their_arrangement() {
    let view = nyse().with_last_known_week();
    assert_eq!(view.is_open(utc("2026-11-26T15:00:00Z")), Ok(false));
    assert_eq!(
        view.session_bounds_with(utc("2026-11-27T17:00:00Z"), SessionKind::Regular),
        Ok(Some((
            utc("2026-11-27T14:30:00Z"),
            utc("2026-11-27T18:00:00Z")
        )))
    );
    assert_eq!(view.is_open(utc("2028-12-25T15:00:00Z")), Ok(false));
}

#[test]
fn a_search_crossing_the_last_window_answers_from_the_week_beyond_it() {
    assert!(matches!(
        nyse().next_session_open_after(utc("2028-12-29T20:30:00Z")),
        Err(CalendarQueryError::OutsideCoveredRange { .. })
    ));
    assert_eq!(
        nyse()
            .with_last_known_week()
            .next_session_open_after(utc("2028-12-29T20:30:00Z")),
        Ok(Some(utc("2029-01-01T12:00:00Z")))
    );
}

#[test]
fn a_search_crossing_the_last_window_keeps_the_audited_row_it_starts_from() {
    let ibex = ExchangeCalendar::for_market_hours_key(MarketHoursKey::BmeIbex);
    let at = utc("2026-12-31T10:00:00Z");
    assert!(matches!(
        ibex.candle_end(at, CalendarResolution::Monthly),
        Err(CalendarQueryError::OutsideCoveredRange { .. })
    ));
    assert_eq!(
        ibex.with_last_known_week()
            .candle_end(at, CalendarResolution::Monthly),
        Ok(Some(utc("2026-12-31T13:00:00Z")))
    );
}

#[test]
fn the_view_agrees_with_the_default_wherever_the_default_answers() {
    for calendar in every_calendar() {
        let Some(coverage) = calendar.holiday_coverage() else {
            continue;
        };
        let view = calendar.with_last_known_week();
        let end = coverage
            .last()
            .and_hms_opt(12, 0, 0)
            .expect("noon exists")
            .and_utc();
        let mut at = end - Duration::days(21);
        while at < end + Duration::days(5) {
            for (bare, seen) in answers(calendar, at).iter().zip(answers(view, at).iter()) {
                if bare.is_ok() {
                    assert_eq!(bare, seen, "{:?} at {at}", calendar.source());
                }
            }
            at += Duration::hours(19);
        }
    }
}

#[test]
fn well_above_the_last_window_the_view_answers_exactly_the_normal_week() {
    for calendar in every_calendar() {
        let Some(coverage) = calendar.holiday_coverage() else {
            continue;
        };
        let view = calendar.with_last_known_week();
        let detached = calendar.without_holidays();
        let start = coverage
            .last()
            .and_hms_opt(12, 0, 0)
            .expect("noon exists")
            .and_utc()
            + Duration::days(60);
        for step in 0..21 {
            let at = start + Duration::hours(step * 11);
            assert_eq!(
                answers(view, at),
                answers(detached, at),
                "{:?} at {at}",
                calendar.source()
            );
        }
    }
}

#[test]
fn an_identity_with_no_table_answers_its_normal_week() {
    let iex = ExchangeCalendar::new(Exchange::Iex);
    let at = utc("2026-11-02T15:00:00Z");
    assert_eq!(
        iex.is_open(at),
        Err(CalendarQueryError::OutsideCoveredRange {
            source: iex.source(),
            date: date(2026, 11, 2),
        })
    );
    let view = iex.with_last_known_week();
    assert_eq!(view.is_open(at), Ok(true));
    assert_eq!(
        view.session_state(at),
        Ok(iex.hours_at(at).session_state(at))
    );
    assert_eq!(view.coverage().holiday_contract(), HolidayContract::NoTable);
}

#[test]
fn a_withheld_date_reads_the_normal_week() {
    let paris = ExchangeCalendar::new(Exchange::EuronextParis);
    let at = utc("2026-12-24T10:00:00Z");
    assert_eq!(
        paris.is_open(at),
        Err(CalendarQueryError::UnresolvedGap {
            source: paris.source(),
            date: date(2026, 12, 24),
        })
    );
    let view = paris.with_last_known_week();
    assert_eq!(view.is_open(at), Ok(true));
    assert_eq!(view.is_open(utc("2026-12-24T20:00:00Z")), Ok(false));
    assert_eq!(
        view.is_closed_trade_date(date(2026, 12, 24), SessionKind::Both),
        Ok(false)
    );
    assert_eq!(
        view.coverage().coverage_on(date(2026, 12, 24)),
        DateCoverage::NormalWeekOnly
    );
}

#[test]
fn a_date_beside_a_withheld_date_answers_every_query() {
    let paris = ExchangeCalendar::new(Exchange::EuronextParis);
    let at = utc("2026-12-23T17:00:00Z");
    assert!(paris.next_session_open_after(at).is_err());
    let view = paris.with_last_known_week();
    assert_eq!(
        view.next_session_open_after(at),
        Ok(Some(utc("2026-12-24T08:00:00Z")))
    );
    assert_eq!(
        view.coverage().coverage_on(date(2026, 12, 23)),
        DateCoverage::Covered
    );
}

#[test]
fn a_carried_era_reads_the_earliest_known_week() {
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
        assert!(
            matches!(
                calendar.is_open(at),
                Err(CalendarQueryError::OutsideCoveredRange { .. })
            ),
            "{:?}",
            calendar.source()
        );
        let view = calendar.with_last_known_week();
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
fn at_or_above_the_floor_the_view_never_refuses_a_date() {
    for calendar in every_calendar() {
        let view = calendar.with_last_known_week();
        let mut at = utc("2010-02-15T11:00:00Z");
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
            at += Duration::hours(24 * 41 + 7);
        }
    }
}

#[test]
fn every_withheld_date_answers_under_the_view() {
    let mut checked = 0;
    for calendar in every_calendar() {
        let view = calendar.with_last_known_week();
        let mut day = date(2025, 1, 1);
        while day <= date(2029, 1, 1) {
            if calendar
                .holiday_on(day)
                .is_some_and(|holiday| holiday.kind() == HolidayKind::Unsourced)
            {
                let at = day.and_hms_opt(12, 0, 0).expect("noon exists").and_utc();
                assert_eq!(
                    view.is_open(at),
                    Ok(view.hours_at(at).is_open(at)),
                    "{:?} on {day}",
                    view.source()
                );
                checked += 1;
            }
            day = day.succ_opt().expect("fixture dates are representable");
        }
    }
    assert!(checked > 0, "no identity withholds a date to check");
}

#[test]
fn a_declared_phase_gap_reads_the_normal_week_under_the_view() {
    let hkex = ExchangeCalendar::for_market_hours_key(MarketHoursKey::HkexIndexFutures);
    let day = date(2015, 6, 2);
    assert_eq!(
        hkex.coverage().coverage_on(day),
        DateCoverage::OutsideCoveredRange
    );
    assert_eq!(
        hkex.with_last_known_week().coverage().coverage_on(day),
        DateCoverage::NormalWeekOnly
    );
}

#[test]
fn a_date_below_the_floor_stays_refused() {
    let view = nyse().with_last_known_week();
    assert_eq!(
        view.is_open(utc("2009-12-31T16:00:00Z")),
        Err(CalendarQueryError::BeforeSupportFloor {
            source: view.source(),
            date: date(2009, 12, 31),
        })
    );
}

#[test]
fn the_coverage_metadata_names_the_normal_week_contract_above_the_last_window() {
    let view = nyse().with_last_known_week().coverage();
    assert_eq!(
        view.coverage_on(date(2029, 3, 1)),
        DateCoverage::NormalWeekOnly
    );
    assert!(!view.is_complete_on(date(2029, 3, 1)));
    assert!(matches!(
        view.holiday_contract(),
        HolidayContract::Audited { .. }
    ));
    let last = view.gaps().last().expect("the walk reaches the open end");
    assert_eq!(last.reason(), CoverageGapReason::NormalWeekOnly);
    assert!(last.range().is_open_ended());
}

#[test]
fn the_last_audited_dates_are_complete_because_their_neighbour_now_answers() {
    assert_eq!(
        nyse().coverage().coverage_on(date(2028, 12, 31)),
        DateCoverage::OutsideCoveredRange
    );
    assert_eq!(
        nyse()
            .with_last_known_week()
            .coverage()
            .coverage_on(date(2028, 12, 31)),
        DateCoverage::Covered
    );
}

#[test]
fn the_views_are_distinct_and_detaching_wins_in_either_order() {
    let calendar = nyse();
    let view = calendar.with_last_known_week();
    assert_ne!(view, calendar);
    assert_ne!(view.coverage(), calendar.coverage());
    assert_eq!(view.source(), calendar.source());
    assert_eq!(view.without_holidays(), calendar.without_holidays());
    assert_eq!(
        calendar.without_holidays().with_last_known_week(),
        calendar.without_holidays()
    );
}

#[test]
fn a_caller_policy_still_applies_above_the_last_window() {
    let overrides = [DayOverride::closed(date(2029, 3, 1))];
    let policy = StaticDayPolicy::new(&overrides).expect("one valid override");
    let view = nyse().with_last_known_week();
    let overlaid = view.with_day_policy(&policy);
    assert_eq!(overlaid.is_open(utc("2029-03-01T15:00:00Z")), Ok(false));
    assert_eq!(overlaid.is_open(utc("2029-03-02T15:00:00Z")), Ok(true));
}

#[test]
fn a_caller_exception_still_applies_above_the_last_window() {
    let view = nyse().with_last_known_week();
    let blocks = [ExceptionBlock::regular(0, 36_000, 43_200)];
    let records = [SessionExceptionRecord::replace_sessions(
        date(2029, 3, 1),
        &blocks,
    )];
    let table =
        StaticSessionExceptions::new(view.source(), date(2029, 3, 1), date(2029, 3, 1), &records)
            .expect("one valid record");
    let overlaid = view.with_session_exceptions(&table).expect("same identity");
    assert_eq!(overlaid.is_open(utc("2029-03-01T15:30:00Z")), Ok(true));
    assert_eq!(overlaid.is_open(utc("2029-03-01T19:00:00Z")), Ok(false));
    assert_eq!(overlaid.is_open(utc("2029-03-02T19:00:00Z")), Ok(true));
}

#[test]
fn the_reported_spans_agree_with_the_verdict_date_by_date() {
    for calendar in every_calendar() {
        let coverage = calendar.with_last_known_week().coverage();
        let complete: Vec<_> = coverage.complete_ranges().collect();
        let gaps: Vec<_> = coverage.gaps().collect();
        let mut day = date(2025, 1, 1);
        while day <= date(2030, 1, 1) {
            let in_complete = complete.iter().any(|range| range.contains(day));
            let gap = gaps.iter().find(|gap| gap.range().contains(day));
            assert_eq!(
                in_complete,
                gap.is_none(),
                "{:?} on {day} must lie in exactly one span",
                coverage.identity()
            );
            let verdict = coverage.coverage_on(day);
            assert_eq!(
                verdict == DateCoverage::Covered,
                in_complete,
                "{:?} on {day}",
                coverage.identity()
            );
            if verdict == DateCoverage::NormalWeekOnly {
                assert_eq!(
                    gap.map(|gap| gap.reason()),
                    Some(CoverageGapReason::NormalWeekOnly),
                    "{:?} on {day}",
                    coverage.identity()
                );
            }
            day = day.succ_opt().expect("fixture dates are representable");
        }
    }
}

#[test]
fn a_date_the_view_calls_answered_is_never_refused_for_another_answered_date() {
    for calendar in every_calendar() {
        let view = calendar.with_last_known_week();
        let coverage = view.coverage();
        let mut day = date(2026, 10, 1);
        while day <= date(2029, 2, 1) {
            let answered = |date| {
                matches!(
                    coverage.coverage_on(date),
                    DateCoverage::Covered | DateCoverage::NormalWeekOnly
                )
            };
            if answered(day) {
                let at = day.and_hms_opt(14, 7, 0).expect("14:07 exists").and_utc();
                for answer in answers(view, at) {
                    if let Err(error) = answer
                        && !matches!(error, CalendarQueryError::SearchExhausted { .. })
                    {
                        assert!(
                            !answered(error.date()),
                            "{:?} at {at} refused naming {}",
                            coverage.identity(),
                            error.date()
                        );
                    }
                }
            }
            day += Duration::days(9);
        }
    }
}

#[test]
fn every_identity_with_no_table_answers_above_its_sourced_week() {
    let mut checked = 0;
    for calendar in every_calendar() {
        let view = calendar.with_last_known_week();
        if view.coverage().holiday_contract() != HolidayContract::NoTable {
            continue;
        }
        for at in [
            utc("2026-11-02T15:00:00Z"),
            utc("2028-03-01T03:00:00Z"),
            utc("2030-10-12T12:00:00Z"),
        ] {
            let day = at.date_naive();
            if day < view.coverage().sourced_normal_week().first() {
                continue;
            }
            assert_eq!(
                view.coverage().coverage_on(day),
                DateCoverage::NormalWeekOnly,
                "{:?} on {day}",
                view.source()
            );
            assert_eq!(
                view.is_open(at),
                Ok(view.hours_at(at).is_open(at)),
                "{:?} at {at}",
                view.source()
            );
            checked += 1;
        }
    }
    assert!(checked > 0, "no identity without a table was checked");
}
