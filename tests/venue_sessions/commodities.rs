// SPDX-License-Identifier: MIT-0

//! COMEX and NYMEX normal-week session boundaries.

use super::prelude::*;

// ---------------------------------------------------------------------------
// COMEX (Metals)
//   17:00–16:00 CT wrap (Sun + Mon–Thu), maintenance 16:00–17:00 daily
//   No Fri overnight.
// ---------------------------------------------------------------------------

#[test]
fn comex_sunday_open() {
    let h = hours_for_exchange(Exchange::Comex);
    let t = ct((2026, 4, 19), (17, 0, 0));
    assert!(h.is_open(t), "COMEX opens Sun 17:00 CT");
}

#[test]
fn comex_monday_trading() {
    let h = hours_for_exchange(Exchange::Comex);
    assert!(
        h.is_open(ct((2026, 4, 20), (10, 0, 0))),
        "COMEX Mon 10:00 CT"
    );
}

#[test]
fn comex_daily_maintenance() {
    let h = hours_for_exchange(Exchange::Comex);
    let t = ct((2026, 4, 20), (16, 30, 0));
    assert!(!h.is_open(t), "COMEX maintenance gap 16:30 CT");
    assert!(
        h.is_maintenance(t),
        "16:30 CT is inside the four-hour-bounded 16:00→17:00 break"
    );
}

#[test]
fn comex_monday_reopen() {
    let h = hours_for_exchange(Exchange::Comex);
    let t = ct((2026, 4, 20), (17, 0, 0));
    assert!(h.is_open(t), "COMEX reopens Mon 17:00 CT");
}

#[test]
fn comex_friday_close() {
    let h = hours_for_exchange(Exchange::Comex);
    let t = ct((2026, 4, 24), (16, 0, 0));
    assert!(!h.is_open(t), "COMEX closes Fri 16:00 CT (end-exclusive)");
    assert!(
        !h.is_open(ct((2026, 4, 24), (17, 0, 0))),
        "COMEX closed Fri 17:00 CT"
    );
}

#[test]
fn comex_saturday_closed() {
    let h = hours_for_exchange(Exchange::Comex);
    assert!(
        !h.is_open(ct((2026, 4, 25), (10, 0, 0))),
        "COMEX closed Saturday"
    );
}

#[test]
fn comex_weekend_boundary() {
    let h = hours_for_exchange(Exchange::Comex);
    assert!(
        !h.is_open(ct((2026, 4, 26), (16, 59, 0))),
        "COMEX closed before Sun 17:00 CT"
    );
    assert!(
        h.is_open(ct((2026, 4, 26), (17, 0, 0))),
        "COMEX opens Sun 17:00 CT"
    );
}

// ---------------------------------------------------------------------------
// NYMEX (Energy) — same profile as COMEX
// ---------------------------------------------------------------------------

#[test]
fn nymex_sunday_open() {
    let h = hours_for_exchange(Exchange::Nymex);
    assert!(
        h.is_open(ct((2026, 4, 19), (17, 0, 0))),
        "NYMEX opens Sun 17:00 CT"
    );
}

#[test]
fn nymex_daily_maintenance() {
    let h = hours_for_exchange(Exchange::Nymex);
    assert!(
        !h.is_open(ct((2026, 4, 20), (16, 30, 0))),
        "NYMEX maintenance 16:30 CT"
    );
}

#[test]
fn nymex_friday_close_and_weekend() {
    let h = hours_for_exchange(Exchange::Nymex);
    assert!(
        !h.is_open(ct((2026, 4, 24), (16, 0, 0))),
        "NYMEX closes Fri 16:00 CT"
    );
    assert!(
        !h.is_open(ct((2026, 4, 25), (10, 0, 0))),
        "NYMEX closed Saturday"
    );
}

// CME moved every COMEX and NYMEX Globex close from 16:15 to 16:00 CT for
// Monday 2015-09-21 while leaving opening times unchanged. The selector turns
// on for the Sunday opening day so the wrapped Monday trade-date session gets
// the sourced new close.
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150907.html
#[test]
fn comex_and_nymex_close_change_applies_to_the_2015_09_21_trade_date() {
    let cutover = ct((2015, 9, 20), (0, 0, 0));
    let monday = (2015, 9, 21);

    for exchange in [Exchange::Comex, Exchange::Nymex] {
        let before = hours_for_exchange_as_of(exchange, cutover - chrono::Duration::seconds(1));
        let after = hours_for_exchange_as_of(exchange, cutover);

        assert!(before.is_open(ct(monday, (16, 14, 59))), "{exchange:?}");
        assert!(!before.is_open(ct(monday, (16, 15, 0))), "{exchange:?}");
        assert!(after.is_open(ct((2015, 9, 20), (17, 0, 0))), "{exchange:?}");
        assert!(!after.is_open(ct(monday, (16, 0, 0))), "{exchange:?}");

        let calendar = calendar_for_exchange(exchange);
        assert!(calendar.is_open(ct(monday, (15, 59, 59))), "{exchange:?}");
        assert!(!calendar.is_open(ct(monday, (16, 0, 0))), "{exchange:?}");
    }
}
