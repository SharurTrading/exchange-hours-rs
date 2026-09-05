// SPDX-License-Identifier: MIT-0

//! CME/CBOT Spot-Quoted Futures (`globex_spot_quoted`): the published grid,
//! the 2025-06-29 launch fence, the closed era before it, and the separations
//! from the three families this key is most likely to be confused with.
//!
//! Every probe is stated in America/Chicago wall-clock and converted, so a DST
//! slip in either direction fails rather than passing on a coincidence.
//! 2026-09-13 is a Sunday and 2026-09-18 the Friday of the same week — the
//! normal week CME's own session service was read for, and late enough that
//! `globex_weather`'s 2026-09-05 knowledge-bound row is also in force, so the
//! two keys can be compared on equal terms. 2025-06-29 is the Sunday on which
//! CME SER-9506R takes effect, for trade date Monday 2025-06-30.

use chrono::{DateTime, NaiveDate, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, DayOverride, MarketHoursKey, SessionState, StaticDayPolicy,
    calendar_for_market_hours_key, hours_for_market_hours_key, session_profile,
};

const SPOT_QUOTED: MarketHoursKey = MarketHoursKey::GlobexSpotQuoted;
const WEATHER: MarketHoursKey = MarketHoursKey::GlobexWeather;
const EQUITY_INDEX: MarketHoursKey = MarketHoursKey::GlobexEquityIndex;
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

fn hours_at(date: (i32, u32, u32)) -> exchange_hours::MarketHours {
    hours_for_market_hours_key(SPOT_QUOTED, ct(date, (0, 0, 0)))
}

fn state_at(instant: DateTime<Utc>) -> SessionState {
    calendar_for_market_hours_key(SPOT_QUOTED).session_state(instant)
}

/// The published current grid, boundary by boundary. CME's contract
/// specification gives "CME Globex: Sunday - Friday 5:00 p.m. - 4:00 p.m. with
/// a 60-minute break each day beginning at 4:00 p.m." for all eight roots, and
/// SER-9506R's Pre-Open line gives Sunday 16:00-17:00 and Monday-Thursday
/// 16:45-17:00 CT. CME's own session service returns exactly those
/// `preopen`/`open`/`closed` events for security groups `D1`-`D4` and
/// `D5`/`D6`/`D7`/`D9` alike.
///
/// Each open is fenced by the second before it, and each close is asserted
/// end-exclusive.
#[test]
fn spot_quoted_serves_the_published_grid_with_end_exclusive_closes() {
    let hours = hours_for_market_hours_key(SPOT_QUOTED, ct((2026, 9, 14), (12, 0, 0)));

    // Sunday queue: nothing is accepted at 15:59:59, everything queues from
    // 16:00, and nothing matches until 17:00.
    assert!(!hours.is_accepting_orders(ct((2026, 9, 13), (15, 59, 59))));
    assert!(hours.is_order_entry_only(ct((2026, 9, 13), (16, 0, 0))));
    assert!(hours.is_order_entry_only(ct((2026, 9, 13), (16, 59, 59))));
    assert!(!hours.is_open(ct((2026, 9, 13), (16, 59, 59))));
    assert!(hours.is_open(ct((2026, 9, 13), (17, 0, 0))));

    // Everything executable is extended. No CME document classifies any part
    // of the spot-quoted session as regular, so `regular` is empty.
    assert_eq!(
        state_at(ct((2026, 9, 13), (17, 0, 0))),
        SessionState::OpenExtended,
    );
    assert!(!hours.is_open_regular(ct((2026, 9, 14), (12, 0, 0))));
    assert!(session_profile(SPOT_QUOTED).regular.is_empty());

    // The leg wraps local midnight and closes 16:00 end-exclusive.
    assert!(hours.is_open(ct((2026, 9, 14), (3, 0, 0))));
    assert!(hours.is_open(ct((2026, 9, 14), (15, 59, 59))));
    assert!(!hours.is_open(ct((2026, 9, 14), (16, 0, 0))));

    // The 60-minute maintenance period: nothing matches, and for its first 45
    // minutes nothing queues either.
    assert_eq!(
        state_at(ct((2026, 9, 14), (16, 30, 0))),
        SessionState::Maintenance,
        "the one-hour break separates two trade dates and stays inside the \
         four-hour maintenance ceiling"
    );
    assert!(!hours.is_accepting_orders(ct((2026, 9, 14), (16, 44, 59))));
    assert!(hours.is_order_entry_only(ct((2026, 9, 14), (16, 45, 0))));
    assert_eq!(
        state_at(ct((2026, 9, 14), (16, 45, 0))),
        SessionState::OrderEntry,
    );
    assert!(hours.is_open(ct((2026, 9, 14), (17, 0, 0))));

    // The fixed snapshot must answer identically to the dated selector.
    let profile = session_profile(SPOT_QUOTED);
    assert!(profile.is_order_entry_only(ct((2026, 9, 13), (16, 0, 0))));
    assert!(profile.is_open(ct((2026, 9, 13), (17, 0, 0))));
    assert!(profile.is_open(ct((2026, 9, 14), (15, 59, 59))));
    assert!(!profile.is_open(ct((2026, 9, 14), (16, 0, 0))));
    assert!(profile.is_order_entry_only(ct((2026, 9, 14), (16, 45, 0))));
    assert!(profile.regular.is_empty());
}

/// The wrapping leg takes the trade date of its close, which is exactly what
/// CME's session service publishes: every 17:00 `open` carries the following
/// business day's `tradingDate` and every 16:00 `closed` carries the current
/// one. One daily bar therefore runs Sunday 17:00 CT to Monday 16:00 CT.
#[test]
fn the_wrapping_leg_takes_its_closing_trade_date() {
    let calendar = calendar_for_market_hours_key(SPOT_QUOTED);

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
        calendar.session_bounds(ct((2026, 9, 14), (12, 0, 0))),
        Some((ct((2026, 9, 13), (17, 0, 0)), ct((2026, 9, 14), (16, 0, 0)))),
    );
}

/// The week ends at the Friday 16:00 CT close: no Friday-evening reopen and no
/// Friday-evening queue, and the next open is Sunday 17:00 CT. CME's session
/// service shows the same shape — Friday carries a lone `closed` event at
/// 16:00 and Saturday carries no events at all.
#[test]
fn the_week_reopens_on_sunday_evening_with_no_friday_evening_leg() {
    let calendar = calendar_for_market_hours_key(SPOT_QUOTED);
    let hours = hours_for_market_hours_key(SPOT_QUOTED, ct((2026, 9, 18), (12, 0, 0)));

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

/// The launch fence. CME SER-9506R: "Effective Sunday, June 29, 2025 for trade
/// date Monday, June 30, 2025 ... will list the six (6) Spot-Quoted Futures
/// (SQF) contracts noted in Table 1. below ... for trading on the CME Globex
/// electronic trading platform".
///
/// The row is keyed to the venue-local **opening** day, not the stated trade
/// date, because the first session that exists at all is the one opening
/// Sunday 2025-06-29 at 17:00 CT (queueing from 16:00 CT) and closing on
/// Monday 2025-06-30. Both sides are evaluated at venue-local midnight, so a
/// revision keyed one day early or late flips an assertion here.
#[test]
fn the_family_opens_for_the_first_time_on_2025_06_29() {
    let before = hours_at((2025, 6, 28));
    let launch = hours_at((2025, 6, 29));

    // Before the launch day the family has no session of any kind, not even a
    // queue: the profile is an explicit closure, not an absent one.
    for (date, time) in [
        ((2025, 6, 29), (16, 0, 0)),
        ((2025, 6, 29), (17, 0, 0)),
        ((2025, 6, 30), (12, 0, 0)),
    ] {
        let instant = ct(date, time);
        assert!(
            !before.is_accepting_orders(instant),
            "{instant}: the family did not exist before 2025-06-29"
        );
    }
    assert!(before.regular.is_empty() && before.extended.is_empty());

    // From the launch day the published grid applies in full, queue included.
    assert!(!launch.is_accepting_orders(ct((2025, 6, 29), (15, 59, 59))));
    assert!(launch.is_order_entry_only(ct((2025, 6, 29), (16, 0, 0))));
    assert!(!launch.is_open(ct((2025, 6, 29), (16, 59, 59))));
    assert!(launch.is_open(ct((2025, 6, 29), (17, 0, 0))));
    assert!(launch.is_open(ct((2025, 6, 30), (15, 59, 59))));
    assert!(
        !launch.is_open(ct((2025, 6, 30), (16, 0, 0))),
        "the first session closes 16:00 CT end-exclusive on the stated trade date"
    );

    // The dated calendar agrees across the boundary rather than only the
    // snapshot: the first open it can find after the launch-eve Saturday is
    // the sourced Sunday-evening one.
    assert_eq!(
        calendar_for_market_hours_key(SPOT_QUOTED)
            .next_session_open_after(ct((2025, 6, 28), (12, 0, 0))),
        Some(ct((2025, 6, 29), (17, 0, 0))),
    );
}

/// QSOL and QXRP joined on 2025-12-14 with a byte-identical hours cell, so the
/// family clock does not move: that listing is caller catalog data, exactly as
/// `MKC`'s 2014 listing is for the mini grains. The fence is that nothing in
/// the profile changes across it in either direction.
#[test]
fn the_december_2025_member_listing_is_not_a_clock_revision() {
    let before = hours_at((2025, 12, 13));
    let after = hours_at((2025, 12, 14));
    let later = hours_at((2025, 12, 15));

    for probe in [
        ((2025, 12, 14), (15, 59, 59)),
        ((2025, 12, 14), (16, 0, 0)),
        ((2025, 12, 14), (16, 30, 0)),
        ((2025, 12, 14), (17, 0, 0)),
        ((2025, 12, 15), (12, 0, 0)),
        ((2025, 12, 15), (16, 45, 0)),
    ] {
        let instant = ct(probe.0, probe.1);
        assert_eq!(
            before.is_open(instant),
            after.is_open(instant),
            "{instant}: SER-9630RR adds products, not hours"
        );
        assert_eq!(
            before.is_accepting_orders(instant),
            later.is_accepting_orders(instant),
            "{instant}: the queue is unchanged by the SOL/XRP listing too"
        );
    }
}

/// Envelope match is not family identity, and this key shares its envelope
/// with `globex_weather` while sharing none of its history. Weather closed
/// 15:15 CT from the January-2010 floor until 2025-04-13; spot-quoted did not
/// exist until 2025-06-29, ten weeks later.
#[test]
fn the_weather_envelope_match_hides_two_unrelated_histories() {
    // 2025-03-17 is a Monday under weather's old grid. Weather is trading;
    // spot-quoted does not exist.
    let early = ct((2025, 3, 17), (12, 0, 0));
    assert!(hours_for_market_hours_key(WEATHER, early).is_open(early));
    assert!(!hours_for_market_hours_key(SPOT_QUOTED, early).is_open(early));
    let early_1530 = ct((2025, 3, 17), (15, 30, 0));
    assert!(
        !hours_for_market_hours_key(WEATHER, early_1530).is_open(early_1530),
        "weather closed 15:15 CT before its 2025-04-13 revision"
    );

    // After weather's revision and before this family's launch the two still
    // disagree: weather now runs to 16:00, spot-quoted still has no session.
    let between = ct((2025, 4, 14), (15, 30, 0));
    assert!(hours_for_market_hours_key(WEATHER, between).is_open(between));
    assert!(!hours_for_market_hours_key(SPOT_QUOTED, between).is_open(between));

    // From weather's 2026-09-05 review row the two are indistinguishable on
    // every public surface across a full week — which is what makes the second
    // key necessary rather than redundant.
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
            let spot_quoted = hours_for_market_hours_key(SPOT_QUOTED, instant);
            let weather = hours_for_market_hours_key(WEATHER, instant);
            assert_eq!(
                spot_quoted.is_open(instant),
                weather.is_open(instant),
                "{instant}: converged envelopes must agree with globex_weather on is_open"
            );
            assert_eq!(
                spot_quoted.is_accepting_orders(instant),
                weather.is_accepting_orders(instant),
                "{instant}: converged envelopes must agree with globex_weather on \
                 order acceptance"
            );
        }
    }

    // They are not even identical for the whole overlap: weather's Sunday
    // queue onset is undated, so its dated profiles serve only the sourced
    // 16:15 intersection until 2026-09-05, while SER-9506R sources 16:00 for
    // this family from its launch day.
    let sunday_1605 = ct((2025, 8, 17), (16, 5, 0));
    assert!(hours_for_market_hours_key(SPOT_QUOTED, sunday_1605).is_accepting_orders(sunday_1605));
    assert!(!hours_for_market_hours_key(WEATHER, sunday_1605).is_accepting_orders(sunday_1605));
}

/// The two folds CME's own documents refuse. `globex_equity_index` publishes
/// an 08:30-15:15 CT regular session that no spot-quoted document states, so
/// the two families answer differently even where their `is_open` envelopes
/// coincide; `globex_cryptocurrency` moved to 24/7 on 2026-05-29 and CME kept
/// spot-quoted on the five-day grid by name.
#[test]
fn neither_the_equity_index_nor_the_cryptocurrency_key_can_stand_in() {
    // Same envelope, different classification. Folding would assert an RTH
    // split for QSPX and its three neighbours out of nothing.
    let midday = ct((2026, 9, 14), (12, 0, 0));
    let calendar = calendar_for_market_hours_key(SPOT_QUOTED);
    assert!(hours_for_market_hours_key(EQUITY_INDEX, midday).is_open(midday));
    assert!(hours_for_market_hours_key(SPOT_QUOTED, midday).is_open(midday));
    assert_eq!(
        calendar_for_market_hours_key(EQUITY_INDEX).session_state(midday),
        SessionState::OpenRegular,
    );
    assert_eq!(calendar.session_state(midday), SessionState::OpenExtended);

    // The equity-index history is inapplicable as well: it was trading a
    // sourced grid a decade before this family was listed.
    let in_2015 = ct((2015, 6, 15), (12, 0, 0));
    assert!(hours_for_market_hours_key(EQUITY_INDEX, in_2015).is_open(in_2015));
    assert!(!hours_for_market_hours_key(SPOT_QUOTED, in_2015).is_open(in_2015));

    // Cryptocurrency runs through the weekend and through the 16:00 CT daily
    // turn; spot-quoted does neither.
    for (date, time) in [((2026, 9, 19), (12, 0, 0)), ((2026, 9, 14), (16, 30, 0))] {
        let instant = ct(date, time);
        assert!(
            hours_for_market_hours_key(CRYPTOCURRENCY, instant).is_open(instant),
            "{instant}: the cryptocurrency family trades 24/7 from 2026-05-29"
        );
        assert!(
            !hours_for_market_hours_key(SPOT_QUOTED, instant).is_open(instant),
            "{instant}: CME kept spot-quoted futures on the five-day schedule"
        );
    }
}

/// The leg is quoted in Central wall-clock, so it survives both DST
/// transitions with its endpoints intact and its elapsed duration changing
/// underneath.
#[test]
fn the_wrapping_leg_survives_both_dst_transitions() {
    // Spring forward: 2026-03-08 is the second Sunday of March.
    let spring = hours_for_market_hours_key(SPOT_QUOTED, ct((2026, 3, 9), (12, 0, 0)));
    assert!(spring.is_open(ct((2026, 3, 8), (17, 0, 0))));
    assert!(spring.is_open(ct((2026, 3, 9), (1, 30, 0))));
    assert!(spring.is_open(ct((2026, 3, 9), (15, 59, 59))));
    assert!(!spring.is_open(ct((2026, 3, 9), (16, 0, 0))));

    // Fall back: 2026-11-01 is the first Sunday of November.
    let fall = hours_for_market_hours_key(SPOT_QUOTED, ct((2026, 11, 2), (12, 0, 0)));
    assert!(fall.is_open(ct((2026, 11, 1), (17, 0, 0))));
    assert!(fall.is_open(ct((2026, 11, 2), (2, 30, 0))));
    assert!(fall.is_open(ct((2026, 11, 2), (15, 59, 59))));
    assert!(!fall.is_open(ct((2026, 11, 2), (16, 0, 0))));
}

/// Holiday behaviour stays caller-owned. CME publishes different half-day
/// closes for the equity and cryptocurrency spot-quoted roots — 12:00 against
/// 13:45 CT on 2026-11-27 — which is precisely the kind of single-trade-date
/// event that belongs in a `DayPolicy` overlay rather than in this profile. A
/// closed trade date removes the whole trading day including the prior
/// evening's wrap, and an early final close clips the wrapping session without
/// touching the queue that precedes it.
#[test]
fn day_policy_overlays_a_closed_date_and_a_subgroup_early_close() {
    let closed_tuesday = day((2026, 9, 15));
    let early_wednesday = day((2026, 9, 16));
    let records = [
        DayOverride::closed(closed_tuesday),
        DayOverride::early_close(early_wednesday, 12 * 3_600),
    ];
    let policy = StaticDayPolicy::new(&records).expect("the fixture records must be valid");
    let calendar = calendar_for_market_hours_key(SPOT_QUOTED).with_day_policy(&policy);
    let plain = calendar_for_market_hours_key(SPOT_QUOTED);

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
fn spot_quoted_key_round_trips_through_its_canonical_name() {
    assert_eq!(SPOT_QUOTED.as_str(), "globex_spot_quoted");
    assert_eq!(SPOT_QUOTED.to_string(), "globex_spot_quoted");
    assert_eq!(
        "globex_spot_quoted".parse::<MarketHoursKey>(),
        Ok(SPOT_QUOTED)
    );
    assert_eq!(
        serde_json::to_string(&SPOT_QUOTED).expect("key serializes"),
        "\"globex_spot_quoted\""
    );
    assert_eq!(
        serde_json::from_str::<MarketHoursKey>("\"globex_spot_quoted\"").expect("key deserializes"),
        SPOT_QUOTED
    );
    for rejected in [
        "globex_spot_quoted_equity",
        "globex_spot_quoted_crypto",
        "spot_quoted",
    ] {
        assert!(
            rejected.parse::<MarketHoursKey>().is_err(),
            "{rejected}: a near-miss name must be rejected, never mapped to the \
             nearest family"
        );
    }
}
