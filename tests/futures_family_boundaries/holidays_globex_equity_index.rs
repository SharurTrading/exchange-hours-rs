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

use chrono::{DateTime, Duration, NaiveDate, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, EvidenceTier, ExchangeCalendar, HolidayKind, MarketHoursKey, SessionKind,
    calendar_for_market_hours_key, hours_for_market_hours_key,
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
fn the_coverage_window_is_exactly_2025_through_2027() {
    let calendar = equity_index();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_equity_index ships a table");
    assert_eq!(coverage.first(), day(2025, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(coverage.contains(day(2025, 1, 1)));
    assert!(coverage.contains(day(2027, 12, 31)));
    assert!(!coverage.contains(day(2024, 12, 31)));
    assert!(!coverage.contains(day(2028, 1, 1)));

    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2028, 1, 1)), None);

    // Christmas Day 2024 is a CME Globex closure one day below the window.
    // The table must not reach it: the crate serves the normal Wednesday.
    let bare = calendar.without_holidays();
    assert!(calendar.is_open(ct((2024, 12, 25), (10, 0, 0))));
    assert_eq!(
        calendar.is_open(ct((2024, 12, 25), (10, 0, 0))),
        bare.is_open(ct((2024, 12, 25), (10, 0, 0)))
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
