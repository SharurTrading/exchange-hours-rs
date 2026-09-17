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

use chrono::{DateTime, Datelike as _, Days, NaiveDate, TimeDelta, TimeZone as _, Utc, Weekday};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, EvidenceTier, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey,
    SessionKind, calendar_for_market_hours_key,
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

/// 15:15 CT, the era's Friday-holiday-eve final close.
const ERA_FRIDAY_EVE_CLOSE: u32 = 15 * 3_600 + 15 * 60;
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

    // The MLK Friday eve, 2010-01-15, states the era's 15:15 CT early close.
    // It is asserted first because it is the row a mutation of any other
    // instant in this block would leave untouched: an unfenced row is one a
    // later edit can silently corrupt.
    let eve = calendar
        .holiday_on(day(2010, 1, 15))
        .expect("2010-01-15 ships a row");
    assert_eq!(
        eve.kind(),
        HolidayKind::EarlyClose {
            close_ssm: ERA_FRIDAY_EVE_CLOSE
        }
    );
    assert_eq!(eve.tier(), EvidenceTier::T1);
    assert_eq!(
        eve.document_id(),
        "2010-martin-luther-king.pdf @2010-03-31T06:42:26Z"
    );
    assert!(calendar.is_open(ct((2010, 1, 15), (15, 14, 59))));
    assert!(!calendar.is_open(ct((2010, 1, 15), (15, 15, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2010, 1, 15), (9, 0, 0))),
        Some((
            ct((2010, 1, 14), (17, 0, 0)),
            ct((2010, 1, 15), (15, 15, 0))
        ))
    );

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

/// Good Friday 2011 is a **full Globex closure**, unlike 2010 and 2012: CME's
/// 2011 sheet prints `CME Globex is closed` with no early close, so the row is
/// `Closed(2011-04-22)` and it removes the trading day that began Thursday
/// evening, exactly as the operator states.
#[test]
fn era_good_friday_2011_is_a_full_closure() {
    let calendar = fx();

    assert_eq!(
        calendar
            .holiday_on(day(2011, 4, 22))
            .expect("2011-04-22 ships a row")
            .kind(),
        HolidayKind::Closed
    );
    assert!(!calendar.is_open(ct((2011, 4, 21), (17, 0, 0))));
    assert!(!calendar.is_open(ct((2011, 4, 22), (9, 0, 0))));
    assert!(!calendar.is_open(ct((2011, 4, 22), (15, 0, 0))));
    // Nothing trades on the Friday evening either: CME prints the next open as
    // Sunday 2011-04-24 at 17:00 CT for trade date Monday 2011-04-25.
    assert!(!calendar.is_open(ct((2011, 4, 22), (18, 0, 0))));
    assert_eq!(
        calendar.trade_date(ct((2011, 4, 24), (18, 0, 0))),
        Some(day(2011, 4, 25))
    );
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

// ---------------------------------------------------------------------------
// The 2016-2018 rows.
// ---------------------------------------------------------------------------

/// The era's early closes clip the wrapped trading day at the printed instant,
/// and the three Independence Day eves CME prints no row for stay ordinary
/// trading days.
#[test]
fn wave2_early_closes_end_the_wrapped_trading_day_at_the_printed_instant() {
    let calendar = fx();

    for (date, previous_day, close_ssm) in [
        ((2016, 2, 15), (2016, 2, 14), 12 * 3_600),
        ((2017, 11, 23), (2017, 11, 22), 12 * 3_600),
        ((2016, 11, 25), (2016, 11, 24), 12 * 3_600 + 15 * 60),
        ((2018, 12, 24), (2018, 12, 23), 12 * 3_600 + 15 * 60),
    ] {
        assert_eq!(
            calendar
                .holiday_on(day(date.0, date.1, date.2))
                .map(Holiday::kind),
            Some(HolidayKind::EarlyClose { close_ssm }),
            "{date:?}"
        );
        let cutoff = ct(date, (close_ssm / 3_600, (close_ssm % 3_600) / 60, 0));
        assert!(calendar.is_open(ct(previous_day, (17, 0, 0))), "{date:?}");
        assert!(calendar.is_open(cutoff - TimeDelta::seconds(1)), "{date:?}");
        assert!(!calendar.is_open(cutoff), "{date:?}: end-exclusive");
        assert_eq!(
            calendar.candle_end(ct(date, (9, 0, 0)), CalendarResolution::Daily),
            Some(cutoff),
            "{date:?}"
        );
    }

    // 2017-07-03 and 2018-07-03 are ordinary 16:00 CT days for this family.
    for date in [(2017, 7, 3), (2018, 7, 3)] {
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "{date:?}"
        );
        assert!(calendar.is_open(ct(date, (15, 30, 0))), "{date:?}");
    }
}

/// A closure removes its trade date and the prior-evening leg, and the era
/// ships no late open.
#[test]
fn wave2_closures_remove_the_trade_date_and_ship_no_late_open() {
    let calendar = fx();

    assert_eq!(
        calendar.holiday_on(day(2016, 3, 25)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(!calendar.is_open(ct((2016, 3, 24), (18, 0, 0))));
    assert!(!calendar.is_open(ct((2016, 3, 25), (10, 0, 0))));
    assert_eq!(
        calendar.next_session_open_after(ct((2016, 3, 25), (10, 0, 0))),
        Some(ct((2016, 3, 27), (17, 0, 0)))
    );

    let mut closures = Vec::new();
    let mut date = day(2016, 1, 1);
    while date <= day(2018, 12, 31) {
        match calendar.holiday_on(date).map(Holiday::kind) {
            Some(HolidayKind::Closed) => closures.push(date),
            Some(HolidayKind::EarlyClose { .. }) | None => {}
            Some(other) => panic!("{date}: the era ships no {other:?}"),
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(
        closures,
        [
            day(2016, 1, 1),
            day(2016, 3, 25),
            day(2016, 12, 26),
            day(2017, 1, 2),
            day(2017, 4, 14),
            day(2017, 12, 25),
            day(2018, 1, 1),
            day(2018, 3, 30),
            day(2018, 12, 25),
        ]
    );
}

// ---------------------------------------------------------------------------
// The 2022-2024 rows.
// ---------------------------------------------------------------------------

/// 12:15 CT, the era's Thanksgiving Friday close.
const ERA_QUARTER_PAST_NOON: u32 = 12 * 3_600 + 15 * 60;
/// 12:45 CT, the era's Christmas Eve 2024 close.
const ERA_TWELVE_FORTY_FIVE: u32 = 12 * 3_600 + 45 * 60;

/// Every row the 2022-2024 window ships, in table order: the venue-local trade
/// date, the kind with the instant the module's own `early_close(..)` payload
/// carries, and the tier beside the row.
///
/// This is the era-wide instant fence: the sweep below walks the whole window
/// and compares against this list row for row, so a dropped, added or moved
/// row fails as loudly as a wrong instant. A sample would let a slipped close
/// move unnoticed on the dates nobody probed.
const ERA_ROWS: &[((i32, u32, u32), HolidayKind, EvidenceTier)] = &[
    ((2022, 4, 15), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2022, 11, 25),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    ((2022, 12, 26), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 1, 2), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 1, 16), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2023, 2, 20), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2023, 4, 7), HolidayKind::Unsourced, EvidenceTier::T2),
    (
        (2023, 11, 24),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    ((2023, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2024, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    ((2024, 3, 29), HolidayKind::Closed, EvidenceTier::T2),
    (
        (2024, 11, 29),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TWELVE_FORTY_FIVE,
        },
        EvidenceTier::T2,
    ),
    ((2024, 12, 25), HolidayKind::Closed, EvidenceTier::T2),
];

/// `ct` for a date the walk computed rather than spelled, so day arithmetic
/// cannot drift out of step with a hand-written tuple.
fn ct_on(date: NaiveDate, time: (u32, u32, u32)) -> DateTime<Utc> {
    ct((date.year(), date.month(), date.day()), time)
}

fn day_before(date: NaiveDate) -> NaiveDate {
    date.checked_sub_days(Days::new(1))
        .expect("the era is far from the representable bound")
}

fn day_after(date: NaiveDate) -> NaiveDate {
    date.checked_add_days(Days::new(1))
        .expect("the era is far from the representable bound")
}

/// The ordinary 17:00 CT evening open that follows a closed trade date: this
/// civil date's own leg when the week has one, otherwise the Sunday evening
/// that opens the next week — the grid has no Friday-evening occurrence.
fn era_reopen_after_closure(date: NaiveDate) -> DateTime<Utc> {
    let reopen = if date.weekday() == Weekday::Fri {
        date.checked_add_days(Days::new(2))
            .expect("the era is far from the representable bound")
    } else {
        date
    };
    ct_on(reopen, (17, 0, 0))
}

/// The era-wide sweep: every shipped date's kind, instant and tier, with both
/// sides of every moved boundary and the trading day's stated end.
#[test]
fn era_2022_2024_sweeps_every_row_kind_tier_and_instant() {
    let calendar = fx();
    let mut index = 0_usize;
    let (mut quarter_past_noon, mut twelve_forty_five) = (0_usize, 0);
    let (mut closures, mut unsourced) = (0_usize, 0);
    let mut date = day(2022, 1, 1);
    while date <= day(2024, 12, 31) {
        if let Some(row) = calendar.holiday_on(date) {
            let (expected, kind, tier) = ERA_ROWS[index];
            assert_eq!(
                (date.year(), date.month(), date.day()),
                expected,
                "the era's rows must ship in order, with none added"
            );
            assert_eq!(row.kind(), kind, "{date}");
            assert_eq!(row.tier(), tier, "{date}");
            match kind {
                HolidayKind::EarlyClose { close_ssm } => {
                    match close_ssm {
                        ERA_QUARTER_PAST_NOON => quarter_past_noon += 1,
                        ERA_TWELVE_FORTY_FIVE => twelve_forty_five += 1,
                        other => panic!("{date}: the era ships no {other} CT close"),
                    }
                    let cutoff = ct_on(
                        date,
                        (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60),
                    );
                    // The wrap that opened this trade date is clipped, not
                    // deleted, and it still carries the trade date.
                    assert!(
                        calendar.is_open(ct_on(day_before(date), (17, 0, 0))),
                        "{date}"
                    );
                    assert!(
                        calendar.is_open(ct_on(day_before(date), (19, 30, 0))),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.trade_date(ct_on(day_before(date), (18, 0, 0))),
                        Some(date),
                        "{date}"
                    );
                    // One second before the close is open; at it, closed.
                    assert!(calendar.is_open(cutoff - TimeDelta::seconds(1)), "{date}");
                    assert!(!calendar.is_open(cutoff), "{date}: end-exclusive");
                    // The trading day's bounds end at the printed instant, and
                    // so does the daily candle.
                    assert_eq!(
                        calendar.session_bounds(ct_on(date, (9, 0, 0))),
                        Some((ct_on(day_before(date), (17, 0, 0)), cutoff)),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.candle_end(ct_on(date, (9, 0, 0)), CalendarResolution::Daily),
                        Some(cutoff),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.trade_date(cutoff - TimeDelta::seconds(1)),
                        Some(date),
                        "{date}"
                    );
                }
                HolidayKind::Closed => closures += 1,
                HolidayKind::Unsourced => unsourced += 1,
                other => panic!("{date}: the era ships no {other:?}"),
            }
            index += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(index, ERA_ROWS.len(), "every planned row ships");
    assert_eq!(
        (quarter_past_noon, twelve_forty_five, closures, unsourced),
        (3, 1, 7, 3),
        "the era's shape"
    );
}

/// A closure deletes the trade date and the leg that opened it the previous
/// evening, and whatever the crate offers next is the ordinary 17:00 CT
/// evening open — named here so a shifted reopen fails.
#[test]
fn era_2022_2024_closures_remove_the_trading_day_and_the_prior_evening_wrap() {
    let calendar = fx();
    let mut closures = 0_usize;
    for (date, kind, _) in ERA_ROWS {
        if *kind != HolidayKind::Closed {
            continue;
        }
        closures += 1;
        let date = day(date.0, date.1, date.2);
        assert!(
            calendar.is_closed_trade_date(date, SessionKind::Both),
            "{date}"
        );
        // The evening leg that would have carried this trade date is gone.
        assert!(
            !calendar.is_open(ct_on(day_before(date), (17, 0, 0))),
            "{date}"
        );
        assert!(
            !calendar.is_open(ct_on(day_before(date), (19, 30, 0))),
            "{date}"
        );
        // And so is the trade date's own civil day.
        assert!(!calendar.is_open(ct_on(date, (9, 0, 0))), "{date}");
        assert!(!calendar.is_open(ct_on(date, (15, 59, 0))), "{date}");
        assert_eq!(calendar.trade_date(ct_on(date, (10, 0, 0))), None, "{date}");

        let reopen = era_reopen_after_closure(date);
        assert_eq!(
            calendar.next_session_open_after(ct_on(date, (10, 0, 0))),
            Some(reopen),
            "{date}: the next session is the ordinary evening open"
        );
        if reopen == ct_on(date, (17, 0, 0)) {
            assert_eq!(calendar.trade_date(reopen), Some(day_after(date)), "{date}");
        }
    }
    assert_eq!(closures, 7, "the era's closures");
}

/// Every query about an `Unsourced` date answers exactly as the detached
/// calendar does: the row states that the date was audited, makes no
/// scheduling claim, and clips nothing.
fn assert_unsourced_changes_nothing(date: NaiveDate, row: Holiday, tier: EvidenceTier) {
    let calendar = fx();
    let detached = calendar.without_holidays();
    assert_eq!(row.kind(), HolidayKind::Unsourced, "{date}");
    assert_eq!(row.tier(), tier, "{date}");
    // An `Unsourced` row closes nothing. 2021-06-19 is a Saturday, which the
    // grid has no session on whether or not a row exists, so the strict claim
    // is made where the family could trade and the equality claim where it
    // could not.
    if matches!(date.weekday(), Weekday::Sat | Weekday::Sun) {
        assert_eq!(
            calendar.is_closed_trade_date(date, SessionKind::Both),
            detached.is_closed_trade_date(date, SessionKind::Both),
            "{date}"
        );
    } else {
        assert!(
            !calendar.is_closed_trade_date(date, SessionKind::Both),
            "{date} must not be reported closed"
        );
    }

    for probe in [
        ct_on(day_before(date), (18, 0, 0)),
        ct_on(date, (9, 0, 0)),
        ct_on(date, (15, 59, 0)),
        ct_on(date, (18, 0, 0)),
    ] {
        assert_eq!(calendar.is_open(probe), detached.is_open(probe), "{probe}");
        assert_eq!(
            calendar.trade_date(probe),
            detached.trade_date(probe),
            "{probe}"
        );
        assert_eq!(
            calendar.session_bounds(probe),
            detached.session_bounds(probe),
            "{probe}"
        );
        assert_eq!(
            calendar.next_session_open_after(probe),
            detached.next_session_open_after(probe),
            "{probe}"
        );
        assert_eq!(
            calendar.candle_end(probe, CalendarResolution::Daily),
            detached.candle_end(probe, CalendarResolution::Daily),
            "{probe}"
        );
    }
}

#[test]
fn era_2022_2024_unsourced_rows_change_no_answer() {
    let calendar = fx();
    for date in [(2023, 1, 16), (2023, 2, 20), (2023, 4, 7)] {
        let date = day(date.0, date.1, date.2);
        let row = calendar
            .holiday_on(date)
            .unwrap_or_else(|| panic!("{date} ships a row"));
        assert_unsourced_changes_nothing(date, row, EvidenceTier::T2);
    }
}

/// The 2022-2024 window sits fourth in the declared coverage, its edges
/// answer, and the 2013-2015 interval below the 2016-2018 wave stays
/// unaudited. (The 2019-2021 interval this test used to fence became a window
/// of its own when that wave shipped; the section below fences it.)
#[test]
fn era_2022_2024_window_sits_fourth_and_the_2013_2015_interval_is_unaudited() {
    let calendar = fx();
    let bare = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_fx ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day(2010, 1, 1), day(2012, 12, 31)),
            (day(2016, 1, 1), day(2018, 12, 31)),
            (day(2019, 1, 1), day(2021, 12, 31)),
            (day(2022, 1, 1), day(2024, 12, 31)),
            (day(2025, 1, 1), day(2027, 12, 31)),
        ]
    );
    assert!(coverage.contains(day(2022, 1, 1)));
    assert!(coverage.contains(day(2024, 12, 31)));
    assert!(coverage.contains(day(2021, 12, 31)));
    assert_eq!(calendar.holiday_on(day(2022, 1, 1)), None);
    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);

    for date in [(2013, 6, 14), (2015, 12, 25)] {
        assert!(!coverage.contains(day(date.0, date.1, date.2)), "{date:?}");
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "{date:?}"
        );
    }
    // Christmas 2015 is a real CME closure no wave audited; the ordinary
    // Friday answers, and the detached calendar answers it the same.
    for probe in [
        ct((2015, 12, 25), (10, 0, 0)),
        ct((2015, 12, 24), (18, 0, 0)),
    ] {
        assert!(calendar.is_open(probe), "{probe}");
        assert_eq!(calendar.is_open(probe), bare.is_open(probe), "{probe}");
    }
}

/// The era's early closes are the family's own instants, not the equity
/// index's: 12:15 CT on the three Thanksgiving Fridays, and 12:45 CT on
/// Christmas Eve 2024 — and no other instant anywhere in the window.
#[test]
fn era_2022_2024_early_close_instants_are_the_familys_own() {
    let calendar = fx();

    for date in [(2022, 11, 25), (2023, 11, 24), (2024, 11, 29)] {
        assert_eq!(
            calendar
                .holiday_on(day(date.0, date.1, date.2))
                .map(Holiday::kind),
            Some(HolidayKind::EarlyClose {
                close_ssm: ERA_QUARTER_PAST_NOON
            }),
            "{date:?}"
        );
        assert!(calendar.is_open(ct(date, (12, 14, 59))), "{date:?}");
        assert!(!calendar.is_open(ct(date, (12, 15, 0))), "{date:?}");
    }

    assert_eq!(
        calendar.holiday_on(day(2024, 12, 24)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: ERA_TWELVE_FORTY_FIVE
        })
    );
    assert!(calendar.is_open(ct((2024, 12, 24), (12, 44, 59))));
    assert!(!calendar.is_open(ct((2024, 12, 24), (12, 45, 0))));
    // The 16:00 CT final close the family prints on an ordinary day is not
    // reached on any of the four.
    for date in [
        (2022, 11, 25),
        (2023, 11, 24),
        (2024, 11, 29),
        (2024, 12, 24),
    ] {
        assert!(!calendar.is_open(ct(date, (15, 0, 0))), "{date:?}");
    }
}

// ---------------------------------------------------------------------------
// The 2019-2021 rows.
// ---------------------------------------------------------------------------

/// The era-wide sweep: every row the 2019-2021 window ships, read from the
/// module rather than copied beside it, with both sides of every instant it
/// states.
///
/// The walk covers the whole window, so a dropped or added row fails on the
/// era's total, a moved instant fails on the count for the instant it moved
/// from or to, and a kind this family does not ship fails outright. An early
/// close must be open one second before its printed instant and closed at it;
/// a closure must take the trade date and the leg that opened it.
#[test]
fn era_2019_2021_sweeps_every_shipped_row_kind_and_instant() {
    let calendar = fx();
    let (mut noons, mut quarters, mut ten_fifteens) = (0_usize, 0, 0);
    let (mut closures, mut unsourced) = (0_usize, 0);
    let mut rows = 0_usize;
    let mut date = day(2019, 1, 1);
    while date <= day(2021, 12, 31) {
        if let Some(row) = calendar.holiday_on(date) {
            assert_eq!(row.tier(), EvidenceTier::T1, "{date}");
            assert!(!row.document_id().is_empty(), "{date} cites no artifact");
            match row.kind() {
                HolidayKind::EarlyClose { close_ssm } => {
                    match close_ssm {
                        ERA_NOON => noons += 1,
                        ERA_QUARTER_PAST_NOON => quarters += 1,
                        ERA_GOOD_FRIDAY_CLOSE => ten_fifteens += 1,
                        other => {
                            panic!("{date}: this family ships no {other}-second CT close here")
                        }
                    }
                    let cutoff = ct_on(
                        date,
                        (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60),
                    );
                    // The wrap that opened this trade date is clipped, not
                    // deleted, and it still carries the trade date.
                    assert!(
                        calendar.is_open(ct_on(day_before(date), (17, 0, 0))),
                        "{date}"
                    );
                    assert!(
                        calendar.is_open(ct_on(day_before(date), (19, 30, 0))),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.trade_date(ct_on(day_before(date), (18, 0, 0))),
                        Some(date),
                        "{date}"
                    );
                    // One second before the close is open; at it, closed.
                    assert!(calendar.is_open(cutoff - TimeDelta::seconds(1)), "{date}");
                    assert!(!calendar.is_open(cutoff), "{date}: end-exclusive");
                    // The trading day's bounds end at the printed instant, and
                    // so does the daily candle. The probe sits just inside the
                    // session, so the 10:15 Good Friday close cannot make the
                    // query answer `None` instead.
                    let inside = cutoff - TimeDelta::minutes(1);
                    assert_eq!(
                        calendar.session_bounds(inside),
                        Some((ct_on(day_before(date), (17, 0, 0)), cutoff)),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.candle_end(inside, CalendarResolution::Daily),
                        Some(cutoff),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.trade_date(cutoff - TimeDelta::seconds(1)),
                        Some(date),
                        "{date}"
                    );
                }
                HolidayKind::Closed => {
                    closures += 1;
                    assert!(
                        calendar.is_closed_trade_date(date, SessionKind::Both),
                        "{date}"
                    );
                    // The evening leg that would have carried this trade date
                    // is gone, and so is the trade date's own session.
                    assert!(
                        !calendar.is_open(ct_on(day_before(date), (17, 0, 0))),
                        "{date}"
                    );
                    assert!(
                        !calendar.is_open(ct_on(day_before(date), (19, 30, 0))),
                        "{date}"
                    );
                    assert!(!calendar.is_open(ct_on(date, (9, 0, 0))), "{date}");
                    assert!(!calendar.is_open(ct_on(date, (15, 59, 0))), "{date}");
                    assert_eq!(calendar.trade_date(ct_on(date, (10, 0, 0))), None, "{date}");
                }
                HolidayKind::Unsourced => unsourced += 1,
                other => panic!("{date}: this era ships no {other:?}"),
            }
            rows += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(rows, 35, "the era's rows");
    assert_eq!(
        (noons, quarters, ten_fifteens, closures, unsourced),
        (18, 5, 1, 8, 3),
        "the era's shape"
    );
}

/// Every `Unsourced` row the era ships changes no answer: the row states that
/// the date was audited, makes no scheduling claim, and clips nothing.
#[test]
fn era_2019_2021_unsourced_rows_change_no_answer() {
    let calendar = fx();
    let mut checked = 0_usize;
    let mut date = day(2019, 1, 1);
    while date <= day(2021, 12, 31) {
        if let Some(row) = calendar.holiday_on(date)
            && row.kind() == HolidayKind::Unsourced
        {
            assert_unsourced_changes_nothing(date, row, EvidenceTier::T1);
            checked += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(checked, 3, "the era's `Unsourced` rows");
}

/// The era is its own declared window: 2019-01-01 and 2021-12-31 are inside
/// it, while 2018-12-31 and 2022-01-01 belong to the waves either side and lie
/// outside it.
#[test]
fn era_2019_2021_window_edges_answer_as_the_module_declares() {
    let calendar = fx();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_fx ships a table");
    let era = (day(2019, 1, 1), day(2021, 12, 31));

    assert!(
        coverage.windows().contains(&era),
        "the 2019-2021 window is declared as a window of its own"
    );
    let inside = |date: NaiveDate| era.0 <= date && date <= era.1;
    assert!(inside(day(2019, 1, 1)));
    assert!(inside(day(2021, 12, 31)));
    assert!(
        !inside(day(2018, 12, 31)),
        "2018-12-31 belongs to the 2016-2018 wave, not to this era"
    );
    assert!(
        !inside(day(2022, 1, 1)),
        "2022-01-01 belongs to the 2022-2024 wave, not to this era"
    );

    // The era's own edges answer for themselves: its first day is the shipped
    // New Year closure, and its last is an ordinary Friday this table audited.
    assert_eq!(
        calendar.holiday_on(day(2019, 1, 1)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_eq!(calendar.holiday_on(day(2021, 12, 31)), None);
    assert!(calendar.is_open(ct((2021, 12, 31), (9, 0, 0))));
    // The neighbouring dates, which other waves audit, carry no row here.
    assert_eq!(calendar.holiday_on(day(2018, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2022, 1, 1)), None);
}

/// The family's coverage names its windows in order, and the 2019-2021 window
/// is one of them: every row the era ships lies inside it, and no row ships on
/// the era's outer neighbours.
#[test]
fn era_2019_2021_window_is_declared_in_order_and_bounds_every_row() {
    let calendar = fx();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_fx ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day(2010, 1, 1), day(2012, 12, 31)),
            (day(2016, 1, 1), day(2018, 12, 31)),
            (day(2019, 1, 1), day(2021, 12, 31)),
            (day(2022, 1, 1), day(2024, 12, 31)),
            (day(2025, 1, 1), day(2027, 12, 31)),
        ]
    );

    // Every shipped row of the era is inside the era's own window: the walk
    // reads the module, and the declared window is what must contain it.
    let mut rows = 0_usize;
    let mut date = day(2019, 1, 1);
    while date <= day(2021, 12, 31) {
        if calendar.holiday_on(date).is_some() {
            assert!(coverage.contains(date), "{date} ships outside its window");
            rows += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(rows, 35, "the era's rows");
    for probe in [day(2018, 12, 31), day(2022, 1, 1)] {
        assert_eq!(calendar.holiday_on(probe), None, "{probe}");
    }
}

/// The 2019-2021 rows as the block records them: date, kind and tier in order,
/// handwritten here rather than read back from the module. The era-wide sweep
/// counts kinds and instants, which a row moved to another audited date with
/// the same kind and instant would leave unchanged; this pins the date set
/// itself, in the order `holiday_on` must answer it.
const ERA_2019_2021_ROWS: &[((i32, u32, u32), HolidayKind, EvidenceTier)] = &[
    ((2019, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2019, 1, 21),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2019, 2, 18),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2019, 4, 19), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2019, 5, 27),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2019, 6, 19), HolidayKind::Unsourced, EvidenceTier::T1),
    (
        (2019, 7, 4),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2019, 9, 2),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2019, 11, 28),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2019, 11, 29),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2019, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2019, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2020, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2020, 1, 20),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2020, 2, 17),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2020, 4, 10), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2020, 5, 25),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2020, 6, 19), HolidayKind::Unsourced, EvidenceTier::T1),
    (
        (2020, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2020, 9, 7),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2020, 11, 26),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2020, 11, 27),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2020, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2020, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2021, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2021, 1, 18),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2021, 2, 15),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2021, 4, 2),
        HolidayKind::EarlyClose {
            close_ssm: 10 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2021, 5, 31),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2021, 6, 19), HolidayKind::Unsourced, EvidenceTier::T1),
    (
        (2021, 7, 5),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2021, 9, 6),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2021, 11, 25),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2021, 11, 26),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2021, 12, 24), HolidayKind::Closed, EvidenceTier::T1),
];

/// The era's audited date, kind and tier set, in order.
#[test]
fn era_2019_2021_rows_are_the_audited_date_kind_and_tier_set() {
    let calendar = fx();
    let mut index = 0_usize;
    let mut date = day(2019, 1, 1);
    while date <= day(2021, 12, 31) {
        if let Some(row) = calendar.holiday_on(date) {
            let (expected, kind, tier) = *ERA_2019_2021_ROWS.get(index).unwrap_or_else(|| {
                panic!("{date}: a row ships in the 2019-2021 window that the block does not record")
            });
            assert_eq!(
                (date.year(), date.month(), date.day()),
                expected,
                "the 2019-2021 rows must ship in order, with none added"
            );
            assert_eq!(row.kind(), kind, "{date}");
            assert_eq!(row.tier(), tier, "{date}");
            index += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(index, ERA_2019_2021_ROWS.len(), "every recorded row ships");
}
