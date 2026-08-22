// SPDX-License-Identifier: MIT-0

//! Policy-aware candle adapters.

use chrono::{DateTime, Utc};

use super::PolicyCalendar;
use crate::calendar::query::candles;
use crate::calendar::{CalendarResolution, SessionKind};

impl PolicyCalendar<'_> {
    /// Returns the policy-aware bar close after `instant`.
    ///
    /// For a key-backed CME cryptocurrency calendar, Friday 16:00 CT remains
    /// the weekly close even though its current profile has no long weekend
    /// shutdown. Policy changes to the following business date do not move
    /// that physical weekly boundary.
    #[must_use]
    pub fn candle_end(
        self,
        instant: DateTime<Utc>,
        resolution: CalendarResolution,
    ) -> Option<DateTime<Utc>> {
        self.candle_end_with(instant, resolution, SessionKind::Both)
    }

    /// Returns the policy-aware bar close after `instant` for `kind`.
    ///
    /// The CME cryptocurrency weekly-boundary convention described by
    /// [`Self::candle_end`] applies here as well.
    #[must_use]
    pub fn candle_end_with(
        self,
        instant: DateTime<Utc>,
        resolution: CalendarResolution,
        kind: SessionKind,
    ) -> Option<DateTime<Utc>> {
        candles::candle_end_with(&self.context(), instant, resolution, kind)
    }

    /// Returns the policy-aware bar start paired with [`Self::candle_end`].
    #[must_use]
    pub fn candle_start(
        self,
        instant: DateTime<Utc>,
        resolution: CalendarResolution,
    ) -> Option<DateTime<Utc>> {
        self.candle_start_with(instant, resolution, SessionKind::Both)
    }

    /// Returns the policy-aware bar start for `kind`.
    #[must_use]
    pub fn candle_start_with(
        self,
        instant: DateTime<Utc>,
        resolution: CalendarResolution,
        kind: SessionKind,
    ) -> Option<DateTime<Utc>> {
        candles::candle_start_with(&self.context(), instant, resolution, kind)
    }

    /// Returns the next policy-aware trading-day close after `instant`.
    #[must_use]
    pub fn time_end_of_day(self, instant: DateTime<Utc>) -> Option<DateTime<Utc>> {
        self.candle_end(instant, CalendarResolution::Daily)
    }
}
