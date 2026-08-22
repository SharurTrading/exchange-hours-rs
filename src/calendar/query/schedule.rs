// SPDX-License-Identifier: MIT-0

//! The two concrete profile sources consumed by the query engine.

use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Tz;

use crate::calendar::exchange_calendar::ExchangeCalendar;
use crate::calendar::hours::MarketHours;
use crate::calendar::local_time::{is_holiday, mk_local_close, mk_local_open};
use crate::calendar::rule::{SessionKind, SessionRule};

const OPEN_DAY_ANCHOR_SSM: u32 = 12 * 3_600;

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
        }
    }

    pub(in crate::calendar) fn date_aware(calendar: ExchangeCalendar) -> Self {
        Self {
            source: ProfileSource::DateAware(calendar),
            tz: calendar.tz(),
        }
    }

    pub(super) const fn tz(self) -> Tz {
        self.tz
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

/// Normal-week calendars have no holiday overlay yet.
pub(super) const fn day_is_holiday(day: NaiveDate) -> bool {
    is_holiday(day)
}

/// Resolves one scheduled occurrence and rejects civil-time collapses.
pub(super) fn resolve_rule_bounds(
    context: &QueryContext<'_>,
    open_day: NaiveDate,
    rule: &SessionRule,
) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    if day_is_holiday(open_day) {
        return None;
    }
    let close_day = if rule.wraps_to_next_day() {
        let next_day = open_day.succ_opt()?;
        if day_is_holiday(next_day) {
            return None;
        }
        next_day
    } else {
        open_day
    };
    let tz = context.tz();
    let open = mk_local_open(tz, open_day, rule.open_ssm).with_timezone(&Utc);
    let close = mk_local_close(tz, close_day, rule.close_ssm).with_timezone(&Utc);
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
