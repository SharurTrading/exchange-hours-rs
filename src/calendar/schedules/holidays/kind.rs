// SPDX-License-Identifier: MIT-0

//! What a built-in table can record about one venue-local trade date.

use crate::calendar::exceptions::ExceptionBlock;

/// What a built-in table records about one venue-local trade date.
///
/// The vocabulary is mostly the scalar vocabulary of
/// [`DayPolicy`](crate::DayPolicy), with one deliberate exception: a special day
/// whose *internal* phase topology changes cannot be expressed as boundary
/// scalars, so [`Self::ReplacementBlocks`] carries a complete ordered block set
/// instead. That set resolves through the same replacement layer a caller's
/// [`SessionExceptionSource`](crate::SessionExceptionSource) uses, so the
/// built-in table and a caller's records state a topology in one vocabulary.
///
/// A special session the table still cannot state — one whose halt instant the
/// operator never published — remains a gap in the owner's evidence file rather
/// than an approximation, however plainly the new representation could encode a
/// hypothetical value.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HolidayKind {
    /// No session belongs to this trade date.
    ///
    /// This normally removes the complete trading day, including a session that
    /// opened the previous evening. A sourced following-business-day family —
    /// CME cryptocurrency — instead rolls its continuous trading past the date,
    /// so the row means "there is no such trade date", not "trading stopped".
    Closed,
    /// The trade date's final close moves earlier.
    EarlyClose {
        /// The replacement final close, in venue-local seconds since midnight
        /// on the trade date, in `0..=86_400`.
        close_ssm: u32,
    },
    /// The trade date's first open moves later.
    LateOpen {
        /// The replacement first open, in venue-local seconds since midnight,
        /// in `0..86_400`. For a wrapped trading day a value at or after the
        /// day's normal first open is interpreted on the **preceding** local
        /// date, exactly as
        /// [`DayPolicy::late_open_ssm`](crate::DayPolicy::late_open_ssm)
        /// interprets a caller's.
        open_ssm: u32,
    },
    /// Both boundaries move.
    LateOpenAndEarlyClose {
        /// The replacement first open; see [`Self::LateOpen`].
        open_ssm: u32,
        /// The replacement final close; see [`Self::EarlyClose`].
        close_ssm: u32,
    },
    /// The trade date is replaced outright by a complete ordered set of blocks.
    ///
    /// This is the vocabulary for a special session whose *internal* phase
    /// topology changes: a trading day that pauses and reopens, one that adds a
    /// session on a normally empty day, one whose regular session ends while
    /// extended trading continues, or one whose blocks carry a following
    /// business date. The slice is ordered by opening day and then by open time,
    /// and it is never empty; a trade date with no blocks is [`Self::Closed`].
    /// The order is non-decreasing rather than strictly increasing, so two
    /// blocks of different kinds may share one opening instant.
    ///
    /// A block row replaces the **complete** trade date: no scalar clip from the
    /// same row is applied on top, and every query family resolves the
    /// replacement through the one path
    /// [`DateException::ReplaceSessions`](crate::DateException::ReplaceSessions)
    /// already drives, so `is_open`, the boundaries, the trade date and the
    /// candle edges cannot disagree about it. An explicit caller `Closed` or
    /// `ReplaceSessions` record for the date wins over the built-in arrangement,
    /// and the caller's [`DayPolicy`](crate::DayPolicy) then clips whatever
    /// survives.
    ReplacementBlocks(&'static [ExceptionBlock]),
    /// The date is inside the table's coverage window and is **not** audited
    /// normal: the operator published nothing this crate could read.
    ///
    /// This changes no answer — it clips nothing, exactly like
    /// [`DateException::OutOfCoverage`](crate::DateException::OutOfCoverage) —
    /// and exists so a contiguous coverage window does not have to choose
    /// between claiming an unknown date is normal and halving the window.
    Unsourced,
}
