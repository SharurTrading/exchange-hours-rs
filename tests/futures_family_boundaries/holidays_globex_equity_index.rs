// SPDX-License-Identifier: MIT-0

//! `globex_equity_index`'s built-in holiday rows, over the public surface.
//!
//! Every probe below is stated in America/Chicago wall clock — the zone CME
//! prints its trading hours in — and converted to UTC by `ct`, so a reader can
//! compare a line here against the operator's published instant without doing
//! the arithmetic. The family's normal trading day for venue-local trade date
//! `D` opens 17:00 CT on the preceding business evening, runs the 08:30-15:15
//! CT regular session and the 15:15-16:00 CT extended leg, and ends at its
//! 16:00 CT final close on `D`; the rows move that close or delete the day.
//!
//! The cases are the design memo's §4.1 seven: a closed day, both sides of an
//! early close, the late-open branch (absent in this window, and fenced as
//! absent), the wrap a closure removes, the trade-date consequence, both edges
//! of the coverage window, and `without_holidays` restoring the normal answer.

use chrono::{DateTime, Datelike as _, Days, Duration, NaiveDate, TimeZone as _, Utc, Weekday};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, EvidenceTier, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey,
    SessionKind, calendar_for_market_hours_key, hours_for_market_hours_key,
};

/// The family calendar under test, with its built-in table attached.
fn equity_index() -> ExchangeCalendar {
    calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
}

/// A venue-local America/Chicago wall clock, converted to the UTC instant the
/// public surface takes.
fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous Central instant")
        .with_timezone(&Utc)
}

fn day(year: i32, month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
}

/// 12:00 CT, the Monday/Thursday-holiday final close.
const NOON: u32 = 12 * 3_600;
/// 12:15 CT, the Christmas-Eve and day-after-Thanksgiving final close.
const QUARTER_PAST_NOON: u32 = 12 * 3_600 + 15 * 60;

// ---------------------------------------------------------------------------
// 1. A closed day.
// ---------------------------------------------------------------------------

/// Christmas 2025 falls on a Thursday and CME publishes no equity-index
/// session for it: the trade date is gone, so nothing the crate would have
/// assigned to it survives — including the Wednesday-evening leg.
#[test]
fn christmas_2025_is_a_closed_trade_date_with_no_session_of_its_own() {
    let calendar = equity_index();

    let holiday = calendar
        .holiday_on(day(2025, 12, 25))
        .expect("2025-12-25 ships a row");
    assert_eq!(holiday.kind(), HolidayKind::Closed);
    assert_eq!(holiday.tier(), EvidenceTier::T2);
    assert_eq!(holiday.document_id(), "CME-SVC-2025-12-24");

    assert!(calendar.is_closed_trade_date(day(2025, 12, 25), SessionKind::Both));

    // Three probes inside the civil day, before the evening leg that belongs
    // to the *next* trade date opens at 17:00 CT.
    for time in [(3, 0, 0), (10, 0, 0), (16, 0, 0)] {
        assert!(
            !calendar.is_open(ct((2025, 12, 25), time)),
            "2025-12-25 {time:?} CT must be closed"
        );
    }

    // The civil day is not wholly closed: 17:00 CT opens trade date 12-26.
    assert!(!calendar.is_closed_all_day_on(day(2025, 12, 25), SessionKind::Both));
    assert!(calendar.is_open(ct((2025, 12, 25), (18, 0, 0))));
}

// ---------------------------------------------------------------------------
// 2 and 3. An early close, on each side of the cutoff.
// ---------------------------------------------------------------------------

/// The day after Thanksgiving 2025 closes 12:15 CT. The trading day opened
/// 17:00 CT on Thursday, so the clip has to land on the Friday and take the
/// 15:15-16:00 CT leg — which opens after the cutoff — with it.
#[test]
fn the_day_after_thanksgiving_2025_closes_at_1215_central() {
    let calendar = equity_index();

    let holiday = calendar
        .holiday_on(day(2025, 11, 28))
        .expect("2025-11-28 ships a row");
    assert_eq!(
        holiday.kind(),
        HolidayKind::EarlyClose {
            close_ssm: QUARTER_PAST_NOON
        }
    );
    assert_eq!(holiday.document_id(), "CME-SVC-2025-11-26");

    // One second before the cutoff, and at it: closes are end-exclusive.
    assert!(calendar.is_open(ct((2025, 11, 28), (12, 14, 59))));
    assert!(!calendar.is_open(ct((2025, 11, 28), (12, 15, 0))));
    // The rest of the regular session is gone, and so is the extended leg
    // that would have opened at 15:15 CT.
    assert!(!calendar.is_open(ct((2025, 11, 28), (14, 0, 0))));
    assert!(!calendar.is_open(ct((2025, 11, 28), (15, 30, 0))));

    assert_eq!(
        calendar.session_bounds(ct((2025, 11, 28), (10, 0, 0))),
        Some((
            ct((2025, 11, 28), (8, 30, 0)),
            ct((2025, 11, 28), (12, 15, 0))
        ))
    );
    assert_eq!(
        calendar.candle_end(ct((2025, 11, 28), (10, 0, 0)), CalendarResolution::Daily),
        Some(ct((2025, 11, 28), (12, 15, 0)))
    );
}

/// Martin Luther King Jr. Day 2025 is the memo's §1.1 worked example: CME
/// prints `12:00 preopen`, matching stops, and the trading day that opened
/// 17:00 CT on Sunday ends there instead of at 16:00 CT.
#[test]
fn martin_luther_king_day_2025_clips_the_sunday_evening_trading_day() {
    let calendar = equity_index();

    assert_eq!(
        calendar
            .holiday_on(day(2025, 1, 20))
            .expect("2025-01-20 ships a row")
            .kind(),
        HolidayKind::EarlyClose { close_ssm: NOON }
    );

    // The Sunday-evening leg that feeds this trade date is clipped, not
    // deleted: it still opens.
    assert!(calendar.is_open(ct((2025, 1, 19), (18, 0, 0))));
    assert!(calendar.is_open(ct((2025, 1, 20), (11, 59, 59))));
    assert!(!calendar.is_open(ct((2025, 1, 20), (12, 0, 0))));
    assert!(!calendar.is_open(ct((2025, 1, 20), (15, 30, 0))));
    // 17:00 CT that evening opens trade date 2025-01-21, which is normal.
    assert!(calendar.is_open(ct((2025, 1, 20), (17, 30, 0))));
    assert_eq!(
        calendar.next_session_open_after(ct((2025, 1, 20), (12, 5, 0))),
        Some(ct((2025, 1, 20), (17, 0, 0)))
    );
}

/// Good Friday 2026 is the one date CME itself flags as an exception, for the
/// U.S. employment release: equity index closes 08:15 CT, before its own
/// regular session would have opened, so the whole day session disappears and
/// only the Thursday-evening leg survives.
#[test]
fn good_friday_2026_closes_equity_index_at_0815_central() {
    let calendar = equity_index();

    assert_eq!(
        calendar
            .holiday_on(day(2026, 4, 3))
            .expect("2026-04-03 ships a row")
            .kind(),
        HolidayKind::EarlyClose {
            close_ssm: 8 * 3_600 + 15 * 60
        }
    );

    assert!(calendar.is_open(ct((2026, 4, 3), (8, 14, 59))));
    assert!(!calendar.is_open(ct((2026, 4, 3), (8, 15, 0))));
    assert!(!calendar.is_open(ct((2026, 4, 3), (10, 0, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2026, 4, 3), (2, 0, 0))),
        Some((ct((2026, 4, 2), (17, 0, 0)), ct((2026, 4, 3), (8, 15, 0))))
    );
}

// ---------------------------------------------------------------------------
// 4. The late-open branch.
// ---------------------------------------------------------------------------

/// No `globex_equity_index` row in this coverage window moves a first open.
///
/// CME reopens every one of these holidays at the family's ordinary 17:00 CT,
/// and the one sourced *later* first open the corpus carries for this family —
/// the 05:00 CT Saturday sessions of 2026-06-20, 2026-07-04 and 2027-06-19 —
/// would have to **create** a session the normal week does not have, which the
/// scalar vocabulary cannot state and which ships as a declared gap instead
/// (design memo D7). This fence keeps that a recorded fact rather than an
/// accident: a late-open row appearing here without its evidence fails.
#[test]
fn no_late_open_row_ships_in_the_2025_2027_window() {
    let calendar = equity_index();
    let mut date = day(2025, 1, 1);
    let last = day(2027, 12, 31);

    while date <= last {
        if let Some(holiday) = calendar.holiday_on(date) {
            assert!(
                matches!(
                    holiday.kind(),
                    HolidayKind::Closed | HolidayKind::EarlyClose { .. }
                ),
                "{date}: this window ships only closures and early closes, not {:?}",
                holiday.kind()
            );
        }
        date = date
            .succ_opt()
            .expect("the window ends well before the epoch bound");
    }

    // The Saturday sessions are gaps, not rows, and the normal week has no
    // Saturday: the crate answers closed on all three.
    for saturday in [(2026, 6, 20), (2026, 7, 4), (2027, 6, 19)] {
        assert!(!calendar.is_open(ct(saturday, (10, 0, 0))));
        assert_eq!(
            calendar.holiday_on(day(saturday.0, saturday.1, saturday.2)),
            None
        );
    }
}

// ---------------------------------------------------------------------------
// 5. A wrap removed by a closure.
// ---------------------------------------------------------------------------

/// Christmas Eve 2025 closes 12:15 CT and does not reopen, because the
/// Wednesday 17:00 CT leg would have carried trade date 2025-12-25 and that
/// date is closed. The next open is the Thursday-evening leg, on the holiday
/// itself, for trade date 2025-12-26.
#[test]
fn the_christmas_2025_closure_removes_the_previous_evenings_wrap() {
    let calendar = equity_index();

    assert_eq!(
        calendar
            .holiday_on(day(2025, 12, 24))
            .expect("2025-12-24 ships a row")
            .kind(),
        HolidayKind::EarlyClose {
            close_ssm: QUARTER_PAST_NOON
        }
    );

    assert!(!calendar.is_open(ct((2025, 12, 24), (12, 15, 0))));
    // The normal 17:00 CT open of Wednesday evening is gone with its trade date.
    assert!(!calendar.is_open(ct((2025, 12, 24), (17, 0, 0))));
    assert!(!calendar.is_open(ct((2025, 12, 24), (17, 30, 0))));
    assert_eq!(
        calendar.next_session_open_after(ct((2025, 12, 24), (12, 20, 0))),
        Some(ct((2025, 12, 25), (17, 0, 0))),
        "the next open is the holiday evening's leg, not a same-day remainder"
    );
}

// ---------------------------------------------------------------------------
// 6. The trade-date consequence.
// ---------------------------------------------------------------------------

/// A shortened day keeps its own trade date; the leg that opens on a closed
/// date carries the date after it.
#[test]
fn holiday_rows_move_the_trade_date_only_where_the_operator_does() {
    let calendar = equity_index();

    // Inside the shortened day after Thanksgiving 2025.
    assert_eq!(
        calendar.trade_date(ct((2025, 11, 28), (10, 0, 0))),
        Some(day(2025, 11, 28))
    );
    // Inside the shortened Martin Luther King Jr. Day 2025.
    assert_eq!(
        calendar.trade_date(ct((2025, 1, 20), (10, 0, 0))),
        Some(day(2025, 1, 20))
    );
    // The evening open on closed Christmas Day 2025 belongs to the Friday.
    assert_eq!(
        calendar.trade_date(ct((2025, 12, 25), (18, 0, 0))),
        Some(day(2025, 12, 26))
    );
    // Nothing is assigned to the closed trade date itself.
    assert_eq!(calendar.trade_date(ct((2025, 12, 25), (10, 0, 0))), None);
    // The evening before a closure has no session at all to assign.
    assert_eq!(calendar.trade_date(ct((2025, 12, 24), (18, 0, 0))), None);
}

// ---------------------------------------------------------------------------
// 7. Both edges of the coverage window.
// ---------------------------------------------------------------------------

/// The window is a claim about every date inside it, so both its edges are
/// fenced: one day out, the table has no answer and the normal week is served
/// unmodified — including on a date that is a CME holiday in its own right.
#[test]
fn the_coverage_window_is_exactly_2010_through_2027() {
    let calendar = equity_index();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_equity_index ships a table");
    assert_eq!(coverage.first(), day(2010, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(coverage.contains(day(2010, 1, 1)));
    assert!(coverage.contains(day(2027, 12, 31)));
    assert!(!coverage.contains(day(2009, 12, 31)));
    assert!(!coverage.contains(day(2028, 1, 1)));
    // The table audits five eras; the 2013-2015 interval between the first two
    // is audited by none, so `contains` is false there and `holiday_on` has no
    // answer rather than calling the date normal. The 2019-2021 interval
    // joined the table when that wave shipped, and the 2024-12-31 edge moved
    // in with the 2022-2024 wave: both are now audited, and a date with no row
    // inside a window answers `None` as an audited normal date rather than as
    // silence.
    assert!(!coverage.contains(day(2013, 6, 14)));
    assert!(coverage.contains(day(2020, 12, 25)));
    assert_eq!(
        calendar.holiday_on(day(2020, 12, 25)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(coverage.contains(day(2024, 12, 31)));
    assert_eq!(calendar.holiday_on(day(2013, 6, 14)), None);
    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);

    assert_eq!(calendar.holiday_on(day(2009, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2028, 1, 1)), None);

    // Christmas Day 2009 is a CME Globex closure one year below the window.
    // The table must not reach it: the crate serves the normal Wednesday.
    let bare = calendar.without_holidays();
    assert!(calendar.is_open(ct((2009, 12, 25), (10, 0, 0))));
    assert_eq!(
        calendar.is_open(ct((2009, 12, 25), (10, 0, 0))),
        bare.is_open(ct((2009, 12, 25), (10, 0, 0)))
    );
    // And New Year's Day 2028, one day above it, answers the normal-week
    // answer for a Saturday.
    assert_eq!(
        calendar.is_open(ct((2028, 1, 1), (10, 0, 0))),
        bare.is_open(ct((2028, 1, 1), (10, 0, 0)))
    );
}

// ---------------------------------------------------------------------------
// 8. `without_holidays` restores the normal answer.
// ---------------------------------------------------------------------------

/// The escape hatch is exact: detaching the table reproduces the pre-table
/// answer over a dense grid across the Christmas 2025 week, and the two
/// calendars must actually disagree inside it, or the grid proves nothing.
#[test]
fn without_holidays_restores_the_normal_week_across_the_christmas_2025_week() {
    let calendar = equity_index();
    let bare = calendar.without_holidays();

    assert_eq!(bare.holiday_coverage(), None);
    assert_eq!(bare.holiday_on(day(2025, 12, 25)), None);

    // The normal week has a session at each of these instants; the table
    // removes or shortens all three.
    assert!(bare.is_open(ct((2025, 12, 24), (17, 30, 0))));
    assert!(bare.is_open(ct((2025, 12, 25), (10, 0, 0))));
    assert!(bare.is_open(ct((2025, 11, 28), (15, 30, 0))));
    assert!(!calendar.is_open(ct((2025, 12, 24), (17, 30, 0))));
    assert!(!calendar.is_open(ct((2025, 12, 25), (10, 0, 0))));
    assert!(!calendar.is_open(ct((2025, 11, 28), (15, 30, 0))));

    // Over the whole week, at five-minute resolution, the detached calendar
    // reproduces the pre-table engine exactly — a detached `MarketHours`
    // snapshot carries no identity and therefore no table, so it is the
    // control — while the attached one differs on at least one probe.
    let mut instant = ct((2025, 12, 21), (0, 0, 0));
    let end = ct((2025, 12, 28), (0, 0, 0));
    let mut differed = false;
    while instant < end {
        let pre_table =
            hours_for_market_hours_key(MarketHoursKey::GlobexEquityIndex, instant).is_open(instant);
        assert_eq!(
            bare.is_open(instant),
            pre_table,
            "without_holidays diverged from the pre-table answer at {instant}"
        );
        differed |= calendar.is_open(instant) != pre_table;
        instant += Duration::minutes(5);
    }
    assert!(
        differed,
        "the table must change an answer inside the Christmas 2025 week"
    );
}

// ---------------------------------------------------------------------------
// 9. The 2010-2012 rows.
// ---------------------------------------------------------------------------

/// 10:30 CT, the era's Monday/Thursday-holiday equity-index final close.
const ERA_MONDAY_HOLIDAY_CLOSE: u32 = 10 * 3_600 + 30 * 60;
/// 08:15 CT, the era's Good Friday equity-index final close.
const ERA_GOOD_FRIDAY_CLOSE: u32 = 8 * 3_600 + 15 * 60;

/// The era's early closes clip a trading day that opened 17:00 CT the previous
/// evening, so the cutoff has to delete the whole of the remaining 08:30-15:15
/// CT day session while leaving the opening wrap in place.
///
/// Martin Luther King Jr. Day 2010 is the 10:30 CT shape; Good Friday 2010 is
/// the 08:15 CT shape, stated before the era's own 08:30 CT day open, so only
/// the Thursday-evening leg survives to it.
#[test]
fn era_early_closes_end_the_wrapped_trading_day_at_the_stated_instant() {
    let calendar = equity_index();

    assert_eq!(
        calendar
            .holiday_on(day(2010, 1, 18))
            .expect("2010-01-18 ships a row")
            .kind(),
        HolidayKind::EarlyClose {
            close_ssm: ERA_MONDAY_HOLIDAY_CLOSE
        }
    );
    // The Sunday-evening leg that feeds this trade date is clipped, not
    // deleted: it still opens.
    assert!(calendar.is_open(ct((2010, 1, 17), (17, 0, 0))));
    assert!(calendar.is_open(ct((2010, 1, 18), (10, 29, 59))));
    // The close instant itself is closed; the rest of the day is gone.
    assert!(!calendar.is_open(ct((2010, 1, 18), (10, 30, 0))));
    assert!(!calendar.is_open(ct((2010, 1, 18), (15, 0, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2010, 1, 18), (9, 0, 0))),
        Some((
            ct((2010, 1, 18), (8, 30, 0)),
            ct((2010, 1, 18), (10, 30, 0))
        ))
    );
    // 17:00 CT that evening opens trade date 2010-01-19, which is normal.
    assert_eq!(
        calendar.trade_date(ct((2010, 1, 18), (18, 0, 0))),
        Some(day(2010, 1, 19))
    );

    assert_eq!(
        calendar
            .holiday_on(day(2010, 4, 2))
            .expect("2010-04-02 ships a row")
            .kind(),
        HolidayKind::EarlyClose {
            close_ssm: ERA_GOOD_FRIDAY_CLOSE
        }
    );
    assert!(calendar.is_open(ct((2010, 4, 2), (8, 14, 59))));
    assert!(!calendar.is_open(ct((2010, 4, 2), (8, 15, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2010, 4, 2), (2, 0, 0))),
        Some((ct((2010, 4, 1), (17, 0, 0)), ct((2010, 4, 2), (8, 15, 0))))
    );
    assert_eq!(calendar.trade_date(ct((2010, 4, 2), (8, 15, 0))), None);
}

/// Every late open the era ships states exactly 05:00 CT, which is *earlier* than
/// the family's normal 17:00 CT first open, so the cutoff lands on the trade date
/// itself: the Monday-evening leg that would have opened trade date 2011-12-27
/// did not run, and matching starts at 05:00 CT on the Tuesday.
///
/// The two Good Friday eves the retrieval also states a `1530 CT - Regular CME
/// Globex open` for — 2010-04-01 and 2012-04-05 — ship **no** row: the crate's
/// own era grid already ends those eves' day sessions at 15:15 CT and reopens
/// at the ordinary 17:00 CT, and the operator's 15:30 instant is the trade
/// date's *own* evening open rather than a delayed one. A `late_open` row at
/// 15:30 would land after its own trade date's close and delete the session;
/// the dates are named as gaps in the family's evidence file instead.
#[test]
fn era_late_opens_land_on_the_trade_date_itself() {
    let calendar = equity_index();
    let era_late_opens = [(2011, 12, 27), (2012, 1, 3), (2012, 12, 26)];
    for (year, month, date) in era_late_opens {
        let row = calendar
            .holiday_on(day(year, month, date))
            .unwrap_or_else(|| panic!("{year}-{month:02}-{date:02} ships a row"));
        assert_eq!(
            row.kind(),
            HolidayKind::LateOpen {
                open_ssm: 5 * 3_600
            },
            "{year}-{month:02}-{date:02} must state the operator's 05:00 CT first open"
        );
        assert!(
            !calendar.is_open(ct((year, month, date), (4, 59, 59))),
            "{year}-{month:02}-{date:02}: matching has not started one second before 05:00 CT"
        );
        assert!(
            calendar.is_open(ct((year, month, date), (5, 0, 0))),
            "{year}-{month:02}-{date:02}: matching starts at 05:00 CT"
        );
        assert_eq!(
            calendar.trade_date(ct((year, month, date), (9, 0, 0))),
            Some(day(year, month, date)),
            "{year}-{month:02}-{date:02}: the late open is stated on its own trade date"
        );
    }
    for (year, month, date) in [(2010, 4, 1), (2012, 4, 5)] {
        assert_eq!(
            calendar.holiday_on(day(year, month, date)),
            None,
            "{year}-{month:02}-{date:02} ships no row: the operator's 1530 CT open is its own evening open"
        );
    }
}

/// The 2012-07-03 equity row is an `EarlyClose` at 12:15 CT on its own trade
/// date. CME also states a `1530 CT - Regular CME Globex open for trade date
/// Thursday, July 5` on that sheet, but that instant belongs to the *next*
/// trade date: the crate's scalar vocabulary cannot state a same-day re-open
/// after a close, so the evening leg of 07-05 is not modelled and is recorded
/// as a gap in the family's evidence file. The row that ships stops the
/// session at 12:15 CT and leaves the ordinary 17:00 CT leg for 07-05 alone.
#[test]
fn era_2012_07_03_early_close_ends_the_trade_date_at_1215_central() {
    let calendar = equity_index();

    assert_eq!(
        calendar
            .holiday_on(day(2012, 7, 3))
            .expect("2012-07-03 ships a row")
            .kind(),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60
        }
    );

    assert!(calendar.is_open(ct((2012, 7, 3), (12, 14, 59))));
    assert!(!calendar.is_open(ct((2012, 7, 3), (12, 15, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2012, 7, 3), (9, 0, 0))),
        Some((ct((2012, 7, 3), (8, 30, 0)), ct((2012, 7, 3), (12, 15, 0))))
    );

    // The next trade date keeps its own ordinary leg, which is exactly the
    // 2012-07-03 17:00 CT open the operator prints for 07-05.
    assert_eq!(
        calendar.next_session_open_after(ct((2012, 7, 3), (12, 15, 0))),
        Some(ct((2012, 7, 3), (17, 0, 0)))
    );
    assert_eq!(
        calendar.trade_date(ct((2012, 7, 3), (17, 0, 0))),
        Some(day(2012, 7, 4))
    );
}

/// A date inside the widened window with no row is audited normal: the crate
/// serves the profile's own 2010 week, not a holiday-shaped guess, so the
/// attached and detached calendars agree instant for instant.
#[test]
fn era_dates_without_rows_are_audited_normal() {
    let calendar = equity_index();
    let bare = calendar.without_holidays();

    for (date, time) in [
        ((2010, 6, 15), (10, 0, 0)),
        ((2010, 6, 14), (17, 0, 0)),
        ((2011, 3, 9), (9, 0, 0)),
        ((2012, 10, 10), (14, 0, 0)),
    ] {
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "{date:?} must ship no row"
        );
        assert!(
            calendar.is_open(ct(date, time)),
            "{date:?} {time:?} CT is an ordinary trading instant"
        );
        assert_eq!(
            calendar.is_open(ct(date, time)),
            bare.is_open(ct(date, time)),
            "the row set must not change {date:?} {time:?} CT"
        );
    }
}

/// The widened window's edges answer exactly as the module declares: it opens
/// on 2010-01-01, closes on 2027-12-31, and Christmas Day 2009 — a real CME
/// closure one year below the window — is not applied, with the detached
/// calendar agreeing.
#[test]
fn era_window_edges_answer_as_the_module_declares() {
    let calendar = equity_index();
    let bare = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_equity_index ships a table");

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

/// 15:30 CT, the open the 2018 Christmas sheet prints on 26 December.
const WAVE2_LATE_OPEN: u32 = 15 * 3_600 + 30 * 60;
/// 12:15 CT is the same instant the 2025-2027 window's half-days close at, so
/// the era reuses the constant above rather than restating it.
const WAVE2_HALF_DAY_CLOSE: u32 = QUARTER_PAST_NOON;

/// The era's early closes clip a trading day that opened 17:00 CT the previous
/// evening: 12:00 CT on the nine Monday and Thursday holidays, 12:15 CT on the
/// three Thanksgiving Fridays, the three Independence Day eves and the two
/// Christmas Eves CME prints one.
#[test]
fn wave2_early_closes_end_the_wrapped_trading_day_at_the_printed_instant() {
    let calendar = equity_index();

    for (date, previous_day, close_ssm) in [
        ((2016, 1, 18), (2016, 1, 17), NOON),
        ((2016, 11, 24), (2016, 11, 23), NOON),
        ((2016, 11, 25), (2016, 11, 24), WAVE2_HALF_DAY_CLOSE),
        ((2017, 7, 3), (2017, 7, 2), WAVE2_HALF_DAY_CLOSE),
        ((2018, 12, 24), (2018, 12, 23), WAVE2_HALF_DAY_CLOSE),
        ((2018, 12, 26), (2018, 12, 25), 16 * 3_600),
    ] {
        if close_ssm == 16 * 3_600 {
            // 2018-12-26 is the late-open case, asserted on its own below.
            continue;
        }
        let kind = calendar
            .holiday_on(day(date.0, date.1, date.2))
            .map(Holiday::kind);
        assert_eq!(
            kind,
            Some(HolidayKind::EarlyClose { close_ssm }),
            "{date:?}"
        );

        // The evening leg that feeds this trade date is clipped, not deleted.
        assert!(calendar.is_open(ct(previous_day, (17, 0, 0))), "{date:?}");
        let cutoff = ct(date, (close_ssm / 3_600, (close_ssm % 3_600) / 60, 0));
        assert!(
            calendar.is_open(cutoff - Duration::seconds(1)),
            "{date:?}: the second before the close is still open"
        );
        assert!(
            !calendar.is_open(cutoff),
            "{date:?}: the close is end-exclusive"
        );
        assert_eq!(
            calendar.candle_end(ct(date, (9, 0, 0)), CalendarResolution::Daily),
            Some(cutoff),
            "{date:?}: the daily candle ends at the printed close"
        );
        // The cut does not delete the trading day: the same trade date still
        // owns the session, and the operator's own evening leg begins the next.
        assert_eq!(
            calendar.trade_date(ct(date, (9, 0, 0))),
            Some(day(date.0, date.1, date.2))
        );
        assert_eq!(
            calendar.trade_date(cutoff - Duration::seconds(1)),
            Some(day(date.0, date.1, date.2)),
            "{date:?}"
        );
    }

    // The holiday's own evening 17:00 CT leg still opens the next trade date.
    assert_eq!(
        calendar.trade_date(ct((2016, 1, 18), (18, 0, 0))),
        Some(day(2016, 1, 19))
    );
    // And a 12:05 CT cut belongs to grains, not here: this family is open.
    assert!(calendar.is_open(ct((2016, 11, 25), (12, 5, 0))));
}

/// A closure removes the trade date and the leg that opened the previous
/// evening, and the operator's own stated re-open — always this family's
/// ordinary 17:00 CT — starts the next trade date.
#[test]
fn wave2_closures_remove_the_trade_date_and_the_prior_evening_leg() {
    let calendar = equity_index();

    // Good Friday 2016 is a full closure: the Thursday-evening leg that fed
    // the Friday trade date goes with it, and the Friday-evening leg CME
    // publishes starts the following Monday's trade date.
    assert_eq!(
        calendar.holiday_on(day(2016, 3, 25)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(calendar.is_closed_trade_date(day(2016, 3, 25), SessionKind::Both));
    assert!(!calendar.is_open(ct((2016, 3, 24), (18, 0, 0))));
    assert!(!calendar.is_open(ct((2016, 3, 25), (10, 0, 0))));
    // The era's grid has no Friday-evening occurrence, and CME's own sheet
    // publishes the Sunday re-open for trade date Monday 2016-03-28, which is
    // exactly what the crate serves.
    assert_eq!(
        calendar.next_session_open_after(ct((2016, 3, 25), (10, 0, 0))),
        Some(ct((2016, 3, 27), (17, 0, 0)))
    );
    assert!(calendar.is_open(ct((2016, 3, 27), (18, 0, 0))));
    assert_eq!(
        calendar.trade_date(ct((2016, 3, 27), (18, 0, 0))),
        Some(day(2016, 3, 28))
    );

    // New Year's Day 2016 fell on a Friday, so the closure is the observed
    // Friday and the next open is the Sunday leg into Monday's trade date.
    assert_eq!(
        calendar.holiday_on(day(2016, 1, 1)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(!calendar.is_open(ct((2015, 12, 31), (17, 0, 0))));
    assert_eq!(
        calendar.next_session_open_after(ct((2016, 1, 1), (12, 0, 0))),
        Some(ct((2016, 1, 3), (17, 0, 0)))
    );

    // Every one of the era's nine closures is a closure, and nothing else in
    // the era is.
    let mut closures = Vec::new();
    let mut date = day(2016, 1, 1);
    while date <= day(2018, 12, 31) {
        if calendar.holiday_on(date).map(Holiday::kind) == Some(HolidayKind::Closed) {
            closures.push(date);
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

/// 2018-12-26 is the era's one late open: the Christmas sheet prints the
/// Equity line's `Pre-opening 15:15` and `Open 15:30` on a date the crate's
/// grid has open at 15:15 CT, so the first open moves half an hour later and
/// nothing else about the day moves.
#[test]
fn wave2_2018_12_26_opens_late_at_1530_central() {
    let calendar = equity_index();
    let date = day(2018, 12, 26);
    let cutoff = ct((2018, 12, 26), (15, 30, 0));

    assert_eq!(
        calendar.holiday_on(date).map(Holiday::kind),
        Some(HolidayKind::LateOpen {
            open_ssm: WAVE2_LATE_OPEN
        })
    );
    assert!(!calendar.is_open(cutoff - Duration::seconds(1)));
    assert!(calendar.is_open(cutoff));
    assert_eq!(
        calendar.session_bounds(cutoff + Duration::seconds(1)),
        Some((cutoff, ct((2018, 12, 26), (16, 0, 0))))
    );
    assert_eq!(calendar.trade_date(cutoff), Some(date));

    // The 16:00 CT final close is the normal week's, and the row does not
    // reach it: the crate and the detached calendar answer it the same.
    assert!(calendar.is_open(ct((2018, 12, 26), (15, 59, 0))));
    assert!(!calendar.is_open(ct((2018, 12, 26), (16, 0, 0))));
}

/// The era's own window edges, and the one unaudited interval below it.
#[test]
fn wave2_window_edges_and_unaudited_neighbours_answer_as_declared() {
    let calendar = equity_index();
    let bare = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_equity_index ships a table");

    assert!(coverage.contains(day(2016, 1, 1)));
    assert!(coverage.contains(day(2018, 12, 31)));
    assert!(!coverage.contains(day(2015, 12, 31)));
    // 2019-01-01 became the first date of a window of its own when the
    // 2019-2021 wave shipped: the only interval no wave audits is 2013-2015.
    assert!(coverage.contains(day(2019, 1, 1)));
    for date in [(2013, 6, 14), (2015, 12, 31)] {
        assert!(!coverage.contains(day(date.0, date.1, date.2)), "{date:?}");
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "{date:?}"
        );
    }

    // 2015-12-25 is a CME closure one week below the era: not applied.
    assert_eq!(calendar.holiday_on(day(2015, 12, 25)), None);
    assert!(calendar.is_open(ct((2015, 12, 25), (10, 0, 0))));
    assert_eq!(
        calendar.is_open(ct((2015, 12, 25), (10, 0, 0))),
        bare.is_open(ct((2015, 12, 25), (10, 0, 0)))
    );
    // 2019-01-01 is one day above it and is now audited: the 2019-2021 wave
    // ships the New Year closure the crate previously served as an ordinary
    // Tuesday, so the attached and detached calendars part company here.
    assert_eq!(
        calendar.holiday_on(day(2019, 1, 1)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(!calendar.is_open(ct((2019, 1, 1), (10, 0, 0))));
    assert!(bare.is_open(ct((2019, 1, 1), (10, 0, 0))));
}

// ---------------------------------------------------------------------------
// The 2022-2024 rows.
// ---------------------------------------------------------------------------

/// Every row the 2022-2024 window ships, in table order: the venue-local trade
/// date, the kind with the instant the module's own `early_close(..)` payload
/// carries, and the tier beside the row.
///
/// This is the era-wide instant fence. A sample would let a slipped close move
/// unnoticed on the dates nobody probed, so the sweep below walks the whole
/// window and compares against this list row for row: a dropped, added or
/// moved row fails as loudly as a wrong instant.
const ERA_ROWS: &[((i32, u32, u32), HolidayKind, EvidenceTier)] = &[
    (
        (2022, 1, 17),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2022, 2, 21),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    ((2022, 4, 15), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2022, 5, 30),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2022, 6, 20),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2022, 7, 4),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2022, 9, 5),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2022, 11, 24),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2022, 11, 25),
        HolidayKind::EarlyClose {
            close_ssm: QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    ((2022, 12, 26), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 1, 2), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 1, 16), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2023, 2, 20), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2023, 4, 7), HolidayKind::Unsourced, EvidenceTier::T2),
    (
        (2023, 5, 29),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2023, 6, 19),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2023, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2023, 7, 4),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2023, 9, 4),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2023, 11, 23),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T1,
    ),
    (
        (2023, 11, 24),
        HolidayKind::EarlyClose {
            close_ssm: QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    ((2023, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2024, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2024, 1, 15),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T2,
    ),
    (
        (2024, 2, 19),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T2,
    ),
    ((2024, 3, 29), HolidayKind::Closed, EvidenceTier::T2),
    (
        (2024, 5, 27),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T2,
    ),
    (
        (2024, 6, 19),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T2,
    ),
    (
        (2024, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: QUARTER_PAST_NOON,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 7, 4),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T2,
    ),
    (
        (2024, 9, 2),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T2,
    ),
    (
        (2024, 11, 28),
        HolidayKind::EarlyClose { close_ssm: NOON },
        EvidenceTier::T2,
    ),
    (
        (2024, 11, 29),
        HolidayKind::EarlyClose {
            close_ssm: QUARTER_PAST_NOON,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: QUARTER_PAST_NOON,
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
/// that opens the next week — the grid has no Friday-evening occurrence, which
/// is why a Good Friday closure's next session is Sunday's.
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
    let calendar = equity_index();
    let mut index = 0_usize;
    let (mut noons, mut quarters, mut closures, mut unsourced) = (0_usize, 0, 0, 0);
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
                    let cutoff = ct_on(
                        date,
                        (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60),
                    );
                    if close_ssm == NOON {
                        noons += 1;
                    } else {
                        quarters += 1;
                    }
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
                    assert!(calendar.is_open(cutoff - Duration::seconds(1)), "{date}");
                    assert!(!calendar.is_open(cutoff), "{date}: end-exclusive");
                    // The day session's bounds end at the printed instant, and
                    // so does the daily candle.
                    assert_eq!(
                        calendar.session_bounds(ct_on(date, (9, 0, 0))),
                        Some((ct_on(date, (8, 30, 0)), cutoff)),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.candle_end(ct_on(date, (9, 0, 0)), CalendarResolution::Daily),
                        Some(cutoff),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.trade_date(cutoff - Duration::seconds(1)),
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
        (noons, quarters, closures, unsourced),
        (19, 6, 7, 3),
        "the era's shape"
    );
}

/// A closure deletes the trade date and the leg that opened it the previous
/// evening, and whatever the crate offers next is the ordinary 17:00 CT
/// evening open — named here so a shifted reopen fails.
#[test]
fn era_2022_2024_closures_remove_the_trading_day_and_the_prior_evening_wrap() {
    let calendar = equity_index();
    let mut closures = 0_usize;
    for (date, kind, _) in ERA_ROWS {
        if *kind != HolidayKind::Closed {
            continue;
        }
        closures += 1;
        let date = day(date.0, date.1, date.2);
        let previous = day_before(date);
        assert!(
            calendar.is_closed_trade_date(date, SessionKind::Both),
            "{date}"
        );
        // The evening leg that would have carried this trade date is gone.
        assert!(!calendar.is_open(ct_on(previous, (17, 0, 0))), "{date}");
        assert!(!calendar.is_open(ct_on(previous, (19, 30, 0))), "{date}");
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
            // A mid-week closure leaves the same day's evening leg alone, and
            // that leg opens the next trade date.
            assert_eq!(calendar.trade_date(reopen), Some(day_after(date)), "{date}");
        }
    }
    assert_eq!(closures, 7, "the era's closures");
}

/// Every query about an `Unsourced` date answers exactly as the detached
/// calendar does: the row states that the date was audited, makes no
/// scheduling claim, and clips nothing.
fn assert_unsourced_changes_nothing(date: NaiveDate, row: Holiday, tier: EvidenceTier) {
    let calendar = equity_index();
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

    let previous = day_before(date);
    for probe in [
        ct_on(previous, (18, 0, 0)),
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
    let calendar = equity_index();
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
    let calendar = equity_index();
    let bare = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_equity_index ships a table");

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
    // 2022-01-01 is a Saturday the operator audits normal, so a date with no
    // row inside the window answers `None`, not a closure.
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
        ct((2015, 12, 25), (14, 0, 0)),
        ct((2015, 12, 24), (18, 0, 0)),
    ] {
        assert!(calendar.is_open(probe), "{probe}");
        assert_eq!(calendar.is_open(probe), bare.is_open(probe), "{probe}");
    }
}

/// This family's 2022-2024 grid keeps the trading day in two phases — the
/// 17:00 CT overnight leg into the 08:30 CT regular session, then the regular
/// session to the final close — and an early close clips only the phase it
/// lands in. The overnight phase still ends at 08:30 CT.
#[test]
fn era_2022_2024_early_closes_clip_the_day_phase_not_the_overnight_one() {
    let calendar = equity_index();

    for (date, close_ssm) in [((2022, 1, 17), NOON), ((2024, 12, 24), QUARTER_PAST_NOON)] {
        let previous = day_before(day(date.0, date.1, date.2));
        let cutoff = ct(date, (close_ssm / 3_600, (close_ssm % 3_600) / 60, 0));
        assert_eq!(
            calendar.session_bounds(ct_on(previous, (18, 0, 0))),
            Some((
                ct_on(previous, (17, 0, 0)),
                ct_on(day(date.0, date.1, date.2), (8, 30, 0))
            )),
            "{date:?}: the overnight phase is clipped to 08:30, not deleted"
        );
        assert_eq!(
            calendar.session_bounds(ct(date, (9, 0, 0))),
            Some((ct(date, (8, 30, 0)), cutoff)),
            "{date:?}: the regular phase ends at the printed instant"
        );
    }
}

// ---------------------------------------------------------------------------
// The 2019-2021 rows.
// ---------------------------------------------------------------------------

/// 08:15 CT, the one Good Friday 2021 close this era prints.
const ERA_EIGHT_FIFTEEN: u32 = 8 * 3_600 + 15 * 60;

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
    let calendar = equity_index();
    let (mut noons, mut quarters, mut eight_fifteens) = (0_usize, 0, 0);
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
                        NOON => noons += 1,
                        QUARTER_PAST_NOON => quarters += 1,
                        ERA_EIGHT_FIFTEEN => eight_fifteens += 1,
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
                    assert!(calendar.is_open(cutoff - Duration::seconds(1)), "{date}");
                    assert!(!calendar.is_open(cutoff), "{date}: end-exclusive");
                    // The trading day ends at the printed instant, and so does
                    // the daily candle. Which phase carries it depends on the
                    // instant: the 08:15 Good Friday close lands inside the
                    // overnight phase, and a 12:00/12:15 close lands inside the
                    // regular one, whose 08:30 CT handoff is untouched.
                    let inside = cutoff - Duration::minutes(1);
                    if close_ssm == ERA_EIGHT_FIFTEEN {
                        assert_eq!(
                            calendar.session_bounds(inside),
                            Some((ct_on(day_before(date), (17, 0, 0)), cutoff)),
                            "{date}: the overnight phase ends at the cutoff"
                        );
                    } else {
                        assert_eq!(
                            calendar.session_bounds(inside),
                            Some((ct_on(date, (8, 30, 0)), cutoff)),
                            "{date}: the regular phase ends at the cutoff"
                        );
                        assert_eq!(
                            calendar.session_bounds(ct_on(day_before(date), (18, 0, 0))),
                            Some((ct_on(day_before(date), (17, 0, 0)), ct_on(date, (8, 30, 0)))),
                            "{date}: the overnight phase is clipped to 08:30, not deleted"
                        );
                    }
                    assert_eq!(
                        calendar.candle_end(inside, CalendarResolution::Daily),
                        Some(cutoff),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.trade_date(cutoff - Duration::seconds(1)),
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
    assert_eq!(rows, 36, "the era's rows");
    assert_eq!(
        (noons, quarters, eight_fifteens, closures, unsourced),
        (18, 6, 1, 8, 3),
        "the era's shape"
    );
}

/// Every `Unsourced` row the era ships changes no answer: the row states that
/// the date was audited, makes no scheduling claim, and clips nothing.
#[test]
fn era_2019_2021_unsourced_rows_change_no_answer() {
    let calendar = equity_index();
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
    let calendar = equity_index();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_equity_index ships a table");
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
    let calendar = equity_index();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_equity_index ships a table");

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
    assert_eq!(rows, 36, "the era's rows");
    for probe in [day(2018, 12, 31), day(2022, 1, 1)] {
        assert_eq!(calendar.holiday_on(probe), None, "{probe}");
    }
}
