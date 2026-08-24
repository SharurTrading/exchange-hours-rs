//! Audit probe.

use chrono::{TimeZone, Utc};
use exchange_hours::{
    Exchange, MarketHoursKey, SessionState, hours_for_exchange, hours_for_market_hours_key,
    hours_for_market_hours_key_as_of,
};

#[test]
fn probe() {
    // 2026-04-20 is a Monday. Berlin is CEST (+02:00).
    let at = |h: u32, m: u32| Utc.with_ymd_and_hms(2026, 4, 20, h, m, 0).unwrap();

    let eurex = hours_for_exchange(Exchange::Eurex);
    for (h, m, label) in [
        (20u32, 30u32, "22:30 CEST Mon"),
        (23, 0, "01:00 CEST Tue"),
        (0, 5, "02:05 CEST Tue"),
        (0, 12, "02:12 CEST Tue"),
    ] {
        println!("EUREX venue {label}: {:?}", eurex.session_state(at(h, m)));
    }

    let fi = hours_for_market_hours_key_as_of(MarketHoursKey::EurexFixedIncome, at(12, 0));
    for (h, m, label) in [
        (20, 2, "22:02 CEST post-trading"),
        (20, 30, "22:30 CEST gap"),
        (23, 30, "01:30 CEST gap"),
        (0, 5, "02:05 CEST pre-trading"),
    ] {
        println!("EUREX FI dated {label}: {:?}", fi.session_state(at(h, m)));
    }

    // Snapshot (session_profile) path
    let snap = hours_for_market_hours_key(MarketHoursKey::EurexFixedIncome);
    println!(
        "SNAP FI 22:02 = {:?}, 02:05 = {:?}",
        snap.session_state(at(20, 2)),
        snap.session_state(at(0, 5))
    );
    let snap_eurex = hours_for_market_hours_key(MarketHoursKey::Eurex);
    println!(
        "SNAP EUREX 02:05 = {:?}, 02:12 = {:?}",
        snap_eurex.session_state(at(0, 5)),
        snap_eurex.session_state(at(0, 12))
    );
    let snap_sgx = hours_for_market_hours_key(MarketHoursKey::Sgx);
    // 07:10 SGT Mon = 23:10 UTC Sunday
    let sgt = |d: u32, h: u32, m: u32| Utc.with_ymd_and_hms(2026, 4, d, h, m, 0).unwrap();
    println!(
        "SNAP SGX 07:10 SGT Mon = {:?}",
        snap_sgx.session_state(sgt(19, 23, 10))
    );
    let dated_sgx = hours_for_market_hours_key_as_of(MarketHoursKey::Sgx, at(12, 0));
    println!(
        "DATED SGX 07:10 SGT Mon = {:?}",
        dated_sgx.session_state(sgt(19, 23, 10))
    );
    let snap_jp = hours_for_market_hours_key(MarketHoursKey::SgxEquityIndexJapan);
    println!(
        "SNAP SGX JP 07:20 SGT Mon = {:?}",
        snap_jp.session_state(sgt(19, 23, 20))
    );
    assert_eq!(SessionState::Closed, SessionState::Closed);
}
