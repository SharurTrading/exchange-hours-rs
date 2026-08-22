// SPDX-License-Identifier: MIT-0

//! Caller-supplied trade-date overrides and their calendar adapter.

mod candles;
mod static_policy;
mod week;

pub use static_policy::{DayOverride, StaticDayPolicy, StaticDayPolicyError};

use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Tz;

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
/// topology needs a complete exception-session provider.
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

/// A date-aware calendar with caller-supplied trade-date overrides.
///
/// The wrapper borrows the policy and remains allocation-free itself. Work or
/// allocation performed inside the caller's policy callbacks is outside the
/// calendar's performance guarantees. [`Self::hours_at`] deliberately returns
/// the unmodified sourced profile; the policy overlays queries, not tables.
#[derive(Clone, Copy)]
pub struct PolicyCalendar<'a> {
    calendar: ExchangeCalendar,
    policy: &'a dyn DayPolicy,
}

impl<'a> PolicyCalendar<'a> {
    pub(super) const fn new(calendar: ExchangeCalendar, policy: &'a dyn DayPolicy) -> Self {
        Self { calendar, policy }
    }

    fn context(self) -> QueryContext<'a> {
        QueryContext::with_policy(self.calendar, self.policy)
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
    /// Applies caller-supplied trade-date overrides to every query.
    #[must_use]
    pub const fn with_day_policy(self, policy: &dyn DayPolicy) -> PolicyCalendar<'_> {
        PolicyCalendar::new(self, policy)
    }
}
