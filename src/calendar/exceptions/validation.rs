// SPDX-License-Identifier: MIT-0

//! The one set of replacement-block shape rules, shared by both layers.
//!
//! A caller's [`StaticSessionExceptions`](crate::StaticSessionExceptions) record
//! and a built-in [`HolidayKind::ReplacementBlocks`](crate::HolidayKind::ReplacementBlocks)
//! row state the same thing — a complete ordered set of [`ExceptionBlock`]s
//! replacing one venue-local trade date — so one function validates both rather
//! than two that have to be kept in step by hand. The caller's layer reports the
//! violation as a
//! [`StaticSessionExceptionsError`](crate::StaticSessionExceptionsError); the
//! built-in fence turns it into a build failure. Neither layer states a rule of
//! its own, so a rule added here reaches both, and the caller-facing tests that
//! fence these rules fence the built-in rows' rules as well.

use super::ExceptionBlock;

/// The upper bound of a venue-local seconds-since-midnight close.
///
/// `86_400` is the end-exclusive close of a complete local day and is a legal
/// close; the matching open bound is one second lower, exactly as
/// [`StaticDayPolicy`](crate::StaticDayPolicy) validates a caller's scalar
/// records.
const SECONDS_PER_DAY: u32 = 86_400;

/// Why a replacement block set is not well formed.
///
/// The variants carry the offending block's position so both callers can name
/// it: the error a caller sees and the message a build failure prints both point
/// at the block that broke the rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BlockViolation {
    /// The set is empty. A trade date with no session is a closure instead.
    Empty,
    /// A block's opening-day offset is outside the permitted range.
    OffsetOutOfRange {
        /// Position of the invalid block inside the set.
        block: usize,
        /// Rejected opening-day offset.
        open_day_offset: i8,
    },
    /// A block's open is outside `0..86_400`.
    OpenOutOfRange {
        /// Position of the invalid block inside the set.
        block: usize,
        /// Rejected open value.
        open_ssm: u32,
    },
    /// A block's close is outside `0..=86_400`.
    CloseOutOfRange {
        /// Position of the invalid block inside the set.
        block: usize,
        /// Rejected close value.
        close_ssm: u32,
    },
    /// A block opening on its own trade date closes on the following local day.
    ///
    /// A trade date is named by the local date of its final close, so a block at
    /// offset `0` may not wrap. A span covering one whole local day is stated as
    /// `open_ssm = 0`, `close_ssm = 86_400`, which does not wrap.
    ClosesAfterTradeDate {
        /// Position of the wrapping block inside the set.
        block: usize,
    },
    /// A block does not start at or after its predecessor.
    NotOrdered {
        /// Position of the out-of-order block inside the set.
        block: usize,
    },
}

/// Returns the first reason `blocks` is not a well-formed replacement set.
///
/// Blocks are visited in order and, within one block, in the order the rules are
/// written here, so a set with several faults always reports the same one: the
/// earliest block, and within it the earliest rule. Both callers depend on that
/// being deterministic — a caller reads it as a validation error and a build
/// reads it as the reason the constant evaluation stopped.
///
/// The rules are the ones a replacement set has always been held to. They are
/// stated once, here, so the built-in table cannot drift from the caller's
/// records: the ordering rule is the order every replacement scan relies on, the
/// instant ranges are the `DayPolicy` ranges, and the no-wrap rule at offset `0`
/// is what keeps a trade date from closing after its own name.
///
/// The ordering rule is **non-decreasing**, deliberately: it rejects a block
/// that starts before its predecessor and accepts one that starts at the same
/// `(open_day_offset, open_ssm)`. Two blocks may legitimately share an opening
/// instant when they state different kinds — an order-entry phase and the
/// tradeable session that begins with it — so equality is not a duplicate. See
/// [`DateException::ReplaceSessions`](crate::DateException::ReplaceSessions),
/// which states the same contract to callers.
pub(crate) const fn first_block_violation(blocks: &[ExceptionBlock]) -> Option<BlockViolation> {
    if blocks.is_empty() {
        return Some(BlockViolation::Empty);
    }
    let mut block = 0;
    while block < blocks.len() {
        let current = blocks[block];
        if current.open_day_offset() < ExceptionBlock::MIN_DAY_OFFSET
            || current.open_day_offset() > ExceptionBlock::MAX_DAY_OFFSET
        {
            return Some(BlockViolation::OffsetOutOfRange {
                block,
                open_day_offset: current.open_day_offset(),
            });
        }
        if current.open_ssm() >= SECONDS_PER_DAY {
            return Some(BlockViolation::OpenOutOfRange {
                block,
                open_ssm: current.open_ssm(),
            });
        }
        if current.close_ssm() > SECONDS_PER_DAY {
            return Some(BlockViolation::CloseOutOfRange {
                block,
                close_ssm: current.close_ssm(),
            });
        }
        if current.open_day_offset() == 0 && current.wraps_to_next_day() {
            return Some(BlockViolation::ClosesAfterTradeDate { block });
        }
        if block > 0 {
            let previous = blocks[block - 1];
            if current.open_day_offset() < previous.open_day_offset()
                || (current.open_day_offset() == previous.open_day_offset()
                    && current.open_ssm() < previous.open_ssm())
            {
                return Some(BlockViolation::NotOrdered { block });
            }
        }
        block += 1;
    }
    None
}
