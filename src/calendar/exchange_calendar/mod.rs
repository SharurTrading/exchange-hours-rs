// SPDX-License-Identifier: MIT-0

//! A date-aware exchange calendar over point-in-time [`MarketHours`] profiles.

mod candles;
mod week;

use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Tz;

use super::query::{QueryContext, sessions, status};
use super::{
    Exchange, MarketHours, MarketHoursKey, SessionKind, SessionState, hours_for_exchange,
    hours_for_exchange_as_of, hours_for_market_hours_key, hours_for_market_hours_key_as_of,
};

/// Identifies the schedule selected by an [`ExchangeCalendar`].
///
/// Venue calendars use [`CalendarSource::Exchange`]. Product-family calendars
/// use [`CalendarSource::MarketHoursKey`] so callers can log and persist the
/// exact family rather than treating it as [`Exchange::Unknown`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CalendarSource {
    /// A venue or market-segment identity.
    Exchange(Exchange),
    /// A product-family market-hours identity.
    MarketHoursKey(MarketHoursKey),
}

/// A deterministic, date-aware calendar for one exchange or product family.
///
/// The calendar reselects a concrete fixed profile for every candidate local
/// opening day. It holds no clock and performs no I/O. It is
/// `Copy + Send + Sync + 'static`; built-in status, session-boundary,
/// trade-date, session-state, and candle-boundary queries allocate nothing.
/// A containing-session predicate takes `O(R + log H)` work for `R` selected
/// rules and `H` dated revisions. Forward and period queries inspect at most a
/// documented bounded number `D` of local days and therefore take
/// `O(D * (R + log H))` work.
/// Caller work inside a [`DayPolicy`](super::DayPolicy) is outside this
/// performance guarantee. The normal-week duration helper is also excluded
/// because it collects and sorts temporary intervals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExchangeCalendar {
    source: CalendarSource,
}

impl ExchangeCalendar {
    /// Creates a date-aware calendar for `exchange`.
    #[must_use]
    pub const fn new(exchange: Exchange) -> Self {
        Self {
            source: CalendarSource::Exchange(exchange),
        }
    }

    /// Creates a date-aware calendar for `key`.
    #[must_use]
    pub const fn for_market_hours_key(key: MarketHoursKey) -> Self {
        Self {
            source: CalendarSource::MarketHoursKey(key),
        }
    }

    /// Returns the schedule identity represented by this calendar.
    #[must_use]
    pub const fn source(self) -> CalendarSource {
        self.source
    }

    /// Returns the represented exchange, or `None` for a product-family calendar.
    #[must_use]
    pub const fn exchange(self) -> Option<Exchange> {
        match self.source {
            CalendarSource::Exchange(exchange) => Some(exchange),
            CalendarSource::MarketHoursKey(_) => None,
        }
    }

    /// Returns the represented product-family key, or `None` for an exchange calendar.
    #[must_use]
    pub const fn market_hours_key(self) -> Option<MarketHoursKey> {
        match self.source {
            CalendarSource::Exchange(_) => None,
            CalendarSource::MarketHoursKey(key) => Some(key),
        }
    }

    /// Resolves the fixed [`MarketHours`] profile in force at `instant`.
    #[must_use]
    pub fn hours_at(self, instant: DateTime<Utc>) -> MarketHours {
        match self.source {
            CalendarSource::Exchange(exchange) => hours_for_exchange_as_of(exchange, instant),
            CalendarSource::MarketHoursKey(key) => hours_for_market_hours_key_as_of(key, instant),
        }
    }

    /// Returns the venue's invariant IANA time zone.
    #[must_use]
    pub fn tz(self) -> Tz {
        match self.source {
            CalendarSource::Exchange(exchange) => hours_for_exchange(exchange).tz,
            CalendarSource::MarketHoursKey(key) => hours_for_market_hours_key(key).tz,
        }
    }

    /// Returns whether any regular or extended session is open at `instant`.
    #[must_use]
    pub fn is_open(self, instant: DateTime<Utc>) -> bool {
        self.is_open_with(instant, SessionKind::Both)
    }

    /// Returns whether the selected session kind is open at `instant`.
    #[must_use]
    pub fn is_open_with(self, instant: DateTime<Utc>, kind: SessionKind) -> bool {
        status::is_open_with(&QueryContext::date_aware(self), instant, kind)
    }

    /// Returns whether a regular session is open at `instant`.
    #[must_use]
    pub fn is_open_regular(self, instant: DateTime<Utc>) -> bool {
        self.is_open_with(instant, SessionKind::Regular)
    }

    /// Returns whether an extended session is open at `instant`.
    #[must_use]
    pub fn is_open_extended(self, instant: DateTime<Utc>) -> bool {
        self.is_open_with(instant, SessionKind::Extended)
    }

    /// Returns the containing or next regular/extended session bounds.
    ///
    /// See [`Self::session_bounds_with`] for the bounded-search semantics.
    #[must_use]
    pub fn session_bounds(self, instant: DateTime<Utc>) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        self.session_bounds_with(instant, SessionKind::Both)
    }

    /// Returns the containing or next bounds for `kind`.
    ///
    /// The forward search runs through 14 venue-local days. `None` means no
    /// matching positive-width session exists within that bounded horizon.
    #[must_use]
    pub fn session_bounds_with(
        self,
        instant: DateTime<Utc>,
        kind: SessionKind,
    ) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        sessions::session_bounds_with(&QueryContext::date_aware(self), instant, kind)
    }

    /// Returns the first regular/extended session opening strictly after
    /// `instant`, reselecting the profile for every candidate opening day.
    ///
    /// See [`Self::next_session_after_with`] for the bounded-search semantics.
    #[must_use]
    pub fn next_session_after(
        self,
        instant: DateTime<Utc>,
    ) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        self.next_session_after_with(instant, SessionKind::Both)
    }

    /// Returns the first session of `kind` opening strictly after `instant`.
    ///
    /// The search runs through 14 venue-local days and skips unavailable or
    /// civil-time-collapsed occurrences. `None` means no matching session was
    /// found within that bounded horizon.
    #[must_use]
    pub fn next_session_after_with(
        self,
        instant: DateTime<Utc>,
        kind: SessionKind,
    ) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        sessions::next_session_after_with(&QueryContext::date_aware(self), instant, kind)
    }

    /// Returns only the next regular/extended session open after `instant`.
    ///
    /// This projects [`Self::next_session_after`], including its bounded
    /// 14-local-day search and `None` semantics.
    #[must_use]
    pub fn next_session_open_after(self, instant: DateTime<Utc>) -> Option<DateTime<Utc>> {
        self.next_session_after(instant).map(|(open, _close)| open)
    }

    /// Returns whether `instant` lies in a short operational maintenance break.
    ///
    /// This is exactly [`SessionState::Maintenance`]; see [`Self::session_state`]
    /// for the four-hour inclusive bound and halt distinction.
    #[must_use]
    pub fn is_maintenance(self, instant: DateTime<Utc>) -> bool {
        status::is_maintenance(&QueryContext::date_aware(self), instant)
    }

    /// Returns the effective state at `instant`.
    ///
    /// `Halt` separates phases assigned to one trade date. `Maintenance` is
    /// normally a complete inter-trade-date gap of at most four elapsed hours
    /// within one ISO week. A continuously traded-week profile also retains an
    /// operator-designated gap of that length inside one trade date. Longer
    /// gaps and weekends are `Closed`.
    #[must_use]
    pub fn session_state(self, instant: DateTime<Utc>) -> SessionState {
        status::session_state(&QueryContext::date_aware(self), instant)
    }

    /// Returns the venue-local trade date of the session containing `instant`.
    ///
    /// The date is taken from the effective trading day's final close, so a
    /// Sunday-evening Globex phase normally maps to Monday. An identified
    /// calendar can retain a sourced convention instead: SET's after-midnight
    /// DR night phase keeps its prior local opening date, and CME
    /// cryptocurrency's weekend blocks use the following open business date.
    /// Closed instants return `None`, including halts and maintenance gaps.
    #[must_use]
    pub fn trade_date(self, instant: DateTime<Utc>) -> Option<NaiveDate> {
        status::trade_date(&QueryContext::date_aware(self), instant)
    }

    /// Returns whether no effective session of `kind` is assigned to `day` as
    /// its venue-local trade date.
    ///
    /// Profiles without a trade-date concept return `true` because no session
    /// can be assigned to the requested trade date. Use the civil-day queries
    /// when asking whether trading intersects a calendar date instead.
    #[must_use]
    pub fn is_closed_trade_date(self, day: NaiveDate, kind: SessionKind) -> bool {
        status::is_closed_trade_date(&QueryContext::date_aware(self), day, kind)
    }

    /// Returns whether no session of `kind` intersects `day` in `calendar_tz`.
    /// A wholly skipped civil date has an empty window and is closed.
    #[must_use]
    pub fn is_closed_all_day_in_calendar(
        self,
        day: NaiveDate,
        calendar_tz: Tz,
        kind: SessionKind,
    ) -> bool {
        status::is_closed_all_day_in_calendar(
            &QueryContext::date_aware(self),
            day,
            calendar_tz,
            kind,
        )
    }

    /// Returns whether no session of `kind` intersects venue-local `day`.
    #[must_use]
    pub fn is_closed_all_day_on(self, day: NaiveDate, kind: SessionKind) -> bool {
        self.is_closed_all_day_in_calendar(day, self.tz(), kind)
    }

    /// Returns whether no session of `kind` intersects the calendar day
    /// containing `instant` in `calendar_tz`.
    #[must_use]
    pub fn is_closed_all_day_at(
        self,
        instant: DateTime<Utc>,
        calendar_tz: Tz,
        kind: SessionKind,
    ) -> bool {
        status::is_closed_all_day_at(&QueryContext::date_aware(self), instant, calendar_tz, kind)
    }
}

/// Creates a date-aware calendar for `exchange`.
#[must_use]
pub const fn calendar_for_exchange(exchange: Exchange) -> ExchangeCalendar {
    ExchangeCalendar::new(exchange)
}

/// Creates a date-aware calendar for a product-family `key`.
///
/// The calendar reselects [`hours_for_market_hours_key_as_of`] for every
/// candidate session-opening day, including while scanning across a sourced
/// historical revision.
#[must_use]
pub const fn calendar_for_market_hours_key(key: MarketHoursKey) -> ExchangeCalendar {
    ExchangeCalendar::for_market_hours_key(key)
}
