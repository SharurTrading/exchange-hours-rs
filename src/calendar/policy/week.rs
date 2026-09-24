// SPDX-License-Identifier: MIT-0

//! Policy-aware normal-week duration.

use chrono::{DateTime, Utc};

use super::PolicyCalendar;
use crate::calendar::CalendarQueryError;
use crate::calendar::query::week;

impl PolicyCalendar<'_> {
    /// Returns distinct effective open seconds in the venue-local week.
    ///
    /// Unlike hot status and boundary queries, this helper may allocate while
    /// collecting and unioning intervals.
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
        let context = self.context();
        context.require_floor_at(instant)?;
        week::normal_week_open_seconds_containing(&context, instant)
    }
}
