// SPDX-License-Identifier: MIT-0

//! The bridged residual of the audited holiday windows, and the refusal
//! context that travels beside a typed refusal (issue #296).
//!
//! Tier 1 (2026-10-06): a span **between** two audited windows of an
//! identity's holiday table, whose normal week the identity sources, answers
//! its session questions from the sourced normal week while the holiday layer
//! stays honestly absent — the metadata reports the span as
//! [`CoverageGapReason::HolidayWindowsBridged`], the complete-calendar claim
//! stays withheld, and no closure is asserted that no operator statement
//! witnesses.
//!
//! Tier 2 (the 2026-10-07 ruling): the bridge extends to the one-flank span
//! **below the first window**. On a date there whose normal week the identity
//! sources, the session questions answer exactly as they do on a two-flank
//! span, while the holiday-table classification refuses a typed
//! [`CalendarQueryError::UnresolvedGap`] — a one-flank date has no bracket,
//! nothing witnesses the holiday layer, and two flanks bracket a span where
//! one does not. The asymmetry is the ruling's substance. A span **above the
//! last window** is not an evidence gap — the operator's publication horizon
//! governs there — and its dates keep refusing everywhere.
//!
//! Tier 3: every refusal is still a refusal; the error value carries the
//! sourced normal-week baseline its date sits inside
//! ([`CalendarQueryError::normal_week_baseline`]) so a consumer can make its
//! own call. The enrichment changes no verdict.
//!
//! Every value below is read back through the public API. The asserted spans
//! and windows are compared against the identity's own shipped tables and
//! timeline, so a flipped window edge, a moved baseline row or a changed
//! bridge input fails here rather than passing by agreement.

#![expect(
    clippy::expect_used,
    reason = "fixture constructors assert their own literals; a bad literal must fail the test"
)]

use chrono::{NaiveDate, TimeZone as _, Utc};
use chrono_tz::{Asia, Europe, US};
use exchange_hours::{
    CalendarCoverage, CalendarQueryError, CoverageGapReason, DateCoverage, Exchange,
    MarketHoursKey, SessionKind, calendar_for_exchange, calendar_for_market_hours_key,
};

/// A venue-local fixture date.
fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("fixture must be a valid date")
}

/// A Singapore-local instant, as UTC.
fn sgt(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> chrono::DateTime<Utc> {
    Asia::Singapore
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .expect("fixture must be an unambiguous Singapore instant")
        .with_timezone(&Utc)
}

/// An India-local instant, as UTC.
fn ist(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> chrono::DateTime<Utc> {
    Asia::Kolkata
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .expect("fixture must be an unambiguous Kolkata instant")
        .with_timezone(&Utc)
}

/// A Chicago-local instant, as UTC.
fn ct(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> chrono::DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .expect("fixture must be an unambiguous Chicago instant")
        .with_timezone(&Utc)
}

/// A Riyadh-local instant, as UTC.
fn riyadh(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> chrono::DateTime<Utc> {
    Asia::Riyadh
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .expect("fixture must be an unambiguous Riyadh instant")
        .with_timezone(&Utc)
}

/// A Shanghai-local instant, as UTC.
fn shanghai(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> chrono::DateTime<Utc> {
    Asia::Shanghai
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .expect("fixture must be an unambiguous Shanghai instant")
        .with_timezone(&Utc)
}

/// An Istanbul-local instant, as UTC.
fn istanbul(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> chrono::DateTime<Utc> {
    Europe::Istanbul
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .expect("fixture must be an unambiguous Istanbul instant")
        .with_timezone(&Utc)
}

/// A Sao Paulo-local instant, as UTC.
fn sao_paulo(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> chrono::DateTime<Utc> {
    chrono_tz::America::Sao_Paulo
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .expect("fixture must be an unambiguous Sao Paulo instant")
        .with_timezone(&Utc)
}

/// A New York-local instant, as UTC.
fn ny(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> chrono::DateTime<Utc> {
    chrono_tz::America::New_York
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .expect("fixture must be an unambiguous New York instant")
        .with_timezone(&Utc)
}

/// The reason the metadata reports for `date`, taken from the gap records.
fn gap_reason_on(coverage: CalendarCoverage, day: NaiveDate) -> Option<CoverageGapReason> {
    coverage
        .gaps()
        .find(|gap| gap.range().contains(day))
        .map(exchange_hours::CoverageGap::reason)
}

/// The **union** of the spans the metadata reports as `HolidayWindowsBridged`,
/// merged where records abut.
///
/// The walk reports the residual over maximal verdict runs and the static
/// boundary candidates may split one residual span into adjacent runs of the
/// same verdict; the union is the span the charter's convention names. The
/// per-date partition fence in `coverage_metadata.rs` holds every record to
/// the per-date accessor, so merging equal-verdict neighbours here cannot
/// hide a verdict change.
fn bridged_union(coverage: CalendarCoverage) -> Vec<(NaiveDate, NaiveDate)> {
    let mut records: Vec<(NaiveDate, NaiveDate)> = coverage
        .gaps()
        .filter(|gap| gap.reason() == CoverageGapReason::HolidayWindowsBridged)
        .map(|gap| (gap.range().first(), gap.range().last()))
        .collect();
    records.sort();
    let mut merged: Vec<(NaiveDate, NaiveDate)> = Vec::new();
    for (first, last) in records {
        match merged.last_mut() {
            Some((_, open_last)) if first.pred_opt() == Some(*open_last) => *open_last = last,
            _ => merged.push((first, last)),
        }
    }
    merged
}

// ----------------------------------------------------------- the bridge mode

#[test]
fn the_between_window_spans_report_the_bridged_residual_not_a_refusal() {
    // `sgx_securities` ships windows 2014-2019, the one 2020-01-01 date and
    // 2025-2026; the 2020-01-02..2024-12-31 span between the last two windows
    // is the bridged residual.
    let sgx = calendar_for_exchange(Exchange::SgxSecurities).coverage();
    assert_eq!(
        bridged_union(sgx),
        vec![
            (date(2010, 1, 1), date(2013, 12, 31)),
            (date(2020, 1, 2), date(2024, 12, 31)),
        ],
        "the sgx below-first span and its capture gap are both residual spans"
    );
    // `nse_india` ships windows 2010-2017 and 2019-2026; 2018 is the bridged
    // residual. 2012 sat beside it until 2026-10-06 UTC, when the 2012 list's
    // own annual circular (`NSE-CIRC-2011-66`) surfaced and the window merged.
    let nse = calendar_for_exchange(Exchange::NseIndia).coverage();
    assert_eq!(
        bridged_union(nse),
        vec![(date(2018, 1, 1), date(2018, 12, 31))],
        "the one unrecovered NSE year is one residual span"
    );
    // The residual spans together cover 1,826 sgx trade dates (2020-01-02 was
    // a holiday in its own right, so the span opens on 01-02) and 365 nse
    // trade dates: 2,191 in all.
    let mut bridged_days = 0_usize;
    for (first, last) in [
        (date(2020, 1, 2), date(2024, 12, 31)),
        (date(2018, 1, 1), date(2018, 12, 31)),
    ] {
        let mut day = first;
        while day <= last {
            bridged_days += 1;
            day = day
                .succ_opt()
                .expect("the residual span stays inside the representable calendar");
        }
    }
    assert_eq!(
        bridged_days, 2_191,
        "the bridged union is the 1,826 sgx days plus the 365 nse days"
    );
    // The residual withholds the complete-calendar claim — it is not an
    // audited normal and not `Covered`.
    for (coverage, day) in [(&sgx, date(2022, 6, 8)), (&nse, date(2018, 3, 12))] {
        assert_eq!(
            gap_reason_on(*coverage, day),
            Some(CoverageGapReason::HolidayWindowsBridged),
            "{day} reports the bridged residual"
        );
        assert_eq!(
            coverage.coverage_on(day),
            DateCoverage::OutsideCoveredRange,
            "{day} withholds the complete-calendar claim"
        );
        assert!(
            !coverage.is_complete_on(day),
            "{day} is not claimed complete: the holiday layer is absent"
        );
    }
}

/// One below-first span row: the identity's wire name, the span's first and
/// last venue-local dates, and the first audited window's own first day.
type BelowFirstSpan = (
    &'static str,
    (i32, u32, u32),
    (i32, u32, u32),
    (i32, u32, u32),
);

/// Every below-first span the shipped tables state, with the identity that
/// carries it and the first audited window's own first day. The spans are
/// derived from the tables' windows and the ledger horizons: a table whose
/// first window opens above the floor leaves the floor-to-window span
/// below-first, clipped to the carried-below horizon where the week itself is
/// carried rather than sourced. A table opening at the floor leaves none.
const BELOW_FIRST_SPANS: &[BelowFirstSpan] = &[
    // (wire name, span first, span last, first window's first day)
    ("sgx_securities", (2010, 1, 1), (2013, 12, 31), (2014, 1, 1)),
    ("cfe", (2010, 1, 1), (2014, 12, 23), (2014, 12, 24)),
    ("cfe_vix", (2010, 1, 1), (2014, 12, 23), (2014, 12, 24)),
    ("sse", (2010, 1, 1), (2010, 12, 31), (2011, 1, 1)),
    ("b3", (2010, 1, 1), (2010, 12, 31), (2011, 1, 1)),
    ("tadawul", (2010, 1, 1), (2020, 12, 31), (2021, 1, 1)),
    ("borsa_istanbul", (2010, 1, 1), (2012, 3, 1), (2012, 3, 2)),
    (
        "coinbase_derivatives",
        (2010, 1, 1),
        (2021, 6, 27),
        (2021, 6, 28),
    ),
    (
        "globex_cryptocurrency",
        (2010, 1, 1),
        (2018, 12, 31),
        (2019, 1, 1),
    ),
    ("iceus", (2010, 1, 1), (2024, 12, 31), (2025, 1, 1)),
    ("ice_us", (2010, 1, 1), (2024, 12, 31), (2025, 1, 1)),
    // The softs' and dollar-index weeks are carried below their own horizons,
    // so only the sourced part of the span bridges.
    ("ice_us_sugar", (2011, 8, 1), (2024, 12, 31), (2025, 1, 1)),
    ("ice_us_coffee", (2011, 8, 1), (2024, 12, 31), (2025, 1, 1)),
    ("ice_us_cocoa", (2011, 8, 1), (2024, 12, 31), (2025, 1, 1)),
    ("ice_us_cotton", (2011, 8, 1), (2024, 12, 31), (2025, 1, 1)),
    (
        "ice_us_orange_juice",
        (2011, 8, 1),
        (2024, 12, 31),
        (2025, 1, 1),
    ),
    (
        "ice_us_dollar_index",
        (2011, 2, 7),
        (2024, 12, 31),
        (2025, 1, 1),
    ),
];

/// The calendar for a wire name, whichever identity enum owns it.
fn calendar_for(name: &str) -> exchange_hours::ExchangeCalendar {
    if let Ok(exchange) = name.parse::<Exchange>() {
        calendar_for_exchange(exchange)
    } else {
        calendar_for_market_hours_key(name.parse::<MarketHoursKey>().expect("a known wire name"))
    }
}

#[test]
fn every_below_first_span_reports_the_bridged_residual() {
    for (name, first, last, window_first) in BELOW_FIRST_SPANS {
        let calendar = calendar_for(name);
        let coverage = calendar.coverage();
        let (first, last, window_first) = (
            date(first.0, first.1, first.2),
            date(last.0, last.1, last.2),
            date(window_first.0, window_first.1, window_first.2),
        );
        // The span is the first bridged run, ending the day before the first
        // audited window opens.
        let spans = bridged_union(coverage);
        assert_eq!(
            spans.first(),
            Some(&(first, last)),
            "{name}: the below-first span is the bridged residual"
        );
        assert_eq!(
            window_first.pred_opt(),
            Some(last),
            "{name}: the span ends the day before the first audited window"
        );
        // The span withholds the complete-calendar claim but answers its
        // date-level facts.
        let probe = first;
        assert_eq!(
            coverage.coverage_on(probe),
            DateCoverage::OutsideCoveredRange,
            "{name}: {probe} withholds the complete-calendar claim"
        );
        assert_eq!(
            gap_reason_on(coverage, probe),
            Some(CoverageGapReason::HolidayWindowsBridged),
            "{name}: {probe} reports the bridged residual"
        );
        // The span is excluded from the complete ranges.
        assert!(
            coverage
                .complete_ranges()
                .all(|range| !range.contains(probe)),
            "{name}: {probe} is not claimed complete"
        );
    }
}

#[test]
fn bridged_dates_answer_their_session_questions_from_the_sourced_normal_week() {
    let sgx = calendar_for_exchange(Exchange::SgxSecurities);
    // A Monday of the bridged span: the 2019-06-03 grid's regular session
    // answers open, and its containing morning session runs 09:00-12:00 SGT
    // end-exclusively.
    assert_eq!(
        sgx.is_open(sgt(2021, 6, 14, 10, 0)),
        Ok(true),
        "the bridged Monday answers its normal-week regular session"
    );
    assert_eq!(
        sgx.session_bounds(sgt(2021, 6, 14, 10, 0)),
        Ok(Some((sgt(2021, 6, 14, 9, 0), sgt(2021, 6, 14, 12, 0)))),
        "the containing session is the normal week's morning leg"
    );
    assert_eq!(
        sgx.trade_date(sgt(2021, 6, 14, 10, 0)),
        Ok(Some(date(2021, 6, 14))),
        "the bridged date carries its own trade date"
    );
    assert_eq!(
        sgx.is_accepting_orders(sgt(2021, 6, 14, 8, 45)),
        Ok(true),
        "the normal-week pre-open queue answers accepting orders"
    );
    // The weekend shape survives the bridge: a bridged Saturday answers shut.
    assert_eq!(
        sgx.is_open(sgt(2021, 6, 19, 10, 0)),
        Ok(false),
        "the bridged Saturday answers the normal week's own weekend closure"
    );
    // `nse_india`: the trade-date closure question answers from the sourced
    // normal week — a Friday and a Monday of the bridged year are open.
    let nse = calendar_for_exchange(Exchange::NseIndia);
    assert_eq!(
        nse.is_closed_trade_date(date(2018, 6, 15), SessionKind::Both),
        Ok(false),
        "the bridged Friday answers not-closed beside the disclosed residual"
    );
    assert_eq!(
        nse.is_closed_trade_date(date(2018, 3, 12), SessionKind::Both),
        Ok(false),
        "the bridged Monday answers not-closed beside the disclosed residual"
    );
    assert_eq!(
        nse.is_open(ist(2018, 6, 15, 10, 0)),
        Ok(true),
        "the bridged NSE Friday answers its normal-week session"
    );
}

/// One below-first date per span, with a mid-session venue-local hour and the
/// weekday the date falls on, so the session-answer fence can compare the
/// date-aware answer against the detached normal week.
const BELOW_FIRST_PROBES: &[(&str, (i32, u32, u32))] = &[
    ("sgx_securities", (2011, 6, 15)),
    ("cfe", (2012, 6, 13)),
    ("cfe_vix", (2012, 6, 13)),
    ("sse", (2010, 6, 16)),
    ("b3", (2010, 6, 16)),
    ("tadawul", (2015, 6, 10)),
    ("borsa_istanbul", (2011, 6, 15)),
    ("coinbase_derivatives", (2015, 6, 10)),
    ("globex_cryptocurrency", (2015, 6, 10)),
    ("iceus", (2015, 6, 10)),
    ("ice_us", (2015, 6, 10)),
    ("ice_us_sugar", (2015, 6, 10)),
    ("ice_us_coffee", (2015, 6, 10)),
    ("ice_us_cocoa", (2015, 6, 10)),
    ("ice_us_cotton", (2015, 6, 10)),
    ("ice_us_orange_juice", (2015, 6, 10)),
    ("ice_us_dollar_index", (2015, 6, 10)),
];

/// The mid-session venue-local instant for a probe date, per identity zone.
fn probe_instant(name: &str, day: (i32, u32, u32)) -> chrono::DateTime<Utc> {
    let (year, month, d) = day;
    match name {
        "sgx_securities" => sgt(year, month, d, 10, 0),
        "sse" => shanghai(year, month, d, 10, 30),
        "tadawul" => riyadh(year, month, d, 11, 0),
        "borsa_istanbul" => istanbul(year, month, d, 11, 0),
        "b3" => sao_paulo(year, month, d, 11, 0),
        // CFE, coinbase_derivatives and the ICE Futures U.S. families all
        // state their grids in Chicago or New York civil time.
        "cfe" | "cfe_vix" | "coinbase_derivatives" => ct(year, month, d, 12, 0),
        _ => ny(year, month, d, 12, 0),
    }
}

#[test]
fn a_below_first_date_answers_its_session_questions_like_the_flanking_week() {
    // A 2011 Wednesday answers the sourced SGX week: the pre-2011-08-01 grid
    // (Practice Note 8.2.1) runs 09:00-12:30 and 14:00-17:00, and the
    // date-aware calendar answers it whole.
    let sgx = calendar_for_exchange(Exchange::SgxSecurities);
    assert_eq!(
        sgx.is_open(sgt(2011, 6, 15, 10, 0)),
        Ok(true),
        "the below-first Wednesday answers the sourced SGX week"
    );
    assert_eq!(
        sgx.session_bounds(sgt(2011, 6, 15, 10, 0)),
        Ok(Some((sgt(2011, 6, 15, 9, 0), sgt(2011, 6, 15, 12, 30)))),
        "the containing session is the era's own morning leg"
    );
    assert_eq!(
        sgx.trade_date(sgt(2011, 6, 15, 10, 0)),
        Ok(Some(date(2011, 6, 15))),
        "the below-first date carries its own trade date"
    );
    assert_eq!(
        sgx.session_state(sgt(2011, 6, 15, 10, 0)),
        Ok(exchange_hours::SessionState::OpenRegular),
        "the below-first date's state reads from the normal week"
    );
    assert_eq!(
        sgx.is_accepting_orders(sgt(2011, 6, 15, 10, 0)),
        Ok(true),
        "an open instant accepts orders"
    );
    // The weekend survives the one-flank bridge the same way it survives the
    // two-flank one: a below-first Saturday answers the week's own closure.
    assert_eq!(
        sgx.is_open(sgt(2011, 6, 18, 10, 0)),
        Ok(false),
        "the below-first Saturday answers the sourced week's weekend"
    );
    // Every remaining span: the date-aware session answers equal the detached
    // normal week's own answers — the same grid `hours_at` states — so the
    // bridge lifts exactly the holiday layer and nothing beside it.
    for (name, probe) in BELOW_FIRST_PROBES {
        let calendar = calendar_for(name);
        let bare = calendar.without_holidays();
        let instant = probe_instant(name, *probe);
        assert_eq!(
            calendar.is_open(instant),
            Ok(bare
                .is_open(instant)
                .expect("the detached week claims no coverage inside the sourced span")),
            "{name}: the below-first date answers the normal week it sources"
        );
        assert_eq!(
            calendar.is_accepting_orders(instant),
            Ok(bare
                .is_accepting_orders(instant)
                .expect("the detached week claims no coverage inside the sourced span")),
            "{name}: the below-first date's order state is the normal week's"
        );
        assert_eq!(
            calendar.session_bounds(instant),
            Ok(bare
                .session_bounds(instant)
                .expect("the detached week states the session")),
            "{name}: the below-first date's bounds are the normal week's"
        );
        assert_eq!(
            calendar.trade_date(instant),
            Ok(bare
                .trade_date(instant)
                .expect("the detached week states the trade date")),
            "{name}: the below-first date's trade date is the normal week's"
        );
    }
}

#[test]
fn a_below_first_date_holiday_classification_refuses_with_the_baseline() {
    // 2011-06-15 sits below sgx's first audited window: the session questions
    // answer (the fence above), but the holiday-table classification refuses
    // — no window brackets the date from below, so nothing witnesses the
    // holiday layer and a "not closed" answer would be fabricated.
    let sgx = calendar_for_exchange(Exchange::SgxSecurities);
    assert_eq!(
        sgx.is_open(sgt(2011, 6, 15, 10, 0)),
        Ok(true),
        "the session layer answers the same date"
    );
    let error = sgx
        .is_closed_trade_date(date(2011, 6, 15), SessionKind::Both)
        .expect_err("the one-flank classification refuses");
    assert!(matches!(
        error,
        CalendarQueryError::UnresolvedGap { source, date: refused }
            if source == sgx.source() && refused == date(2011, 6, 15)
    ));
    // The enriched refusal: the baseline names the sourced week the date sits
    // inside — the pre-2011-08-01 grid the operator's own Practice Note 8.2.1
    // amendment states.
    let baseline = error
        .normal_week_baseline()
        .expect("the sourced week states a baseline");
    assert_eq!(baseline.identity(), sgx.source());
    assert_eq!(baseline.weekday(), chrono::Weekday::Wed);
    let rendered = baseline.to_string();
    assert!(
        rendered.contains("normal week Wednesday")
            && rendered.contains("open 09:00-12:30")
            && rendered.contains("open 14:00-17:00"),
        "the baseline renders the era's own session legs: {rendered}"
    );
    assert!(
        error.to_string().contains(&rendered),
        "the enriched refusal renders the baseline beside the verdict"
    );
    // The same refusal shape on every remaining span, dates of each.
    for (name, probe) in [
        ("cfe", (2012, 6, 13)),
        ("cfe_vix", (2012, 6, 13)),
        ("sse", (2010, 6, 16)),
        ("b3", (2010, 6, 16)),
        ("tadawul", (2015, 6, 10)),
        ("borsa_istanbul", (2011, 6, 15)),
        ("coinbase_derivatives", (2015, 6, 10)),
        ("globex_cryptocurrency", (2015, 6, 10)),
        ("iceus", (2015, 6, 10)),
        ("ice_us", (2015, 6, 10)),
    ] {
        let calendar = calendar_for(name);
        assert!(
            matches!(
                calendar.is_closed_trade_date(date(probe.0, probe.1, probe.2), SessionKind::Both),
                Err(CalendarQueryError::UnresolvedGap { .. })
            ),
            "{name}: the below-first classification refuses typed"
        );
    }
    // A carried week below a one-flank span still refuses as carried, and
    // states no baseline: the ledger records the week as carried, not
    // sourced.
    let sugar = calendar_for_market_hours_key(MarketHoursKey::IceUsSugar);
    let error = sugar
        .is_closed_trade_date(date(2011, 6, 15), SessionKind::Both)
        .expect_err("the carried part of the span refuses");
    assert!(matches!(
        error,
        CalendarQueryError::OutsideCoveredRange { .. }
    ));
    assert!(
        error.normal_week_baseline().is_none(),
        "a carried week states no baseline"
    );
}

#[test]
fn the_flank_kinds_disagree_on_classification_at_the_first_window() {
    // The ruling's asymmetry, pinned on one boundary pair: 2013-12-31 sits in
    // sgx's one-flank below-first span and its classification refuses, while
    // 2014-01-02 — the first audited window's second day, audited normal —
    // answers not-closed from the table. Two flanks bracket a span; one does
    // not.
    let sgx = calendar_for_exchange(Exchange::SgxSecurities);
    assert_eq!(
        sgx.is_closed_trade_date(date(2013, 12, 31), SessionKind::Both),
        Err(CalendarQueryError::UnresolvedGap {
            source: sgx.source(),
            date: date(2013, 12, 31),
        }),
        "the one-flank side of the boundary refuses typed"
    );
    assert_eq!(
        sgx.is_closed_trade_date(date(2014, 1, 2), SessionKind::Both),
        Ok(false),
        "the audited-normal side of the boundary answers from the table"
    );
    // And the two-flank bridged span answers its classification beside the
    // residual, exactly as the one-flank span refuses.
    assert_eq!(
        sgx.is_closed_trade_date(date(2022, 6, 8), SessionKind::Both),
        Ok(false),
        "the two-flank bridged Wednesday answers not-closed beside the residual"
    );
}

#[test]
fn the_bridge_asserts_no_holiday_and_leaves_the_flank_rows_whole() {
    let sgx = calendar_for_exchange(Exchange::SgxSecurities);
    let nse = calendar_for_exchange(Exchange::NseIndia);
    // Inside the bridge the table states nothing: no row, no audited normal,
    // and no closure fabricated from either flank.
    for (calendar, day) in [
        (&sgx, date(2020, 6, 15)),
        (&sgx, date(2021, 6, 14)),
        (&sgx, date(2024, 12, 30)),
        (&sgx, date(2011, 6, 14)),
        (&nse, date(2018, 6, 15)),
        (&nse, date(2018, 3, 12)),
    ] {
        assert_eq!(
            calendar.holiday_on(day),
            None,
            "{day} inside the bridge carries no row and asserts no holiday"
        );
    }
    // The flanking windows' own rows are untouched: the one 2020 date the
    // 2019 sheet prints answers shut, and the withheld NSE Muhurat date
    // inside its window still refuses as an unresolved gap.
    assert_eq!(
        sgx.holiday_on(date(2020, 1, 1))
            .map(exchange_hours::Holiday::kind),
        Some(exchange_hours::HolidayKind::Closed),
        "the 2020-01-01 row survives at the bridge's edge"
    );
    assert!(
        !sgx.is_open(sgt(2020, 1, 1, 10, 0))
            .expect("inside a window"),
        "the 2020 New Year closure answers shut"
    );
    assert!(matches!(
        nse.is_closed_trade_date(date(2014, 10, 23), SessionKind::Both),
        Err(CalendarQueryError::UnresolvedGap { .. })
    ));
}

#[test]
fn the_flanks_answer_unchanged_and_need_no_bridge() {
    let sgx = calendar_for_exchange(Exchange::SgxSecurities);
    // Inside the first audited window: the sheet's own Hari Raya Haji
    // closure of 2018-06-15 answers shut, and an audited-normal Wednesday of
    // the same window answers open — both exactly as the table ships them.
    assert_eq!(
        sgx.is_open(sgt(2018, 6, 15, 10, 0)),
        Ok(false),
        "the 2018 window's own closure answers shut"
    );
    assert_eq!(
        sgx.holiday_on(date(2018, 6, 20)),
        None,
        "2018-06-20 is audited normal (no row inside the window)"
    );
    assert_eq!(
        sgx.is_open(sgt(2018, 6, 20, 10, 0)),
        Ok(true),
        "the audited-normal Wednesday answers open"
    );
    assert_eq!(
        sgx.coverage().coverage_on(date(2021, 6, 14)),
        DateCoverage::OutsideCoveredRange,
        "the bridged Monday is not metadata-complete"
    );
}

#[test]
fn above_the_last_window_still_refuses_everywhere() {
    // Dates above the last audited window are not an evidence gap: the
    // operator's publication horizon governs there (the 2026-10-07 ruling
    // scopes the bridge below-first only), so the whole date refuses and the
    // classification refuses with it — as `NoHolidayCoverage`/`OutsideCoveredRange`,
    // never as a bridged residual.
    for (name, probe) in [
        ("sgx_securities", (2027, 6, 15)),
        ("nse_india", (2027, 6, 15)),
        ("cfe", (2027, 6, 15)),
        ("tadawul", (2028, 6, 13)),
        ("b3", (2027, 6, 15)),
        ("sse", (2027, 6, 15)),
        ("borsa_istanbul", (2027, 6, 15)),
    ] {
        let calendar = calendar_for(name);
        let coverage = calendar.coverage();
        let day = date(probe.0, probe.1, probe.2);
        assert_eq!(
            gap_reason_on(coverage, day),
            Some(CoverageGapReason::NoHolidayCoverage),
            "{name}: {day} above the last window is the recorded gap, not a bridge"
        );
        assert_eq!(
            coverage.coverage_on(day),
            DateCoverage::OutsideCoveredRange,
            "{name}: {day} withholds the complete-calendar claim"
        );
        assert!(
            calendar.is_open(probe_instant(name, probe)).is_err(),
            "{name}: the session questions above the last window refuse"
        );
        assert!(
            calendar
                .is_closed_trade_date(day, SessionKind::Both)
                .is_err_and(|error| matches!(
                    error,
                    CalendarQueryError::OutsideCoveredRange { .. }
                )),
            "{name}: the classification above the last window refuses as before"
        );
    }
}

// ------------------------------------------------- the Tier-3 refusal context

#[test]
fn a_withheld_date_refusal_carries_its_normal_week_baseline() {
    let nse = calendar_for_exchange(Exchange::NseIndia);
    // 2014-10-23 is one of the Muhurat dates the operator footnotes without
    // instants: the refusal is unchanged, and the baseline names the sourced
    // normal week it sits inside.
    let error = nse
        .is_closed_trade_date(date(2014, 10, 23), SessionKind::Both)
        .expect_err("the withheld date refuses");
    assert!(matches!(
        error,
        CalendarQueryError::UnresolvedGap { source, date: refused }
            if source == nse.source() && refused == date(2014, 10, 23)
    ));
    let baseline = error
        .normal_week_baseline()
        .expect("the sourced week states a baseline");
    assert_eq!(
        baseline.identity(),
        nse.source(),
        "the baseline belongs to the refusing identity"
    );
    assert_eq!(baseline.tz(), Asia::Kolkata);
    assert_eq!(
        baseline.weekday(),
        chrono::Weekday::Thu,
        "2014-10-23 is a Thursday in the venue's own zone"
    );
    // The tradeable windows are the timeline's own 2011-10-03 grid: the
    // 09:15-15:30 continuous session, the 09:07-09:15 routine and the
    // 15:40-16:00 closing session; the order-entry queue opens 09:00.
    let tradeable: Vec<(u32, u32)> = baseline
        .tradeable_rules()
        .map(|rule| (rule.open_ssm, rule.close_ssm))
        .collect();
    // In table order: the regular session first, then the tradeable routine
    // slices the module's extended set carries.
    assert_eq!(
        tradeable,
        vec![
            (9 * 3_600 + 15 * 60, 15 * 3_600 + 30 * 60),
            (9 * 3_600 + 7 * 60, 9 * 3_600 + 15 * 60),
            (15 * 3_600 + 40 * 60, 16 * 3_600),
        ],
        "the baseline states the normal week's own tradeable slices"
    );
    assert_eq!(
        baseline
            .order_entry_rules()
            .map(|rule| (rule.open_ssm, rule.close_ssm))
            .collect::<Vec<_>>(),
        vec![(9 * 3_600, 9 * 3_600 + 7 * 60)],
        "the baseline states the normal week's own order-entry queue"
    );
    // The rendered context reads as the issue's example does, and the
    // baseline itself says nothing about the holiday arrangement.
    let rendered = baseline.to_string();
    assert!(
        rendered.contains("normal week Thursday"),
        "the baseline names the weekday: {rendered}"
    );
    assert!(
        rendered.contains("open 09:15-15:30"),
        "the baseline renders the ordinary session: {rendered}"
    );
    assert!(
        !rendered.contains("holiday"),
        "the baseline states the week only: {rendered}"
    );
    assert!(
        error.to_string().contains(&rendered),
        "the enriched refusal renders the baseline beside the verdict"
    );
    assert!(
        error
            .to_string()
            .contains("the holiday arrangement is unsourced"),
        "the enrichment names what is withheld: {error}"
    );
}

#[test]
fn the_baseline_is_absent_where_the_normal_week_is_not_sourced() {
    let nse = calendar_for_exchange(Exchange::NseIndia);
    // A pre-floor refusal states no baseline: the supported domain has not
    // started, and the below-floor contract is unchanged.
    let error = nse
        .is_open(ist(2009, 12, 31, 10, 0))
        .expect_err("a pre-floor instant refuses");
    assert!(matches!(
        error,
        CalendarQueryError::BeforeSupportFloor { .. }
    ));
    assert!(
        error.normal_week_baseline().is_none(),
        "no baseline is claimed below the support floor"
    );
    // A carried-week refusal states no baseline: the ledger records the week
    // as carried, not sourced, and claiming one would fabricate it.
    let krx = calendar_for_exchange(Exchange::Krx);
    let error = krx
        .is_closed_trade_date(date(2015, 6, 15), SessionKind::Both)
        .expect_err("a carried date refuses");
    assert!(matches!(
        error,
        CalendarQueryError::OutsideCoveredRange { .. }
    ));
    assert_eq!(
        gap_reason_on(krx.coverage(), date(2015, 6, 15)),
        Some(CoverageGapReason::NormalWeekCarried),
        "the refusal is the carried normal week"
    );
    assert!(
        error.normal_week_baseline().is_none(),
        "a carried week states no baseline"
    );
}
