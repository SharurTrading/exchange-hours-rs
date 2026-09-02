// SPDX-License-Identifier: MIT-0

#![forbid(unsafe_code)]

//! The calendar: exchange trading hours, sessions, and bar boundaries.
//!
//! Everything in this crate lives here and is re-exported flat from the crate
//! root, so `calendar` is an organisational boundary rather than a public path.
//! The submodules are layered by responsibility:
//!
//! | Layer | Modules | Owns |
//! |---|---|---|
//! | Values | [`exchange`], [`rule`], [`resolution`], [`hours`], [`exchange_calendar`] | Venue identity, session slices, fixed and date-aware calendar values |
//! | Overlays | [`policy`], [`exceptions`] | Caller-owned boundary overrides and replacement trading days |
//! | Schedules | [`schedules`], [`futures_profile`], [`presets`] | Sourced static tables, revisions, and exhaustive venue routing |
//! | Civil time | [`local_time`] | The one place a local wall-clock becomes a UTC instant |
//! | Queries | [`query`] with [`session`] / [`candle`] adapters | One engine for open/closed, bounds, periods, and bar boundaries |
//! | Collections | [`bulk`] | Deliberate regional venue membership |
//!
//! ## The three things that decide every answer
//!
//! **Time semantics.** All public helpers take and return
//! `chrono::DateTime<Utc>`. The exchange's local zone is used only internally —
//! to interpret a rule's seconds-since-local-midnight (SSM) and to define "the
//! day". Weekdays are indexed Monday = 0 through Sunday = 6 throughout.
//!
//! **End-exclusive closes.** An instant exactly equal to a close is *closed*.
//! Without this, adjacent sessions overlap and every duration double-counts at
//! its boundary.
//!
//! **Wraps.** `open_ssm >= close_ssm` means the session runs into the next local
//! day. Equal endpoints encode one complete local-day span. This is how
//! overnight and Globex-style trading days are expressed, and it is why "is it
//! open?" must consult yesterday's rules as well as today's.
//!
//! ## What this calendar is not
//!
//! Built-in profiles are **normal-week** models and ship no holiday data.
//! Callers can overlay their own deterministic, trade-date-keyed closures and
//! early/late boundaries with [`DayPolicy`], [`StaticDayPolicy`], and
//! [`PolicyCalendar`]. A special day that replaces or splits internal phases
//! needs the exception layer instead: [`SessionExceptionSource`] and
//! [`StaticSessionExceptions`] replace a whole trade date with an ordered
//! [`ExceptionBlock`] set, and the caller's [`DayPolicy`] then overlays that
//! replacement exactly as it overlays a normal week. Each
//! source-backed schedule states its venue, segment, or product-family scope
//! explicitly; it is not a claim about products outside that scope. All fixed,
//! date-aware, and overlay-aware query paths share the same resolver.

mod bulk;
mod candle;
mod exceptions;
mod exchange;
mod exchange_calendar;
mod futures_profile;
mod hours;
mod local_time;
mod policy;
mod presets;
mod query;
mod resolution;
mod rule;
mod schedules;
mod session;
mod state;

pub use bulk::{
    hours_for_all, hours_for_apac_equities, hours_for_eu_equities, hours_for_global_equities,
    hours_for_us_equities, hours_map_apac_equities, hours_map_eu_equities, hours_map_for,
    hours_map_global_equities, hours_map_us_equities,
};
pub use candle::{candle_end, candle_end_with, candle_start, candle_start_with, time_end_of_day};
pub use exceptions::{
    DateException, ExceptionBlock, ExceptionBlockKind, ExceptionCoverage, ExceptionScopeError,
    SessionExceptionRecord, SessionExceptionSource, StaticSessionExceptions,
    StaticSessionExceptionsError,
};
pub use exchange::{Exchange, ParseExchangeError};
pub use exchange_calendar::{
    CalendarSource, ExchangeCalendar, calendar_for_exchange, calendar_for_market_hours_key,
};
pub use futures_profile::{
    FuturesSessionProfile, MarketHoursKey, ParseMarketHoursKeyError, hours_for_market_hours_key,
    session_profile,
};
pub use hours::MarketHours;
pub use policy::{
    DayOverride, DayPolicy, NoPolicy, PolicyCalendar, StaticDayPolicy, StaticDayPolicyError,
};
pub use presets::hours_for_exchange;
pub use resolution::CalendarResolution;
pub use rule::{SessionKind, SessionRule, SessionRuleError};
pub use session::{
    next_session_after, next_session_after_with, next_session_open_after, session_bounds,
    session_bounds_with,
};
pub use state::SessionState;
