// SPDX-License-Identifier: MIT-0

//! Date-aware candle adapters over the shared query engine.

use chrono::{DateTime, Utc};

use super::ExchangeCalendar;
use crate::calendar::query::{QueryContext, candles};
use crate::calendar::{CalendarQueryError, CalendarResolution, SessionKind};

impl ExchangeCalendar {
    /// Returns the date-aware bar close after `instant`.
    ///
    /// A key-backed CME cryptocurrency calendar retains Friday 16:00 CT as
    /// the weekly close even though its current profile has no long weekend
    /// shutdown. An identity-erased fixed snapshot cannot apply that
    /// product-family convention.
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
    pub fn candle_end(
        self,
        instant: DateTime<Utc>,
        resolution: CalendarResolution,
    ) -> Result<Option<DateTime<Utc>>, CalendarQueryError> {
        self.candle_end_with(instant, resolution, SessionKind::Both)
    }

    /// Returns the date-aware bar close after `instant` for `kind`.
    ///
    /// The CME cryptocurrency weekly-boundary convention described by
    /// [`Self::candle_end`] applies here as well.
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
    pub fn candle_end_with(
        self,
        instant: DateTime<Utc>,
        resolution: CalendarResolution,
        kind: SessionKind,
    ) -> Result<Option<DateTime<Utc>>, CalendarQueryError> {
        let context = QueryContext::date_aware(self);
        context.require_floor_at(instant)?;
        candles::candle_end_with(&context, instant, resolution, kind)
    }

    /// Returns the date-aware bar start paired with [`candle_end`](Self::candle_end).
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
    pub fn candle_start(
        self,
        instant: DateTime<Utc>,
        resolution: CalendarResolution,
    ) -> Result<Option<DateTime<Utc>>, CalendarQueryError> {
        self.candle_start_with(instant, resolution, SessionKind::Both)
    }

    /// Returns the date-aware bar start paired with
    /// [`candle_end_with`](Self::candle_end_with) for `kind`.
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
    pub fn candle_start_with(
        self,
        instant: DateTime<Utc>,
        resolution: CalendarResolution,
        kind: SessionKind,
    ) -> Result<Option<DateTime<Utc>>, CalendarQueryError> {
        let context = QueryContext::date_aware(self);
        context.require_floor_at(instant)?;
        candles::candle_start_with(&context, instant, resolution, kind)
    }

    /// Returns the next date-aware trading-day close after `instant`.
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
    pub fn time_end_of_day(
        self,
        instant: DateTime<Utc>,
    ) -> Result<Option<DateTime<Utc>>, CalendarQueryError> {
        self.candle_end(instant, CalendarResolution::Daily)
    }
}
