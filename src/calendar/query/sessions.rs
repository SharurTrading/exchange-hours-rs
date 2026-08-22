// SPDX-License-Identifier: MIT-0

//! Containing- and next-session queries over a [`QueryContext`].

use chrono::{DateTime, Datelike, Duration, Utc};

use super::schedule::{QueryContext, resolve_rule_bounds, rules};
use crate::calendar::local_time::bounded_utc;
use crate::calendar::rule::SessionKind;

const SESSION_LOOKAHEAD_DAYS: i64 = 14;
type SessionBounds = (DateTime<Utc>, DateTime<Utc>);

fn merge_occurrences_on_day(
    context: &QueryContext<'_>,
    day: chrono::NaiveDate,
    kind: SessionKind,
    bounds: &mut SessionBounds,
) {
    let selected = context.profile_for_open_day(day);
    let weekday = day.weekday().num_days_from_monday() as usize;
    for rule in rules(selected.as_ref(), kind).filter(|rule| rule.days[weekday]) {
        let Some(candidate) = resolve_rule_bounds(context, day, rule) else {
            continue;
        };
        if candidate.0 <= bounds.1 && candidate.1 >= bounds.0 {
            bounds.0 = bounds.0.min(candidate.0);
            bounds.1 = bounds.1.max(candidate.1);
        }
    }
}

/// Unions adjacent or overlapping occurrences of one concrete session kind.
///
/// `Both` is deliberately never passed here: a regular/extended handoff is a
/// public phase boundary even when the market remains open at the same instant.
fn coalesce_same_kind(
    context: &QueryContext<'_>,
    seed: SessionBounds,
    kind: SessionKind,
) -> SessionBounds {
    if !context.joins_adjacent_same_kind() {
        return seed;
    }
    // A genuinely continuous profile has no finite session bounds. Preserve
    // its existing rule-occurrence projection instead of inventing a horizon.
    if !context.has_daily_close_at(seed.0) {
        return seed;
    }

    let tz = context.tz();
    let mut bounds = seed;
    for _ in 0..=SESSION_LOOKAHEAD_DAYS {
        let before = bounds;
        let start_day = bounded_utc(bounds.0, tz).with_timezone(&tz).date_naive();
        let end_day = bounded_utc(bounds.1, tz).with_timezone(&tz).date_naive();

        for boundary_day in [start_day, end_day] {
            if let Some(previous_day) = boundary_day.pred_opt() {
                merge_occurrences_on_day(context, previous_day, kind, &mut bounds);
            }
            merge_occurrences_on_day(context, boundary_day, kind, &mut bounds);
        }
        if bounds == before {
            break;
        }
    }
    bounds
}

fn containing_occurrence_of_kind(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Option<SessionBounds> {
    let tz = context.tz();
    let local = bounded_utc(instant, tz).with_timezone(&tz);
    let day = local.date_naive();
    let weekday = day.weekday().num_days_from_monday() as usize;

    let selected = context.profile_for_open_day(day);
    for rule in rules(selected.as_ref(), kind).filter(|rule| rule.days[weekday]) {
        let Some(candidate) = resolve_rule_bounds(context, day, rule) else {
            continue;
        };
        if candidate.0 <= instant && instant < candidate.1 {
            return Some(candidate);
        }
    }

    let yesterday = day.pred_opt()?;
    let selected = context.profile_for_open_day(yesterday);
    let previous_weekday = yesterday.weekday().num_days_from_monday() as usize;
    for rule in rules(selected.as_ref(), kind)
        .filter(|rule| rule.days[previous_weekday] && rule.wraps_to_next_day())
    {
        let Some(candidate) = resolve_rule_bounds(context, yesterday, rule) else {
            continue;
        };
        if candidate.0 <= instant && instant < candidate.1 {
            return Some(candidate);
        }
    }
    None
}

fn containing_concrete_kind(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Option<SessionBounds> {
    containing_occurrence_of_kind(context, instant, kind)
        .map(|candidate| coalesce_same_kind(context, candidate, kind))
}

pub(super) fn contains_in_session_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> bool {
    match kind {
        SessionKind::Regular => {
            containing_occurrence_of_kind(context, instant, SessionKind::Regular).is_some()
        }
        SessionKind::Extended => {
            containing_occurrence_of_kind(context, instant, SessionKind::Extended).is_some()
        }
        SessionKind::Both => {
            containing_occurrence_of_kind(context, instant, SessionKind::Regular).is_some()
                || containing_occurrence_of_kind(context, instant, SessionKind::Extended).is_some()
        }
    }
}

pub(in crate::calendar) fn session_bounds_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    containing_session_with(context, instant, kind)
        .or_else(|| next_session_after_with(context, instant, kind))
}

pub(in crate::calendar) fn containing_session_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    match kind {
        SessionKind::Regular => containing_concrete_kind(context, instant, SessionKind::Regular),
        SessionKind::Extended => containing_concrete_kind(context, instant, SessionKind::Extended),
        SessionKind::Both => containing_concrete_kind(context, instant, SessionKind::Regular)
            .or_else(|| containing_concrete_kind(context, instant, SessionKind::Extended)),
    }
}

fn consider_next_on_day(
    context: &QueryContext<'_>,
    day: chrono::NaiveDate,
    instant: DateTime<Utc>,
    kind: SessionKind,
    best: &mut Option<SessionBounds>,
) {
    let selected = context.profile_for_open_day(day);
    let weekday = day.weekday().num_days_from_monday() as usize;
    for rule in rules(selected.as_ref(), kind).filter(|rule| rule.days[weekday]) {
        let Some(candidate) = resolve_rule_bounds(context, day, rule) else {
            continue;
        };
        if candidate.0 <= instant {
            continue;
        }
        let merged = coalesce_same_kind(context, candidate, kind);
        if merged.0 <= instant {
            continue;
        }
        if best.is_none_or(|current| merged.0 < current.0) {
            *best = Some(merged);
        }
    }
}

pub(in crate::calendar) fn next_session_after_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    let tz = context.tz();
    let base_day = bounded_utc(instant, tz).with_timezone(&tz).date_naive();

    for offset in 0..=SESSION_LOOKAHEAD_DAYS {
        let Some(day) = base_day.checked_add_signed(Duration::days(offset)) else {
            break;
        };
        let mut best = None;
        match kind {
            SessionKind::Regular => {
                consider_next_on_day(context, day, instant, SessionKind::Regular, &mut best);
            }
            SessionKind::Extended => {
                consider_next_on_day(context, day, instant, SessionKind::Extended, &mut best);
            }
            SessionKind::Both => {
                consider_next_on_day(context, day, instant, SessionKind::Regular, &mut best);
                consider_next_on_day(context, day, instant, SessionKind::Extended, &mut best);
            }
        }
        if best.is_some() {
            return best;
        }
    }
    None
}

fn consider_previous_on_day(
    context: &QueryContext<'_>,
    day: chrono::NaiveDate,
    instant: DateTime<Utc>,
    kind: SessionKind,
    best: &mut Option<SessionBounds>,
) {
    let selected = context.profile_for_open_day(day);
    let weekday = day.weekday().num_days_from_monday() as usize;
    for rule in rules(selected.as_ref(), kind).filter(|rule| rule.days[weekday]) {
        let Some(candidate) = resolve_rule_bounds(context, day, rule) else {
            continue;
        };
        let merged = coalesce_same_kind(context, candidate, kind);
        if merged.1 > instant {
            continue;
        }
        if best.is_none_or(|current| merged.1 > current.1) {
            *best = Some(merged);
        }
    }
}

pub(super) fn previous_session_before_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
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
                consider_previous_on_day(context, day, instant, SessionKind::Regular, &mut best);
            }
            SessionKind::Extended => {
                consider_previous_on_day(context, day, instant, SessionKind::Extended, &mut best);
            }
            SessionKind::Both => {
                consider_previous_on_day(context, day, instant, SessionKind::Regular, &mut best);
                consider_previous_on_day(context, day, instant, SessionKind::Extended, &mut best);
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
    best
}
