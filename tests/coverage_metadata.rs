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
    SUPPORT_FLOOR, SessionState, calendar_for_exchange, calendar_for_market_hours_key,
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
    // at all, so the post-close queue convention the charter records for
    // `globex_grains` and `globex_livestock` (#152, retired as a declaration
    // 2026-10-03) cannot apply to it. Since the #225
    // remainder modelled the old grid (2026-09-30 UTC), the normal week is
    // sourced from the floor itself and the first audited window opens there.
    let coverage = key_coverage(MarketHoursKey::GlobexNikkei225Dollar);
    assert_eq!(
        coverage.identity(),
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexNikkei225Dollar)
    );
    assert_eq!(
        coverage.normal_week_sourced_from(),
        Some(date(2010, 1, 1)),
        "the old grid's own row keys at the support floor"
    );
    assert_eq!(coverage.sourced_normal_week(), unbounded(SUPPORT_FLOOR));

    // At the 2010 floor the scope's earlier history is honestly partial: the
    // 2010-01-01..2010-04-10 interval is unaudited (the 2011-2024 windows
    // opened with the 2026-09-29 wave; on 2026-09-30 UTC the first window
    // moved back to the dated 2010-04-11 grid start, and the 2010 rows
    // entered with it), and the 2016-2024 windows withhold dates, so the
    // complete spans begin inside that window and split around the
    // withheld dates instead of forming one span from the floor.
    let complete: Vec<DateRange> = coverage.complete_ranges().collect();
    // The first audited window's own first day is not complete under the
    // resolution-edge rule (#151): answering 2010-04-11 completely consults
    // 2010-04-10, the unmodelled edge outside the window, so the first
    // complete span begins one day inside it. The same rule ends each span
    // at the last date whose forward reach stays inside the window:
    // 2012-12-28's Saturday-to-Monday tail reaches 2013-01-01, outside the
    // window, and flips.
    assert_eq!(
        complete.first(),
        Some(&DateRange::new(date(2010, 1, 4), date(2012, 12, 28)).expect("ascending")),
        "the first complete span opens at the floor era's first session day \
         (2010-01-04, the sourced CST grid's first Monday): January 1-3's \
         resolution reach crosses the pre-floor edge into 2009-12-31, where the \
         crate models nothing (#151)"
    );
    assert_eq!(
        complete.last(),
        Some(&DateRange::new(date(2025, 1, 5), date(2027, 12, 30)).expect("ascending")),
        "the last complete span is the complete 2025+ era inside its edges"
    );
    let gaps: Vec<CoverageGap> = coverage.gaps().collect();
    assert_eq!(
        gaps.first().map(|gap| (gap.range().first(), gap.reason())),
        Some((date(2010, 1, 1), CoverageGapReason::ResolutionEdge)),
        "the first gap is the pre-floor resolution edge the floor era's January \
         1-3 dates reach across (#151); the sourced old grid answers from January 4"
    );
    assert!(
        gaps.iter()
            .any(|gap| gap.range().contains(date(2019, 6, 19))
                && gap.reason() == CoverageGapReason::WithheldDate),
        "the Juneteenth marker's WithheldDate gap survives between the sourced eras"
    );
    assert_eq!(
        gaps.last().map(|gap| (gap.range().first(), gap.reason())),
        Some((date(2028, 1, 1), CoverageGapReason::NoHolidayCoverage)),
        "the trailing gap past the data is unchanged"
    );

    assert!(coverage.is_complete_on(date(2025, 6, 2)));
    // The window's own edges are not complete: 2025-01-01 reaches back to
    // 2024-12-31 and 2027-12-31 reaches forward to 2028-01-01, both outside
    // the audited window (#151).
    assert_eq!(
        coverage.coverage_on(date(2027, 12, 31)),
        DateCoverage::OutsideCoveredRange
    );
    assert_eq!(
        gaps.last().map(|gap| gap.range().first()),
        Some(date(2028, 1, 1))
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
        // The window's own first day is not complete (#151): answering
        // 2025-01-01 consults 2024-12-31, outside the window. The first
        // complete span begins the day after it.
        assert_eq!(
            coverage.complete_ranges().next().map(DateRange::first),
            Some(date(2025, 1, 2)),
            "the audited window opens at the 2025 floor, and the first complete              date is the window's second day"
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
        // 2017-04-10, the window's own first day, reaches back to 2017-04-09
        // for its wrapped Sunday-evening leg and so is not complete (#151);
        // the first complete span begins the day after it.
        assert_eq!(
            coverage.complete_ranges().next().map(DateRange::first),
            Some(date(2017, 4, 11))
        );
        assert!(
            coverage
                .holiday_contract()
                .coverage()
                .is_some_and(|windows| windows.first() == date(2017, 4, 10)
                    && windows.last() == date(2026, 12, 31)),
            "cfe must audit from the earliest surviving operator artifact through the \
             published 2026 schedule"
        );
    }
    // The one date the venue withholds inside that window: 2017-07-03 is
    // `Unsourced` because the rules page states the eve close only as a
    // default and no controlling circular survives, and it is the reason the
    // first complete range breaks where it does.
    let coverage = exchange_coverage(Exchange::Cfe);
    assert_eq!(
        coverage.coverage_on(date(2017, 7, 3)),
        DateCoverage::UnresolvedGap
    );
    assert_eq!(
        gap_reason_on(coverage, date(2017, 7, 3)),
        Some(CoverageGapReason::WithheldDate)
    );
}

#[test]
fn withheld_dates_are_unresolved_gaps_inside_an_audited_window() {
    // `cme` is deliberately absent: it now declares the Sunday quarter-hour
    // phase-level gap (#79), which is checked before the date-level facts, so
    // inside the dated era its per-date verdict for a withheld date is
    // `OutsideCoveredRange` rather than `UnresolvedGap`. From 2026-08-22 the
    // declaration is retired and 13 of its 32 withheld dates again answer
    // `UnresolvedGap`; neither is the plain shape these two intersections show.
    // The `neighbour` is the first answered day after the withheld date, and
    // the resolution-edge rule (#151) means it is not complete either: every
    // query on it reaches the withheld day, so its verdict is
    // `OutsideCoveredRange` with the `ResolutionEdge` reason, and the first
    // complete date after the withheld one is the day past the zone. Cbot's
    // Saturday sits between the two and its Sunday session survives, so its
    // zone ends on the Saturday; iceus trades the whole span and its zone ends
    // two days out.
    let fixtures = [
        (
            Exchange::Cbot,
            date(2025, 1, 2),
            date(2025, 1, 3),
            date(2025, 1, 5),
        ),
        (
            Exchange::Iceus,
            date(2026, 1, 19),
            date(2026, 1, 20),
            date(2026, 1, 21),
        ),
    ];
    for (exchange, withheld, neighbour, complete_again) in fixtures {
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
        assert_eq!(
            coverage.coverage_on(neighbour),
            DateCoverage::OutsideCoveredRange,
            "{exchange:?}: answering {neighbour} completely would consult the              withheld {withheld}"
        );
        assert_eq!(
            gap_reason_on(coverage, neighbour),
            Some(CoverageGapReason::ResolutionEdge),
            "{exchange:?}"
        );
        assert!(
            coverage.is_complete_on(complete_again),
            "{exchange:?} answers {complete_again} completely"
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
        Some(date(2010, 1, 1))
    );
    // The 2018-04-09 Pillar row no longer bounds a carried interval: the
    // operator's own historical timeline and Rule 51 filings source the core
    // session to the floor, so the horizon reads 2010-01-01 and the clipped
    // sourced span opens there.
    let nyse = exchange_coverage(Exchange::Nyse);
    assert_eq!(nyse.normal_week_sourced_from(), Some(date(2010, 1, 1)));
    assert_eq!(nyse.sourced_normal_week().first(), date(2010, 1, 1));
    // The same floor move for `nasdaq` (2026-09-30 UTC): the operator's own
    // SEC filings and archived Trading Hours page source the 07:00-20:00 grid
    // below the 2013-03-18 cutover, so the horizon reads 2010-01-01 and the
    // 1,172 dates that used to refuse answer.
    let nasdaq = exchange_coverage(Exchange::Nasdaq);
    assert_eq!(nasdaq.normal_week_sourced_from(), Some(date(2010, 1, 1)));
    assert_eq!(nasdaq.sourced_normal_week().first(), date(2010, 1, 1));
    assert_eq!(
        nasdaq.coverage_on(date(2012, 6, 5)),
        DateCoverage::Covered,
        "a mid-era Tuesday below the old horizon is a covered date now"
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

    // The SR15 row remains the dated revision; the horizon is the earliest
    // capture of the pre-SR15 phase timetable (2013-09-16), so 2025 dates are
    // all sourced and 2013-09-15 is the last carried day.
    let asx = exchange_coverage(Exchange::Asx);
    assert_eq!(asx.normal_week_sourced_from(), Some(date(2013, 9, 16)));
    assert_eq!(
        gap_reason_on(asx, date(2013, 9, 15)),
        Some(CoverageGapReason::NormalWeekCarried)
    );
    assert_ne!(
        gap_reason_on(asx, date(2013, 9, 16)),
        Some(CoverageGapReason::NormalWeekCarried)
    );
    assert_eq!(
        gap_reason_on(asx, date(2025, 6, 22)),
        None,
        "the below-SR15 grid is sourced now, so no 2025 date is carried"
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
    // declaration is `EveryDay` and its spans blanket every audited window —
    // `eurex` before its dated German-scope rows shipped was that case, whose
    // editions ended 2026-12-31 with the bound the day after. A bounded span
    // that starts inside a window leaves the dates below it answering
    // (`globex_grains`' bracketed regime, `eurex` since its 2025-01-01 `tba`
    // bound), so the derivation walks the spans over each window.
    let all_every_day = coverage
        .phase_gaps()
        .iter()
        .all(|gap| gap.shape() == exchange_hours::PhaseGapShape::EveryDay);
    let nothing_survives = all_every_day
        && coverage
            .holiday_contract()
            .coverage()
            .is_some_and(|audited| {
                audited.windows().iter().all(|(first, last)| {
                    let mut spans: Vec<(NaiveDate, NaiveDate)> = coverage
                        .phase_gaps()
                        .iter()
                        .map(|gap| {
                            (
                                gap.applies_since().unwrap_or(*first),
                                gap.applies_until()
                                    .and_then(|until| until.pred_opt())
                                    .unwrap_or(*last),
                            )
                        })
                        .collect();
                    spans.sort();
                    let mut cursor = *first;
                    for (start, end) in spans {
                        if start > cursor {
                            return false;
                        }
                        if end >= cursor
                            && let Some(next) = end.succ_opt()
                        {
                            cursor = next;
                        }
                    }
                    cursor > *last
                })
            });
    assert_eq!(
        complete.is_empty(),
        whole_domain || nothing_survives,
        "{identity:?}: no complete span exactly while nothing survives the declarations"
    );
    assert!(
        gaps.iter().any(|gap| gap.phase_gap().is_some()),
        "{identity:?} reports its declarations among its records"
    );
    // The declaration records the walk reports are collected in one ascending
    // pass — a fresh `gaps()` walk per record would make this check quadratic
    // in the record count, which the date-scoped declarations drive into the
    // hundreds per identity. The identity of a record is its range paired with
    // its declaration, so a second pass over the collected records is the same
    // assertion the per-record `find` made.
    let walked_records: Vec<(DateRange, exchange_hours::PhaseGap)> = coverage
        .gaps()
        .filter_map(|gap| {
            gap.phase_gap()
                .map(|declaration| (gap.range(), declaration))
        })
        .collect();
    for gap in gaps {
        let Some(declaration) = gap.phase_gap() else {
            continue;
        };
        assert!(
            walked_records
                .iter()
                .any(|(range, walked)| *range == gap.range() && *walked == declaration),
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
        "{identity:?} reports a bounded span count (32 observed for \
         `globex_equity_index` and 28 for `globex_grains`: a date-scoped \
         declaration used to report one span per maximal run its shape resolved — \
         the bracket-era Sundays one record each, 1021 spans for \
         `globex_equity_index`, until the 2026-10-04 retirement collapsed the \
         runs back into the date-level edges)"
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
        "{identity:?} reports a bounded gap count (16 observed for \
         `globex_grains`, the date-level eras alone since the #152, #123, #259 \
         and #79 retirements lifted their records)"
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
    // Inside the audited windows the scope answers day by day, except at the
    // resolution edges (#151): the window's own first and last dates reach
    // outside it and are reported incomplete by the same accessor.
    for day in days_from_floor(date(2027, 12, 30)) {
        if day >= date(2025, 1, 5) {
            assert!(coverage.is_complete_on(day), "{day}");
            assert_eq!(gap_reason_on(coverage, day), None, "{day}");
        }
    }
    for day in [date(2025, 1, 1), date(2025, 1, 3), date(2027, 12, 31)] {
        assert_eq!(
            coverage.coverage_on(day),
            DateCoverage::OutsideCoveredRange,
            "{day}: a date whose resolution reach leaves the audited window is              not called complete"
        );
    }
}

#[test]
fn the_quarter_hour_residual_serves_the_intersection_on_every_bracket_era_sunday() {
    // #79's retirement (2026-10-04): the seven scopes' bracket-era Sundays now
    // answer from the tables, and the disputed 16:00-16:15 CT slice is served
    // as the closed half of the sourced 16:15-17:00 CT intersection. This
    // fence pins that on the dates the retired declaration used to refuse: a
    // Tuesday in the same era, a Sunday before the 2012-05-28 capture, a
    // Sunday whose evening leg a holiday removes, and the Sundays from the
    // 2026-08-22 knowledge-bound row on — all `Covered`, none refused.
    let sample = date(2025, 6, 10);
    let bracket_sunday = date(2015, 6, 14);
    let pre_bracket_sunday = date(2012, 5, 27);
    let first_bracket_sunday = date(2012, 6, 3);
    let holiday_sunday = date(2017, 12, 24);
    let after_the_bound = date(2026, 8, 23);
    for key in [MarketHoursKey::GlobexEquityIndex, MarketHoursKey::GlobexFx] {
        let coverage = key_coverage(key);
        assert!(
            coverage.phase_gaps().is_empty(),
            "{key:?}: the #79 retirement leaves no declaration"
        );
        for day in [
            sample,
            bracket_sunday,
            pre_bracket_sunday,
            first_bracket_sunday,
            holiday_sunday,
            after_the_bound,
        ] {
            assert_eq!(
                coverage.coverage_on(day),
                DateCoverage::Covered,
                "{key:?}: {day} answers under the residual convention"
            );
        }
    }

    // The query surface agrees per date: the disputed slice answers as the
    // served intersection's closed verdict, the sourced queue accepts after
    // it, and a Sunday whose evening leg a holiday removes answers closed.
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
        Ok(false),
        "the disputed slice answers as the served intersection's closed verdict, not a refusal"
    );
    assert_eq!(
        fx.is_accepting_orders(at(bracket_sunday, 16, 20)),
        Ok(true),
        "the sourced 16:15-17:00 CT queue accepts after the disputed slice"
    );
    assert_eq!(
        fx.is_accepting_orders(at(holiday_sunday, 16, 5)),
        Ok(false),
        "the holiday-removed Sunday answers closed"
    );
    assert_eq!(
        fx.is_accepting_orders(at(after_the_bound, 16, 5)),
        Ok(true),
        "after the knowledge-bound row the quarter-hour is served"
    );
    assert_eq!(
        fx.session_state(at(bracket_sunday, 16, 5)),
        Ok(SessionState::Closed),
        "the state names the served closed verdict the residual discloses"
    );
}

#[test]
fn the_queue_dates_answer_and_the_regime_serves_its_captured_queues() {
    // The #259 retirement (2026-10-04): the 2012-05-20..2013-04-06 regime's
    // queues are served from the regime's own dated start as CME's own
    // trading-hours captures print them twice inside the regime, and every
    // date of the regime answers. `globex_grains` declares nothing; neither
    // does `globex_livestock`; the queue days all read complete, in every
    // queue era.
    let grains = key_coverage(MarketHoursKey::GlobexGrains);
    let livestock = key_coverage(MarketHoursKey::GlobexLivestock);
    assert!(grains.phase_gaps().is_empty());
    assert!(livestock.phase_gaps().is_empty());

    // The queue days read complete: a Sunday, a mid-week queue day, the first
    // week of livestock's sourced 2016-06-06 Post-Close onset, grains' whole
    // 2010-2012 PCP era (serving the 14:30-16:00 sourced intersection since
    // #283 closed as a residual) and the era after the regime — all answer.
    for (coverage, day, why) in [
        (
            &grains,
            date(2021, 3, 14),
            "a Sunday carries no post-close queue",
        ),
        (
            &grains,
            date(2021, 3, 16),
            "a Tuesday in a queue era carries the 14:30-16:00 CT PCP, dated by the session it feeds",
        ),
        (
            &grains,
            date(2011, 6, 14),
            "the 2010-2012 PCP era is a queue era: the convention dates it like any other",
        ),
        (
            &grains,
            date(2013, 6, 10),
            "the 2013-04-07 notice's queue era carries the PCP again",
        ),
        (
            &livestock,
            date(2021, 3, 16),
            "livestock's Post-Close queue answers the same convention",
        ),
        (
            &livestock,
            date(2015, 6, 10),
            "livestock serves no post-close queue before its 2016-06-06 onset",
        ),
        (
            &livestock,
            date(2016, 6, 8),
            "the first week of the sourced Post-Close era answers under the convention",
        ),
    ] {
        assert_eq!(
            coverage.coverage_on(day),
            DateCoverage::Covered,
            "{day}: {why}"
        );
    }

    // The regime's own dates answer, and the queue phases they serve are the
    // captured ones: Sunday Pre-Open 16:00-17:00 CT, the weekday 14:30-16:00
    // PCP and the Monday-Thursday 16:45-17:00 evening Pre-Open, with the
    // morning queue the pre-regime grid carried gone — the wrapped session
    // runs through the morning on the 17:00-14:00 grid.
    for day in [date(2012, 5, 20), date(2012, 6, 3), date(2013, 4, 6)] {
        assert_eq!(
            grains.coverage_on(day),
            DateCoverage::Covered,
            "{day}: the regime answers under the residual convention"
        );
    }
    assert_eq!(
        grains.coverage_on(date(2012, 5, 13)),
        DateCoverage::Covered,
        "the regime's dated neighbours answer: 2012-05-13 is before it and served"
    );
    let grains_cal = calendar_for_market_hours_key(MarketHoursKey::GlobexGrains);
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
    // A regime Sunday: accepting from the captured 16:00 onset — the instant
    // inside the onset bracket's disputed quarter-hour included, which is what
    // the residual discloses — through 17:00, matching at 17:30.
    assert_eq!(
        grains_cal.is_accepting_orders(at(date(2012, 6, 3), 16, 5)),
        Ok(true),
        "the regime Sunday Pre-Open opens at the captured 16:00 CT onset"
    );
    assert_eq!(
        grains_cal.is_accepting_orders(at(date(2012, 6, 3), 16, 30)),
        Ok(true),
        "the regime Sunday Pre-Open queue accepts orders, as both captures print"
    );
    assert_eq!(
        grains_cal.is_open(at(date(2012, 6, 3), 17, 30)),
        Ok(true),
        "the regime's 17:00 CT electronic open matches"
    );
    // A regime weekday: the PCP accepts at 15:00, the evening Pre-Open at
    // 16:50, and the morning — a queue on the pre-regime grid — matches,
    // because the wrapped session runs through it.
    assert_eq!(
        grains_cal.is_accepting_orders(at(date(2012, 6, 6), 15, 0)),
        Ok(true),
        "the regime's 14:30-16:00 CT PCP accepts orders"
    );
    assert_eq!(
        grains_cal.is_accepting_orders(at(date(2012, 6, 6), 16, 50)),
        Ok(true),
        "the regime's Monday-Thursday 16:45-17:00 CT evening Pre-Open accepts orders"
    );
    assert_eq!(
        grains_cal.is_open(at(date(2012, 6, 6), 8, 15)),
        Ok(true),
        "the regime carries no morning queue: the wrapped session matches through it"
    );
    // The pre-regime grid still answers its own queues beside the regime.
    assert_eq!(
        grains_cal.is_accepting_orders(at(date(2012, 5, 16), 8, 15)),
        Ok(true),
        "the pre-regime morning queue 08:00-09:30 CT still accepts"
    );
    assert_eq!(
        grains_cal.is_accepting_orders(at(date(2012, 5, 13), 16, 5)),
        Ok(false),
        "the pre-regime Sunday onset is 16:15 CT: 16:05 is closed"
    );
    // The pre-regime queue set serves the sourced intersection of the two
    // bracketing states (#283, 2026-10-04 convention): notice 20100405 dated a
    // 13:15:30-16:00 CT PCP from 2010-04-19, the 2012-05-11 capture prints the
    // PCP at 14:30-16:00 with a 16:45 weekday evening Pre-Open, and no dated
    // artifact separates them — so 15:00 CT accepts under every sourced state
    // while 13:30 (inside only the notice's state) and 17:00 (inside only the
    // capture's evening queue) answer closed, under-reporting exactly as an
    // omitted queue would. Probed on a weekday of each undated era: 2011-06-14
    // (the 07:15 morning-queue era) and 2012-05-16 (the 08:00 one).
    for (day, era) in [
        (date(2011, 6, 14), "the 07:15-morning-queue era"),
        (date(2012, 5, 16), "the 08:00-morning-queue era"),
    ] {
        assert_eq!(
            grains_cal.is_accepting_orders(at(day, 15, 0)),
            Ok(true),
            "{era}: the served 14:30-16:00 CT PCP accepts at 15:00, the sourced intersection"
        );
        assert_eq!(
            grains_cal.session_state(at(day, 13, 30)),
            Ok(SessionState::Closed),
            "{era}: 13:30 CT is inside only the notice's 13:15:30 PCP start, so the \
             intersection answers closed"
        );
    }
    assert_eq!(
        grains_cal.session_state(at(date(2012, 5, 16), 17, 0)),
        Ok(SessionState::Closed),
        "the pre-regime weekday evening Pre-Open is served absent: the 16:45-18:00 CT \
         queue is witnessed only by the capture side of the bracket, and 17:00 CT \
         answers closed until the 18:00 electronic open"
    );
    assert_eq!(
        grains_cal.is_accepting_orders(at(date(2012, 5, 16), 17, 0)),
        Ok(false),
        "no evening queue over-reports order acceptance on a pre-regime weekday"
    );
    // The post-close queue and both of its verdicts are answered on every date
    // that carries it, under the charter convention the fence in
    // `tests/futures_family_boundaries/` pins instant by instant.
    let queue_instant = US::Central
        .with_ymd_and_hms(2021, 3, 16, 15, 0, 0)
        .single()
        .expect("a single 15:00 CT instant")
        .with_timezone(&Utc);
    assert_eq!(
        grains_cal.is_accepting_orders(queue_instant),
        Ok(true),
        "the post-close queue answers: it is served, and its trade date is the session it feeds"
    );
}

#[test]
fn the_five_day_era_answers_from_the_queue_absent_grid() {
    // #123's retirement (2026-10-04): the five-day era's normal-week queues
    // are sourced nowhere — the workbook rows that print a `Pre-opening` state
    // holiday arrangements, not the ordinary week — so the grid with the
    // queues absent is the narrowest state that holds under every sourced one,
    // and the whole era answers from it with the omission disclosed as a
    // residual in the evidence file.
    let crypto = key_coverage(MarketHoursKey::GlobexCryptocurrency);
    assert!(
        crypto.phase_gaps().is_empty(),
        "the #123 retirement leaves no declaration"
    );
    for day in [
        date(2019, 6, 9),
        date(2020, 6, 1),
        date(2026, 5, 24),
        date(2026, 5, 28),
    ] {
        assert_eq!(
            crypto.coverage_on(day),
            DateCoverage::Covered,
            "{day}: the five-day era answers under the residual convention"
        );
    }
    assert_eq!(
        crypto.coverage_on(date(2026, 5, 29)),
        DateCoverage::Covered,
        "the bridge day's own profile serves the published Pre-Open"
    );
    assert_eq!(
        crypto.coverage_on(date(2026, 6, 1)),
        DateCoverage::Covered,
        "the 24/7 era serves the published Pre-Open and answers"
    );
    // Before the grid the family's era is a sourced launch closure, and the
    // shipped holiday windows do not reach it — the honest reason those dates
    // refuse, unchanged by the retirement.
    assert_ne!(
        gap_reason_on(crypto, date(2015, 1, 1)),
        Some(CoverageGapReason::NormalWeekPhaseWithheld),
        "the pre-launch era was never the Pre-Open gap's to refuse"
    );
    assert_eq!(
        crypto.coverage_on(date(2015, 1, 1)),
        DateCoverage::OutsideCoveredRange
    );
    // The query surface answers from the queue-absent grid: the Sunday evening
    // before the 17:00 CT open is closed, matching opens at 17:30, and the
    // 24/7 era serves its published queues.
    let cal = calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency);
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
        cal.is_accepting_orders(at(date(2019, 6, 9), 16, 30)),
        Ok(false),
        "the era's Sunday evening answers from the queue-absent grid: no queue is served"
    );
    assert_eq!(
        cal.is_open(at(date(2019, 6, 9), 17, 30)),
        Ok(true),
        "the era's 17:00 CT open matches"
    );
    assert_eq!(
        cal.is_accepting_orders(at(date(2026, 5, 17), 16, 30)),
        Ok(false),
        "the era's own last ordinary Sundays answer from the same queue-absent grid"
    );
    // The 24/7 era serves the operator's published queues from the bridge day,
    // and the merged spans state their own 16:00 CT queues as blocks — the
    // residual is the ordinary week's alone.
    assert_eq!(
        cal.is_accepting_orders(at(date(2026, 6, 1), 16, 1)),
        Ok(true),
        "the 24/7 era's 16:01-16:02 CT queue accepts orders"
    );
    assert_eq!(
        cal.is_accepting_orders(at(date(2026, 5, 30), 3, 50)),
        Ok(true),
        "the 24/7 era's Saturday 03:45-04:00 CT queue accepts orders"
    );
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
    // closures are undated in the 2025 and 2026 editions (`tba` / `to be
    // announced`), so the site is incomplete on every date the declaration
    // covers while its ordinary week and order-entry queues are still served.
    // The dated German-scope rows of the 2014-2018 editions ship, so the
    // declaration starts where the `tba` note does. The reason travels with
    // the declaration, because the query gate answers through this reason and
    // refuses through the other two.
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
            .find_map(|gap| gap.applies_since()),
        Some(date(2025, 1, 1)),
        "the declaration starts at the tba era: the 2024 edition is the last one \
         without the German-scope note, and the dated rows before it ship"
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
        eurex.is_complete_on(date(2016, 10, 3)),
        "eurex answers the dated 2016 German Unity Day from the shipped row, so the date \
         is complete"
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
    // The #172 regression: a whole-domain declaration once refused every date
    // of 2025-01-01..2027-12-31 for the queue families, and
    // `UnpublishedClosureDates` did the same for eurex before #180 bounded it.
    // The date-scoped engine answers the dates the evidence does not withhold.
    // The #152 label declaration — the last whole-interval refusal these two
    // scopes carried — is retired (2026-10-03, the charter's Post-Close
    // trade-date convention), so both scopes now answer the whole interval
    // except the trailing resolution edge (#151): the last audited day's reach
    // crosses into 2028, outside every window, exactly as
    // `globex_nikkei_225_dollar`'s complete span ends 2027-12-30. Derivation:
    // the previous walk pinned `globex_grains` at 375 covered days and
    // `globex_livestock` at 341, with every refusal a 15:00-CT queue day (720
    // and 754 of them, recorded beside the retired declaration and in the two
    // evidence files) and no other refusal anywhere in the interval — so the
    // retirement lifts 720 + 754 refusals and leaves 1,095 − 1 = 1,094.
    let grains = key_coverage(MarketHoursKey::GlobexGrains);
    let livestock = key_coverage(MarketHoursKey::GlobexLivestock);
    assert_eq!(
        complete_days(grains),
        1094,
        "globex_grains answers 1,094 of the 1,095 days: the retired label gap's 720 refusals \
         lift, and only the trailing window edge (#151) refuses"
    );
    assert_eq!(
        complete_days(livestock),
        1094,
        "globex_livestock answers 1,094 of the 1,095 days: the retired label gap's 754 refusals \
         lift, and only the trailing window edge (#151) refuses"
    );

    // The queue day is still exactly a day that accepts orders at 15:00 CT;
    // after the retirement it is a **covered** day — the charter convention
    // dates its trade date by the session it feeds — with the one exception
    // the audited window's own trailing edge makes (#151): 2027-12-31 accepts
    // orders and still refuses, because its reach crosses into 2028, exactly
    // as `globex_nikkei_225_dollar`'s complete span ends 2027-12-30.
    // Livestock carried 754 such days — the count its retired declaration
    // refused — and grains 748: the 720 the retired declaration refused plus
    // the 28 block days whose restated or adjusted queue accepts beside a
    // complete day.
    let livestock_cal = calendar_for_market_hours_key(MarketHoursKey::GlobexLivestock);
    let mut day = REGRESSION_FIRST;
    let mut livestock_queue_days = 0_usize;
    let mut livestock_refused: Vec<NaiveDate> = Vec::new();
    while day <= REGRESSION_LAST {
        let verdict = livestock.coverage_on(day);
        let accepts = livestock_cal.is_accepting_orders(post_close_instant(day)) == Ok(true);
        if accepts {
            if day != REGRESSION_LAST {
                assert_eq!(
                    verdict,
                    DateCoverage::Covered,
                    "globex_livestock {day}: a queue day answers under the charter convention"
                );
            }
            livestock_queue_days += 1;
        }
        if verdict != DateCoverage::Covered {
            livestock_refused.push(day);
        }
        day = day
            .succ_opt()
            .expect("the walk stays inside the year range");
    }
    assert_eq!(
        livestock_queue_days, 754,
        "the queue days the retired #152 declaration refused are the queue days there are"
    );
    assert_eq!(
        livestock_refused,
        vec![REGRESSION_LAST],
        "only the audited window's trailing edge refuses livestock now (#151)"
    );

    // Grains refuses on nothing but that same trailing edge, and its queue
    // days all answer: 748 accept orders at 15:00 CT — the 720 the retired
    // declaration refused plus the 28 days whose own replacement blocks state
    // the adjusted day outright or restate the pre-eve queue (#175), the
    // fourteen closure eves and the fourteen run-ups that the family file
    // fences by name.
    let grains_cal = calendar_for_market_hours_key(MarketHoursKey::GlobexGrains);
    let mut grains_queue_days = 0_usize;
    let mut grains_refused: Vec<NaiveDate> = Vec::new();
    day = REGRESSION_FIRST;
    while day <= REGRESSION_LAST {
        let verdict = grains.coverage_on(day);
        let accepts = grains_cal.is_accepting_orders(post_close_instant(day)) == Ok(true);
        if verdict != DateCoverage::Covered {
            grains_refused.push(day);
            assert!(
                !accepts || day == REGRESSION_LAST,
                "a refused date must be the trailing edge or a quiet day, not a queue day \
                 answered incomplete: {day}"
            );
        }
        if accepts {
            grains_queue_days += 1;
        }
        day = day
            .succ_opt()
            .expect("the walk stays inside the year range");
    }
    assert_eq!(
        grains_refused,
        vec![REGRESSION_LAST],
        "only the audited window's trailing edge refuses grains now (#151)"
    );
    assert_eq!(
        grains_queue_days, 748,
        "the 720 queue days the retired #152 declaration refused plus the 28 block days \
         are the queue days there are"
    );

    // Eurex stays at zero complete days over 2025-2027, and for a narrowed
    // reason: the 2014-2018 editions date the German equity/equity-index
    // scope and those rows ship (2010-01-01..2024-12-31 answers Covered
    // whole), so what refuses inside the regression interval is the `tba`
    // note the 2025 and 2026 editions print for that scope (#157) — and 2027
    // lies outside the audited windows as before.
    let eurex = exchange_coverage(Exchange::Eurex);
    assert_eq!(complete_days(eurex), 0);
    let unpublished: Vec<(DateRange, Option<&str>)> = eurex
        .gaps()
        .filter(|gap| gap.reason() == CoverageGapReason::UnpublishedClosureDates)
        .map(|gap| (gap.range(), gap.closing_condition()))
        .collect();
    // The resolution-edge rule (#151) splits the run at the tba era's end:
    // 2026-12-31 answers incompletely because its next session reaches
    // 2027-01-01, outside every window, while 2025-01-01..2026-12-30 refuse
    // on the declared gap alone.
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
        "the #157 record spans the tba era the German scope stands unencoded across, \
         ending where the editions in hand end"
    );
}

/// The metadata/query agreement the coverage contract promises (#151).
///
/// `DateCoverage::Covered` promises that every question about every instant of
/// the date answers, or refuses naming a date the metadata itself does not call
/// covered. This walks identities' covered dates — one instant per day, the
/// five question families plus both order-entry probes — and asserts the
/// promise: a date-level coverage error raised on a `Covered` date must name a
/// date whose own verdict is not `Covered` (a withheld neighbour, a declared
/// phase-gap day, a date outside the audited windows), never a date the caller
/// would have trusted. `SearchExhausted` is not a date-level error and is
/// accepted; a query that answers is accepted outright.
///
/// Every identity is walked across the recent five years, which contains every
/// audited window end and every withheld date the 2025+ tables ship; the six
/// identities whose withheld dates carried #151's original disagreement are
/// walked from the 2010 floor, where their dense markers live.
#[test]
fn a_covered_date_is_never_refused_by_a_query_naming_another_covered_date() {
    fn assert_agreement(calendar: ExchangeCalendar, first: NaiveDate, last: NaiveDate) {
        let coverage = calendar.coverage();
        let label = format!("{:?}", calendar.source());
        let mut day = first;
        while day <= last {
            if coverage.coverage_on(day) == DateCoverage::Covered {
                let instant = Utc
                    .with_ymd_and_hms(day.year(), day.month(), day.day(), 14, 7, 0)
                    .single()
                    .expect("14:07Z falls once on every date");
                for (family, error) in [
                    ("is_open", calendar.is_open(instant).err()),
                    ("session_state", calendar.session_state(instant).err()),
                    ("trade_date", calendar.trade_date(instant).err()),
                    ("session_bounds", calendar.session_bounds(instant).err()),
                    (
                        "next_session_after",
                        calendar.next_session_after(instant).err(),
                    ),
                    (
                        "is_accepting_orders",
                        calendar.is_accepting_orders(instant).err(),
                    ),
                    (
                        "is_order_entry_only",
                        calendar.is_order_entry_only(instant).err(),
                    ),
                ] {
                    // `SearchExhausted` is the bounded search's own refusal, not
                    // a date-level verdict, so only the three date-shaped
                    // errors promise anything about the named date.
                    let Some(error) = error else { continue };
                    if matches!(error, CalendarQueryError::SearchExhausted { .. }) {
                        continue;
                    }
                    let named = error.date();
                    assert_ne!(
                        coverage.coverage_on(named),
                        DateCoverage::Covered,
                        "{label}: {family} at 14:07Z on {day} refuses naming {named}, \
                         which the metadata itself calls covered"
                    );
                    // The variant the query raises must be the one the
                    // metadata publishes for the day it names — the mapping
                    // `CalendarQueryError` documents, with the doubly-gapped
                    // -date divergence issue #128 recorded now removed.
                    let published = match coverage.coverage_on(named) {
                        DateCoverage::OutsideCoveredRange => {
                            Some(CalendarQueryError::OutsideCoveredRange {
                                source: calendar.source(),
                                date: named,
                            })
                        }
                        DateCoverage::UnresolvedGap => Some(CalendarQueryError::UnresolvedGap {
                            source: calendar.source(),
                            date: named,
                        }),
                        DateCoverage::BeforeSupportFloor => {
                            Some(CalendarQueryError::BeforeSupportFloor {
                                source: calendar.source(),
                                date: named,
                            })
                        }
                        _ => None,
                    };
                    if let Some(expected) = published {
                        assert_eq!(
                            error, expected,
                            "{label}: {family} at 14:07Z on {day} refuses {named} \
                             with a variant the metadata does not publish for it"
                        );
                    }
                }
            }
            day = day.succ_opt().unwrap_or(last);
            if day > last {
                break;
            }
        }
    }

    let recent = (date(2024, 1, 1), date(2028, 12, 31));
    let floor = (SUPPORT_FLOOR, date(2028, 12, 31));
    for &exchange in Exchange::ALL {
        let calendar = calendar_for_exchange(exchange);
        let (first, last) = if matches!(
            exchange,
            Exchange::Cbot | Exchange::Cme | Exchange::Iceus | Exchange::Comex
        ) {
            floor
        } else {
            recent
        };
        assert_agreement(calendar, first, last);
    }
    for &key in MarketHoursKey::ALL {
        let calendar = calendar_for_market_hours_key(key);
        let (first, last) = if matches!(
            key,
            MarketHoursKey::GlobexNikkei225Dollar | MarketHoursKey::IceUsSugar
        ) {
            floor
        } else {
            recent
        };
        assert_agreement(calendar, first, last);
    }
}
