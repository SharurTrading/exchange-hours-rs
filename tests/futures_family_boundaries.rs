// SPDX-License-Identifier: MIT-0

//! Dated-boundary contracts for the product-family keys added in 1.0.
//!
//! Each assertion probes an instant that falls on one side of a sourced cutover
//! and would flip if the revision were mis-keyed or the profile mis-encoded. A
//! 15-minute encoding slip is otherwise invisible to the rest of the suite.

use chrono::{DateTime, TimeZone as _, Utc};
use exchange_hours::{MarketHoursKey, SessionState, hours_for_market_hours_key};

/// Builds a UTC probe instant from literals. UTC has no ambiguous local times,
/// so `single()` always resolves here; the epoch fallback keeps the helper total
/// without a panic path, and any probe that somehow reached it would fail its
/// assertion loudly rather than pass silently.
fn utc(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .unwrap_or(DateTime::UNIX_EPOCH)
}

fn open_at(key: MarketHoursKey, instant: DateTime<Utc>) -> bool {
    hours_for_market_hours_key(key, instant).is_open(instant)
}

/// Regular session only. The ICE families run order-entry phases outside the
/// executable session, so `is_open` would answer true during a pre-open.
fn open_regular_at(key: MarketHoursKey, instant: DateTime<Utc>) -> bool {
    hours_for_market_hours_key(key, instant).is_open_regular(instant)
}

/// CME Nikkei 225 Dollar moved 15:15 CT -> 16:15 CT (2012), kept 16:15 CT after
/// the halt removal (2013), then moved to 16:00 CT (2015-09-20, CME Globex
/// Notice #20150817). 21:10Z is 16:10 CT on a US summer date, so it is inside
/// the session only while the close is 16:15 CT. Before 2012-11-18 the dated
/// route serves the sourced pre-2012 grid — CME's own trading-hours pages give
/// Electronic Trading (Sunday) "17:00-15:15" and (Weekday) "15:30-16:30,
/// 17:00-15:15" — carried back to the January-2010 floor because no primary
/// source names a cutover inside that interval.
#[test]
fn nkd_close_tracks_its_three_sourced_revisions() {
    let key = MarketHoursKey::GlobexNikkei225Dollar;

    // 18:30 CT on a Wednesday evening is inside the pre-2012 evening session,
    // which runs 17:00 CT to 15:15 CT on the next trade date.
    assert!(
        open_at(key, utc(2011, 6, 15, 23, 30)),
        "the pre-2012 grid opens at 17:00 CT, so 18:30 CT that evening is open"
    );
    // 16:10 CT is inside the pre-2012 post-halt segment, which ran to 16:30 CT
    // — fifteen minutes longer than the 16:15 CT close SER-6465 introduced.
    assert!(
        open_at(key, utc(2011, 6, 15, 21, 10)),
        "the pre-2012 post-halt segment closes at 16:30 CT, so 16:10 CT is open"
    );
    // ...and the extra quarter-hour is the difference from the 2012 grid: 16:20
    // CT is open before 2012-11-18 and closed after it.
    assert!(
        open_at(key, utc(2011, 6, 15, 21, 20)),
        "16:20 CT is inside the pre-2012 16:30 CT close"
    );
    assert!(
        !open_at(key, utc(2013, 6, 19, 21, 20)),
        "SER-6465 pulled the close to 16:15 CT, so 16:20 CT must be closed after it"
    );
    // The carry-back reaches the January-2010 audit floor.
    assert!(
        open_at(key, utc(2010, 1, 6, 23, 30)),
        "the sourced pre-2012 grid is carried back to the 2010 audit floor"
    );
    assert!(
        open_at(key, utc(2014, 6, 18, 21, 10)),
        "between 2013-03-03 and 2015-09-19 the close is 16:15 CT, so 16:10 CT is open"
    );
    assert!(
        !open_at(key, utc(2026, 6, 17, 21, 10)),
        "from 2015-09-20 the close is 16:00 CT, so 16:10 CT must be closed"
    );
}

/// SER-6465 (session-opening Sunday 2012-11-18) extended the close to 16:15 CT
/// and introduced a 15:15–15:30 CT halt; SER-6554R (session-opening Sunday
/// 2013-03-03) removed that halt for the International Equity Index contracts
/// it names explicitly. Both sides of both cutovers are probed on the snapshot
/// selected after each Sunday opening, so a selector with either effective
/// date shifted fails here.
#[test]
fn nkd_halt_revision_and_removal_are_keyed_to_their_sunday_opening_days() {
    let key = MarketHoursKey::GlobexNikkei225Dollar;

    // 2012-11-18: the halt regime begins. The Monday probes are
    // 15:14/15:20/15:30/16:10/16:20 CT (CST).
    let halted = hours_for_market_hours_key(key, utc(2012, 11, 18, 23, 30));
    assert!(
        halted.is_open(utc(2012, 11, 19, 21, 14)),
        "15:14 CT still trades ahead of the halt"
    );
    assert!(
        !halted.is_open(utc(2012, 11, 19, 21, 20)),
        "the 15:15-15:30 CT halt matches nothing"
    );
    assert!(
        halted.is_open(utc(2012, 11, 19, 21, 30)),
        "the halt is end-exclusive: 15:30 CT trades again"
    );
    assert!(
        halted.is_open(utc(2012, 11, 19, 22, 10)),
        "the close is 16:15 CT"
    );
    assert!(
        !halted.is_open(utc(2012, 11, 19, 22, 20)),
        "16:20 CT is past the 16:15 CT close"
    );

    // The post-halt continuation runs on every closing day Monday–Friday.
    // Friday's segment belongs to the Thursday-evening session, and a
    // Sunday-afternoon instance never existed (Sunday opens at 17:00 CT).
    assert!(
        halted.is_open(utc(2012, 11, 23, 21, 30)),
        "Friday 15:30 CT trades after the halt"
    );
    assert!(
        halted.is_open(utc(2012, 11, 23, 22, 10)),
        "Friday's close is 16:15 CT"
    );
    assert!(
        !halted.is_open(utc(2012, 11, 25, 21, 30)),
        "Sunday afternoon has no post-halt session"
    );
    assert!(
        !halted.is_open(utc(2012, 11, 25, 22, 10)),
        "Sunday afternoon has no 16:15 CT close either"
    );

    // The Sunday before: pre-2012 dates are sessionless, so the same Monday
    // probe answers closed everywhere.
    let before = hours_for_market_hours_key(key, utc(2012, 11, 11, 23, 30));
    assert!(!before.is_open(utc(2012, 11, 12, 21, 20)));

    // 2013-03-03: the halt is gone and the 16:15 CT close remains.
    let unhalting = hours_for_market_hours_key(key, utc(2013, 3, 3, 23, 30));
    assert!(
        unhalting.is_open(utc(2013, 3, 4, 21, 20)),
        "15:20 CT trades again after the halt removal"
    );
    assert!(
        unhalting.is_open(utc(2013, 3, 4, 22, 10)),
        "the close is still 16:15 CT"
    );

    // The Sunday before: the 2012 regime still halts at 15:15 CT.
    let still_halted = hours_for_market_hours_key(key, utc(2013, 2, 24, 23, 30));
    assert!(!still_halted.is_open(utc(2013, 2, 25, 21, 20)));
}

/// The 2015-09-20 revision is keyed to the session-opening Sunday for trade date
/// Monday 2015-09-21, matching `cme_group`. A revision mis-keyed to the Monday
/// would leave the preceding session on the old profile.
#[test]
fn nkd_2015_revision_is_keyed_to_the_session_opening_day() {
    let key = MarketHoursKey::GlobexNikkei225Dollar;

    assert!(
        open_at(key, utc(2015, 9, 17, 21, 10)),
        "trade date 2015-09-17 still closes 16:15 CT"
    );
    // Select the snapshot after the Sunday 2015-09-20 17:00 CT opening
    // (22:01Z). A selector mis-keyed to the Monday civil date still returns
    // the old profile here, so this probes the opening-day key itself.
    let monday_session = hours_for_market_hours_key(key, utc(2015, 9, 20, 22, 1));
    assert!(
        !monday_session.is_open(utc(2015, 9, 21, 21, 10)),
        "trade date 2015-09-21 is the first close at 16:00 CT"
    );
    assert!(
        monday_session.is_open(utc(2015, 9, 21, 20, 50)),
        "the Monday session itself still trades through 15:50 CT"
    );
}

/// ICE Sugar No. 11 moved its open across 2012 and its whole grid on
/// 2014-02-03. 08:00Z is 03:00 NY in winter, before the current 03:30 open but
/// after the 02:30 open in force between 2012-11-05 and 2014-01-31.
#[test]
fn ice_sugar_open_tracks_its_sourced_revisions() {
    let key = MarketHoursKey::IceUsSugar;

    assert!(
        open_regular_at(key, utc(2013, 1, 16, 8, 0)),
        "2012-11-05 through 2014-01-31 opens 02:30 NY, so 03:00 NY is executable"
    );
    assert!(
        !open_regular_at(key, utc(2026, 1, 14, 8, 0)),
        "from 2014-02-03 the open is 03:30 NY, so 03:00 NY is not yet executable"
    );
}

/// The five SGX equity-index grids must disagree. If any two collapsed onto one
/// profile, the crate would be substituting one market's hours for another's -
/// the exact failure the split exists to prevent.
#[test]
fn sgx_equity_index_grids_do_not_collapse_onto_each_other() {
    let keys = [
        MarketHoursKey::SgxEquityIndexJapan,
        MarketHoursKey::SgxEquityIndexChina,
        MarketHoursKey::SgxEquityIndexSingapore,
        MarketHoursKey::SgxEquityIndexTaiwan,
        MarketHoursKey::SgxEquityIndexNtrUsd,
    ];

    // 08:00 Singapore on a Wednesday = 00:00Z. Japan (07:30) and NTR (07:25)
    // have opened; China (09:00), Singapore (08:30) and Taiwan (08:45) have not.
    let probe = utc(2026, 6, 17, 0, 0);
    let open_count = keys.iter().filter(|&&key| open_at(key, probe)).count();

    assert_eq!(
        open_count, 2,
        "at 08:00 Singapore exactly the Japan and NTR (USD) grids are open; \
         a different count means two grids have collapsed together"
    );
}

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

/// SGX equity-index history: sessionless before the first sourced calendar
/// edition, then one sourced window, then the verified-current grid from the
/// knowledge-bound review row.
///
/// Six editions of SGX's Derivatives Trading Calendar disagree in two places
/// and state neither transition day, so the dated surface serves the
/// intersection of all six rather than keying revisions to an edition's year —
/// an annual edition's year is a publication scope, not an effective date.
/// Japan's T closes at 14:25 and its T+1 opens at 15:25 in that window, which
/// are the narrowest bounds any edition gives.
///
/// Singapore has no DST, so 06:30Z is 14:30 SGT, 07:15Z is 15:15 SGT, 07:40Z is
/// 15:40 SGT and 08:50Z is 16:50 SGT. 2026-09-16 falls after the review row.
#[test]
fn sgx_equity_index_serves_the_sourced_window_then_the_verified_grid() {
    let japan = MarketHoursKey::SgxEquityIndexJapan;

    assert!(
        !open_at(japan, utc(2015, 6, 17, 6, 30)),
        "pre-2020 SGX dates are sessionless: no edition of the calendar survives"
    );

    // Inside the sourced window the narrowest bounds apply, so both the T close
    // beyond 14:25 and the T+1 open before 15:25 stay closed.
    for year in [2022, 2025] {
        for (h, m, what) in [
            (6u32, 30u32, "14:30 SGT, past the 14:25 T close"),
            (7, 15, "15:15 SGT, before the 15:25 T+1 open"),
        ] {
            let t = utc(year, 6, if year == 2022 { 15 } else { 18 }, h, m);
            assert!(
                !open_at(japan, t),
                "{year}: {what} is outside the sourced window every edition agrees on"
            );
        }
        let inside = utc(year, 6, if year == 2022 { 15 } else { 18 }, 7, 40);
        assert!(
            open_at(japan, inside),
            "{year}: 15:40 SGT is inside the T+1 session in every sourced edition"
        );
    }

    // After the knowledge-bound row the verified-current grid applies: T runs to
    // 14:55 and T+1 opens at 15:10.
    for (h, m) in [(6u32, 30u32), (7, 15)] {
        let t = utc(2026, 9, 16, h, m);
        assert!(
            open_at(japan, t),
            "the verified-current grid opens Japan at both 14:30 and 15:15 SGT"
        );
    }

    // China's T+1 opens at 17:00 in the sourced window and 16:45 currently.
    let china = MarketHoursKey::SgxEquityIndexChina;
    assert!(
        !open_at(china, utc(2022, 6, 15, 8, 50)),
        "16:50 SGT precedes China's 17:00 T+1 open in the sourced window"
    );
    assert!(
        open_at(china, utc(2026, 9, 16, 8, 50)),
        "the verified-current grid opens China's T+1 at 16:45 SGT"
    );

    for key in [
        MarketHoursKey::SgxEquityIndexSingapore,
        MarketHoursKey::SgxEquityIndexTaiwan,
        MarketHoursKey::SgxEquityIndexNtrUsd,
    ] {
        assert!(
            !open_at(key, utc(2015, 6, 17, 6, 30)),
            "{key:?} must be sessionless before the first sourced edition"
        );
    }
}
