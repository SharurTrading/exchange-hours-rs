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
    ExchangeCalendar, MarketHoursKey, NoPolicy, SUPPORT_FLOOR, SessionKind, SessionState,
    calendar_for_exchange, calendar_for_market_hours_key, hours_for_market_hours_key,
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

/// Asserts a query refuses `date` because the venue-local day precedes the
/// permanent 2025 support floor (LAW-COVERAGE).
fn assert_before_floor<T: std::fmt::Debug>(
    answer: Result<T, CalendarQueryError>,
    source: CalendarSource,
    date: NaiveDate,
    label: &str,
) {
    assert_refusal(
        answer,
        CalendarQueryError::BeforeSupportFloor { source, date },
        label,
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

/// Asserts a query refuses on a date before the 2025 floor without pinning which
/// pre-floor day it had to resolve.
///
/// A query may need the **opening day** of the session containing the caller's
/// instant rather than the instant's own local day, and one local day earlier is
/// still before the floor. The variant, the identity and the fact that the named
/// day precedes [`SUPPORT_FLOOR`] are all asserted; only the choice between two
/// adjacent pre-floor days is left to the query, which is the part of the
/// contract the caller cannot predict. The two-year sweep below is the only
/// place this looseness is needed; every per-instant assertion in this file
/// names its date exactly.
fn assert_before_floor_any_day<T: std::fmt::Debug>(
    answer: Result<T, CalendarQueryError>,
    source: CalendarSource,
    label: &str,
) {
    let error = answer.expect_err(&format!("{label}: expected a pre-floor refusal"));
    assert_eq!(
        error.source(),
        source,
        "{label}: the refusal names the wrong identity"
    );
    assert!(
        matches!(
            error,
            CalendarQueryError::BeforeSupportFloor { date, .. } if date < SUPPORT_FLOOR
        ),
        "{label}: expected a refusal on a day before the floor, got {error:?}"
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

    // Which of those comparisons are comparisons of refusals, stated rather
    // than left implicit: at 2026-04-20 16:30 CT the state and the trade date
    // resolve the next session through the Sunday queue that
    // `GlobexEquityIndex` withholds (#79), so the coverage error is the only
    // legal answer there. The open and bounds queries resolve no withheld phase
    // at that instant and still answer.
    let after_the_close = ct((2026, 4, 20), (16, 30, 0));
    let globex = CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEquityIndex);
    assert_outside_coverage(
        policy.session_state(after_the_close),
        globex,
        day(2026, 4, 20),
        "the state after the close",
    );
    assert_outside_coverage(
        policy.trade_date(after_the_close),
        globex,
        day(2026, 4, 20),
        "the trade date after the close",
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
            .is_open(ct((2026, 6, 8), (16, 1, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .trade_date(ct((2026, 6, 8), (16, 1, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(tuesday)
    );
    assert_eq!(
        calendar
            .candle_start(ct((2026, 6, 7), (12, 0, 0)), CalendarResolution::Daily,)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2026, 6, 5), (16, 1, 0)))
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
        Some(ct((2026, 6, 5), (16, 1, 0)))
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
    // answers `None`. The trade date cannot be answered at all:
    // `GlobexCryptocurrency` declares the span unsourced, so the query that
    // would have to name a trade date refuses rather than reporting the removal
    // as a missing date — a coverage refusal is never `None` (LAW-COVERAGE).
    let friday_evening = ct((2026, 6, 5), (17, 0, 0));
    assert_eq!(
        closed
            .session_bounds(friday_evening)
            .expect("the covered instant still answers under a policy"),
        None
    );
    assert_outside_coverage(
        closed.trade_date(friday_evening),
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexCryptocurrency),
        day(2026, 6, 5),
        "the trade date under an always-closed policy",
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
    assert_outside_coverage(
        calendar.is_closed_trade_date(wednesday, SessionKind::Both),
        thailand,
        day(2025, 5, 6),
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
    // The state at 13:00 CT — after the policy's 12:15 close — needs the next
    // session, which opens through the Sunday-evening queue that
    // `GlobexEquityIndex` withholds (#79). That phase is refused rather than
    // reported as `Closed` (LAW-COVERAGE); the clamp itself is asserted above by
    // the regular-session and bar probes, which resolve no withheld phase.
    assert_outside_coverage(
        calendar.session_state(ct((2026, 4, 20), (13, 0, 0))),
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEquityIndex),
        day(2026, 4, 20),
        "the state after the policy's early close",
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

    // CME's Globex coverage begins at the permanent 2025 floor, so every probe
    // on this 2014 date refuses: the crate ships no 2014 holiday data and will
    // not carry the modern normal week back to a date it cannot source
    // (LAW-COVERAGE). The policy mechanism itself is exercised on covered 2026
    // dates throughout this file; what this policy does to Good Friday is no
    // longer observable.
    let globex = CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEquityIndex);
    assert_before_floor(
        calendar.is_open(ct((2014, 4, 18), (10, 0, 0))),
        globex,
        good_friday,
        "the closed Friday",
    );
    assert_before_floor(
        calendar.is_closed_trade_date(good_friday, SessionKind::Both),
        globex,
        good_friday,
        "the closed trade date",
    );
    assert_before_floor(
        calendar.is_closed_all_day_on(good_friday, SessionKind::Both),
        globex,
        good_friday,
        "the closed all-day window",
    );
    assert_before_floor(
        calendar.next_session_open_after(ct((2014, 4, 17), (16, 15, 0))),
        globex,
        day(2014, 4, 17),
        "the scan to the next session",
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
    // The daily bar resolves the opening day of the trade date the policy clips,
    // which is 2015-11-26 and precedes the 2025 floor: every pre-floor probe
    // refuses, on the overlay surfaces exactly as on the bare calendar, so the
    // notice's 12:15 CT close is not observable here (LAW-COVERAGE). The
    // identity's modern early closes are asserted by the covered 2026 fixtures
    // above and below.
    let globex = CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEquityIndex);
    assert_before_floor(
        thanksgiving.candle_end(ct((2015, 11, 26), (18, 0, 0)), CalendarResolution::Daily),
        globex,
        day(2015, 11, 26),
        "the post-Thanksgiving daily bar",
    );
    assert_before_floor(
        thanksgiving.is_open(ct((2015, 11, 27), (12, 15, 0))),
        globex,
        post_thanksgiving,
        "the shrunken post-Thanksgiving session",
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
    // and both refuse: the bar's own day (`2015-12-23`, the opening day of the
    // trade date the policy clips) and the Christmas Eve the scan was addressed
    // to. Which instant each names is part of the contract, so both are stated.
    assert_before_floor(
        christmas.candle_end(ct((2015, 12, 23), (18, 0, 0)), CalendarResolution::Daily),
        globex,
        day(2015, 12, 23),
        "the Christmas Eve bar",
    );
    assert_before_floor(
        christmas.next_session_open_after(ct((2015, 12, 24), (12, 15, 0))),
        globex,
        christmas_eve,
        "the scan past Christmas",
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
    // Both probes are venue-local 2018 dates, before the permanent 2025 floor,
    // so the forward scan refuses before it walks: whether the fourteenth
    // candidate day is inside the window and the fifteenth outside it is no
    // longer observable on this identity, and a refusal is not `None`
    // (LAW-COVERAGE). The refusal names the local day each scan was addressed
    // to, which is the date it could not source.
    let nyse_national = CalendarSource::Exchange(Exchange::NyseNational);
    assert_before_floor(
        calendar.next_session_open_after(et((2018, 5, 7), (7, 0, 0))),
        nyse_national,
        day(2018, 5, 7),
        "the scan from the first Monday",
    );
    assert_before_floor(
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
    // 16:30 CT is the daily halt, and naming the state or the trade date there
    // resolves the next session through the Sunday-evening queue that
    // `GlobexEquityIndex` withholds (#79), so both queries refuse that date
    // (LAW-COVERAGE). The halt's own shape is stated where it resolves no
    // withheld phase — the extended state at 15:20 above and the daily bounds.
    let globex = CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEquityIndex);
    assert_outside_coverage(
        calendar.trade_date(ct((2026, 4, 20), (16, 30, 0))),
        globex,
        monday,
        "the trade date at the daily halt",
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
    assert_outside_coverage(
        calendar.session_state(ct((2026, 4, 20), (16, 30, 0))),
        globex,
        monday,
        "the state at the daily halt",
    );
    // The Saturday probe is a `Closed` the identity will not state either: the
    // weekend state is derived from the next session, which is the withheld
    // Sunday queue.
    assert_outside_coverage(
        calendar.session_state(ct((2026, 4, 25), (12, 0, 0))),
        globex,
        day(2026, 4, 25),
        "the weekend state",
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
                SessionState::Halt | SessionState::Maintenance | SessionState::Closed => {
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

    // The comparison this fence used to make — the date-aware calendar against
    // the dated fixed snapshot, hour by hour, over 2022-2023 — can no longer be
    // made: both years lie before the permanent 2025 support floor, so every key
    // refuses every probe (LAW-COVERAGE) and there is no calendar answer for the
    // snapshot to be compared against. What remains, and what this test now
    // asserts, is the refusal sweep itself: over two years of hourly probes no
    // identity may carry a pre-floor schedule back to a date it cannot source.
    //
    // The Rough Rice daily-bar carve-out this fence guarded (CBOT Submission
    // 18-001's non-wrapping 19:00-21:00 CT evening leg, which only an identified
    // calendar can key to the following trade date) is consequently not
    // derivable here any more: it distinguished two daily bars, and no calendar
    // bar exists in this window. It stays documented in
    // `docs/schedules/sources.md` and fenced by the Rough Rice family tests; the
    // flag that used to prove it was load-bearing is gone rather than left
    // silently satisfied.
    let mut instant = instant;
    let mut probes = 0_u64;
    while instant < end {
        for &key in MarketHoursKey::ALL {
            let calendar = calendar_for_market_hours_key(key);
            let local = instant.with_timezone(&calendar.tz()).date_naive();
            let label = format!("{key} at {instant}");
            assert_eq!(
                calendar.coverage().coverage_on(local),
                DateCoverage::BeforeSupportFloor,
                "{label}: the venue-local day {local} precedes the support floor"
            );
            assert_before_floor_any_day(calendar.is_open(instant), calendar.source(), &label);
            assert_before_floor_any_day(
                calendar.candle_end(instant, CalendarResolution::Daily),
                calendar.source(),
                &label,
            );
            // The other half of the original pairing is unchanged and still
            // total: an identity-erased fixed snapshot claims no coverage, so
            // `hours_at` keeps stating the schedule the identity now refuses to
            // source — it is where this window's shipped normal weeks stay
            // readable.
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
