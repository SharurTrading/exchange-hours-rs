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
//! `DeclaredSourcing::phase_gap` is the sibling assertion for
//! **phase-level** gaps: the shapes `docs/schedules/coverage-2025.md`'s
//! `Missing / disputed` column records but no date walk can find, because the
//! arrangement the operator publishes applies to every date the claim covers
//! rather than to a span of dates. `CoverageGapReason::NormalWeekPhaseWithheld`
//! is the required-phase shape (#79) and
//! `CoverageGapReason::SpecialSessionUnrepresentable` the special-session shape
//! (#93). Each declaration carries the issue whose closure discharges it, so the
//! metadata reports a gap with a closing condition rather than a bare verdict —
//! LAW-COVERAGE's "a recorded gap with a closing condition", asserted by the
//! crate rather than inferred from its data.
//!
//! Only an identity whose gap survives the permanent 2025 floor is declared
//! here, because that is the interval the completeness claim covers: a scope
//! whose withheld phase or unstateable session lies entirely before 2025 is not
//! incomplete in the claimed interval and must not be declared. Three scopes
//! carry one today, and the fence in
//! `tests/schedule_documentation/coverage_inventory.rs` holds them to the
//! inventory's own verdicts.
//!
//! `tests/schedule_documentation/horizons.rs` re-reads the ledger and fails if
//! any arm here drifts from the cell it restates.

use chrono::NaiveDate;

use super::timeline::horizon;
use crate::calendar::coverage::{CoverageGapReason, PhaseGap};
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
    /// The phase-level gap this identity carries into the 2025-onward claim, or
    /// `None` when it carries none.
    ///
    /// Like `observes_no_holidays`, this is an affirmative assertion and never
    /// an inference: an identity with no declaration reports no phase-level gap,
    /// and the inventory's `Missing / disputed` cell is what a reviewer reads
    /// before adding one.
    pub(crate) phase_gap: Option<PhaseGap>,
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
            phase_gap: None,
        }
    }

    /// The ledger's em dash: nothing is carried, and this identity ships a
    /// holiday table.
    const fn nothing_carried() -> Self {
        Self {
            carried_below: None,
            observes_no_holidays: false,
            phase_gap: None,
        }
    }

    /// A scope whose own definition has no holiday closures: the two synthetic
    /// 24×7 identities, which are library policy rather than a venue, and the
    /// one operator that publishes continuous 24/7 availability.
    const fn no_holidays() -> Self {
        Self {
            carried_below: None,
            observes_no_holidays: true,
            phase_gap: None,
        }
    }

    /// `nothing_carried()`, plus the phase-level gap the identity declares.
    ///
    /// The three declarations below are the ones `docs/schedules/coverage-2025.md`
    /// calls incomplete for a reason its date columns cannot show, so each also
    /// carries that page's closing issue.
    const fn nothing_carried_with(phase_gap: PhaseGap) -> Self {
        Self {
            carried_below: None,
            observes_no_holidays: false,
            phase_gap: Some(phase_gap),
        }
    }

    /// `carried_below(horizon)`, plus the phase-level gap the identity declares.
    const fn carried_below_with(horizon: Option<NaiveDate>, phase_gap: PhaseGap) -> Self {
        Self {
            carried_below: horizon,
            observes_no_holidays: false,
            phase_gap: Some(phase_gap),
        }
    }
}

/// The required-phase gap `globex_equity_index` carries: CME's Sunday
/// 16:00-16:15 CT quarter-hour is withheld (#79).
///
/// `docs/schedules/coverage-2025.md` records it as "the 16:00-16:15 CT Sunday
/// quarter-hour, withheld (#79)" and `docs/evidence/cme.md` plus the family's
/// own module record that only the disputed quarter-hour depends on the undated
/// 2012 move, so the crate serves the sourced 16:15-17:00 CT intersection
/// instead.
const fn withheld_sunday_quarter_hour() -> PhaseGap {
    PhaseGap::new(CoverageGapReason::NormalWeekPhaseWithheld, "#79")
}

/// The special-session gap `globex_fx` and `globex_cryptocurrency` carry: CME
/// publishes sessions the scalar layer cannot state (#93).
///
/// `docs/schedules/coverage-2025.md` records both scopes as "special-session
/// dates the scalar layer cannot state (#93)"; each owner's evidence file lists
/// the Saturday sessions and merged trade dates behind that verdict.
const fn unstateable_special_sessions() -> PhaseGap {
    PhaseGap::new(CoverageGapReason::SpecialSessionUnrepresentable, "#93")
}

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
        Exchange::Nasdaq => DeclaredSourcing::carried_below(horizon!(2013, 3, 18)),
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
        Exchange::Nyse => DeclaredSourcing::carried_below(horizon!(2018, 4, 9)),
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
        Exchange::Cme => DeclaredSourcing::carried_below(horizon!(2012, 5, 3)),
        Exchange::Cbot => DeclaredSourcing::carried_below(horizon!(2010, 3, 15)),
        Exchange::Comex => DeclaredSourcing::carried_below(horizon!(2012, 5, 11)),
        Exchange::Nymex => DeclaredSourcing::carried_below(horizon!(2012, 5, 11)),
        Exchange::Cfe => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        // `—`: closed before FairX's exact 2021-06-28 08:00 CT launch.
        Exchange::CoinbaseDerivatives => DeclaredSourcing::nothing_carried(),
        // `—`: closed before its own 2020-05-18 first trade date, and closed from 2025-03-24.
        Exchange::Smfe => DeclaredSourcing::nothing_carried(),
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
        Exchange::Asx => DeclaredSourcing::carried_below(horizon!(2025, 6, 23)),
        // `—`: closed before the 2011-10-31 Chi-X Australia launch.
        Exchange::TmxAustralia => DeclaredSourcing::nothing_carried(),
        Exchange::Nzx => DeclaredSourcing::carried_below(horizon!(2020, 4, 6)),
        Exchange::Tse => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::NseIndia => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::BseIndia => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::Hkex => DeclaredSourcing::carried_below(horizon!(2011, 3, 3)),
        Exchange::SgxSecurities => DeclaredSourcing::carried_below(horizon!(2011, 8, 1)),
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
        Exchange::BorsaIstanbul => DeclaredSourcing::carried_below(horizon!(2012, 3, 2)),
        Exchange::Tsx => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        Exchange::Jse => DeclaredSourcing::carried_below(horizon!(2012, 5, 25)),
        Exchange::Tadawul => DeclaredSourcing::carried_below(horizon!(2013, 6, 29)),
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
        // The Sunday 16:00-16:15 CT quarter-hour is withheld, so no date in the
        // 2025-onward claim is answered from a complete normal week (#79).
        MarketHoursKey::GlobexEquityIndex => DeclaredSourcing::carried_below_with(
            horizon!(2012, 5, 3),
            withheld_sunday_quarter_hour(),
        ),
        MarketHoursKey::GlobexEnergy => DeclaredSourcing::carried_below(horizon!(2012, 5, 11)),
        MarketHoursKey::GlobexGrains => DeclaredSourcing::carried_below(horizon!(2010, 3, 15)),
        MarketHoursKey::GlobexMiniGrains => DeclaredSourcing::carried_below(horizon!(2010, 4, 5)),
        // CME publishes FX sessions the scalar layer cannot state (#93).
        MarketHoursKey::GlobexFx => DeclaredSourcing::carried_below_with(
            horizon!(2012, 5, 3),
            unstateable_special_sessions(),
        ),
        MarketHoursKey::GlobexInterestRates => {
            DeclaredSourcing::carried_below(horizon!(2010, 1, 1))
        }
        MarketHoursKey::GlobexLivestock => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        // `—`: closed before the exact 2017-12-17 launch grid. CME publishes
        // cryptocurrency sessions the scalar layer cannot state (#93).
        MarketHoursKey::GlobexCryptocurrency => {
            DeclaredSourcing::nothing_carried_with(unstateable_special_sessions())
        }
        MarketHoursKey::CfeVix => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        MarketHoursKey::Eurex => DeclaredSourcing::carried_below(horizon!(2010, 1, 1)),
        // `—`: closed before the sourced 2017-11-07 launch-eve opening.
        MarketHoursKey::IceUs => DeclaredSourcing::nothing_carried(),
        MarketHoursKey::IceUsSugar => DeclaredSourcing::carried_below(horizon!(2011, 8, 1)),
        MarketHoursKey::IceUsCoffee => DeclaredSourcing::carried_below(horizon!(2011, 8, 1)),
        MarketHoursKey::IceUsCocoa => DeclaredSourcing::carried_below(horizon!(2011, 8, 1)),
        MarketHoursKey::IceUsCotton => DeclaredSourcing::carried_below(horizon!(2011, 8, 1)),
        MarketHoursKey::IceUsOrangeJuice => DeclaredSourcing::carried_below(horizon!(2011, 8, 1)),
        MarketHoursKey::IceUsDollarIndex => DeclaredSourcing::carried_below(horizon!(2011, 2, 7)),
        // `—`: sessionless before its grid's 2011-01-12 first sourced appearance — a recorded gap below the support floor.
        MarketHoursKey::GlobexNikkei225Dollar => DeclaredSourcing::nothing_carried(),
        MarketHoursKey::EurexFixedIncome => DeclaredSourcing::carried_below(horizon!(2018, 11, 15)),
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
