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

use super::query::{QueryContext, status, week};
use super::{Exchange, SessionKind, SessionRule};

/// Normal-week trading-hours definition.
///
/// Built-in values state their exchange, segment, or product-family scope in
/// the verification ledger. They do not capture holidays or products outside
/// that scope; consult the relevant contract specifications before trading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketHours {
    /// Which exchange these hours represent.
    pub exchange: Exchange,
    /// Exchange’s local time zone (used to interpret `SessionRule`s and handle DST).
    pub tz: Tz,
    /// Primary/pit ("regular") trading sessions (e.g., RTH for equities/futures).
    /// Uses Cow so callers can borrow from static tables without allocation.
    pub regular: Cow<'static, [SessionRule]>,
    /// Electronic/overnight and other non-regular sessions.
    /// Uses Cow so callers can borrow from static tables without allocation.
    pub extended: Cow<'static, [SessionRule]>,
    /// True if there is a distinct daily close (used for daily candle boundaries).
    pub has_daily_close: bool,
    /// True if the exchange has a true weekend close (used for weekly boundaries).
    pub has_weekend_close: bool,
}

impl MarketHours {
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
    /// Session existence follows the crate-wide contract (stated on the
    /// internal holiday hook in `local_time`): a same-day session on local day `D` exists iff
    /// `D` is not a holiday, and a wrap session opening on `D` exists iff
    /// neither `D` nor `D+1` is one. This predicate and
    /// [`session_bounds_with`](super::session_bounds_with) share the same
    /// containing-session resolver, so their answers cannot drift. Holidays are
    /// outside this normal-week model, and its internal policy returns `false`.
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

    /// True if `t` falls inside a daily maintenance break: a closed gap
    /// between two sessions whose **whole** close-to-reopen span is shorter
    /// than six hours.
    ///
    /// The gap is measured from the previous session close to the next open
    /// (across regular and extended sessions), so the entire break qualifies
    /// from its first closed instant — CME's 16:00–17:00 CT hour, ICE's
    /// two-hour 18:00–20:00 ET window, and CBOT grains' 13:20→19:00 CT
    /// afternoon alike. Longer closures never count: an equity venue's
    /// overnight, a weekend, and the run-up to a Sunday reopen are closed but
    /// not maintenance. Always-open venues are never in maintenance because
    /// they are never closed, and a profile with no sessions at all is never
    /// in maintenance because nothing reopens.
    ///
    /// This crate deliberately does not model breaks as explicit rules; the
    /// six-hour threshold is what separates the shipped schedules' longest
    /// intraday break (CBOT grains, 5h40) from their shortest overnight
    /// closure (SIX, 8h).
    #[must_use]
    pub fn is_maintenance(&self, t: DateTime<Utc>) -> bool {
        status::is_maintenance(&QueryContext::fixed(self), t)
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
    /// resolver; if `day` is the maximum representable `NaiveDate`, the window
    /// end saturates to the far future rather than failing.
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
