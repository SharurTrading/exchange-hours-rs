// SPDX-License-Identifier: MIT-0

//! The two concrete profile sources consumed by the query engine.

use chrono::{DateTime, Datelike, NaiveDate, Timelike, Utc};
use chrono_tz::Tz;

use crate::calendar::coverage::{CalendarCoverage, CoverageGapReason, DateCoverage};
use crate::calendar::exceptions::{DateException, SessionExceptionSource};
use crate::calendar::exchange_calendar::ExchangeCalendar;
use crate::calendar::hours::MarketHours;
use crate::calendar::local_time::{bounded_utc, mk_local_close, mk_local_open};
use crate::calendar::policy::DayPolicy;
use crate::calendar::rule::{SessionKind, SessionRule};
use crate::calendar::schedules::holidays::{Holiday, HolidayKind, HolidayTable};
use crate::calendar::{CalendarQueryError, CalendarResolution, CalendarSource, SUPPORT_FLOOR};

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
///
/// The built-in holiday table is resolved once here, not per rule and not per
/// candidate day: an identity's table is a `&'static` borrow, so carrying it is
/// one pointer and resolving it is one match over the identity.
#[derive(Clone, Copy)]
pub(in crate::calendar) struct QueryContext<'a> {
    source: ProfileSource<'a>,
    tz: Tz,
    holidays: Option<&'static HolidayTable>,
    policy: Option<&'a dyn DayPolicy>,
    exceptions: Option<&'a dyn SessionExceptionSource>,
    /// The identity's coverage metadata, or `None` for a detached snapshot.
    ///
    /// This is deliberately a field of its own rather than something derived
    /// from [`Self::holidays`]: [`Self::baseline`] drops the day-level layers
    /// to resolve the sourced normal week without re-entering them, and the
    /// coverage verdict is a fact about the whole identity that must survive
    /// that narrowing unchanged. Deriving it from the dropped table would make
    /// every overlay identity look table-less inside its own baseline walks.
    coverage: Option<CalendarCoverage>,
}

/// The scalar trade-date clip one layer contributes.
///
/// Every layer below the caller's replacement provider speaks this vocabulary,
/// so the built-in table and the caller's [`DayPolicy`] compose by the same
/// rule instead of each having its own precedence branch. Composition is
/// monotone-tightening (see [`Self::tighten`]).
#[derive(Clone, Copy)]
struct DayClip {
    closed: bool,
    /// A layer stated a boundary outside its documented range, which makes the
    /// trade date unavailable rather than clipping it. Only a caller's
    /// [`DayPolicy`] can set this; a built-in row's instants are fenced during
    /// constant evaluation.
    unavailable: bool,
    early_close_ssm: Option<u32>,
    late_open_ssm: Option<u32>,
}

impl DayClip {
    /// The clip that changes nothing.
    const NONE: Self = Self {
        closed: false,
        unavailable: false,
        early_close_ssm: None,
        late_open_ssm: None,
    };

    /// Returns whether this clip leaves the normal trading day untouched.
    const fn is_none(self) -> bool {
        !self.closed
            && !self.unavailable
            && self.early_close_ssm.is_none()
            && self.late_open_ssm.is_none()
    }

    /// Composes two clips by tightening, never by widening.
    ///
    /// A closure is an `OR`, an early close takes the `min` and a late open
    /// the `max`, so a caller's [`DayPolicy`] can always make a trading day
    /// shorter than the built-in table's answer and can never make it longer.
    fn tighten(self, other: Self) -> Self {
        Self {
            closed: self.closed || other.closed,
            unavailable: self.unavailable || other.unavailable,
            early_close_ssm: min_option(self.early_close_ssm, other.early_close_ssm),
            late_open_ssm: max_option(self.late_open_ssm, other.late_open_ssm),
        }
    }
}

fn min_option(left: Option<u32>, right: Option<u32>) -> Option<u32> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.min(right)),
        (value, None) | (None, value) => value,
    }
}

fn max_option(left: Option<u32>, right: Option<u32>) -> Option<u32> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.max(right)),
        (value, None) | (None, value) => value,
    }
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
    /// Builds a context over a detached fixed snapshot.
    ///
    /// A snapshot carries no identity — the crate never guesses a family from
    /// coincident rules — so no built-in holiday table attaches to it.
    pub(in crate::calendar) fn fixed(hours: &'a MarketHours) -> Self {
        Self {
            source: ProfileSource::Fixed(hours),
            tz: hours.tz,
            holidays: None,
            policy: None,
            exceptions: None,
            coverage: None,
        }
    }

    pub(in crate::calendar) fn date_aware(calendar: ExchangeCalendar) -> Self {
        Self {
            source: ProfileSource::DateAware(calendar),
            tz: calendar.tz(),
            holidays: calendar.holiday_table(),
            policy: None,
            exceptions: None,
            coverage: Some(calendar.coverage()),
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
            holidays: calendar.holiday_table(),
            policy,
            exceptions,
            coverage: Some(calendar.coverage()),
        }
    }

    /// Drops every day-level layer, leaving the sourced normal week.
    ///
    /// The overlay paths resolve a normal trading day first and then modify it,
    /// so they need a view of the profile that cannot re-enter themselves. The
    /// built-in table is dropped for exactly the same reason as the caller's
    /// two layers: it is a day-level modification of that normal week, and a
    /// baseline that kept it would recurse.
    ///
    /// The coverage metadata is **not** dropped: it describes the identity
    /// rather than a day-level layer, and the days a baseline walk touches are
    /// the same days the enclosing query already had to answer for.
    pub(super) const fn baseline(self) -> Self {
        Self {
            source: self.source,
            tz: self.tz,
            holidays: None,
            policy: None,
            exceptions: None,
            coverage: self.coverage,
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

    /// Returns whether any day-level layer is attached.
    ///
    /// A built-in family table counts: it is the innermost layer, it modifies
    /// the same trade-date boundaries the caller's layers do, and the
    /// following-business-day roll in
    /// [`identity::assign_normal`](super::identity) is gated on this predicate
    /// — so a shipped table that did not count here would leave the roll dead
    /// and delete a 24/7 family's trading on every holiday.
    pub(super) const fn has_overlay(self) -> bool {
        self.holidays.is_some() || self.policy.is_some() || self.exceptions.is_some()
    }

    /// Fails unless this identity answers `date` completely (LAW-COVERAGE).
    ///
    /// This is Stage 2B's single coverage seam: every identity-backed query
    /// resolves the venue-local days it depends on through this method, so the
    /// floor, an unsourced span and a withheld date are refused in one place
    /// instead of at each of the eighteen public entry points. A detached
    /// fixed snapshot answers `Ok(())` for every date, because it carries no
    /// identity and therefore claims nothing about coverage.
    ///
    /// The check is taken on the **dates the query needs**, never on the
    /// supplied instant alone: a caller asking what happens at 2025-01-01
    /// 00:30 local depends on the trading day that opened the previous
    /// evening, and a search that walks forward depends on every day it walks
    /// over (plan section 6).
    ///
    /// A declared **phase-level** gap does not refuse the date: it withholds a
    /// phase, not a day, so the day's normal week and holiday layer are still
    /// sourced and a query that never reads that phase has a real answer.
    /// Consulting the phase gap here would refuse a Tuesday afternoon for a
    /// Sunday queue — precisely the coverage-error-read-as-closure failure
    /// LAW-COVERAGE exists to prevent. The entry points that *do* probe the
    /// phase ask [`Self::require_phase_coverage`] instead.
    ///
    /// Cost stays on the built-in hot path's budget: one comparison against a
    /// floor constant, then the same bounded static-table walk the holiday
    /// layer already performs, and no allocation.
    pub(super) fn require_answerable(self, date: NaiveDate) -> Result<(), CalendarQueryError> {
        let Some(coverage) = self.coverage else {
            return Ok(());
        };
        if date < SUPPORT_FLOOR {
            return Ok(());
        }
        let verdict = match coverage.date_level_gap_on(date) {
            None => DateCoverage::Covered,
            Some(CoverageGapReason::WithheldDate) => DateCoverage::UnresolvedGap,
            Some(CoverageGapReason::NormalWeekOnly) => DateCoverage::NormalWeekOnly,
            Some(_) => DateCoverage::OutsideCoveredRange,
        };
        match verdict {
            DateCoverage::Covered => Ok(()),
            // A normal-week-only calendar still has a sourced-normal-week start
            // below which its weekday profile is carried rather than sourced,
            // so the one relaxation reaches exactly as far as that start and no
            // further. Reported rather than hidden: refusing here would let a
            // caller read a coverage error as a market closure.
            DateCoverage::NormalWeekOnly => {
                let sourced = coverage.sourced_normal_week();
                if date >= sourced.first() {
                    Ok(())
                } else {
                    Err(CalendarQueryError::OutsideCoveredRange {
                        source: coverage.identity(),
                        date,
                    })
                }
            }
            DateCoverage::BeforeSupportFloor => Err(CalendarQueryError::BeforeSupportFloor {
                source: coverage.identity(),
                date,
            }),
            DateCoverage::UnresolvedGap => Err(CalendarQueryError::UnresolvedGap {
                source: coverage.identity(),
                date,
            }),
            DateCoverage::OutsideCoveredRange => Err(CalendarQueryError::OutsideCoveredRange {
                source: coverage.identity(),
                date,
            }),
        }
    }

    /// Returns whether this query resolves through a real identity.
    ///
    /// A detached caller-supplied [`MarketHours`](crate::MarketHours) snapshot
    /// carries no identity and therefore claims nothing about coverage, which is
    /// why every gate early-returns for it. Callers that make a *claim* rather
    /// than a coverage check — withholding a pre-floor trade date, for instance —
    /// must ask this first, or they will weaken a detached snapshot's answers.
    pub(super) const fn is_identity_backed(self) -> bool {
        self.coverage.is_some()
    }

    /// Fails when the venue-local day containing `instant` precedes the floor.
    ///
    /// Every instant-addressed query starts here, so the floor is decided once
    /// from the caller's own instant rather than from each day a scan later
    /// walks over (see [`Self::require_floor`]).
    pub(in crate::calendar) fn require_floor_at(
        self,
        instant: DateTime<Utc>,
    ) -> Result<(), CalendarQueryError> {
        let local_day = bounded_utc(instant, self.tz)
            .with_timezone(&self.tz)
            .date_naive();
        self.require_floor(Some(local_day))
    }

    /// Fails when the caller's own instant or date precedes the support floor.
    ///
    /// The floor is a fact about the **queried** day, never about every day a
    /// scan walks over: the plan requires that a session opening before the
    /// floor is still returned whole to an in-range query, while a query
    /// addressed to an earlier instant errors (LAW-COVERAGE, plan section 6).
    /// `date` is the venue-local day the caller's instant belongs to, and
    /// `None` marks a detached snapshot, which carries no identity and claims
    /// no coverage.
    pub(super) fn require_floor(self, date: Option<NaiveDate>) -> Result<(), CalendarQueryError> {
        let Some(coverage) = self.coverage else {
            return Ok(());
        };
        let Some(date) = date else {
            return Ok(());
        };
        if date < SUPPORT_FLOOR {
            return Err(CalendarQueryError::BeforeSupportFloor {
                source: coverage.identity(),
                date,
            });
        }
        Ok(())
    }

    /// Fails unless this identity answers the **withheld phase** on `date`.
    ///
    /// This is the strict sibling of [`Self::require_coverage`], for the entry
    /// points whose answer *is* the arrangement a declared phase gap withholds:
    /// the order-entry queue scans. An identity that declares no phase gap is
    /// unaffected, so this costs one slice check on that path.
    pub(super) fn require_phase_coverage(self, date: NaiveDate) -> Result<(), CalendarQueryError> {
        let Some(coverage) = self.coverage else {
            return Ok(());
        };
        // The floor governs first. Below it the date is not "outside a covered
        // range" — no range has been claimed there at all — and reporting the
        // phase's verdict would both mask the floor and move the answer the day
        // #117 lands. `require_answerable` deliberately passes below the floor,
        // so the check has to be made here.
        if date < SUPPORT_FLOOR {
            return Err(CalendarQueryError::BeforeSupportFloor {
                source: coverage.identity(),
                date,
            });
        }
        // Otherwise the date-level verdict governs: a date the identity cannot
        // answer at all is refused for its own reason, not the phase's.
        self.require_answerable(date)?;
        match coverage.phase_gap_on(date) {
            None => Ok(()),
            Some(_gap) => Err(CalendarQueryError::OutsideCoveredRange {
                source: coverage.identity(),
                date,
            }),
        }
    }

    /// Returns the built-in row for `trade_date`, if the identity has a table.
    ///
    /// This reports the table alone; it is not the composed answer, which the
    /// caller's layers can tighten.
    pub(in crate::calendar) fn holiday_on(self, trade_date: NaiveDate) -> Option<Holiday> {
        self.holidays.and_then(|table| table.holiday_on(trade_date))
    }

    /// Returns the built-in clip for `trade_date`, if it is not suppressed.
    ///
    /// An explicit caller record wins outright (D12): a `Closed` or
    /// `ReplaceSessions` record from the caller's provider suppresses the
    /// built-in row for that date. `KnownNormal` deliberately does **not**
    /// suppress it — `StaticSessionExceptions` returns `KnownNormal` both for
    /// an audited-normal date and for a covered date with no record, so
    /// treating it as an assertion would silently disable the whole built-in
    /// table for every caller who attaches a provider. The per-date undo
    /// channel is `ReplaceSessions`; the coarse one is `without_holidays`.
    fn builtin_clip(self, trade_date: NaiveDate) -> DayClip {
        if matches!(
            self.exception_on(trade_date),
            DateException::Closed | DateException::ReplaceSessions(_)
        ) {
            return DayClip::NONE;
        }
        match self.holiday_on(trade_date).map(Holiday::kind) {
            None | Some(HolidayKind::Unsourced) => DayClip::NONE,
            Some(HolidayKind::Closed) => DayClip {
                closed: true,
                ..DayClip::NONE
            },
            Some(HolidayKind::EarlyClose { close_ssm }) => DayClip {
                early_close_ssm: Some(close_ssm),
                ..DayClip::NONE
            },
            Some(HolidayKind::LateOpen { open_ssm }) => DayClip {
                late_open_ssm: Some(open_ssm),
                ..DayClip::NONE
            },
            Some(HolidayKind::LateOpenAndEarlyClose {
                open_ssm,
                close_ssm,
            }) => DayClip {
                early_close_ssm: Some(close_ssm),
                late_open_ssm: Some(open_ssm),
                ..DayClip::NONE
            },
        }
    }

    /// Returns the caller's [`DayPolicy`] clip for `trade_date`.
    ///
    /// A boundary outside the trait's documented range makes the trade date
    /// unavailable, which is not the same as closing it: an invalid record is
    /// not evidence that the operator was shut, so it never feeds the
    /// following-business-day roll.
    fn policy_clip(self, trade_date: NaiveDate) -> DayClip {
        self.policy.map_or(DayClip::NONE, |policy| {
            let early_close_ssm = policy.early_close_ssm(trade_date);
            let late_open_ssm = policy.late_open_ssm(trade_date);
            DayClip {
                closed: policy.is_closed(trade_date),
                unavailable: early_close_ssm.is_some_and(|ssm| ssm > SECONDS_PER_DAY)
                    || late_open_ssm.is_some_and(|ssm| ssm >= SECONDS_PER_DAY),
                early_close_ssm,
                late_open_ssm,
            }
        })
    }

    /// Returns whether any attached layer can hold a record in `first..=last`.
    ///
    /// This is the coverage gate. It runs before any trading-day derivation, so
    /// a day no layer says anything about costs one binary search per attached
    /// table and nothing else — which is what lets a built-in table sit on the
    /// consumer's hot path (LAW-HOLIDAY-SCOPE).
    ///
    /// A caller's [`DayPolicy`] is opaque, so it always answers `true`; giving
    /// the trait a coverage question of its own is a named follow-up, not a
    /// wave-0 API addition. A caller's exception provider publishes a coverage
    /// window and is gated on it, except that a provider claiming **no**
    /// coverage is treated as possibly relevant rather than trusted to return
    /// nothing: the trait documents that contract but cannot enforce it, and a
    /// missed exception is worse than a missed optimisation.
    fn any_layer_may_affect(self, first: NaiveDate, last: NaiveDate) -> bool {
        if self.policy.is_some() {
            return true;
        }
        if let Some(provider) = self.exceptions {
            match provider.coverage() {
                None => return true,
                Some(coverage) => {
                    if coverage.first() <= last && first <= coverage.last() {
                        return true;
                    }
                }
            }
        }
        self.holidays
            .is_some_and(|table| table.may_affect(first, last))
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

    /// Returns whether any layer removes `trade_date` completely.
    ///
    /// The caller's exception layer answers first, the built-in family table
    /// next, and the caller's [`DayPolicy`] last, overlaying whatever survives
    /// exactly as it overlays a normal week.
    pub(super) fn trade_date_is_closed(self, trade_date: NaiveDate) -> bool {
        if matches!(self.exception_on(trade_date), DateException::Closed) {
            return true;
        }
        self.builtin_clip(trade_date)
            .tighten(self.policy_clip(trade_date))
            .closed
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
    pub(super) fn contains_order_entry(
        self,
        instant: DateTime<Utc>,
    ) -> Result<bool, CalendarQueryError> {
        let day = bounded_utc(instant, self.tz)
            .with_timezone(&self.tz)
            .date_naive();
        let hit = |open: DateTime<Utc>, close: DateTime<Utc>| {
            (open <= instant && instant < close).then_some(())
        };
        if find_occurrence(&self, day, RuleSet::OrderEntry, false, hit)?.is_some() {
            return Ok(true);
        }
        // A queue that opened yesterday can still be running, and the day it
        // opened on is the day this answer depends on — so the probe is gated
        // by the same rule as the one above rather than being read as a bare
        // civil-date fact.
        let Some(yesterday) = day.pred_opt() else {
            return Ok(false);
        };
        Ok(find_occurrence(&self, yesterday, RuleSet::OrderEntry, true, hit)?.is_some())
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
) -> Result<Option<T>, CalendarQueryError> {
    // The gate sits ahead of the profile selection, not inside it: resolving a
    // profile reads the identity's zone through a post-floor epoch snapshot,
    // and the plan requires that resolution never be reached on a date this
    // identity cannot answer (LAW-COVERAGE).
    context.require_answerable(open_day)?;
    // Only the order-entry scans read the phase a declared phase-level gap
    // withholds: a tradeable-session scan is answered by the sourced normal
    // week, so refusing it for a queue it never consults would report a
    // coverage error where the crate has a real answer.
    if matches!(set, RuleSet::OrderEntry) {
        context.require_phase_coverage(open_day)?;
    }
    let weekday = open_day.weekday().num_days_from_monday() as usize;
    let selected = context.profile_for_open_day(open_day);
    for rule in rules(selected.as_ref(), set)
        .filter(|rule| rule.days[weekday] && (!wrapped_only || rule.wraps_to_next_day()))
    {
        if let Some((open, close)) = resolve_rule_bounds(context, open_day, set, rule)?
            && let Some(found) = probe(open, close)
        {
            return Ok(Some(found));
        }
    }
    Ok(replacement::find_occurrence(
        context,
        open_day,
        set,
        wrapped_only,
        probe,
    ))
}

/// Resolves one scheduled occurrence and rejects civil-time collapses.
pub(super) fn resolve_rule_bounds(
    context: &QueryContext<'_>,
    open_day: NaiveDate,
    set: RuleSet,
    rule: &SessionRule,
) -> Result<Option<crate::calendar::exchange_calendar::SessionWindow>, CalendarQueryError> {
    let close_day = if rule.wraps_to_next_day() {
        open_day.succ_opt()
    } else {
        Some(open_day)
    };
    let Some(close_day) = close_day else {
        return Ok(None);
    };
    let tz = context.tz();
    let raw_open = mk_local_open(tz, open_day, rule.open_ssm).with_timezone(&Utc);
    let raw_close = mk_local_close(tz, close_day, rule.close_ssm).with_timezone(&Utc);
    if raw_open >= raw_close {
        return Ok(None);
    }
    if !context.has_overlay() {
        return Ok(Some((raw_open, raw_close)));
    }
    // The coverage gate, ahead of every other overlay check. Deriving the
    // trading day below costs a full daily window per rule; the set of trade
    // dates this occurrence can be assigned to is bounded by the identity's own
    // conventions, so ask every layer whether it holds a record in that window
    // before paying for any of it. When a session opening on this day still
    // reaches `raw_open` the occurrence is dated by its own trading day and the
    // window is `[D, D + 1]`; otherwise it is the close walk's own reach,
    // `[D - 1, D + 19]` — see
    // [`identity::trade_date_window`](super::identity::trade_date_window).
    // This sits above the daily-close guard because all three of these branches
    // return the same unmodified bounds, and the guard resolves a profile to
    // answer.
    if let Some((first, last)) = identity::trade_date_window(context, open_day, set, raw_open)
        && !context.any_layer_may_affect(first, last)
    {
        return Ok(Some((raw_open, raw_close)));
    }
    if !context.has_daily_close_at(raw_open) {
        // Without a final daily close this profile has no trade-date identity.
        // Applying an overlay to its storage-rule close would invent a date and
        // could close an always-open market one civil day early.
        return Ok(Some((raw_open, raw_close)));
    }

    let baseline = context.baseline();
    let final_close = match candles::candle_end_with(
        &baseline,
        raw_open,
        CalendarResolution::Daily,
        SessionKind::Both,
    )? {
        Some(resolved) => resolved,
        // No final daily close resolves for this occurrence, so keep the rule's
        // own close as the trade-date anchor rather than inventing a date.
        None => raw_close,
    };
    let trade_date = context.normal_trade_date_for_bounds(raw_open, final_close);
    // The caller's exception layer resolves the trading day before anything
    // clips it: a replaced or closed trade date deletes its normal-week
    // occurrences, and the caller's replacement blocks stand in their place.
    if context.trade_date_is_replaced(trade_date)
        || matches!(context.exception_on(trade_date), DateException::Closed)
    {
        return Ok(None);
    }
    let clip = context
        .builtin_clip(trade_date)
        .tighten(context.policy_clip(trade_date));
    if clip.closed {
        return Ok(None);
    }
    if clip.is_none() {
        return Ok(Some((raw_open, raw_close)));
    }
    clamp_to_clip(context, clip, trade_date, raw_open, raw_close)
}

/// Applies the composed trade-date clip to one resolved occurrence.
///
/// The clip is stated on the **trade date**, so an early close lands on the
/// correct civil day for a session that opened the previous evening, and an
/// occurrence that would begin after the cutoff disappears rather than
/// inverting.
fn clamp_to_clip(
    context: &QueryContext<'_>,
    clip: DayClip,
    trade_date: NaiveDate,
    raw_open: DateTime<Utc>,
    raw_close: DateTime<Utc>,
) -> Result<Option<crate::calendar::exchange_calendar::SessionWindow>, CalendarQueryError> {
    if clip.unavailable {
        return Ok(None);
    }
    let tz = context.tz();
    let mut open = raw_open;
    let mut close = raw_close;
    if let Some(ssm) = clip.early_close_ssm {
        let cutoff = mk_local_close(tz, trade_date, ssm).with_timezone(&Utc);
        close = close.min(cutoff);
    }
    if let Some(ssm) = clip.late_open_ssm {
        // The trading day's own first open decides which local date a late-open
        // wall clock belongs to, and it is needed only on this branch — late
        // opens are rare, so it is derived here rather than beside the final
        // close above.
        let first_open = candles::candle_start_with(
            &context.baseline(),
            raw_open,
            CalendarResolution::Daily,
            SessionKind::Both,
        )?
        .unwrap_or(raw_open);
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
    Ok((open < close).then_some((open, close)))
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
