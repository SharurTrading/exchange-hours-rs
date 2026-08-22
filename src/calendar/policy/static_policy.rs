// SPDX-License-Identifier: MIT-0

//! Allocation-free day policies backed by caller-owned static records.

use chrono::NaiveDate;

use super::DayPolicy;

const SECONDS_PER_DAY: u32 = 86_400;

/// One caller-supplied override for a venue-local trade date.
///
/// An override can close the complete trading day, move its final close
/// earlier, delay its first open, or apply both boundary changes. It cannot
/// replace arbitrary intraday phases, add a split session, or express a
/// product-specific topology change; those cases are outside the [`DayPolicy`]
/// model and require a richer replacement-session layer.
///
/// Boundary values are checked when the record is placed in a
/// [`StaticDayPolicy`]. The explicit constructors prevent a closed date from
/// also carrying boundary changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DayOverride {
    trade_date: NaiveDate,
    kind: DayOverrideKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DayOverrideKind {
    Closed,
    Modified {
        late_open_ssm: Option<u32>,
        early_close_ssm: Option<u32>,
    },
}

impl DayOverride {
    /// Closes the complete trading day assigned to `trade_date`.
    #[must_use]
    pub const fn closed(trade_date: NaiveDate) -> Self {
        Self {
            trade_date,
            kind: DayOverrideKind::Closed,
        }
    }

    /// Moves `trade_date`'s final close to `early_close_ssm`.
    ///
    /// The value is venue-local seconds since midnight and is validated by
    /// [`StaticDayPolicy::new`].
    #[must_use]
    pub const fn early_close(trade_date: NaiveDate, early_close_ssm: u32) -> Self {
        Self {
            trade_date,
            kind: DayOverrideKind::Modified {
                late_open_ssm: None,
                early_close_ssm: Some(early_close_ssm),
            },
        }
    }

    /// Delays `trade_date`'s first open to `late_open_ssm`.
    ///
    /// The value is venue-local seconds since midnight and is validated by
    /// [`StaticDayPolicy::new`].
    #[must_use]
    pub const fn late_open(trade_date: NaiveDate, late_open_ssm: u32) -> Self {
        Self {
            trade_date,
            kind: DayOverrideKind::Modified {
                late_open_ssm: Some(late_open_ssm),
                early_close_ssm: None,
            },
        }
    }

    /// Delays the first open and moves the final close for `trade_date`.
    ///
    /// Both values are venue-local seconds since midnight and are validated by
    /// [`StaticDayPolicy::new`]. Their numeric order is deliberately not
    /// constrained: a wrapped trading day can open on the preceding local date
    /// at a numerically later wall clock than its final close on `trade_date`.
    #[must_use]
    pub const fn late_open_and_early_close(
        trade_date: NaiveDate,
        late_open_ssm: u32,
        early_close_ssm: u32,
    ) -> Self {
        Self {
            trade_date,
            kind: DayOverrideKind::Modified {
                late_open_ssm: Some(late_open_ssm),
                early_close_ssm: Some(early_close_ssm),
            },
        }
    }

    /// Returns the venue-local trade date this record modifies.
    #[must_use]
    pub const fn trade_date(self) -> NaiveDate {
        self.trade_date
    }

    /// Returns whether this record closes the complete trading day.
    #[must_use]
    pub const fn is_closed(self) -> bool {
        matches!(self.kind, DayOverrideKind::Closed)
    }

    /// Returns the replacement final-close SSM, if present.
    #[must_use]
    pub const fn early_close_ssm(self) -> Option<u32> {
        match self.kind {
            DayOverrideKind::Closed => None,
            DayOverrideKind::Modified {
                early_close_ssm, ..
            } => early_close_ssm,
        }
    }

    /// Returns the replacement first-open SSM, if present.
    #[must_use]
    pub const fn late_open_ssm(self) -> Option<u32> {
        match self.kind {
            DayOverrideKind::Closed => None,
            DayOverrideKind::Modified { late_open_ssm, .. } => late_open_ssm,
        }
    }
}

/// An allocation-free [`DayPolicy`] over caller-owned, sorted records.
///
/// Records are keyed by venue-local **trade date**, not calendar date. Missing
/// dates retain the normal-week schedule. Construction validates the entire
/// slice once; queries then use exact-date binary search without allocation,
/// recurrence inference, I/O, or clock reads.
///
/// This type stores no operator data and makes no source or coverage claim. It
/// represents only the closed/early-close/late-open vocabulary of
/// [`DayPolicy`], not arbitrary intraday topology changes.
///
/// # Example
///
/// ```
/// use chrono::NaiveDate;
/// use exchange_hours::{DayOverride, DayPolicy, StaticDayPolicy};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let trade_date = NaiveDate::from_ymd_opt(2026, 4, 20).ok_or("invalid date")?;
/// let records = [DayOverride::closed(trade_date)];
/// let policy = StaticDayPolicy::new(&records)?;
///
/// assert!(policy.is_closed(trade_date));
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticDayPolicy<'a> {
    overrides: &'a [DayOverride],
}

impl<'a> StaticDayPolicy<'a> {
    /// Validates and borrows a slice of trade-date overrides.
    ///
    /// An empty slice is valid and behaves like [`super::NoPolicy`]. Records
    /// must be strictly increasing by trade date, so duplicates are rejected.
    /// Early closes use `0..=86_400`; late opens use `0..86_400`.
    /// This is a `const fn`, so callers can evaluate validation while constructing
    /// a constant or static policy.
    ///
    /// Validation scans from left to right. For each record it checks ordering,
    /// then the early close, then the late open, returning the first violation.
    ///
    /// # Errors
    ///
    /// Returns [`StaticDayPolicyError`] for the first duplicate or out-of-order
    /// date, early close outside `0..=86_400`, or late open outside
    /// `0..86_400`.
    pub const fn new(overrides: &'a [DayOverride]) -> Result<Self, StaticDayPolicyError> {
        let mut index = 0;
        while index < overrides.len() {
            let current = overrides[index];
            if index > 0
                && current
                    .trade_date
                    .signed_duration_since(overrides[index - 1].trade_date)
                    .num_days()
                    <= 0
            {
                return Err(StaticDayPolicyError::DatesNotStrictlyIncreasing { index });
            }
            if let Some(early_close_ssm) = current.early_close_ssm()
                && early_close_ssm > SECONDS_PER_DAY
            {
                return Err(StaticDayPolicyError::EarlyCloseOutOfRange {
                    index,
                    early_close_ssm,
                });
            }
            if let Some(late_open_ssm) = current.late_open_ssm()
                && late_open_ssm >= SECONDS_PER_DAY
            {
                return Err(StaticDayPolicyError::LateOpenOutOfRange {
                    index,
                    late_open_ssm,
                });
            }
            index += 1;
        }
        Ok(Self { overrides })
    }

    /// Returns the exact override for `trade_date`, if one exists.
    ///
    /// Lookup is allocation-free and logarithmic in the number of records.
    #[must_use]
    pub fn override_on(self, trade_date: NaiveDate) -> Option<DayOverride> {
        self.overrides
            .binary_search_by_key(&trade_date, |record| record.trade_date)
            .ok()
            .and_then(|index| self.overrides.get(index).copied())
    }

    /// Returns the validated records in their strictly increasing order.
    #[must_use]
    pub const fn overrides(self) -> &'a [DayOverride] {
        self.overrides
    }
}

impl DayPolicy for StaticDayPolicy<'_> {
    fn is_closed(&self, trade_date: NaiveDate) -> bool {
        self.override_on(trade_date)
            .is_some_and(DayOverride::is_closed)
    }

    fn early_close_ssm(&self, trade_date: NaiveDate) -> Option<u32> {
        self.override_on(trade_date)
            .and_then(DayOverride::early_close_ssm)
    }

    fn late_open_ssm(&self, trade_date: NaiveDate) -> Option<u32> {
        self.override_on(trade_date)
            .and_then(DayOverride::late_open_ssm)
    }
}

/// A [`StaticDayPolicy`] record-set invariant violation.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaticDayPolicyError {
    /// The record at `index` is not later than its predecessor.
    DatesNotStrictlyIncreasing {
        /// Index of the duplicate or out-of-order record.
        index: usize,
    },
    /// An early close is outside `0..=86_400`.
    EarlyCloseOutOfRange {
        /// Index of the invalid record.
        index: usize,
        /// Rejected early-close value.
        early_close_ssm: u32,
    },
    /// A late open is outside `0..86_400`.
    LateOpenOutOfRange {
        /// Index of the invalid record.
        index: usize,
        /// Rejected late-open value.
        late_open_ssm: u32,
    },
}

impl core::fmt::Display for StaticDayPolicyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DatesNotStrictlyIncreasing { index } => {
                write!(
                    f,
                    "override at index {index} is not later than its predecessor"
                )
            }
            Self::EarlyCloseOutOfRange {
                index,
                early_close_ssm,
            } => write!(
                f,
                "early_close_ssm {early_close_ssm} at index {index} is outside 0..=86400"
            ),
            Self::LateOpenOutOfRange {
                index,
                late_open_ssm,
            } => write!(
                f,
                "late_open_ssm {late_open_ssm} at index {index} is outside 0..86400"
            ),
        }
    }
}

impl std::error::Error for StaticDayPolicyError {}
