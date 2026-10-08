// SPDX-License-Identifier: MIT-0

//! What a calendar can answer, and the vocabulary for what it cannot
//! (LAW-COVERAGE).
//!
//! The support floor is fixed at **1 January 2010 in each venue's own
//! local-date domain** ([`SUPPORT_FLOOR`]) — never a rolling window, and never a
//! single UTC midnight, because a Tokyo session's 2010-01-01 begins while
//! Chicago is still on 2009-12-31. At or above that floor an identity-backed
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
//!    [`HolidayKind::Unsourced`](crate::HolidayKind::Unsourced);
//! 4. **answering the date completely stays inside the dates the identity
//!    answers**: a session can open on the previous civil day, and the next
//!    session's trade date can lie beyond it, so a date whose resolution
//!    reach crosses an unanswerable neighbour is reported as
//!    [`CoverageGapReason::ResolutionEdge`] rather than called complete
//!    (#151).
//!
//! [`CalendarCoverage::coverage_on`] reports the verdict for one venue-local
//! date; [`CalendarCoverage::complete_ranges`] and [`CalendarCoverage::gaps`]
//! report the same derivation as ascending spans and their reasons.
//! [`CalendarQueryError`] is the explicit error vocabulary Stage 2B's
//! identity-backed queries return instead of answering an unsupported date.
//!
//! A **date-shaped** gap is what the three facts above decide, and
//! [`CoverageGapReason::NormalWeekCarried`], [`CoverageGapReason::NoHolidayTable`],
//! [`CoverageGapReason::NoHolidayCoverage`], [`CoverageGapReason::WithheldDate`],
//! [`CoverageGapReason::NormalWeekOnly`] and
//! [`CoverageGapReason::HolidayWindowsBridged`] name its six kinds. A
//! **declared** gap is not date-shaped: the operator publishes an arrangement
//! no shipped row states exactly. `schedules/sourcing.rs` declares those
//! per identity; nothing declares one today, and the four shapes the shipped
//! scopes have retired are
//! [`CoverageGapReason::SpecialSessionUnrepresentable`] (#93),
//! [`CoverageGapReason::PostCloseQueueTradeDateLabel`] (#152),
//! [`CoverageGapReason::NormalWeekPhaseWithheld`] (#79, #123, #259) and
//! [`CoverageGapReason::UnpublishedClosureDates`] (#157).
//!
//! No declaration stands.
//! [`CoverageGapReason::UnpublishedClosureDates`]
//! withheld no phase — it stated that the operator names closures it has not
//! dated — so it was a completeness fact alone and the phase's own queries
//! answered through it. The four retired shapes each withheld something once:
//! a phase's own answer
//! ([`CoverageGapReason::NormalWeekPhaseWithheld`], whose declaration a query
//! whose answer was that phase could not answer through), a session no row
//! stated, a trade-date label, and an undated holiday scope that verified to
//! no closures at all. They retired in the order their resolutions
//! arrived — the special sessions when every scope's merged trade dates
//! shipped as rows, the label when the charter's Post-Close trade-date
//! convention resolved the divergence (2026-10-03, #152), the phase-level
//! shape when the charter's sourced-intersection residual convention resolved
//! #79, #123 and #259 (2026-10-04): a phase whose endpoints are sourced at two
//! values with only the changeover day undated, and whose dated-artifact hunts
//! have closed negative, is served as the sourced intersection with the
//! disputed remainder disclosed as a residual in the owner's evidence file,
//! never refused as a declaration — and the holiday scope when the no-changes
//! verification of 2026-10-05 established that the `tba` closures the Eurex
//! 2025/2026 editions never dated never existed: the operator's day-by-day
//! Holiday regulations tables state no German-scope closure on any date of
//! either year, every candidate date has passed answering ordinary, and the
//! regulation channel such a closure would travel is enumerated complete and
//! empty of it (#157). All four stay on the enum as the
//! vocabulary a future gap of the same shape would declare, which is also why
//! the pass-through gate arms below remain.
//!
//! A declared gap applies to the dates its own declaration names, not to the
//! whole supported domain. Three bounds narrow it: a **start bound**
//! ([`PhaseGap::since`], the first venue-local date the withholding is live —
//! the era the gap is a property of, never an inferred cutover), an **end
//! bound** ([`PhaseGap::until`], the first date the profile serves the withheld
//! arrangement), and a **shape** ([`PhaseGapShape`]) — `EveryDay`, or an
//! order-entry window whose occurrence on the identity's own calendar decides
//! the individual dates. Where a
//! declaration is bounded or shaped, every verdict here follows it: the dates
//! it names keep the phase-level reason, and all others fall through to the
//! ordinary date-level facts — so [`CalendarCoverage::coverage_on`],
//! [`CalendarCoverage::is_complete_on`], [`CalendarCoverage::complete_ranges`]
//! and [`CalendarCoverage::gaps`] agree date by date. A whole-domain
//! declaration (no bounds, `EveryDay`) leaves an identity incomplete wherever
//! its ordinary facts would otherwise answer, as does shipping no holiday table
//! at all — most identities do, though the three whose own definition observes
//! no holidays are complete without one. The #172 date-scoping this paragraph
//! describes was exercised by the three declarations the 2026-10-04
//! convention retired: the #79 quarter-hour was shaped to the served Sunday
//! Pre-Open window, and the retirement moved those Sundays to the ordinary
//! date-level facts their neighbours already answered from.
//!
//! Nothing here changes an existing query's signature. Inspectable metadata is
//! not permission to return a fabricated schedule: a date this module reports as
//! outside the covered range has no sourced answer, and the caller's overlay is
//! the only layer that can supply one.

mod baseline;
mod error;
mod ranges;

pub use baseline::NormalWeekBaseline;
pub use error::CalendarQueryError;
pub use ranges::{CompleteRanges, CoverageGaps};

use chrono::NaiveDate;

use super::exchange_calendar::CalendarSource;
use super::schedules::holidays::{self, HolidayCoverage, HolidayKind, HolidayTable};
use super::schedules::sourcing;
use super::schedules::timeline::effective_date;

/// The permanent support floor, **1 January 2010 in the venue's own local-date
/// domain** (LAW-COVERAGE; the 2026-09-27 amendment moved it back from
/// 2025-01-01).
///
/// This is a *local* date, not a UTC instant: the instant at which an identity
/// reaches its floor depends on that identity's IANA zone, and a query's instant
/// is judged by the venue-local date it falls on. The floor is fixed and never a
/// rolling previous-year window. A later sourced launch stays the identity's own
/// coverage start (`docs/schedules/coverage-2025.md`).
pub const SUPPORT_FLOOR: NaiveDate = effective_date(2010, 1, 1);

/// The most days one resolution-reach walk may visit before it gives up and
/// withholds the `Covered` claim ([`CalendarCoverage::
/// resolution_reach_answerable`]).
///
/// The walk crosses only session-day-free, answered dates — weekends and
/// audited closures. The longest such run any shipped table states is a
/// week-long new-year or lunar-new-year closure; the bound is an order of
/// magnitude above it, and exceeding it means the identity's tables state a
/// closure run this crate never audited, so the conservative verdict (not
/// completely answerable) invents nothing.
const RESOLUTION_WALK_BOUND: u32 = 400;

/// The distance beyond which a date is fast-pathed to "answerable reach" in
/// [`CalendarCoverage::resolution_reach_answerable`].
///
/// The reach walk only runs long through **session-day-free, answered** dates —
/// weekends and audited closures — and it stops at the first session day. The
/// longest session-day-free chain any shipped table states is a week-long
/// new-year or lunar-new-year closure, so a date more than a few weeks from
/// every unanswerable day cannot flip, and the shortcut answers it with three
/// binary searches instead of a profile selection per probed day. The full walk
/// still runs inside the radius, so the shortcut can only ever move a verdict
/// for an identity whose grid carries no session day for sixty-three straight
/// answered dates — no shipped profile does, and the metadata/query agreement
/// fence in `tests/coverage_metadata.rs` would catch one that starts to.
const RESOLUTION_FAST_RADIUS: u32 = 64;

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
    /// The date is inside this identity's complete covered calendar: every
    /// question about every instant of it answers, and any refusal a query
    /// there can still raise names a **neighbouring** date this vocabulary
    /// itself does not call covered (the resolution-edge promise, #151).
    Covered,
    /// The date precedes the venue-local [`SUPPORT_FLOOR`].
    BeforeSupportFloor,
    /// The date is at or after the floor but outside the ranges this identity
    /// has a sourced answer for: its weekday profile is carried backwards
    /// there, its holiday layer has no answer there, a declared phase-level gap
    /// applies on the date, its holiday layer is the bridged residual of an
    /// unaudited span the sourced week crosses
    /// ([`CoverageGapReason::HolidayWindowsBridged`] — the session layer still
    /// answers there, and the holiday-table classification answers beside the
    /// residual on a two-flank span but refuses typed on a one-flank span
    /// below the first window), or answering it completely would consult a
    /// neighbouring date the identity does not answer
    /// ([`CoverageGapReason::ResolutionEdge`], #151). A date refused for the
    /// last reason — or for the bridged residual — may still answer the
    /// questions that need only its own facts; what the metadata withholds is
    /// the claim that *every* query on it answers.
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
    /// **The bridged residual (issue #296, Tier 1; the 2026-10-07 Tier-2
    /// ruling extends it below the first window).** The date sits in an
    /// **unaudited span between audited windows** of the identity's shipped
    /// holiday table — a window ends before it and another begins after it —
    /// or **below the identity's first audited window**, whose normal week the
    /// identity sources; the table's holiday layer is honestly absent there,
    /// while the identity's sourced normal week answers across the span.
    ///
    /// This is the charter's sourced-intersection convention
    /// ("an undated changeover is a disclosed residual, not a refused day")
    /// generalized from one undated changeover to a whole evidence span: the
    /// maintainer's principle of 2026-10-05 is that sourced hours on both
    /// sides of an evidence gap are not refused wholesale, and the 2026-10-07
    /// ruling extends it to a span with only one flank **below** — a date
    /// whose window begins after it but none of whose ends before it. The
    /// dates inside either span answer their session questions from the normal
    /// week the timeline serves — the state that holds under every sourced
    /// state — while the metadata withholds the complete-calendar claim and
    /// reports this reason instead. The residual is a disclosure beside a
    /// served answer, never a fabricated "no holiday": no closure is asserted
    /// that no operator statement witnesses, and no open is asserted silently —
    /// the span verdict says the holiday layer is absent, and a witnessed
    /// arrangement (the operator declaring the same recurring closure every
    /// observed year, the `coinbase_derivatives` 2022 Thanksgiving shape)
    /// arrives as table data with its own audited window, which shrinks the
    /// bridge as the family sweeps land.
    ///
    /// **The two span kinds differ at the holiday-table classification, and
    /// that asymmetry is the 2026-10-07 ruling's substance.** Two flanks
    /// bracket a span; one does not. A two-flank date's classification
    /// (`is_closed_trade_date` and every query that reads the holiday table's
    /// classification) answers `Ok(false)` beside the residual — on both
    /// flanks the sourced windows witness the ordinary layer. A one-flank
    /// date's classification refuses a typed
    /// [`CalendarQueryError::UnresolvedGap`], enriched with the
    /// [`NormalWeekBaseline`](crate::NormalWeekBaseline) the date sits inside,
    /// because nothing witnesses the holiday layer there and the crate's
    /// accuracy bar ("nothing answered where it cannot back") forbids a
    /// "not closed" answer.
    ///
    /// A span **above the last window** is not this reason and never was: the
    /// operator's publication horizon governs there (LAW-HOLIDAY-SCOPE), so
    /// those dates are not an evidence gap and keep their whole-date refusal
    /// as [`Self::NoHolidayCoverage`].
    ///
    /// Unlike [`Self::NoHolidayCoverage`], this reason is **not a refusal**
    /// for the session layer: the date-level gate answers both kinds, and the
    /// record exists so a consumer walking [`CalendarCoverage::gaps`] sees
    /// that the holiday layer — not the session layer — is what the span
    /// lacks. [`DateCoverage`] reports the date as
    /// [`DateCoverage::OutsideCoveredRange`] for exactly that reason, and the
    /// queries' own refusals there name the neighbouring unanswerable dates
    /// as anywhere else.
    HolidayWindowsBridged,
    /// **Declared phase-level — retired as a declaration, kept as vocabulary.**
    /// The identity's sourced normal week contains a required phase the calendar
    /// withholds, so no date its declaration covers is answered from a complete
    /// normal week.
    ///
    /// The shape is a slice of one phase's boundary: the operator publishes the
    /// phase, the crate's scalar rules serve only the part of it that holds under
    /// every sourced state, and the remainder depends on a change this crate
    /// cannot date (LAW-NO-FABRICATED-DATES). Three declarations of this shape
    /// stood while no dated artifact existed: seven **served** scopes withheld
    /// CME's Sunday 16:00-16:15 CT queue in favour of the 16:15-17:00 CT
    /// intersection carried from the 2010 floor (#79), `globex_cryptocurrency`
    /// withheld its five-day era's undated Pre-Open onset (#123), and
    /// `globex_grains` withheld the 2012-05-20..2013-04-06 regime whose queue
    /// states its captures print (#259). The charter's sourced-intersection
    /// residual convention retired all three on 2026-10-04 (AGENTS.md,
    /// "Modeling conventions", the 2026-10-04 decision): each was a phase whose
    /// endpoints were sourced at two values with only the changeover day
    /// undated, and every dated-artifact hunt had closed negative, so the dates
    /// now answer from the served intersection — from the regime's own dated
    /// start for #259, whose queue rows ship in `grains.rs` — while the disputed
    /// remainders are disclosed as residuals in the owners' evidence files, each
    /// with a named closer. No scope declares this reason any more, but the
    /// variant stays: it is the vocabulary a future required-phase gap would
    /// declare — one whose hunts have not closed negative, whose span no sourced
    /// state pins, or whose served answer would be wrong under a sourced state —
    /// and the refusing gate arms in this module and `query::gate` keep treating
    /// it as a reason that withholds a phase.
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
    /// `globex_cryptocurrency` was the last shipped case: its 24/7-era merged
    /// trade dates shipped as rows on 2026-09-26 UTC, and no scope has declared
    /// the reason since. `globex_fx` carried it until its rows landed — the
    /// operator rows and their evidence in Stage 4 (#116), the merged trade
    /// dates in Stage 5.
    SpecialSessionUnrepresentable,
    /// **Declared phase-level — retired as a declaration, kept as vocabulary.**
    /// The operator prints a trade date on its post-close order-entry queue
    /// that the crate does not, so one answer a caller reads from a covered
    /// date — the trade date — would be the crate's own convention rather than
    /// the operator's label, and a scope carrying the reason could not claim a
    /// complete calendar.
    ///
    /// An order-entry occurrence is dated by the session it feeds
    /// (`next_session_after_with`), so a date's `14:30-16:00` CT Post-Close
    /// queue reads with the **next** trade date while CME's own service prints
    /// it carrying the date the queue is printed on. The charter resolved that
    /// divergence on 2026-10-03 (AGENTS.md, "Trade dates and state", closing
    /// #152): the operator's own T1 Post-Close notice (Globex notice 20160530)
    /// describes the queue as order entry "for the next trade date" that
    /// "should not be considered an extension of the current day trading
    /// session", so the session-fed dating is the operator's own prose, and the
    /// T2 service's per-event labels are the divergent printing the convention
    /// resolves. No scope declares this reason any more — the declarations
    /// `globex_grains` and `globex_livestock` carried are retired — but the
    /// variant stays: it is the vocabulary a future labelling divergence would
    /// declare, and the pass-through arms in this module and in
    /// `query::gate` keep treating it as a reason whose phase is served.
    ///
    /// **A declaration under this reason withholds no answer, so it refuses no
    /// query.** The window, its `is_open` verdict and its `is_accepting_orders`
    /// verdict are all sourced and served; only the label differs. The gate in
    /// `query::schedule::require_phase_coverage` therefore passes this reason
    /// through, unlike the two reasons above, whose phases the crate does not
    /// carry at all. A replacement-block row cannot supply the operator's label
    /// either: the only row shape that yields it is `tradeable`, which would
    /// assert matching in a window the operator marks `pcp`
    /// (LAW-SESSION-NOT-EXPIRY) — the measurement that settled the retired
    /// declaration's shape.
    PostCloseQueueTradeDateLabel,
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
    /// `eurex` was the shipped case, from the declaration's introduction until
    /// its 2026-10-05 retirement. The *Eurex trading calendar 2025* prints
    /// `Kein Handel und keine Ausübung in deutschen Aktien- und
    /// Aktienindex-derivaten sowie in ETF- und ETC-Derivaten, die auf
    /// Xetra@-Börsen-notierungen basieren: tba.`, and the 2026 edition carried
    /// the same note in English still saying `to be announced`; FDAX and FDXM
    /// are German equity-index derivatives this identity serves, so both years
    /// could have carried closures no shipped row stated. The verification the
    /// maintainer set on 2026-10-05 established there were none: the operator's
    /// day-by-day Holiday regulations tables — whose grammar printed the
    /// German-scope clause, futures carve-out and all, in 2020 — state no
    /// German-scope closure on any date of either year, every candidate date
    /// has passed answering ordinary, and the circular channel that dated every
    /// 2014-2018 German-scope closure is enumerated complete with no such item
    /// (#157, closed). The reason stays on the enum as the vocabulary a future
    /// undated holiday scope would declare — an operator calendar that names a
    /// closure set it has not dated, whose no-changes verification has not been
    /// made.
    ///
    /// Because no phase is withheld, a declared span of this shape is *not* a
    /// phase gap: the order-entry queue scans of
    /// `CalendarQueryContext::require_phase_coverage` answer through it.
    UnpublishedClosureDates,
    /// **Date-shaped, not declared.** Answering this date completely needs a
    /// **neighbouring date the identity does not answer**: a session can open on
    /// the previous civil day (the wrapped evening leg), the next session's
    /// trade date can lie beyond it, and the gap classification between
    /// sessions reads the trade dates on both sides — so a date whose own
    /// facts are sourced is still not fully answerable when the reach of its
    /// queries crosses the edge of an audited window or a withheld date
    /// (#151).
    ///
    /// The shape is the **resolution reach**: the dates
    /// [`DateCoverage`]'s five questions consult for any instant of the date —
    /// from the last session day before it (whose wrapped leg an instant at
    /// local midnight lands in) through the first session day at or after it
    /// and that session's trade date. Where every date in that reach answers
    /// at the date level, the date is [`DateCoverage::Covered`]; where one
    /// does not — an audited window starts or ends inside the reach, or a
    /// withheld date sits inside it — the date is
    /// [`DateCoverage::OutsideCoveredRange`] and this is the recorded reason.
    /// A query addressed to such a date may still answer (the questions that
    /// need only the date's own facts do); what the metadata refuses is the
    /// claim that *every* query on the date answers, so a consumer walking a
    /// range never walks into a surprise refusal. Every refusal a query on a
    /// `Covered` date can still raise names a date this vocabulary already
    /// flags, which is the agreement `tests/coverage_metadata.rs` fences.
    ResolutionEdge,
}

/// The dates one declared gap applies to, within its own span.
///
/// A declaration's span ([`PhaseGap::since`] through [`PhaseGap::until`]) states
/// the era the gap is a property of; the shape states **which dates inside that
/// era** the withholding is live on. The default is every date of the span. An
/// order-entry window narrows it to the dates whose own calendar resolves an
/// order-entry occurrence of exactly that window — the truthful granularity for
/// a gap that withholds one phase on one weekday (#79's Sunday Pre-Open
/// quarter-hour and #152's post-close trade-date label were the two shaped
/// declarations before their 2026-10-03 and 2026-10-04 retirements), where the
/// rest of the span's dates are fully answered by the tables.
///
/// A shape is evaluated against the identity's own built-in layers only — its
/// profile timeline, its holiday table, never a caller's overlay — because the
/// declaration is a fact about the identity, and the same value must be
/// reported by every view of it, detached calendars included.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhaseGapShape {
    /// Every date of the declaration's span. The shape for a gap whose
    /// withholding is not keyed to a served occurrence: an era whose queue rows
    /// are omitted outright (`globex_grains`' 2012-05-20..2013-04-06 regime) or
    /// a scope whose closure dates the operator has not published (`eurex`).
    EveryDay,
    /// The dates on which the identity's own calendar resolves an
    /// **order-entry** occurrence opening on the date, of a rule whose
    /// venue-local window matches exactly: `close_ssm` always, and `open_ssm`
    /// when it is `Some`. `None` matches any opening — the retired #152
    /// declaration used it because the post-close queue's opening moved across
    /// eras while its 16:00 CT close did not.
    ///
    /// The occurrence must survive the built-in layers — a trade date the
    /// table closes or replaces removes the occurrence, and the date then
    /// answers completely. Resolution is the same walk the order-entry queries
    /// run, so a shape can never apply where the crate itself serves nothing.
    OrderEntryWindow {
        /// The rule's venue-local opening, in seconds since local midnight, or
        /// `None` to match any opening with the named close.
        open_ssm: Option<u32>,
        /// The rule's venue-local close, in seconds since local midnight.
        close_ssm: u32,
    },
}

/// A completeness gap one identity declares about itself: a known internal gap
/// that no date walk over the identity's tables can find.
///
/// Three shapes are declared here. A **phase-level** gap withholds part of a
/// phase the crate's scalar rules do not serve
/// ([`CoverageGapReason::NormalWeekPhaseWithheld`],
/// [`CoverageGapReason::SpecialSessionUnrepresentable`]). A **labelling** gap
/// ([`CoverageGapReason::PostCloseQueueTradeDateLabel`]) serves every phase and
/// states that one answer read from them, the trade date, is the crate's
/// convention rather than the operator's printing — the shape the charter's
/// 2026-10-03 Post-Close decision retired, kept for a future divergence of the
/// same kind. A **holiday-scope** gap
/// ([`CoverageGapReason::UnpublishedClosureDates`]) withholds no phase at all:
/// the operator's calendar names dates it closes without publishing them, so the
/// site is incomplete while every phase still answers — the shape `eurex`'s
/// `tba` era declared until the 2026-10-05 no-changes verification retired it
/// (#157), kept for a future undated holiday scope.
///
/// Declared in `schedules/sourcing.rs` beside the identity it belongs to, never
/// inferred from a timeline or a holiday table. LAW-COVERAGE requires complete
/// coverage to contain "no unresolved normal-week, required-phase, holiday or
/// special-session gap"; the first two of those are shapes the tables under
/// `schedules/` cannot express today, so they are stated affirmatively, exactly
/// as `observes_no_holidays` states the absence of holiday closures.
///
/// A declaration is **bounded** when the era it is about is dated on either
/// side: [`Self::since`] is the first date the withholding is live (the
/// five-day era's own first day, for `globex_cryptocurrency`'s undated
/// Pre-Open), and [`Self::until`] the first date the profile serves the
/// withheld arrangement — the day a knowledge-bound revision row the module
/// already ships begins (LAW-NO-FABRICATED-DATES: the bound restates a row,
/// never an inference). A declaration is **shaped** when the withholding is
/// live only on the dates a served occurrence decides ([`PhaseGapShape`]) —
/// as #152's label was, before the charter retired it, on the dates that
/// carried the post-close queue, and as #79's quarter-hour was on the Sundays
/// whose Pre-Open resolved, until the 2026-10-04 convention retired that too.
/// Span and shape compose, so a
/// declaration names exactly the dates its evidence cannot answer and the
/// identity's metadata answers every other date beside it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhaseGap {
    /// Which phase-level gap the identity carries.
    reason: CoverageGapReason,
    /// The issue whose closure would discharge it, as the declaration in
    /// `schedules/sourcing.rs` and the coverage inventory write it (`#93`,
    /// `#157`).
    closing_condition: &'static str,
    /// The first venue-local date on which the gap applies, or `None` when it
    /// applies from the support floor.
    since: Option<NaiveDate>,
    /// The first venue-local date from which the gap no longer applies, or
    /// `None` when no era that serves the arrangement is known.
    until: Option<NaiveDate>,
    /// Which dates inside the span the withholding is live on.
    shape: PhaseGapShape,
}

impl PhaseGap {
    /// Declares a phase-level gap and the issue that closes it.
    ///
    /// The declaration covers every date of the supported domain its evidence
    /// withholds — every date, until [`Self::since`], [`Self::until`] or a
    /// [`PhaseGapShape`] narrows it.
    #[must_use]
    pub const fn new(reason: CoverageGapReason, closing_condition: &'static str) -> Self {
        Self {
            reason,
            closing_condition,
            since: None,
            until: None,
            shape: PhaseGapShape::EveryDay,
        }
    }

    /// Bounds the gap to the era **at and after** `since`.
    ///
    /// `since` is the first venue-local date on which the withholding is live —
    /// the first day of the era the gap is a property of, dated by a row the
    /// module already ships (`globex_cryptocurrency`'s five-day grid launches
    /// 2017-12-17 at T1, so its Pre-Open gap applies from that day and not to
    /// the sourced launch closures before it). A date before the bound is
    /// judged by the ordinary date-level facts.
    ///
    /// Public so a caller can compose a declaration of its own over
    /// [`Self::new`]'s whole-domain default; the crate's own declarations live
    /// in `schedules/sourcing.rs`.
    #[must_use]
    pub const fn since(self, since: NaiveDate) -> Self {
        Self {
            since: Some(since),
            ..self
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
    /// [`Self::new`]'s whole-domain default; the crate's own declarations live
    /// in `schedules/sourcing.rs`.
    #[must_use]
    pub const fn until(self, until: NaiveDate) -> Self {
        Self {
            until: Some(until),
            ..self
        }
    }

    /// Narrows the gap to the dates a served occurrence decides
    /// ([`PhaseGapShape::OrderEntryWindow`]).
    ///
    /// Public so a caller can compose a declaration of its own; the crate's own
    /// declarations live in `schedules/sourcing.rs`.
    #[must_use]
    pub const fn with_shape(self, shape: PhaseGapShape) -> Self {
        Self { shape, ..self }
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

    /// Returns the first venue-local date on which the gap applies, or `None`
    /// when it applies from the support floor.
    ///
    /// `None` is not missing data: it states that no dated era bounds the
    /// withholding from below, which is the truth for a gap whose phase the
    /// operator has published since before the floor. A `Some` bound is a day a
    /// sourced row already carries (LAW-NO-FABRICATED-DATES).
    #[must_use]
    pub const fn applies_since(self) -> Option<NaiveDate> {
        self.since
    }

    /// Returns the first venue-local date from which the gap no longer applies,
    /// or `None` when no era that serves the arrangement is known.
    ///
    /// `None` is not missing data: it is the affirmative "this profile never
    /// serves the arrangement", which is what the inventory's `Missing /
    /// disputed` cell records for a scope whose operator still publishes it. A
    /// `Some` bound is the day a knowledge-bound revision row begins serving it.
    #[must_use]
    pub const fn applies_until(self) -> Option<NaiveDate> {
        self.until
    }

    /// Returns which dates inside the span the withholding is live on.
    #[must_use]
    pub const fn shape(self) -> PhaseGapShape {
        self.shape
    }

    /// Returns whether this declaration's **span** contains venue-local `date`.
    ///
    /// This is the span check alone. The full "does the gap apply here" test
    /// also consults the shape and the identity's own tables, and it lives on
    /// [`CalendarCoverage::phase_gap_on`], which is the only reader that has
    /// them.
    #[must_use]
    pub fn applies_on(self, date: NaiveDate) -> bool {
        self.since.is_none_or(|since| date >= since) && self.until.is_none_or(|until| date < until)
    }
}

/// One span inside the supported domain this identity cannot answer completely.
///
/// A **date-shaped** record comes from the walk over the identity's timeline and
/// holiday windows and carries no declaration. A **phase-level** record reports
/// one of the identity's declared gaps over one maximal span the declaration
/// answers for: a bounded `EveryDay` declaration reports its era as one span, a
/// date-scoped declaration reports one span per run its shape resolves — #79's
/// quarter-hour came back as the bracket-era Sundays, one record each, until
/// its 2026-10-04 retirement — and a
/// whole-domain declaration spans everything no earlier declaration took.
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
    /// **phase-level** gap declared per identity spans one maximal run its own
    /// declaration is the answer for — the whole supported domain when the
    /// declaration is whole-domain and unshadowed, the era between its bounds
    /// when it is bounded, or the single dates its shape resolves, which is why
    /// a date-scoped declaration's records arrive one per run.
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
    /// the declaration in `schedules/sourcing.rs` and
    /// `docs/schedules/coverage-2025.md` write it. A record reports one
    /// declaration; only the first declaration applying to a date reaches
    /// [`CalendarCoverage::gaps`] — a shadowed one has no record there, so read
    /// [`CalendarCoverage::phase_gaps`] for the whole list at once.
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

/// Which kind of unaudited span a date outside every audited window sits in.
///
/// Private to the coverage engine: the kind decides which questions the span's
/// dates answer, never the metadata verdict they report — both kinds read as
/// [`DateCoverage::OutsideCoveredRange`] with
/// [`CoverageGapReason::HolidayWindowsBridged`] as the reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BridgeSpanKind {
    /// A window ends before the date and another begins after it — both
    /// flanks exist, and the span between them is the charter's bridged
    /// residual (2026-10-06, issue #296 Tier 1). The session questions and
    /// the holiday-table classification both answer, the classification
    /// beside the disclosed residual.
    TwoFlank,
    /// A window begins after the date and none ends before it — the date sits
    /// **below the family's first audited window** (the 2026-10-07 Tier-2
    /// ruling, issue #296). The session questions answer from the sourced
    /// normal week exactly as on a two-flank span, but the holiday-table
    /// classification refuses: a one-flank date has no bracket, nothing
    /// witnesses the holiday layer, and the crate's accuracy bar forbids a
    /// "not closed" answer there. A one-flank span **above** the last window
    /// is deliberately absent from this enum — the operator's publication
    /// horizon governs there, those dates are not an evidence gap, and they
    /// keep their whole-date refusal.
    OneFlankBelow,
}

/// What one identity's calendar can answer, as of the shipped data.
///
/// Built by [`ExchangeCalendar::coverage`](crate::ExchangeCalendar::coverage);
/// it borrows the identity's static tables, so reading it allocates nothing and
/// the value stays `Copy + Send + Sync + 'static`. Every range it reports is
/// clipped to [`SUPPORT_FLOOR`], the permanent 2010-01-01 local-date floor.
///
/// [`Self::coverage_on`] is the per-date verdict, [`Self::complete_ranges`] the
/// spans that answer completely, and [`Self::gaps`] the rest with their
/// reasons. An identity that declares a **whole-domain** gap in
/// `schedules/sourcing.rs` reports no complete range at all. The bounded and
/// shaped declarations answer only the dates they name — the era before a
/// `PhaseGap::until` bound, the Sundays an order-entry shape resolves — and
/// every other date is decided by the ordinary date-level facts, so `cme`
/// reports its bracket-era Sundays and answers its Tuesdays beside them. Where
/// several declarations overlap, the first that applies takes the span and a
/// shadowed one has no record of its own. Bounding one declaration does not
/// make the identity complete: one that also carries an unbounded declaration
/// is still outside covered range wherever that declaration applies.
///
/// The spans are **derived, not duplicated**: the iterators walk the identity's
/// static timeline horizon and holiday-window edges in ascending order and stop
/// at the next edge, so reading the metadata allocates nothing and never sorts.
/// Inside the span of a date-scoped declaration the walk merges verdicts day by
/// day, because the shape can flip them mid-week; the region is bounded by the
/// declaration's own era. Deriving everything per call is also what lets a
/// detached calendar report the normal-week contract from the very same value.
#[derive(Clone, Copy)]
pub struct CalendarCoverage {
    source: CalendarSource,
    carried_below: Option<NaiveDate>,
    phase_gaps: &'static [PhaseGap],
    holidays: HolidayContract,
    /// The table this **view** consults: the identity's shipped table, or
    /// `None` once the calendar detached it.
    table: Option<&'static HolidayTable>,
    /// The table the identity **ships**, detached or not.
    ///
    /// A declared gap is a fact about the identity, so its shape must resolve
    /// the same way on every view of it: the shape asks whether the identity's
    /// own calendar resolves a served occurrence, which is a question about the
    /// shipped tables even when this view answers only the normal-week
    /// contract. The attached walk never reads past [`Self::table`]; this field
    /// is the shape machinery's alone.
    shipped: Option<&'static HolidayTable>,
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
            shipped,
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
    /// it is incomplete on every date the declaration covers — the dates its
    /// bounds and shape name, not the whole claimed interval. An empty slice is
    /// an
    /// affirmative "no such gap declared", not missing data — the declarations
    /// live in `schedules/sourcing.rs`, one arm per identity, and are never
    /// inferred from a timeline or a holiday table. The accessor keeps the name
    /// of the phase-level shape it was introduced for; `eurex`'s undated closure
    /// scope is declared here too, and it withholds no phase.
    ///
    /// A scope can carry several because the shapes stack: `globex_fx`
    /// withholds the Sunday quarter-hour *and* publishes special sessions no
    /// shipped row states. Each of these carries its own reason and
    /// closing condition. [`Self::gaps`] reports a declaration over each maximal
    /// span it answers for, and only where no earlier declaration shadows it.
    #[must_use]
    pub const fn phase_gaps(self) -> &'static [PhaseGap] {
        self.phase_gaps
    }

    /// Returns the first declaration that **applies** to venue-local `date`, or
    /// `None` when none does.
    ///
    /// A declaration applies when three tests hold, in cost order: its span
    /// contains the date ([`PhaseGap::applies_on`]), the identity's own facts
    /// would otherwise answer the date — where they already refuse it, that
    /// refusal is the operative reason and the declaration adds nothing — and
    /// its shape resolves against the identity's built-in layers. The first
    /// declaration passing all three is the answer for the date, in declaration
    /// order. The "own facts" test is the `identity_answers` logic below, so a
    /// detached view reports the same declarations the attached one does.
    #[must_use]
    pub fn phase_gap_on(self, date: NaiveDate) -> Option<PhaseGap> {
        if !self.identity_answers(date) {
            return None;
        }
        self.phase_gaps
            .iter()
            .copied()
            .find(|gap| gap.applies_on(date) && self.shape_resolves(*gap, date))
    }

    /// Returns whether the **identity** — this view's calendar, whatever layers
    /// it detached — would answer `date` from its ordinary date-level facts.
    ///
    /// An attached view answers exactly when [`Self::date_level_gap_on`] finds
    /// no refusal. A detached view claims no complete calendar
    /// ([`HolidayContract::NormalWeekOnly`]) for every date, which is the view's
    /// contract and not the identity's fact, so the question is re-asked against
    /// the table the identity ships: the same carried-below horizon, withheld
    /// dates and audited windows the attached view reads. This is what keeps a
    /// declared gap's shape reporting the same records on every view of the
    /// identity.
    fn identity_answers(self, date: NaiveDate) -> bool {
        if self.holidays != HolidayContract::NormalWeekOnly {
            return self.date_level_gap_on(date).is_none();
        }
        if date < SUPPORT_FLOOR {
            return true;
        }
        if self.carried_below.is_some_and(|carried| date < carried) {
            return false;
        }
        match self.shipped {
            None => false,
            Some(table) => {
                !Self::withholds_shipped(table, date) && table.coverage().contains(date)
                    || Self::bridge_span_kind(self.windows(), self.carried_below, date).is_some()
            }
        }
    }

    /// Returns whether the shipped table withholds `date` as `Unsourced`.
    fn withholds_shipped(table: &'static HolidayTable, date: NaiveDate) -> bool {
        table
            .holiday_on(date)
            .is_some_and(|holiday| holiday.kind() == HolidayKind::Unsourced)
    }

    /// Returns whether any declaration whose span contains `date` withholds a
    /// phase **without** keying the withholding to a served occurrence — an
    /// `EveryDay` declaration whose reason refuses. An omitted-queue era
    /// leaves no rule for a scan to match — #123's five-day grid was the
    /// shipped case until its 2026-10-04 retirement — so every
    /// order-entry query inside such an era consults the question the crate
    /// cannot answer and the scan gate has to fire whether or not a rule
    /// exists.
    pub(in crate::calendar) fn has_unscoped_refusing_phase_gap_on(self, date: NaiveDate) -> bool {
        self.phase_gaps.iter().copied().any(|gap| {
            gap.applies_on(date)
                && gap.shape() == PhaseGapShape::EveryDay
                && !matches!(
                    gap.reason(),
                    CoverageGapReason::PostCloseQueueTradeDateLabel
                        | CoverageGapReason::UnpublishedClosureDates
                )
        })
    }

    /// Returns whether `gap`'s shape resolves on `date`, against the built-in
    /// layers only.
    ///
    /// The declaration is a fact about the identity, so a caller's overlay is
    /// deliberately out of the question: the same value must be reported by the
    /// attached calendar, a detached one, and the metadata iterators. A
    /// resolution error withholds — a shape that cannot prove the date answers
    /// keeps the gap applied, which is the conservative direction.
    fn shape_resolves(self, gap: PhaseGap, date: NaiveDate) -> bool {
        match gap.shape() {
            PhaseGapShape::EveryDay => true,
            PhaseGapShape::OrderEntryWindow {
                open_ssm,
                close_ssm,
            } => crate::calendar::query::schedule::builtin_resolves_order_entry(
                self.source,
                date,
                open_ssm,
                close_ssm,
            ),
        }
    }

    /// Returns whether any declaration carries an
    /// [`PhaseGapShape::OrderEntryWindow`] shape whose span reaches this
    /// segment — the walk's signal that verdicts inside the segment can change
    /// from date to date and must be merged by day rather than taken whole.
    ///
    /// The reach test is the declaration's span, and the segment must be a
    /// **finite, answered** one: past the last audited window the ordinary facts
    /// refuse every date uniformly, so no shape can flip anything and the
    /// static-edge walk resumes. Every shipped shaped declaration sits inside an
    /// audited window; a shaped declaration on a table-less identity would have
    /// no such bound and is not expressible here.
    pub(super) fn has_date_scoped_shapes_between(self, first: NaiveDate, last: NaiveDate) -> bool {
        let Some(table) = self.shipped else {
            return false;
        };
        if last == NaiveDate::MAX {
            return false;
        }
        let answered_through = table
            .windows
            .iter()
            .filter_map(|&(.., last_year, last_month, last_day)| {
                NaiveDate::from_ymd_opt(last_year, last_month, last_day)
            })
            .max();
        let Some(answered_through) = answered_through else {
            return false;
        };
        last <= answered_through
            && self.phase_gaps.iter().any(|gap| {
                matches!(gap.shape, PhaseGapShape::OrderEntryWindow { .. })
                    && gap.applies_on(first)
                    && gap.applies_on(last)
            })
    }

    /// Returns the coverage verdict for venue-local `date`.
    ///
    /// A date a declared **phase-level** gap applies to reports
    /// [`DateCoverage::OutsideCoveredRange`], not a new verdict: LAW-COVERAGE
    /// makes an unresolved normal-week or special-session gap the same
    /// "no sourced answer" case as a carried horizon, and Stage 2B maps it to
    /// the same [`CalendarQueryError::OutsideCoveredRange`]. A declaration
    /// applies only where its own bounds and shape name the date
    /// ([`Self::phase_gap_on`]), so a Tuesday in the bracket era answers from
    /// the tables while the bracket-era Sunday beside it refuses, and a date at
    /// or after a `PhaseGap::until` bound is judged by the ordinary date-level
    /// facts — the day the profile began serving the withheld phase. The
    /// distinction between the two lives in [`Self::gaps`], which carries the
    /// closing condition a caller needs in order to tell "not worked up yet"
    /// from "answered, and the answer is ordinary".
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
                | CoverageGapReason::HolidayWindowsBridged
                | CoverageGapReason::NormalWeekPhaseWithheld
                | CoverageGapReason::SpecialSessionUnrepresentable
                | CoverageGapReason::PostCloseQueueTradeDateLabel
                | CoverageGapReason::UnpublishedClosureDates
                | CoverageGapReason::ResolutionEdge,
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
    /// The spans are disjoint and separated by [`Self::complete_ranges`]; the
    /// last one may be open-ended. Every span whose reason is a declared
    /// **phase-level** gap carries that declaration
    /// ([`CoverageGap::phase_gap`]), and a shaped declaration reports one span
    /// per maximal run its shape resolves — #79's quarter-hour was reported as
    /// the bracket-era Sundays one day at a time until its 2026-10-04
    /// retirement, because the dates between them
    /// answer. A whole-domain declaration that another declaration shadows on
    /// every date has no record, because no date has it as its answer; read
    /// [`Self::phase_gaps`] for the whole declaration list. The record's reason
    /// and closing condition are the declaration's own.
    #[must_use]
    pub fn gaps(self) -> CoverageGaps {
        CoverageGaps::new(self)
    }

    /// Returns the reason `date` is not complete, or `None` when it is.
    ///
    /// Below the floor nothing is a gap: the supported domain starts there. A
    /// declared phase-level gap is checked **first**, because it states a fact the
    /// timeline and holiday walk cannot carry: where one applies, the date is
    /// incomplete whatever its tables say. "Applies" is
    /// [`Self::phase_gap_on`]'s full test — the span, the ordinary facts, and
    /// the shape — so a declaration a bound has retired, or whose served
    /// occurrence the date does not carry, is not consulted for that date at
    /// all. The first declaration applying to the date supplies the reason;
    /// every declaration is listed by [`Self::phase_gaps`], and by [`Self::gaps`]
    /// where no earlier one shadows it.
    ///
    /// A date whose own facts answer is then judged, in order, by the **bridge
    /// residual** and its **resolution reach**
    /// ([`Self::resolution_reach_answerable`]). The bridge residual
    /// ([`CoverageGapReason::HolidayWindowsBridged`]) is the date's own fact —
    /// between two audited windows the holiday layer is honestly absent — while
    /// the reach consults neighbouring dates, so the bridge is reported first
    /// and a date that both bridges and reaches an unanswerable neighbour
    /// reports the bridge. Answering every question about every instant of a
    /// date consults neighbouring dates, and a date whose reach crosses an
    /// unanswerable one is reported as
    /// [`CoverageGapReason::ResolutionEdge`] rather than called complete (#151).
    pub(super) fn gap_reason_on(self, date: NaiveDate) -> Option<CoverageGapReason> {
        if date < SUPPORT_FLOOR {
            return None;
        }
        if let Some(phase_gap) = self.phase_gap_on(date) {
            return Some(phase_gap.reason());
        }
        if let Some(reason) = self.date_level_gap_on(date) {
            return Some(reason);
        }
        if Self::bridge_span_kind(self.windows(), self.carried_below, date).is_some() {
            return Some(CoverageGapReason::HolidayWindowsBridged);
        }
        if !self.resolution_reach_answerable(date) {
            return Some(CoverageGapReason::ResolutionEdge);
        }
        None
    }

    /// Returns whether the identity's built-in calendar resolves a surviving
    /// tradeable occurrence opening on `day` — the reach walk's "session day".
    ///
    /// A session day is a day whose profile grid carries a tradeable rule for
    /// its weekday and whose holiday layer does not remove that day's complete
    /// grid (a `Closed` row removes every occurrence belonging to the day, and a
    /// replacement row replaces it with blocks this conservative test cannot
    /// prove resolve). Early closes and late opens survive, so they keep the day
    /// a session day. The test reads the identity's shipped table, never a
    /// caller's overlay, because the reach is a fact about the identity.
    fn surviving_session_day(self, day: NaiveDate) -> bool {
        if !crate::calendar::query::schedule::builtin_has_tradeable_rule(self.source, day) {
            return false;
        }
        match self.shipped.and_then(|table| table.holiday_on(day)) {
            Some(holiday) => !matches!(
                holiday.kind(),
                HolidayKind::Closed | HolidayKind::ReplacementBlocks(_)
            ),
            None => true,
        }
    }

    /// Returns whether answering **every** question about every instant of
    /// `date` stays inside dates the identity answers at the date level (#151).
    ///
    /// The reach is what the query surface actually consults. Backward: the
    /// previous-session walk processes `date` and every older day down to the
    /// last session day whose occurrence it reports, plus that day's own
    /// predecessor (a wrapped opening), and its trade-date probes read one day
    /// before that. Forward: the next-session walk processes every day from
    /// `date` to the first session day with a surviving occurrence, and the
    /// found session's trade date can be that day's successor. The walk stops
    /// at the first unanswerable day — a refusal there is exactly what the
    /// query surface raises — and treats a walk past its bound as unanswerable,
    /// the conservative direction: no shipped table chains that many
    /// session-day-free dates, and withholding the `Covered` claim invents
    /// nothing.
    ///
    /// A date further than the walk's own bound from every unanswerable day
    /// cannot flip, and answers so without any walk: the shortcut asks the same
    /// question the walk would, one binary search per static source, and no
    /// shipped profile leaves a session-day-free chain long enough to disagree
    /// with it.
    fn resolution_reach_answerable(self, date: NaiveDate) -> bool {
        if !self.unanswerable_within(date, Some(RESOLUTION_FAST_RADIUS)) {
            return true;
        }
        // Backward reach: `date` itself is answerable (the caller checked), so
        // walk to the last session day at or before it and require its own
        // predecessor.
        let mut day = date;
        let mut steps: u32 = 0;
        loop {
            if self.surviving_session_day(day) {
                match day.pred_opt() {
                    Some(prev) if self.identity_answers(prev) => break,
                    Some(_) => return false,
                    None => break,
                }
            }
            match day.pred_opt() {
                Some(prev) if self.identity_answers(prev) => day = prev,
                Some(_) => return false,
                None => break,
            }
            steps += 1;
            if steps > RESOLUTION_WALK_BOUND {
                return false;
            }
        }
        // Forward reach: walk to the first session day at or after `date` and
        // require its successor (the found session's trade date).
        let mut day = date;
        let mut steps: u32 = 0;
        loop {
            if self.surviving_session_day(day) {
                match day.succ_opt() {
                    Some(next) if self.identity_answers(next) => break,
                    Some(_) => return false,
                    None => break,
                }
            }
            match day.succ_opt() {
                Some(next) if self.identity_answers(next) => day = next,
                Some(_) => return false,
                None => break,
            }
            steps += 1;
            if steps > RESOLUTION_WALK_BOUND {
                return false;
            }
        }
        true
    }

    /// Returns whether an unanswerable day sits within `bound` days of `date`
    /// in either direction.
    ///
    /// The unanswerable days are the dates below the carried-below horizon, the
    /// dates outside every audited window — including a bridged span the bridge
    /// now serves, whose edges only trigger the conservative full walk —
    /// and the withheld rows. All three
    /// sources are sorted static tables, so this is three binary searches and
    /// no walk — the fast path behind
    /// [`Self::resolution_reach_answerable`].
    fn unanswerable_within(self, date: NaiveDate, bound: Option<u32>) -> bool {
        let within = |candidate: Option<NaiveDate>| {
            candidate.is_some_and(|candidate| {
                bound.is_none_or(|bound| {
                    candidate.signed_duration_since(date).num_days().abs() <= i64::from(bound)
                })
            })
        };
        if within(self.carried_below) {
            return true;
        }
        // Outside-window land begins the day before the first window's first
        // date and the day after each window's last.
        for &(first_year, first_month, first_day, last_year, last_month, last_day) in self.windows()
        {
            if let Some(first) = NaiveDate::from_ymd_opt(first_year, first_month, first_day)
                && within(first.pred_opt())
            {
                return true;
            }
            if let Some(last) = NaiveDate::from_ymd_opt(last_year, last_month, last_day)
                && within(last.succ_opt())
            {
                return true;
            }
        }
        if let Some(table) = self.table {
            let index = table.rows.partition_point(|row| row.trade_date < date);
            // Walk the rows on both sides of `date` out to the radius: rows are
            // sorted, so each direction stops at the first row past it. (A
            // single row on the near side of the partition is not enough — a
            // closed day can sit between the date and the withheld row beside
            // it.)
            let mut back = index;
            while back > 0 {
                let row = &table.rows[back - 1];
                let days = date.signed_duration_since(row.trade_date).num_days();
                if days > i64::from(RESOLUTION_FAST_RADIUS) {
                    break;
                }
                if row.kind == HolidayKind::Unsourced {
                    return true;
                }
                back -= 1;
            }
            for row in &table.rows[index..] {
                let days = row.trade_date.signed_duration_since(date).num_days();
                if days > i64::from(RESOLUTION_FAST_RADIUS) {
                    break;
                }
                if row.kind == HolidayKind::Unsourced {
                    return true;
                }
            }
        }
        false
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
    ///
    /// A date **between two audited windows** whose normal week the identity
    /// sources answers here (issue #296, Tier 1), and so does a date **below
    /// the first window** whose week is sourced (the 2026-10-07 Tier-2
    /// ruling): the bridge lifts the whole-date refusal this method used to
    /// raise for every date outside the windows, and the holiday layer's
    /// residual is reported by [`Self::gap_reason_on`] as
    /// [`CoverageGapReason::HolidayWindowsBridged`] instead — a disclosure
    /// beside the served session answers, never a fabricated normal date. The
    /// two span kinds differ one level up, at the holiday-table
    /// classification ([`Self::holiday_classification_refused_on`]): a
    /// two-flank date's classification answers beside the residual, a
    /// one-flank-below date's refuses.
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
                } else if Self::bridge_span_kind(self.windows(), self.carried_below, date).is_some()
                {
                    // The bridged span answers at the date level; the
                    // residual is `gap_reason_on`'s to report.
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

    /// Returns which kind of unaudited span venue-local `date` sits in, with
    /// the normal week sourced there — the bridged residual of issue #296
    /// (Tier 1 between windows; the 2026-10-07 Tier-2 ruling below the first
    /// window).
    ///
    /// Three conditions, in cost order: `date` lies outside every window the
    /// identity ships (the callers' audited-contains checks usually settle the
    /// opposite), the flanks decide the kind, and the carried-below horizon
    /// does not swallow the date, so the normal week the timeline serves
    /// across the span is sourced rather than carried. The windows are the
    /// build-validated ascending set, so this is one bounded scan of a handful
    /// of static tuples: allocation-free, total, and identical on every view
    /// of the identity.
    ///
    /// The one-flank-**above** shape — a window ends before the date and none
    /// begins after it — is deliberately not a span kind: above the last
    /// audited window the operator's publication horizon governs
    /// (LAW-HOLIDAY-SCOPE), so those dates are not an evidence gap and keep
    /// their whole-date refusal ([`CoverageGapReason::NoHolidayCoverage`]).
    /// The 2026-10-07 ruling bridges below-the-first-window spans only.
    fn bridge_span_kind(
        windows: &'static [(i32, u32, u32, i32, u32, u32)],
        carried_below: Option<NaiveDate>,
        date: NaiveDate,
    ) -> Option<BridgeSpanKind> {
        let mut lower_last: Option<NaiveDate> = None;
        let mut upper_first: Option<NaiveDate> = None;
        for &(first_year, first_month, first_day, last_year, last_month, last_day) in windows {
            let Some(first) = NaiveDate::from_ymd_opt(first_year, first_month, first_day) else {
                continue;
            };
            let Some(last) = NaiveDate::from_ymd_opt(last_year, last_month, last_day) else {
                continue;
            };
            if first <= date && date <= last {
                // Inside a window: audited, not bridged.
                return None;
            }
            if last < date {
                lower_last = Some(match lower_last {
                    Some(current) if current > last => current,
                    _ => last,
                });
            }
            if first > date {
                upper_first = Some(match upper_first {
                    Some(current) if current < first => current,
                    _ => first,
                });
            }
        }
        let kind = match (lower_last, upper_first) {
            (Some(_), Some(_)) => BridgeSpanKind::TwoFlank,
            // Below the first window: one flank, and the span the 2026-10-07
            // ruling bridges.
            (None, Some(_)) => BridgeSpanKind::OneFlankBelow,
            // Above the last window, or no windows at all: not bridged.
            (_, None) => return None,
        };
        // State compatibility: the flanking windows are audited states, and the
        // week the timeline serves across the span must itself be
        // sourced (LAW-COVERAGE) — a horizon inside the span keeps its own
        // dates carried, and they refuse through the ordinary facts.
        carried_below
            .is_none_or(|carried| date >= carried)
            .then_some(kind)
    }

    /// Returns whether the **holiday-table classification** of venue-local
    /// `date` refuses — the 2026-10-07 Tier-2 ruling on the bridged residual
    /// (issue #296).
    ///
    /// A date on a one-flank span below the identity's first audited window
    /// has no bracket: nothing witnesses the holiday layer there, so
    /// `is_closed_trade_date` refuses a typed
    /// [`CalendarQueryError::UnresolvedGap`] — enriched, per Tier 3, with the
    /// [`NormalWeekBaseline`](crate::NormalWeekBaseline) the date sits inside
    /// — instead of answering "not closed" from the normal week, while the
    /// same date's session questions answer from that week. A two-flank date
    /// keeps answering its classification beside the disclosed residual — two
    /// flanks bracket the span, one does not — and a date above the last
    /// window keeps its whole-date refusal, the publication horizon governing
    /// there. Only an attached audited table classifies: a detached view
    /// claims no holiday layer anywhere, so its contract is unchanged.
    pub(in crate::calendar) fn holiday_classification_refused_on(self, date: NaiveDate) -> bool {
        date >= SUPPORT_FLOOR
            && matches!(self.holidays, HolidayContract::Audited { .. })
            && Self::bridge_span_kind(self.windows(), self.carried_below, date)
                == Some(BridgeSpanKind::OneFlankBelow)
    }

    /// Returns the first venue-local date strictly after `date` at which the
    /// verdict can change, or `None` when no later boundary exists.
    ///
    /// The candidates are the support floor, the carried-below horizon, both
    /// edges of every declaration that carries a bound, both edges of every
    /// audited window, every withheld date and the day after it, and the two
    /// edges of every **resolution-reach zone** around a finite unanswerable
    /// day (below) — all static and bounded, so the walk allocates nothing. An
    /// ordinary holiday row needs no edge of its own: inside a date-scoped
    /// declaration's reach the walk judges each date on its own tables, and
    /// outside that reach no shape can flip a verdict.
    ///
    /// A declaration bounded with [`PhaseGap::since`] or [`PhaseGap::until`]
    /// contributes both of its edges, not just the bound: a run walk that
    /// stopped only at the bound could straddle the two eras, because the run
    /// that *starts* on the last date before the bound ends there as well. A
    /// whole-domain declaration contributes no edge — it applies to every date,
    /// so its own run can end only where another static edge does, and a scope
    /// declaring both shapes (`globex_fx`) still walks its bounded declaration's
    /// era correctly.
    fn next_boundary_after(self, date: NaiveDate) -> Option<NaiveDate> {
        let mut best: Option<NaiveDate> = None;
        let mut consider = |candidate: Option<NaiveDate>| {
            best = [best, candidate].into_iter().flatten().min();
        };
        consider((SUPPORT_FLOOR > date).then_some(SUPPORT_FLOOR));
        consider(self.carried_below.filter(|boundary| *boundary > date));
        // A date-scoped declaration can flip a verdict from date to date, so an
        // identity carrying one segments on the windows its **shipped** table
        // audits — a detached view included, because the declaration is a fact
        // about the identity and its day-merged reach is bounded by those
        // windows. The windows bound every identity's walk regardless: a
        // NoHolidayCoverage-to-Covered flip at a window's first day is exactly
        // the kind of verdict change only a window edge can end a run on.
        for gap in self.phase_gaps {
            for bound in [gap.applies_since(), gap.applies_until()] {
                consider(bound.filter(|bound| *bound > date));
                consider(
                    bound
                        .and_then(|bound| bound.pred_opt())
                        .filter(|last| *last > date),
                );
            }
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
        // A withheld date flips this view's verdict only where the view
        // consults the table at all.
        if let Some(table) = self.table {
            for row in table.rows {
                if row.kind == HolidayKind::Unsourced && row.trade_date >= SUPPORT_FLOOR {
                    consider(Some(row.trade_date).filter(|withheld| *withheld > date));
                    consider(row.trade_date.succ_opt().filter(|after| *after > date));
                }
            }
        }
        // The resolution-reach zones (#151): around every finite unanswerable
        // day, the verdict of the answered days whose queries consult it flips
        // to `ResolutionEdge`. The zone's edges are derived from the same
        // session-day walk the reach test runs, so a run claimed between two
        // reported edges carries one verdict across its whole span; the
        // per-date verification in `tests/coverage_metadata.rs` holds every
        // reported span to `coverage_on` date by date and would catch a missed
        // edge. A detached view's verdict never flips — `NormalWeekOnly`
        // covers every date uniformly and its gate never reaches the reach
        // test — so its zones would only split runs for nothing.
        if self.holidays == HolidayContract::NormalWeekOnly {
            return best;
        }
        if let Some(carried_below) = self.carried_below
            && let Some(before) = carried_below.pred_opt()
        {
            for edge in self.resolution_zone_edges(before) {
                consider(edge.filter(|edge| *edge > date));
            }
        }
        for &(first_year, first_month, first_day, last_year, last_month, last_day) in self.windows()
        {
            if let Some(first) = NaiveDate::from_ymd_opt(first_year, first_month, first_day)
                && let Some(before) = first.pred_opt()
            {
                for edge in self.resolution_zone_edges(before) {
                    consider(edge.filter(|edge| *edge > date));
                }
            }
            if let Some(last) = NaiveDate::from_ymd_opt(last_year, last_month, last_day)
                && let Some(after) = last.succ_opt()
            {
                for edge in self.resolution_zone_edges(after) {
                    consider(edge.filter(|edge| *edge > date));
                }
            }
        }
        if let Some(table) = self.table {
            // Only withheld rows near the frontier can contribute a boundary
            // past `date`: a zone's edges sit at most the walk bound plus two
            // days from its own unanswerable day, so earlier rows' zones end
            // before `date`. The rows are sorted, so the frontier is one
            // binary search.
            let first_relevant = table.rows.partition_point(|row| {
                row.trade_date
                    .checked_add_signed(chrono::Duration::days(
                        i64::from(RESOLUTION_WALK_BOUND) + 2,
                    ))
                    .is_some_and(|limit| limit <= date)
            });
            for row in &table.rows[first_relevant..] {
                if row.kind == HolidayKind::Unsourced {
                    for edge in self.resolution_zone_edges(row.trade_date) {
                        consider(edge.filter(|edge| *edge > date));
                    }
                }
            }
        }
        best
    }

    /// Returns the verdict-flip edges of the resolution zone around one finite
    /// unanswerable day `x` (#151).
    ///
    /// A day `d` below `x` flips when its forward session-day walk stops on a
    /// session day whose successor is `x`, or walks onto `x` itself: that is
    /// every day after the last session-or-unanswerable day at or before
    /// `x - 2`, so the zone's first day is the boundary candidate. A day `d`
    /// above `x` flips when its backward walk stops on a session day whose
    /// predecessor is `x`, or walks onto `x` itself: that is every day from
    /// `x + 1` up to the day before the first session-or-unanswerable day at
    /// or after `x + 2` — the non-session days between the two session days
    /// stop on the nearer one, whose predecessor is `x` — so that first day at
    /// or after `x + 2` is the other candidate. Candidates that name no
    /// answered day are dropped, and a candidate is only a *possible* edge: a
    /// run walk that reports it and finds one verdict across the span is still
    /// exact, and the per-date verification in `tests/coverage_metadata.rs`
    /// holds every reported span to `coverage_on` date by date.
    fn resolution_zone_edges(self, x: NaiveDate) -> [Option<NaiveDate>; 2] {
        // Below: walk back from two days before `x` to the last session day or
        // unanswerable day; the flip zone starts the day after it.
        let mut below = x.pred_opt().and_then(|first| first.pred_opt());
        if let Some(start) = below {
            let mut day = start;
            let mut steps = 0;
            loop {
                if !self.identity_answers(day) || self.surviving_session_day(day) {
                    break;
                }
                match day.pred_opt() {
                    Some(prev) => day = prev,
                    None => break,
                }
                steps += 1;
                if steps > RESOLUTION_WALK_BOUND {
                    break;
                }
            }
            below = day.succ_opt();
        }
        let below = below.filter(|edge| self.identity_answers(*edge));
        // Above: walk forward from two days after `x` to the first session day
        // or unanswerable day; the flip zone ends the day before it.
        let mut above = None;
        if let Some(start) = x.succ_opt().and_then(|first| first.succ_opt()) {
            let mut day = start;
            let mut steps = 0;
            loop {
                if !self.identity_answers(day) || self.surviving_session_day(day) {
                    break;
                }
                match day.succ_opt() {
                    Some(next) => day = next,
                    None => return [below, None],
                }
                steps += 1;
                if steps > RESOLUTION_WALK_BOUND {
                    return [below, None];
                }
            }
            above = Some(day);
        }
        let above = above.filter(|edge| self.identity_answers(*edge));
        [below, above]
    }

    /// Returns the audited holiday windows the identity **ships**, or an empty
    /// slice when it ships no table — a view that detached its table still
    /// segments on the shipped windows where a date-scoped declaration needs
    /// the bounds. Splitting a uniform run at one of them is harmless; walking
    /// past an edge that could have ended a day-merged run is not.
    fn windows(self) -> &'static [(i32, u32, u32, i32, u32, u32)] {
        self.shipped.map_or(&[], |table| table.windows)
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

/// Returns the gap one maximal run of `reason` reports when no declaration is
/// the answer for it.
///
/// The walk reports a declaration on the runs
/// [`CalendarCoverage::phase_gap_on`] names it on; a run no declaration answers
/// is decided by the ordinary date-level facts and carries none — `cme` yields
/// its bracket-era Sundays with the declaration and its own withheld dates
/// without one, in one ascending walk.
const fn gap_of(range: DateRange, reason: CoverageGapReason) -> CoverageGap {
    CoverageGap {
        range,
        reason,
        phase_gap: None,
    }
}
