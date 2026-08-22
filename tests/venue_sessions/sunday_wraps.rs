// SPDX-License-Identifier: MIT-0

//! Sunday open-side behavior for overnight wrap sessions.

use super::prelude::*;

// ---------------------------------------------------------------------------
// Overnight-wrap open-side regression
//
// A wrap rule (`open_ssm >= close_ssm`) that is enabled on a given weekday
// contributes only its OPEN side on that day. Its close belongs to the previous
// day's instance of the same rule, so a venue whose previous local day ran no
// session must read closed right up to today's open.
//
// COMEX/NYMEX (17:00→16:00 CT) and ICE US (Sunday 18:00→18:00 ET) are the venues that
// expose this: their wrap close falls late in the day, so every Sunday instant
// before that close previously reported open even though Saturday never opened
// a session. CME is pinned alongside them because its wrap closes at 08:30,
// which makes it insensitive to the same defect — its behaviour must not move.
// ---------------------------------------------------------------------------

#[test]
fn comex_sunday_morning_closed_before_globex_open() {
    let h = hours_for_exchange(Exchange::Comex);
    let t = ct((2026, 4, 19), (10, 0, 0));
    assert!(
        !h.is_open(t),
        "COMEX closed Sun 10:00 CT: Saturday opened no session to wrap into Sunday"
    );
}

#[test]
fn nymex_sunday_morning_closed_before_globex_open() {
    let h = hours_for_exchange(Exchange::Nymex);
    let t = ct((2026, 4, 19), (10, 0, 0));
    assert!(
        !h.is_open(t),
        "NYMEX closed Sun 10:00 CT: Saturday opened no session to wrap into Sunday"
    );
}

#[test]
fn iceus_sunday_morning_closed_before_open() {
    let h = hours_for_exchange(Exchange::Iceus);
    let t = et((2026, 4, 19), (10, 0, 0));
    assert!(
        !h.is_open(t),
        "ICE US closed Sun 10:00 ET: Saturday opened no session to wrap into Sunday"
    );
}

#[test]
fn iceus_equal_endpoint_rule_is_one_continuous_sunday_session() {
    // ICE's FANG+ specification publishes 20:00-18:00 ET and an exceptional
    // Sunday 18:00 open, preceded by a 17:30 Pre-Open. Equal endpoints
    // describe one complete local day after that distinct queue phase.
    // https://www.ice.com/products/66380320/NYSE-FANG-Index-Future
    let h = hours_for_exchange(Exchange::Iceus);
    let pre_open = et((2026, 4, 19), (17, 30, 0));
    let sunday_open = et((2026, 4, 19), (18, 0, 0));
    let monday_close = et((2026, 4, 20), (18, 0, 0));

    assert!(!h.is_open(pre_open - chrono::Duration::seconds(1)));
    assert!(h.is_open_extended(pre_open));
    assert!(h.is_open(sunday_open - chrono::Duration::seconds(1)));
    assert!(h.is_open(sunday_open));
    assert!(h.is_open(et((2026, 4, 20), (0, 0, 0))));
    assert!(h.is_open(monday_close - chrono::Duration::seconds(1)));
    assert!(!h.is_open(monday_close));
    assert_eq!(
        session_bounds(&h, sunday_open),
        Some((sunday_open, monday_close))
    );
    assert_eq!(
        session_bounds(&h, et((2026, 4, 20), (12, 0, 0))),
        Some((sunday_open, monday_close))
    );
    assert_eq!(
        candle_end(&h, sunday_open, CalendarResolution::Daily),
        Some(monday_close)
    );
    assert_eq!(
        candle_start(&h, et((2026, 4, 20), (12, 0, 0)), CalendarResolution::Daily,),
        Some(pre_open)
    );
    assert_eq!(h.normal_week_open_seconds(), 114 * 3600 + 30 * 60);

    let before = pre_open - chrono::Duration::seconds(1);
    assert_eq!(
        next_session_after(&h, before),
        Some((pre_open, sunday_open))
    );
    let calendar = calendar_for_exchange(Exchange::Iceus);
    assert_eq!(
        calendar.session_bounds(sunday_open),
        Some((sunday_open, monday_close))
    );
    assert_eq!(
        calendar.candle_start(et((2026, 4, 20), (12, 0, 0)), CalendarResolution::Daily,),
        Some(pre_open)
    );
    assert_eq!(
        calendar.candle_end(sunday_open, CalendarResolution::Daily),
        Some(monday_close)
    );
    assert_eq!(
        calendar.normal_week_open_seconds_containing(sunday_open),
        114 * 3600 + 30 * 60
    );
}

#[test]
fn cme_sunday_morning_closed_before_globex_open() {
    let h = hours_for_exchange(Exchange::Cme);
    let t = ct((2026, 4, 19), (10, 0, 0));
    assert!(!h.is_open(t), "CME closed Sun 10:00 CT");
}

#[test]
fn sunday_morning_is_open_agrees_with_session_bounds() {
    // The defect showed up as two public queries disagreeing: `is_open` said
    // open while `session_bounds` returned a session starting hours later.
    for (exchange, instant) in [
        (Exchange::Comex, ct((2026, 4, 19), (10, 0, 0))),
        (Exchange::Nymex, ct((2026, 4, 19), (10, 0, 0))),
        (Exchange::Iceus, et((2026, 4, 19), (10, 0, 0))),
        (Exchange::Cme, ct((2026, 4, 19), (10, 0, 0))),
    ] {
        let h = hours_for_exchange(exchange);
        let (open, close) = session_bounds(&h, instant).expect("a session follows the weekend");
        let contained = open <= instant && instant < close;
        assert_eq!(
            h.is_open(instant),
            contained,
            "{exchange:?}: is_open disagrees with session_bounds at {instant} \
             (bounds {open}..{close})"
        );
    }
}

#[test]
fn wrap_venues_reopen_at_their_published_sunday_open() {
    // The fix must not close a venue that genuinely is open: each of these is
    // the first instant of the Sunday-evening session.
    for (exchange, instant) in [
        (Exchange::Comex, ct((2026, 4, 19), (17, 0, 0))),
        (Exchange::Nymex, ct((2026, 4, 19), (17, 0, 0))),
        (Exchange::Iceus, et((2026, 4, 19), (18, 0, 0))),
        (Exchange::Cme, ct((2026, 4, 19), (17, 0, 0))),
    ] {
        let h = hours_for_exchange(exchange);
        assert!(h.is_open(instant), "{exchange:?} open at its Sunday open");
    }
}
