// SPDX-License-Identifier: MIT-0

//! The constant-evaluation fences behind the `holidays!` macro.
//!
//! Every invariant a holiday table relies on at runtime — a total order for
//! the partition-point gate and the binary-search lookup, rows inside the
//! published coverage window, a named artifact at an admissible tier, and
//! instants inside the `DayPolicy` ranges — is asserted during constant
//! evaluation, so a violation is a build failure rather than a wrong answer.
//! This copies the idiom `revisions!` established for schedule timelines.
use chrono::NaiveDate;

use super::{EvidenceTier, HolidayKind, HolidayRow};

/// The upper bound of a venue-local seconds-since-midnight close.
///
/// `86_400` is the end-exclusive close of a complete local day and is a legal
/// early close; the matching open bound is one second lower, exactly as
/// [`StaticDayPolicy`](crate::StaticDayPolicy) validates a caller's records.
const SECONDS_PER_DAY: u32 = 86_400;

/// Builds a hard-coded holiday trade date during constant evaluation.
#[expect(
    clippy::panic,
    reason = "const-eval only: an invalid sourced date must fail the build"
)]
pub(crate) const fn holiday_date(year: i32, month: u32, day: u32) -> NaiveDate {
    match NaiveDate::from_ymd_opt(year, month, day) {
        Some(date) => date,
        None => panic!("invalid hard-coded holiday trade date"),
    }
}

/// Orders `(year, month, day)` as one comparable scalar for constant
/// evaluation; tuple `PartialOrd` is not const-callable.
const fn day_key(date: (i32, u32, u32)) -> i64 {
    date.0 as i64 * 10_000 + date.1 as i64 * 100 + date.2 as i64
}

/// Fails the build unless every `holidays!` invariant holds.
///
/// `dates` carries the same trade dates as `rows`, in the same order, as raw
/// `(year, month, day)` triples: `NaiveDate`'s comparison operators are not
/// const-callable, so ordering is decided on [`day_key`] scalars instead.
pub(crate) const fn assert_table(
    first: (i32, u32, u32),
    last: (i32, u32, u32),
    dates: &[(i32, u32, u32)],
    rows: &[HolidayRow],
) {
    assert!(
        day_key(first) <= day_key(last),
        "holiday coverage window is inverted: its last trade date precedes its first"
    );
    assert!(
        dates.len() == rows.len(),
        "holiday fence received a different number of trade dates than rows"
    );

    let mut index = 0;
    while index < dates.len() {
        if index > 0 {
            assert!(
                day_key(dates[index]) > day_key(dates[index - 1]),
                "holiday table is not strictly ascending by trade date; \
                 a row is out of order or shadowed by a duplicate"
            );
        }
        assert!(
            day_key(dates[index]) >= day_key(first) && day_key(dates[index]) <= day_key(last),
            "holiday row falls outside its table's coverage window"
        );
        assert!(
            !rows[index].document.as_str().is_empty(),
            "holiday row carries no document id"
        );
        assert!(
            matches!(rows[index].tier, EvidenceTier::T1 | EvidenceTier::T2),
            "holiday row is sourced below T2; LAW-PRIMARY-SOURCES admits only the \
             operator's own statement or the operator's own machine channel"
        );
        assert_instants(rows[index].kind);
        index += 1;
    }
}

/// Fails the build when a row's instants leave the `DayPolicy` ranges.
///
/// The numeric order of an open and a close on one row is deliberately **not**
/// constrained: a wrapped trading day can open on the preceding local date at a
/// numerically later wall clock than its final close on the trade date, which
/// is exactly what
/// [`DayOverride::late_open_and_early_close`](crate::DayOverride::late_open_and_early_close)
/// documents for a caller's own records.
const fn assert_instants(kind: HolidayKind) {
    let (open_ssm, close_ssm) = match kind {
        HolidayKind::Closed | HolidayKind::Unsourced => return,
        HolidayKind::EarlyClose { close_ssm } => (0, close_ssm),
        HolidayKind::LateOpen { open_ssm } => (open_ssm, 0),
        HolidayKind::LateOpenAndEarlyClose {
            open_ssm,
            close_ssm,
        } => (open_ssm, close_ssm),
    };
    assert!(
        open_ssm < SECONDS_PER_DAY,
        "holiday row's late open is outside 0..86_400"
    );
    assert!(
        close_ssm <= SECONDS_PER_DAY,
        "holiday row's early close is outside 0..=86_400"
    );
}

/// Constructs [`HolidayKind::EarlyClose`], for a terse table row.
pub(crate) const fn early_close(close_ssm: u32) -> HolidayKind {
    HolidayKind::EarlyClose { close_ssm }
}

/// Constructs [`HolidayKind::LateOpen`], for a terse table row.
pub(crate) const fn late_open(open_ssm: u32) -> HolidayKind {
    HolidayKind::LateOpen { open_ssm }
}

/// Constructs [`HolidayKind::LateOpenAndEarlyClose`], for a terse table row.
pub(crate) const fn late_open_and_early_close(open_ssm: u32, close_ssm: u32) -> HolidayKind {
    HolidayKind::LateOpenAndEarlyClose {
        open_ssm,
        close_ssm,
    }
}
