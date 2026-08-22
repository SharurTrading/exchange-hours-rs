// SPDX-License-Identifier: MIT-0

//! Policy-aware normal-week duration.

use chrono::{DateTime, Utc};

use super::PolicyCalendar;
use crate::calendar::query::week;

impl PolicyCalendar<'_> {
    /// Returns distinct effective open seconds in the venue-local week.
    ///
    /// Unlike hot status and boundary queries, this helper may allocate while
    /// collecting and unioning intervals.
    #[must_use]
    pub fn normal_week_open_seconds_containing(self, instant: DateTime<Utc>) -> u64 {
        week::normal_week_open_seconds_containing(&self.context(), instant)
    }
}
