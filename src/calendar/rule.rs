// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! The atom of the calendar: one weekday-masked open/close slice.
//!
//! A [`SessionRule`] is stated in the **exchange's** local time as seconds since
//! local midnight (SSM), never in UTC and never as a wall-clock string; the
//! local→UTC mapping (including DST) happens at query time in
//! [`super::local_time`]. [`SessionKind`] selects which of a profile's two rule
//! sets a query consults.

use serde::{Deserialize, Serialize};

/// Seconds in a nominal 24-hour day, used for normal-week interval arithmetic
/// only. Deliberately DST-free: the normal-week model counts scheduled seconds,
/// not elapsed wall-clock seconds.
pub(crate) const SECONDS_PER_NORMAL_DAY: u64 = 86_400;
/// Seconds in a nominal seven-day week; the saturation ceiling for
/// [`super::MarketHours::normal_week_open_seconds`].
pub(crate) const SECONDS_PER_NORMAL_WEEK: u64 = 7 * SECONDS_PER_NORMAL_DAY;

// Common weekday masks (Mon=0 .. Sun=6) used across presets.
// Monday–Friday open days (no weekend trading)
pub(crate) const MON_FRI: [bool; 7] = [true, true, true, true, true, false, false];
// Open on Sunday and Mon–Thu (i.e., Sunday evening open, no Friday overnight open)
pub(crate) const SUN_PLUS_MON_THU: [bool; 7] = [true, true, true, true, false, false, true];
// Open on all seven weekdays.
pub(crate) const ALL_DAYS: [bool; 7] = [true, true, true, true, true, true, true];

/// One schedule slice for a market session.
///
/// - `days`: Monday=0 .. Sunday=6 mask; `true` enables the rule on that weekday.
/// - `open_ssm` / `close_ssm`: seconds since local midnight in the exchange time zone.
///   If `open_ssm <= close_ssm` the session is same-day; if `open_ssm > close_ssm`
///   the session wraps into the next local day and closes at `close_ssm` there.
/// - Close comparisons are end-exclusive: an instant exactly equal to `close_ssm`
///   is considered closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionRule {
    /// Weekday activation mask (Mon=0 .. Sun=6).
    pub days: [bool; 7],
    /// Open time in seconds since local midnight.
    pub open_ssm: u32,
    /// Close time in seconds since local midnight, end-exclusive.
    pub close_ssm: u32,
}

/// Selects which session set a query consults.
///
/// `MarketHours` keeps `regular` and `extended` rule sets separately; every
/// open/closed and boundary query takes a `SessionKind` to choose between them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionKind {
    /// Consult only primary/RTH (`regular`) sessions.
    Regular,
    /// Consult only electronic/overnight/auction (`extended`) sessions.
    Extended,
    /// Consult regular and extended sessions together.
    Both,
}

/// Projects one rule onto the `[0, SECONDS_PER_NORMAL_WEEK)` normal-week axis as
/// half-open `(start, end)` second offsets, one per enabled weekday.
///
/// A wrap rule that runs past Sunday midnight is split into two intervals so the
/// caller can union them without special-casing the week boundary. Holidays and
/// DST are deliberately absent: this is the normal-week schedule, not elapsed
/// time.
pub(crate) fn normal_week_rule_intervals(rule: &SessionRule) -> Vec<(u64, u64)> {
    let mut intervals = Vec::with_capacity(8);
    for (weekday, enabled) in (0_u64..).zip(rule.days) {
        if !enabled {
            continue;
        }
        let start = weekday
            .saturating_mul(SECONDS_PER_NORMAL_DAY)
            .saturating_add(u64::from(rule.open_ssm));
        let duration = if rule.open_ssm <= rule.close_ssm {
            u64::from(rule.close_ssm - rule.open_ssm)
        } else {
            SECONDS_PER_NORMAL_DAY
                .saturating_sub(u64::from(rule.open_ssm))
                .saturating_add(u64::from(rule.close_ssm))
        };
        let end = start.saturating_add(duration);
        if end <= SECONDS_PER_NORMAL_WEEK {
            intervals.push((start, end));
        } else {
            intervals.push((start, SECONDS_PER_NORMAL_WEEK));
            intervals.push((0, end.saturating_sub(SECONDS_PER_NORMAL_WEEK)));
        }
    }
    intervals
}
