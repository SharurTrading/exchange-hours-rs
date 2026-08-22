// SPDX-License-Identifier: MIT-0

//! Containing- and next-session queries over a [`QueryContext`].

use chrono::{DateTime, Datelike, Duration, Utc};

use super::schedule::{QueryContext, day_is_holiday, resolve_rule_bounds, rules};
use crate::calendar::local_time::bounded_utc;
use crate::calendar::rule::SessionKind;

const SESSION_LOOKAHEAD_DAYS: i64 = 14;

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
    let tz = context.tz();
    let local = bounded_utc(instant, tz).with_timezone(&tz);
    let day = local.date_naive();
    let weekday = day.weekday().num_days_from_monday() as usize;

    if !day_is_holiday(day) {
        let selected = context.profile_for_open_day(day);
        for rule in rules(selected.as_ref(), kind).filter(|rule| rule.days[weekday]) {
            let Some(candidate) = resolve_rule_bounds(context, day, rule) else {
                continue;
            };
            if candidate.0 <= instant && instant < candidate.1 {
                return Some(candidate);
            }
        }
    }

    let yesterday = day.pred_opt()?;
    if day_is_holiday(yesterday) || day_is_holiday(day) {
        return None;
    }
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
        if day_is_holiday(day) {
            continue;
        }
        let selected = context.profile_for_open_day(day);
        let weekday = day.weekday().num_days_from_monday() as usize;
        let mut best = None;

        for rule in rules(selected.as_ref(), kind).filter(|rule| rule.days[weekday]) {
            let Some(candidate) = resolve_rule_bounds(context, day, rule) else {
                continue;
            };
            let open = candidate.0;
            if open <= instant {
                continue;
            }
            if best.is_none_or(|current: (DateTime<Utc>, DateTime<Utc>)| candidate.0 < current.0) {
                best = Some(candidate);
            }
        }
        if best.is_some() {
            return best;
        }
    }
    None
}
