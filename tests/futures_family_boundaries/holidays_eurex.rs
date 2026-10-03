// SPDX-License-Identifier: MIT-0

//! Built-in holiday rows for Eurex (`eurex` venue, `eurex` and
//! `eurex_fixed_income` keys), 2010 through 2026.
//!
//! Eurex publishes closures only — no early close and no late open — so §4.1
//! cases 2, 3 and 4 have nothing to exercise here and the block says so rather
//! than inventing a row. Case 5's wrap removal and case 6's trade-date
//! consequence are **not** vacuous: `eurex_fixed_income` carries Eurex's
//! post-trading order-entry phase, whose occurrence on local date `D` belongs
//! to trade date `D + 1`, so a closure deletes the preceding evening's leg.
//! The two benchmark-index identities run 02:15-22:00 CET and have no such
//! leg; that absence is fenced below rather than assumed. Coverage runs from
//! the 2010-01-01 support floor — the operator's own "all derivatives" rows,
//! keyed to its annual Trading Calendar editions 2010-2024 and to the Holiday
//! regulations page for 2025-2026, with the Christmas block ends trading-only
//! and Whit Monday 2015 the one extra trading-only closure — and stops at
//! 2026-12-31 because Eurex's 2027 calendar is published "on a preliminary and
//! indicative basis", which LAW-NO-FABRICATED-DATES keeps out of a runtime
//! table; the last test is the fence on that.
//!
//! The benchmark-index identities alone carry the editions' **dated
//! German-scope closures** (2014, 2016, 2017 and 2018: Unity Day, Whit Monday
//! and the one-off 2017 Reformation Day): the operator's note closes German
//! equity and equity-index derivatives and the Xetra-based ETF/ETC
//! derivatives, which is FDAX and FDXM. The fixed-income family the note never
//! names ships the all-derivatives rows alone and answers those dates open,
//! and the 2019-2021 editions' own parenthetical — trading in German equity
//! index futures takes place — keeps every later German holiday in the
//! benchmark scope open too, so the `tba` note of 2025-2026 (#157) is the only
//! span the German scope still withholds.

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
/// from the 2010-01-01 floor. Since the German-scope split, `eurex` carries
/// the 108 all-derivatives rows plus the eight dated German-scope rows and
/// `eurex_fixed_income` carries the 108 all-derivatives rows alone — the
/// German lines never name fixed income — and, since the 2026-09-30 wave,
/// sources its pre-2018 baseline grid from the operator's archived Contract
/// Specifications amendments back to the floor, so all three Eurex
/// identities answer sessions from 2010-01-01.
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

/// One dated German-scope closure and the dates both sides of it need for the
/// probes.
struct GermanClosure {
    /// The closure day the edition states.
    closure: (i32, u32, u32),
    /// The last session day before it, whose fixed-income evening leg carries
    /// the closure's trade date.
    eve: (i32, u32, u32),
    /// An ordinary control whose own evening leg survives.
    control: (i32, u32, u32),
    /// The first session day after the closure.
    successor: (i32, u32, u32),
}

/// The eight dated German-scope closures the 2014, 2016, 2017 and 2018
/// editions state. Every date here is stated by the edition's German-scope
/// note and keys the benchmark `TABLE` alone.
const GERMAN_SCOPE_CLOSURES: [GermanClosure; 8] = [
    GermanClosure {
        closure: (2014, 10, 3),
        eve: (2014, 10, 2),
        control: (2014, 10, 1),
        successor: (2014, 10, 6),
    },
    GermanClosure {
        closure: (2016, 5, 16),
        eve: (2016, 5, 13),
        control: (2016, 5, 12),
        successor: (2016, 5, 17),
    },
    GermanClosure {
        closure: (2016, 10, 3),
        eve: (2016, 9, 30),
        control: (2016, 9, 29),
        successor: (2016, 10, 4),
    },
    GermanClosure {
        closure: (2017, 6, 5),
        eve: (2017, 6, 2),
        control: (2017, 6, 1),
        successor: (2017, 6, 6),
    },
    GermanClosure {
        closure: (2017, 10, 3),
        eve: (2017, 10, 2),
        control: (2017, 9, 29),
        successor: (2017, 10, 4),
    },
    GermanClosure {
        closure: (2017, 10, 31),
        eve: (2017, 10, 30),
        control: (2017, 10, 27),
        successor: (2017, 11, 1),
    },
    GermanClosure {
        closure: (2018, 5, 21),
        eve: (2018, 5, 18),
        control: (2018, 5, 17),
        successor: (2018, 5, 22),
    },
    GermanClosure {
        closure: (2018, 10, 3),
        eve: (2018, 10, 2),
        control: (2018, 10, 1),
        successor: (2018, 10, 4),
    },
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
    // the floor on: the benchmark identities' grid and, since the 2026-09-30
    // wave, `eurex_fixed_income`'s own baseline are sourced to the floor, so
    // all three answer every closure date in the window.
    for (name, calendar) in identities() {
        for date in &closures {
            assert!(
                calendar
                    .is_closed_trade_date(*date, SessionKind::Both)
                    .expect("the coverage contract must answer a covered date"),
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
/// data, never a recurring rule (LAW-HOLIDAY-SCOPE). The 2016 Whit Monday is
/// the German-scope row the 2016 edition adds — a benchmark-index closure the
/// fixed-income family keeps trading through — so the no-row fence moves to
/// the years the editions state no closure for.
#[test]
fn the_2015_whit_monday_is_the_one_extra_trading_only_closure() {
    for (name, calendar) in identities() {
        assert_eq!(
            calendar.holiday_on(day(2015, 5, 25)).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{name}: the 2015 edition closes Whit Monday for trading"
        );
        // The Whit Mondays the editions close for no Eurex identity carry no
        // row: 2014's edition states no Whit Monday at all, and 2019's states
        // it for the German scope with the futures carve-out.
        for whit_monday in [day(2014, 6, 9), day(2019, 6, 10)] {
            assert_eq!(
                calendar.holiday_on(whit_monday),
                None,
                "{name}: {whit_monday} is audited normal"
            );
        }
    }
    // The benchmark identities close 2016-05-16 on the 2016 edition's
    // German-scope line; the fixed-income family, which that line never
    // names, does not.
    for (name, calendar) in benchmark_identities() {
        assert_eq!(
            calendar.holiday_on(day(2016, 5, 16)).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{name}: the 2016 edition closes Whit Monday in the German scope"
        );
        assert!(
            !calendar
                .is_open(cet((2015, 5, 25), (12, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{name}: 25 May 2015 is a full trading closure"
        );
        assert!(
            !calendar
                .is_open(cet((2016, 5, 16), (12, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{name}: 16 May 2016 is the German-scope Whit Monday closure"
        );
        for whit_monday in [day(2014, 6, 9), day(2019, 6, 10)] {
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
    let fixed_income = calendar_for_market_hours_key(MarketHoursKey::EurexFixedIncome);
    assert_eq!(
        fixed_income.holiday_on(day(2016, 5, 16)),
        None,
        "the fixed-income family carries no German-scope row"
    );
    assert!(
        fixed_income
            .is_open(cet((2016, 5, 16), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "fixed income trades through the German-scope Whit Monday"
    );
}

/// The dated German-scope closures the 2014, 2016, 2017 and 2018 editions
/// state close the benchmark-index identities and only them, on both sides of
/// every encoded date: the closure day answers shut at Berlin noon, its
/// neighbouring session days answer open, and the fixed-income family answers
/// open through every one of them — its evening leg on the eve of each closure
/// still feeds this family's own session on the closure date, which is the
/// fence that the German rows never reached the fixed-income table.
#[test]
fn the_dated_german_scope_closures_close_the_benchmark_identities_alone() {
    for (record, (name, calendar)) in GERMAN_SCOPE_CLOSURES
        .iter()
        .flat_map(|record| benchmark_identities().map(move |identity| (record, identity)))
    {
        let closure_day = day(record.closure.0, record.closure.1, record.closure.2);
        assert_eq!(
            calendar.holiday_on(closure_day).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{name} on {closure_day}"
        );
        let noon = cet(record.closure, (12, 0, 0));
        assert!(
            !calendar
                .is_open(noon)
                .expect("the coverage contract must answer a covered date"),
            "{name} at {noon}"
        );
        // Both sides: the last session day before the closure and the first
        // one after it are ordinary days that trade.
        for side in [record.eve, record.successor] {
            let side_noon = cet(side, (12, 0, 0));
            assert_eq!(
                calendar.holiday_on(day(side.0, side.1, side.2)),
                None,
                "{name}: the day beside {closure_day} is audited normal"
            );
            assert!(
                calendar
                    .is_open(side_noon)
                    .expect("the coverage contract must answer a covered date"),
                "{name} at {side_noon}"
            );
        }
    }
    // The fixed-income family trades through every German-scope closure and
    // carries no row for any of them.
    let fixed_income = calendar_for_market_hours_key(MarketHoursKey::EurexFixedIncome);
    for record in GERMAN_SCOPE_CLOSURES {
        let closure_day = day(record.closure.0, record.closure.1, record.closure.2);
        assert_eq!(
            fixed_income.holiday_on(closure_day),
            None,
            "fixed income on {closure_day}"
        );
        assert!(
            fixed_income
                .is_open(cet(record.closure, (12, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "fixed income trades through {closure_day}"
        );
        for side in [record.eve, record.control, record.successor] {
            let side_day = day(side.0, side.1, side.2);
            assert!(
                fixed_income
                    .is_open(cet(side, (12, 0, 0)))
                    .expect("the coverage contract must answer a covered date"),
                "fixed income trades {side_day}"
            );
        }
    }
    // And the fixed-income evening leg on the last session day before each
    // closure **keeps** carrying the German-scope date as its trade date,
    // because that date is an ordinary fixed-income session: the leg feeds
    // this family's own session there. The Christmas closures delete the same
    // leg — the test above fences that — because 24 December closes every
    // family; the German-scope closures close none of this family's days, and
    // this is the fence that the split is real rather than a routing slip.
    // The control leg the day earlier carries the session day it feeds.
    for record in GERMAN_SCOPE_CLOSURES {
        let closure_day = day(record.closure.0, record.closure.1, record.closure.2);
        let leg_day = day(record.eve.0, record.eve.1, record.eve.2);
        let control_day = day(record.control.0, record.control.1, record.control.2);
        assert_eq!(
            fixed_income
                .trade_date(cet(record.eve, (22, 5, 0)))
                .expect("the coverage contract must answer a covered date"),
            Some(closure_day),
            "the {leg_day} evening leg feeds the fixed-income session on {closure_day}, \
             which this family keeps trading"
        );
        assert_eq!(
            fixed_income
                .trade_date(cet(record.control, (22, 5, 0)))
                .expect("the coverage contract must answer a covered date"),
            Some(leg_day),
            "the {control_day} evening leg carries trade date {leg_day} and survives"
        );
    }
}

/// The German-scope dates the operator's own bytes leave open: the 2019, 2020
/// and 2021 editions date the scope but print `(trading in German equity index
/// futures takes place!)`, and the 2010-2013, 2015 and 2022-2024 editions
/// print no German-scope line at all. Every Unity Day and Whit Monday outside
/// 2014-2018 is therefore an ordinary day for all three identities — and 2025
/// and 2026 refuse through the `tba` declaration (#157) rather than answering.
#[test]
fn the_german_holidays_outside_the_dated_editions_answer_open() {
    let open_days = [
        day(2011, 10, 3), // Unity Day, Monday: the 2011 edition states no German line
        day(2012, 10, 3), // Unity Day, Wednesday: no German line
        day(2013, 10, 3), // Unity Day, Thursday: no German line
        day(2019, 6, 10), // Whit Monday: the 2019 edition's futures carve-out
        day(2019, 10, 3), // Unity Day: the same carve-out
        day(2020, 6, 1),  // Whit Monday: the 2020 edition's carve-out
        day(2021, 5, 24), // Whit Monday: the 2021 edition's carve-out
        day(2022, 10, 3), // Unity Day, Monday: the 2022 edition states no German line
        day(2023, 10, 3), // Unity Day, Tuesday: no German line
        day(2024, 10, 3), // Unity Day, Thursday: no German line
    ];
    for (name, calendar) in identities() {
        for date in open_days {
            assert_eq!(
                calendar.holiday_on(date),
                None,
                "{name}: {date} is audited normal"
            );
        }
    }
    for (name, calendar) in benchmark_identities() {
        for date in open_days {
            assert!(
                calendar
                    .is_open(cet((date.year(), date.month(), date.day()), (12, 0, 0)))
                    .expect("the coverage contract must answer a covered date"),
                "{name}: {date} trades"
            );
        }
    }
    // The Unity Days the editions print no German line for and that fall on a
    // Saturday (2015-10-03, 2020-10-03) carry no row either: the absence of
    // the line is the operator's own enumeration, not silence to infer from.
    for (name, calendar) in identities() {
        for saturday in [day(2015, 10, 3), day(2020, 10, 3)] {
            assert_eq!(
                calendar.holiday_on(saturday),
                None,
                "{name}: {saturday} is audited normal"
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
    // `eurex_fixed_income` shares the rows and, since the 2026-09-30 wave,
    // sources its pre-2018 baseline from the operator's archived Contract
    // Specifications amendments, so its horizon is the floor too: mid-window
    // dates answer from the sourced baseline (2015-05-25 was the one Whit
    // Monday closure, so noon is closed rather than a refusal), and the
    // 2018-11-15 horizon-era probes answer from the dated grid.
    let fixed_income = calendar_for_market_hours_key(MarketHoursKey::EurexFixedIncome);
    assert!(
        !fixed_income
            .is_open(cet((2015, 5, 25), (12, 0, 0)))
            .expect("the floor-era baseline answers a covered date"),
        "2015-05-25 is the sourced Whit Monday closure: closed at noon, not refused"
    );
    assert!(
        fixed_income
            .is_open(cet((2015, 5, 26), (12, 0, 0)))
            .expect("the floor-era baseline answers a covered date"),
        "the day after the closure trades on the sourced 08:00-22:00 baseline"
    );
    assert!(
        fixed_income
            .is_open(cet((2018, 11, 16), (12, 0, 0)))
            .expect("2018-11-16 is a fully answerable fixed-income day"),
        "the first post-circular day is a trading Friday"
    );
}
