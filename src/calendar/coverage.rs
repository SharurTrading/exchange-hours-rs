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
//! and [`CoverageGapReason::NormalWeekOnly`] name its five kinds. A **declared**
//! gap is not date-shaped: the operator publishes the arrangement, and no shipped
//! row states it. `schedules/sourcing.rs` declares those
//! per identity, and this module reports them as
//! [`CoverageGapReason::NormalWeekPhaseWithheld`] (#79),
//! [`CoverageGapReason::SpecialSessionUnrepresentable`] (#93) and
//! [`CoverageGapReason::UnpublishedClosureDates`] (#157).
//!
//! A declared gap applies to the whole supported domain **unless its own
//! declaration carries an end bound** ([`PhaseGap::until`]), because a profile can
//! start serving the withheld phase in a later era: the seven scopes withholding
//! CME's Sunday 16:00-16:15 CT quarter-hour each begin a knowledge-bound era on
//! 2026-08-22 that widens the queue to 16:00-17:00 CT, so the quarter-hour is
//! served from that day and the gap is bounded to the dated era before it. Where a
//! declaration is bounded, every verdict here follows the era: the dates inside
//! the bound keep the phase-level reason, and dates at or after it fall through
//! to the ordinary date-level facts — so [`CalendarCoverage::coverage_on`],
//! [`CalendarCoverage::is_complete_on`], [`CalendarCoverage::complete_ranges`]
//! and [`CalendarCoverage::gaps`] agree on where the gap stops. A whole-domain
//! declaration (no bound) leaves an identity incomplete everywhere, as does
//! shipping no holiday table at all — most identities do, though the three whose
//! own definition observes no holidays are complete without one.
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
    /// there, its holiday layer has no answer, or a declared phase-level gap
    /// applies on the date.
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
/// and are derived from the identity's own timeline and holiday table. Three are
/// **declared**, beside the identity in `schedules/sourcing.rs` (LAW-COVERAGE's
/// required-phase, special-session and holiday gaps): they apply to the
/// dates their declaration names — the whole claimed interval when it carries no
/// end bound — so a date walk over the tables can never find them. An identity
/// carrying a declared gap is incomplete wherever that gap applies, and
/// [`CoverageGap::closing_condition`] names what would discharge it. Two of the
/// three are phase-level, withholding a phase the crate's scalar rules do not
/// serve; [`Self::UnpublishedClosureDates`] withholds no phase and is a
/// completeness fact alone.
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
    /// required phase the calendar withholds, so no date its declaration covers
    /// is answered from a complete normal week.
    ///
    /// The shape is a slice of one phase's boundary: the operator publishes the
    /// phase, the crate's scalar rules serve only the part of it that holds under
    /// every sourced state, and the remainder depends on a change this crate
    /// cannot date (LAW-NO-FABRICATED-DATES). Seven **served** scopes are the
    /// declared case for the same quarter-hour — CME's Sunday 16:00-16:15 CT
    /// queue, withheld in favour of the 16:15-17:00 CT intersection the crate
    /// carries from its 2010 floor (#79) — and the owners' evidence files record
    /// the undated 2012 move it depends on: `cme`, `comex`, `nymex`,
    /// `globex_energy`, `globex_equity_index`, `globex_fx` and
    /// `globex_interest_rates`. Four **dormant** identities show the same shape
    /// and declare nothing, their gaps recorded in their own evidence files
    /// instead: `globex_weather`, `globex_gold_tas`, `globex_silver_tas` and
    /// `globex_copper_tas`. Dormant coverage does not block release, and their
    /// eras differ — `globex_weather`'s knowledge-bound row is 2026-09-05, not
    /// 2026-08-22 — so the shared bound below is not theirs to reuse. Each
    /// declaration is **bounded to the dated era before each module's own
    /// knowledge-bound 2026-08-22 row**, which widens the queue to 16:00-17:00 CT
    /// and therefore serves the quarter-hour from that day on.
    /// `globex_cryptocurrency` carries a second instance of the shape, its
    /// five-day era's undated Pre-Open onset, which is bounded to no era: its
    /// evidence records no day the gap stops applying.
    ///
    /// Because the withheld slice lies inside a phase on a recurring grid rather
    /// than on one trade date, this is not [`Self::WithheldDate`]: it is not a
    /// date-level exception, it cannot be discharged by an
    /// [`ExceptionBlock`](crate::ExceptionBlock) row alone, and it does not move
    /// with the identity's holidays.
    NormalWeekPhaseWithheld,
    /// **Declared phase-level.** The identity's operator publishes at least one
    /// session no shipped row states, so no date its declaration covers is
    /// answered from a complete calendar.
    ///
    /// The shape is a *whole session that the modelled week has no slot for* —
    /// an extra session on a weekday the normal week does not trade, or a
    /// trade-date arrangement the scalar rows cannot key. Unlike the rest of this
    /// vocabulary it is not a withheld answer on a date the caller can see: the
    /// date may be answered plausibly by the ordinary week, which is exactly why
    /// it is a completeness gap rather than an error at query time.
    ///
    /// The closing condition is a replacement-block row for the date. The
    /// vocabulary such a row needs has shipped — the replacement-block engine of
    /// LAW-HOLIDAY-SCOPE landed in Stage 3 (#93), and
    /// [`ExceptionBlock`](crate::ExceptionBlock) already reaches callers, so a
    /// caller can supply the session the crate has no row for. The variant keeps
    /// the name it has always had, because renaming it would break a consumer
    /// that matches on it.
    ///
    /// `globex_cryptocurrency` is the remaining shipped case: its 24/7-era
    /// sessions and merged trade dates are unstated. `globex_fx` carried this
    /// reason until its rows landed — the operator rows and their evidence in
    /// Stage 4 (#116), the merged trade dates in Stage 5 — after which every
    /// session CME publishes for that family is stated and it declares only the
    /// `#79` quarter-hour.
    SpecialSessionUnrepresentable,
    /// **Declared, not phase-level.** The operator declares closures inside this
    /// identity's own product scope but has not dated them, so no date its
    /// declaration covers is certified complete.
    ///
    /// The shape is an **undated closure scope**, and it differs from both
    /// declared variants above. No phase is withheld: every phase the crate
    /// models for an ordinary day is served, so this declaration is a
    /// completeness fact alone and the query gate answers through it rather than
    /// refusing an order-entry queue it does not touch. Nor is a row missing from
    /// the vocabulary: a closure is [`HolidayKind::Closed`], which ships. What is
    /// missing is the *date* — the operator's own calendar names a scope that
    /// closes and prints `tba` where its dates belong — so no date-level row can
    /// state it and no date walk over the identity's tables can find it.
    ///
    /// `eurex` is the shipped case. The *Eurex trading calendar 2025* prints
    /// `Kein Handel und keine Ausübung in deutschen Aktien- und
    /// Aktienindex-derivaten sowie in ETF- und ETC-Derivaten, die auf
    /// Xetra@-Börsen-notierungen basieren: tba.`, and the 2026 edition carries
    /// the same note in English and still says `to be announced`; FDAX and FDXM
    /// are German equity-index derivatives this identity serves, so both years
    /// could carry closures no shipped row states. The withholding is the
    /// **operator's**, not this crate's: the Holiday regulations page never
    /// carries the note, so it cannot be closed from that page, and
    /// LAW-PRIMARY-SOURCES forbids closing it from a T3 restatement. The closing
    /// condition is an Eurex announcement or Trading Calendar edition that dates
    /// the German-scope closures.
    ///
    /// Because no phase is withheld, a declared span is *not* a phase gap: the
    /// order-entry queue scans of `CalendarQueryContext::require_phase_coverage`
    /// answer through it.
    UnpublishedClosureDates,
}

/// A completeness gap one identity declares about itself: a known internal gap
/// that no date walk over the identity's tables can find.
///
/// Two shapes are declared here. A **phase-level** gap withholds part of a
/// phase the crate's scalar rules do not serve
/// ([`CoverageGapReason::NormalWeekPhaseWithheld`],
/// [`CoverageGapReason::SpecialSessionUnrepresentable`]). A **holiday-scope**
/// gap ([`CoverageGapReason::UnpublishedClosureDates`]) withholds no phase at
/// all: the operator's calendar names dates it closes without publishing them,
/// so the site is incomplete while every phase still answers.
///
/// Declared in `schedules/sourcing.rs` beside the identity it belongs to, never
/// inferred from a timeline or a holiday table. LAW-COVERAGE requires complete
/// coverage to contain "no unresolved normal-week, required-phase, holiday or
/// special-session gap"; the first two of those are shapes the tables under
/// `schedules/` cannot express today, so they are stated affirmatively, exactly
/// as `observes_no_holidays` states the absence of holiday closures.
///
/// A declaration is bounded when the profile serves the withheld arrangement from
/// a later era on: [`Self::until`] is then the first date the gap no longer
/// applies, and it is the same day the identity's own timeline begins the profile
/// that serves it. The bound is never invented — it restates a knowledge-bound
/// revision row the module already ships (LAW-NO-FABRICATED-DATES).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhaseGap {
    /// Which phase-level gap the identity carries.
    reason: CoverageGapReason,
    /// The issue whose closure would discharge it, as the declaration in
    /// `schedules/sourcing.rs` and the coverage inventory write it (`#79`,
    /// `#93`, `#123`).
    closing_condition: &'static str,
    /// The first venue-local date from which the gap no longer applies, or
    /// `None` when it applies to the whole supported domain.
    until: Option<NaiveDate>,
}

impl PhaseGap {
    /// Declares a phase-level gap and the issue that closes it.
    ///
    /// The declaration covers the whole supported domain; [`Self::until`] narrows
    /// it to the era in which the identity still withholds the arrangement.
    #[must_use]
    pub const fn new(reason: CoverageGapReason, closing_condition: &'static str) -> Self {
        Self {
            reason,
            closing_condition,
            until: None,
        }
    }

    /// Bounds the gap to the era **before** `until`.
    ///
    /// `until` is the first venue-local date on which the gap no longer applies —
    /// for the seven scopes withholding the Sunday quarter-hour, the 2026-08-22
    /// knowledge-bound row each module carries, which widens the Sunday queue to
    /// 16:00-17:00 CT and so serves the quarter-hour. A date at or after the bound
    /// is judged by the ordinary date-level facts instead.
    ///
    /// Public so a caller can compose a declaration of its own over
    /// [`Self::new`]'s whole-domain default; the crate's own declarations live in
    /// `schedules/sourcing.rs`.
    #[must_use]
    pub const fn until(self, until: NaiveDate) -> Self {
        Self {
            until: Some(until),
            ..self
        }
    }

    /// Returns which phase-level gap the identity carries.
    #[must_use]
    pub const fn reason(self) -> CoverageGapReason {
        self.reason
    }

    /// Returns the issue whose closure would discharge the gap, as the
    /// declaration in `schedules/sourcing.rs` and
    /// `docs/schedules/coverage-2025.md` write it.
    ///
    /// A static string, never a `String`: the metadata stays allocation-free and
    /// `Copy`.
    #[must_use]
    pub const fn closing_condition(self) -> &'static str {
        self.closing_condition
    }

    /// Returns the first venue-local date from which the gap no longer applies,
    /// or `None` when the identity withholds the arrangement for the whole
    /// supported domain.
    ///
    /// `None` is not missing data: it is the affirmative "this profile never
    /// serves the arrangement", which is what the inventory's `Missing /
    /// disputed` cell records for a scope whose operator still publishes it. A
    /// `Some` bound is the day a knowledge-bound revision row begins serving it.
    #[must_use]
    pub const fn applies_until(self) -> Option<NaiveDate> {
        self.until
    }

    /// Returns whether this declaration applies to venue-local `date`.
    #[must_use]
    pub fn applies_on(self, date: NaiveDate) -> bool {
        self.until.is_none_or(|until| date < until)
    }
}

/// One span inside the supported domain this identity cannot answer completely.
///
/// A **date-shaped** record comes from the walk over the identity's timeline and
/// holiday windows and carries no declaration. A **phase-level** record reports
/// one of the identity's declared gaps over the span that declaration is the
/// answer for: the whole supported domain for a declaration with no bound, the era
/// before its [`PhaseGap::until`] day for a bounded one, and what a bounded
/// declaration left for a whole-domain one that follows it. Because the shapes
/// stack, a declaration that no earlier one shadows reports its own record; a
/// shadowed one has none, so `globex_cryptocurrency`'s `#93` precedes its `#123`
/// and only `#93` reaches [`CalendarCoverage::gaps`].
/// [`CalendarCoverage::phase_gaps`] is the declaration list itself.
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
    /// **phase-level** gap declared per identity spans the dates its own
    /// declaration is the answer for: the whole supported domain when the
    /// declaration carries no bound and none precedes it, the era before its
    /// [`PhaseGap::until`] day when it does, or the dates a bounded predecessor
    /// left when a whole-domain declaration follows one.
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
    /// requires a gap to carry: the issue whose closure would discharge it, as the
    /// declaration in `schedules/sourcing.rs` and
    /// `docs/schedules/coverage-2025.md` write it. A record reports one
    /// declaration, and only the first one applying to a date reaches
    /// [`CalendarCoverage::gaps`] — a declaration an earlier one shadows has no
    /// record there, so read [`CalendarCoverage::phase_gaps`] for the whole list
    /// at once.
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
/// reasons. An identity that declares a **whole-domain phase-level** gap in
/// `schedules/sourcing.rs` reports no complete range at all. Its `#79`-style
/// declarations are reported from [`Self::gaps`] only where they are the answer;
/// from the day such a declaration stops applying the date-level walk supplies
/// the records instead, so `cme` reports both its declaration's span and its own
/// withheld dates. Where several declarations overlap, the one that answers first
/// takes the span and a shadowed one has no record of its own — which is
/// `globex_cryptocurrency`'s shape, whose `#93` precedes its `#123`. Bounding one
/// declaration does not make the identity complete: one that also carries an
/// unbounded declaration is still outside covered range after the bound, which is
/// `globex_fx`'s shape.
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
    phase_gaps: &'static [PhaseGap],
    holidays: HolidayContract,
    table: Option<&'static HolidayTable>,
}

impl CalendarCoverage {
    /// Builds the metadata one identity's calendar reports.
    ///
    /// `holidays_attached` is the calendar's own flag: a detached calendar
    /// reports the normal-week contract ([`HolidayContract::NormalWeekOnly`])
    /// and claims no complete calendar, while its normal-week side is unchanged.
    /// An identity whose own definition observes no holidays keeps
    /// [`HolidayContract::NoHolidays`] either way — there is no table to detach —
    /// and keeps its complete range.
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
            phase_gaps: declared.phase_gaps,
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

    /// Returns the phase-level gaps this identity declares about itself, in
    /// declaration order; an empty slice means it declares none.
    ///
    /// This is the completeness fact no date walk can derive: the identity's
    /// normal week or calendar carries an arrangement no shipped row states, so
    /// it is incomplete on every date the declaration covers —
    /// the whole claimed interval unless [`PhaseGap::applies_until`] bounds it to
    /// the era in which the identity still withholds it. An empty slice is an
    /// affirmative "no such gap declared", not missing data — the declarations
    /// live in `schedules/sourcing.rs`, one arm per identity, and are never
    /// inferred from a timeline or a holiday table. The accessor keeps the name
    /// of the phase-level shape it was introduced for; `eurex`'s undated closure
    /// scope is declared here too, and it withholds no phase.
    ///
    /// A scope can carry several because the shapes stack: `globex_fx`
    /// withholds the Sunday quarter-hour *and* publishes special sessions no
    /// shipped row states. Each of these carries its own reason and
    /// closing condition. [`Self::gaps`] reports a declaration over the span it
    /// answers for, and only where no earlier declaration shadows it.
    #[must_use]
    pub const fn phase_gaps(self) -> &'static [PhaseGap] {
        self.phase_gaps
    }

    /// Returns the first declaration that applies to venue-local `date`, or
    /// `None` when none does.
    pub(in crate::calendar) fn phase_gap_on(self, date: NaiveDate) -> Option<PhaseGap> {
        self.phase_gaps
            .iter()
            .copied()
            .find(|gap| gap.applies_on(date))
    }

    /// Returns the coverage verdict for venue-local `date`.
    ///
    /// A date inside a declared **phase-level** gap reports
    /// [`DateCoverage::OutsideCoveredRange`], not a new verdict: LAW-COVERAGE
    /// makes an unresolved normal-week or special-session gap the same
    /// "no sourced answer" case as a carried horizon, and Stage 2B maps it to
    /// the same [`CalendarQueryError::OutsideCoveredRange`]. A declaration its
    /// identity has bounded with [`PhaseGap::until`] applies only before that day,
    /// so a date at or after the bound is judged by the ordinary date-level facts
    /// — the day the profile began serving the withheld phase. The distinction
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
                | CoverageGapReason::SpecialSessionUnrepresentable
                | CoverageGapReason::UnpublishedClosureDates,
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
    /// The spans are disjoint and separated by [`Self::complete_ranges`]. Each
    /// declared **phase-level** gap that some date has as its answer is reported
    /// once, over the span it is the answer for — the era before its [`PhaseGap::until`] day
    /// for a bounded declaration, the whole supported domain for one that carries
    /// no bound and has none before it, or what a bounded predecessor left for a
    /// whole-domain declaration that follows one. A whole-domain declaration an
    /// earlier one already answers for on every date has no record here, because
    /// no date has it as its answer; read [`Self::phase_gaps`] for the whole
    /// declaration list. The record's reason and closing condition are the
    /// declaration's own.
    #[must_use]
    pub fn gaps(self) -> CoverageGaps {
        CoverageGaps::new(self)
    }

    /// Returns the reason `date` is not complete, or `None` when it is.
    ///
    /// Below the floor nothing is a gap: the supported domain starts there. A
    /// declared phase-level gap is checked **first**, because it states a fact the
    /// timeline and holiday walk cannot carry: where one applies, an identity has
    /// no covered date whatever its tables say. A declaration an era bound has
    /// retired ([`PhaseGap::until`]) is not consulted for a later date at all, so
    /// the date is decided by the ordinary facts. The first declaration applying
    /// to the date supplies the reason; every declaration is listed by
    /// [`Self::phase_gaps`], and by [`Self::gaps`] where no earlier one shadows
    /// it.
    pub(super) fn gap_reason_on(self, date: NaiveDate) -> Option<CoverageGapReason> {
        if date < SUPPORT_FLOOR {
            return None;
        }
        if let Some(phase_gap) = self.phase_gap_on(date) {
            return Some(phase_gap.reason());
        }
        self.date_level_gap_on(date)
    }

    /// Returns the **date-level** reason `date` is not complete, ignoring any
    /// declared phase-level gap.
    ///
    /// A declared gap describes a withheld **phase**, not a withheld date, so it
    /// cannot by itself make a whole venue-local day unanswerable: the crate
    /// still serves that day's normal week and holiday layer, and only a query
    /// whose answer *is* that phase has no sourced value. Stage 2B's query gate
    /// therefore consults this method — never [`Self::gap_reason_on`] — because
    /// refusing a Tuesday afternoon for a Sunday queue that the Tuesday query
    /// never reads is exactly the "coverage error read as a market closure"
    /// failure LAW-COVERAGE exists to prevent. The entry points that do probe
    /// the withheld phase ask [`Self::phase_gap_on`] themselves.
    pub(super) fn date_level_gap_on(self, date: NaiveDate) -> Option<CoverageGapReason> {
        if date < SUPPORT_FLOOR {
            return None;
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
    /// edges of every audited window, every withheld date and the day after it, and
    /// **both edges of every declaration that carries a bound** — all static and
    /// bounded, so the walk allocates nothing.
    ///
    /// A declaration bounded with [`PhaseGap::until`] contributes both of its
    /// edges, not just the bound: a run walk that stopped only at the bound could
    /// straddle the two eras, because the run that *starts* on the last date before
    /// the bound ends there as well. A whole-domain declaration contributes no edge
    /// — it applies to every date, so its own run can end only where another
    /// static edge does, and a scope declaring both shapes (`globex_fx`) still
    /// walks its bounded declaration's era correctly.
    fn next_boundary_after(self, date: NaiveDate) -> Option<NaiveDate> {
        let mut best: Option<NaiveDate> = None;
        let mut consider = |candidate: Option<NaiveDate>| {
            best = [best, candidate].into_iter().flatten().min();
        };
        consider((SUPPORT_FLOOR > date).then_some(SUPPORT_FLOOR));
        consider(self.carried_below.filter(|boundary| *boundary > date));
        for gap in self.phase_gaps {
            consider(gap.applies_until().filter(|bound| *bound > date));
            consider(
                gap.applies_until()
                    .and_then(|bound| bound.pred_opt())
                    .filter(|last| *last > date),
            );
        }
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
            .field("phase_gaps", &self.phase_gaps)
            .field("holidays", &self.holidays)
            .finish_non_exhaustive()
    }
}

impl PartialEq for CalendarCoverage {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
            && self.carried_below == other.carried_below
            && self.phase_gaps == other.phase_gaps
            && self.holidays == other.holidays
    }
}

impl Eq for CalendarCoverage {}

/// Returns the gap one maximal run of `reason` reports.
///
/// Declarations are reported from [`CoverageGaps`](super::CoverageGaps) before
/// the walk, and the walk still runs for the days no declaration answers, so a
/// walk record never carries a declaration and a declaring identity reports
/// both — `cme` yields its declaration's span and its own withheld dates.
const fn gap_of(range: DateRange, reason: CoverageGapReason) -> CoverageGap {
    CoverageGap {
        range,
        reason,
        phase_gap: None,
    }
}
