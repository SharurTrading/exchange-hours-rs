// SPDX-License-Identifier: MIT-0

//! The two concrete profile sources consumed by the query engine.

use chrono::{DateTime, Datelike, NaiveDate, Timelike, Utc};
use chrono_tz::Tz;

use crate::calendar::exceptions::{DateException, SessionExceptionSource};
use crate::calendar::exchange_calendar::ExchangeCalendar;
use crate::calendar::hours::MarketHours;
use crate::calendar::local_time::{bounded_utc, mk_local_close, mk_local_open};
use crate::calendar::policy::DayPolicy;
use crate::calendar::rule::{SessionKind, SessionRule};
use crate::calendar::{CalendarResolution, CalendarSource};

use super::{candles, identity, replacement};

// Sessions opening on a civil day are governed by the profile in force at the
// end of that opening day. Midnight-keyed revisions select the same profile
// at any post-midnight anchor, so this only distinguishes sourced intraday
// cutovers: one that lands in an intraday gap after noon (ICE Canada's 18:30
// CT pre-open move) must govern the sessions opening later that day. The last
// second of the local day exists in every zone — DST transitions never
// collapse or duplicate 23:59:59 — and `mk_local_open` resolves earliest on
// ambiguity regardless.
const OPEN_DAY_ANCHOR_SSM: u32 = 86_399;
const SECONDS_PER_DAY: u32 = 86_400;

#[derive(Clone, Copy)]
enum ProfileSource<'a> {
    Fixed(&'a MarketHours),
    DateAware(ExchangeCalendar),
}

/// Which of a profile's rule sets a scan consults.
///
/// `order_entry` is deliberately not a [`SessionKind`]: it is not tradeable, so
/// it can never join a session union. Keeping the two apart in one enum lets
/// every occurrence scan — sessions and queues alike — share one code path,
/// including the caller-supplied replacement layer.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum RuleSet {
    /// Tradeable rules selected by `kind`.
    Sessions(SessionKind),
    /// Order-entry-only rules.
    OrderEntry,
}

/// A per-query schedule source with its invariant venue zone cached once.
#[derive(Clone, Copy)]
pub(in crate::calendar) struct QueryContext<'a> {
    source: ProfileSource<'a>,
    tz: Tz,
    policy: Option<&'a dyn DayPolicy>,
    exceptions: Option<&'a dyn SessionExceptionSource>,
}

pub(super) enum ResolvedHours<'a> {
    Borrowed(&'a MarketHours),
    Selected(MarketHours),
}

impl AsRef<MarketHours> for ResolvedHours<'_> {
    fn as_ref(&self) -> &MarketHours {
        match self {
            Self::Borrowed(hours) => hours,
            Self::Selected(hours) => hours,
        }
    }
}

impl<'a> QueryContext<'a> {
    pub(in crate::calendar) fn fixed(hours: &'a MarketHours) -> Self {
        Self {
            source: ProfileSource::Fixed(hours),
            tz: hours.tz,
            policy: None,
            exceptions: None,
        }
    }

    pub(in crate::calendar) fn date_aware(calendar: ExchangeCalendar) -> Self {
        Self {
            source: ProfileSource::DateAware(calendar),
            tz: calendar.tz(),
            policy: None,
            exceptions: None,
        }
    }

    pub(in crate::calendar) fn overlay(
        calendar: ExchangeCalendar,
        policy: Option<&'a dyn DayPolicy>,
        exceptions: Option<&'a dyn SessionExceptionSource>,
    ) -> Self {
        Self {
            source: ProfileSource::DateAware(calendar),
            tz: calendar.tz(),
            policy,
            exceptions,
        }
    }

    /// Drops both caller overlays, leaving the sourced normal week.
    ///
    /// The overlay paths resolve a normal trading day first and then modify it,
    /// so they need a view of the profile that cannot re-enter themselves.
    pub(super) const fn baseline(self) -> Self {
        Self {
            source: self.source,
            tz: self.tz,
            policy: None,
            exceptions: None,
        }
    }

    pub(super) const fn tz(self) -> Tz {
        self.tz
    }

    pub(super) const fn policy(self) -> Option<&'a dyn DayPolicy> {
        self.policy
    }

    pub(super) const fn exceptions(self) -> Option<&'a dyn SessionExceptionSource> {
        self.exceptions
    }

    /// Returns whether either caller-owned overlay is attached.
    pub(super) const fn has_overlay(self) -> bool {
        self.policy.is_some() || self.exceptions.is_some()
    }

    /// Returns the schedule identity, or `None` for a detached fixed snapshot.
    pub(super) const fn identity(self) -> Option<CalendarSource> {
        match self.source {
            ProfileSource::Fixed(_) => None,
            ProfileSource::DateAware(calendar) => Some(calendar.source()),
        }
    }

    /// Returns what the caller's exception provider knows about `trade_date`.
    pub(super) fn exception_on(self, trade_date: NaiveDate) -> DateException<'a> {
        self.exceptions
            .map_or(DateException::KnownNormal, |provider| {
                provider.exception_on(trade_date)
            })
    }

    /// Returns whether the exception layer replaces `trade_date` outright.
    pub(super) fn trade_date_is_replaced(self, trade_date: NaiveDate) -> bool {
        matches!(
            self.exception_on(trade_date),
            DateException::ReplaceSessions(_)
        )
    }

    /// Returns whether either overlay removes `trade_date` completely.
    ///
    /// The exception layer answers first; the caller's [`DayPolicy`] then
    /// overlays it exactly as it overlays a normal week.
    pub(super) fn trade_date_is_closed(self, trade_date: NaiveDate) -> bool {
        matches!(self.exception_on(trade_date), DateException::Closed)
            || self
                .policy
                .is_some_and(|policy| policy.is_closed(trade_date))
    }

    /// Assigns a resolved session block to its venue-local trade date.
    ///
    /// A replacement block carries its assignment explicitly, so it wins over
    /// every derived convention. Everything else falls through to
    /// [`Self::normal_trade_date_for_bounds`].
    pub(super) fn trade_date_for_bounds(
        self,
        open: DateTime<Utc>,
        close: DateTime<Utc>,
    ) -> NaiveDate {
        replacement::replacement_trade_date(&self, open)
            .unwrap_or_else(|| identity::assign_normal(&self, open, close))
    }

    /// Assigns bounds produced by a normal-week rule to their trade date.
    ///
    /// Most profiles use the local date of the final close. Identified
    /// calendars retain three sourced exceptions: SET's after-midnight DR night
    /// phase belongs to its prior local opening date, CBOT Rough Rice's
    /// evening leg belongs to the following local date, and CME
    /// cryptocurrency's weekend blocks carry the following business date. A
    /// detached fixed snapshot has no identity with which to apply any of
    /// them.
    pub(super) fn normal_trade_date_for_bounds(
        self,
        open: DateTime<Utc>,
        close: DateTime<Utc>,
    ) -> NaiveDate {
        identity::assign_normal(&self, open, close)
    }

    /// Returns whether this identified calendar joins storage-only rule pieces.
    ///
    /// This is intentionally an identity capability, not a shape heuristic:
    /// adjacent rules are real phase boundaries for several other profiles.
    pub(super) fn joins_adjacent_same_kind(self) -> bool {
        identity::joins_adjacent_same_kind(&self)
    }

    pub(super) fn has_daily_close_at(self, instant: DateTime<Utc>) -> bool {
        match self.source {
            ProfileSource::Fixed(hours) => hours.has_daily_close,
            ProfileSource::DateAware(calendar) => calendar.hours_at(instant).has_daily_close,
        }
    }

    pub(super) fn has_weekend_close_at(self, instant: DateTime<Utc>) -> bool {
        match self.source {
            ProfileSource::Fixed(hours) => hours.has_weekend_close,
            ProfileSource::DateAware(calendar) => calendar.hours_at(instant).has_weekend_close,
        }
    }

    /// True when `instant` falls in an order-entry-only phase occurrence.
    ///
    /// Resolves through the same opening-day-keyed selection as every session
    /// query: today's occurrences from today's profile and wrapped occurrences
    /// from yesterday's profile, so a phase is always answered by the profile
    /// that owns its opening day even when a revision takes effect on the
    /// following civil date. Both caller overlays apply as they do to tradeable
    /// sessions: a closed trade date removes the complete trading day including
    /// the queue that feeds it, and a replaced trade date serves only the
    /// order-entry blocks the caller supplied for it.
    pub(super) fn contains_order_entry(self, instant: DateTime<Utc>) -> bool {
        let day = bounded_utc(instant, self.tz)
            .with_timezone(&self.tz)
            .date_naive();
        let hit = |open: DateTime<Utc>, close: DateTime<Utc>| {
            (open <= instant && instant < close).then_some(())
        };
        if find_occurrence(&self, day, RuleSet::OrderEntry, false, hit).is_some() {
            return true;
        }
        let Some(yesterday) = day.pred_opt() else {
            return false;
        };
        find_occurrence(&self, yesterday, RuleSet::OrderEntry, true, hit).is_some()
    }

    /// Returns whether this source exposes a real weekly candle boundary.
    ///
    /// Most profiles use their explicit weekend-close flag. CME's key-backed
    /// cryptocurrency calendar is the sourced exception: its continuous week
    /// has no long weekend shutdown, but Friday 16:00 CT remains the final
    /// close of that trade-date week before Monday Pre-Open starts at 16:01.
    /// The identity-erased fixed snapshot cannot apply that convention.
    pub(super) fn has_weekly_close_at(self, instant: DateTime<Utc>) -> bool {
        if self.has_weekend_close_at(instant) {
            return true;
        }
        identity::joins_adjacent_same_kind(&self) && self.has_daily_close_at(instant)
    }

    /// Selects a profile by the venue-local day on which a session opens.
    pub(super) fn profile_for_open_day(self, day: NaiveDate) -> ResolvedHours<'a> {
        match self.source {
            ProfileSource::Fixed(hours) => ResolvedHours::Borrowed(hours),
            ProfileSource::DateAware(calendar) => {
                let anchor = mk_local_open(self.tz, day, OPEN_DAY_ANCHOR_SSM).with_timezone(&Utc);
                ResolvedHours::Selected(calendar.hours_at(anchor))
            }
        }
    }
}

/// Visits every effective occurrence opening on `open_day`, newest layer last.
///
/// `probe` receives resolved `(open, close)` bounds and returns `Some` to stop
/// the scan with that value, so one helper serves both "find the containing
/// occurrence" and "fold over all of them". `wrapped_only` restricts the scan
/// to occurrences that close on the following local day, which is what a
/// containment query needs when it looks back one opening day.
///
/// Normal-week occurrences come first; a caller-supplied replacement day then
/// contributes its own blocks. The two never overlap for one trade date:
/// [`resolve_rule_bounds`] drops every normal occurrence whose trade date the
/// exception layer replaced or closed.
pub(super) fn find_occurrence<T>(
    context: &QueryContext<'_>,
    open_day: NaiveDate,
    set: RuleSet,
    wrapped_only: bool,
    mut probe: impl FnMut(DateTime<Utc>, DateTime<Utc>) -> Option<T>,
) -> Option<T> {
    let weekday = open_day.weekday().num_days_from_monday() as usize;
    let selected = context.profile_for_open_day(open_day);
    for rule in rules(selected.as_ref(), set)
        .filter(|rule| rule.days[weekday] && (!wrapped_only || rule.wraps_to_next_day()))
    {
        if let Some((open, close)) = resolve_rule_bounds(context, open_day, rule)
            && let Some(found) = probe(open, close)
        {
            return Some(found);
        }
    }
    replacement::find_occurrence(context, open_day, set, wrapped_only, probe)
}

/// Resolves one scheduled occurrence and rejects civil-time collapses.
pub(super) fn resolve_rule_bounds(
    context: &QueryContext<'_>,
    open_day: NaiveDate,
    rule: &SessionRule,
) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    let close_day = if rule.wraps_to_next_day() {
        open_day.succ_opt()?
    } else {
        open_day
    };
    let tz = context.tz();
    let raw_open = mk_local_open(tz, open_day, rule.open_ssm).with_timezone(&Utc);
    let raw_close = mk_local_close(tz, close_day, rule.close_ssm).with_timezone(&Utc);
    if raw_open >= raw_close {
        return None;
    }
    if !context.has_overlay() {
        return Some((raw_open, raw_close));
    }
    if !context.has_daily_close_at(raw_open) {
        // Without a final daily close this profile has no trade-date identity.
        // Applying an overlay to its storage-rule close would invent a date and
        // could close an always-open market one civil day early.
        return Some((raw_open, raw_close));
    }

    let baseline = context.baseline();
    let final_close = candles::candle_end_with(
        &baseline,
        raw_open,
        CalendarResolution::Daily,
        SessionKind::Both,
    )
    .unwrap_or(raw_close);
    let first_open = candles::candle_start_with(
        &baseline,
        raw_open,
        CalendarResolution::Daily,
        SessionKind::Both,
    )
    .unwrap_or(raw_open);
    let trade_date = context.normal_trade_date_for_bounds(raw_open, final_close);
    // The exception layer resolves the trading day before the policy clips it:
    // a replaced or closed trade date deletes its normal-week occurrences, and
    // the caller's replacement blocks stand in their place.
    if context.trade_date_is_replaced(trade_date) || context.trade_date_is_closed(trade_date) {
        return None;
    }
    let Some(policy) = context.policy else {
        return Some((raw_open, raw_close));
    };
    clamp_to_policy(context, policy, trade_date, first_open, raw_open, raw_close)
}

fn clamp_to_policy(
    context: &QueryContext<'_>,
    policy: &dyn DayPolicy,
    trade_date: NaiveDate,
    first_open: DateTime<Utc>,
    raw_open: DateTime<Utc>,
    raw_close: DateTime<Utc>,
) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    let tz = context.tz();
    let mut open = raw_open;
    let mut close = raw_close;
    if let Some(ssm) = policy.early_close_ssm(trade_date) {
        if ssm > SECONDS_PER_DAY {
            return None;
        }
        let cutoff = mk_local_close(tz, trade_date, ssm).with_timezone(&Utc);
        close = close.min(cutoff);
    }
    if let Some(ssm) = policy.late_open_ssm(trade_date) {
        if ssm >= SECONDS_PER_DAY {
            return None;
        }
        let first_local = first_open.with_timezone(&tz);
        let first_day = first_local.date_naive();
        let first_ssm = first_local.time().num_seconds_from_midnight();
        let cutoff_day = if first_day < trade_date && ssm >= first_ssm {
            first_day
        } else {
            trade_date
        };
        let cutoff = mk_local_open(tz, cutoff_day, ssm).with_timezone(&Utc);
        open = open.max(cutoff);
    }
    (open < close).then_some((open, close))
}

pub(super) fn rules(hours: &MarketHours, set: RuleSet) -> impl Iterator<Item = &SessionRule> {
    let (first, second): (&[SessionRule], &[SessionRule]) = match set {
        RuleSet::Sessions(SessionKind::Regular) => (&hours.regular, &[]),
        RuleSet::Sessions(SessionKind::Extended) => (&[], &hours.extended),
        RuleSet::Sessions(SessionKind::Both) => (&hours.regular, &hours.extended),
        RuleSet::OrderEntry => (&hours.order_entry, &[]),
    };
    first.iter().chain(second)
}
