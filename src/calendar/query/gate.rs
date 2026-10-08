// SPDX-License-Identifier: MIT-0

//! The compile-time split between a detached fixed snapshot and an
//! identity-backed calendar (issue #245).
//!
//! [`QueryContext`](super::schedule::QueryContext) is parameterised over
//! [`SourceGate`], and the trait pairs each source state with the coverage it
//! carries and the error its queries can raise:
//!
//! - [`FixedSnapshot`] — a detached caller-supplied snapshot. It carries **no
//!   coverage at all** (`Coverage` is `()`) and can raise **no error**
//!   (`Error` is [`Infallible`]): every refusal the engine knows is a fact
//!   about an identity's coverage metadata, and a snapshot carries none.
//! - [`Identified`] — a built-in calendar, bare or overlaid. It always
//!   carries its [`CalendarCoverage`] and reports every refusal through
//!   [`CalendarQueryError`] (LAW-COVERAGE).
//!
//! Two properties therefore hold structurally rather than by review:
//!
//! 1. **The invalid combination is unrepresentable.** There is no
//!    `coverage: None` state to reach: a context that can carry coverage
//!    always does, and a context without one cannot name a refusal, so a
//!    future gate cannot silently divide the states the way the former
//!    `Option<CalendarCoverage>` field could.
//! 2. **The fixed adapters' `Result` -> answer collapse cannot swallow an
//!    error.** The engine returns `Result<T, G::Error>`; for a fixed context
//!    that is `Result<T, Infallible>`, and [`answered`] unwraps it with a
//!    match that has **no error arm to write**. The collapse is unreachable
//!    from a coverage-carrying context at compile time, because
//!    [`CalendarQueryError`] does not unify with [`Infallible`] — the shape
//!    issue #245 required, replacing the `unwrap_or` collapse the #246 audit
//!    found documented-but-not-eliminated.
//!
//! The engine itself is written once against the trait — backtest and live
//! remain one code path — and monomorphises into the two states.

use std::convert::Infallible;

use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Tz;

use crate::calendar::coverage::{CalendarCoverage, CoverageGapReason, DateCoverage};
use crate::calendar::exchange_calendar::ExchangeCalendar;
use crate::calendar::hours::MarketHours;
use crate::calendar::local_time::{bounded_utc, mk_local_open};
use crate::calendar::{CalendarQueryError, CalendarSource, SUPPORT_FLOOR};

use super::schedule::ResolvedHours;

// Sessions opening on a civil day are governed by the profile in force at the
// end of that opening day. Midnight-keyed revisions select the same profile
// at any post-midnight anchor, so this only distinguishes sourced intraday
// cutovers: one that lands in an intraday gap after noon (ICE Canada's 18:30
// CT pre-open move) must govern the sessions opening later that day. The last
// second of the local day exists in every zone — DST transitions never
// collapse or duplicate 23:59:59 — and `mk_local_open` resolves earliest on
// ambiguity regardless.
const OPEN_DAY_ANCHOR_SSM: u32 = 86_399;

/// The source state a [`QueryContext`](super::schedule::QueryContext) is
/// compiled for: the coverage gate of issue #245.
///
/// The trait carries everything that differs between the two states — the
/// profile source itself, the coverage it claims, the error its queries can
/// raise, and the refusal gates — so the shared engine asks one vocabulary
/// and the compiler proves which refusals each state can produce.
pub(in crate::calendar) trait SourceGate: Copy {
    /// The profile source this state consults.
    type Source<'a>: Copy;

    /// The coverage metadata this state carries.
    type Coverage: Copy;

    /// The error a query over this state can produce.
    type Error;

    /// Returns the schedule identity behind the source, or `None` for a
    /// detached fixed snapshot.
    fn identity(source: Self::Source<'_>) -> Option<CalendarSource>;

    /// Selects the profile governing the sessions opening on `day`.
    fn profile_for_open_day(source: Self::Source<'_>, tz: Tz, day: NaiveDate) -> ResolvedHours<'_>;

    /// Returns whether the source's profile carries a final daily close at
    /// `instant`.
    fn has_daily_close_at(source: Self::Source<'_>, instant: DateTime<Utc>) -> bool;

    /// Returns whether the source's profile carries a weekend close at
    /// `instant`.
    fn has_weekend_close_at(source: Self::Source<'_>, instant: DateTime<Utc>) -> bool;

    /// Fails unless this source answers `date` completely (LAW-COVERAGE).
    ///
    /// This is Stage 2B's single coverage seam: every identity-backed query
    /// resolves the venue-local days it depends on through this gate, so the
    /// floor, an unsourced span and a withheld date are refused in one place
    /// instead of at each of the eighteen public entry points.
    ///
    /// The check is taken on the **dates the query needs**, never on the
    /// supplied instant alone: a caller asking what happens at 2025-01-01
    /// 00:30 local depends on the trading day that opened the previous
    /// evening, and a search that walks forward depends on every day it walks
    /// over (plan section 6).
    ///
    /// A declared **phase-level** gap does not refuse the date: it withholds a
    /// phase, not a day, so the day's normal week and holiday layer are still
    /// sourced and a query that never reads that phase has a real answer.
    /// Consulting the phase gap here would refuse a Tuesday afternoon for a
    /// Sunday queue — precisely the coverage-error-read-as-closure failure
    /// LAW-COVERAGE exists to prevent. The entry points that *do* probe the
    /// phase ask [`SourceGate::require_phase_coverage`] instead.
    ///
    /// Cost stays on the built-in hot path's budget: one comparison against a
    /// floor constant, then the same bounded static-table walk the holiday
    /// layer already performs, and no allocation.
    fn require_answerable(coverage: Self::Coverage, date: NaiveDate) -> Result<(), Self::Error>;

    /// Fails when the queried venue-local day precedes the support floor.
    ///
    /// The floor is a fact about the **queried** day, never about every day a
    /// scan walks over: the plan requires that a session opening before the
    /// floor is still returned whole to an in-range query, while a query
    /// addressed to an earlier day errors (LAW-COVERAGE, plan section 6).
    fn require_floor(coverage: Self::Coverage, date: NaiveDate) -> Result<(), Self::Error>;

    /// Fails when the venue-local day containing `instant` precedes the floor.
    ///
    /// Every instant-addressed query starts here, so the floor is decided from
    /// the caller's own instant rather than from each day a scan later walks
    /// over (see [`SourceGate::require_floor`]).
    fn require_floor_at(
        coverage: Self::Coverage,
        tz: Tz,
        instant: DateTime<Utc>,
    ) -> Result<(), Self::Error>;

    /// Fails unless this source answers the **withheld phase** on `date`.
    ///
    /// This is the strict sibling of [`SourceGate::require_answerable`], for
    /// the entry points whose answer *is* the arrangement a declared phase gap
    /// withholds: the order-entry queue scans. An identity that declares no
    /// phase gap is unaffected, so this costs one span check on that path.
    ///
    /// The declaration the metadata applies decides —
    /// [`CalendarCoverage::phase_gap_on`], the first declaration in order whose
    /// span contains the date *and* whose shape resolves against the built-in
    /// layers, which is the same shadowing rule `coverage_on` and
    /// [`CalendarCoverage::gaps`] report by. Deciding from the span alone would
    /// let an earlier pass-through declaration whose queue is absent on the
    /// date — the retired #152 shape resolved to no occurrence inside
    /// `globex_grains`'s omitted 2012-05-20..2013-04-06 regime, whose refusing
    /// declaration sits behind it — shadow the refusing declaration
    /// behind it, and a queue scan would answer absence where `coverage_on`
    /// refuses. A declared gap withholds a queue exactly when its reason names
    /// one: [`CoverageGapReason::NormalWeekPhaseWithheld`] and
    /// [`CoverageGapReason::SpecialSessionUnrepresentable`] do, and
    /// [`CoverageGapReason::PostCloseQueueTradeDateLabel`] and
    /// [`CoverageGapReason::UnpublishedClosureDates`] do not, because their
    /// phases are served — a labelling gap withholds a label, not a window, and
    /// an ordinary day's queues answer through Eurex's undated closures. (No
    /// shipped scope declares the labelling reason any more — the charter's
    /// 2026-10-03 Post-Close trade-date convention retired it (#152) — and the
    /// arm stays because the reason remains the vocabulary a future labelling
    /// divergence would declare.) A pass-through reason refuses nothing. A
    /// refusing
    /// reason refuses the scan: the #79 quarter-hour refused the bracket-era
    /// Sundays whose Pre-Open resolved, and answered the Tuesday beside one,
    /// until the charter's 2026-10-04 residual convention retired it —
    /// refusing a Tuesday for a Sunday queue is precisely the
    /// coverage-error-read-as-closure failure LAW-COVERAGE exists to prevent.
    /// An unrecognized reason refuses, which is the conservative direction: a
    /// new declaration shape answers no queue until it says so.
    fn require_phase_coverage(coverage: Self::Coverage, date: NaiveDate)
    -> Result<(), Self::Error>;

    /// Fails when the source's **holiday-table classification** of `date` is
    /// refused.
    ///
    /// This is the classification sibling of [`SourceGate::require_answerable`],
    /// added by the 2026-10-07 Tier-2 ruling on the bridged residual (issue
    /// #296): a date on a one-flank span **below the identity's first audited
    /// window** answers its session questions from the sourced normal week —
    /// `require_answerable` passes it — but its holiday classification refuses
    /// a typed `UnresolvedGap`, because a one-flank date has no bracket:
    /// nothing witnesses the holiday layer, and answering "not closed" from
    /// the normal week would fabricate one. A two-flank date keeps answering
    /// its classification beside the disclosed residual, and a date above the
    /// last window keeps its whole-date refusal (the publication horizon
    /// governs there, so this gate never fires on one). The floor is the
    /// caller's check, as for `require_answerable`.
    fn require_holiday_classification(
        coverage: Self::Coverage,
        date: NaiveDate,
    ) -> Result<(), Self::Error>;

    /// Returns whether the coverage metadata declares an unscoped refusing
    /// phase gap on `day`.
    ///
    /// This is the cheap half of the order-entry scan's gate: the scan consults
    /// [`SourceGate::require_phase_coverage`] only where this predicate, the
    /// day's own rules, or a nearby block row say the day could reach the
    /// withheld arrangement, so an ordinary Tuesday never pays the refusal
    /// check.
    fn has_unscoped_refusing_phase_gap_on(coverage: Self::Coverage, day: NaiveDate) -> bool;

    /// Gives the bounded-search-exhaustion verdict for the close walks.
    ///
    /// Running out of [`super::periods`]' bounded horizon is a different answer
    /// from a day the source refuses: an identity-backed source attributes the
    /// exhaustion to itself as [`CalendarQueryError::SearchExhausted`], while a
    /// detached snapshot has no coverage to attribute an error to and keeps
    /// its exhaustive `None` absence (LAW-PANIC's bounded-search contract).
    fn search_exhausted(coverage: Self::Coverage, day: NaiveDate) -> Result<(), Self::Error>;
}

/// Marker: a detached caller-supplied [`MarketHours`] snapshot.
///
/// A snapshot is exactly its supplied rules — no identity, no built-in holiday
/// table, no coverage claim (the crate never guesses a family from coincident
/// rules) — so there is no day it can refuse and no error it can raise. Each
/// gate below is a total function returning `Ok` under an [`Infallible`]
/// return type: the "no refusal" claim is the signature, not a branch.
#[derive(Clone, Copy)]
pub(in crate::calendar) struct FixedSnapshot;

impl SourceGate for FixedSnapshot {
    type Source<'a> = &'a MarketHours;
    type Coverage = ();
    type Error = Infallible;

    fn identity(_source: &MarketHours) -> Option<CalendarSource> {
        None
    }

    fn profile_for_open_day(
        source: Self::Source<'_>,
        _tz: Tz,
        _day: NaiveDate,
    ) -> ResolvedHours<'_> {
        ResolvedHours::Borrowed(source)
    }

    fn has_daily_close_at(source: &MarketHours, _instant: DateTime<Utc>) -> bool {
        source.has_daily_close
    }

    fn has_weekend_close_at(source: &MarketHours, _instant: DateTime<Utc>) -> bool {
        source.has_weekend_close
    }

    fn require_answerable(_coverage: (), _date: NaiveDate) -> Result<(), Infallible> {
        Ok(())
    }

    fn require_floor(_coverage: (), _date: NaiveDate) -> Result<(), Infallible> {
        Ok(())
    }

    fn require_floor_at(_coverage: (), _tz: Tz, _instant: DateTime<Utc>) -> Result<(), Infallible> {
        Ok(())
    }

    fn require_phase_coverage(_coverage: (), _date: NaiveDate) -> Result<(), Infallible> {
        Ok(())
    }

    fn require_holiday_classification(_coverage: (), _date: NaiveDate) -> Result<(), Infallible> {
        Ok(())
    }

    fn has_unscoped_refusing_phase_gap_on(_coverage: (), _day: NaiveDate) -> bool {
        false
    }

    fn search_exhausted(_coverage: (), _day: NaiveDate) -> Result<(), Infallible> {
        Ok(())
    }
}

/// Marker: an identity-backed calendar, bare or overlaid.
///
/// The context always carries its [`CalendarCoverage`], and every refusal the
/// coverage gates produce is reported through [`CalendarQueryError`] — never
/// as absence, `false` or a default schedule (LAW-COVERAGE).
#[derive(Clone, Copy)]
pub(in crate::calendar) struct Identified;

impl SourceGate for Identified {
    type Source<'a> = ExchangeCalendar;
    type Coverage = CalendarCoverage;
    type Error = CalendarQueryError;

    fn identity(source: ExchangeCalendar) -> Option<CalendarSource> {
        Some(source.source())
    }

    fn profile_for_open_day(
        calendar: Self::Source<'_>,
        tz: Tz,
        day: NaiveDate,
    ) -> ResolvedHours<'_> {
        let anchor = mk_local_open(tz, day, OPEN_DAY_ANCHOR_SSM).with_timezone(&Utc);
        ResolvedHours::Selected(calendar.hours_at(anchor))
    }

    fn has_daily_close_at(calendar: ExchangeCalendar, instant: DateTime<Utc>) -> bool {
        calendar.hours_at(instant).has_daily_close
    }

    fn has_weekend_close_at(calendar: ExchangeCalendar, instant: DateTime<Utc>) -> bool {
        calendar.hours_at(instant).has_weekend_close
    }

    fn require_answerable(
        coverage: CalendarCoverage,
        date: NaiveDate,
    ) -> Result<(), CalendarQueryError> {
        if date < SUPPORT_FLOOR {
            return Ok(());
        }
        let verdict = match coverage.date_level_gap_on(date) {
            None => DateCoverage::Covered,
            Some(CoverageGapReason::WithheldDate) => DateCoverage::UnresolvedGap,
            Some(CoverageGapReason::NormalWeekOnly) => DateCoverage::NormalWeekOnly,
            Some(_) => DateCoverage::OutsideCoveredRange,
        };
        match verdict {
            DateCoverage::Covered => Ok(()),
            // A normal-week-only calendar still has a sourced-normal-week start
            // below which its weekday profile is carried rather than sourced,
            // so the one relaxation reaches exactly as far as that start and no
            // further. Reported rather than hidden: refusing here would let a
            // caller read a coverage error as a market closure.
            DateCoverage::NormalWeekOnly => {
                let sourced = coverage.sourced_normal_week();
                if date >= sourced.first() {
                    Ok(())
                } else {
                    Err(CalendarQueryError::OutsideCoveredRange {
                        source: coverage.identity(),
                        date,
                    })
                }
            }
            DateCoverage::BeforeSupportFloor => Err(CalendarQueryError::BeforeSupportFloor {
                source: coverage.identity(),
                date,
            }),
            DateCoverage::UnresolvedGap => Err(CalendarQueryError::UnresolvedGap {
                source: coverage.identity(),
                date,
            }),
            DateCoverage::OutsideCoveredRange => Err(CalendarQueryError::OutsideCoveredRange {
                source: coverage.identity(),
                date,
            }),
        }
    }

    fn require_floor(
        coverage: CalendarCoverage,
        date: NaiveDate,
    ) -> Result<(), CalendarQueryError> {
        if date < SUPPORT_FLOOR {
            return Err(CalendarQueryError::BeforeSupportFloor {
                source: coverage.identity(),
                date,
            });
        }
        Ok(())
    }

    fn require_floor_at(
        coverage: CalendarCoverage,
        tz: Tz,
        instant: DateTime<Utc>,
    ) -> Result<(), CalendarQueryError> {
        let local_day = bounded_utc(instant, tz).with_timezone(&tz).date_naive();
        Self::require_floor(coverage, local_day)
    }

    fn require_phase_coverage(
        coverage: CalendarCoverage,
        date: NaiveDate,
    ) -> Result<(), CalendarQueryError> {
        // The floor governs first. Below it the date is not "outside a covered
        // range" — no range has been claimed there at all — and reporting the
        // phase's verdict would both mask the floor and move the answer the day
        // #117 lands. `require_answerable` deliberately passes below the floor,
        // so the check has to be made here.
        if date < SUPPORT_FLOOR {
            return Err(CalendarQueryError::BeforeSupportFloor {
                source: coverage.identity(),
                date,
            });
        }
        // Otherwise the date-level verdict governs: a date the identity cannot
        // answer at all is refused for its own reason, not the phase's.
        Self::require_answerable(coverage, date)?;
        match coverage.phase_gap_on(date) {
            // No declaration applies to the date: the phase answers from the
            // sourced tables.
            None => Ok(()),
            // A pass-through reason's phase is served — only its label is the
            // crate's convention — so the scan answers from the sourced tables.
            Some(gap)
                if matches!(
                    gap.reason(),
                    CoverageGapReason::PostCloseQueueTradeDateLabel
                        | CoverageGapReason::UnpublishedClosureDates
                ) =>
            {
                Ok(())
            }
            // A refusing declaration applies: the phase the scan consults is
            // the one the identity withholds on this date.
            Some(_) => Err(CalendarQueryError::OutsideCoveredRange {
                source: coverage.identity(),
                date,
            }),
        }
    }

    fn require_holiday_classification(
        coverage: CalendarCoverage,
        date: NaiveDate,
    ) -> Result<(), CalendarQueryError> {
        if coverage.holiday_classification_refused_on(date) {
            // The enriched refusal (Tier 3): the baseline derives from the
            // error itself, naming the sourced normal week the date sits
            // inside.
            return Err(CalendarQueryError::UnresolvedGap {
                source: coverage.identity(),
                date,
            });
        }
        Ok(())
    }

    fn has_unscoped_refusing_phase_gap_on(coverage: CalendarCoverage, day: NaiveDate) -> bool {
        coverage.has_unscoped_refusing_phase_gap_on(day)
    }

    fn search_exhausted(
        coverage: CalendarCoverage,
        day: NaiveDate,
    ) -> Result<(), CalendarQueryError> {
        Err(CalendarQueryError::SearchExhausted {
            source: coverage.identity(),
            date: day,
            bound: day,
        })
    }
}

/// Unwraps an engine answer whose error type is [`Infallible`].
///
/// This is the whole fixed-snapshot collapse (issue #245): a
/// [`FixedSnapshot`] context raises no [`CalendarQueryError`], so the engine's
/// `Result` carries nothing to swallow and the match below has no error arm to
/// write. A context that can carry coverage produces [`CalendarQueryError`]
/// instead, which does not unify with [`Infallible`] — the collapse is
/// unreachable from it at compile time, so a future error variant can never
/// resurface here as market absence.
pub(in crate::calendar) fn answered<T>(result: Result<T, Infallible>) -> T {
    match result {
        Ok(answer) => answer,
    }
}
