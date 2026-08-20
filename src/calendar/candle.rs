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
use super::session::{next_session_after_with, session_bounds_with};
use super::{CalendarResolution, MarketHours, SessionKind};

/// Returns the session-aware bar close after `t` for `res` and `kind`, or
/// `None` when no bar exists.
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
/// A returned end is always strictly after `t`. `None` means the calendar
/// cannot produce an advancing bar: the profile has no session of `kind`
/// inside the bounded search horizon (for example a venue queried before its
/// go-live date), or the interval is zero (`Seconds(0)` / `Minutes(0)` /
/// `Hours(0)`), which could never advance.
#[must_use]
pub fn candle_end_with(
    hours: &MarketHours,
    t: DateTime<Utc>,
    res: CalendarResolution,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    if is_zero_interval(res) {
        return None;
    }
    match res {
        CalendarResolution::Seconds(s) => Some(t + Duration::seconds(i64::from(s))),
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

/// True for the intraday variants whose interval count is zero — a bar that
/// could never advance, reported as `None` by the candle queries.
fn is_zero_interval(res: CalendarResolution) -> bool {
    matches!(
        res,
        CalendarResolution::Seconds(0)
            | CalendarResolution::Minutes(0)
            | CalendarResolution::Hours(0)
    )
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
) -> Option<DateTime<Utc>> {
    let (open, close) = session_bounds_with(hours, t, kind)?;
    let anchor = if t < open { open } else { t };
    let mut end = (anchor + step).min(close);

    if end == close {
        let close_day = close.with_timezone(&hours.tz).date_naive();
        if let Some(dc) = daily_close_for_local_day(hours, close_day, kind)
            && dc == close
            && hours.is_maintenance(close)
            && let Some((next_open, _next_close)) = next_session_after_with(hours, close, kind)
        {
            end = next_open;
        }
    }

    Some(end)
}

/// Returns [`candle_end_with`] over [`SessionKind::Both`] (regular + extended).
#[inline]
#[must_use]
pub fn candle_end(
    hours: &MarketHours,
    t: DateTime<Utc>,
    interval: CalendarResolution,
) -> Option<DateTime<Utc>> {
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
/// `None` mirrors [`candle_end`]: a profile with no session inside the
/// bounded search horizons (for example a venue queried before its go-live
/// date), or a zero intraday interval, has no bar and therefore no start.
#[inline]
#[must_use]
pub fn candle_start(
    hours: &MarketHours,
    t: DateTime<Utc>,
    interval: CalendarResolution,
) -> Option<DateTime<Utc>> {
    candle_start_with(hours, t, interval, SessionKind::Both)
}

/// Returns the opening instant paired with [`candle_end_with`] for the same
/// `kind`: the kind-aware form of [`candle_start`], consulting only the
/// selected session set for anchors and period boundaries. Returns `None`
/// exactly when the paired [`candle_end_with`] cannot produce a bar.
#[must_use]
pub fn candle_start_with(
    hours: &MarketHours,
    t: DateTime<Utc>,
    interval: CalendarResolution,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    if is_zero_interval(interval) {
        return None;
    }
    match interval {
        CalendarResolution::Seconds(_) => Some(t),
        CalendarResolution::Minutes(_) | CalendarResolution::Hours(_) => {
            let (session_open, _) = session_bounds_with(hours, t, kind)?;
            Some(t.max(session_open))
        }
        CalendarResolution::Daily | CalendarResolution::Weekly | CalendarResolution::Monthly => {
            period_start_of(hours, t, interval, kind)
        }
    }
}

/// Finds the open instant of the daily/weekly/monthly period containing (or
/// following) `t`: the first session open after the period's *preceding* daily
/// close. Returns `None` when the calendar cannot produce one inside its
/// bounded horizons — no period close after `t` (a no-session profile), no
/// preceding daily close, or no session open between the two. Intraday
/// resolutions are not calendar periods and also return `None`; callers route
/// them through the fixed-grid path instead.
fn period_start_of(
    hours: &MarketHours,
    t: DateTime<Utc>,
    interval: CalendarResolution,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    let period_end = match interval {
        CalendarResolution::Daily => next_daily_close_after_with(hours, t, kind)?,
        CalendarResolution::Weekly => next_weekly_close_after_with(hours, t, kind)?,
        CalendarResolution::Monthly => next_monthly_close_after_with(hours, t, kind)?,
        CalendarResolution::Seconds(_)
        | CalendarResolution::Minutes(_)
        | CalendarResolution::Hours(_) => return None,
    };
    let end_local = period_end.with_timezone(&hours.tz);
    let end_day = end_local.date_naive();
    let end_iso_week = end_local.iso_week();
    let end_month = (end_local.year(), end_local.month());

    // Walk backwards to the earliest daily close still inside the period.
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
            if let Some(close) = daily_close_for_local_day(hours, day, kind) {
                first_close = close;
            }
            day -= Duration::days(1);
        }
    }

    // The period opens at the first session open after the daily close that
    // *precedes* its first close. 21 days clears any weekend/holiday run in a
    // normal-week profile; failing to find one inside that horizon means the
    // calendar cannot anchor the period.
    let mut day = first_close.with_timezone(&hours.tz).date_naive() - Duration::days(1);
    let mut previous_close = None;
    for _ in 0..21 {
        if let Some(close) = daily_close_for_local_day(hours, day, kind)
            && close < first_close
        {
            previous_close = Some(close);
            break;
        }
        day -= Duration::days(1);
    }
    let previous_close = previous_close?;

    // Probe just before the previous close so a continuous session whose next
    // open equals that close (for example a 24×7 UTC profile) is included.
    let open_probe = previous_close.checked_sub_signed(Duration::nanoseconds(1))?;
    let (period_start, _) = next_session_after_with(hours, open_probe, kind)?;
    (period_start < first_close).then_some(period_start)
}

/// Returns the next trading-day close after `t` ("end of day"), or `None`
/// when the profile has no daily close within the bounded search horizon.
///
/// Alias for [`candle_end`] with [`CalendarResolution::Daily`].
#[inline]
#[must_use]
pub fn time_end_of_day(hours: &MarketHours, t: DateTime<Utc>) -> Option<DateTime<Utc>> {
    candle_end(hours, t, CalendarResolution::Daily)
}
