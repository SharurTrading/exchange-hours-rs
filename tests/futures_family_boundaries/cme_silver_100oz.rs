// SPDX-License-Identifier: MIT-0

//! COMEX 100-Ounce Silver futures (`globex_silver_100oz`): the shared
//! energy/metals era by reference, the 2026-09-11 bridge day with its
//! 30-minute first-weekend maintenance, the 24/7 class grid, the notice-dated
//! Saturday extensions that govern its channel, and the separations from the
//! keys this one is most likely to be confused with.
//!
//! Every probe is stated in America/Chicago wall-clock and converted, so a DST
//! slip in either direction fails rather than passing on a coincidence.

use chrono::{DateTime, NaiveDate, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarQueryError, MarketHoursKey, SessionKind, SessionState, calendar_for_market_hours_key,
    hours_for_market_hours_key, session_bounds_with, session_profile,
};

const SILVER: MarketHoursKey = MarketHoursKey::GlobexSilver100Oz;
const ENERGY: MarketHoursKey = MarketHoursKey::GlobexEnergy;
const CRYPTOCURRENCY: MarketHoursKey = MarketHoursKey::GlobexCryptocurrency;

/// A probe instant stated in the venue's own wall clock.
fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous Central instant")
        .with_timezone(&Utc)
}

fn day(date: (i32, u32, u32)) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.0, date.1, date.2).expect("fixture must be a valid date")
}

fn open_at(key: MarketHoursKey, instant: DateTime<Utc>) -> bool {
    hours_for_market_hours_key(key, instant).is_open(instant)
}

fn state_at(instant: DateTime<Utc>) -> SessionState {
    hours_for_market_hours_key(SILVER, instant).session_state(instant)
}

/// Returns the venue-local date of the close of the extended session enclosing
/// `instant`, which is the trade date the close-date convention names.
fn close_trade_date(hours: &exchange_hours::MarketHours, instant: DateTime<Utc>) -> NaiveDate {
    session_bounds_with(hours, instant, SessionKind::Extended)
        .expect("the fixed snapshot states the enclosing session")
        .1
        .with_timezone(&US::Central)
        .date_naive()
}

/// Asserts the answer a **dormant** identity's date-aware surface gives: no
/// holiday table routes here, so the covered ranges are empty and the
/// calendar-backed surface refuses every date rather than naming a schedule.
fn assert_refused<T: core::fmt::Debug + Copy>(answer: Result<T, CalendarQueryError>, label: &str) {
    assert!(
        matches!(
            answer,
            Err(CalendarQueryError::BeforeSupportFloor { .. }
                | CalendarQueryError::OutsideCoveredRange { .. })
        ),
        "{label}: a dormant identity must refuse, got {answer:?}"
    );
}

/// The product rode the energy/metals grid to 2026-09-10: the two keys agree
/// at every instant of a full 2024 week, and the 2015 close revision (16:00,
/// not the 2010 floor's 16:15) is on both sides of it.
#[test]
fn the_shared_era_is_the_energy_metals_grid_by_reference() {
    let mut instant = ct((2024, 6, 9), (0, 0, 0));
    let end = ct((2024, 6, 16), (0, 0, 0));
    while instant < end {
        assert_eq!(
            hours_for_market_hours_key(SILVER, instant).session_state(instant),
            hours_for_market_hours_key(ENERGY, instant).session_state(instant),
            "{instant}: the shared era must be the energy/metals grid exactly"
        );
        instant += chrono::Duration::minutes(30);
    }
    // The floor-era close is 16:15 CT and the 2015-09-20 revision's is 16:00,
    // on this key's own timeline rather than only the family's: mid-week,
    // 16:05 sits inside the running session on the floor grid and inside the
    // inter-session gap on the revised one.
    assert!(open_at(SILVER, ct((2012, 6, 13), (16, 5, 0))));
    assert!(!open_at(SILVER, ct((2012, 6, 13), (16, 20, 0))));
    assert!(!open_at(SILVER, ct((2016, 6, 15), (16, 5, 0))));
    assert!(open_at(SILVER, ct((2016, 6, 15), (15, 59, 59))));
    // The last five-day Saturday is closed and the last five-day weekday
    // maintenance is the family's 60-minute break with its 16:45 queue.
    assert!(!open_at(SILVER, ct((2026, 9, 5), (12, 0, 0))));
    assert!(!open_at(SILVER, ct((2026, 9, 9), (16, 30, 0))));
    assert!(!open_at(SILVER, ct((2026, 9, 9), (16, 44, 59))));
    assert_eq!(
        state_at(ct((2026, 9, 9), (16, 50, 0))),
        SessionState::OrderEntry,
        "the family's 16:45-17:00 CT Pre-Open rides along by reference"
    );
}

/// The cutover day. Thursday's leg still opens 17:00 CT under the old grid and
/// closes Friday 16:00; that Friday's maintenance runs to 16:30; the first
/// weekend leg then opens and runs into the standard Saturday window.
#[test]
fn the_bridge_day_closes_under_the_old_grid_and_reopens_at_1630() {
    assert!(open_at(SILVER, ct((2026, 9, 10), (18, 0, 0))));
    assert!(open_at(SILVER, ct((2026, 9, 11), (15, 59, 59))));
    assert!(!open_at(SILVER, ct((2026, 9, 11), (16, 0, 0))));
    assert!(!open_at(SILVER, ct((2026, 9, 11), (16, 29, 59))));
    assert_eq!(
        state_at(ct((2026, 9, 11), (16, 15, 0))),
        SessionState::Maintenance,
        "the 30-minute first-weekend extension is the operator-designated \
         maintenance between Friday's trade date and the weekend block's Monday"
    );
    assert!(open_at(SILVER, ct((2026, 9, 11), (16, 30, 0))));
    assert!(open_at(SILVER, ct((2026, 9, 11), (23, 59, 59))));
    assert!(open_at(SILVER, ct((2026, 9, 12), (1, 59, 59))));
    assert!(!open_at(SILVER, ct((2026, 9, 12), (2, 0, 0))));
    assert_eq!(
        state_at(ct((2026, 9, 12), (3, 0, 0))),
        SessionState::Maintenance,
        "the Saturday 02:00-04:00 window is the weekly maintenance"
    );
    assert!(open_at(SILVER, ct((2026, 9, 12), (4, 0, 0))));
    assert!(
        open_at(SILVER, ct((2026, 9, 13), (12, 0, 0))),
        "Sunday trades"
    );
}

/// The 24/7 week: the two-minute daily maintenance, no weekly close, and the
/// fixed-current profile's shape.
#[test]
fn the_247_week_matches_the_class_template() {
    for monday in [(2026, 9, 14), (2027, 1, 11)] {
        assert!(
            open_at(SILVER, ct(monday, (0, 0, 0))),
            "{monday:?}: midnight"
        );
        assert!(
            open_at(SILVER, ct(monday, (15, 59, 59))),
            "{monday:?}: 15:59:59"
        );
        assert!(
            !open_at(SILVER, ct(monday, (16, 0, 0))),
            "{monday:?}: 16:00"
        );
        assert!(!open_at(SILVER, ct(monday, (16, 1, 59))));
        assert!(open_at(SILVER, ct(monday, (16, 2, 0))), "{monday:?}: 16:02");
    }
    // A Friday evening leg exists from the week after the bridge on, and the
    // weekend block is continuous except the Saturday window.
    assert!(open_at(SILVER, ct((2026, 9, 18), (20, 0, 0))));
    assert!(open_at(SILVER, ct((2026, 9, 19), (1, 59, 59))));

    let profile = session_profile(SILVER);
    assert!(
        profile.regular.is_empty(),
        "no regular session is published"
    );
    assert!(!profile.has_weekend_close);
    assert!(profile.order_entry.is_empty());
    assert!(profile.is_open(ct((2026, 9, 14), (10, 0, 0))));
    assert!(!profile.is_open(ct((2026, 9, 14), (16, 1, 0))));
    assert!(profile.is_open(ct((2026, 9, 20), (0, 0, 0))));
}

/// The key-backed calendar joins the Friday-16:30-to-Monday storage pieces
/// into one block and carries the following open business date, as the notice
/// states; the dormant identity's date-aware surface refuses rather than
/// answering, which is asserted beside each fixed-snapshot answer.
#[test]
fn the_weekend_block_is_joined_and_carries_mondays_trade_date() {
    let calendar = calendar_for_market_hours_key(SILVER);
    let hours = hours_for_market_hours_key(SILVER, ct((2026, 9, 14), (10, 0, 0)));
    let monday = day((2026, 9, 14));

    for instant in [
        ct((2026, 9, 11), (17, 0, 0)),
        ct((2026, 9, 12), (1, 0, 0)),
        ct((2026, 9, 12), (5, 0, 0)),
        ct((2026, 9, 13), (12, 0, 0)),
        ct((2026, 9, 14), (10, 0, 0)),
    ] {
        let snapshot = hours_for_market_hours_key(SILVER, instant);
        assert_refused(
            calendar.session_bounds_with(instant, SessionKind::Extended),
            &format!("{instant} joined bounds"),
        );
        assert_refused(
            calendar.trade_date(instant),
            &format!("{instant} trade date"),
        );
        assert!(
            snapshot.is_open(instant),
            "{instant}: inside the weekend block"
        );
    }
    assert_refused(
        calendar.trade_date(ct((2026, 9, 12), (3, 0, 0))),
        "the maintenance window's trade date",
    );
    assert_eq!(
        close_trade_date(&hours, ct((2026, 9, 14), (10, 0, 0))),
        monday,
        "the piece the weekend ends in closes on Monday's trade date"
    );
    // The first-weekend block from the bridge day is executable from Friday
    // 16:30 too; its Friday-evening piece closes Saturday 02:00, so the
    // close-date convention names Saturday there, and the identity's roll to
    // the following business date is the date-aware layer this dormant key
    // does not expose.
    let bridge_hours = hours_for_market_hours_key(SILVER, ct((2026, 9, 11), (17, 0, 0)));
    assert_eq!(
        close_trade_date(&bridge_hours, ct((2026, 9, 11), (17, 0, 0))),
        day((2026, 9, 12)),
        "the Friday-evening piece closes Saturday 02:00 CT"
    );
}

/// The joined weekend block survives both DST transitions without dropping or
/// double-counting an hour, and the daily maintenance sits at the same local
/// instants on both sides of each change. The block runs Saturday 04:00 CT to
/// Monday 16:00 CT, so it is 61 wall-clock hours across the November
/// fall-back and 59 across the March spring-forward.
#[test]
fn the_joined_weekend_block_survives_both_dst_transitions() {
    for (saturday, monday, hours) in [
        ((2026, 10, 31), (2026, 11, 2), 61i64),
        ((2027, 3, 13), (2027, 3, 15), 59),
    ] {
        let open = ct(saturday, (4, 0, 0));
        let close = ct(monday, (16, 0, 0));
        assert_eq!(
            (close - open).num_hours(),
            hours,
            "{saturday:?}: block length"
        );
        let snapshot = hours_for_market_hours_key(SILVER, open);
        let sunday_noon = open + chrono::Duration::hours(32);
        for instant in [open, sunday_noon, close - chrono::Duration::seconds(1)] {
            assert!(
                snapshot.is_open(instant),
                "{saturday:?}: the weekend block is executable across the clock change"
            );
        }
        assert_eq!(
            close_trade_date(&snapshot, close - chrono::Duration::seconds(1)),
            day(monday)
        );
        assert!(
            !snapshot.is_open(close),
            "{monday:?}: 16:00 closes end-exclusive"
        );
        assert_eq!(
            snapshot.session_state(ct(monday, (16, 1, 0))),
            SessionState::Maintenance,
            "{monday:?}: the daily maintenance follows the block"
        );
        assert!(snapshot.is_open(ct(monday, (16, 2, 0))));
    }
}

/// The September 19 extension is scoped to the channel table that includes
/// this product's channel 329; the October ones are class-generic. Each moves
/// only that Saturday's reopen and reverts after.
#[test]
fn the_notice_dated_saturday_extensions_move_only_that_saturdays_reopen() {
    for (saturday, reopen, last_closed) in [
        ((2026, 9, 19), (8u32, 0u32), (7u32, 59u32, 59u32)),
        ((2026, 10, 3), (5, 0), (4, 59, 59)),
        ((2026, 10, 24), (15, 30), (15, 29, 59)),
    ] {
        assert!(
            !open_at(SILVER, ct(saturday, (4, 0, 0))),
            "{saturday:?}: 04:00 stays closed"
        );
        assert!(!open_at(SILVER, ct(saturday, last_closed)));
        assert!(
            open_at(SILVER, ct(saturday, (reopen.0, reopen.1, 0))),
            "{saturday:?}: reopens at {:02}:{:02}",
            reopen.0,
            reopen.1
        );
        assert!(
            open_at(SILVER, ct(saturday, (1, 59, 59))),
            "{saturday:?}: before 02:00"
        );
    }
    // The Saturdays either side keep the standard 04:00 reopen, including the
    // first weekend after the go-live and the bridge weekend itself.
    for saturday in [(2026, 9, 12), (2026, 9, 26), (2026, 10, 10), (2026, 10, 31)] {
        assert!(!open_at(SILVER, ct(saturday, (3, 59, 59))), "{saturday:?}");
        assert!(
            open_at(SILVER, ct(saturday, (4, 0, 0))),
            "{saturday:?}: standard window"
        );
    }
    // The August extensions predate the go-live: on 2026-08-29 the product was
    // still on the five-day grid and closed for the weekend.
    assert!(!open_at(SILVER, ct((2026, 8, 29), (12, 0, 0))));
}

/// Neither neighbour can stand in. The energy/metals key keeps the five-day
/// grid after the cutover; the cryptocurrency key has a sourced Pre-Open where
/// this key serves none.
#[test]
fn neither_the_energy_nor_the_cryptocurrency_key_can_stand_in() {
    let saturday = ct((2026, 9, 19), (12, 0, 0));
    assert!(
        open_at(SILVER, saturday),
        "the 24/7 product trades the Saturday"
    );
    assert!(
        !open_at(ENERGY, saturday),
        "the family it rode did not migrate"
    );
    assert!(
        open_at(CRYPTOCURRENCY, ct((2026, 9, 19), (9, 0, 0))),
        "the cryptocurrency key reopens 08:00 that day too, and trades on"
    );

    let weekday_queue = ct((2026, 9, 14), (16, 1, 30));
    assert_eq!(state_at(weekday_queue), SessionState::Maintenance);
    assert_eq!(
        hours_for_market_hours_key(CRYPTOCURRENCY, weekday_queue).session_state(weekday_queue),
        SessionState::OrderEntry,
        "the cryptocurrency key has a sourced queue; this key has none"
    );
    // The energy key's own Sunday queue is served after its knowledge-bound
    // row; this key's 24/7 grid has no Sunday queue at all.
    let sunday = ct((2026, 9, 20), (16, 5, 0));
    assert!(open_at(SILVER, sunday), "Sunday trades straight through");
    assert_eq!(
        hours_for_market_hours_key(ENERGY, sunday).session_state(sunday),
        SessionState::OrderEntry,
        "the family key queues on Sunday evening; this product does not"
    );
}

#[test]
fn silver_100oz_key_round_trips_through_its_canonical_name() {
    assert_eq!(SILVER.as_str(), "globex_silver_100oz");
    assert_eq!("globex_silver_100oz".parse::<MarketHoursKey>(), Ok(SILVER));
    let json = serde_json::to_string(&SILVER).expect("serializes");
    assert_eq!(json, "\"globex_silver_100oz\"");
    assert_eq!(
        serde_json::from_str::<MarketHoursKey>(&json).expect("deserializes"),
        SILVER
    );
}
