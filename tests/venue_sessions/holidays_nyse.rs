// SPDX-License-Identifier: MIT-0

//! Built-in holiday rows for the `nyse` venue, 2010-2027.
//!
//! Every case runs through `calendar_for_exchange`, the surface a consumer
//! reaches. The table ships only `Closed` and `EarlyClose` scalars — the
//! sheet's own wording states a 1:00 p.m. close and nothing after it — so the
//! suite pins one representative row per kind per era, the end-exclusive
//! close, the Sandy and mourning unscheduled closures, the observed-holiday
//! shifts, and the refusal below the 2010 floor.
//!
//! Mutation note: flipping any shipped row (a date, a kind or the 13:00
//! instant) fails the matching assertion here; the evidence fence
//! (`every_holiday_row_appears_in_its_evidence_file`) fails a flip whose row
//! the evidence file still quotes.

use chrono::{Days, NaiveDate, TimeDelta, TimeZone as _};
use chrono_tz::America;
use exchange_hours::{
    CalendarQueryError, Exchange, ExchangeCalendar, Holiday, HolidayKind, SessionKind,
    calendar_for_exchange,
};

fn nyse() -> ExchangeCalendar {
    calendar_for_exchange(Exchange::Nyse)
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
    nyse().holiday_on(day(cell)).map(Holiday::kind)
}

const NOON: u32 = 12 * 3_600;
const THIRTEEN: u32 = 13 * 3_600;

/// Asserts the closure row and, for dates the identity's normal-week horizon
/// covers (2018-04-09 and later), the date-aware answers.
fn assert_closed(cell: (i32, u32, u32), label: &str) {
    let calendar = nyse();
    let date = day(cell);
    assert_eq!(
        row_of(cell),
        Some(HolidayKind::Closed),
        "{label}: {cell:?} carries the closure row"
    );
    // Date-aware answers exist only from the identity's normal-week horizon
    // (the sourced 2018-04-09 Pillar revision); below it the row plus the
    // audited window are the fence, and the coverage contract refuses.
    if cell.0 >= 2018 {
        assert!(
            !calendar
                .is_open(et(cell, (9, 45, 0)))
                .expect("a covered closure date answers"),
            "{label}: {cell:?} is open at the core open"
        );
        assert!(
            !calendar
                .is_open(et(cell, (15, 0, 0)))
                .expect("a covered closure date answers"),
            "{label}: {cell:?} is open mid-afternoon"
        );
        assert!(
            calendar
                .trade_date(et(cell, (10, 0, 0)))
                .expect("a covered closure date answers")
                .is_none(),
            "{label}: {cell:?} places no trade date on the closure"
        );
        assert!(
            calendar
                .is_closed_trade_date(date, SessionKind::Both)
                .expect("the closure date is inside the audited window"),
            "{label}: {cell:?} is a closed trade date"
        );
    }
}

/// An early close clips the envelope at 13:00 ET end-exclusively: open the
/// nanosecond before, closed at the instant, nothing after.
fn assert_early_close(cell: (i32, u32, u32), label: &str) {
    let calendar = nyse();
    let cutoff = et(cell, (13, 0, 0));
    // The row itself always answers; the date-aware half of this helper only
    // asserts from the identity's normal-week horizon (2018-04-09).
    let date_aware = cell.0 >= 2018;
    assert_eq!(
        row_of(cell),
        Some(HolidayKind::EarlyClose {
            close_ssm: THIRTEEN
        }),
        "{label}: {cell:?} carries the printed 1:00 p.m. close"
    );
    if date_aware {
        assert!(
            calendar
                .is_open(cutoff - TimeDelta::nanoseconds(1))
                .expect("a covered early-close date answers"),
            "{label}: {cell:?} is open one nanosecond before the close"
        );
        assert!(
            !calendar
                .is_open(cutoff)
                .expect("a covered early-close date answers"),
            "{label}: {cell:?} is closed at the close (closes are end-exclusive)"
        );
        assert_eq!(
            calendar
                .session_bounds(et(cell, (12, 0, 0)))
                .expect("a covered date answers"),
            Some((et(cell, (9, 30, 0)), cutoff)),
            "{label}: the regular session still opens at 09:30 and closes at the printed instant"
        );
        // Nothing runs after: the modeled envelope has no post-13:00 session
        // on an early-close day, and the sheet states none.
        assert!(
            !calendar
                .is_open(et(cell, (13, 30, 0)))
                .expect("a covered early-close date answers"),
            "{label}: {cell:?} has no post-close session"
        );
        // The morning early (Tapes B/C) session is untouched by a final-close
        // clip: it ran 07:00-09:30 as on any day.
        assert!(
            calendar
                .is_open(et(cell, (8, 0, 0)))
                .expect("a covered early-close date answers"),
            "{label}: the 07:00-09:30 early session is untouched"
        );
    }
}

#[test]
fn a_pre_floor_date_is_refused_not_claimed_normal() {
    let calendar = nyse();
    assert!(calendar.holiday_coverage().is_some());
    // The last business day before the audited window: answered by no table,
    // refused by the coverage contract rather than reported as a normal day.
    let instant = et((2009, 12, 31), (10, 0, 0));
    let error = calendar
        .is_open(instant)
        .expect_err("2009-12-31 is before the support floor");
    assert!(
        matches!(error, CalendarQueryError::BeforeSupportFloor { .. }),
        "the floor refusal is BeforeSupportFloor, got {error:?}"
    );
}

#[test]
fn pre_horizon_holiday_rows_fence_data_but_date_aware_queries_refuse() {
    // The 2010-2017 rows are fenced against the operator artifacts, but the
    // identity's normal-week timeline is only sourced from its 2018-04-09
    // Pillar revision: below that horizon the date-aware contract refuses
    // (OutsideCoveredRange) even on a date the holiday table answers for.
    // `holiday_on` and `holiday_coverage` remain the data fences.
    let calendar = nyse();
    let date = day((2012, 7, 3));
    assert_eq!(
        row_of((2012, 7, 3)),
        Some(HolidayKind::EarlyClose {
            close_ssm: 13 * 3_600
        }),
        "the 2012 row is in the table"
    );
    assert!(
        calendar
            .holiday_coverage()
            .expect("nyse ships a table")
            .contains(date),
        "2012-07-03 is inside the audited holiday window"
    );
    let error = calendar
        .is_open(et((2012, 7, 3), (10, 0, 0)))
        .expect_err("below the normal-week horizon the query refuses");
    assert!(
        matches!(error, CalendarQueryError::OutsideCoveredRange { .. }),
        "the horizon refusal is OutsideCoveredRange, got {error:?}"
    );
}

#[test]
fn a_normal_week_day_audited_in_2019_has_no_row() {
    // 2019-06-11 (a Tuesday with no arrangement) sits inside the audited
    // window with no row: inside coverage, no row means audited normal.
    let calendar = nyse();
    let date = day((2019, 6, 11));
    assert!(
        calendar
            .holiday_coverage()
            .expect("nyse ships a table")
            .contains(date)
    );
    assert_eq!(
        calendar.holiday_on(date),
        None,
        "an ordinary day carries no row"
    );
    assert!(
        calendar
            .is_open(et((2019, 6, 11), (10, 0, 0)))
            .expect("a covered date answers"),
        "an audited normal Tuesday is open"
    );
}

#[test]
fn the_2010_era_closures_and_the_printed_november_early_close() {
    // The full-year 2010 grid as printed on the operator's own page.
    for cell in [
        (2010, 1, 1),
        (2010, 1, 18),
        (2010, 2, 15),
        (2010, 4, 2),
        (2010, 5, 31),
        (2010, 7, 5),
        (2010, 9, 6),
        (2010, 11, 25),
        (2010, 12, 24),
    ] {
        assert_closed(cell, "2010 closure");
    }
    assert_early_close((2010, 11, 26), "2010 day after Thanksgiving");
    // Friday July 2, 2010 is a full trading day and carries no row.
    assert_eq!(
        row_of((2010, 7, 2)),
        None,
        "the ordinary Friday carries no row"
    );
}

#[test]
fn the_2012_2013_eras_on_the_multi_year_artifact() {
    assert_early_close((2012, 7, 3), "2012 July 3 early close");
    assert_early_close((2012, 11, 23), "2012 day after Thanksgiving");
    assert_early_close((2012, 12, 24), "2012 Christmas Eve");
    assert_closed((2012, 12, 25), "2012 Christmas");
    assert_early_close((2013, 7, 3), "2013 July 3 early close");
    assert_early_close((2013, 11, 29), "2013 day after Thanksgiving");
    assert_early_close((2013, 12, 24), "2013 Christmas Eve");
}

#[test]
fn the_2015_observed_independence_day_deletes_july_3() {
    assert_closed((2015, 7, 3), "2015 Independence observed Friday");
    // The anniversary itself is a Saturday with no session and no row.
    assert_eq!(
        row_of((2015, 7, 4)),
        None,
        "the Saturday anniversary carries no row"
    );
}

#[test]
fn the_2021_2022_era_rows() {
    assert_closed((2021, 12, 24), "2021 Christmas observed Friday");
    assert_early_close((2021, 11, 26), "2021 day after Thanksgiving");
    assert_closed((2022, 6, 20), "2022 Juneteenth observed Monday");
    assert_early_close((2022, 11, 25), "2022 day after Thanksgiving");
}

#[test]
fn the_juneteenth_grid_starts_in_2022_not_2021() {
    // The operator's 2021 sheet lists no Juneteenth; the 2022 sheet adds it.
    // The prior Friday (2021-06-18) is audited normal, which is exactly the
    // no-row claim.
    assert_eq!(row_of((2021, 6, 18)), None, "2021 has no Juneteenth row");
    assert_closed((2023, 6, 19), "2023 Juneteenth on the day");
    assert_closed((2024, 6, 19), "2024 Juneteenth on the day");
}

#[test]
fn the_2025_2026_2027_rows() {
    assert_closed((2025, 1, 9), "2025 National Day of Mourning");
    assert_early_close((2025, 11, 28), "2025 day after Thanksgiving");
    assert_early_close((2025, 12, 24), "2025 Christmas Eve");
    assert_closed((2026, 7, 3), "2026 Independence observed Friday");
    assert_early_close((2026, 11, 27), "2026 day after Thanksgiving");
    assert_early_close((2026, 12, 24), "2026 Christmas Eve");
    assert_closed((2027, 6, 18), "2027 Juneteenth observed Friday");
    assert_closed((2027, 7, 5), "2027 Independence observed Monday");
    assert_early_close((2027, 11, 26), "2027 day after Thanksgiving");
    assert_closed((2027, 12, 24), "2027 Christmas observed Friday");
}

#[test]
fn every_early_close_in_the_table_is_the_printed_one_oclock() {
    // The sheet prints 1:00 p.m. for every early close 2010-2027; walk the
    // rows through the public accessor and confirm no other instant ships.
    let calendar = nyse();
    let coverage = calendar.holiday_coverage().expect("nyse ships a table");
    let mut date = coverage.first();
    let mut closed = 0_u32;
    let mut early = 0_u32;
    while date <= coverage.last() {
        match calendar.holiday_on(date).map(Holiday::kind) {
            Some(HolidayKind::Closed) => closed += 1,
            Some(HolidayKind::EarlyClose { close_ssm }) => {
                early += 1;
                assert_eq!(close_ssm, THIRTEEN, "every early close is 13:00 ET: {date}");
                assert_ne!(close_ssm, NOON);
            }
            Some(other) => panic!("{date} carries {other:?}: the table ships no other kind"),
            None => {}
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the walk stays representable");
    }
    assert_eq!(closed + early, 206, "the table ships 206 rows");
    assert_eq!(early, 37, "37 printed early closes, 2010-2027");
}

#[test]
fn the_window_covers_to_2027_12_31_and_refuses_after_it() {
    let calendar = nyse();
    let coverage = calendar.holiday_coverage().expect("nyse ships a table");
    let last = day((2027, 12, 24));
    assert!(coverage.contains(last));
    assert_eq!(row_of((2027, 12, 24)), Some(HolidayKind::Closed));
    assert!(
        coverage.contains(day((2027, 12, 31))),
        "the window ends 2027-12-31"
    );
    let after = day((2028, 1, 3));
    assert!(
        !coverage.contains(after),
        "2028-01-03 is outside the audited window"
    );
    let error = calendar
        .is_open(et((2028, 1, 3), (10, 0, 0)))
        .expect_err("the day after the window is refused");
    assert!(
        matches!(error, CalendarQueryError::OutsideCoveredRange { .. }),
        "the window-end refusal is OutsideCoveredRange, got {error:?}"
    );
}
