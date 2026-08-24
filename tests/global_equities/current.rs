// SPDX-License-Identifier: MIT-0

//! Current-session baselines for major global cash-equity venues.

use super::prelude::*;

#[test]
fn tsx_current_baseline() {
    let h = hours_for_exchange(Exchange::Tsx);
    let tz = America::Toronto;
    let date = (2026, 8, 19);
    assert!(!h.is_open(local(tz, date, (6, 59, 0))));
    // TSX pre-open order entry; matching begins at 09:30.
    assert!(h.is_order_entry_only(local(tz, date, (7, 0, 0))));
    assert!(h.is_open_regular(local(tz, date, (9, 30, 0))));
    assert!(h.is_open_extended(local(tz, date, (16, 0, 0))));
    assert!(!h.is_open(local(tz, date, (16, 10, 0))));
    assert!(h.is_open_extended(local(tz, date, (16, 15, 0))));
    assert!(!h.is_open(local(tz, date, (17, 0, 0))));
    assert!(!h.is_open(local(tz, (2026, 8, 22), (10, 0, 0))));
}

#[test]
fn borsa_istanbul_current_baseline() {
    let h = hours_for_exchange(Exchange::BorsaIstanbul);
    let tz = Europe::Istanbul;
    let date = (2026, 8, 19);
    assert!(!h.is_open(local(tz, date, (9, 39, 0))));
    // BIST opening order-collection phase ahead of the opening auction.
    assert!(h.is_order_entry_only(local(tz, date, (9, 40, 0))));
    assert!(h.is_open_regular(local(tz, date, (10, 0, 0))));
    assert!(h.is_open_extended(local(tz, date, (18, 0, 0))));
    assert!(!h.is_open(local(tz, date, (18, 10, 0))));
    assert!(!h.is_open(local(tz, (2026, 8, 22), (12, 0, 0))));
}

#[test]
fn jse_current_baseline() {
    let h = hours_for_exchange(Exchange::Jse);
    let tz = Africa::Johannesburg;
    let date = (2026, 8, 19);
    assert!(!h.is_open(local(tz, date, (8, 29, 0))));
    assert!(h.is_open_extended(local(tz, date, (8, 30, 0))));
    assert!(h.is_open_regular(local(tz, date, (9, 0, 0))));
    assert!(h.is_open_extended(local(tz, date, (16, 50, 0))));
    assert!(!h.is_open(local(tz, date, (17, 0, 0))));
    assert!(h.is_open_extended(local(tz, date, (17, 2, 0))));
    assert!(!h.is_open(local(tz, date, (17, 10, 0))));
    assert!(!h.is_open(local(tz, (2026, 8, 22), (10, 0, 0))));
}

#[test]
fn tadawul_current_baseline_and_workweek() {
    let h = hours_for_exchange(Exchange::Tadawul);
    let tz = Asia::Riyadh;
    let date = (2026, 8, 19);
    assert!(!h.is_open(local(tz, date, (9, 29, 0))));
    // Tadawul pre-open order entry ahead of the opening auction.
    assert!(h.is_order_entry_only(local(tz, date, (9, 30, 0))));
    assert!(h.is_open_regular(local(tz, date, (10, 0, 0))));
    assert!(h.is_open_extended(local(tz, date, (15, 0, 0))));
    assert!(!h.is_open(local(tz, date, (15, 20, 0))));
    assert!(!h.is_open(local(tz, (2026, 8, 21), (11, 0, 0))));
    assert!(h.is_open_regular(local(tz, (2026, 8, 23), (11, 0, 0))));
}

#[test]
fn lse_sets_current_auction_envelope() {
    let h = hours_for_exchange(Exchange::Lse);
    let tz = Europe::London;
    let date = (2026, 8, 19);

    assert!(!h.is_open(local(tz, date, (6, 59, 59))));
    // LSE pre-trading: no on-book execution before the opening auction.
    assert!(h.is_order_entry_only(local(tz, date, (7, 0, 0))));
    assert!(h.is_open_extended(local(tz, date, (8, 0, 29))));
    assert!(h.is_open_regular(local(tz, date, (8, 0, 30))));
    assert!(h.is_open_extended(local(tz, date, (12, 0, 0))));
    assert!(h.is_open_extended(local(tz, date, (12, 2, 29))));
    assert!(h.is_open_regular(local(tz, date, (12, 2, 30))));
    assert!(h.is_open_extended(local(tz, date, (16, 30, 0))));
    let (_, closing_auction_end) = exchange_hours::session_bounds_with(
        &h,
        local(tz, date, (16, 35, 29)),
        exchange_hours::SessionKind::Extended,
    )
    .expect("LSE closing auction");
    assert_eq!(closing_auction_end, local(tz, date, (16, 35, 30)));
    let (crossing_start, crossing_end) = exchange_hours::session_bounds_with(
        &h,
        local(tz, date, (16, 35, 30)),
        exchange_hours::SessionKind::Extended,
    )
    .expect("LSE Closing Price Crossing");
    assert_eq!(crossing_start, local(tz, date, (16, 35, 30)));
    assert_eq!(crossing_end, local(tz, date, (16, 40, 0)));
    assert!(h.is_open_extended(local(tz, date, (16, 39, 59))));
    assert!(!h.is_open(local(tz, date, (16, 40, 0))));
    assert!(!h.is_open(local(tz, (2026, 8, 22), (10, 0, 0))));
}

#[test]
fn bme_current_auction_and_trading_at_last_envelope() {
    let h = hours_for_exchange(Exchange::Bme);
    let tz = Europe::Madrid;
    let date = (2026, 8, 19);

    assert!(!h.is_open(local(tz, date, (8, 29, 59))));
    assert!(h.is_open_extended(local(tz, date, (8, 30, 0))));
    assert!(h.is_open_extended(local(tz, date, (9, 0, 29))));
    assert!(!h.is_open_regular(local(tz, date, (9, 0, 29))));
    assert!(h.is_open_regular(local(tz, date, (9, 0, 30))));
    assert!(h.is_open_extended(local(tz, date, (17, 30, 0))));
    let (_, closing_auction_end) = exchange_hours::session_bounds_with(
        &h,
        local(tz, date, (17, 35, 29)),
        exchange_hours::SessionKind::Extended,
    )
    .expect("BME closing auction");
    assert_eq!(closing_auction_end, local(tz, date, (17, 35, 30)));
    let (tal_start, tal_end) = exchange_hours::session_bounds_with(
        &h,
        local(tz, date, (17, 35, 30)),
        exchange_hours::SessionKind::Extended,
    )
    .expect("BME Trading-at-Last");
    assert_eq!(tal_start, local(tz, date, (17, 35, 30)));
    assert_eq!(tal_end, local(tz, date, (17, 45, 0)));
    assert!(h.is_open_extended(local(tz, date, (17, 44, 59))));
    assert!(!h.is_open(local(tz, date, (17, 45, 0))));
    assert!(!h.is_open(local(tz, (2026, 8, 22), (10, 0, 0))));
}

#[test]
fn vienna_atx_current_auction_envelope() {
    let h = hours_for_exchange(Exchange::Vienna);
    let tz = Europe::Vienna;
    let date = (2026, 8, 19);

    assert!(!h.is_open(local(tz, date, (7, 59, 59))));
    // Xetra pre-trading: order book closed to matching.
    assert!(h.is_order_entry_only(local(tz, date, (8, 0, 0))));
    assert!(h.is_open_extended(local(tz, date, (8, 55, 0))));
    assert!(h.is_open_extended(local(tz, date, (9, 0, 29))));
    assert!(!h.is_open_regular(local(tz, date, (9, 0, 29))));
    assert!(h.is_open_regular(local(tz, date, (9, 0, 30))));
    assert!(h.is_open_extended(local(tz, date, (12, 0, 0))));
    assert!(!h.is_open_regular(local(tz, date, (12, 3, 29))));
    assert!(h.is_open_regular(local(tz, date, (12, 3, 30))));
    assert!(h.is_open_extended(local(tz, date, (17, 30, 0))));
    let (_, closing_auction_end) = exchange_hours::session_bounds_with(
        &h,
        local(tz, date, (17, 35, 29)),
        exchange_hours::SessionKind::Extended,
    )
    .expect("Vienna closing auction");
    assert_eq!(closing_auction_end, local(tz, date, (17, 35, 30)));
    let (tac_start, tac_end) = exchange_hours::session_bounds_with(
        &h,
        local(tz, date, (17, 35, 30)),
        exchange_hours::SessionKind::Extended,
    )
    .expect("Vienna Trade-at-Close");
    assert_eq!(tac_start, local(tz, date, (17, 35, 30)));
    assert_eq!(tac_end, local(tz, date, (17, 45, 0)));
    assert!(h.is_open_extended(local(tz, date, (17, 44, 59))));
    // Vienna post-trading tail: order maintenance only, no matching.
    assert!(h.is_order_entry_only(local(tz, date, (17, 45, 0))));
    assert!(!h.is_open(local(tz, date, (17, 50, 0))));
    assert!(!h.is_open(local(tz, (2026, 8, 22), (10, 0, 0))));
}
