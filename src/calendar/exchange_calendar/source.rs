// SPDX-License-Identifier: MIT-0

//! Which kind of identity a calendar's schedule is selected by.

use crate::calendar::{Exchange, MarketHoursKey};

/// Identifies the schedule selected by an
/// [`ExchangeCalendar`](crate::ExchangeCalendar).
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

impl From<Exchange> for CalendarSource {
    fn from(exchange: Exchange) -> Self {
        Self::Exchange(exchange)
    }
}

impl From<MarketHoursKey> for CalendarSource {
    fn from(key: MarketHoursKey) -> Self {
        Self::MarketHoursKey(key)
    }
}
