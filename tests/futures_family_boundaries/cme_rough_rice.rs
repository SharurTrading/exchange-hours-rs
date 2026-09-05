// SPDX-License-Identifier: MIT-0

//! CBOT Rough Rice (`ZR`/`OZR`): the published grid, its 2018-01-21 divergence
//! from the grain and oilseed clock, and the caller-owned day overlay.
//!
//! Every probe here is stated in America/Chicago wall-clock and converted, so a
//! DST slip in either direction fails rather than passing on a coincidence.
//! 2026-06-14 is a Sunday and 2026-06-19 the Friday of the same week;
//! 2018-01-21 is the Sunday on which CBOT Submission 18-001 takes effect.

use chrono::{DateTime, NaiveDate, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, DayOverride, MarketHoursKey, SessionState, StaticDayPolicy,
    calendar_for_market_hours_key, hours_for_market_hours_key, session_profile,
};

const ZR: MarketHoursKey = MarketHoursKey::GlobexRoughRice;

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

fn state_at(instant: DateTime<Utc>) -> SessionState {
    calendar_for_market_hours_key(ZR).session_state(instant)
}

/// The published current grid, boundary by boundary: CME's Rough Rice
/// specification gives Globex Sunday-Thursday 19:00-21:00 CT, Monday-Friday
/// 08:30-13:20 CT, and Pre-Opens Sunday 16:00-19:00 and Monday-Thursday
/// 16:45-19:00 CT.
///
/// Each open is fenced by the second before it, and each close is asserted
/// end-exclusive. The two negative probes at 08:29:59 and 15:00 are the whole
/// point of the key existing separately: standard grains queue from 08:00 and
/// run a 14:30-16:00 post-close Pre-Open, and Rough Rice's specification
/// publishes neither, so this profile must report no order acceptance there.
#[test]
fn rough_rice_serves_the_published_grid_with_end_exclusive_closes() {
    let hours = hours_for_market_hours_key(ZR, ct((2026, 6, 15), (12, 0, 0)));

    // Sunday evening queue, then the electronic open.
    assert!(!hours.is_accepting_orders(ct((2026, 6, 14), (15, 59, 59))));
    assert!(hours.is_order_entry_only(ct((2026, 6, 14), (16, 0, 0))));
    assert!(hours.is_order_entry_only(ct((2026, 6, 14), (18, 59, 59))));
    assert!(!hours.is_open(ct((2026, 6, 14), (18, 59, 59))));
    assert!(hours.is_open(ct((2026, 6, 14), (19, 0, 0))));
    assert_eq!(
        state_at(ct((2026, 6, 14), (19, 0, 0))),
        SessionState::OpenExtended,
        "18-001's own column heading classifies the evening leg as extended"
    );

    // The 21:00 close is end-exclusive, and nothing wraps past it.
    assert!(hours.is_open(ct((2026, 6, 14), (20, 59, 59))));
    assert!(!hours.is_open(ct((2026, 6, 14), (21, 0, 0))));
    assert!(!hours.is_open(ct((2026, 6, 15), (0, 0, 0))));
    assert!(!hours.is_open(ct((2026, 6, 15), (7, 45, 0))));

    // No morning queue: the grains 08:00-08:30 Pre-Open is not Rough Rice's.
    assert!(!hours.is_accepting_orders(ct((2026, 6, 15), (8, 0, 0))));
    assert!(!hours.is_accepting_orders(ct((2026, 6, 15), (8, 29, 59))));
    assert!(hours.is_open(ct((2026, 6, 15), (8, 30, 0))));
    assert_eq!(
        state_at(ct((2026, 6, 15), (8, 30, 0))),
        SessionState::OpenRegular,
    );

    // The 13:20 regular close is end-exclusive, and no post-close Pre-Open runs.
    assert!(hours.is_open(ct((2026, 6, 15), (13, 19, 59))));
    assert!(!hours.is_open(ct((2026, 6, 15), (13, 20, 0))));
    assert!(!hours.is_accepting_orders(ct((2026, 6, 15), (14, 30, 0))));
    assert!(!hours.is_accepting_orders(ct((2026, 6, 15), (15, 0, 0))));

    // Weekday evening queue, then the next evening leg.
    assert!(!hours.is_accepting_orders(ct((2026, 6, 15), (16, 44, 59))));
    assert!(hours.is_order_entry_only(ct((2026, 6, 15), (16, 45, 0))));
    assert!(hours.is_open(ct((2026, 6, 15), (19, 0, 0))));

    // The fixed snapshot must say the same thing as the dated selector.
    let profile = session_profile(ZR);
    assert!(profile.is_open(ct((2026, 6, 14), (19, 0, 0))));
    assert!(!profile.is_open(ct((2026, 6, 14), (21, 0, 0))));
    assert!(!profile.is_accepting_orders(ct((2026, 6, 15), (8, 0, 0))));
}

/// The 21:00-08:30 CT gap is a `Halt`, not a close: the evening leg and the
/// regular session that follows it share one trade date, so a caller must not
/// see the trading day end at 21:00.
///
/// The 13:20-19:00 CT gap is the opposite case and is asserted beside it: it
/// separates two trade dates and is longer than the four-hour maintenance
/// ceiling, so it is `Closed` outside the 16:45 queue.
#[test]
fn the_evening_leg_and_the_next_regular_session_are_one_halted_trade_date() {
    let calendar = calendar_for_market_hours_key(ZR);

    for (hour, minute) in [(21_u32, 0_u32), (23, 59), (2, 0), (8, 29)] {
        let instant = if hour >= 21 {
            ct((2026, 6, 14), (hour, minute, 0))
        } else {
            ct((2026, 6, 15), (hour, minute, 0))
        };
        assert_eq!(
            state_at(instant),
            SessionState::Halt,
            "{instant}: the 21:00-08:30 CT break sits inside one trade date"
        );
    }

    // Same trade date on both sides of the break.
    assert_eq!(
        calendar.trade_date(ct((2026, 6, 14), (20, 0, 0))),
        Some(day((2026, 6, 15))),
        "the Sunday-evening leg belongs to Monday's trade date"
    );
    assert_eq!(
        calendar.trade_date(ct((2026, 6, 15), (12, 0, 0))),
        Some(day((2026, 6, 15))),
    );
    assert_eq!(
        calendar.trade_date(ct((2026, 6, 15), (20, 0, 0))),
        Some(day((2026, 6, 16))),
        "the Monday-evening leg belongs to Tuesday's trade date"
    );

    // The trading day therefore ends at the 13:20 regular close, not at 21:00:
    // one daily bar spans Sunday 19:00 CT through Monday 13:20 CT.
    assert_eq!(
        calendar.candle_end(ct((2026, 6, 14), (20, 0, 0)), CalendarResolution::Daily),
        Some(ct((2026, 6, 15), (13, 20, 0))),
    );
    assert_eq!(
        calendar.candle_end(ct((2026, 6, 15), (12, 0, 0)), CalendarResolution::Daily),
        Some(ct((2026, 6, 15), (13, 20, 0))),
    );

    // The afternoon break crosses a trade date and is longer than four hours.
    assert_eq!(
        state_at(ct((2026, 6, 15), (14, 0, 0))),
        SessionState::Closed
    );
    assert_eq!(
        state_at(ct((2026, 6, 15), (16, 45, 0))),
        SessionState::OrderEntry,
    );
}

/// The week ends at the Friday 13:20 CT close. There is no Friday-evening leg,
/// because 18-001 moved the extended session to Sunday-Thursday, and no Friday
/// evening queue either.
#[test]
fn rough_rice_has_no_friday_evening_session_or_queue() {
    let calendar = calendar_for_market_hours_key(ZR);
    let hours = hours_for_market_hours_key(ZR, ct((2026, 6, 19), (12, 0, 0)));

    assert!(hours.is_open(ct((2026, 6, 19), (13, 19, 59))));
    for (date, time) in [
        ((2026, 6, 19), (16, 45, 0)),
        ((2026, 6, 19), (19, 0, 0)),
        ((2026, 6, 19), (20, 0, 0)),
        ((2026, 6, 20), (12, 0, 0)),
        ((2026, 6, 21), (12, 0, 0)),
    ] {
        let instant = ct(date, time);
        assert!(
            !hours.is_accepting_orders(instant),
            "{instant}: the weekend break admits neither trading nor queueing"
        );
    }

    assert_eq!(
        calendar.next_session_open_after(ct((2026, 6, 19), (13, 20, 0))),
        Some(ct((2026, 6, 21), (19, 0, 0))),
        "the week reopens on Sunday evening, not Friday evening"
    );
}

/// Both sides of the divergence, evaluated at venue-local midnight.
///
/// CBOT Submission 18-001 certifies the reduction of Rough Rice extended hours
/// "effective on Sunday, January 21, 2018 for trade date Monday, January 22,
/// 2018", replacing "Sunday - Friday, 7:00 p.m. - 7:45 a.m. CT" with "Sunday -
/// Thursday, 7:00 p.m. - 9:00 p.m. CT". Chicago is UTC-6 in January, so
/// 2018-01-20 23:59:59 CT is the last instant the grain and oilseed grid
/// governs and 2018-01-21 00:00:00 CT the first the reduced one does. Each
/// probe is judged against the profile its own boundary instant selects, so a
/// revision keyed one day early or late fails here.
#[test]
fn rough_rice_extended_hours_shrink_to_1900_2100_from_2018_01_21() {
    let earlier = hours_for_market_hours_key(ZR, ct((2018, 1, 20), (23, 59, 59)));
    let revised = hours_for_market_hours_key(ZR, ct((2018, 1, 21), (0, 0, 0)));

    // The 19:00 open is common to both grids, pinned to the second.
    let evening_open = ct((2018, 1, 21), (19, 0, 0));
    for (label, hours) in [("earlier", &earlier), ("revised", &revised)] {
        assert!(
            !hours.is_open(evening_open - chrono::Duration::seconds(1)),
            "{label}: the evening leg opens a second too early"
        );
        assert!(hours.is_open(evening_open), "{label}: 19:00 CT must open");
    }

    // Everything after 21:00 is the divergence itself.
    for (hour, minute, what) in [
        (21_u32, 0_u32, "21:00 CT, the new close"),
        (23, 30, "23:30 CT, inside the withdrawn overnight leg"),
    ] {
        let instant = ct((2018, 1, 21), (hour, minute, 0));
        assert!(
            earlier.is_open(instant),
            "{what}: the Sunday-Friday 19:00-07:45 grid was still open here through 2018-01-20"
        );
        assert!(
            !revised.is_open(instant),
            "{what}: 18-001 closes Rough Rice at 21:00 CT from 2018-01-21"
        );
    }
    for (hour, minute, what) in [
        (3_u32, 0_u32, "03:00 CT, past the vanished midnight wrap"),
        (7, 44, "07:44 CT, one minute inside the old 07:45 close"),
    ] {
        let instant = ct((2018, 1, 22), (hour, minute, 0));
        assert!(earlier.is_open(instant), "{what}: open on the earlier grid");
        assert!(
            !revised.is_open(instant),
            "{what}: from 2018-01-21 Rough Rice has no session crossing local midnight"
        );
    }

    // Regular trading hours are untouched by 18-001, on both sides.
    for (label, hours) in [("earlier", &earlier), ("revised", &revised)] {
        assert!(
            hours.is_open(ct((2018, 1, 22), (8, 30, 0))),
            "{label}: the 08:30 CT regular open is not part of the divergence"
        );
        assert!(
            !hours.is_open(ct((2018, 1, 22), (13, 20, 0))),
            "{label}: the 13:20 CT regular close is end-exclusive on both grids"
        );
    }

    // The queue set narrows at the same boundary. 18-001 does not state this,
    // and `docs/schedules/verification.md` records that as the reason the row
    // is Partial; the profile withdraws the inherited grains windows rather
    // than claiming queues CME's Rough Rice specification does not publish.
    for (hour, minute, what) in [
        (8_u32, 10_u32, "the grains 08:00-08:30 CT morning Pre-Open"),
        (15, 0, "the grains 14:30-16:00 CT post-close Pre-Open"),
    ] {
        let instant = ct((2018, 1, 22), (hour, minute, 0));
        assert!(
            earlier.is_accepting_orders(instant),
            "{what} is inherited from the grain and oilseed grid before the divergence"
        );
        assert!(
            !revised.is_accepting_orders(instant),
            "{what} is not published for Rough Rice and is not claimed after the divergence"
        );
    }
    // The two evening queues survive the divergence unchanged.
    assert!(revised.is_order_entry_only(ct((2018, 1, 21), (16, 0, 0))));
    assert!(revised.is_order_entry_only(ct((2018, 1, 22), (16, 45, 0))));
}

/// Rough Rice does not share the grain and oilseed clock after 2018-01-21. If
/// the key were ever wired to `cbot_profile_at`, or the grains profile were
/// edited on Rough Rice evidence, these probes would agree instead of
/// disagreeing.
#[test]
fn rough_rice_and_the_grain_grid_disagree_after_the_divergence() {
    let overnight = ct((2026, 6, 16), (3, 0, 0));
    let grains = hours_for_market_hours_key(MarketHoursKey::GlobexGrains, overnight);
    let rough_rice = hours_for_market_hours_key(ZR, overnight);

    assert!(
        grains.is_open(overnight),
        "standard grains still run 19:00-07:45 CT"
    );
    assert!(
        !rough_rice.is_open(overnight),
        "Rough Rice closes at 21:00 CT and must not borrow the grain overnight leg"
    );
    assert!(
        session_profile(MarketHoursKey::GlobexGrains)
            .is_accepting_orders(ct((2026, 6, 16), (8, 10, 0))),
        "standard grains keep the 08:00-08:30 CT morning Pre-Open"
    );

    // Before the divergence the two grids agree, which is what makes the
    // inherited pre-2018 history legitimate rather than a coincidence.
    let before = ct((2016, 6, 15), (3, 0, 0));
    assert!(hours_for_market_hours_key(MarketHoursKey::GlobexGrains, before).is_open(before));
    assert!(hours_for_market_hours_key(ZR, before).is_open(before));
}

/// The caller-owned day overlay applies to Rough Rice like any other family: a
/// closed trade date removes the whole trading day including the prior evening
/// leg that belongs to it, and an early final close clips the regular session
/// without touching the queues that precede it.
#[test]
fn day_policy_overlays_a_closed_date_and_an_early_close() {
    let closed_tuesday = day((2026, 6, 16));
    let early_wednesday = day((2026, 6, 17));
    let records = [
        DayOverride::closed(closed_tuesday),
        DayOverride::early_close(early_wednesday, 12 * 3_600),
    ];
    let policy = StaticDayPolicy::new(&records).expect("the fixture records must be valid");
    let calendar = calendar_for_market_hours_key(ZR).with_day_policy(&policy);
    let plain = calendar_for_market_hours_key(ZR);

    // Tuesday's whole trading day goes, including Monday's evening leg.
    for (date, time) in [((2026, 6, 15), (20, 0, 0)), ((2026, 6, 16), (12, 0, 0))] {
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
    assert!(calendar.is_open(ct((2026, 6, 16), (20, 0, 0))));

    // Wednesday's regular session ends at the overridden 12:00 CT.
    assert!(calendar.is_open(ct((2026, 6, 17), (11, 59, 59))));
    assert!(!calendar.is_open(ct((2026, 6, 17), (12, 0, 0))));
    assert!(
        plain.is_open(ct((2026, 6, 17), (12, 0, 0))),
        "the override, not the profile, is what closed Wednesday early"
    );
    assert_eq!(
        calendar.session_bounds(ct((2026, 6, 17), (10, 0, 0))),
        Some((ct((2026, 6, 17), (8, 30, 0)), ct((2026, 6, 17), (12, 0, 0)))),
    );
}

/// The wire identity round-trips through every public spelling.
#[test]
fn rough_rice_key_round_trips_through_its_canonical_name() {
    assert_eq!(ZR.as_str(), "globex_rough_rice");
    assert_eq!(ZR.to_string(), "globex_rough_rice");
    assert_eq!("globex_rough_rice".parse::<MarketHoursKey>(), Ok(ZR));
    assert_eq!(
        serde_json::to_string(&ZR).expect("key serializes"),
        "\"globex_rough_rice\""
    );
    assert_eq!(
        serde_json::from_str::<MarketHoursKey>("\"globex_rough_rice\"").expect("key deserializes"),
        ZR
    );
    assert!(
        "globex_rice".parse::<MarketHoursKey>().is_err(),
        "a near-miss name must be rejected, never mapped to the nearest family"
    );
}
