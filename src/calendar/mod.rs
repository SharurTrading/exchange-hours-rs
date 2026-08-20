// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

#![forbid(unsafe_code)]

//! The calendar: exchange trading hours, sessions, and bar boundaries.
//!
//! Everything in this crate lives here and is re-exported flat from the crate
//! root, so `calendar` is an organisational boundary rather than a public path.
//! The submodules are layered — each one depends only on the ones above it:
//!
//! | Layer | Modules | Owns |
//! |---|---|---|
//! | Values | [`exchange`], [`rule`], [`resolution`] | Venue identity, one session slice, one bar interval |
//! | Data | [`profiles`], [`futures_profile`] | The static venue tables, addressed by venue and by product family |
//! | Resolution | [`local_time`] | The one place a local wall-clock becomes an instant |
//! | Queries | [`hours`], [`session`], [`period`], [`candle`] | Open/closed, session bounds, period closes, bar boundaries |
//! | Entry points | [`presets`], [`bulk`] | Venue → hours, one venue or many |
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
//! **Wraps.** `open_ssm > close_ssm` means the session runs past local midnight
//! and closes the next local day. This is how overnight and Globex-style trading
//! days are expressed, and it is why "is it open?" must consult yesterday's
//! rules as well as today's.
//!
//! ## What this calendar is not
//!
//! It is a **normal-week** model. Holidays, early closes, half-days, and
//! product-level variations are out of scope: [`local_time::is_holiday`] is a
//! stub that always returns `false`, and the profiles are pragmatic
//! exchange-level defaults, not contract-exact schedules. The wrap and daily-close
//! paths already route through the holiday hook, so a real calendar lands as a
//! body change rather than a control-flow change.

mod bulk;
mod candle;
mod exchange;
mod futures_profile;
mod hours;
mod local_time;
mod period;
mod presets;
mod profiles;
mod resolution;
mod rule;
mod session;

pub use bulk::{
    hours_for_all, hours_for_eu_equities, hours_for_us_equities, hours_map_eu_equities,
    hours_map_for, hours_map_us_equities,
};
pub use candle::{candle_end, candle_end_with, candle_start, time_end_of_day};
pub use exchange::Exchange;
pub use futures_profile::{
    FuturesSessionProfile, MarketHoursKey, hours_for_market_hours_key, session_profile,
};
pub use hours::MarketHours;
pub use presets::{hours_for_exchange, hours_for_exchange_as_of};
pub use profiles::{
    BLUE_OCEAN_EXTENDED, NYSE_TEXAS_EXTENDED, US_EQUITY_EXTENDED, US_EQUITY_REGULAR,
};
pub use resolution::CalendarResolution;
pub use rule::{SessionKind, SessionRule};
pub use session::{
    next_session_after, next_session_after_with, next_session_open_after, session_bounds,
    session_bounds_with,
};
