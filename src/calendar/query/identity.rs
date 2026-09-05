// SPDX-License-Identifier: MIT-0

//! Identity-derived trade-date assignment and rule-adjacency conventions.
//!
//! These are capabilities of a named schedule identity, never shape
//! heuristics: adjacent rules are real phase boundaries for most profiles, and
//! a detached fixed snapshot has no identity with which to apply either
//! convention.

use chrono::{DateTime, Datelike, Duration, NaiveDate, Timelike, Utc, Weekday};

use super::schedule::QueryContext;
use crate::calendar::{CalendarSource, Exchange, MarketHoursKey};

const TRADE_DATE_LOOKAHEAD_DAYS: usize = 14;

/// Returns whether this identified calendar joins storage-only rule pieces.
pub(super) fn joins_adjacent_same_kind(context: &QueryContext<'_>) -> bool {
    matches!(
        context.identity(),
        Some(CalendarSource::MarketHoursKey(
            MarketHoursKey::GlobexCryptocurrency
        ))
    )
}

/// Assigns bounds produced by a normal-week rule to their venue-local trade date.
///
/// Most profiles use the local date of the final close. Three sourced
/// exceptions survive: SET's after-midnight DR night phase belongs to its prior
/// local opening date, CBOT Rough Rice's evening leg belongs to the following
/// local date, and CME cryptocurrency's weekend blocks carry the following open
/// business date.
pub(super) fn assign_normal(
    context: &QueryContext<'_>,
    open: DateTime<Utc>,
    close: DateTime<Utc>,
) -> NaiveDate {
    let tz = context.tz();
    let default = close.with_timezone(&tz).date_naive();
    let Some(source) = context.identity() else {
        return default;
    };
    let local_open = open.with_timezone(&tz);
    if matches!(source, CalendarSource::Exchange(Exchange::SetThailand)) {
        return if local_open.time().num_seconds_from_midnight() < 3 * 3_600 {
            local_open.date_naive().pred_opt().unwrap_or(default)
        } else {
            local_open.date_naive()
        };
    }
    // CBOT Rough Rice stopped wrapping past local midnight on 2018-01-21, so
    // the close-date default would put Sunday's 19:00-21:00 CT session on
    // Sunday. CBOT Submission 18-001 states the operator's own assignment for
    // this contract: the session effective "on Sunday, January 21, 2018" is
    // "for trade date Monday, January 22, 2018". Every evening leg therefore
    // carries the following local date. Before the divergence the leg wrapped
    // and the close-date default already produced that same answer, so this
    // branch changes no pre-2018 result.
    if matches!(
        source,
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexRoughRice)
    ) {
        return if local_open.time().num_seconds_from_midnight() >= 19 * 3_600 {
            local_open.date_naive().succ_opt().unwrap_or(default)
        } else {
            default
        };
    }
    if !matches!(
        source,
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexCryptocurrency)
    ) {
        return default;
    }

    let days_to_monday = match local_open.weekday() {
        Weekday::Fri if local_open.time().num_seconds_from_midnight() >= 16 * 3_600 => 3,
        Weekday::Sat => 2,
        Weekday::Sun => 1,
        _ => 0,
    };
    let nominal = if days_to_monday == 0 {
        default
    } else {
        local_open
            .date_naive()
            .checked_add_signed(Duration::days(days_to_monday))
            .unwrap_or(default)
    };

    // The permanent 24/7 schedule assigns holiday/weekend trading to the
    // following business day. A date the caller's overlays close — a
    // `DayPolicy` closed date or an exception provider's `Closed` record — is
    // skipped rather than deleting the connected trading block. Legacy
    // five-day profiles retain ordinary closed-trade-date behavior.
    if !context.has_overlay() || context.has_weekend_close_at(open) {
        return nominal;
    }
    let mut candidate = nominal;
    for _ in 0..=TRADE_DATE_LOOKAHEAD_DAYS {
        if !matches!(candidate.weekday(), Weekday::Sat | Weekday::Sun)
            && !context.trade_date_is_closed(candidate)
        {
            return candidate;
        }
        let Some(next) = candidate.succ_opt() else {
            return nominal;
        };
        candidate = next;
    }
    nominal
}
