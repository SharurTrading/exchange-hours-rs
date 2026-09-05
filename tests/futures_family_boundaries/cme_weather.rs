// SPDX-License-Identifier: MIT-0

//! CME weather temperature-index futures (`globex_weather`): the published
//! grid, the single dated revision of 2025-04-13, the queue carry-back, and
//! the separation from the two families whose current envelope it shares.
//!
//! Every probe here is stated in America/Chicago wall-clock and converted, so
//! a DST slip in either direction fails rather than passing on a coincidence.
//! 2026-09-13 is a Sunday and 2026-09-18 the Friday of the same week — a week
//! after this key's 2026-09-05 knowledge-bound row, so the dated selector and
//! the fixed snapshot answer identically there. 2025-04-13 is the Sunday on
//! which CME SER-9519 takes effect, for trade date Monday 2025-04-14.

use chrono::{DateTime, NaiveDate, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, DayOverride, MarketHoursKey, SessionState, StaticDayPolicy,
    calendar_for_market_hours_key, hours_for_market_hours_key, session_profile,
};

const WEATHER: MarketHoursKey = MarketHoursKey::GlobexWeather;
const FX: MarketHoursKey = MarketHoursKey::GlobexFx;
const ENERGY: MarketHoursKey = MarketHoursKey::GlobexEnergy;

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

fn hours_at(date: (i32, u32, u32)) -> exchange_hours::MarketHours {
    hours_for_market_hours_key(WEATHER, ct(date, (0, 0, 0)))
}

fn state_at(instant: DateTime<Utc>) -> SessionState {
    calendar_for_market_hours_key(WEATHER).session_state(instant)
}

/// The published current grid, boundary by boundary. CME's contract
/// specification gives "CME Globex: Sunday - Friday 5:00 p.m. - 4:00 p.m. CT
/// with a 60-minute break each day beginning at 4:00 p.m. CT", and SER-9519's
/// unchanged Pre-Open column gives Sunday 16:00-17:00 and Monday-Thursday
/// 16:45-17:00 CT. CME's own session-event service returns exactly those
/// `preopen`/`open`/`closed` events for security group `HW`.
///
/// Each open is fenced by the second before it, and each close is asserted
/// end-exclusive.
#[test]
fn weather_serves_the_published_grid_with_end_exclusive_closes() {
    let hours = hours_for_market_hours_key(WEATHER, ct((2026, 9, 14), (12, 0, 0)));

    // Sunday queue, then the electronic open one second later than the last
    // instant that accepts nothing.
    assert!(!hours.is_accepting_orders(ct((2026, 9, 13), (15, 59, 59))));
    assert!(hours.is_order_entry_only(ct((2026, 9, 13), (16, 0, 0))));
    assert!(hours.is_order_entry_only(ct((2026, 9, 13), (16, 59, 59))));
    assert!(!hours.is_open(ct((2026, 9, 13), (16, 59, 59))));
    assert!(hours.is_open(ct((2026, 9, 13), (17, 0, 0))));

    // Everything executable here is extended: weather futures have never been
    // pit-eligible, so `regular` is empty in every era.
    assert_eq!(
        state_at(ct((2026, 9, 13), (17, 0, 0))),
        SessionState::OpenExtended,
    );
    assert!(!hours.is_open_regular(ct((2026, 9, 14), (12, 0, 0))));
    assert!(session_profile(WEATHER).regular.is_empty());

    // The leg wraps local midnight and closes 16:00 end-exclusive.
    assert!(hours.is_open(ct((2026, 9, 14), (3, 0, 0))));
    assert!(hours.is_open(ct((2026, 9, 14), (15, 59, 59))));
    assert!(!hours.is_open(ct((2026, 9, 14), (16, 0, 0))));

    // The 60-minute daily break: nothing matches and nothing queues until
    // 16:45, and the operator reopens at 17:00.
    assert_eq!(
        state_at(ct((2026, 9, 14), (16, 30, 0))),
        SessionState::Maintenance,
        "the one-hour halt separates two trade dates and stays inside the \
         four-hour maintenance ceiling"
    );
    assert!(!hours.is_accepting_orders(ct((2026, 9, 14), (16, 44, 59))));
    assert!(hours.is_order_entry_only(ct((2026, 9, 14), (16, 45, 0))));
    assert_eq!(
        state_at(ct((2026, 9, 14), (16, 45, 0))),
        SessionState::OrderEntry,
    );
    assert!(hours.is_open(ct((2026, 9, 14), (17, 0, 0))));

    // The fixed snapshot must say the same thing as the dated selector, and it
    // is the table that carries the current Sunday 16:00 queue.
    let profile = session_profile(WEATHER);
    assert!(profile.is_order_entry_only(ct((2026, 9, 13), (16, 0, 0))));
    assert!(profile.is_open(ct((2026, 9, 13), (17, 0, 0))));
    assert!(profile.is_open(ct((2026, 9, 14), (15, 59, 59))));
    assert!(!profile.is_open(ct((2026, 9, 14), (16, 0, 0))));
    assert!(profile.is_order_entry_only(ct((2026, 9, 14), (16, 45, 0))));
    assert!(profile.regular.is_empty());
}

/// The wrapping leg takes the trade date of its close, so the Sunday-evening
/// session belongs to Monday and one daily bar runs Sunday 17:00 CT to Monday
/// 16:00 CT. The queues carry the trade date of the session they feed.
#[test]
fn the_wrapping_leg_takes_its_closing_trade_date() {
    let calendar = calendar_for_market_hours_key(WEATHER);

    assert_eq!(
        calendar.trade_date(ct((2026, 9, 13), (20, 0, 0))),
        Some(day((2026, 9, 14))),
        "the Sunday-evening leg wraps, so it belongs to Monday's trade date"
    );
    assert_eq!(
        calendar.trade_date(ct((2026, 9, 14), (12, 0, 0))),
        Some(day((2026, 9, 14))),
    );
    assert_eq!(
        calendar.trade_date(ct((2026, 9, 14), (16, 45, 0))),
        Some(day((2026, 9, 15))),
        "Monday's evening queue feeds the session that closes Tuesday"
    );
    assert_eq!(
        calendar.candle_end(ct((2026, 9, 13), (20, 0, 0)), CalendarResolution::Daily),
        Some(ct((2026, 9, 14), (16, 0, 0))),
    );
    assert_eq!(
        calendar.candle_end(ct((2026, 9, 14), (12, 0, 0)), CalendarResolution::Daily),
        Some(ct((2026, 9, 14), (16, 0, 0))),
    );
    assert_eq!(
        calendar.session_bounds(ct((2026, 9, 14), (12, 0, 0))),
        Some((ct((2026, 9, 13), (17, 0, 0)), ct((2026, 9, 14), (16, 0, 0)))),
    );
}

/// The week ends at the Friday 16:00 CT close: there is no Friday-evening
/// reopen and no Friday-evening queue, and the next open is Sunday 17:00 CT.
/// CME's session-event service shows the same shape — Friday carries a
/// `closed` event at 16:00 and nothing after it.
#[test]
fn the_week_reopens_on_sunday_evening_with_no_friday_evening_leg() {
    let calendar = calendar_for_market_hours_key(WEATHER);
    let hours = hours_for_market_hours_key(WEATHER, ct((2026, 9, 18), (12, 0, 0)));

    assert!(hours.is_open(ct((2026, 9, 18), (15, 59, 59))));
    assert!(!hours.is_open(ct((2026, 9, 18), (16, 0, 0))));
    for (date, time) in [
        ((2026, 9, 18), (16, 45, 0)),
        ((2026, 9, 18), (17, 0, 0)),
        ((2026, 9, 19), (12, 0, 0)),
        ((2026, 9, 20), (12, 0, 0)),
    ] {
        let instant = ct(date, time);
        assert!(
            !hours.is_accepting_orders(instant),
            "{instant}: the weekend break admits neither trading nor queueing"
        );
    }
    assert_eq!(
        state_at(ct((2026, 9, 19), (12, 0, 0))),
        SessionState::Closed,
        "the weekend gap outruns the four-hour maintenance ceiling"
    );
    assert_eq!(
        calendar.next_session_open_after(ct((2026, 9, 18), (16, 0, 0))),
        Some(ct((2026, 9, 20), (17, 0, 0))),
        "the week reopens on Sunday evening, not Friday evening"
    );
}

/// 2025-04-13 is the family's only dated revision. CME SER-9519: "Effective
/// Sunday, April 13, 2025, for trade date Monday, April 14, 2025 ... will
/// expand the CME Globex electronic trading ... hours of all weather futures
/// and options on weather contracts", moving "Sunday 5:00 p.m. - Friday 3:15
/// p.m. / Daily trading halts 3:15 p.m. - 5:00 p.m." to "Sunday 5:00 p.m. -
/// Friday 4:00 p.m. / Daily trading halts 4:00 p.m. - 5:00 p.m.".
///
/// The row is keyed to the venue-local **opening** day, because the first
/// session that behaves differently is the one that opens Sunday 2025-04-13 at
/// 17:00 CT and closes on the stated Monday trade date. Both sides are
/// evaluated at venue-local midnight, so a revision keyed one day early or
/// late flips an assertion.
#[test]
fn the_globex_close_moves_from_1515_to_1600_on_2025_04_13() {
    let earlier = hours_at((2025, 4, 12));
    let revised = hours_at((2025, 4, 13));

    for (label, hours) in [("earlier", &earlier), ("revised", &revised)] {
        assert!(
            hours.is_open(ct((2025, 4, 13), (17, 0, 0))),
            "{label}: SER-9519 leaves the 17:00 CT Sunday open unchanged"
        );
        assert!(
            hours.is_open(ct((2025, 4, 14), (15, 0, 0))),
            "{label}: both grids are still matching at 15:00 CT"
        );
    }

    // The outgoing close, end-exclusive at 15:15 CT.
    assert!(earlier.is_open(ct((2025, 4, 14), (15, 14, 59))));
    assert!(
        !earlier.is_open(ct((2025, 4, 14), (15, 15, 0))),
        "through 2025-04-12 the daily close is 15:15 CT"
    );
    // The incoming close: the 45 minutes SER-9519 added, then 16:00
    // end-exclusive.
    assert!(
        revised.is_open(ct((2025, 4, 14), (15, 30, 0))),
        "from 2025-04-13 the expanded leg runs through 15:30 CT"
    );
    assert!(revised.is_open(ct((2025, 4, 14), (15, 59, 59))));
    assert!(
        !revised.is_open(ct((2025, 4, 14), (16, 0, 0))),
        "the expanded close is end-exclusive at 16:00 CT"
    );

    // The daily halt moves with the close: 15:15-17:00 CT before, 16:00-17:00
    // CT after, so 16:30 is halted on both sides while 15:30 is not.
    assert!(!earlier.is_open(ct((2025, 4, 14), (16, 30, 0))));
    assert!(!revised.is_open(ct((2025, 4, 14), (16, 30, 0))));

    // "the CME Globex Pre-Open hours ... shall remain unchanged": the queues
    // are identical on both sides of the cutover.
    for (label, hours) in [("earlier", &earlier), ("revised", &revised)] {
        assert!(
            hours.is_order_entry_only(ct((2025, 4, 13), (16, 30, 0))),
            "{label}: the Sunday queue is unchanged by SER-9519"
        );
        assert!(
            hours.is_order_entry_only(ct((2025, 4, 14), (16, 45, 0))),
            "{label}: the Monday-Thursday 16:45 queue is unchanged by SER-9519"
        );
    }
}

/// The queue record is a sourced intersection, not a dated cutover. CME's
/// weather trading-hours pages publish a Sunday Pre-Open of 16:15 on the
/// captures of 2012-05-03..2012-05-28 and 16:00 from 2012-06-07 onward, with
/// no notice naming the day, so the dated profiles carry 16:15-17:00 — the
/// window that holds under both states — from the January-2010 floor, and the
/// disputed 16:00-16:15 quarter-hour arrives only at the 2026-09-05
/// knowledge-bound row. The Monday-Thursday 16:45 queue is undisputed.
#[test]
fn the_sunday_queue_carries_its_sourced_intersection_until_the_review_row() {
    for date in [(2010, 6, 14), (2015, 6, 15), (2026, 9, 4)] {
        let hours = hours_at(date);
        let (year, month, day_of_month) = date;
        // The nearest Sunday at or after each probe date is not needed: the
        // rule set is a normal week, so any Sunday answers for the era.
        let sunday = ct((2026, 9, 13), (16, 0, 0));
        let sunday_1620 = ct((2026, 9, 13), (16, 20, 0));
        assert!(
            !hours.is_accepting_orders(sunday),
            "{year}-{month:02}-{day_of_month:02}: the dated era queues from 16:15, not 16:00"
        );
        assert!(
            hours.is_accepting_orders(sunday_1620),
            "{year}-{month:02}-{day_of_month:02}: 16:15-17:00 is the sourced intersection"
        );
        assert!(
            hours.is_accepting_orders(ct((2026, 9, 14), (16, 45, 0))),
            "{year}-{month:02}-{day_of_month:02}: the weekday 16:45 queue is undisputed"
        );
    }

    // From the knowledge-bound row the verified-current queue applies.
    let reviewed = hours_at((2026, 9, 5));
    assert!(
        reviewed.is_accepting_orders(ct((2026, 9, 13), (16, 0, 0))),
        "the 2026-09-05 review row adds the sourced 16:00-16:15 quarter-hour"
    );
    assert!(
        session_profile(WEATHER).is_accepting_orders(ct((2026, 9, 13), (16, 0, 0))),
        "the fixed-current table carries the queue CME publishes today"
    );
}

/// Envelope match is not family identity, and weather is the clearest case in
/// the crate: its **current** envelope is byte-for-byte `globex_fx` and
/// `globex_energy`, and its history matches neither. FX has run 17:00-16:00 CT
/// since the January-2010 floor; energy closed 16:15 CT until 2015-09-20 and
/// 16:00 CT after; weather closed 15:15 CT until 2025-04-13. Three keys, one
/// envelope, three histories.
#[test]
fn envelope_match_is_not_family_identity() {
    // 2015-06-15 is a Monday on which all three grids differ. Weather has
    // already closed at 15:15, FX closes 16:00, energy 16:15.
    let probe = ct((2015, 6, 15), (12, 0, 0));
    let weather = hours_for_market_hours_key(WEATHER, probe);
    let fx = hours_for_market_hours_key(FX, probe);
    let energy = hours_for_market_hours_key(ENERGY, probe);

    let at_1530 = ct((2015, 6, 15), (15, 30, 0));
    assert!(!weather.is_open(at_1530), "weather closed at 15:15 CT");
    assert!(fx.is_open(at_1530));
    assert!(energy.is_open(at_1530));

    let at_1605 = ct((2015, 6, 15), (16, 5, 0));
    assert!(!weather.is_open(at_1605));
    assert!(!fx.is_open(at_1605), "FX closed at 16:00 CT");
    assert!(
        energy.is_open(at_1605),
        "energy still closed at 16:15 CT in 2015"
    );

    // Weather sat out the 2015-09-21 NYMEX/COMEX move entirely: every capture
    // of CME's weather specification after that day still read 3:15 p.m., from
    // 2016-12-01 through 2022-06-30. Reusing `globex_energy` would fabricate a
    // cutover weather never had.
    for date in [(2015, 9, 18), (2015, 9, 22), (2016, 12, 1)] {
        let instant = ct(date, (15, 30, 0));
        assert!(
            !hours_for_market_hours_key(WEATHER, instant).is_open(instant),
            "{instant}: weather still closes 15:15 CT on both sides of the energy move"
        );
        assert!(
            hours_for_market_hours_key(ENERGY, instant).is_open(instant),
            "{instant}: the energy family is matching at 15:30 CT"
        );
    }

    // After 2025-04-13 the three envelopes coincide, which is exactly what
    // makes the separate key necessary rather than redundant: a full week of
    // probes taken after this key's review row must agree on every surface.
    for day_offset in 0..7_i64 {
        for (hour, minute) in [
            (3, 0),
            (12, 0),
            (15, 30),
            (16, 5),
            (16, 30),
            (16, 50),
            (20, 0),
        ] {
            let instant = ct((2026, 9, 13), (0, 0, 0))
                + chrono::Duration::days(day_offset)
                + chrono::Duration::hours(hour)
                + chrono::Duration::minutes(minute);
            let weather = hours_for_market_hours_key(WEATHER, instant);
            for (label, other) in [
                ("globex_fx", hours_for_market_hours_key(FX, instant)),
                ("globex_energy", hours_for_market_hours_key(ENERGY, instant)),
            ] {
                assert_eq!(
                    weather.is_open(instant),
                    other.is_open(instant),
                    "{instant}: converged envelopes must agree with {label} on is_open"
                );
                assert_eq!(
                    weather.is_accepting_orders(instant),
                    other.is_accepting_orders(instant),
                    "{instant}: converged envelopes must agree with {label} on order acceptance"
                );
            }
        }
    }
}

/// The leg is quoted in Central wall-clock, so it survives both DST
/// transitions with its endpoints intact and its elapsed duration changing
/// underneath.
#[test]
fn the_wrapping_leg_survives_both_dst_transitions() {
    // Spring forward: 2026-03-08 is the second Sunday of March.
    let spring = hours_for_market_hours_key(WEATHER, ct((2026, 3, 9), (12, 0, 0)));
    assert!(spring.is_open(ct((2026, 3, 8), (17, 0, 0))));
    assert!(spring.is_open(ct((2026, 3, 9), (1, 30, 0))));
    assert!(spring.is_open(ct((2026, 3, 9), (3, 0, 0))));
    assert!(spring.is_open(ct((2026, 3, 9), (15, 59, 59))));
    assert!(!spring.is_open(ct((2026, 3, 9), (16, 0, 0))));

    // Fall back: 2026-11-01 is the first Sunday of November.
    let fall = hours_for_market_hours_key(WEATHER, ct((2026, 11, 2), (12, 0, 0)));
    assert!(fall.is_open(ct((2026, 11, 1), (17, 0, 0))));
    assert!(fall.is_open(ct((2026, 11, 2), (2, 30, 0))));
    assert!(fall.is_open(ct((2026, 11, 2), (15, 59, 59))));
    assert!(!fall.is_open(ct((2026, 11, 2), (16, 0, 0))));
}

/// The caller-owned day overlay applies to weather like any other family: a
/// closed trade date removes the whole trading day including the prior evening
/// leg that belongs to it, and an early final close clips the wrapping session
/// without touching the queue that precedes it.
#[test]
fn day_policy_overlays_a_closed_date_and_an_early_close() {
    let closed_tuesday = day((2026, 9, 15));
    let early_wednesday = day((2026, 9, 16));
    let records = [
        DayOverride::closed(closed_tuesday),
        DayOverride::early_close(early_wednesday, 12 * 3_600),
    ];
    let policy = StaticDayPolicy::new(&records).expect("the fixture records must be valid");
    let calendar = calendar_for_market_hours_key(WEATHER).with_day_policy(&policy);
    let plain = calendar_for_market_hours_key(WEATHER);

    // Tuesday's whole trading day goes, including Monday's evening leg.
    for (date, time) in [((2026, 9, 14), (20, 0, 0)), ((2026, 9, 15), (12, 0, 0))] {
        let instant = ct(date, time);
        assert!(
            plain.is_open(instant),
            "{instant}: open on the normal-week grid"
        );
        assert!(
            !calendar.is_open(instant),
            "{instant}: a closed Tuesday removes the trade date it belongs to"
        );
    }
    // Tuesday's own evening leg belongs to Wednesday and survives.
    assert!(calendar.is_open(ct((2026, 9, 15), (20, 0, 0))));

    // Wednesday's session ends at the overridden 12:00 CT.
    assert!(calendar.is_open(ct((2026, 9, 16), (11, 59, 59))));
    assert!(!calendar.is_open(ct((2026, 9, 16), (12, 0, 0))));
    assert!(
        plain.is_open(ct((2026, 9, 16), (12, 0, 0))),
        "the override, not the profile, is what closed Wednesday early"
    );
    assert_eq!(
        calendar.session_bounds(ct((2026, 9, 16), (10, 0, 0))),
        Some((ct((2026, 9, 15), (17, 0, 0)), ct((2026, 9, 16), (12, 0, 0)))),
    );
}

/// The wire identity round-trips through every public spelling.
#[test]
fn weather_key_round_trips_through_its_canonical_name() {
    assert_eq!(WEATHER.as_str(), "globex_weather");
    assert_eq!(WEATHER.to_string(), "globex_weather");
    assert_eq!("globex_weather".parse::<MarketHoursKey>(), Ok(WEATHER));
    assert_eq!(
        serde_json::to_string(&WEATHER).expect("key serializes"),
        "\"globex_weather\""
    );
    assert_eq!(
        serde_json::from_str::<MarketHoursKey>("\"globex_weather\"").expect("key deserializes"),
        WEATHER
    );
    assert!(
        "globex_weather_options".parse::<MarketHoursKey>().is_err(),
        "weather options are not a shipped family and must not resolve"
    );
    assert!(
        "weather".parse::<MarketHoursKey>().is_err(),
        "a near-miss name must be rejected, never mapped to the nearest family"
    );
}
