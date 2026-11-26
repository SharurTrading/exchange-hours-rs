// SPDX-License-Identifier: MIT-0

//! Resolves venue-local dates and wall-clock boundaries.
//!
//! An open uses the earlier mapping during a fold, and a close uses the later
//! mapping. Inside a gap both resolve to the first instant after the jump.

use jiff::civil::{Date, DateTime, Time};
use jiff::tz::{AmbiguousOffset, Offset, TimeZone};
use jiff::{SignedDuration, Timestamp};

/// Returns the venue-local date. Every Jiff timestamp has a civil
/// representation at every supported UTC offset.
#[inline]
pub(crate) fn local_date(tz: &TimeZone, instant: Timestamp) -> Date {
    tz.to_datetime(instant).date()
}

/// Resolves an opening wall clock, taking the earlier instant in a fold.
/// A boundary outside Jiff's representable range returns `None`.
#[inline]
pub(crate) fn mk_local_open(tz: &TimeZone, day: Date, ssm: u32) -> Option<Timestamp> {
    resolve_boundary(tz, day, ssm, FoldBias::Earlier).timestamp()
}

/// Resolves a closing wall clock, taking the later instant in a fold.
/// A boundary outside Jiff's representable range returns `None`.
#[inline]
pub(crate) fn mk_local_close(tz: &TimeZone, day: Date, ssm: u32) -> Option<Timestamp> {
    resolve_boundary(tz, day, ssm, FoldBias::Later).timestamp()
}

/// A civil day whose midnight window has no representable interval.
#[derive(Debug)]
pub(crate) struct UnrepresentableDayWindow;

/// Resolves the half-open interval between the earliest consecutive midnights.
/// An empty interval between represented midnights marks a skipped day.
/// Midnight windows at the timestamp limits are intersected with Jiff's range.
pub(crate) fn local_day_window(
    tz: &TimeZone,
    day: Date,
) -> Result<Option<(Timestamp, Timestamp)>, UnrepresentableDayWindow> {
    let start = resolve_boundary(tz, day, 0, FoldBias::Earlier);
    let end = day.tomorrow().map_or(Boundary::AfterRange, |next| {
        resolve_boundary(tz, next, 0, FoldBias::Earlier)
    });
    if let (Boundary::At(start), Boundary::At(end)) = (start, end) {
        return Ok((start < end).then_some((start, end)));
    }
    let start = start.clamped().ok_or(UnrepresentableDayWindow)?;
    let end = end.clamped().ok_or(UnrepresentableDayWindow)?;
    if start >= end {
        return Err(UnrepresentableDayWindow);
    }
    Ok(Some((start, end)))
}

/// Applies a late-open cutoff to a representable opening. A cutoff before
/// Jiff's range leaves the existing opening in force.
pub(crate) fn clip_local_open(
    tz: &TimeZone,
    day: Date,
    ssm: u32,
    open: Timestamp,
) -> Option<Timestamp> {
    match resolve_boundary(tz, day, ssm, FoldBias::Earlier) {
        Boundary::BeforeRange => Some(open),
        Boundary::At(cutoff) => Some(open.max(cutoff)),
        Boundary::AfterRange | Boundary::Unresolved => None,
    }
}

/// Applies an early-close cutoff to a representable close. A cutoff after
/// Jiff's range leaves the existing close in force.
pub(crate) fn clip_local_close(
    tz: &TimeZone,
    day: Date,
    ssm: u32,
    close: Timestamp,
) -> Option<Timestamp> {
    match resolve_boundary(tz, day, ssm, FoldBias::Later) {
        Boundary::BeforeRange | Boundary::Unresolved => None,
        Boundary::At(cutoff) => Some(close.min(cutoff)),
        Boundary::AfterRange => Some(close),
    }
}

#[derive(Clone, Copy)]
enum FoldBias {
    Earlier,
    Later,
}

#[derive(Clone, Copy)]
enum Boundary {
    BeforeRange,
    At(Timestamp),
    AfterRange,
    Unresolved,
}

impl Boundary {
    fn timestamp(self) -> Option<Timestamp> {
        match self {
            Self::At(timestamp) => Some(timestamp),
            Self::BeforeRange | Self::AfterRange | Self::Unresolved => None,
        }
    }

    fn clamped(self) -> Option<Timestamp> {
        match self {
            Self::At(timestamp) => Some(timestamp),
            Self::BeforeRange => Some(Timestamp::MIN),
            Self::AfterRange => Some(Timestamp::MAX),
            Self::Unresolved => None,
        }
    }
}

fn resolve_boundary(tz: &TimeZone, day: Date, ssm: u32, bias: FoldBias) -> Boundary {
    // A close at 86,400 represents midnight on the following civil day.
    let Ok(civil) = day
        .to_datetime(Time::midnight())
        .checked_add(SignedDuration::from_secs(i64::from(ssm)))
    else {
        // The duration is nonnegative. Its civil overflow can only be above
        // the upper limit, and even the largest offset cannot bring it back.
        return Boundary::AfterRange;
    };
    let offset = match tz.to_ambiguous_timestamp(civil).offset() {
        AmbiguousOffset::Unambiguous { offset } => offset,
        AmbiguousOffset::Fold { before, after } => match bias {
            FoldBias::Earlier => before,
            FoldBias::Later => after,
        },
        AmbiguousOffset::Gap { before, after } => return end_of_gap(tz, civil, before, after),
    };
    project(civil, offset)
}

fn project(civil: DateTime, offset: Offset) -> Boundary {
    // Jiff reports an error here only when the projected instant is outside
    // its range. The civil date of the lower limit tells us which side it is.
    match offset.to_timestamp(civil) {
        Ok(timestamp) => Boundary::At(timestamp),
        Err(_) if civil < offset.to_datetime(Timestamp::MIN) => Boundary::BeforeRange,
        Err(_) => Boundary::AfterRange,
    }
}

/// Resolves the jump itself. Applying either offset directly would shift the
/// wall clock by part of the gap rather than land on its boundary.
fn end_of_gap(tz: &TimeZone, civil: DateTime, before: Offset, after: Offset) -> Boundary {
    const NANOSECOND: SignedDuration = SignedDuration::from_nanos(1);

    // The pre-jump offset gives the inclusive upper limit of the search, and
    // the post-jump offset gives the exclusive lower limit.
    let lower = project(civil, after);
    let upper = project(civil, before);
    if matches!(upper, Boundary::BeforeRange) {
        return Boundary::BeforeRange;
    }
    let start = match lower {
        Boundary::BeforeRange => {
            // The lower timestamp limit itself can be a transition.
            // following() excludes its starting instant.
            let Ok(probe) = Timestamp::MIN.checked_add(NANOSECOND) else {
                return Boundary::Unresolved;
            };
            if tz.preceding(probe).next().is_some_and(|transition| {
                transition.timestamp() == Timestamp::MIN && transition.offset() == after
            }) {
                return Boundary::At(Timestamp::MIN);
            }
            Timestamp::MIN
        }
        Boundary::At(timestamp) => timestamp,
        Boundary::AfterRange | Boundary::Unresolved => return lower,
    };
    let transition = tz
        .following(start)
        .take_while(|transition| before.to_datetime(transition.timestamp()) <= civil)
        .find(|transition| {
            transition.offset() == after
                && transition
                    .timestamp()
                    .checked_sub(NANOSECOND)
                    .is_ok_and(|previous| tz.to_offset(previous) == before)
        });
    if let Some(transition) = transition {
        Boundary::At(transition.timestamp())
    } else if matches!(lower, Boundary::BeforeRange) {
        Boundary::BeforeRange
    } else if matches!(upper, Boundary::AfterRange) {
        Boundary::AfterRange
    } else {
        Boundary::Unresolved
    }
}
