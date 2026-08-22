// SPDX-License-Identifier: MIT-0

//! Open-state, maintenance-gap, and closed-day queries.

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};
use chrono_tz::Tz;

use super::periods::{daily_close_for_local_day, next_daily_close_after_with};
use super::schedule::QueryContext;
use super::sessions::{
    containing_session_with, contains_in_session_with, next_session_after_with,
    previous_session_before_with,
};
use crate::calendar::SessionState;
use crate::calendar::local_time::{bounded_utc, mk_local_open};
use crate::calendar::rule::SessionKind;

const MAX_MAINTENANCE_GAP: Duration = Duration::hours(4);

pub(in crate::calendar) fn is_open_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> bool {
    contains_in_session_with(context, instant, kind)
}

pub(in crate::calendar) fn is_maintenance(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
) -> bool {
    session_state(context, instant) == SessionState::Maintenance
}

pub(in crate::calendar) fn trade_date(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
) -> Option<NaiveDate> {
    let (open, _session_close) = containing_session_with(context, instant, SessionKind::Both)?;
    let close = next_daily_close_after_with(context, instant, SessionKind::Both)?;
    Some(context.trade_date_for_bounds(open, close))
}

pub(in crate::calendar) fn session_state(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
) -> SessionState {
    if is_open_with(context, instant, SessionKind::Regular) {
        return SessionState::OpenRegular;
    }
    if is_open_with(context, instant, SessionKind::Extended) {
        return SessionState::OpenExtended;
    }
    let Some((_previous_open, previous_close)) =
        previous_session_before_with(context, instant, SessionKind::Both)
    else {
        return SessionState::Closed;
    };
    let Some((next_open, _next_close)) =
        next_session_after_with(context, instant, SessionKind::Both)
    else {
        return SessionState::Closed;
    };
    let gap = next_open - previous_close;
    if gap <= MAX_MAINTENANCE_GAP && !context.has_weekend_close_at(instant) {
        // A source-designated continuously-traded week can retain short
        // operational maintenance inside one trade date (CME cryptocurrency's
        // Saturday window). Its absence of a weekend close is the explicit
        // profile capability; do not infer this exception from rule shapes.
        return SessionState::Maintenance;
    }
    let Some(previous_probe) = previous_close.checked_sub_signed(Duration::nanoseconds(1)) else {
        return SessionState::Closed;
    };
    let (Some(previous_trade_date), Some(next_trade_date)) = (
        trade_date(context, previous_probe),
        trade_date(context, next_open),
    ) else {
        return SessionState::Closed;
    };
    if previous_trade_date == next_trade_date {
        return SessionState::Halt;
    }
    let same_week = previous_trade_date.iso_week() == next_trade_date.iso_week();
    if same_week && gap <= MAX_MAINTENANCE_GAP {
        SessionState::Maintenance
    } else {
        SessionState::Closed
    }
}

pub(in crate::calendar) fn is_closed_trade_date(
    context: &QueryContext<'_>,
    day: NaiveDate,
    kind: SessionKind,
) -> bool {
    daily_close_for_local_day(context, day, kind).is_none()
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
