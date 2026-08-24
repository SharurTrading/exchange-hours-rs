// SPDX-License-Identifier: MIT-0

//! Exchange trading hours, session boundaries, and calendar-aware bar boundaries.
//!
//! `exchange-hours` answers three questions about a `chrono::DateTime<Utc>`
//! instant: is this venue or product family open, what are the bounds of the
//! containing (or next) trading session, and where does a bar of a given
//! [`CalendarResolution`] start and end without spanning a closed period. It
//! holds no runtime state, performs no I/O, and never returns a local time — a
//! schedule's own zone is used only internally, to interpret session rules and
//! resolve DST deterministically.
//!
//! # Quick start
//!
//! CME equity-index futures match 17:00→16:00 CT and accept weekday orders from
//! 16:45; RTH runs 08:30–15:15 CT:
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
//! // search horizon (for example, on a pre-go-live date).
//! let monday_10am = ct(2026, 4, 20, 10, 0);
//! assert!(hours.is_open_regular(monday_10am));
//! let (open, close) = session_bounds(&hours, monday_10am).expect("CME trades this week");
//! assert_eq!(open, ct(2026, 4, 20, 8, 30));
//! assert_eq!(close, ct(2026, 4, 20, 15, 15)); // end-exclusive
//!
//! // 16:30 CT is the daily maintenance break: closed, inside an inter-trade-date
//! // gap (16:00→16:45) no longer than the documented four-hour bound.
//! let monday_evening = ct(2026, 4, 20, 16, 30);
//! assert!(!hours.is_open(monday_evening));
//! assert!(hours.is_maintenance(monday_evening));
//!
//! // After Friday's close the next SESSION is Sunday's 17:00 matching open, not
//! // Saturday. Sunday's 16:00 Pre-Open accepts orders but matches nothing, so it
//! // is an order-entry phase rather than a session.
//! let friday_after_close = ct(2026, 4, 24, 16, 30);
//! let (next_open, _) = next_session_after(&hours, friday_after_close).expect("reopens Sunday");
//! assert_eq!(next_open, ct(2026, 4, 26, 17, 0));
//! assert!(hours.is_order_entry_only(ct(2026, 4, 26, 16, 0)));
//! assert!(!hours.is_open(ct(2026, 4, 26, 16, 0)));
//!
//! // Bar boundaries follow the same rules: a daily bar closes at the venue's
//! // session close, not at midnight.
//! let daily_close = candle_end(&hours, monday_10am, CalendarResolution::Daily);
//! assert_eq!(daily_close, Some(ct(2026, 4, 20, 16, 0)));
//! ```
//!
//! # Model
//!
//! A fixed venue snapshot is a [`MarketHours`] value: a time zone plus `regular`
//! `extended` and `order_entry` [`SessionRule`] sets, each rule a weekday mask
//! and an open/close pair in seconds since local midnight. `regular` and
//! `extended` are tradeable, so `is_open` means a trade can print; `order_entry`
//! holds pre-open queues and post-close order windows where nothing matches. [`ExchangeCalendar`] is the
//! date-aware surface for either an [`Exchange`] venue identity or a
//! [`MarketHoursKey`] product family; [`CalendarSource`] reports which identity
//! it carries. [`PolicyCalendar`] applies a caller's [`DayPolicy`] overrides to
//! the same query surface without changing the underlying profile;
//! [`StaticDayPolicy`] is the validated hard-coded record format for those
//! boundary-level overrides.
//! Three conventions decide every answer —
//! weekdays are Monday = 0 through Sunday = 6; closes are **end-exclusive**, so
//! the instant equal to a close is closed and adjacent sessions never overlap;
//! and `open_ssm >= close_ssm` marks a session that **wraps** into the next local
//! day. Equal endpoints encode one complete local-day span, which supports
//! exact 24-hour sessions without making a venue always open.
//!
//! Ambiguous and skipped local times are resolved deterministically: opens take
//! the earliest valid mapping, closes the latest, and a wall-clock inside a
//! spring-forward gap snaps to the first representable instant after it.
//!
//! Built-in fixed status/boundary/candle queries and date-aware
//! status/boundary/trade-date/candle queries allocate nothing and inspect
//! bounded static rule sets. [`ExchangeCalendar`] is
//! `Copy + Send + Sync + 'static`; work performed inside a caller's
//! [`DayPolicy`] is outside that guarantee.
//!
//! # Scope
//!
//! Built-in profiles are **normal-week** calendars and contain no holiday or
//! half-day data. A caller can overlay sourced closed trade dates, early final
//! closes, and late first opens through [`DayPolicy`]. Complex special days
//! that replace or split phases require a complete exception-session provider,
//! not scalar boundary clipping. Product-level variations
//! outside a profile remain out of scope. In particular, this crate does not
//! map symbols, roots, product codes, or MICs to [`MarketHoursKey`] values; a
//! caller's instrument catalog must select the exact supported family.
//!
//! # Entry points
//!
//! - [`hours_for_exchange`] / [`hours_for_exchange_as_of`] — venue → a
//!   default fixed snapshot or the snapshot at a point in time.
//! - [`calendar_for_exchange`] — a date-aware calendar that reselects the
//!   applicable profile while scanning sessions and bar boundaries.
//! - [`session_profile`] / [`hours_for_market_hours_key`] — fixed-current
//!   futures profiles addressed by [`MarketHoursKey`] (product family) rather
//!   than by venue; [`hours_for_market_hours_key_as_of`] selects a sourced
//!   dated snapshot.
//! - [`calendar_for_market_hours_key`] — the same date-aware calendar surface
//!   for a product family. Use [`ExchangeCalendar::source`] to retain its exact
//!   [`CalendarSource`] identity. Identity-specific topology, including CME
//!   cryptocurrency's multi-day weekend bounds and following-business-day
//!   assignment, is available here; a detached fixed snapshot preserves exact
//!   open/closed state but not those coalesced bounds or trade dates.
//! - [`ExchangeCalendar::with_day_policy`] — a borrowed [`PolicyCalendar`]
//!   overlay for caller-supplied closed days, early closes, and late opens.
//! - [`ExchangeCalendar::trade_date`] and [`ExchangeCalendar::session_state`]
//!   — one containing-session trade date and one mutually exclusive
//!   [`SessionState`] classification per instant. An always-open profile has no
//!   final close, so its trade date is `None`.
//! - [`MarketHours::is_open`] and friends, [`session_bounds`],
//!   [`next_session_after`], [`candle_start`], [`candle_end`].

#![forbid(unsafe_code)]

mod calendar;

pub use calendar::*;
