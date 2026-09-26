// SPDX-License-Identifier: MIT-0

//! Built-in holiday rows for CFE (`cfe` venue, `cfe_vix` key), 2025-2026.
//!
//! One table serves both identities, so every case below is asserted through
//! the key and the venue agreement is asserted once at the end.
//!
//! CFE has no late open in this window: every Cboe row whose regular cell reads
//! `None` stops the overnight leg early rather than starting it late, so §4.1
//! case 4 has nothing to exercise here and says so rather than inventing a row.
//!
//! The 2025 rows are not copies of the 2026 shapes and the suite says so: Good
//! Friday 2025-04-18 is a closure where 2026-04-03 is an early close, and
//! 2025-01-09 is the only `ReplacementBlocks` row the table ships.

use chrono::{DateTime, Days, NaiveDate, TimeDelta, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarQueryError, CalendarResolution, ExceptionBlockKind, Exchange, ExchangeCalendar,
    Holiday, HolidayKind, MarketHoursKey, SessionKind, calendar_for_exchange,
    calendar_for_market_hours_key,
};

fn cfe() -> ExchangeCalendar {
    calendar_for_market_hours_key(MarketHoursKey::CfeVix)
}

fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous Chicago instant")
        .with_timezone(&Utc)
}

fn day(year: i32, month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
}

fn date_of(cell: (i32, u32, u32)) -> NaiveDate {
    day(cell.0, cell.1, cell.2)
}

fn early_close_at(minutes: u32) -> HolidayKind {
    HolidayKind::EarlyClose {
        close_ssm: minutes * 60,
    }
}

/// Asserts a date's row kind, that it is closed at the cutoff and open the
/// nanosecond before it, and that the bounded search still finds a session
/// inside the date rather than reporting the day absent.
fn assert_early_close(cell: (i32, u32, u32), minutes: u32, label: &str) {
    let calendar = cfe();
    let (hour, minute) = (minutes / 60, minutes % 60);
    let cutoff = ct(cell, (hour, minute, 0));

    assert_eq!(
        calendar.holiday_on(date_of(cell)).map(Holiday::kind),
        Some(early_close_at(minutes)),
        "{label}: {cell:?} carries the printed close"
    );
    assert!(
        calendar
            .is_open(cutoff - TimeDelta::nanoseconds(1))
            .expect("the coverage contract must answer a covered date"),
        "{label}: {cell:?} is open one nanosecond before its close"
    );
    assert!(
        !calendar
            .is_open(cutoff)
            .expect("the coverage contract must answer a covered date"),
        "{label}: {cell:?} is closed at its close (closes are end-exclusive)"
    );
    assert_eq!(
        calendar
            .session_bounds(ct(cell, (hour, minute - 1, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((ct(cell, (8, 30, 0)), cutoff)),
        "{label}: {cell:?} still begins at the normal regular open"
    );
    assert_eq!(
        calendar
            .candle_end(ct(cell, (hour, minute - 1, 0)), CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(cutoff),
        "{label}: {cell:?} candle edge is the printed close"
    );
    // The 15:00-16:00 CT extended window opens after the cutoff, so it is gone.
    assert!(
        !calendar
            .is_open(ct(cell, (15, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        "{label}: {cell:?} has no afternoon extended window"
    );
}

/// Asserts a date's row is `Closed` and no session at all belongs to it.
fn assert_closed(cell: (i32, u32, u32), label: &str) {
    let calendar = cfe();
    let date = date_of(cell);

    assert_eq!(
        calendar.holiday_on(date).map(Holiday::kind),
        Some(HolidayKind::Closed),
        "{label}: {cell:?} is a closure"
    );
    assert!(
        calendar
            .is_closed_trade_date(date, SessionKind::Both)
            .expect("the coverage contract must answer a covered date"),
        "{label}: {cell:?} has no session in either phase"
    );
    for probe in [(2, 0, 0), (10, 0, 0), (15, 30, 0)] {
        assert!(
            !calendar
                .is_open(ct(cell, probe))
                .expect("the coverage contract must answer a covered date"),
            "{label}: {cell:?} must be shut at {probe:?}"
        );
    }
}

#[test]
fn christmas_day_removes_the_whole_cfe_trading_day() {
    let calendar = cfe();
    let christmas = day(2026, 12, 25);

    assert_eq!(
        calendar.holiday_on(christmas).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(
        calendar
            .is_closed_trade_date(christmas, SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    for probe in [(2, 0, 0), (10, 0, 0), (15, 30, 0)] {
        assert!(
            !calendar
                .is_open(ct((2026, 12, 25), probe))
                .expect("the coverage contract must answer a covered date"),
            "CFE must be shut at {probe:?} on Christmas Day"
        );
    }
    // The Christmas-Eve evening leg belongs to trade date 2026-12-25, so the
    // closure removes it; the row is what deletes the prior evening.
    assert!(
        !calendar
            .is_open(ct((2026, 12, 24), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

#[test]
fn the_session_after_christmas_eve_is_the_sunday_evening_reopening() {
    let calendar = cfe();
    let next = calendar
        .next_session_after(ct((2026, 12, 24), (12, 20, 0)))
        .expect("the coverage contract must answer a covered date")
        .expect("a reopening must exist inside the bounded search");

    assert_eq!(next.0, ct((2026, 12, 27), (17, 0, 0)));
}

#[test]
fn martin_luther_king_day_closes_the_overnight_leg_at_1030_central() {
    let calendar = cfe();
    let cutoff = ct((2026, 1, 19), (10, 30, 0));

    assert_eq!(
        calendar.holiday_on(day(2026, 1, 19)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 10 * 3_600 + 30 * 60
        })
    );
    assert!(
        calendar
            .is_open(cutoff - TimeDelta::nanoseconds(1))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(cutoff)
            .expect("the coverage contract must answer a covered date")
    );
    // The Sunday-evening open is untouched: only the close moves.
    assert!(
        calendar
            .is_open(ct((2026, 1, 18), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .session_bounds(ct((2026, 1, 19), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2026, 1, 19), (8, 30, 0)), cutoff))
    );
    assert_eq!(
        calendar
            .candle_end(ct((2026, 1, 19), (9, 0, 0)), CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(cutoff)
    );
    // The 15:00-16:00 extended window opens after the cutoff, so it is gone.
    assert!(
        !calendar
            .is_open(ct((2026, 1, 19), (15, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

#[test]
fn good_friday_ends_the_cfe_trading_day_at_the_regular_open() {
    let calendar = cfe();
    let cutoff = ct((2026, 4, 3), (8, 30, 0));

    assert_eq!(
        calendar.holiday_on(day(2026, 4, 3)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 8 * 3_600 + 30 * 60
        })
    );
    assert!(
        calendar
            .is_open(cutoff - TimeDelta::nanoseconds(1))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(cutoff)
            .expect("the coverage contract must answer a covered date")
    );
    // 08:30 is also the regular open, so the regular session collapses to an
    // empty span and disappears rather than inverting.
    assert!(
        !calendar
            .is_open(ct((2026, 4, 3), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

#[test]
fn new_years_day_deletes_the_prior_evening_and_keeps_its_own() {
    let calendar = cfe();

    // The table's own claim is unchanged: 2026-01-01 ships as a `Closed` row.
    assert_eq!(
        calendar.holiday_on(day(2026, 1, 1)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    // The 2025 window now reaches 2025-12-31, so the trade-date question
    // answers rather than refusing: the closure deletes that Wednesday's
    // evening leg, which is trade date 2026-01-01's own first block.
    assert!(
        calendar
            .is_closed_trade_date(day(2026, 1, 1), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 12, 31), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        "New Year's Day removes the 2025-12-31 evening leg"
    );
    // Thursday 17:00 CT feeds trade date 2026-01-02 and survives: the evening
    // leg that belongs to the holiday is gone, the next trade date's is not.
    assert!(
        calendar
            .is_open(ct((2026, 1, 1), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the 2026-01-01 evening leg belongs to trade date 2026-01-02"
    );
}

#[test]
fn a_shortened_day_keeps_its_trade_date_and_the_evening_takes_the_next() {
    let calendar = cfe();

    assert_eq!(
        calendar
            .trade_date(ct((2026, 1, 19), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 1, 19))
    );
    assert_eq!(
        calendar
            .trade_date(ct((2026, 1, 19), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 1, 20))
    );
}

#[test]
fn the_thanksgiving_and_christmas_half_days_close_at_1215_central() {
    for cell in [
        (2025, 11, 28),
        (2025, 12, 24),
        (2026, 11, 27),
        (2026, 12, 24),
    ] {
        assert_early_close(cell, 12 * 60 + 15, "half day");
    }
}

/// The 2025 rows, one case each, from Cboe's own per-holiday notices.
///
/// Every entry states the printed close and the holiday's own date, so a row
/// that moves to the wrong date or loses its clip fails on the date it names
/// rather than on a count.
#[test]
fn every_2025_early_close_matches_the_notice_that_states_it() {
    for cell in [
        (2025, 1, 20),
        (2025, 2, 17),
        (2025, 5, 26),
        (2025, 6, 19),
        (2025, 9, 1),
        (2025, 11, 27),
    ] {
        assert_early_close(cell, 10 * 60 + 30, "2025 Monday/Thursday holiday");
    }
    // Independence Day 2025 fell on a Friday, so the half day is the Thursday
    // trade date that opened 2025-07-02 at 17:00 CT.
    assert_early_close((2025, 7, 3), 12 * 60 + 15, "Independence Day observed");
}

#[test]
fn every_2025_closure_is_a_date_its_notice_gives_no_session() {
    for (cell, label) in [
        ((2025, 1, 1), "New Year's Day"),
        ((2025, 4, 18), "Good Friday"),
        ((2025, 7, 4), "Independence Day"),
        ((2025, 12, 25), "Christmas Day"),
    ] {
        assert_closed(cell, label);
    }
}

/// 2025-04-18 is a closure, and the row that says so is not the 2026 shape.
///
/// Cboe's 2025 Good Friday notice prints no Friday close and no Friday trade
/// date, where its 2026 counterpart prints `ETH Close 8:30 AM` on the Friday
/// and is therefore an early close. This case pins the difference: the 2025
/// date is `Closed`, and the 2026 date is still `EarlyClose { 08:30 }`.
#[test]
fn good_friday_2025_is_a_closure_where_2026_is_an_early_close() {
    let calendar = cfe();

    assert_eq!(
        calendar.holiday_on(day(2025, 4, 18)).map(Holiday::kind),
        Some(HolidayKind::Closed),
        "the 2025 notice prints no Friday session"
    );
    assert_eq!(
        calendar.holiday_on(day(2026, 4, 3)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 8 * 3_600 + 30 * 60
        }),
        "the 2026 notice prints a Friday 08:30 close"
    );
    // The closure deletes the Thursday-evening leg that would have been trade
    // date 2025-04-18's own; the Thursday daytime session is untouched.
    assert!(
        calendar
            .is_open(ct((2025, 4, 17), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "Maundy Thursday still trades"
    );
    assert!(
        !calendar
            .is_open(ct((2025, 4, 17), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the evening leg into 2025-04-18 is deleted"
    );
    assert!(
        !calendar
            .is_open(ct((2025, 4, 18), (8, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

/// The National Day of Mourning: a trading day with no regular session.
///
/// The row ships one `Extended` block opening on the preceding local day, so
/// the day is open in that leg, closed through the ordinary regular window, and
/// closed from the 08:30 CT close onward — including the ordinary 15:00-16:00
/// CT extended window, which the notice does not carry.
#[test]
fn the_mourning_day_is_extended_only_and_ends_at_0830_central() {
    let calendar = cfe();
    let date = day(2025, 1, 9);
    let close = ct((2025, 1, 9), (8, 30, 0));

    let Some(HolidayKind::ReplacementBlocks(blocks)) = calendar.holiday_on(date).map(Holiday::kind)
    else {
        panic!("2025-01-09 must ship a replacement block set");
    };
    assert_eq!(
        blocks
            .iter()
            .map(|block| (
                block.kind(),
                block.open_day_offset(),
                block.open_ssm(),
                block.close_ssm()
            ))
            .collect::<Vec<_>>(),
        vec![(
            ExceptionBlockKind::Extended,
            -1,
            17 * 3_600,
            8 * 3_600 + 30 * 60
        )],
        "the notice states one extended session and no regular block"
    );

    // Open in the leg that opened 17:00 CT on 2025-01-08 ...
    assert!(
        calendar
            .is_open(ct((2025, 1, 8), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the extended leg runs through Wednesday evening"
    );
    // ... through the last open instant before the close ...
    assert!(
        calendar
            .is_open(close - TimeDelta::nanoseconds(1))
            .expect("the coverage contract must answer a covered date"),
        "08:30 CT minus one nanosecond is still open"
    );
    // ... and closed at the close, which is end-exclusive.
    assert!(
        !calendar
            .is_open(close)
            .expect("the coverage contract must answer a covered date"),
        "08:30 CT is the close and closes are end-exclusive"
    );
    // The ordinary regular window is deleted outright, not merely clipped.
    for probe in [(8, 35, 0), (10, 0, 0), (14, 0, 0)] {
        assert!(
            !calendar
                .is_open(ct((2025, 1, 9), probe))
                .expect("the coverage contract must answer a covered date"),
            "no regular session exists on 2025-01-09 at {probe:?}"
        );
    }
    // The ordinary afternoon extended window is gone too.
    assert!(
        !calendar
            .is_open(ct((2025, 1, 9), (15, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // It is a trading day, not a closure: the trade date still resolves.
    assert_eq!(
        calendar
            .trade_date(ct((2025, 1, 9), (8, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(date)
    );
    assert!(
        !calendar
            .is_closed_trade_date(date, SessionKind::Both)
            .expect("the coverage contract must answer a covered date"),
        "the mourning day is a trading day with no regular hours"
    );
    // Normal trading resumes with the next trade date, as the notice states.
    assert!(
        calendar
            .is_open(ct((2025, 1, 10), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the notice resumes a normal schedule for trade date 2025-01-10"
    );
    // The block set replaces the complete day, so the mourning day's own
    // session has already ended. 17:30 CT on 2025-01-09 is the ordinary
    // evening leg of the **next** trade date, and the crate says so rather
    // than extending the mourning day past its 08:30 CT close.
    assert_eq!(
        calendar
            .trade_date(ct((2025, 1, 9), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 1, 10)),
        "the Thursday-evening leg belongs to trade date 2025-01-10"
    );
}

#[test]
fn coverage_runs_from_the_2025_floor_to_the_end_of_the_published_2026_schedule() {
    let calendar = cfe();
    let coverage = calendar
        .holiday_coverage()
        .expect("CFE ships a built-in table");

    assert_eq!(coverage.first(), day(2025, 1, 1));
    assert_eq!(coverage.last(), day(2026, 12, 31));
    assert_eq!(
        calendar.holiday_on(
            coverage
                .first()
                .checked_sub_days(Days::new(1))
                .expect("a representable date")
        ),
        None
    );
    assert_eq!(
        calendar.holiday_on(
            coverage
                .last()
                .checked_add_days(Days::new(1))
                .expect("a representable date")
        ),
        None
    );
    // 2025-12-31 used to be below the window and is now inside it: the plain
    // normal week answers there rather than refusing.
    let last_2025 = ct((2025, 12, 31), (10, 0, 0));
    assert!(
        calendar
            .is_open(last_2025)
            .expect("2025-12-31 is inside the audited window"),
        "the 2025 window reaches its own last weekday"
    );
    assert_eq!(
        calendar
            .trade_date(last_2025)
            .expect("2025-12-31 is inside the audited window"),
        Some(day(2025, 12, 31))
    );
    // 2027-01-01 is a CFE holiday in fact, but Cboe has published no 2027
    // schedule, so the table must not reach past its window. The identity
    // refuses the date outright: it returns no answer where its holiday layer
    // has none. The normal week is still observable on the detached snapshot,
    // which claims no coverage at all.
    let outside = ct((2027, 1, 1), (10, 0, 0));
    assert!(
        matches!(
            calendar.is_open(outside),
            Err(CalendarQueryError::OutsideCoveredRange {
                date,
                ..
            }) if date == day(2027, 1, 1)
        ),
        "2027-01-01 is outside the audited 2026 window and must be refused"
    );
    assert!(
        calendar
            .without_holidays()
            .is_open(outside)
            .expect("a detached snapshot claims no coverage"),
        "the detached snapshot answers the pure normal week outside the built-in window"
    );
}

#[test]
fn detaching_the_table_restores_the_normal_week() {
    let calendar = cfe();
    let inside_the_removed_afternoon = ct((2026, 1, 19), (14, 0, 0));

    assert!(
        !calendar
            .is_open(inside_the_removed_afternoon)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .without_holidays()
            .is_open(inside_the_removed_afternoon)
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar.without_holidays().holiday_on(day(2026, 1, 19)),
        None
    );
    assert_eq!(calendar.without_holidays().holiday_coverage(), None);
}

#[test]
fn the_venue_and_the_key_answer_from_the_same_table() {
    let venue = calendar_for_exchange(Exchange::Cfe);
    let key = cfe();

    assert_eq!(venue.holiday_coverage(), key.holiday_coverage());
    for date in [
        day(2025, 1, 1),
        day(2025, 1, 9),
        day(2025, 4, 18),
        day(2025, 7, 3),
        day(2025, 11, 28),
        day(2025, 12, 24),
        day(2026, 1, 1),
        day(2026, 4, 3),
        day(2026, 9, 7),
        day(2026, 11, 27),
        day(2026, 12, 25),
    ] {
        assert_eq!(venue.holiday_on(date), key.holiday_on(date), "{date}");
    }
    // Every date in the window answers, through both identities, or refuses
    // with the same error: the venue is not a partial intersection here.
    let mut date = key
        .holiday_coverage()
        .expect("CFE ships a built-in table")
        .first();
    let last = key
        .holiday_coverage()
        .expect("CFE ships a built-in table")
        .last();
    while date <= last {
        assert_eq!(
            venue
                .is_closed_trade_date(date, SessionKind::Both)
                .map_err(|error| format!("{error:?}")),
            key.is_closed_trade_date(date, SessionKind::Both)
                .map_err(|error| format!("{error:?}")),
            "{date}"
        );
        date = date
            .checked_add_days(Days::new(1))
            .expect("the window stays inside the representable calendar");
    }
}

/// §4.1 case 4, as a negative over the whole window: the table ships only
/// closures, early closes and the one replacement day, and no late open.
///
/// The counts are per date, read back from the shipped table: 26 rows in all,
/// six closures, thirteen 10:30 CT early closes, five 12:15 CT ones, one 08:30
/// CT one and the single mourning replacement. No row is a `LateOpen` or a
/// `LateOpenAndEarlyClose`, which is the negative this test exists for, and the
/// three distinct early-close instants are counted separately so a row that
/// silently takes a neighbouring holiday's close fails here.
#[test]
fn the_window_ships_only_closures_early_closes_and_one_replacement() {
    let calendar = cfe();
    let coverage = calendar
        .holiday_coverage()
        .expect("CFE ships a built-in table");
    let mut closed = 0_usize;
    let mut early_ten_thirty = 0_usize;
    let mut early_twelve_fifteen = 0_usize;
    let mut early_eight_thirty = 0_usize;
    let mut replacements = 0_usize;
    let mut date = coverage.first();
    while date <= coverage.last() {
        match calendar.holiday_on(date).map(Holiday::kind) {
            None => {}
            Some(HolidayKind::Closed) => closed += 1,
            Some(HolidayKind::EarlyClose { close_ssm }) if close_ssm == 10 * 3_600 + 30 * 60 => {
                early_ten_thirty += 1;
            }
            Some(HolidayKind::EarlyClose { close_ssm }) if close_ssm == 12 * 3_600 + 15 * 60 => {
                early_twelve_fifteen += 1;
            }
            Some(HolidayKind::EarlyClose { close_ssm }) if close_ssm == 8 * 3_600 + 30 * 60 => {
                early_eight_thirty += 1;
            }
            Some(HolidayKind::ReplacementBlocks(_)) => replacements += 1,
            Some(other) => {
                panic!("{date} ships a kind CFE's 2025-2026 schedules do not state: {other:?}")
            }
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the coverage window stays inside the representable calendar");
    }
    assert_eq!(
        (
            closed,
            early_ten_thirty,
            early_twelve_fifteen,
            early_eight_thirty,
            replacements
        ),
        (6, 13, 5, 1, 1),
        "closures, 10:30 CT / 12:15 CT / 08:30 CT early closes and replacement rows, 2025-2026"
    );
}
