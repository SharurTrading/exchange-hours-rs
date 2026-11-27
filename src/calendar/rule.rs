// SPDX-License-Identifier: MIT-0

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
// Monday only.
pub(crate) const MON_ONLY: [bool; 7] = [true, false, false, false, false, false, false];
// Monday-Thursday only.
pub(crate) const MON_THU: [bool; 7] = [true, true, true, true, false, false, false];
// Friday only.
pub(crate) const FRI: [bool; 7] = [false, false, false, false, true, false, false];
// Tuesday only.
pub(crate) const TUE_ONLY: [bool; 7] = [false, true, false, false, false, false, false];
// Tuesday-Friday only.
pub(crate) const TUE_FRI: [bool; 7] = [false, true, true, true, true, false, false];
// Sunday only.
pub(crate) const SUN_ONLY: [bool; 7] = [false, false, false, false, false, false, true];
// Open on Sunday and Mon–Thu (i.e., Sunday evening open, no Friday overnight open)
pub(crate) const SUN_PLUS_MON_THU: [bool; 7] = [true, true, true, true, false, false, true];
// Open on all seven weekdays.
pub(crate) const ALL_DAYS: [bool; 7] = [true, true, true, true, true, true, true];

/// One schedule slice for a market session.
///
/// - `days`: Monday=0 .. Sunday=6 mask; `true` enables the rule on that weekday.
/// - `open_ssm` / `close_ssm`: seconds since local midnight in the exchange time zone.
///   If `open_ssm < close_ssm` the session is same-day. If
///   `open_ssm >= close_ssm`, it wraps into the next local day and closes at
///   `close_ssm` there; equal endpoints therefore encode exactly one complete
///   local-day span anchored on each enabled opening day.
/// - Close comparisons are end-exclusive: an instant exactly equal to `close_ssm`
///   is considered closed.
///
/// # Domain invariants
///
/// A well-formed rule has `open_ssm < 86_400`, `close_ssm <= 86_400` (the
/// `86_400` sentinel closes a same-day session exactly at next local
/// midnight), and at least one enabled weekday. Construction and
/// deserialization enforce all three. Use the accessors to read a rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SessionRuleFields")]
pub struct SessionRule {
    pub(super) days: [bool; 7],
    pub(super) open_ssm: u32,
    pub(super) close_ssm: u32,
}

#[derive(Deserialize)]
#[serde(rename = "SessionRule")]
struct SessionRuleFields {
    days: [bool; 7],
    open_ssm: u32,
    close_ssm: u32,
}

impl TryFrom<SessionRuleFields> for SessionRule {
    type Error = SessionRuleError;

    fn try_from(fields: SessionRuleFields) -> Result<Self, Self::Error> {
        Self::new(fields.days, fields.open_ssm, fields.close_ssm)
    }
}

impl SessionRule {
    /// Builds a weekday-masked rule with end-exclusive local bounds.
    ///
    /// # Errors
    ///
    /// Returns the first out-of-range boundary or an empty weekday mask.
    /// Equal endpoints represent one complete local-day span.
    pub const fn new(
        days: [bool; 7],
        open_ssm: u32,
        close_ssm: u32,
    ) -> Result<Self, SessionRuleError> {
        let rule = Self {
            days,
            open_ssm,
            close_ssm,
        };
        match rule.validate() {
            Ok(()) => Ok(rule),
            Err(error) => Err(error),
        }
    }

    /// Returns the opening weekday mask, starting with Monday.
    #[must_use]
    pub const fn days(&self) -> [bool; 7] {
        self.days
    }

    /// Returns the opening time in seconds since local midnight.
    #[must_use]
    pub const fn open_ssm(&self) -> u32 {
        self.open_ssm
    }

    /// Returns the end-exclusive closing time. A value of 86,400 denotes
    /// the midnight after the opening day.
    #[must_use]
    pub const fn close_ssm(&self) -> u32 {
        self.close_ssm
    }

    /// Checks the same bounds and weekday mask as construction.
    ///
    /// # Errors
    ///
    /// Returns the first violated invariant; see [`SessionRule::new`].
    pub const fn validate(&self) -> Result<(), SessionRuleError> {
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
        if matches!(self.days, [false, false, false, false, false, false, false]) {
            return Err(SessionRuleError::NoEnabledDays);
        }
        Ok(())
    }

    /// Returns whether this session closes on the next local day.
    ///
    /// Equal endpoints return `true`: they represent one complete local-day
    /// span anchored on each enabled opening day, not an empty interval.
    #[must_use]
    pub const fn wraps_to_next_day(&self) -> bool {
        self.open_ssm >= self.close_ssm
    }
}

/// [`SECONDS_PER_NORMAL_DAY`] in the `u32` domain of [`SessionRule`] fields.
const SECONDS_PER_NORMAL_DAY_U32: u32 = 86_400;
// Compile-time tie to the u64 source so the two constants cannot drift.
// Evaluated during constant evaluation only — this is not a runtime assert.
const _: () = assert!(SECONDS_PER_NORMAL_DAY_U32 as u64 == SECONDS_PER_NORMAL_DAY);

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
        let duration = if rule.wraps_to_next_day() {
            SECONDS_PER_NORMAL_DAY
                .saturating_sub(u64::from(rule.open_ssm))
                .saturating_add(u64::from(rule.close_ssm))
        } else {
            u64::from(rule.close_ssm - rule.open_ssm)
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
