// SPDX-License-Identifier: MIT-0

//! Shared imports and fixture helpers for this suite.

use crate::support::local_opt;
use crate::support::zones::{BERLIN, LONDON, NEW_YORK, SINGAPORE, US_CENTRAL, UTC};
pub(super) use exchange_hours::*;
pub(super) use jiff::Timestamp;
use jiff::tz::TimeZone;

pub(super) fn zoned(tz: &TimeZone, date: (i16, i8, i8), time: (u32, u32, u32)) -> Timestamp {
    local_opt(tz, date, time).expect("valid zoned instant")
}

pub(super) fn utc(date: (i16, i8, i8), time: (u32, u32, u32)) -> Timestamp {
    zoned(&UTC, date, time)
}

pub(super) fn ct(date: (i16, i8, i8), time: (u32, u32, u32)) -> Timestamp {
    zoned(&US_CENTRAL, date, time)
}

pub(super) fn et(date: (i16, i8, i8), time: (u32, u32, u32)) -> Timestamp {
    zoned(&NEW_YORK, date, time)
}

pub(super) fn cet(date: (i16, i8, i8), time: (u32, u32, u32)) -> Timestamp {
    zoned(&BERLIN, date, time)
}

pub(super) fn lon(date: (i16, i8, i8), time: (u32, u32, u32)) -> Timestamp {
    zoned(&LONDON, date, time)
}

pub(super) fn sgt(date: (i16, i8, i8), time: (u32, u32, u32)) -> Timestamp {
    zoned(&SINGAPORE, date, time)
}

/// Asserts a date-aware query refused with the error variant `expected`
/// maps onto, whatever date inside the error it names.
///
/// This is the shape for a query whose answer depends on a run of days the
/// caller did not name: `is_closed_trade_date(2025-01-01)` reads the
/// holiday layer on 2025-01-01 but settles the trade date that opens on
/// 2024-12-31, so it can refuse naming either day — and with the variant
/// that applies to *that* day. The variant is the claim; the date inside it
/// is the query's own business. The claim is always the refusal, never a
/// `bool`, and never a date the identity would have had to guess.
pub(super) fn assert_refused_variant(
    answer: &Result<impl core::fmt::Debug, CalendarQueryError>,
    expected: DateCoverage,
    label: &str,
) {
    let refused = matches!(
        (expected, answer),
        (
            DateCoverage::BeforeSupportFloor,
            Err(CalendarQueryError::BeforeSupportFloor { .. })
        ) | (
            DateCoverage::OutsideCoveredRange,
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ) | (
            DateCoverage::UnresolvedGap,
            Err(CalendarQueryError::UnresolvedGap { .. })
        )
    );
    assert!(
        refused,
        "{label} must be refused with the error variant {expected:?} maps onto; \
             got {answer:?}"
    );
}

/// Asserts a date-aware query refused the **venue-local day of `instant`**
/// because that day precedes the permanent support floor (LAW-COVERAGE).
///
/// The era sweeps in this suite probe historical cutovers whose dates lie
/// before 2025-01-01. The published grid of those eras is still a sourced
/// fact and the fixed snapshot states it — `hours_for_exchange` /
/// `hours_for_market_hours_key` — but the date-aware calendar has no
/// coverage there and must refuse rather than report the refusal as a
/// closure. The floor is a *local* date, so it is checked against the
/// identity's own zone, never the UTC day.
///
/// The `Result` is taken by value so each call site stays one expression;
/// the query helpers return owned payloads (`bool`, `Option<SessionWindow>`,
/// `Option<Date>`).
pub(super) fn assert_refuses_before_floor<T: core::fmt::Debug>(
    answer: Result<T, CalendarQueryError>,
    calendar: ExchangeCalendar,
    instant: Timestamp,
) {
    let local_date = instant.to_zoned_datetime(calendar.tz()).date();
    // A date the metadata calls covered answers: the value may be `false`
    // for a real pre-launch closure.
    if calendar.coverage().coverage_on(local_date) == DateCoverage::Covered {
        assert!(
            answer.is_ok(),
            "{:?}: {instant} is venue-local {local_date}, a covered date, so the \
                 date-aware surface must answer it; got {answer:?}",
            calendar.source(),
        );
        return;
    }
    // A declared phase gap shadows the date-level verdict: on a date whose
    // date-level facts answer, session queries answer through the shadow.
    // So does the resolution edge (#151): a date incomplete only because
    // answering it completely would consult a neighbour keeps its own
    // facts sourced, and the queries that need only them answer — which is
    // why the fixture asserts which queries answer rather than that every
    // query refuses.
    // So does the bridged residual (#296, the 2026-10-07 ruling): a date
    // on a bridged span — between two audited windows, or below the first
    // window with the week sourced — answers its session questions from
    // the sourced normal week, while its holiday classification refuses.
    let phase_shadow_here = phase_shadow_answers(calendar, local_date);
    let resolution_edge_here = calendar.coverage().gaps().any(|gap| {
        gap.reason() == CoverageGapReason::ResolutionEdge && gap.range().contains(local_date)
    });
    let bridged_here = calendar.coverage().gaps().any(|gap| {
        gap.reason() == CoverageGapReason::HolidayWindowsBridged && gap.range().contains(local_date)
    });
    if answer.is_ok() && (phase_shadow_here || resolution_edge_here || bridged_here) {
        return;
    }
    let error =
        answer.expect_err("{:?}: the date-aware surface must refuse, never answer a closure");
    // At the 2010 floor most era-sweep probes sit at or above the floor, so
    // the refusal they earn is the identity's own date-level verdict
    // (carried era, unaudited window, withheld date) rather than the
    // pre-floor one — read from the metadata at the date the error names,
    // so a scan that steps into a neighbouring unsourced day is checked
    // against that day. A declared phase gap shadows the date-level
    // verdict in the metadata: on a date whose date-level facts answer,
    // session queries answer through the shadow (`false` may be a real
    // pre-launch closure) and a withheld row refuses as `UnresolvedGap`.
    let named = error.date();
    let named_verdict = calendar.coverage().coverage_on(named);
    let variant = match &error {
        CalendarQueryError::BeforeSupportFloor { .. } => "before",
        CalendarQueryError::OutsideCoveredRange { .. } => "outside",
        CalendarQueryError::UnresolvedGap { .. } => "unresolved",
        CalendarQueryError::SearchExhausted { .. } => "exhausted",
        _ => "other",
    };
    let verdict_name = match named_verdict {
        DateCoverage::BeforeSupportFloor => "before",
        DateCoverage::OutsideCoveredRange => "outside",
        DateCoverage::UnresolvedGap => "unresolved",
        DateCoverage::Covered | DateCoverage::NormalWeekOnly => "covered",
        _ => "other",
    };
    let phase_shadow = named_verdict == DateCoverage::OutsideCoveredRange
        && calendar
            .coverage()
            .phase_gaps()
            .iter()
            .any(|gap| gap.applies_on(named))
        && calendar
            .coverage()
            .normal_week_sourced_from()
            .is_none_or(|h| named >= h)
        && calendar
            .coverage()
            .holiday_contract()
            .coverage()
            .is_some_and(|windows| windows.contains(named));
    let stated = variant == verdict_name || (variant == "unresolved" && phase_shadow);
    assert!(
        error.source() == calendar.source() && stated,
        "{instant} is venue-local {local_date}, which the identity derives {named_verdict:?} \
             for at {named}; the refusal must state that verdict ({variant} vs {verdict_name})"
    );
}

/// Whether a declared phase gap shadows the date-level verdict on `date`:
/// a gap applies on the date while the date-level facts — sourced normal
/// week, audited holiday window, no withheld row — would answer it.
fn phase_shadow_answers(calendar: ExchangeCalendar, date: jiff::civil::Date) -> bool {
    let coverage = calendar.coverage();
    let gap_applies = coverage.phase_gaps().iter().any(|gap| gap.applies_on(date));
    let date_level_answers = coverage
        .normal_week_sourced_from()
        .is_none_or(|horizon| date >= horizon)
        && coverage
            .holiday_contract()
            .coverage()
            .is_some_and(|windows| windows.contains(date))
        && !calendar
            .holiday_on(date)
            .is_some_and(|holiday| holiday.kind() == exchange_hours::HolidayKind::Unsourced);
    gap_applies && date_level_answers
}

/// Asserts a date-aware query refused the venue-local day of `instant` with
/// `expected`, the verdict the identity's own `coverage()` reports for it.
///
/// Post-floor refusals split in two: [`DateCoverage::OutsideCoveredRange`]
/// (the identity ships no holiday table and claims none, its weekday profile
/// is carried there, or it declares a phase-level gap) and
/// [`DateCoverage::UnresolvedGap`] (an audited window withholds the date as
/// `Unsourced`). Both map one-to-one onto a [`CalendarQueryError`] variant,
/// so the caller states the verdict it expects and this asserts the query
/// answered exactly that — never a `bool`, and never a different variant.
#[expect(
    clippy::needless_pass_by_value,
    reason = "query helpers return owned Results; taking them by value keeps \
                  every call site a single expression"
)]
pub(super) fn assert_refused(
    answer: Result<impl core::fmt::Debug, CalendarQueryError>,
    expected: DateCoverage,
    calendar: ExchangeCalendar,
    instant: Timestamp,
) {
    let local_date = instant.to_zoned_datetime(calendar.tz()).date();
    let reported = calendar.coverage().coverage_on(local_date);
    assert_eq!(
        reported,
        expected,
        "{:?}: the fixture's expected verdict must be the identity's own for \
             venue-local {local_date}",
        calendar.source(),
    );
    let refused = matches!(
        (expected, &answer),
        (
            DateCoverage::BeforeSupportFloor,
            Err(CalendarQueryError::BeforeSupportFloor { .. })
        ) | (
            DateCoverage::OutsideCoveredRange,
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ) | (
            DateCoverage::UnresolvedGap,
            Err(CalendarQueryError::UnresolvedGap { .. })
        )
    );
    assert!(
        refused,
        "{:?}: {instant} is venue-local {local_date}, which this identity covers \
             as {expected:?}, so the date-aware surface must refuse it with the \
             matching CalendarQueryError; got {answer:?}",
        calendar.source(),
    );
}
