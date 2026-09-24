// SPDX-License-Identifier: MIT-0

//! CME Event Contracts on Bitcoin Futures (`globex_event_contracts_btc`): the
//! sessionless era before the 2023-03-12 listing, the shared event-contract
//! grid to 2026-05-28, the 2026-05-29 transition day, the 24/7 sourced
//! intersection with its withheld hour, the three notice-dated Saturday
//! extensions, and the separations from the two keys this one is most likely
//! to be confused with.
//!
//! Every probe is stated in America/Chicago wall-clock and converted, so a DST
//! slip in either direction fails rather than passing on a coincidence.

use chrono::{DateTime, NaiveDate, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarQueryError, MarketHoursKey, SessionKind, SessionState, calendar_for_market_hours_key,
    hours_for_market_hours_key, session_bounds_with, session_profile,
};

const BTC_EVENTS: MarketHoursKey = MarketHoursKey::GlobexEventContractsBtc;
const EVENT_CONTRACTS: MarketHoursKey = MarketHoursKey::GlobexEventContracts;
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

/// The state the **fixed snapshot** reports, which is the surface these grid
/// assertions are about.
///
/// `GlobexEventContractsBtc` is dormant (LAW-SERVICE-TIERS): CME publishes no
/// holiday table in the crate for it, so its date-aware coverage is empty above
/// the 2025 floor and the calendar-backed surface refuses every date rather
/// than answering. The published grid is still a sourced fact and
/// `hours_for_market_hours_key` states it, so the boundary and classification
/// assertions below keep their force through that surface; `assert_refused` is
/// how the calendar-backed surface is asserted to refuse the same dates.
fn state_at(instant: DateTime<Utc>) -> SessionState {
    hours_for_market_hours_key(BTC_EVENTS, instant).session_state(instant)
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

/// Asserts the answer a **dormant** identity's date-aware surface gives.
///
/// No date is sourced for this key: one at or after the permanent 2025 floor
/// has no covered range, and one before it precedes the floor. Either way the
/// calendar-backed surface refuses instead of naming a schedule
/// (LAW-COVERAGE), which is the fact these tests now state in place of the
/// schedule they used to read back out of it.
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

/// The root did not exist before SER-9092's listing day and rode the
/// event-contract grid from it: the two keys agree at every instant of a full
/// 2024 week, state by state, and the day before the listing is sessionless.
#[test]
fn the_shared_era_is_the_event_contract_grid_by_reference() {
    assert!(
        !open_at(BTC_EVENTS, ct((2023, 3, 8), (12, 0, 0))),
        "before the 2023-03-12 listing the root is sessionless"
    );
    assert!(
        open_at(BTC_EVENTS, ct((2023, 3, 13), (12, 0, 0))),
        "SER-9092 lists the root for trade date Monday 2023-03-13"
    );

    let shared = calendar_for_market_hours_key(EVENT_CONTRACTS);
    let own = calendar_for_market_hours_key(BTC_EVENTS);
    // 2024-06-09 is a Sunday; every 30 minutes through Saturday 2024-06-15.
    let mut instant = ct((2024, 6, 9), (0, 0, 0));
    let end = ct((2024, 6, 16), (0, 0, 0));
    while instant < end {
        // 2024 precedes the permanent floor, so neither date-aware surface
        // answers: both refuse rather than stating the shared era. The grid
        // comparison the probe makes is therefore read from the two fixed
        // snapshots, which are the surface that still states it.
        assert_refused(own.session_state(instant), &format!("{instant} btc"));
        assert_refused(
            shared.session_state(instant),
            &format!("{instant} event contracts"),
        );
        assert_eq!(
            hours_for_market_hours_key(BTC_EVENTS, instant).session_state(instant),
            hours_for_market_hours_key(EVENT_CONTRACTS, instant).session_state(instant),
            "{instant}: the shared era must be the event-contract grid exactly"
        );
        instant += chrono::Duration::minutes(30);
    }
    assert_eq!(
        state_at(ct((2024, 6, 9), (16, 30, 0))),
        SessionState::OrderEntry,
        "the Sunday 16:00-17:00 CT Pre-Open is order entry, as the launch document states"
    );
}

/// The cutover day. The wiki states the instant — 16:00 CT on Friday
/// 2026-05-29 — so Thursday's open closes at Friday 16:00 under the old grid,
/// the first 24/7 queue opens at 16:01, matching resumes at 16:02, and the
/// Saturday window follows.
#[test]
fn the_transition_day_closes_under_the_old_grid_and_reopens_under_the_new() {
    assert!(open_at(BTC_EVENTS, ct((2026, 5, 29), (15, 30, 0))));
    assert!(!open_at(BTC_EVENTS, ct((2026, 5, 29), (16, 0, 0))));
    assert!(!open_at(BTC_EVENTS, ct((2026, 5, 29), (16, 0, 59))));
    assert_eq!(
        state_at(ct((2026, 5, 29), (16, 1, 30))),
        SessionState::OrderEntry
    );
    assert!(open_at(BTC_EVENTS, ct((2026, 5, 29), (16, 2, 0))));
    assert!(open_at(BTC_EVENTS, ct((2026, 5, 30), (1, 59, 59))));
    assert!(!open_at(BTC_EVENTS, ct((2026, 5, 30), (2, 0, 0))));
    assert_eq!(
        state_at(ct((2026, 5, 30), (3, 0, 0))),
        SessionState::Maintenance
    );
    assert_eq!(
        state_at(ct((2026, 5, 30), (3, 50, 0))),
        SessionState::OrderEntry
    );
    assert!(open_at(BTC_EVENTS, ct((2026, 5, 30), (4, 0, 0))));
    assert!(
        open_at(BTC_EVENTS, ct((2026, 5, 31), (12, 0, 0))),
        "Sunday trades"
    );
}

/// The sourced intersection: open 16:02→15:00 CT Monday-Friday, with the
/// disputed 15:00-16:00 hour withheld as maintenance rather than served, in
/// both a summer and a winter week.
#[test]
fn the_weekday_close_is_the_intersection_and_the_disputed_hour_is_withheld() {
    for monday in [(2026, 6, 8), (2027, 1, 11)] {
        assert!(
            open_at(BTC_EVENTS, ct(monday, (14, 59, 59))),
            "{monday:?}: 14:59:59"
        );
        assert!(
            !open_at(BTC_EVENTS, ct(monday, (15, 0, 0))),
            "{monday:?}: 15:00 closes"
        );
        assert!(
            !open_at(BTC_EVENTS, ct(monday, (15, 30, 0))),
            "{monday:?}: the disputed hour"
        );
        assert!(
            !open_at(BTC_EVENTS, ct(monday, (16, 0, 0))),
            "{monday:?}: 16:00"
        );
        assert_eq!(
            state_at(ct(monday, (15, 30, 0))),
            SessionState::Maintenance,
            "{monday:?}: the withheld hour sits inside one continuous week"
        );
        assert_eq!(state_at(ct(monday, (16, 1, 30))), SessionState::OrderEntry);
        assert!(
            open_at(BTC_EVENTS, ct(monday, (16, 2, 0))),
            "{monday:?}: 16:02 reopens"
        );
    }

    let profile = session_profile(BTC_EVENTS);
    assert!(profile.regular.is_empty());
    assert!(!profile.has_weekend_close);
    assert!(profile.is_open(ct((2026, 6, 5), (14, 59, 59))));
    assert!(!profile.is_open(ct((2026, 6, 5), (15, 0, 0))));
    assert!(!profile.is_open(ct((2026, 6, 5), (16, 1, 30))));
    assert!(profile.is_open(ct((2026, 6, 5), (16, 2, 0))));
    assert!(profile.is_open(ct((2026, 6, 7), (0, 0, 0))));
}

/// The key-backed calendar joins the Saturday-04:00-to-Monday-15:00 storage
/// pieces into one block and carries the following open business date, as
/// both CME documents say the daily window rolls the trade date.
#[test]
fn the_weekend_block_is_joined_and_carries_mondays_trade_date() {
    let calendar = calendar_for_market_hours_key(BTC_EVENTS);
    let hours = hours_for_market_hours_key(BTC_EVENTS, ct((2026, 6, 8), (10, 0, 0)));
    let monday = day((2026, 6, 8));

    // The key is dormant, so the date-aware surface refuses the whole query set
    // this probe used to read: the joined block's bounds, the trade date it
    // carries and the forward scan. What the fixed snapshot still states is
    // that every one of these instants is executable, and — through the
    // close-date convention — that the piece the weekend ends in closes on the
    // Monday the block carries.
    for instant in [
        ct((2026, 6, 5), (17, 0, 0)),
        ct((2026, 6, 6), (1, 0, 0)),
        ct((2026, 6, 6), (5, 0, 0)),
        ct((2026, 6, 7), (12, 0, 0)),
        ct((2026, 6, 8), (10, 0, 0)),
    ] {
        let snapshot = hours_for_market_hours_key(BTC_EVENTS, instant);
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
    // The 02:00-04:00 CT window between Friday's leg and Saturday's reopen is
    // the operator's maintenance period, not a closure. It has no trade date,
    // and the date-aware surface refuses to name one.
    assert_eq!(
        state_at(ct((2026, 6, 6), (3, 0, 0))),
        SessionState::Maintenance
    );
    assert_refused(
        calendar.trade_date(ct((2026, 6, 6), (3, 0, 0))),
        "the maintenance window's trade date",
    );
    assert_refused(
        calendar.next_session_after(ct((2026, 6, 6), (5, 0, 0))),
        "the weekend forward scan",
    );
    // The block's final piece closes Monday 15:00 CT, which is the trade date
    // the close-date convention names.
    assert_eq!(
        close_trade_date(&hours, ct((2026, 6, 8), (10, 0, 0))),
        monday,
        "the piece the weekend ends in closes on Monday's trade date"
    );
}

/// The joined weekend block straddles both US DST transitions: the Saturday
/// 04:00 CT open to the Monday 15:00 CT close is 60 wall-clock hours across
/// the November fall-back and 58 across the March spring-forward. The join must
/// neither drop nor double-count that hour, and the trade date stays Monday.
#[test]
fn the_joined_weekend_block_survives_both_dst_transitions() {
    let calendar = calendar_for_market_hours_key(BTC_EVENTS);
    for (saturday, monday, hours) in [
        ((2026, 10, 31), (2026, 11, 2), 60i64),
        ((2027, 3, 13), (2027, 3, 15), 58),
    ] {
        let open = ct(saturday, (4, 0, 0));
        let close = ct(monday, (15, 0, 0));
        assert_eq!(
            (close - open).num_hours(),
            hours,
            "{saturday:?}: block length"
        );
        let sunday_noon = open + chrono::Duration::hours(32);
        let snapshot = hours_for_market_hours_key(BTC_EVENTS, open);
        // The joined bounds and the trade date are date-aware conventions this
        // dormant key no longer states, so each probe asserts that refusal.
        // What the fixed snapshot still states is that the block is executable
        // at the same instants, which is the DST claim: neither the fall-back's
        // extra hour nor the spring-forward's missing one drops it.
        for instant in [open, sunday_noon, close - chrono::Duration::seconds(1)] {
            assert_refused(
                calendar.session_bounds_with(instant, SessionKind::Extended),
                &format!("{saturday:?} joined bounds"),
            );
            assert_refused(
                calendar.trade_date(instant),
                &format!("{saturday:?} trade date"),
            );
            assert!(
                snapshot.is_open(instant),
                "{saturday:?}: the weekend block is executable across the clock change"
            );
        }
        // The block's final piece closes on the Monday it carries.
        assert_eq!(
            close_trade_date(&snapshot, close - chrono::Duration::seconds(1)),
            day(monday)
        );
        assert_refused(calendar.is_open(close), &format!("{monday:?} close"));
        assert!(
            !snapshot.is_open(close),
            "{monday:?}: 15:00 CT closes end-exclusive"
        );
        assert_refused(
            calendar.session_state(ct(monday, (15, 30, 0))),
            &format!("{monday:?} maintenance"),
        );
        assert_eq!(
            hours_for_market_hours_key(BTC_EVENTS, ct(monday, (15, 30, 0)))
                .session_state(ct(monday, (15, 30, 0))),
            SessionState::Maintenance
        );
        assert_refused(
            calendar.is_open(ct(monday, (16, 2, 0))),
            &format!("{monday:?} reopen"),
        );
        assert!(
            hours_for_market_hours_key(BTC_EVENTS, ct(monday, (16, 2, 0)))
                .is_open(ct(monday, (16, 2, 0))),
            "{monday:?}: 16:02 CT reopens"
        );
        // The Friday evening block ahead of it is executable on the right side
        // of the clock change too.
        let friday = (saturday.0, saturday.1, saturday.2 - 1);
        let friday_evening = ct(friday, (20, 0, 0));
        assert_refused(
            calendar.session_bounds_with(friday_evening, SessionKind::Extended),
            &format!("{friday:?} Friday-evening bounds"),
        );
        assert_refused(
            calendar.trade_date(friday_evening),
            &format!("{friday:?} Friday-evening trade date"),
        );
        assert!(
            hours_for_market_hours_key(BTC_EVENTS, friday_evening).is_open(friday_evening),
            "{friday:?}: Friday evening is executable"
        );
    }
}

/// Three one-day Saturday extensions CME announced for channel 329, each
/// reverting to the standard window the following week.
#[test]
fn the_notice_dated_saturday_extensions_move_only_that_saturdays_reopen() {
    for (saturday, reopen_hour) in [((2026, 8, 1), 9u32), ((2026, 8, 29), 6), ((2026, 9, 19), 8)] {
        assert!(
            !open_at(BTC_EVENTS, ct(saturday, (4, 0, 0))),
            "{saturday:?}: 04:00 stays closed"
        );
        assert!(!open_at(
            BTC_EVENTS,
            ct(saturday, (reopen_hour - 1, 59, 59))
        ));
        assert!(
            open_at(BTC_EVENTS, ct(saturday, (reopen_hour, 0, 0))),
            "{saturday:?}: reopens at {reopen_hour}:00"
        );
        assert!(
            open_at(BTC_EVENTS, ct(saturday, (1, 59, 59))),
            "{saturday:?}: before 02:00"
        );
    }
    // The Saturdays either side keep the standard 04:00 reopen.
    for saturday in [(2026, 7, 25), (2026, 8, 8), (2026, 9, 5), (2026, 9, 26)] {
        assert!(
            !open_at(BTC_EVENTS, ct(saturday, (3, 59, 59))),
            "{saturday:?}"
        );
        assert!(
            open_at(BTC_EVENTS, ct(saturday, (4, 0, 0))),
            "{saturday:?}: standard window"
        );
    }
}

/// Neither neighbour can stand in. The event-contract key stays five-day after
/// the cutover; the cryptocurrency key had no sourced Pre-Open in 2024 and
/// trades the hour this key withholds.
#[test]
fn neither_the_event_contract_nor_the_cryptocurrency_key_can_stand_in() {
    let saturday = ct((2026, 6, 6), (12, 0, 0));
    assert!(open_at(BTC_EVENTS, saturday));
    assert!(
        !open_at(EVENT_CONTRACTS, saturday),
        "the siblings did not migrate"
    );

    let disputed = ct((2026, 6, 8), (15, 30, 0));
    assert!(!open_at(BTC_EVENTS, disputed), "withheld here");
    assert!(
        open_at(CRYPTOCURRENCY, disputed),
        "served by the cryptocurrency key"
    );

    let sunday_queue = ct((2024, 6, 9), (16, 30, 0));
    assert_eq!(state_at(sunday_queue), SessionState::OrderEntry);
    // 2024 precedes the floor, so the cryptocurrency calendar refuses the
    // instant rather than answering; the comparison the claim needs is stated
    // by the same key's fixed snapshot.
    assert_refused(
        calendar_for_market_hours_key(CRYPTOCURRENCY).session_state(sunday_queue),
        "the cryptocurrency five-day probe",
    );
    assert_ne!(
        hours_for_market_hours_key(CRYPTOCURRENCY, sunday_queue).session_state(sunday_queue),
        SessionState::OrderEntry,
        "the cryptocurrency key has no sourced five-day Pre-Open"
    );
}

#[test]
fn bitcoin_event_contracts_key_round_trips_through_its_canonical_name() {
    assert_eq!(BTC_EVENTS.as_str(), "globex_event_contracts_btc");
    assert_eq!(
        "globex_event_contracts_btc".parse::<MarketHoursKey>(),
        Ok(BTC_EVENTS)
    );
    let json = serde_json::to_string(&BTC_EVENTS).expect("serializes");
    assert_eq!(json, "\"globex_event_contracts_btc\"");
    assert_eq!(
        serde_json::from_str::<MarketHoursKey>(&json).expect("deserializes"),
        BTC_EVENTS
    );
}
