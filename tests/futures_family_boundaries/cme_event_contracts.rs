// SPDX-License-Identifier: MIT-0

//! CME Group Event Contracts (`globex_event_contracts`): the published grid,
//! the 2022-09-18 launch fence, the closed era before it, the finding that a
//! contract's Termination of Trading time is not a session close, and the
//! separations from the families this key is most likely to be confused with.
//!
//! Every probe is stated in America/Chicago wall-clock and converted, so a DST
//! slip in either direction fails rather than passing on a coincidence.
//! 2026-09-13 is a Sunday and 2026-09-18 the Friday of the same week, late
//! enough that `globex_weather`'s 2026-09-05 knowledge-bound row is in force
//! too, so all three converged keys can be compared on equal terms.
//! 2022-09-18 is the Sunday on which CME SER-8968R takes effect, for trade
//! date Monday 2022-09-19; 2023-03-12 is SER-9092's Sunday for `ECBTC`.

use chrono::{DateTime, NaiveDate, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarQueryError, CalendarResolution, DayOverride, MarketHoursKey, SessionState,
    StaticDayPolicy, calendar_for_market_hours_key, candle_end, hours_for_market_hours_key,
    next_session_open_after, session_bounds, session_profile,
};

const EVENT_CONTRACTS: MarketHoursKey = MarketHoursKey::GlobexEventContracts;
const SPOT_QUOTED: MarketHoursKey = MarketHoursKey::GlobexSpotQuoted;
const WEATHER: MarketHoursKey = MarketHoursKey::GlobexWeather;
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
    hours_for_market_hours_key(EVENT_CONTRACTS, ct(date, (0, 0, 0)))
}

/// The state the **fixed snapshot** reports, which is the surface these grid
/// assertions are about.
///
/// `GlobexEventContracts` is dormant (LAW-SERVICE-TIERS): CME publishes no
/// holiday table in the crate for the event-contract family, so its date-aware
/// coverage is empty above the 2025 floor and the calendar-backed surface
/// refuses every date rather than answering. The published grid is still a
/// sourced fact and `hours_for_market_hours_key` states it, so the boundary and
/// classification assertions below keep their force through that surface;
/// `assert_refused` is how the calendar-backed surface is asserted to refuse
/// the same dates.
fn state_at(instant: DateTime<Utc>) -> SessionState {
    hours_for_market_hours_key(EVENT_CONTRACTS, instant).session_state(instant)
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

/// The published current grid, boundary by boundary. SER-9624 gives "CME
/// Globex Pre-Open: Sunday 5:00 p.m. - 6:00 p.m. Eastern Time/ET / Monday -
/// Thursday 5:45 p.m. - 6:00 p.m. ET // CME Globex: Sunday 6:00 p.m. - Friday
/// 5:00 p.m. ET with a daily maintenance period from 5:00 p.m. - 6:00 p.m.
/// ET", and SER-9740R gives the same cell in Central time for `ECBTC`.
///
/// Each open is fenced by the second before it, and each close is asserted
/// end-exclusive.
#[test]
fn event_contracts_serve_the_published_grid_with_end_exclusive_closes() {
    let hours = hours_for_market_hours_key(EVENT_CONTRACTS, ct((2026, 9, 14), (12, 0, 0)));

    // Sunday queue: nothing is accepted at 15:59:59, everything queues from
    // 16:00, and nothing matches until 17:00.
    assert!(!hours.is_accepting_orders(ct((2026, 9, 13), (15, 59, 59))));
    assert!(hours.is_order_entry_only(ct((2026, 9, 13), (16, 0, 0))));
    assert!(hours.is_order_entry_only(ct((2026, 9, 13), (16, 59, 59))));
    assert!(!hours.is_open(ct((2026, 9, 13), (16, 59, 59))));
    assert!(hours.is_open(ct((2026, 9, 13), (17, 0, 0))));

    // Everything executable is extended. No CME document about this family
    // splits its session into RTH and ETH, so `regular` is empty.
    assert_eq!(
        state_at(ct((2026, 9, 13), (17, 0, 0))),
        SessionState::OpenExtended,
    );
    assert!(!hours.is_open_regular(ct((2026, 9, 14), (12, 0, 0))));
    assert!(session_profile(EVENT_CONTRACTS).regular.is_empty());

    // The leg wraps local midnight and closes 16:00 end-exclusive.
    assert!(hours.is_open(ct((2026, 9, 14), (3, 0, 0))));
    assert!(hours.is_open(ct((2026, 9, 14), (15, 59, 59))));
    assert!(!hours.is_open(ct((2026, 9, 14), (16, 0, 0))));

    // The 60-minute maintenance period CME names in words: nothing matches,
    // and for its first 45 minutes nothing queues either. The date-aware
    // surface refuses the day, so the classification is read from the fixed
    // snapshot, which reports the same one.
    assert_refused(
        calendar_for_market_hours_key(EVENT_CONTRACTS)
            .is_maintenance(ct((2026, 9, 14), (16, 30, 0))),
        "the maintenance probe",
    );
    assert!(hours.is_maintenance(ct((2026, 9, 14), (16, 30, 0))));
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

    // The static current table must state the same grid as the instant selector.
    let profile = session_profile(EVENT_CONTRACTS);
    assert!(profile.is_order_entry_only(ct((2026, 9, 13), (16, 0, 0))));
    assert!(profile.is_open(ct((2026, 9, 13), (17, 0, 0))));
    assert!(profile.is_open(ct((2026, 9, 14), (15, 59, 59))));
    assert!(!profile.is_open(ct((2026, 9, 14), (16, 0, 0))));
    assert!(profile.is_order_entry_only(ct((2026, 9, 14), (16, 45, 0))));
    assert!(profile.regular.is_empty());
}

/// **The finding, pinned.** CME publishes a different Termination of Trading
/// time for every root — 15:00 CT for the equity indices, 14:00 for `EC6E`,
/// 13:30 for `ECCL`/`ECNG`, 12:30 for `ECGC`, 12:25 for `ECSI`, 12:00 for
/// `ECHG`, and 09:00/10:00/12:00/14:00/15:00 CT for the hourly Chapter 23A
/// contracts — against **one** Trading Hours cell whose only daily boundary is
/// the 16:00 CT maintenance period. Termination is the contract's stated
/// expiration time, so the family is still open at each of those instants.
///
/// If a future edit ever encodes one of them as a session close, this test is
/// the one that fails, and it fails in both the current era and the launch era
/// so a mis-dated "fix" cannot hide in one of them.
#[test]
fn a_termination_of_trading_time_is_not_a_session_close() {
    // The six Chapter 23 termination times and the five Chapter 23A ones,
    // stated in Central time. 09:00 and 10:00 are the hourly contracts' 10:00
    // and 11:00 ET values; 12:00, 14:00 and 15:00 CT are shared between the
    // two chapters.
    const TERMINATION_TIMES: [(u32, u32); 8] = [
        (9, 0),
        (10, 0),
        (12, 0),
        (12, 25),
        (12, 30),
        (13, 30),
        (14, 0),
        (15, 0),
    ];

    for era in [(2023, 6, 14), (2026, 9, 15)] {
        let hours = hours_for_market_hours_key(EVENT_CONTRACTS, ct(era, (12, 0, 0)));
        for (hour, minute) in TERMINATION_TIMES {
            let instant = ct(era, (hour, minute, 0));
            assert!(
                hours.is_open(instant),
                "{instant}: a contract's termination time is its stated expiration, \
                 not this family's session close"
            );
        }
        assert!(
            hours.is_open(ct(era, (15, 59, 59))),
            "the session runs to the second before the maintenance period"
        );
        assert!(
            !hours.is_open(ct(era, (16, 0, 0))),
            "the only daily close is CME's 16:00 CT maintenance boundary"
        );
    }

    // Friday is the same statement in the other direction: the per-root
    // "Sunday 5:00 p.m. - Friday 12:00 p.m." style cells end at that week's
    // last expiry, and the week's executable session runs on past all of them
    // to the same 16:00 CT close.
    let friday = hours_for_market_hours_key(EVENT_CONTRACTS, ct((2026, 9, 18), (12, 0, 0)));
    for (hour, minute) in TERMINATION_TIMES {
        let instant = ct((2026, 9, 18), (hour, minute, 0));
        assert!(
            friday.is_open(instant),
            "{instant}: the week's executable session outruns every root's expiry"
        );
    }
    assert!(!friday.is_open(ct((2026, 9, 18), (16, 0, 0))));
}

/// The wrapping leg takes the trade date of its close, so one daily bar runs
/// Sunday 17:00 CT to Monday 16:00 CT — which is also what CME's own
/// "Next day's Event Contract will list at 5:00 p.m." implies: the evening
/// queue belongs to the session that settles the following day.
#[test]
fn the_wrapping_leg_takes_its_closing_trade_date() {
    let calendar = calendar_for_market_hours_key(EVENT_CONTRACTS);
    let hours = hours_for_market_hours_key(EVENT_CONTRACTS, ct((2026, 9, 13), (20, 0, 0)));

    // The identity is dormant, so its date-aware surface refuses to name a
    // trade date, a daily bar or a session. The wrap's shape is still a sourced
    // fact, and the fixed snapshot states it: one daily bar runs Sunday 17:00
    // CT to Monday 16:00 CT, and that close's venue-local date is the trade
    // date the calendar would have named.
    assert_refused(
        calendar.trade_date(ct((2026, 9, 13), (20, 0, 0))),
        "the Sunday-evening leg",
    );
    assert_refused(
        calendar.candle_end(ct((2026, 9, 13), (20, 0, 0)), CalendarResolution::Daily),
        "the Sunday-evening daily bar",
    );
    assert_eq!(
        candle_end(
            &hours,
            ct((2026, 9, 13), (20, 0, 0)),
            CalendarResolution::Daily
        ),
        Some(ct((2026, 9, 14), (16, 0, 0))),
        "the Sunday-evening leg wraps, so it belongs to Monday's trade date"
    );
    assert_refused(
        calendar.trade_date(ct((2026, 9, 14), (12, 0, 0))),
        "Monday midday",
    );
    assert_eq!(
        candle_end(
            &hours,
            ct((2026, 9, 14), (12, 0, 0)),
            CalendarResolution::Daily
        ),
        Some(ct((2026, 9, 14), (16, 0, 0))),
    );
    // Monday's evening queue feeds the session that closes Tuesday, so the
    // queue's own daily bar ends on Tuesday's 16:00 CT close.
    assert_refused(
        calendar.trade_date(ct((2026, 9, 14), (16, 45, 0))),
        "Monday's evening queue",
    );
    assert_eq!(
        candle_end(
            &hours,
            ct((2026, 9, 14), (16, 45, 0)),
            CalendarResolution::Daily
        ),
        Some(ct((2026, 9, 15), (16, 0, 0))),
        "Monday's evening queue feeds the session that closes Tuesday"
    );
    assert_refused(
        calendar.session_bounds(ct((2026, 9, 14), (12, 0, 0))),
        "Monday's session bounds",
    );
    assert_eq!(
        session_bounds(&hours, ct((2026, 9, 14), (12, 0, 0))),
        Some((ct((2026, 9, 13), (17, 0, 0)), ct((2026, 9, 14), (16, 0, 0)))),
    );
}

/// The week ends at the Friday 16:00 CT close: no Friday-evening reopen and no
/// Friday-evening queue, and the next open is Sunday 17:00 CT. CME's Pre-Open
/// row names Sunday and Monday-Thursday only, which is the sourced absence
/// this asserts.
#[test]
fn the_week_reopens_on_sunday_evening_with_no_friday_evening_leg() {
    let calendar = calendar_for_market_hours_key(EVENT_CONTRACTS);
    let hours = hours_for_market_hours_key(EVENT_CONTRACTS, ct((2026, 9, 18), (12, 0, 0)));

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
    // The forward scan is a date-aware query, so the dormant identity refuses
    // it rather than reporting the next session; the fixed snapshot still
    // states that next open.
    assert_refused(
        calendar.next_session_open_after(ct((2026, 9, 18), (16, 0, 0))),
        "the weekend forward scan",
    );
    assert_eq!(
        next_session_open_after(&hours, ct((2026, 9, 18), (16, 0, 0))),
        Some(ct((2026, 9, 20), (17, 0, 0))),
        "the week reopens on Sunday evening, not Friday evening"
    );
}

/// The launch fence. CME SER-8968R: "Effective Sunday, September 18, 2022, for
/// trade date Monday, September 19, 2022 ... will list Event Contracts
/// ("Event Contracts") on certain CME Group futures contracts as listed in
/// Table 1. for trading on the CME Globex electronic trading platform".
///
/// The row is keyed to the venue-local **opening** day, not the stated trade
/// date, because the first session that exists at all is the one opening
/// Sunday 2022-09-18 at 17:00 CT (queueing from 16:00 CT) and closing on
/// Monday 2022-09-19. Both sides are evaluated at venue-local midnight, so a
/// revision keyed one day early or late flips an assertion here.
#[test]
fn the_family_opens_for_the_first_time_on_2022_09_18() {
    let before = hours_at((2022, 9, 17));
    let launch = hours_at((2022, 9, 18));

    // Before the launch day the family has no session of any kind, not even a
    // queue: the profile is an explicit closure, not an absent one.
    for (date, time) in [
        ((2022, 9, 18), (16, 0, 0)),
        ((2022, 9, 18), (17, 0, 0)),
        ((2022, 9, 19), (12, 0, 0)),
    ] {
        let instant = ct(date, time);
        assert!(
            !before.is_accepting_orders(instant),
            "{instant}: the family did not exist before 2022-09-18"
        );
    }
    assert!(before.regular.is_empty() && before.extended.is_empty());

    // From the launch day the published grid applies in full, queue included.
    assert!(!launch.is_accepting_orders(ct((2022, 9, 18), (15, 59, 59))));
    assert!(launch.is_order_entry_only(ct((2022, 9, 18), (16, 0, 0))));
    assert!(!launch.is_open(ct((2022, 9, 18), (16, 59, 59))));
    assert!(launch.is_open(ct((2022, 9, 18), (17, 0, 0))));
    assert!(launch.is_open(ct((2022, 9, 19), (15, 59, 59))));
    assert!(
        !launch.is_open(ct((2022, 9, 19), (16, 0, 0))),
        "the first session closes 16:00 CT end-exclusive on the stated trade date"
    );

    // The dated calendar refuses the launch-eve Saturday rather than scanning
    // forward for a session: the identity claims no coverage, so "the first
    // open it can find" is not an answer it gives. The launch boundary itself
    // stays fenced by the snapshot probes above — the eve accepts nothing and
    // the launch evening answers the published grid in full.
    assert_refused(
        calendar_for_market_hours_key(EVENT_CONTRACTS)
            .next_session_open_after(ct((2022, 9, 17), (12, 0, 0))),
        "the launch-eve forward scan",
    );
}

/// `ECBTC` joined on 2023-03-12 with a byte-identical hours cell — SER-9092's
/// Exhibit 1 reprints "CME Globex Pre-Open: Sunday 4:00 - 5:00 p.m. Monday -
/// Thursday 4:45 - 5:00 p.m. / CME Globex: Sunday 5:00 p.m.- Friday 3:00 p.m."
/// — so the family clock does not move and that listing is caller catalog
/// data, exactly as `MKC`'s 2014 listing is for the mini grains and
/// `QSOL`/`QXRP`'s 2025 listing is for spot-quoted. The fence is that nothing
/// in the profile changes across it in either direction.
#[test]
fn the_march_2023_bitcoin_listing_is_not_a_clock_revision() {
    let before = hours_at((2023, 3, 11));
    let after = hours_at((2023, 3, 12));
    let later = hours_at((2023, 3, 13));

    for probe in [
        ((2023, 3, 12), (15, 59, 59)),
        ((2023, 3, 12), (16, 0, 0)),
        ((2023, 3, 12), (16, 30, 0)),
        ((2023, 3, 12), (17, 0, 0)),
        ((2023, 3, 13), (12, 0, 0)),
        ((2023, 3, 13), (15, 0, 0)),
        ((2023, 3, 13), (16, 45, 0)),
    ] {
        let instant = ct(probe.0, probe.1);
        assert_eq!(
            before.is_open(instant),
            after.is_open(instant),
            "{instant}: SER-9092 adds a product, not hours"
        );
        assert_eq!(
            before.is_accepting_orders(instant),
            later.is_accepting_orders(instant),
            "{instant}: the queue is unchanged by the ECBTC listing too"
        );
    }
}

/// `ECBTC` left this family on 2026-05-29 and nothing in the key moved with
/// it: SER-9740R expanded that one root to 24/7 while CME's client-systems
/// wiki kept the rest — "Other event contracts will continue on the current
/// schedule." The key must therefore answer the *unchanged* five-day grid on
/// both sides of that Friday. A caller trading `ECBTC` past it has left this
/// key's documented scope, which is a catalog obligation rather than a
/// profile one.
#[test]
fn the_bitcoin_root_leaving_scope_does_not_move_this_key() {
    let before = hours_at((2026, 5, 28));
    let after = hours_at((2026, 5, 30));

    for probe in [
        ((2026, 5, 29), (12, 0, 0)),
        ((2026, 5, 29), (15, 0, 0)),
        ((2026, 5, 29), (15, 59, 59)),
        ((2026, 5, 29), (16, 0, 0)),
        ((2026, 5, 30), (12, 0, 0)),
        ((2026, 5, 31), (17, 0, 0)),
    ] {
        let instant = ct(probe.0, probe.1);
        assert_eq!(
            before.is_open(instant),
            after.is_open(instant),
            "{instant}: the 24/7 expansion was scoped to ECBTC alone"
        );
    }

    // The weekend after the expansion is still closed on this key, and the
    // cryptocurrency key — which ECBTC's own migration document expands — is
    // not. That contrast is the reason the root leaves scope rather than the
    // key changing shape.
    let saturday = ct((2026, 5, 30), (12, 0, 0));
    assert!(!hours_for_market_hours_key(EVENT_CONTRACTS, saturday).is_open(saturday));
    assert!(hours_for_market_hours_key(CRYPTOCURRENCY, saturday).is_open(saturday));
    assert!(
        hours_for_market_hours_key(MarketHoursKey::GlobexEventContractsBtc, saturday)
            .is_open(saturday),
        "the departed root is served by its own key from that day"
    );
}

/// Envelope match is not family identity, and this key now shares its envelope
/// with **two** others. All three agree across a full current week and none of
/// their histories overlaps: weather closed 15:15 CT until 2025-04-13,
/// spot-quoted did not exist until 2025-06-29, and this family has run one
/// grid since 2022-09-18.
#[test]
fn three_keys_share_one_envelope_today_and_no_history_at_all() {
    // A single midweek instant in 2023 separates all three: event contracts
    // are trading, weather closed at 15:15, spot-quoted does not exist.
    let midweek_2023 = ct((2023, 6, 14), (15, 30, 0));
    assert!(hours_for_market_hours_key(EVENT_CONTRACTS, midweek_2023).is_open(midweek_2023));
    assert!(!hours_for_market_hours_key(WEATHER, midweek_2023).is_open(midweek_2023));
    assert!(!hours_for_market_hours_key(SPOT_QUOTED, midweek_2023).is_open(midweek_2023));

    // In 2015 weather is the only one of the three that exists at all.
    let in_2015 = ct((2015, 6, 15), (12, 0, 0));
    assert!(hours_for_market_hours_key(WEATHER, in_2015).is_open(in_2015));
    assert!(!hours_for_market_hours_key(EVENT_CONTRACTS, in_2015).is_open(in_2015));
    assert!(!hours_for_market_hours_key(SPOT_QUOTED, in_2015).is_open(in_2015));

    // The queues separate them too. SER-8968R sources the Sunday 16:00 queue
    // on this family's launch day, while weather's Sunday onset is undated and
    // its dated profiles serve only the 16:15 intersection until 2026-09-05.
    let sunday_1605 = ct((2025, 8, 17), (16, 5, 0));
    assert!(
        hours_for_market_hours_key(EVENT_CONTRACTS, sunday_1605).is_accepting_orders(sunday_1605)
    );
    assert!(!hours_for_market_hours_key(WEATHER, sunday_1605).is_accepting_orders(sunday_1605));

    // From weather's 2026-09-05 review row the three are indistinguishable on
    // every public surface across a full week — which is what makes the third
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
            let event_contracts = hours_for_market_hours_key(EVENT_CONTRACTS, instant);
            for other in [SPOT_QUOTED, WEATHER] {
                let other_hours = hours_for_market_hours_key(other, instant);
                assert_eq!(
                    event_contracts.is_open(instant),
                    other_hours.is_open(instant),
                    "{instant}: converged envelopes must agree with {} on is_open",
                    other.as_str()
                );
                assert_eq!(
                    event_contracts.is_accepting_orders(instant),
                    other_hours.is_accepting_orders(instant),
                    "{instant}: converged envelopes must agree with {} on order acceptance",
                    other.as_str()
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
    let spring = hours_for_market_hours_key(EVENT_CONTRACTS, ct((2026, 3, 9), (12, 0, 0)));
    assert!(spring.is_open(ct((2026, 3, 8), (17, 0, 0))));
    assert!(spring.is_open(ct((2026, 3, 9), (1, 30, 0))));
    assert!(spring.is_open(ct((2026, 3, 9), (15, 59, 59))));
    assert!(!spring.is_open(ct((2026, 3, 9), (16, 0, 0))));

    // Fall back: 2026-11-01 is the first Sunday of November.
    let fall = hours_for_market_hours_key(EVENT_CONTRACTS, ct((2026, 11, 2), (12, 0, 0)));
    assert!(fall.is_open(ct((2026, 11, 1), (17, 0, 0))));
    assert!(fall.is_open(ct((2026, 11, 2), (2, 30, 0))));
    assert!(fall.is_open(ct((2026, 11, 2), (15, 59, 59))));
    assert!(!fall.is_open(ct((2026, 11, 2), (16, 0, 0))));
}

/// Holiday behaviour stays caller-owned. CME treats event contracts as their
/// own holiday category — SER-9499R closes "All U.S.-based equity Event
/// Contracts" on 2025-01-09 while "All other Event Contracts will have a
/// normal trading day" — which is precisely the kind of single-trade-date
/// event that belongs in a `DayPolicy` overlay under LAW-HOLIDAY-SCOPE rather
/// than in this profile. A closed trade date removes the whole trading day
/// including the prior evening's wrap, and an early final close clips the
/// wrapping session without touching the queue that precedes it.
#[test]
fn day_policy_overlays_a_closed_date_and_an_early_close() {
    let closed_tuesday = day((2026, 9, 15));
    let early_wednesday = day((2026, 9, 16));
    let records = [
        DayOverride::closed(closed_tuesday),
        DayOverride::early_close(early_wednesday, 12 * 3_600),
    ];
    let policy = StaticDayPolicy::new(&records).expect("the fixture records must be valid");
    let calendar = calendar_for_market_hours_key(EVENT_CONTRACTS).with_day_policy(&policy);
    let plain = calendar_for_market_hours_key(EVENT_CONTRACTS);

    // The overlay mechanism itself is exercised on served identities elsewhere
    // (`tests/static_day_policy.rs`, `tests/calendar_policies.rs`). What this
    // identity can state is the negative: `GlobexEventContracts` is dormant, so
    // neither the bare calendar nor the same calendar carrying a caller's
    // `DayPolicy` has a sourced answer, and a policy — which can only tighten a
    // day — never manufactures coverage the identity does not claim
    // (LAW-COVERAGE).
    for (date, time) in [
        ((2026, 9, 14), (20, 0, 0)),
        ((2026, 9, 15), (12, 0, 0)),
        ((2026, 9, 15), (20, 0, 0)),
        ((2026, 9, 16), (11, 59, 59)),
        ((2026, 9, 16), (12, 0, 0)),
    ] {
        let instant = ct(date, time);
        assert_refused(plain.is_open(instant), &format!("{instant} bare"));
        assert_refused(calendar.is_open(instant), &format!("{instant} overlaid"));
    }
    assert_refused(
        calendar.session_bounds(ct((2026, 9, 16), (10, 0, 0))),
        "the overlaid boundary query",
    );

    // The grid the overlay would have modified is still stated by the fixed
    // snapshot: Monday's evening leg and Tuesday's session are open, and
    // Wednesday's session runs past the 12:00 final close the overlay names —
    // which is what makes that close the caller's, not the profile's.
    for (date, time) in [
        ((2026, 9, 14), (20, 0, 0)),
        ((2026, 9, 15), (12, 0, 0)),
        ((2026, 9, 15), (20, 0, 0)),
        ((2026, 9, 16), (11, 59, 59)),
        ((2026, 9, 16), (12, 0, 0)),
    ] {
        let instant = ct(date, time);
        assert!(
            hours_for_market_hours_key(EVENT_CONTRACTS, instant).is_open(instant),
            "{instant}: open on the normal-week grid"
        );
    }
}

/// The wire identity round-trips through every public spelling.
#[test]
fn event_contracts_key_round_trips_through_its_canonical_name() {
    assert_eq!(EVENT_CONTRACTS.as_str(), "globex_event_contracts");
    assert_eq!(EVENT_CONTRACTS.to_string(), "globex_event_contracts");
    assert_eq!(
        "globex_event_contracts".parse::<MarketHoursKey>(),
        Ok(EVENT_CONTRACTS)
    );
    assert_eq!(
        serde_json::to_string(&EVENT_CONTRACTS).expect("key serializes"),
        "\"globex_event_contracts\""
    );
    assert_eq!(
        serde_json::from_str::<MarketHoursKey>("\"globex_event_contracts\"")
            .expect("key deserializes"),
        EVENT_CONTRACTS
    );
    // `globex_event_contracts_btc` is no longer a near-miss: it resolves to the
    // departed root's own key, and its own suite fences that round-trip.
    for rejected in ["globex_event_contracts_hourly", "event_contracts"] {
        assert!(
            rejected.parse::<MarketHoursKey>().is_err(),
            "{rejected}: a near-miss name must be rejected, never mapped to the \
             nearest family"
        );
    }
}
