// SPDX-License-Identifier: MIT-0

//! Caller-supplied trade-date policy and state contracts over the public API.
//!
//! **Coverage (Stage 2B, LAW-COVERAGE).** An identity-backed date-aware query
//! refuses a date the identity cannot source: before the permanent 2025 floor
//! (`BeforeSupportFloor`), outside the ranges it has a sourced answer for
//! (`OutsideCoveredRange`), on a date it withholds (`UnresolvedGap`), or when a
//! bounded search runs out on a day it cannot establish (`SearchExhausted`).
//! `Exchange::SetThailand` and the Singapore scopes ship no holiday table, and
//! the CME scopes withhold the Sunday 16:00-16:15 CT queue (#79), so those
//! refusals are the answers this file has to state; where a query still answers,
//! its original claim is kept verbatim. A caller's `DayPolicy` can only tighten
//! a day — it never licenses a date the identity does not answer.

#![expect(
    clippy::expect_used,
    reason = "fixture constructors assert their own literals; a bad literal must fail the test"
)]

use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Utc};
use chrono_tz::{America, Asia, US};
use exchange_hours::{
    CalendarQueryError, CalendarResolution, CalendarSource, DateCoverage, DayPolicy, Exchange,
    ExchangeCalendar, HolidayKind, MarketHoursKey, NoPolicy, SUPPORT_FLOOR, SessionKind,
    SessionState, calendar_for_exchange, calendar_for_market_hours_key, hours_for_market_hours_key,
};

/// Asserts an identity-backed query returns exactly `expected`, the coverage
/// error the shipped data declares. The variant, identity and date are all part
/// of the contract: a refusal that named the wrong day would be as wrong as an
/// answer.
fn assert_refusal<T: std::fmt::Debug>(
    answer: Result<T, CalendarQueryError>,
    expected: CalendarQueryError,
    label: &str,
) {
    let error = answer.expect_err(&format!(
        "{label}: expected the coverage refusal {expected:?}"
    ));
    assert_eq!(
        error, expected,
        "{label}: the query must state the refusal its identity declares"
    );
}

/// Asserts a query refuses `date` because the identity has no sourced answer for
/// it at or above the floor.
fn assert_outside_coverage<T: std::fmt::Debug>(
    answer: Result<T, CalendarQueryError>,
    source: CalendarSource,
    date: NaiveDate,
    label: &str,
) {
    assert_refusal(
        answer,
        CalendarQueryError::OutsideCoveredRange { source, date },
        label,
    );
}

/// Asserts a refusal is exactly the one `calendar` publishes for the day the
/// query named, so a query verdict and the identity's own coverage metadata can
/// never disagree.
///
/// Driving the expectation from
/// [`exchange_hours::CalendarCoverage::coverage_on`] keeps a many-date sweep
/// honest: the sweep states the verdict the shipped data declares rather than a
/// hand-copied list of dates. `SearchExhausted` is the one refusal that is not a
/// per-date verdict — a bounded walk ran out — so it is checked for the identity
/// and for the documented shape of its report (`date` is the day the walk
/// stopped on, and `bound` equals it).
fn assert_published_refusal(error: CalendarQueryError, calendar: ExchangeCalendar, label: &str) {
    assert_eq!(
        error.source(),
        calendar.source(),
        "{label}: the refusal names the wrong identity"
    );
    let date = error.date();
    let verdict = calendar.coverage().coverage_on(date);
    // `DateCoverage` is `#[non_exhaustive]`. A verdict this file does not know
    // cannot be mapped to a refusal, and comparing the error with itself would
    // fence nothing, so it fails here, loudly, before the match below.
    assert!(
        matches!(
            verdict,
            DateCoverage::Covered
                | DateCoverage::NormalWeekOnly
                | DateCoverage::BeforeSupportFloor
                | DateCoverage::OutsideCoveredRange
                | DateCoverage::UnresolvedGap
        ),
        "{label}: unrecognised coverage verdict {verdict:?} for {date}, got {error:?}"
    );
    let declared = match verdict {
        DateCoverage::OutsideCoveredRange => Some(CalendarQueryError::OutsideCoveredRange {
            source: calendar.source(),
            date,
        }),
        DateCoverage::UnresolvedGap => Some(CalendarQueryError::UnresolvedGap {
            source: calendar.source(),
            date,
        }),
        DateCoverage::BeforeSupportFloor => Some(CalendarQueryError::BeforeSupportFloor {
            source: calendar.source(),
            date,
        }),
        DateCoverage::Covered | DateCoverage::NormalWeekOnly | _ => None,
    };
    match declared {
        Some(expected) => assert_eq!(
            error, expected,
            "{label}: the query must state the coverage verdict its identity publishes"
        ),
        None => assert!(
            matches!(
                error,
                CalendarQueryError::SearchExhausted { date: stopped, bound, .. } if stopped == bound
            ),
            "{label}: only a bounded search may refuse a date the identity declares              complete, and it reports the day it stopped on as its bound, got {error:?}"
        ),
    }
}

fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be a valid CT instant")
        .with_timezone(&Utc)
}

fn et(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    America::New_York
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be a valid ET instant")
        .with_timezone(&Utc)
}

fn sgt(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    Asia::Singapore
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be a valid SGT instant")
        .with_timezone(&Utc)
}

fn bangkok(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    Asia::Bangkok
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be a valid Bangkok instant")
        .with_timezone(&Utc)
}

fn day(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("fixture must be a valid date")
}

struct TestPolicy<'a> {
    closed: &'a [NaiveDate],
    early: Option<(NaiveDate, u32)>,
    late: Option<(NaiveDate, u32)>,
}

impl DayPolicy for TestPolicy<'_> {
    fn is_closed(&self, trade_date: NaiveDate) -> bool {
        self.closed.contains(&trade_date)
    }

    fn early_close_ssm(&self, trade_date: NaiveDate) -> Option<u32> {
        self.early
            .filter(|(date, _ssm)| *date == trade_date)
            .map(|(_date, ssm)| ssm)
    }

    fn late_open_ssm(&self, trade_date: NaiveDate) -> Option<u32> {
        self.late
            .filter(|(date, _ssm)| *date == trade_date)
            .map(|(_date, ssm)| ssm)
    }
}

struct Closed;

impl DayPolicy for Closed {
    fn is_closed(&self, _trade_date: NaiveDate) -> bool {
        true
    }

    fn early_close_ssm(&self, _trade_date: NaiveDate) -> Option<u32> {
        None
    }
}

#[test]
fn calendar_identity_distinguishes_exchanges_from_product_families() {
    let exchange = calendar_for_exchange(Exchange::Cme);
    assert_eq!(exchange.exchange(), Some(Exchange::Cme));
    assert_eq!(exchange.market_hours_key(), None);

    let key = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    assert_eq!(key.exchange(), None);
    assert_eq!(
        key.market_hours_key(),
        Some(MarketHoursKey::GlobexEquityIndex)
    );
}

#[test]
fn no_policy_preserves_the_complete_calendar_surface() {
    let base = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let policy = base.with_day_policy(&NoPolicy);

    for instant in [
        ct((2026, 4, 19), (18, 0, 0)),
        ct((2026, 4, 20), (10, 0, 0)),
        ct((2026, 4, 20), (15, 20, 0)),
        ct((2026, 4, 20), (16, 30, 0)),
    ] {
        assert_eq!(policy.hours_at(instant), base.hours_at(instant));
        // `NoPolicy` preserves the complete surface, refusals included: the
        // equality is asserted over the `Result`s rather than over unwrapped
        // answers, so an identical coverage verdict still proves the layer
        // changed nothing (LAW-COVERAGE).
        assert_eq!(policy.is_open(instant), base.is_open(instant));
        assert_eq!(policy.session_bounds(instant), base.session_bounds(instant));
        assert_eq!(policy.session_state(instant), base.session_state(instant));
        assert_eq!(policy.trade_date(instant), base.trade_date(instant));
        assert_eq!(
            policy.candle_end(instant, CalendarResolution::Daily),
            base.candle_end(instant, CalendarResolution::Daily)
        );
    }

    // 2026-04-20 16:30 CT is the daily halt: the state is the sourced
    // 16:00-16:45 CT maintenance gap, and the trade date is absent, with or
    // without the layer — a weekday of the bracket era answers from the tables
    // (#172), and the layer changed no answer above.
    let after_the_close = ct((2026, 4, 20), (16, 30, 0));
    assert_eq!(
        policy.session_state(after_the_close),
        base.session_state(after_the_close),
        "the state after the close: the layer changed nothing"
    );
    assert_eq!(
        policy.trade_date(after_the_close),
        base.trade_date(after_the_close),
        "the trade date after the close: the layer changed nothing"
    );
}

#[test]
fn closed_trade_date_removes_the_prior_evening_but_not_the_next_trade_date() {
    let monday = day(2026, 4, 20);
    let closed = [monday];
    let policy = TestPolicy {
        closed: &closed,
        early: None,
        late: None,
    };
    let base = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let calendar = base.with_day_policy(&policy);

    assert!(
        !calendar
            .is_open(ct((2026, 4, 19), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2026, 4, 20), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct((2026, 4, 20), (17, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_closed_trade_date(monday, SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_closed_all_day_on(monday, SessionKind::Both)
            .expect("the coverage contract must answer a covered date"),
        "Tuesday's trade date opens during civil Monday"
    );
    assert_eq!(
        calendar
            .session_bounds(ct((2026, 4, 19), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        // Sessions now begin at the matching open. Previously this returned the
        // 16:45-17:00 pre-open queue as if it were a fifteen-minute session.
        Some((ct((2026, 4, 20), (17, 0, 0)), ct((2026, 4, 21), (8, 30, 0)),))
    );
}

#[test]
fn a_closed_friday_scan_falls_forward_to_sunday() {
    let friday = day(2026, 4, 24);
    let closed = [friday];
    let policy = TestPolicy {
        closed: &closed,
        early: None,
        late: None,
    };
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex).with_day_policy(&policy);
    assert_eq!(
        calendar
            .next_session_open_after(ct((2026, 4, 23), (16, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2026, 4, 26), (17, 0, 0)))
    );
}

#[test]
fn closed_crypto_monday_rolls_weekend_into_the_following_business_day() {
    let monday = day(2026, 6, 8);
    let tuesday = day(2026, 6, 9);
    let closed = [monday];
    let policy = TestPolicy {
        closed: &closed,
        early: None,
        late: None,
    };
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency)
        .with_day_policy(&policy);

    for instant in [
        ct((2026, 6, 5), (17, 0, 0)),
        ct((2026, 6, 6), (1, 0, 0)),
        ct((2026, 6, 6), (5, 0, 0)),
        ct((2026, 6, 7), (12, 0, 0)),
        ct((2026, 6, 8), (10, 0, 0)),
    ] {
        assert!(
            calendar
                .is_open(instant)
                .expect("the coverage contract must answer a covered date"),
            "weekend trading vanished at {instant}"
        );
        assert_eq!(
            calendar
                .trade_date(instant)
                .expect("the coverage contract must answer a covered date"),
            Some(tuesday)
        );
    }
    assert!(
        calendar
            .is_closed_trade_date(monday, SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct((2026, 6, 8), (16, 2, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .trade_date(ct((2026, 6, 8), (16, 2, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(tuesday)
    );
    assert_eq!(
        calendar
            .candle_start(ct((2026, 6, 7), (12, 0, 0)), CalendarResolution::Daily,)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2026, 6, 5), (16, 2, 0)))
    );
    assert_eq!(
        calendar
            .candle_end(ct((2026, 6, 7), (12, 0, 0)), CalendarResolution::Daily,)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2026, 6, 9), (16, 0, 0)))
    );
    assert_eq!(
        calendar
            .candle_start(ct((2026, 6, 7), (12, 0, 0)), CalendarResolution::Weekly,)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2026, 6, 5), (16, 2, 0)))
    );
    assert_eq!(
        calendar
            .candle_end(ct((2026, 6, 7), (12, 0, 0)), CalendarResolution::Weekly,)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2026, 6, 12), (16, 0, 0)))
    );

    let closed = calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency)
        .with_day_policy(&Closed);
    // An always-closed policy removes every session, so `session_bounds` still
    // answers `None`, and the trade date cannot be answered either: with the
    // policy closing every date there is no session left to carry one, so the
    // query reports the removal as `None` rather than naming a date. The
    // identity covers the day, so the answer is an answer and not a coverage
    // refusal (LAW-COVERAGE).
    let friday_evening = ct((2026, 6, 5), (17, 0, 0));
    assert_eq!(
        closed
            .session_bounds(friday_evening)
            .expect("the covered instant still answers under a policy"),
        None
    );
    assert_eq!(
        closed
            .trade_date(friday_evening)
            .expect("the covered instant still answers under a policy"),
        None,
        "an always-closed policy leaves no session to carry a trade date"
    );
}

#[test]
fn closed_set_trade_date_removes_its_day_and_after_midnight_tail() {
    let wednesday = day(2025, 5, 7);
    let closed = [wednesday];
    let policy = TestPolicy {
        closed: &closed,
        early: None,
        late: None,
    };
    let calendar = calendar_for_exchange(Exchange::SetThailand).with_day_policy(&policy);

    // `Exchange::SetThailand` ships no holiday table, so it declares no sourced
    // answer for any date at or above the 2025 floor and refuses each of the
    // probes below rather than reading a policy result as a market state
    // (LAW-COVERAGE). What the policy does to the Thai trade date — including
    // the after-midnight tail the test is named for — is therefore not
    // observable through this identity; the policy's own record is asserted
    // where the type is exercised (see `tests/static_day_policy.rs`).
    let thailand = CalendarSource::Exchange(Exchange::SetThailand);
    assert_outside_coverage(
        calendar.is_open_extended(bangkok((2025, 5, 7), (2, 50, 0))),
        thailand,
        day(2025, 5, 7),
        "the after-midnight tail",
    );
    assert_outside_coverage(
        calendar.trade_date(bangkok((2025, 5, 7), (2, 50, 0))),
        thailand,
        day(2025, 5, 7),
        "the tail's trade date",
    );
    assert_outside_coverage(
        calendar.is_open(bangkok((2025, 5, 7), (12, 0, 0))),
        thailand,
        day(2025, 5, 7),
        "the closed daytime session",
    );
    assert_outside_coverage(
        calendar.is_open(bangkok((2025, 5, 7), (19, 0, 0))),
        thailand,
        day(2025, 5, 7),
        "the closed evening session",
    );
    assert_outside_coverage(
        calendar.is_open(bangkok((2025, 5, 8), (2, 50, 0))),
        thailand,
        day(2025, 5, 8),
        "the following night's tail",
    );
    assert_outside_coverage(
        calendar.is_open(bangkok((2025, 5, 8), (10, 0, 0))),
        thailand,
        day(2025, 5, 8),
        "the following day's session",
    );
    // The refusal names the queried day itself: the daily-close derivation
    // answers a question *about* that day, so the day is gated at the entry
    // (issue #107's per-occurrence window removed the accidental path that
    // first touched the day the walk backed into). The identity's metadata
    // declares no answer for 2025-05-07 either way.
    assert_outside_coverage(
        calendar.is_closed_trade_date(wednesday, SessionKind::Both),
        thailand,
        day(2025, 5, 7),
        "the closed trade date's own close",
    );
}

#[test]
fn early_close_clamps_sessions_candles_and_state() {
    let monday = day(2026, 4, 20);
    let policy = TestPolicy {
        closed: &[],
        early: Some((monday, 12 * 3_600 + 15 * 60)),
        late: None,
    };
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex).with_day_policy(&policy);

    assert!(
        calendar
            .is_open_regular(ct((2026, 4, 20), (12, 14, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2026, 4, 20), (12, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .candle_end(ct((2026, 4, 19), (18, 0, 0)), CalendarResolution::Daily,)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2026, 4, 20), (12, 15, 0)))
    );
    assert_eq!(
        calendar
            .candle_end(
                ct((2026, 4, 20), (12, 14, 0)),
                CalendarResolution::Minutes(5),
            )
            .expect("the covered five-minute bar still answers"),
        Some(ct((2026, 4, 20), (12, 15, 0)))
    );
    // The state at 13:00 CT — after the policy's 12:15 close — is the sourced
    // gap between the clipped close and the 17:00 CT evening open: longer than
    // the four-hour maintenance bound and crossing into the next trade date,
    // so it classifies as `Closed`. The clamp itself is asserted above by the
    // regular-session and bar probes.
    assert_eq!(
        calendar.session_state(ct((2026, 4, 20), (13, 0, 0))),
        Ok(SessionState::Closed),
        "the state after the policy's early close is the sourced gap, not a refusal"
    );
}

#[test]
fn early_close_on_the_last_trade_date_clamps_weekly_and_monthly_bars() {
    let friday = day(2026, 5, 29);
    let policy = TestPolicy {
        closed: &[],
        early: Some((friday, 12 * 3_600 + 15 * 60)),
        late: None,
    };
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex).with_day_policy(&policy);
    let expected = ct((2026, 5, 29), (12, 15, 0));

    assert_eq!(
        calendar
            .candle_end(ct((2026, 5, 24), (18, 0, 0)), CalendarResolution::Weekly,)
            .expect("the coverage contract must answer a covered date"),
        Some(expected)
    );
    assert_eq!(
        calendar
            .candle_end(ct((2026, 5, 1), (10, 0, 0)), CalendarResolution::Monthly,)
            .expect("the coverage contract must answer a covered date"),
        Some(expected)
    );
}

#[test]
fn cme_good_friday_closed_reference_case_uses_caller_data() {
    // CME's 2014 advisory states that Globex had its regular Thursday close,
    // was closed Friday April 18, and resumed normal hours Sunday April 20.
    // This fixture proves the overlay; the crate intentionally ships no
    // holiday calendar.
    // https://www.cmegroup.com/tools-information/lookups/advisories/clearing/files/Chadv14-136.pdf
    let good_friday = day(2014, 4, 18);
    let closed = [good_friday];
    let policy = TestPolicy {
        closed: &closed,
        early: None,
        late: None,
    };
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex).with_day_policy(&policy);

    // At the 2010 floor the 2014 date answers again, so the caller policy is
    // observable once more: the crate ships no 2014 Good Friday row, the
    // admissible evidence states the closure, and the caller's record is what
    // makes the day read closed.
    assert!(
        !calendar
            .is_open(ct((2014, 4, 18), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the closed Friday"
    );
    assert!(
        calendar
            .is_closed_trade_date(good_friday, SessionKind::Both)
            .expect("the coverage contract must answer a covered date"),
        "the closed trade date"
    );
    assert!(
        calendar
            .is_closed_all_day_on(good_friday, SessionKind::Both)
            .expect("the coverage contract must answer a covered date"),
        "the closed all-day window"
    );
    // The scan skips the caller-closed Friday and lands on the Sunday-evening
    // reopen the advisory states.
    assert_eq!(
        calendar
            .next_session_open_after(ct((2014, 4, 17), (16, 15, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2014, 4, 20), (17, 0, 0))),
        "the scan to the next session"
    );

    // What the fixed snapshot still states: it carries no identity and claims no
    // coverage, so it answers its supplied rules — here the ordinary week the
    // advisory's closure removes, unchanged.
    let good_friday_morning = ct((2014, 4, 18), (10, 0, 0));
    assert!(
        hours_for_market_hours_key(MarketHoursKey::GlobexEquityIndex, good_friday_morning)
            .is_open(good_friday_morning),
        "the fixed snapshot still states the ordinary Friday session"
    );
}

#[test]
fn cme_christmas_eve_and_post_thanksgiving_close_at_12_15() {
    // These are operator-published Globex fixtures. The Thanksgiving source
    // makes the important distinction explicit: Wednesday was normal and the
    // 12:15 CT equity close was Friday, the day after Thanksgiving.
    // https://www.cmegroup.com/tools-information/holiday-calendar/files/2015-thanksgiving-holiday-schedule.pdf
    // https://www.cmegroup.com/tools-information/holiday-calendar/files/2015-christmas-holiday-schedule.pdf
    let post_thanksgiving = day(2015, 11, 27);
    let thanksgiving_policy = TestPolicy {
        closed: &[],
        early: Some((post_thanksgiving, 12 * 3_600 + 15 * 60)),
        late: None,
    };
    let thanksgiving = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
        .with_day_policy(&thanksgiving_policy);
    // At the 2010 floor the 2015 dates answer again, so the notice's 12:15 CT
    // close is observable through the caller's policy: the daily bar for the
    // Thanksgiving trade date ends at the clipped Friday close, and the
    // shrunken session is end-exclusive at it.
    assert_eq!(
        thanksgiving
            .candle_end(ct((2015, 11, 26), (18, 0, 0)), CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2015, 11, 27), (12, 15, 0))),
        "the post-Thanksgiving daily bar"
    );
    assert!(
        !thanksgiving
            .is_open(ct((2015, 11, 27), (12, 15, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the shrunken post-Thanksgiving session is end-exclusive"
    );

    let christmas_eve = day(2015, 12, 24);
    let christmas_day = day(2015, 12, 25);
    let christmas_closed = [christmas_day];
    let christmas_policy = TestPolicy {
        closed: &christmas_closed,
        early: Some((christmas_eve, 12 * 3_600 + 15 * 60)),
        late: None,
    };
    let christmas = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
        .with_day_policy(&christmas_policy);
    // Both Christmas probes resolve a pre-floor day the identity cannot source,
    // At the 2010 floor both Christmas probes answer through the caller's
    // policy: the Eve's daily bar ends at the clipped 12:15 CT close, and the
    // scan past Christmas jumps the caller-closed Christmas Day to the
    // Sunday-evening reopen.
    assert_eq!(
        christmas
            .candle_end(ct((2015, 12, 23), (18, 0, 0)), CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2015, 12, 24), (12, 15, 0))),
        "the Christmas Eve bar"
    );
    assert_eq!(
        christmas
            .next_session_open_after(ct((2015, 12, 24), (12, 15, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2015, 12, 27), (17, 0, 0))),
        "the scan past Christmas"
    );
}

#[test]
fn cme_mlk_and_presidents_day_close_at_noon_then_reopen_at_five() {
    // CME's live 2026 product table gives equities a 12:00 CT holiday close
    // and 17:00 CT reopen on both dates. These are caller-policy fixtures, not
    // a built-in or prospective holiday dataset.
    // https://www.cmegroup.com/trading-hours.html#tradeDate=2026-01-19
    // https://www.cmegroup.com/trading-hours.html#tradeDate=2026-02-16
    for holiday in [day(2026, 1, 19), day(2026, 2, 16)] {
        let policy = TestPolicy {
            closed: &[],
            early: Some((holiday, 12 * 3_600)),
            late: None,
        };
        let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
            .with_day_policy(&policy);
        let previous_day = holiday
            .pred_opt()
            .expect("holiday fixture has a predecessor");

        // The daily candle ends at the **trade date's** final close, and since
        // the crate began stating CME's merged trade dates these holidays no
        // longer own one: the span from the previous evening belongs to the
        // following business day. A `DayPolicy` is keyed by trade date too, so
        // the caller's `early` above — which names the holiday — correctly clips
        // nothing here; the holiday's own noon close is now built in.
        let trade_date = holiday.succ_opt().expect("holiday fixture has a successor");

        assert_eq!(
            calendar
                .candle_end(
                    ct(
                        (
                            previous_day.year(),
                            previous_day.month(),
                            previous_day.day()
                        ),
                        (18, 0, 0),
                    ),
                    CalendarResolution::Daily,
                )
                .expect("the coverage contract must answer a covered date"),
            Some(ct(
                (trade_date.year(), trade_date.month(), trade_date.day()),
                (16, 0, 0),
            ))
        );
        assert!(
            !calendar
                .is_open(ct(
                    (holiday.year(), holiday.month(), holiday.day()),
                    (12, 0, 0),
                ))
                .expect("the coverage contract must answer a covered date")
        );
        assert_eq!(
            calendar
                .next_session_open_after(ct(
                    (holiday.year(), holiday.month(), holiday.day()),
                    (12, 0, 0),
                ))
                .expect("the coverage contract must answer a covered date"),
            // 16:45 is the pre-open queue, not a session; matching resumes at
            // 17:00 and that is where the next session opens.
            Some(ct(
                (holiday.year(), holiday.month(), holiday.day()),
                (17, 0, 0),
            ))
        );
        assert!(
            calendar
                .is_open_extended(ct(
                    (holiday.year(), holiday.month(), holiday.day()),
                    (17, 0, 0),
                ))
                .expect("the coverage contract must answer a covered date")
        );
    }
}

#[test]
fn late_open_clips_every_earlier_phase_of_the_trade_date() {
    let monday = day(2026, 4, 20);
    let policy = TestPolicy {
        closed: &[],
        early: None,
        late: Some((monday, 9 * 3_600 + 30 * 60)),
    };
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex).with_day_policy(&policy);

    assert!(
        !calendar
            .is_open(ct((2026, 4, 19), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2026, 4, 20), (9, 29, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open_regular(ct((2026, 4, 20), (9, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .trade_date(ct((2026, 4, 20), (9, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(monday)
    );
}

#[test]
fn policy_scans_are_bounded_and_hours_at_is_unmodified() {
    let base = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let instant = ct((2026, 4, 20), (10, 0, 0));
    let calendar = base.with_day_policy(&Closed);
    assert_eq!(
        calendar
            .session_bounds(instant)
            .expect("the coverage contract must answer a covered date"),
        None
    );
    assert_eq!(
        calendar
            .next_session_after(instant)
            .expect("the coverage contract must answer a covered date"),
        None
    );
    assert_eq!(calendar.hours_at(instant), base.hours_at(instant));
}

#[test]
fn policy_does_not_invent_trade_dates_for_an_always_open_profile() {
    let monday = day(2026, 4, 20);
    let closed = [monday];
    let policy = TestPolicy {
        closed: &closed,
        early: Some((monday, 12 * 3_600)),
        late: Some((monday, 9 * 3_600)),
    };
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::AlwaysOpen).with_day_policy(&policy);

    for instant in [
        ct((2026, 4, 19), (12, 0, 0)),
        ct((2026, 4, 20), (0, 0, 0)),
        ct((2026, 4, 20), (12, 0, 0)),
    ] {
        // The profile is continuously open, so a policy that closes a trade
        // date cannot remove the session: `is_open` still answers, and the
        // policy does not close a market that never closes.
        assert!(
            calendar
                .is_open(instant)
                .expect("a continuously open profile still answers"),
            "the policy closed a session the profile does not have"
        );
        // It does not invent a trade date either. There is none to name, and the
        // bounded walk that would find one runs out at its documented bound, so
        // the answer is the explicit `SearchExhausted` refusal — never
        // `Ok(Some(..))`, which is what "does not invent" means under
        // LAW-COVERAGE. The bound is the day the walk stopped on.
        let answer = calendar.trade_date(instant);
        assert!(
            matches!(
                answer,
                Err(CalendarQueryError::SearchExhausted { date: stopped, bound, .. })
                    if stopped == bound
            ),
            "an always-open profile must not invent a trade date, got {answer:?}"
        );
    }
    assert!(
        calendar
            .is_closed_trade_date(monday, SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_closed_all_day_on(monday, SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
}

#[test]
fn invalid_policy_seconds_fail_closed_for_that_trade_date() {
    let monday = day(2026, 4, 20);
    let invalid_early = TestPolicy {
        closed: &[],
        early: Some((monday, 86_401)),
        late: None,
    };
    let invalid_late = TestPolicy {
        closed: &[],
        early: None,
        late: Some((monday, 86_400)),
    };
    let base = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);

    for policy in [&invalid_early, &invalid_late] {
        let calendar = base.with_day_policy(policy);
        assert!(
            !calendar
                .is_open(ct((2026, 4, 19), (18, 0, 0)))
                .expect("the coverage contract must answer a covered date")
        );
        assert!(
            !calendar
                .is_open(ct((2026, 4, 20), (10, 0, 0)))
                .expect("the coverage contract must answer a covered date")
        );
        assert!(
            calendar
                .is_closed_trade_date(monday, SessionKind::Both)
                .expect("the coverage contract must answer a covered date")
        );
    }
}

#[test]
fn next_session_scan_includes_day_fourteen_and_excludes_day_fifteen() {
    let calendar = calendar_for_exchange(Exchange::NyseNational);
    // This identity ships no holiday table, so at the 2010 floor every date is
    // carried-or-unaudited at the date level and the forward scan refuses
    // naming the first day it cannot source — a refusal, never a `None`
    // (LAW-COVERAGE). Whether the fourteenth candidate day is inside the scan
    // window and the fifteenth outside it is fenced by the bound the scan
    // names: 2018-05-07 is day fourteen of the walk from 2018-04-14, and the
    // error names it rather than a fifteenth day.
    let nyse_national = CalendarSource::Exchange(Exchange::NyseNational);
    assert_outside_coverage(
        calendar.next_session_open_after(et((2018, 5, 7), (7, 0, 0))),
        nyse_national,
        day(2018, 5, 7),
        "the scan from the first Monday",
    );
    assert_outside_coverage(
        calendar.next_session_open_after(et((2018, 5, 6), (7, 0, 0))),
        nyse_national,
        day(2018, 5, 6),
        "the scan from the Sunday before it",
    );
}

#[test]
fn trade_dates_and_states_cover_the_globex_day() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let monday = day(2026, 4, 20);

    assert_eq!(
        calendar
            .trade_date(ct((2026, 4, 19), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(monday)
    );
    assert_eq!(
        calendar
            .trade_date(ct((2026, 4, 20), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(monday)
    );
    assert_eq!(
        calendar
            .trade_date(ct((2026, 4, 20), (15, 20, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(monday)
    );
    // 16:30 CT is the daily halt: no session and no queue holds the instant,
    // so the trade date is absent and the state is the sourced 16:00-16:45 CT
    // maintenance gap — a weekday of the bracket era answers from the tables
    // (#172). The halt's own shape is stated beside it — the extended state at
    // 15:20 above and the daily bounds.
    assert_eq!(
        calendar.trade_date(ct((2026, 4, 20), (16, 30, 0))),
        Ok(None),
        "the trade date at the daily halt is absent, not refused"
    );
    assert_eq!(
        calendar
            .session_state(ct((2026, 4, 20), (10, 0, 0)))
            .expect("the covered regular session still answers"),
        SessionState::OpenRegular
    );
    assert_eq!(
        calendar
            .session_state(ct((2026, 4, 19), (18, 0, 0)))
            .expect("the covered wrapping session still answers"),
        SessionState::OpenExtended
    );
    assert_eq!(
        calendar
            .session_state(ct((2026, 4, 20), (15, 20, 0)))
            .expect("the covered extended session still answers"),
        SessionState::OpenExtended
    );
    assert_eq!(
        calendar.session_state(ct((2026, 4, 20), (16, 30, 0))),
        Ok(SessionState::Maintenance),
        "the state at the daily halt is the sourced 16:00-16:45 CT gap"
    );
    // The Saturday state is derived from the surrounding sessions, which the
    // sourced grid answers: the weekend closes and the Sunday queue opens it
    // again, so the state answers `Closed` rather than refusing (#172 — the
    // withheld quarter-hour is the Sunday 16:00-16:15 slice alone).
    assert_eq!(
        calendar.session_state(ct((2026, 4, 25), (12, 0, 0))),
        Ok(SessionState::Closed),
        "the weekend state answers from the sourced grid"
    );
    // The bracket-era Sunday's own quarter-hour is the one instant class the
    // identity refuses: the state there states the coverage verdict rather
    // than reading as a closed grid.
    let globex = CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEquityIndex);
    assert_outside_coverage(
        calendar.session_state(ct((2026, 4, 19), (16, 5, 0))),
        globex,
        day(2026, 4, 19),
        "the withheld quarter-hour on its bracket-era Sunday",
    );

    let livestock = calendar_for_market_hours_key(MarketHoursKey::GlobexLivestock);
    assert_eq!(
        livestock
            .trade_date(ct((2026, 4, 20), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(monday)
    );
    assert_eq!(
        livestock
            .trade_date(ct((2026, 4, 25), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        None
    );

    assert_eq!(
        calendar
            .trade_date(ct((2026, 3, 8), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 3, 9)),
        "the Central DST transition does not change the Globex trade date"
    );

    let cfe = calendar_for_market_hours_key(MarketHoursKey::CfeVix);
    assert_eq!(
        cfe.trade_date(ct((2026, 4, 19), (16, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(monday),
        "the Sunday CFE order-entry queue belongs to Monday"
    );
    assert_eq!(
        cfe.trade_date(ct((2026, 4, 20), (16, 50, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 4, 21)),
        "the Monday-evening CFE queue belongs to Tuesday"
    );

    // The Singapore scopes ship no holiday table, so every post-floor date is
    // unsourced for them: the T+1 phase claim is no longer observable and both
    // probes refuse. The same claim is stated for `CfeVix` above, whose coverage
    // is complete.
    let sgx = calendar_for_market_hours_key(MarketHoursKey::Sgx);
    let sgx_source = CalendarSource::MarketHoursKey(MarketHoursKey::Sgx);
    assert_outside_coverage(
        sgx.trade_date(sgt((2026, 4, 20), (10, 0, 0))),
        sgx_source,
        monday,
        "the SGX daytime trade date",
    );
    assert_outside_coverage(
        sgx.trade_date(sgt((2026, 4, 20), (22, 0, 0))),
        sgx_source,
        monday,
        "the SGX T+1 trade date",
    );
}

#[test]
fn session_state_and_trade_date_are_consistent_for_every_key() {
    let mut instant = Utc
        .with_ymd_and_hms(2026, 4, 19, 0, 0, 0)
        .single()
        .expect("fixture must have a valid start");
    let end = instant + Duration::days(7);

    while instant < end {
        for &key in MarketHoursKey::ALL {
            let calendar = calendar_for_market_hours_key(key);
            let label = format!("{key} at {instant}");
            // The split is the coverage line (LAW-COVERAGE): where the identity
            // answers the instant, the consistency this fence is about is
            // claimable exactly as before; where it declares no answer for a day
            // the instant needs — a withheld phase, an unsourced span, or a
            // bounded walk that ran out — the query refuses, and the refusal is
            // asserted to be the one the identity publishes rather than being
            // treated as a state.
            let state = match calendar.session_state(instant) {
                Ok(state) => state,
                Err(error) => {
                    assert_published_refusal(error, calendar, &label);
                    continue;
                }
            };
            assert_eq!(
                calendar
                    .is_maintenance(instant)
                    .expect("a state that answered answers its maintenance case too"),
                state == SessionState::Maintenance,
                "{label}"
            );
            if key == MarketHoursKey::AlwaysOpen {
                // A continuously open profile has no trade date to name, and the
                // walk that would find one runs out at its documented bound: the
                // refusal is the settled answer, and it never invents a date.
                let answer = calendar.trade_date(instant);
                assert!(
                    matches!(
                        answer,
                        Err(CalendarQueryError::SearchExhausted { date: stopped, bound, .. })
                            if stopped == bound
                    ),
                    "{label}: an always-open profile must not invent a trade date, got {answer:?}"
                );
            } else {
                // A trade date exists wherever the venue is doing business -
                // including an order-entry phase, which carries the trade date
                // of the session it feeds.
                assert_eq!(
                    calendar
                        .trade_date(instant)
                        .expect("a state that answered answers its trade date too")
                        .is_some(),
                    calendar
                        .is_accepting_orders(instant)
                        .expect("a state that answered answers order acceptance too"),
                    "{label}"
                );
            }
            match state {
                SessionState::OpenRegular => {
                    assert!(
                        calendar
                            .is_open_regular(instant)
                            .expect("a state that answered answers the phase too"),
                        "{label}"
                    );
                }
                SessionState::OpenExtended => {
                    assert!(
                        !calendar
                            .is_open_regular(instant)
                            .expect("a state that answered answers the phase too"),
                        "{label}"
                    );
                    assert!(
                        calendar
                            .is_open_extended(instant)
                            .expect("a state that answered answers the phase too"),
                        "{label}"
                    );
                }
                SessionState::OrderEntry => {
                    // Nothing matches, so the market is not open - but orders
                    // are accepted, which is the whole point of the phase.
                    assert!(
                        !calendar
                            .is_open(instant)
                            .expect("a state that answered answers the phase too"),
                        "{label}"
                    );
                    assert!(
                        calendar
                            .is_accepting_orders(instant)
                            .expect("a state that answered answers order acceptance too"),
                        "{label}: OrderEntry state must accept orders"
                    );
                }
                SessionState::Maintenance | SessionState::Closed => {
                    assert!(
                        !calendar
                            .is_open(instant)
                            .expect("a state that answered answers the phase too"),
                        "{label}"
                    );
                }
                // SessionState is #[non_exhaustive]; a future state must not
                // silently satisfy this assertion.
                _ => panic!("unhandled SessionState variant at {instant} for {key}"),
            }
        }
        instant += Duration::hours(1);
    }
}

#[test]
fn all_key_calendars_match_dated_snapshots_over_two_years() {
    let instant = Utc
        .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
        .single()
        .expect("fixture must have a valid start");
    let end = Utc
        .with_ymd_and_hms(2024, 1, 1, 0, 0, 0)
        .single()
        .expect("fixture must have a valid end");

    // At the 2010 floor the 2022-2023 window answers again, so the hourly
    // sweep asserts the coverage contract itself: no probe may read as a
    // pre-floor refusal any more, every query's answer must agree with the
    // identity's own date-level verdict — answered exactly where the identity
    // sources its facts, refused with the matching error where it does not —
    // and the identity-erased fixed snapshot keeps stating the shipped normal
    // week beside it.
    let mut instant = instant;
    let mut probes = 0_u64;
    while instant < end {
        for &key in MarketHoursKey::ALL {
            let calendar = calendar_for_market_hours_key(key);
            let local = instant.with_timezone(&calendar.tz()).date_naive();
            let label = format!("{key} at {instant}");
            assert_ne!(
                calendar.coverage().coverage_on(local),
                DateCoverage::BeforeSupportFloor,
                "{label}: {local} is at or after the support floor"
            );
            // The date-level facts decide whether the query answers: a
            // withheld date refuses as `UnresolvedGap`, a carried or
            // unaudited one as `OutsideCoveredRange`, and everything else
            // answers. Session queries ignore declared phase gaps on purpose.
            let date_level = {
                let coverage = calendar.coverage();
                if local < SUPPORT_FLOOR {
                    Some(DateCoverage::BeforeSupportFloor)
                } else {
                    match coverage.coverage_on(local) {
                        DateCoverage::UnresolvedGap => Some(DateCoverage::UnresolvedGap),
                        DateCoverage::OutsideCoveredRange => {
                            // A declared phase gap shadows the date-level
                            // verdict in the metadata; session queries answer
                            // through it, so re-derive the date-level fact.
                            // A withheld row on the date itself is one of the
                            // facts the phase gap can shadow.
                            if calendar
                                .holiday_on(local)
                                .is_some_and(|holiday| holiday.kind() == HolidayKind::Unsourced)
                            {
                                Some(DateCoverage::UnresolvedGap)
                            } else if coverage
                                .normal_week_sourced_from()
                                .is_some_and(|h| local < h)
                            {
                                Some(DateCoverage::OutsideCoveredRange)
                            } else if coverage
                                .holiday_contract()
                                .coverage()
                                .is_some_and(|c| c.contains(local))
                            {
                                // The audited window contains the date and no
                                // phase gap shadows it: the date-level facts
                                // answer.
                                None
                            } else {
                                Some(DateCoverage::OutsideCoveredRange)
                            }
                        }
                        other => Some(other),
                    }
                }
            };
            // A query whose bounded derivation needs a *neighbouring* date the
            // identity does not source refuses naming that neighbour; that is
            // admissible on any day. A refusal naming the probe's own day must
            // match the day's date-level verdict.
            let open = calendar.is_open(instant);
            let candle = calendar.candle_end(instant, CalendarResolution::Daily);
            let open_named_other = open.as_ref().err().is_some_and(|e| e.date() != local);
            let candle_named_other = candle.as_ref().err().is_some_and(|e| e.date() != local);
            match date_level {
                None | Some(DateCoverage::NormalWeekOnly | DateCoverage::Covered) => {
                    assert!(
                        open.is_ok()
                            || (open_named_other
                                && matches!(
                                    open,
                                    Err(CalendarQueryError::UnresolvedGap { .. }
                                        | CalendarQueryError::OutsideCoveredRange { .. })
                                )),
                        "{label}: a covered day answers, or refuses naming a neighbour it needed"
                    );
                    assert!(
                        candle.is_ok()
                            || (candle_named_other
                                && matches!(
                                    candle,
                                    Err(CalendarQueryError::UnresolvedGap { .. }
                                        | CalendarQueryError::OutsideCoveredRange { .. })
                                )),
                        "{label}: a covered day answers the candle, or refuses naming a                          neighbour it needed"
                    );
                }
                Some(DateCoverage::UnresolvedGap) => {
                    assert!(
                        calendar.is_open(instant).is_err_and(|error| matches!(
                            error,
                            CalendarQueryError::UnresolvedGap { .. }
                        )),
                        "{label}: a withheld day is refused as UnresolvedGap"
                    );
                }
                Some(DateCoverage::OutsideCoveredRange) => {
                    assert!(
                        calendar.is_open(instant).is_err_and(|error| matches!(
                            error,
                            CalendarQueryError::OutsideCoveredRange { .. }
                        )),
                        "{label}: a carried or unaudited day is refused as OutsideCoveredRange"
                    );
                }
                other => panic!("{label}: unhandled verdict {other:?}"),
            }
            // The other half of the original pairing is unchanged and still
            // total: an identity-erased fixed snapshot claims no coverage, so
            // `hours_at` keeps stating the schedule beside the identity.
            assert_eq!(
                calendar.hours_at(instant),
                hours_for_market_hours_key(key, instant),
                "{label}: the identity-erased fixed snapshot still states the schedule"
            );
            probes += 1;
        }
        instant += Duration::hours(1);
    }

    assert!(probes > 0, "the sweep must have probed the window");
}
