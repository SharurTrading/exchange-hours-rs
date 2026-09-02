// SPDX-License-Identifier: MIT-0

//! Caller-supplied trade-date overrides and their calendar adapter.

mod candles;
mod static_policy;
mod week;

pub use static_policy::{DayOverride, StaticDayPolicy, StaticDayPolicyError};

use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Tz;

use super::exceptions::{DateException, ExceptionScopeError, SessionExceptionSource};
use super::query::{QueryContext, sessions, status};
use super::{
    CalendarSource, Exchange, ExchangeCalendar, MarketHours, MarketHoursKey, SessionKind,
    SessionState,
};

/// Day-level schedule overrides supplied by a caller.
///
/// Dates are venue-local **trade dates**, normally the local date of the
/// trading session's final close. For an ordinary wrapped Globex session,
/// closing Monday therefore removes the session that opened Sunday evening.
/// A sourced family convention can assign trading to the next open business
/// date instead: CME cryptocurrency weekend trading remains open and rolls to
/// Tuesday when Monday is closed. Implementations must be deterministic and
/// should perform no I/O or clock reads. The crate ships no holiday data. A
/// profile without a final daily close has no trade date, so its queries ignore
/// this overlay rather than inventing one.
/// Callers with hard-coded records can use [`StaticDayPolicy`] instead of
/// implementing this trait themselves. This boundary API cannot replace or
/// split arbitrary intraday phases; a special day with different internal
/// topology goes through [`SessionExceptionSource`], which this overlay then
/// clips exactly as it clips a normal week.
pub trait DayPolicy: Send + Sync {
    /// Returns whether the market does not trade on `trade_date`.
    ///
    /// This normally removes that complete trading day. An identified family
    /// with a sourced following-business-day convention may instead reassign
    /// its continuous trading to the next date for which this returns `false`.
    fn is_closed(&self, trade_date: NaiveDate) -> bool;

    /// Moves the trade date's final close to venue-local seconds since midnight.
    ///
    /// Later phases are removed. `None` keeps the normal close. Values greater
    /// than `86_400` are invalid and make that trade date unavailable.
    fn early_close_ssm(&self, trade_date: NaiveDate) -> Option<u32>;

    /// Delays the trade date's first open to venue-local seconds since midnight.
    ///
    /// `None` keeps the normal open. For a wrapped trading day, a value at or
    /// after the normal first-open wall clock is interpreted on the preceding
    /// opening date; smaller values are interpreted on the trade date. Values
    /// at or above `86_400` are invalid and make that trade date unavailable.
    fn late_open_ssm(&self, _trade_date: NaiveDate) -> Option<u32> {
        None
    }
}

/// A policy that leaves every normal-week session unchanged.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoPolicy;

impl DayPolicy for NoPolicy {
    fn is_closed(&self, _trade_date: NaiveDate) -> bool {
        false
    }

    fn early_close_ssm(&self, _trade_date: NaiveDate) -> Option<u32> {
        None
    }
}

impl core::fmt::Debug for PolicyCalendar<'_> {
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
pub struct PolicyCalendar<'a> {
    calendar: ExchangeCalendar,
    policy: Option<&'a dyn DayPolicy>,
    exceptions: Option<&'a dyn SessionExceptionSource>,
}

impl<'a> PolicyCalendar<'a> {
    pub(super) const fn new(
        calendar: ExchangeCalendar,
        policy: Option<&'a dyn DayPolicy>,
        exceptions: Option<&'a dyn SessionExceptionSource>,
    ) -> Self {
        Self {
            calendar,
            policy,
            exceptions,
        }
    }

    fn context(self) -> QueryContext<'a> {
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
    pub fn session_exception_on(self, trade_date: NaiveDate) -> Option<DateException<'a>> {
        self.exceptions
            .map(|provider| provider.exception_on(trade_date))
    }

    /// Returns the underlying date-aware calendar.
    #[must_use]
    pub const fn calendar(self) -> ExchangeCalendar {
        self.calendar
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
    pub fn hours_at(self, instant: DateTime<Utc>) -> MarketHours {
        self.calendar.hours_at(instant)
    }

    /// Returns the schedule's invariant IANA time zone.
    #[must_use]
    pub fn tz(self) -> Tz {
        self.calendar.tz()
    }

    /// Returns whether any effective regular or extended session is open.
    #[must_use]
    pub fn is_open(self, instant: DateTime<Utc>) -> bool {
        self.is_open_with(instant, SessionKind::Both)
    }

    /// Returns whether the effective selected session kind is open.
    #[must_use]
    pub fn is_open_with(self, instant: DateTime<Utc>, kind: SessionKind) -> bool {
        status::is_open_with(&self.context(), instant, kind)
    }

    /// Returns whether an effective regular session is open.
    #[must_use]
    pub fn is_open_regular(self, instant: DateTime<Utc>) -> bool {
        self.is_open_with(instant, SessionKind::Regular)
    }

    /// Returns whether an effective extended session is open.
    #[must_use]
    pub fn is_open_extended(self, instant: DateTime<Utc>) -> bool {
        self.is_open_with(instant, SessionKind::Extended)
    }

    /// Returns whether orders may be entered, amended or cancelled at `instant`.
    ///
    /// True during effective order-entry-only phases and during any effective
    /// tradeable session. A closed trade date removes the queue that feeds it;
    /// a replaced trade date serves only the order-entry blocks the caller
    /// supplied for it.
    #[must_use]
    pub fn is_accepting_orders(self, instant: DateTime<Utc>) -> bool {
        status::is_accepting_orders(&self.context(), instant)
    }

    /// Returns whether `instant` falls in an effective order-entry-only phase.
    ///
    /// Mutually exclusive with [`Self::is_open`].
    #[must_use]
    pub fn is_order_entry_only(self, instant: DateTime<Utc>) -> bool {
        status::is_order_entry_only(&self.context(), instant)
    }

    /// Returns the containing or next effective session bounds.
    #[must_use]
    pub fn session_bounds(self, instant: DateTime<Utc>) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        self.session_bounds_with(instant, SessionKind::Both)
    }

    /// Returns the containing or next effective bounds for `kind`.
    #[must_use]
    pub fn session_bounds_with(
        self,
        instant: DateTime<Utc>,
        kind: SessionKind,
    ) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        sessions::session_bounds_with(&self.context(), instant, kind)
    }

    /// Returns the first effective regular/extended session after `instant`.
    #[must_use]
    pub fn next_session_after(
        self,
        instant: DateTime<Utc>,
    ) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        self.next_session_after_with(instant, SessionKind::Both)
    }

    /// Returns the first effective session of `kind` after `instant`.
    #[must_use]
    pub fn next_session_after_with(
        self,
        instant: DateTime<Utc>,
        kind: SessionKind,
    ) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        sessions::next_session_after_with(&self.context(), instant, kind)
    }

    /// Returns only the next effective regular-or-extended session open.
    #[must_use]
    pub fn next_session_open_after(self, instant: DateTime<Utc>) -> Option<DateTime<Utc>> {
        self.next_session_after(instant).map(|(open, _close)| open)
    }

    /// Returns whether `instant` is in a documented short maintenance gap.
    #[must_use]
    pub fn is_maintenance(self, instant: DateTime<Utc>) -> bool {
        status::is_maintenance(&self.context(), instant)
    }

    /// Returns the effective policy-aware state at `instant`.
    ///
    /// Maintenance normally separates trade dates within the four-hour bound;
    /// a continuously traded-week profile can retain an operator-designated
    /// short maintenance gap inside one trade date.
    #[must_use]
    pub fn session_state(self, instant: DateTime<Utc>) -> SessionState {
        status::session_state(&self.context(), instant)
    }

    /// Returns the effective local trade date containing `instant`.
    ///
    /// An identified following-business-day schedule skips policy-closed dates;
    /// CME cryptocurrency weekend trading therefore returns Tuesday when its
    /// usual Monday trade date is closed.
    #[must_use]
    pub fn trade_date(self, instant: DateTime<Utc>) -> Option<NaiveDate> {
        status::trade_date(&self.context(), instant)
    }

    /// Returns whether no effective session of `kind` is assigned to `day`.
    ///
    /// Profiles without a trade-date concept return `true` because no session
    /// can be assigned to the requested trade date. Use the civil-day queries
    /// when asking whether trading intersects a calendar date instead.
    #[must_use]
    pub fn is_closed_trade_date(self, day: NaiveDate, kind: SessionKind) -> bool {
        status::is_closed_trade_date(&self.context(), day, kind)
    }

    /// Returns whether no effective session intersects `day` in `calendar_tz`.
    #[must_use]
    pub fn is_closed_all_day_in_calendar(
        self,
        day: NaiveDate,
        calendar_tz: Tz,
        kind: SessionKind,
    ) -> bool {
        status::is_closed_all_day_in_calendar(&self.context(), day, calendar_tz, kind)
    }

    /// Returns whether no effective session intersects venue-local `day`.
    ///
    /// This remains a calendar-date overlap query. Closing Monday's trade date
    /// does not make Monday wholly closed when Tuesday's session opens Monday
    /// evening; use [`Self::is_closed_trade_date`] for that question.
    #[must_use]
    pub fn is_closed_all_day_on(self, day: NaiveDate, kind: SessionKind) -> bool {
        self.is_closed_all_day_in_calendar(day, self.tz(), kind)
    }

    /// Returns whether no effective session intersects the selected calendar day.
    #[must_use]
    pub fn is_closed_all_day_at(
        self,
        instant: DateTime<Utc>,
        calendar_tz: Tz,
        kind: SessionKind,
    ) -> bool {
        status::is_closed_all_day_at(&self.context(), instant, calendar_tz, kind)
    }
}

impl ExchangeCalendar {
    /// Applies caller-supplied trade-date boundary overrides to every query.
    #[must_use]
    pub const fn with_day_policy(self, policy: &dyn DayPolicy) -> PolicyCalendar<'_> {
        PolicyCalendar::new(self, Some(policy), None)
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
    ) -> Result<PolicyCalendar<'_>, ExceptionScopeError> {
        let provider = exceptions.source();
        if provider == self.source() {
            Ok(PolicyCalendar::new(self, None, Some(exceptions)))
        } else {
            Err(ExceptionScopeError {
                calendar: self.source(),
                provider,
            })
        }
    }
}
