// SPDX-License-Identifier: MIT-0

//! A date-aware exchange calendar over point-in-time
//! [`MarketHours`](crate::MarketHours) profiles.

mod calendar;
mod candles;
mod queries;
mod source;
mod week;

pub(in crate::calendar) use calendar::HolidayView;
pub use calendar::{
    ExchangeCalendar, SessionWindow, calendar_for_exchange, calendar_for_market_hours_key,
};
pub use source::CalendarSource;
