// SPDX-License-Identifier: MIT-0

//! Built-in holiday rows for Eurex (`eurex` venue, `eurex` and
//! `eurex_fixed_income` keys), 2026.
//!
//! Eurex publishes closures only — no early close and no late open — so §4.1
//! cases 2, 3 and 4 have nothing to exercise here and the block says so rather
//! than inventing a row. Coverage stops at 2026-12-31 because Eurex's 2027
//! calendar is published "on a preliminary and indicative basis", which
//! LAW-NO-FABRICATED-DATES keeps out of a runtime table; the last test is the
//! fence on that.

use chrono::{DateTime, Days, NaiveDate, TimeZone as _, Utc};
use chrono_tz::Europe;
use exchange_hours::{
    Exchange, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey, SessionKind,
    calendar_for_exchange, calendar_for_market_hours_key,
};

fn identities() -> [(&'static str, ExchangeCalendar); 3] {
    [
        ("Exchange::Eurex", calendar_for_exchange(Exchange::Eurex)),
        (
            "MarketHoursKey::Eurex",
            calendar_for_market_hours_key(MarketHoursKey::Eurex),
        ),
        (
            "MarketHoursKey::EurexFixedIncome",
            calendar_for_market_hours_key(MarketHoursKey::EurexFixedIncome),
        ),
    ]
}

fn cet(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    Europe::Berlin
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous Berlin instant")
        .with_timezone(&Utc)
}

fn day(year: i32, month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
}

#[test]
fn every_eurex_identity_ships_the_same_seven_closures() {
    let closures = [
        day(2026, 1, 1),
        day(2026, 4, 3),
        day(2026, 4, 6),
        day(2026, 5, 1),
        day(2026, 12, 24),
        day(2026, 12, 25),
        day(2026, 12, 31),
    ];
    for (name, calendar) in identities() {
        for date in closures {
            assert_eq!(
                calendar.holiday_on(date).map(Holiday::kind),
                Some(HolidayKind::Closed),
                "{name} must be closed on {date}"
            );
            assert!(
                calendar.is_closed_trade_date(date, SessionKind::Both),
                "{name} on {date}"
            );
        }
    }
}

#[test]
fn the_christmas_closures_delete_their_trading_days() {
    for (name, calendar) in identities() {
        for probe in [(9, 0, 0), (12, 0, 0), (17, 0, 0)] {
            assert!(
                !calendar.is_open(cet((2026, 12, 24), probe)),
                "{name}: 24 December is a full trading closure, not a half day, at {probe:?}"
            );
            assert!(
                !calendar.is_open(cet((2026, 12, 25), probe)),
                "{name}: 25 December at {probe:?}"
            );
        }
        // 23 December is inside coverage with no row: audited normal.
        assert_eq!(calendar.holiday_on(day(2026, 12, 23)), None, "{name}");
        assert!(calendar.is_open(cet((2026, 12, 23), (12, 0, 0))), "{name}");
    }
}

#[test]
fn trading_resumes_on_the_first_open_day_after_the_christmas_block() {
    for (name, calendar) in identities() {
        let next = calendar
            .next_session_after(cet((2026, 12, 23), (21, 0, 0)))
            .expect("a reopening must exist inside the bounded search");
        assert_eq!(
            next.0.with_timezone(&Europe::Berlin).date_naive(),
            day(2026, 12, 28),
            "{name} must reopen on 28 December"
        );
    }
}

#[test]
fn new_years_day_is_closed_and_the_second_of_january_is_not() {
    for (name, calendar) in identities() {
        assert!(!calendar.is_open(cet((2026, 1, 1), (12, 0, 0))), "{name}");
        assert!(calendar.is_open(cet((2026, 1, 2), (12, 0, 0))), "{name}");
    }
}

#[test]
fn the_indicative_2027_calendar_does_not_ship() {
    for (name, calendar) in identities() {
        let coverage = calendar
            .holiday_coverage()
            .unwrap_or_else(|| panic!("{name} ships a built-in table"));
        assert_eq!(coverage.first(), day(2026, 1, 1), "{name}");
        assert_eq!(coverage.last(), day(2026, 12, 31), "{name}");
        assert_eq!(
            calendar.holiday_on(
                coverage
                    .first()
                    .checked_sub_days(Days::new(1))
                    .expect("a representable date")
            ),
            None,
            "{name}"
        );
        // 2027-01-01 appears on Eurex's indicative calendar and is a Friday, so
        // an accidental row would be visible; the table must not reach it.
        assert_eq!(
            calendar.holiday_on(
                coverage
                    .last()
                    .checked_add_days(Days::new(1))
                    .expect("a representable date")
            ),
            None,
            "{name}"
        );
        let outside = cet((2027, 1, 1), (12, 0, 0));
        assert_eq!(
            calendar.is_open(outside),
            calendar.without_holidays().is_open(outside),
            "{name} must answer the pure normal week outside its coverage window"
        );
        assert!(calendar.is_open(outside), "{name}");
    }
}

#[test]
fn detaching_the_table_restores_the_normal_week() {
    for (name, calendar) in identities() {
        let good_friday = cet((2026, 4, 3), (12, 0, 0));
        assert!(!calendar.is_open(good_friday), "{name}");
        assert!(calendar.without_holidays().is_open(good_friday), "{name}");
        assert_eq!(
            calendar.without_holidays().holiday_coverage(),
            None,
            "{name}"
        );
    }
}
