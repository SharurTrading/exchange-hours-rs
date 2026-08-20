// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! Bar boundaries that respect the trading calendar.
//!
//! Intraday bars step by their interval from `max(t, session_open)` and are
//! **clamped** to the enclosing session close, so a bar never spans a closed
//! period. One exception is deliberate: when a bar would end exactly at a
//! venue's daily close and a maintenance gap follows, the end snaps forward to
//! the next session open — a bar must not terminate at the *start* of the break.
//! [`CalendarResolution::Seconds`] is the one unclamped variant: it is a pure
//! `t + s` offset, by contract.
//!
//! Calendar bars (`Daily` / `Weekly` / `Monthly`) take their boundaries from
//! session closes ([`super::period`]), never from a fixed grid. The consequence
//! callers most often get wrong: a calendar bar's **start can fall on the
//! preceding civil day, month, or year** — the CME trading day closing Monday
//! 16:00 CT starts Sunday 17:00 CT. An end-exclusive provider close marker must
//! therefore be probed at `close - 1ns`, not at `close`.

use chrono::{DateTime, Datelike, Duration, Utc};

use super::period::{
    daily_close_for_local_day, next_daily_close_after_with, next_monthly_close_after_with,
    next_weekly_close_after_with,
};
use super::session::{
    next_session_after, next_session_after_with, session_bounds, session_bounds_with,
};
use super::{CalendarResolution, MarketHours, SessionKind};

/// Returns the session-aware bar close strictly **after** `t` for `res` and `kind`.
///
/// - `Seconds`: a pure `t + interval` step (no session clamping).
/// - `Minutes`/`Hours`: steps from `max(t, session_open)` by the interval,
///   clamped to the enclosing session close. If that close coincides with the
///   venue's daily close and a maintenance gap follows, the end snaps forward to
///   the next session open so bars do not terminate at the maintenance start.
/// - `Daily`: the next day-close after `t` (latest close on a local calendar day).
/// - `Weekly`: the last day-close in `t`'s ISO week.
/// - `Monthly`: the last day-close in `t`'s exchange-local calendar month.
///
/// # Panics
///
/// Panics only on a malformed calendar; see [`session_bounds_with`].
#[must_use]
pub fn candle_end_with(
    hours: &MarketHours,
    t: DateTime<Utc>,
    res: CalendarResolution,
    kind: SessionKind,
) -> DateTime<Utc> {
    match res {
        CalendarResolution::Seconds(s) => t + Duration::seconds(i64::from(s)),
        CalendarResolution::Minutes(m) => {
            fixed_grid_end(hours, t, Duration::minutes(i64::from(m)), kind)
        }
        CalendarResolution::Hours(h) => {
            fixed_grid_end(hours, t, Duration::hours(i64::from(h)), kind)
        }
        CalendarResolution::Daily => next_daily_close_after_with(hours, t, kind),
        CalendarResolution::Weekly => next_weekly_close_after_with(hours, t, kind),
        CalendarResolution::Monthly => next_monthly_close_after_with(hours, t, kind),
    }
}

/// Shared body of the `Minutes`/`Hours` arms: step by `step` from
/// `max(t, session_open)`, clamped to the enclosing session close.
///
/// If the computed end lands exactly on the session close and that close is the
/// exchange's daily close (e.g., CME 16:00 CT), skip short “maintenance” gaps by
/// snapping to the next session open. This prevents intraday bars from closing
/// at the maintenance start and instead closes them after the break.
fn fixed_grid_end(
    hours: &MarketHours,
    t: DateTime<Utc>,
    step: Duration,
    kind: SessionKind,
) -> DateTime<Utc> {
    let (open, close) = session_bounds_with(kind, hours, t);
    let anchor = if t < open { open } else { t };
    let mut end = (anchor + step).min(close);

    if end == close {
        let close_day = close.with_timezone(&hours.tz).date_naive();
        if let Some(dc) = daily_close_for_local_day(hours, close_day, kind)
            && dc == close
            && hours.is_maintenance(close)
        {
            let (next_open, _next_close) = next_session_after_with(kind, hours, close);
            end = next_open;
        }
    }

    end
}

/// Returns [`candle_end_with`] over [`SessionKind::Both`] (regular + extended).
///
/// # Panics
///
/// Panics only on a malformed calendar; see [`candle_end_with`].
#[inline]
#[must_use]
pub fn candle_end(
    hours: &MarketHours,
    t: DateTime<Utc>,
    interval: CalendarResolution,
) -> DateTime<Utc> {
    candle_end_with(hours, t, interval, SessionKind::Both)
}

/// Returns the opening instant paired with [`candle_end`] for `t` and `interval`.
///
/// - `Seconds`: `t`, matching the interval stepped directly from `t`.
/// - `Minutes`/`Hours`: `max(t, session_open)`, matching the anchor used by
///   [`candle_end`].
/// - `Daily`: the first session open after the preceding trading-day close.
/// - `Weekly`: the first session open whose daily close belongs to the target
///   ISO week.
/// - `Monthly`: the first session open whose daily close belongs to the target
///   exchange-local month.
///
/// Calendar starts can fall on the preceding civil day, month, or year. For
/// example, the CME trading day that closes Monday at 16:00 CT starts Sunday at
/// 17:00 CT. Callers converting an end-exclusive provider close marker should
/// query with an instant immediately before that marker.
///
/// # Panics
///
/// Panics only when `hours` is malformed such that [`candle_end`] finds a
/// calendar close but no preceding daily close or subsequent session open can
/// be found within the calendar's bounded search horizons.
#[must_use]
pub fn candle_start(
    hours: &MarketHours,
    t: DateTime<Utc>,
    interval: CalendarResolution,
) -> DateTime<Utc> {
    match interval {
        CalendarResolution::Seconds(_) => t,
        CalendarResolution::Minutes(_) | CalendarResolution::Hours(_) => {
            let (session_open, _) = session_bounds(hours, t);
            t.max(session_open)
        }
        CalendarResolution::Daily | CalendarResolution::Weekly | CalendarResolution::Monthly => {
            calendar_period_start(hours, t, interval)
        }
    }
}

fn calendar_period_start(
    hours: &MarketHours,
    t: DateTime<Utc>,
    interval: CalendarResolution,
) -> DateTime<Utc> {
    let period_end = candle_end(hours, t, interval);
    let end_local = period_end.with_timezone(&hours.tz);
    let end_day = end_local.date_naive();
    let end_iso_week = end_local.iso_week();
    let end_month = (end_local.year(), end_local.month());

    let mut first_close = period_end;
    if interval != CalendarResolution::Daily {
        let mut day = end_day - Duration::days(1);
        for _ in 0..31 {
            let same_period = match interval {
                CalendarResolution::Weekly => day.iso_week() == end_iso_week,
                CalendarResolution::Monthly => (day.year(), day.month()) == end_month,
                CalendarResolution::Daily
                | CalendarResolution::Seconds(_)
                | CalendarResolution::Minutes(_)
                | CalendarResolution::Hours(_) => false,
            };
            if !same_period {
                break;
            }
            if let Some(close) = daily_close_for_local_day(hours, day, SessionKind::Both) {
                first_close = close;
            }
            day -= Duration::days(1);
        }
    }

    let mut day = first_close.with_timezone(&hours.tz).date_naive() - Duration::days(1);
    let mut previous_close = None;
    for _ in 0..21 {
        if let Some(close) = daily_close_for_local_day(hours, day, SessionKind::Both)
            && close < first_close
        {
            previous_close = Some(close);
            break;
        }
        day -= Duration::days(1);
    }
    let Some(previous_close) = previous_close else {
        panic!("calendar period has no preceding daily close");
    };

    // Probe just before the previous close so a continuous session whose next
    // open equals that close (for example a 24×7 UTC profile) is included.
    let Some(open_probe) = previous_close.checked_sub_signed(Duration::nanoseconds(1)) else {
        panic!("calendar period's preceding close cannot be probed");
    };
    let period_start = next_session_after(hours, open_probe).0;
    if period_start < first_close {
        period_start
    } else {
        panic!("calendar period has no session open before its first daily close");
    }
}

/// Returns the next trading-day close after `t` ("end of day").
///
/// Alias for [`candle_end`] with [`CalendarResolution::Daily`].
///
/// # Panics
///
/// Panics only on a malformed calendar; see [`candle_end_with`].
#[inline]
#[must_use]
pub fn time_end_of_day(hours: &MarketHours, t: DateTime<Utc>) -> DateTime<Utc> {
    candle_end(hours, t, CalendarResolution::Daily)
}
