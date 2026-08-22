// SPDX-License-Identifier: MIT-0

//! Boundary-query fixtures and stable serde forms.

use super::prelude::*;

// ---------------------------------------------------------------------------
// Session bounds sanity checks
// ---------------------------------------------------------------------------

#[test]
fn cme_session_bounds_monday_rth() {
    let h = hours_for_exchange(Exchange::Cme);
    let t = ct((2026, 4, 20), (10, 0, 0));
    let (open, close) = session_bounds(&h, t).expect("a session contains or follows t");
    assert_eq!(
        open,
        ct((2026, 4, 20), (8, 30, 0)),
        "Session opens at 08:30 CT"
    );
    assert_eq!(
        close,
        ct((2026, 4, 20), (15, 15, 0)),
        "Session closes at 15:15 CT"
    );
}

#[test]
fn cme_next_session_after_friday_close() {
    let h = hours_for_exchange(Exchange::Cme);
    let t = ct((2026, 4, 24), (16, 30, 0));
    let (open, _close) = next_session_after(&h, t).expect("the next session exists");
    assert_eq!(
        open,
        ct((2026, 4, 26), (16, 0, 0)),
        "Next order-entry phase after Fri close is Sun 16:00 CT"
    );
}

#[test]
fn cbot_session_bounds_day_session() {
    let h = hours_for_exchange(Exchange::Cbot);
    let t = ct((2026, 4, 20), (10, 0, 0));
    let (open, close) = session_bounds(&h, t).expect("a session contains or follows t");
    assert_eq!(open, ct((2026, 4, 20), (8, 30, 0)));
    assert_eq!(close, ct((2026, 4, 20), (13, 20, 0)));
}

// Serde string compatibility: CamelCase variants must serialize to/from the same
// snake_case strings that existed before the rename, ensuring no wire-format break.
#[test]
fn exchange_serde_snake_case_nasdaq_bx() {
    let json = serde_json::to_string(&Exchange::NasdaqBx).unwrap();
    assert_eq!(json, "\"nasdaq_bx\"");
    let rt: Exchange = serde_json::from_str(&json).unwrap();
    assert_eq!(rt, Exchange::NasdaqBx);
}

#[test]
fn exchange_serde_snake_case_cboe_options_c1() {
    let json = serde_json::to_string(&Exchange::CboeOptionsC1).unwrap();
    assert_eq!(json, "\"cboe_options_c1\"");
    let rt: Exchange = serde_json::from_str(&json).unwrap();
    assert_eq!(rt, Exchange::CboeOptionsC1);
}

#[test]
fn exchange_serde_snake_case_ice_europe_commodities() {
    let json = serde_json::to_string(&Exchange::IceEuropeCommodities).unwrap();
    assert_eq!(json, "\"ice_europe_commodities\"");
    let rt: Exchange = serde_json::from_str(&json).unwrap();
    assert_eq!(rt, Exchange::IceEuropeCommodities);
}

#[test]
fn exchange_serde_snake_case_binance_futures() {
    let json = serde_json::to_string(&Exchange::BinanceFutures).unwrap();
    assert_eq!(json, "\"binance_futures\"");
    let rt: Exchange = serde_json::from_str(&json).unwrap();
    assert_eq!(rt, Exchange::BinanceFutures);
}
