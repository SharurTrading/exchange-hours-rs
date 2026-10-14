// SPDX-License-Identifier: MIT-0

//! The `eurex_smi` and `euronext_index_futures` families: public-surface
//! baselines, the 2010-07-05 SMI cutover on both sides at Zurich midnight, the
//! holiday-table splits that make each table its own scope, and the Euronext
//! floor's explicit refusals below the 2026-10-11 retrieval.

use super::prelude::*;
use chrono::TimeZone as _;
use chrono_tz::Europe;
use exchange_hours::{
    CalendarQueryError, Holiday, HolidayKind, SessionState, calendar_for_market_hours_key,
};

fn zurich(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    Europe::Zurich
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous Zurich instant")
        .with_timezone(&Utc)
}

fn paris(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    Europe::Paris
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous Paris instant")
        .with_timezone(&Utc)
}

fn utc_seconds(instant: (i32, u32, u32, u32, u32, u32)) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(
        instant.0, instant.1, instant.2, instant.3, instant.4, instant.5,
    )
    .single()
    .expect("fixture must be an unambiguous UTC instant")
}

fn day(year: i32, month: u32, date: u32) -> chrono::NaiveDate {
    chrono::NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
}

// ---------------------------------------------------------------------------
// eurex_smi
// ---------------------------------------------------------------------------

/// The baseline grid before the 2010-07-05 amendment: continuous trading
/// 07:50-17:27 with 07:30-07:50 pre-trading and 17:27-20:30 post-trading,
/// stated by the 2009-10-26 amendment that predates the support floor.
#[test]
fn smi_baseline_serves_the_pre_extension_grid_on_both_sides_of_every_boundary() {
    let key = MarketHoursKey::EurexSmi;
    let calendar = calendar_for_market_hours_key(key);

    // A winter baseline day (CET): the open and the instant before it, the
    // end-exclusive close, and both order-entry windows.
    let open = zurich((2010, 2, 1), (7, 50, 0));
    assert!(
        !open_at(key, just_before(open)),
        "the instant before the published open is closed"
    );
    assert!(
        open_regular_at(key, open),
        "07:50 CET opens the regular session"
    );
    let close = zurich((2010, 2, 1), (17, 27, 0));
    assert!(
        open_at(key, close - Duration::seconds(1)),
        "the second before the close trades"
    );
    assert!(
        !open_at(key, close),
        "closes are end-exclusive: 17:27:00 is closed"
    );
    let pre = zurich((2010, 2, 1), (7, 40, 0));
    assert_eq!(
        hours_for_market_hours_key(key, pre).session_state(pre),
        SessionState::OrderEntry,
        "07:30-07:50 CET pre-trading accepts orders without matching"
    );
    let post = zurich((2010, 2, 1), (18, 0, 0));
    assert_eq!(
        hours_for_market_hours_key(key, post).session_state(post),
        SessionState::OrderEntry,
        "17:27-20:30 CET post-trading accepts orders without matching"
    );
    assert_eq!(
        calendar
            .trade_date(post)
            .expect("the coverage contract must answer a covered date"),
        Some(day(2010, 2, 2)),
        "the post-close queue persists past the session's close, so it is dated by \
         the next session's trade date, not the civil day it prints on"
    );

    // The same grid on a summer day (CEST): identical local seconds, so the
    // UTC instants sit an hour earlier. This is the fence on the local-stable
    // reading the evidence file records.
    let summer_open = zurich((2010, 6, 1), (7, 50, 0));
    assert_eq!(
        summer_open,
        utc(2010, 6, 1, 5, 50),
        "07:50 local on a CEST day is 05:50 UTC"
    );
    assert!(open_regular_at(key, summer_open));
    assert!(!open_at(key, just_before(summer_open)));
    let summer_close = zurich((2010, 6, 1), (17, 27, 0));
    assert!(open_at(key, summer_close - Duration::seconds(1)));
    assert!(!open_at(key, summer_close));
}

/// The 2010-07-05 amendment extends continuous trading to 22:00 and moves
/// post-trading to 22:00-22:30. Both sides are tested at Zurich local
/// midnight: the last instant of 2010-07-04 resolves the baseline grid, the
/// first instant of 2010-07-05 resolves the extended one.
#[test]
fn the_smi_extension_cutover_holds_on_both_sides_of_zurich_midnight() {
    let key = MarketHoursKey::EurexSmi;

    // Zurich midnight of 2010-07-05 is 2010-07-04T22:00Z (CEST, UTC+2).
    let at = zurich((2010, 7, 5), (0, 0, 0));
    assert_eq!(
        at,
        utc(2010, 7, 4, 22, 0),
        "the cutover midnight resolves as documented"
    );
    let before = at - Duration::seconds(1);
    assert_eq!(
        before,
        utc_seconds((2010, 7, 4, 21, 59, 59)),
        "the last instant of 2010-07-04 local"
    );

    let baseline = hours_for_market_hours_key(key, before);
    assert_eq!(
        baseline.regular[0].close_ssm,
        17 * 3600 + 27 * 60,
        "2010-07-04 still closes continuous trading at 17:27 local"
    );
    assert_eq!(
        baseline.order_entry[1].close_ssm,
        20 * 3600 + 30 * 60,
        "and runs the 17:27-20:30 post-trading leg"
    );

    let extended = hours_for_market_hours_key(key, at);
    assert_eq!(
        extended.regular[0].close_ssm,
        22 * 3600,
        "2010-07-05 opens on the amended 22:00 close"
    );
    assert_eq!(
        extended.order_entry[1].open_ssm,
        22 * 3600,
        "with the post-trading leg moved to 22:00-22:30"
    );

    // Behaviourally, both sides: the baseline Friday trades to 17:27 and its
    // 19:00 post-close queue accepts no matching order; the extended grid
    // trades at 19:00, where the baseline was dark, and ends end-exclusively
    // at 22:00 local.
    assert!(
        open_at(key, zurich((2010, 7, 2), (15, 0, 0))),
        "the baseline trades to 17:27"
    );
    assert!(
        !open_at(key, zurich((2010, 7, 2), (19, 0, 0))),
        "the baseline's post-close is order-entry only"
    );
    assert!(
        open_at(key, zurich((2010, 7, 6), (19, 0, 0))),
        "the extended grid trades at 19:00, where the baseline was dark"
    );
    let new_close = zurich((2010, 7, 6), (22, 0, 0));
    assert!(open_at(key, new_close - Duration::seconds(1)));
    assert!(
        !open_at(key, new_close),
        "the extended close is end-exclusive too"
    );
    let new_post = zurich((2010, 7, 6), (22, 15, 0));
    assert_eq!(
        hours_for_market_hours_key(key, new_post).session_state(new_post),
        SessionState::OrderEntry,
        "22:00-22:30 is the post-close queue"
    );
    assert_eq!(
        calendar_for_market_hours_key(key)
            .trade_date(new_post)
            .expect("the coverage contract must answer a covered date"),
        Some(day(2010, 7, 7)),
        "the amended post-close queue feeds the next session's trade date too"
    );
}

/// The holiday table is the all-derivatives rows plus the Swiss-scope rows:
/// every Swiss clause date closes FSMI, the German-scope dates never reach it,
/// and the all-derivatives rows close it like every other Eurex family.
#[test]
fn the_smi_table_closes_the_swiss_and_all_derivatives_dates_and_keeps_the_german_ones() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::EurexSmi);
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a built-in table");
    assert_eq!(
        coverage.first(),
        day(2010, 1, 1),
        "the window opens at the floor"
    );
    assert_eq!(
        coverage.last(),
        day(2026, 12, 31),
        "the window stops at the operator's horizon"
    );

    // Swiss-scope rows, one per era shape: the 2010 pair, the 2012 four-date
    // pattern, the 2015 two-date year, the 2020 and 2021 years without 1
    // August (a Saturday and a Sunday), and the newest editions.
    for date in [
        (2010, 5, 13),
        (2010, 5, 24),
        (2012, 1, 2),
        (2012, 5, 17),
        (2012, 5, 28),
        (2012, 8, 1),
        (2015, 1, 2),
        (2015, 5, 14),
        (2020, 6, 1),
        (2021, 5, 24),
        (2025, 8, 1),
        (2026, 5, 14),
    ] {
        let d = day(date.0, date.1, date.2);
        assert_eq!(
            calendar.holiday_on(d).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "the Swiss clause closes {d}"
        );
        assert!(
            !calendar
                .is_open(zurich(date, (12, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{d:?} is shut at noon"
        );
    }

    // All-derivatives rows close this family like every Eurex identity,
    // including the 2015 Whit Monday the edition's CHF annotation rides.
    for date in [(2013, 4, 1), (2015, 5, 25), (2021, 12, 31), (2026, 12, 24)] {
        assert_eq!(
            calendar
                .holiday_on(day(date.0, date.1, date.2))
                .map(Holiday::kind),
            Some(HolidayKind::Closed),
            "the all-derivatives list closes {date:?}"
        );
    }

    // The German-scope rows stay the benchmark tables' alone: FSMI trades
    // through every German-only date, and the panel's own Swiss line is what
    // puts rows in this table. The Whit Mondays the German note closes
    // (2016-05-16, 2017-06-05, 2018-05-21) are **also** Swiss clause dates,
    // so they close here regardless — the table above fences them.
    for date in [
        (2014, 10, 3),
        (2016, 10, 3),
        (2017, 10, 3),
        (2017, 10, 31),
        (2018, 10, 3),
    ] {
        let d = day(date.0, date.1, date.2);
        assert_eq!(
            calendar.holiday_on(d),
            None,
            "the German-scope note never names this family: {d} is audited normal"
        );
        assert!(
            calendar
                .is_open(zurich(date, (12, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{date:?} trades through the German-scope closure"
        );
    }

    // A date without a row beside the closures is audited normal.
    assert_eq!(
        calendar.holiday_on(day(2026, 5, 13)),
        None,
        "13 May 2026 is audited normal"
    );
    assert!(
        calendar
            .is_open(zurich((2026, 5, 13), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );

    // The window's edges: 2027 is the operator's indicative-calendar era and
    // refuses with the table attached; below the support floor is the floor
    // error.
    let outside = zurich((2027, 1, 4), (12, 0, 0));
    assert!(matches!(
        calendar.is_open(outside),
        Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == day(2027, 1, 4)
    ));
    assert_eq!(
        calendar.holiday_on(day(2027, 1, 4)),
        None,
        "the indicative 2027 calendar ships no row"
    );
    let below = zurich((2009, 12, 31), (12, 0, 0));
    assert!(matches!(
        calendar.is_open(below),
        Err(CalendarQueryError::BeforeSupportFloor { date, .. }) if date == day(2009, 12, 31)
    ));

    // No wrap: the family's sessions stay inside one Zurich civil day, so
    // detaching the table restores the same grid and the weekend boundary is
    // the ordinary close.
    assert!(
        !calendar
            .is_open(zurich((2026, 5, 16), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "Saturday 16 May 2026 is closed"
    );
    assert!(calendar.holiday_coverage().is_some());
}

/// The canonical wire name round-trips through serde and `FromStr`.
#[test]
fn the_smi_wire_name_is_canonical() {
    let name = "eurex_smi";
    let key: MarketHoursKey = name.parse().expect("the canonical name parses");
    assert_eq!(key.as_str(), name);
    assert_eq!(
        serde_json::to_string(&key).expect("a unit variant serializes"),
        "\"eurex_smi\""
    );
    assert_eq!(
        serde_json::from_str::<MarketHoursKey>("\"eurex_smi\"")
            .expect("the canonical string deserializes"),
        MarketHoursKey::EurexSmi
    );
}

// ---------------------------------------------------------------------------
// euronext_index_futures
// ---------------------------------------------------------------------------

/// The family answers its shared Central Order Book grid on and after the
/// 2026-10-11 contract-page retrieval, on both sides of the DST changeover,
/// and refuses every date below the retrieval explicitly.
#[test]
fn the_euronext_grid_answers_from_its_horizon_and_refuses_below_it() {
    let key = MarketHoursKey::EuronextIndexFutures;
    let calendar = calendar_for_market_hours_key(key);

    // 2026-10-12 is a Monday inside CEST: the open is 08:00 local = 06:00 UTC,
    // the close 22:00 local = 20:00 UTC, end-exclusive.
    let open = paris((2026, 10, 12), (8, 0, 0));
    assert_eq!(open, utc(2026, 10, 12, 6, 0), "08:00 CEST is 06:00 UTC");
    assert!(!open_at(key, just_before(open)));
    assert!(open_regular_at(key, open));
    let close = paris((2026, 10, 12), (22, 0, 0));
    assert_eq!(close, utc(2026, 10, 12, 20, 0));
    assert!(open_at(key, close - Duration::seconds(1)));
    assert!(!open_at(key, close), "the 22:00 close is end-exclusive");

    // 2026-11-16 is inside CET: the same local seconds, one UTC hour later.
    // This is the fence on the local-stable CET/CEST reading.
    let winter_open = paris((2026, 11, 16), (8, 0, 0));
    assert_eq!(winter_open, utc(2026, 11, 16, 7, 0));
    assert!(open_regular_at(key, winter_open));
    let winter_close = paris((2026, 11, 16), (22, 0, 0));
    assert_eq!(winter_close, utc(2026, 11, 16, 21, 0));
    assert!(open_at(key, winter_close - Duration::seconds(1)));
    assert!(!open_at(key, winter_close));

    // No lunch break and no order-entry phase: the grid runs 08:00-22:00
    // straight through, and order acceptance is exactly the open state.
    let mid = paris((2026, 11, 16), (13, 0, 0));
    assert!(open_at(key, mid), "the grid has no lunch break");
    assert_eq!(
        hours_for_market_hours_key(key, mid).session_state(mid),
        SessionState::OpenRegular
    );
    let early = paris((2026, 11, 16), (7, 45, 0));
    assert!(
        !hours_for_market_hours_key(key, early).is_accepting_orders(early),
        "no pre-open phase is published, so 07:45 accepts nothing"
    );

    // The weekend boundary: Friday trades to the end-exclusive close, Saturday
    // is dark, and Monday opens at the published instant.
    assert!(open_at(key, paris((2026, 10, 16), (21, 59, 0))));
    assert!(
        !calendar
            .is_open(paris((2026, 10, 17), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "Saturday 17 October 2026 is closed"
    );
    assert!(!open_at(key, just_before(open)));
    let monday = paris((2026, 10, 19), (8, 0, 0));
    assert!(open_regular_at(key, monday));

    // Trade dates: the day session ends inside its own civil day, so the
    // trade date is the Paris date of the final close.
    assert_eq!(
        calendar
            .trade_date(paris((2026, 10, 12), (21, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 10, 12))
    );

    // Below the horizon the whole date refuses: the grid is carried, not
    // sourced, and the error contract names the refusal rather than serving an
    // inferred history. The holiday layer's own rows still resolve for the
    // dates they audited.
    let carried = paris((2025, 6, 10), (12, 0, 0));
    assert!(matches!(
        calendar.is_open(carried),
        Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == day(2025, 6, 10)
    ));
    assert!(calendar.trade_date(carried).is_err());
    assert_eq!(
        calendar.holiday_on(day(2025, 12, 25)).map(Holiday::kind),
        Some(HolidayKind::Closed),
        "the audited 2025 Christmas row resolves as data even while the date refuses sessions"
    );
}

/// The family's holiday rows on the answerable side of the horizon: the
/// Christmas closure answers, the two announced-but-unstated half days refuse
/// as `Unsourced`, and the table's window is the operator's published one.
#[test]
fn the_euronext_table_classifies_its_answerable_dates_and_withholds_the_unstated_eves() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::EuronextIndexFutures);
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a built-in table");
    assert_eq!(coverage.first(), day(2010, 1, 1));
    assert_eq!(coverage.last(), day(2026, 12, 31));

    // The one closure inside the answerable range: Christmas 2026.
    let christmas = day(2026, 12, 25);
    assert_eq!(
        calendar.holiday_on(christmas).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(
        !calendar
            .is_open(paris((2026, 12, 25), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );

    // The two 2026 half days are announced with unstated instants: the table
    // withholds them and the classification refuses typed instead of claiming
    // an ordinary or shortened day.
    for eve in [day(2026, 12, 24), day(2026, 12, 31)] {
        assert_eq!(
            calendar.holiday_on(eve).map(Holiday::kind),
            Some(HolidayKind::Unsourced),
            "{eve} is the operator's announced half day with unstated hours"
        );
        assert!(matches!(
            calendar.is_closed_trade_date(eve, exchange_hours::SessionKind::Both),
            Err(CalendarQueryError::UnresolvedGap { date, .. }) if date == eve
        ));
    }

    // The 2011 eves are the operator's own full-day statement for index
    // products, so they carry no row: audited normal.
    assert_eq!(calendar.holiday_on(day(2011, 12, 23)), None);
    assert_eq!(calendar.holiday_on(day(2011, 12, 30)), None);
    // The 2016/2017 eves close at the usual times for cash AND derivatives.
    assert_eq!(calendar.holiday_on(day(2016, 12, 23)), None);
    assert_eq!(calendar.holiday_on(day(2017, 12, 22)), None);
    // The 2022/2023 eves print Full Day Trading for Paris.
    assert_eq!(calendar.holiday_on(day(2022, 12, 23)), None);

    // The window's far edge: 2027 is unpublished and refuses with the table
    // attached; detaching it restores the pure normal week inside the sourced
    // range only.
    let outside = paris((2027, 1, 4), (12, 0, 0));
    assert!(matches!(
        calendar.is_open(outside),
        Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == day(2027, 1, 4)
    ));
}

/// The canonical wire name round-trips through serde and `FromStr`.
#[test]
fn the_euronext_wire_name_is_canonical() {
    let name = "euronext_index_futures";
    let key: MarketHoursKey = name.parse().expect("the canonical name parses");
    assert_eq!(key.as_str(), name);
    assert_eq!(
        serde_json::to_string(&key).expect("a unit variant serializes"),
        "\"euronext_index_futures\""
    );
    assert_eq!(
        serde_json::from_str::<MarketHoursKey>("\"euronext_index_futures\"")
            .expect("the canonical string deserializes"),
        MarketHoursKey::EuronextIndexFutures
    );
}
