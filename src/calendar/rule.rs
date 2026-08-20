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
///
/// # Domain invariants
///
/// A well-formed rule has `open_ssm < 86_400`, `close_ssm <= 86_400` (the
/// `86_400` sentinel closes a same-day session exactly at next local
/// midnight), `open_ssm != close_ssm` (a zero-length session matches no
/// instant), and at least one enabled weekday. [`SessionRule::new`] enforces
/// all four; construction by struct literal or via `Deserialize` bypasses them,
/// so validate untrusted input with [`SessionRule::validate`]. Queries stay
/// total for out-of-domain values but their answers are unspecified — an
/// out-of-range `open_ssm` silently degrades the rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionRule {
    /// Weekday activation mask (Mon=0 .. Sun=6).
    pub days: [bool; 7],
    /// Open time in seconds since local midnight, in `0..86_400`.
    pub open_ssm: u32,
    /// Close time in seconds since local midnight, end-exclusive, in
    /// `0..=86_400`.
    pub close_ssm: u32,
}

impl SessionRule {
    /// Builds a rule after checking the domain invariants listed on
    /// [`SessionRule`].
    ///
    /// # Errors
    ///
    /// Returns the first violated invariant: an out-of-range `open_ssm` or
    /// `close_ssm`, a zero-length interval (`open_ssm == close_ssm`), or an
    /// all-`false` weekday mask.
    pub fn new(days: [bool; 7], open_ssm: u32, close_ssm: u32) -> Result<Self, SessionRuleError> {
        let rule = Self {
            days,
            open_ssm,
            close_ssm,
        };
        rule.validate()?;
        Ok(rule)
    }

    /// Checks the domain invariants listed on [`SessionRule`] without
    /// consuming the rule — the check for rules that arrived by struct
    /// literal or `Deserialize` rather than through [`SessionRule::new`].
    ///
    /// # Errors
    ///
    /// Returns the first violated invariant; see [`SessionRule::new`].
    pub fn validate(&self) -> Result<(), SessionRuleError> {
        if self.open_ssm >= SECONDS_PER_NORMAL_DAY_U32 {
            return Err(SessionRuleError::OpenOutOfRange {
                open_ssm: self.open_ssm,
            });
        }
        if self.close_ssm > SECONDS_PER_NORMAL_DAY_U32 {
            return Err(SessionRuleError::CloseOutOfRange {
                close_ssm: self.close_ssm,
            });
        }
        if self.open_ssm == self.close_ssm {
            return Err(SessionRuleError::EmptyInterval { ssm: self.open_ssm });
        }
        if !self.days.iter().any(|&enabled| enabled) {
            return Err(SessionRuleError::NoEnabledDays);
        }
        Ok(())
    }
}

/// `SECONDS_PER_NORMAL_DAY` in the `u32` domain of [`SessionRule`] fields.
const SECONDS_PER_NORMAL_DAY_U32: u32 = 86_400;

/// A [`SessionRule`] domain-invariant violation, reported by
/// [`SessionRule::new`] and [`SessionRule::validate`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionRuleError {
    /// `open_ssm` is not inside `0..86_400`.
    OpenOutOfRange {
        /// The rejected open value.
        open_ssm: u32,
    },
    /// `close_ssm` is not inside `0..=86_400`.
    CloseOutOfRange {
        /// The rejected close value.
        close_ssm: u32,
    },
    /// `open_ssm == close_ssm`: a zero-length session matches no instant.
    EmptyInterval {
        /// The shared open/close value.
        ssm: u32,
    },
    /// Every weekday is disabled, so the rule can never apply.
    NoEnabledDays,
}

impl core::fmt::Display for SessionRuleError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OpenOutOfRange { open_ssm } => {
                write!(f, "open_ssm {open_ssm} is outside 0..86400")
            }
            Self::CloseOutOfRange { close_ssm } => {
                write!(f, "close_ssm {close_ssm} is outside 0..=86400")
            }
            Self::EmptyInterval { ssm } => {
                write!(
                    f,
                    "open_ssm and close_ssm are both {ssm}; the session is empty"
                )
            }
            Self::NoEnabledDays => write!(f, "no weekday is enabled"),
        }
    }
}

impl std::error::Error for SessionRuleError {}

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
