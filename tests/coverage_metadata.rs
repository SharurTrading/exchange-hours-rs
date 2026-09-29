// SPDX-License-Identifier: MIT-0

//! Public-surface contracts for coverage metadata and the coverage-error
//! vocabulary (LAW-COVERAGE).
//!
//! Every value here is read back through the crate's public API; nothing peeks
//! at a module's tables. The fixtures pin the ledger's own cells: a scope that
//! is complete to its published horizon, three served scopes that ship no 2025
//! holiday coverage, the two venue intersections that withhold dates as
//! `Unsourced`, an identity with no holiday table, and the scopes whose own
//! definition has no holidays at all.

#![expect(
    clippy::expect_used,
    reason = "fixture constructors assert their own literals; a bad literal must fail the test"
)]

use chrono::{DateTime, Datelike as _, NaiveDate, TimeZone as _, Utc};
use chrono_tz::{Asia, US};
use exchange_hours::{
    CalendarCoverage, CalendarQueryError, CalendarSource, CoverageGap, CoverageGapReason,
    DateCoverage, DateRange, Exchange, ExchangeCalendar, HolidayContract, MarketHoursKey,
    SUPPORT_FLOOR, calendar_for_exchange, calendar_for_market_hours_key,
};

/// The last date the per-identity date-by-date cross-check walks.
///
/// Every shipped holiday window ends by 2028-01-03, so this reaches past all
/// audited data while staying bounded for an open-ended span.
const CHECK_HORIZON: NaiveDate = match NaiveDate::from_ymd_opt(2028, 12, 31) {
    Some(date) => date,
    None => NaiveDate::MAX,
};

/// A venue-local fixture date.
fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("fixture must be a valid date")
}

/// The open-ended span the crate uses for "no known end".
fn unbounded(first: NaiveDate) -> DateRange {
    DateRange::new(first, NaiveDate::MAX).expect("an open-ended span is ascending")
}

/// The metadata a venue calendar reports.
fn exchange_coverage(exchange: Exchange) -> CalendarCoverage {
    calendar_for_exchange(exchange).coverage()
}

/// The metadata a product-family calendar reports.
fn key_coverage(key: MarketHoursKey) -> CalendarCoverage {
    calendar_for_market_hours_key(key).coverage()
}

/// The reason `day` is incomplete, taken from the reported gap spans.
fn gap_reason_on(coverage: CalendarCoverage, day: NaiveDate) -> Option<CoverageGapReason> {
    coverage
        .gaps()
        .find(|gap| gap.range().contains(day))
        .map(CoverageGap::reason)
}

/// Every date from the floor through `last`, for a bounded walk.
fn days_from_floor(last: NaiveDate) -> Vec<NaiveDate> {
    let mut days = Vec::new();
    let mut day = SUPPORT_FLOOR;
    while day <= last {
        days.push(day);
        match day.succ_opt() {
            Some(next) => day = next,
            None => break,
        }
    }
    days
}

#[test]
fn the_support_floor_is_local_first_of_january_2010() {
    assert_eq!(SUPPORT_FLOOR, date(2010, 1, 1));
}

#[test]
fn a_complete_scope_reports_one_complete_span_and_a_trailing_gap() {
    // `globex_nikkei_225_dollar` is the one scope that still reaches 2027-12-31
    // with nothing withheld *in the 2025+ era*: it ships no order-entry phase
    // at all, so the post-close queue label `globex_grains` and
    // `globex_livestock` declare (#152) cannot apply to it. At the 2010 floor
    // its earlier history is honestly partial, and the spans show it.
    let coverage = key_coverage(MarketHoursKey::GlobexNikkei225Dollar);
    assert_eq!(
        coverage.identity(),
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexNikkei225Dollar)
    );
    assert_eq!(coverage.normal_week_sourced_from(), None);
    assert_eq!(coverage.sourced_normal_week(), unbounded(SUPPORT_FLOOR));

    // At the 2010 floor the scope's earlier history is honestly partial: the
    // 2010-2015 interval is unaudited (its first audited window opens
    // 2016-01-01) and the 2016-2024 windows withhold dates, so the complete
    // spans begin at that window and split around the withheld dates instead
    // of forming one span from the floor.
    let complete: Vec<DateRange> = coverage.complete_ranges().collect();
    assert_eq!(
        complete.first(),
        Some(&DateRange::new(date(2016, 1, 1), date(2018, 12, 31)).expect("ascending")),
        "the first complete span is the first audited window"
    );
    assert_eq!(
        complete.last(),
        Some(&DateRange::new(date(2025, 1, 1), date(2027, 12, 31)).expect("ascending")),
        "the last complete span is the complete 2025+ era"
    );
    let gaps: Vec<CoverageGap> = coverage.gaps().collect();
    assert_eq!(
        gaps.first().map(|gap| (gap.range().first(), gap.reason())),
        Some((date(2010, 1, 1), CoverageGapReason::NoHolidayCoverage)),
        "the floor-to-first-window interval is an unaudited gap"
    );
    assert_eq!(
        gaps.last().map(|gap| (gap.range().first(), gap.reason())),
        Some((date(2028, 1, 1), CoverageGapReason::NoHolidayCoverage)),
        "the trailing gap past the data is unchanged"
    );

    assert!(coverage.is_complete_on(date(2025, 6, 2)));
    assert_eq!(
        coverage.coverage_on(date(2027, 12, 31)),
        DateCoverage::Covered
    );
    assert_eq!(
        coverage.coverage_on(date(2028, 1, 1)),
        DateCoverage::OutsideCoveredRange
    );

    let contract = coverage.holiday_contract();
    assert!(matches!(contract, HolidayContract::Audited { .. }));
    assert!(contract.rows().is_some_and(|rows| rows > 0));
    assert!(contract.coverage().is_some_and(|windows| {
        windows.first() == date(2016, 1, 1) && windows.last() == date(2027, 12, 31)
    }));
}

#[test]
fn scopes_without_2025_holiday_coverage_report_outside_range() {
    // The list this test was written for is empty: `cfe` and `eurex` left it
    // when their 2025 rows landed on 2026-09-26 UTC, and `iceus` — the last
    // served scope whose window opened after the floor — left it the same day
    // when its 2025 rows landed. `iceus_2025_holiday_rows_report_covered` below
    // is what holds it to the opposite claim, and `iceus` is asserted there to
    // withhold fourteen 2025 dates rather than to answer 2025 completely.
    //
    // The claim is kept as a loop over every served exchange so a scope that
    // regresses to a post-floor window fails here rather than passing by
    // absence. The per-date verdict is deliberately not asserted `Covered`:
    // `cme` answers `OutsideCoveredRange` on the sample for its own declared
    // Sunday quarter-hour gap (#79), which is a different denial from the
    // missing-window shape this test exists for.
    for exchange in [
        Exchange::Cme,
        Exchange::Cbot,
        Exchange::Comex,
        Exchange::Nymex,
        Exchange::Cfe,
        Exchange::Eurex,
        Exchange::Iceus,
        Exchange::CoinbaseDerivatives,
    ] {
        let coverage = exchange_coverage(exchange);
        let day = date(2025, 6, 2);
        assert_ne!(
            gap_reason_on(coverage, day),
            Some(CoverageGapReason::NoHolidayCoverage),
            "{exchange:?} answers a holiday question in 2025"
        );
        assert!(
            coverage
                .holiday_contract()
                .coverage()
                .is_some_and(|windows| windows.first() <= date(2025, 1, 1)),
            "{exchange:?} opens its audited window no later than the 2025 floor"
        );
    }
}

/// The reverse claim for the scope that left the list above.
///
/// A served market with no 2025 answer was the whole defect, so this asserts
/// the public metadata now answers 2025 — through the venue and through the
/// routed keys — and cannot silently fall back out of its window. The 2025
/// interval is **not** complete: fourteen of its dates are withheld as
/// `Unsourced`, which is what the inventory row's `**incomplete**` verdict and
/// its withheld count record.
#[test]
fn iceus_2025_holiday_rows_report_covered() {
    let day = date(2025, 6, 2);
    for calendar in [
        calendar_for_exchange(Exchange::Iceus),
        calendar_for_market_hours_key(MarketHoursKey::IceUs),
        calendar_for_market_hours_key(MarketHoursKey::IceUsSugar),
        calendar_for_market_hours_key(MarketHoursKey::IceUsOrangeJuice),
        calendar_for_market_hours_key(MarketHoursKey::IceUsDollarIndex),
    ] {
        let coverage = calendar.coverage();
        assert_eq!(coverage.coverage_on(day), DateCoverage::Covered);
        assert_eq!(gap_reason_on(coverage, day), None);
        assert!(coverage.is_complete_on(day));
        assert_eq!(
            coverage.complete_ranges().next().map(DateRange::first),
            Some(date(2025, 1, 1)),
            "the audited window now opens at the 2025 floor"
        );
        assert!(
            coverage
                .holiday_contract()
                .coverage()
                .is_some_and(|windows| windows.first() == date(2025, 1, 1)
                    && windows.last() == date(2028, 1, 3)),
            "iceus must audit the 2025 floor through the 2027 calendar's last entry"
        );
    }
    // The dates the venue withholds inside that window: Juneteenth 2025 is one
    // (the softs close while the index families shorten), and it is the reason
    // the interval cannot read complete.
    let coverage = exchange_coverage(Exchange::Iceus);
    assert_eq!(
        coverage.coverage_on(date(2025, 6, 19)),
        DateCoverage::UnresolvedGap
    );
    assert_eq!(
        gap_reason_on(coverage, date(2025, 6, 19)),
        Some(CoverageGapReason::WithheldDate)
    );
}

/// The reverse claim for the scope that left the list above.
///
/// A served market with no 2025 answer was the whole defect, so this asserts
/// the public metadata now answers 2025 in both directions — through the venue
/// and through the routed key — and cannot silently fall back out of its
/// window.
#[test]
fn cfe_2025_holiday_rows_report_covered() {
    let day = date(2025, 6, 2);
    for calendar in [
        calendar_for_exchange(Exchange::Cfe),
        calendar_for_market_hours_key(MarketHoursKey::CfeVix),
    ] {
        let coverage = calendar.coverage();
        assert_eq!(coverage.coverage_on(day), DateCoverage::Covered);
        assert_eq!(gap_reason_on(coverage, day), None);
        assert!(coverage.is_complete_on(day));
        assert_eq!(
            coverage.complete_ranges().next().map(DateRange::first),
            Some(date(2025, 1, 1))
        );
        assert!(
            coverage
                .holiday_contract()
                .coverage()
                .is_some_and(|windows| windows.first() == date(2025, 1, 1)
                    && windows.last() == date(2026, 12, 31)),
            "cfe must audit the 2025 floor through the published 2026 schedule"
        );
    }
}

#[test]
fn withheld_dates_are_unresolved_gaps_inside_an_audited_window() {
    // `cme` is deliberately absent: it now declares the Sunday quarter-hour
    // phase-level gap (#79), which is checked before the date-level facts, so
    // inside the dated era its per-date verdict for a withheld date is
    // `OutsideCoveredRange` rather than `UnresolvedGap`. From 2026-08-22 the
    // declaration is retired and 13 of its 32 withheld dates again answer
    // `UnresolvedGap`; neither is the plain shape these two intersections show.
    let fixtures = [
        (Exchange::Cbot, date(2025, 1, 2), date(2025, 1, 3)),
        (Exchange::Iceus, date(2026, 1, 19), date(2026, 1, 20)),
    ];
    for (exchange, withheld, neighbour) in fixtures {
        let coverage = exchange_coverage(exchange);
        assert_eq!(
            coverage.coverage_on(withheld),
            DateCoverage::UnresolvedGap,
            "{exchange:?} withholds {withheld} as Unsourced"
        );
        assert_eq!(
            gap_reason_on(coverage, withheld),
            Some(CoverageGapReason::WithheldDate),
            "{exchange:?}"
        );
        assert!(
            coverage.is_complete_on(neighbour),
            "{exchange:?} answers {neighbour} completely"
        );
        assert!(
            coverage
                .complete_ranges()
                .all(|range| !range.contains(withheld)),
            "{exchange:?} must exclude a withheld date from its complete ranges"
        );
    }
}

#[test]
fn without_holidays_selects_the_normal_week_contract() {
    // `cbot`, not `cme`: the venue intersection with no phase-level declaration,
    // so the normal-week contract is the first fact the detach selects. `cme`
    // declares the withheld Sunday quarter-hour (#79), which is checked before
    // the holiday layer and therefore would outrank `NormalWeekOnly`.
    let attached = calendar_for_exchange(Exchange::Cbot).coverage();
    let detached = calendar_for_exchange(Exchange::Cbot)
        .without_holidays()
        .coverage();

    assert_eq!(detached.holiday_contract(), HolidayContract::NormalWeekOnly);
    assert_eq!(detached.complete_ranges().count(), 0);
    // At the 2010 floor `cbot`'s 2010-03-15 horizon splits the contract into
    // its carried era and its sourced era, and the shipped audited windows
    // segment the rest — the walk's edges are the windows the identity ships,
    // so a detached view reports the same segmentation the attached one does.
    let gaps: Vec<CoverageGap> = detached.gaps().collect();
    assert_eq!(gaps.len(), 8);
    assert_eq!(
        gaps[0].range(),
        DateRange::new(SUPPORT_FLOOR, date(2010, 3, 14)).expect("ascending")
    );
    // The carried era reads `NormalWeekCarried` — the horizon is a
    // normal-week fact the detach keeps — and every sourced era reads
    // `NormalWeekOnly`, split at the shipped windows' edges.
    assert_eq!(gaps[0].reason(), CoverageGapReason::NormalWeekCarried);
    assert_eq!(
        gaps[1].range(),
        DateRange::new(date(2010, 3, 15), date(2012, 12, 31)).expect("ascending")
    );
    assert_eq!(gaps[1].reason(), CoverageGapReason::NormalWeekOnly);
    assert!(
        gaps[2..]
            .iter()
            .all(|gap| gap.reason() == CoverageGapReason::NormalWeekOnly)
    );
    assert_eq!(
        gaps.last().map(|gap| gap.range().is_open_ended()),
        Some(true)
    );
    assert_eq!(
        detached.coverage_on(date(2025, 6, 2)),
        DateCoverage::NormalWeekOnly
    );

    // Detaching is a view of the same identity, not a different normal week.
    assert_eq!(
        detached.normal_week_sourced_from(),
        attached.normal_week_sourced_from()
    );
    assert_eq!(
        detached.sourced_normal_week(),
        attached.sourced_normal_week()
    );
    assert_eq!(detached.identity(), attached.identity());
    assert_ne!(detached, attached);
    assert!(
        calendar_for_exchange(Exchange::Cbot)
            .holiday_coverage()
            .is_some(),
        "the identity still ships its table; only this calendar detached it"
    );
}

#[test]
fn an_identity_with_no_holiday_table_reports_no_table() {
    // `nasdaq_bx` stands in here since 2026-09-28 UTC: the `nasdaq` venue
    // shipped its own holiday table and left the no-table set. `bmv` took
    // `tsx`'s place the same day for the same reason.

    for exchange in [Exchange::NasdaqBx, Exchange::MemxEq, Exchange::Bmv] {
        let coverage = exchange_coverage(exchange);
        assert_eq!(coverage.holiday_contract(), HolidayContract::NoTable);
        assert_eq!(coverage.complete_ranges().count(), 0);
        assert_eq!(
            coverage.coverage_on(date(2025, 6, 2)),
            DateCoverage::OutsideCoveredRange
        );
        assert_eq!(
            gap_reason_on(coverage, date(2025, 6, 2)),
            Some(CoverageGapReason::NoHolidayTable)
        );
    }
}

#[test]
fn a_no_holiday_assertion_is_distinct_from_a_missing_table() {
    let binance = exchange_coverage(Exchange::BinanceFutures);
    assert_eq!(binance.holiday_contract(), HolidayContract::NoHolidays);
    assert_eq!(binance.normal_week_sourced_from(), None);
    assert!(binance.is_complete_on(date(2025, 6, 2)));
    assert_eq!(binance.gaps().count(), 0);
    assert_eq!(
        binance.complete_ranges().next(),
        Some(unbounded(SUPPORT_FLOOR))
    );

    let synthetic = key_coverage(MarketHoursKey::AlwaysOpen);
    assert_eq!(synthetic.holiday_contract(), HolidayContract::NoHolidays);
    assert_eq!(synthetic.gaps().count(), 0);

    assert_ne!(binance.holiday_contract(), HolidayContract::NoTable);
    assert_ne!(synthetic.holiday_contract(), HolidayContract::NoTable);

    // A table with real coverage and zero rows says "every date in this window
    // was audited and found normal" — the audited-normal assertion the plan
    // requires, and a different statement from "no table".
    let audited_normal = HolidayContract::Audited {
        coverage: calendar_for_exchange(Exchange::Cfe)
            .holiday_coverage()
            .expect("CFE ships a 2026 window"),
        rows: 0,
    };
    assert_eq!(audited_normal.rows(), Some(0));
    assert!(audited_normal.coverage().is_some());
    assert_ne!(audited_normal, HolidayContract::NoTable);
    assert_ne!(audited_normal, HolidayContract::NoHolidays);
}

#[test]
fn sourced_normal_week_reports_the_ledger_horizon() {
    assert_eq!(
        exchange_coverage(Exchange::Cme).normal_week_sourced_from(),
        Some(date(2012, 5, 3))
    );
    assert_eq!(
        exchange_coverage(Exchange::Nyse).normal_week_sourced_from(),
        Some(date(2018, 4, 9))
    );
    // The ledger's em dash: nothing is carried below the identity's own first
    // row, so there is no date below which its rows are carried.
    assert_eq!(
        exchange_coverage(Exchange::CoinbaseDerivatives).normal_week_sourced_from(),
        None
    );
    assert_eq!(
        exchange_coverage(Exchange::NasdaqPsx).normal_week_sourced_from(),
        None
    );

    let carteret = exchange_coverage(Exchange::FinraTrfCarteret);
    assert_eq!(carteret.normal_week_sourced_from(), Some(date(2026, 3, 30)));
    assert_eq!(carteret.sourced_normal_week().first(), date(2026, 3, 30));
    assert_eq!(
        carteret.coverage_on(date(2025, 6, 2)),
        DateCoverage::OutsideCoveredRange
    );
    assert_eq!(
        gap_reason_on(carteret, date(2025, 6, 2)),
        Some(CoverageGapReason::NormalWeekCarried)
    );

    let asx = exchange_coverage(Exchange::Asx);
    assert_eq!(asx.normal_week_sourced_from(), Some(date(2025, 6, 23)));
    assert_eq!(
        gap_reason_on(asx, date(2025, 6, 22)),
        Some(CoverageGapReason::NormalWeekCarried)
    );
    assert_ne!(
        gap_reason_on(asx, date(2025, 6, 23)),
        Some(CoverageGapReason::NormalWeekCarried)
    );
}

#[test]
fn a_date_before_the_floor_is_before_floor_for_every_identity() {
    let before = date(2009, 12, 31);
    for &exchange in Exchange::ALL {
        let coverage = exchange_coverage(exchange);
        assert_eq!(
            coverage.coverage_on(before),
            DateCoverage::BeforeSupportFloor,
            "{exchange:?}"
        );
        assert!(!coverage.is_complete_on(before));
    }
    for &key in MarketHoursKey::ALL {
        let coverage = key_coverage(key);
        assert_eq!(
            coverage.coverage_on(before),
            DateCoverage::BeforeSupportFloor,
            "{key:?}"
        );
        assert!(!coverage.is_complete_on(before));
    }
}

#[test]
fn the_floor_is_a_local_date_not_a_utc_instant() {
    let instant = Utc
        .with_ymd_and_hms(2010, 1, 1, 0, 0, 0)
        .single()
        .expect("a valid instant");
    let tokyo_day = instant.with_timezone(&Asia::Tokyo).date_naive();
    let chicago_day = instant.with_timezone(&US::Central).date_naive();
    assert_eq!(tokyo_day, date(2010, 1, 1));
    assert_eq!(chicago_day, date(2009, 12, 31));

    // One UTC instant, two verdicts: Tokyo's 2010-01-01 is inside the supported
    // domain while Chicago is still on 2009-12-31. A single UTC midnight for
    // every venue would misjudge one of them.
    assert_ne!(
        exchange_coverage(Exchange::Tse).coverage_on(tokyo_day),
        DateCoverage::BeforeSupportFloor
    );
    assert_eq!(
        exchange_coverage(Exchange::Cme).coverage_on(chicago_day),
        DateCoverage::BeforeSupportFloor
    );
}

#[test]
fn calendar_query_errors_are_distinguishable_by_a_caller() {
    let source = CalendarSource::Exchange(Exchange::Cme);
    let bound = date(2025, 6, 16);
    let errors = [
        CalendarQueryError::BeforeSupportFloor {
            source,
            date: date(2009, 12, 31),
        },
        CalendarQueryError::OutsideCoveredRange {
            source,
            date: date(2025, 6, 2),
        },
        CalendarQueryError::UnresolvedGap {
            source,
            date: date(2025, 1, 2),
        },
        CalendarQueryError::SearchExhausted {
            source,
            date: date(2025, 6, 2),
            bound,
        },
    ];

    for error in errors {
        assert_eq!(error.source(), source);
    }
    assert_eq!(errors[0].date(), date(2009, 12, 31));
    match errors[3] {
        CalendarQueryError::SearchExhausted { bound: hit, .. } => assert_eq!(hit, bound),
        other => panic!("expected an exhausted search, got {other:?}"),
    }

    // Unsupported coverage and bounded search exhaustion stay separable, which
    // is the distinction the release plan requires.
    assert!(matches!(
        errors[1],
        CalendarQueryError::OutsideCoveredRange { .. }
    ));
    assert!(matches!(
        errors[3],
        CalendarQueryError::SearchExhausted { .. }
    ));

    for (index, first) in errors.iter().enumerate() {
        for second in &errors[index + 1..] {
            assert_ne!(first, second);
            assert_ne!(
                first.to_string(),
                second.to_string(),
                "two variants must not read alike"
            );
        }
    }

    let boxed: Box<dyn std::error::Error> = Box::new(errors[0]);
    let rendered = boxed.to_string();
    assert!(rendered.contains("cme"), "{rendered}");
    assert!(rendered.contains("2009-12-31"), "{rendered}");
    assert!(rendered.contains("2010-01-01"), "{rendered}");
}

#[test]
fn every_identity_reports_bounded_consistent_metadata() {
    for &exchange in Exchange::ALL {
        check_metadata(
            calendar_for_exchange(exchange),
            CalendarSource::Exchange(exchange),
        );
    }
    for &key in MarketHoursKey::ALL {
        check_metadata(
            calendar_for_market_hours_key(key),
            CalendarSource::MarketHoursKey(key),
        );
    }
}

/// Asserts one identity's declared phase-level gaps are part of its partition:
/// every record a declaration is the answer for carries it over one maximal run,
/// and a whole-domain `EveryDay` declaration leaves nothing complete beside it.
fn check_declarations(
    coverage: CalendarCoverage,
    complete: &[DateRange],
    gaps: &[CoverageGap],
    identity: CalendarSource,
) {
    let whole_domain = coverage.phase_gaps().iter().any(|gap| {
        gap.applies_since().is_none()
            && gap.applies_until().is_none()
            && gap.shape() == exchange_hours::PhaseGapShape::EveryDay
    });
    if coverage.phase_gaps().is_empty() {
        assert!(
            gaps.iter().all(|gap| gap.phase_gap().is_none()),
            "{identity:?} declares nothing, so no gap may carry a declaration"
        );
        return;
    }
    // Complete spans are empty exactly while nothing survives the declarations:
    // a whole-domain `EveryDay` declaration claims every answered date, or every
    // declaration is `EveryDay`, bounded, and outlives the audited windows —
    // `eurex` is the shipped case, whose editions end 2026-12-31 and whose bound
    // is the day after.
    let all_every_day = coverage
        .phase_gaps()
        .iter()
        .all(|gap| gap.shape() == exchange_hours::PhaseGapShape::EveryDay);
    let audited_end = coverage
        .holiday_contract()
        .coverage()
        .map(exchange_hours::HolidayCoverage::last);
    let latest_bound = coverage
        .phase_gaps()
        .iter()
        .filter_map(|gap| gap.applies_until())
        .max();
    let nothing_survives = all_every_day
        && matches!((audited_end, latest_bound), (Some(end), Some(bound)) if end < bound);
    assert_eq!(
        complete.is_empty(),
        whole_domain || nothing_survives,
        "{identity:?}: no complete span exactly while nothing survives the declarations"
    );
    assert!(
        gaps.iter().any(|gap| gap.phase_gap().is_some()),
        "{identity:?} reports its declarations among its records"
    );
    for gap in gaps {
        let Some(declaration) = gap.phase_gap() else {
            continue;
        };
        assert_eq!(
            coverage
                .gaps()
                .find(|candidate| candidate.range() == gap.range()
                    && candidate.phase_gap() == Some(declaration)),
            Some(*gap),
            "{identity:?}: the declaration's record is what the walk reports there"
        );
        assert!(declaration.applies_on(gap.range().first()), "{identity:?}");
        assert!(declaration.applies_on(gap.range().last()), "{identity:?}");
        // The record's run is one maximal verdict run, but audited-window edges
        // may split two runs of the same declaration apart — the partition and
        // per-date checks above are what fence walk drift, so here only the
        // record's own dates are asserted to carry the declaration.
    }
}

/// Asserts one identity's metadata is total, bounded and a partition of the
/// supported domain.
fn check_metadata(calendar: ExchangeCalendar, identity: CalendarSource) {
    let coverage = calendar.coverage();
    assert_eq!(coverage.identity(), identity);
    assert!(coverage.sourced_normal_week().first() >= SUPPORT_FLOOR);
    // The metadata reuses the calendar's own holiday coverage, so the two
    // readers cannot drift apart.
    assert_eq!(
        coverage.holiday_contract().coverage(),
        calendar.holiday_coverage(),
        "{identity:?}"
    );

    let complete: Vec<DateRange> = coverage.complete_ranges().collect();
    let gaps: Vec<CoverageGap> = coverage.gaps().collect();
    assert!(
        complete.len() < 4096,
        "{identity:?} reports a bounded span count (1021 observed for \
         `globex_equity_index` and 1017 for `globex_grains`: a date-scoped \
         declaration reports one span per maximal run its shape resolves, so the \
         bracket-era Sundays come back one record each)"
    );
    // The walk streams every record a run at a time and stores nothing, so the
    // only bound is on the declarations themselves: the most any shipped
    // identity carries is two (`globex_grains`), against a capacity of 256.
    assert!(
        coverage.phase_gaps().len() <= exchange_hours::CoverageGaps::capacity(),
        "{identity:?} declares {} gaps, past the capacity the charter's fence \
         holds ({})",
        coverage.phase_gaps().len(),
        exchange_hours::CoverageGaps::capacity()
    );
    assert!(
        gaps.len() < 4096,
        "{identity:?} reports a bounded gap count (1069 observed for \
         `globex_grains` at the 2010 floor, one record per #152 queue run plus \
         the date-level eras)"
    );

    // A declared phase-level gap is part of the partition, not a special case
    // beside it: the identity answers nothing completely exactly while a
    // whole-domain declaration applies, and the records a declaration is the
    // answer for carry it.
    check_declarations(coverage, &complete, &gaps, identity);

    // The two walks partition [floor, NaiveDate::MAX]: no hole, no overlap. A
    // declaration record is one of the gap records, so no extra span enters the
    // partition and no deduplication is needed.
    let mut runs: Vec<(DateRange, bool)> = complete
        .iter()
        .map(|range| (*range, false))
        .chain(gaps.iter().map(|gap| (gap.range(), true)))
        .collect();
    runs.sort_by_key(|(range, _)| range.first());
    let mut next = SUPPORT_FLOOR;
    for (index, (range, _)) in runs.iter().enumerate() {
        assert_eq!(range.first(), next, "a hole or overlap for {identity:?}");
        assert!(range.last() >= range.first(), "{identity:?}");
        if range.last() == NaiveDate::MAX {
            assert_eq!(index, runs.len() - 1, "{identity:?} ended before its tail");
            break;
        }
        next = range
            .last()
            .succ_opt()
            .expect("only NaiveDate::MAX is final");
    }
    assert_eq!(
        runs.last().map(|(range, _)| range.last()),
        Some(NaiveDate::MAX),
        "{identity:?} covers the domain through its tail"
    );

    // Every reported span agrees with the per-date accessor, date by date,
    // through the end of the identity's audited data: a span that swallowed a
    // withheld date, or a date the accessor calls covered inside a gap, fails
    // here.
    for (range, is_gap) in &runs {
        let end = if range.last() < CHECK_HORIZON {
            range.last()
        } else {
            CHECK_HORIZON
        };
        let mut day = range.first();
        while day <= end {
            let verdict = coverage.coverage_on(day);
            assert_eq!(
                verdict == DateCoverage::Covered,
                !is_gap,
                "{identity:?} reports {verdict:?} for {day} inside a span it calls {}",
                if *is_gap { "incomplete" } else { "complete" }
            );
            match day.succ_opt() {
                Some(next) => day = next,
                None => break,
            }
        }
    }

    // A detached view of the same identity keeps the normal-week side and
    // consults no table: where a table shipped, no complete calendar is
    // claimed at all, and where none shipped the no-holiday assertion stands.
    let detached = calendar.without_holidays().coverage();
    assert!(
        !matches!(detached.holiday_contract(), HolidayContract::Audited { .. }),
        "{identity:?}"
    );
    if calendar.holiday_coverage().is_some() {
        assert_eq!(detached.complete_ranges().count(), 0, "{identity:?}");
    }
    assert_eq!(
        detached.normal_week_sourced_from(),
        coverage.normal_week_sourced_from(),
        "{identity:?}"
    );
    assert!(
        detached.gaps().all(|gap| !matches!(
            gap.reason(),
            CoverageGapReason::WithheldDate | CoverageGapReason::NoHolidayCoverage
        )),
        "{identity:?}"
    );
}

#[test]
fn a_complete_scope_answers_every_day_inside_its_span() {
    // A spot walk over the one scope's fully-complete era, so the span report
    // and the per-date accessor are compared date by date rather than only at
    // the edges. At the 2010 floor that era is the 2025+ interval; the earlier
    // windows split around their withheld dates.
    let coverage = key_coverage(MarketHoursKey::GlobexNikkei225Dollar);
    for day in days_from_floor(date(2027, 12, 31)) {
        if day >= date(2025, 1, 1) {
            assert!(coverage.is_complete_on(day), "{day}");
            assert_eq!(gap_reason_on(coverage, day), None, "{day}");
        }
    }
}

/// One expected declaration: reason, closing issue, `since`, `until`, shape.
type DeclaredGap = (
    CoverageGapReason,
    &'static str,
    Option<NaiveDate>,
    Option<NaiveDate>,
    exchange_hours::PhaseGapShape,
);

/// Asserts one fixture's declared phase-level gaps, the dates each applies to,
/// and the shape each carries.
///
/// The expected declaration carries its bounds and its shape verbatim, so the
/// fixture is the fence for what the declaration states — the era it names on
/// both sides and the dates its shape resolves.
fn check_declared_gaps(key: MarketHoursKey, expected: &[DeclaredGap]) {
    let coverage = key_coverage(key);
    let declared: Vec<DeclaredGap> = coverage
        .phase_gaps()
        .iter()
        .map(|gap| {
            (
                gap.reason(),
                gap.closing_condition(),
                gap.applies_since(),
                gap.applies_until(),
                gap.shape(),
            )
        })
        .collect();
    assert_eq!(declared, expected, "{key:?}");
    assert!(
        !expected.is_empty() || coverage.complete_ranges().count() > 0 || {
            // A scope with no declaration still reports no complete span when its
            // ordinary facts answer nothing — that is the date-level walk's
            // business, not the declarations'.
            false
        }
    );
}

#[test]
fn a_declared_phase_gap_applies_only_to_the_dates_its_shape_resolves() {
    // #79's quarter-hour is withheld on the bracket-era Sundays whose served
    // Pre-Open resolves — and on nothing else. A Tuesday in the same era
    // answers from the sourced weekday grid; a Sunday before the 2012-05-28
    // capture answers because the sourced state still printed 16:15 CT then; a
    // Sunday whose evening leg a holiday removes answers closed; and from the
    // 2026-08-22 knowledge-bound row the quarter-hour itself is served.
    let sample = date(2025, 6, 10);
    let bracket_sunday = date(2015, 6, 14);
    let pre_bracket_sunday = date(2012, 5, 27);
    let first_bracket_sunday = date(2012, 6, 3);
    let holiday_sunday = date(2017, 12, 24);
    let after_the_bound = date(2026, 8, 23);
    let quarter_hour = date(2026, 8, 22);
    let sunday_window = exchange_hours::PhaseGapShape::OrderEntryWindow {
        open_ssm: Some(16 * 3600 + 15 * 60),
        close_ssm: 17 * 3600,
    };
    for key in [MarketHoursKey::GlobexEquityIndex, MarketHoursKey::GlobexFx] {
        check_declared_gaps(
            key,
            &[(
                CoverageGapReason::NormalWeekPhaseWithheld,
                "#79",
                Some(date(2012, 5, 28)),
                Some(quarter_hour),
                sunday_window,
            )],
        );
        let coverage = key_coverage(key);
        assert_eq!(
            coverage.coverage_on(sample),
            DateCoverage::Covered,
            "{key:?}: a Tuesday in the bracket era answers from the sourced weekday grid"
        );
        assert_eq!(
            coverage.coverage_on(bracket_sunday),
            DateCoverage::OutsideCoveredRange,
            "{key:?}: the bracket-era Sunday withholds the quarter-hour"
        );
        assert_eq!(
            coverage.coverage_on(pre_bracket_sunday),
            DateCoverage::Covered,
            "{key:?}: the last 16:15-CT capture is 2012-05-28, so earlier Sundays answer"
        );
        assert_eq!(
            coverage.coverage_on(first_bracket_sunday),
            DateCoverage::OutsideCoveredRange,
            "{key:?}: 2012-06-03 is the first Sunday whose queue onset is in doubt"
        );
        assert_eq!(
            coverage.coverage_on(holiday_sunday),
            DateCoverage::Covered,
            "{key:?}: a Sunday whose evening leg the holiday removes answers closed"
        );
        assert_eq!(
            coverage.coverage_on(after_the_bound),
            DateCoverage::Covered,
            "{key:?}: the knowledge-bound row serves the quarter-hour"
        );
        // The gap records are the bracket-era Sundays, one run each.
        let sunday_records: Vec<DateRange> = coverage
            .gaps()
            .filter(|gap| gap.closing_condition() == Some("#79"))
            .map(exchange_hours::CoverageGap::range)
            .collect();
        assert_eq!(
            sunday_records.first(),
            Some(&DateRange::new(first_bracket_sunday, first_bracket_sunday).expect("ascending")),
            "{key:?}: the first #79 record is the first bracket-era Sunday alone"
        );
        assert!(
            sunday_records
                .iter()
                .all(|range| range.first() == range.last()
                    && range.first().format("%A").to_string() == "Sunday"),
            "{key:?}: every #79 record is one Sunday"
        );
        assert!(
            !sunday_records
                .iter()
                .any(|range| range.contains(holiday_sunday)),
            "{key:?}: the holiday-removed Sunday answers, so no record claims it"
        );
    }

    // The query surface agrees per date: the withheld phase refuses where the
    // declaration applies and answers everywhere else — a refusal is never read
    // as a closed grid, and an answered grid is never refused.
    let fx = calendar_for_market_hours_key(MarketHoursKey::GlobexFx);
    let at = |day: NaiveDate, hour: u32, minute: u32| {
        US::Central
            .from_local_datetime(
                &day.and_hms_opt(hour, minute, 0)
                    .expect("a valid local time"),
            )
            .single()
            .expect("a single local instant")
            .with_timezone(&Utc)
    };
    assert_eq!(
        fx.is_accepting_orders(at(sample, 10, 0)),
        Ok(true),
        "a Tuesday 10:00 CT in the bracket era is answered"
    );
    assert_eq!(
        fx.is_accepting_orders(at(bracket_sunday, 16, 5)),
        Err(CalendarQueryError::OutsideCoveredRange {
            source: fx.source(),
            date: bracket_sunday,
        }),
        "the withheld quarter-hour refuses the bracket-era Sunday, not the era"
    );
    assert_eq!(
        fx.is_accepting_orders(at(bracket_sunday, 12, 0)),
        Err(CalendarQueryError::OutsideCoveredRange {
            source: fx.source(),
            date: bracket_sunday,
        }),
        "the queue gate is keyed to the date the scan opens on: a bracket-era \
         Sunday whose quarter-hour is unsourced is an incomplete date, so its \
         order-entry probes state the date's verdict rather than answering \
         through a phase the same day leaves unsourceable"
    );
    assert_eq!(
        fx.is_accepting_orders(at(holiday_sunday, 16, 5)),
        Ok(false),
        "the holiday-removed Sunday answers closed instead of refusing"
    );
    assert_eq!(
        fx.is_accepting_orders(at(after_the_bound, 16, 5)),
        Ok(true),
        "after the knowledge-bound row the quarter-hour is served"
    );
}

#[test]
fn the_post_close_label_gap_applies_only_to_the_dates_that_carry_the_queue() {
    // #152 withholds the trade-date label on exactly the dates whose profile
    // serves the post-close queue. Weekends, closed dates and the eras before
    // each family's sourced queue onset answer completely; the queue days
    // refuse, because their trade date is the crate's convention rather than
    // the operator's printing.
    let grains = key_coverage(MarketHoursKey::GlobexGrains);
    let livestock = key_coverage(MarketHoursKey::GlobexLivestock);
    let any_close = exchange_hours::PhaseGapShape::OrderEntryWindow {
        open_ssm: None,
        close_ssm: 16 * 3600,
    };
    check_declared_gaps(
        MarketHoursKey::GlobexGrains,
        &[
            (
                CoverageGapReason::PostCloseQueueTradeDateLabel,
                "#152",
                None,
                None,
                any_close,
            ),
            (
                CoverageGapReason::NormalWeekPhaseWithheld,
                "#116",
                Some(date(2012, 5, 20)),
                Some(date(2013, 4, 7)),
                exchange_hours::PhaseGapShape::EveryDay,
            ),
        ],
    );
    check_declared_gaps(
        MarketHoursKey::GlobexLivestock,
        &[(
            CoverageGapReason::PostCloseQueueTradeDateLabel,
            "#152",
            None,
            None,
            any_close,
        )],
    );

    // The task's own truth table: a Sunday answers, a queue day refuses.
    assert_eq!(
        grains.coverage_on(date(2021, 3, 14)),
        DateCoverage::Covered,
        "a Sunday carries no post-close queue, so its answers are complete"
    );
    assert_eq!(
        grains.coverage_on(date(2021, 3, 16)),
        DateCoverage::OutsideCoveredRange,
        "a Tuesday in a queue era carries the 14:30-16:00 CT PCP the label gap is about"
    );
    assert_eq!(
        livestock.coverage_on(date(2021, 3, 14)),
        DateCoverage::Covered,
        "{livestock:?}: the same for livestock's grid"
    );
    assert_eq!(
        livestock.coverage_on(date(2021, 3, 16)),
        DateCoverage::OutsideCoveredRange
    );
    // Livestock's Post-Close begins on the sourced 2016-06-06 notice, so the
    // years before it answer completely — the whole-domain declaration the #172
    // issue replaced used to refuse them too.
    assert_eq!(
        livestock.coverage_on(date(2015, 6, 10)),
        DateCoverage::Covered,
        "livestock serves no post-close queue before its 2016-06-06 onset"
    );
    assert_eq!(
        livestock.coverage_on(date(2016, 6, 8)),
        DateCoverage::OutsideCoveredRange,
        "the first week of the sourced Post-Close era carries the queue"
    );
    // Grains' PCP closes at 16:00 CT in every era that serves one, so the
    // 2010-2012 PCP era (13:15:30-16:00 then) is a queue era too.
    assert_eq!(
        grains.coverage_on(date(2011, 6, 14)),
        DateCoverage::OutsideCoveredRange,
        "the 2010-2012 PCP era is a queue era: the label gap applies there as well"
    );
    // The omitted 2012-05-20..2013-04-06 regime refuses as the phase-level gap
    // its own declaration states — the #152 shape serves no queue there, so the
    // label gap cannot apply, and without the second declaration these dates
    // would read Covered while their queue rows are omitted.
    assert_eq!(
        grains.coverage_on(date(2012, 6, 1)),
        DateCoverage::OutsideCoveredRange,
        "the omitted-queue regime refuses as the phase-level gap its declaration states"
    );
    assert_eq!(
        grains.coverage_on(date(2012, 5, 13)),
        DateCoverage::Covered,
        "the regime's dated neighbours answer: 2012-05-13 is before it and served"
    );
    assert_eq!(
        grains.coverage_on(date(2013, 6, 10)),
        DateCoverage::OutsideCoveredRange,
        "the 2013-04-07 notice's queue era carries the PCP again"
    );

    // The #152 reason refuses no query: the queue and both of its verdicts are
    // served; only the label is the crate's convention.
    let grains_cal = calendar_for_market_hours_key(MarketHoursKey::GlobexGrains);
    let queue_instant = US::Central
        .with_ymd_and_hms(2021, 3, 16, 15, 0, 0)
        .single()
        .expect("a single 15:00 CT instant")
        .with_timezone(&Utc);
    assert_eq!(
        grains_cal.is_accepting_orders(queue_instant),
        Ok(true),
        "the post-close queue answers: it is served, and its declaration withholds no phase"
    );

    // Inside the omitted regime the queue question refuses instead: the #152
    // shape resolves to no occurrence there, so the metadata's shadowing rule
    // leaves #116 operative, and the order-entry probe — the query whose answer
    // *is* the omitted arrangement — states that rather than reading as a
    // sourced absence (2012-06-01 14:30-15:30 CT is closed and order-entryless
    // in the regime grid, so the probe reaches the phase gate).
    let regime_instant = US::Central
        .with_ymd_and_hms(2012, 6, 1, 15, 0, 0)
        .single()
        .expect("a single 15:00 CT instant")
        .with_timezone(&Utc);
    assert_eq!(
        grains_cal.is_accepting_orders(regime_instant),
        Err(CalendarQueryError::OutsideCoveredRange {
            source: CalendarSource::MarketHoursKey(MarketHoursKey::GlobexGrains),
            date: date(2012, 6, 1),
        }),
        "an order-entry probe inside the omitted regime refuses: the queue question is live \
         and unsourced there"
    );
}

#[test]
fn the_five_day_pre_open_gap_names_exactly_the_five_day_era() {
    // #123 is a property of the five-day 17:00-16:00 CT grid, whose own first
    // and last days are dated at T1: the 2017-12-17 launch row opens the era and
    // the 2026-05-29 bridge row closes it, and the declaration names those days
    // rather than refusing the sourced launch closures before the grid or the
    // 24/7 era that serves the Pre-Open from its bridge day.
    let crypto = key_coverage(MarketHoursKey::GlobexCryptocurrency);
    let launch = date(2017, 12, 17);
    let bridge = date(2026, 5, 29);
    check_declared_gaps(
        MarketHoursKey::GlobexCryptocurrency,
        &[(
            CoverageGapReason::NormalWeekPhaseWithheld,
            "#123",
            Some(launch),
            Some(bridge),
            exchange_hours::PhaseGapShape::EveryDay,
        )],
    );
    assert_eq!(
        crypto.coverage_on(date(2020, 6, 1)),
        DateCoverage::OutsideCoveredRange,
        "a five-day-era date withholds the Pre-Open whose onset is undated"
    );
    assert_eq!(
        crypto.coverage_on(date(2026, 5, 28)),
        DateCoverage::OutsideCoveredRange,
        "the five-day era's own last day is the bridge row's predecessor, and it \
         still withholds the Pre-Open"
    );
    assert_eq!(
        crypto.coverage_on(bridge),
        DateCoverage::Covered,
        "the bridge day's own profile serves the published Pre-Open for the first \
         time, so the declaration's era ends the day before it"
    );
    assert_eq!(
        crypto.coverage_on(date(2026, 6, 1)),
        DateCoverage::Covered,
        "the 24/7 era serves the published Pre-Open and answers"
    );
    // Before the grid the family's era is a sourced launch closure. The
    // declaration's `since` names the grid's first day, so those dates are the
    // date-level walk's to answer — and in the 2010-2018 stretch the shipped
    // holiday windows do not reach, which is the honest reason they refuse.
    let pre_launch = crypto.coverage_on(date(2015, 1, 1));
    assert_ne!(
        gap_reason_on(crypto, date(2015, 1, 1)),
        Some(CoverageGapReason::NormalWeekPhaseWithheld),
        "the pre-launch era is not the Pre-Open gap's to refuse"
    );
    assert_eq!(pre_launch, DateCoverage::OutsideCoveredRange);
    // The declaration record is the era where the identity answers: the shipped
    // windows open 2019-01-01, so that is where its record starts, and the
    // era's own withheld dates split the records the walk reports.
    let record = crypto
        .gaps()
        .find(|gap| gap.closing_condition() == Some("#123"))
        .expect("the five-day era's declaration has a record inside the audited windows");
    assert_eq!(record.range().first(), date(2019, 1, 1));
    let last_record = crypto
        .gaps()
        .filter(|gap| gap.closing_condition() == Some("#123"))
        .map(exchange_hours::CoverageGap::range)
        .last()
        .expect("the era's records");
    assert_eq!(last_record.last(), date(2026, 5, 28));
}

#[test]
fn a_declared_phase_gap_is_era_aware_and_reported_for_the_span_it_answers() {
    // The scopes the quarter-hour probe cleared declare nothing, so a
    // declaration cannot leak onto a profile whose grid simply has no session in
    // the disputed window. The behavioural half of that claim is fenced in
    // `tests/schedule_documentation/coverage_inventory.rs`.
    assert!(
        key_coverage(MarketHoursKey::GlobexNikkei225Dollar)
            .phase_gaps()
            .is_empty(),
        "globex_nikkei_225_dollar runs no order-entry phase, so the post-close \
         queue label cannot apply to it"
    );
    for exchange in [
        Exchange::Cbot,
        Exchange::Cfe,
        Exchange::CoinbaseDerivatives,
        Exchange::Iceus,
    ] {
        assert!(
            exchange_coverage(exchange).phase_gaps().is_empty(),
            "{exchange:?}"
        );
    }

    // `eurex` is the one *venue* that declares a gap, and it is the one shape
    // that withholds no phase: the operator's German equity/equity-index
    // closures are undated, so the site is incomplete on every date the
    // declaration covers while its ordinary week and order-entry queues are
    // still served. The reason travels with the declaration, because the query
    // gate answers through this reason and refuses through the other two.
    let eurex = exchange_coverage(Exchange::Eurex);
    let declared: Vec<(CoverageGapReason, &str)> = eurex
        .phase_gaps()
        .iter()
        .map(|gap| (gap.reason(), gap.closing_condition()))
        .collect();
    assert_eq!(
        declared,
        vec![(CoverageGapReason::UnpublishedClosureDates, "#157")]
    );
    assert_eq!(
        eurex
            .phase_gaps()
            .iter()
            .find_map(|gap| gap.applies_until()),
        Some(date(2027, 1, 1)),
        "the declaration spans the editions that carry the note and stops where they do: \
         2027-01-01 is the first day the note does not establish, not a day the operator \
         resolved it"
    );
    assert!(
        !eurex.is_complete_on(date(2025, 6, 10)),
        "eurex on 2025-06-10"
    );
    assert_eq!(
        eurex.coverage_on(date(2025, 6, 10)),
        DateCoverage::OutsideCoveredRange
    );
    assert!(
        !key_coverage(MarketHoursKey::GlobexCryptocurrency)
            .phase_gaps()
            .iter()
            .any(|gap| gap.closing_condition() == "#79"),
        "globex_cryptocurrency is closed at both 16:05 and 16:20 CT, so it must \
         not declare the quarter-hour the probe cleared it of"
    );
}

#[test]
fn a_date_shaped_gap_carries_no_closing_condition() {
    // A date-shaped gap is closed by data rather than by an issue, so this
    // vocabulary does not invent a number for it.
    let complete = key_coverage(MarketHoursKey::GlobexNikkei225Dollar);
    let trailing = complete
        .gaps()
        .next()
        .expect("globex_nikkei_225_dollar has a trailing gap");
    assert_eq!(trailing.closing_condition(), None);
    assert_eq!(trailing.phase_gap(), None);
    assert!(complete.phase_gaps().is_empty());
}

#[test]
fn detaching_the_holiday_table_neither_hides_nor_manufactures_a_phase_gap() {
    // A declared gap is a fact about the identity, not about which layer this
    // calendar consults.
    let attached = key_coverage(MarketHoursKey::GlobexFx);
    let detached = calendar_for_market_hours_key(MarketHoursKey::GlobexFx)
        .without_holidays()
        .coverage();
    assert_eq!(detached.phase_gaps(), attached.phase_gaps());
    assert_eq!(
        detached
            .gaps()
            .filter(|gap| gap.phase_gap().is_some())
            .count(),
        attached
            .gaps()
            .filter(|gap| gap.phase_gap().is_some())
            .count(),
        "the detached view reports the same declaration records"
    );
}

/// The inspection interval the #172 regression probe walks: the 1,095 days
/// 2025-01-01..2027-12-31 over which the whole-domain declarations once
/// answered zero complete days for three served identities.
const REGRESSION_FIRST: NaiveDate = match NaiveDate::from_ymd_opt(2025, 1, 1) {
    Some(date) => date,
    None => NaiveDate::MAX,
};
const REGRESSION_LAST: NaiveDate = match NaiveDate::from_ymd_opt(2027, 12, 31) {
    Some(date) => date,
    None => NaiveDate::MAX,
};

/// The 15:00 CT instant of `day`, the middle of the post-close queue window
/// the #152 label gap is about.
fn post_close_instant(day: NaiveDate) -> DateTime<Utc> {
    use chrono_tz::US;
    US::Central
        .with_ymd_and_hms(day.year(), day.month(), day.day(), 15, 0, 0)
        .single()
        .expect("15:00 CT is never ambiguous")
        .with_timezone(&Utc)
}

/// Walks the interval and returns the number of `Covered` days.
fn complete_days(coverage: CalendarCoverage) -> usize {
    let mut days = 0;
    let mut day = REGRESSION_FIRST;
    while day <= REGRESSION_LAST {
        days += usize::from(coverage.coverage_on(day) == DateCoverage::Covered);
        day = day
            .succ_opt()
            .expect("the walk stays inside the year range");
    }
    days
}

#[test]
fn a_date_scoped_declaration_zeroes_no_identity_over_2025_2027() {
    // The #172 regression: `PostCloseQueueTradeDateLabel` declared whole-domain
    // refused every date of 2025-01-01..2027-12-31 for both queue families, and
    // `UnpublishedClosureDates` did the same for eurex before #180 bounded it.
    // The date-scoped engine answers the dates the evidence does not withhold,
    // so the walk counts here are nonzero and every refused date is one whose
    // own calendar carries the disputed arrangement.
    let grains = key_coverage(MarketHoursKey::GlobexGrains);
    let livestock = key_coverage(MarketHoursKey::GlobexLivestock);
    assert_eq!(
        complete_days(grains),
        375,
        "globex_grains answers 375 of the 1,095 days; the refused rest carry the post-close queue"
    );
    assert_eq!(
        complete_days(livestock),
        341,
        "globex_livestock answers 341 of the 1,095 days; the refused rest carry the post-close queue"
    );

    // Livestock's queue day is exactly a refused day: the 15:00 CT instant
    // accepts orders iff the profile serves the 14:30-16:00 CT post-close
    // queue the label gap withholds, and on every other day the date answers
    // completely.
    let livestock_cal = calendar_for_market_hours_key(MarketHoursKey::GlobexLivestock);
    let mut day = REGRESSION_FIRST;
    while day <= REGRESSION_LAST {
        let refused = livestock.coverage_on(day) == DateCoverage::OutsideCoveredRange;
        let accepts = livestock_cal.is_accepting_orders(post_close_instant(day)) == Ok(true);
        assert_eq!(refused, accepts, "globex_livestock on {day}");
        day = day
            .succ_opt()
            .expect("the walk stays inside the year range");
    }

    // Grains refuses on the same condition, but fourteen dates answer
    // completely while a queue accepts orders at 15:00 CT: the closure eves
    // whose complete replacement blocks state the adjusted day outright
    // (coverage-2025 §2), so the 16:00-ending occurrence the shape resolves is
    // absent and the label gap cannot apply.
    let grains_cal = calendar_for_market_hours_key(MarketHoursKey::GlobexGrains);
    let mut block_eves = 0;
    day = REGRESSION_FIRST;
    while day <= REGRESSION_LAST {
        let refused = grains.coverage_on(day) == DateCoverage::OutsideCoveredRange;
        let accepts = grains_cal.is_accepting_orders(post_close_instant(day)) == Ok(true);
        if refused {
            assert!(
                accepts,
                "a refused date must be one the post-close queue runs on: {day}"
            );
        } else if accepts {
            block_eves += 1;
        }
        day = day
            .succ_opt()
            .expect("the walk stays inside the year range");
    }
    assert_eq!(
        block_eves, 14,
        "the complete replacement-blocks closure eves are the only covered dates that \
         accept orders at 15:00 CT"
    );

    // Eurex stays at zero complete days — the operator's `tba` note is
    // evidence in the 2025 and 2026 editions and in no later one, so every
    // audited date is withheld and 2027 lies outside the audited windows — but
    // the record it streams spans exactly those editions, not the whole floor
    // the whole-domain rendering used to shadow (2010-01-01..2026-12-31).
    let eurex = exchange_coverage(Exchange::Eurex);
    assert_eq!(complete_days(eurex), 0);
    let unpublished: Vec<(DateRange, Option<&str>)> = eurex
        .gaps()
        .filter(|gap| gap.reason() == CoverageGapReason::UnpublishedClosureDates)
        .map(|gap| (gap.range(), gap.closing_condition()))
        .collect();
    assert_eq!(
        unpublished,
        vec![
            (
                DateRange::new(date(2025, 1, 1), date(2026, 12, 30)).expect("ascending"),
                Some("#157")
            ),
            (
                DateRange::new(date(2026, 12, 31), date(2026, 12, 31)).expect("ascending"),
                Some("#157")
            ),
        ],
        "the #157 record covers exactly the editions that carry the note, each with its \
         closing condition"
    );
}
