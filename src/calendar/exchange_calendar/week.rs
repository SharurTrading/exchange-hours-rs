// SPDX-License-Identifier: MIT-0

//! Date-aware normal-week duration over the shared query engine.

use chrono::{DateTime, Utc};

use super::ExchangeCalendar;
use crate::calendar::query::{QueryContext, week};

impl ExchangeCalendar {
    /// Returns distinct scheduled open seconds in the venue-local week that
    /// contains `instant`, selecting each session by its actual opening day.
    #[must_use]
    pub fn normal_week_open_seconds_containing(self, instant: DateTime<Utc>) -> u64 {
        week::normal_week_open_seconds_containing(&QueryContext::date_aware(self), instant)
    }
}
