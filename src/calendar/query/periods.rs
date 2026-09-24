// SPDX-License-Identifier: MIT-0

//! Daily, weekly, and monthly close discovery over a [`QueryContext`].

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};

use super::replacement::{self, ExceptionDailyClose};
use super::schedule::{QueryContext, RuleSet, resolve_rule_bounds, rules};
use super::sessions::containing_session_with;
use crate::calendar::CalendarQueryError;
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
) -> Result<(), CalendarQueryError> {
    if context.trade_date_for_bounds(candidate_open, candidate) == day
        && containing_session_with(context, candidate, kind)?.is_none()
        && ceiling.is_none_or(|limit| candidate <= limit)
        && latest.is_none_or(|current| candidate > current)
    {
        *latest = Some(candidate);
    }
    Ok(())
}

fn latest_close_for_trade_date(
    context: &QueryContext<'_>,
    day: NaiveDate,
    kind: SessionKind,
    ceiling: Option<DateTime<Utc>>,
) -> Result<Option<DateTime<Utc>>, CalendarQueryError> {
    let weekday = day.weekday().num_days_from_monday() as usize;
    let today = context.profile_for_open_day(day);
    let mut latest = None;

    for rule in rules(today.as_ref(), RuleSet::Sessions(kind)).filter(|rule| rule.days[weekday]) {
        if let Some((open, close)) =
            resolve_rule_bounds(context, day, RuleSet::Sessions(kind), rule)?
        {
            update_latest(context, day, kind, &mut latest, open, close, ceiling)?;
        }
    }

    if let Some(yesterday) = day.pred_opt() {
        let previous_weekday = yesterday.weekday().num_days_from_monday() as usize;
        let previous = context.profile_for_open_day(yesterday);
        for rule in rules(previous.as_ref(), RuleSet::Sessions(kind))
            .filter(|rule| rule.days[previous_weekday])
        {
            if let Some((open, close)) =
                resolve_rule_bounds(context, yesterday, RuleSet::Sessions(kind), rule)?
            {
                update_latest(context, day, kind, &mut latest, open, close, ceiling)?;
            }
        }
    }
    if let Some(tomorrow) = day.succ_opt() {
        let next_weekday = tomorrow.weekday().num_days_from_monday() as usize;
        let next = context.profile_for_open_day(tomorrow);
        for rule in
            rules(next.as_ref(), RuleSet::Sessions(kind)).filter(|rule| rule.days[next_weekday])
        {
            if let Some((open, close)) =
                resolve_rule_bounds(context, tomorrow, RuleSet::Sessions(kind), rule)?
            {
                update_latest(context, day, kind, &mut latest, open, close, ceiling)?;
            }
        }
    }
    Ok(latest)
}

/// Returns the final close assigned to venue-local trade date `day`.
///
/// A caller-supplied exception record answers first and completely: a closed
/// trade date has no close, and a replaced one takes its close from its own
/// blocks rather than from the normal-week neighbour scan below, whose
/// one-day-either-side window cannot reach a block that opens several local
/// days before its trade date.
pub(in crate::calendar) fn daily_close_for_trade_date(
    context: &QueryContext<'_>,
    day: NaiveDate,
    kind: SessionKind,
) -> Result<Option<DateTime<Utc>>, CalendarQueryError> {
    match replacement::daily_close(context, day, kind) {
        ExceptionDailyClose::NoSession => Ok(None),
        ExceptionDailyClose::Close(close) => Ok(Some(close)),
        ExceptionDailyClose::NotGoverned => latest_close_for_trade_date(context, day, kind, None),
    }
}

pub(in crate::calendar) fn next_daily_close_after_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Result<Option<DateTime<Utc>>, CalendarQueryError> {
    Ok(
        next_daily_close_and_trade_date_after_with(context, instant, kind)?
            .map(|(_day, close)| close),
    )
}

/// Walks forward from `instant` for the next daily close, at most
/// `CLOSE_LOOKAHEAD_DAYS` days.
///
/// Running out of window is [`CalendarQueryError::SearchExhausted`], reported
/// against the day the walk stopped on: it is a different answer from a day the
/// identity refuses, which the per-day gate below raises instead (LAW-COVERAGE).
/// A **detached fixed snapshot** has no identity to attribute that error to, so
/// it keeps its previous exhaustive `None` rather than naming a source it does
/// not have.
fn next_daily_close_and_trade_date_after_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Result<Option<(NaiveDate, DateTime<Utc>)>, CalendarQueryError> {
    let tz = context.tz();
    let local_day = bounded_utc(instant, tz).with_timezone(&tz).date_naive();
    let mut day = local_day.pred_opt().unwrap_or(local_day);
    for _ in 0..CLOSE_LOOKAHEAD_DAYS {
        if let Some(close) = daily_close_for_trade_date(context, day, kind)?
            && close > instant
        {
            return Ok(Some((day, close)));
        }
        let Some(next) = day.succ_opt() else {
            return Ok(None);
        };
        day = next;
    }
    match context.identity() {
        Some(source) => Err(CalendarQueryError::SearchExhausted {
            source,
            date: day,
            bound: day,
        }),
        None => Ok(None),
    }
}

pub(in crate::calendar) fn trade_date_for_daily_close(
    context: &QueryContext<'_>,
    close: DateTime<Utc>,
    kind: SessionKind,
) -> Result<Option<NaiveDate>, CalendarQueryError> {
    let Some(probe) = close.checked_sub_signed(Duration::nanoseconds(1)) else {
        return Ok(None);
    };
    let Some((open, resolved_close)) = containing_session_with(context, probe, kind)? else {
        return Ok(None);
    };
    Ok((resolved_close == close).then(|| context.trade_date_for_bounds(open, close)))
}

pub(in crate::calendar) fn next_weekly_close_after_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Result<Option<DateTime<Utc>>, CalendarQueryError> {
    let Some((mut trade_date, mut close)) =
        next_daily_close_and_trade_date_after_with(context, instant, kind)?
    else {
        return Ok(None);
    };
    loop {
        let week = trade_date.iso_week();
        let Some(probe) = close.checked_add_signed(Duration::nanoseconds(1)) else {
            return Ok(Some(close));
        };
        let Some((next_trade_date, next)) =
            next_daily_close_and_trade_date_after_with(context, probe, kind)?
        else {
            return Ok(Some(close));
        };
        if next_trade_date.iso_week() != week {
            return Ok(Some(close));
        }
        trade_date = next_trade_date;
        close = next;
    }
}

pub(in crate::calendar) fn next_monthly_close_after_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Result<Option<DateTime<Utc>>, CalendarQueryError> {
    let Some((mut trade_date, mut close)) =
        next_daily_close_and_trade_date_after_with(context, instant, kind)?
    else {
        return Ok(None);
    };
    loop {
        let month = (trade_date.year(), trade_date.month());
        let Some(probe) = close.checked_add_signed(Duration::nanoseconds(1)) else {
            return Ok(Some(close));
        };
        let Some((next_trade_date, next)) =
            next_daily_close_and_trade_date_after_with(context, probe, kind)?
        else {
            return Ok(Some(close));
        };
        if (next_trade_date.year(), next_trade_date.month()) != month {
            return Ok(Some(close));
        }
        trade_date = next_trade_date;
        close = next;
    }
}
