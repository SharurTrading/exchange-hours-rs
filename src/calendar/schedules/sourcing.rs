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
//! was the required-phase shape (#79, #123, #259) and
//! `CoverageGapReason::UnpublishedClosureDates` the undated
//! holiday-scope shape (#157), which withholds no phase at all. Each declaration
//! carries the issue whose closure discharges it, so the
//! metadata reports a gap with a closing condition rather than a bare verdict —
//! LAW-COVERAGE's "a recorded gap with a closing condition", asserted by the
//! crate rather than inferred from its data. One identity can carry several,
//! because the shape recurs per phase rather than per scope, and the list stays a
//! slice so the next phase that needs its own declaration does not need the
//! type to change. A declaration is removed when the rows that discharge it
//! ship, and
//! `CoverageGapReason::SpecialSessionUnrepresentable` — the shape #93 named — is
//! what that looks like when it has happened for every scope that declared it:
//! `globex_fx` declared the quarter-hour and the #93 special sessions until its
//! merged trade dates landed, and `globex_cryptocurrency` declared the #93 shape
//! until its own merged trade dates landed on 2026-09-26 UTC. No arm below cites
//! it now, and the reason stays on the enum because it is still the vocabulary a
//! future special-session gap would use. The same retirement befell
//! `CoverageGapReason::PostCloseQueueTradeDateLabel` (#152) on 2026-10-03 UTC:
//! the charter's Post-Close trade-date convention (AGENTS.md, "Trade dates and
//! state") resolved the label divergence the declaration recorded, so neither
//! `globex_grains` nor `globex_livestock` declares it any more and the reason
//! stays on the enum as the vocabulary a future labelling divergence would use.
//! And the three `NormalWeekPhaseWithheld` declarations retired on 2026-10-04
//! UTC under the charter's sourced-intersection residual convention (AGENTS.md,
//! "Modeling conventions", the 2026-10-04 decision): the seven #79 quarter-hour
//! scopes, `globex_cryptocurrency`'s #123 five-day era and `globex_grains`'
//! #259 regime each had a phase sourced at two values with only the changeover
//! day undated, and every dated-artifact hunt had closed negative, so their
//! dates answer from the served intersection — from the regime's own dated
//! start for #259, whose queue rows ship in `grains.rs` — while the disputed
//! remainders are disclosed as residuals in the owners' evidence files. The
//! reason stays on the enum as the vocabulary a future required-phase gap would
//! declare: one whose hunts have not closed negative, or whose served answer
//! would be wrong under a sourced state. Nothing declares it today.
//!
//! The `UnpublishedClosureDates` declaration `eurex` carried for its `tba` era
//! (2025-01-01 through the last edition in hand, #157) retired on 2026-10-05 UTC
//! under the no-changes verification the maintainer set that day: the operator's
//! day-by-day Holiday regulations tables — whose grammar printed the German-scope
//! clause, futures carve-out and all, in 2020 — state no German-scope closure on
//! any date of 2025 or 2026, every candidate date of both years has passed
//! answering ordinary, and the Management-Board regulation channel such a closure
//! would travel (Conditions for Trading 1.2) is enumerated complete and empty of
//! it. `docs/evidence/eurex.md` holds the chain, and the reason stays on the enum
//! as the vocabulary a future undated holiday scope would declare.
//!
//! Every declaration also states the dates it applies to, so the metadata
//! withholds exactly what the evidence withholds (#172): `PhaseGap::since` and
//! `PhaseGap::until` bound the era on either side — each bound restating a dated
//! row the module already ships — and a `PhaseGapShape` narrows the era to the
//! dates a served occurrence decides. Both bounds are
//! dated rows, never inferences.
//!
//! Only an identity whose gap survives the permanent 2010 floor is declared
//! here, because that is the interval the completeness claim covers: a scope
//! whose withheld phase or unstateable session lies entirely before 2010 is not
//! incomplete in the claimed interval and must not be declared. No identity
//! declares today, and the fences in
//! `tests/schedule_documentation/coverage_inventory.rs` hold them to the
//! inventory's own verdicts while `tests/coverage_metadata.rs` holds each
//! declaration to the shipped profile's behaviour.
//!
//! `tests/schedule_documentation/horizons.rs` re-reads the ledger and fails if
//! any arm here drifts from the cell it restates.

use chrono::NaiveDate;

use super::timeline::horizon;
use crate::calendar::coverage::PhaseGap;
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
}

// The #152 post-close label declaration both queue scopes carried is retired
// (2026-10-03): the charter's Post-Close trade-date convention (AGENTS.md,
// "Trade dates and state") dates an order-entry queue by the session it feeds,
// the `CoverageGapReason::PostCloseQueueTradeDateLabel` reason stays on the
// enum as vocabulary, and the two families' evidence files disclose the T2
// feed's divergent labels beside the fences that pin the convention.

// The three `NormalWeekPhaseWithheld` declarations that used to live here —
// the seven-scope Sunday quarter-hour (#79), `globex_cryptocurrency`'s
// five-day-era Pre-Open onset (#123) and `globex_grains`' omitted
// 2012-05-20..2013-04-06 regime queues (#259) — retired on 2026-10-04 UTC
// under the charter's sourced-intersection residual convention (AGENTS.md,
// "Modeling conventions", the 2026-10-04 decision).

// Each was a phase whose endpoints were sourced at two values with only the
// changeover day undated, and every dated-artifact hunt had closed negative,
// so the retirement serves what the tables serve — the sourced 16:15-17:00 CT
// intersection across the #79 span; the five-day grid with the queues absent,
// the narrowest sourced state of the #123 era; and, for #259, the regime
// queue rows `grains.rs` keys to the regime's own dated start.

// The in-regime captures print those queue states twice (2012-05-28 and
// 2012-06-07); the disputed remainders moved into the owners' evidence files
// as disclosed residuals, each with the CME desk ask as its named closer, and
// `CoverageGapReason::NormalWeekPhaseWithheld` stays on the enum as the
// vocabulary a future required-phase gap would declare, exactly as the #152
// and #93 reasons before it.

// The `UnpublishedClosureDates` declaration that used to live here — `eurex`'s
// undated German-scope closures for the `tba` era of the 2025 and 2026 Trading
// Calendar editions (#157) — retired on 2026-10-05 UTC, verified no-changes.
// The four-leg chain, with every artifact's URL, retrieval instant and
// sha256, is in `docs/evidence/eurex.md` ("The #157 verification") and the
// research store's `holidays/raw/eurex/negation-2026-10-05/`.

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
        // The floor-era grid and both queues are sourced (notices
        // 20090831/0907/0914); the Sunday quarter-hour's undated 2012 move is a
        // disclosed residual (#79, retired 2026-10-04), not a declaration.
        Exchange::Cme => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::Cbot => DeclaredSourcing::carried_below(horizon!(2010, 3, 15)),
        Exchange::Comex => DeclaredSourcing::carried_below(horizon!(2012, 5, 11)),
        Exchange::Nymex => DeclaredSourcing::carried_below(horizon!(2012, 5, 11)),
        Exchange::Cfe => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        // `—`: closed before FairX's exact 2021-06-28 08:00 CT launch.
        Exchange::CoinbaseDerivatives => DeclaredSourcing::nothing_carried(),
        // `—`: closed before its own 2020-05-18 first trade date, and closed from 2025-03-24.
        Exchange::Smfe => DeclaredSourcing::nothing_carried(),
        // The floor is sourced (archived specifications through 2017, circular
        // 088/2018's redline); the `tba` German-scope declaration this arm used
        // to carry is retired (2026-10-05, no-changes verified — see the note
        // above and docs/evidence/eurex.md).
        Exchange::Eurex => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
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
        // (notices 20090831/0907/0914); the Sunday quarter-hour's undated 2012
        // move is a disclosed residual (#79, retired 2026-10-04), not a
        // declaration.
        MarketHoursKey::GlobexEquityIndex => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        MarketHoursKey::GlobexEnergy => DeclaredSourcing::carried_below(horizon!(2012, 5, 11)),
        // The 100-oz silver family shares the energy/metals tables by
        // reference, so it carries the family's own horizon; like Rough Rice,
        // a key referencing another family's rows declares no phase gaps of
        // its own — the family's Sunday quarter-hour residual (#79, retired
        // 2026-10-04) is the family's, recorded beside its rows and in this
        // key's evidence file.
        MarketHoursKey::GlobexSilver100Oz => DeclaredSourcing::carried_below(horizon!(2012, 5, 11)),
        // The 2012-05-20..2013-04-06 regime's queues are served from the
        // regime's own dated start (the #259 retirement, 2026-10-04); the
        // eight-day onset bracket is the residual the evidence file records.
        MarketHoursKey::GlobexGrains => DeclaredSourcing::carried_below(horizon!(2010, 3, 15)),
        MarketHoursKey::GlobexMiniGrains => DeclaredSourcing::carried_below(horizon!(2010, 4, 5)),
        // Every special session CME publishes for this family now ships as a
        // row, so the #93 declaration it used to carry is gone; the floor-era
        // matching grid is sourced (FX-hours page, capture 2009-05-02), and the
        // Sunday quarter-hour's undated 2012 move is a disclosed residual
        // (#79, retired 2026-10-04), not a declaration.
        MarketHoursKey::GlobexFx => DeclaredSourcing::carried_below(horizon!(2012, 5, 3)),
        MarketHoursKey::GlobexInterestRates => {
            DeclaredSourcing::carried_below(horizon!(2010, 1, 1))
        }
        // The #152 post-close label declaration this arm used to carry is
        // retired by the charter's 2026-10-03 trade-date convention; the arm
        // restates the horizon alone.
        MarketHoursKey::GlobexLivestock => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        // `—`: closed before the exact 2017-12-17 launch grid. Both of this
        // family's declared session shapes now ship as rows; the five-day
        // era's undated Pre-Open onset is a disclosed residual (#123, retired
        // 2026-10-04) — the era answers from the grid with the queues absent.
        MarketHoursKey::GlobexCryptocurrency => DeclaredSourcing::nothing_carried(),
        MarketHoursKey::CfeVix => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        // Like the venue's own arm: the `tba` declaration retired 2026-10-05.
        MarketHoursKey::Eurex => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
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
