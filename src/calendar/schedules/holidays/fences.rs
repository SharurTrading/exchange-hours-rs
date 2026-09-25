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
use crate::calendar::exceptions::ExceptionBlock;

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
///
/// `windows` is the table's coverage, one inclusive span per audited era. It is
/// checked ordered and non-overlapping, and every row must fall inside one of
/// them, because a row outside every window would be a date the table answers
/// for while claiming not to have audited it.
pub(crate) const fn assert_table(
    windows: &[(i32, u32, u32, i32, u32, u32)],
    dates: &[(i32, u32, u32)],
    rows: &[HolidayRow],
) {
    assert!(
        !windows.is_empty(),
        "a holiday table must declare at least one coverage window"
    );
    assert!(
        dates.len() == rows.len(),
        "holiday fence received a different number of trade dates than rows"
    );
    assert_ordered_windows(windows);

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
            in_windows(windows, dates[index]),
            "holiday row falls outside every coverage window its table declares"
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

/// The earliest `first` among a table's coverage windows.
pub(crate) const fn window_first(windows: &[(i32, u32, u32, i32, u32, u32)]) -> NaiveDate {
    assert!(
        !windows.is_empty(),
        "a holiday table must declare at least one coverage window"
    );
    let mut best = (windows[0].0, windows[0].1, windows[0].2);
    let mut index = 1;
    while index < windows.len() {
        let candidate = (windows[index].0, windows[index].1, windows[index].2);
        if day_key(candidate) < day_key(best) {
            best = candidate;
        }
        index += 1;
    }
    holiday_date(best.0, best.1, best.2)
}

/// The latest `last` among a table's coverage windows.
pub(crate) const fn window_last(windows: &[(i32, u32, u32, i32, u32, u32)]) -> NaiveDate {
    assert!(
        !windows.is_empty(),
        "a holiday table must declare at least one coverage window"
    );
    let mut best = (windows[0].3, windows[0].4, windows[0].5);
    let mut index = 1;
    while index < windows.len() {
        let candidate = (windows[index].3, windows[index].4, windows[index].5);
        if day_key(candidate) > day_key(best) {
            best = candidate;
        }
        index += 1;
    }
    holiday_date(best.0, best.1, best.2)
}

/// Fails the build unless the coverage windows ascend without overlapping.
///
/// Both endpoints of **every** window are built here, not only the outer ones:
/// [`day_key`] cannot tell month 13 from a real date, so a window whose invalid
/// endpoint is not the outermost would otherwise survive the build and then be
/// silently rejected by [`HolidayCoverage::contains`] while `windows()` still
/// listed a bound derived from the outermost pair.
const fn assert_ordered_windows(windows: &[(i32, u32, u32, i32, u32, u32)]) {
    let mut index = 0;
    while index < windows.len() {
        let (first_year, first_month, first_day, last_year, last_month, last_day) = windows[index];
        let _first = holiday_date(first_year, first_month, first_day);
        let _last = holiday_date(last_year, last_month, last_day);
        let first = day_key((first_year, first_month, first_day));
        let last = day_key((last_year, last_month, last_day));
        assert!(
            first <= last,
            "a holiday coverage window is inverted: its last date precedes its first"
        );
        if index > 0 {
            let (_, _, _, prev_last_year, prev_last_month, prev_last_day) = windows[index - 1];
            assert!(
                day_key((prev_last_year, prev_last_month, prev_last_day)) < first,
                "holiday coverage windows overlap or are out of order; a table \
                 ships one window per audited era, ascending"
            );
        }
        index += 1;
    }
}

/// Whether one raw date triple lies inside any coverage window.
///
/// The endpoints have already been built by [`assert_ordered_windows`], so the
/// comparisons here are between real dates.
const fn in_windows(windows: &[(i32, u32, u32, i32, u32, u32)], date: (i32, u32, u32)) -> bool {
    let key = day_key(date);
    let mut index = 0;
    while index < windows.len() {
        let (first_year, first_month, first_day, last_year, last_month, last_day) = windows[index];
        if day_key((first_year, first_month, first_day)) <= key
            && key <= day_key((last_year, last_month, last_day))
        {
            return true;
        }
        index += 1;
    }
    false
}

/// Fails the build when a row's instants leave the `DayPolicy` ranges.
///
/// A replacement row states no scalar instant of its own, so its complete block
/// set is handed to [`assert_blocks`] instead; every other kind is checked here.
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
        // A replacement row states its instants block by block, so the whole
        // set is validated here and the scalars below do not apply to it. The
        // rule this arm exists for is the last one: a block set is the only
        // kind that can state an instant on a day other than its trade date.
        HolidayKind::ReplacementBlocks(set) => {
            assert_blocks(set);
            return;
        }
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

/// Whether any row of a table states a replacement block set.
///
/// Resolved during constant evaluation so the query hot path reads one bit
/// instead of walking the table: a table that ships no block row must not pay
/// for the replacement scan, and the scan must not be reachable for it.
pub(crate) const fn any_blocks(rows: &[HolidayRow]) -> bool {
    let mut index = 0;
    while index < rows.len() {
        if matches!(rows[index].kind, HolidayKind::ReplacementBlocks(_)) {
            return true;
        }
        index += 1;
    }
    false
}

/// Fails the build unless a row's replacement block set is well formed.
///
/// These are exactly the rules
/// [`StaticSessionExceptions::new`](crate::StaticSessionExceptions::new)
/// applies to a caller's records, so a built-in row and a caller's record can
/// never state differently shaped topologies: the set is non-empty, every
/// offset is inside the block day range, every instant is inside the
/// `DayPolicy` ranges, no block opens on its own trade date and closes after it
/// — a trade date is named by the local date of its final close — and the set
/// is ordered by opening day and then open time, which is the order every
/// replacement scan and the block-ordering rule below rely on.
///
/// Each rule is a separate `assert!` so the build failure names the rule that
/// broke rather than only the row.
const fn assert_blocks(set: &[ExceptionBlock]) {
    assert!(
        !set.is_empty(),
        "holiday replacement row carries an empty block set; a trade date with no \
         blocks is HolidayKind::Closed"
    );
    let mut index = 0;
    while index < set.len() {
        let current = set[index];
        assert!(
            current.open_day_offset() >= ExceptionBlock::MIN_DAY_OFFSET
                && current.open_day_offset() <= ExceptionBlock::MAX_DAY_OFFSET,
            "holiday replacement block's opening day offset is outside the block day range"
        );
        assert!(
            current.open_ssm() < SECONDS_PER_DAY,
            "holiday replacement block's open is outside 0..86_400"
        );
        assert!(
            current.close_ssm() <= SECONDS_PER_DAY,
            "holiday replacement block's close is outside 0..=86_400"
        );
        assert!(
            !(current.open_day_offset() == 0 && current.wraps_to_next_day()),
            "holiday replacement block opens on its own trade date and closes after it; \
             a trade date is the local date of its final close"
        );
        if index > 0 {
            let previous = set[index - 1];
            assert!(
                current.open_day_offset() > previous.open_day_offset()
                    || (current.open_day_offset() == previous.open_day_offset()
                        && current.open_ssm() >= previous.open_ssm()),
                "holiday replacement blocks are not ordered by opening day and open time"
            );
        }
        index += 1;
    }
}
