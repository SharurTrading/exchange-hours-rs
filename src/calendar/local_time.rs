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

/// The UTC pieces of one local date. A backward jump across midnight can
/// separate them with time that belongs to a different date.
pub(crate) struct LocalDayWindows<'a> {
    tz: &'a TimeZone,
    first: DateTime,
    limit: Option<DateTime>,
    cursor: Timestamp,
    end: Timestamp,
    pending: Option<(Timestamp, Timestamp)>,
}

impl LocalDayWindows<'_> {
    fn next_piece(&mut self) -> Option<(Timestamp, Timestamp)> {
        while self.cursor < self.end {
            let segment_start = self.cursor;
            let segment_end = self
                .tz
                .following(segment_start)
                .next()
                .map_or(self.end, |transition| self.end.min(transition.timestamp()));
            self.cursor = segment_end;

            // Within this offset segment the civil date maps to one interval.
            let offset = self.tz.to_offset(segment_start);
            let start = project(self.first, offset).clamped()?.max(segment_start);
            let end = self
                .limit
                .map_or(Some(Timestamp::MAX), |limit| {
                    project(limit, offset).clamped()
                })?
                .min(segment_end);
            if start < end {
                return Some((start, end));
            }
        }
        None
    }
}

impl Iterator for LocalDayWindows<'_> {
    type Item = (Timestamp, Timestamp);

    fn next(&mut self) -> Option<Self::Item> {
        let (start, mut end) = self.pending.take().or_else(|| self.next_piece())?;
        // An ordinary DST change inside the date leaves its UTC interval
        // contiguous. Preserve that single interval for the coverage walk.
        while let Some(next) = self.next_piece() {
            if end != next.0 {
                self.pending = Some(next);
                break;
            }
            end = next.1;
        }
        Some((start, end))
    }
}

/// Resolves the half-open UTC pieces of a civil day. The outer interval uses
/// the first midnight and the last following midnight, so it includes both
/// visits to a date around a rollback. The iterator intersects that interval
/// with the civil day separately on each side of any offset change. A skipped
/// day has no pieces. The bounds are clipped to Jiff's timestamp range.
pub(crate) fn local_day_windows(
    tz: &TimeZone,
    day: Date,
) -> Result<LocalDayWindows<'_>, UnrepresentableDayWindow> {
    let tomorrow = day.tomorrow().ok();
    let start = resolve_boundary(tz, day, 0, FoldBias::Earlier);
    let end = tomorrow.map_or(Boundary::AfterRange, |next| {
        resolve_boundary(tz, next, 0, FoldBias::Later)
    });
    let (cursor, end) = if let (Boundary::At(start), Boundary::At(end)) = (start, end) {
        (start, end)
    } else {
        let start = start.clamped().ok_or(UnrepresentableDayWindow)?;
        let end = end.clamped().ok_or(UnrepresentableDayWindow)?;
        if start >= end {
            return Err(UnrepresentableDayWindow);
        }
        (start, end)
    };
    Ok(LocalDayWindows {
        tz,
        first: day.to_datetime(Time::midnight()),
        limit: tomorrow.map(|next| next.to_datetime(Time::midnight())),
        cursor,
        end,
        pending: None,
    })
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
