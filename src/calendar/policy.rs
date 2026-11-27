// SPDX-License-Identifier: MIT-0

//! Caller-supplied trade-date overrides and their calendar adapter.

mod candles;
mod outcome;
mod static_policy;
mod week;

pub use outcome::{DayAdjustment, DayAdjustmentError, DayPolicyOutcome};
pub use static_policy::{DayOverride, StaticDayPolicy, StaticDayPolicyError};

use jiff::tz::TimeZone;
use jiff::{Timestamp, civil::Date};

use super::exceptions::{DateException, ExceptionScopeError, SessionExceptionSource};
use super::exchange_calendar::SessionWindow;
use super::query::gate::Identified;
use super::query::{QueryContext, sessions, status};
use super::schedules::holidays::{Holiday, HolidayCoverage};
use super::{
    CalendarQueryError, CalendarSource, Exchange, MarketHours, MarketHoursKey, SessionKind,
    SessionState, ZonedExchangeCalendar,
};

/// Day-level schedule overrides supplied by a caller.
///
/// Dates are venue-local **trade dates**, normally the local date of the
/// trading session's final close. For an ordinary wrapped Globex session,
/// closing Monday therefore removes the session that opened Sunday evening.
/// A sourced family convention can assign trading to the next open business
/// date instead: CME cryptocurrency weekend trading remains open and rolls to
/// Tuesday when Monday is closed. Implementations must be deterministic and
/// should perform no I/O or clock reads. The crate's own per-family holiday
/// tables are the innermost layer below this overlay, which only ever tightens
/// what they answer. A profile without a final daily close has no trade date,
/// so its queries ignore this overlay rather than inventing one.
/// Callers with hard-coded records can use [`StaticDayPolicy`] instead of
/// implementing this trait themselves. This boundary API cannot replace or
/// split arbitrary intraday phases; a special day with different internal
/// topology goes through [`SessionExceptionSource`], which this overlay then
/// clips exactly as it clips a normal week.
pub trait DayPolicy: Send + Sync {
    /// Returns the policy's validated outcome for `trade_date`.
    ///
    /// A closure normally removes the whole trading day. Families with a
    /// sourced following-business-day convention can reassign continuous
    /// trading to the next date without a closure. Boundary adjustments only
    /// clip sessions assigned to the date.
    ///
    /// Validate raw input with [`DayAdjustment`] before storing or returning
    /// it. The checked constructors report [`DayAdjustmentError`] to the code
    /// accepting the input.
    fn outcome_on(&self, trade_date: Date) -> DayPolicyOutcome;

    /// Returns whether this policy holds any record that can land on a trade
    /// date in the inclusive window `first..=last`.
    ///
    /// This is the coverage question the query engine's gate asks before it
    /// derives a trading day: the overlay can only change an occurrence's
    /// answer by holding a record for a trade date that occurrence could be
    /// assigned to, so a policy answering `false` for the whole window lets
    /// the query finish at the bare calendar's cost. The default answer is
    /// `true` — *possibly*, which keeps the derivation for implementations
    /// that do not track their own span.
    ///
    /// Returning `false` for a window that in fact holds a record is a
    /// soundness bug in the implementation, not a tunable: the engine will
    /// skip the derivation and serve the unmodified normal week for an
    /// occurrence the record should have moved. [`StaticDayPolicy`] answers
    /// from one binary search over its validated, sorted records.
    fn may_affect(&self, first: Date, last: Date) -> bool {
        let _ = (first, last);
        true
    }
}

/// A policy that leaves every normal-week session unchanged.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoPolicy;

impl DayPolicy for NoPolicy {
    fn outcome_on(&self, _trade_date: Date) -> DayPolicyOutcome {
        DayPolicyOutcome::Normal
    }

    /// No record anywhere: the policy can never affect an answer, so the
    /// engine's gate may always skip its derivation.
    fn may_affect(&self, _first: Date, _last: Date) -> bool {
        false
    }
}

impl core::fmt::Debug for ZonedPolicyCalendar<'_, '_> {
    /// Reports the identity and which overlays are attached.
    ///
    /// Neither overlay is a `Debug` value — both are caller-supplied trait
    /// objects — so this states their presence rather than their contents.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PolicyCalendar")
            .field("source", &self.calendar.source())
            .field("day_policy", &self.policy.is_some())
            .field("session_exceptions", &self.exceptions.is_some())
            .finish()
    }
}

/// A date-aware calendar with the caller's own day overlays applied.
///
/// Two independent overlays can be attached, and both are optional. A
/// [`DayPolicy`] clips a trading day's outer boundaries; a
/// [`SessionExceptionSource`] replaces a trade date outright with a complete
/// ordered block set. Precedence is fixed and one-directional: the exception
/// layer resolves the trading day, then the policy overlays it exactly as it
/// overlays a normal week. Two replacement layers never compose — attaching a
/// provider replaces any provider already attached.
///
/// The wrapper borrows both overlays and remains allocation-free itself. Work
/// or allocation performed inside the caller's callbacks is outside the
/// calendar's performance guarantees. [`Self::hours_at`] deliberately returns
/// the unmodified sourced profile; the overlays apply to queries, not tables.
#[derive(Clone, Copy)]
pub struct ZonedPolicyCalendar<'a, 'z> {
    calendar: ZonedExchangeCalendar<'z>,
    policy: Option<&'a dyn DayPolicy>,
    exceptions: Option<&'a dyn SessionExceptionSource>,
}

/// An overlaid calendar using the precompiled IANA rules.
pub type PolicyCalendar<'a> = ZonedPolicyCalendar<'a, 'static>;

impl<'a, 'z> ZonedPolicyCalendar<'a, 'z> {
    pub(super) const fn new(
        calendar: ZonedExchangeCalendar<'z>,
        policy: Option<&'a dyn DayPolicy>,
        exceptions: Option<&'a dyn SessionExceptionSource>,
    ) -> Self {
        Self {
            calendar,
            policy,
            exceptions,
        }
    }

    fn context<'q>(self) -> QueryContext<'q, Identified>
    where
        'a: 'q,
        'z: 'q,
    {
        QueryContext::overlay(self.calendar, self.policy, self.exceptions)
    }

    /// Returns whether a [`DayPolicy`] is attached.
    #[must_use]
    pub const fn has_day_policy(self) -> bool {
        self.policy.is_some()
    }

    /// Returns whether a [`SessionExceptionSource`] is attached.
    #[must_use]
    pub const fn has_session_exceptions(self) -> bool {
        self.exceptions.is_some()
    }

    /// Applies caller-supplied trade-date boundary overrides to every query.
    ///
    /// Replaces any policy already attached.
    #[must_use]
    pub const fn with_day_policy(self, policy: &'a dyn DayPolicy) -> Self {
        Self::new(self.calendar, Some(policy), self.exceptions)
    }

    /// Applies a caller-supplied replacement-session provider to every query.
    ///
    /// Replaces any provider already attached: two replacement layers never
    /// compose.
    ///
    /// # Errors
    ///
    /// Returns [`ExceptionScopeError`] when `exceptions` is scoped to a
    /// different [`CalendarSource`] than this calendar represents.
    pub fn with_session_exceptions(
        self,
        exceptions: &'a dyn SessionExceptionSource,
    ) -> Result<Self, ExceptionScopeError> {
        let provider = exceptions.source();
        if provider == self.calendar.source() {
            Ok(Self::new(self.calendar, self.policy, Some(exceptions)))
        } else {
            Err(ExceptionScopeError {
                calendar: self.calendar.source(),
                provider,
            })
        }
    }

    /// Returns what the attached provider knows about `trade_date`.
    ///
    /// Returns `None` when no provider is attached. This is the only way to
    /// tell an audited-and-normal date from one the provider never covered:
    /// runtime queries necessarily serve the normal week for both.
    #[must_use]
    pub fn session_exception_on(self, trade_date: Date) -> Option<DateException<'a>> {
        self.exceptions
            .map(|provider| provider.exception_on(trade_date))
    }

    /// Returns the underlying date-aware calendar.
    #[must_use]
    pub const fn calendar(self) -> ZonedExchangeCalendar<'z> {
        self.calendar
    }

    /// Detaches the identity's built-in holiday table, keeping both overlays.
    ///
    /// See [`crate::ExchangeCalendar::without_holidays`].
    #[must_use]
    pub const fn without_holidays(self) -> Self {
        Self::new(
            self.calendar.without_holidays(),
            self.policy,
            self.exceptions,
        )
    }

    /// Returns the **built-in** holiday row for venue-local `trade_date`.
    ///
    /// This reports the crate's own table only, never the composed answer: the
    /// caller's own layers are introspected through
    /// [`Self::session_exception_on`] and through the [`DayPolicy`] the caller
    /// already holds. See [`crate::ExchangeCalendar::holiday_on`] for what `None`
    /// means.
    #[must_use]
    pub fn holiday_on(self, trade_date: Date) -> Option<Holiday> {
        self.calendar.holiday_on(trade_date)
    }

    /// Returns the **built-in** table's audited trade-date window, or `None`
    /// when this identity has no table.
    ///
    /// This is the crate's own coverage claim; a caller's exception provider
    /// publishes its own through
    /// [`SessionExceptionSource::coverage`].
    #[must_use]
    pub fn holiday_coverage(self) -> Option<HolidayCoverage> {
        self.calendar.holiday_coverage()
    }

    /// Returns the schedule identity represented by this calendar.
    #[must_use]
    pub const fn source(self) -> CalendarSource {
        self.calendar.source()
    }

    /// Returns the represented exchange, or `None` for a product-family calendar.
    #[must_use]
    pub const fn exchange(self) -> Option<Exchange> {
        self.calendar.exchange()
    }

    /// Returns the represented family key, or `None` for an exchange calendar.
    #[must_use]
    pub const fn market_hours_key(self) -> Option<MarketHoursKey> {
        self.calendar.market_hours_key()
    }

    /// Resolves the unmodified fixed profile in force at `instant`.
    #[must_use]
    pub fn hours_at(self, instant: Timestamp) -> MarketHours {
        self.calendar.hours_at(instant)
    }

    /// Returns the schedule's invariant IANA time zone.
    #[must_use]
    pub fn tz(self) -> TimeZone {
        self.calendar.tz()
    }

    /// Returns whether any effective regular or extended session is open.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CalendarQueryErrorKind::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`crate::CalendarQueryErrorKind::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`crate::CalendarQueryErrorKind::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_open(self, instant: Timestamp) -> Result<bool, CalendarQueryError> {
        self.is_open_with(instant, SessionKind::Both)
    }

    /// Returns whether the effective selected session kind is open.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CalendarQueryErrorKind::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`crate::CalendarQueryErrorKind::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`crate::CalendarQueryErrorKind::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_open_with(
        self,
        instant: Timestamp,
        kind: SessionKind,
    ) -> Result<bool, CalendarQueryError> {
        status::is_open_with(&self.context(), instant, kind)
    }

    /// Returns whether an effective regular session is open.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CalendarQueryErrorKind::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`crate::CalendarQueryErrorKind::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`crate::CalendarQueryErrorKind::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_open_regular(self, instant: Timestamp) -> Result<bool, CalendarQueryError> {
        self.is_open_with(instant, SessionKind::Regular)
    }

    /// Returns whether an effective extended session is open.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CalendarQueryErrorKind::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`crate::CalendarQueryErrorKind::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`crate::CalendarQueryErrorKind::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_open_extended(self, instant: Timestamp) -> Result<bool, CalendarQueryError> {
        self.is_open_with(instant, SessionKind::Extended)
    }

    /// Returns whether orders may be entered, amended or cancelled at `instant`.
    ///
    /// True during effective order-entry-only phases and during any effective
    /// tradeable session. A closed trade date removes the queue that feeds it;
    /// a replaced trade date serves only the order-entry blocks the caller
    /// supplied for it.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CalendarQueryErrorKind::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`crate::CalendarQueryErrorKind::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`crate::CalendarQueryErrorKind::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_accepting_orders(self, instant: Timestamp) -> Result<bool, CalendarQueryError> {
        status::is_accepting_orders(&self.context(), instant)
    }

    /// Returns whether `instant` falls in an effective order-entry-only phase.
    ///
    /// Mutually exclusive with [`Self::is_open`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::CalendarQueryErrorKind::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`crate::CalendarQueryErrorKind::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`crate::CalendarQueryErrorKind::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_order_entry_only(self, instant: Timestamp) -> Result<bool, CalendarQueryError> {
        status::is_order_entry_only(&self.context(), instant)
    }

    /// Returns the containing or next effective session bounds.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CalendarQueryErrorKind::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`crate::CalendarQueryErrorKind::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`crate::CalendarQueryErrorKind::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    ///
    /// This scan does **not** return
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`]: exhausting its own 14-day horizon
    /// is reported as `Ok(None)`, because the horizon is the query's documented
    /// bound rather than a day it failed to establish. Only the period walks
    /// (`candle_end`, `candle_start` and `trade_date`) report `SearchExhausted`,
    /// where a close must be found for the answer to exist.
    pub fn session_bounds(
        self,
        instant: Timestamp,
    ) -> Result<Option<SessionWindow>, CalendarQueryError> {
        self.session_bounds_with(instant, SessionKind::Both)
    }

    /// Returns the containing or next effective bounds for `kind`.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CalendarQueryErrorKind::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`crate::CalendarQueryErrorKind::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`crate::CalendarQueryErrorKind::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    ///
    /// This scan does **not** return
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`]: exhausting its own 14-day horizon
    /// is reported as `Ok(None)`, because the horizon is the query's documented
    /// bound rather than a day it failed to establish. Only the period walks
    /// (`candle_end`, `candle_start` and `trade_date`) report `SearchExhausted`,
    /// where a close must be found for the answer to exist.
    pub fn session_bounds_with(
        self,
        instant: Timestamp,
        kind: SessionKind,
    ) -> Result<Option<SessionWindow>, CalendarQueryError> {
        sessions::session_bounds_with(&self.context(), instant, kind)
    }

    /// Returns the first effective regular/extended session after `instant`.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CalendarQueryErrorKind::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`crate::CalendarQueryErrorKind::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`crate::CalendarQueryErrorKind::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    ///
    /// This scan does **not** return
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`]: exhausting its own 14-day horizon
    /// is reported as `Ok(None)`, because the horizon is the query's documented
    /// bound rather than a day it failed to establish. Only the period walks
    /// (`candle_end`, `candle_start` and `trade_date`) report `SearchExhausted`,
    /// where a close must be found for the answer to exist.
    pub fn next_session_after(
        self,
        instant: Timestamp,
    ) -> Result<Option<SessionWindow>, CalendarQueryError> {
        self.next_session_after_with(instant, SessionKind::Both)
    }

    /// Returns the first effective session of `kind` after `instant`.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CalendarQueryErrorKind::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`crate::CalendarQueryErrorKind::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`crate::CalendarQueryErrorKind::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    ///
    /// This scan does **not** return
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`]: exhausting its own 14-day horizon
    /// is reported as `Ok(None)`, because the horizon is the query's documented
    /// bound rather than a day it failed to establish. Only the period walks
    /// (`candle_end`, `candle_start` and `trade_date`) report `SearchExhausted`,
    /// where a close must be found for the answer to exist.
    pub fn next_session_after_with(
        self,
        instant: Timestamp,
        kind: SessionKind,
    ) -> Result<Option<SessionWindow>, CalendarQueryError> {
        sessions::next_session_after_with(&self.context(), instant, kind)
    }

    /// Returns only the next effective regular-or-extended session open.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CalendarQueryErrorKind::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`crate::CalendarQueryErrorKind::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`crate::CalendarQueryErrorKind::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn next_session_open_after(
        self,
        instant: Timestamp,
    ) -> Result<Option<Timestamp>, CalendarQueryError> {
        Ok(self.next_session_after(instant)?.map(|(open, _close)| open))
    }

    /// Returns whether `instant` is in a documented short maintenance gap.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CalendarQueryErrorKind::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`crate::CalendarQueryErrorKind::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`crate::CalendarQueryErrorKind::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_maintenance(self, instant: Timestamp) -> Result<bool, CalendarQueryError> {
        status::is_maintenance(&self.context(), instant)
    }

    /// Returns the effective policy-aware state at `instant`.
    ///
    /// Maintenance normally separates trade dates within the four-hour bound;
    /// a continuously traded-week profile can retain an operator-designated
    /// short maintenance gap inside one trade date.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CalendarQueryErrorKind::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`crate::CalendarQueryErrorKind::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`crate::CalendarQueryErrorKind::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn session_state(self, instant: Timestamp) -> Result<SessionState, CalendarQueryError> {
        status::session_state(&self.context(), instant)
    }

    /// Returns the effective local trade date containing `instant`.
    ///
    /// An identified following-business-day schedule skips policy-closed dates;
    /// CME cryptocurrency weekend trading therefore returns Tuesday when its
    /// usual Monday trade date is closed.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CalendarQueryErrorKind::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`crate::CalendarQueryErrorKind::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`crate::CalendarQueryErrorKind::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn trade_date(self, instant: Timestamp) -> Result<Option<Date>, CalendarQueryError> {
        status::trade_date(&self.context(), instant)
    }

    /// Returns whether no effective session of `kind` is assigned to `day`.
    ///
    /// Profiles without a trade-date concept return `true` because no session
    /// can be assigned to the requested trade date. Use the civil-day queries
    /// when asking whether trading intersects a calendar date instead.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CalendarQueryErrorKind::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`crate::CalendarQueryErrorKind::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`crate::CalendarQueryErrorKind::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_closed_trade_date(
        self,
        day: Date,
        kind: SessionKind,
    ) -> Result<bool, CalendarQueryError> {
        status::is_closed_trade_date(&self.context(), day, kind)
    }

    /// Returns whether no effective session intersects `day` in `calendar_tz`.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CalendarQueryErrorKind::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`crate::CalendarQueryErrorKind::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`crate::CalendarQueryErrorKind::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_closed_all_day_in_calendar(
        self,
        day: Date,
        calendar_tz: &TimeZone,
        kind: SessionKind,
    ) -> Result<bool, CalendarQueryError> {
        status::is_closed_all_day_in_calendar(&self.context(), day, calendar_tz, kind)
    }

    /// Returns whether no effective session intersects venue-local `day`.
    ///
    /// This remains a calendar-date overlap query. Closing Monday's trade date
    /// does not make Monday wholly closed when Tuesday's session opens Monday
    /// evening; use [`Self::is_closed_trade_date`] for that question.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CalendarQueryErrorKind::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`crate::CalendarQueryErrorKind::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`crate::CalendarQueryErrorKind::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_closed_all_day_on(
        self,
        day: Date,
        kind: SessionKind,
    ) -> Result<bool, CalendarQueryError> {
        self.is_closed_all_day_in_calendar(day, self.calendar.zone(), kind)
    }

    /// Returns whether no effective session intersects the selected calendar day.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CalendarQueryErrorKind::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`crate::CalendarQueryErrorKind::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`crate::CalendarQueryErrorKind::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`crate::CalendarQueryErrorKind::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_closed_all_day_at(
        self,
        instant: Timestamp,
        calendar_tz: &TimeZone,
        kind: SessionKind,
    ) -> Result<bool, CalendarQueryError> {
        status::is_closed_all_day_at(&self.context(), instant, calendar_tz, kind)
    }
}

impl<'z> ZonedExchangeCalendar<'z> {
    /// Applies caller-supplied trade-date boundary overrides to every query.
    #[must_use]
    pub const fn with_day_policy(self, policy: &dyn DayPolicy) -> ZonedPolicyCalendar<'_, 'z> {
        ZonedPolicyCalendar::new(self, Some(policy), None)
    }

    /// Applies a caller-supplied replacement-session provider to every query.
    ///
    /// The provider replaces whole trade dates with complete ordered block
    /// sets, which scalar [`DayPolicy`] boundaries cannot express. Attach a
    /// policy as well with
    /// [`PolicyCalendar::with_day_policy`](PolicyCalendar::with_day_policy);
    /// the policy then overlays the replacement exactly as it overlays a
    /// normal week.
    ///
    /// # Errors
    ///
    /// Returns [`ExceptionScopeError`] when `exceptions` is scoped to a
    /// different [`CalendarSource`] than this calendar represents. One venue's
    /// holiday topology is never evidence about another's, so the mismatch is
    /// refused rather than applied.
    pub fn with_session_exceptions(
        self,
        exceptions: &dyn SessionExceptionSource,
    ) -> Result<ZonedPolicyCalendar<'_, 'z>, ExceptionScopeError> {
        let provider = exceptions.source();
        if provider == self.source() {
            Ok(ZonedPolicyCalendar::new(self, None, Some(exceptions)))
        } else {
            Err(ExceptionScopeError {
                calendar: self.source(),
                provider,
            })
        }
    }
}
