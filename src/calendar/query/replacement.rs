// SPDX-License-Identifier: MIT-0

//! Resolution of caller-supplied replacement trading days.
//!
//! A replacement record restates one venue-local trade date as a complete
//! ordered set of [`ExceptionBlock`]s. This module turns those blocks into the
//! same `(open, close)` UTC bounds the normal-week path produces, so every
//! status, boundary, trade-date and candle query treats them identically.
//!
//! Two invariants keep the scan bounded. A block spans at most one local
//! midnight, exactly like a [`SessionRule`](crate::SessionRule), so a
//! containment query still needs only today's and yesterday's opening days. And
//! a block's opening day sits within
//! [`ExceptionBlock::MIN_DAY_OFFSET`]`..=`[`ExceptionBlock::MAX_DAY_OFFSET`] of
//! its trade date, so the set of trade dates that can open on a given local day
//! is a fixed, small window.

use chrono::{DateTime, Duration, NaiveDate, Utc};

use super::schedule::{QueryContext, RuleSet};
use crate::calendar::exceptions::{DateException, ExceptionBlock, ExceptionBlockKind};
use crate::calendar::local_time::{bounded_utc, mk_local_close, mk_local_open};
use crate::calendar::rule::SessionKind;

const SECONDS_PER_DAY: u32 = 86_400;

/// The exception layer's answer for one trade date's final close.
pub(super) enum ExceptionDailyClose {
    /// The exception layer does not govern this trade date.
    NotGoverned,
    /// The trade date carries no session of the requested kind.
    NoSession,
    /// The trade date's final close of the requested kind.
    Close(DateTime<Utc>),
}

/// Returns whether a trade can print in this block.
pub(super) fn is_tradeable(block: ExceptionBlock) -> bool {
    matches!(
        block.kind(),
        ExceptionBlockKind::Regular | ExceptionBlockKind::Extended
    )
}

fn selects(block: ExceptionBlock, set: RuleSet) -> bool {
    match set {
        RuleSet::OrderEntry => matches!(block.kind(), ExceptionBlockKind::OrderEntry),
        RuleSet::Sessions(SessionKind::Regular) => {
            matches!(block.kind(), ExceptionBlockKind::Regular)
        }
        RuleSet::Sessions(SessionKind::Extended) => {
            matches!(block.kind(), ExceptionBlockKind::Extended)
        }
        RuleSet::Sessions(SessionKind::Both) => is_tradeable(block),
    }
}

fn opening_day(trade_date: NaiveDate, block: ExceptionBlock) -> Option<NaiveDate> {
    trade_date.checked_add_signed(Duration::days(i64::from(block.open_day_offset())))
}

/// Returns the local date a late-open override's wall clock belongs to.
///
/// This mirrors the normal path: for a wrapped trading day, a value at or after
/// the day's own first open is interpreted on the preceding opening date, and a
/// smaller value on the trade date itself.
fn late_open_day(trade_date: NaiveDate, blocks: &[ExceptionBlock], ssm: u32) -> NaiveDate {
    let Some(first) = blocks.iter().copied().find(|block| is_tradeable(*block)) else {
        return trade_date;
    };
    let Some(first_day) = opening_day(trade_date, first) else {
        return trade_date;
    };
    if first_day < trade_date && ssm >= first.open_ssm() {
        first_day
    } else {
        trade_date
    }
}

/// Resolves one replacement block, then applies the caller's `DayPolicy`.
///
/// Precedence is fixed and one-directional: the exception layer decides what
/// the trading day is, and the policy then clips it exactly as it clips a
/// normal week. `blocks` is the record's complete ordered set, needed only to
/// anchor a late-open override to the replaced day's own first open.
pub(super) fn resolve_block_bounds(
    context: &QueryContext<'_>,
    trade_date: NaiveDate,
    blocks: &[ExceptionBlock],
    block: ExceptionBlock,
) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    let open_day = opening_day(trade_date, block)?;
    let close_day = if block.wraps_to_next_day() {
        open_day.succ_opt()?
    } else {
        open_day
    };
    let tz = context.tz();
    let raw_open = mk_local_open(tz, open_day, block.open_ssm()).with_timezone(&Utc);
    let raw_close = mk_local_close(tz, close_day, block.close_ssm()).with_timezone(&Utc);
    if raw_open >= raw_close {
        return None;
    }
    if !context.has_daily_close_at(raw_open) {
        // A profile with no final daily close has no trade-date identity, so a
        // trade-date-keyed replacement has nothing to attach to. This is the
        // same refusal the `DayPolicy` path makes for the same reason.
        return None;
    }
    let Some(policy) = context.policy() else {
        return Some((raw_open, raw_close));
    };
    if policy.is_closed(trade_date) {
        return None;
    }
    let mut open = raw_open;
    let mut close = raw_close;
    if let Some(ssm) = policy.early_close_ssm(trade_date) {
        if ssm > SECONDS_PER_DAY {
            return None;
        }
        close = close.min(mk_local_close(tz, trade_date, ssm).with_timezone(&Utc));
    }
    if let Some(ssm) = policy.late_open_ssm(trade_date) {
        if ssm >= SECONDS_PER_DAY {
            return None;
        }
        let cutoff_day = late_open_day(trade_date, blocks, ssm);
        open = open.max(mk_local_open(tz, cutoff_day, ssm).with_timezone(&Utc));
    }
    (open < close).then_some((open, close))
}

/// Visits every replacement occurrence that opens on `open_day`.
///
/// Scans the fixed offset window rather than the record table, so the work per
/// opening day is bounded by the offset range and independent of how many
/// exception records the caller supplied.
///
/// The records this reads are the composed answer
/// [`QueryContext::exception_on`] gives, so a built-in table's replacement row
/// and a caller's `ReplaceSessions` record are served by this one scan and
/// cannot diverge between query families. The scan is entered only when some
/// layer can actually supply a replacement, which is one bit on the context.
///
/// This layer answers from records only, so it can never refuse a date: it
/// returns `Option` rather than the `Result` its caller uses, and the caller
/// applies its own coverage verdict to the day before asking.
pub(super) fn find_occurrence<T>(
    context: &QueryContext<'_>,
    open_day: NaiveDate,
    set: RuleSet,
    wrapped_only: bool,
    mut probe: impl FnMut(DateTime<Utc>, DateTime<Utc>) -> Option<T>,
) -> Option<T> {
    if !context.has_replacement_layer() {
        return None;
    }
    let mut offset = ExceptionBlock::MIN_DAY_OFFSET;
    while offset <= ExceptionBlock::MAX_DAY_OFFSET {
        if let Some(trade_date) = open_day.checked_sub_signed(Duration::days(i64::from(offset)))
            && let DateException::ReplaceSessions(blocks) = context.exception_on(trade_date)
        {
            for block in blocks.iter().copied().filter(|block| {
                block.open_day_offset() == offset
                    && selects(*block, set)
                    && (!wrapped_only || block.wraps_to_next_day())
            }) {
                if let Some((open, close)) =
                    resolve_block_bounds(context, trade_date, blocks, block)
                    && let Some(found) = probe(open, close)
                {
                    return Some(found);
                }
            }
        }
        offset += 1;
    }
    None
}

/// Returns whether an attached replacement block meets `window`.
///
/// A replacement block claims every instant inside its own resolved window.
/// That claim must be visible to the **normal-week** scan as well as to the
/// replacement scan, because a block may open on an instant that a normal-week
/// occurrence of a *different* trade date also opens on — the shape issue #130
/// records. The normal scan runs first, so without this test it answers such an
/// instant from the neighbouring trade date's occurrence while
/// [`replacement_trade_date`] assigns the block's own trade date to it, and a
/// caller pairing `session_bounds` with `trade_date` receives two different
/// sessions.
///
/// **What a replacement record means here.** A record states a complete trading
/// day, so a block that meets a normal occurrence takes that occurrence's place
/// rather than sharing the day with it: the occurrence is not reported, and the
/// caller is answered from the replacement. A record whose blocks cover less
/// than the normal week would otherwise leave a session with no stated opening,
/// which is not an arrangement any operator publishes — a short day is stated as
/// a block set that spans the day, not as a fragment of one. A caller that wants
/// the normal week around a block fills the day with blocks instead of
/// overlapping it.
///
/// The test is kind-aware: only a block of the same rule set displaces an
/// occurrence, so an `order_entry` block never deletes the tradeable session
/// opening at the same instant, and a replaced date stating only a tradeable
/// block keeps its ordinary queue.
///
/// Resolution decides containment rather than a raw wall-clock comparison, so
/// the test inherits the block's own wrap, DST bias and `DayPolicy` clip: a
/// block the policy removed governs nothing.
pub(super) fn governs_instant(
    context: &QueryContext<'_>,
    window: (DateTime<Utc>, DateTime<Utc>),
    set: RuleSet,
) -> bool {
    if !context.has_replacement_layer() {
        return false;
    }
    let (window_open, window_close) = window;
    let tz = context.tz();
    let first_day = bounded_utc(window_open, tz).with_timezone(&tz).date_naive();
    let last_day = bounded_utc(window_close, tz)
        .with_timezone(&tz)
        .date_naive();
    // A block meeting this occurrence opens in `[first_day + MIN, last_day +
    // MAX]`. Walking the trade dates that reach those opening days visits each
    // record once and asks it directly, rather than probing the record table
    // once per offset.
    let Some(first_trade_date) =
        first_day.checked_add_signed(Duration::days(i64::from(ExceptionBlock::MIN_DAY_OFFSET)))
    else {
        return false;
    };
    // The upper bound is `last_day - MIN_DAY_OFFSET`, not `last_day +
    // MAX_DAY_OFFSET`: a block opening on day `D` belongs to a trade date in
    // `[D, D - MIN_DAY_OFFSET]`, so a record keyed up to seven days after the
    // window can still reach into it with a negative offset. Bounding at
    // `last_day` alone misses every block that opens two or more local days
    // before its trade date, which left #130's collision reachable for exactly
    // the shapes a merged or multi-day trade date states.
    let Some(last_trade_date) =
        last_day.checked_sub_signed(Duration::days(i64::from(ExceptionBlock::MIN_DAY_OFFSET)))
    else {
        return false;
    };
    let mut trade_date = first_trade_date;
    loop {
        if let DateException::ReplaceSessions(blocks) = context.exception_on(trade_date) {
            // A block meeting this occurrence opens on one of its own local
            // days or on the day before it; the resolved comparison below
            // rejects the rest whatever they hold.
            let span = trade_date
                .signed_duration_since(first_day)
                .num_days()
                .clamp(0, i64::from(-ExceptionBlock::MIN_DAY_OFFSET));
            let mut offset = -i8::try_from(span).unwrap_or(ExceptionBlock::MIN_DAY_OFFSET);
            while offset <= ExceptionBlock::MAX_DAY_OFFSET {
                for block in blocks
                    .iter()
                    .copied()
                    .filter(|block| block.open_day_offset() == offset && selects(*block, set))
                {
                    if let Some((block_open, block_close)) =
                        resolve_block_bounds(context, trade_date, blocks, block)
                        && block_open < window_close
                        && block_close > window_open
                    {
                        return true;
                    }
                }
                offset += 1;
            }
        }
        if trade_date >= last_trade_date {
            return false;
        }
        let Some(next) = trade_date.succ_opt() else {
            return false;
        };
        trade_date = next;
    }
}

/// Returns the trade date a replacement block opening exactly at `open` carries.
///
/// A replacement record states its own trade-date assignment, so it overrides
/// every derived convention. The open instant identifies the block because a
/// normal-week occurrence sharing that instant either belongs to the replaced
/// date — and is therefore already suppressed by the normal scan's own
/// replaced-trade-date test — or belongs to another trade date, in which case
/// [`governs_instant`] kept that occurrence out of the normal scan's answer, so
/// this assignment names the session the caller was given.
pub(super) fn replacement_trade_date(
    context: &QueryContext<'_>,
    open: DateTime<Utc>,
) -> Option<NaiveDate> {
    if !context.has_replacement_layer() {
        return None;
    }
    let tz = context.tz();
    let local_day = bounded_utc(open, tz).with_timezone(&tz).date_naive();
    let mut offset = ExceptionBlock::MIN_DAY_OFFSET;
    while offset <= ExceptionBlock::MAX_DAY_OFFSET {
        if let Some(trade_date) = local_day.checked_sub_signed(Duration::days(i64::from(offset)))
            && let DateException::ReplaceSessions(blocks) = context.exception_on(trade_date)
        {
            for block in blocks
                .iter()
                .copied()
                .filter(|block| block.open_day_offset() == offset)
            {
                if resolve_block_bounds(context, trade_date, blocks, block)
                    .is_some_and(|(block_open, _close)| block_open == open)
                {
                    return Some(trade_date);
                }
            }
        }
        offset += 1;
    }
    None
}

/// Returns the exception layer's final close for `trade_date`, if it governs it.
///
/// A replaced trade date answers directly from its own blocks rather than
/// through the normal-week neighbour scan, because its blocks may open several
/// local days earlier than that scan reaches.
pub(super) fn daily_close(
    context: &QueryContext<'_>,
    trade_date: NaiveDate,
    kind: SessionKind,
) -> ExceptionDailyClose {
    // A profile with no final daily close has no trade-date identity, so a
    // trade-date-keyed record has nothing to attach to -- the same refusal
    // `resolve_block_bounds` makes one layer down. Without this the record's
    // blocks would each fail to resolve, the date would read as having no
    // session, and `is_closed_trade_date` would call an always-open venue
    // closed instead of deferring to the normal path.
    //
    // No test fences this today and none can: every shipped profile with no
    // daily close is also weekend-open, and on those the normal path finds no
    // daily close either, so both answers agree. The guard exists because that
    // agreement is a coincidence of the current tables, not a property of the
    // engine -- the first continuous-weekday profile would break it silently.
    let opening_midnight = mk_local_open(context.tz(), trade_date, 0).with_timezone(&Utc);
    if !context.has_daily_close_at(opening_midnight) {
        return ExceptionDailyClose::NotGoverned;
    }
    match context.exception_on(trade_date) {
        DateException::Closed => ExceptionDailyClose::NoSession,
        DateException::ReplaceSessions(blocks) => {
            let mut latest: Option<DateTime<Utc>> = None;
            for block in blocks
                .iter()
                .copied()
                .filter(|block| selects(*block, RuleSet::Sessions(kind)))
            {
                if let Some((_open, close)) =
                    resolve_block_bounds(context, trade_date, blocks, block)
                    && latest.is_none_or(|current| close > current)
                {
                    latest = Some(close);
                }
            }
            latest.map_or(ExceptionDailyClose::NoSession, ExceptionDailyClose::Close)
        }
        DateException::KnownNormal | DateException::OutOfCoverage => {
            ExceptionDailyClose::NotGoverned
        }
    }
}
