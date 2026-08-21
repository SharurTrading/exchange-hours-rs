// SPDX-License-Identifier: MIT-0

//! Public-surface contract invariants, grouped by the behavior they fence.

mod current_fences;
mod historical_expectations;
mod historical_fences;
mod identity;
mod identity_expectations;
mod probe_support;
mod properties;
mod property_support;
mod rule_tables;

mod prelude {
    pub(super) use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Utc};
    pub(super) use exchange_hours::{
        CalendarResolution, Exchange, MarketHours, SessionKind, candle_end, candle_end_with,
        hours_for_exchange, hours_for_exchange_as_of, next_session_after, next_session_after_with,
        next_session_open_after, session_bounds, session_bounds_with,
    };
}
