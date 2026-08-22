// SPDX-License-Identifier: MIT-0

//! Date-aware candle adapters over the shared query engine.

use chrono::{DateTime, Utc};

use super::ExchangeCalendar;
use crate::calendar::query::{QueryContext, candles};
use crate::calendar::{CalendarResolution, SessionKind};

impl ExchangeCalendar {
    /// Returns the date-aware bar close after `instant`.
    ///
    /// A key-backed CME cryptocurrency calendar retains Friday 16:00 CT as
    /// the weekly close even though its current profile has no long weekend
    /// shutdown. An identity-erased fixed snapshot cannot apply that
    /// product-family convention.
    #[must_use]
    pub fn candle_end(
        self,
        instant: DateTime<Utc>,
        resolution: CalendarResolution,
    ) -> Option<DateTime<Utc>> {
        self.candle_end_with(instant, resolution, SessionKind::Both)
    }

    /// Returns the date-aware bar close after `instant` for `kind`.
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
        candles::candle_end_with(&QueryContext::date_aware(self), instant, resolution, kind)
    }

    /// Returns the date-aware bar start paired with [`candle_end`](Self::candle_end).
    #[must_use]
    pub fn candle_start(
        self,
        instant: DateTime<Utc>,
        resolution: CalendarResolution,
    ) -> Option<DateTime<Utc>> {
        self.candle_start_with(instant, resolution, SessionKind::Both)
    }

    /// Returns the date-aware bar start paired with
    /// [`candle_end_with`](Self::candle_end_with) for `kind`.
    #[must_use]
    pub fn candle_start_with(
        self,
        instant: DateTime<Utc>,
        resolution: CalendarResolution,
        kind: SessionKind,
    ) -> Option<DateTime<Utc>> {
        candles::candle_start_with(&QueryContext::date_aware(self), instant, resolution, kind)
    }

    /// Returns the next date-aware trading-day close after `instant`.
    #[must_use]
    pub fn time_end_of_day(self, instant: DateTime<Utc>) -> Option<DateTime<Utc>> {
        self.candle_end(instant, CalendarResolution::Daily)
    }
}
