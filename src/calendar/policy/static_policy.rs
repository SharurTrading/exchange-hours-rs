// SPDX-License-Identifier: MIT-0

//! Allocation-free day policies backed by caller-owned static records.

use jiff::civil::Date;

use super::{DayAdjustment, DayAdjustmentError, DayPolicy, DayPolicyOutcome};

/// One validated override for a venue-local trade date.
///
/// An override can close the trading day, tighten its outer boundaries, or
/// combine both boundary changes. To replace or split intraday phases, use
/// [`StaticSessionExceptions`](crate::StaticSessionExceptions).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DayOverride {
    trade_date: Date,
    outcome: DayPolicyOutcome,
}

impl DayOverride {
    /// Closes the complete trading day assigned to `trade_date`.
    #[must_use]
    pub const fn closed(trade_date: Date) -> Self {
        Self {
            trade_date,
            outcome: DayPolicyOutcome::Closed,
        }
    }

    /// Applies an already validated adjustment to `trade_date`.
    #[must_use]
    pub const fn adjusted(trade_date: Date, adjustment: DayAdjustment) -> Self {
        Self {
            trade_date,
            outcome: DayPolicyOutcome::Adjusted(adjustment),
        }
    }

    /// Moves `trade_date`'s final close earlier.
    ///
    /// # Errors
    ///
    /// Returns [`DayAdjustmentError::EarlyCloseOutOfRange`] above `86_400`.
    pub const fn early_close(
        trade_date: Date,
        early_close_ssm: u32,
    ) -> Result<Self, DayAdjustmentError> {
        match DayAdjustment::early_close(early_close_ssm) {
            Ok(adjustment) => Ok(Self::adjusted(trade_date, adjustment)),
            Err(error) => Err(error),
        }
    }

    /// Delays `trade_date`'s first open.
    ///
    /// # Errors
    ///
    /// Returns [`DayAdjustmentError::LateOpenOutOfRange`] at or above `86_400`.
    pub const fn late_open(
        trade_date: Date,
        late_open_ssm: u32,
    ) -> Result<Self, DayAdjustmentError> {
        match DayAdjustment::late_open(late_open_ssm) {
            Ok(adjustment) => Ok(Self::adjusted(trade_date, adjustment)),
            Err(error) => Err(error),
        }
    }

    /// Delays the first open and moves the final close earlier for `trade_date`.
    ///
    /// # Errors
    ///
    /// Returns [`DayAdjustmentError`] if either cutoff is out of range. See
    /// [`DayAdjustment::late_open_and_early_close`] for wrapped trading days.
    pub const fn late_open_and_early_close(
        trade_date: Date,
        late_open_ssm: u32,
        early_close_ssm: u32,
    ) -> Result<Self, DayAdjustmentError> {
        match DayAdjustment::late_open_and_early_close(late_open_ssm, early_close_ssm) {
            Ok(adjustment) => Ok(Self::adjusted(trade_date, adjustment)),
            Err(error) => Err(error),
        }
    }

    /// Returns the venue-local trade date this record modifies.
    #[must_use]
    pub const fn trade_date(self) -> Date {
        self.trade_date
    }

    /// Returns this record's validated outcome.
    #[must_use]
    pub const fn outcome(self) -> DayPolicyOutcome {
        self.outcome
    }

    /// Returns whether this record closes the complete trading day.
    #[must_use]
    pub const fn is_closed(self) -> bool {
        self.outcome.is_closed()
    }

    /// Returns the replacement final-close SSM, if present.
    #[must_use]
    pub const fn early_close_ssm(self) -> Option<u32> {
        self.outcome.early_close_ssm()
    }

    /// Returns the replacement first-open SSM, if present.
    #[must_use]
    pub const fn late_open_ssm(self) -> Option<u32> {
        self.outcome.late_open_ssm()
    }
}

/// An allocation-free [`DayPolicy`] over caller-owned, sorted records.
///
/// Records are keyed by venue-local **trade date**. Missing dates retain the
/// schedule. Construction validates record order once. Queries use exact-date
/// binary search without allocation, recurrence inference, I/O, or clock reads.
///
/// This type stores no operator data and makes no source or coverage claim.
///
/// # Example
///
/// ```
/// use jiff::civil::Date;
/// use exchange_hours::{DayOverride, DayPolicy, DayPolicyOutcome, StaticDayPolicy};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let trade_date = Date::new(2026, 4, 20)?;
/// let records = [DayOverride::early_close(trade_date, 13 * 3_600)?];
/// let policy = StaticDayPolicy::new(&records)?;
///
/// assert!(matches!(policy.outcome_on(trade_date), DayPolicyOutcome::Adjusted(_)));
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticDayPolicy<'a> {
    overrides: &'a [DayOverride],
}

impl<'a> StaticDayPolicy<'a> {
    /// Validates the order of the borrowed trade-date overrides.
    ///
    /// An empty slice is valid and behaves like [`super::NoPolicy`]. Records
    /// must be strictly increasing by trade date. Each override has already
    /// validated its boundary values.
    ///
    /// # Errors
    ///
    /// Returns [`StaticDayPolicyError`] for the first duplicate or out-of-order
    /// date.
    pub fn new(overrides: &'a [DayOverride]) -> Result<Self, StaticDayPolicyError> {
        if let Some(index) = overrides
            .iter()
            .zip(overrides.iter().skip(1))
            .position(|(previous, current)| previous.trade_date >= current.trade_date)
        {
            return Err(StaticDayPolicyError::DatesNotStrictlyIncreasing { index: index + 1 });
        }
        Ok(Self { overrides })
    }

    /// Returns the exact override for `trade_date`, if one exists.
    ///
    /// Lookup is allocation-free and logarithmic in the number of records.
    #[must_use]
    pub fn override_on(self, trade_date: Date) -> Option<DayOverride> {
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
    fn outcome_on(&self, trade_date: Date) -> DayPolicyOutcome {
        self.override_on(trade_date)
            .map_or(DayPolicyOutcome::Normal, DayOverride::outcome)
    }

    fn may_affect(&self, first: Date, last: Date) -> bool {
        let start = self
            .overrides
            .partition_point(|record| record.trade_date < first);
        self.overrides
            .get(start)
            .is_some_and(|record| record.trade_date <= last)
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
        }
    }
}

impl std::error::Error for StaticDayPolicyError {}
