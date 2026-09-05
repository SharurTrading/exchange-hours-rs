// SPDX-License-Identifier: MIT-0

//! CBOT mini-sized grains (`XC`/`XK`/`XW`/`MKC`): the published grid, the
//! seven dated revisions of the mini timeline, and the family's separation
//! from the standard grain clock.
//!
//! Every probe here is stated in America/Chicago wall-clock and converted, so
//! a DST slip in either direction fails rather than passing on a coincidence.
//! 2026-06-14 is a Sunday and 2026-06-19 the Friday of the same week; each
//! cutover test evaluates both sides of its revision at venue-local midnight,
//! so a revision keyed one day early or late flips an assertion.

use chrono::{DateTime, NaiveDate, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, MarketHoursKey, SessionState, calendar_for_market_hours_key,
    hours_for_market_hours_key, session_profile,
};

const MINI: MarketHoursKey = MarketHoursKey::GlobexMiniGrains;
const GRAINS: MarketHoursKey = MarketHoursKey::GlobexGrains;

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
    hours_for_market_hours_key(MINI, ct(date, (0, 0, 0)))
}

fn state_at(instant: DateTime<Utc>) -> SessionState {
    calendar_for_market_hours_key(MINI).session_state(instant)
}

/// The published current grid, boundary by boundary. SER-9049's Pre-Open
/// column and the October-2022 Globex notices give Sunday 16:00-19:00 and
/// Monday-Thursday 16:45-19:00 CT Pre-Opens, Monday-Friday 08:00-08:30 CT
/// Pre-Open, an unchanged Sunday-Friday 19:00-07:45 CT overnight leg, and the
/// amended Monday-Friday 08:30-13:20 CT day session.
///
/// Each open is fenced by the second before it, and each close is asserted
/// end-exclusive.
#[test]
fn mini_grains_serves_the_published_grid_with_end_exclusive_closes() {
    let hours = hours_for_market_hours_key(MINI, ct((2026, 6, 15), (12, 0, 0)));

    // Sunday evening queue, then the electronic open.
    assert!(!hours.is_accepting_orders(ct((2026, 6, 14), (15, 59, 59))));
    assert!(hours.is_order_entry_only(ct((2026, 6, 14), (16, 0, 0))));
    assert!(hours.is_order_entry_only(ct((2026, 6, 14), (18, 59, 59))));
    assert!(!hours.is_open(ct((2026, 6, 14), (18, 59, 59))));
    assert!(hours.is_open(ct((2026, 6, 14), (19, 0, 0))));
    assert_eq!(
        state_at(ct((2026, 6, 14), (19, 0, 0))),
        SessionState::OpenExtended,
        "the overnight leg is the extended session, as on the standard grid"
    );

    // The overnight leg wraps local midnight and closes 07:45 end-exclusive.
    assert!(hours.is_open(ct((2026, 6, 15), (3, 0, 0))));
    assert!(hours.is_open(ct((2026, 6, 15), (7, 44, 59))));
    assert!(!hours.is_open(ct((2026, 6, 15), (7, 45, 0))));

    // The 07:45-08:00 break sits inside the same trade date, then the morning
    // Pre-Open runs 08:00-08:30 and the regular session opens 08:30.
    assert_eq!(state_at(ct((2026, 6, 15), (7, 50, 0))), SessionState::Halt);
    assert!(!hours.is_accepting_orders(ct((2026, 6, 15), (7, 59, 59))));
    assert!(hours.is_order_entry_only(ct((2026, 6, 15), (8, 0, 0))));
    assert!(hours.is_order_entry_only(ct((2026, 6, 15), (8, 29, 59))));
    assert!(hours.is_open(ct((2026, 6, 15), (8, 30, 0))));
    assert_eq!(
        state_at(ct((2026, 6, 15), (8, 30, 0))),
        SessionState::OpenRegular,
    );

    // The 13:20 regular close is end-exclusive — the convergence with the
    // standard grid — and the post-close PCP runs 14:30-16:00.
    assert!(hours.is_open(ct((2026, 6, 15), (13, 19, 59))));
    assert!(!hours.is_open(ct((2026, 6, 15), (13, 20, 0))));
    assert!(!hours.is_accepting_orders(ct((2026, 6, 15), (14, 0, 0))));
    assert!(hours.is_order_entry_only(ct((2026, 6, 15), (14, 30, 0))));
    assert!(hours.is_order_entry_only(ct((2026, 6, 15), (15, 59, 59))));

    // Nothing accepts orders in the 16:00-16:45 PCP-to-evening-queue gap; the
    // Monday-Thursday evening queue then runs to the 19:00 open.
    assert!(!hours.is_accepting_orders(ct((2026, 6, 15), (16, 30, 0))));
    assert!(hours.is_order_entry_only(ct((2026, 6, 15), (16, 45, 0))));
    assert!(hours.is_open(ct((2026, 6, 15), (19, 0, 0))));

    // The fixed snapshot must say the same thing as the dated selector.
    let profile = session_profile(MINI);
    assert!(profile.is_open(ct((2026, 6, 14), (19, 0, 0))));
    assert!(profile.is_open(ct((2026, 6, 15), (3, 0, 0))));
    assert!(!profile.is_open(ct((2026, 6, 15), (13, 20, 0))));
    assert!(profile.is_accepting_orders(ct((2026, 6, 15), (8, 10, 0))));
    assert!(profile.is_accepting_orders(ct((2026, 6, 15), (15, 0, 0))));
}

/// The overnight leg wraps local midnight, so the generic close-date
/// convention applies and no trade-date exception is needed: Sunday evening's
/// leg, Monday's queues and day session, and Monday's PCP resolve to the
/// trade dates their closes name.
#[test]
fn the_wrapping_grid_needs_no_trade_date_exception() {
    let calendar = calendar_for_market_hours_key(MINI);

    assert_eq!(
        calendar.trade_date(ct((2026, 6, 14), (20, 0, 0))),
        Some(day((2026, 6, 15))),
        "the Sunday-evening leg wraps, so it belongs to Monday's trade date"
    );
    assert_eq!(
        calendar.trade_date(ct((2026, 6, 15), (12, 0, 0))),
        Some(day((2026, 6, 15))),
    );
    // The queues run up to the next session, whose close names the trade date
    // they attach to: Monday's PCP and evening queue precede the leg that
    // closes Tuesday 13:20 CT.
    assert_eq!(
        calendar.trade_date(ct((2026, 6, 15), (15, 0, 0))),
        Some(day((2026, 6, 16))),
    );
    assert_eq!(
        calendar.trade_date(ct((2026, 6, 15), (17, 0, 0))),
        Some(day((2026, 6, 16))),
    );

    // One daily bar spans the Sunday 19:00 CT open through Monday 13:20 CT.
    assert_eq!(
        calendar.candle_end(ct((2026, 6, 14), (20, 0, 0)), CalendarResolution::Daily),
        Some(ct((2026, 6, 15), (13, 20, 0))),
    );
    assert_eq!(
        calendar.candle_end(ct((2026, 6, 15), (12, 0, 0)), CalendarResolution::Daily),
        Some(ct((2026, 6, 15), (13, 20, 0))),
    );

    // The 13:20-19:00 CT stretch separates two trade dates and outlasts the
    // four-hour maintenance ceiling, so its orderless sub-gaps are `Closed`
    // while the PCP and evening queue inside it are `OrderEntry`.
    assert_eq!(
        state_at(ct((2026, 6, 15), (13, 30, 0))),
        SessionState::Closed
    );
    assert_eq!(
        state_at(ct((2026, 6, 15), (15, 0, 0))),
        SessionState::OrderEntry
    );
    assert_eq!(
        state_at(ct((2026, 6, 15), (16, 30, 0))),
        SessionState::Closed
    );
}

/// The week ends at the Friday 13:20 CT close. The evening queue is
/// Monday-Thursday only, and the next open is Sunday evening.
#[test]
fn the_week_reopens_on_sunday_evening_after_the_weekend_close() {
    let calendar = calendar_for_market_hours_key(MINI);
    let hours = hours_for_market_hours_key(MINI, ct((2026, 6, 19), (12, 0, 0)));

    assert!(hours.is_open(ct((2026, 6, 19), (13, 19, 59))));
    assert!(hours.is_order_entry_only(ct((2026, 6, 19), (15, 0, 0))));
    for (date, time) in [
        ((2026, 6, 19), (16, 45, 0)),
        ((2026, 6, 19), (19, 0, 0)),
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

/// The overnight leg is quoted in Central wall-clock, so it survives both DST
/// transitions with its wall-clock endpoints intact and its elapsed duration
/// changing underneath.
#[test]
fn the_overnight_leg_survives_both_dst_transitions() {
    // Spring forward: 2026-03-08 is the second Sunday of March.
    let spring = hours_for_market_hours_key(MINI, ct((2026, 3, 9), (12, 0, 0)));
    assert!(spring.is_open(ct((2026, 3, 8), (19, 0, 0))));
    assert!(spring.is_open(ct((2026, 3, 9), (1, 30, 0))));
    assert!(spring.is_open(ct((2026, 3, 9), (3, 0, 0))));
    assert!(spring.is_open(ct((2026, 3, 9), (7, 44, 59))));
    assert!(!spring.is_open(ct((2026, 3, 9), (7, 45, 0))));
    assert!(spring.is_open(ct((2026, 3, 9), (8, 30, 0))));

    // Fall back: 2026-11-01 is the first Sunday of November.
    let fall = hours_for_market_hours_key(MINI, ct((2026, 11, 2), (12, 0, 0)));
    assert!(fall.is_open(ct((2026, 11, 1), (19, 0, 0))));
    assert!(fall.is_open(ct((2026, 11, 2), (2, 30, 0))));
    assert!(fall.is_open(ct((2026, 11, 2), (6, 30, 0))));
    assert!(!fall.is_open(ct((2026, 11, 2), (7, 45, 0))));
}

/// 2010-04-19 re-anchors the afternoon PCP to 30 seconds after the mini
/// 13:45 day close. Globex notice 20100405 carries a mini-specific block:
/// "CBOT Mini-sized grain futures / Trading ends at 1:45 p.m. Central time
/// (CT) ... Post-close pre-open begins at 1:45.30 CT". Chicago is UTC-5 in
/// April, so 2010-04-18 23:59:59 CT is the last instant the floor PCP
/// governs. The mini close itself is unchanged, and the mini PCP already
/// differs from the standard grains' 13:15:30 re-anchor on this same day.
#[test]
fn the_pcp_reanchors_to_134530_from_2010_04_19() {
    let earlier = hours_at((2010, 4, 18));
    let revised = hours_at((2010, 4, 19));

    for (label, hours) in [("earlier", &earlier), ("revised", &revised)] {
        assert!(
            hours.is_open(ct((2010, 4, 19), (13, 30, 0))),
            "{label}: the 13:45 mini close is untouched by the PCP change"
        );
        assert!(
            !hours.is_open(ct((2010, 4, 19), (13, 45, 0))),
            "{label}: the mini day close stays 13:45, end-exclusive"
        );
        assert!(
            hours.is_accepting_orders(ct((2010, 4, 19), (8, 0, 0))),
            "{label}: the 07:15 morning queue is untouched"
        );
    }

    assert!(
        !revised.is_accepting_orders(ct((2010, 4, 19), (13, 45, 29))),
        "the PCP opens 30 seconds after the mini close"
    );
    assert!(
        !earlier.is_accepting_orders(ct((2010, 4, 19), (14, 0, 0))),
        "the floor PCP still starts at 14:30"
    );
    assert!(
        revised.is_accepting_orders(ct((2010, 4, 19), (14, 0, 0))),
        "from 2010-04-19 the PCP covers 14:00, starting 13:45:30"
    );
    for (label, hours) in [("earlier", &earlier), ("revised", &revised)] {
        assert!(
            hours.is_accepting_orders(ct((2010, 4, 19), (15, 59, 59))),
            "{label}: the PCP still reaches 16:00"
        );
        assert!(
            !hours.is_accepting_orders(ct((2010, 4, 19), (16, 0, 0))),
            "{label}: the PCP close is end-exclusive"
        );
    }

    // The family discriminator at the floor: standard grains close 13:15 and
    // re-anchored their PCP to 13:15:30, so at 13:35 the minis are still
    // matching while the standard grains are already queueing.
    let grains = hours_for_market_hours_key(GRAINS, ct((2010, 4, 19), (12, 0, 0)));
    assert!(
        grains.is_order_entry_only(ct((2010, 4, 19), (13, 35, 0))),
        "standard grains close 13:15 and queue from 13:15:30"
    );
    assert!(
        revised.is_open(ct((2010, 4, 19), (13, 35, 0))),
        "the minis are still matching at 13:35 — the mini premium is a family fact"
    );
}

/// 2011-12-27 moves the morning Pre-Open's start from 07:15 to 08:00 for CBOT
/// agricultural futures (CFTC filing rul120711cbot001, effective "Tuesday,
/// December 27, 2011"); CME's trading-hours table of 25 January 2012 shows
/// the mini row at the same 08:00 start.
#[test]
fn the_morning_queue_starts_at_0800_from_2011_12_27() {
    let earlier = hours_at((2011, 12, 26));
    let revised = hours_at((2011, 12, 27));

    assert!(
        earlier.is_accepting_orders(ct((2011, 12, 27), (7, 30, 0))),
        "the morning queue starts at 07:15 through 2011-12-26"
    );
    assert!(
        !revised.is_accepting_orders(ct((2011, 12, 27), (7, 30, 0))),
        "from 2011-12-27 the morning queue waits for 08:00"
    );
    for (label, hours) in [("earlier", &earlier), ("revised", &revised)] {
        assert!(
            hours.is_accepting_orders(ct((2011, 12, 27), (8, 0, 0))),
            "{label}: 08:00 accepts on both grids"
        );
        assert!(
            !hours.is_open(ct((2011, 12, 27), (9, 29, 59))),
            "{label}: the floor-era day session still opens 09:30"
        );
        assert!(
            hours.is_open(ct((2011, 12, 27), (9, 30, 0))),
            "{label}: the 09:30-13:45 day session is unchanged"
        );
    }
}

/// 2012-05-20 joins the standard grains' 21-hour continuous session — the
/// only window before 2022 where the two grids are identical. Advisory
/// 20120518: "Effective this Sunday, May 20 (trade date Monday, May 21) ...
/// Sunday to Friday: 17:00 CT to 14:00 Central Time (CT)". The advisory
/// states only matching hours, so this era serves no order-entry phases —
/// the same conservative choice the standard grid makes for the same
/// undated queue switch.
#[test]
fn the_minis_join_the_21_hour_session_from_2012_05_20() {
    let earlier = hours_at((2012, 5, 19));
    let revised = hours_at((2012, 5, 20));

    assert!(
        !earlier.is_open(ct((2012, 5, 20), (17, 0, 0))),
        "the floor grid's Sunday open is 18:00"
    );
    assert!(
        revised.is_open(ct((2012, 5, 20), (17, 0, 0))),
        "the continuous session opens Sunday 17:00"
    );
    assert!(
        earlier.is_open(ct((2012, 5, 20), (18, 0, 0))),
        "the floor grid opens Sunday 18:00"
    );

    // Monday 08:00 separates a continuous regime from the overnight-plus-day
    // floor grid, whose overnight closed 07:15 and whose day opens 09:30.
    assert!(
        earlier.is_open(ct((2012, 5, 21), (3, 0, 0))),
        "the floor overnight leg is open at 03:00"
    );
    assert!(!earlier.is_open(ct((2012, 5, 21), (8, 0, 0))));
    assert!(
        revised.is_open(ct((2012, 5, 21), (8, 0, 0))),
        "the continuous session runs straight through 08:00"
    );
    assert!(
        !revised.is_open(ct((2012, 5, 21), (14, 0, 0))),
        "the continuous close is end-exclusive at 14:00"
    );
    assert!(
        revised.is_open(ct((2012, 5, 21), (13, 50, 0))),
        "the mini day session reaches 14:00 through the 13:45 open-outcry close"
    );
    assert!(
        !revised.is_accepting_orders(ct((2012, 5, 21), (14, 30, 0))),
        "no queue is dated inside the 21-hour regime, so nothing accepts orders"
    );
    assert!(
        earlier.is_accepting_orders(ct((2012, 5, 21), (14, 30, 0))),
        "the floor PCP still starts at 14:30 on the earlier grid"
    );
}

/// 2012-09-16 is the divergence the whole key exists for: "Effective Sunday,
/// September 16 (trade date Monday, September 17) ... Sunday to Friday:
/// 17:00 CT to 14:30 Central Time (CT) / Pause State: 14:30 (CT) / Close
/// State: 14:35 (CT) / Post Close/Pre Open (PCP)14:40 (CT)", while the
/// standard grains stay at 14:00. The advisory dates the PCP start; its
/// 16:00 end comes from CME's trading-hours page captured 15 September 2012.
#[test]
fn the_minis_diverge_to_1430_from_2012_09_16() {
    let earlier = hours_at((2012, 9, 15));
    let revised = hours_at((2012, 9, 16));

    for (label, hours) in [("earlier", &earlier), ("revised", &revised)] {
        assert!(
            hours.is_open(ct((2012, 9, 17), (13, 50, 0))),
            "{label}: both continuous regimes run through the 13:45 mini day open"
        );
    }
    assert!(
        !earlier.is_open(ct((2012, 9, 17), (14, 15, 0))),
        "the 2012-05-20 regime closes at 14:00"
    );
    assert!(
        revised.is_open(ct((2012, 9, 17), (14, 15, 0))),
        "the mini extension runs to 14:30 — the divergence, one probe wide"
    );
    assert!(revised.is_open(ct((2012, 9, 17), (14, 29, 59))));
    assert!(
        !revised.is_open(ct((2012, 9, 17), (14, 30, 0))),
        "the 14:30 pause state ends matching, end-exclusive"
    );

    assert!(
        !earlier.is_accepting_orders(ct((2012, 9, 17), (14, 45, 0))),
        "the earlier era serves no queues at all"
    );
    assert!(
        revised.is_order_entry_only(ct((2012, 9, 17), (14, 45, 0))),
        "the advisory dates the PCP from 14:40"
    );
    assert!(
        !revised.is_accepting_orders(ct((2012, 9, 17), (16, 30, 0))),
        "the PCP ends 16:00 and no other queue is dated in this era"
    );
}

/// 2013-04-07 is the great reduction, restoring the named mini premium:
/// "Sunday-Friday: 19:00 to 07:45 CT / Monday-Friday: Break from 07:45 to
/// 08:30 CT / Monday-Friday: 08:30 to 13:15 CT, Mini-Sized Grains: 08:30 to
/// 13:45 CT" (Global Command Center notice of 22 March 2013; CBOT Submission
/// 13-092 certifies "both electronic and floor hours ... will close daily at
/// 1:45 p.m. CT"). The same notice establishes the queue set.
#[test]
fn the_2013_reduction_restores_the_mini_premium() {
    let earlier = hours_at((2013, 4, 6));
    let revised = hours_at((2013, 4, 7));

    assert!(
        earlier.is_open(ct((2013, 4, 7), (17, 30, 0))),
        "the continuous regime opens Sunday 17:00"
    );
    assert!(
        revised.is_order_entry_only(ct((2013, 4, 7), (17, 30, 0))),
        "the reduced grid queues Sunday 16:00-19:00 instead"
    );
    assert!(
        revised.is_open(ct((2013, 4, 7), (19, 0, 0))),
        "the reduced overnight leg opens Sunday 19:00"
    );

    assert!(
        earlier.is_open(ct((2013, 4, 8), (8, 0, 0))),
        "the continuous regime runs through the morning break"
    );
    assert!(
        !revised.is_open(ct((2013, 4, 8), (8, 0, 0))),
        "the reduced grid breaks 07:45-08:30"
    );
    assert!(
        revised.is_order_entry_only(ct((2013, 4, 8), (8, 20, 0))),
        "the new morning Pre-Open runs 08:15-08:30"
    );

    assert!(
        revised.is_open(ct((2013, 4, 8), (13, 44, 59))),
        "the mini day session opens 08:30"
    );
    assert!(
        !revised.is_open(ct((2013, 4, 8), (13, 50, 0))),
        "the mini premium is back: the minis close 13:45, end-exclusive"
    );
    assert!(
        earlier.is_open(ct((2013, 4, 8), (14, 15, 0))),
        "the continuous regime was still running to 14:30 the day before"
    );
    assert!(
        revised.is_order_entry_only(ct((2013, 4, 8), (15, 0, 0))),
        "the notice dates the PCP at 14:30-16:00"
    );
}

/// 2013-08-18 widens the morning Pre-Open from 08:15 to 08:00: "Effective
/// Sunday, August 18, 2013 (trade date Monday, August 19), the Pre-Open
/// market hours will be expanded to: Monday – Friday, 08:00 Central Time
/// (CT) to 08:30 CT for the following products: CBOT Grain and Oilseed
/// futures and options ...", with the PCP restated unchanged. SER-9049 later
/// lists the same 08:00-08:30 window for all four minis as "(unchanged)".
#[test]
fn the_morning_queue_returns_to_0800_from_2013_08_18() {
    let earlier = hours_at((2013, 8, 17));
    let revised = hours_at((2013, 8, 18));

    assert!(
        !revised.is_accepting_orders(ct((2013, 8, 20), (7, 59, 59))),
        "the widened queue still starts no earlier than 08:00"
    );
    assert!(
        !earlier.is_accepting_orders(ct((2013, 8, 20), (8, 5, 0))),
        "through 2013-08-17 the morning Pre-Open waits for 08:15"
    );
    assert!(
        revised.is_accepting_orders(ct((2013, 8, 20), (8, 5, 0))),
        "from 2013-08-18 it accepts from 08:00"
    );
    assert!(
        earlier.is_accepting_orders(ct((2013, 8, 20), (8, 15, 0))),
        "the outgoing 08:15-08:30 window ends at the same 08:30 close"
    );
    for (label, hours) in [("earlier", &earlier), ("revised", &revised)] {
        assert!(
            hours.is_open(ct((2013, 8, 20), (8, 30, 0))),
            "{label}: the 08:30-13:45 day session is untouched"
        );
        assert!(
            hours.is_order_entry_only(ct((2013, 8, 20), (15, 0, 0))),
            "{label}: the PCP stays 14:30-16:00"
        );
    }
}

/// 2022-10-02 is the convergence: "Effective Sunday, October 2 (trade date
/// Monday, October 3), the trading hours for the following CBOT mini-sized
/// agriculture futures will be amended" from a 1:45 p.m. to a 1:20 p.m. day
/// close, one table row each for XC, MKC, XK and XW; SER-9049 states the
/// intent to "align the trading hours of the Contracts with the
/// corresponding standard sized agricultural futures contracts", with
/// Pre-Open hours unchanged.
#[test]
fn the_grid_converges_to_the_standard_close_from_2022_10_02() {
    let earlier = hours_at((2022, 10, 1));
    let revised = hours_at((2022, 10, 2));

    for (label, hours) in [("earlier", &earlier), ("revised", &revised)] {
        assert!(
            hours.is_open(ct((2022, 10, 2), (19, 0, 0))),
            "{label}: the overnight leg is unchanged by the convergence"
        );
        assert!(
            hours.is_open(ct((2022, 10, 3), (13, 10, 0))),
            "{label}: the day session runs past 13:10"
        );
        assert!(
            hours.is_order_entry_only(ct((2022, 10, 3), (15, 0, 0))),
            "{label}: the PCP is unchanged"
        );
    }
    assert!(
        earlier.is_open(ct((2022, 10, 3), (13, 30, 0))),
        "through 2022-10-01 the mini close is 13:45"
    );
    assert!(
        !revised.is_open(ct((2022, 10, 3), (13, 30, 0))),
        "from 2022-10-02 the mini close is 13:20 — the premium is gone"
    );
    assert!(revised.is_open(ct((2022, 10, 3), (13, 19, 59))));
    assert!(!revised.is_open(ct((2022, 10, 3), (13, 20, 0))));
}

/// Envelope match is not family identity. The minis diverged from the
/// standard grain grid on 2012-09-16, skipped its 2015-07-05 close change,
/// and converged only on 2022-10-02 — so between those days a caller reading
/// the minis through `globex_grains` gets a day close 25 or 30 minutes too
/// early, and MKC's KC lineage must not pull the family onto the KC clock.
#[test]
fn mini_grains_and_the_standard_grain_grid_disagree_outside_the_converged_eras() {
    // The 2012-05-20..2012-09-15 overlap: the two grids genuinely agree, which
    // is what makes the shared 21-hour encoding legitimate rather than lazy.
    let overlap_mini = hours_for_market_hours_key(MINI, ct((2012, 6, 15), (12, 0, 0)));
    let overlap_grains = hours_for_market_hours_key(GRAINS, ct((2012, 6, 15), (12, 0, 0)));
    assert!(overlap_mini.is_open(ct((2012, 6, 15), (3, 0, 0))));
    assert!(overlap_grains.is_open(ct((2012, 6, 15), (3, 0, 0))));
    assert!(overlap_mini.is_open(ct((2012, 6, 15), (13, 50, 0))));
    assert!(overlap_grains.is_open(ct((2012, 6, 15), (13, 50, 0))));
    assert!(!overlap_mini.is_open(ct((2012, 6, 15), (14, 30, 0))));
    assert!(!overlap_grains.is_open(ct((2012, 6, 15), (14, 30, 0))));

    // After 2012-09-16 the extended session itself diverges.
    let diverged_mini = hours_for_market_hours_key(MINI, ct((2013, 1, 15), (12, 0, 0)));
    let diverged_grains = hours_for_market_hours_key(GRAINS, ct((2013, 1, 15), (12, 0, 0)));
    assert!(diverged_mini.is_open(ct((2013, 1, 15), (14, 15, 0))));
    assert!(!diverged_grains.is_open(ct((2013, 1, 15), (14, 15, 0))));

    // The minis skipped SER-7395R: every year from 2015 to 2021 reads a 13:45
    // mini close against the standard grid's 13:20. Each probe date is a
    // weekday in its year.
    for (year, month, day_of_month) in [(2015, 6, 15), (2016, 6, 15), (2019, 6, 17), (2021, 6, 15)]
    {
        let instant = ct((year, month, day_of_month), (12, 0, 0));
        let mini = hours_for_market_hours_key(MINI, instant);
        let grains = hours_for_market_hours_key(GRAINS, instant);
        assert!(
            mini.is_open(ct((year, month, day_of_month), (13, 30, 0))),
            "{year}: the mini day session still runs to 13:45"
        );
        assert!(
            !grains.is_open(ct((year, month, day_of_month), (13, 30, 0))),
            "{year}: the standard grid closed at 13:20"
        );
    }

    // After the convergence the two keys must answer identically — a full
    // week of probes, phases and states alike.
    for day_offset in 0..7_i64 {
        for (hour, minute) in [
            (3, 0),
            (7, 50),
            (8, 10),
            (12, 0),
            (13, 30),
            (15, 0),
            (16, 30),
            (17, 0),
            (20, 0),
        ] {
            let instant = ct((2026, 6, 14), (0, 0, 0))
                + chrono::Duration::days(day_offset)
                + chrono::Duration::hours(hour)
                + chrono::Duration::minutes(minute);
            let mini = hours_for_market_hours_key(MINI, instant);
            let grains = hours_for_market_hours_key(GRAINS, instant);
            assert_eq!(
                mini.is_open(instant),
                grains.is_open(instant),
                "{instant}: the converged grids must agree on is_open"
            );
            assert_eq!(
                mini.is_accepting_orders(instant),
                grains.is_accepting_orders(instant),
                "{instant}: the converged grids must agree on order acceptance"
            );
            assert_eq!(
                calendar_for_market_hours_key(MINI).session_state(instant),
                calendar_for_market_hours_key(GRAINS).session_state(instant),
                "{instant}: the converged grids must agree on session state"
            );
        }
    }
}

/// The wire identity round-trips through every public spelling.
#[test]
fn mini_grains_key_round_trips_through_its_canonical_name() {
    assert_eq!(MINI.as_str(), "globex_mini_grains");
    assert_eq!(MINI.to_string(), "globex_mini_grains");
    assert_eq!("globex_mini_grains".parse::<MarketHoursKey>(), Ok(MINI));
    assert_eq!(
        serde_json::to_string(&MINI).expect("key serializes"),
        "\"globex_mini_grains\""
    );
    assert_eq!(
        serde_json::from_str::<MarketHoursKey>("\"globex_mini_grains\"").expect("key deserializes"),
        MINI
    );
    assert!(
        "globex_mini".parse::<MarketHoursKey>().is_err(),
        "a near-miss name must be rejected, never mapped to the nearest family"
    );
    assert!(
        "globex_grains".parse::<MarketHoursKey>() != Ok(MINI),
        "the standard grain key must stay a distinct wire identity"
    );
}
