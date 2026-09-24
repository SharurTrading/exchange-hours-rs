// SPDX-License-Identifier: MIT-0

//! Open-state, maintenance-gap, and closed-day queries.

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};
use chrono_tz::Tz;

use super::periods::{daily_close_for_trade_date, next_daily_close_after_with};
use super::schedule::QueryContext;
use super::sessions::{
    containing_session_with, contains_in_session_with, next_session_after_with,
    previous_session_before_with,
};
use crate::calendar::SessionState;
use crate::calendar::local_time::{bounded_utc, mk_local_open};
use crate::calendar::rule::SessionKind;
use crate::calendar::{CalendarQueryError, SUPPORT_FLOOR};

const MAX_MAINTENANCE_GAP: Duration = Duration::hours(4);

pub(in crate::calendar) fn is_open_with(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Result<bool, CalendarQueryError> {
    // An in-range instant whose session opened before the floor is still
    // answered whole (the plan permits retaining earlier context), so the
    // containment probe runs first. A **negative** answer then has to pass the
    // floor: it may be negative only because the floor withheld the day that
    // would have said otherwise, and LAW-COVERAGE forbids reporting that as a
    // closure.
    let open = contains_in_session_with(context, instant, kind)?;
    context.require_floor_at(instant)?;
    Ok(open)
}

pub(in crate::calendar) fn is_order_entry_only(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
) -> Result<bool, CalendarQueryError> {
    Ok(!is_open_with(context, instant, SessionKind::Both)?
        && context.contains_order_entry(instant)?)
}

pub(in crate::calendar) fn is_accepting_orders(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
) -> Result<bool, CalendarQueryError> {
    Ok(is_open_with(context, instant, SessionKind::Both)?
        || context.contains_order_entry(instant)?)
}

pub(in crate::calendar) fn is_maintenance(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
) -> Result<bool, CalendarQueryError> {
    Ok(session_state(context, instant)? == SessionState::Maintenance)
}

pub(in crate::calendar) fn trade_date(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
) -> Result<Option<NaiveDate>, CalendarQueryError> {
    // Resolve first, then judge -- and judge on **every** exit, including the
    // early one. A containing session answers the instant, but its trade date
    // is still a date this identity must have a sourced answer for: reporting
    // `Ok(Some(2024-06-03))` for a pre-floor instant would hand a caller a
    // trade date the identity cannot state (LAW-COVERAGE).
    let answer = resolve_trade_date(context, instant)?;
    let Some(day) = answer else {
        context.require_floor_at(instant)?;
        return Ok(None);
    };
    context.require_floor(Some(day))?;
    // The containing session's own opening day may precede the floor when the
    // instant is in range, which the plan permits; only the *trade date* has to
    // be one this identity can state. That is a claim only an **identity** makes:
    // a detached caller-supplied snapshot claims nothing about coverage, so it
    // keeps the trade date it derives. Withholding it there would delete the
    // pre-floor `Halt`/`Maintenance` classification `session_state` reads from
    // this answer, turning a sourced gap into a plain closure.
    if context.is_identity_backed() && day < SUPPORT_FLOOR {
        return Ok(None);
    }
    Ok(Some(day))
}

/// Resolves the trade date an instant belongs to, before any coverage verdict.
///
/// An order-entry phase is not a session, so it has no containing bounds - but
/// it exists to feed the session that follows it, and an order queued in a
/// Sunday pre-open belongs to Monday's trade date. That case resolves through
/// the next session rather than reporting absence.
fn resolve_trade_date(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
) -> Result<Option<NaiveDate>, CalendarQueryError> {
    if let Some((open, _session_close)) =
        containing_session_with(context, instant, SessionKind::Both)?
    {
        let Some(close) = next_daily_close_after_with(context, instant, SessionKind::Both)? else {
            return Ok(None);
        };
        return Ok(Some(context.trade_date_for_bounds(open, close)));
    }
    if context.contains_order_entry(instant)? {
        let Some((open, _next_close)) =
            next_session_after_with(context, instant, SessionKind::Both)?
        else {
            return Ok(None);
        };
        let Some(close) = next_daily_close_after_with(context, open, SessionKind::Both)? else {
            return Ok(None);
        };
        return Ok(Some(context.trade_date_for_bounds(open, close)));
    }
    Ok(None)
}

pub(in crate::calendar) fn session_state(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
) -> Result<SessionState, CalendarQueryError> {
    context.require_floor_at(instant)?;
    if is_open_with(context, instant, SessionKind::Regular)? {
        return Ok(SessionState::OpenRegular);
    }
    if is_open_with(context, instant, SessionKind::Extended)? {
        return Ok(SessionState::OpenExtended);
    }
    // Checked before the gap classification below: an order-entry window that
    // sits inside a maintenance or post-close gap is more precisely described
    // by what a caller can actually do in it than by the gap around it.
    if context.contains_order_entry(instant)? {
        return Ok(SessionState::OrderEntry);
    }
    let Some((_previous_open, previous_close)) =
        previous_session_before_with(context, instant, SessionKind::Both)?
    else {
        return Ok(SessionState::Closed);
    };
    let Some((next_open, _next_close)) =
        next_session_after_with(context, instant, SessionKind::Both)?
    else {
        return Ok(SessionState::Closed);
    };
    let gap = next_open - previous_close;
    if gap <= MAX_MAINTENANCE_GAP && !context.has_weekend_close_at(instant) {
        // A source-designated continuously-traded week can retain short
        // operational maintenance inside one trade date (CME cryptocurrency's
        // Saturday window). Its absence of a weekend close is the explicit
        // profile capability; do not infer this exception from rule shapes.
        return Ok(SessionState::Maintenance);
    }
    let Some(previous_probe) = previous_close.checked_sub_signed(Duration::nanoseconds(1)) else {
        return Ok(SessionState::Closed);
    };
    let (Some(previous_trade_date), Some(next_trade_date)) = (
        trade_date(context, previous_probe)?,
        trade_date(context, next_open)?,
    ) else {
        return Ok(SessionState::Closed);
    };
    if previous_trade_date == next_trade_date {
        return Ok(SessionState::Halt);
    }
    let same_week = previous_trade_date.iso_week() == next_trade_date.iso_week();
    if same_week && gap <= MAX_MAINTENANCE_GAP {
        Ok(SessionState::Maintenance)
    } else {
        Ok(SessionState::Closed)
    }
}

pub(in crate::calendar) fn is_closed_trade_date(
    context: &QueryContext<'_>,
    day: NaiveDate,
    kind: SessionKind,
) -> Result<bool, CalendarQueryError> {
    context.require_floor(Some(day))?;
    Ok(daily_close_for_trade_date(context, day, kind)?.is_none())
}

pub(in crate::calendar) fn is_closed_all_day_in_calendar(
    context: &QueryContext<'_>,
    day: NaiveDate,
    calendar_tz: Tz,
    kind: SessionKind,
) -> Result<bool, CalendarQueryError> {
    context.require_floor(Some(day))?;
    let start = mk_local_open(calendar_tz, day, 0).with_timezone(&Utc);
    let end = day.succ_opt().map_or(DateTime::<Utc>::MAX_UTC, |next| {
        mk_local_open(calendar_tz, next, 0).with_timezone(&Utc)
    });
    if start >= end {
        return Ok(true);
    }
    if is_open_with(context, start, kind)? {
        return Ok(false);
    }
    Ok(next_session_after_with(context, start, kind)?
        .is_none_or(|(next_open, _close)| next_open >= end))
}

pub(in crate::calendar) fn is_closed_all_day_at(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
    calendar_tz: Tz,
    kind: SessionKind,
) -> Result<bool, CalendarQueryError> {
    let day = bounded_utc(instant, calendar_tz)
        .with_timezone(&calendar_tz)
        .date_naive();
    is_closed_all_day_in_calendar(context, day, calendar_tz, kind)
}
