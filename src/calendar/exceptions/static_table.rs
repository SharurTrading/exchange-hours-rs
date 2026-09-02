// SPDX-License-Identifier: MIT-0

//! Allocation-free exception providers backed by caller-owned static records.

use chrono::NaiveDate;

use super::{DateException, ExceptionBlock, ExceptionCoverage, SessionExceptionSource};
use crate::calendar::exchange_calendar::CalendarSource;

const SECONDS_PER_DAY: u32 = 86_400;

/// One caller-supplied record for a venue-local trade date.
///
/// The three constructors are mutually exclusive by construction: a record
/// either states that the date is normal, that it carries no session at all, or
/// that a complete ordered block set replaces it. Nothing can be closed and
/// replaced at once, and no record carries scalar boundary edits — those belong
/// to [`DayPolicy`](crate::DayPolicy), which overlays this layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionExceptionRecord<'a> {
    trade_date: NaiveDate,
    state: RecordState<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RecordState<'a> {
    KnownNormal,
    Closed,
    ReplaceSessions(&'a [ExceptionBlock]),
}

impl<'a> SessionExceptionRecord<'a> {
    /// Records that `trade_date` was audited and follows the normal week.
    ///
    /// Explicit normal records are optional: every uncovered-by-a-record date
    /// inside a table's coverage window already reads as
    /// [`DateException::KnownNormal`]. They document an audit that found
    /// nothing.
    #[must_use]
    pub const fn known_normal(trade_date: NaiveDate) -> Self {
        Self {
            trade_date,
            state: RecordState::KnownNormal,
        }
    }

    /// Records that no session belongs to `trade_date`.
    #[must_use]
    pub const fn closed(trade_date: NaiveDate) -> Self {
        Self {
            trade_date,
            state: RecordState::Closed,
        }
    }

    /// Records the complete ordered block set that replaces `trade_date`.
    ///
    /// The slice is validated by [`StaticSessionExceptions::new`]: it must be
    /// non-empty, ordered by opening day then open time, and every block must
    /// hold the [`ExceptionBlock`] domain invariants.
    #[must_use]
    pub const fn replace_sessions(trade_date: NaiveDate, blocks: &'a [ExceptionBlock]) -> Self {
        Self {
            trade_date,
            state: RecordState::ReplaceSessions(blocks),
        }
    }

    /// Returns the venue-local trade date this record describes.
    #[must_use]
    pub const fn trade_date(self) -> NaiveDate {
        self.trade_date
    }

    /// Returns this record's state as a [`DateException`].
    ///
    /// A record never reports [`DateException::OutOfCoverage`]; that state
    /// belongs to dates a table has no record *and* no coverage for.
    #[must_use]
    pub const fn exception(self) -> DateException<'a> {
        match self.state {
            RecordState::KnownNormal => DateException::KnownNormal,
            RecordState::Closed => DateException::Closed,
            RecordState::ReplaceSessions(blocks) => DateException::ReplaceSessions(blocks),
        }
    }
}

/// An allocation-free [`SessionExceptionSource`] over caller-owned records.
///
/// Records are keyed by venue-local **trade date** and are exact dates, never
/// inferred weekday or holiday rules. Construction validates the whole slice
/// once; queries then use exact-date binary search without allocation, I/O, or
/// clock reads.
///
/// The table stores no operator data of its own. It carries the caller's
/// identity scope and audited window so the engine can refuse a provider built
/// for another schedule and so an unaudited date stays distinguishable from an
/// ordinary weekday.
///
/// # Example
///
/// ```
/// use chrono::NaiveDate;
/// use exchange_hours::{
///     CalendarSource, DateException, Exchange, ExceptionBlock, SessionExceptionRecord,
///     SessionExceptionSource, StaticSessionExceptions,
/// };
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let day = |d| NaiveDate::from_ymd_opt(2011, 11, d).ok_or("invalid date");
///
/// // A regular-only early close: extended trading continues past 13:00 local.
/// static BLOCKS: [ExceptionBlock; 2] = [
///     ExceptionBlock::regular(0, 9 * 3_600 + 30 * 60, 13 * 3_600),
///     ExceptionBlock::extended(0, 13 * 3_600, 17 * 3_600),
/// ];
/// let records = [SessionExceptionRecord::replace_sessions(day(25)?, &BLOCKS)];
/// let table = StaticSessionExceptions::new(
///     CalendarSource::Exchange(Exchange::Nasdaq),
///     day(21)?,
///     day(28)?,
///     &records,
/// )?;
///
/// assert_eq!(table.exception_on(day(25)?), DateException::ReplaceSessions(&BLOCKS));
/// assert_eq!(table.exception_on(day(24)?), DateException::KnownNormal);
/// assert_eq!(table.exception_on(day(29)?), DateException::OutOfCoverage);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticSessionExceptions<'a> {
    source: CalendarSource,
    coverage: ExceptionCoverage,
    records: &'a [SessionExceptionRecord<'a>],
}

impl<'a> StaticSessionExceptions<'a> {
    /// Validates and borrows a slice of trade-date records for one identity.
    ///
    /// `first_covered` and `last_covered` publish the audited window: dates
    /// inside it with no record are [`DateException::KnownNormal`], and dates
    /// outside it are [`DateException::OutOfCoverage`]. An empty record slice
    /// is valid and asserts that the whole window was audited and found normal.
    /// This is a `const fn`, so a caller can validate while constructing a
    /// constant or static table.
    ///
    /// Validation scans left to right, checking the coverage bounds, then each
    /// record's ordering and window membership, then each replacement block's
    /// ordering and domain, and returns the first violation.
    ///
    /// # Errors
    ///
    /// Returns [`StaticSessionExceptionsError`] for inverted coverage bounds,
    /// the first duplicate or out-of-order record date, the first record
    /// outside the coverage window, an empty replacement set, or the first
    /// replacement block that is out of order, carries an offset outside
    /// [`ExceptionBlock::MIN_DAY_OFFSET`]`..=`[`ExceptionBlock::MAX_DAY_OFFSET`],
    /// opens outside `0..86_400`, or closes outside `0..=86_400`.
    pub const fn new(
        source: CalendarSource,
        first_covered: NaiveDate,
        last_covered: NaiveDate,
        records: &'a [SessionExceptionRecord<'a>],
    ) -> Result<Self, StaticSessionExceptionsError> {
        if last_covered.signed_duration_since(first_covered).num_days() < 0 {
            return Err(StaticSessionExceptionsError::CoverageBoundsInverted);
        }
        let mut index = 0;
        while index < records.len() {
            let current = records[index];
            if index > 0
                && current
                    .trade_date
                    .signed_duration_since(records[index - 1].trade_date)
                    .num_days()
                    <= 0
            {
                return Err(StaticSessionExceptionsError::DatesNotStrictlyIncreasing { index });
            }
            if current
                .trade_date
                .signed_duration_since(first_covered)
                .num_days()
                < 0
                || last_covered
                    .signed_duration_since(current.trade_date)
                    .num_days()
                    < 0
            {
                return Err(StaticSessionExceptionsError::RecordOutsideCoverage { index });
            }
            if let RecordState::ReplaceSessions(blocks) = current.state
                && let Err(error) = validate_blocks(index, blocks)
            {
                return Err(error);
            }
            index += 1;
        }
        Ok(Self {
            source,
            coverage: ExceptionCoverage {
                first: first_covered,
                last: last_covered,
            },
            records,
        })
    }

    /// Returns the identity these records are scoped to.
    #[must_use]
    pub const fn source(self) -> CalendarSource {
        self.source
    }

    /// Returns the audited trade-date window.
    #[must_use]
    pub const fn coverage(self) -> ExceptionCoverage {
        self.coverage
    }

    /// Returns the exact record for `trade_date`, if one exists.
    ///
    /// Lookup is allocation-free and logarithmic in the number of records. A
    /// date outside the coverage window still returns its record if one was
    /// supplied, which validation makes impossible.
    #[must_use]
    pub fn record_on(self, trade_date: NaiveDate) -> Option<SessionExceptionRecord<'a>> {
        self.records
            .binary_search_by_key(&trade_date, |record| record.trade_date)
            .ok()
            .and_then(|index| self.records.get(index).copied())
    }

    /// Returns what this table knows about `trade_date`.
    #[must_use]
    pub fn exception_on(self, trade_date: NaiveDate) -> DateException<'a> {
        if !self.coverage.contains(trade_date) {
            return DateException::OutOfCoverage;
        }
        self.record_on(trade_date).map_or(
            DateException::KnownNormal,
            SessionExceptionRecord::exception,
        )
    }

    /// Returns the validated records in their strictly increasing order.
    #[must_use]
    pub const fn records(self) -> &'a [SessionExceptionRecord<'a>] {
        self.records
    }
}

const fn validate_blocks(
    index: usize,
    blocks: &[ExceptionBlock],
) -> Result<(), StaticSessionExceptionsError> {
    if blocks.is_empty() {
        return Err(StaticSessionExceptionsError::EmptyReplacement { index });
    }
    let mut block = 0;
    while block < blocks.len() {
        let current = blocks[block];
        if current.open_day_offset() < ExceptionBlock::MIN_DAY_OFFSET
            || current.open_day_offset() > ExceptionBlock::MAX_DAY_OFFSET
        {
            return Err(StaticSessionExceptionsError::BlockOffsetOutOfRange {
                index,
                block,
                open_day_offset: current.open_day_offset(),
            });
        }
        if current.open_ssm() >= SECONDS_PER_DAY {
            return Err(StaticSessionExceptionsError::BlockOpenOutOfRange {
                index,
                block,
                open_ssm: current.open_ssm(),
            });
        }
        if current.close_ssm() > SECONDS_PER_DAY {
            return Err(StaticSessionExceptionsError::BlockCloseOutOfRange {
                index,
                block,
                close_ssm: current.close_ssm(),
            });
        }
        if current.open_day_offset() == 0 && current.wraps_to_next_day() {
            return Err(StaticSessionExceptionsError::BlockClosesAfterTradeDate { index, block });
        }
        if block > 0 {
            let previous = blocks[block - 1];
            if current.open_day_offset() < previous.open_day_offset()
                || (current.open_day_offset() == previous.open_day_offset()
                    && current.open_ssm() < previous.open_ssm())
            {
                return Err(StaticSessionExceptionsError::BlocksNotOrdered { index, block });
            }
        }
        block += 1;
    }
    Ok(())
}

impl SessionExceptionSource for StaticSessionExceptions<'_> {
    fn source(&self) -> CalendarSource {
        StaticSessionExceptions::source(*self)
    }

    fn coverage(&self) -> Option<ExceptionCoverage> {
        Some(StaticSessionExceptions::coverage(*self))
    }

    fn exception_on(&self, trade_date: NaiveDate) -> DateException<'_> {
        StaticSessionExceptions::exception_on(*self, trade_date)
    }
}

/// A [`StaticSessionExceptions`] record-set invariant violation.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaticSessionExceptionsError {
    /// The last covered trade date precedes the first.
    CoverageBoundsInverted,
    /// The record at `index` is not later than its predecessor.
    DatesNotStrictlyIncreasing {
        /// Index of the duplicate or out-of-order record.
        index: usize,
    },
    /// The record at `index` falls outside the published coverage window.
    RecordOutsideCoverage {
        /// Index of the out-of-window record.
        index: usize,
    },
    /// The replacement at `index` carries no blocks; use a closed record.
    EmptyReplacement {
        /// Index of the empty replacement record.
        index: usize,
    },
    /// A block's opening-day offset is outside the permitted range.
    BlockOffsetOutOfRange {
        /// Index of the owning record.
        index: usize,
        /// Position of the invalid block inside that record.
        block: usize,
        /// Rejected opening-day offset.
        open_day_offset: i8,
    },
    /// A block's open is outside `0..86_400`.
    BlockOpenOutOfRange {
        /// Index of the owning record.
        index: usize,
        /// Position of the invalid block inside that record.
        block: usize,
        /// Rejected open value.
        open_ssm: u32,
    },
    /// A block's close is outside `0..=86_400`.
    BlockCloseOutOfRange {
        /// Index of the owning record.
        index: usize,
        /// Position of the invalid block inside that record.
        block: usize,
        /// Rejected close value.
        close_ssm: u32,
    },
    /// A block opening on its trade date closes on the following local date.
    ///
    /// A trade date is named by the local date of its final close, so a block
    /// at offset `0` may not wrap. A span covering one whole local day is
    /// stated as `open_ssm = 0`, `close_ssm = 86_400`, which does not wrap.
    BlockClosesAfterTradeDate {
        /// Index of the owning record.
        index: usize,
        /// Position of the wrapping block inside that record.
        block: usize,
    },
    /// A block does not start at or after its predecessor.
    BlocksNotOrdered {
        /// Index of the owning record.
        index: usize,
        /// Position of the out-of-order block inside that record.
        block: usize,
    },
}

impl core::fmt::Display for StaticSessionExceptionsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CoverageBoundsInverted => {
                write!(f, "last covered trade date precedes the first")
            }
            Self::DatesNotStrictlyIncreasing { index } => write!(
                f,
                "record at index {index} is not later than its predecessor"
            ),
            Self::RecordOutsideCoverage { index } => write!(
                f,
                "record at index {index} falls outside the published coverage window"
            ),
            Self::EmptyReplacement { index } => write!(
                f,
                "replacement record at index {index} carries no blocks; record it as closed"
            ),
            Self::BlockOffsetOutOfRange {
                index,
                block,
                open_day_offset,
            } => write!(
                f,
                "block {block} of record {index} has open_day_offset {open_day_offset} outside {}..={}",
                ExceptionBlock::MIN_DAY_OFFSET,
                ExceptionBlock::MAX_DAY_OFFSET
            ),
            Self::BlockOpenOutOfRange {
                index,
                block,
                open_ssm,
            } => write!(
                f,
                "block {block} of record {index} has open_ssm {open_ssm} outside 0..86400"
            ),
            Self::BlockCloseOutOfRange {
                index,
                block,
                close_ssm,
            } => write!(
                f,
                "block {block} of record {index} has close_ssm {close_ssm} outside 0..=86400"
            ),
            Self::BlockClosesAfterTradeDate { index, block } => write!(
                f,
                "block {block} of record {index} opens on its trade date and closes on the next local date"
            ),
            Self::BlocksNotOrdered { index, block } => write!(
                f,
                "block {block} of record {index} starts before its predecessor"
            ),
        }
    }
}

impl std::error::Error for StaticSessionExceptionsError {}
