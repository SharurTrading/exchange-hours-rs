// SPDX-License-Identifier: MIT-0

//! Built-in holiday rows for ICE Futures U.S. — the seven product-family keys
//! and the `iceus` venue — 2026 through the 2027 calendar's last entry.
//!
//! Three tables serve the five softs (Sugar, Coffee and Cocoa share one, FCOJ
//! and Cotton each differ by one row), and the two index families and the
//! venue have one each. The late-open branch exercised here is the one whose
//! wall clock falls **on** the trade date; no ICE row in this window states a
//! late open at or after a family's own first open, so the preceding-local-date
//! branch has nothing to exercise and this block says so rather than inventing
//! a row.

use chrono::{DateTime, Days, NaiveDate, TimeDelta, TimeZone as _, Utc};
use chrono_tz::America;
use exchange_hours::{
    CalendarResolution, Exchange, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey,
    SessionKind, calendar_for_exchange, calendar_for_market_hours_key,
};

fn key(key: MarketHoursKey) -> ExchangeCalendar {
    calendar_for_market_hours_key(key)
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
    assert!(sugar.is_closed_trade_date(memorial, SessionKind::Both));
    for probe in [(4, 0, 0), (9, 0, 0), (12, 30, 0)] {
        assert!(
            !sugar.is_open(ny((2026, 5, 25), probe)),
            "Sugar must be shut at {probe:?} on Memorial Day"
        );
    }
    // The Friday before is untouched, and the Tuesday after reopens normally.
    assert!(sugar.is_open(ny((2026, 5, 22), (9, 0, 0))));
    let next = sugar
        .next_session_after(ny((2026, 5, 22), (13, 30, 0)))
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
            !calendar.is_open(ny((2026, 4, 6), normal_open)),
            "{which:?} must not open at its normal {normal_open:?}"
        );
        assert!(
            !calendar.is_open(cutoff - TimeDelta::nanoseconds(1)),
            "{which:?}"
        );
        assert!(calendar.is_open(cutoff), "{which:?}");
        assert_eq!(
            calendar.session_bounds(cutoff).map(|(open, _)| open),
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
    assert!(key(MarketHoursKey::IceUsOrangeJuice).is_open(ny((2026, 4, 6), (8, 0, 0))));
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
    assert!(cotton.is_open(ny((2026, 7, 6), (21, 30, 0))));
    assert_eq!(
        cotton.trade_date(ny((2026, 7, 6), (21, 30, 0))),
        Some(day(2026, 7, 7))
    );
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
        assert!(!calendar.is_open(ny((2026, 4, 2), (21, 0, 0))), "{which:?}");
        assert!(
            !calendar.is_open(open_cutoff - TimeDelta::nanoseconds(1)),
            "{which:?}"
        );
        assert!(calendar.is_open(open_cutoff), "{which:?}");
        assert!(
            calendar.is_open(close_cutoff - TimeDelta::nanoseconds(1)),
            "{which:?}"
        );
        assert!(!calendar.is_open(close_cutoff), "{which:?}");
        assert_eq!(
            calendar.session_bounds(ny((2026, 4, 3), (6, 0, 0))),
            Some((open_cutoff, close_cutoff)),
            "{which:?}"
        );
        assert_eq!(
            calendar.candle_end(ny((2026, 4, 3), (6, 0, 0)), CalendarResolution::Daily),
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
            fang.is_open(fang_cutoff - TimeDelta::nanoseconds(1)),
            "{date:?}"
        );
        assert!(!fang.is_open(fang_cutoff), "{date:?}");
        assert!(
            usdx.is_open(usdx_cutoff - TimeDelta::nanoseconds(1)),
            "{date:?}"
        );
        assert!(!usdx.is_open(usdx_cutoff), "{date:?}");
    }
    // Labor Day 2026: FANG+ closes early, the dollar index keeps regular hours.
    assert!(!fang.is_open(ny((2026, 9, 7), (13, 0, 0))));
    assert_eq!(usdx.holiday_on(day(2026, 9, 7)), None);
    assert!(usdx.is_open(ny((2026, 9, 7), (16, 0, 0))));
}

#[test]
fn a_shortened_index_day_keeps_its_trade_date() {
    let fang = key(MarketHoursKey::IceUs);

    assert_eq!(
        fang.trade_date(ny((2026, 5, 24), (19, 0, 0))),
        Some(day(2026, 5, 25))
    );
    assert_eq!(
        fang.trade_date(ny((2026, 5, 25), (12, 0, 0))),
        Some(day(2026, 5, 25))
    );
    assert_eq!(
        fang.trade_date(ny((2026, 5, 25), (21, 0, 0))),
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
    assert!(!fang.is_closed_trade_date(boxing_day, SessionKind::Both));
    let probe = ny((2026, 12, 28), (12, 0, 0));
    assert_eq!(fang.is_open(probe), fang.without_holidays().is_open(probe));
    assert!(fang.is_open(probe));
}

#[test]
fn the_venue_ships_only_the_dates_every_family_agrees_on() {
    let venue = calendar_for_exchange(Exchange::Iceus);

    for date in [
        day(2026, 1, 1),
        day(2026, 12, 25),
        day(2027, 1, 1),
        day(2027, 12, 24),
    ] {
        assert_eq!(
            venue.holiday_on(date).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{date}: every modelled ICE family is closed"
        );
        assert!(
            venue.is_closed_trade_date(date, SessionKind::Both),
            "{date}"
        );
    }
    // Memorial Day 2026: the softs close, the index families shorten, so the
    // venue states no scheduling row and says the date is not audited normal.
    assert_eq!(
        venue.holiday_on(day(2026, 5, 25)).map(Holiday::kind),
        Some(HolidayKind::Unsourced)
    );
    let probe = ny((2026, 5, 25), (12, 0, 0));
    assert_eq!(
        venue.is_open(probe),
        venue.without_holidays().is_open(probe)
    );
}

#[test]
fn the_new_years_closure_removes_the_prior_evening_for_the_index_families() {
    let fang = key(MarketHoursKey::IceUs);

    assert!(fang.is_closed_trade_date(day(2026, 1, 1), SessionKind::Both));
    // Wednesday 20:00 NY would have fed trade date 2026-01-01: gone.
    assert!(!fang.is_open(ny((2025, 12, 31), (21, 0, 0))));
    // Thursday 20:00 NY feeds trade date 2026-01-02: untouched.
    assert!(fang.is_open(ny((2026, 1, 1), (21, 0, 0))));
}

#[test]
fn every_ice_table_covers_2026_through_the_2027_calendars_last_entry() {
    for (name, calendar) in every_ice_identity() {
        let coverage = calendar
            .holiday_coverage()
            .unwrap_or_else(|| panic!("{name} ships a built-in table"));
        assert_eq!(coverage.first(), day(2026, 1, 1), "{name}");
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
    // 2028-01-18 is a Martin Luther King Day the softs will observe, but it is
    // outside every window, so no table may reach it.
    let sugar = key(MarketHoursKey::IceUsSugar);
    let outside = ny((2028, 1, 18), (9, 0, 0));
    assert_eq!(sugar.holiday_on(day(2028, 1, 18)), None);
    assert_eq!(
        sugar.is_open(outside),
        sugar.without_holidays().is_open(outside)
    );
    assert!(sugar.is_open(outside));
}

#[test]
fn detaching_the_table_restores_the_normal_week() {
    let sugar = key(MarketHoursKey::IceUsSugar);
    let memorial = ny((2026, 5, 25), (9, 0, 0));

    assert!(!sugar.is_open(memorial));
    assert!(sugar.without_holidays().is_open(memorial));
    assert_eq!(sugar.without_holidays().holiday_coverage(), None);
    assert_eq!(sugar.without_holidays().holiday_on(day(2026, 5, 25)), None);
}
