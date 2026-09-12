// SPDX-License-Identifier: MIT-0

//! Built-in holiday rows for CFE (`cfe` venue, `cfe_vix` key), 2026.
//!
//! One table serves both identities, so every case below is asserted through
//! the key and the venue agreement is asserted once at the end.
//!
//! CFE has no late open in this window: every Cboe row whose regular cell reads
//! `None` stops the overnight leg early rather than starting it late, so §4.1
//! case 4 has nothing to exercise here and says so rather than inventing a row.

use chrono::{DateTime, Days, NaiveDate, TimeDelta, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, Exchange, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey,
    SessionKind, calendar_for_exchange, calendar_for_market_hours_key,
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

#[test]
fn christmas_day_removes_the_whole_cfe_trading_day() {
    let calendar = cfe();
    let christmas = day(2026, 12, 25);

    assert_eq!(
        calendar.holiday_on(christmas).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(calendar.is_closed_trade_date(christmas, SessionKind::Both));
    for probe in [(2, 0, 0), (10, 0, 0), (15, 30, 0)] {
        assert!(
            !calendar.is_open(ct((2026, 12, 25), probe)),
            "CFE must be shut at {probe:?} on Christmas Day"
        );
    }
    // The Christmas-Eve evening leg belongs to trade date 2026-12-25, so the
    // closure removes it; the row is what deletes the prior evening.
    assert!(!calendar.is_open(ct((2026, 12, 24), (17, 30, 0))));
}

#[test]
fn the_session_after_christmas_eve_is_the_sunday_evening_reopening() {
    let calendar = cfe();
    let next = calendar
        .next_session_after(ct((2026, 12, 24), (12, 20, 0)))
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
    assert!(calendar.is_open(cutoff - TimeDelta::nanoseconds(1)));
    assert!(!calendar.is_open(cutoff));
    // The Sunday-evening open is untouched: only the close moves.
    assert!(calendar.is_open(ct((2026, 1, 18), (18, 0, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2026, 1, 19), (9, 0, 0))),
        Some((ct((2026, 1, 19), (8, 30, 0)), cutoff))
    );
    assert_eq!(
        calendar.candle_end(ct((2026, 1, 19), (9, 0, 0)), CalendarResolution::Daily),
        Some(cutoff)
    );
    // The 15:00-16:00 extended window opens after the cutoff, so it is gone.
    assert!(!calendar.is_open(ct((2026, 1, 19), (15, 30, 0))));
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
    assert!(calendar.is_open(cutoff - TimeDelta::nanoseconds(1)));
    assert!(!calendar.is_open(cutoff));
    // 08:30 is also the regular open, so the regular session collapses to an
    // empty span and disappears rather than inverting.
    assert!(!calendar.is_open(ct((2026, 4, 3), (12, 0, 0))));
}

#[test]
fn new_years_day_deletes_the_prior_evening_and_keeps_its_own() {
    let calendar = cfe();

    assert!(calendar.is_closed_trade_date(day(2026, 1, 1), SessionKind::Both));
    // Wednesday 17:00 CT would have fed trade date 2026-01-01: gone.
    assert!(!calendar.is_open(ct((2025, 12, 31), (17, 30, 0))));
    // Thursday 17:00 CT feeds trade date 2026-01-02: untouched.
    assert!(calendar.is_open(ct((2026, 1, 1), (17, 30, 0))));
}

#[test]
fn a_shortened_day_keeps_its_trade_date_and_the_evening_takes_the_next() {
    let calendar = cfe();

    assert_eq!(
        calendar.trade_date(ct((2026, 1, 19), (9, 0, 0))),
        Some(day(2026, 1, 19))
    );
    assert_eq!(
        calendar.trade_date(ct((2026, 1, 19), (17, 30, 0))),
        Some(day(2026, 1, 20))
    );
}

#[test]
fn the_thanksgiving_and_christmas_half_days_close_at_1215_central() {
    let calendar = cfe();
    for date in [(2026, 11, 27), (2026, 12, 24)] {
        let cutoff = ct(date, (12, 15, 0));
        assert_eq!(
            calendar
                .holiday_on(day(date.0, date.1, date.2))
                .map(Holiday::kind),
            Some(HolidayKind::EarlyClose {
                close_ssm: 12 * 3_600 + 15 * 60
            }),
            "{date:?} is a Cboe half day"
        );
        assert!(
            calendar.is_open(cutoff - TimeDelta::nanoseconds(1)),
            "{date:?}"
        );
        assert!(!calendar.is_open(cutoff), "{date:?}");
    }
}

#[test]
fn coverage_stops_at_the_end_of_the_published_2026_schedule() {
    let calendar = cfe();
    let coverage = calendar
        .holiday_coverage()
        .expect("CFE ships a built-in table");

    assert_eq!(coverage.first(), day(2026, 1, 1));
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
    // 2027-01-01 is a CFE holiday in fact, but Cboe has published no 2027
    // schedule, so the table must not reach past its window.
    let outside = ct((2027, 1, 1), (10, 0, 0));
    assert!(calendar.is_open(outside));
    assert_eq!(
        calendar.is_open(outside),
        calendar.without_holidays().is_open(outside)
    );
}

#[test]
fn detaching_the_table_restores_the_normal_week() {
    let calendar = cfe();
    let inside_the_removed_afternoon = ct((2026, 1, 19), (14, 0, 0));

    assert!(!calendar.is_open(inside_the_removed_afternoon));
    assert!(
        calendar
            .without_holidays()
            .is_open(inside_the_removed_afternoon)
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
        day(2026, 1, 1),
        day(2026, 4, 3),
        day(2026, 9, 7),
        day(2026, 11, 27),
        day(2026, 12, 25),
    ] {
        assert_eq!(venue.holiday_on(date), key.holiday_on(date), "{date}");
    }
}
