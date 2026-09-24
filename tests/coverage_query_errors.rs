// SPDX-License-Identifier: MIT-0

//! Public-surface contracts for the coverage-error vocabulary as **queries**
//! return it (LAW-COVERAGE, Stage 2B).
//!
//! `tests/coverage_metadata.rs` pins what an identity *reports* about its own
//! coverage. This file pins what its date-aware queries *do* with that
//! knowledge: which date each verdict refuses, which error variant it raises,
//! and — just as important — which queries keep answering because a detached
//! snapshot, a normal-week contract or an overlay is not a completeness claim.
//!
//! Every assertion goes through the public API. The fixtures name identities
//! whose coverage `coverage_metadata.rs` already derives, so a query verdict
//! and a metadata verdict are read from the same source of truth.

#![expect(
    clippy::expect_used,
    reason = "fixture constructors assert their own literals; a bad literal must fail the test"
)]

use chrono::{DateTime, NaiveDate, TimeZone as _, Utc};
use chrono_tz::{Asia, US};
use exchange_hours::{
    CalendarQueryError, CalendarResolution, DateCoverage, Exchange, ExchangeCalendar, Holiday,
    MarketHours, MarketHoursKey, SUPPORT_FLOOR, SessionExceptionRecord, SessionKind, SessionState,
    StaticDayPolicy, StaticSessionExceptions, calendar_for_exchange, calendar_for_market_hours_key,
    hours_for_exchange, session_profile,
};

/// A venue-local fixture date.
fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("fixture must be a valid date")
}

/// A Chicago-local instant.
fn ct(day: (i32, u32, u32), time: (u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(day.0, day.1, day.2, time.0, time.1, 0)
        .single()
        .expect("fixture must be an unambiguous Chicago instant")
        .with_timezone(&Utc)
}

/// A Tokyo-local instant, for the opposing-zone half of the floor contract.
fn jst(day: (i32, u32, u32), time: (u32, u32)) -> DateTime<Utc> {
    Asia::Tokyo
        .with_ymd_and_hms(day.0, day.1, day.2, time.0, time.1, 0)
        .single()
        .expect("fixture must be an unambiguous Tokyo instant")
        .with_timezone(&Utc)
}

/// A complete CME equity-index family calendar.
fn globex() -> ExchangeCalendar {
    calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
}

// ---------------------------------------------------------------- the floor

#[test]
fn a_query_before_the_floor_refuses_the_local_date_it_needs() {
    let calendar = globex();
    // 2024-12-31 10:00 CT is a venue-local 2024 date, so the floor refuses it
    // even though the instant is only hours before the floor begins.
    let error = calendar
        .is_open(ct((2024, 12, 31), (10, 0)))
        .expect_err("a pre-floor instant must be refused, not answered");

    match error {
        CalendarQueryError::BeforeSupportFloor {
            source,
            date: refused,
        } => {
            assert_eq!(source, calendar.source());
            assert_eq!(refused, date(2024, 12, 31));
        }
        other => panic!("expected BeforeSupportFloor, got {other:?}"),
    }
}

#[test]
fn the_floor_is_a_local_date_so_two_zones_refuse_at_different_utc_instants() {
    // The floor is a **venue-local** date, so an instant near it is a covered
    // date for one zone and a pre-floor date for another. The first fixture
    // ships complete coverage (the metadata fence pins its span), so the
    // refusal below can only come from the floor.
    let chicago = calendar_for_market_hours_key(MarketHoursKey::GlobexGrains);

    // Singapore's 2025-01-01 00:30 is still 2024-12-31 in Chicago.
    let singapore_early = jst((2025, 1, 1), (0, 30));
    assert!(
        matches!(
            chicago.is_open(singapore_early),
            Err(CalendarQueryError::BeforeSupportFloor { .. })
        ),
        "for Chicago the instant is local 2024-12-31 and must be refused"
    );

    // A Chicago-local date inside the covered window answers. Using one identity
    // for both halves isolates the floor as the only variable: the difference is
    // the date, not the calendar.
    assert!(
        chicago
            .is_open(ct((2025, 6, 4), (10, 0)))
            .expect("a covered Chicago date answers"),
        "2025-06-04 10:00 CT is inside the grains session"
    );

    // The Singapore-keyed scope is the documented counter-example: it ships no
    // holiday table and claims none, so its coverage is empty above the floor and
    // it refuses every post-floor date rather than guessing. That is the
    // coverage-data fact `tests/coverage_metadata.rs` records, surfacing here as
    // an error exactly as LAW-COVERAGE requires.
    let singapore = calendar_for_market_hours_key(MarketHoursKey::SgxEquityIndexJapan);
    assert_eq!(
        singapore.coverage().complete_ranges().count(),
        0,
        "a no-table scope claims no complete range"
    );
    assert!(
        matches!(
            singapore.is_open(jst((2025, 6, 4), (10, 0))),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ),
        "an unsourced scope refuses rather than answering"
    );
}

#[test]
fn the_floor_answers_from_its_own_first_local_day() {
    let calendar = globex();
    // A weekday inside the first covered week answers normally.
    assert!(
        calendar
            .is_open(ct((2025, 1, 2), (10, 0)))
            .expect("a covered weekday must answer"),
        "2025-01-02 10:00 CT is inside the regular session"
    );
}

// -------------------------------------------------- the query surface agrees

#[test]
fn every_migrated_query_refuses_the_same_pre_floor_date() {
    let calendar = globex();
    let instant = ct((2024, 6, 3), (10, 0));
    let day = date(2024, 6, 3);
    let tz = calendar.tz();

    // Each of these resolves a date through the identity, so each must refuse.
    assert!(calendar.is_open(instant).is_err());
    assert!(
        calendar
            .is_open_with(instant, SessionKind::Regular)
            .is_err()
    );
    assert!(calendar.is_open_regular(instant).is_err());
    assert!(calendar.is_open_extended(instant).is_err());
    assert!(calendar.is_accepting_orders(instant).is_err());
    assert!(calendar.is_order_entry_only(instant).is_err());
    assert!(calendar.is_maintenance(instant).is_err());
    assert!(calendar.session_state(instant).is_err());
    assert!(calendar.session_bounds(instant).is_err());
    assert!(
        calendar
            .session_bounds_with(instant, SessionKind::Both)
            .is_err()
    );
    assert!(calendar.next_session_after(instant).is_err());
    assert!(
        calendar
            .next_session_after_with(instant, SessionKind::Both)
            .is_err()
    );
    assert!(calendar.next_session_open_after(instant).is_err());
    assert!(calendar.trade_date(instant).is_err());
    assert!(
        calendar
            .is_closed_trade_date(day, SessionKind::Both)
            .is_err()
    );
    assert!(
        calendar
            .is_closed_all_day_on(day, SessionKind::Both)
            .is_err()
    );
    assert!(
        calendar
            .is_closed_all_day_in_calendar(day, tz, SessionKind::Both)
            .is_err()
    );
    assert!(
        calendar
            .is_closed_all_day_at(instant, tz, SessionKind::Both)
            .is_err()
    );
    assert!(
        calendar
            .candle_end(instant, CalendarResolution::Daily)
            .is_err()
    );
    assert!(
        calendar
            .candle_start(instant, CalendarResolution::Daily)
            .is_err()
    );
    assert!(calendar.time_end_of_day(instant).is_err());
}

#[test]
fn a_refusal_is_never_a_false_or_a_none() {
    // The whole point of the migration: unsupported coverage must not be
    // readable as "the market was closed" or "there was no session".
    let calendar = globex();
    let instant = ct((2020, 3, 16), (10, 0));

    let open = calendar.is_open(instant);
    assert!(
        !matches!(open, Ok(false)),
        "a refused instant must not read as closed"
    );
    let state = calendar.session_state(instant);
    assert!(
        !matches!(state, Ok(SessionState::Closed)),
        "a refused instant must not read as closed"
    );
    let bounds = calendar.session_bounds(instant);
    assert!(
        !matches!(bounds, Ok(None)),
        "a refused instant must not read as no session"
    );
    let trade_date = calendar.trade_date(instant);
    assert!(
        !matches!(trade_date, Ok(None)),
        "a refused instant must not read as no trade date"
    );
}

// ------------------------------------------ detached snapshots keep answering

#[test]
fn a_detached_caller_snapshot_answers_before_the_floor() {
    // A fixed snapshot carries no identity, so it claims no coverage and
    // answers exactly its supplied rules — including before 2025.
    let hours: MarketHours = hours_for_exchange(Exchange::Cme, ct((2026, 1, 2), (10, 0)));
    assert!(
        hours.is_open(ct((2020, 3, 16), (10, 0))),
        "a detached snapshot keeps its no-holiday contract"
    );
    assert!(
        exchange_hours::session_bounds(&hours, ct((2020, 3, 16), (10, 0))).is_some(),
        "a detached snapshot keeps its bounded-session contract"
    );
}

#[test]
fn the_free_fixed_adapters_still_answer_before_the_floor() {
    let hours: MarketHours = hours_for_exchange(Exchange::Cme, ct((2026, 1, 2), (10, 0)));
    assert!(
        exchange_hours::session_bounds(&hours, ct((2020, 3, 16), (10, 0))).is_some(),
        "the free session_bounds adapter keeps its signature and answer"
    );
}

// ------------------------------------------------ without_holidays contract

#[test]
fn without_holidays_answers_the_normal_week_it_claims() {
    let detached = globex().without_holidays();
    // A plain covered weekday answers, because the normal-week contract is
    // complete on its own terms inside the sourced normal week.
    assert!(
        detached
            .is_open(ct((2025, 1, 2), (10, 0)))
            .expect("the normal-week contract answers a sourced weekday"),
        "2025-01-02 10:00 CT is inside the normal week"
    );
    // The floor still applies: a detached calendar does not claim coverage
    // below it, so an unsourced pre-floor date is still refused.
    assert!(
        detached.is_open(ct((2024, 12, 31), (10, 0))).is_err(),
        "detaching the holiday table does not license pre-floor dates"
    );
}

// ------------------------------------------------------- overlays and policy

#[test]
fn a_caller_policy_cannot_bypass_the_identity_floor() {
    let policy = StaticDayPolicy::new(&[]).expect("an empty policy is valid");
    let overlaid = globex().with_day_policy(&policy);
    assert!(
        overlaid.is_open(ct((2024, 12, 31), (10, 0))).is_err(),
        "no overlay may bypass the identity floor"
    );
    assert!(
        overlaid
            .is_open(ct((2025, 1, 2), (10, 0)))
            .expect("a covered weekday still answers through a policy"),
        "an empty policy leaves a covered answer unchanged"
    );
}

// ----------------------------------------------------- bounded-search bounds

#[test]
fn an_always_open_identity_has_no_daily_close_to_find() {
    // A 24x7 profile never closes, so no daily candle bound exists. The
    // answer is a settled `Ok(None)` -- not an error -- because the data
    // itself says there is no close; the walk is not what ran out.
    let calendar = calendar_for_exchange(Exchange::BinanceFutures);
    assert_eq!(
        calendar
            .candle_end(jst((2026, 3, 2), (0, 0)), CalendarResolution::Daily)
            .expect("a covered 24x7 instant is answerable"),
        None,
        "a continuously open profile has no final daily close"
    );
}

#[test]
fn a_closed_identity_answers_closed_rather_than_refusing() {
    // A known closure inside covered data is a closure, not missing evidence.
    let calendar = globex();
    let christmas = ct((2026, 12, 25), (10, 0));
    assert!(
        !calendar.is_open(christmas).expect("a covered date answers"),
        "Christmas Day is a sourced closure"
    );
    assert_eq!(
        calendar
            .session_state(christmas)
            .expect("a covered date answers"),
        SessionState::Closed
    );
}

// ------------------------------------------------- supported answers are stable

#[test]
fn a_covered_scope_still_answers_its_supported_dates() {
    let calendar = globex();
    let instant = ct((2026, 4, 20), (10, 0));

    assert!(
        calendar.is_open(instant).expect("a covered date answers"),
        "Monday 2026-04-20 10:00 CT is inside the regular session"
    );
    assert_eq!(
        calendar
            .session_state(instant)
            .expect("a covered date answers"),
        SessionState::OpenRegular
    );
    assert_eq!(
        calendar
            .trade_date(instant)
            .expect("a covered date answers"),
        Some(date(2026, 4, 20))
    );
    assert!(
        calendar
            .next_session_open_after(instant)
            .expect("a covered date answers")
            .is_some(),
        "a next session must exist inside the bounded search"
    );
    assert_eq!(
        calendar
            .session_bounds(instant)
            .expect("a covered date answers")
            .map(|(open, _close)| open),
        Some(ct((2026, 4, 20), (8, 30))),
        "the containing session opens at 08:30 CT"
    );
}

#[test]
fn the_documented_static_accessors_are_unaffected() {
    // `session_profile` resolves no date, so it stays infallible and returns
    // the family's static current table (see #77 for the seasonal exception).
    let profile = session_profile(MarketHoursKey::GlobexEquityIndex);
    assert_eq!(profile.tz, US::Central);

    // `without_holidays` and `hours_at` likewise keep their shapes.
    let calendar = globex();
    let _: ExchangeCalendar = calendar.without_holidays();
    let _: MarketHours = calendar.hours_at(ct((2024, 12, 31), (10, 0)));
    let _ = calendar.holiday_on(date(2026, 12, 25)).map(Holiday::kind);
}

#[test]
fn a_caller_exception_provider_does_not_improve_the_builtin_ledger() {
    // A provider that supplies a replacement for a pre-floor date still cannot
    // make the identity answer it: caller data never improves the ledger.
    let calendar = globex();
    let records = [SessionExceptionRecord::closed(date(2024, 6, 3))];
    let Ok(blocks) = StaticSessionExceptions::new(
        calendar.source(),
        date(2024, 1, 1),
        date(2030, 1, 1),
        &records,
    ) else {
        return;
    };
    let Ok(overlaid) = calendar.with_session_exceptions(&blocks) else {
        return;
    };
    assert!(
        overlaid.is_open(ct((2024, 6, 3), (10, 0))).is_err(),
        "a replacement record cannot license a pre-floor query"
    );
}

#[test]
fn the_support_floor_constant_is_the_documented_date() {
    assert_eq!(SUPPORT_FLOOR, date(2025, 1, 1));
}

/// The whole-date verdict and a session query answer **different questions**,
/// and both are right.
///
/// `calendar.coverage().coverage_on(day)` answers "is this date completely
/// covered?", so it is `OutsideCoveredRange` wherever a declared phase-level gap
/// applies — for the CME-family scopes that is every venue-local date before the
/// #79 bound (2026-08-22), because the Sunday 16:00-16:15 CT queue is withheld
/// across that span and the date therefore has no complete answer.
///
/// A *session* query asks something narrower: what does the sourced normal week
/// and holiday layer say about the tradeable day? That answer does not depend on
/// the withheld queue at all, so it is given rather than refused — refusing a
/// Tuesday for a Sunday queue is precisely the coverage-error-read-as-closure
/// failure LAW-COVERAGE exists to prevent (see `QueryContext::require_answerable`
/// and the order-entry-only `require_phase_coverage` beside it).
///
/// This test exists because the distinction is easy to mistake for a bug: it was
/// reported as one and diagnosed as a metadata/query contradiction before the
/// phase-gap explanation was found. Pinning both halves keeps a future change
/// from collapsing them in either direction.
#[test]
fn a_withheld_phase_refuses_the_order_entry_query_but_not_the_session_query() {
    let cal = calendar_for_exchange(Exchange::Cme);
    let day = date(2026, 4, 20);
    let instant = ct((2026, 4, 20), (10, 0));
    assert_eq!(instant.with_timezone(&cal.tz()).date_naive(), day);

    // The date is not *completely* covered: a phase-level gap applies to it.
    assert_eq!(
        cal.coverage().coverage_on(day),
        DateCoverage::OutsideCoveredRange,
        "a declared phase gap makes the whole date incomplete"
    );
    assert!(!cal.coverage().is_complete_on(day));
    assert!(
        cal.coverage()
            .phase_gaps()
            .iter()
            .any(|gap| gap.applies_on(day)),
        "the incompleteness must come from a declared phase gap, not a date-level one"
    );

    // ...yet the session query answers, because it never reads that phase.
    assert!(
        cal.is_open(instant).expect("a sourced session answers"),
        "the regular session on that date is sourced"
    );
    assert!(
        cal.normal_week_open_seconds_containing(instant).is_ok(),
        "and so is the normal week around it"
    );
}
