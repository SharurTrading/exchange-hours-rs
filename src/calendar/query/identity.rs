// SPDX-License-Identifier: MIT-0

//! Identity-derived trade-date assignment and rule-adjacency conventions.
//!
//! These are capabilities of a named schedule identity, never shape
//! heuristics: adjacent rules are real phase boundaries for most profiles, and
//! a detached fixed snapshot has no identity with which to apply either
//! convention.

use chrono::{DateTime, Datelike, Duration, NaiveDate, Timelike, Utc, Weekday};

use super::schedule::{QueryContext, RuleSet, rules};
use crate::calendar::local_time::mk_local_close;
use crate::calendar::rule::SessionKind;
use crate::calendar::{CalendarSource, Exchange, MarketHoursKey};

const TRADE_DATE_LOOKAHEAD_DAYS: usize = 14;

/// The forward half of a **self-dated** occurrence's window.
///
/// A session that opens on the occurrence's own local day and still closes
/// after `raw_open` is what makes the occurrence self-dated; see
/// [`trade_date_window`] for why its trade date cannot leave `[D, D + 1]`.
const SELF_DATED_AFTER: i64 = 1;

/// How far past the close walk's own reach a rolling family's trade date can
/// land.
///
/// Three days carry a Friday-evening open to Monday, the business-date roll
/// then walks at most [`TRADE_DATE_LOOKAHEAD_DAYS`] further, and one more day
/// covers the close-date default's own wrap: `3 + 14 + 1`.
const ROLLING_WINDOW_DAYS: i64 = 18;

/// The close walk's own reach — the window every occurrence that is not
/// self-dated gets.
///
/// The trade date is the local date of the **trading day's** final close
/// (`candle_end_with(.., Daily, Both)`), never of the rule's own close, so an
/// occurrence that opens after its own trading day's final close — CBOT's
/// 14:30-16:00 CT order-entry window on a Friday, `ICE`'s post-close queues —
/// is dated by the next trading day, three or more local days later.
/// `next_daily_close_and_trade_date_after_with` starts one local day back and
/// walks `CLOSE_LOOKAHEAD_DAYS` forward, so every derivable trade date lies in
/// `[D - 1, D + 19]`.
const DERIVED_BEFORE: i64 = 1;

/// The forward half of [`DERIVED_BEFORE`]'s bound.
const DERIVED_AFTER: i64 = 19;

/// Returns the inclusive trade-date window an occurrence can be assigned to.
///
/// This is the coverage gate's search window, and it is a claim about what the
/// derivation can reach rather than an estimate: a day-level layer can only
/// change this occurrence's answer by holding a record for a date
/// [`assign_normal`] could return for it, and `assign_normal` is applied on top
/// of a trade date the close walk has already produced. The gate's whole
/// question is therefore which trade dates this occurrence can carry, and there
/// are two answers.
///
/// **A self-dated occurrence takes `[D, D + 1]`.** When a `Regular` or
/// `Extended` rule that is active on `open_day` still closes after `raw_open`,
/// the occurrence lies inside — or before the end of — a tradeable block that
/// opened on its own local day. The close walk then stops at that block's own
/// final close, whose local date is the trade date, and no shipped session
/// occurrence is dated more than one local day past its own open
/// (`every_shipped_session_occurrence_is_dated_by_its_own_open_or_the_next_day`
/// fences that), so the trade date is `D` or `D + 1`. The layers cannot move
/// it: the walk runs over [`QueryContext::baseline`], which holds no layer at
/// all, and the close-date default reads nothing but that walk.
///
/// A tradeable rule answers that question with its own close, so the test costs
/// nothing on the session path; an order-entry rule is not a session and the
/// day's sessions have to be asked instead.
///
/// **Everything else takes the walk's full reach.** An occurrence whose own
/// trading day has already closed — CBOT's Friday 14:30 CT order-entry window,
/// ICE's post-close queues — is dated by the *next* trading day, three or more
/// local days later over a weekend. That is
/// `next_daily_close_and_trade_date_after_with`'s own span: it starts one local
/// day back and walks `CLOSE_LOOKAHEAD_DAYS` forward, so it is `[D - 1, D +
/// 19]`, widened by each identity convention's own documented offset — SET
/// Thailand's night phase can step one further back, and the business-date roll
/// of CME cryptocurrency and `ECBTC` can step [`ROLLING_WINDOW_DAYS`] further
/// forward *and* is itself layer-sensitive, because it skips the dates the
/// caller's layers close.
///
/// `None` means the window could not be formed at the extremes of the
/// representable calendar, which sends the caller down the ungated path — the
/// answer is the same either way, only the cost differs.
pub(super) fn trade_date_window(
    context: &QueryContext<'_>,
    open_day: NaiveDate,
    set: RuleSet,
    raw_open: DateTime<Utc>,
) -> Option<(NaiveDate, NaiveDate)> {
    if assigns_by_close_date(context) && is_self_dated(context, open_day, set, raw_open) {
        return Some((
            open_day,
            open_day.checked_add_signed(Duration::days(SELF_DATED_AFTER))?,
        ));
    }
    let (before, after) = match context.identity() {
        Some(CalendarSource::Exchange(Exchange::SetThailand)) => {
            (DERIVED_BEFORE + 1, DERIVED_AFTER)
        }
        Some(CalendarSource::MarketHoursKey(
            MarketHoursKey::GlobexCryptocurrency | MarketHoursKey::GlobexEventContractsBtc,
        )) => (DERIVED_BEFORE, DERIVED_AFTER + ROLLING_WINDOW_DAYS),
        _ => (DERIVED_BEFORE, DERIVED_AFTER),
    };
    let first = open_day.checked_sub_signed(Duration::days(before))?;
    let last = open_day.checked_add_signed(Duration::days(after))?;
    Some((first, last))
}

/// Returns whether [`assign_normal`] dates an occurrence by its close's own
/// local date.
///
/// The three sourced conventions move a trade date away from that default —
/// SET Thailand's night phase belongs to its prior local opening date, CBOT
/// Rough Rice's evening leg to the following local date, and the
/// cryptocurrency and `ECBTC` roll to the following open business date — so
/// those identities keep the walk's full window. The self-dated narrowing is a
/// statement about the close-date default only.
fn assigns_by_close_date(context: &QueryContext<'_>) -> bool {
    !matches!(
        context.identity(),
        Some(
            CalendarSource::Exchange(Exchange::SetThailand)
                | CalendarSource::MarketHoursKey(
                    MarketHoursKey::GlobexRoughRice
                        | MarketHoursKey::GlobexCryptocurrency
                        | MarketHoursKey::GlobexEventContractsBtc
                )
        )
    )
}

/// Returns whether this occurrence is dated by its own trading day.
///
/// A tradeable rule's own occurrence is what makes `raw_open` self-dated: the
/// rule is active on `open_day`, and the caller has already rejected
/// `raw_open >= raw_close`, so its close reaches past `raw_open` without a
/// lookup — which is why the session path, the one every `is_open`,
/// `session_state` and `session_bounds` query walks, pays nothing for the
/// narrowing.
///
/// An order-entry rule is not a session: it never joins the union the close
/// walk reads, so its occurrence is dated by the trading day its own day's
/// sessions belong to, and those have to be asked directly.
fn is_self_dated(
    context: &QueryContext<'_>,
    open_day: NaiveDate,
    set: RuleSet,
    raw_open: DateTime<Utc>,
) -> bool {
    matches!(set, RuleSet::Sessions(_)) || a_session_reaches(context, open_day, raw_open)
}

/// Returns whether a session active on `open_day` still closes after `raw_open`.
///
/// The test is asked of the profile selected for `open_day` — the same profile
/// [`find_occurrence`](super::schedule::find_occurrence) resolves the
/// occurrence from — and of each rule's *raw* close, which is exactly what the
/// walk's own baseline sees: the rules the caller's layers clip are still the
/// rules that end the trading day for the trade-date assignment, because that
/// assignment runs above the clip.
///
/// Deliberately not inlined: only an order-entry rule reaches it, and letting
/// it into [`trade_date_window`]'s own body moves the multi-candidate queries —
/// `trade_date`, `candle_end`, the closed-instant `is_open` scan — by tens of
/// percent, because every session occurrence resolves through the same
/// function. `benches/calendar_queries.rs` measures both.
#[inline(never)]
fn a_session_reaches(
    context: &QueryContext<'_>,
    open_day: NaiveDate,
    raw_open: DateTime<Utc>,
) -> bool {
    let tz = context.tz();
    let weekday = open_day.weekday().num_days_from_monday() as usize;
    let selected = context.profile_for_open_day(open_day);
    for rule in rules(selected.as_ref(), RuleSet::Sessions(SessionKind::Both)) {
        if !rule.days[weekday] {
            continue;
        }
        let close_day = if rule.wraps_to_next_day() {
            let Some(next) = open_day.succ_opt() else {
                continue;
            };
            next
        } else {
            open_day
        };
        if mk_local_close(tz, close_day, rule.close_ssm).with_timezone(&Utc) > raw_open {
            return true;
        }
    }
    false
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
