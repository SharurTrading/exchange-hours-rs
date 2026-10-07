// SPDX-License-Identifier: MIT-0

//! The bridged residual between audited holiday windows, and the refusal
//! context that travels beside a typed refusal (issue #296).
//!
//! Tier 1: a span **between** two audited windows of an identity's holiday
//! table, whose normal week the identity sources, answers its session
//! questions from the sourced normal week while the holiday layer stays
//! honestly absent — the metadata reports the span as
//! [`CoverageGapReason::HolidayWindowsBridged`], the complete-calendar claim
//! stays withheld, and no closure is asserted that no operator statement
//! witnesses. A span with only one flank refuses as before.
//!
//! Tier 3: every refusal is still a refusal; the error value now carries the
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
use chrono_tz::Asia;
use exchange_hours::{
    CalendarCoverage, CalendarQueryError, CoverageGapReason, DateCoverage, Exchange, SessionKind,
    calendar_for_exchange,
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
        vec![(date(2020, 1, 2), date(2024, 12, 31))],
        "the sgx capture gap between its audited windows is one residual span"
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
    // The one-flank spans beside them are not bridged: below the first
    // window and above the last, the whole date still refuses.
    for (coverage, day, label) in [
        (&sgx, date(2011, 6, 13), "sgx below its first window"),
        (&sgx, date(2027, 6, 15), "sgx above its last window"),
        (&nse, date(2027, 6, 15), "nse above its last window"),
    ] {
        assert_eq!(
            gap_reason_on(*coverage, day),
            Some(CoverageGapReason::NoHolidayCoverage),
            "{label} keeps the one-flank refusal"
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

#[test]
fn a_one_flank_refusal_still_carries_its_sourced_week_baseline() {
    let sgx = calendar_for_exchange(Exchange::SgxSecurities);
    // 2011-06-13 sits below the first audited window and refuses — but the
    // normal week there is sourced (the horizon sits at the floor), so the
    // refusal carries the baseline it sits inside: the pre-2011-08-01 grid
    // the operator's own Practice Note 8.2.1 amendment states.
    let error = sgx
        .is_open(sgt(2011, 6, 13, 10, 0))
        .expect_err("the one-flank span refuses");
    assert!(matches!(
        error,
        CalendarQueryError::OutsideCoveredRange { source, date: refused }
            if source == sgx.source() && refused == date(2011, 6, 13)
    ));
    let baseline = error
        .normal_week_baseline()
        .expect("the sourced week states a baseline");
    assert_eq!(baseline.weekday(), chrono::Weekday::Mon);
    let rendered = baseline.to_string();
    assert!(
        rendered.contains("open 09:00-12:30") && rendered.contains("open 14:00-17:00"),
        "the baseline renders the era's own session legs: {rendered}"
    );
}
