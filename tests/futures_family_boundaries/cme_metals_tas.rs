// SPDX-License-Identifier: MIT-0

//! The five CME metals Trading at Settlement keys: the sourced closure before
//! each launch, the launch day itself, the published open and the instant
//! before it, the end-exclusive close, the inter-trade-date gap and the
//! weekend, the two order-entry revisions the COMEX three share, the withheld
//! Sunday quarter-hour, and the separations from the keys each is most likely
//! to be confused with.
//!
//! Every probe is stated in `America/Chicago` wall clock and converted, so a
//! DST slip in either direction fails rather than passing on a coincidence.
//! Every dated row is exercised through `calendar_for_market_hours_key`, which
//! reselects per venue-local opening day, as well as through the instant
//! selector.

use chrono::{DateTime, NaiveDate, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    MarketHoursKey, SessionKind, SessionState, calendar_for_market_hours_key,
    hours_for_market_hours_key, session_profile,
};

const GOLD: MarketHoursKey = MarketHoursKey::GlobexGoldTas;
const SILVER: MarketHoursKey = MarketHoursKey::GlobexSilverTas;
const COPPER: MarketHoursKey = MarketHoursKey::GlobexCopperTas;
const PLATINUM: MarketHoursKey = MarketHoursKey::GlobexPlatinumTas;
const PALLADIUM: MarketHoursKey = MarketHoursKey::GlobexPalladiumTas;
/// The COMEX metals outrights, whose 2015 close move these keys did not share.
const OUTRIGHTS: MarketHoursKey = MarketHoursKey::GlobexEnergy;

/// A venue-local calendar date stated as `(year, month, day)`.
type Ymd = (i32, u32, u32);
/// A venue-local wall-clock time stated as `(hour, minute)`.
type Hm = (u32, u32);

/// `(key, launch opening day, close hour and minute)`.
const KEYS: [(MarketHoursKey, Ymd, Hm); 5] = [
    (GOLD, (2010, 4, 11), (12, 30)),
    (SILVER, (2010, 4, 11), (12, 25)),
    (COPPER, (2011, 1, 23), (12, 0)),
    (PLATINUM, (2017, 5, 21), (12, 5)),
    (PALLADIUM, (2018, 11, 18), (12, 0)),
];

/// A probe instant stated in the venue's own wall clock.
fn ct(date: Ymd, time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous Central instant")
        .with_timezone(&Utc)
}

fn day(date: Ymd) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.0, date.1, date.2).expect("fixture must be a valid date")
}

fn open_at(key: MarketHoursKey, instant: DateTime<Utc>) -> bool {
    hours_for_market_hours_key(key, instant).is_open(instant)
}

fn state_at(key: MarketHoursKey, instant: DateTime<Utc>) -> SessionState {
    hours_for_market_hours_key(key, instant).session_state(instant)
}

/// The same question through the date-aware surface, which reselects the
/// profile for every venue-local opening day rather than once.
fn calendar_state_at(key: MarketHoursKey, instant: DateTime<Utc>) -> SessionState {
    calendar_for_market_hours_key(key).session_state(instant)
}

/// The day before each launch is sessionless, the launch evening opens at
/// 17:00 CT, and the boundary holds at venue-local midnight on both selectors.
///
/// The pre-launch era is a sourced closure rather than an unworked gap: CME's
/// own complete TAS eligibility lists — RA0907-4's eleven Globex codes and
/// RA1001-4/RA1002-4's reprint — contain no metals at all, so nothing is
/// carried back to the January-2010 floor.
#[test]
fn each_key_is_closed_before_its_launch_and_opens_on_the_launch_evening() {
    for (key, launch, _) in KEYS {
        let week_before = (launch.0, launch.1, launch.2 - 7);
        assert!(
            !open_at(key, ct(week_before, (17, 0, 0))),
            "{key:?}: the Sunday a week before the launch is sessionless"
        );
        assert_eq!(
            state_at(key, ct(week_before, (16, 30, 0))),
            SessionState::Closed,
            "{key:?}: the pre-launch era has no queue either"
        );
        assert!(
            !open_at(key, ct(launch, (16, 59, 59))),
            "{key:?}: the instant before the published open"
        );
        assert!(
            open_at(key, ct(launch, (17, 0, 0))),
            "{key:?}: the launch evening opens at 17:00 CT"
        );

        // Both sides of the revision at venue-local midnight, on the instant
        // selector: the profile in force late on the preceding local day has no
        // rules at all, and the one in force from local midnight carries the
        // wrap to the following trade date.
        let first_trade_date_probe = ct((launch.0, launch.1, launch.2 + 1), (9, 0, 0));
        let eve = (launch.0, launch.1, launch.2 - 1);
        let before = hours_for_market_hours_key(key, ct(eve, (23, 59, 59)));
        let after = hours_for_market_hours_key(key, ct(launch, (0, 0, 0)));
        assert!(before.regular.is_empty() && before.extended.is_empty());
        assert!(!before.is_open(first_trade_date_probe), "{key:?}");
        assert!(after.is_open(first_trade_date_probe), "{key:?}");

        // And through the date-aware surface, which reselects per opening day.
        assert!(
            !calendar_for_market_hours_key(key).is_open(ct(week_before, (20, 0, 0))),
            "{key:?}: the calendar is closed the week before the launch"
        );
        assert!(
            calendar_for_market_hours_key(key).is_open(first_trade_date_probe),
            "{key:?}: the calendar opens the first trade date"
        );
    }
}

/// No regular session in any era, the whole matching window extended, and the
/// Pre-Open an order-entry phase in which no trade can print.
#[test]
fn the_whole_window_is_extended_and_the_pre_open_is_order_entry() {
    // Monday 2026-09-14, a plain week with no holiday and no DST transition.
    for (key, _, _) in KEYS {
        let profile = session_profile(key);
        assert!(
            profile.regular.is_empty(),
            "{key:?}: the TAS shape has no sourced regular session"
        );
        assert!(!profile.order_entry.is_empty(), "{key:?}");
        assert!(
            profile.has_daily_close && profile.has_weekend_close,
            "{key:?}"
        );

        let hours = hours_for_market_hours_key(key, ct((2026, 9, 14), (9, 0, 0)));
        assert!(
            !hours.is_open_regular(ct((2026, 9, 14), (9, 0, 0))),
            "{key:?}"
        );
        assert!(
            hours.is_open_extended(ct((2026, 9, 14), (9, 0, 0))),
            "{key:?}"
        );
        assert_eq!(
            state_at(key, ct((2026, 9, 14), (9, 0, 0))),
            SessionState::OpenExtended,
            "{key:?}: the matching leg is extended"
        );
        assert_eq!(
            state_at(key, ct((2026, 9, 14), (16, 50, 0))),
            SessionState::OrderEntry,
            "{key:?}: the Monday-Thursday queue opens 16:45 CT"
        );
        assert!(
            !open_at(key, ct((2026, 9, 14), (16, 50, 0))),
            "{key:?}: no trade prints in the queue"
        );
    }
}

/// The close is end-exclusive, and the gap from it to the next 17:00 CT open
/// exceeds four hours on every key, so it is `Closed` and never `Maintenance`.
#[test]
fn the_close_is_end_exclusive_and_the_daily_gap_is_closed_not_maintenance() {
    for (key, _, (hour, minute)) in KEYS {
        // A summer week and a winter week, so a DST slip cannot pass.
        for tuesday in [(2026, 9, 15), (2027, 1, 12)] {
            assert!(
                open_at(
                    key,
                    ct(tuesday, (hour, minute, 0)) - chrono::Duration::seconds(1)
                ),
                "{key:?} {tuesday:?}: one second before the close"
            );
            assert!(
                !open_at(key, ct(tuesday, (hour, minute, 0))),
                "{key:?} {tuesday:?}: the close is end-exclusive"
            );
            assert_eq!(
                state_at(key, ct(tuesday, (14, 0, 0))),
                SessionState::Closed,
                "{key:?} {tuesday:?}: the inter-trade-date gap exceeds four hours"
            );
            assert!(
                !hours_for_market_hours_key(key, ct(tuesday, (14, 0, 0)))
                    .is_maintenance(ct(tuesday, (14, 0, 0))),
                "{key:?} {tuesday:?}: and so is never maintenance"
            );
            assert_eq!(
                calendar_state_at(key, ct(tuesday, (14, 0, 0))),
                SessionState::Closed,
                "{key:?} {tuesday:?}: the date-aware surface agrees"
            );
        }
    }
}

/// There is no Friday-evening reopen, and the weekend is one closed block from
/// the Friday close to the Sunday queue.
#[test]
fn there_is_no_friday_evening_reopen_and_the_weekend_is_closed() {
    for (key, _, (hour, minute)) in KEYS {
        let friday = (2026, 9, 18);
        let sunday = (2026, 9, 20);
        assert!(
            open_at(
                key,
                ct(friday, (hour, minute, 0)) - chrono::Duration::seconds(1)
            ),
            "{key:?}: Friday trades to its close"
        );
        for probe in [
            ct(friday, (hour, minute, 0)),
            ct(friday, (17, 0, 0)),
            ct(friday, (20, 0, 0)),
            ct((2026, 9, 19), (12, 0, 0)),
            ct(sunday, (12, 0, 0)),
            ct(sunday, (16, 59, 59)),
        ] {
            assert!(
                !open_at(key, probe),
                "{key:?}: {probe} is inside the weekend"
            );
        }
        assert_eq!(
            state_at(key, ct(friday, (17, 0, 0))),
            SessionState::Closed,
            "{key:?}: no Friday-evening reopen"
        );
        assert!(
            open_at(key, ct(sunday, (17, 0, 0))),
            "{key:?}: the week restarts Sunday 17:00 CT"
        );

        let calendar = calendar_for_market_hours_key(key);
        assert_eq!(
            calendar.next_session_after(ct(friday, (hour, minute, 0))),
            Some((ct(sunday, (17, 0, 0)), ct((2026, 9, 21), (hour, minute, 0)))),
            "{key:?}: the next session after the weekly close"
        );
        assert_eq!(
            calendar.trade_date(ct(sunday, (20, 0, 0))),
            Some(day((2026, 9, 21))),
            "{key:?}: the Sunday evening leg belongs to Monday"
        );
        assert_eq!(
            calendar.session_bounds_with(ct(sunday, (20, 0, 0)), SessionKind::Extended),
            Some((ct(sunday, (17, 0, 0)), ct((2026, 9, 21), (hour, minute, 0)))),
            "{key:?}: the wrap is one session"
        );
    }
}

/// RA1104-4's per-product stagger, effective Sunday 2011-04-10 for trade date
/// Monday 2011-04-11, moves the queue onset and nothing else.
///
/// Both sides at venue-local midnight, on both selectors, and the executable
/// boundaries are asserted unchanged across the row.
#[test]
fn the_2011_stagger_moves_only_the_queue_onset() {
    for (key, sunday_minute, weekday_minute, close) in [
        (GOLD, 18_u32, 48_u32, (12_u32, 30_u32)),
        (SILVER, 19, 49, (12, 25)),
        (COPPER, 20, 50, (12, 0)),
    ] {
        // The Sunday before the row: the shared 16:15 onset is live.
        assert_eq!(
            state_at(key, ct((2011, 4, 3), (16, 16, 0))),
            SessionState::OrderEntry,
            "{key:?}: 16:16 CT queues under the pre-stagger profile"
        );
        // The row's own Sunday: the queue now starts one to two minutes later,
        // so 16:16 is closed and the staggered minute queues.
        assert_eq!(
            state_at(key, ct((2011, 4, 10), (16, 16, 0))),
            SessionState::Closed,
            "{key:?}: the stagger takes effect on its own opening day"
        );
        assert_eq!(
            state_at(key, ct((2011, 4, 10), (16, sunday_minute, 0))),
            SessionState::OrderEntry,
            "{key:?}: RA1104-4's Sunday onset"
        );
        assert_eq!(
            calendar_state_at(key, ct((2011, 4, 10), (16, sunday_minute, 0))),
            SessionState::OrderEntry,
            "{key:?}: and through the date-aware surface"
        );
        assert_eq!(
            calendar_state_at(key, ct((2011, 4, 3), (16, 16, 0))),
            SessionState::OrderEntry,
            "{key:?}: which still reselects the old profile the week before"
        );
        // The weekday onset moves the same way.
        assert_eq!(
            state_at(key, ct((2011, 4, 11), (16, 46, 0))),
            SessionState::Closed,
            "{key:?}: 16:46 CT no longer queues"
        );
        assert_eq!(
            state_at(key, ct((2011, 4, 11), (16, weekday_minute, 0))),
            SessionState::OrderEntry,
            "{key:?}: RA1104-4's Monday-Thursday onset"
        );
        // And the executable window does not move: the same table leaves the
        // close column unchanged.
        for probe_day in [(2011, 4, 4), (2011, 4, 11)] {
            assert!(
                open_at(
                    key,
                    ct(probe_day, (close.0, close.1, 0)) - chrono::Duration::seconds(1)
                ),
                "{key:?} {probe_day:?}: close unchanged"
            );
            assert!(!open_at(key, ct(probe_day, (close.0, close.1, 0))));
            assert!(open_at(key, ct(probe_day, (9, 0, 0))));
        }
    }
}

/// CME Globex notice 20120409, effective Sunday 2012-04-15 for trade date
/// Monday 2012-04-16, ends the stagger and restores the shared 16:15 / 16:45 CT
/// onsets as one randomised minute per group.
#[test]
fn the_2012_notice_restores_the_shared_queue_onsets() {
    for (key, sunday_minute, weekday_minute) in
        [(GOLD, 18_u32, 48_u32), (SILVER, 19, 49), (COPPER, 20, 50)]
    {
        // The Sunday before the row is still staggered.
        assert_eq!(
            state_at(key, ct((2012, 4, 8), (16, 16, 0))),
            SessionState::Closed,
            "{key:?}: the stagger is still live the week before"
        );
        assert_eq!(
            state_at(key, ct((2012, 4, 8), (16, sunday_minute, 0))),
            SessionState::OrderEntry,
            "{key:?}"
        );
        // The row's own Sunday restores 16:15 exactly.
        assert_eq!(
            state_at(key, ct((2012, 4, 15), (16, 14, 0))),
            SessionState::Closed,
            "{key:?}: the minute before the restored Sunday onset"
        );
        assert_eq!(
            state_at(key, ct((2012, 4, 15), (16, 15, 0))),
            SessionState::OrderEntry,
            "{key:?}: the notice takes effect on its own opening day"
        );
        assert_eq!(
            calendar_state_at(key, ct((2012, 4, 15), (16, 16, 0))),
            SessionState::OrderEntry,
            "{key:?}: and through the date-aware surface"
        );
        assert_eq!(
            calendar_state_at(key, ct((2012, 4, 8), (16, 16, 0))),
            SessionState::Closed,
            "{key:?}: which still reselects the stagger the week before"
        );
        // And the weekday onset returns to 16:45 exactly.
        assert_eq!(
            state_at(key, ct((2012, 4, 16), (16, 44, 0))),
            SessionState::Closed,
            "{key:?}: the minute before the restored weekday onset"
        );
        assert_eq!(
            state_at(key, ct((2012, 4, 16), (16, 45, 0))),
            SessionState::OrderEntry,
            "{key:?}: 16:45 CT queues again"
        );
        assert_eq!(
            state_at(key, ct((2012, 4, 9), (16, 46, 0))),
            SessionState::Closed,
            "{key:?}: it did not, one week earlier"
        );
        assert_eq!(
            state_at(key, ct((2012, 4, 9), (16, weekday_minute, 0))),
            SessionState::OrderEntry,
            "{key:?}: the staggered weekday onset was still live"
        );
    }
}

/// The COMEX three withhold the Sunday 16:00-16:15 CT quarter-hour that CME's
/// undated 2012 move leaves in dispute (issue #79); the platinum-group keys,
/// which launched after that move, serve 16:00-17:00 CT with nothing withheld.
#[test]
fn the_sunday_quarter_hour_is_withheld_on_comex_and_served_on_the_pgm_keys() {
    // Sunday 2026-09-20 and Sunday 2027-01-10: a summer and a winter probe.
    for sunday in [(2026, 9, 20), (2027, 1, 10)] {
        for key in [GOLD, SILVER, COPPER] {
            assert_eq!(
                state_at(key, ct(sunday, (16, 5, 0))),
                SessionState::Closed,
                "{key:?} {sunday:?}: 16:00-16:15 CT is withheld"
            );
            assert_eq!(
                state_at(key, ct(sunday, (16, 15, 0))),
                SessionState::OrderEntry,
                "{key:?} {sunday:?}: the served intersection starts at 16:15 CT"
            );
        }
        for key in [PLATINUM, PALLADIUM] {
            assert_eq!(
                state_at(key, ct(sunday, (16, 5, 0))),
                SessionState::OrderEntry,
                "{key:?} {sunday:?}: nothing is withheld after the 2012 move"
            );
            assert_eq!(
                state_at(key, ct(sunday, (15, 59, 59))),
                SessionState::Closed,
                "{key:?} {sunday:?}: and 16:00 CT is still the onset"
            );
        }
    }
}

/// Copper TAS and palladium TAS share a 12:00 CT close and nothing else.
///
/// The separation the module comments name: different exchange (COMEX against
/// NYMEX), different security group (HT against PX), launches seven years
/// apart, and a Sunday queue that differs because only one of them predates
/// CME's undated 2012 move.
#[test]
fn copper_and_palladium_are_not_interchangeable_on_their_shared_close() {
    // Seven years in which copper trades and palladium does not exist.
    let monday_2012 = ct((2012, 3, 5), (9, 0, 0));
    assert!(open_at(COPPER, monday_2012), "copper TAS listed 2011-01-23");
    assert!(
        !open_at(PALLADIUM, monday_2012),
        "palladium TAS does not list until 2018-11-18"
    );

    // And after both exist, their Sunday queues still disagree.
    let sunday_quarter_hour = ct((2019, 6, 9), (16, 5, 0));
    assert_eq!(
        state_at(COPPER, sunday_quarter_hour),
        SessionState::Closed,
        "copper withholds the disputed quarter-hour"
    );
    assert_eq!(
        state_at(PALLADIUM, sunday_quarter_hour),
        SessionState::OrderEntry,
        "palladium serves it"
    );

    // Platinum is not copper either: a five-minute later close.
    assert!(open_at(PLATINUM, ct((2019, 6, 11), (12, 2, 0))));
    assert!(!open_at(COPPER, ct((2019, 6, 11), (12, 2, 0))));
}

/// The TAS books are not the metals outrights, and CME's 2015-09-20 DCM-wide
/// close move is the instant that proves it.
///
/// Globex notice 20150817 moved the outrights' close from 16:15 to 16:00 CT for
/// COMEX and NYMEX; a 12:30 CT TAS book cannot be reached by a move to 16:00,
/// and the last statement before the observation gap (2015-03-22) and the first
/// after it (2019-11-18) are the same values.
#[test]
fn the_tas_keys_are_not_the_metals_outright_key() {
    // Monday 2015-10-12, three weeks after the outrights moved.
    let afternoon = ct((2015, 10, 12), (14, 0, 0));
    assert!(
        open_at(OUTRIGHTS, afternoon),
        "the COMEX metals outrights trade all afternoon"
    );
    for (key, _, _) in KEYS.into_iter().take(3) {
        assert!(
            !open_at(key, afternoon),
            "{key:?}: the TAS book closed at midday"
        );
    }
    // And the TAS close is the same value on either side of that move.
    for probe_day in [(2015, 3, 17), (2019, 11, 19)] {
        assert!(
            open_at(GOLD, ct(probe_day, (12, 29, 59))),
            "{probe_day:?}: 12:29:59 CT trades"
        );
        assert!(
            !open_at(GOLD, ct(probe_day, (12, 30, 0))),
            "{probe_day:?}: 12:30 CT closes"
        );
    }
}

#[test]
fn the_metals_tas_keys_round_trip_through_their_canonical_names() {
    for (key, name) in [
        (GOLD, "globex_gold_tas"),
        (SILVER, "globex_silver_tas"),
        (COPPER, "globex_copper_tas"),
        (PLATINUM, "globex_platinum_tas"),
        (PALLADIUM, "globex_palladium_tas"),
    ] {
        assert_eq!(key.as_str(), name);
        assert_eq!(name.parse::<MarketHoursKey>(), Ok(key));
        let json = serde_json::to_string(&key).expect("serializes");
        assert_eq!(json, format!("\"{name}\""));
        assert_eq!(
            serde_json::from_str::<MarketHoursKey>(&json).expect("deserializes"),
            key
        );
    }
}
