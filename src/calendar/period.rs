// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! Calendar-period closes: daily, weekly, monthly.
//!
//! A "day close" here is **not** midnight and not a fixed hour — it is the
//! latest session close that lands on a given exchange-local calendar day,
//! counting both same-day sessions and wrap sessions that opened the previous
//! day. That definition is what makes the CME trading day that opens Sunday
//! 17:00 CT close on *Monday*, and it is why every period boundary in the crate
//! is derived from session rules rather than from elapsed duration.
//!
//! Weekly and monthly boundaries are then found by walking daily closes forward
//! until the next one falls in a different ISO week / `(year, month)`. Grouping
//! by the period the close belongs to — rather than by the period the bar
//! started in — is what makes year rollover and short months fall out by
//! construction.
//!
//! Scans are bounded at 21 local days, enough to clear any weekend or
//! consecutive-holiday run in a normal-week profile. Every function here
//! returns `Option`: `None` means no close exists inside that horizon — a
//! profile with no rules at all — and the public candle surface maps it to the
//! documented degenerate value instead of looping or panicking. The walk loops
//! terminate by construction: a local day contributes at most one daily close,
//! so each iteration advances at least one day and a week/month boundary is
//! reached within at most 7/31 closes.

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};

use super::local_time::{is_holiday, mk_local_close};
use super::{MarketHours, SessionKind};

/// Next "day close" strictly after `t` for this market and session kind, or
/// `None` if no session close exists within the bounded 21-day horizon.
/// A "day close" is the **latest session close that occurs on a local calendar day**.
/// - Same-day sessions contribute closes on that day.
/// - Wrap sessions contribute a close on the **next** local day, but only if the
///   previous day (the open day) is not a holiday (no wrap across holidays).
pub(crate) fn next_daily_close_after_with(
    hours: &MarketHours,
    t: DateTime<Utc>,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    let tz = hours.tz;
    let mut day = t.with_timezone(&tz).date_naive();

    for _ in 0..21 {
        if let Some(close) = daily_close_for_local_day(hours, day, kind)
            && close > t
        {
            return Some(close);
        }
        day += Duration::days(1);
    }

    None
}

/// Compute the last close **that occurs on** `day` (exchange-local day).
pub(crate) fn daily_close_for_local_day(
    hours: &MarketHours,
    day: NaiveDate,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    // If the close occurs on `day`, we may have two sources:
    //  (A) same-day sessions on `day`
    //  (B) wrap sessions opened on `day-1` that close on `day` (only if `day-1` not a holiday)
    let tz = hours.tz;

    if is_holiday(hours, day) {
        return None;
    }

    let w_today = day.weekday().num_days_from_monday() as usize;

    let mut best: Option<DateTime<Utc>> = None;

    for r in hours.iter_rules(kind) {
        if r.open_ssm <= r.close_ssm {
            // same-day close occurs on `day` if rule is active on `day`
            if r.days[w_today] {
                let close = mk_local_close(tz, day, r.close_ssm).with_timezone(&Utc);
                best = Some(best.map_or(close, |b| b.max(close)));
            }
        } else {
            // wrap: close occurs on `day` if:
            // - `day-1` is active in rule
            // - `day-1` is NOT a holiday (no wrap bleed into holiday)
            let yday = day - Duration::days(1);
            if is_holiday(hours, yday) {
                continue;
            }
            let w_yday = yday.weekday().num_days_from_monday() as usize;
            if r.days[w_yday] {
                let close = mk_local_close(tz, day, r.close_ssm).with_timezone(&Utc);
                best = Some(best.map_or(close, |b| b.max(close)));
            }
        }
    }

    best
}

/// Next weekly close strictly after `t`: find the **last** day-close
/// that belongs to its ISO week (in exchange TZ). `None` if no daily close
/// exists within the bounded horizon at all.
pub(crate) fn next_weekly_close_after_with(
    hours: &MarketHours,
    t: DateTime<Utc>,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    let tz = hours.tz;

    // First candidate daily close after t.
    let mut c = next_daily_close_after_with(hours, t, kind)?;

    // Keep advancing daily closes while they are still in the same ISO week
    // as the candidate `c`. The first close whose **next** daily close is in a
    // different ISO week is the weekly boundary. Terminates: each daily close
    // lands on a later local day, so the week changes within at most 7 steps.
    loop {
        let c_local = c.with_timezone(&tz);
        let c_week = (c_local.iso_week().year(), c_local.iso_week().week());

        // Tiny epsilon to move strictly past `c`. If the horizon runs out,
        // `c` is the last close the calendar knows about — return it.
        let Some(c_next) = next_daily_close_after_with(hours, c + Duration::nanoseconds(1), kind)
        else {
            return Some(c);
        };
        let n_local = c_next.with_timezone(&tz);
        let n_week = (n_local.iso_week().year(), n_local.iso_week().week());

        if n_week != c_week {
            return Some(c); // `c` is the last close of its week
        }

        c = c_next;
    }
}

/// Next monthly close strictly after `t`: find the **last** day-close that
/// belongs to its exchange-local calendar month. Mirrors
/// [`next_weekly_close_after_with`] but groups by `(year, month)` instead of ISO
/// week, so the boundary handles month lengths and year rollover by construction.
/// `None` if no daily close exists within the bounded horizon at all.
pub(crate) fn next_monthly_close_after_with(
    hours: &MarketHours,
    t: DateTime<Utc>,
    kind: SessionKind,
) -> Option<DateTime<Utc>> {
    let tz = hours.tz;

    // First candidate daily close after `t`.
    let mut c = next_daily_close_after_with(hours, t, kind)?;

    // Keep advancing daily closes while the next close is still in the same
    // calendar month as the candidate `c`. The first close whose **next** daily
    // close lands in a different month is the monthly boundary. Terminates for
    // the same reason as the weekly walk (at most 31 steps).
    loop {
        let c_local = c.with_timezone(&tz);
        let c_month = (c_local.year(), c_local.month());

        // Tiny epsilon to move strictly past `c`. If the horizon runs out,
        // `c` is the last close the calendar knows about — return it.
        let Some(c_next) = next_daily_close_after_with(hours, c + Duration::nanoseconds(1), kind)
        else {
            return Some(c);
        };
        let n_local = c_next.with_timezone(&tz);
        let n_month = (n_local.year(), n_local.month());

        if n_month != c_month {
            return Some(c); // `c` is the last close of its month
        }

        c = c_next;
    }
}
