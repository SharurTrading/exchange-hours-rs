// SPDX-License-Identifier: MIT-0

//! The date-aware status, session-boundary, trade-date and civil-day query
//! surface. Every method delegates through the one date-aware
//! [`QueryContext`] path.

use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Tz;

use super::calendar::{ExchangeCalendar, SessionWindow};
use crate::calendar::query::{QueryContext, sessions, status};
use crate::calendar::{CalendarQueryError, SessionKind, SessionState};

impl ExchangeCalendar {
    /// Returns whether any regular or extended session is open at `instant`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarQueryError::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`CalendarQueryError::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`CalendarQueryError::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`CalendarQueryError::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_open(self, instant: DateTime<Utc>) -> Result<bool, CalendarQueryError> {
        self.is_open_with(instant, SessionKind::Both)
    }

    /// Returns whether the selected session kind is open at `instant`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarQueryError::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`CalendarQueryError::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`CalendarQueryError::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`CalendarQueryError::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_open_with(
        self,
        instant: DateTime<Utc>,
        kind: SessionKind,
    ) -> Result<bool, CalendarQueryError> {
        status::is_open_with(&QueryContext::date_aware(self), instant, kind)
    }

    /// Returns whether a regular session is open at `instant`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarQueryError::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`CalendarQueryError::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`CalendarQueryError::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`CalendarQueryError::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_open_regular(self, instant: DateTime<Utc>) -> Result<bool, CalendarQueryError> {
        self.is_open_with(instant, SessionKind::Regular)
    }

    /// Returns whether an extended session is open at `instant`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarQueryError::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`CalendarQueryError::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`CalendarQueryError::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`CalendarQueryError::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_open_extended(self, instant: DateTime<Utc>) -> Result<bool, CalendarQueryError> {
        self.is_open_with(instant, SessionKind::Extended)
    }

    /// Returns whether orders may be entered, amended or cancelled at `instant`.
    ///
    /// True during order-entry-only phases and during any tradeable session.
    /// This is a different question from [`Self::is_open`], which asks whether a
    /// trade can print. Resolved through the same date-aware profile selection
    /// as [`Self::is_open`], so a revision reselects per candidate opening day
    /// and a caller's [`DayPolicy`](crate::DayPolicy) overlay applies here too.
    /// Broker-side storage or modification of pending instructions outside a
    /// published session phase is outside this query's scope. For example,
    /// OANDA permits pending orders while its markets are closed; this method
    /// does not report the availability of OANDA's order-placement API.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarQueryError::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`CalendarQueryError::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`CalendarQueryError::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`CalendarQueryError::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_accepting_orders(self, instant: DateTime<Utc>) -> Result<bool, CalendarQueryError> {
        status::is_accepting_orders(&QueryContext::date_aware(self), instant)
    }

    /// Returns whether `instant` falls in an order-entry-only phase where no
    /// trade can match.
    ///
    /// Mutually exclusive with [`Self::is_open`]. Resolved through the same
    /// date-aware profile selection as [`Self::is_open`], so a revision
    /// reselects per candidate opening day and a caller's
    /// [`DayPolicy`](crate::DayPolicy) overlay applies here too.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarQueryError::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`CalendarQueryError::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`CalendarQueryError::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`CalendarQueryError::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_order_entry_only(self, instant: DateTime<Utc>) -> Result<bool, CalendarQueryError> {
        status::is_order_entry_only(&QueryContext::date_aware(self), instant)
    }

    /// Returns the containing or next regular/extended session bounds.
    ///
    /// See [`Self::session_bounds_with`] for the bounded-search semantics.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarQueryError::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`CalendarQueryError::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`CalendarQueryError::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`CalendarQueryError::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    ///
    /// This scan does **not** return
    /// [`CalendarQueryError::SearchExhausted`]: exhausting its own 14-day horizon
    /// is reported as `Ok(None)`, because the horizon is the query's documented
    /// bound rather than a day it failed to establish. Only the period walks
    /// (`candle_end`, `candle_start` and `trade_date`) report `SearchExhausted`,
    /// where a close must be found for the answer to exist.
    pub fn session_bounds(
        self,
        instant: DateTime<Utc>,
    ) -> Result<Option<SessionWindow>, CalendarQueryError> {
        self.session_bounds_with(instant, SessionKind::Both)
    }

    /// Returns the containing or next bounds for `kind`.
    ///
    /// The forward search runs through 14 venue-local days. `None` means no
    /// matching positive-width session exists within that bounded horizon.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarQueryError::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`CalendarQueryError::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`CalendarQueryError::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`CalendarQueryError::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    ///
    /// This scan does **not** return
    /// [`CalendarQueryError::SearchExhausted`]: exhausting its own 14-day horizon
    /// is reported as `Ok(None)`, because the horizon is the query's documented
    /// bound rather than a day it failed to establish. Only the period walks
    /// (`candle_end`, `candle_start` and `trade_date`) report `SearchExhausted`,
    /// where a close must be found for the answer to exist.
    pub fn session_bounds_with(
        self,
        instant: DateTime<Utc>,
        kind: SessionKind,
    ) -> Result<Option<SessionWindow>, CalendarQueryError> {
        sessions::session_bounds_with(&QueryContext::date_aware(self), instant, kind)
    }

    /// Returns the first regular/extended session opening strictly after
    /// `instant`, reselecting the profile for every candidate opening day.
    ///
    /// See [`Self::next_session_after_with`] for the bounded-search semantics.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarQueryError::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`CalendarQueryError::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`CalendarQueryError::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`CalendarQueryError::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    ///
    /// This scan does **not** return
    /// [`CalendarQueryError::SearchExhausted`]: exhausting its own 14-day horizon
    /// is reported as `Ok(None)`, because the horizon is the query's documented
    /// bound rather than a day it failed to establish. Only the period walks
    /// (`candle_end`, `candle_start` and `trade_date`) report `SearchExhausted`,
    /// where a close must be found for the answer to exist.
    pub fn next_session_after(
        self,
        instant: DateTime<Utc>,
    ) -> Result<Option<SessionWindow>, CalendarQueryError> {
        self.next_session_after_with(instant, SessionKind::Both)
    }

    /// Returns the first session of `kind` opening strictly after `instant`.
    ///
    /// The search runs through 14 venue-local days and skips unavailable or
    /// civil-time-collapsed occurrences. `None` means no matching session was
    /// found within that bounded horizon.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarQueryError::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`CalendarQueryError::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`CalendarQueryError::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`CalendarQueryError::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    ///
    /// This scan does **not** return
    /// [`CalendarQueryError::SearchExhausted`]: exhausting its own 14-day horizon
    /// is reported as `Ok(None)`, because the horizon is the query's documented
    /// bound rather than a day it failed to establish. Only the period walks
    /// (`candle_end`, `candle_start` and `trade_date`) report `SearchExhausted`,
    /// where a close must be found for the answer to exist.
    pub fn next_session_after_with(
        self,
        instant: DateTime<Utc>,
        kind: SessionKind,
    ) -> Result<Option<SessionWindow>, CalendarQueryError> {
        sessions::next_session_after_with(&QueryContext::date_aware(self), instant, kind)
    }

    /// Returns only the next regular/extended session open after `instant`.
    ///
    /// This projects [`Self::next_session_after`], including its bounded
    /// 14-local-day search and `None` semantics.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarQueryError::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`CalendarQueryError::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`CalendarQueryError::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`CalendarQueryError::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn next_session_open_after(
        self,
        instant: DateTime<Utc>,
    ) -> Result<Option<DateTime<Utc>>, CalendarQueryError> {
        Ok(self.next_session_after(instant)?.map(|(open, _close)| open))
    }

    /// Returns whether `instant` lies in a short operational maintenance break.
    ///
    /// This is exactly [`SessionState::Maintenance`]; see [`Self::session_state`]
    /// for the four-hour inclusive bound and halt distinction.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarQueryError::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`CalendarQueryError::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`CalendarQueryError::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`CalendarQueryError::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_maintenance(self, instant: DateTime<Utc>) -> Result<bool, CalendarQueryError> {
        status::is_maintenance(&QueryContext::date_aware(self), instant)
    }

    /// Returns the effective state at `instant`.
    ///
    /// `Maintenance` is normally a complete inter-trade-date gap of at most
    /// four elapsed hours within one ISO week. A continuously traded-week
    /// profile also retains an operator-designated gap of that length inside
    /// one trade date. Same-trade-date gaps — a scheduled lunch break, for
    /// example — and longer gaps and weekends are `Closed`: the crate models
    /// scheduled hours only and has no halt concept.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarQueryError::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`CalendarQueryError::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`CalendarQueryError::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`CalendarQueryError::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn session_state(self, instant: DateTime<Utc>) -> Result<SessionState, CalendarQueryError> {
        status::session_state(&QueryContext::date_aware(self), instant)
    }

    /// Returns the venue-local trade date of the session containing `instant`.
    ///
    /// The date is taken from the effective trading day's final close, so a
    /// Sunday-evening Globex phase normally maps to Monday. An identified
    /// calendar can retain a sourced convention instead: SET's after-midnight
    /// DR night phase keeps its prior local opening date, CBOT Rough Rice's
    /// non-wrapping 19:00-21:00 CT evening leg takes the following local date,
    /// and CME cryptocurrency's weekend blocks use the following open business
    /// date.
    /// Closed instants return `None`, including halts and maintenance gaps.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarQueryError::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`CalendarQueryError::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`CalendarQueryError::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`CalendarQueryError::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn trade_date(
        self,
        instant: DateTime<Utc>,
    ) -> Result<Option<NaiveDate>, CalendarQueryError> {
        status::trade_date(&QueryContext::date_aware(self), instant)
    }

    /// Returns whether no effective session of `kind` is assigned to `day` as
    /// its venue-local trade date.
    ///
    /// Profiles without a trade-date concept return `true` because no session
    /// can be assigned to the requested trade date. Use the civil-day queries
    /// when asking whether trading intersects a calendar date instead.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarQueryError::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`CalendarQueryError::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`CalendarQueryError::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`CalendarQueryError::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_closed_trade_date(
        self,
        day: NaiveDate,
        kind: SessionKind,
    ) -> Result<bool, CalendarQueryError> {
        status::is_closed_trade_date(&QueryContext::date_aware(self), day, kind)
    }

    /// Returns whether no session of `kind` intersects `day` in `calendar_tz`.
    /// A wholly skipped civil date has an empty window and is closed.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarQueryError::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`CalendarQueryError::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`CalendarQueryError::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`CalendarQueryError::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_closed_all_day_in_calendar(
        self,
        day: NaiveDate,
        calendar_tz: Tz,
        kind: SessionKind,
    ) -> Result<bool, CalendarQueryError> {
        status::is_closed_all_day_in_calendar(
            &QueryContext::date_aware(self),
            day,
            calendar_tz,
            kind,
        )
    }

    /// Returns whether no session of `kind` intersects venue-local `day`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarQueryError::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`CalendarQueryError::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`CalendarQueryError::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`CalendarQueryError::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_closed_all_day_on(
        self,
        day: NaiveDate,
        kind: SessionKind,
    ) -> Result<bool, CalendarQueryError> {
        self.is_closed_all_day_in_calendar(day, self.tz(), kind)
    }

    /// Returns whether no session of `kind` intersects the calendar day
    /// containing `instant` in `calendar_tz`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarQueryError::BeforeSupportFloor`] when the venue-local
    /// day this query is addressed to precedes
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR),
    /// [`CalendarQueryError::OutsideCoveredRange`] when the identity has no
    /// sourced answer for a day the query depends on,
    /// [`CalendarQueryError::UnresolvedGap`] when that day is one the identity
    /// withholds as `Unsourced`, and
    /// [`CalendarQueryError::SearchExhausted`] when a bounded forward search
    /// runs out of window on a day it cannot establish. An error is never
    /// reported as `false`, `None`, or a default schedule (LAW-COVERAGE).
    pub fn is_closed_all_day_at(
        self,
        instant: DateTime<Utc>,
        calendar_tz: Tz,
        kind: SessionKind,
    ) -> Result<bool, CalendarQueryError> {
        status::is_closed_all_day_at(&QueryContext::date_aware(self), instant, calendar_tz, kind)
    }
}
