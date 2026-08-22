// SPDX-License-Identifier: MIT-0

//! The two concrete profile sources consumed by the query engine.

use chrono::{DateTime, Datelike, Duration, NaiveDate, Timelike, Utc, Weekday};
use chrono_tz::Tz;

use crate::calendar::exchange_calendar::ExchangeCalendar;
use crate::calendar::hours::MarketHours;
use crate::calendar::local_time::{mk_local_close, mk_local_open};
use crate::calendar::policy::DayPolicy;
use crate::calendar::rule::{SessionKind, SessionRule};
use crate::calendar::{CalendarResolution, CalendarSource, MarketHoursKey};

use super::candles;

const OPEN_DAY_ANCHOR_SSM: u32 = 12 * 3_600;
const TRADE_DATE_LOOKAHEAD_DAYS: usize = 14;

#[derive(Clone, Copy)]
enum ProfileSource<'a> {
    Fixed(&'a MarketHours),
    DateAware(ExchangeCalendar),
}

/// A per-query schedule source with its invariant venue zone cached once.
#[derive(Clone, Copy)]
pub(in crate::calendar) struct QueryContext<'a> {
    source: ProfileSource<'a>,
    tz: Tz,
    policy: Option<&'a dyn DayPolicy>,
}

pub(super) enum ResolvedHours<'a> {
    Borrowed(&'a MarketHours),
    Selected(MarketHours),
}

impl AsRef<MarketHours> for ResolvedHours<'_> {
    fn as_ref(&self) -> &MarketHours {
        match self {
            Self::Borrowed(hours) => hours,
            Self::Selected(hours) => hours,
        }
    }
}

impl<'a> QueryContext<'a> {
    pub(in crate::calendar) fn fixed(hours: &'a MarketHours) -> Self {
        Self {
            source: ProfileSource::Fixed(hours),
            tz: hours.tz,
            policy: None,
        }
    }

    pub(in crate::calendar) fn date_aware(calendar: ExchangeCalendar) -> Self {
        Self {
            source: ProfileSource::DateAware(calendar),
            tz: calendar.tz(),
            policy: None,
        }
    }

    pub(in crate::calendar) fn with_policy(
        calendar: ExchangeCalendar,
        policy: &'a dyn DayPolicy,
    ) -> Self {
        Self {
            source: ProfileSource::DateAware(calendar),
            tz: calendar.tz(),
            policy: Some(policy),
        }
    }

    pub(super) const fn without_policy(self) -> Self {
        Self {
            source: self.source,
            tz: self.tz,
            policy: None,
        }
    }

    pub(super) const fn tz(self) -> Tz {
        self.tz
    }

    /// Assigns a resolved session block to its venue-local trade date.
    ///
    /// Most profiles use the local date of the final close. CME's 24/7
    /// cryptocurrency schedule is the sourced exception: both weekend blocks
    /// after Friday's daily close carry the following business date, even
    /// though Saturday maintenance separates them. Only a key-backed calendar
    /// has enough identity to apply that product-family convention.
    pub(super) fn trade_date_for_bounds(
        self,
        open: DateTime<Utc>,
        close: DateTime<Utc>,
    ) -> NaiveDate {
        let default = close.with_timezone(&self.tz).date_naive();
        let ProfileSource::DateAware(calendar) = self.source else {
            return default;
        };
        if !matches!(
            calendar.source(),
            CalendarSource::MarketHoursKey(MarketHoursKey::GlobexCryptocurrency)
        ) {
            return default;
        }

        let local_open = open.with_timezone(&self.tz);
        let days_to_monday = match local_open.weekday() {
            Weekday::Fri if local_open.time().num_seconds_from_midnight() >= 16 * 3_600 => 3,
            Weekday::Sat => 2,
            Weekday::Sun => 1,
            _ => 0,
        };
        let nominal = if days_to_monday == 0 {
            default
        } else {
            local_open
                .date_naive()
                .checked_add_signed(Duration::days(days_to_monday))
                .unwrap_or(default)
        };

        // The permanent 24/7 schedule assigns holiday/weekend trading to the
        // following business day. A closed day in the caller's policy is
        // skipped rather than deleting the connected trading block. Legacy
        // five-day profiles retain ordinary closed-trade-date behavior.
        let Some(policy) = self.policy else {
            return nominal;
        };
        if self.has_weekend_close_at(open) {
            return nominal;
        }
        let mut candidate = nominal;
        for _ in 0..=TRADE_DATE_LOOKAHEAD_DAYS {
            if !matches!(candidate.weekday(), Weekday::Sat | Weekday::Sun)
                && !policy.is_closed(candidate)
            {
                return candidate;
            }
            let Some(next) = candidate.succ_opt() else {
                return nominal;
            };
            candidate = next;
        }
        nominal
    }

    /// Returns whether this identified calendar joins storage-only rule pieces.
    ///
    /// This is intentionally an identity capability, not a shape heuristic:
    /// adjacent rules are real phase boundaries for several other profiles.
    pub(super) const fn joins_adjacent_same_kind(self) -> bool {
        let ProfileSource::DateAware(calendar) = self.source else {
            return false;
        };
        matches!(
            calendar.source(),
            CalendarSource::MarketHoursKey(MarketHoursKey::GlobexCryptocurrency)
        )
    }

    pub(super) fn has_daily_close_at(self, instant: chrono::DateTime<Utc>) -> bool {
        match self.source {
            ProfileSource::Fixed(hours) => hours.has_daily_close,
            ProfileSource::DateAware(calendar) => calendar.hours_at(instant).has_daily_close,
        }
    }

    pub(super) fn has_weekend_close_at(self, instant: chrono::DateTime<Utc>) -> bool {
        match self.source {
            ProfileSource::Fixed(hours) => hours.has_weekend_close,
            ProfileSource::DateAware(calendar) => calendar.hours_at(instant).has_weekend_close,
        }
    }

    /// Returns whether this source exposes a real weekly candle boundary.
    ///
    /// Most profiles use their explicit weekend-close flag. CME's key-backed
    /// cryptocurrency calendar is the sourced exception: its continuous week
    /// has no long weekend shutdown, but Friday 16:00 CT remains the final
    /// close of that trade-date week before Monday trading starts at 16:02.
    /// The identity-erased fixed snapshot cannot apply that convention.
    pub(super) fn has_weekly_close_at(self, instant: chrono::DateTime<Utc>) -> bool {
        if self.has_weekend_close_at(instant) {
            return true;
        }
        let ProfileSource::DateAware(calendar) = self.source else {
            return false;
        };
        matches!(
            calendar.source(),
            CalendarSource::MarketHoursKey(MarketHoursKey::GlobexCryptocurrency)
        ) && self.has_daily_close_at(instant)
    }

    /// Selects a profile by the venue-local day on which a session opens.
    pub(super) fn profile_for_open_day(self, day: NaiveDate) -> ResolvedHours<'a> {
        match self.source {
            ProfileSource::Fixed(hours) => ResolvedHours::Borrowed(hours),
            ProfileSource::DateAware(calendar) => {
                let anchor = mk_local_open(self.tz, day, OPEN_DAY_ANCHOR_SSM).with_timezone(&Utc);
                ResolvedHours::Selected(calendar.hours_at(anchor))
            }
        }
    }
}

/// Resolves one scheduled occurrence and rejects civil-time collapses.
pub(super) fn resolve_rule_bounds(
    context: &QueryContext<'_>,
    open_day: NaiveDate,
    rule: &SessionRule,
) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    let close_day = if rule.wraps_to_next_day() {
        open_day.succ_opt()?
    } else {
        open_day
    };
    let tz = context.tz();
    let raw_open = mk_local_open(tz, open_day, rule.open_ssm).with_timezone(&Utc);
    let raw_close = mk_local_close(tz, close_day, rule.close_ssm).with_timezone(&Utc);
    if raw_open >= raw_close {
        return None;
    }
    let Some(policy) = context.policy else {
        return Some((raw_open, raw_close));
    };
    if !context.has_daily_close_at(raw_open) {
        // Without a final daily close this profile has no trade-date identity.
        // Applying a policy to its storage-rule close would invent a date and
        // could close an always-open market one civil day early.
        return Some((raw_open, raw_close));
    }

    let baseline = context.without_policy();
    let final_close = candles::candle_end_with(
        &baseline,
        raw_open,
        CalendarResolution::Daily,
        SessionKind::Both,
    )
    .unwrap_or(raw_close);
    let first_open = candles::candle_start_with(
        &baseline,
        raw_open,
        CalendarResolution::Daily,
        SessionKind::Both,
    )
    .unwrap_or(raw_open);
    let trade_date = context.trade_date_for_bounds(raw_open, final_close);
    if policy.is_closed(trade_date) {
        return None;
    }

    let mut open = raw_open;
    let mut close = raw_close;
    if let Some(ssm) = policy.early_close_ssm(trade_date) {
        if ssm > 86_400 {
            return None;
        }
        let cutoff = mk_local_close(tz, trade_date, ssm).with_timezone(&Utc);
        close = close.min(cutoff);
    }
    if let Some(ssm) = policy.late_open_ssm(trade_date) {
        if ssm >= 86_400 {
            return None;
        }
        let first_local = first_open.with_timezone(&tz);
        let first_day = first_local.date_naive();
        let first_ssm = first_local.time().num_seconds_from_midnight();
        let cutoff_day = if first_day < trade_date && ssm >= first_ssm {
            first_day
        } else {
            trade_date
        };
        let cutoff = mk_local_open(tz, cutoff_day, ssm).with_timezone(&Utc);
        open = open.max(cutoff);
    }
    (open < close).then_some((open, close))
}

pub(super) fn rules(hours: &MarketHours, kind: SessionKind) -> impl Iterator<Item = &SessionRule> {
    let (regular, extended): (&[SessionRule], &[SessionRule]) = match kind {
        SessionKind::Regular => (&hours.regular, &[]),
        SessionKind::Extended => (&[], &hours.extended),
        SessionKind::Both => (&hours.regular, &hours.extended),
    };
    regular.iter().chain(extended)
}
