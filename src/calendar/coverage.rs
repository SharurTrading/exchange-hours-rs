// SPDX-License-Identifier: MIT-0

//! What a calendar can answer, and the vocabulary for what it cannot
//! (LAW-COVERAGE).
//!
//! The support floor is fixed at **1 January 2025 in each venue's own
//! local-date domain** ([`SUPPORT_FLOOR`]) — never a rolling window, and never a
//! single UTC midnight, because a Tokyo session's 2025-01-01 begins while
//! Chicago is still on 2024-12-31. At or above that floor an identity-backed
//! calendar answers a date only where three separate facts hold, and this module
//! reports each of them rather than collapsing them into one flag:
//!
//! 1. **the normal week is sourced**, not carried backwards
//!    ([`CalendarCoverage::normal_week_sourced_from`], from the verification
//!    ledger's `Horizon` column through `schedules/sourcing.rs`);
//! 2. **the holiday layer has an answer** — either an audited trade-date window
//!    the identity's built-in table covers, or an affirmative no-holiday
//!    assertion ([`CalendarCoverage::holiday_contract`]); and
//! 3. **the identity does not withhold the date** as
//!    [`HolidayKind::Unsourced`](crate::HolidayKind::Unsourced).
//!
//! [`CalendarCoverage::coverage_on`] reports the verdict for one venue-local
//! date; [`CalendarCoverage::complete_ranges`] and [`CalendarCoverage::gaps`]
//! report the same derivation as ascending spans and their reasons.
//! [`CalendarQueryError`] is the explicit error vocabulary Stage 2B's
//! identity-backed queries return instead of answering an unsupported date.
//!
//! A **date-shaped** gap is what the three facts above decide, and
//! [`CoverageGapReason::NormalWeekCarried`], [`CoverageGapReason::NoHolidayTable`],
//! [`CoverageGapReason::NoHolidayCoverage`], [`CoverageGapReason::WithheldDate`]
//! and [`CoverageGapReason::NormalWeekOnly`] name its five kinds. A **phase-level**
//! gap is not date-shaped at all: the operator publishes the arrangement, and the
//! crate's own static vocabulary cannot state it, on *every* date the claim
//! covers. `schedules/sourcing.rs` declares those per identity, and this module
//! reports them as [`CoverageGapReason::NormalWeekPhaseWithheld`] (#79) and
//! [`CoverageGapReason::SpecialSessionUnrepresentable`] (#93). An identity
//! carrying one is **not complete anywhere in its supported domain**, so its
//! complete ranges are empty and its single gap spans the whole domain — the
//! verdict LAW-COVERAGE and `docs/schedules/coverage-2025.md` already state.
//!
//! Nothing here changes an existing query's signature. Inspectable metadata is
//! not permission to return a fabricated schedule: a date this module reports as
//! outside the covered range has no sourced answer, and the caller's overlay is
//! the only layer that can supply one.

mod error;
mod ranges;

pub use error::CalendarQueryError;
pub use ranges::{CompleteRanges, CoverageGaps};

use chrono::NaiveDate;

use super::exchange_calendar::CalendarSource;
use super::schedules::holidays::{self, HolidayCoverage, HolidayKind, HolidayTable};
use super::schedules::sourcing;
use super::schedules::timeline::effective_date;

/// The permanent support floor, **1 January 2025 in the venue's own local-date
/// domain** (LAW-COVERAGE).
///
/// This is a *local* date, not a UTC instant: the instant at which an identity
/// reaches its floor depends on that identity's IANA zone, and a query's instant
/// is judged by the venue-local date it falls on. The floor is fixed and never a
/// rolling previous-year window. A later sourced launch stays the identity's own
/// coverage start (`docs/schedules/coverage-2025.md`).
pub const SUPPORT_FLOOR: NaiveDate = effective_date(2025, 1, 1);

/// One inclusive, ascending venue-local date span.
///
/// A span whose [`Self::last`] is [`NaiveDate::MAX`] has no known end: the
/// sourced history continues as far as the crate's data does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DateRange {
    first: NaiveDate,
    last: NaiveDate,
}

impl DateRange {
    /// Builds a span, or `None` when `last` precedes `first`.
    #[must_use]
    pub fn new(first: NaiveDate, last: NaiveDate) -> Option<Self> {
        (first <= last).then_some(Self { first, last })
    }

    /// Returns the first venue-local date.
    #[must_use]
    pub const fn first(self) -> NaiveDate {
        self.first
    }

    /// Returns the last venue-local date.
    ///
    /// [`NaiveDate::MAX`] means "no known end", never "the world ends here".
    #[must_use]
    pub const fn last(self) -> NaiveDate {
        self.last
    }

    /// Returns whether `date` falls inside this span.
    #[must_use]
    pub fn contains(self, date: NaiveDate) -> bool {
        self.first <= date && date <= self.last
    }

    /// Returns whether this span has no known end.
    #[must_use]
    pub fn is_open_ended(self) -> bool {
        self.last == NaiveDate::MAX
    }
}

/// The coverage verdict for one venue-local date.
///
/// This is the date-level answer [`CalendarCoverage`] derives, and the input
/// Stage 2B maps onto [`CalendarQueryError`]: [`Self::BeforeSupportFloor`],
/// [`Self::OutsideCoveredRange`] and [`Self::UnresolvedGap`] each have exactly
/// one error counterpart, and a bounded search that runs past the data reports
/// [`CalendarQueryError::SearchExhausted`] instead.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateCoverage {
    /// The date is inside this identity's complete covered calendar.
    Covered,
    /// The date precedes the venue-local [`SUPPORT_FLOOR`].
    BeforeSupportFloor,
    /// The date is at or after the floor but outside the ranges this identity
    /// has a sourced answer for: its weekday profile is carried backwards
    /// there, or its holiday layer has no answer.
    OutsideCoveredRange,
    /// The date is inside an audited window on a date the identity explicitly
    /// withholds as [`HolidayKind::Unsourced`](crate::HolidayKind::Unsourced).
    UnresolvedGap,
    /// The calendar detached its built-in holiday table with
    /// [`without_holidays`](crate::ExchangeCalendar::without_holidays): it
    /// answers the normal-week contract and claims no complete calendar.
    NormalWeekOnly,
}

/// Why a span inside the supported domain is not complete.
///
/// Five variants are **date-shaped**: they describe a span of venue-local dates
/// and are derived from the identity's own timeline and holiday table. Two are
/// **phase-level**, declared once per identity in `schedules/sourcing.rs`
/// (LAW-COVERAGE's required-phase and special-session gaps): they apply to every
/// date in the claimed interval, so a date walk over the tables can never find
/// them. An identity carrying a phase-level gap is incomplete everywhere, and
/// [`CoverageGap::closing_condition`] names what would discharge it.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoverageGapReason {
    /// The weekday profile below the identity's carried-below horizon is
    /// carried backwards rather than sourced.
    NormalWeekCarried,
    /// No built-in holiday table ships for this identity and none is claimed.
    NoHolidayTable,
    /// The date lies outside every trade-date window this identity's built-in
    /// holiday table audited.
    NoHolidayCoverage,
    /// The identity explicitly withholds the date as
    /// [`HolidayKind::Unsourced`](crate::HolidayKind::Unsourced).
    WithheldDate,
    /// The calendar detached its built-in holiday table with
    /// [`without_holidays`](crate::ExchangeCalendar::without_holidays).
    NormalWeekOnly,
    /// **Declared phase-level.** The identity's sourced normal week contains a
    /// required phase the calendar withholds, so no date in the claimed interval
    /// is answered from a complete normal week.
    ///
    /// The shape is a slice of one phase's boundary: the operator publishes the
    /// phase, the crate's scalar rules serve only the part of it that holds under
    /// every sourced state, and the remainder depends on a change this crate
    /// cannot date (LAW-NO-FABRICATED-DATES). `globex_equity_index` is the one
    /// shipped case — CME's Sunday 16:00-16:15 CT quarter-hour, withheld in
    /// favour of the 16:15-17:00 CT intersection the crate carries from its 2010
    /// floor (#79) — and the owner's evidence file records the 2012 move the
    /// disputed quarter-hour depends on.
    ///
    /// Because the withheld slice lies inside a phase on a recurring grid rather
    /// than on one trade date, this is not [`Self::WithheldDate`]: it is not a
    /// date-level exception, it cannot be discharged by an
    /// [`ExceptionBlock`](crate::ExceptionBlock) row alone, and it does not move
    /// with the identity's holidays.
    NormalWeekPhaseWithheld,
    /// **Declared phase-level.** The identity's operator publishes at least one
    /// session this crate's scalar vocabulary cannot state, so no date in the
    /// claimed interval is answered from a complete calendar.
    ///
    /// The shape is a *whole session that the modelled week has no slot for* —
    /// an extra session on a weekday the normal week does not trade, or a
    /// trade-date arrangement the scalar rows cannot key. Unlike the rest of this
    /// vocabulary it is not a withheld answer on a date the caller can see: the
    /// date may be answered plausibly by the ordinary week, which is exactly why
    /// it is a completeness gap rather than an error at query time. The closing
    /// condition is the replacement-block engine of LAW-HOLIDAY-SCOPE (#93), and
    /// [`ExceptionBlock`](crate::ExceptionBlock) already reaches callers, so a
    /// caller can supply the session the crate cannot.
    ///
    /// `globex_fx` and `globex_cryptocurrency` are the shipped cases: CME's
    /// Saturday sessions and merged trade dates, recorded in each owner's
    /// evidence file.
    SpecialSessionUnrepresentable,
}

/// A phase-level completeness gap one identity declares about itself: a known
/// internal gap that no date walk over the identity's tables can find.
///
/// Declared in `schedules/sourcing.rs` beside the identity it belongs to, never
/// inferred from a timeline or a holiday table. LAW-COVERAGE requires complete
/// coverage to contain "no unresolved normal-week, required-phase, holiday or
/// special-session gap"; the first two of those are shapes the tables under
/// `schedules/` cannot express today, so they are stated affirmatively, exactly
/// as `observes_no_holidays` states the absence of holiday closures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhaseGap {
    /// Which phase-level gap the identity carries.
    reason: CoverageGapReason,
    /// The issue whose closure would discharge it, as it is written in the
    /// verification ledger and the coverage inventory (`#79`, `#93`).
    closing_condition: &'static str,
}

impl PhaseGap {
    /// Declares a phase-level gap and the issue that closes it.
    pub(crate) const fn new(reason: CoverageGapReason, closing_condition: &'static str) -> Self {
        Self {
            reason,
            closing_condition,
        }
    }

    /// Returns which phase-level gap the identity carries.
    #[must_use]
    pub const fn reason(self) -> CoverageGapReason {
        self.reason
    }

    /// Returns the issue whose closure would discharge the gap, as the ledger
    /// and `docs/schedules/coverage-2025.md` write it.
    ///
    /// A static string, never a `String`: the metadata stays allocation-free and
    /// `Copy`.
    #[must_use]
    pub const fn closing_condition(self) -> &'static str {
        self.closing_condition
    }
}

/// One span inside the supported domain this identity cannot answer completely.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CoverageGap {
    range: DateRange,
    reason: CoverageGapReason,
    phase_gap: Option<PhaseGap>,
}

impl CoverageGap {
    /// Returns the venue-local span the gap covers.
    ///
    /// A **date-shaped** gap spans the dates its reason applies to. A
    /// **phase-level** gap declared per identity spans the whole supported
    /// domain, because the arrangement it records is missing on every date the
    /// claim covers and not merely on the dates a boundary falls between.
    #[must_use]
    pub const fn range(self) -> DateRange {
        self.range
    }

    /// Returns why the span is not complete.
    #[must_use]
    pub const fn reason(self) -> CoverageGapReason {
        self.reason
    }

    /// Returns the declared phase-level gap this span reports, or `None` when
    /// the span is date-shaped.
    ///
    /// This is where a caller finds the **closing condition** LAW-COVERAGE
    /// requires a gap to carry: the issue whose closure would discharge it, as
    /// the verification ledger and `docs/schedules/coverage-2025.md` write it.
    #[must_use]
    pub const fn phase_gap(self) -> Option<PhaseGap> {
        self.phase_gap
    }

    /// Returns the issue whose closure would discharge this gap, or `None` when
    /// the gap is date-shaped and no single issue owns it.
    ///
    /// A date-shaped gap is closed by data — a sourced row, a wider audited
    /// window, a holiday table the identity does not have yet — and this
    /// vocabulary does not invent an issue number for it. A phase-level gap is
    /// declared with exactly one.
    #[must_use]
    pub const fn closing_condition(self) -> Option<&'static str> {
        match self.phase_gap {
            Some(phase_gap) => Some(phase_gap.closing_condition()),
            None => None,
        }
    }
}

/// What the built-in holiday layer asserts for one calendar.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HolidayContract {
    /// No built-in table ships for this identity and none is claimed: the crate
    /// has no holiday answer for any date, and the caller's
    /// [`DayPolicy`](crate::DayPolicy) overlay is the only holiday layer.
    NoTable,
    /// The identity's own definition has no holiday closures, so every date is
    /// normal.
    ///
    /// An **affirmative** assertion recorded beside the identity — the two
    /// synthetic 24×7 identities, whose continuity is library policy, and the
    /// one operator that publishes continuous 24/7 availability — never an
    /// inference from a missing table.
    NoHolidays,
    /// The calendar detached its built-in table with
    /// [`without_holidays`](crate::ExchangeCalendar::without_holidays): it
    /// answers the normal-week contract, and complete calendar coverage is not
    /// claimed.
    NormalWeekOnly,
    /// A built-in table audited these trade-date windows.
    ///
    /// Inside a window a date with no row is **audited normal**; outside every
    /// window the crate has no holiday answer at all. A window carrying zero
    /// rows is therefore a pure audited-normal claim — "every date here was
    /// audited and found normal" — which is a different statement from
    /// [`Self::NoTable`].
    Audited {
        /// The inclusive venue-local trade-date windows the table audited.
        coverage: HolidayCoverage,
        /// How many dated rows the table carries; zero means the windows above
        /// are an audited-normal claim rather than a change list.
        rows: usize,
    },
}

impl HolidayContract {
    /// Returns the audited windows, or `None` when no built-in table applies.
    #[must_use]
    pub const fn coverage(self) -> Option<HolidayCoverage> {
        match self {
            Self::Audited { coverage, .. } => Some(coverage),
            _ => None,
        }
    }

    /// Returns the number of dated rows, or `None` when no built-in table
    /// applies.
    #[must_use]
    pub const fn rows(self) -> Option<usize> {
        match self {
            Self::Audited { rows, .. } => Some(rows),
            _ => None,
        }
    }
}

/// What one identity's calendar can answer, as of the shipped data.
///
/// Built by [`ExchangeCalendar::coverage`](crate::ExchangeCalendar::coverage);
/// it borrows the identity's static tables, so reading it allocates nothing and
/// the value stays `Copy + Send + Sync + 'static`. Every range it reports is
/// clipped to [`SUPPORT_FLOOR`], the permanent 2025-01-01 local-date floor.
///
/// [`Self::coverage_on`] is the per-date verdict, [`Self::complete_ranges`] the
/// spans that answer completely, and [`Self::gaps`] the rest with their
/// reasons. An identity that declares a **phase-level** gap in
/// `schedules/sourcing.rs` reports no complete range at all and one gap spanning
/// the whole domain ([`Self::phase_gap`]).
///
/// The spans are **derived, not duplicated**: the iterators walk the identity's
/// static timeline horizon and holiday-window edges in ascending order and stop
/// at the next edge, so reading the metadata allocates nothing and never sorts.
/// Deriving them per call is also what lets a detached calendar report the
/// normal-week contract from the very same value.
#[derive(Clone, Copy)]
pub struct CalendarCoverage {
    source: CalendarSource,
    carried_below: Option<NaiveDate>,
    phase_gap: Option<PhaseGap>,
    holidays: HolidayContract,
    table: Option<&'static HolidayTable>,
}

impl CalendarCoverage {
    /// Builds the metadata one identity's calendar reports.
    ///
    /// `holidays_attached` is the calendar's own flag: a detached calendar
    /// reports the normal-week contract ([`HolidayContract::NormalWeekOnly`])
    /// and claims no complete calendar, while its normal-week side is unchanged.
    pub(crate) const fn new(source: CalendarSource, holidays_attached: bool) -> Self {
        let declared = sourcing::declared(source);
        let shipped = holidays::table_for(source);
        let holidays = match shipped {
            Some(table) if holidays_attached => HolidayContract::Audited {
                coverage: table.coverage(),
                rows: table.rows.len(),
            },
            Some(_) => HolidayContract::NormalWeekOnly,
            None if declared.observes_no_holidays => HolidayContract::NoHolidays,
            None => HolidayContract::NoTable,
        };
        Self {
            source,
            carried_below: declared.carried_below,
            phase_gap: declared.phase_gap,
            holidays,
            table: if holidays_attached { shipped } else { None },
        }
    }

    /// Returns the identity this metadata describes.
    #[must_use]
    pub const fn identity(self) -> CalendarSource {
        self.source
    }

    /// Returns the venue-local date below which this identity's normal-week rows
    /// are **carried** backwards rather than sourced, or `None` when the
    /// verification ledger records that nothing is carried.
    ///
    /// `None` is the ledger's em dash, not missing data: the identity carries no
    /// row below its own first one, so its weekday profile is sourced wherever
    /// its own timeline governs. It is not a claim that every modelled date is
    /// sourced — an identity can still withhold an era it cannot state, and the
    /// ledger's `Basis` cell and the owner's evidence file record that
    /// separately.
    #[must_use]
    pub const fn normal_week_sourced_from(self) -> Option<NaiveDate> {
        self.carried_below
    }

    /// Returns the venue-local span whose weekday profile is sourced rather than
    /// carried backwards, clipped to the support floor.
    ///
    /// The span is open-ended: the sourced history continues as far as the
    /// crate's data does. Only the normal-week fact is reported here; whether a
    /// date inside it can be answered completely is
    /// [`Self::coverage_on`]'s question.
    #[must_use]
    pub fn sourced_normal_week(self) -> DateRange {
        let first = match self.carried_below {
            Some(carried_below) if carried_below > SUPPORT_FLOOR => carried_below,
            _ => SUPPORT_FLOOR,
        };
        DateRange {
            first,
            last: NaiveDate::MAX,
        }
    }

    /// Returns what the built-in holiday layer asserts for this calendar.
    #[must_use]
    pub const fn holiday_contract(self) -> HolidayContract {
        self.holidays
    }

    /// Returns the phase-level gap this identity declares about itself, or
    /// `None` when it declares none.
    ///
    /// This is the completeness fact no date walk can derive: the identity's
    /// normal week or calendar carries an arrangement its own static vocabulary
    /// cannot state, so it is incomplete on **every** date in the claimed
    /// interval. `None` is an affirmative "no such gap declared", not missing
    /// data — the declaration lives in `schedules/sourcing.rs`, one arm per
    /// identity, and is never inferred from a timeline or a holiday table.
    #[must_use]
    pub const fn phase_gap(self) -> Option<PhaseGap> {
        self.phase_gap
    }

    /// Returns the coverage verdict for venue-local `date`.
    ///
    /// A date inside a declared **phase-level** gap reports
    /// [`DateCoverage::OutsideCoveredRange`], not a new verdict: LAW-COVERAGE
    /// makes an unresolved normal-week or special-session gap the same
    /// "no sourced answer" case as a carried horizon, and Stage 2B maps it to
    /// the same [`CalendarQueryError::OutsideCoveredRange`]. The distinction
    /// between the two lives in [`Self::gaps`], which carries the closing
    /// condition a caller needs in order to tell "not worked up yet" from
    /// "answered, and the answer is ordinary".
    #[must_use]
    pub fn coverage_on(self, date: NaiveDate) -> DateCoverage {
        if date < SUPPORT_FLOOR {
            return DateCoverage::BeforeSupportFloor;
        }
        match self.gap_reason_on(date) {
            None => DateCoverage::Covered,
            Some(CoverageGapReason::WithheldDate) => DateCoverage::UnresolvedGap,
            Some(CoverageGapReason::NormalWeekOnly) => DateCoverage::NormalWeekOnly,
            Some(
                CoverageGapReason::NormalWeekCarried
                | CoverageGapReason::NoHolidayTable
                | CoverageGapReason::NoHolidayCoverage
                | CoverageGapReason::NormalWeekPhaseWithheld
                | CoverageGapReason::SpecialSessionUnrepresentable,
            ) => DateCoverage::OutsideCoveredRange,
        }
    }

    /// Returns whether `date` is inside this identity's complete covered
    /// calendar.
    #[must_use]
    pub fn is_complete_on(self, date: NaiveDate) -> bool {
        self.coverage_on(date) == DateCoverage::Covered
    }

    /// Iterates the maximal spans, at or after the support floor, that this
    /// identity answers completely, in ascending order.
    ///
    /// The spans are disjoint and separated by [`Self::gaps`]; the last one may
    /// be open-ended.
    #[must_use]
    pub fn complete_ranges(self) -> CompleteRanges {
        CompleteRanges::new(self)
    }

    /// Iterates the maximal spans, at or after the support floor, that this
    /// identity cannot answer completely, in ascending order, each with the
    /// reason that applies inside it.
    ///
    /// The spans are disjoint and separated by [`Self::complete_ranges`].
    #[must_use]
    pub fn gaps(self) -> CoverageGaps {
        CoverageGaps::new(self)
    }

    /// Returns the reason `date` is not complete, or `None` when it is.
    ///
    /// Below the floor nothing is a gap: the supported domain starts there. A
    /// declared phase-level gap is checked **first**, because it is a fact about
    /// the whole claimed interval rather than about a span of dates inside it: an
    /// identity that carries one has no covered date, whatever its timeline and
    /// its holiday windows say.
    pub(super) fn gap_reason_on(self, date: NaiveDate) -> Option<CoverageGapReason> {
        if date < SUPPORT_FLOOR {
            return None;
        }
        if let Some(phase_gap) = self.phase_gap {
            return Some(phase_gap.reason());
        }
        if let Some(carried_below) = self.carried_below
            && date < carried_below
        {
            return Some(CoverageGapReason::NormalWeekCarried);
        }
        match self.holidays {
            HolidayContract::NormalWeekOnly => Some(CoverageGapReason::NormalWeekOnly),
            HolidayContract::NoHolidays => None,
            HolidayContract::NoTable => Some(CoverageGapReason::NoHolidayTable),
            HolidayContract::Audited { coverage, .. } => {
                if self.withholds(date) {
                    Some(CoverageGapReason::WithheldDate)
                } else if coverage.contains(date) {
                    None
                } else {
                    Some(CoverageGapReason::NoHolidayCoverage)
                }
            }
        }
    }

    /// Returns whether this identity's built-in table withholds `date` as
    /// `Unsourced`.
    fn withholds(self, date: NaiveDate) -> bool {
        self.table.is_some_and(|table| {
            table
                .holiday_on(date)
                .is_some_and(|holiday| holiday.kind() == HolidayKind::Unsourced)
        })
    }

    /// Returns the first venue-local date strictly after `date` at which the
    /// verdict can change, or `None` when no later boundary exists.
    ///
    /// The candidates are the support floor, the carried-below horizon, both
    /// edges of every audited window and the two dates around every withheld
    /// row — all static and bounded, so the walk allocates nothing.
    ///
    /// A declared phase-level gap has no boundary at all: the same reason holds
    /// from the floor to the end of the domain, so the walk returns `None`
    /// immediately rather than visiting edges that cannot change the verdict.
    fn next_boundary_after(self, date: NaiveDate) -> Option<NaiveDate> {
        if self.phase_gap.is_some() {
            return None;
        }
        let mut best: Option<NaiveDate> = None;
        let mut consider = |candidate: Option<NaiveDate>| {
            best = [best, candidate].into_iter().flatten().min();
        };
        consider((SUPPORT_FLOOR > date).then_some(SUPPORT_FLOOR));
        consider(self.carried_below.filter(|boundary| *boundary > date));
        for &(first_year, first_month, first_day, last_year, last_month, last_day) in self.windows()
        {
            consider(
                NaiveDate::from_ymd_opt(first_year, first_month, first_day)
                    .filter(|first| *first > date),
            );
            consider(
                NaiveDate::from_ymd_opt(last_year, last_month, last_day)
                    .and_then(|last| last.succ_opt())
                    .filter(|after| *after > date),
            );
        }
        if let Some(table) = self.table {
            for row in table.rows {
                if row.kind == HolidayKind::Unsourced && row.trade_date >= SUPPORT_FLOOR {
                    consider(Some(row.trade_date).filter(|withheld| *withheld > date));
                    consider(row.trade_date.succ_opt().filter(|after| *after > date));
                }
            }
        }
        best
    }

    /// Returns the audited holiday windows, or an empty slice when no built-in
    /// table applies to this calendar.
    fn windows(self) -> &'static [(i32, u32, u32, i32, u32, u32)] {
        self.table.map_or(&[], |table| table.windows)
    }

    /// Returns the last date of the run starting at `first`, or
    /// [`NaiveDate::MAX`] when the run has no later boundary.
    pub(super) fn run_end(self, first: NaiveDate) -> NaiveDate {
        match self
            .next_boundary_after(first)
            .and_then(|boundary| boundary.pred_opt())
        {
            Some(last) => last,
            None => NaiveDate::MAX,
        }
    }
}

impl core::fmt::Debug for CalendarCoverage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CalendarCoverage")
            .field("identity", &self.source)
            .field("support_floor", &SUPPORT_FLOOR)
            .field("normal_week_sourced_from", &self.carried_below)
            .field("phase_gap", &self.phase_gap)
            .field("holidays", &self.holidays)
            .finish_non_exhaustive()
    }
}

impl PartialEq for CalendarCoverage {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
            && self.carried_below == other.carried_below
            && self.phase_gap == other.phase_gap
            && self.holidays == other.holidays
    }
}

impl Eq for CalendarCoverage {}

/// Returns the gap one maximal run of `reason` reports, or `None` when the run
/// is complete.
///
/// A run inside a declared phase-level gap carries the identity's declaration
/// with it, so [`CoverageGap::closing_condition`] is readable from the walk
/// rather than only from [`CalendarCoverage::phase_gap`]. The two are the same
/// value by construction: the reason came from that declaration.
const fn gap_of(
    coverage: CalendarCoverage,
    range: DateRange,
    reason: CoverageGapReason,
) -> CoverageGap {
    CoverageGap {
        range,
        reason,
        phase_gap: coverage.phase_gap,
    }
}
