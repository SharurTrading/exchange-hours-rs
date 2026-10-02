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
use crate::calendar::schedules::holidays::{Holiday, HolidayKind, HolidayTable};
use crate::calendar::{CalendarResolution, CalendarSource};

use super::gate::{FixedSnapshot, Identified, SourceGate};
use super::{candles, identity, replacement};

// Sessions opening on a civil day are governed by the profile in force at the
// end of that opening day. Midnight-keyed revisions select the same profile
// at any post-midnight anchor, so this only distinguishes sourced intraday
// cutovers: one that lands in an intraday gap after noon (ICE Canada's 18:30
// CT pre-open move) must govern the sessions opening later that day. The last
// second of the local day exists in every zone — DST transitions never
// collapse or duplicate 23:59:59 — and `mk_local_open` resolves earliest on
// ambiguity regardless.
const SECONDS_PER_DAY: u32 = 86_400;

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
///
/// The context is parameterised over [`SourceGate`], the compile-time split of
/// issue #245: a [`FixedSnapshot`] context carries no coverage at all and
/// cannot raise an error, while an [`Identified`] context always carries its
/// coverage metadata and reports every refusal through
/// [`CalendarQueryError`](crate::CalendarQueryError). The constructors below
/// are the only ways to build one, so the pairing of a source with the
/// coverage it claims is a property of the types, not of review.
#[derive(Clone, Copy)]
pub(in crate::calendar) struct QueryContext<'a, G: SourceGate> {
    source: G::Source<'a>,
    tz: Tz,
    holidays: Option<&'static HolidayTable>,
    policy: Option<&'a dyn DayPolicy>,
    exceptions: Option<&'a dyn SessionExceptionSource>,
    /// The coverage metadata this source state carries — the [`Identified`]
    /// state's [`CalendarCoverage`](crate::CalendarCoverage), or nothing at all
    /// for a [`FixedSnapshot`].
    ///
    /// There is no `None` state to reach: the gate types make "a context that
    /// can carry coverage but does not" unrepresentable, which is the
    /// invariant the former `Option<CalendarCoverage>` field could only
    /// document. See [`SourceGate`] for the full mechanism.
    ///
    /// This is deliberately a field of its own rather than something derived
    /// from [`Self::holidays`]: [`Self::baseline`] drops the day-level layers
    /// to resolve the sourced normal week without re-entering them, and the
    /// coverage verdict is a fact about the whole source state that must
    /// survive that narrowing unchanged.
    coverage: G::Coverage,
    /// Whether some attached layer can supply a replacement trading day.
    ///
    /// Resolved once here rather than asked per scan. The caller's provider is
    /// cheap to test, but a built-in table's answer lives behind its pointer, so
    /// asking per occurrence would make the replacement gate dereference a
    /// static table on the normal-week hot path — for a bit that cannot change
    /// while the context lives.
    replacement_layer: bool,
    /// Whether the identity's own table carries a replacement block row.
    ///
    /// Separate from [`Self::replacement_layer`] because it answers a different
    /// question: that one decides whether the replacement *scan* runs, this one
    /// whether a lookup of the built-in table can contribute a replacement at
    /// all. A context with a caller's provider and a table of scalar rows has
    /// `replacement_layer == true` and `builtin_blocks == false`, and must not
    /// pay a `holiday_on` binary search per occurrence to discover that its
    /// table has nothing to add. Every table the crate ships answers `false`.
    builtin_blocks: bool,
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

/// Returns whether a built-in table can supply a replacement trading day.
///
/// The answer itself was decided during constant evaluation, so this is one
/// field read behind the table pointer. It exists as a function only so the
/// context constructors can cache it: asking it once per scan would put that
/// pointer chase on the normal-week hot path.
const fn table_can_replace(holidays: Option<&HolidayTable>) -> bool {
    match holidays {
        Some(table) => table.carries_replacement_blocks(),
        None => false,
    }
}

pub(in crate::calendar) enum ResolvedHours<'a> {
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

impl<'a> QueryContext<'a, FixedSnapshot> {
    /// Builds a context over a detached fixed snapshot.
    ///
    /// A snapshot carries no identity — the crate never guesses a family from
    /// coincident rules — so no built-in holiday table attaches to it and no
    /// coverage exists to claim: the context cannot raise an error
    /// ([`FixedSnapshot`], issue #245).
    pub(in crate::calendar) fn fixed(hours: &'a MarketHours) -> Self {
        Self {
            source: hours,
            tz: hours.tz,
            holidays: None,
            policy: None,
            exceptions: None,
            coverage: (),
            replacement_layer: false,
            builtin_blocks: false,
        }
    }
}

impl<'a> QueryContext<'a, Identified> {
    pub(in crate::calendar) fn date_aware(calendar: ExchangeCalendar) -> Self {
        let holidays = calendar.holiday_table();
        let builtin_blocks = table_can_replace(holidays);
        Self {
            source: calendar,
            tz: calendar.tz(),
            holidays,
            policy: None,
            exceptions: None,
            coverage: calendar.coverage(),
            replacement_layer: builtin_blocks,
            builtin_blocks,
        }
    }

    pub(in crate::calendar) fn overlay(
        calendar: ExchangeCalendar,
        policy: Option<&'a dyn DayPolicy>,
        exceptions: Option<&'a dyn SessionExceptionSource>,
    ) -> Self {
        let holidays = calendar.holiday_table();
        let builtin_blocks = table_can_replace(holidays);
        Self {
            source: calendar,
            tz: calendar.tz(),
            holidays,
            policy,
            exceptions,
            coverage: calendar.coverage(),
            replacement_layer: exceptions.is_some() || builtin_blocks,
            builtin_blocks,
        }
    }
}

impl<'a, G: SourceGate> QueryContext<'a, G> {
    /// Drops every day-level layer, leaving the sourced normal week.
    ///
    /// The overlay paths resolve a normal trading day first and then modify it,
    /// so they need a view of the profile that cannot re-enter themselves. The
    /// built-in table is dropped for exactly the same reason as the caller's
    /// two layers: it is a day-level modification of that normal week, and a
    /// baseline that kept it would recurse.
    ///
    /// The coverage metadata is **not** dropped: it describes the source state
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
            // The baseline drops every layer that could supply a replacement, so
            // the gate must drop with them or a baseline walk would re-enter the
            // layer it exists to avoid.
            replacement_layer: false,
            builtin_blocks: false,
        }
    }

    pub(super) const fn tz(self) -> Tz {
        self.tz
    }

    pub(super) const fn policy(self) -> Option<&'a dyn DayPolicy> {
        self.policy
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

    /// The date-level coverage gate ([`SourceGate::require_answerable`],
    /// LAW-COVERAGE): every identity-backed query resolves the venue-local
    /// days it depends on through it, while a [`FixedSnapshot`] context's
    /// gate is total and cannot refuse.
    pub(super) fn require_answerable(self, date: NaiveDate) -> Result<(), G::Error> {
        G::require_answerable(self.coverage, date)
    }

    /// The instant floor gate ([`SourceGate::require_floor_at`]): the floor is
    /// decided from the caller's own instant's venue-local day.
    pub(in crate::calendar) fn require_floor_at(
        self,
        instant: DateTime<Utc>,
    ) -> Result<(), G::Error> {
        G::require_floor_at(self.coverage, self.tz, instant)
    }

    /// The day floor gate ([`SourceGate::require_floor`]): a fact about the
    /// **queried** day, never about every day a scan walks over.
    pub(super) fn require_floor(self, date: NaiveDate) -> Result<(), G::Error> {
        G::require_floor(self.coverage, date)
    }

    /// The withheld-phase gate ([`SourceGate::require_phase_coverage`]) for
    /// the entry points whose answer *is* the arrangement a declared phase
    /// gap withholds: the order-entry queue scans.
    pub(super) fn require_phase_coverage(self, date: NaiveDate) -> Result<(), G::Error> {
        G::require_phase_coverage(self.coverage, date)
    }

    /// Whether the coverage metadata declares an unscoped refusing phase gap
    /// on `day` — the cheap half of the order-entry scan's gate.
    pub(super) fn has_unscoped_refusing_phase_gap_on(self, day: NaiveDate) -> bool {
        G::has_unscoped_refusing_phase_gap_on(self.coverage, day)
    }

    /// The bounded-search-exhaustion verdict
    /// ([`SourceGate::search_exhausted`]): an identity-backed source
    /// attributes the exhaustion to itself, a detached snapshot keeps its
    /// exhaustive `None` absence.
    pub(super) fn search_exhausted(self, day: NaiveDate) -> Result<(), G::Error> {
        G::search_exhausted(self.coverage, day)
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
    ///
    /// A built-in replacement-block row contributes **no** scalar clip: that
    /// row's trade date is governed by the replacement layer, and clipping it
    /// here as well would apply one row to its own date twice.
    fn builtin_clip(self, trade_date: NaiveDate) -> DayClip {
        if matches!(
            self.caller_exception_on(trade_date),
            DateException::Closed | DateException::ReplaceSessions(_)
        ) {
            return DayClip::NONE;
        }
        match self.holiday_on(trade_date).map(Holiday::kind) {
            // Three kinds contribute no scalar clip, for two different reasons.
            // `None` is a date the table says nothing about and `Unsourced` is
            // one it expressly withholds; a replacement row states its whole
            // arrangement through the replacement layer, so clipping the normal
            // week here as well would apply one row to its own date twice.
            None | Some(HolidayKind::Unsourced | HolidayKind::ReplacementBlocks(_)) => {
                DayClip::NONE
            }
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

    /// Returns whether the built-in table may hold a replacement block row on
    /// `day`.
    ///
    /// This is the built-in half of [`Self::any_layer_may_affect`] alone, and
    /// it is what the order-entry scan gate consults: a caller's [`DayPolicy`]
    /// only clips scalar boundaries and a caller's exception provider can only
    /// remove an occurrence or supply its own blocks — caller data, never the
    /// identity's withheld arrangement — so neither can turn a queue probe
    /// into a consultation of the withheld phase. Counting them here would
    /// flip an answerable wrapped-queue probe into a coverage error whenever
    /// any layer is attached, which is exactly the overlay-neutrality the
    /// overlay contracts promise not to break.
    fn builtin_block_may_exist_on(self, day: NaiveDate) -> bool {
        self.builtin_blocks
            && self
                .holidays
                .is_some_and(|table| table.may_affect(day, day))
    }

    /// Returns whether any layer's replacement blocks can reach the inclusive
    /// local-day span `first..=last` — meaning a record keyed so that one of
    /// its blocks can open on a day inside it.
    ///
    /// This is the cheap pre-filter in front of the two block walks
    /// ([`replacement::governs_instant`](super::replacement::governs_instant)
    /// and [`replacement::find_occurrence`](super::replacement::find_occurrence)),
    /// which otherwise probe record tables once per candidate trade date on
    /// every resolved occurrence. It answers **false only when no block can
    /// exist**: a `false` here always means the walks would find nothing, so
    /// skipping them cannot move an answer (LAW-INVARIANT). A caller's
    /// exception provider answers from its own
    /// [`may_affect`](crate::SessionExceptionSource::may_affect) (issue #127),
    /// so an audited-normal provider no longer keeps the full walk alive; an
    /// implementation that does not track its records defaults to `true` and
    /// keeps today's walk. The built-in table, whose rows the crate ships and
    /// fences, is asked over its block rows alone.
    pub(super) fn replacement_blocks_may_reach(self, first: NaiveDate, last: NaiveDate) -> bool {
        if !self.replacement_layer {
            return false;
        }
        if let Some(provider) = self.exceptions
            && provider.may_affect(first, last)
        {
            return true;
        }
        self.builtin_blocks
            && self
                .holidays
                .is_some_and(|table| table.blocks_may_affect(first, last))
    }
    /// Returns whether any attached layer can hold a record in `first..=last`.
    ///
    /// This is the coverage gate. It runs before any trading-day derivation, so
    /// a day no layer says anything about costs one binary search per attached
    /// table and nothing else — which is what lets a built-in table sit on the
    /// consumer's hot path (LAW-HOLIDAY-SCOPE).
    ///
    /// A caller's [`DayPolicy`] answers from its own
    /// [`may_affect`](crate::DayPolicy::may_affect), which defaults to `true`
    /// for an implementation that does not track its span and is one binary
    /// search for [`StaticDayPolicy`](crate::StaticDayPolicy) (issue #94). A
    /// caller's exception provider is gated on its coverage window **and** on
    /// its own [`may_affect`](crate::SessionExceptionSource::may_affect)
    /// (issue #127): a window only states where records may lie, so a
    /// provider whose records sit elsewhere — or that audited a window and
    /// found nothing — must no more force the derivation than an unattached
    /// layer. A provider claiming **no** coverage is still treated as
    /// possibly relevant rather than trusted to return nothing: the trait
    /// documents that contract but cannot enforce it, and a missed exception
    /// is worse than a missed optimisation.
    fn any_layer_may_affect(self, first: NaiveDate, last: NaiveDate) -> bool {
        if self
            .policy
            .is_some_and(|policy| policy.may_affect(first, last))
        {
            return true;
        }
        if let Some(provider) = self.exceptions {
            match provider.coverage() {
                None => return true,
                Some(coverage) => {
                    if coverage.first() <= last
                        && first <= coverage.last()
                        && provider.may_affect(first, last)
                    {
                        return true;
                    }
                }
            }
        }
        self.holidays
            .is_some_and(|table| table.may_affect(first, last))
    }

    /// Returns the schedule identity, or `None` for a detached fixed snapshot.
    pub(super) fn identity(self) -> Option<CalendarSource> {
        G::identity(self.source)
    }

    /// Returns what the caller's own exception provider knows about `trade_date`.
    ///
    /// This is the caller's layer alone, with the built-in table deliberately
    /// excluded: [`Self::builtin_clip`] asks it whether the caller suppressed
    /// the built-in row, and the composed answer would report a built-in row's
    /// own blocks as if the caller had supplied them.
    fn caller_exception_on(self, trade_date: NaiveDate) -> DateException<'a> {
        self.exceptions
            .map_or(DateException::KnownNormal, |provider| {
                provider.exception_on(trade_date)
            })
    }

    /// Returns the replacement arrangement in force for `trade_date`.
    ///
    /// Precedence is fixed and one-directional. An explicit caller record wins
    /// outright (D12), so a caller `Closed` or `ReplaceSessions` is returned
    /// unchanged and the built-in table is not consulted. Otherwise a built-in
    /// row carrying a replacement block set is served through the same
    /// [`DateException::ReplaceSessions`] arm a caller's record uses, which is
    /// what makes one resolver — `query::replacement` — answer for both layers,
    /// so every query family observes the same trading day.
    ///
    /// A caller's `KnownNormal` and `OutOfCoverage` both fall through to the
    /// built-in row, exactly as they fall through to the built-in scalar clip:
    /// the crate's own table is sourced evidence about the date whether or not
    /// the caller's provider holds an opinion about it.
    ///
    /// The table is consulted only when it actually carries a block row
    /// ([`Self::builtin_blocks`]). Every table the crate ships answers `false`,
    /// so this returns the caller's answer after one virtual call and no
    /// lookup — which matters because the callers of this method sit in the
    /// per-occurrence path. Without that guard a bare calendar would pay a
    /// `holiday_on` binary search per occurrence for a row that cannot exist.
    ///
    /// The `match` below lists every [`HolidayKind`] rather than falling through
    /// on a wildcard, so a sixth kind that needs built-in handling here is a
    /// build failure instead of a silent fall-through to the caller's answer.
    pub(super) fn exception_on(self, trade_date: NaiveDate) -> DateException<'a> {
        let caller = self.caller_exception_on(trade_date);
        if !self.builtin_blocks
            || matches!(
                caller,
                DateException::Closed | DateException::ReplaceSessions(_)
            )
        {
            return caller;
        }
        match self.holiday_on(trade_date).map(Holiday::kind) {
            Some(HolidayKind::ReplacementBlocks(set)) => DateException::ReplaceSessions(set),
            None
            | Some(
                HolidayKind::Closed
                | HolidayKind::EarlyClose { .. }
                | HolidayKind::LateOpen { .. }
                | HolidayKind::LateOpenAndEarlyClose { .. }
                | HolidayKind::Unsourced,
            ) => caller,
        }
    }

    /// Returns whether any attached layer can supply a replacement trading day.
    ///
    /// This is the gate in front of the replacement scan. It is deliberately a
    /// one-bit question asked of a field: the scan walks the fixed block-offset
    /// window and asks for a record on each day of it, and the normal-week paths
    /// that decide whether to run the scan resolve one occurrence at a time, so
    /// deriving the answer here would put a table dereference in that loop for a
    /// bit that cannot change while the context lives.
    ///
    /// Once Stage 4 (#116) ships a block row this becomes true for that
    /// identity, and the scan is exactly the work the row requires. Until then
    /// it is `false` for every shipped table, so the new kind costs the hot path
    /// this one field read.
    pub(super) const fn has_replacement_layer(self) -> bool {
        self.replacement_layer
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
        G::has_daily_close_at(self.source, instant)
    }

    pub(super) fn has_weekend_close_at(self, instant: DateTime<Utc>) -> bool {
        G::has_weekend_close_at(self.source, instant)
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
    pub(super) fn contains_order_entry(self, instant: DateTime<Utc>) -> Result<bool, G::Error> {
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
        G::profile_for_open_day(self.source, self.tz, day)
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
/// contributes its own blocks. No instant is claimed by both layers.
/// [`resolve_rule_bounds`] drops every normal occurrence whose trade date the
/// exception layer replaced or closed, and the check below drops an occurrence
/// of a *different* trade date whose window a replacement block of the same
/// rule set overlaps — otherwise the normal scan would answer first with a
/// session that [`QueryContext::trade_date_for_bounds`] does not assign to that
/// block's trade date, which is issue #130.
pub(super) fn find_occurrence<G: SourceGate, T>(
    context: &QueryContext<'_, G>,
    open_day: NaiveDate,
    set: RuleSet,
    wrapped_only: bool,
    mut probe: impl FnMut(DateTime<Utc>, DateTime<Utc>) -> Option<T>,
) -> Result<Option<T>, G::Error> {
    // The date-level gate sits ahead of the profile selection, not inside it:
    // resolving a profile reads the identity's zone through a post-floor epoch
    // snapshot, and the plan requires that resolution never be reached on a date
    // this identity cannot answer (LAW-COVERAGE).
    context.require_answerable(open_day)?;
    let weekday = open_day.weekday().num_days_from_monday() as usize;
    let selected = context.profile_for_open_day(open_day);
    // Only the order-entry scans read the phase a declared phase-level gap
    // withholds: a tradeable-session scan is answered by the sourced normal
    // week, so refusing it for a queue it never consults would report a
    // coverage error where the crate has a real answer. And even a queue scan
    // is gated only where its own day could consult the withheld arrangement:
    // an `EveryDay` declaration whose reason refuses fires on every scan of
    // its era, because an omitted phase leaves no rule for the scan to match
    // and the question is live whether or not one does; a date-scoped
    // declaration fires only where its own day could yield an occurrence of
    // the set — a normal rule of the set matching the day, or a replacement
    // layer that may hold a record on it. A wrapped-Sunday probe against a
    // grid whose Sunday queue never wraps, or any day whose grid carries no
    // matching rule and no nearby block row, answers `None` from the sourced
    // tables without consulting the withheld phase, and a coverage error
    // there would refuse an answer the identity has.
    if matches!(set, RuleSet::OrderEntry) && {
        context.has_unscoped_refusing_phase_gap_on(open_day)
            || rules(selected.as_ref(), set)
                .any(|rule| rule.days[weekday] && (!wrapped_only || rule.wraps_to_next_day()))
            || context.builtin_block_may_exist_on(open_day)
    } {
        context.require_phase_coverage(open_day)?;
    }
    for rule in rules(selected.as_ref(), set)
        .filter(|rule| rule.days[weekday] && (!wrapped_only || rule.wraps_to_next_day()))
    {
        if let Some(bounds) = resolve_rule_bounds(context, open_day, set, rule)? {
            // A block this occurrence meets takes its place: the plan's "a
            // replaced trade date serves only its own blocks" applies to a
            // neighbouring trade date's occurrence too, so the normal scan
            // yields to the replacement scan below instead of answering with a
            // session `trade_date_for_bounds` does not assign to that block
            // (#130). The occurrence's own opening day and wrap bound the
            // local days its window can touch — a clip only ever shortens it —
            // so the reach test needs no fresh resolution of the bounds, and a
            // day no block is keyed near skips the walk entirely (issue #125).
            if replacement::governs_instant(
                context,
                bounds,
                set,
                open_day,
                rule.wraps_to_next_day(),
            ) {
                continue;
            }
            if let Some(found) = probe(bounds.0, bounds.1) {
                return Ok(Some(found));
            }
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

/// Returns whether the identity's **built-in** calendar resolves an
/// order-entry occurrence of `open_ssm..close_ssm` opening on `date`.
///
/// This is the shape check behind
/// [`CalendarCoverage::phase_gap_on`](crate::CalendarCoverage::phase_gap_on):
/// a declared gap whose shape names an order-entry window applies exactly to
/// the dates this answers `true` for. The walk is the identity's own — its
/// profile timeline for the opening day, its built-in holiday table, and the
/// same occurrence resolution every queue scan runs — never a caller's
/// overlay, because the declaration is a fact about the identity and the same
/// value must be reported by every view of it. The window pre-filter runs
/// before any resolution, so a date whose weekday or grid carries no matching
/// rule costs one profile selection and a rule scan; a failure to resolve
/// withholds, which is the conservative direction.
pub(in crate::calendar) fn builtin_resolves_order_entry(
    source: CalendarSource,
    date: NaiveDate,
    open_ssm: Option<u32>,
    close_ssm: u32,
) -> bool {
    let calendar = match source {
        CalendarSource::Exchange(exchange) => {
            crate::calendar::exchange_calendar::calendar_for_exchange(exchange)
        }
        CalendarSource::MarketHoursKey(key) => {
            crate::calendar::exchange_calendar::calendar_for_market_hours_key(key)
        }
    };
    let context = QueryContext::date_aware(calendar);
    let weekday = date.weekday().num_days_from_monday() as usize;
    let selected = context.profile_for_open_day(date);
    let mut unresolved = false;
    for rule in rules(selected.as_ref(), RuleSet::OrderEntry) {
        if !rule.days[weekday] || rule.close_ssm != close_ssm {
            continue;
        }
        if open_ssm.is_some_and(|open| rule.open_ssm != open) {
            continue;
        }
        match resolve_rule_bounds(&context, date, RuleSet::OrderEntry, rule) {
            Ok(Some(_bounds)) => return true,
            Ok(None) => {}
            Err(_) => unresolved = true,
        }
    }
    unresolved
}

/// Returns whether the identity's built-in timeline gives `day`'s weekday any
/// **tradeable** rule (regular or extended).
///
/// This is the coverage walk's "session day" test: a date whose profile grid
/// carries a tradeable rule is a day a session scan can find an occurrence on,
/// whatever the holiday layer later does to that occurrence. It is the same
/// profile selection every query runs
/// ([`Self::profile_for_open_day`]), so the metadata and the query surface
/// cannot disagree about which days carry rules.
pub(in crate::calendar) fn builtin_has_tradeable_rule(
    source: CalendarSource,
    day: NaiveDate,
) -> bool {
    let calendar = match source {
        CalendarSource::Exchange(exchange) => {
            crate::calendar::exchange_calendar::calendar_for_exchange(exchange)
        }
        CalendarSource::MarketHoursKey(key) => {
            crate::calendar::exchange_calendar::calendar_for_market_hours_key(key)
        }
    };
    let context = QueryContext::date_aware(calendar);
    let weekday = day.weekday().num_days_from_monday() as usize;
    let selected = context.profile_for_open_day(day);
    rules(selected.as_ref(), RuleSet::Sessions(SessionKind::Both)).any(|rule| rule.days[weekday])
}

/// Resolves one scheduled occurrence and rejects civil-time collapses.
pub(super) fn resolve_rule_bounds<G: SourceGate>(
    context: &QueryContext<'_, G>,
    open_day: NaiveDate,
    set: RuleSet,
    rule: &SessionRule,
) -> Result<Option<crate::calendar::exchange_calendar::SessionWindow>, G::Error> {
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
    // reaches `raw_open` the occurrence is dated by its own trading day: a
    // rule closing on its own local day is dated by that day alone, `[D, D]`,
    // and a wrapping one by `[D, D + 1]`; otherwise the window is the close
    // walk's own reach, `[D - 1, D + 19]` — see
    // [`identity::trade_date_window`](super::identity::trade_date_window).
    // This sits above the daily-close guard because all three of these branches
    // return the same unmodified bounds, and the guard resolves a profile to
    // answer.
    if let Some((first, last)) =
        identity::trade_date_window(context, open_day, set, rule.wraps_to_next_day(), raw_open)
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
    // The exception layer resolves the trading day before anything clips it: a
    // replaced or closed trade date deletes its normal-week occurrences, and
    // the replacement blocks -- the caller's record or the built-in table's
    // row, whichever governs the date -- stand in their place.
    //
    // The two probes run only when some layer can actually replace or close a
    // date by record. `has_replacement_layer` false means neither a caller
    // provider nor a block row is attached, and then `exception_on` can only
    // answer `KnownNormal`, so both tests are already false. Skipping them keeps
    // two virtual calls out of the per-occurrence path, which is where this
    // function is called from.
    if context.has_replacement_layer()
        && (context.trade_date_is_replaced(trade_date)
            || matches!(context.exception_on(trade_date), DateException::Closed))
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
fn clamp_to_clip<G: SourceGate>(
    context: &QueryContext<'_, G>,
    clip: DayClip,
    trade_date: NaiveDate,
    raw_open: DateTime<Utc>,
    raw_close: DateTime<Utc>,
) -> Result<Option<crate::calendar::exchange_calendar::SessionWindow>, G::Error> {
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
        // close above. The walk runs on `baseline()`, which carries no layer
        // and holds the rule this occurrence resolves from, so `None` would
        // mean the first open cannot be derived at the representable calendar's
        // edge; anchoring at this occurrence's own resolved open then keeps the
        // cutoff on the opening day it was derived from. It is a real value in
        // the same derivation, never an invented one.
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
