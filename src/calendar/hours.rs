// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! [`MarketHours`] — a venue's rule sets plus the open/closed query surface.
//!
//! A `MarketHours` value is a *normal-week* schedule: a time zone, two
//! [`SessionRule`](super::SessionRule) sets, and two flags. It holds no holiday
//! overlay and no product-level exceptions, so every answer here is the
//! exchange-level default for an ordinary week.
//!
//! Two invariants bind every query in this module. Closes are **end-exclusive**,
//! so an instant exactly at a close is closed and adjacent sessions never
//! double-count. And a rule with `open_ssm > close_ssm` **wraps** past local
//! midnight, so "is it open?" must consult yesterday's rules as well as today's
//! — every predicate below does, and one that forgets is how overnight venues
//! read as closed after midnight.

use std::borrow::Cow;

use chrono::{DateTime, Datelike, Duration, LocalResult, NaiveDate, TimeZone, Timelike, Utc};
use chrono_tz::Tz;

use super::rule::{SECONDS_PER_NORMAL_WEEK, normal_week_rule_intervals};
use super::session::{next_session_after, next_session_after_with};
use super::{Exchange, SessionKind, SessionRule};

/// Exchange-level trading hours definition.
///
/// This is a pragmatic calendar describing typical hours for an exchange. It may not
/// capture product-specific exceptions; consult contract specs for exact rules.
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
    /// Iterates the rule set(s) named by `kind` in `regular`-then-`extended`
    /// order. Order is stable but carries no meaning: every consumer either
    /// tests all rules or takes a min/max over them.
    #[inline]
    pub(crate) fn iter_rules(&self, kind: SessionKind) -> impl Iterator<Item = &SessionRule> + '_ {
        let (regular, extended): (&[SessionRule], &[SessionRule]) = match kind {
            SessionKind::Regular => (&self.regular, &[]),
            SessionKind::Extended => (&[], &self.extended),
            SessionKind::Both => (&self.regular, &self.extended),
        };
        regular.iter().chain(extended.iter())
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
        let mut intervals = self
            .iter_rules(SessionKind::Both)
            .flat_map(normal_week_rule_intervals)
            .collect::<Vec<_>>();
        intervals.sort_unstable();

        let mut merged = Vec::<(u64, u64)>::new();
        for (start, end) in intervals {
            if let Some((_, previous_end)) = merged.last_mut()
                && start <= *previous_end
            {
                *previous_end = (*previous_end).max(end);
            } else {
                merged.push((start, end));
            }
        }
        merged
            .into_iter()
            .map(|(start, end)| end.saturating_sub(start))
            .sum::<u64>()
            .min(SECONDS_PER_NORMAL_WEEK)
    }

    /// True if **any** (regular or extended) session is open at `t`.
    #[must_use]
    pub fn is_open(&self, t: DateTime<Utc>) -> bool {
        self.is_open_with(t, SessionKind::Both)
    }

    /// True if a session of the requested kind is open at `t`.
    #[must_use]
    pub fn is_open_with(&self, t: DateTime<Utc>, kind: SessionKind) -> bool {
        let local = t.with_timezone(&self.tz);
        // Holidays are modeled via status; exchange-level defaults do not gate by holidays.
        let w_today = local.weekday().num_days_from_monday() as usize;
        let ssm = local.num_seconds_from_midnight();

        // Direct rule evaluation (no exchange-specific hard gates; breaks are modeled as rules)
        if self.iter_rules(kind).any(|r| {
            if !r.days[w_today] {
                return false;
            }
            if r.open_ssm <= r.close_ssm {
                ssm >= r.open_ssm && ssm < r.close_ssm
            } else {
                // Wrap: today's instance of the rule contributes only its open
                // side. The close belongs to *yesterday's* instance, which the
                // scan below evaluates against yesterday's weekday — testing
                // `ssm < close_ssm` here would report the venue open before its
                // own open on any day whose predecessor ran no session.
                ssm >= r.open_ssm
            }
        }) {
            return true;
        }

        // Yesterday's wrap may spill into today unless yesterday was a holiday.
        let yday_date = local.date_naive() - Duration::days(1);
        let yday = yday_date.weekday().num_days_from_monday() as usize;
        self.iter_rules(kind).any(|r| {
            if r.open_ssm <= r.close_ssm {
                return false;
            }
            if !r.days[yday] {
                return false;
            }
            ssm < r.close_ssm
        })
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

    /// True if `t` falls in a short pre-open "maintenance" gap.
    ///
    /// Heuristic: the market is currently closed **and** the next session opens
    /// within 90 minutes (across regular and extended sessions). This captures
    /// daily maintenance breaks such as CME's 16:00–17:00 CT window without
    /// modeling them as explicit rules. Always-open venues are never in
    /// maintenance because they are never closed.
    #[must_use]
    pub fn is_maintenance(&self, t: DateTime<Utc>) -> bool {
        // “Maintenance” := closed now but next session is within ~90 minutes.
        if self.is_open(t) {
            return false;
        }
        let (open, _close) = next_session_after(self, t);
        (open - t) <= chrono::Duration::minutes(90)
    }

    /// Return true iff the market is closed for the entire **calendar day** `day`
    /// interpreted in `calendar_tz`. This checks whether *any* session (of `kind`)
    /// intersects the `[day@00:00, next_day@00:00)` window.
    ///
    /// `calendar_tz` need not be the exchange time zone: a venue can be open for
    /// part of a day defined in another zone, so the answer is decided strictly
    /// by session overlap with that window, never by a weekday shortcut.
    ///
    /// # Panics
    ///
    /// Panics only if Chrono cannot resolve the fallback local time used for an
    /// exceptional midnight DST gap, or if `day` is the maximum representable
    /// `NaiveDate` and therefore has no successor.
    #[must_use]
    pub fn is_closed_all_day_in_calendar(
        &self,
        day: NaiveDate,
        calendar_tz: Tz,
        kind: SessionKind,
    ) -> bool {
        // Convert the calendar day's bounds to UTC, DST-safe.
        fn midnight_utc_for(tz: Tz, d: NaiveDate) -> DateTime<Utc> {
            match tz.with_ymd_and_hms(d.year(), d.month(), d.day(), 0, 0, 0) {
                LocalResult::Single(dt) => dt.with_timezone(&Utc),
                LocalResult::Ambiguous(a, b) => a.min(b).with_timezone(&Utc),
                LocalResult::None => {
                    // Extremely rare at midnight; fallback to 01:00 local.
                    tz.with_ymd_and_hms(d.year(), d.month(), d.day(), 1, 0, 0)
                        .single()
                        .expect("resolvable local time")
                        .with_timezone(&Utc)
                }
            }
        }

        let start_utc = midnight_utc_for(calendar_tz, day);
        let end_utc = midnight_utc_for(calendar_tz, day.succ_opt().expect("valid next day"));

        // NOTE: Do not short-circuit on exchange-local holidays here; a calendar
        // day in another TZ can still overlap valid trading. Decide strictly by overlap.

        // If any session is active at window start → not closed all day.
        if self.is_open_with(start_utc, kind) {
            return false;
        }

        // Otherwise, check the *next* session after the window start; if it opens
        // before the window ends, the day is not fully closed.
        let (next_open, _next_close) = next_session_after_with(kind, self, start_utc);
        next_open >= end_utc
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
        let day = ts_utc.with_timezone(&calendar_tz).date_naive();
        self.is_closed_all_day_in_calendar(day, calendar_tz, kind)
    }
}
