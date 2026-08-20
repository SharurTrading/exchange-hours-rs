// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! Session bounds: which `[open, close)` window an instant sits in, and which
//! one comes next.
//!
//! Every function here returns a half-open UTC pair — the close is exclusive, so
//! `close` is the first instant *not* in the session. The search order is
//! deliberate: today's rules, then yesterday's wrap rules (an overnight session
//! that has not closed yet), then forward to the next session. Skipping the
//! second step is how an instant at 02:00 inside a 17:00→16:00 Globex session
//! would be misreported as belonging to the *next* session.
//!
//! Forward scans are bounded at 14 local days. A profile with no rule at all
//! (for example a venue queried before its go-live date) therefore returns the
//! degenerate `(end_excl, end_excl)` pair rather than looping; callers that need
//! progress must treat `open == close` as "no session".

use chrono::{DateTime, Datelike, Duration, Timelike, Utc};

use super::local_time::{is_holiday, mk_local_close, mk_local_open};
use super::{MarketHours, SessionKind};

/// Returns the `[open, close)` UTC bounds of the session of `kind` that contains `t`.
///
/// When `t` lies inside a session, that session's bounds are returned. Otherwise
/// the search falls back, in order, to: (1) a previous-day wrap session that
/// spills into `t`'s local day (unless the open day is a holiday), then (2) the
/// next session strictly after `t` via [`next_session_after_with`]. Bounds are
/// end-exclusive on the close; opens resolve to the earliest DST mapping and
/// closes to the latest.
#[must_use]
pub fn session_bounds_with(
    hours: &MarketHours,
    t: DateTime<Utc>,
    kind: SessionKind,
) -> (DateTime<Utc>, DateTime<Utc>) {
    let local = t.with_timezone(&hours.tz);
    let day = local.date_naive();
    let ssm = i64::from(local.num_seconds_from_midnight());

    let w_today = day.weekday().num_days_from_monday() as usize;
    let yday_date = day - chrono::Duration::days(1);
    let yday = yday_date.weekday().num_days_from_monday() as usize;

    // 1) Try all of TODAY's rules — none exist if today is a holiday
    //    (mirrors the day-skip in `next_session_after_with`).
    if !is_holiday(hours, day) {
        for r in hours.iter_rules(kind).filter(|r| r.days[w_today]) {
            if (i64::from(r.open_ssm)) <= (i64::from(r.close_ssm)) {
                // same-day
                if ssm >= i64::from(r.open_ssm) && ssm < i64::from(r.close_ssm) {
                    let open = mk_local_open(hours.tz, day, r.open_ssm);
                    let close = mk_local_close(hours.tz, day, r.close_ssm);
                    return (open.with_timezone(&Utc), close.with_timezone(&Utc));
                }
            } else {
                // wrap (open today, close tomorrow)
                if ssm >= i64::from(r.open_ssm) {
                    // ensure tomorrow isn't a holiday (no wrap bleed into holiday)
                    if is_holiday(hours, day + chrono::Duration::days(1)) {
                        continue;
                    }
                    let open = mk_local_open(hours.tz, day, r.open_ssm);
                    let close =
                        mk_local_close(hours.tz, day + chrono::Duration::days(1), r.close_ssm);
                    return (open.with_timezone(&Utc), close.with_timezone(&Utc));
                }
            }
        }
    }

    // 2) Try all of YESTERDAY's WRAP rules that spill into today — the wrap
    //    exists only if neither its open day nor its close day (today) is a
    //    holiday.
    if !is_holiday(hours, yday_date) && !is_holiday(hours, day) {
        for r in hours.iter_rules(kind).filter(|r| r.days[yday]) {
            if r.open_ssm > r.close_ssm {
                // yesterday had a wrap; if we're before today's wrap close, we are inside it
                if ssm < i64::from(r.close_ssm) {
                    let open = mk_local_open(hours.tz, day - chrono::Duration::days(1), r.open_ssm);
                    let close = mk_local_close(hours.tz, day, r.close_ssm);
                    return (open.with_timezone(&Utc), close.with_timezone(&Utc));
                }
            }
        }
    }

    // 3) Otherwise, fall forward to the next session after t.
    next_session_after_with(hours, t, kind)
}

/// Returns [`session_bounds_with`] over [`SessionKind::Both`] (regular + extended).
#[must_use]
pub fn session_bounds(hours: &MarketHours, t: DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
    session_bounds_with(hours, t, SessionKind::Both)
}

/// Returns the `[open, close)` UTC bounds of the next session of `kind` that
/// opens strictly after `end_excl`.
///
/// Scans up to 14 local days forward, skipping holidays and refusing wrap
/// sessions whose close day is a holiday; within a day it picks the earliest
/// qualifying open. If no session is found in that horizon it returns the
/// degenerate `(end_excl, end_excl)`.
#[must_use]
pub fn next_session_after_with(
    hours: &MarketHours,
    end_excl: DateTime<Utc>,
    kind: SessionKind,
) -> (DateTime<Utc>, DateTime<Utc>) {
    let local = end_excl.with_timezone(&hours.tz);
    let base_day = local.date_naive();

    for dd in 0..14 {
        let d = base_day + Duration::days(dd);
        if is_holiday(hours, d) {
            continue;
        }

        let mut best_open: Option<(DateTime<Utc>, DateTime<Utc>)> = None;
        let w = d.weekday().num_days_from_monday() as usize; // Mon=0..Sun=6

        for r in hours.iter_rules(kind) {
            if !r.days[w] {
                continue;
            }

            if r.open_ssm <= r.close_ssm {
                // same-day
                let open_l = mk_local_open(hours.tz, d, r.open_ssm);
                if open_l <= local {
                    continue; // strictly after end_excl
                }
                let close_l = mk_local_close(hours.tz, d, r.close_ssm);
                let cand = (open_l.with_timezone(&Utc), close_l.with_timezone(&Utc));
                best_open = match best_open {
                    None => Some(cand),
                    Some(cur) => Some(if cand.0 < cur.0 { cand } else { cur }),
                };
            } else {
                // wrap (open d, close d+1) — only valid if next day is not a holiday
                if is_holiday(hours, d + Duration::days(1)) {
                    continue;
                }
                let open_l = mk_local_open(hours.tz, d, r.open_ssm);
                if open_l <= local {
                    continue;
                }
                let close_l = mk_local_close(hours.tz, d + Duration::days(1), r.close_ssm);
                let cand = (open_l.with_timezone(&Utc), close_l.with_timezone(&Utc));
                best_open = match best_open {
                    None => Some(cand),
                    Some(cur) => Some(if cand.0 < cur.0 { cand } else { cur }),
                };
            }
        }

        if let Some(b) = best_open {
            return b;
        }
    }
    // Fallback: if nothing found within 14 days, return a degenerate bound.
    (end_excl, end_excl)
}

/// Returns [`next_session_after_with`] over [`SessionKind::Both`] (regular + extended).
#[must_use]
pub fn next_session_after(
    hours: &MarketHours,
    end_excl: DateTime<Utc>,
) -> (DateTime<Utc>, DateTime<Utc>) {
    next_session_after_with(hours, end_excl, SessionKind::Both)
}

/// Returns only the open instant of the next session after `after_utc`.
///
/// Thin projection of [`next_session_after`], returning only its `.0` field.
#[inline]
#[must_use]
pub fn next_session_open_after(mh: &MarketHours, after_utc: DateTime<Utc>) -> DateTime<Utc> {
    next_session_after(mh, after_utc).0
}
