// SPDX-License-Identifier: MIT-0

//! Candle boundaries over the shared fixed/date-aware query context.

use chrono::{DateTime, Datelike, Duration, Utc};

use super::periods::{
    daily_close_for_trade_date, next_daily_close_after_with, next_monthly_close_after_with,
    next_weekly_close_after_with, trade_date_for_daily_close,
};
use super::schedule::QueryContext;
use super::sessions::{next_session_after_with, session_bounds_with};
use crate::calendar::resolution::CalendarResolution;
use crate::calendar::rule::SessionKind;

const PERIOD_LOOKBACK_DAYS: i64 = 31;
const PREVIOUS_CLOSE_LOOKBACK_DAYS: i64 = 21;
const FIRST_OPEN_LOOKBACK_DAYS: i64 = 13;

pub(in crate::calendar) fn candle_end_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    resolution: CalendarResolution,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    if is_zero_interval(resolution) {
        return None;
    }
    if matches!(
        resolution,
        CalendarResolution::Daily | CalendarResolution::Monthly
    ) && !context.has_daily_close_at(instant)
    {
        return None;
    }
    if resolution == CalendarResolution::Weekly && !context.has_weekly_close_at(instant) {
        return None;
    }
    match resolution {
        CalendarResolution::Seconds(seconds) => {
            instant.checked_add_signed(Duration::seconds(i64::from(seconds)))
        }
        CalendarResolution::Minutes(minutes) => fixed_grid_end(
            context,
            instant,
            Duration::minutes(i64::from(minutes)),
            kind,
        ),
        CalendarResolution::Hours(hours) => {
            fixed_grid_end(context, instant, Duration::hours(i64::from(hours)), kind)
        }
        CalendarResolution::Daily => next_daily_close_after_with(context, instant, kind),
        CalendarResolution::Weekly => next_weekly_close_after_with(context, instant, kind),
        CalendarResolution::Monthly => next_monthly_close_after_with(context, instant, kind),
    }
}

pub(in crate::calendar) fn candle_start_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    resolution: CalendarResolution,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    if is_zero_interval(resolution) {
        return None;
    }
    match resolution {
        CalendarResolution::Seconds(seconds) => instant
            .checked_add_signed(Duration::seconds(i64::from(seconds)))
            .map(|_| instant),
        CalendarResolution::Minutes(_) | CalendarResolution::Hours(_) => {
            let (open, _close) = session_bounds_with(context, instant, kind)?;
            Some(instant.max(open))
        }
        CalendarResolution::Daily | CalendarResolution::Weekly | CalendarResolution::Monthly => {
            period_start(context, instant, resolution, kind)
        }
    }
}

fn is_zero_interval(resolution: CalendarResolution) -> bool {
    matches!(
        resolution,
        CalendarResolution::Seconds(0)
            | CalendarResolution::Minutes(0)
            | CalendarResolution::Hours(0)
    )
}

fn fixed_grid_end(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    step: Duration,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    let (open, close) = session_bounds_with(context, instant, kind)?;
    Some(
        instant
            .max(open)
            .checked_add_signed(step)
            .map_or(close, |end| end.min(close)),
    )
}

fn period_start(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    resolution: CalendarResolution,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    let end = candle_end_with(context, instant, resolution, kind)?;
    let end_day = trade_date_for_daily_close(context, end, kind)?;
    let end_week = end_day.iso_week();
    let end_month = (end_day.year(), end_day.month());
    let mut first_close = end;
    let mut first_trade_date = end_day;

    if resolution != CalendarResolution::Daily
        && let Some(mut day) = end_day.pred_opt()
    {
        for _ in 0..PERIOD_LOOKBACK_DAYS {
            let same_period = match resolution {
                CalendarResolution::Weekly => day.iso_week() == end_week,
                CalendarResolution::Monthly => (day.year(), day.month()) == end_month,
                CalendarResolution::Daily
                | CalendarResolution::Seconds(_)
                | CalendarResolution::Minutes(_)
                | CalendarResolution::Hours(_) => false,
            };
            if !same_period {
                break;
            }
            if let Some(close) = daily_close_for_trade_date(context, day, kind) {
                first_close = close;
                first_trade_date = day;
            }
            let Some(previous_day) = day.pred_opt() else {
                break;
            };
            day = previous_day;
        }
    }

    let Some(mut day) = first_trade_date.pred_opt() else {
        return first_representable_period_open(context, first_close, kind);
    };
    let mut previous_close = None;
    for _ in 0..PREVIOUS_CLOSE_LOOKBACK_DAYS {
        if let Some(close) = daily_close_for_trade_date(context, day, kind)
            && close < first_close
        {
            previous_close = Some(close);
            break;
        }
        let Some(previous_day) = day.pred_opt() else {
            return first_representable_period_open(context, first_close, kind);
        };
        day = previous_day;
    }
    let Some(previous_close) = previous_close else {
        return first_open_without_previous_close(context, first_close, kind);
    };
    let probe = previous_close.checked_sub_signed(Duration::nanoseconds(1))?;
    let (open, _close) = next_session_after_with(context, probe, kind)?;
    (open < first_close).then_some(open)
}

fn first_open_without_previous_close(
    context: &QueryContext<'_>,
    first_close: DateTime<Utc>,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    let Some(probe) = first_close.checked_sub_signed(Duration::days(FIRST_OPEN_LOOKBACK_DAYS))
    else {
        return first_representable_period_open(context, first_close, kind);
    };
    let (open, _close) = next_session_after_with(context, probe, kind)?;
    (open < first_close).then_some(open)
}

fn first_representable_period_open(
    context: &QueryContext<'_>,
    first_close: DateTime<Utc>,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    let (open, _close) = session_bounds_with(context, DateTime::<Utc>::MIN_UTC, kind)?;
    (open < first_close).then_some(open)
}
