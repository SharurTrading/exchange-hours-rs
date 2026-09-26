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

use chrono::{NaiveDate, TimeZone as _, Utc};
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
fn the_support_floor_is_local_first_of_january_2025() {
    assert_eq!(SUPPORT_FLOOR, date(2025, 1, 1));
}

#[test]
fn a_complete_scope_reports_one_complete_span_and_a_trailing_gap() {
    // `globex_nikkei_225_dollar` is the one scope that still reaches 2027-12-31
    // with nothing withheld: it ships no order-entry phase at all, so the
    // post-close queue label `globex_grains` and `globex_livestock` declare
    // (#152) cannot apply to it, and `comex`, `nymex`, `globex_energy` and
    // `globex_interest_rates` are complete no longer, because each withholds the
    // Sunday 16:00-16:15 CT quarter-hour its ledger row records (#79).
    let coverage = key_coverage(MarketHoursKey::GlobexNikkei225Dollar);
    assert_eq!(
        coverage.identity(),
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexNikkei225Dollar)
    );
    assert_eq!(coverage.normal_week_sourced_from(), None);
    assert_eq!(coverage.sourced_normal_week(), unbounded(SUPPORT_FLOOR));

    let complete: Vec<DateRange> = coverage.complete_ranges().collect();
    assert_eq!(
        complete,
        vec![DateRange::new(date(2025, 1, 1), date(2027, 12, 31)).expect("ascending")]
    );
    let gaps: Vec<CoverageGap> = coverage.gaps().collect();
    assert_eq!(gaps.len(), 1);
    assert_eq!(gaps[0].range(), unbounded(date(2028, 1, 1)));
    assert_eq!(gaps[0].reason(), CoverageGapReason::NoHolidayCoverage);

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
    // `eurex` used to be the third scope here. Its 2025 rows shipped, so its
    // window now opens at the 2025 floor: 2025-06-02 is `Covered` for it, and
    // the reason its *verdict* is still not completeness is the operator's
    // undated German closure scope, declared as `UnpublishedClosureDates` and
    // asserted in `a_declared_phase_gap_is_era_aware_and_reported_for_the_span_it_answers`.
    for exchange in [Exchange::Cfe, Exchange::Iceus] {
        let coverage = exchange_coverage(exchange);
        let day = date(2025, 6, 2);
        assert_eq!(
            coverage.coverage_on(day),
            DateCoverage::OutsideCoveredRange,
            "{exchange:?} shipped no 2025 holiday coverage"
        );
        assert_eq!(
            gap_reason_on(coverage, day),
            Some(CoverageGapReason::NoHolidayCoverage),
            "{exchange:?} answers no holiday question in 2025"
        );
        let first_complete = coverage
            .complete_ranges()
            .next()
            .expect("each of the two audited a 2026 window");
        assert_eq!(first_complete.first(), date(2026, 1, 1), "{exchange:?}");
        assert!(
            coverage
                .holiday_contract()
                .coverage()
                .is_some_and(|windows| windows.first() >= date(2026, 1, 1)),
            "{exchange:?}"
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
    let gaps: Vec<CoverageGap> = detached.gaps().collect();
    assert_eq!(gaps.len(), 1);
    assert_eq!(gaps[0].range(), unbounded(SUPPORT_FLOOR));
    assert_eq!(gaps[0].reason(), CoverageGapReason::NormalWeekOnly);
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
    for exchange in [Exchange::Nasdaq, Exchange::MemxEq, Exchange::Tsx] {
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
    let before = date(2024, 12, 31);
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
        .with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
        .single()
        .expect("a valid instant");
    let tokyo_day = instant.with_timezone(&Asia::Tokyo).date_naive();
    let chicago_day = instant.with_timezone(&US::Central).date_naive();
    assert_eq!(tokyo_day, date(2025, 1, 1));
    assert_eq!(chicago_day, date(2024, 12, 31));

    // One UTC instant, two verdicts: Tokyo's 2025-01-01 is inside the supported
    // domain while Chicago is still on 2024-12-31. A single UTC midnight for
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
            date: date(2024, 12, 31),
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
    assert_eq!(errors[0].date(), date(2024, 12, 31));
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
    assert!(rendered.contains("2024-12-31"), "{rendered}");
    assert!(rendered.contains("2025-01-01"), "{rendered}");
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
/// it answers nothing completely exactly while a whole-domain declaration
/// applies, and every record a declaration is the answer for carries it.
fn check_declarations(
    coverage: CalendarCoverage,
    complete: &[DateRange],
    gaps: &[CoverageGap],
    identity: CalendarSource,
) {
    let whole_domain = coverage
        .phase_gaps()
        .iter()
        .any(|gap| gap.applies_until().is_none());
    if coverage.phase_gaps().is_empty() {
        assert!(
            gaps.iter().all(|gap| gap.phase_gap().is_none()),
            "{identity:?} declares nothing, so no gap may carry a declaration"
        );
        return;
    }
    assert_eq!(
        complete.is_empty(),
        whole_domain,
        "{identity:?}: no complete span exactly while a whole-domain declaration applies"
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
        complete.len() < 128,
        "{identity:?} reports a bounded span count"
    );
    assert!(
        gaps.len() <= exchange_hours::CoverageGaps::capacity(),
        "{identity:?} reports {} gaps, past the capacity the walk precomputes \
         ({}) — past it a declaration's record would be dropped while the walk \
         still skipped the dates it claimed, leaving a hole",
        gaps.len(),
        exchange_hours::CoverageGaps::capacity()
    );
    assert!(gaps.len() < 128, "{identity:?} reports a bounded gap count");

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
    // A spot walk over the one complete scope, so the span report and the
    // per-date accessor are compared date by date rather than only at the edges.
    let coverage = key_coverage(MarketHoursKey::GlobexNikkei225Dollar);
    for day in days_from_floor(date(2027, 12, 31)) {
        assert!(coverage.is_complete_on(day), "{day}");
        assert_eq!(gap_reason_on(coverage, day), None, "{day}");
    }
}

/// Asserts one fixture's declared phase-level gaps, the era each covers, and the
/// records they produce.
///
/// `complete_after_the_bound` says whether the identity answers the era that
/// begins on `bound` completely: true when its only bounded declaration is the
/// quarter-hour, false when a whole-domain gap still applies there.
fn check_declared_gap_era(
    key: MarketHoursKey,
    expected: &[(CoverageGapReason, &str)],
    complete_after_the_bound: bool,
    sample: NaiveDate,
    bound: NaiveDate,
    after: NaiveDate,
) {
    let coverage = key_coverage(key);
    let declared: Vec<(CoverageGapReason, &str)> = coverage
        .phase_gaps()
        .iter()
        .map(|gap| (gap.reason(), gap.closing_condition()))
        .collect();
    assert_eq!(declared, expected, "{key:?}");

    // Inside the dated era the declaration applies, the horizon the ledger
    // declares is untouched, and the verdict is the phase-level one: a phase gap
    // is additional information, not a re-dating.
    assert!(!coverage.is_complete_on(sample), "{key:?} on {sample}");
    assert_eq!(
        coverage.coverage_on(sample),
        DateCoverage::OutsideCoveredRange
    );

    // Only the quarter-hour declaration is bounded, and only where it is the last
    // one: `globex_fx` and `globex_cryptocurrency` also carry a whole-domain gap.
    // The quarter-hour declaration is the only bounded one in these fixtures, and
    // it is bounded for every scope that declares it — including `globex_fx`,
    // whose second, whole-domain declaration is what keeps the era it opens from
    // being complete.
    for declaration in coverage.phase_gaps() {
        assert_eq!(
            declaration.applies_until(),
            (declaration.closing_condition() == "#79").then_some(bound),
            "{key:?}: only the quarter-hour declaration is bounded"
        );
    }
    assert_eq!(
        coverage.is_complete_on(after),
        complete_after_the_bound,
        "{key:?}: a scope whose only bounded declaration is the quarter-hour answers the era it \
         opens"
    );
    assert_eq!(
        coverage.complete_ranges().count() == 0,
        !complete_after_the_bound,
        "{key:?}"
    );

    // Reportable: a record for each declaration the accessor names on some date,
    // each with its own reason and closing condition. A whole-domain declaration
    // that an earlier one already covers on every date is reported by
    // `phase_gaps` alone, because no date has it as its answer.
    let gaps: Vec<CoverageGap> = coverage.gaps().collect();
    for (reason, closing) in expected {
        let Some(gap) = gaps
            .iter()
            .find(|gap| gap.closing_condition() == Some(*closing))
        else {
            continue;
        };
        assert_eq!(gap.reason(), *reason, "{key:?}");
        assert_eq!(
            gap.phase_gap()
                .map(exchange_hours::PhaseGap::closing_condition),
            Some(*closing),
            "{key:?}"
        );
        assert!(
            coverage
                .phase_gaps()
                .iter()
                .find(|candidate| candidate.closing_condition() == *closing)
                .is_some_and(|declaration| declaration.applies_on(gap.range().first())),
            "{key:?}: the record starts inside its own declaration's dates"
        );
    }
    assert!(
        gaps.iter().filter(|gap| gap.phase_gap().is_some()).count() <= expected.len(),
        "{key:?} reports no more declaration records than it declares"
    );
}

#[test]
fn a_declared_phase_gap_is_era_aware_and_reported_for_the_span_it_answers() {
    // The scopes whose gap is a property of the normal week or the calendar
    // rather than of a span of dates. A date walk over an identity's tables
    // cannot find them, which is exactly why they are declared beside the
    // horizon: before those declarations `is_complete_on(2025-06-10)` answered
    // `true` for several of them while `docs/schedules/coverage-2025.md` called
    // each one incomplete.
    //
    // One of them carries a **second** gap, so the declaration is a list:
    // `globex_cryptocurrency` publishes special sessions the scalar layer
    // cannot state *and* withholds its five-day era's undated Pre-Open onset.
    // CME's `globex_fx` used to be the other; its merged trade dates now ship
    // as rows, so only the Sunday quarter-hour remains withheld.
    let sample = date(2025, 6, 10);
    let bound = date(2026, 8, 22);
    let after = date(2026, 8, 23);
    let fixtures = [
        (
            MarketHoursKey::GlobexEquityIndex,
            vec![(CoverageGapReason::NormalWeekPhaseWithheld, "#79")],
            true,
        ),
        (
            MarketHoursKey::GlobexFx,
            vec![(CoverageGapReason::NormalWeekPhaseWithheld, "#79")],
            true,
        ),
        (
            MarketHoursKey::GlobexCryptocurrency,
            vec![
                (CoverageGapReason::SpecialSessionUnrepresentable, "#93"),
                (CoverageGapReason::NormalWeekPhaseWithheld, "#123"),
            ],
            false,
        ),
        // The two scopes whose whole-domain declaration serves its phase: the
        // post-close queue is answered, and only its trade-date label is the
        // crate's own convention rather than the operator's printing (#152).
        (
            MarketHoursKey::GlobexGrains,
            vec![(CoverageGapReason::PostCloseQueueTradeDateLabel, "#152")],
            false,
        ),
        (
            MarketHoursKey::GlobexLivestock,
            vec![(CoverageGapReason::PostCloseQueueTradeDateLabel, "#152")],
            false,
        ),
    ];
    for (key, expected, complete_after_the_bound) in fixtures {
        check_declared_gap_era(
            key,
            &expected,
            complete_after_the_bound,
            sample,
            bound,
            after,
        );
    }

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
    assert!(
        eurex
            .phase_gaps()
            .iter()
            .all(|gap| gap.applies_until().is_none()),
        "the German-scope note is unpublished in the 2026 edition too, so the \
         declaration carries no era bound"
    );
    assert!(!eurex.is_complete_on(sample), "eurex on {sample}");
    assert_eq!(eurex.coverage_on(sample), DateCoverage::OutsideCoveredRange);
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
