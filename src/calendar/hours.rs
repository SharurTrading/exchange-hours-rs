// SPDX-License-Identifier: MIT-0

//! [`MarketHours`] — a venue's rule sets plus the open/closed query surface.
//!
//! A `MarketHours` value is a *normal-week* schedule: a time zone, two
//! [`SessionRule`] sets, and two flags. It holds no holiday
//! overlay and no product-level exceptions, so every answer here is the
//! profile's explicitly stated venue, segment, or product-family scope for an
//! ordinary week.
//!
//! Two invariants bind every query in this module. Closes are **end-exclusive**,
//! so an instant exactly at a close is closed and adjacent sessions never
//! double-count. And a rule with `open_ssm >= close_ssm` **wraps** into the next
//! local day (equality is exactly one complete local-day span), so "is it
//! open?" must consult yesterday's rules as well as today's — every predicate
//! below does, and one that forgets is how overnight venues read as closed
//! after midnight.

use std::borrow::Cow;

use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Tz;

use super::exchange_calendar::CalendarSource;
use super::query::{QueryContext, status, week};
use super::{Exchange, SessionKind, SessionRule, SessionState};

/// Normal-week trading-hours definition.
///
/// Built-in values state their exchange, segment, or product-family scope in
/// the verification ledger. They do not capture holidays or products outside
/// that scope; consult the relevant contract specifications before trading.
/// Open-state predicates and session-bound queries traverse the borrowed rule
/// slices directly: they allocate nothing and take `O(rules)` work per call.
/// Forward-looking queries use a documented, bounded civil-day scan rather
/// than an unbounded search.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketHours {
    /// Which identity these hours represent.
    ///
    /// A venue-backed profile carries [`CalendarSource::Exchange`]; a
    /// product-family profile carries [`CalendarSource::MarketHoursKey`]. The
    /// field previously held a bare [`Exchange`] and was set to
    /// [`Exchange::Unknown`] for every key-backed profile, which collided with
    /// the crate's own 24x7-fallback sentinel.
    pub source: CalendarSource,
    /// Exchange’s local time zone (used to interpret `SessionRule`s and handle DST).
    pub tz: Tz,
    /// Primary/pit ("regular") trading sessions (e.g., RTH for equities/futures).
    /// Uses Cow so callers can borrow from static tables without allocation.
    pub regular: Cow<'static, [SessionRule]>,
    /// Electronic/overnight and other non-regular sessions.
    /// Uses Cow so callers can borrow from static tables without allocation.
    pub extended: Cow<'static, [SessionRule]>,
    /// Order-entry-only phases.
    ///
    /// Pre-open queues and post-close order windows in which orders may be
    /// entered, amended or cancelled but **no trade can match**. These are
    /// deliberately separate from `extended`, which holds genuinely tradeable
    /// electronic and overnight sessions: `is_open` counts `regular` and
    /// `extended` only, so a caller asking whether a market is open is asking
    /// whether a trade can print. Use [`MarketHours::is_accepting_orders`] for
    /// the order-entry question.
    pub order_entry: Cow<'static, [SessionRule]>,
    /// True if there is a distinct daily close (used for daily candle boundaries).
    pub has_daily_close: bool,
    /// True if this fixed profile has a true weekend close used for generic
    /// weekly boundaries.
    ///
    /// An identity-aware calendar may expose a sourced product-family weekly
    /// boundary without a long weekend shutdown. CME cryptocurrency is the
    /// built-in example; use `calendar_for_market_hours_key` for that result.
    pub has_weekend_close: bool,
}

impl MarketHours {
    /// Builds a profile for `source`.
    ///
    /// The struct is `#[non_exhaustive]`, so this is the constructor available
    /// to callers outside the crate. `source` accepts an [`Exchange`], a
    /// [`MarketHoursKey`](crate::MarketHoursKey), or a [`CalendarSource`]
    /// directly.
    ///
    /// The built-in tables are reached through `hours_for_exchange`,
    /// `hours_for_market_hours_key`, and their `as_of` variants; this exists for
    /// callers modelling a venue the crate does not ship.
    #[must_use]
    pub fn new(
        source: impl Into<CalendarSource>,
        tz: Tz,
        regular: Cow<'static, [SessionRule]>,
        extended: Cow<'static, [SessionRule]>,
        order_entry: Cow<'static, [SessionRule]>,
        has_daily_close: bool,
        has_weekend_close: bool,
    ) -> Self {
        Self {
            source: source.into(),
            tz,
            regular,
            extended,
            order_entry,
            has_daily_close,
            has_weekend_close,
        }
    }

    /// True when orders may be entered, amended or cancelled at `t`.
    ///
    /// This is **not** the same question as [`MarketHours::is_open`]. It is true
    /// during order-entry-only phases such as pre-open queues and post-close
    /// order windows, where nothing can match, and also whenever a tradeable
    /// session is running. A venue that publishes no order-entry phase returns
    /// the same answer as `is_open`.
    #[must_use]
    pub fn is_accepting_orders(&self, t: DateTime<Utc>) -> bool {
        self.is_open(t) || self.is_order_entry_only(t)
    }

    /// True when `t` falls in an order-entry-only phase and no trade can match.
    ///
    /// False whenever a tradeable session is running, so the two states are
    /// mutually exclusive: a caller can branch on them without ordering care.
    #[must_use]
    pub fn is_order_entry_only(&self, t: DateTime<Utc>) -> bool {
        !self.is_open(t)
            && self
                .order_entry_view()
                .is_open_with(t, SessionKind::Regular)
    }

    /// The order-entry phases viewed as a schedule in their own right.
    ///
    /// Reuses the one query engine rather than re-deriving wrap and DST
    /// handling for a second rule slice. Borrowed rule slices stay borrowed, so
    /// a static profile allocates nothing here.
    fn order_entry_view(&self) -> Self {
        Self {
            source: self.source,
            tz: self.tz,
            regular: self.order_entry.clone(),
            extended: Cow::Borrowed(&[]),
            order_entry: Cow::Borrowed(&[]),
            has_daily_close: self.has_daily_close,
            has_weekend_close: self.has_weekend_close,
        }
    }

    /// The identity these hours represent.
    #[must_use]
    pub const fn source(&self) -> CalendarSource {
        self.source
    }

    /// The venue identity, or `None` when these hours are product-family keyed.
    ///
    /// Mirrors [`ExchangeCalendar::exchange`](crate::ExchangeCalendar::exchange)
    /// so a value and the calendar that produced it answer identically.
    #[must_use]
    pub const fn exchange(&self) -> Option<Exchange> {
        match self.source {
            CalendarSource::Exchange(exchange) => Some(exchange),
            CalendarSource::MarketHoursKey(_) => None,
        }
    }

    /// The product-family identity, or `None` when these hours are venue keyed.
    #[must_use]
    pub const fn market_hours_key(&self) -> Option<crate::MarketHoursKey> {
        match self.source {
            CalendarSource::MarketHoursKey(key) => Some(key),
            CalendarSource::Exchange(_) => None,
        }
    }

    /// Returns the number of distinct scheduled open seconds in this profile's
    /// normal week, consulting regular and extended sessions together.
    ///
    /// Overlapping and adjacent rules are unioned before their durations are
    /// summed, and rules that wrap across the Sunday/Monday boundary are split
    /// at that boundary. Holiday and daylight-saving overlays are intentionally
    /// excluded because [`MarketHours`] is a normal-week profile.
    #[must_use]
    pub fn normal_week_open_seconds(&self) -> u64 {
        week::fixed_normal_week_open_seconds(self)
    }

    /// True if **any** (regular or extended) session is open at `t`.
    #[must_use]
    pub fn is_open(&self, t: DateTime<Utc>) -> bool {
        self.is_open_with(t, SessionKind::Both)
    }

    /// True if a session of the requested kind is open at `t`.
    ///
    /// This predicate and [`session_bounds_with`](super::session_bounds_with)
    /// share the same containing-session resolver, so their answers cannot
    /// drift. This fixed profile is a normal-week grid and has no holiday
    /// overlay; use a date-aware calendar with a caller-supplied
    /// [`DayPolicy`](super::DayPolicy) when day-level overrides are required.
    #[must_use]
    pub fn is_open_with(&self, t: DateTime<Utc>, kind: SessionKind) -> bool {
        status::is_open_with(&QueryContext::fixed(self), t, kind)
    }

    /// True if a **regular** (primary/RTH) session is open at `t`.
    ///
    /// Shorthand for [`is_open_with`](Self::is_open_with) with
    /// [`SessionKind::Regular`]; extended/overnight sessions are ignored.
    #[must_use]
    pub fn is_open_regular(&self, t: DateTime<Utc>) -> bool {
        self.is_open_with(t, SessionKind::Regular)
    }

    /// True if an **extended** (electronic/overnight/auction) session is open at `t`.
    ///
    /// Shorthand for [`is_open_with`](Self::is_open_with) with
    /// [`SessionKind::Extended`]; regular sessions are ignored.
    #[must_use]
    pub fn is_open_extended(&self, t: DateTime<Utc>) -> bool {
        self.is_open_with(t, SessionKind::Extended)
    }

    /// True if `t` falls inside a daily maintenance break.
    ///
    /// The gap is measured from the previous session close to the next open
    /// (across regular and extended sessions). It normally qualifies only when
    /// it lies between different trade dates in the same ISO week and the
    /// complete close-to-reopen span is at most four elapsed hours. A profile
    /// that explicitly has no weekend close also retains operator-designated
    /// breaks of that length inside one trade date. Other same-trade-date gaps
    /// are [`SessionState::Halt`]; longer afternoon, overnight, and weekend
    /// closures are [`SessionState::Closed`].
    ///
    /// This crate deliberately derives breaks from adjacent sourced sessions
    /// rather than inserting synthetic maintenance rules.
    #[must_use]
    pub fn is_maintenance(&self, t: DateTime<Utc>) -> bool {
        status::is_maintenance(&QueryContext::fixed(self), t)
    }

    /// Returns one mutually exclusive open, halt, maintenance, or closed state.
    ///
    /// Maintenance is normally a complete inter-trade-date gap of at most four
    /// elapsed hours within one ISO week. A profile with no weekend close also
    /// retains operator-designated short maintenance inside one trade date;
    /// longer closures are [`SessionState::Closed`].
    #[must_use]
    pub fn session_state(&self, t: DateTime<Utc>) -> SessionState {
        status::session_state(&QueryContext::fixed(self), t)
    }

    /// Return true iff the market is closed for the entire **calendar day** `day`
    /// interpreted in `calendar_tz`. This checks whether *any* session (of `kind`)
    /// intersects the `[day@00:00, next_day@00:00)` window.
    ///
    /// `calendar_tz` need not be the exchange time zone: a venue can be open for
    /// part of a day defined in another zone, so the answer is decided strictly
    /// by session overlap with that window, never by a weekday shortcut.
    ///
    /// A skipped local midnight (some zones start DST at 00:00) resolves to the
    /// first representable instant of the day via the crate's shared local-time
    /// resolver. A wholly skipped civil date has an empty window and is closed.
    /// If `day` is the maximum representable `NaiveDate`, the window end
    /// saturates to the far future rather than failing.
    #[must_use]
    pub fn is_closed_all_day_in_calendar(
        &self,
        day: NaiveDate,
        calendar_tz: Tz,
        kind: SessionKind,
    ) -> bool {
        status::is_closed_all_day_in_calendar(&QueryContext::fixed(self), day, calendar_tz, kind)
    }

    /// Convenience: interpret the date in the **exchange TZ** (what your old
    /// `is_closed_all_day_on` did).
    #[inline]
    #[must_use]
    pub fn is_closed_all_day_on(&self, local_day: NaiveDate, kind: SessionKind) -> bool {
        self.is_closed_all_day_in_calendar(local_day, self.tz, kind)
    }

    /// Convenience: starting from a UTC timestamp, use a chosen calendar TZ to define “the day”.
    #[inline]
    #[must_use]
    pub fn is_closed_all_day_at(
        &self,
        ts_utc: DateTime<Utc>,
        calendar_tz: Tz,
        kind: SessionKind,
    ) -> bool {
        status::is_closed_all_day_at(&QueryContext::fixed(self), ts_utc, calendar_tz, kind)
    }
}
