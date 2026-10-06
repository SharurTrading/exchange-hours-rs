// SPDX-License-Identifier: MIT-0

//! The normal-week baseline a coverage refusal sits inside (issue #296,
//! Tier 3).
//!
//! A typed refusal is the crate's honest answer where no sourced state decides
//! a date, and Tier 3 enriches it without changing its verdict: the error value
//! can now name the **sourced normal week** the refused date sits inside, so a
//! consumer can make its own call — treat the date as open beside a normal
//! 09:30-16:30 Wednesday, or exclude it — instead of receiving a bare refusal.
//! The baseline is derived, never stored: it reads the same static timeline the
//! query engine reads, anchored on the refused date's own opening day, so a
//! backtest and a live caller deriving it for the same date see one value
//! (LAW-DETERMINISM).
//!
//! The derivation allocates nothing on the rule slices — they arrive borrowed
//! from the static tables — and it is total: a date whose normal week is
//! carried rather than sourced, or that precedes the support floor, has **no**
//! baseline, because claiming one would fabricate a sourced state the ledger
//! does not record.

use chrono::{Datelike as _, Utc, Weekday};
use chrono_tz::Tz;

use super::SUPPORT_FLOOR;
use crate::calendar::exchange_calendar::{calendar_for_exchange, calendar_for_market_hours_key};
use crate::calendar::local_time::mk_local_open;
use crate::calendar::schedules::sourcing;
use crate::calendar::{CalendarQueryError, CalendarSource, MarketHours, SessionRule};

/// The last second of a venue-local day, the anchor the query gate uses to
/// select the profile governing the sessions opening on that day.
const OPEN_DAY_ANCHOR_SSM: u32 = 86_399;

/// The sourced normal-week baseline one coverage refusal sits inside.
///
/// The value carries the profile the identity's timeline serves for the
/// refused date's opening day — the era selection is the same one the query
/// engine performs, anchored at that day's last local second — and the
/// refused date's own weekday in the profile's zone. Its accessors project the
/// weekday's rules, and its [`Display`](core::fmt::Display) renders the
/// ordinary-week shape a consumer reads beside a holiday-arrangement refusal.
///
/// A baseline is a statement about the **normal week only**: it says nothing
/// about the refused date's holiday arrangement, which is exactly what the
/// refusal withholds. It is `Clone` rather than `Copy` because it carries the
/// profile; the rule slices behind it are borrowed static tables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalWeekBaseline {
    profile: MarketHours,
    weekday: Weekday,
}

impl NormalWeekBaseline {
    /// Returns the identity the baseline belongs to.
    #[must_use]
    pub fn identity(&self) -> CalendarSource {
        self.profile.source
    }

    /// Returns the profile's venue zone.
    #[must_use]
    pub fn tz(&self) -> Tz {
        self.profile.tz
    }

    /// Returns the refused date's weekday in the profile's zone.
    #[must_use]
    pub const fn weekday(&self) -> Weekday {
        self.weekday
    }

    /// Returns the profile the timeline serves for the refused date's opening
    /// day, with its rule sets readable through [`MarketHours`]'s public
    /// fields.
    #[must_use]
    pub const fn profile(&self) -> &MarketHours {
        &self.profile
    }

    /// Returns the refused date's weekday tradeable rules — the profile's
    /// `regular` and `extended` sets together, the union every `is_open`
    /// answer reads — as the static tables state them: seconds since local
    /// midnight, wrap rules included
    /// ([`SessionRule::wraps_to_next_day`](crate::SessionRule::wraps_to_next_day)).
    ///
    /// A weekday with no tradeable rule yields an empty iterator, which is the
    /// normal week's own statement that nothing tradeable is scheduled.
    pub fn tradeable_rules(&self) -> impl Iterator<Item = SessionRule> + use<'_> {
        self.weekday_rules(
            self.profile
                .regular
                .iter()
                .chain(self.profile.extended.iter()),
        )
    }

    /// Returns the refused date's weekday order-entry rules — the profile's
    /// `order_entry` set, whose windows accept orders that cannot match — as
    /// the static tables state them.
    pub fn order_entry_rules(&self) -> impl Iterator<Item = SessionRule> + use<'_> {
        self.weekday_rules(self.profile.order_entry.iter())
    }

    /// Filters one rule-set chain down to the baseline's own weekday.
    fn weekday_rules<'a>(
        &'a self,
        rules: impl Iterator<Item = &'a SessionRule> + 'a,
    ) -> impl Iterator<Item = SessionRule> + 'a {
        let weekday_index = self.weekday.num_days_from_monday() as usize;
        rules.copied().filter(move |rule| rule.days[weekday_index])
    }
}

/// Renders the baseline the way a refusal's context names it: the weekday and
/// the ordinary-week windows, in the profile's own local clock, chronological
/// within each phase class.
impl core::fmt::Display for NormalWeekBaseline {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "normal week {}: ", weekday_name(self.weekday))?;
        let mut tradeable: Vec<(u32, u32, bool)> = self
            .tradeable_rules()
            .map(|rule| (rule.open_ssm, rule.close_ssm, rule.wraps_to_next_day()))
            .collect();
        tradeable.sort_unstable();
        if tradeable.is_empty() {
            write!(f, "no ordinary session")?;
        }
        for (index, (open_ssm, close_ssm, wraps)) in tradeable.iter().enumerate() {
            if index > 0 {
                write!(f, ", ")?;
            }
            write!(f, "open {}", window_text(*open_ssm, *close_ssm, *wraps))?;
        }
        let mut order_entry: Vec<(u32, u32, bool)> = self
            .order_entry_rules()
            .map(|rule| (rule.open_ssm, rule.close_ssm, rule.wraps_to_next_day()))
            .collect();
        order_entry.sort_unstable();
        for (open_ssm, close_ssm, wraps) in order_entry {
            write!(
                f,
                ", order entry {}",
                window_text(open_ssm, close_ssm, wraps)
            )?;
        }
        Ok(())
    }
}

/// Renders one rule's slice as a local-clock window.
fn window_text(open_ssm: u32, close_ssm: u32, wraps: bool) -> String {
    if wraps {
        format!("{}-{} (+1)", clock_text(open_ssm), clock_text(close_ssm))
    } else {
        format!("{}-{}", clock_text(open_ssm), clock_text(close_ssm))
    }
}

/// Renders seconds since local midnight as `HH:MM` or `HH:MM:SS`.
fn clock_text(ssm: u32) -> String {
    let (hour, rest) = (ssm / 3_600, ssm % 3_600);
    let (minute, second) = (rest / 60, rest % 60);
    if second == 0 {
        format!("{hour:02}:{minute:02}")
    } else {
        format!("{hour:02}:{minute:02}:{second:02}")
    }
}

/// The weekday names the baseline renders, Monday-first.
fn weekday_name(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }
}

/// Derives the sourced normal-week baseline an error's date sits inside, or
/// `None` when the crate states none.
///
/// Three refusals keep the baseline absent, because claiming it would state a
/// sourced fact the ledger does not record: the date precedes the support
/// floor; the identity's normal week is carried backwards below its recorded
/// horizon at the date (LAW-COVERAGE's carried-below fact); or the identity
/// resolves no profile at all. Everywhere else the baseline is the timeline's
/// own era selection for the date's opening day — the same selection the query
/// gate performs — so the context a caller reads beside the refusal is the
/// week the crate's answers actually come from.
pub(crate) fn for_error(error: CalendarQueryError) -> Option<NormalWeekBaseline> {
    let source = error.source();
    let date = error.date();
    if date < SUPPORT_FLOOR {
        return None;
    }
    let declared = sourcing::declared(source);
    if declared.carried_below.is_some_and(|carried| date < carried) {
        return None;
    }
    let calendar = match source {
        CalendarSource::Exchange(exchange) => calendar_for_exchange(exchange),
        CalendarSource::MarketHoursKey(key) => calendar_for_market_hours_key(key),
    };
    let tz = calendar.tz();
    let anchor = mk_local_open(tz, date, OPEN_DAY_ANCHOR_SSM).with_timezone(&Utc);
    let profile = calendar.hours_at(anchor);
    Some(NormalWeekBaseline {
        weekday: date.weekday(),
        profile,
    })
}
