// SPDX-License-Identifier: MIT-0

//! Containing- and next-session queries over a [`QueryContext`].

use chrono::{DateTime, Duration, Utc};

use super::schedule::{QueryContext, RuleSet, find_occurrence};
use crate::calendar::CalendarQueryError;
use crate::calendar::local_time::bounded_utc;
use crate::calendar::rule::SessionKind;

const SESSION_LOOKAHEAD_DAYS: i64 = 14;
type SessionBounds = (DateTime<Utc>, DateTime<Utc>);

fn merge_occurrences_on_day(
    context: &QueryContext<'_>,
    day: chrono::NaiveDate,
    kind: SessionKind,
    bounds: &mut SessionBounds,
) -> Result<(), CalendarQueryError> {
    // The probe never stops the scan: it folds every occurrence into `bounds`.
    let _: Option<()> = find_occurrence(
        context,
        day,
        RuleSet::Sessions(kind),
        false,
        |open, close| {
            if open <= bounds.1 && close >= bounds.0 {
                bounds.0 = bounds.0.min(open);
                bounds.1 = bounds.1.max(close);
            }
            None
        },
    )?;
    Ok(())
}

/// Unions adjacent or overlapping occurrences of one concrete session kind.
///
/// `Both` is deliberately never passed here: a regular/extended handoff is a
/// public phase boundary even when the market remains open at the same instant.
fn coalesce_same_kind(
    context: &QueryContext<'_>,
    seed: SessionBounds,
    kind: SessionKind,
) -> Result<SessionBounds, CalendarQueryError> {
    if !context.joins_adjacent_same_kind() {
        return Ok(seed);
    }
    // A genuinely continuous profile has no finite session bounds. Preserve
    // its existing rule-occurrence projection instead of inventing a horizon.
    if !context.has_daily_close_at(seed.0) {
        return Ok(seed);
    }

    let tz = context.tz();
    let mut bounds = seed;
    for _ in 0..=SESSION_LOOKAHEAD_DAYS {
        let before = bounds;
        let start_day = bounded_utc(bounds.0, tz).with_timezone(&tz).date_naive();
        let end_day = bounded_utc(bounds.1, tz).with_timezone(&tz).date_naive();

        for boundary_day in [start_day, end_day] {
            if let Some(previous_day) = boundary_day.pred_opt() {
                merge_occurrences_on_day(context, previous_day, kind, &mut bounds)?;
            }
            merge_occurrences_on_day(context, boundary_day, kind, &mut bounds)?;
        }
        if bounds == before {
            break;
        }
    }
    Ok(bounds)
}

fn containing_occurrence_of_kind(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Result<Option<SessionBounds>, CalendarQueryError> {
    let tz = context.tz();
    let day = bounded_utc(instant, tz).with_timezone(&tz).date_naive();
    let hit = |open: DateTime<Utc>, close: DateTime<Utc>| {
        (open <= instant && instant < close).then_some((open, close))
    };

    if let Some(found) = find_occurrence(context, day, RuleSet::Sessions(kind), false, hit)? {
        return Ok(Some(found));
    }
    // A wrapped occurrence belongs to the previous opening day, so an instant in
    // the first hours of the floor's own first local day is answered from a
    // pre-floor opening day. That is the spanning rule the plan requires: an
    // in-range instant whose session opened before the floor is answered whole
    // (section 6). `require_answerable` passes below the floor precisely so this
    // probe can answer; only a query *addressed* to an earlier day is refused,
    // and that verdict is taken at the entry point from the caller's own instant
    // (see `status::is_open_with`).
    let Some(yesterday) = day.pred_opt() else {
        return Ok(None);
    };
    find_occurrence(context, yesterday, RuleSet::Sessions(kind), true, hit)
}

fn containing_concrete_kind(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Result<Option<SessionBounds>, CalendarQueryError> {
    containing_occurrence_of_kind(context, instant, kind)?
        .map(|candidate| coalesce_same_kind(context, candidate, kind))
        .transpose()
}

pub(super) fn contains_in_session_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Result<bool, CalendarQueryError> {
    let found = match kind {
        SessionKind::Regular => {
            containing_occurrence_of_kind(context, instant, SessionKind::Regular)?
        }
        SessionKind::Extended => {
            containing_occurrence_of_kind(context, instant, SessionKind::Extended)?
        }
        SessionKind::Both => {
            containing_occurrence_of_kind(context, instant, SessionKind::Regular)?.or(
                containing_occurrence_of_kind(context, instant, SessionKind::Extended)?,
            )
        }
    };
    Ok(found.is_some())
}

pub(in crate::calendar) fn session_bounds_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Result<Option<SessionBounds>, CalendarQueryError> {
    context.require_floor_at(instant)?;
    match containing_session_with(context, instant, kind)? {
        Some(bounds) => Ok(Some(bounds)),
        None => next_session_after_with(context, instant, kind),
    }
}

pub(in crate::calendar) fn containing_session_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Result<Option<SessionBounds>, CalendarQueryError> {
    match kind {
        SessionKind::Regular => containing_concrete_kind(context, instant, SessionKind::Regular),
        SessionKind::Extended => containing_concrete_kind(context, instant, SessionKind::Extended),
        SessionKind::Both => {
            match containing_concrete_kind(context, instant, SessionKind::Regular)? {
                Some(bounds) => Ok(Some(bounds)),
                None => containing_concrete_kind(context, instant, SessionKind::Extended),
            }
        }
    }
}

/// Collects the first occurrence opening after `instant` on one opening day.
///
/// The coalescing fold runs **outside** the probe: the probe's own signature is
/// `Option`-valued, so a coverage error raised while coalescing could only be
/// swallowed there, and plan section 6 forbids exactly that. Returning the raw
/// bounds and coalescing here keeps `?` available.
fn next_occurrence_after_on_day(
    context: &QueryContext<'_>,
    day: chrono::NaiveDate,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Result<Option<SessionBounds>, CalendarQueryError> {
    let raw = find_occurrence(
        context,
        day,
        RuleSet::Sessions(kind),
        false,
        |open, close| (open > instant).then_some((open, close)),
    )?;
    let Some(raw) = raw else {
        return Ok(None);
    };
    let merged = coalesce_same_kind(context, raw, kind)?;
    Ok((merged.0 > instant).then_some(merged))
}

fn consider_next_on_day(
    context: &QueryContext<'_>,
    day: chrono::NaiveDate,
    instant: DateTime<Utc>,
    kind: SessionKind,
    best: &mut Option<SessionBounds>,
) -> Result<(), CalendarQueryError> {
    if let Some(merged) = next_occurrence_after_on_day(context, day, instant, kind)?
        && best.is_none_or(|current| merged.0 < current.0)
    {
        *best = Some(merged);
    }
    Ok(())
}

pub(in crate::calendar) fn next_session_after_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Result<Option<SessionBounds>, CalendarQueryError> {
    context.require_floor_at(instant)?;
    let tz = context.tz();
    let base_day = bounded_utc(instant, tz).with_timezone(&tz).date_naive();

    for offset in 0..=SESSION_LOOKAHEAD_DAYS {
        let Some(day) = base_day.checked_add_signed(Duration::days(offset)) else {
            break;
        };
        let mut best = None;
        match kind {
            SessionKind::Regular => {
                consider_next_on_day(context, day, instant, SessionKind::Regular, &mut best)?;
            }
            SessionKind::Extended => {
                consider_next_on_day(context, day, instant, SessionKind::Extended, &mut best)?;
            }
            SessionKind::Both => {
                consider_next_on_day(context, day, instant, SessionKind::Regular, &mut best)?;
                consider_next_on_day(context, day, instant, SessionKind::Extended, &mut best)?;
            }
        }
        if best.is_some() {
            return Ok(best);
        }
    }
    Ok(None)
}

/// Collects the last occurrence closing at or before `instant` on one day.
///
/// Coalescing runs outside the probe for the same reason as
/// [`next_occurrence_after_on_day`].
fn previous_occurrence_before_on_day(
    context: &QueryContext<'_>,
    day: chrono::NaiveDate,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Result<Option<SessionBounds>, CalendarQueryError> {
    let raw = find_occurrence(
        context,
        day,
        RuleSet::Sessions(kind),
        false,
        |open, close| (close <= instant).then_some((open, close)),
    )?;
    let Some(raw) = raw else {
        return Ok(None);
    };
    let merged = coalesce_same_kind(context, raw, kind)?;
    Ok((merged.1 <= instant).then_some(merged))
}

fn consider_previous_on_day(
    context: &QueryContext<'_>,
    day: chrono::NaiveDate,
    instant: DateTime<Utc>,
    kind: SessionKind,
    best: &mut Option<SessionBounds>,
) -> Result<(), CalendarQueryError> {
    if let Some(merged) = previous_occurrence_before_on_day(context, day, instant, kind)?
        && best.is_none_or(|current| merged.1 > current.1)
    {
        *best = Some(merged);
    }
    Ok(())
}

pub(super) fn previous_session_before_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Result<Option<SessionBounds>, CalendarQueryError> {
    let tz = context.tz();
    let base_day = bounded_utc(instant, tz).with_timezone(&tz).date_naive();
    let mut best = None;
    let mut scan_one_older_day = false;

    for offset in 0..=SESSION_LOOKAHEAD_DAYS {
        let Some(day) = base_day.checked_sub_signed(Duration::days(offset)) else {
            break;
        };
        match kind {
            SessionKind::Regular => {
                consider_previous_on_day(context, day, instant, SessionKind::Regular, &mut best)?;
            }
            SessionKind::Extended => {
                consider_previous_on_day(context, day, instant, SessionKind::Extended, &mut best)?;
            }
            SessionKind::Both => {
                consider_previous_on_day(context, day, instant, SessionKind::Regular, &mut best)?;
                consider_previous_on_day(context, day, instant, SessionKind::Extended, &mut best)?;
            }
        }
        if best.is_some() {
            if scan_one_older_day {
                break;
            }
            // A rule opened one day earlier can wrap past a same-day rule, so
            // inspect exactly one additional opening day before stopping.
            scan_one_older_day = true;
        }
    }
    Ok(best)
}
