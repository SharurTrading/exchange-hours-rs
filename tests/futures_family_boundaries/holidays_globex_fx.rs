// SPDX-License-Identifier: MIT-0

//! `globex_fx` built-in holiday rows, 2025-2027, through the public surface.
//!
//! The family's grid is one wrapping Sunday-to-Thursday 17:00→16:00 CT matching
//! block, so every trading day opens on the previous local evening and every
//! row below is keyed by the crate's own venue-local trade date, never by the
//! operator's event date. Probes are stated in `America/Chicago` wall clock and
//! converted, because that is the clock the operator publishes in and the only
//! one in which a 12:45 CT close is recognisable.
//!
//! The design memo's §4.1 asks for seven cases per family. Six of them have
//! real rows here; the seventh — a late open — has none, because CME never
//! reopens standard-grid FX later than its normal 17:00 CT in this window. That
//! absence is fenced rather than skipped: every shipped row's kind is asserted,
//! so a late open cannot appear without a test changing.

use chrono::{DateTime, Days, NaiveDate, TimeDelta, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, EvidenceTier, ExchangeCalendar, HolidayKind, MarketHoursKey, SessionKind,
    calendar_for_market_hours_key,
};

/// The family under test, as a date-aware calendar.
fn fx() -> ExchangeCalendar {
    calendar_for_market_hours_key(MarketHoursKey::GlobexFx)
}

fn day(year: i32, month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
}

/// A probe instant, stated in the venue's own wall clock and converted.
fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous CT instant")
        .with_timezone(&Utc)
}

/// Every trade date the module ships a row for, with the kind it ships.
///
/// This is the handwritten fence the charter asks for: it is compared against
/// the shipped table, never generated from it, so a row that appears, vanishes
/// or changes kind fails here.
fn shipped_rows() -> Vec<(NaiveDate, HolidayKind)> {
    let early = |hour: u32, minute: u32| HolidayKind::EarlyClose {
        close_ssm: hour * 3_600 + minute * 60,
    };
    vec![
        (day(2025, 1, 1), HolidayKind::Closed),
        (day(2025, 4, 18), HolidayKind::Closed),
        (day(2025, 7, 4), early(12, 0)),
        (day(2025, 11, 28), early(13, 45)),
        (day(2025, 11, 29), HolidayKind::Closed),
        (day(2025, 12, 24), early(12, 45)),
        (day(2025, 12, 25), HolidayKind::Closed),
        (day(2026, 1, 1), HolidayKind::Closed),
        (day(2026, 4, 3), early(10, 15)),
        (day(2026, 6, 19), early(12, 0)),
        (day(2026, 7, 3), early(12, 0)),
        (day(2026, 11, 27), early(13, 45)),
        (day(2026, 12, 24), early(12, 45)),
        (day(2026, 12, 25), HolidayKind::Closed),
        (day(2027, 1, 1), HolidayKind::Closed),
        (day(2027, 3, 26), HolidayKind::Closed),
        (day(2027, 6, 18), early(12, 0)),
        (day(2027, 11, 26), early(13, 45)),
        (day(2027, 12, 24), HolidayKind::Closed),
    ]
}

// ---------------------------------------------------------------------------
// Case 1 — a closed day.
// ---------------------------------------------------------------------------

/// Christmas 2026 falls on a Friday, so the closure removes the family's whole
/// civil day: the leg that fed it opened Thursday evening and there is no
/// Friday-evening leg on this grid to survive it.
#[test]
fn christmas_2026_closes_the_whole_trade_date_and_its_previous_evening() {
    let calendar = fx();
    let holiday = calendar
        .holiday_on(day(2026, 12, 25))
        .expect("2026-12-25 ships a row");
    assert_eq!(holiday.kind(), HolidayKind::Closed);
    assert_eq!(holiday.tier(), EvidenceTier::T2);
    assert_eq!(holiday.document_id(), "CME-SVC-2026-12-24");

    assert!(calendar.is_closed_trade_date(day(2026, 12, 25), SessionKind::Both));
    assert!(calendar.is_closed_all_day_on(day(2026, 12, 25), SessionKind::Both));
    for probe in [(2, 0, 0), (10, 0, 0), (20, 0, 0)] {
        let instant = ct((2026, 12, 25), probe);
        assert!(
            !calendar.is_open(instant),
            "Christmas Day must be closed at {instant}"
        );
        assert_eq!(calendar.trade_date(instant), None);
    }

    // The previous evening's normal 17:00 CT open fed trade date 2026-12-25, so
    // it is gone with it.
    assert!(!calendar.is_open(ct((2026, 12, 24), (17, 30, 0))));
    // From the eve's morning the next open is Sunday's reopen, two civil days
    // past the holiday.
    assert_eq!(
        calendar.next_session_open_after(ct((2026, 12, 24), (8, 0, 0))),
        Some(ct((2026, 12, 27), (17, 0, 0)))
    );
}

// ---------------------------------------------------------------------------
// Cases 2 and 3 — an early close, on each side of its cutoff.
// ---------------------------------------------------------------------------

/// The day after Thanksgiving 2025 closes at 13:45 CT. The clip is stated on
/// the trade date, so it lands on Friday even though the session opened on
/// Thursday evening.
#[test]
fn the_2025_thanksgiving_friday_closes_at_1345_ct() {
    let calendar = fx();
    assert_eq!(
        calendar
            .holiday_on(day(2025, 11, 28))
            .map(exchange_hours::Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 13 * 3_600 + 45 * 60
        })
    );

    // Before the cutoff.
    assert!(calendar.is_open(ct((2025, 11, 28), (13, 44, 59))));
    // At the cutoff: closes are end-exclusive.
    assert!(!calendar.is_open(ct((2025, 11, 28), (13, 45, 0))));
    // And after it, for the rest of the civil day.
    assert!(!calendar.is_open(ct((2025, 11, 28), (15, 0, 0))));
    assert!(!calendar.is_open(ct((2025, 11, 28), (18, 0, 0))));

    // The trading day runs from Thursday's 17:00 CT open to the cutoff.
    assert_eq!(
        calendar.session_bounds(ct((2025, 11, 28), (10, 0, 0))),
        Some((
            ct((2025, 11, 27), (17, 0, 0)),
            ct((2025, 11, 28), (13, 45, 0))
        ))
    );
    assert_eq!(
        calendar.candle_end(ct((2025, 11, 28), (10, 0, 0)), CalendarResolution::Daily),
        Some(ct((2025, 11, 28), (13, 45, 0)))
    );

    // Thanksgiving Thursday itself ships no row for this family — CME moved its
    // trade date, not its matching phases — so the Wednesday-evening leg that
    // feeds trade date 2025-11-27 is untouched.
    assert!(calendar.is_open(ct((2025, 11, 27), (10, 0, 0))));
}

/// Good Friday 2026 is the exception CME's own page flags: this family traded
/// that morning and stopped at 10:15 CT, where the full Globex closure of Good
/// Friday 2025 and 2027 removes the day outright.
#[test]
fn good_friday_2026_closes_at_1015_ct_while_2025_and_2027_are_shut() {
    let calendar = fx();
    assert!(calendar.is_open(ct((2026, 4, 3), (10, 14, 59))));
    assert!(!calendar.is_open(ct((2026, 4, 3), (10, 15, 0))));
    assert_eq!(
        calendar.candle_end(ct((2026, 4, 3), (6, 0, 0)), CalendarResolution::Daily),
        Some(ct((2026, 4, 3), (10, 15, 0)))
    );

    for closed in [day(2025, 4, 18), day(2027, 3, 26)] {
        assert!(calendar.is_closed_trade_date(closed, SessionKind::Both));
        assert!(calendar.is_closed_all_day_on(closed, SessionKind::Both));
    }
}

// ---------------------------------------------------------------------------
// Case 4 — a late open. This family ships none, and that is fenced.
// ---------------------------------------------------------------------------

/// CME never reopens standard-grid FX later than its normal 17:00 CT in this
/// window, so no row is a late open and the post-closure reopen is the ordinary
/// one. Asserting every shipped kind is what keeps that true: a late open added
/// without a test fails here.
#[test]
fn the_family_ships_no_late_open_and_reopens_at_the_normal_1700_ct() {
    let calendar = fx();
    // Walk the whole coverage window, so a row on a date the fence does not
    // name fails here too: the shipped rows are exactly the handwritten list.
    let coverage = calendar
        .holiday_coverage()
        .expect("this family ships a table");
    let mut walked = Vec::new();
    // The handwritten fence below is the 2025-2027 block; the 2010-2012 rows
    // are fenced in this file's `era_` tests.
    let mut date = day(2025, 1, 1);
    let modern_last = day(2027, 12, 31);
    let _ = coverage.last();
    while date <= modern_last {
        if let Some(row) = calendar.holiday_on(date) {
            walked.push((date, row.kind()));
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the coverage window stays inside the representable calendar");
    }
    assert_eq!(
        walked,
        shipped_rows(),
        "the table's rows over its whole window are exactly the handwritten fence"
    );
    for (date, kind) in shipped_rows() {
        let row = calendar
            .holiday_on(date)
            .unwrap_or_else(|| panic!("{date} must ship a row"));
        assert_eq!(row.kind(), kind, "{date} ships the wrong kind");
        assert!(
            matches!(
                row.kind(),
                HolidayKind::Closed | HolidayKind::EarlyClose { .. }
            ),
            "{date}: this family ships only closures and early closes"
        );
        assert_eq!(row.tier(), EvidenceTier::T2, "{date} must be sourced at T2");
    }

    // Christmas Day 2025 is closed, and the evening of the holiday reopens for
    // the next trade date at the normal hour, not later.
    assert!(!calendar.is_open(ct((2025, 12, 25), (16, 59, 59))));
    assert!(calendar.is_open(ct((2025, 12, 25), (17, 0, 0))));
    // The same after the Good Friday 2027 closure: Sunday's normal reopen.
    assert_eq!(
        calendar.next_session_open_after(ct((2027, 3, 25), (17, 30, 0))),
        Some(ct((2027, 3, 28), (17, 0, 0)))
    );
}

// ---------------------------------------------------------------------------
// Case 5 — a wrap removed by a closure.
// ---------------------------------------------------------------------------

/// Christmas Eve 2025 closes at 12:45 CT and Christmas Day is closed, so the
/// evening leg that would have carried trade date 2025-12-25 is deleted by the
/// closure rather than clipped by the early close.
#[test]
fn the_christmas_2025_closure_removes_the_eves_evening_wrap() {
    let calendar = fx();
    assert!(calendar.is_open(ct((2025, 12, 24), (12, 44, 59))));
    assert!(!calendar.is_open(ct((2025, 12, 24), (12, 45, 0))));
    // The eve's own trading day opened Tuesday evening and ends at the cutoff.
    assert_eq!(
        calendar.session_bounds(ct((2025, 12, 24), (9, 0, 0))),
        Some((
            ct((2025, 12, 23), (17, 0, 0)),
            ct((2025, 12, 24), (12, 45, 0))
        ))
    );
    // No session at 17:30 CT on the eve: that leg's trade date is closed.
    assert!(!calendar.is_open(ct((2025, 12, 24), (17, 30, 0))));
    // So the next open is Christmas evening, for trade date 2025-12-26.
    assert_eq!(
        calendar.next_session_open_after(ct((2025, 12, 24), (12, 50, 0))),
        Some(ct((2025, 12, 25), (17, 0, 0)))
    );
    // Christmas Day is not closed *all day* — the next trade date's session
    // opens inside it. `is_closed_trade_date` is the holiday question.
    assert!(calendar.is_closed_trade_date(day(2025, 12, 25), SessionKind::Both));
    assert!(!calendar.is_closed_all_day_on(day(2025, 12, 25), SessionKind::Both));
}

// ---------------------------------------------------------------------------
// Case 6 — the trade-date consequence.
// ---------------------------------------------------------------------------

/// A shortened day keeps its own trade date; the evening leg that opens on a
/// closed holiday carries the *post*-holiday date.
#[test]
fn the_holiday_rows_keep_the_trade_dates_the_operator_prints() {
    let calendar = fx();
    // Inside the shortened Thanksgiving-Friday day.
    assert_eq!(
        calendar.trade_date(ct((2025, 11, 28), (10, 0, 0))),
        Some(day(2025, 11, 28))
    );
    // The eve's evening open, on the closed holiday itself.
    assert_eq!(
        calendar.trade_date(ct((2025, 12, 25), (18, 0, 0))),
        Some(day(2025, 12, 26))
    );
    // Juneteenth 2026: the Thursday-evening leg is clipped at Friday noon and
    // still belongs to the crate's Friday trade date.
    assert_eq!(
        calendar.trade_date(ct((2026, 6, 18), (20, 0, 0))),
        Some(day(2026, 6, 19))
    );
    assert!(calendar.is_open(ct((2026, 6, 19), (11, 59, 59))));
    assert!(!calendar.is_open(ct((2026, 6, 19), (12, 0, 0))));
}

// ---------------------------------------------------------------------------
// Case 7 — both edges of the coverage window.
// ---------------------------------------------------------------------------

/// Inside the window a date with no row is audited normal; outside it the table
/// has no answer at all, and a known CME holiday one day — or one year — beyond
/// the edge is not applied.
#[test]
fn the_table_answers_inside_its_window_and_nowhere_else() {
    let calendar = fx();
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a coverage window");
    assert_eq!(coverage.first(), day(2010, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(coverage.contains(day(2010, 1, 1)));
    assert!(coverage.contains(day(2027, 12, 31)));
    assert!(!coverage.contains(day(2009, 12, 31)));
    assert!(!coverage.contains(day(2028, 1, 1)));

    assert_eq!(calendar.holiday_on(day(2009, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2028, 1, 1)), None);
    // Christmas 2009 is a real CME closure one year below the window. The table
    // does not silently extend to it.
    assert_eq!(calendar.holiday_on(day(2009, 12, 25)), None);
    let bare = calendar.without_holidays();
    assert!(calendar.is_open(ct((2009, 12, 25), (10, 0, 0))));
    assert_eq!(
        calendar.is_open(ct((2009, 12, 25), (10, 0, 0))),
        bare.is_open(ct((2009, 12, 25), (10, 0, 0)))
    );
    // And the first day above the window answers the pure normal-week answer.
    assert_eq!(
        calendar.is_open(ct((2028, 1, 3), (10, 0, 0))),
        bare.is_open(ct((2028, 1, 3), (10, 0, 0)))
    );
}

// ---------------------------------------------------------------------------
// `without_holidays` restores the normal-week answer.
// ---------------------------------------------------------------------------

/// Detaching the table gives back exactly the pre-table calendar, and the
/// difference it makes on a holiday week is the rows themselves.
#[test]
fn without_holidays_restores_the_normal_week() {
    let calendar = fx();
    let bare = calendar.without_holidays();
    assert_eq!(bare.holiday_coverage(), None);
    assert_eq!(bare.holiday_on(day(2026, 12, 25)), None);

    // The normal week has a Thursday-evening leg into Christmas Friday 2026 and
    // a full Thursday day session; the table removes both.
    assert!(bare.is_open(ct((2026, 12, 25), (10, 0, 0))));
    assert!(!calendar.is_open(ct((2026, 12, 25), (10, 0, 0))));
    assert!(bare.is_open(ct((2026, 12, 24), (15, 0, 0))));
    assert!(!calendar.is_open(ct((2026, 12, 24), (15, 0, 0))));
    assert_eq!(
        bare.candle_end(ct((2026, 12, 24), (9, 0, 0)), CalendarResolution::Daily),
        Some(ct((2026, 12, 24), (16, 0, 0)))
    );

    // Away from the rows the two agree instant for instant. 2026-10-18 ..
    // 2026-10-24 is CME's own reference week for this family.
    let mut instant = ct((2026, 10, 18), (0, 0, 0));
    let end = ct((2026, 10, 25), (0, 0, 0));
    while instant < end {
        assert_eq!(
            calendar.is_open(instant),
            bare.is_open(instant),
            "is_open diverged at {instant} on an ordinary week"
        );
        assert_eq!(
            calendar.trade_date(instant),
            bare.trade_date(instant),
            "trade_date diverged at {instant} on an ordinary week"
        );
        assert_eq!(
            calendar.session_bounds(instant),
            bare.session_bounds(instant),
            "session_bounds diverged at {instant} on an ordinary week"
        );
        instant += TimeDelta::minutes(37);
    }
}

// ---------------------------------------------------------------------------
// The 2010-2012 rows.
// ---------------------------------------------------------------------------

/// 12:00 CT, the era's Monday/Thursday-holiday FX final close.
const ERA_NOON: u32 = 12 * 3_600;
/// 10:15 CT, the era's Good Friday FX final close.
const ERA_GOOD_FRIDAY_CLOSE: u32 = 10 * 3_600 + 15 * 60;
/// 05:00 CT, the era's post-holiday first open.
const ERA_REOPEN: u32 = 5 * 3_600;

/// The era's early closes clip a trading day that opened 17:00 CT the previous
/// evening: Martin Luther King Jr. Day 2010 stops at 12:00 CT and reopens the
/// same civil evening for the next trade date, and Good Friday 2010 stops at
/// 10:15 CT — the instant the 2026 table reuses for the same holiday.
#[test]
fn era_early_closes_end_the_wrapped_trading_day_at_the_stated_instant() {
    let calendar = fx();

    let mlk = calendar
        .holiday_on(day(2010, 1, 18))
        .expect("2010-01-18 ships a row");
    assert_eq!(
        mlk.kind(),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON
        }
    );
    assert_eq!(mlk.tier(), EvidenceTier::T1);
    assert_eq!(
        mlk.document_id(),
        "2010-martin-luther-king.pdf @2010-03-31T06:42:26Z"
    );
    // The Sunday-evening leg that feeds this trade date is clipped, not
    // deleted: it still opens.
    assert!(calendar.is_open(ct((2010, 1, 17), (17, 0, 0))));
    assert!(calendar.is_open(ct((2010, 1, 18), (11, 59, 59))));
    assert!(!calendar.is_open(ct((2010, 1, 18), (12, 0, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2010, 1, 18), (9, 0, 0))),
        Some((ct((2010, 1, 17), (17, 0, 0)), ct((2010, 1, 18), (12, 0, 0))))
    );
    // 17:00 CT the same civil evening opens the next trade date, which is
    // normal: this family publishes no holiday-evening row.
    assert_eq!(
        calendar.trade_date(ct((2010, 1, 18), (18, 0, 0))),
        Some(day(2010, 1, 19))
    );

    let good_friday = calendar
        .holiday_on(day(2010, 4, 2))
        .expect("2010-04-02 ships a row");
    assert_eq!(
        good_friday.kind(),
        HolidayKind::EarlyClose {
            close_ssm: ERA_GOOD_FRIDAY_CLOSE
        }
    );
    assert!(calendar.is_open(ct((2010, 4, 2), (10, 14, 59))));
    assert!(!calendar.is_open(ct((2010, 4, 2), (10, 15, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2010, 4, 2), (9, 0, 0))),
        Some((ct((2010, 4, 1), (17, 0, 0)), ct((2010, 4, 2), (10, 15, 0))))
    );
    assert_eq!(
        calendar.next_session_open_after(ct((2010, 4, 2), (10, 20, 0))),
        Some(ct((2010, 4, 4), (17, 0, 0)))
    );
}

/// Good Friday 2011 ships **no row**: the table carries 2010-04-02 and
/// 2012-04-06 at 10:15 CT but nothing for 2011-04-22, so the crate serves that
/// Good Friday as an ordinary full Friday, 17:00 CT Thursday to 16:00 CT
/// Friday. FINDING, asserted rather than glossed: this is a hole in the
/// widened block, not a deliberate normal, and the detached calendar agreeing
/// shows the gap is the row set and not the profile.
#[test]
fn era_good_friday_2011_ships_no_row_and_is_audited_normal() {
    let calendar = fx();
    let bare = calendar.without_holidays();

    assert_eq!(
        calendar.holiday_on(day(2011, 4, 22)),
        None,
        "2011-04-22 ships no row in the shipped table"
    );
    assert_eq!(
        calendar.session_bounds(ct((2011, 4, 22), (9, 0, 0))),
        Some((ct((2011, 4, 21), (17, 0, 0)), ct((2011, 4, 22), (16, 0, 0))))
    );
    assert!(calendar.is_open(ct((2011, 4, 22), (10, 15, 0))));
    assert!(calendar.is_open(ct((2011, 4, 22), (15, 59, 59))));
    assert!(!calendar.is_open(ct((2011, 4, 22), (16, 0, 0))));
    assert_eq!(
        calendar.is_open(ct((2011, 4, 22), (10, 15, 0))),
        bare.is_open(ct((2011, 4, 22), (10, 15, 0)))
    );

    // The two Good Fridays either side of it do ship, at the same instant, so
    // the missing row is a gap in the block rather than a family that never
    // closes early on this holiday.
    for date in [(2010, 4, 2), (2012, 4, 6)] {
        assert!(
            matches!(
                calendar
                    .holiday_on(day(date.0, date.1, date.2))
                    .unwrap_or_else(|| panic!("{date:?} ships a row"))
                    .kind(),
                HolidayKind::EarlyClose {
                    close_ssm: ERA_GOOD_FRIDAY_CLOSE
                }
            ),
            "{date:?} must ship the era's 10:15 CT Good Friday close"
        );
    }
}

/// The era's late opens state 05:00 CT — earlier than the family's normal
/// 17:00 CT first open — so each cutoff lands on the trade date itself: the
/// leg that would have opened the Monday evening did not run, and trade date
/// 2011-12-27 begins on its own civil day.
#[test]
fn era_late_opens_land_on_the_trade_date_itself() {
    let calendar = fx();

    let holiday = calendar
        .holiday_on(day(2011, 12, 27))
        .expect("2011-12-27 ships a row");
    assert_eq!(
        holiday.kind(),
        HolidayKind::LateOpen {
            open_ssm: ERA_REOPEN
        }
    );
    assert_eq!(
        holiday.document_id(),
        "2011-christmas.pdf @2012-01-25T02:05:48Z"
    );
    assert!(!calendar.is_open(ct((2011, 12, 26), (17, 0, 0))));
    assert!(!calendar.is_open(ct((2011, 12, 27), (4, 59, 59))));
    assert!(calendar.is_open(ct((2011, 12, 27), (5, 0, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2011, 12, 27), (5, 0, 0))),
        Some((
            ct((2011, 12, 27), (5, 0, 0)),
            ct((2011, 12, 27), (16, 0, 0))
        ))
    );
    assert_eq!(
        calendar.trade_date(ct((2011, 12, 27), (9, 0, 0))),
        Some(day(2011, 12, 27))
    );
    // The next ordinary evening leg carries the next trade date.
    assert_eq!(
        calendar.trade_date(ct((2011, 12, 27), (18, 0, 0))),
        Some(day(2011, 12, 28))
    );

    // The era's three late opens are all the same 05:00 CT shape, so none can
    // take the preceding-local-date branch a 17:00 CT-or-later statement would.
    for (year, month, date) in [(2011, 12, 27), (2012, 1, 3), (2012, 12, 26)] {
        let row = calendar
            .holiday_on(day(year, month, date))
            .unwrap_or_else(|| panic!("{year}-{month:02}-{date:02} ships a row"));
        assert!(matches!(
            row.kind(),
            HolidayKind::LateOpen {
                open_ssm: ERA_REOPEN
            }
        ));
    }
}

/// A date inside the widened window with no row is audited normal: the crate
/// serves the profile's own 2010 week, and the detached calendar agrees, so
/// the answer comes from the row set and not from a profile change.
#[test]
fn era_dates_without_rows_are_audited_normal() {
    let calendar = fx();
    let bare = calendar.without_holidays();

    for (date, previous_day, time) in [
        ((2010, 6, 15), (2010, 6, 14), (10, 0, 0)),
        ((2011, 3, 9), (2011, 3, 8), (9, 0, 0)),
        ((2012, 10, 10), (2012, 10, 9), (14, 0, 0)),
    ] {
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "{date:?} must ship no row"
        );
        for probe in [(previous_day, (17, 0, 0)), (date, time)] {
            assert!(
                calendar.is_open(ct(probe.0, probe.1)),
                "{:?} {:?} CT is an ordinary trading instant",
                probe.0,
                probe.1
            );
            assert_eq!(
                calendar.is_open(ct(probe.0, probe.1)),
                bare.is_open(ct(probe.0, probe.1)),
                "the row set must not change {:?}",
                probe.0
            );
        }
    }
}

/// The widened window's edges answer as the module declares: 2010-01-01 to
/// 2027-12-31, with Christmas Day 2009 — a real CME closure one year below it —
/// not applied and the detached calendar agreeing.
#[test]
fn era_window_edges_answer_as_the_module_declares() {
    let calendar = fx();
    let bare = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_fx ships a table");

    assert_eq!(coverage.first(), day(2010, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(!coverage.contains(day(2009, 12, 25)));

    assert_eq!(calendar.holiday_on(day(2009, 12, 25)), None);
    assert!(calendar.is_open(ct((2009, 12, 25), (10, 0, 0))));
    assert_eq!(
        calendar.is_open(ct((2009, 12, 25), (10, 0, 0))),
        bare.is_open(ct((2009, 12, 25), (10, 0, 0)))
    );
}
