// SPDX-License-Identifier: MIT-0

//! Validated caller outcomes for one trade date.

const SECONDS_PER_DAY: u32 = 86_400;

/// The effect of a caller's day policy on one trade date.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayPolicyOutcome {
    /// Leave the date's scheduled boundaries as they are.
    Normal,
    /// Close the complete trading day.
    Closed,
    /// Tighten one or both of the trading day's outer boundaries.
    Adjusted(DayAdjustment),
}

impl DayPolicyOutcome {
    /// Returns whether this policy explicitly closes the trading day.
    #[must_use]
    pub const fn is_closed(self) -> bool {
        matches!(self, Self::Closed)
    }

    /// Returns the final-close cutoff in local seconds since midnight, if set.
    #[must_use]
    pub const fn early_close_ssm(self) -> Option<u32> {
        match self {
            Self::Adjusted(adjustment) => adjustment.early_close_ssm(),
            Self::Normal | Self::Closed => None,
        }
    }

    /// Returns the first-open cutoff in local seconds since midnight, if set.
    #[must_use]
    pub const fn late_open_ssm(self) -> Option<u32> {
        match self {
            Self::Adjusted(adjustment) => adjustment.late_open_ssm(),
            Self::Normal | Self::Closed => None,
        }
    }
}

/// Validated outer-boundary cutoffs for a trade date.
///
/// Cutoffs are venue-local seconds since midnight. A close of `86_400` means
/// the following midnight. An open is strictly before `86_400`. For a wrapped
/// trade date, a late open at or after the normal first-open wall clock is
/// interpreted on the preceding opening date. Smaller values are interpreted
/// on the trade date.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DayAdjustment {
    late_open_ssm: Option<u32>,
    early_close_ssm: Option<u32>,
}

impl DayAdjustment {
    /// Moves the final close earlier.
    ///
    /// # Errors
    ///
    /// Returns [`DayAdjustmentError::EarlyCloseOutOfRange`] above `86_400`.
    pub const fn early_close(early_close_ssm: u32) -> Result<Self, DayAdjustmentError> {
        if early_close_ssm > SECONDS_PER_DAY {
            return Err(DayAdjustmentError::EarlyCloseOutOfRange { early_close_ssm });
        }
        Ok(Self {
            late_open_ssm: None,
            early_close_ssm: Some(early_close_ssm),
        })
    }

    /// Delays the first open.
    ///
    /// # Errors
    ///
    /// Returns [`DayAdjustmentError::LateOpenOutOfRange`] at or above `86_400`.
    pub const fn late_open(late_open_ssm: u32) -> Result<Self, DayAdjustmentError> {
        if late_open_ssm >= SECONDS_PER_DAY {
            return Err(DayAdjustmentError::LateOpenOutOfRange { late_open_ssm });
        }
        Ok(Self {
            late_open_ssm: Some(late_open_ssm),
            early_close_ssm: None,
        })
    }

    /// Delays the first open and moves the final close earlier.
    ///
    /// The cutoffs may have any numeric order. A wrapped day can open on an
    /// earlier civil date. When the resolved cutoffs meet or cross, the clip
    /// removes the affected sessions.
    ///
    /// # Errors
    ///
    /// Returns [`DayAdjustmentError`] if either cutoff is out of range,
    /// checking the early close first.
    pub const fn late_open_and_early_close(
        late_open_ssm: u32,
        early_close_ssm: u32,
    ) -> Result<Self, DayAdjustmentError> {
        if let Err(error) = Self::early_close(early_close_ssm) {
            return Err(error);
        }
        if let Err(error) = Self::late_open(late_open_ssm) {
            return Err(error);
        }
        Ok(Self {
            late_open_ssm: Some(late_open_ssm),
            early_close_ssm: Some(early_close_ssm),
        })
    }

    /// Returns the final-close cutoff, if set.
    #[must_use]
    pub const fn early_close_ssm(self) -> Option<u32> {
        self.early_close_ssm
    }

    /// Returns the first-open cutoff, if set.
    #[must_use]
    pub const fn late_open_ssm(self) -> Option<u32> {
        self.late_open_ssm
    }
}

/// A raw boundary rejected before it can become a policy adjustment.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayAdjustmentError {
    /// An early close is outside `0..=86_400`.
    EarlyCloseOutOfRange {
        /// Rejected early-close value.
        early_close_ssm: u32,
    },
    /// A late open is outside `0..86_400`.
    LateOpenOutOfRange {
        /// Rejected late-open value.
        late_open_ssm: u32,
    },
}

impl core::fmt::Display for DayAdjustmentError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EarlyCloseOutOfRange { early_close_ssm } => {
                write!(f, "early close {early_close_ssm} is outside 0..=86400")
            }
            Self::LateOpenOutOfRange { late_open_ssm } => {
                write!(f, "late open {late_open_ssm} is outside 0..86400")
            }
        }
    }
}

impl std::error::Error for DayAdjustmentError {}
