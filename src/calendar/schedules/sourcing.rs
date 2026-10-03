// SPDX-License-Identifier: MIT-0

//! Each identity's declared sourcing boundary (LAW-COVERAGE).
//!
//! The verification ledger's `Horizon` column records, per identity, the
//! venue-local date below which that identity's normal-week rows are **carried**
//! rather than **sourced** — or an em dash when nothing is carried.
//! `docs/schedules/coverage-2025.md` repeats the same value for every served
//! scope. This module is that column in the type system, and it is the only
//! place the crate states it: `CalendarCoverage` reads it, and nothing derives a
//! horizon from a timeline, because a timeline cannot know whether the baseline
//! below its first revision is sourced or carried (LAW-NO-FABRICATED-DATES).
//!
//! Two matches, no wildcard, one arm per identity — the same structure
//! `holidays/routing.rs` uses for the holiday layer. Adding an `Exchange` or a
//! `MarketHoursKey` is therefore a compile error until someone reads that
//! identity's ledger row and restates it here.
//!
//! `horizon!(year, month, day)` restates a dated cell; `horizon!(none)` restates
//! the ledger's em dash and means the identity carries nothing back — the state
//! below its own first row is either a sourced launch closure or, for a
//! synthetic identity, library policy. `DeclaredSourcing::no_holidays` marks the
//! scopes whose own definition has no holiday closures, an affirmative
//! assertion (LAW-HOLIDAY-SCOPE) and never an inference from a missing table.
//!
//! `DeclaredSourcing::phase_gaps` is the sibling assertion for the **declared**
//! gaps: the shapes `docs/schedules/coverage-2025.md`'s
//! `Missing / disputed` column records but no date walk can find, because the
//! arrangement the operator publishes is missing from the whole span the
//! declaration names rather than from the dates a boundary falls between — the
//! whole claimed interval when the declaration carries no bound, and the era
//! before `PhaseGap::until` when the identity's own knowledge-bound row begins
//! serving it. `CoverageGapReason::NormalWeekPhaseWithheld`
//! is the required-phase shape (#79 and #123),
//! `CoverageGapReason::PostCloseQueueTradeDateLabel` the served-but-relabelled
//! shape (#152) and `CoverageGapReason::UnpublishedClosureDates` the undated
//! holiday-scope shape (#157), which withholds no phase at all. Each declaration
//! carries the issue whose closure discharges it, so the
//! metadata reports a gap with a closing condition rather than a bare verdict —
//! LAW-COVERAGE's "a recorded gap with a closing condition", asserted by the
//! crate rather than inferred from its data. One identity can carry several,
//! because the shape recurs per phase rather than per scope: `globex_grains`
//! carries the #152 label gap and the omitted 2012-05-20..2013-04-06 regime
//! beside it, and the list stays a slice so the next phase that needs its own
//! declaration does not need the type to change. A declaration is removed when
//! the rows that discharge it ship, and
//! `CoverageGapReason::SpecialSessionUnrepresentable` — the shape #93 named — is
//! what that looks like when it has happened for every scope that declared it:
//! `globex_fx` declared the quarter-hour and the #93 special sessions until its
//! merged trade dates landed, and `globex_cryptocurrency` declared the #93 shape
//! until its own merged trade dates landed on 2026-09-26 UTC. No arm below cites
//! it now, and the reason stays on the enum because it is still the vocabulary a
//! future special-session gap would use.
//!
//! Every declaration also states the dates it applies to, so the metadata
//! withholds exactly what the evidence withholds (#172): `PhaseGap::since` and
//! `PhaseGap::until` bound the era on either side — each bound restating a dated
//! row the module already ships — and a `PhaseGapShape` narrows the era to the
//! dates a served occurrence decides, which is what lets the #79 scopes answer
//! their Tuesdays while withholding the bracket-era Sundays. Both bounds are
//! dated rows, never inferences: the bracket that starts #79's span is the
//! 2012-05-28 platform-wide capture (the last state printing 16:15 CT), and a
//! gap whose onset the evidence cannot date starts at its era's own sourced
//! first day (#123's 2017-12-17 launch row).
//!
//! Only an identity whose gap survives the permanent 2010 floor is declared
//! here, because that is the interval the completeness claim covers: a scope
//! whose withheld phase or unstateable session lies entirely before 2010 is not
//! incomplete in the claimed interval and must not be declared. Eleven identities
//! declare today — twelve declarations in all, `globex_grains` carrying two — and
//! the fences in
//! `tests/schedule_documentation/coverage_inventory.rs` hold them to the
//! inventory's own verdicts while `tests/coverage_metadata.rs` holds each
//! declaration to the shipped profile's behaviour.
//!
//! `tests/schedule_documentation/horizons.rs` re-reads the ledger and fails if
//! any arm here drifts from the cell it restates.

use chrono::NaiveDate;

use super::timeline::{effective_date, horizon};
use crate::calendar::coverage::{CoverageGapReason, PhaseGap, PhaseGapShape};
use crate::calendar::{CalendarSource, Exchange, MarketHoursKey};

/// What one identity declares about its own sourcing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DeclaredSourcing {
    /// The venue-local date below which this identity's normal-week rows are
    /// carried rather than sourced, or `None` when nothing is carried.
    pub(crate) carried_below: Option<NaiveDate>,
    /// Whether the identity's own definition has no holiday closures.
    ///
    /// An affirmative assertion recorded beside the identity, never inferred
    /// from the absence of a holiday table: an identity with no table and no
    /// such assertion reports that it has no holiday answer at all.
    pub(crate) observes_no_holidays: bool,
    /// The phase-level gaps this identity carries into the 2025-onward claim, in
    /// declaration order; empty when it carries none.
    ///
    /// Like `observes_no_holidays`, this is an affirmative assertion and never
    /// an inference: an identity with no declaration reports no phase-level gap,
    /// and the inventory's `Missing / disputed` cell is what a reviewer reads
    /// before adding one. A slice rather than an `Option`, because the shapes
    /// stack: one scope can withhold a required phase *and* publish a session
    /// no shipped row states.
    pub(crate) phase_gaps: &'static [PhaseGap],
}

impl DeclaredSourcing {
    /// A ledger horizon: rows below the date inside are carried, and this
    /// identity ships a holiday table.
    ///
    /// The argument is `horizon!(year, month, day)` or `horizon!(none)`, so the
    /// macro and this constructor cannot disagree about which ledger cell is
    /// being restated.
    const fn carried_below(horizon: Option<NaiveDate>) -> Self {
        Self {
            carried_below: horizon,
            observes_no_holidays: false,
            phase_gaps: &[],
        }
    }

    /// The ledger's em dash: nothing is carried, and this identity ships a
    /// holiday table.
    const fn nothing_carried() -> Self {
        Self {
            carried_below: None,
            observes_no_holidays: false,
            phase_gaps: &[],
        }
    }

    /// A scope whose own definition has no holiday closures: the two synthetic
    /// 24×7 identities, which are library policy rather than a venue, and the
    /// one operator that publishes continuous 24/7 availability.
    const fn no_holidays() -> Self {
        Self {
            carried_below: None,
            observes_no_holidays: true,
            phase_gaps: &[],
        }
    }

    /// `nothing_carried()`, plus the phase-level gaps the identity declares.
    ///
    /// The declarations below are the ones `docs/schedules/coverage-2025.md`
    /// calls incomplete for a reason its date columns cannot show, so each also
    /// carries the closing issue that page's `Closing issues` cell names for the
    /// scope.
    const fn nothing_carried_with(phase_gaps: &'static [PhaseGap]) -> Self {
        Self {
            carried_below: None,
            observes_no_holidays: false,
            phase_gaps,
        }
    }

    /// `carried_below(horizon)`, plus the phase-level gaps the identity declares.
    const fn carried_below_with(
        horizon: Option<NaiveDate>,
        phase_gaps: &'static [PhaseGap],
    ) -> Self {
        Self {
            carried_below: horizon,
            observes_no_holidays: false,
            phase_gaps,
        }
    }

    /// `carried_below(horizon)`, plus the withheld Sunday quarter-hour (#79).
    ///
    /// Six of the seven scopes that withhold that quarter-hour declare nothing
    /// else, so their arm in the per-identity table stays one line of horizon
    /// restatement.
    const fn carried_below_with_quarter_hour(horizon: Option<NaiveDate>) -> Self {
        Self::carried_below_with(horizon, &WITHHELD_QUARTER_HOUR)
    }
}

/// The required-phase gap seven scopes carry: CME's Sunday 16:00-16:15 CT
/// quarter-hour is withheld (#79) until the era that serves it.
///
/// `docs/schedules/coverage-2025.md` records it as "the 16:00-16:15 CT Sunday
/// quarter-hour, withheld (#79)" and `docs/evidence/cme.md` plus each family's
/// own module record that only the disputed quarter-hour depends on the undated
/// 2012 move, so the crate serves the sourced 16:15-17:00 CT intersection
/// instead. The Sunday queue moved from a 16:15 to a 16:00 CT onset in 2012
/// without an operator-stated day, and the same grid governs every scope here:
/// `cme`, `comex`, `nymex`, `globex_energy`, `globex_equity_index`, `globex_fx`
/// and `globex_interest_rates`.
///
/// **The operative reason is narrower than "the 2012 change was never dated".** On the
/// bytes, no sourced state has printed Sunday 16:15 CT since 2012-05-28: Globex
/// notice 20120402 dates the old value at 2012-04-15, and notice 20121112 prints
/// the new one. What the crate actually lacks is an admissible artifact whose own
/// scope covers an **ordinary week inside the claimed interval** — its 2025
/// evidence is holiday geometry, and its only ordinary-week artifact is
/// 2026-10-18..24 — and the charter says a later observation alone does not prove
/// the intervening period complete. The 2012 bracket is therefore a second,
/// independent reason rather than the first.
///
/// **Bounded to the dated era, and shaped to the Sundays it withholds.** The
/// four modules these seven scopes route to — `cme_group.rs`, `energy_metals.rs`,
/// `fx.rs` and `interest_rates.rs` — each end their timelines in a knowledge-bound
/// 2026-08-22 row whose profile widens the Sunday queue to 16:00-17:00 CT, so from
/// that day the withheld quarter-hour *is* served and the gap no longer holds: the
/// declaration's `until` bound is that row's day, verified per module rather than
/// assumed. The `since` bound is 2012-05-28, the platform-wide capture that is the
/// last state printing 16:15 CT: on every earlier Sunday the sourced value *is*
/// 16:15, the served intersection is the truth, and the quarter-hour is affirmatively
/// quiet — refusing those dates would withhold answers the evidence gives. The
/// **shape** names the served Sunday Pre-Open window itself (16:15-17:00 CT): the
/// withheld slice is the fifteen minutes before it, and the question it cannot
/// answer exists exactly on the dates whose Sunday queue resolves — a Tuesday in
/// the same era answers from the sourced weekday grid, and a Sunday whose evening
/// leg a holiday removes answers closed. What remains is the bracket-to-bound span
/// of resolving Sundays: the dates the evidence actually cannot state.
const fn withheld_sunday_quarter_hour() -> PhaseGap {
    PhaseGap::new(CoverageGapReason::NormalWeekPhaseWithheld, "#79")
        .since(effective_date(2012, 5, 28))
        .until(effective_date(2026, 8, 22))
        .with_shape(PhaseGapShape::OrderEntryWindow {
            open_ssm: Some(16 * 3600 + 15 * 60),
            close_ssm: 17 * 3600,
        })
}

/// The post-close queue's trade-date label, which `globex_grains` and
/// `globex_livestock` both carry (#152).
///
/// CME's own trading-hours service prints a trade date on every post-close
/// `pcp` event, and on an ordinary date that is the date the queue is printed
/// on. The crate dates an order-entry occurrence by the session it feeds, so
/// the same queue reads with the **next** trade date instead — `D + 1` on a
/// Monday to Thursday, `D + 3` over a weekend. Measured over
/// 2025-01-01..2027-12-31 (re-measured 2026-09-29 UTC after the closure-eve
/// replacement blocks landed), the shape refuses **720** of the 1,095 days for
/// `globex_grains` — every day whose calendar serves the 14:30-16:00 CT queue —
/// and **754** for `globex_livestock`, each answering a trade date other than
/// the operator's label; the closure-eve days whose complete replacement blocks
/// state the adjusted day answer completely, and the walk fence in
/// `tests/coverage_metadata.rs` pins the counts.
///
/// **Shaped to the dates that carry the queue.** The shape names any
/// order-entry window closing at 16:00 CT on a weekday — the PCP's own close,
/// which held while its opening moved (14:30 CT at the 2010 floor and again
/// from 2013-04-07, 13:15:30 CT in the 2010-04-19..2012-05-19 era) — so the
/// gap applies exactly where the crate serves the queue whose label diverges:
/// the 2012-05-20..2013-04-06 regime serves no queue and is `globex_grains`'
/// separate declaration below, weekends and closed dates answer completely,
/// and livestock's dates before the sourced 2016-06-06 onset of its own
/// Post-Close answer likewise. The phase is served, so this declaration
/// withholds no answer and refuses no query — see
/// [`CoverageGapReason::PostCloseQueueTradeDateLabel`](crate::CoverageGapReason::PostCloseQueueTradeDateLabel),
/// which also records why no data row can close it. What it states is that a
/// queue date's trade date is the crate's convention rather than the
/// operator's printing, which is why neither scope may claim a complete
/// calendar. `docs/evidence/globex_grains.md` and
/// `docs/evidence/globex_livestock.md` record it with the probe instants that
/// show it; #152 is the issue that closes it.
const fn post_close_queue_trade_date_label() -> PhaseGap {
    PhaseGap::new(CoverageGapReason::PostCloseQueueTradeDateLabel, "#152").with_shape(
        PhaseGapShape::OrderEntryWindow {
            open_ssm: None,
            close_ssm: 16 * 3600,
        },
    )
}

/// `globex_grains`' second gap: the 2012-05-20..2013-04-06 regime's queue
/// states are omitted outright, so no date of that regime answers its
/// order-entry phases.
///
/// The 2012-05-20 expansion's queue times are sourced as *states* — CME's own
/// trading-hours captures of 2012-05-28 and 2012-06-07 print Sunday Pre-Open
/// 16:00 and weekday "14:30-16:00, 16:45-17:00" against 16:15 and the
/// 18:00-07:15 grid on 2012-05-11 — but the switch-on day is bracketed to
/// 2012-05-11..2012-05-28 and advisory 20120518 states only the matching
/// hours, so no queue revision is keyed inside the regime and the profile
/// serves its matching grid with no queues at all. The bracketed start of the
/// regime itself is the dated 2012-05-20 revision row and the dated
/// 2013-04-07 notice ends it, so the declaration bounds itself to exactly
/// those days rather than refusing either neighbour. The whole
/// [`PhaseGapShape::EveryDay`] regime refuses because the omitted queues are
/// order-entry phases the crate serves nowhere in it — there is no served
/// window to shape the applicability onto. The sessions themselves are
/// sourced and answered; `docs/evidence/globex_grains.md` records the states,
/// the bracket and the closing condition — a CME document that states those
/// queue times in session language on a day-level effective date — and the
/// scope's inventory row names #259, the live issue that carries the gap as
/// its closing condition (its former citation #116 closed without this
/// data), alongside #117.
const fn grains_omitted_regime_queues() -> PhaseGap {
    PhaseGap::new(CoverageGapReason::NormalWeekPhaseWithheld, "#259")
        .since(effective_date(2012, 5, 20))
        .until(effective_date(2013, 4, 7))
}

/// The second gap `globex_cryptocurrency` carries: the five-day era's Sunday and
/// weekday Pre-Open onset is undated, so dated history omits those queues.
///
/// The ledger's basis note states it, and `docs/evidence/globex_cryptocurrency.md`
/// records the closing condition — "a CME artifact that states the Pre-Open in
/// session language on a day-level effective date" — after the 2026-08-31
/// review confirmed the 2017-12-14, 2017-12-22 and 2018-01-04 specification
/// captures publish the matching grid only. That evidence file said the gap was
/// "tracked as an issue" and named no issue number, so #123 was opened for it and
/// is now cited here, in `docs/schedules/coverage-2025.md`'s `Closing issues` cell
/// for this scope and in that evidence file's own gap note:
/// LAW-FOLLOW-UPS-ARE-ISSUES is discharged rather than waived, and the issue
/// exists rather than being promised.
///
/// **Bounded to the five-day era on both sides.** The gap is a property of the
/// five-day 17:00-16:00 CT grid, whose own first day is dated at T1 — CME
/// SER-8051R's 2017-12-17 launch row, before which the family's era is a sourced
/// launch closure that answers without the declaration — and whose own last day
/// is dated at T1 as well: CME filing 26-114 moves the family to the 24/7 grid on
/// the 2026-05-29 bridge row, whose profile serves the operator's published
/// Pre-Open for the first time (`ORDER_ENTRY_2026_05_29`, and the era's two
/// queues from 2026-05-30). So the withheld phase is live exactly between those
/// two sourced days, and the declaration says so rather than refusing the launch
/// closures before the grid or denying the 24/7 era coverage its profiles serve.
/// What the evidence never dates is the day the Pre-Open *began* inside the
/// five-day grid — the 2017-12-14, 2017-12-22 and 2018-01-04 specification
/// captures publish the matching grid only — and the shape stays
/// [`PhaseGapShape::EveryDay`] because the queues are omitted from the whole
/// era: no date of it can prove the phase it does not carry (LAW-NO-FABRICATED-DATES).
const fn undated_five_day_pre_open() -> PhaseGap {
    PhaseGap::new(CoverageGapReason::NormalWeekPhaseWithheld, "#123")
        .since(effective_date(2017, 12, 17))
        .until(effective_date(2026, 5, 29))
}

/// The one-declaration list the six quarter-hour scopes other than `globex_fx`
/// carry.
const WITHHELD_QUARTER_HOUR: [PhaseGap; 1] = [withheld_sunday_quarter_hour()];

/// The declaration `globex_livestock` carries: the post-close queue's label,
/// shaped to the dates that carry the queue.
const LIVESTOCK_GAPS: [PhaseGap; 1] = [post_close_queue_trade_date_label()];

/// The declarations `globex_grains` carries: the post-close queue's label
/// (#152), plus the 2012-05-20..2013-04-06 regime whose queue states are
/// omitted outright.
const GRAINS_GAPS: [PhaseGap; 2] = [
    post_close_queue_trade_date_label(),
    grains_omitted_regime_queues(),
];

/// The undated closure scope `eurex` declares: Eurex's own trading calendars say
/// the German equity and equity-index scope closes on dates it has not published
/// (#157).
///
/// `docs/schedules/coverage-2025.md` records the scope's verdict as incomplete
/// for it and `docs/evidence/eurex.md` quotes both editions: the *Eurex trading
/// calendar 2025* prints `Kein Handel und keine Ausübung in deutschen Aktien- und
/// Aktienindex-derivaten sowie in ETF- und ETC-Derivaten, die auf
/// Xetra@-Börsen-notierungen basieren: tba.`, and the 2026 edition carries the
/// same note in English and still says `to be announced`. FDAX and FDXM are
/// German equity-index derivatives behind this identity, so 2025 and 2026 could
/// carry closures the shipped table cannot state.
///
/// **Bounded by the editions that carry the note, and no phase.** The note is
/// evidence in the 2025 and 2026 editions and in no later one — the 2027-2036
/// material is the operator's own `preliminary and indicative` publishing, and
/// the archive's Trading Calendar list stops at 2026 — so the gap's span ends
/// where the last edition in hand ends: 2027-01-01 is the first day the note
/// does not establish, not a day the operator resolved it. If a 2027 edition
/// carries the note again, extending the bound is that edition's dated change.
/// The gap withholds no phase — Eurex serves every phase it models on an
/// ordinary day — so the declaration is a completeness fact rather than a reason
/// to refuse an order-entry queue, and
/// `CalendarQueryContext::require_phase_coverage` answers through it.
const fn undated_german_closures() -> PhaseGap {
    PhaseGap::new(CoverageGapReason::UnpublishedClosureDates, "#157")
        .until(effective_date(2027, 1, 1))
}

/// `eurex`'s one declaration: the operator's undated German-scope closures.
const EUREX_UNDATED_CLOSURES: [PhaseGap; 1] = [undated_german_closures()];

/// `globex_cryptocurrency` declares one gap: the five-day era's undated Pre-Open
/// onset. Its 24/7-era merged trade dates shipped as rows on 2026-09-26 UTC, so
/// the special-session declaration it carried until then is gone.
const GLOBEX_CRYPTOCURRENCY_GAPS: [PhaseGap; 1] = [undated_five_day_pre_open()];

/// Returns what `source` declares about its own sourcing.
pub(crate) const fn declared(source: CalendarSource) -> DeclaredSourcing {
    match source {
        CalendarSource::Exchange(exchange) => for_exchange(exchange),
        CalendarSource::MarketHoursKey(key) => for_market_hours_key(key),
    }
}

/// The venue half of the match, in `Exchange::ALL` order.
#[expect(
    clippy::match_same_arms,
    reason = "the per-variant arms are the fence: one arm per identity is what a \
              reviewer edits when that identity's ledger row moves, and collapsing \
              them would let the next venue inherit a boundary nobody decided"
)]
const fn for_exchange(exchange: Exchange) -> DeclaredSourcing {
    match exchange {
        // `—`: synthetic 24×7 fallback: library policy, not a venue, and no holiday closures.
        Exchange::Unknown => DeclaredSourcing::no_holidays(),
        // Sourced from the floor: the operator's own quoted in-force rulebook
        // text (SR-NASDAQ-2010-008, "Nasdaq market hours (7 a.m. to 8 p.m.
        // ET)"), its Rule 4120(b)(4) statements in the 2012-2013 filings, its
        // archived pre-2013 Trading Hours page, and SR-NASDAQ-2013-033's
        // marked "[7:00]" rule text and named implementation day — see
        // docs/evidence/nasdaq.md.
        Exchange::Nasdaq => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::NasdaqBx => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        // `—`: closed before the operator-dated 2010-10-08 launch.
        Exchange::NasdaqPsx => DeclaredSourcing::nothing_carried(),
        Exchange::CboeBzx => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        // `—`: closed before the sourced 2010-10-15 launch.
        Exchange::CboeByx => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the sourced 2010-07-02 first-symbol launch.
        Exchange::CboeEdga => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the sourced 2010-07-02 launch.
        Exchange::CboeEdgx => DeclaredSourcing::nothing_carried(),
        // Sourced from the floor: NYSE's own historical note (regular trading
        // 9:30-16:00 since 1985-09-30, "As of January 26, 2005") and its Rule 51
        // statements in the 2014-2017 SEC filings — see docs/evidence/nyse.md.
        Exchange::Nyse => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::NyseArca => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::NyseAmerican => DeclaredSourcing::carried_below(horizon!(2017, 7, 24)),
        Exchange::NyseNational => DeclaredSourcing::carried_below(horizon!(2010, 8, 2)),
        Exchange::NyseTexas => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        // `—`: closed before the operator-dated 2020-09-21 live launch.
        Exchange::MemxEq => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the sourced 2020-09-29 live launch.
        Exchange::MiaxPearlEq => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the 2016-08-19 first production-symbol launch.
        Exchange::Iex => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the SEC-recorded 2020-08-28 commencement of operations.
        Exchange::Ltse => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the SEC-recorded 2025-10-14 commencement of trading.
        Exchange::TwentyFourX => DeclaredSourcing::nothing_carried(),
        // `—`: closed through the July-2026 non-clearing test-symbol period; first live NMS production is 2026-07-10.
        Exchange::Txse => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the operator-dated 2021-10-05 launch.
        Exchange::BlueOceanAts => DeclaredSourcing::nothing_carried(),
        Exchange::FinraTrfCarteret => DeclaredSourcing::carried_below(horizon!(2026, 3, 30)),
        // `—`: closed before the sourced 2018-09-10 facility launch.
        Exchange::FinraTrfChicago => DeclaredSourcing::nothing_carried(),
        Exchange::FinraTrfNyse => DeclaredSourcing::carried_below(horizon!(2026, 3, 30)),
        Exchange::CboeOptionsC1 => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        // `—`: closed before the operator-dated 2010-10-29 launch.
        Exchange::CboeC2Options => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the operator-dated 2010-02-26 launch.
        Exchange::CboeBzxOptions => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the operator-dated 2015-11-02 phase-one launch.
        Exchange::CboeEdgxOptions => DeclaredSourcing::nothing_carried(),
        Exchange::NyseArcaOptions => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::NyseAmericanOptions => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::NasdaqPhlx => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::NasdaqIse => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::NasdaqNom => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        // `—`: closed before the operator-dated 2016-02-16 launch.
        Exchange::NasdaqMrx => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the operator-dated 2013-08-05 launch.
        Exchange::NasdaqGemx => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the operator-dated 2012-06-29 launch.
        Exchange::NasdaqBxOptions => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the operator-dated 2012-12-07 launch.
        Exchange::MiaxOptions => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the operator-dated 2019-03-01 launch.
        Exchange::MiaxEmeraldOptions => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the operator-dated 2017-02-06 launch.
        Exchange::MiaxPearlOptions => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the operator-dated 2024-08-12 launch.
        Exchange::MiaxSapphireOptions => DeclaredSourcing::nothing_carried(),
        Exchange::BoxOptions => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        // `—`: closed before the operator-dated 2023-09-27 launch.
        Exchange::MemxOptions => DeclaredSourcing::nothing_carried(),
        // The Sunday 16:00-16:15 CT quarter-hour is withheld (#79). The
        // floor-era grid and both queues are sourced (notices 20090831/0907/0914).
        Exchange::Cme => DeclaredSourcing::carried_below_with_quarter_hour(horizon!(2010, 1, 1)),
        Exchange::Cbot => DeclaredSourcing::carried_below(horizon!(2010, 3, 15)),
        // The Sunday 16:00-16:15 CT quarter-hour is withheld (#79).
        Exchange::Comex => DeclaredSourcing::carried_below_with_quarter_hour(horizon!(2012, 5, 11)),
        // The Sunday 16:00-16:15 CT quarter-hour is withheld (#79).
        Exchange::Nymex => DeclaredSourcing::carried_below_with_quarter_hour(horizon!(2012, 5, 11)),
        Exchange::Cfe => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        // `—`: closed before FairX's exact 2021-06-28 08:00 CT launch.
        Exchange::CoinbaseDerivatives => DeclaredSourcing::nothing_carried(),
        // `—`: closed before its own 2020-05-18 first trade date, and closed from 2025-03-24.
        Exchange::Smfe => DeclaredSourcing::nothing_carried(),
        Exchange::Eurex => {
            DeclaredSourcing::carried_below_with(horizon!(2010, 1, 1), &EUREX_UNDATED_CLOSURES)
        }
        // `—`: closed before the sourced 2024-03-25 launch.
        Exchange::Eex => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the sourced 2017-11-07 launch-eve opening.
        Exchange::Iceus => DeclaredSourcing::nothing_carried(),
        Exchange::Iceeu => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::IceEuropeCommodities => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        // `—`: closed before its sourced 2014-11-17 migration.
        Exchange::IceEuropeFinancials => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the sourced 2013-10-07 transfer.
        Exchange::IceEndex => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the sourced 2021-03-29 launch.
        Exchange::IceAbuDhabi => DeclaredSourcing::nothing_carried(),
        Exchange::IceCanada => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        // `—`: closed before the sourced 2024-07-29 launch.
        Exchange::Sgx => DeclaredSourcing::nothing_carried(),
        // The pre-SR15 staggered-open timetable is the operator's own page
        // print from 2013-09-16 through 2020-10-22 and the SR15 amendments'
        // struck-through old text — see docs/evidence/asx.md.
        Exchange::Asx => DeclaredSourcing::carried_below(horizon!(2013, 9, 16)),
        // `—`: closed before the 2011-10-31 Chi-X Australia launch.
        Exchange::TmxAustralia => DeclaredSourcing::nothing_carried(),
        // Sourced from 2010-01-05: the operator's own key-dates trading-hours
        // page prints the pre-2020 grid, and every later capture corroborates it
        // to the dated 2020-04-06 revision — see docs/evidence/nzx.md.
        Exchange::Nzx => DeclaredSourcing::carried_below(horizon!(2010, 1, 5)),
        Exchange::Tse => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::NseIndia => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::BseIndia => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        // Sourced from the floor: the operator's own Trading Hours page (footer
        // "Updated: 23/03/2009"), captured 2010-05-24..2011-01-19 with the same
        // grid and superseded at the dated 2011-03-07 change — see
        // docs/evidence/hkex.md.
        Exchange::Hkex => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::SgxSecurities => DeclaredSourcing::carried_below(horizon!(2011, 8, 1)),
        // sgx_securities horizon unchanged: the 2009-05-14 Trading Hours page
        // capture is pre-floor and dates no day inside the claimed interval
        // (docs/evidence/sgx_securities.md).
        Exchange::BursaMalaysia => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::SetThailand => DeclaredSourcing::carried_below(horizon!(2024, 3, 25)),
        Exchange::Idx => DeclaredSourcing::carried_below(horizon!(2010, 8, 31)),
        Exchange::Pse => DeclaredSourcing::carried_below(horizon!(2011, 10, 1)),
        Exchange::Hose => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::Sse => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::Szse => DeclaredSourcing::carried_below(horizon!(2016, 5, 9)),
        Exchange::Krx => DeclaredSourcing::carried_below(horizon!(2016, 8, 1)),
        Exchange::Twse => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::Lse => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::Xetra => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::Six => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::EuronextParis => DeclaredSourcing::carried_below(horizon!(2010, 12, 24)),
        Exchange::EuronextAmsterdam => DeclaredSourcing::carried_below(horizon!(2010, 12, 24)),
        Exchange::EuronextBrussels => DeclaredSourcing::carried_below(horizon!(2010, 12, 24)),
        Exchange::EuronextLisbon => DeclaredSourcing::carried_below(horizon!(2010, 12, 24)),
        Exchange::EuronextDublin => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::EuronextMilan => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::Bme => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::NasdaqStockholm => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::NasdaqHelsinki => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::NasdaqCopenhagen => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::Vienna => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        // Sourced from 2010-03-25: the operator's own İşlem Saatleri page prints
        // the pre-2012 grid, and the 2011-07-27 capture corroborates it — see
        // docs/evidence/borsa_istanbul.md.
        Exchange::BorsaIstanbul => DeclaredSourcing::carried_below(horizon!(2010, 3, 25)),
        Exchange::Tsx => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::Jse => DeclaredSourcing::carried_below(horizon!(2012, 5, 25)),
        // Sourced from 2010-01-12: the operator's own Trading Times page
        // (Arabic) states the Saturday-Wednesday 11:00-15:30 session, and the
        // 2011-04-29 English page restates it — see docs/evidence/tadawul.md.
        Exchange::Tadawul => DeclaredSourcing::carried_below(horizon!(2010, 1, 12)),
        Exchange::B3 => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::Bmv => DeclaredSourcing::carried_below(horizon!(2010, 2, 18)),
        // `—`: closed before the archived 2019-09-13 04:00 UTC launch, then continuously open; the operator publishes 24/7 trading and no holiday closures.
        Exchange::BinanceFutures => DeclaredSourcing::no_holidays(),
    }
}

/// The product-family half of the match, in `MarketHoursKey::ALL` order.
#[expect(
    clippy::match_same_arms,
    reason = "the per-variant arms are the fence: one arm per family is what a \
              reviewer edits when that family's ledger row moves, and collapsing \
              them would let the next key inherit a boundary nobody decided"
)]
const fn for_market_hours_key(key: MarketHoursKey) -> DeclaredSourcing {
    match key {
        // The floor-era grid and both queues are sourced like the venue's own
        // (notices 20090831/0907/0914); the Sunday quarter-hour is still
        // withheld (#79).
        MarketHoursKey::GlobexEquityIndex => {
            DeclaredSourcing::carried_below_with_quarter_hour(horizon!(2010, 1, 1))
        }
        MarketHoursKey::GlobexEnergy => {
            DeclaredSourcing::carried_below_with_quarter_hour(horizon!(2012, 5, 11))
        }
        // The 100-oz silver family shares the energy/metals tables by
        // reference, so it carries the family's own horizon; like Rough Rice,
        // a key referencing another family's rows declares no phase gaps of
        // its own — the family's bracketed Sunday quarter-hour (#79) is the
        // family's, recorded beside its rows and in this key's evidence file.
        MarketHoursKey::GlobexSilver100Oz => DeclaredSourcing::carried_below(horizon!(2012, 5, 11)),
        MarketHoursKey::GlobexGrains => {
            DeclaredSourcing::carried_below_with(horizon!(2010, 3, 15), &GRAINS_GAPS)
        }
        MarketHoursKey::GlobexMiniGrains => DeclaredSourcing::carried_below(horizon!(2010, 4, 5)),
        // Only the Sunday 16:00-16:15 CT quarter-hour is withheld (#79): every
        // special session CME publishes for this family now ships as a row, so
        // the #93 declaration it used to carry is gone. The floor-era matching
        // grid is sourced (FX-hours page, capture 2009-05-02), but the Sunday
        // Pre-Open has no pre-2012 statement of its own.
        MarketHoursKey::GlobexFx => {
            DeclaredSourcing::carried_below_with_quarter_hour(horizon!(2012, 5, 3))
        }
        MarketHoursKey::GlobexInterestRates => {
            DeclaredSourcing::carried_below_with_quarter_hour(horizon!(2010, 1, 1))
        }
        MarketHoursKey::GlobexLivestock => {
            DeclaredSourcing::carried_below_with(horizon!(2010, 1, 1), &LIVESTOCK_GAPS)
        }
        // `—`: closed before the exact 2017-12-17 launch grid. Both of this
        // family's declared session shapes now ship as rows; what is left is
        // the five-day era's Sunday and weekday Pre-Open onset (#123).
        MarketHoursKey::GlobexCryptocurrency => {
            DeclaredSourcing::nothing_carried_with(&GLOBEX_CRYPTOCURRENCY_GAPS)
        }
        MarketHoursKey::CfeVix => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        MarketHoursKey::Eurex => {
            DeclaredSourcing::carried_below_with(horizon!(2010, 1, 1), &EUREX_UNDATED_CLOSURES)
        }
        // `—`: closed before the sourced 2017-11-07 launch-eve opening.
        MarketHoursKey::IceUs => DeclaredSourcing::nothing_carried(),
        MarketHoursKey::IceUsSugar => DeclaredSourcing::carried_below(horizon!(2011, 8, 1)),
        MarketHoursKey::IceUsCoffee => DeclaredSourcing::carried_below(horizon!(2011, 8, 1)),
        MarketHoursKey::IceUsCocoa => DeclaredSourcing::carried_below(horizon!(2011, 8, 1)),
        MarketHoursKey::IceUsCotton => DeclaredSourcing::carried_below(horizon!(2011, 8, 1)),
        MarketHoursKey::IceUsOrangeJuice => DeclaredSourcing::carried_below(horizon!(2011, 8, 1)),
        MarketHoursKey::IceUsDollarIndex => DeclaredSourcing::carried_below(horizon!(2011, 2, 7)),
        // Sourced from the floor: the operator's own equities-hours page
        // (captures 2009-04-06 and 2010-04-02, byte-identical) states the old
        // grid in both DST spellings, and Globex notice 20100405 ends it at
        // the dated Sunday 2010-04-11 (#225). The Sunday Pre-Open onset
        // (16:15 -> 16:00 CT) is undated like #79's, but the quarter-hour is
        // served, not withheld (evidence file).
        MarketHoursKey::GlobexNikkei225Dollar => {
            DeclaredSourcing::carried_below(horizon!(2010, 1, 1))
        }
        // The pre-2018-12-10 baseline is sourced through the floor: the
        // operator's archived Contract Specifications amendments of
        // 2009-09-14 .. 2017-08-28 each print the FGBL/FGBM/FGBS/FGBX row.
        MarketHoursKey::EurexFixedIncome => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        MarketHoursKey::SgxEquityIndexJapan => {
            DeclaredSourcing::carried_below(horizon!(2010, 1, 1))
        }
        MarketHoursKey::SgxEquityIndexChina => {
            DeclaredSourcing::carried_below(horizon!(2010, 1, 1))
        }
        MarketHoursKey::SgxEquityIndexSingapore => {
            DeclaredSourcing::carried_below(horizon!(2010, 1, 1))
        }
        // `—`: the family boundary is its sourced 2020-07-20 launch rather than a carried grid.
        MarketHoursKey::SgxEquityIndexTaiwan => DeclaredSourcing::nothing_carried(),
        // `—`: the knowledge boundary is the 2018-04-16 Monday the 2018 (Apr) calendar edition was created for; the era before it is reported sessionless, below the support floor.
        MarketHoursKey::SgxEquityIndexNtrUsd => DeclaredSourcing::nothing_carried(),
        MarketHoursKey::GlobexRoughRice => DeclaredSourcing::carried_below(horizon!(2010, 3, 15)),
        MarketHoursKey::GlobexWeather => DeclaredSourcing::carried_below(horizon!(2010, 2, 6)),
        // `—`: every date before its 2025-06-29 first row is a sourced closure.
        MarketHoursKey::GlobexSpotQuoted => DeclaredSourcing::nothing_carried(),
        // `—`: sessionless before SER-9092's sourced 2022-09-18 listing day.
        MarketHoursKey::GlobexEventContracts => DeclaredSourcing::nothing_carried(),
        // `—`: sessionless before SER-9092's sourced listing day.
        MarketHoursKey::GlobexEventContractsBtc => DeclaredSourcing::nothing_carried(),
        // `—`: the pre-launch era is a sourced closure on the complete TAS lists.
        MarketHoursKey::GlobexGoldTas => DeclaredSourcing::nothing_carried(),
        // `—`: the pre-launch era is a sourced closure on the complete TAS lists.
        MarketHoursKey::GlobexSilverTas => DeclaredSourcing::nothing_carried(),
        // `—`: the pre-launch era is a sourced closure on the complete TAS lists.
        MarketHoursKey::GlobexCopperTas => DeclaredSourcing::nothing_carried(),
        // `—`: the pre-launch era is a sourced closure on the specification captures.
        MarketHoursKey::GlobexPlatinumTas => DeclaredSourcing::nothing_carried(),
        // `—`: the pre-launch era is a sourced closure on the specification captures.
        MarketHoursKey::GlobexPalladiumTas => DeclaredSourcing::nothing_carried(),
        // `—`: closed before the sourced 2024-07-29 launch.
        MarketHoursKey::Sgx => DeclaredSourcing::nothing_carried(),
        // `—`: synthetic 24×7 fallback: library policy, not a venue, and no holiday closures.
        MarketHoursKey::AlwaysOpen => DeclaredSourcing::no_holidays(),
    }
}
