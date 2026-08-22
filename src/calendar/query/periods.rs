// SPDX-License-Identifier: MIT-0

//! Daily, weekly, and monthly close discovery over a [`QueryContext`].

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};

use super::schedule::{QueryContext, resolve_rule_bounds, rules};
use super::sessions::containing_session_with;
use crate::calendar::local_time::bounded_utc;
use crate::calendar::rule::SessionKind;

const CLOSE_LOOKAHEAD_DAYS: i64 = 21;

fn update_latest(
    context: &QueryContext<'_>,
    day: NaiveDate,
    kind: SessionKind,
    latest: &mut Option<DateTime<Utc>>,
    candidate_open: DateTime<Utc>,
    candidate: DateTime<Utc>,
    ceiling: Option<DateTime<Utc>>,
) {
    if candidate.with_timezone(&context.tz()).date_naive() == day
        && context.trade_date_for_bounds(candidate_open, candidate) == day
        && containing_session_with(context, candidate, kind).is_none()
        && ceiling.is_none_or(|limit| candidate <= limit)
        && latest.is_none_or(|current| candidate > current)
    {
        *latest = Some(candidate);
    }
}

fn latest_close_landing_on_day(
    context: &QueryContext<'_>,
    day: NaiveDate,
    kind: SessionKind,
    ceiling: Option<DateTime<Utc>>,
) -> Option<DateTime<Utc>> {
    let weekday = day.weekday().num_days_from_monday() as usize;
    let today = context.profile_for_open_day(day);
    let mut latest = None;

    for rule in rules(today.as_ref(), kind).filter(|rule| rule.days[weekday]) {
        if let Some((open, close)) = resolve_rule_bounds(context, day, rule) {
            update_latest(context, day, kind, &mut latest, open, close, ceiling);
        }
    }

    if let Some(yesterday) = day.pred_opt() {
        let previous_weekday = yesterday.weekday().num_days_from_monday() as usize;
        let previous = context.profile_for_open_day(yesterday);
        for rule in rules(previous.as_ref(), kind).filter(|rule| rule.days[previous_weekday]) {
            if let Some((open, close)) = resolve_rule_bounds(context, yesterday, rule) {
                update_latest(context, day, kind, &mut latest, open, close, ceiling);
            }
        }
    }
    latest
}

pub(in crate::calendar) fn daily_close_for_local_day(
    context: &QueryContext<'_>,
    day: NaiveDate,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    latest_close_landing_on_day(context, day, kind, None)
}

pub(in crate::calendar) fn next_daily_close_after_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    let tz = context.tz();
    let mut day = bounded_utc(instant, tz).with_timezone(&tz).date_naive();
    for _ in 0..CLOSE_LOOKAHEAD_DAYS {
        if let Some(close) = daily_close_for_local_day(context, day, kind)
            && close > instant
        {
            return Some(close);
        }
        day = day.succ_opt()?;
    }
    None
}

pub(in crate::calendar) fn next_weekly_close_after_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    let tz = context.tz();
    let mut close = next_daily_close_after_with(context, instant, kind)?;
    loop {
        let week = close.with_timezone(&tz).iso_week();
        let Some(probe) = close.checked_add_signed(Duration::nanoseconds(1)) else {
            return Some(close);
        };
        let Some(next) = next_daily_close_after_with(context, probe, kind) else {
            return Some(close);
        };
        if next.with_timezone(&tz).iso_week() != week {
            return Some(close);
        }
        close = next;
    }
}

pub(in crate::calendar) fn next_monthly_close_after_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    let tz = context.tz();
    let mut close = next_daily_close_after_with(context, instant, kind)?;
    loop {
        let local = close.with_timezone(&tz);
        let month = (local.year(), local.month());
        let Some(probe) = close.checked_add_signed(Duration::nanoseconds(1)) else {
            return Some(close);
        };
        let Some(next) = next_daily_close_after_with(context, probe, kind) else {
            return Some(close);
        };
        let next_local = next.with_timezone(&tz);
        if (next_local.year(), next_local.month()) != month {
            return Some(close);
        }
        close = next;
    }
}
