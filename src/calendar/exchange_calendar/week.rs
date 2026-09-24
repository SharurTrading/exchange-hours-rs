// SPDX-License-Identifier: MIT-0

//! Date-aware normal-week duration over the shared query engine.

use chrono::{DateTime, Utc};

use super::ExchangeCalendar;
use crate::calendar::CalendarQueryError;
use crate::calendar::query::{QueryContext, week};

impl ExchangeCalendar {
    /// Returns distinct scheduled open seconds in the venue-local week that
    /// contains `instant`, selecting each session by its actual opening day.
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
    pub fn normal_week_open_seconds_containing(
        self,
        instant: DateTime<Utc>,
    ) -> Result<u64, CalendarQueryError> {
        let context = QueryContext::date_aware(self);
        context.require_floor_at(instant)?;
        week::normal_week_open_seconds_containing(&context, instant)
    }
}
