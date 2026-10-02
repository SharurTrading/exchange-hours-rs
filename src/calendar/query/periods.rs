// SPDX-License-Identifier: MIT-0

//! Daily, weekly, and monthly close discovery over a [`QueryContext`].

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};

use super::gate::SourceGate;
use super::identity;
use super::replacement::{self, ExceptionDailyClose};
use super::schedule::{QueryContext, RuleSet, resolve_rule_bounds, rules};
use super::sessions::containing_session_with;
use crate::calendar::local_time::bounded_utc;
use crate::calendar::rule::SessionKind;

const CLOSE_LOOKAHEAD_DAYS: i64 = 21;

fn update_latest<G: SourceGate>(
    context: &QueryContext<'_, G>,
    day: NaiveDate,
    kind: SessionKind,
    latest: &mut Option<DateTime<Utc>>,
    candidate_open: DateTime<Utc>,
    candidate: DateTime<Utc>,
    ceiling: Option<DateTime<Utc>>,
) -> Result<(), G::Error> {
    if context.trade_date_for_bounds(candidate_open, candidate) == day
        && containing_session_with(context, candidate, kind)?.is_none()
        && ceiling.is_none_or(|limit| candidate <= limit)
        && latest.is_none_or(|current| candidate > current)
    {
        *latest = Some(candidate);
    }
    Ok(())
}

fn latest_close_for_trade_date<G: SourceGate>(
    context: &QueryContext<'_, G>,
    day: NaiveDate,
    kind: SessionKind,
    ceiling: Option<DateTime<Utc>>,
) -> Result<Option<DateTime<Utc>>, G::Error> {
    // This scan describes `day` from the identity's own tables, so a date
    // those tables withhold refuses instead of being described (LAW-COVERAGE;
    // see `daily_close_for_trade_date` for why the gate sits below the
    // caller-record path).
    context.require_answerable(day)?;
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
///
/// The day is gated at [`latest_close_for_trade_date`], the normal-week
/// derivation, not here: a caller record for a withheld date states the day
/// itself, and the overlay contracts keep that observable — the composed
/// daily bar over a replaced `Unsourced` date answers from the record
/// (`a_day_policy_clips_a_replaced_trading_day`). The derivation, by
/// contrast, describes the date from the identity's own tables, so a date
/// those tables withhold must refuse rather than let the neighbour scan
/// describe its normal week (LAW-COVERAGE). That refusal used to be an
/// accident of the derivation's containment probes reaching the day; the
/// per-occurrence gate window (issue #107) legitimately stopped most
/// resolves from consulting it, which would have let `is_closed_trade_date`
/// answer a withheld Muhurat Sunday from its normal week — an unsourced gap
/// reported as a plain closure is precisely the failure LAW-COVERAGE exists
/// to prevent.
pub(in crate::calendar) fn daily_close_for_trade_date<G: SourceGate>(
    context: &QueryContext<'_, G>,
    day: NaiveDate,
    kind: SessionKind,
) -> Result<Option<DateTime<Utc>>, G::Error> {
    match replacement::daily_close(context, day, kind) {
        ExceptionDailyClose::NoSession => Ok(None),
        ExceptionDailyClose::Close(close) => Ok(Some(close)),
        ExceptionDailyClose::NotGoverned => latest_close_for_trade_date(context, day, kind, None),
    }
}

pub(in crate::calendar) fn next_daily_close_after_with<G: SourceGate>(
    context: &QueryContext<'_, G>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Result<Option<DateTime<Utc>>, G::Error> {
    Ok(
        next_daily_close_and_trade_date_after_with(context, instant, kind)?
            .map(|(_day, close)| close),
    )
}

/// Walks forward from `instant` for the next daily close, at most
/// `CLOSE_LOOKAHEAD_DAYS` days.
///
/// Running out of window is [`CalendarQueryError::SearchExhausted`] for an
/// identity-backed source, reported against the day the walk stopped on: it is
/// a different answer from a day the identity refuses, which the per-day gate
/// below raises instead (LAW-COVERAGE). A **detached fixed snapshot** has no
/// coverage to attribute an error to — its gate is infallible — so it keeps
/// its exhaustive `None` absence.
fn next_daily_close_and_trade_date_after_with<G: SourceGate>(
    context: &QueryContext<'_, G>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Result<Option<(NaiveDate, DateTime<Utc>)>, G::Error> {
    let tz = context.tz();
    let local_day = bounded_utc(instant, tz).with_timezone(&tz).date_naive();
    let mut day = local_day.pred_opt().unwrap_or(local_day);
    // The walk starts one local day back because a close can still be assigned
    // to the day before the instant's own: SET Thailand's convention dates an
    // after-midnight night phase to its prior local opening date
    // (`back_dates_trade_dates`), and a replacement record keyed to the start
    // day can own blocks whose windows reach past it. Under the close-date
    // default every occurrence assigned to the start day closes within it —
    // strictly before the instant's local day begins — and a wrapping rule
    // opening there is dated by its own close's day, which the walk reaches
    // through the next trade date's neighbour scan without reading the start
    // day at all. Visiting a start day that cannot contribute demands an
    // answer for the neighbour itself, and a refused window-edge neighbour
    // would then refuse a walk whose answer lies entirely on the queried day
    // and after it (#257).
    if day < local_day
        && !identity::back_dates_trade_dates(context)
        && !context.replacement_blocks_may_reach(day, day)
    {
        day = local_day;
    }
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
    // The bounded horizon ran out. The gate attributes the exhaustion for an
    // identity-backed source and is infallible for a detached snapshot, whose
    // bounded `None` is then the walk's own answer.
    context.search_exhausted(day)?;
    Ok(None)
}

pub(in crate::calendar) fn trade_date_for_daily_close<G: SourceGate>(
    context: &QueryContext<'_, G>,
    close: DateTime<Utc>,
    kind: SessionKind,
) -> Result<Option<NaiveDate>, G::Error> {
    let Some(probe) = close.checked_sub_signed(Duration::nanoseconds(1)) else {
        return Ok(None);
    };
    let Some((open, resolved_close)) = containing_session_with(context, probe, kind)? else {
        return Ok(None);
    };
    Ok((resolved_close == close).then(|| context.trade_date_for_bounds(open, close)))
}

pub(in crate::calendar) fn next_weekly_close_after_with<G: SourceGate>(
    context: &QueryContext<'_, G>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Result<Option<DateTime<Utc>>, G::Error> {
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

pub(in crate::calendar) fn next_monthly_close_after_with<G: SourceGate>(
    context: &QueryContext<'_, G>,
    instant: DateTime<Utc>,
    kind: SessionKind,
) -> Result<Option<DateTime<Utc>>, G::Error> {
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
