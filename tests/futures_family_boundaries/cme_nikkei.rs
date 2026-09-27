// SPDX-License-Identifier: MIT-0

//! CME Nikkei 225 Dollar (`NKD`): every sourced close, halt and removal revision.

use super::prelude::*;
use exchange_hours::calendar_for_market_hours_key;

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
    // The 2011 grid is NOT carried to the audit floor. CME's 2010-03-10 and
    // 2010-04-07 captures show a materially different, DST-dependent grid whose
    // evening segment ran only 17:00-18:00 CT, so serving the 17:00-15:15
    // continuous grid there would report the contract open all night when it was
    // closed. The changeover day is undated, so 2010 is sessionless.
    for probe in [
        utc(2010, 1, 6, 23, 30),
        utc(2010, 6, 16, 23, 30),
        utc(2010, 12, 15, 23, 30),
    ] {
        assert!(
            !open_at(key, probe),
            "2010 predates the first sourced appearance of this grid and must be sessionless"
        );
    }
    // ...and the grid is served from its first sourced capture onward.
    assert!(
        open_at(key, utc(2011, 1, 13, 23, 30)),
        "the 2011 grid applies from its first sourced capture (2011-01-12)"
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
