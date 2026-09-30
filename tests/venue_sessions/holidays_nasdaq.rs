// SPDX-License-Identifier: MIT-0

//! Built-in holiday rows for the `nasdaq` venue, 2010-2026.
//!
//! Every case runs through `calendar_for_exchange`. The table's scalars are
//! `Closed` and `EarlyClose{13:00}` — the sheet prints "Early Close - U.S.
//! 1:00 p.m." and nothing after it — and four dates ship `Unsourced`, so the
//! suite pins the scalar kinds, the unsourced refusal semantics, the Sandy
//! alert closure, and the window's end at 2026-12-31 (the operator has
//! published no 2027 schedule).

use chrono::{Days, NaiveDate, TimeZone as _};
use chrono_tz::America;
use exchange_hours::{
    CalendarQueryError, Exchange, ExchangeCalendar, Holiday, HolidayKind, SessionKind,
    calendar_for_exchange,
};

fn nasdaq() -> ExchangeCalendar {
    calendar_for_exchange(Exchange::Nasdaq)
}

fn et(date: (i32, u32, u32), time: (u32, u32, u32)) -> chrono::DateTime<chrono::Utc> {
    America::New_York
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous New York instant")
        .with_timezone(&chrono::Utc)
}

fn day(cell: (i32, u32, u32)) -> NaiveDate {
    NaiveDate::from_ymd_opt(cell.0, cell.1, cell.2).expect("fixture must be a valid date")
}

fn row_of(cell: (i32, u32, u32)) -> Option<HolidayKind> {
    nasdaq().holiday_on(day(cell)).map(Holiday::kind)
}

fn assert_closed(cell: (i32, u32, u32), label: &str) {
    let calendar = nasdaq();
    let date = day(cell);
    assert_eq!(
        row_of(cell),
        Some(HolidayKind::Closed),
        "{label}: {cell:?} carries the closure row"
    );
    // The normal week sources to the 2010-01-01 floor, so every closure date
    // inside the audited window answers through the date-aware surface.
    assert!(
        !calendar
            .is_open(et(cell, (10, 0, 0)))
            .expect("a covered closure date answers"),
        "{label}: {cell:?} is closed mid-morning"
    );
    // The trade-date walk may reach across an adjacent `Unsourced` date (the
    // Sandy Monday's walk reaches the withheld Tuesday), so only a closure
    // whose walk stays inside sourced data asserts the closed trade date.
    if cell != (2012, 10, 29) {
        assert!(
            calendar
                .is_closed_trade_date(date, SessionKind::Both)
                .expect("the closure date is inside the audited window"),
            "{label}: {cell:?} is a closed trade date"
        );
    }
}

fn assert_early_close(cell: (i32, u32, u32), label: &str) {
    let calendar = nasdaq();
    let cutoff = et(cell, (13, 0, 0));
    assert_eq!(
        row_of(cell),
        Some(HolidayKind::EarlyClose {
            close_ssm: 13 * 3_600
        }),
        "{label}: {cell:?} carries the printed 1:00 p.m. close"
    );
    assert!(
        calendar
            .is_open(cutoff - chrono::TimeDelta::nanoseconds(1))
            .expect("a covered early-close date answers"),
        "{label}: open one nanosecond before the close"
    );
    assert!(
        !calendar
            .is_open(cutoff)
            .expect("a covered early-close date answers"),
        "{label}: closed at the close (end-exclusive)"
    );
    assert!(
        !calendar
            .is_open(et(cell, (13, 30, 0)))
            .expect("a covered early-close date answers"),
        "{label}: the sheet states no post-close session, and none answers"
    );
    // The pre-market open (07:00 below 2013-03-18, 04:00 from it) is untouched
    // by a final-close clip.
    assert!(
        calendar
            .is_open(et(cell, (9, 0, 0)))
            .expect("a covered early-close date answers"),
        "{label}: the pre-market session runs to 09:30 as on any day"
    );
}

fn assert_unsourced(cell: (i32, u32, u32), label: &str) {
    let calendar = nasdaq();
    let date = day(cell);
    assert_eq!(
        row_of(cell),
        Some(HolidayKind::Unsourced),
        "{label}: {cell:?} is withheld as Unsourced"
    );
    assert!(
        calendar
            .holiday_coverage()
            .expect("nasdaq ships a table")
            .contains(date),
        "{label}: the withheld date stays inside the audited window"
    );
    // The coverage contract refuses the date instead of claiming it normal or
    // closed: an unsourced arrangement is not an answer. Below the old
    // 2013-03-18 horizon the horizon refusal used to precede this; since the
    // 2026-09-30 floor sourcing the holiday layer's own withholding is what
    // refuses.
    let error = calendar
        .is_open(et(cell, (10, 0, 0)))
        .expect_err("an Unsourced date must not answer as open or closed");
    assert!(
        matches!(error, CalendarQueryError::UnresolvedGap { .. }),
        "the withheld date refuses with UnresolvedGap, got {error:?}"
    );
    let trade = calendar
        .trade_date(et(cell, (10, 0, 0)))
        .expect_err("an Unsourced date must not place a trade date");
    assert!(
        matches!(trade, CalendarQueryError::UnresolvedGap { .. }),
        "the trade-date walk refuses too, got {trade:?}"
    );
}

#[test]
fn the_2012_era_scalar_rows() {
    // The first year the sheet prints the cash market's early close times.
    assert_early_close((2012, 7, 3), "2012 July 3 early close");
    assert_early_close((2012, 11, 23), "2012 day after Thanksgiving");
    assert_early_close((2012, 12, 24), "2012 Christmas Eve");
    assert_closed((2012, 7, 4), "2012 Independence Day");
    assert_closed((2012, 12, 25), "2012 Christmas");
}

#[test]
fn the_two_tba_early_closes_are_withheld() {
    assert_unsourced((2010, 11, 26), "2010 TBA early close");
    assert_unsourced((2011, 11, 25), "2011 TBA early close");
}

#[test]
fn the_sandy_monday_is_sourced_and_the_tuesday_is_withheld() {
    assert_closed((2012, 10, 29), "2012 Sandy Monday (ETA2012-44)");
    assert_unsourced(
        (2012, 10, 30),
        "2012 Sandy Tuesday (confirmation unrecovered)",
    );
    // The markets reopened Wednesday; the row fence answers and, since the
    // 2026-09-30 floor sourcing, so do the 2012 date-aware queries. The
    // Monday's closed-trade-date settlement still refuses: it walks across
    // the withheld Tuesday (UnresolvedGap), the holiday layer's own refusal,
    // and the instant itself sits in no session, so the trade date is None.
    assert_eq!(row_of((2012, 10, 31)), None, "2012-10-31 is audited normal");
    assert!(
        nasdaq()
            .trade_date(et((2012, 10, 29), (10, 0, 0)))
            .expect("the instant is inside the audited window")
            .is_none(),
        "a fully closed day places no trade date"
    );
    let closed = nasdaq()
        .is_closed_trade_date(day((2012, 10, 29)), SessionKind::Both)
        .expect_err("the settlement walk reaches the withheld Tuesday");
    assert!(
        matches!(closed, CalendarQueryError::UnresolvedGap { .. }),
        "the Sandy Monday's trade-date settlement refuses with UnresolvedGap, got {closed:?}"
    );
}

#[test]
fn pre_horizon_rows_fence_data_and_date_aware_queries_answer() {
    // The normal week sources to the 2010-01-01 floor (the operator's own SEC
    // filings and archived Trading Hours page), so a 2012 early-close date the
    // holiday table answers now answers through the date-aware contract too:
    // open mid-morning, closed at the printed 1:00 p.m. edge.
    let calendar = nasdaq();
    assert!(
        calendar
            .is_open(et((2012, 11, 23), (10, 0, 0)))
            .expect("2012-11-23 is inside the sourced span"),
        "the 2012 early close is open mid-morning"
    );
    assert!(
        !calendar
            .is_open(et((2012, 11, 23), (13, 0, 0)))
            .expect("2012-11-23 is inside the sourced span"),
        "the printed 1:00 p.m. close is end-exclusive"
    );
}

#[test]
fn the_2025_2026_rows() {
    assert_closed((2025, 7, 4), "2025 Independence Day");
    assert_early_close((2025, 11, 28), "2025 day after Thanksgiving");
    assert_early_close((2025, 12, 24), "2025 Christmas Eve");
    assert_early_close((2026, 11, 27), "2026 day after Thanksgiving");
    assert_early_close((2026, 12, 24), "2026 Christmas Eve");
    assert_closed((2026, 12, 25), "2026 Christmas");
}

#[test]
fn the_mourning_day_is_withheld_not_claimed_normal() {
    // The operator's own 2025 sheet omits 2025-01-09 and no notice was
    // recoverable; the row refuses the date instead of calling it normal.
    assert_unsourced((2025, 1, 9), "2025 National Day of Mourning");
    assert_eq!(
        row_of((2025, 1, 20)),
        Some(HolidayKind::Closed),
        "MLK 2025 still ships"
    );
}

#[test]
fn every_scalar_early_close_is_the_printed_one_oclock() {
    let calendar = nasdaq();
    let coverage = calendar.holiday_coverage().expect("nasdaq ships a table");
    let mut date = coverage.first();
    let mut closed = 0_u32;
    let mut early = 0_u32;
    let mut unsourced = 0_u32;
    while date <= coverage.last() {
        match calendar.holiday_on(date).map(Holiday::kind) {
            Some(HolidayKind::Closed) => closed += 1,
            Some(HolidayKind::EarlyClose { close_ssm }) => {
                early += 1;
                assert_eq!(
                    close_ssm,
                    13 * 3_600,
                    "every early close is 13:00 ET: {date}"
                );
            }
            Some(HolidayKind::Unsourced) => unsourced += 1,
            Some(other) => panic!("{date} carries {other:?}: the table ships no other kind"),
            None => {}
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the walk stays representable");
    }
    assert_eq!(closed + early + unsourced, 194, "the table ships 194 rows");
    assert_eq!(unsourced, 4, "exactly the four recorded withholdings");
}

#[test]
fn the_window_ends_where_the_operator_stops_publishing() {
    let calendar = nasdaq();
    let coverage = calendar.holiday_coverage().expect("nasdaq ships a table");
    assert_eq!(
        coverage.last(),
        day((2026, 12, 31)),
        "the operator publishes no 2027"
    );
    let error = calendar
        .is_open(et((2027, 1, 4), (10, 0, 0)))
        .expect_err("2027-01-04 is outside the audited window");
    assert!(
        matches!(error, CalendarQueryError::OutsideCoveredRange { .. }),
        "the horizon refusal is OutsideCoveredRange, got {error:?}"
    );
}
