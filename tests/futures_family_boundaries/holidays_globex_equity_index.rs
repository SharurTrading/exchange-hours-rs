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

/// Every late open the era ships states an instant *earlier* than the family's
/// normal 17:00 CT first open, so the cutoff lands on the trade date itself:
/// the Monday-evening leg that would have opened trade date 2011-12-27 did not
/// run, and matching starts at 05:00 CT on the Tuesday.
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
        let open_ssm = match row.kind() {
            HolidayKind::LateOpen { open_ssm } => open_ssm,
            other => panic!("{year}-{month:02}-{date:02} ships {other:?}, not a late open"),
        };
        assert!(
            open_ssm < 17 * 3_600,
            "{year}-{month:02}-{date:02}: {open_ssm} is not earlier than the era's 17:00 CT open"
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
