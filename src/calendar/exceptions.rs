// SPDX-License-Identifier: MIT-0

//! Caller-owned exception sessions: complete replacement trading days.
//!
//! [`DayPolicy`](super::DayPolicy) clips a normal-week trading day's outer
//! boundaries. Some published special days cannot be reduced to those scalars:
//! a trade date that pauses and reopens, one whose regular session ends while
//! extended trading continues, or one whose blocks span several civil dates.
//! This layer replaces such a trade date outright with an ordered set of
//! [`ExceptionBlock`] values.
//!
//! The crate ships **no** exception data. A caller supplies a
//! [`SessionExceptionSource`] — usually the validated, allocation-free
//! [`StaticSessionExceptions`] table — and attaches it with
//! [`ExchangeCalendar::with_session_exceptions`](super::ExchangeCalendar::with_session_exceptions).
//!
//! Four states are distinguished per venue-local trade date, so a date outside
//! the audited window can never be mistaken for one audited and found normal:
//! [`DateException::KnownNormal`], [`DateException::Closed`],
//! [`DateException::ReplaceSessions`], and [`DateException::OutOfCoverage`].
//!
//! Precedence is fixed: the exception layer resolves the trading day, then the
//! caller's [`DayPolicy`](super::DayPolicy) overlays it exactly as it overlays
//! a normal week. Two replacement layers never compose — attaching a provider
//! replaces any provider already attached.

mod static_table;

pub use static_table::{
    SessionExceptionRecord, StaticSessionExceptions, StaticSessionExceptionsError,
};

use chrono::NaiveDate;

use super::exchange_calendar::CalendarSource;

/// Which of a profile's rule sets a replacement block belongs to.
///
/// The three variants mirror [`MarketHours`](super::MarketHours)'s `regular`,
/// `extended`, and `order_entry` sets, so a replaced trade date can restate a
/// complete trading day rather than only its tradeable part.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExceptionBlockKind {
    /// A primary/RTH session; a trade can print.
    Regular,
    /// An electronic, overnight, or auction session; a trade can print.
    Extended,
    /// An order-entry-only phase in which no trade can match.
    OrderEntry,
}

/// One block of a replacement trading day.
///
/// A block is stated exactly like a [`SessionRule`](super::SessionRule): venue-
/// local seconds since midnight, end-exclusive close, and `open_ssm >=
/// close_ssm` meaning the block wraps into the next local day. Opens resolve to
/// the earliest valid instant across a DST fold and closes to the latest, the
/// same asymmetric bias normal profiles use.
///
/// `open_day_offset` places the block's **opening** local day relative to the
/// trade date the record is keyed by, so a Globex-style day that opens the
/// previous evening uses `-1`. Like a `SessionRule`, one block spans at most
/// one local midnight; continuous trading across more days is stated as
/// adjacent blocks, and whether those read back as one session follows the
/// calendar identity's normal adjacency convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExceptionBlock {
    kind: ExceptionBlockKind,
    open_day_offset: i8,
    open_ssm: u32,
    close_ssm: u32,
}

impl ExceptionBlock {
    /// The earliest opening day a block may declare, relative to its trade date.
    ///
    /// Seven days back covers every published arrangement this crate has seen,
    /// including a weekend block carried to the following open business date,
    /// and bounds the engine's per-day replacement scan.
    pub const MIN_DAY_OFFSET: i8 = -7;

    /// The latest opening day a block may declare, relative to its trade date.
    ///
    /// A trade date is named by the local date of its final close, so no block
    /// opens after it.
    pub const MAX_DAY_OFFSET: i8 = 0;

    /// Builds a tradeable regular block.
    #[must_use]
    pub const fn regular(open_day_offset: i8, open_ssm: u32, close_ssm: u32) -> Self {
        Self {
            kind: ExceptionBlockKind::Regular,
            open_day_offset,
            open_ssm,
            close_ssm,
        }
    }

    /// Builds a tradeable extended block.
    #[must_use]
    pub const fn extended(open_day_offset: i8, open_ssm: u32, close_ssm: u32) -> Self {
        Self {
            kind: ExceptionBlockKind::Extended,
            open_day_offset,
            open_ssm,
            close_ssm,
        }
    }

    /// Builds an order-entry-only block in which no trade can match.
    #[must_use]
    pub const fn order_entry(open_day_offset: i8, open_ssm: u32, close_ssm: u32) -> Self {
        Self {
            kind: ExceptionBlockKind::OrderEntry,
            open_day_offset,
            open_ssm,
            close_ssm,
        }
    }

    /// Returns which rule set this block belongs to.
    #[must_use]
    pub const fn kind(self) -> ExceptionBlockKind {
        self.kind
    }

    /// Returns the opening local day relative to the record's trade date.
    #[must_use]
    pub const fn open_day_offset(self) -> i8 {
        self.open_day_offset
    }

    /// Returns the open time in venue-local seconds since midnight.
    #[must_use]
    pub const fn open_ssm(self) -> u32 {
        self.open_ssm
    }

    /// Returns the end-exclusive close in venue-local seconds since midnight.
    #[must_use]
    pub const fn close_ssm(self) -> u32 {
        self.close_ssm
    }

    /// Returns whether this block closes on the local day after it opens.
    ///
    /// Equal endpoints wrap, matching [`SessionRule`](super::SessionRule): they
    /// encode one complete local-day span rather than an empty interval.
    #[must_use]
    pub const fn wraps_to_next_day(self) -> bool {
        self.open_ssm >= self.close_ssm
    }
}

/// What a provider knows about one venue-local trade date.
///
/// The [`Self::OutOfCoverage`] state exists so a date the provider never
/// audited cannot silently read as an ordinary weekday. Runtime queries treat
/// it exactly like [`Self::KnownNormal`] — there is nothing else a
/// deterministic engine can do with an unknown date — so a caller that must not
/// trade on unaudited dates has to check
/// [`PolicyCalendar::session_exception_on`](super::PolicyCalendar::session_exception_on)
/// or the provider's own coverage window itself.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateException<'a> {
    /// The date was audited and carries no exception; the normal week applies.
    KnownNormal,
    /// No session belongs to this trade date.
    Closed,
    /// The complete ordered set of blocks that replaces this trade date.
    ///
    /// The slice is ordered by opening day and then open time, and it is never
    /// empty: a trade date with no blocks is [`Self::Closed`].
    ReplaceSessions(&'a [ExceptionBlock]),
    /// The provider has no authoritative answer for this date.
    OutOfCoverage,
}

/// The inclusive venue-local trade-date window a provider claims to have audited.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExceptionCoverage {
    first: NaiveDate,
    last: NaiveDate,
}

impl ExceptionCoverage {
    /// Builds a coverage window, or `None` when `last` precedes `first`.
    #[must_use]
    pub fn new(first: NaiveDate, last: NaiveDate) -> Option<Self> {
        (first <= last).then_some(Self { first, last })
    }

    /// Returns the first audited trade date.
    #[must_use]
    pub const fn first(self) -> NaiveDate {
        self.first
    }

    /// Returns the last audited trade date.
    #[must_use]
    pub const fn last(self) -> NaiveDate {
        self.last
    }

    /// Returns whether `trade_date` falls inside the audited window.
    #[must_use]
    pub fn contains(self, trade_date: NaiveDate) -> bool {
        self.first <= trade_date && trade_date <= self.last
    }
}

/// A caller-owned provider of replacement trading days.
///
/// Implementations must be deterministic and perform no I/O or clock reads.
/// Callers with hard-coded records should use [`StaticSessionExceptions`]
/// rather than implementing this trait themselves.
///
/// A provider is scoped to exactly one [`CalendarSource`]: attaching it to a
/// calendar with a different identity is rejected, because one venue's holiday
/// topology is never evidence about another's.
pub trait SessionExceptionSource: Send + Sync {
    /// Returns the single schedule identity these records describe.
    fn source(&self) -> CalendarSource;

    /// Returns the audited trade-date window, or `None` when nothing is audited.
    ///
    /// Every date outside this window must return
    /// [`DateException::OutOfCoverage`] from [`Self::exception_on`].
    fn coverage(&self) -> Option<ExceptionCoverage>;

    /// Returns what this provider knows about `trade_date`.
    ///
    /// Dates inside [`Self::coverage`] with no recorded exception return
    /// [`DateException::KnownNormal`].
    fn exception_on(&self, trade_date: NaiveDate) -> DateException<'_>;
}

/// A [`SessionExceptionSource`] attached to a calendar with another identity.
///
/// Returned by
/// [`ExchangeCalendar::with_session_exceptions`](super::ExchangeCalendar::with_session_exceptions)
/// instead of letting one schedule's exception data drive another's answers.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExceptionScopeError {
    /// The identity the calendar represents.
    pub calendar: CalendarSource,
    /// The identity the rejected provider is scoped to.
    pub provider: CalendarSource,
}

impl core::fmt::Display for ExceptionScopeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "session-exception provider is scoped to {:?}, not to this calendar's {:?}",
            self.provider, self.calendar
        )
    }
}

impl std::error::Error for ExceptionScopeError {}
