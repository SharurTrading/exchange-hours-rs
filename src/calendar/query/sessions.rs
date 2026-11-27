// SPDX-License-Identifier: MIT-0

//! Containing- and next-session queries over a [`QueryContext`].

use jiff::Timestamp;

use super::gate::SourceGate;
use super::schedule::{QueryContext, RuleSet, find_occurrence};
use crate::calendar::local_time::mk_local_open;
use crate::calendar::rule::SessionKind;

const SESSION_LOOKAHEAD_DAYS: i64 = 14;
type SessionBounds = (Timestamp, Timestamp);

fn merge_occurrences_on_day<G: SourceGate>(
    context: &QueryContext<'_, G>,
    day: jiff::civil::Date,
    kind: SessionKind,
    bounds: &mut SessionBounds,
) -> Result<(), G::Error> {
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
fn coalesce_same_kind<G: SourceGate>(
    context: &QueryContext<'_, G>,
    seed: SessionBounds,
    kind: SessionKind,
) -> Result<SessionBounds, G::Error> {
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
        let start_day = tz.to_datetime(bounds.0).date();
        let end_day = tz.to_datetime(bounds.1).date();

        for boundary_day in [start_day, end_day] {
            if let Ok(previous_day) = boundary_day.yesterday() {
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

fn containing_concrete_kind<G: SourceGate>(
    context: &QueryContext<'_, G>,
    instant: Timestamp,
    kind: SessionKind,
) -> Result<Option<SessionBounds>, G::Error> {
    let day = context.tz().to_datetime(instant).date();
    context
        .containing_occurrence(instant, day, RuleSet::Sessions(kind))?
        .map(|candidate| coalesce_same_kind(context, candidate, kind))
        .transpose()
}

/// [`is_open`'s](crate::ExchangeCalendar::is_open) containment probe over a
/// caller-resolved venue-local day.
///
/// `is_open` resolves the day once for both the containment probe and its own
/// floor check, so the crate's hottest query performs one timezone walk where
/// two used to run (issue #125). The answers are identical: the day is a pure
/// function of the instant and the context's zone.
pub(super) fn contains_in_session_on<G: SourceGate>(
    context: &QueryContext<'_, G>,
    instant: Timestamp,
    day: jiff::civil::Date,
    kind: SessionKind,
) -> Result<bool, G::Error> {
    let found = match kind {
        SessionKind::Regular => {
            context.containing_occurrence(instant, day, RuleSet::Sessions(SessionKind::Regular))?
        }
        SessionKind::Extended => {
            context.containing_occurrence(instant, day, RuleSet::Sessions(SessionKind::Extended))?
        }
        SessionKind::Both => {
            // Lazy, like `containing_session_with`'s `Both` arm: a Regular hit
            // settles the question, so the Extended probe — whose own wrapped
            // lookback can refuse where the Regular answer already resolved —
            // runs only when it is the query's answer (#257).
            let regular = context.containing_occurrence(
                instant,
                day,
                RuleSet::Sessions(SessionKind::Regular),
            )?;
            if regular.is_some() {
                regular
            } else {
                context.containing_occurrence(
                    instant,
                    day,
                    RuleSet::Sessions(SessionKind::Extended),
                )?
            }
        }
    };
    Ok(found.is_some())
}

pub(in crate::calendar) fn session_bounds_with<G: SourceGate>(
    context: &QueryContext<'_, G>,
    instant: Timestamp,
    kind: SessionKind,
) -> Result<Option<SessionBounds>, G::Error> {
    context.require_floor_at(instant)?;
    match containing_session_with(context, instant, kind)? {
        Some(bounds) => Ok(Some(bounds)),
        None => next_session_after_with(context, instant, kind),
    }
}

pub(in crate::calendar) fn containing_session_with<G: SourceGate>(
    context: &QueryContext<'_, G>,
    instant: Timestamp,
    kind: SessionKind,
) -> Result<Option<SessionBounds>, G::Error> {
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

/// Selects the earliest occurrence opening after `instant` on one opening day.
/// Normal rules and replacement offsets are not visited in chronological order.
fn next_occurrence_after_on_day<G: SourceGate>(
    context: &QueryContext<'_, G>,
    day: jiff::civil::Date,
    instant: Timestamp,
    kind: SessionKind,
    wrapped_only: bool,
) -> Result<Option<SessionBounds>, G::Error> {
    let mut best = None;
    let error = find_occurrence(
        context,
        day,
        RuleSet::Sessions(kind),
        wrapped_only,
        |open, close| {
            if open > instant {
                match coalesce_same_kind(context, (open, close), kind) {
                    Ok(merged) => {
                        if merged.0 > instant
                            && best.is_none_or(|current: SessionBounds| merged.0 < current.0)
                        {
                            best = Some(merged);
                        }
                    }
                    Err(error) => return Some(error),
                }
            }
            None
        },
    )?;
    match error {
        Some(error) => Err(error),
        None => Ok(best),
    }
}

fn consider_next_on_day<G: SourceGate>(
    context: &QueryContext<'_, G>,
    day: jiff::civil::Date,
    instant: Timestamp,
    kind: SessionKind,
    wrapped_only: bool,
    best: &mut Option<SessionBounds>,
) -> Result<(), G::Error> {
    let candidates = [SessionKind::Regular, SessionKind::Extended];
    for concrete in candidates {
        if (kind == SessionKind::Both || kind == concrete)
            && (!wrapped_only
                || context.wrapped_occurrence_possible(day, RuleSet::Sessions(concrete)))
            && let Some(candidate) =
                next_occurrence_after_on_day(context, day, instant, concrete, wrapped_only)?
            && best.is_none_or(|current| candidate.0 < current.0)
        {
            *best = Some(candidate);
        }
    }
    Ok(())
}

pub(in crate::calendar) fn next_session_after_with<G: SourceGate>(
    context: &QueryContext<'_, G>,
    instant: Timestamp,
    kind: SessionKind,
) -> Result<Option<SessionBounds>, G::Error> {
    context.require_floor_at(instant)?;
    let tz = context.tz();
    let mut day = tz.to_datetime(instant).date();
    let mut best = None;

    for offset in 0..=SESSION_LOOKAHEAD_DAYS {
        consider_next_on_day(context, day, instant, kind, false, &mut best)?;
        if offset == 0
            && context.has_late_open_on(day)
            && let Ok(previous) = day.yesterday()
        {
            // A wrapping rule can be clipped into the queried civil day while
            // remaining owned by yesterday's profile. Compare its actual
            // opening with candidates from today before choosing either.
            let mut prior = None;
            if let Err(error) =
                consider_next_on_day(context, previous, instant, kind, true, &mut prior)
            {
                let mut possible = None;
                let retry = consider_next_on_day(
                    &context.with_sourced_week_anchors(),
                    previous,
                    instant,
                    kind,
                    true,
                    &mut possible,
                );
                // The retry can only eliminate a candidate: the sourced
                // normal week fixes where its cutoff falls, independent of
                // an unrelated holiday refusal inside the raw anchor walk.
                if retry.is_err() || possible.is_some() {
                    return Err(error);
                }
            } else if let Some(candidate) = prior
                && best.is_none_or(|current| candidate.0 < current.0)
            {
                best = prior;
            }
        }
        let Ok(tomorrow) = day.tomorrow() else {
            break;
        };
        if let Some((open, _)) = best
            && mk_local_open(tz, tomorrow, 0).is_some_and(|midnight| open <= midnight)
        {
            break;
        }
        // A candidate past the next day's first midnight may be preceded
        // by that day's own occurrence. Keep it while inspecting that day.
        day = tomorrow;
    }
    Ok(best)
}

/// Collects the last occurrence closing at or before `instant` on one day.
///
/// A coalescing error stops the scan and propagates through the result.
fn previous_occurrence_before_on_day<G: SourceGate>(
    context: &QueryContext<'_, G>,
    day: jiff::civil::Date,
    instant: Timestamp,
    kind: SessionKind,
) -> Result<Option<SessionBounds>, G::Error> {
    let mut best = None;
    let error = find_occurrence(
        context,
        day,
        RuleSet::Sessions(kind),
        false,
        |open, close| {
            if close <= instant {
                match coalesce_same_kind(context, (open, close), kind) {
                    Ok(merged) => {
                        if merged.1 <= instant
                            && best.is_none_or(|current: SessionBounds| merged.1 > current.1)
                        {
                            best = Some(merged);
                        }
                    }
                    Err(error) => return Some(error),
                }
            }
            None
        },
    )?;
    match error {
        Some(error) => Err(error),
        None => Ok(best),
    }
}

fn consider_previous_on_day<G: SourceGate>(
    context: &QueryContext<'_, G>,
    day: jiff::civil::Date,
    instant: Timestamp,
    kind: SessionKind,
    best: &mut Option<SessionBounds>,
) -> Result<(), G::Error> {
    if let Some(merged) = previous_occurrence_before_on_day(context, day, instant, kind)?
        && best.is_none_or(|current| merged.1 > current.1)
    {
        *best = Some(merged);
    }
    Ok(())
}

pub(super) fn previous_session_before_with<G: SourceGate>(
    context: &QueryContext<'_, G>,
    instant: Timestamp,
    kind: SessionKind,
) -> Result<Option<SessionBounds>, G::Error> {
    let tz = context.tz();
    let base_day = tz.to_datetime(instant).date();
    let mut best = None;
    let mut scan_one_older_day = false;

    for offset in 0..=SESSION_LOOKAHEAD_DAYS {
        let Some(day) = base_day
            .checked_sub(jiff::SignedDuration::from_hours(24 * (offset)))
            .ok()
        else {
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
