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

/// How far past its opening day a rolling family's trade date can land.
///
/// Three days carry a Friday-evening open to Monday, the business-date roll
/// then walks at most [`TRADE_DATE_LOOKAHEAD_DAYS`] further, and one more day
/// covers the close-date default's own wrap: `3 + 14 + 1`.
const ROLLING_WINDOW_DAYS: i64 = 18;

/// Returns the inclusive trade-date window an occurrence opening on `open_day`
/// can be assigned to.
///
/// This is the coverage gate's search window, and it is a claim about the
/// identity's own conventions rather than an estimate: a day-level layer can
/// only change this occurrence's answer by holding a record for a date
/// [`assign_normal`] could return for it.
///
/// | Identity class | Reachable trade dates | Window |
/// |---|---|---|
/// | ordinary (close-date default) | `D`, `D+1` | `[D, D+1]` |
/// | SET Thailand (prior opening date) | `D-1`, `D` | `[D-1, D+1]` |
/// | CBOT Rough Rice (following local date) | `D`, `D+1` | `[D, D+1]` |
/// | CME cryptocurrency, `ECBTC` (business-date roll) | up to `D+3+14` | `[D, D+18]` |
///
/// A rule spans at most one local midnight, so the close-date default can only
/// land on `D` or `D+1`; the three identity conventions extend that by their
/// own documented amount, and the rolling families by their weekend offset plus
/// the roll's bounded lookahead. Each bound is a safe superset of what
/// [`assign_normal`] can actually produce.
///
/// `None` means the window could not be formed at the extremes of the
/// representable calendar, which sends the caller down the ungated path — the
/// answer is the same either way, only the cost differs.
pub(super) fn trade_date_window(
    context: &QueryContext<'_>,
    open_day: NaiveDate,
) -> Option<(NaiveDate, NaiveDate)> {
    let (before, after) = match context.identity() {
        Some(CalendarSource::Exchange(Exchange::SetThailand)) => (1, 1),
        Some(CalendarSource::MarketHoursKey(
            MarketHoursKey::GlobexCryptocurrency | MarketHoursKey::GlobexEventContractsBtc,
        )) => (0, ROLLING_WINDOW_DAYS),
        _ => (0, 1),
    };
    let first = open_day.checked_sub_signed(Duration::days(before))?;
    let last = open_day.checked_add_signed(Duration::days(after))?;
    Some((first, last))
}

/// Returns whether this identified calendar joins storage-only rule pieces.
pub(super) fn joins_adjacent_same_kind(context: &QueryContext<'_>) -> bool {
    matches!(
        context.identity(),
        Some(CalendarSource::MarketHoursKey(
            MarketHoursKey::GlobexCryptocurrency | MarketHoursKey::GlobexEventContractsBtc
        ))
    )
}

/// Assigns bounds produced by a normal-week rule to their venue-local trade date.
///
/// Most profiles use the local date of the final close. Three sourced
/// exceptions survive: SET's after-midnight DR night phase belongs to its prior
/// local opening date, CBOT Rough Rice's evening leg belongs to the following
/// local date, and the weekend blocks of CME cryptocurrency and of `ECBTC` —
/// whose documents both say the daily window rolls the trade date — carry the
/// following open business date.
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
        CalendarSource::MarketHoursKey(
            MarketHoursKey::GlobexCryptocurrency | MarketHoursKey::GlobexEventContractsBtc
        )
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
