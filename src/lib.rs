// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! Exchange trading hours, session boundaries, and calendar-aware bar boundaries.
//!
//! `exchange-hours` answers three questions about a `chrono::DateTime<Utc>`
//! instant: is this venue open, what are the bounds of the containing (or next)
//! trading session, and where does a bar of a given [`CalendarResolution`] start
//! and end without spanning a closed period. It holds no runtime state, performs
//! no I/O, and never returns a local time — an exchange's own zone is used only
//! internally, to interpret session rules and resolve DST deterministically.
//!
//! # Quick start
//!
//! CME equity-index futures trade 17:00→16:00 CT with a one-hour daily break,
//! and RTH runs 08:30–15:15 CT:
//!
//! ```
//! use chrono::{TimeZone, Utc};
//! use chrono_tz::US;
//! use exchange_hours::{
//!     CalendarResolution, Exchange, candle_end, hours_for_exchange, next_session_after,
//!     session_bounds,
//! };
//!
//! let ct = |y, m, d, hh, mm| {
//!     US::Central
//!         .with_ymd_and_hms(y, m, d, hh, mm, 0)
//!         .single()
//!         .expect("valid CT instant")
//!         .with_timezone(&Utc)
//! };
//!
//! let hours = hours_for_exchange(Exchange::Cme);
//!
//! // Monday mid-morning sits inside the regular session. Boundary queries
//! // return `Option`: `None` means no matching session exists in the bounded
//! // search horizon (e.g. a pre-go-live date) or the interval is zero.
//! let monday_10am = ct(2026, 4, 20, 10, 0);
//! assert!(hours.is_open_regular(monday_10am));
//! let (open, close) = session_bounds(&hours, monday_10am).expect("CME trades this week");
//! assert_eq!(open, ct(2026, 4, 20, 8, 30));
//! assert_eq!(close, ct(2026, 4, 20, 15, 15)); // end-exclusive
//!
//! // 16:30 CT is the daily maintenance gap: closed, but reopening within 90 min.
//! let monday_evening = ct(2026, 4, 20, 16, 30);
//! assert!(!hours.is_open(monday_evening));
//! assert!(hours.is_maintenance(monday_evening));
//!
//! // After Friday's close the next session is Sunday evening, not Saturday.
//! let friday_after_close = ct(2026, 4, 24, 16, 30);
//! let (next_open, _) = next_session_after(&hours, friday_after_close).expect("reopens Sunday");
//! assert_eq!(next_open, ct(2026, 4, 26, 17, 0));
//!
//! // Bar boundaries follow the same rules: a daily bar closes at the venue's
//! // session close, not at midnight.
//! let daily_close = candle_end(&hours, monday_10am, CalendarResolution::Daily);
//! assert_eq!(daily_close, Some(ct(2026, 4, 20, 16, 0)));
//! ```
//!
//! # Model
//!
//! A venue is a [`MarketHours`] value: a time zone plus `regular` and `extended`
//! [`SessionRule`] sets, each rule a weekday mask and an open/close pair in
//! seconds since local midnight. Three conventions decide every answer —
//! weekdays are Monday = 0 through Sunday = 6; closes are **end-exclusive**, so
//! the instant equal to a close is closed and adjacent sessions never overlap;
//! and `open_ssm > close_ssm` marks a session that **wraps** past local midnight,
//! which is how overnight and Globex-style trading days are expressed.
//!
//! Ambiguous and skipped local times are resolved deterministically: opens take
//! the earliest valid mapping, closes the latest, and a wall-clock inside a
//! spring-forward gap snaps to the first representable instant after it.
//!
//! # Scope
//!
//! This is a **normal-week** calendar. Holidays, early closes, half-days, and
//! product-level variations are deliberately out of scope — the internal
//! `is_holiday` hook is a stub that always returns `false` — so the profiles are
//! pragmatic exchange-level defaults, not contract-exact schedules. Verify
//! specific contract rules before trading on them.
//!
//! # Entry points
//!
//! - [`hours_for_exchange`] / [`hours_for_exchange_as_of`] — venue → hours, now
//!   or at a point in time.
//! - [`session_profile`] / [`hours_for_market_hours_key`] — the shared futures
//!   profiles, addressed by [`MarketHoursKey`] (product family) rather than by
//!   venue.
//! - [`MarketHours::is_open`] and friends, [`session_bounds`],
//!   [`next_session_after`], [`candle_start`], [`candle_end`].

#![forbid(unsafe_code)]

mod calendar;

pub use calendar::*;
