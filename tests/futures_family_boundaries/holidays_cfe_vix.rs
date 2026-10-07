// SPDX-License-Identifier: MIT-0

//! Built-in holiday rows for CFE (`cfe` venue, `cfe_vix` key) over three
//! audited windows: 2014-12-24..2015-01-02, 2015-02-15..2016-01-20 and
//! 2017-04-10 through 2026.
//!
//! One table serves both identities, so every case below is asserted through
//! the key and the venue agreement is asserted once at the end.
//!
//! CFE's pre-migration era adds one shape the post-2018 windows never exercise:
//! on the 2014-06-22 grid the next business day's session begins 15:30 CT the
//! previous afternoon, but every pre-migration holiday chart and notice prints
//! the day-after column as beginning 5:00 p.m. on the holiday, so the day after
//! each Mon-Thu holiday ships a `LateOpen` row (a `LateOpenAndEarlyClose` where
//! the day after is itself a half day). The 2015 windows test that shape
//! directly, and the 2017-2018 companions are asserted beside the rows they
//! accompany.
//!
//! The 2025 rows are not copies of the 2026 shapes and the suite says so: Good
//! Friday 2025-04-18 is a closure where 2026-04-03 is an early close, and
//! 2025-01-09 is the only `ReplacementBlocks` row the table ships. The
//! pre-2025 rows are the 2017 rules-page shapes, the recovered 2014-2016 press
//! releases and information circulars, and the per-holiday notices of
//! 2018-2024, tested per year band below, with 2017-07-03 the windows' one
//! `Unsourced` date.

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

/// The pre-migration half of [`assert_early_close`], for the 2017 rows.
///
/// The grid those rows sit on is the one the 2017 captures print: the next
/// trade date's extended hours begin at the 15:30 CT tail session of the
/// holiday itself (`3:30 p.m. (previous day) to 8:30 a.m.`), so an instant at
/// 15:30 on one of these holidays belongs to the *following* trade date and
/// answers open, and the daily envelope at 09:00 is the overnight leg that
/// opened the prior evening rather than a regular session. The assertions
/// name that shape instead of borrowing the 2021-grid probes.
fn assert_early_close_pre_migration(cell: (i32, u32, u32), minutes: u32, label: &str) {
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
    // Dead after the printed close: 13:00 CT is past both instants the era
    // prints (10:30 and 12:15).
    assert!(
        !calendar
            .is_open(ct(cell, (13, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "{label}: {cell:?} is dead after its printed close"
    );
}

/// The shortened morning still answers for the holiday's own trade date.
///
/// Split out of [`assert_early_close_pre_migration`] because one shipped row
/// cannot make this probe: 2017-07-04's overnight leg opened on the withheld
/// 2017-07-03, so every instant inside that leg refuses on the withheld day
/// rather than answering.
fn assert_pre_migration_morning(cell: (i32, u32, u32), label: &str) {
    let calendar = cfe();
    assert!(
        calendar
            .is_open(ct(cell, (9, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "{label}: {cell:?} trades its shortened morning"
    );
    assert_eq!(
        calendar
            .trade_date(ct(cell, (9, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(date_of(cell)),
        "{label}: {cell:?} still owns its morning"
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
    // The closure's own wrap-opening day sits inside the audited window for
    // every row this helper asserts: the earliest is 2017-04-14, whose wrap
    // opened 2017-04-13, four days inside the 2017-04-10 window start. (The
    // 2014-12-25 closure sits one window day from the edge and its queries
    // refuse naming the out-of-window opening day; it is asserted beside its
    // own row instead.)
    assert!(
        calendar
            .is_closed_trade_date(date, SessionKind::Both)
            .expect("the coverage contract must answer a covered date"),
        "{label}: {cell:?} has no session in either phase"
    );
    // The probes are hours the holiday trade date itself owns on both era
    // grids: the overnight leg and the regular window. 15:30 CT is
    // deliberately absent — on the pre-migration grid that instant belongs to
    // the *next* trade date's opening tail (the rules page's own
    // `3:30 p.m. (previous day) to 8:30 a.m.` cell), so it is not an hour the
    // closure answers for.
    for probe in [(2, 0, 0), (10, 0, 0), (12, 0, 0)] {
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

/// The pre-2025 Monday/Thursday and mid-week floating holidays, one case per
/// row, from the rules page (2017) and the per-holiday notices (2018-2024).
///
/// Every entry states the printed 10:30 CT close, so a row that moves to the
/// wrong date or loses its clip fails on the date it names rather than on a
/// count.
#[test]
fn every_pre_2025_floating_holiday_stops_the_overnight_leg_at_1030_central() {
    // 2017 sits on the pre-migration grid, so its rows read through the
    // era helper below.
    // 2017-07-04 is asserted in the withheld-day test instead: even its
    // cutoff probes walk into the leg that opened on the withheld
    // 2017-07-03, so the helper's instant probes refuse there.
    for cell in [(2017, 5, 29), (2017, 9, 4), (2017, 11, 23)] {
        assert_early_close_pre_migration(cell, 10 * 60 + 30, "2017 rules-page holiday");
    }
    // The 2017-07-04 morning is probed in the withheld-day test instead: its
    // overnight leg opened on the withheld 2017-07-03 and refuses there.
    for cell in [(2017, 5, 29), (2017, 9, 4), (2017, 11, 23)] {
        assert_pre_migration_morning(cell, "2017 rules-page holiday");
    }
    // 2018-01-15 and 2018-02-19 precede the 2018-02-25 migration, so they sit
    // on the pre-migration grid with the 2017 rows.
    for cell in [(2018, 1, 15), (2018, 2, 19)] {
        assert_early_close_pre_migration(cell, 10 * 60 + 30, "pre-migration 2018 holiday");
    }
    for cell in [(2018, 1, 15), (2018, 2, 19)] {
        assert_pre_migration_morning(cell, "pre-migration 2018 holiday");
    }
    for cell in [
        (2018, 5, 28),
        (2018, 7, 4),
        (2018, 9, 3),
        (2018, 11, 22),
        (2019, 1, 21),
        (2019, 2, 18),
        (2019, 5, 27),
        (2019, 7, 4),
        (2019, 9, 2),
        (2019, 11, 28),
        (2020, 1, 20),
        (2020, 2, 17),
        (2020, 5, 25),
        (2020, 7, 3),
        (2020, 9, 7),
        (2020, 11, 26),
        (2021, 1, 18),
        (2021, 2, 15),
        (2021, 5, 31),
        (2021, 7, 5),
        (2021, 9, 6),
        (2021, 11, 25),
        (2022, 1, 17),
        (2022, 2, 21),
        (2022, 5, 30),
        (2022, 6, 20),
        (2022, 7, 4),
        (2022, 9, 5),
        (2022, 11, 24),
        (2023, 1, 16),
        (2023, 2, 20),
        (2023, 5, 29),
        (2023, 6, 19),
        (2023, 7, 4),
        (2023, 9, 4),
        (2023, 11, 23),
        (2024, 1, 15),
        (2024, 2, 19),
        (2024, 5, 27),
        (2024, 6, 19),
        (2024, 7, 4),
        (2024, 9, 2),
        (2024, 11, 28),
    ] {
        assert_early_close(cell, 10 * 60 + 30, "pre-2025 Monday/Thursday holiday");
    }
}

/// The pre-2025 half days: the Thanksgiving Fridays, the December eves and the
/// Independence Day eves the notices state at 12:15 CT. 2017-11-24 and
/// 2015-11-27 are absent from this list on purpose: their charts print the
/// Friday leg beginning 5:00 p.m. CT on the holiday, so they ship
/// `LateOpenAndEarlyClose` rows and are asserted in
/// [`the_pre_migration_half_days_begin_1700_on_the_holiday`] below.
#[test]
fn every_pre_2025_half_day_closes_at_1215_central() {
    for cell in [
        (2015, 12, 24),
        (2018, 11, 23),
        (2018, 12, 24),
        (2019, 7, 3),
        (2019, 11, 29),
        (2019, 12, 24),
        (2020, 11, 27),
        (2020, 12, 24),
        (2021, 11, 26),
        (2022, 11, 25),
        (2023, 7, 3),
        (2023, 11, 24),
        (2024, 7, 3),
        (2024, 11, 29),
        (2024, 12, 24),
    ] {
        assert_early_close(cell, 12 * 60 + 15, "pre-2025 half day");
    }
}

/// The pre-migration half days whose charts print the Friday leg beginning
/// 5:00 p.m. CT on the holiday: both boundaries move, so the row is
/// `LateOpenAndEarlyClose`, and the 15:30-17:00 stretch on the holiday itself
/// — which the standing grid would have traded — is shut.
#[test]
fn the_pre_migration_half_days_begin_1700_on_the_holiday() {
    let calendar = cfe();
    for (cell, holiday) in [
        ((2017, 11, 24), (2017, 11, 23)),
        ((2015, 11, 27), (2015, 11, 26)),
    ] {
        assert_eq!(
            calendar.holiday_on(date_of(cell)).map(Holiday::kind),
            Some(HolidayKind::LateOpenAndEarlyClose {
                open_ssm: 17 * 3_600,
                close_ssm: 12 * 3_600 + 15 * 60
            }),
            "{cell:?} carries the chart's own Friday column"
        );
        // The holiday afternoon stretch the standing grid would have traded
        // (15:30-17:00 CT) is shut; the leg begins at 17:00.
        assert!(
            !calendar
                .is_open(ct(holiday, (16, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{holiday:?} must be shut inside the 15:30-17:00 stretch"
        );
        assert!(
            calendar
                .is_open(ct(holiday, (17, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{holiday:?}'s 17:00 CT resume opens the Friday leg"
        );
        // The Friday still trades its shortened morning and closes 12:15.
        assert!(
            calendar
                .is_open(ct(cell, (9, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{cell:?} trades its shortened morning"
        );
        assert!(
            !calendar
                .is_open(ct(cell, (12, 15, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{cell:?} is closed at its 12:15 CT close"
        );
    }
}

/// Every pre-2025 closure: the Good Fridays the notices close outright, the
/// Monday-Thursday New Year's Days and Christmases whose chart prints no
/// holiday-day session, and the observed Fridays and Mondays that replace a
/// weekend holiday.
#[test]
fn every_pre_2025_closure_is_a_date_its_document_gives_no_session() {
    for (cell, label) in [
        ((2017, 4, 14), "Good Friday 2017 (rules page Friday chart)"),
        ((2017, 12, 25), "Christmas 2017 (Monday-Thursday chart)"),
        ((2018, 1, 1), "New Year's Day 2018"),
        ((2018, 3, 30), "Good Friday 2018"),
        ((2018, 12, 25), "Christmas 2018"),
        ((2019, 1, 1), "New Year's Day 2019"),
        ((2019, 4, 19), "Good Friday 2019"),
        ((2019, 12, 25), "Christmas 2019"),
        ((2020, 1, 1), "New Year's Day 2020"),
        ((2020, 4, 10), "Good Friday 2020"),
        ((2020, 12, 25), "Christmas 2020"),
        ((2021, 1, 1), "New Year's Day 2021"),
        ((2021, 12, 24), "Christmas observed Friday 2021"),
        ((2022, 4, 15), "Good Friday 2022"),
        ((2022, 12, 26), "Christmas observed Monday 2022"),
        ((2023, 1, 2), "New Year's Day observed Monday 2023"),
        ((2023, 12, 25), "Christmas 2023"),
        ((2024, 1, 1), "New Year's Day 2024"),
        ((2024, 3, 29), "Good Friday 2024"),
        ((2024, 12, 25), "Christmas 2024"),
    ] {
        assert_closed(cell, label);
    }
}

/// The two pre-2025 Good Fridays that trade to the regular open, where the
/// overnight leg stops at 08:30 CT and no regular session runs — the 2026
/// shape, on the notices' own tables.
#[test]
fn good_friday_2021_and_2023_end_the_overnight_leg_at_the_regular_open() {
    let calendar = cfe();
    for cell in [(2021, 4, 2), (2023, 4, 7)] {
        let cutoff = ct(cell, (8, 30, 0));
        assert_eq!(
            calendar.holiday_on(date_of(cell)).map(Holiday::kind),
            Some(HolidayKind::EarlyClose {
                close_ssm: 8 * 3_600 + 30 * 60
            }),
            "{cell:?}: the notice stops the Thursday-evening leg at the regular open"
        );
        assert!(
            calendar
                .is_open(cutoff - TimeDelta::nanoseconds(1))
                .expect("the coverage contract must answer a covered date"),
            "{cell:?} is open one nanosecond before 08:30"
        );
        assert!(
            !calendar
                .is_open(cutoff)
                .expect("the coverage contract must answer a covered date"),
            "{cell:?} is closed at 08:30 (closes are end-exclusive)"
        );
        assert!(
            !calendar
                .is_open(ct(cell, (12, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{cell:?} runs no regular session"
        );
    }
}

/// The wrap deletions the closures and half days state: an evening leg whose
/// trade date is closed does not exist, and one whose trade date trades does.
#[test]
fn the_prior_evening_leg_exists_exactly_when_its_trade_date_trades() {
    let calendar = cfe();
    // Evenings deleted by the next day's closure: the Christmas and New Year
    // Monday-Thursday reopen on the holiday itself, and the 2021 observed
    // Friday deletes the Thursday-evening leg outright.
    for cell in [
        (2017, 12, 24),
        (2018, 12, 24),
        (2019, 12, 24),
        (2020, 12, 24),
        (2020, 12, 31),
        (2021, 12, 23),
        (2024, 12, 24),
        (2018, 12, 31),
    ] {
        assert!(
            !calendar
                .is_open(ct(cell, (17, 30, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{cell:?}'s evening leg belongs to a closed trade date and must be deleted"
        );
    }
    // Evenings that trade: the next trade date is open, so its own opening leg
    // runs from 17:00 CT on these half days. Only Monday-Thursday rows appear:
    // no CFE grid runs a Friday-evening leg — the week's last trade date ends
    // at its own afternoon extended close and the next opens Sunday 17:00 CT —
    // and 2017-11-24 is further absent because the pre-migration Friday
    // envelope ended at its own Saturday-morning close.
    for cell in [(2019, 7, 3), (2022, 11, 24), (2023, 7, 3), (2024, 7, 3)] {
        assert!(
            calendar
                .is_open(ct(cell, (17, 30, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{cell:?}'s evening leg is the next trade date's own and must be open"
        );
    }
}

/// The 2017 rules-page rows follow the chart's own shapes: the Friday chart
/// deletes Good Friday outright, and the Monday-Thursday Christmas chart
/// deletes the trade date and the Sunday-evening leg with it — December 24,
/// 2017 was a Sunday, so the `typically 12:15` Christmas-Eve default had no
/// session to shorten.
#[test]
fn the_2017_rules_page_rows_key_the_2017_calendar() {
    let calendar = cfe();
    // Good Friday 2017-04-14: Extended None / Regular None deletes the day and
    // the Thursday-evening leg with it.
    assert!(
        !calendar
            .is_open(ct((2017, 4, 13), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the closure deletes the 2017-04-13 evening leg"
    );
    // Christmas 2017-12-25: the Sunday-evening leg would have been trade date
    // 12-25's own and is deleted with the day.
    assert!(
        !calendar
            .is_open(ct((2017, 12, 24), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the Sunday-evening leg into 2017-12-25 is deleted"
    );
    // The reopen is 17:00 CT on the holiday itself, which is trade date
    // 2017-12-26's opening leg — and the chart's day-after column states that
    // beginning outright, so 2017-12-26's LateOpen row makes the 15:30-17:00
    // stretch shut and 17:00 the first open.
    assert!(
        !calendar
            .is_open(ct((2017, 12, 25), (16, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the 2017-12-25 afternoon stretch the standing grid would have traded is shut"
    );
    assert!(
        calendar
            .is_open(ct((2017, 12, 25), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the 2017-12-25 evening leg belongs to trade date 2017-12-26"
    );
    // The Thanksgiving chart's Friday moves 2017-11-24's close to 12:15 and
    // its first open to 17:00 CT on the holiday; the day still trades its
    // morning (asserted beside its own row).
    assert!(
        calendar
            .is_open(ct((2017, 11, 24), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "2017-11-24 trades its shortened morning"
    );
}

/// 2017-07-03 is the window's one `Unsourced` date: the rules page states the
/// eve close only as a default and no controlling circular survives, so the
/// day is withheld — not claimed normal and not claimed closed — and the
/// identity-backed query refuses it (LAW-COVERAGE).
#[test]
fn the_withheld_2017_july_3_date_is_unsourced_not_normal() {
    let calendar = cfe();

    assert_eq!(
        calendar.holiday_on(day(2017, 7, 3)).map(Holiday::kind),
        Some(HolidayKind::Unsourced),
        "2017-07-03 withholds its answer"
    );
    assert!(matches!(
        calendar.is_open(ct((2017, 7, 3), (10, 0, 0))),
        Err(CalendarQueryError::UnresolvedGap { date, .. }) if date == day(2017, 7, 3)
    ));
    // The withheld day does not leak onto its neighbours' rows: July 4 carries
    // the rules page's 10:30 early close (asserted below through its own row)
    // and July 5 carries the chart's own July 5 column — its leg begins 5:00
    // p.m. CT on the holiday, so the day ships a `LateOpen` row rather than
    // reading the standing 15:30 beginning.
    assert_eq!(
        calendar.holiday_on(day(2017, 7, 5)).map(Holiday::kind),
        Some(HolidayKind::LateOpen {
            open_ssm: 17 * 3_600
        }),
    );
    // And the withholding reaches one day sideways, exactly as far as the
    // sessions that opened on it: July 4's overnight leg opened 17:00 CT on
    // the withheld 2017-07-03, and a trade date's complete session set
    // includes that leg, so **every** instant query on 2017-07-04 — mid-leg,
    // one nanosecond before the row's own 10:30 close, and in the afternoon
    // tail — refuses naming the withheld day. The crate never reads the
    // withholding as an open or closed grid, and the row layer itself still
    // answers.
    assert_eq!(
        calendar.holiday_on(day(2017, 7, 4)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 10 * 3_600 + 30 * 60
        }),
    );
    for probe in [(9, 0, 0), (10, 29, 59), (15, 45, 0)] {
        assert!(
            matches!(
                calendar.is_open(ct((2017, 7, 4), probe)),
                Err(CalendarQueryError::UnresolvedGap { date, .. }) if date == day(2017, 7, 3)
            ),
            "2017-07-04 at {probe:?} must refuse on the withheld opening day"
        );
    }
    // The reach is exactly two days: 2017-07-05's trading day still includes
    // the overnight leg that opened on the withheld 2017-07-03, so its trade
    // date refuses naming the withheld day — while noon itself sits inside the
    // Wednesday regular session, which opens on the sourced 2017-07-05 and
    // answers, and 2017-07-06 — whose chain stops on sourced days — answers
    // again. The row layer answers throughout.
    // With the July 5 LateOpen row shipped, the day's chain derivation reads
    // the July 4 row, whose own chain reads the withheld July 3, so noon
    // refuses there too — the row layer still answers, and the answer resumes
    // on 2017-07-06, whose chain stops on the sourced 2017-07-05 17:00 open.
    assert!(matches!(
        calendar.is_open(ct((2017, 7, 5), (12, 0, 0))),
        Err(CalendarQueryError::UnresolvedGap { date, .. }) if date == day(2017, 7, 3)
    ));
    assert!(matches!(
        calendar.trade_date(ct((2017, 7, 5), (12, 0, 0))),
        Err(CalendarQueryError::UnresolvedGap { date, .. }) if date == day(2017, 7, 3)
    ));
    assert!(
        calendar
            .is_open(ct((2017, 7, 6), (12, 0, 0)))
            .expect("2017-07-06's session chain stops on sourced days"),
        "the answer resumes on 2017-07-06"
    );
}

/// The 2014 window rows recovered on 2026-10-06 UTC, from the CBOE Holdings
/// press release of 2014-12-16. The window (2014-12-24..2015-01-02) probes the
/// pre-migration grid directly, and the shapes are the release's own printed
/// columns — including the day-after legs beginning 5:00 p.m. CT on the
/// holiday, which ship as `LateOpen` rows.
#[test]
fn the_2014_window_rows_answer_from_the_recovered_release() {
    let calendar = cfe();

    // Christmas Eve 2014-12-24: the release's CFE column closes VX at 12:15
    // p.m. CT. Morning instants sit inside the leg that opened 2014-12-23
    // (outside the window) and refuse; the regular morning answers.
    assert_eq!(
        calendar.holiday_on(day(2014, 12, 24)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60
        }),
    );
    // The early close's chain derivation reads the leg that opened
    // 2014-12-23, outside the window, so every instant query on the day
    // refuses naming it while the row layer answers.
    for probe in [(9, 0, 0), (12, 15, 0), (13, 0, 0)] {
        assert!(
            matches!(
                calendar.is_open(ct((2014, 12, 24), probe)),
                Err(CalendarQueryError::OutsideCoveredRange { date, .. })
                    if date == day(2014, 12, 23)
            ),
            "2014-12-24 at {probe:?} refuses on the out-of-window opening day"
        );
    }
    // Christmas Day 2014-12-25: no session belongs to the day; the
    // Wednesday-evening wrap is deleted with it.
    assert_eq!(
        calendar.holiday_on(day(2014, 12, 25)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    // The closure's own query walks the deleted wrap, which opened
    // 2014-12-23 outside the window, so the instant and closure queries
    // refuse naming it; the row layer answers.
    assert!(matches!(
        calendar.is_closed_trade_date(day(2014, 12, 25), SessionKind::Both),
        Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == day(2014, 12, 23)
    ));
    for probe_date in [(2014, 12, 24), (2014, 12, 25)] {
        assert!(
            matches!(
                calendar.is_open(ct(probe_date, (16, 0, 0))),
                Err(CalendarQueryError::OutsideCoveredRange { date, .. })
                    if date == day(2014, 12, 23)
            ),
            "{probe_date:?} at 16:00 CT refuses on the out-of-window opening day"
        );
    }
    assert!(
        calendar
            .is_open(ct((2014, 12, 25), (17, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the printed 17:00 CT resume opens the 2014-12-26 leg"
    );
    assert_eq!(
        calendar.holiday_on(day(2014, 12, 26)).map(Holiday::kind),
        Some(HolidayKind::LateOpen {
            open_ssm: 17 * 3_600
        }),
    );
    // New Year's Day 2015-01-01: no session belongs to the day; the
    // 2014-12-31 evening wrap is deleted with it.
    assert_eq!(
        calendar.holiday_on(day(2015, 1, 1)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(
        calendar
            .is_closed_trade_date(day(2015, 1, 1), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2014, 12, 31), (16, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the release closes VX from 15:15 CT on 2014-12-31 until 17:00 CT on the holiday"
    );
    assert!(
        calendar
            .is_open(ct((2015, 1, 1), (17, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the printed 17:00 CT resume opens the 2015-01-02 leg"
    );
    assert_eq!(
        calendar.holiday_on(day(2015, 1, 2)).map(Holiday::kind),
        Some(HolidayKind::LateOpen {
            open_ssm: 17 * 3_600
        }),
    );
    assert!(
        calendar
            .is_open(ct((2015, 1, 2), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "2015-01-02 trades its regular session"
    );
}

/// The spring-and-summer 2015 rows, from the 2015 press releases and the CFE
/// information circulars IC15-011, IC15-023 and IC15-028. The second window
/// (2015-02-15..2016-01-20) probes the pre-migration grid directly and the
/// shapes are the operators' own printed columns.
#[test]
fn the_2015_spring_rows_answer_from_the_releases_and_circulars() {
    let calendar = cfe();

    // Presidents' Day 2015-02-16: the release halts VX at 10:30 a.m. CT, and
    // the release itself names the Sunday-evening wrap (the 17:00 regular
    // open for business day Tuesday), which is why the window opens on
    // 2015-02-15 and the day answers like the 2017 Monday rows.
    assert_early_close_pre_migration((2015, 2, 16), 10 * 60 + 30, "2015 Presidents' Day release");
    assert_pre_migration_morning((2015, 2, 16), "2015 Presidents' Day release");
    // The Tuesday leg begins 17:00 CT on the holiday (the release's own
    // resume), so the 15:30-17:00 stretch is shut.
    assert!(
        !calendar
            .is_open(ct((2015, 2, 16), (16, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the 2015-02-16 stretch before the printed 17:00 resume is shut"
    );
    assert!(
        calendar
            .is_open(ct((2015, 2, 16), (17, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the printed 17:00 CT resume opens the 2015-02-17 leg"
    );
    assert_eq!(
        calendar.holiday_on(day(2015, 2, 17)).map(Holiday::kind),
        Some(HolidayKind::LateOpen {
            open_ssm: 17 * 3_600
        }),
    );
    // Good Friday 2015-04-03: the circular's own session table runs one
    // continuous extended session from 15:30 CT Thursday to the operator's
    // printed 08:15 CT Friday close, with Regular `None` — so the Thursday
    // evening trades straight through and the Friday closes at 08:15.
    assert_eq!(
        calendar.holiday_on(day(2015, 4, 3)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 8 * 3_600 + 15 * 60
        }),
    );
    for probe in [(2015, 4, 2, 16, 0), (2015, 4, 2, 23, 0), (2015, 4, 3, 7, 0)] {
        assert!(
            calendar
                .is_open(ct((probe.0, probe.1, probe.2), (probe.3, probe.4, 0)))
                .expect("the coverage contract must answer a covered date"),
            "the holiday session runs continuously from Thursday 15:30 CT: {probe:?}"
        );
    }
    assert!(
        !calendar
            .is_open(ct((2015, 4, 3), (8, 15, 0)))
            .expect("the coverage contract must answer a covered date"),
        "2015-04-03 is closed at the printed 08:15 CT close"
    );
    assert!(
        !calendar
            .is_open(ct((2015, 4, 3), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "no regular session runs on 2015-04-03"
    );
    // Independence Day observed Friday 2015-07-03: the circular states the
    // closure, the Thursday 15:15 close, and a normal Sunday reopen.
    assert_eq!(
        calendar.holiday_on(day(2015, 7, 3)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(
        calendar
            .is_closed_trade_date(day(2015, 7, 3), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2015, 7, 2), (16, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the Thursday-evening wrap is deleted with the closed Friday"
    );
    assert!(
        calendar
            .is_open(ct((2015, 7, 5), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the Sunday reopen is the normal 17:00 CT"
    );
}

/// The autumn-2015 rows and the 2015-12/2016-01 closures, from the IC15-039
/// information circular and the per-holiday releases: the Monday-holiday
/// shape, the combined Thanksgiving half day and the year-end deletions whose
/// Sunday reopenings are the normal 17:00 CT.
#[test]
fn the_2015_autumn_rows_and_the_2016_new_year_delete_their_dates() {
    let calendar = cfe();
    // Memorial Day and Labor Day 2015: the circulars print the Monday-holiday
    // shape (Sunday-evening leg to 10:30 a.m., Regular None) and the Tuesday
    // column beginning 5:00 p.m. CT on the holiday.
    for cell in [(2015, 5, 25), (2015, 9, 7)] {
        assert_early_close_pre_migration(cell, 10 * 60 + 30, "2015 Monday-holiday circular");
        assert_pre_migration_morning(cell, "2015 Monday-holiday circular");
    }
    for (cell, holiday) in [((2015, 5, 26), (2015, 5, 25)), ((2015, 9, 8), (2015, 9, 7))] {
        assert_eq!(
            calendar.holiday_on(date_of(cell)).map(Holiday::kind),
            Some(HolidayKind::LateOpen {
                open_ssm: 17 * 3_600
            }),
            "{cell:?} carries the circular's Tuesday column"
        );
        assert!(
            !calendar
                .is_open(ct(holiday, (16, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{holiday:?} must be shut inside the 15:30-17:00 stretch"
        );
        assert!(
            calendar
                .is_open(ct(holiday, (17, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{holiday:?}'s 17:00 CT resume opens the Tuesday leg"
        );
    }
    // Thanksgiving 2015: the Thursday row is the pre-migration 10:30 shape;
    // the Friday row is asserted with the other combined half days.
    assert_early_close_pre_migration((2015, 11, 26), 10 * 60 + 30, "2015 Thanksgiving");
    assert_pre_migration_morning((2015, 11, 26), "2015 Thanksgiving");
    // Christmas Eve 2015-12-24 closes 12:15 CT; Christmas Day 2015-12-25 and
    // New Year's Day 2016-01-01 delete their trade dates and prior-evening
    // wraps; the Sunday reopenings are the normal 17:00 CT.
    assert_early_close_pre_migration((2015, 12, 24), 12 * 60 + 15, "2015 Christmas Eve");
    assert_pre_migration_morning((2015, 12, 24), "2015 Christmas Eve");
    for (closure, wrap, resume) in [
        ((2015, 12, 25), (2015, 12, 24), (2015, 12, 27)),
        ((2016, 1, 1), (2015, 12, 31), (2016, 1, 3)),
    ] {
        assert_eq!(
            calendar.holiday_on(date_of(closure)).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{closure:?} is a closure"
        );
        assert!(
            calendar
                .is_closed_trade_date(date_of(closure), SessionKind::Both)
                .expect("the coverage contract must answer a covered date"),
            "{closure:?} has no session in either phase"
        );
        assert!(
            !calendar
                .is_open(ct(wrap, (16, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "the {wrap:?} evening stretch is deleted with the closed {closure:?}"
        );
        assert!(
            calendar
                .is_open(ct(resume, (17, 30, 0)))
                .expect("the coverage contract must answer a covered date"),
            "the {resume:?} Sunday reopen is the normal 17:00 CT"
        );
    }
}

/// MLK Jr. Day 2016 (the 2016-01-11 release) closes the recovered windows, and
/// the pre-migration Monday rows already inside the 2017-04-10 window gain
/// their charts' and notices' own Tuesday columns as `LateOpen` rows.
#[test]
fn the_2016_mlk_and_pre_migration_monday_rows_gain_their_tuesday_columns() {
    let calendar = cfe();
    // MLK Jr. Day 2016-01-18: the release's Monday-holiday shape, and the
    // Tuesday column beginning 5:00 p.m. CT on the holiday.
    assert_early_close_pre_migration((2016, 1, 18), 10 * 60 + 30, "2016 MLK release");
    assert_pre_migration_morning((2016, 1, 18), "2016 MLK release");
    assert_eq!(
        calendar.holiday_on(day(2016, 1, 19)).map(Holiday::kind),
        Some(HolidayKind::LateOpen {
            open_ssm: 17 * 3_600
        }),
    );
    assert!(
        !calendar
            .is_open(ct((2016, 1, 18), (16, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the 2016-01-18 stretch before the printed 17:00 resume is shut"
    );
    assert!(
        calendar
            .is_open(ct((2016, 1, 18), (17, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the printed 17:00 CT resume opens the 2016-01-19 leg"
    );
    // The pre-migration Monday rows already inside the 2017-04-10 window gain
    // their chart's own Tuesday columns as LateOpen rows.
    for (cell, holiday) in [
        ((2017, 5, 30), (2017, 5, 29)),
        ((2017, 9, 5), (2017, 9, 4)),
        ((2018, 1, 16), (2018, 1, 15)),
        ((2018, 2, 20), (2018, 2, 19)),
    ] {
        assert_eq!(
            calendar.holiday_on(date_of(cell)).map(Holiday::kind),
            Some(HolidayKind::LateOpen {
                open_ssm: 17 * 3_600
            }),
            "{cell:?} carries the chart's or notice's Tuesday column"
        );
        assert!(
            !calendar
                .is_open(ct(holiday, (16, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{holiday:?} must be shut inside the 15:30-17:00 stretch"
        );
        assert!(
            calendar
                .is_open(ct(holiday, (17, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{holiday:?}'s 17:00 CT resume opens the Tuesday leg"
        );
    }
}

/// Juneteenth enters the observed set in 2022: the 2022 notice's 10:30 row is
/// the first, and the operator's own 2021 notice states CFE traded the 2021
/// dates unadjusted, so they carry no row.
#[test]
fn juneteenth_enters_the_set_in_2022_and_2021_traded_unadjusted() {
    let calendar = cfe();

    assert_eq!(
        calendar.holiday_on(day(2021, 6, 18)).map(Holiday::kind),
        None,
        "C2021061701 states CFE traded Friday 2021-06-18 unadjusted"
    );
    assert_eq!(
        calendar.holiday_on(day(2021, 6, 21)).map(Holiday::kind),
        None,
        "C2021061701 states CFE traded Monday 2021-06-21 unadjusted"
    );
    assert_early_close((2022, 6, 20), 10 * 60 + 30, "first Juneteenth row");
    assert_eq!(
        calendar.holiday_on(day(2023, 6, 19)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 10 * 3_600 + 30 * 60
        }),
    );
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
fn coverage_runs_over_the_three_audited_windows() {
    let calendar = cfe();
    let coverage = calendar
        .holiday_coverage()
        .expect("CFE ships a built-in table");

    assert_eq!(coverage.first(), day(2014, 12, 24));
    assert_eq!(coverage.last(), day(2026, 12, 31));
    assert_eq!(
        coverage.windows(),
        vec![
            (day(2014, 12, 24), day(2015, 1, 2)),
            (day(2015, 2, 15), day(2016, 1, 20)),
            (day(2017, 4, 10), day(2026, 12, 31)),
        ],
        "the three audited windows stop at each unrecovered span's edge"
    );
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
    // Each window's own first day reaches back before the window for its
    // wrapped evening leg. 2014-12-24's leg opened 2014-12-23, below the first
    // window where only one flank exists, so the instant query refuses naming
    // it while the row layer answers. 2017-04-10's wrap opened 2017-04-09,
    // inside the span the second and third windows flank — so the sourced
    // normal week answers there beside the disclosed bridged residual, and the
    // trade date still resolves to the window's own first day. 2017-04-10
    // itself is audited normal; 2014-12-24 and 2015-02-15 carry their own
    // rows.
    assert_eq!(calendar.holiday_on(day(2017, 4, 10)), None);
    assert!(matches!(
        calendar.is_open(ct((2014, 12, 24), (7, 0, 0))),
        Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == day(2014, 12, 23)
    ));
    assert!(
        calendar
            .is_open(ct((2017, 4, 10), (7, 0, 0)))
            .expect("the bridged span answers the normal week it sources"),
        "the third window's own first day answers inside the wrap the bridged \
         span opened"
    );
    assert!(
        calendar
            .is_open(ct((2017, 4, 10), (12, 0, 0)))
            .expect("the regular session opens on the window's own first day"),
        "the window's first day answers inside its own regular session"
    );
    assert_eq!(
        calendar
            .trade_date(ct((2017, 4, 10), (12, 0, 0)))
            .expect("the window's first day answers inside its own regular session"),
        Some(day(2017, 4, 10))
    );
    assert!(
        calendar
            .is_open(ct((2017, 4, 11), (12, 0, 0)))
            .expect("2017-04-11's wrap opened inside the window"),
        "the window's second day answers like the rest of the window"
    );
    // The two interior spans sit between two audited windows each —
    // 2015-01-03..2015-02-14 (MLK 2015-01-19 inside) and 2016-01-21..2017-04-09
    // (the 2016 Presidents'-on arrangements and the early-2017 rows inside) —
    // so the bridged residual (2026-10-06 convention, #296) answers their
    // session questions from the sourced normal week while the holiday layer
    // stays honestly absent: no row, no audited normal, and the metadata
    // reports `HolidayWindowsBridged` rather than a verdict it cannot claim.
    for gap in [(2015, 1, 20), (2016, 6, 15), (2017, 2, 21)] {
        assert_eq!(
            calendar.holiday_on(date_of(gap)),
            None,
            "{} sits in an unaudited span and ships no row",
            date_of(gap)
        );
        assert!(
            calendar
                .is_open(ct(gap, (10, 0, 0)))
                .expect("the bridged span answers its sourced normal week"),
            "{} answers its session questions beside the disclosed residual",
            date_of(gap)
        );
        assert_eq!(
            calendar.coverage().coverage_on(date_of(gap)),
            exchange_hours::DateCoverage::OutsideCoveredRange,
            "{} withholds the complete-calendar claim",
            date_of(gap)
        );
    }
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

fn refusal_shape(error: &CalendarQueryError) -> (chrono::NaiveDate, &'static str) {
    // `CalendarQueryError` is `#[non_exhaustive]`.
    let kind = match error {
        CalendarQueryError::BeforeSupportFloor { .. } => "before-floor",
        CalendarQueryError::OutsideCoveredRange { .. } => "outside",
        CalendarQueryError::UnresolvedGap { .. } => "unresolved",
        CalendarQueryError::SearchExhausted { .. } => "exhausted",
        _ => "other",
    };
    (error.date(), kind)
}

#[test]
fn the_venue_and_the_key_answer_from_the_same_table() {
    let venue = calendar_for_exchange(Exchange::Cfe);
    let key = cfe();

    assert_eq!(venue.holiday_coverage(), key.holiday_coverage());
    for date in [
        day(2014, 12, 24),
        day(2014, 12, 25),
        day(2014, 12, 26),
        day(2015, 1, 1),
        day(2015, 2, 16),
        day(2015, 4, 3),
        day(2015, 7, 3),
        day(2015, 11, 27),
        day(2015, 12, 25),
        day(2016, 1, 18),
        day(2017, 7, 5),
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
    // The two identities label their refusals with their own source, so the
    // comparison is on the refusal's date and variant, not its rendering.
    while date <= last {
        assert_eq!(
            venue
                .is_closed_trade_date(date, SessionKind::Both)
                .map_err(|error| refusal_shape(&error)),
            key.is_closed_trade_date(date, SessionKind::Both)
                .map_err(|error| refusal_shape(&error)),
            "{date}"
        );
        date = date
            .checked_add_days(Days::new(1))
            .expect("the window stays inside the representable calendar");
    }
}

/// §4.1 case 4, as a negative over the whole coverage: the table ships only
/// closures, early closes, late opens, the one replacement day and the one
/// withheld date.
///
/// The counts are per date, read back from the shipped table: 140 rows in all,
/// thirty-one closures, sixty-seven 10:30 CT early closes, twenty-one 12:15 CT
/// ones, three 08:30 CT ones, the single pre-migration 08:15 CT Good Friday,
/// thirteen 17:00 CT late opens, two combined late-open-and-early-close half
/// days, the single mourning replacement and the single `Unsourced` date. The
/// distinct instants are counted separately so a row that silently takes a
/// neighbouring holiday's close fails here.
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
    let mut early_eight_fifteen = 0_usize;
    let mut late_open = 0_usize;
    let mut late_open_and_early_close = 0_usize;
    let mut replacements = 0_usize;
    let mut unsourced = 0_usize;
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
            Some(HolidayKind::EarlyClose { close_ssm }) if close_ssm == 8 * 3_600 + 15 * 60 => {
                early_eight_fifteen += 1;
            }
            Some(HolidayKind::LateOpen { open_ssm }) if open_ssm == 17 * 3_600 => {
                late_open += 1;
            }
            Some(HolidayKind::LateOpenAndEarlyClose {
                open_ssm,
                close_ssm,
            }) if open_ssm == 17 * 3_600 && close_ssm == 12 * 3_600 + 15 * 60 => {
                late_open_and_early_close += 1;
            }
            Some(HolidayKind::ReplacementBlocks(_)) => replacements += 1,
            Some(HolidayKind::Unsourced) => unsourced += 1,
            Some(other) => {
                panic!("{date} ships a kind CFE's schedules do not state: {other:?}")
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
            early_eight_fifteen,
            late_open,
            late_open_and_early_close,
            replacements,
            unsourced
        ),
        (31, 67, 21, 3, 1, 13, 2, 1, 1),
        "closures, 10:30/12:15/08:30/08:15 CT early closes, 17:00 late opens, combined \
         half-day rows, the mourning replacement and the withheld 2017-07-03, across the \
         three windows"
    );
}
