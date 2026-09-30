// SPDX-License-Identifier: MIT-0

//! CME family queues and phase machines: snapshot agreement, overnight
//! order-entry gaps, the carried-back Sunday pre-open, and the livestock queue.

use super::prelude::*;

/// Every new key must expose a current snapshot without panicking, and the
/// snapshot must agree with the dated selector at a present-day instant.
#[test]
fn current_snapshots_agree_with_dated_selectors_today() {
    let now = utc(2026, 6, 17, 12, 0);
    for key in MarketHoursKey::ALL {
        let snapshot = hours_for_market_hours_key(
            *key,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
        );
        let dated = hours_for_market_hours_key(*key, now);
        assert_eq!(
            snapshot.is_open(now),
            dated.is_open(now),
            "{}: fixed snapshot and dated selector disagree today",
            key.as_str()
        );
    }
}

/// The overnight phase machine classifies by phase kind, not by envelope.
///
/// Eurex fixed income runs pre-trading 02:00-02:10 and post-trading
/// 22:00-22:10 CEST around its 02:10-22:00 continuous session; SGX's
/// Three-Month SORA and Japan equity-index grids open their T sessions at
/// 07:25 and 07:30 SGT behind pre-opening order windows. Each boundary below
/// is derived from those published grids, and the fixed snapshot must answer
/// a current week identically to the dated selector.
#[test]
fn overnight_order_entry_and_closed_gaps_match_the_published_phase_machine() {
    // 2026-04-20 is a Monday; Berlin is CEST (+02:00), Singapore is SGT (+08:00).
    let dated =
        hours_for_market_hours_key(MarketHoursKey::EurexFixedIncome, utc(2026, 4, 20, 12, 0));
    let snapshot = hours_for_market_hours_key(
        MarketHoursKey::EurexFixedIncome,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    for hours in [&dated, &snapshot] {
        // 22:02 CEST Monday: post-trading accepts orders, nothing matches.
        assert_eq!(
            hours.session_state(utc(2026, 4, 20, 20, 2)),
            SessionState::OrderEntry
        );
        // 22:30 CEST Monday and 01:30 CEST Tuesday: the 22:00 -> 02:10
        // matching gap exceeds the four-hour maintenance bound.
        assert_eq!(
            hours.session_state(utc(2026, 4, 20, 20, 30)),
            SessionState::Closed
        );
        assert_eq!(
            hours.session_state(utc(2026, 4, 20, 23, 30)),
            SessionState::Closed
        );
        // 02:05 CEST Tuesday: pre-trading before the 02:10 continuous open.
        assert_eq!(
            hours.session_state(utc(2026, 4, 21, 0, 5)),
            SessionState::OrderEntry
        );
    }

    // 07:10 SGT Monday (23:10 UTC Sunday): SORA's T pre-opening window runs
    // 07:10-07:25, so the market accepts orders but nothing matches.
    for hours in [
        hours_for_market_hours_key(
            MarketHoursKey::Sgx,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
        ),
        hours_for_market_hours_key(MarketHoursKey::Sgx, utc(2026, 4, 20, 12, 0)),
    ] {
        assert_eq!(
            hours.session_state(utc(2026, 4, 19, 23, 10)),
            SessionState::OrderEntry
        );
    }

    // 07:20 SGT Monday: the Japan grid's pre-opening window runs 07:15-07:30
    // ahead of its 07:30 T session.
    let japan = hours_for_market_hours_key(
        MarketHoursKey::SgxEquityIndexJapan,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert_eq!(
        japan.session_state(utc(2026, 4, 19, 23, 20)),
        SessionState::OrderEntry
    );
}

/// The Sunday Pre-Open queue is carried back to the January-2010 floor at its
/// narrowest sourced value, 16:15 CT, because CME's queue only ever widened
/// inside the modelled window (16:15 at the audit floor, 16:00 verified
/// current). These probes fence the intersection on both sides so a future edit
/// cannot silently drop the historical Sunday queue again — the state this
/// suite previously did not cover at all — nor quietly widen it to 16:00 on a
/// dated instant, which would assert the undated 2012 cutover.
///
/// 21:00Z is 16:00 CT and 21:30Z is 16:30 CT on a US summer Sunday.
#[test]
fn sunday_pre_open_carries_back_at_its_narrowest_sourced_edge() {
    const FAMILIES: [MarketHoursKey; 4] = [
        MarketHoursKey::GlobexEquityIndex,
        MarketHoursKey::GlobexEnergy,
        MarketHoursKey::GlobexFx,
        MarketHoursKey::GlobexInterestRates,
    ];

    // Sundays spread across the modelled window, each side of the undated
    // 2012-05-28..2012-06-07 bracket and well beyond it.
    const SUNDAYS: [(i32, u32, u32); 4] = [(2010, 6, 6), (2011, 6, 5), (2015, 6, 7), (2025, 6, 8)];

    for key in FAMILIES {
        for (year, month, day) in SUNDAYS {
            let inside = utc(year, month, day, 21, 30);
            assert_eq!(
                hours_for_market_hours_key(key, inside).session_state(inside),
                SessionState::OrderEntry,
                "{key:?} must queue orders at Sunday 16:30 CT on {year}-{month}-{day}: \
                 16:30 is inside the queue under every sourced Sunday value"
            );

            // 16:00 CT is inside the queue only under the verified-current
            // grid, whose onset day is undated, so a dated instant must not
            // claim it.
            let disputed = utc(year, month, day, 21, 0);
            assert_eq!(
                hours_for_market_hours_key(key, disputed).session_state(disputed),
                SessionState::Closed,
                "{key:?} must not extend the dated Sunday queue to 16:00 CT on \
                 {year}-{month}-{day}: that quarter-hour depends on the undated 2012 cutover"
            );
        }
    }
}

/// The 2026-09-30 wave sourced the equity families' floor-era grid and
/// queues (Globex notices 20090831/0907/0914 for the Sunday queue, the
/// archived equities-hours page of 2009-04-06 for the grid) and the fixed
/// income key's baseline (the operator's archived Contract Specifications
/// amendments of 2009-09-14 .. 2017-08-28), so the horizons moved to the
/// 2010-01-01 floor: the first floor week answers instead of refusing.
///
/// January probes sit in CST (UTC-6) and CET (UTC+1).
#[test]
fn the_floor_week_answers_from_the_sourced_grid() {
    use exchange_hours::{
        DateCoverage, Exchange, calendar_for_exchange, calendar_for_market_hours_key,
    };

    // Monday 2010-01-04, the floor week's first trading day, on the era
    // equity grid: the 17:00->08:30 overnight leg closes at 08:30, the
    // 08:30-15:15 day session trades, the 15:30-16:30 post-halt slice trades,
    // and 17:00 reopens the next leg.
    let monday = [
        (utc(2010, 1, 4, 14, 29), SessionState::OpenExtended),
        (utc(2010, 1, 4, 14, 30), SessionState::OpenRegular),
        (utc(2010, 1, 4, 20, 0), SessionState::OpenRegular),
        (utc(2010, 1, 4, 21, 35), SessionState::OpenExtended),
        (utc(2010, 1, 4, 22, 0), SessionState::OpenExtended),
        (utc(2010, 1, 4, 23, 0), SessionState::OpenExtended),
    ];
    for (instant, state) in monday {
        assert_eq!(
            hours_for_market_hours_key(MarketHoursKey::GlobexEquityIndex, instant)
                .session_state(instant),
            state,
            "globex_equity_index at {instant} on the sourced floor-era grid"
        );
    }
    // Sunday 2010-01-10: the sourced 16:15 CT queue, then the 17:00 CT open.
    assert_eq!(
        hours_for_market_hours_key(MarketHoursKey::GlobexEquityIndex, utc(2010, 1, 10, 22, 30))
            .session_state(utc(2010, 1, 10, 22, 30)),
        SessionState::OrderEntry,
        "the floor-era Sunday 16:15-17:00 CT Pre-Open queues orders"
    );
    assert_eq!(
        hours_for_market_hours_key(MarketHoursKey::GlobexEquityIndex, utc(2010, 1, 10, 23, 0))
            .session_state(utc(2010, 1, 10, 23, 0)),
        SessionState::OpenExtended,
        "the floor-era Sunday 17:00 CT open matches"
    );

    // Eurex fixed income on the sourced 2009-09-14 baseline: Pre-Trading
    // 07:30-08:00 CET queues, Continuous Trading 08:00-22:00 CET matches,
    // Post-Trading until 22:30 CET queues, then the overnight close.
    let baseline = [
        (utc(2010, 1, 4, 6, 30), SessionState::OrderEntry),
        (utc(2010, 1, 4, 8, 0), SessionState::OpenRegular),
        (utc(2010, 1, 4, 12, 0), SessionState::OpenRegular),
        (utc(2010, 1, 4, 21, 15), SessionState::OrderEntry),
        (utc(2010, 1, 4, 22, 0), SessionState::Closed),
    ];
    for (instant, state) in baseline {
        assert_eq!(
            hours_for_market_hours_key(MarketHoursKey::EurexFixedIncome, instant)
                .session_state(instant),
            state,
            "eurex_fixed_income at {instant} on the sourced 2009 baseline"
        );
    }

    // The metadata agrees: the floor day is inside the sourced normal week
    // for all three identities, not a carried refusal.
    let floor_day = chrono::NaiveDate::from_ymd_opt(2010, 1, 4).expect("a valid floor-week day");
    for (name, coverage) in [
        ("cme", calendar_for_exchange(Exchange::Cme).coverage()),
        (
            "globex_equity_index",
            calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex).coverage(),
        ),
        (
            "eurex_fixed_income",
            calendar_for_market_hours_key(MarketHoursKey::EurexFixedIncome).coverage(),
        ),
    ] {
        assert_eq!(
            coverage.coverage_on(floor_day),
            DateCoverage::Covered,
            "{name} must answer the floor week from its sourced grid"
        );
    }
}

/// CME dated the livestock morning Pre-Open moving "from 06:00 to 08:00" on
/// 2020-05-31, which states the outgoing 06:00 value. No source names a cutover
/// between SER-7591's 2016-02-29 grid — the 08:30 open this queue runs into —
/// and that move, so 06:00-08:30 is carried across the interval. It is not
/// carried further back: the pre-2016 around-the-clock grid has no 08:30 open.
///
/// 11:00Z is 06:00 CT and 13:10Z is 08:10 CT on a US summer date.
#[test]
fn livestock_morning_queue_spans_its_sourced_matching_grid() {
    let key = MarketHoursKey::GlobexLivestock;

    let inside = utc(2017, 6, 14, 11, 0);
    assert_eq!(
        hours_for_market_hours_key(key, inside).session_state(inside),
        SessionState::OrderEntry,
        "06:00 CT queues orders between 2016-02-29 and the 2020-05-31 move"
    );

    // After the sourced move the queue starts at 08:00, so 06:00 CT is closed.
    let after = utc(2021, 6, 16, 11, 0);
    assert_eq!(
        hours_for_market_hours_key(key, after).session_state(after),
        SessionState::Closed,
        "SER-8599R moved the start to 08:00 CT, so 06:00 CT must be closed after it"
    );

    // 08:10 CT is inside the queue on both sides of that move.
    for instant in [utc(2017, 6, 14, 13, 10), utc(2021, 6, 16, 13, 10)] {
        assert_eq!(
            hours_for_market_hours_key(key, instant).session_state(instant),
            SessionState::OrderEntry,
            "08:10 CT is inside the morning queue under both sourced starts"
        );
    }

    // The pre-2016 around-the-clock grid keeps no morning queue.
    let old = utc(2013, 6, 12, 11, 0);
    assert_ne!(
        hours_for_market_hours_key(key, old).session_state(old),
        SessionState::OrderEntry,
        "the morning queue is not carried back past the 2016-02-29 grid it belongs to"
    );
}
