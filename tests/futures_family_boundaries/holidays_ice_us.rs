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
        if date == day(2026, 1, 1) {
            // The row ships, but the trade-date query needs the trading day
            // that opened the previous evening, and 2025-12-31 is one day
            // below the audited window. Stage 2B refuses the date rather than
            // answering it from an unaudited normal week.
            assert_declared_refusal(
                venue.is_closed_trade_date(date, SessionKind::Both),
                DateCoverage::OutsideCoveredRange,
                venue,
                "New Year's Day's trade-date answer rests on the unaudited 2025-12-31",
            );
            continue;
        }
        assert!(
            venue
                .is_closed_trade_date(date, SessionKind::Both)
                .expect("the trade date's own day is inside the audited window"),
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

#[test]
fn the_new_years_closure_removes_the_prior_evening_for_the_index_families() {
    let fang = key(MarketHoursKey::IceUs);

    // The 2026-01-01 row ships, but both halves of the claim this test names
    // depend on 2025-12-31 — the trade date's own opening day — which is one
    // day below the audited window. The deletion is therefore refused rather
    // than asserted.
    assert_eq!(
        fang.holiday_on(day(2026, 1, 1)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_declared_refusal(
        fang.is_closed_trade_date(day(2026, 1, 1), SessionKind::Both),
        DateCoverage::OutsideCoveredRange,
        fang,
        "New Year's Day's trade-date answer rests on the unaudited 2025-12-31",
    );
    // Wednesday 20:00 NY would have fed trade date 2026-01-01: refused.
    assert_declared_refusal(
        fang.is_open(ny((2025, 12, 31), (21, 0, 0))),
        DateCoverage::OutsideCoveredRange,
        fang,
        "2025-12-31 is below the audited window",
    );
    // Thursday 20:00 NY feeds trade date 2026-01-02, but resolving whether a
    // session was already running still reads the preceding local date, so the
    // "untouched" half of the name is no longer claimable either.
    assert_declared_refusal(
        fang.is_open(ny((2026, 1, 1), (21, 0, 0))),
        DateCoverage::OutsideCoveredRange,
        fang,
        "the probe depends on 2025-12-31, below the audited window",
    );
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
    // outside every window, so no table may reach it. Stage 2B refuses the date
    // for the identity calendar too: the old claim that it answers the pure
    // normal week outside its window is no longer claimable, because an
    // identity with an attached table returns no answer where the table has
    // none. The normal week is still observable on the detached snapshot.
    let sugar = key(MarketHoursKey::IceUsSugar);
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
