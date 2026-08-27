// SPDX-License-Identifier: MIT-0

//! Always-open and cross-venue futures contracts.

use super::prelude::*;

// ---------------------------------------------------------------------------
// Always-open venues (crypto perpetuals)
//
// These must be continuously open every day of the week with no maintenance
// gaps and no weekend boundaries.  The contract is explicit and separate
// from futures-session venues.
// ---------------------------------------------------------------------------

#[test]
fn binance_futures_always_open() {
    let h = hours_for_exchange(
        Exchange::BinanceFutures,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(
        h.is_open(utc((2026, 4, 20), (0, 0, 0))),
        "Binance Mon midnight UTC"
    );
    assert!(
        h.is_open(utc((2026, 4, 25), (12, 0, 0))),
        "Binance Sat 12:00 UTC"
    );
    assert!(
        h.is_open(utc((2026, 4, 26), (3, 0, 0))),
        "Binance Sun 03:00 UTC"
    );
}

#[test]
fn binance_futures_opens_at_its_exact_sourced_launch_instant() {
    let launch = utc((2019, 9, 13), (4, 0, 0));
    let before = hours_for_exchange(
        Exchange::BinanceFutures,
        launch - chrono::Duration::nanoseconds(1),
    );
    let after = hours_for_exchange(Exchange::BinanceFutures, launch);

    assert!(before.regular.is_empty());
    assert!(before.extended.is_empty());
    assert!(!before.is_open(launch - chrono::Duration::nanoseconds(1)));
    assert!(after.is_open_regular(launch));
    assert!(after.is_open_regular(launch + chrono::Duration::nanoseconds(1)));

    let calendar = calendar_for_exchange(Exchange::BinanceFutures);
    assert!(!calendar.is_open(launch - chrono::Duration::nanoseconds(1)));
    assert!(calendar.is_open_regular(launch));
    assert_eq!(
        calendar.session_bounds(launch),
        Some((launch, utc((2019, 9, 14), (0, 0, 0))))
    );
}

#[test]
fn always_open_no_maintenance() {
    let h = hours_for_exchange(
        Exchange::BinanceFutures,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    for day in 19..26 {
        let t = utc((2026, 4, day), (12, 0, 0));
        assert!(
            !h.is_maintenance(t),
            "Always-open venue never in maintenance"
        );
    }
}

#[test]
fn always_open_no_daily_or_weekend_close() {
    let h = hours_for_exchange(
        Exchange::BinanceFutures,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(!h.has_daily_close, "Always-open venue has no daily close");
    assert!(
        !h.has_weekend_close,
        "Always-open venue has no weekend close"
    );

    let instant = utc((2026, 4, 20), (12, 0, 0));
    for resolution in [
        CalendarResolution::Daily,
        CalendarResolution::Weekly,
        CalendarResolution::Monthly,
    ] {
        assert_eq!(candle_start(&h, instant, resolution), None);
        assert_eq!(candle_end(&h, instant, resolution), None);
    }

    let calendar = calendar_for_exchange(Exchange::BinanceFutures);
    assert_eq!(calendar.time_end_of_day(instant), None);
    assert_eq!(
        calendar.candle_end(instant, CalendarResolution::Weekly),
        None
    );
}

#[test]
fn always_open_distinct_from_futures() {
    let always_open = hours_for_exchange(
        Exchange::BinanceFutures,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let cme = hours_for_exchange(
        Exchange::Cme,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );

    assert!(
        !always_open.has_daily_close,
        "Always-open venue no daily close"
    );
    assert!(cme.has_daily_close, "CME has daily close");
    assert!(
        !always_open.has_weekend_close,
        "Always-open venue no weekend close"
    );
    assert!(cme.has_weekend_close, "CME has weekend close");

    assert!(
        always_open.regular.len() == 1 && always_open.extended.is_empty(),
        "Always-open has single regular rule, no extended"
    );
    assert!(
        !cme.regular.is_empty() && !cme.extended.is_empty(),
        "CME has multi-rule session pattern"
    );
}

// ---------------------------------------------------------------------------
// Cross-venue: is_closed_all_day_on for weekend days
// ---------------------------------------------------------------------------

#[test]
fn futures_venues_closed_saturday() {
    let saturday = chrono::NaiveDate::from_ymd_opt(2026, 4, 25).unwrap();
    for &exch in &[
        Exchange::Cme,
        Exchange::Cbot,
        Exchange::Comex,
        Exchange::Nymex,
        Exchange::Eurex,
        Exchange::Iceus,
        Exchange::Iceeu,
        Exchange::Cfe,
    ] {
        let h = hours_for_exchange(
            exch,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
        );
        assert!(
            h.is_closed_all_day_on(saturday, SessionKind::Both),
            "{exch:?} should be closed all Saturday"
        );
    }
}

#[test]
fn sgx_saturday_partial_trading_from_friday_wrap() {
    let h = hours_for_exchange(
        Exchange::Sgx,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let saturday = chrono::NaiveDate::from_ymd_opt(2026, 4, 25).unwrap();
    assert!(
        !h.is_closed_all_day_on(saturday, SessionKind::Both),
        "SGX Friday T+1 wrap extends into Saturday morning"
    );
    assert!(
        h.is_open(sgt((2026, 4, 25), (3, 0, 0))),
        "SGX Sat 03:00 SGT still in Friday's T+1 wrap"
    );
    assert!(
        !h.is_open(sgt((2026, 4, 25), (6, 0, 0))),
        "SGX closed Sat after 05:15 SGT"
    );
}

#[test]
fn futures_venues_sunday_morning_closed() {
    let sunday = chrono::NaiveDate::from_ymd_opt(2026, 4, 26).unwrap();
    for &exch in &[Exchange::Eurex, Exchange::Sgx] {
        let h = hours_for_exchange(
            exch,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
        );
        assert!(
            h.is_closed_all_day_on(sunday, SessionKind::Both),
            "{exch:?} should be closed all Sunday"
        );
    }
}

#[test]
fn futures_venues_sunday_has_sessions() {
    let sunday = chrono::NaiveDate::from_ymd_opt(2026, 4, 26).unwrap();
    for &exch in &[
        Exchange::Cme,
        Exchange::Comex,
        Exchange::Nymex,
        Exchange::Iceus,
        Exchange::Iceeu,
        Exchange::Cfe,
    ] {
        let h = hours_for_exchange(
            exch,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
        );
        assert!(
            !h.is_closed_all_day_on(sunday, SessionKind::Both),
            "{exch:?} has Sunday evening sessions"
        );
    }
}
