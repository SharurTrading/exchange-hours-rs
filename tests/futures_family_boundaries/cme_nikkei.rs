// SPDX-License-Identifier: MIT-0

//! CME Nikkei 225 Dollar (`NKD`): every sourced close, halt and removal revision.

use super::prelude::*;
use chrono::NaiveDate;
use exchange_hours::calendar_for_market_hours_key;

/// CME Nikkei 225 Dollar moved 15:15 CT -> 16:15 CT (2012), kept 16:15 CT after
/// the halt removal (2013), then moved to 16:00 CT (2015-09-20, CME Globex
/// Notice #20150817). 21:10Z is 16:10 CT on a US summer date, so it is inside
/// the session only while the close is 16:15 CT. Before 2010-04-11 the dated
/// route serves the old daytime-anchored grid the operator's own equities-hours
/// page prints — CST 02:00-15:15 and 15:30-16:30 (no Sunday hours) from the
/// floor, CDT 03:00-15:15, 15:30-16:30 and the 17:00-18:00 tail (Sunday
/// included) from the 2010-03-14 DST entry — and CME's Globex notice of
/// 2010-04-05 dates the served continuous grid from that Sunday.
#[test]
fn nkd_close_tracks_its_sourced_revisions() {
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
    // The old grid's standard-time weekdays run 02:00-15:15 CT and 15:30-16:30
    // CT, each on its own local day: CME's equities-hours page prints `CST:
    // 02:00-15:15; reopens 15:30-16:30; closes 16:30` with `CST: No Sunday
    // Hours`, so the 2010 winter mornings open at 02:00 CT and the day is over
    // at 16:30 CT — nothing like the served grid's 17:00 CT evening open.
    assert!(
        open_at(key, utc(2010, 1, 6, 8, 0)),
        "02:00 CT is the old grid's CST open (2010-01-06, a Wednesday)"
    );
    assert!(
        !open_at(key, utc(2010, 1, 6, 7, 59)),
        "one second before the 02:00 CT CST open is closed"
    );
    assert!(
        open_at(key, utc(2010, 1, 6, 21, 14)),
        "15:14 CT still trades ahead of the CST grid's 15:15 CT break"
    );
    assert!(
        !open_at(key, utc(2010, 1, 6, 21, 20)),
        "the 15:15-15:30 CT break matches nothing on the CST grid"
    );
    assert!(
        open_at(key, utc(2010, 1, 6, 22, 29)),
        "the CST grid's reopen runs to its 16:30 CT close"
    );
    assert!(
        !open_at(key, utc(2010, 1, 6, 22, 30)),
        "16:30 CT is the CST grid's final close, end-exclusive"
    );
    assert!(
        !open_at(key, utc(2010, 1, 10, 16, 0)),
        "the CST grid has no Sunday hours at all (2010-01-10, a Sunday)"
    );
    // From the 2010-03-14 DST entry the page's CDT spelling governs: three
    // weekday runs (03:00-15:15, 15:30-16:30, and the 17:00-18:00 tail behind
    // the 16:30-17:00 CT maintenance halt) plus the Sunday evening leg, which
    // prints as the Sunday column's own `CDT: Opens 17:00-18:00`.
    assert!(
        !open_at(key, utc(2010, 3, 16, 7, 30)),
        "02:30 CT is closed on the CDT grid, whose open moved to 03:00 CT"
    );
    assert!(
        open_at(key, utc(2010, 3, 16, 8, 0)),
        "03:00 CT is the CDT grid's open (2010-03-16, a Tuesday)"
    );
    assert!(
        !open_at(key, utc(2010, 3, 16, 21, 35)),
        "16:35 CT sits inside the CDT grid's 16:30-17:00 CT maintenance halt"
    );
    assert!(
        open_at(key, utc(2010, 3, 16, 22, 30)),
        "17:30 CT is inside the CDT grid's 17:00-18:00 CT tail"
    );
    assert!(
        !open_at(key, utc(2010, 3, 16, 23, 0)),
        "18:00 CT is the tail's end-exclusive close"
    );
    assert!(
        open_at(key, utc(2010, 3, 14, 22, 30)),
        "the DST regime's first Sunday leg runs 17:00-18:00 CT on 2010-03-14"
    );
    // ...and the served grid is NOT carried back over the old-grid era: its
    // 17:00 CT Sunday open wraps into a 15:15 CT Monday close only from the
    // notice's dated Sunday 2010-04-11. The old grid's last session opened
    // Friday 2010-04-09 (its 17:00-18:00 CT tail); Saturday carries nothing.
    assert!(
        open_at(key, utc(2010, 4, 9, 22, 30)),
        "the old grid's last session trades its Friday 17:00-18:00 CT tail"
    );
    assert!(
        open_at(key, utc(2010, 4, 12, 23, 30)),
        "the served grid applies from the notice's effective Sunday 2010-04-11"
    );
    assert!(
        open_at(key, utc(2010, 6, 16, 23, 30)),
        "between 2010-04-11 and 2012-11-17 the close is 15:15 CT, so 18:30 CT is inside the wrapped evening leg"
    );
    assert!(
        open_at(key, utc(2010, 12, 15, 23, 30)),
        "the same 2010 grid answers in December 2010"
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

/// The old-grid era is keyed at venue-local midnights: the support floor
/// (2010-01-01, the CST spelling), the 2010-03-14 DST entry (the CDT regime's
/// first local opening day) and the notice-dated 2010-04-11 changeover. A
/// snapshot selected one second either side of each midnight carries a
/// different profile, fenced on an instant the two profiles disagree about.
#[test]
fn the_old_grid_and_changeover_are_keyed_at_venue_local_midnights() {
    let key = MarketHoursKey::GlobexNikkei225Dollar;

    // Floor: 2009-12-31 23:59 CT has no shipped row (below the timeline's
    // first), 2010-01-01 00:00 CT carries the CST old grid. The disagreement
    // instant is Tuesday 2010-01-05 02:30 CT: open on the CST grid, closed on
    // anything the served grid would state.
    let floor_day = hours_for_market_hours_key(key, utc(2010, 1, 1, 6, 0));
    assert!(
        floor_day.is_open(utc(2010, 1, 5, 8, 30)),
        "the floor-day snapshot serves the CST old grid: 02:30 CT is inside 02:00-15:15"
    );

    // DST entry: 2010-03-13 23:59 CT selects the CST grid, 2010-03-14 00:00 CT
    // selects the CDT grid. 02:30 local on Tuesday 2010-03-16 distinguishes
    // them — open on the CST grid, closed on the CDT one.
    let cst = hours_for_market_hours_key(key, utc(2010, 3, 14, 5, 59));
    assert!(
        cst.is_open(utc(2010, 3, 16, 7, 30)),
        "the 2010-03-13 snapshot serves the CST grid: 02:30 CT is open"
    );
    let cdt = hours_for_market_hours_key(key, utc(2010, 3, 14, 6, 0));
    assert!(
        !cdt.is_open(utc(2010, 3, 16, 7, 30)),
        "the 2010-03-14 snapshot serves the CDT grid: 02:30 CT is closed, the open is 03:00 CT"
    );

    // Changeover: 2010-04-10 23:59 CT (the old grid's last civil day, a
    // Saturday) selects the CDT old grid; 2010-04-11 00:00 CT (the notice's
    // Sunday) selects the served grid. Two instants distinguish them: Monday
    // 02:30 CT is closed on the old grid (it opens 03:00 CT) and inside the
    // served grid's Sunday-opened wrap; Sunday 18:30 CT is past the old grid's
    // one-hour Sunday leg and inside the served grid's wrap.
    let old = hours_for_market_hours_key(key, utc(2010, 4, 11, 4, 59));
    assert!(
        !old.is_open(utc(2010, 4, 12, 7, 30)),
        "the 2010-04-10 snapshot serves the old grid: Monday 02:30 CT is closed"
    );
    assert!(
        !old.is_open(utc(2010, 4, 11, 23, 30)),
        "the 2010-04-10 snapshot serves the old grid: Sunday 18:30 CT is past its one-hour leg"
    );
    let served = hours_for_market_hours_key(key, utc(2010, 4, 11, 5, 0));
    assert!(
        served.is_open(utc(2010, 4, 12, 7, 30)),
        "the 2010-04-11 snapshot serves the notice's grid: Monday 02:30 CT is inside the wrap"
    );
    assert!(
        served.is_open(utc(2010, 4, 11, 23, 30)),
        "the 2010-04-11 snapshot serves the notice's grid: Sunday 17:00 CT wraps into Monday"
    );
}

/// President's Day 2010-02-15: the operator's own sheet carves the Nikkei out
/// of the equity holiday pattern — `Exception: USD & JY denominated Nikkei
/// will open at their regularly scheduled times of 02:00 & 05:00 Monday
/// morning.` — so NKD trades its regular CST grid while the equity family
/// halts at 10:30 CT, and the table ships no row for the day.
#[test]
fn presidents_day_2010_trades_the_nkd_instants_not_the_equity_pattern() {
    let key = MarketHoursKey::GlobexNikkei225Dollar;
    let nkd = calendar_for_market_hours_key(key);
    let equity = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);

    // The 02:00 CT open is the NKD-specific instant the exception names (05:00
    // is the yen contract's), and the day runs to the regular 16:30 CT close.
    assert!(
        nkd.is_open(utc(2010, 2, 15, 8, 0))
            .expect("the coverage contract must answer a covered date"),
        "02:00 CT is the excepted NKD open on 2010-02-15"
    );
    assert!(
        !nkd.is_open(utc(2010, 2, 15, 7, 59))
            .expect("the coverage contract must answer a covered date"),
        "one second before the excepted open is closed"
    );
    assert!(
        nkd.is_open(utc(2010, 2, 15, 16, 30))
            .expect("the coverage contract must answer a covered date"),
        "10:30 CT is an ordinary NKD trading minute: the equity halt is not its arrangement"
    );
    assert!(
        !equity
            .is_open(utc(2010, 2, 15, 16, 30))
            .expect("the coverage contract must answer a covered date"),
        "the equity family halts at 10:30 CT on the same day — the pattern NKD is excepted from"
    );
    assert!(
        nkd.is_open(utc(2010, 2, 15, 22, 29))
            .expect("the coverage contract must answer a covered date"),
        "the regular 15:30-16:30 CT reopen still runs"
    );
    assert!(
        !nkd.is_open(utc(2010, 2, 15, 22, 30))
            .expect("the coverage contract must answer a covered date"),
        "16:30 CT closes the holiday Monday exactly as it closes an ordinary day"
    );
    assert_eq!(
        nkd.holiday_on(NaiveDate::from_ymd_opt(2010, 2, 15).expect("valid date")),
        None,
        "the table ships no row for 2010-02-15: the contract traded its regular grid"
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

/// The normal-week Pre-Open is published and is served as `order_entry`.
///
/// CME's own trading-hours service prints `16:45 preopen` Monday through
/// Thursday and `16:00 preopen` on the Sunday that opens the week, each handing
/// over to the 17:00 CT open, and CME's own legend defines `preopen` as "Order
/// Entry, modification, and cancel are allowed. No order matching." The module
/// asserted the opposite until this fence existed: it read "CME publishes no
/// normal-week pre-open or order-entry start time for NKD".
///
/// Both onsets are probed at the window's first and last second and one second
/// before it, and the handover is asserted through `session_bounds`, so a queue
/// that moved, widened, narrowed or became a session fails here. October is
/// CDT, so every instant below is the printed CT wall clock plus five hours.
#[test]
fn nkd_serves_the_published_normal_week_pre_open_as_order_entry() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexNikkei225Dollar);

    // (queue open, queue close, the matching session's own final close).
    let windows = [
        // The Sunday that opens the week: `16:00 preopen` -> `17:00 open`.
        (
            utc(2026, 10, 18, 21, 0),
            utc(2026, 10, 18, 22, 0),
            utc(2026, 10, 19, 21, 0),
        ),
        // Monday through Thursday: `16:45 preopen` -> `17:00 open`.
        (
            utc(2026, 10, 19, 21, 45),
            utc(2026, 10, 19, 22, 0),
            utc(2026, 10, 20, 21, 0),
        ),
        (
            utc(2026, 10, 20, 21, 45),
            utc(2026, 10, 20, 22, 0),
            utc(2026, 10, 21, 21, 0),
        ),
        (
            utc(2026, 10, 21, 21, 45),
            utc(2026, 10, 21, 22, 0),
            utc(2026, 10, 22, 21, 0),
        ),
        (
            utc(2026, 10, 22, 21, 45),
            utc(2026, 10, 22, 22, 0),
            utc(2026, 10, 23, 21, 0),
        ),
    ];

    for (open, close, session_close) in windows {
        for instant in [open, close - Duration::seconds(1)] {
            assert_eq!(
                calendar.session_state(instant),
                Ok(SessionState::OrderEntry),
                "{instant} is inside the published Pre-Open"
            );
            assert_eq!(
                calendar.is_order_entry_only(instant),
                Ok(true),
                "{instant} is order-entry only"
            );
            assert_eq!(
                calendar.is_accepting_orders(instant),
                Ok(true),
                "{instant} accepts orders"
            );
            assert_eq!(
                calendar.is_open(instant),
                Ok(false),
                "{instant}: a pre-open matches no trade and stays out of is_open"
            );
        }
        assert_ne!(
            calendar.session_state(open - Duration::seconds(1)),
            Ok(SessionState::OrderEntry),
            "no queue runs before its published onset"
        );
        // 17:00 CT is the queue's end-exclusive close and the matching session's
        // open: one instant, both statements.
        assert_eq!(
            calendar.session_state(close),
            Ok(SessionState::OpenRegular),
            "{close} hands the queue to the matching session"
        );
        assert_eq!(
            calendar.session_bounds(close),
            Ok(Some((close, session_close))),
            "{close} opens the session this family models as regular"
        );
    }
}
