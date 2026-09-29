// SPDX-License-Identifier: MIT-0

//! Built-in holiday rows for Eurex (`eurex` venue, `eurex` and
//! `eurex_fixed_income` keys), 2010 through 2026.
//!
//! Eurex publishes closures only — no early close and no late open — so §4.1
//! cases 2, 3 and 4 have nothing to exercise here and the block says so rather
//! than inventing a row. Case 5's wrap removal and case 6's trade-date
//! consequence are **not** vacuous: `eurex_fixed_income` carries Eurex's
//! 22:00-22:10 CET post-trading phase, whose occurrence on local date `D`
//! belongs to trade date `D + 1`, so a closure deletes the preceding evening's
//! leg. The two benchmark-index identities run 02:15-22:00 CET and have no such
//! leg; that absence is fenced below rather than assumed. Coverage runs from
//! the 2010-01-01 support floor — the operator's own "all derivatives" rows,
//! keyed to its annual Trading Calendar editions 2010-2024 and to the Holiday
//! regulations page for 2025-2026, with the Christmas block ends trading-only
//! and Whit Monday 2015 the one extra trading-only closure — and stops at
//! 2026-12-31 because Eurex's 2027 calendar is published "on a preliminary and
//! indicative basis", which LAW-NO-FABRICATED-DATES keeps out of a runtime
//! table; the last test is the fence on that.

use chrono::{DateTime, Datelike as _, Days, NaiveDate, TimeZone as _, Utc};
use chrono_tz::Europe;
use exchange_hours::{
    CalendarQueryError, Exchange, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey,
    SessionKind, calendar_for_exchange, calendar_for_market_hours_key,
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

/// The two benchmark-index identities, whose normal-week history is sourced
/// from the 2010-01-01 floor. `eurex_fixed_income` shares the holiday table
/// but its own grid is carried below its 2018-11-15 horizon, so its
/// session-level queries refuse below that day while its row-level answers
/// reach the floor; tests that probe sessions below 2018-11-15 iterate this
/// list and fence the fixed-income refusal beside it.
fn benchmark_identities() -> [(&'static str, ExchangeCalendar); 2] {
    [
        ("Exchange::Eurex", calendar_for_exchange(Exchange::Eurex)),
        (
            "MarketHoursKey::Eurex",
            calendar_for_market_hours_key(MarketHoursKey::Eurex),
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

/// The 93 closures the Trading Calendar editions state for 2010-2024, in
/// document order. Every date is one the edition prints under `in all
/// derivatives`, weekday or not: 2015-12-26 fell on a Saturday and ships
/// because the operator stated it.
const CALENDAR_EDITION_CLOSURES: [(i32, u32, u32); 93] = [
    (2010, 1, 1),
    (2010, 4, 2),
    (2010, 4, 5),
    (2010, 12, 24),
    (2010, 12, 31),
    (2011, 4, 22),
    (2011, 4, 25),
    (2011, 12, 26),
    (2012, 4, 6),
    (2012, 4, 9),
    (2012, 5, 1),
    (2012, 12, 24),
    (2012, 12, 25),
    (2012, 12, 26),
    (2012, 12, 31),
    (2013, 1, 1),
    (2013, 3, 29),
    (2013, 4, 1),
    (2013, 5, 1),
    (2013, 12, 24),
    (2013, 12, 25),
    (2013, 12, 26),
    (2013, 12, 31),
    (2014, 1, 1),
    (2014, 4, 18),
    (2014, 4, 21),
    (2014, 5, 1),
    (2014, 12, 24),
    (2014, 12, 25),
    (2014, 12, 26),
    (2014, 12, 31),
    (2015, 1, 1),
    (2015, 4, 3),
    (2015, 4, 6),
    (2015, 5, 1),
    (2015, 5, 25),
    (2015, 12, 24),
    (2015, 12, 25),
    (2015, 12, 26),
    (2015, 12, 31),
    (2016, 1, 1),
    (2016, 3, 25),
    (2016, 3, 28),
    (2016, 12, 26),
    (2017, 4, 14),
    (2017, 4, 17),
    (2017, 5, 1),
    (2017, 12, 25),
    (2017, 12, 26),
    (2018, 1, 1),
    (2018, 3, 30),
    (2018, 4, 2),
    (2018, 5, 1),
    (2018, 12, 24),
    (2018, 12, 25),
    (2018, 12, 26),
    (2018, 12, 31),
    (2019, 1, 1),
    (2019, 4, 19),
    (2019, 4, 22),
    (2019, 5, 1),
    (2019, 12, 24),
    (2019, 12, 25),
    (2019, 12, 26),
    (2019, 12, 31),
    (2020, 1, 1),
    (2020, 4, 10),
    (2020, 4, 13),
    (2020, 5, 1),
    (2020, 12, 24),
    (2020, 12, 25),
    (2020, 12, 31),
    (2021, 1, 1),
    (2021, 4, 2),
    (2021, 4, 5),
    (2021, 12, 24),
    (2021, 12, 31),
    (2022, 4, 15),
    (2022, 4, 18),
    (2022, 12, 26),
    (2023, 4, 7),
    (2023, 4, 10),
    (2023, 5, 1),
    (2023, 12, 25),
    (2023, 12, 26),
    (2024, 1, 1),
    (2024, 3, 29),
    (2024, 4, 1),
    (2024, 5, 1),
    (2024, 12, 24),
    (2024, 12, 25),
    (2024, 12, 26),
    (2024, 12, 31),
];

#[test]
fn every_eurex_identity_ships_the_same_hundred_and_eight_closures() {
    let mut closures = Vec::new();
    for (year, month, date) in CALENDAR_EDITION_CLOSURES {
        closures.push(day(year, month, date));
    }
    closures.extend([
        day(2025, 1, 1),
        day(2025, 4, 18),
        day(2025, 4, 21),
        day(2025, 5, 1),
        day(2025, 12, 24),
        day(2025, 12, 25),
        day(2025, 12, 26),
        day(2025, 12, 31),
        day(2026, 1, 1),
        day(2026, 4, 3),
        day(2026, 4, 6),
        day(2026, 5, 1),
        day(2026, 12, 24),
        day(2026, 12, 25),
        day(2026, 12, 31),
    ]);
    for (name, calendar) in identities() {
        for date in &closures {
            assert_eq!(
                calendar.holiday_on(*date).map(Holiday::kind),
                Some(HolidayKind::Closed),
                "{name} must be closed on {date}"
            );
        }
    }
    // The window opens at the 2010 floor and Eurex wraps no session across
    // midnight, so every closure's trade-date consequence is answerable from
    // the floor on — for the benchmark identities, whose grid is sourced to
    // the floor. `eurex_fixed_income`'s own grid is carried below its
    // 2018-11-15 horizon, so its session queries refuse there while its row
    // layer answers above; both behaviours are fenced.
    for (name, calendar) in benchmark_identities() {
        for date in &closures {
            assert!(
                calendar
                    .is_closed_trade_date(*date, SessionKind::Both)
                    .expect("the coverage contract must answer a covered date"),
                "{name} on {date}"
            );
        }
    }
    let fixed_income = calendar_for_market_hours_key(MarketHoursKey::EurexFixedIncome);
    for date in closures.iter().filter(|date| **date < day(2018, 11, 15)) {
        assert!(
            fixed_income
                .is_closed_trade_date(*date, SessionKind::Both)
                .is_err_and(|error| matches!(
                    error,
                    CalendarQueryError::OutsideCoveredRange { .. }
                )),
            "eurex_fixed_income on {date}: below its carried horizon the session query \
             refuses"
        );
    }
}

#[test]
fn the_christmas_closures_delete_their_trading_days() {
    for (name, calendar) in identities() {
        for probe in [(9, 0, 0), (12, 0, 0), (17, 0, 0)] {
            assert!(
                !calendar
                    .is_open(cet((2026, 12, 24), probe))
                    .expect("the coverage contract must answer a covered date"),
                "{name}: 24 December is a full trading closure, not a half day, at {probe:?}"
            );
            assert!(
                !calendar
                    .is_open(cet((2026, 12, 25), probe))
                    .expect("the coverage contract must answer a covered date"),
                "{name}: 25 December at {probe:?}"
            );
        }
        // 23 December is inside coverage with no row: audited normal.
        assert_eq!(calendar.holiday_on(day(2026, 12, 23)), None, "{name}");
        assert!(
            calendar
                .is_open(cet((2026, 12, 23), (12, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{name}"
        );
    }
}

#[test]
fn trading_resumes_on_the_first_open_day_after_the_christmas_block() {
    for (name, calendar) in identities() {
        let next = calendar
            .next_session_after(cet((2026, 12, 23), (21, 0, 0)))
            .expect("the coverage contract must answer a covered date")
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
        // Both New Year's Days now answer from their own audited windows. Until
        // the 2025 rows landed the 2026-01-01 probe refused: resolving it reads
        // the local date 2025-12-31, which was then one day below the window the
        // table audited. 2025-12-31 is inside the window now — and is itself a
        // trading closure — so the probe reads the closure it names.
        for date in [day(2025, 1, 1), day(2026, 1, 1)] {
            assert_eq!(
                calendar.holiday_on(date).map(Holiday::kind),
                Some(HolidayKind::Closed),
                "{name} on {date}"
            );
        }
        // Both predecessors are inside the audited window now (the window
        // opens at the 2010 floor), so both days answer shut.
        for probe in [cet((2025, 1, 1), (12, 0, 0)), cet((2026, 1, 1), (12, 0, 0))] {
            assert!(
                !calendar
                    .is_open(probe)
                    .expect("New Year's Day's predecessor is inside the audited window"),
                "{name} at {probe}"
            );
        }
        for probe in [cet((2025, 1, 2), (12, 0, 0)), cet((2026, 1, 2), (12, 0, 0))] {
            assert!(
                calendar
                    .is_open(probe)
                    .expect("2 January and its predecessor are inside the audited window"),
                "{name} at {probe}"
            );
        }
    }
}

#[test]
fn the_indicative_2027_calendar_does_not_ship() {
    for (name, calendar) in identities() {
        let coverage = calendar
            .holiday_coverage()
            .unwrap_or_else(|| panic!("{name} ships a built-in table"));
        assert_eq!(coverage.first(), day(2010, 1, 1), "{name}");
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
        // Nor does the identity answer the date: with the table attached, the
        // holiday layer has no audited window over 2027, so Stage 2B refuses
        // it. The old claim — that the identity answers the pure normal week
        // outside its coverage window — is no longer claimable; the detached
        // snapshot, which claims no coverage, is where the normal week shows.
        let outside = cet((2027, 1, 1), (12, 0, 0));
        assert!(
            matches!(
                calendar.is_open(outside),
                Err(CalendarQueryError::OutsideCoveredRange {
                    date,
                    ..
                }) if date == day(2027, 1, 1)
            ),
            "{name}: 2027-01-01 is outside the audited 2025-2026 window"
        );
        assert!(
            calendar
                .without_holidays()
                .is_open(outside)
                .expect("a detached snapshot claims no coverage"),
            "{name} answers the pure normal week only once the table is detached"
        );
    }
}

/// The 2025 rows this change adds, date by date: each of the eight is closed,
/// the ordinary week around them is untouched, and the table's window now opens
/// at the 2025-01-01 support floor.
#[test]
fn the_2025_closures_and_the_ordinary_week_around_them() {
    let closed = [
        day(2025, 1, 1),
        day(2025, 4, 18),
        day(2025, 4, 21),
        day(2025, 5, 1),
        day(2025, 12, 24),
        day(2025, 12, 25),
        day(2025, 12, 26),
        day(2025, 12, 31),
    ];
    // Weekdays adjacent to the closures, plus the coverage inventory's sample
    // date. None carries a row for these three identities, so each is an
    // audited-normal day and must still answer as an ordinary session.
    let ordinary = [
        day(2025, 1, 2),
        day(2025, 4, 17),
        day(2025, 4, 22),
        day(2025, 5, 2),
        day(2025, 6, 10),
        day(2025, 12, 23),
        day(2025, 12, 29),
        day(2025, 12, 30),
    ];
    for (name, calendar) in identities() {
        let coverage = calendar
            .holiday_coverage()
            .unwrap_or_else(|| panic!("{name} ships a built-in table"));
        assert_eq!(coverage.first(), day(2010, 1, 1), "{name}");
        assert_eq!(coverage.last(), day(2026, 12, 31), "{name}");
        // Below the window and past it the table has no answer at all: the
        // endpoints are bounds, not rows. 2024-12-31 used to sit below the
        // window and is now inside it as the edition's own Christmas Eve row.
        assert_eq!(calendar.holiday_on(day(2009, 12, 31)), None, "{name}");
        assert_eq!(
            calendar.holiday_on(day(2024, 12, 31)).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{name}"
        );
        assert_eq!(calendar.holiday_on(day(2027, 1, 1)), None, "{name}");

        for date in closed {
            assert_eq!(
                calendar.holiday_on(date).map(Holiday::kind),
                Some(HolidayKind::Closed),
                "{name} on {date}"
            );
            let noon = cet((date.year(), date.month(), date.day()), (12, 0, 0));
            assert!(
                !calendar
                    .is_open(noon)
                    .expect("the coverage contract must answer a covered date"),
                "{name} at {noon}"
            );
        }
        for date in ordinary {
            assert_eq!(calendar.holiday_on(date), None, "{name} on {date}");
            let noon = cet((date.year(), date.month(), date.day()), (12, 0, 0));
            assert!(
                calendar
                    .is_open(noon)
                    .expect("the coverage contract must answer a covered date"),
                "{name} at {noon}: an ordinary weekday with no row"
            );
        }
    }
}

#[test]
fn detaching_the_table_restores_the_normal_week() {
    for (name, calendar) in identities() {
        let good_friday = cet((2026, 4, 3), (12, 0, 0));
        assert!(
            !calendar
                .is_open(good_friday)
                .expect("the coverage contract must answer a covered date"),
            "{name}"
        );
        assert!(
            calendar
                .without_holidays()
                .is_open(good_friday)
                .expect("the coverage contract must answer a covered date"),
            "{name}"
        );
        assert_eq!(
            calendar.without_holidays().holiday_coverage(),
            None,
            "{name}"
        );
    }
}

#[test]
fn the_christmas_eve_closure_takes_the_evening_leg_off_its_trade_date() {
    // §4.1 case 6, the trade-date consequence. `eurex_fixed_income` is the one
    // Eurex identity with a post-close leg: its 22:00-22:10 CET order-entry
    // window on 23 December carries trade date 2026-12-24, so the closure
    // deletes it. The two benchmark-index identities have no such leg, and the
    // absence is fenced rather than assumed.
    let eve_evening = cet((2026, 12, 23), (22, 5, 0));
    let control = cet((2026, 12, 22), (22, 5, 0));

    let fixed_income = calendar_for_market_hours_key(MarketHoursKey::EurexFixedIncome);
    assert_eq!(
        fixed_income
            .without_holidays()
            .trade_date(eve_evening)
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 12, 24)),
        "the eve's evening leg belongs to the holiday's trade date"
    );
    assert_eq!(
        fixed_income
            .trade_date(eve_evening)
            .expect("the coverage contract must answer a covered date"),
        None,
        "the 24 December closure removes it"
    );
    assert!(
        !fixed_income
            .is_accepting_orders(eve_evening)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        fixed_income
            .without_holidays()
            .is_accepting_orders(eve_evening)
            .expect("the coverage contract must answer a covered date")
    );
    // The same leg one day earlier carries an open trade date and survives.
    assert_eq!(
        fixed_income
            .trade_date(control)
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 12, 23))
    );
    assert!(
        fixed_income
            .is_accepting_orders(control)
            .expect("the coverage contract must answer a covered date")
    );
    // Inside 23 December the trade date is the civil date and is untouched.
    assert_eq!(
        fixed_income
            .trade_date(cet((2026, 12, 23), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 12, 23))
    );

    for (name, calendar) in [
        ("Exchange::Eurex", calendar_for_exchange(Exchange::Eurex)),
        (
            "MarketHoursKey::Eurex",
            calendar_for_market_hours_key(MarketHoursKey::Eurex),
        ),
    ] {
        assert_eq!(
            calendar
                .without_holidays()
                .trade_date(eve_evening)
                .expect("the coverage contract must answer a covered date"),
            None,
            "{name} has no post-close leg for a closure to remove"
        );
        assert_eq!(
            calendar
                .trade_date(eve_evening)
                .expect("the coverage contract must answer a covered date"),
            None,
            "{name}"
        );
        assert_eq!(
            calendar
                .trade_date(cet((2026, 12, 23), (12, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            Some(day(2026, 12, 23)),
            "{name}"
        );
    }
}

/// One edition, one shape: each Trading Calendar year closes its Good Friday
/// and Easter Monday, the ordinary days around them carry no row, and the
/// years whose editions state December trading closures ship them. These are
/// spot checks per edition — the complete 93-date list is the sweep above.
#[test]
fn each_trading_calendar_edition_keys_its_own_year() {
    let spot_checks = [
        // (Good Friday, Easter Monday, an ordinary weekday that year)
        (day(2010, 4, 2), day(2010, 4, 5), day(2010, 4, 6)),
        (day(2011, 4, 22), day(2011, 4, 25), day(2011, 4, 26)),
        (day(2012, 4, 6), day(2012, 4, 9), day(2012, 4, 10)),
        (day(2013, 3, 29), day(2013, 4, 1), day(2013, 4, 2)),
        (day(2014, 4, 18), day(2014, 4, 21), day(2014, 4, 22)),
        (day(2015, 4, 3), day(2015, 4, 6), day(2015, 4, 7)),
        (day(2016, 3, 25), day(2016, 3, 28), day(2016, 3, 29)),
        (day(2017, 4, 14), day(2017, 4, 17), day(2017, 4, 18)),
        (day(2018, 3, 30), day(2018, 4, 2), day(2018, 4, 3)),
        (day(2019, 4, 19), day(2019, 4, 22), day(2019, 4, 23)),
        (day(2020, 4, 10), day(2020, 4, 13), day(2020, 4, 14)),
        (day(2021, 4, 2), day(2021, 4, 5), day(2021, 4, 6)),
        (day(2022, 4, 15), day(2022, 4, 18), day(2022, 4, 19)),
        (day(2023, 4, 7), day(2023, 4, 10), day(2023, 4, 11)),
        (day(2024, 3, 29), day(2024, 4, 1), day(2024, 4, 2)),
    ];
    for (name, calendar) in identities() {
        for (good_friday, easter_monday, ordinary) in spot_checks {
            assert_eq!(
                calendar.holiday_on(good_friday).map(Holiday::kind),
                Some(HolidayKind::Closed),
                "{name} on {good_friday}"
            );
            assert_eq!(
                calendar.holiday_on(easter_monday).map(Holiday::kind),
                Some(HolidayKind::Closed),
                "{name} on {easter_monday}"
            );
            assert_eq!(
                calendar.holiday_on(ordinary),
                None,
                "{name}: {ordinary} is audited normal"
            );
        }
    }
    for (name, calendar) in benchmark_identities() {
        for (_, _, ordinary) in spot_checks {
            assert!(
                calendar
                    .is_open(cet(
                        (ordinary.year(), ordinary.month(), ordinary.day()),
                        (12, 0, 0)
                    ))
                    .expect("the coverage contract must answer a covered date"),
                "{name}: {ordinary} trades"
            );
        }
    }
}

/// Whit Monday 2015 is the one extra trading-only closure the editions state,
/// and it ships for 2015 alone: a date-bounded arrangement is date-exception
/// data, never a recurring rule (LAW-HOLIDAY-SCOPE).
#[test]
fn the_2015_whit_monday_is_the_one_extra_trading_only_closure() {
    for (name, calendar) in identities() {
        assert_eq!(
            calendar.holiday_on(day(2015, 5, 25)).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{name}: the 2015 edition closes Whit Monday for trading"
        );
        // The Whit Mondays either side carry no row.
        for whit_monday in [day(2014, 6, 9), day(2016, 5, 16), day(2019, 6, 10)] {
            assert_eq!(
                calendar.holiday_on(whit_monday),
                None,
                "{name}: {whit_monday} is audited normal"
            );
        }
    }
    for (name, calendar) in benchmark_identities() {
        assert!(
            !calendar
                .is_open(cet((2015, 5, 25), (12, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{name}: 25 May 2015 is a full trading closure"
        );
        for whit_monday in [day(2014, 6, 9), day(2016, 5, 16), day(2019, 6, 10)] {
            assert!(
                calendar
                    .is_open(cet(
                        (whit_monday.year(), whit_monday.month(), whit_monday.day()),
                        (12, 0, 0)
                    ))
                    .expect("the coverage contract must answer a covered date"),
                "{name}: {whit_monday} trades"
            );
        }
    }
}

/// The window reaches the 2010-01-01 support floor and stops exactly there:
/// below it the identity refuses with the floor error, on it the edition's New
/// Year's Day closure answers, and Eurex's intraday-only grid has no wrap for
/// the boundary to split.
#[test]
fn the_window_reaches_the_floor_and_refuses_below_it() {
    for (name, calendar) in identities() {
        assert_eq!(
            calendar.holiday_on(day(2010, 1, 1)).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{name}: the 2010 edition's first closure is the floor day itself"
        );
    }
    for (name, calendar) in benchmark_identities() {
        assert!(
            !calendar
                .is_open(cet((2010, 1, 1), (12, 0, 0)))
                .expect("2010-01-01 is the floor day and answers"),
            "{name}: New Year's Day 2010 is closed"
        );
        assert!(
            calendar
                .is_open(cet((2010, 1, 4), (12, 0, 0)))
                .expect("2010-01-04 is inside the audited window"),
            "{name}: the first Monday of 2010 trades"
        );
        let below = cet((2009, 12, 31), (12, 0, 0));
        assert!(
            matches!(
                calendar.is_open(below),
                Err(CalendarQueryError::BeforeSupportFloor {
                    date,
                    ..
                }) if date == day(2009, 12, 31)
            ),
            "{name}: 2009-12-31 is below the support floor and must be refused as such"
        );
    }
    // `eurex_fixed_income` shares the rows but its grid is carried below its
    // 2018-11-15 horizon: session queries refuse on the dates below it and
    // answer from it on.
    let fixed_income = calendar_for_market_hours_key(MarketHoursKey::EurexFixedIncome);
    assert!(
        fixed_income
            .is_open(cet((2015, 5, 25), (12, 0, 0)))
            .is_err_and(|error| matches!(error, CalendarQueryError::OutsideCoveredRange { .. })),
        "eurex_fixed_income refuses below its carried horizon"
    );
    // The horizon day itself still refuses: the fixed-income family carries a
    // 22:00-22:10 CET post-close leg whose occurrence on 2018-11-14 belongs to
    // trade date 2018-11-15, so the horizon day's complete session set starts
    // one day below the horizon and a support boundary never splits a session.
    // The first fully answerable day is 2018-11-16.
    assert!(
        fixed_income
            .is_open(cet((2018, 11, 15), (12, 0, 0)))
            .is_err_and(|error| matches!(error, CalendarQueryError::OutsideCoveredRange { .. })),
        "eurex_fixed_income refuses its horizon day, whose opening leg sits below it"
    );
    assert!(
        fixed_income
            .is_open(cet((2018, 11, 16), (12, 0, 0)))
            .expect("2018-11-16 is the first fully answerable fixed-income day"),
        "the first answerable day is a trading Friday"
    );
}
