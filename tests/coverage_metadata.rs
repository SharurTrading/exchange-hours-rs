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
    let coverage = exchange_coverage(Exchange::Comex);
    assert_eq!(
        coverage.identity(),
        CalendarSource::Exchange(Exchange::Comex)
    );
    assert_eq!(coverage.normal_week_sourced_from(), Some(date(2012, 5, 11)));
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
        windows.first() == date(2010, 1, 1) && windows.last() == date(2027, 12, 31)
    }));
}

#[test]
fn scopes_without_2025_holiday_coverage_report_outside_range() {
    for exchange in [Exchange::Cfe, Exchange::Eurex, Exchange::Iceus] {
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
            .expect("each of the three audited a 2026 window");
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
    let fixtures = [
        (Exchange::Cme, date(2025, 1, 2), date(2025, 1, 3)),
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
    let attached = calendar_for_exchange(Exchange::Cme).coverage();
    let detached = calendar_for_exchange(Exchange::Cme)
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
        calendar_for_exchange(Exchange::Cme)
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
    assert!(gaps.len() < 128, "{identity:?} reports a bounded gap count");

    // The two walks partition [floor, NaiveDate::MAX]: no hole, no overlap.
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
    // A spot walk over one complete scope, so the span report and the per-date
    // accessor are compared date by date rather than only at the edges.
    let coverage = exchange_coverage(Exchange::Comex);
    for day in days_from_floor(date(2027, 12, 31)) {
        assert!(coverage.is_complete_on(day), "{day}");
        assert_eq!(gap_reason_on(coverage, day), None, "{day}");
    }
}

#[test]
fn a_declared_phase_gap_denies_completeness_across_the_whole_domain() {
    // The three scopes whose gap is a property of the normal week or the
    // calendar rather than of a span of dates. A date walk over an identity's
    // tables cannot find them, which is exactly why they are declared beside the
    // horizon: before this declaration `is_complete_on(2025-06-10)` answered
    // `true` for all three while `docs/schedules/coverage-2025.md` called each
    // one incomplete.
    let fixtures = [
        (
            MarketHoursKey::GlobexEquityIndex,
            CoverageGapReason::NormalWeekPhaseWithheld,
            "#79",
        ),
        (
            MarketHoursKey::GlobexFx,
            CoverageGapReason::SpecialSessionUnrepresentable,
            "#93",
        ),
        (
            MarketHoursKey::GlobexCryptocurrency,
            CoverageGapReason::SpecialSessionUnrepresentable,
            "#93",
        ),
    ];
    let sample = date(2025, 6, 10);
    for (key, reason, closing) in fixtures {
        let coverage = key_coverage(key);
        let declared = coverage
            .phase_gap()
            .unwrap_or_else(|| panic!("{key:?} declares a phase-level gap"));
        assert_eq!(declared.reason(), reason, "{key:?}");
        assert_eq!(declared.closing_condition(), closing, "{key:?}");

        // Nowhere in the supported domain, and the horizon the ledger declares
        // is untouched: a phase gap is additional information, not a re-dating.
        assert!(!coverage.is_complete_on(sample), "{key:?} on {sample}");
        assert!(
            !coverage.is_complete_on(SUPPORT_FLOOR),
            "{key:?} at the floor"
        );
        assert_eq!(coverage.complete_ranges().count(), 0, "{key:?}");
        assert_eq!(
            coverage.coverage_on(sample),
            DateCoverage::OutsideCoveredRange,
            "{key:?}"
        );

        // Reportable, with the closing condition on the reported gap.
        let gaps: Vec<CoverageGap> = coverage.gaps().collect();
        assert_eq!(gaps.len(), 1, "{key:?}");
        assert_eq!(gaps[0].range(), unbounded(SUPPORT_FLOOR), "{key:?}");
        assert_eq!(gaps[0].reason(), reason, "{key:?}");
        assert_eq!(gaps[0].closing_condition(), Some(closing), "{key:?}");
        assert_eq!(
            gaps[0]
                .phase_gap()
                .map(exchange_hours::PhaseGap::closing_condition),
            Some(closing),
            "{key:?}"
        );
    }

    // A date-shaped gap carries no closing condition: this vocabulary does not
    // invent an issue number for a gap the crate's own data closes.
    let comex = exchange_coverage(Exchange::Comex);
    let trailing = comex.gaps().next().expect("Comex has a trailing gap");
    assert_eq!(trailing.closing_condition(), None);
    assert_eq!(trailing.phase_gap(), None);
    assert_eq!(comex.phase_gap(), None);

    // Detaching the holiday table does not manufacture or hide a phase gap: it
    // is a fact about the identity, not about which layer this calendar consults.
    let detached = calendar_for_market_hours_key(MarketHoursKey::GlobexFx)
        .without_holidays()
        .coverage();
    assert_eq!(
        detached.phase_gap(),
        key_coverage(MarketHoursKey::GlobexFx).phase_gap()
    );
    assert_eq!(
        detached.gaps().next().map(CoverageGap::reason),
        Some(CoverageGapReason::SpecialSessionUnrepresentable)
    );
}
