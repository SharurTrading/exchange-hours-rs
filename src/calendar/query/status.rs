// SPDX-License-Identifier: MIT-0

//! Open-state, maintenance-gap, and closed-day queries.

use chrono::{DateTime, Duration, NaiveDate, Utc};
use chrono_tz::Tz;

use super::periods::latest_close_at_or_before;
use super::schedule::QueryContext;
use super::sessions::{containing_session_with, next_session_after_with};
use crate::calendar::local_time::{bounded_utc, mk_local_open};
use crate::calendar::rule::SessionKind;

pub(in crate::calendar) fn is_open_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> bool {
    containing_session_with(context, instant, kind).is_some()
}

pub(in crate::calendar) fn is_maintenance(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
) -> bool {
    let max_gap = Duration::hours(6);
    if is_open_with(context, instant, SessionKind::Both) {
        return false;
    }
    let Some((next_open, _close)) = next_session_after_with(context, instant, SessionKind::Both)
    else {
        return false;
    };
    if next_open - instant >= max_gap {
        return false;
    }
    latest_close_at_or_before(context, instant, SessionKind::Both)
        .is_some_and(|previous_close| next_open - previous_close < max_gap)
}

pub(in crate::calendar) fn is_closed_all_day_in_calendar(
    context: &QueryContext<'_>,
    day: NaiveDate,
    calendar_tz: Tz,
    kind: SessionKind,
) -> bool {
    let start = mk_local_open(calendar_tz, day, 0).with_timezone(&Utc);
    let end = day.succ_opt().map_or(DateTime::<Utc>::MAX_UTC, |next| {
        mk_local_open(calendar_tz, next, 0).with_timezone(&Utc)
    });
    if start >= end {
        return true;
    }
    if is_open_with(context, start, kind) {
        return false;
    }
    next_session_after_with(context, start, kind).is_none_or(|(next_open, _close)| next_open >= end)
}

pub(in crate::calendar) fn is_closed_all_day_at(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    calendar_tz: Tz,
    kind: SessionKind,
) -> bool {
    let day = bounded_utc(instant, calendar_tz)
        .with_timezone(&calendar_tz)
        .date_naive();
    is_closed_all_day_in_calendar(context, day, calendar_tz, kind)
}
