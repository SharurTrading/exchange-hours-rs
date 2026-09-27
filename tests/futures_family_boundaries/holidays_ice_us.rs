// SPDX-License-Identifier: MIT-0

//! Built-in holiday rows for ICE Futures U.S. — the seven product-family keys
//! and the `iceus` venue — 2025 through the 2027 calendar's last entry.
//!
//! Three tables serve the five softs (Sugar, Coffee and Cocoa share one, FCOJ
//! and Cotton each differ by one row), and the two index families and the
//! venue have one each. The late-open branch exercised here is the one whose
//! wall clock falls **on** the trade date; no ICE row in this window states a
//! late open at or after a family's own first open, so the preceding-local-date
//! branch has nothing to exercise and this block says so rather than inventing
//! a row. The 2025 rows include two London-bank-holiday late opens the annual
//! calendar does not list, and four dates the two unretrieved 2025 notices leave
//! `Unsourced` — asserted as refusals rather than as answers, because a withheld
//! date has no session to check.

use chrono::{DateTime, Days, NaiveDate, TimeDelta, TimeZone as _, Utc};
use chrono_tz::America;
use exchange_hours::{
    CalendarQueryError, CalendarResolution, DateCoverage, Exchange, ExchangeCalendar, Holiday,
    HolidayKind, MarketHoursKey, SessionKind, calendar_for_exchange, calendar_for_market_hours_key,
};

fn key(key: MarketHoursKey) -> ExchangeCalendar {
    calendar_for_market_hours_key(key)
}

/// Asserts a date-aware query refused with the error variant matching
/// `expected`, the verdict the identity's own `coverage()` reports for the
/// venue-local date the error names (LAW-COVERAGE).
///
/// Stage 2B refuses a date an identity cannot source, and the refusal's variant
/// is that date's verdict: [`DateCoverage::BeforeSupportFloor`] below the 2025
/// floor, [`DateCoverage::OutsideCoveredRange`] for an unsourced span or a
/// declared phase-level gap, and [`DateCoverage::UnresolvedGap`] for a date the
/// identity withholds as `Unsourced`. The helper checks both halves — the
/// identity really does derive `expected` for the date the error names, and the
/// error is that verdict's variant. An `Ok` answer fails here rather than being
/// tolerated, so a converted site states the refusal instead of swallowing it.
fn assert_declared_refusal<T: core::fmt::Debug>(
    result: Result<T, CalendarQueryError>,
    expected: DateCoverage,
    calendar: ExchangeCalendar,
    claim: &str,
) {
    let error = result.expect_err(claim);
    let date = error.date();
    let verdict = calendar.coverage().coverage_on(date);
    assert_eq!(
        verdict, expected,
        "{claim}: {date} is the date the query needed, and the identity derives {verdict:?} for it"
    );
    assert!(
        matches!(
            (expected, error),
            (
                DateCoverage::BeforeSupportFloor,
                CalendarQueryError::BeforeSupportFloor { .. }
            ) | (
                DateCoverage::OutsideCoveredRange,
                CalendarQueryError::OutsideCoveredRange { .. }
            ) | (
                DateCoverage::UnresolvedGap,
                CalendarQueryError::UnresolvedGap { .. }
            )
        ),
        "{claim}: {date} is {expected:?}, so the query must refuse it with the matching \
         CalendarQueryError; got {error}"
    );
}

fn ny(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    America::New_York
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous New York instant")
        .with_timezone(&Utc)
}

fn day(year: i32, month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
}

/// Every ICE identity that ships a table, for the coverage fence.
fn every_ice_identity() -> [(&'static str, ExchangeCalendar); 8] {
    [
        ("iceus", calendar_for_exchange(Exchange::Iceus)),
        ("ice_us", key(MarketHoursKey::IceUs)),
        ("ice_us_sugar", key(MarketHoursKey::IceUsSugar)),
        ("ice_us_coffee", key(MarketHoursKey::IceUsCoffee)),
        ("ice_us_cocoa", key(MarketHoursKey::IceUsCocoa)),
        ("ice_us_cotton", key(MarketHoursKey::IceUsCotton)),
        ("ice_us_orange_juice", key(MarketHoursKey::IceUsOrangeJuice)),
        ("ice_us_dollar_index", key(MarketHoursKey::IceUsDollarIndex)),
    ]
}

#[test]
fn memorial_day_removes_the_whole_softs_trading_day() {
    let sugar = key(MarketHoursKey::IceUsSugar);
    let memorial = day(2026, 5, 25);

    assert_eq!(
        sugar.holiday_on(memorial).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(
        sugar
            .is_closed_trade_date(memorial, SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    for probe in [(4, 0, 0), (9, 0, 0), (12, 30, 0)] {
        assert!(
            !sugar
                .is_open(ny((2026, 5, 25), probe))
                .expect("the coverage contract must answer a covered date"),
            "Sugar must be shut at {probe:?} on Memorial Day"
        );
    }
    // The Friday before is untouched, and the Tuesday after reopens normally.
    assert!(
        sugar
            .is_open(ny((2026, 5, 22), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    let next = sugar
        .next_session_after(ny((2026, 5, 22), (13, 30, 0)))
        .expect("the coverage contract must answer a covered date")
        .expect("a reopening must exist inside the bounded search");
    assert_eq!(next.0, ny((2026, 5, 26), (3, 30, 0)));
}

#[test]
fn easter_monday_moves_the_sugar_coffee_and_cocoa_open_to_0730_new_york() {
    let cutoff_by_key = [
        (MarketHoursKey::IceUsSugar, (3, 30, 0)),
        (MarketHoursKey::IceUsCoffee, (4, 15, 0)),
        (MarketHoursKey::IceUsCocoa, (4, 45, 0)),
    ];
    for (which, normal_open) in cutoff_by_key {
        let calendar = key(which);
        let cutoff = ny((2026, 4, 6), (7, 30, 0));
        assert_eq!(
            calendar.holiday_on(day(2026, 4, 6)).map(Holiday::kind),
            Some(HolidayKind::LateOpen {
                open_ssm: 7 * 3_600 + 30 * 60
            }),
            "{which:?}"
        );
        // The normal open is inside the delayed window, so it is gone.
        assert!(
            !calendar
                .is_open(ny((2026, 4, 6), normal_open))
                .expect("the coverage contract must answer a covered date"),
            "{which:?} must not open at its normal {normal_open:?}"
        );
        assert!(
            !calendar
                .is_open(cutoff - TimeDelta::nanoseconds(1))
                .expect("the coverage contract must answer a covered date"),
            "{which:?}"
        );
        assert!(
            calendar
                .is_open(cutoff)
                .expect("the coverage contract must answer a covered date"),
            "{which:?}"
        );
        assert_eq!(
            calendar
                .session_bounds(cutoff)
                .expect("the coverage contract must answer a covered date")
                .map(|(open, _)| open),
            Some(cutoff),
            "{which:?}"
        );
    }
}

#[test]
fn fcoj_and_cotton_are_not_part_of_the_easter_monday_late_open() {
    for which in [
        MarketHoursKey::IceUsOrangeJuice,
        MarketHoursKey::IceUsCotton,
    ] {
        let calendar = key(which);
        assert_eq!(
            calendar.holiday_on(day(2026, 4, 6)),
            None,
            "{which:?}: ICE gives the late open to Sugar, Coffee and Cocoa only"
        );
    }
    // FCOJ's own regular open is untouched that day.
    assert!(
        key(MarketHoursKey::IceUsOrangeJuice)
            .is_open(ny((2026, 4, 6), (8, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

#[test]
fn cotton_carries_its_own_monday_late_open_row() {
    let cotton = key(MarketHoursKey::IceUsCotton);

    assert_eq!(
        cotton.holiday_on(day(2026, 7, 6)).map(Holiday::kind),
        Some(HolidayKind::LateOpen {
            open_ssm: 8 * 3_600
        })
    );
    // Cotton's modelled grid opens for trade date D at 21:00 on D-1 and names
    // no Sunday, so the crate has no trade date 2026-07-06 for the row to clip.
    // The row records ICE's statement; the Monday-evening leg of trade date
    // 2026-07-07 is unaffected, which is the answer that would break first if
    // the row were mis-keyed.
    assert!(
        cotton
            .is_open(ny((2026, 7, 6), (21, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        cotton
            .trade_date(ny((2026, 7, 6), (21, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 7, 7))
    );
    // Answer-neutrality, against the detached calendar: over the whole span
    // around the row, every instant answers exactly as it would with no table.
    let bare = cotton.without_holidays();
    let mut probe = ny((2026, 7, 5), (12, 0, 0));
    while probe <= ny((2026, 7, 7), (18, 0, 0)) {
        assert_eq!(
            cotton
                .is_open(probe)
                .expect("the coverage contract must answer a covered date"),
            bare.is_open(probe)
                .expect("the coverage contract must answer a covered date"),
            "{probe}"
        );
        assert_eq!(
            cotton
                .trade_date(probe)
                .expect("the coverage contract must answer a covered date"),
            bare.trade_date(probe)
                .expect("the coverage contract must answer a covered date"),
            "{probe}"
        );
        probe += TimeDelta::minutes(30);
    }
}

#[test]
fn good_friday_gives_the_index_families_a_late_open_and_an_early_close() {
    let cases = [
        (MarketHoursKey::IceUs, 9 * 3_600 + 15 * 60, (9, 15, 0)),
        (
            MarketHoursKey::IceUsDollarIndex,
            11 * 3_600 + 15 * 60,
            (11, 15, 0),
        ),
    ];
    for (which, close_ssm, close_time) in cases {
        let calendar = key(which);
        let open_cutoff = ny((2026, 4, 3), (5, 0, 0));
        let close_cutoff = ny((2026, 4, 3), close_time);

        assert_eq!(
            calendar.holiday_on(day(2026, 4, 3)).map(Holiday::kind),
            Some(HolidayKind::LateOpenAndEarlyClose {
                open_ssm: 5 * 3_600,
                close_ssm,
            }),
            "{which:?}"
        );
        // The Thursday-evening leg of this trade date does not run.
        assert!(
            !calendar
                .is_open(ny((2026, 4, 2), (21, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{which:?}"
        );
        assert!(
            !calendar
                .is_open(open_cutoff - TimeDelta::nanoseconds(1))
                .expect("the coverage contract must answer a covered date"),
            "{which:?}"
        );
        assert!(
            calendar
                .is_open(open_cutoff)
                .expect("the coverage contract must answer a covered date"),
            "{which:?}"
        );
        assert!(
            calendar
                .is_open(close_cutoff - TimeDelta::nanoseconds(1))
                .expect("the coverage contract must answer a covered date"),
            "{which:?}"
        );
        assert!(
            !calendar
                .is_open(close_cutoff)
                .expect("the coverage contract must answer a covered date"),
            "{which:?}"
        );
        assert_eq!(
            calendar
                .session_bounds(ny((2026, 4, 3), (6, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            Some((open_cutoff, close_cutoff)),
            "{which:?}"
        );
        assert_eq!(
            calendar
                .candle_end(ny((2026, 4, 3), (6, 0, 0)), CalendarResolution::Daily)
                .expect("the coverage contract must answer a covered date"),
            Some(close_cutoff),
            "{which:?}"
        );
    }
}

#[test]
fn the_index_families_take_their_own_early_close_instants() {
    let fang = key(MarketHoursKey::IceUs);
    let usdx = key(MarketHoursKey::IceUsDollarIndex);

    for date in [(2026, 5, 25), (2026, 6, 19), (2026, 7, 3)] {
        let fang_cutoff = ny(date, (13, 0, 0));
        let usdx_cutoff = ny(date, (14, 30, 0));
        assert!(
            fang.is_open(fang_cutoff - TimeDelta::nanoseconds(1))
                .expect("the coverage contract must answer a covered date"),
            "{date:?}"
        );
        assert!(
            !fang
                .is_open(fang_cutoff)
                .expect("the coverage contract must answer a covered date"),
            "{date:?}"
        );
        assert!(
            usdx.is_open(usdx_cutoff - TimeDelta::nanoseconds(1))
                .expect("the coverage contract must answer a covered date"),
            "{date:?}"
        );
        assert!(
            !usdx
                .is_open(usdx_cutoff)
                .expect("the coverage contract must answer a covered date"),
            "{date:?}"
        );
    }
    // Labor Day 2026: FANG+ closes early, the dollar index keeps regular hours.
    assert!(
        !fang
            .is_open(ny((2026, 9, 7), (13, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(usdx.holiday_on(day(2026, 9, 7)), None);
    assert!(
        usdx.is_open(ny((2026, 9, 7), (16, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

#[test]
fn a_shortened_index_day_keeps_its_trade_date() {
    let fang = key(MarketHoursKey::IceUs);

    assert_eq!(
        fang.trade_date(ny((2026, 5, 24), (19, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 5, 25))
    );
    assert_eq!(
        fang.trade_date(ny((2026, 5, 25), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 5, 25))
    );
    assert_eq!(
        fang.trade_date(ny((2026, 5, 25), (21, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 5, 26))
    );
}

#[test]
fn an_unsourced_row_is_reported_and_changes_no_answer() {
    let fang = key(MarketHoursKey::IceUs);
    let boxing_day = day(2026, 12, 28);

    assert_eq!(
        fang.holiday_on(boxing_day).map(Holiday::kind),
        Some(HolidayKind::Unsourced)
    );
    // The row's whole point is that the identity has no answer for the date:
    // Stage 2B reports that as `UnresolvedGap` rather than resolving the date
    // to a scheduling claim. The old assertion — that an `Unsourced` date
    // changes no answer — is exactly what the refusal replaces, so the
    // neutrality half is restated on the detached snapshot below.
    assert_declared_refusal(
        fang.is_closed_trade_date(boxing_day, SessionKind::Both),
        DateCoverage::UnresolvedGap,
        fang,
        "an `Unsourced` trade date is withheld, not answered",
    );
    let probe = ny((2026, 12, 28), (12, 0, 0));
    assert_declared_refusal(
        fang.is_open(probe),
        DateCoverage::UnresolvedGap,
        fang,
        "an `Unsourced` date is withheld by every date-aware query",
    );
    // The normal week the row does not clip is still observable where no
    // coverage is claimed: the detached snapshot answers the probe.
    assert!(
        fang.without_holidays()
            .is_open(probe)
            .expect("a detached snapshot claims no coverage"),
        "the `Unsourced` row clips nothing, so the normal week trades at {probe}"
    );
}

#[test]
fn the_venue_ships_only_the_dates_every_family_agrees_on() {
    let venue = calendar_for_exchange(Exchange::Iceus);

    // The seven dates on which every modelled family prints `closed`. The
    // trade-date deletion answers from the date's own day, except where it
    // depends on a day the table withholds: 2025-12-25's opening day is the
    // withheld 2025-12-24.
    for (date, withheld_by) in [
        (day(2025, 1, 1), None),
        (day(2025, 4, 18), None),
        (day(2025, 12, 25), Some(DateCoverage::UnresolvedGap)),
        (day(2026, 1, 1), None),
        (day(2026, 12, 25), None),
        (day(2027, 1, 1), None),
        (day(2027, 12, 24), None),
    ] {
        assert_eq!(
            venue.holiday_on(date).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{date}: every modelled ICE family is closed"
        );
        let closed = venue.is_closed_trade_date(date, SessionKind::Both);
        if let Some(verdict) = withheld_by {
            assert_declared_refusal(
                closed,
                verdict,
                venue,
                "the trade date's deletion depends on a day the table withholds",
            );
        } else {
            assert!(
                closed.expect("the trade date's own day is inside the audited window"),
                "{date}"
            );
        }
    }
    // 2025-01-01 answers its own closure, but naming its trade date reads the
    // evening of 2024-12-31, which is below the 2025 floor.
    assert!(
        !venue
            .is_open(ny((2025, 1, 1), (12, 0, 0)))
            .expect("the closure's own day is inside the audited window")
    );
    assert_declared_refusal(
        venue.trade_date(ny((2025, 1, 1), (12, 0, 0))),
        DateCoverage::BeforeSupportFloor,
        venue,
        "the trade date's opening day is below the 2025 floor",
    );
    // A plain 2025 weekday answers: the 2025 floor is inside the window now.
    let weekday = ny((2025, 6, 11), (12, 0, 0));
    assert!(
        venue
            .is_open(weekday)
            .expect("2025 is inside the audited window"),
        "the hole this change closes: 2025-06-11 used to be OutsideCoveredRange"
    );
    assert_eq!(
        venue
            .trade_date(weekday)
            .expect("2025 is inside the audited window"),
        Some(day(2025, 6, 11))
    );
    // Juneteenth 2025 and Memorial Day 2026: the softs close, the index families
    // shorten, so the venue states no scheduling row and says the date is not
    // audited normal.
    for (date, probe) in [
        (day(2025, 6, 19), ny((2025, 6, 19), (12, 0, 0))),
        (day(2026, 5, 25), ny((2026, 5, 25), (12, 0, 0))),
    ] {
        assert_eq!(
            venue.holiday_on(date).map(Holiday::kind),
            Some(HolidayKind::Unsourced),
            "{date}"
        );
        // An `Unsourced` venue date is withheld: no answer is a claim about it.
        assert_declared_refusal(
            venue.is_open(probe),
            DateCoverage::UnresolvedGap,
            venue,
            "the venue withholds a date its families disagree on",
        );
        assert!(
            venue
                .without_holidays()
                .is_open(probe)
                .expect("a detached snapshot claims no coverage"),
            "the withheld row clips nothing, so the normal week trades at {probe}"
        );
    }
}

#[test]
fn the_new_years_closure_removes_the_prior_evening_for_the_index_families() {
    let fang = key(MarketHoursKey::IceUs);

    // 2025-12-31 is the opening day of trade date 2026-01-01, and it is inside
    // the window now that the 2025 floor ships. The 2026 New Year's notice
    // prints `Regular Hours` for every modelled group in its Wed, Dec 31
    // column, so the day carries no row and the ordinary week trades.
    assert_eq!(fang.holiday_on(day(2025, 12, 31)), None);
    assert_eq!(
        fang.trade_date(ny((2025, 12, 31), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 12, 31))
    );
    assert!(
        fang.is_open(ny((2025, 12, 31), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // The Wednesday 20:00 NY leg feeds trade date 2026-01-01, whose `Closed`
    // row deletes the complete trading day, prior-evening wrap included.
    assert!(
        !fang
            .is_open(ny((2025, 12, 31), (21, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        fang.holiday_on(day(2026, 1, 1)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(
        fang.is_closed_trade_date(day(2026, 1, 1), SessionKind::Both)
            .expect("2025-12-31 is inside the audited window, so the trade date answers")
    );
    // Thursday 20:00 NY opens trade date 2026-01-02, which the closure does not
    // touch.
    assert!(
        fang.is_open(ny((2026, 1, 1), (21, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        fang.trade_date(ny((2026, 1, 1), (21, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 1, 2))
    );
}

#[test]
fn every_ice_table_covers_the_2025_floor_through_the_2027_calendars_last_entry() {
    for (name, calendar) in every_ice_identity() {
        let coverage = calendar
            .holiday_coverage()
            .unwrap_or_else(|| panic!("{name} ships a built-in table"));
        assert_eq!(coverage.first(), day(2025, 1, 1), "{name}");
        assert_eq!(coverage.last(), day(2028, 1, 3), "{name}");
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
    }
    // 2024-12-31 is below the permanent 2025 floor, and 2028-01-18 is a Martin
    // Luther King Day the softs will observe that is outside every window, so
    // no table may reach either. Stage 2B refuses both for the identity
    // calendar too; the normal week is still observable on the detached
    // snapshot.
    let sugar = key(MarketHoursKey::IceUsSugar);
    assert_declared_refusal(
        sugar.is_open(ny((2024, 12, 31), (9, 0, 0))),
        DateCoverage::BeforeSupportFloor,
        sugar,
        "2024-12-31 is below the 2025 floor",
    );
    let outside = ny((2028, 1, 18), (9, 0, 0));
    assert_eq!(sugar.holiday_on(day(2028, 1, 18)), None);
    assert_declared_refusal(
        sugar.is_open(outside),
        DateCoverage::OutsideCoveredRange,
        sugar,
        "2028-01-18 is outside the audited window",
    );
    assert!(
        sugar
            .without_holidays()
            .is_open(outside)
            .expect("a detached snapshot claims no coverage"),
        "the detached snapshot answers the pure normal week outside the window"
    );
}

#[test]
fn detaching_the_table_restores_the_normal_week() {
    let sugar = key(MarketHoursKey::IceUsSugar);
    let memorial = ny((2026, 5, 25), (9, 0, 0));

    assert!(
        !sugar
            .is_open(memorial)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        sugar
            .without_holidays()
            .is_open(memorial)
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(sugar.without_holidays().holiday_coverage(), None);
    assert_eq!(sugar.without_holidays().holiday_on(day(2026, 5, 25)), None);
}

#[test]
fn the_2025_softs_closures_remove_the_whole_trading_day() {
    // The ten 2025 dates every one of the five softs families prints `closed`
    // for: nine from a per-holiday notice or the calendar, and 2025-04-18 from
    // the Good Friday notice, which closes the index families too.
    let closures = [
        (2025, 1, 1),
        (2025, 1, 20),
        (2025, 2, 17),
        (2025, 4, 18),
        (2025, 5, 26),
        (2025, 6, 19),
        (2025, 7, 4),
        (2025, 9, 1),
        (2025, 11, 27),
        (2025, 12, 25),
    ];
    for which in [
        MarketHoursKey::IceUsSugar,
        MarketHoursKey::IceUsCoffee,
        MarketHoursKey::IceUsCocoa,
        MarketHoursKey::IceUsOrangeJuice,
        MarketHoursKey::IceUsCotton,
    ] {
        let calendar = key(which);
        for (year, month, date) in closures {
            assert_eq!(
                calendar
                    .holiday_on(day(year, month, date))
                    .map(Holiday::kind),
                Some(HolidayKind::Closed),
                "{which:?} {year}-{month:02}-{date:02}"
            );
            assert!(
                calendar
                    .is_closed_trade_date(day(year, month, date), SessionKind::Both)
                    .expect("the coverage contract must answer a covered date"),
                "{which:?} {year}-{month:02}-{date:02} is a closed trade date"
            );
            assert!(
                !calendar
                    .is_open(ny((year, month, date), (12, 0, 0)))
                    .expect("the coverage contract must answer a covered date"),
                "{which:?} must be shut at noon on {year}-{month:02}-{date:02}"
            );
        }
    }
}

#[test]
fn the_2025_index_early_closes_take_their_own_instants() {
    let fang = key(MarketHoursKey::IceUs);
    let usdx = key(MarketHoursKey::IceUsDollarIndex);

    // FANG+ at 13:00 NY, the instant ICE prints in its NYSE-index bullet.
    for date in [
        (2025, 1, 20),
        (2025, 2, 17),
        (2025, 5, 26),
        (2025, 6, 19),
        (2025, 9, 1),
        (2025, 11, 27),
    ] {
        let cutoff = ny(date, (13, 0, 0));
        assert_eq!(
            fang.holiday_on(day(date.0, date.1, date.2))
                .map(Holiday::kind),
            Some(HolidayKind::EarlyClose {
                close_ssm: 13 * 3_600
            }),
            "{date:?}"
        );
        assert!(
            fang.is_open(cutoff - TimeDelta::nanoseconds(1))
                .expect("the coverage contract must answer a covered date"),
            "{date:?}"
        );
        assert!(
            !fang
                .is_open(cutoff)
                .expect("the coverage contract must answer a covered date"),
            "{date:?}"
        );
    }
    // The day after Thanksgiving is its own earlier instant for both families.
    for (name, calendar) in [("FANG+", fang), ("dollar index", usdx)] {
        let cutoff = ny((2025, 11, 28), (13, 15, 0));
        assert_eq!(
            calendar.holiday_on(day(2025, 11, 28)).map(Holiday::kind),
            Some(HolidayKind::EarlyClose {
                close_ssm: 13 * 3_600 + 15 * 60
            }),
            "{name}"
        );
        assert!(
            calendar
                .is_open(cutoff - TimeDelta::nanoseconds(1))
                .expect("the coverage contract must answer a covered date"),
            "{name}"
        );
        assert!(
            !calendar
                .is_open(cutoff)
                .expect("the coverage contract must answer a covered date"),
            "{name}"
        );
    }
    // On Thanksgiving Day itself the two families differ: FANG+ closes at 13:00
    // (asserted in the loop above) and the dollar index at 13:15.
    let cutoff = ny((2025, 11, 27), (13, 15, 0));
    assert_eq!(
        usdx.holiday_on(day(2025, 11, 27)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 13 * 3_600 + 15 * 60
        })
    );
    assert!(
        usdx.is_open(cutoff - TimeDelta::nanoseconds(1))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !usdx
            .is_open(cutoff)
            .expect("the coverage contract must answer a covered date")
    );
    // The dollar index bullets print `Regular Hours` on MLK, Presidents Day,
    // Memorial Day, Juneteenth and Labor Day 2025, so those dates carry no row
    // and the family trades its ordinary Thursday or Monday.
    for date in [
        (2025, 1, 20),
        (2025, 2, 17),
        (2025, 5, 26),
        (2025, 6, 19),
        (2025, 9, 1),
    ] {
        assert_eq!(
            usdx.holiday_on(day(date.0, date.1, date.2)),
            None,
            "{date:?}"
        );
        assert!(
            usdx.is_open(ny(date, (16, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{date:?}"
        );
    }
}

#[test]
fn the_2025_mourning_notice_closes_fang_at_0930_and_the_venue_withholds_the_date() {
    let fang = key(MarketHoursKey::IceUs);
    let venue = calendar_for_exchange(Exchange::Iceus);
    let date = day(2025, 1, 9);
    // ICE prints `9:30 am NY time`, and the module states venue-local seconds
    // since midnight, so the row is 09:30 in `America/New_York`.
    let cutoff = ny((2025, 1, 9), (9, 30, 0));

    assert_eq!(
        fang.holiday_on(date).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 9 * 3_600 + 30 * 60
        })
    );
    assert_eq!(
        fang.holiday_on(date).map(Holiday::document_id),
        Some("IFUS-NOTICE-2025-MOMENT-OF-SILENCE")
    );
    // The clipping lands on the session that opened at 20:00 NY the evening
    // before and still carries this trade date, so the evening leg is open and
    // the close itself is end-exclusive.
    assert_eq!(
        fang.trade_date(ny((2025, 1, 8), (20, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(date)
    );
    assert!(
        fang.is_open(ny((2025, 1, 8), (20, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        fang.is_open(cutoff - TimeDelta::nanoseconds(1))
            .expect("the coverage contract must answer a covered date"),
        "09:29:59 NY is inside the shortened session"
    );
    assert!(
        !fang
            .is_open(cutoff)
            .expect("the coverage contract must answer a covered date"),
        "09:30:00 NY is the close, and a close is end-exclusive"
    );
    // The notice's second instant belongs to the SOFR and mortgage contracts,
    // which no crate identity models: FANG+ is already shut there, so the
    // 13:15 NY list is not this family's answer.
    assert!(
        !fang
            .is_open(ny((2025, 1, 9), (13, 14, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !fang
            .is_open(ny((2025, 1, 9), (13, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );

    // The venue table is the five-family intersection, and 2025-01-09 moves
    // exactly one family: the notice's own closing line leaves the softs and
    // the dollar index on regular hours, so every other routed family trades a
    // full session and the venue ships `Unsourced` rather than one family's
    // instant or the silence that would claim the date audited normal.
    assert_eq!(
        venue.holiday_on(date).map(Holiday::kind),
        Some(HolidayKind::Unsourced)
    );
    assert_eq!(
        venue.holiday_on(date).map(Holiday::document_id),
        Some("IFUS-NOTICE-2025-MOMENT-OF-SILENCE")
    );
    assert_declared_refusal(
        venue.is_open(ny((2025, 1, 9), (12, 0, 0))),
        DateCoverage::UnresolvedGap,
        venue,
        "the venue withholds 2025-01-09 because its families disagree",
    );
    for which in [
        MarketHoursKey::IceUsSugar,
        MarketHoursKey::IceUsCoffee,
        MarketHoursKey::IceUsCocoa,
        MarketHoursKey::IceUsCotton,
        MarketHoursKey::IceUsOrangeJuice,
        MarketHoursKey::IceUsDollarIndex,
    ] {
        let calendar = key(which);
        assert_eq!(
            calendar.holiday_on(date),
            None,
            "{which:?} is regular on 2025-01-09, which is what makes the venue withhold it"
        );
        assert!(
            calendar
                .is_open(ny((2025, 1, 9), (12, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{which:?} trades its ordinary session on 2025-01-09"
        );
    }
}

#[test]
fn the_2025_london_bank_holidays_delay_the_three_softs_opens() {
    for date in [(2025, 5, 5), (2025, 8, 25)] {
        let open = ny(date, (7, 30, 0));
        for (name, which) in [
            ("sugar", MarketHoursKey::IceUsSugar),
            ("coffee", MarketHoursKey::IceUsCoffee),
            ("cocoa", MarketHoursKey::IceUsCocoa),
        ] {
            let calendar = key(which);
            assert_eq!(
                calendar
                    .holiday_on(day(date.0, date.1, date.2))
                    .map(Holiday::kind),
                Some(HolidayKind::LateOpen {
                    open_ssm: 7 * 3_600 + 30 * 60
                }),
                "{name} {date:?}"
            );
            assert!(
                !calendar
                    .is_open(open - TimeDelta::nanoseconds(1))
                    .expect("the coverage contract must answer a covered date"),
                "{name} {date:?}"
            );
            assert!(
                calendar
                    .is_open(open)
                    .expect("the coverage contract must answer a covered date"),
                "{name} {date:?}"
            );
            assert_eq!(
                calendar
                    .session_bounds(ny(date, (9, 0, 0)))
                    .expect("the coverage contract must answer a covered date")
                    .map(|(start, _)| start),
                Some(open),
                "{name} {date:?}"
            );
        }
        // The notice's own words: "All other Exchange products will follow
        // their regular trading schedules". FCOJ's regular open is 08:00, and
        // the date is not a Cotton trade date at all — 2025-05-05 and
        // 2025-08-25 are Mondays, which that grid's opening days exclude.
        let usdx = key(MarketHoursKey::IceUsDollarIndex);
        assert_eq!(usdx.holiday_on(day(date.0, date.1, date.2)), None);
        assert_eq!(
            key(MarketHoursKey::IceUsOrangeJuice).holiday_on(day(date.0, date.1, date.2)),
            None
        );
        assert_eq!(
            key(MarketHoursKey::IceUsCotton).holiday_on(day(date.0, date.1, date.2)),
            None
        );
        assert!(
            key(MarketHoursKey::IceUsOrangeJuice)
                .is_open(ny(date, (8, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{date:?}"
        );
    }
}

#[test]
fn the_2025_thanksgiving_friday_states_each_softs_family_separately() {
    let friday = day(2025, 11, 28);

    // Sugar, Coffee and Cocoa keep regular hours, so the shared table carries
    // no row and Sugar closes at its ordinary 13:00 NY.
    let sugar = key(MarketHoursKey::IceUsSugar);
    assert_eq!(sugar.holiday_on(friday), None);
    assert!(
        sugar
            .is_open(ny((2025, 11, 28), (12, 59, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !sugar
            .is_open(ny((2025, 11, 28), (13, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // FCOJ closes early at 13:30 NY.
    let fcoj = key(MarketHoursKey::IceUsOrangeJuice);
    assert_eq!(
        fcoj.holiday_on(friday).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 13 * 3_600 + 30 * 60
        })
    );
    // Cotton opens late at 08:00 NY and closes early at 13:30 NY.
    let cotton = key(MarketHoursKey::IceUsCotton);
    assert_eq!(
        cotton.holiday_on(friday).map(Holiday::kind),
        Some(HolidayKind::LateOpenAndEarlyClose {
            open_ssm: 8 * 3_600,
            close_ssm: 13 * 3_600 + 30 * 60
        })
    );
    for (name, calendar) in [("FCOJ", fcoj), ("Cotton", cotton)] {
        assert!(
            calendar
                .is_open(ny((2025, 11, 28), (13, 29, 59)))
                .expect("the coverage contract must answer a covered date"),
            "{name}"
        );
        assert!(
            !calendar
                .is_open(ny((2025, 11, 28), (13, 30, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{name}"
        );
    }
    // Cotton's Friday trading day opens on Thanksgiving evening, so the late
    // open is observable there: the holiday evening leg does not run.
    assert!(
        !cotton
            .is_open(ny((2025, 11, 27), (21, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // Thanksgiving Day itself: the softs are closed and the index families take
    // their own instants.
    for which in [
        MarketHoursKey::IceUsSugar,
        MarketHoursKey::IceUsCoffee,
        MarketHoursKey::IceUsCocoa,
        MarketHoursKey::IceUsCotton,
        MarketHoursKey::IceUsOrangeJuice,
    ] {
        assert_eq!(
            key(which).holiday_on(day(2025, 11, 27)).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{which:?}"
        );
    }
}

#[test]
fn the_two_unretrieved_2025_notices_are_withheld_not_answered() {
    // The Independence Day and Christmas / Boxing Day notices were not
    // retrieved, so the index families' `open1` cells and the customary eve
    // early closes have no source. Those four dates ship `Unsourced` — the
    // third thing the vocabulary can say — rather than an invented instant or
    // the silence that would claim the date was audited normal.
    for which in [MarketHoursKey::IceUs, MarketHoursKey::IceUsDollarIndex] {
        let calendar = key(which);
        for date in [(2025, 7, 3), (2025, 7, 4), (2025, 12, 24), (2025, 12, 26)] {
            let holiday = day(date.0, date.1, date.2);
            assert_eq!(
                calendar.holiday_on(holiday).map(Holiday::kind),
                Some(HolidayKind::Unsourced),
                "{which:?} {holiday}"
            );
            assert_declared_refusal(
                calendar.is_open(ny(date, (12, 0, 0))),
                DateCoverage::UnresolvedGap,
                calendar,
                "an `Unsourced` date is withheld by every date-aware query",
            );
        }
        // Christmas Day itself is `closed` on the calendar's own column, so it
        // ships as a closure.
        assert_eq!(
            calendar.holiday_on(day(2025, 12, 25)).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{which:?}"
        );
    }
    // The softs columns are the calendar's own day-level answer: `closed` on
    // Independence Day and Christmas, plain `open` on Boxing Day, so those rows
    // ship and the ordinary week trades.
    let sugar = key(MarketHoursKey::IceUsSugar);
    assert_eq!(
        sugar.holiday_on(day(2025, 7, 4)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_eq!(
        sugar.holiday_on(day(2025, 12, 25)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_eq!(sugar.holiday_on(day(2025, 12, 26)), None);
    assert!(
        sugar
            .is_open(ny((2025, 12, 26), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

#[test]
fn the_2025_ordinary_week_around_the_rows_still_trades() {
    let sugar = key(MarketHoursKey::IceUsSugar);

    for (date, probe) in [
        ((2025, 3, 11), (3, 30, 0)),
        ((2025, 3, 12), (9, 0, 0)),
        ((2025, 3, 13), (12, 59, 59)),
    ] {
        assert!(
            sugar
                .is_open(ny(date, probe))
                .expect("the coverage contract must answer a covered date"),
            "{date:?} {probe:?}"
        );
        assert_eq!(
            sugar
                .trade_date(ny(date, probe))
                .expect("the coverage contract must answer a covered date"),
            Some(day(date.0, date.1, date.2)),
            "{date:?}"
        );
    }
    // The end-exclusive close and the weekend boundary, neither of which any
    // 2025 row moves.
    assert!(
        !sugar
            .is_open(ny((2025, 3, 13), (13, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !sugar
            .is_open(ny((2025, 3, 15), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // The Friday before Memorial Day and the Tuesday after it are ordinary.
    for (date, probe) in [((2025, 5, 23), (12, 0, 0)), ((2025, 5, 27), (12, 0, 0))] {
        assert!(
            sugar
                .is_open(ny(date, probe))
                .expect("the coverage contract must answer a covered date"),
            "{date:?}"
        );
    }
    // FANG+ keeps its ordinary 20:00-to-18:00 grid on the days around the
    // year-end rows.
    let fang = key(MarketHoursKey::IceUs);
    for (date, probe) in [
        ((2025, 11, 26), (12, 0, 0)),
        ((2025, 12, 1), (12, 0, 0)),
        ((2025, 12, 30), (21, 0, 0)),
    ] {
        assert!(
            fang.is_open(ny(date, probe))
                .expect("the coverage contract must answer a covered date"),
            "{date:?} {probe:?}"
        );
    }
}

/// Design memo D17: the `iceus` table **is** the intersection of the five
/// tables its seven routed keys select.
///
/// This recomputes the venue's layer from the families' own public `holiday_on`
/// answers rather than reading `VENUE` back, so a venue row that copies one
/// family's early close onto a date the others dispute fails here, and so would
/// a row dropped from a family table without the intersection following. A
/// family that states no row has audited the date normal, which is an answer,
/// so it disputes a row another family states on that date: the venue's answer
/// for a disputed date is `Unsourced`, never silence.
///
/// The seven keys share one coverage window, so no family abstains and the
/// three-state `abstains` branch the CME venue fence needs has nothing to do
/// here.
#[test]
fn the_venue_table_is_the_intersection_of_the_five_tables_its_keys_select() {
    let families = [
        MarketHoursKey::IceUs,
        MarketHoursKey::IceUsSugar,
        MarketHoursKey::IceUsCoffee,
        MarketHoursKey::IceUsCocoa,
        MarketHoursKey::IceUsCotton,
        MarketHoursKey::IceUsOrangeJuice,
        MarketHoursKey::IceUsDollarIndex,
    ];
    let venue = calendar_for_exchange(Exchange::Iceus);
    let coverage = venue
        .holiday_coverage()
        .expect("the venue ships a built-in table");
    let mut date = coverage.first();
    while date <= coverage.last() {
        let stated = families.map(|which| key(which).holiday_on(date).map(Holiday::kind));
        let expected = match stated.first() {
            // Every table audits the same window, so the first entry is a
            // sample: `None` here means audited normal.
            None => None,
            Some(first) if stated.iter().all(|other| *other == *first) => *first,
            Some(_) => Some(HolidayKind::Unsourced),
        };
        assert_eq!(
            venue.holiday_on(date).map(Holiday::kind),
            expected,
            "{date}: every family that answers must agree, or the venue withholds the date"
        );
        date = date
            .checked_add_days(Days::new(1))
            .expect("the scan stays inside the representable calendar");
    }
}
