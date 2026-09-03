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
