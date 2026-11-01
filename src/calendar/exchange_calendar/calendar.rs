// SPDX-License-Identifier: MIT-0

//! The calendar type itself: construction, identity and holiday accessors,
//! and the constructors every caller starts from.

use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Tz;

use super::source::CalendarSource;
use crate::calendar::schedules::holidays::{self, Holiday, HolidayCoverage, HolidayTable};
use crate::calendar::{
    CalendarCoverage, Exchange, MarketHours, MarketHoursKey, hours_for_exchange,
    hours_for_market_hours_key,
};

/// A venue's zone is invariant across its revision eras, so any fixed instant
/// resolves it; the epoch keeps this independent of schedule data and of any
/// caller's clock.
const TZ_RESOLUTION_INSTANT: DateTime<Utc> = DateTime::<Utc>::UNIX_EPOCH;

/// One session's `(open, close)` bounds in UTC, as every boundary query
/// reports them: opens resolve earliest and closes latest under DST ambiguity.
pub type SessionWindow = (DateTime<Utc>, DateTime<Utc>);

/// Which answer the built-in holiday layer gives a calendar's queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::calendar) enum HolidayView {
    /// The shipped table applies, and a date it has no answer for refuses.
    Attached,
    /// The shipped table applies, and a date it has no answer for reads the
    /// sourced normal week.
    LastKnownWeek,
    /// No table applies.
    Detached,
}

/// A deterministic, date-aware calendar for one exchange or product family.
///
/// The calendar reselects a concrete fixed profile for every candidate local
/// opening day. It holds no clock and performs no I/O. It is
/// `Copy + Send + Sync + 'static`; built-in status, session-boundary,
/// trade-date, session-state, and candle-boundary queries allocate nothing.
/// A containing-session predicate takes `O(R + log H)` work for `R` selected
/// rules and `H` dated revisions. Forward and period queries inspect at most a
/// documented bounded number `D` of local days and therefore take
/// `O(D * (R + log H))` work.
/// Caller work inside a [`DayPolicy`](crate::DayPolicy) is outside this
/// performance guarantee. The normal-week duration helper is also excluded
/// because it collects and sorts temporary intervals. A built-in holiday
/// table, where the identity has one, is a binary search over a sorted static
/// table and is gated so a date the table says nothing about costs only that
/// search.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExchangeCalendar {
    source: CalendarSource,
    holidays: HolidayView,
    next_known_week: bool,
}

impl ExchangeCalendar {
    /// Creates a date-aware calendar for `exchange`.
    ///
    /// The identity's built-in holiday table, if it has one, applies to every
    /// query; [`Self::without_holidays`] detaches it.
    #[must_use]
    pub const fn new(exchange: Exchange) -> Self {
        Self {
            source: CalendarSource::Exchange(exchange),
            holidays: HolidayView::Attached,
            next_known_week: false,
        }
    }

    /// Creates a date-aware calendar for `key`.
    ///
    /// The family's built-in holiday table, if it has one, applies to every
    /// query; [`Self::without_holidays`] detaches it.
    #[must_use]
    pub const fn for_market_hours_key(key: MarketHoursKey) -> Self {
        Self {
            source: CalendarSource::MarketHoursKey(key),
            holidays: HolidayView::Attached,
            next_known_week: false,
        }
    }

    /// Detaches this identity's built-in holiday table from every query.
    ///
    /// The result answers exactly as the crate did before any table shipped:
    /// the sourced normal week, plus whatever the caller's own overlays say.
    /// This is the consumer's escape hatch when a shipped row is wrong and a
    /// release cannot wait, and the exact A/B control for measuring what the
    /// table costs. A detached calendar also reports no
    /// [`Self::holiday_on`] row and no [`Self::holiday_coverage`] window: it
    /// has no table, rather than a table it declines to apply.
    #[must_use]
    pub const fn without_holidays(self) -> Self {
        Self {
            source: self.source,
            holidays: HolidayView::Detached,
            next_known_week: self.next_known_week,
        }
    }

    /// Answers from the week the identity knows wherever the default calendar
    /// would refuse, for a consumer that must keep operating without a sourced
    /// answer.
    ///
    /// A default calendar refuses a date above the identity's last audited
    /// holiday window, because the operator's publication horizon governs
    /// there; every date of an identity whose contract is
    /// [`HolidayContract::NoTable`](crate::HolidayContract::NoTable); a date it
    /// withholds as [`HolidayKind::Unsourced`](crate::HolidayKind::Unsourced);
    /// and a date below its sourced normal week. This view is the caller's
    /// explicit choice to read all of those from the weekly profile
    /// [`Self::hours_at`] serves there.
    ///
    /// Every other answer is the default calendar's: an audited row keeps its
    /// arrangement, including inside a search that crosses the last window.
    /// Only a date below the support floor still refuses, as does a query in the
    /// first days above it that has to read below it, and a bounded search on an
    /// identity with no sessions to find still reports its exhaustion.
    ///
    /// These answers are not evidence. An unlisted or withheld holiday reads as
    /// an ordinary day, and a carried era reads as the earliest week the
    /// identity knows. [`Self::coverage`] reports each such date as
    /// [`DateCoverage::NormalWeekOnly`](crate::DateCoverage::NormalWeekOnly), so
    /// a consumer can disclose it or decline to infer a closure from it. A
    /// calendar that detached its table with [`Self::without_holidays`] stays
    /// detached.
    ///
    /// ```
    /// use chrono::{DateTime, Utc};
    /// use exchange_hours::{CalendarQueryError, DateCoverage, Exchange, ExchangeCalendar};
    ///
    /// let thursday: DateTime<Utc> = "2029-03-01T15:00:00Z".parse().unwrap();
    /// let nyse = ExchangeCalendar::new(Exchange::Nyse);
    /// assert!(matches!(
    ///     nyse.is_open(thursday),
    ///     Err(CalendarQueryError::OutsideCoveredRange { .. })
    /// ));
    ///
    /// let view = nyse.with_last_known_week();
    /// assert_eq!(view.is_open(thursday), Ok(true));
    /// assert_eq!(
    ///     view.coverage().coverage_on(thursday.date_naive()),
    ///     DateCoverage::NormalWeekOnly
    /// );
    /// ```
    #[must_use]
    pub const fn with_last_known_week(self) -> Self {
        Self {
            source: self.source,
            holidays: match self.holidays {
                HolidayView::Detached => HolidayView::Detached,
                HolidayView::Attached | HolidayView::LastKnownWeek => HolidayView::LastKnownWeek,
            },
            next_known_week: self.next_known_week,
        }
    }

    /// Answers a date before this identity's coverage from the next week it knows,
    /// for a consumer replaying history that must not stop there, such as a
    /// backtest.
    ///
    /// A default calendar refuses a date below the support floor and a date below
    /// the identity's sourced normal week. This view is the caller's explicit choice
    /// to read those dates forward, from the weekly profile [`Self::hours_at`] serves
    /// there, which is the earliest one the identity knows.
    ///
    /// Every answer inside coverage stays the default calendar's, and a date after
    /// coverage is unchanged; [`Self::with_last_known_week`] is the view for those,
    /// and with both no date refuses. A bounded search on an identity with no
    /// sessions to find still reports its exhaustion.
    ///
    /// These answers are not evidence. A holiday in that era reads as an ordinary
    /// day and the hours may have differed. [`Self::coverage`] reports each such date
    /// as [`DateCoverage::NormalWeekOnly`](crate::DateCoverage::NormalWeekOnly).
    ///
    /// ```
    /// use chrono::{DateTime, Utc};
    /// use exchange_hours::{CalendarQueryError, DateCoverage, Exchange, ExchangeCalendar};
    ///
    /// let tuesday: DateTime<Utc> = "2005-03-01T16:00:00Z".parse().unwrap();
    /// let nyse = ExchangeCalendar::new(Exchange::Nyse);
    /// assert!(matches!(
    ///     nyse.is_open(tuesday),
    ///     Err(CalendarQueryError::BeforeSupportFloor { .. })
    /// ));
    ///
    /// let view = nyse.with_next_known_week();
    /// assert_eq!(view.is_open(tuesday), Ok(true));
    /// assert_eq!(
    ///     view.coverage().coverage_on(tuesday.date_naive()),
    ///     DateCoverage::NormalWeekOnly
    /// );
    /// ```
    #[must_use]
    pub const fn with_next_known_week(self) -> Self {
        Self {
            source: self.source,
            holidays: self.holidays,
            next_known_week: true,
        }
    }

    /// Returns the built-in holiday row for venue-local `trade_date`.
    ///
    /// `None` means one of three different things — this identity has no
    /// table, the date is outside the table's coverage window, or the date was
    /// audited and found normal. [`Self::holiday_coverage`] separates them,
    /// which is why both exist and why neither is enough alone.
    ///
    /// The `Holidays` column of the verification ledger records which
    /// identities have a table and its audited trade-date windows
    /// (LAW-HOLIDAY-SCOPE). An identity without a table answers `None` for
    /// every date. This metadata lookup does not establish coverage for a
    /// date-aware session query; inspect [`Self::coverage`] for that contract.
    #[must_use]
    pub fn holiday_on(self, trade_date: NaiveDate) -> Option<Holiday> {
        self.holiday_table()
            .and_then(|table| table.holiday_on(trade_date))
    }

    /// Returns the inclusive trade-date window this identity's built-in table
    /// audited, or `None` when it has no table.
    ///
    /// Inside an audited window, a date with no row is audited normal.
    /// Outside every audited window, the table has no classification for
    /// that date. Date-aware session queries enforce [`Self::coverage`]:
    /// they may answer from a sourced normal week across a disclosed holiday
    /// evidence gap, or return a typed coverage error. The absence of a
    /// holiday row does not by itself establish ordinary trading.
    ///
    /// The verification ledger's `Holidays` column records the per-identity
    /// inventory. Use [`HolidayCoverage::windows`] and
    /// [`HolidayCoverage::contains`] to distinguish audited windows from
    /// gaps between the outer endpoints (LAW-HOLIDAY-SCOPE).
    #[must_use]
    pub fn holiday_coverage(self) -> Option<HolidayCoverage> {
        self.holiday_table().map(HolidayTable::coverage)
    }

    /// Resolves this identity's built-in table, honouring
    /// [`Self::without_holidays`].
    pub(in crate::calendar) const fn holiday_table(self) -> Option<&'static HolidayTable> {
        match self.holidays {
            HolidayView::Attached | HolidayView::LastKnownWeek => holidays::table_for(self.source),
            HolidayView::Detached => None,
        }
    }

    /// Returns what this identity's calendar can and cannot answer
    /// (LAW-COVERAGE).
    ///
    /// The metadata reports, separately: the identity; the venue-local date
    /// below which its normal week is carried rather than sourced (the
    /// verification ledger's `Horizon` column); the holiday layer's own
    /// contract, including an audited window a date with no row is normal
    /// inside; the venue-local ranges it answers completely; and the ranges it
    /// does not, each with a reason.
    /// [`CalendarQueryError`](crate::CalendarQueryError) is the matching error
    /// vocabulary, and the crate's support floor is
    /// [`SUPPORT_FLOOR`](crate::SUPPORT_FLOOR).
    ///
    /// [`Self::without_holidays`] is respected: where the identity ships a
    /// table, a detached calendar reports the **normal-week contract** rather
    /// than complete calendar coverage — its complete ranges are empty and its
    /// holiday contract is
    /// [`HolidayContract::NormalWeekOnly`](crate::HolidayContract::NormalWeekOnly)
    /// — while the normal-week side is unchanged. An identity with no table is
    /// unaffected either way. [`Self::with_last_known_week`] and
    /// [`Self::with_next_known_week`] keep the holiday contract and report each
    /// date they read from the normal week as
    /// [`DateCoverage::NormalWeekOnly`](crate::DateCoverage::NormalWeekOnly).
    ///
    /// The value borrows static tables, allocates nothing, and is
    /// `Copy + Send + Sync + 'static`. Inspecting it changes no query: the
    /// metadata is not permission to return a fabricated schedule.
    #[must_use]
    pub const fn coverage(self) -> CalendarCoverage {
        CalendarCoverage::new(self.source, self.holidays, self.next_known_week)
    }

    /// Returns the schedule identity represented by this calendar.
    #[must_use]
    pub const fn source(self) -> CalendarSource {
        self.source
    }

    /// Returns the represented exchange, or `None` for a product-family calendar.
    #[must_use]
    pub const fn exchange(self) -> Option<Exchange> {
        match self.source {
            CalendarSource::Exchange(exchange) => Some(exchange),
            CalendarSource::MarketHoursKey(_) => None,
        }
    }

    /// Returns the represented product-family key, or `None` for an exchange calendar.
    #[must_use]
    pub const fn market_hours_key(self) -> Option<MarketHoursKey> {
        match self.source {
            CalendarSource::Exchange(_) => None,
            CalendarSource::MarketHoursKey(key) => Some(key),
        }
    }

    /// Resolves the fixed [`MarketHours`] profile in force at `instant`.
    #[must_use]
    pub fn hours_at(self, instant: DateTime<Utc>) -> MarketHours {
        match self.source {
            CalendarSource::Exchange(exchange) => hours_for_exchange(exchange, instant),
            CalendarSource::MarketHoursKey(key) => hours_for_market_hours_key(key, instant),
        }
    }

    /// Returns the venue's invariant IANA time zone.
    #[must_use]
    pub fn tz(self) -> Tz {
        match self.source {
            CalendarSource::Exchange(exchange) => {
                hours_for_exchange(exchange, TZ_RESOLUTION_INSTANT).tz
            }
            CalendarSource::MarketHoursKey(key) => {
                hours_for_market_hours_key(key, TZ_RESOLUTION_INSTANT).tz
            }
        }
    }
}

/// Creates a date-aware calendar for `exchange`.
#[must_use]
pub const fn calendar_for_exchange(exchange: Exchange) -> ExchangeCalendar {
    ExchangeCalendar::new(exchange)
}

/// Creates a date-aware calendar for a product-family `key`.
///
/// The calendar reselects [`hours_for_market_hours_key`] for every
/// candidate session-opening day, including while scanning across a sourced
/// historical revision.
#[must_use]
pub const fn calendar_for_market_hours_key(key: MarketHoursKey) -> ExchangeCalendar {
    ExchangeCalendar::for_market_hours_key(key)
}
