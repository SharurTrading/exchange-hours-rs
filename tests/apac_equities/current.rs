// SPDX-License-Identifier: MIT-0

//! Current published-session baselines for APAC cash equities.

use super::prelude::*;

#[test]
fn asx_current_baseline() {
    let h = hours_for_exchange(
        Exchange::Asx,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let tz = Australia::Sydney;
    assert!(!h.is_open(local(tz, (2026, 8, 19), (6, 59, 0))));
    assert!(h.is_open_extended(local(tz, (2026, 8, 19), (7, 0, 0))));
    assert!(h.is_open_regular(local(tz, (2026, 8, 19), (10, 0, 0))));
    assert!(!h.is_open_regular(local(tz, (2026, 8, 19), (16, 0, 0))));
    assert!(h.is_open_extended(local(tz, (2026, 8, 19), (16, 15, 0))));
    assert!(!h.is_open(local(tz, (2026, 8, 19), (16, 21, 30))));
    assert_weekend_closed(Exchange::Asx, tz);
}

#[test]
fn tmx_australia_current_baseline() {
    let h = hours_for_exchange(
        Exchange::TmxAustralia,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let tz = Australia::Sydney;
    let date = (2026, 8, 19);
    assert!(!h.is_open(local(tz, date, (6, 59, 59))));
    assert!(h.is_open_extended(local(tz, date, (7, 0, 0))));
    assert!(h.is_open_regular(local(tz, date, (10, 0, 0))));
    assert!(h.is_open_regular(local(tz, date, (16, 0, 0))));
    assert!(h.is_open_extended(local(tz, date, (16, 0, 0))));
    assert!(!h.is_open_regular(local(tz, date, (16, 13, 0))));
    assert!(h.is_open_extended(local(tz, date, (16, 13, 0))));
    assert!(!h.is_open(local(tz, date, (16, 20, 0))));
    assert_weekend_closed(Exchange::TmxAustralia, tz);
}

#[test]
fn tokyo_current_baseline() {
    let h = hours_for_exchange(
        Exchange::Tse,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let tz = Asia::Tokyo;
    assert!(!h.is_open(local(tz, (2026, 8, 19), (7, 59, 0))));
    // TSE pre-opening: Itayose order acceptance, no matching until 09:00.
    assert!(h.is_order_entry_only(local(tz, (2026, 8, 19), (8, 0, 0))));
    assert!(h.is_open_regular(local(tz, (2026, 8, 19), (9, 0, 0))));
    assert!(h.is_open_extended(local(tz, (2026, 8, 19), (12, 0, 0))));
    assert!(h.is_open_regular(local(tz, (2026, 8, 19), (12, 30, 0))));
    assert!(h.is_open_extended(local(tz, (2026, 8, 19), (15, 25, 0))));
    assert!(h.is_open_extended(local(tz, (2026, 8, 19), (17, 59, 59))));
    assert!(!h.is_open(local(tz, (2026, 8, 19), (18, 0, 0))));
    assert_weekend_closed(Exchange::Tse, tz);
}

#[test]
fn nzx_current_baseline() {
    let h = hours_for_exchange(
        Exchange::Nzx,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let tz = Pacific::Auckland;
    assert!(!h.is_open(local(tz, (2026, 8, 19), (8, 29, 0))));
    assert!(h.is_open_extended(local(tz, (2026, 8, 19), (8, 30, 0))));
    assert!(h.is_open_regular(local(tz, (2026, 8, 19), (10, 0, 0))));
    // NZX Pre-Close: order entry for the closing auction, nothing matches.
    assert!(h.is_order_entry_only(local(tz, (2026, 8, 19), (16, 45, 0))));
    // The closing uncross is randomised within 30 seconds EITHER side of
    // 17:00, so 17:00 itself is still inside the tradeable auction window.
    assert!(h.is_open_extended(local(tz, (2026, 8, 19), (17, 0, 0))));
    assert!(!h.is_open(local(tz, (2026, 8, 19), (17, 1, 0))));
    assert_weekend_closed(Exchange::Nzx, tz);
}

#[test]
fn india_current_baselines() {
    let tz = Asia::Kolkata;
    for exchange in [Exchange::NseIndia, Exchange::BseIndia] {
        let h = hours_for_exchange(
            exchange,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
        );
        assert!(!h.is_open(local(tz, (2026, 8, 19), (8, 59, 0))));
        // India pre-open order-collection sub-window; the matching
        // sub-window that follows it stays tradeable.
        assert!(h.is_order_entry_only(local(tz, (2026, 8, 19), (9, 0, 0))));
        assert!(h.is_open_regular(local(tz, (2026, 8, 19), (9, 15, 0))));
        assert!(h.is_open_regular(local(tz, (2026, 8, 19), (15, 20, 0))));
        assert!(h.is_open_extended(local(tz, (2026, 8, 19), (15, 20, 0))));
        assert!(h.is_open_extended(local(tz, (2026, 8, 19), (15, 30, 0))));
        assert!(!h.is_open(local(tz, (2026, 8, 19), (15, 35, 0))));
        assert!(h.is_open_extended(local(tz, (2026, 8, 19), (15, 50, 0))));
        assert!(!h.is_open(local(tz, (2026, 8, 19), (16, 0, 0))));
        assert_weekend_closed(exchange, tz);
    }
}

#[test]
fn hong_kong_current_baseline() {
    let h = hours_for_exchange(
        Exchange::Hkex,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let tz = Asia::Hong_Kong;
    assert!(!h.is_open(local(tz, (2026, 8, 19), (8, 59, 0))));
    // HKEX pre-opening order-input period; the auction match follows.
    assert!(h.is_order_entry_only(local(tz, (2026, 8, 19), (9, 0, 0))));
    assert!(h.is_open_regular(local(tz, (2026, 8, 19), (9, 30, 0))));
    assert!(h.is_open_regular(local(tz, (2026, 8, 19), (12, 30, 0))));
    assert!(h.is_open_regular(local(tz, (2026, 8, 19), (13, 0, 0))));
    assert!(h.is_open_extended(local(tz, (2026, 8, 19), (16, 0, 0))));
    assert!(!h.is_open(local(tz, (2026, 8, 19), (16, 10, 0))));
    assert_weekend_closed(Exchange::Hkex, tz);
}

#[test]
fn sgx_securities_current_baseline() {
    let h = hours_for_exchange(
        Exchange::SgxSecurities,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let tz = Asia::Singapore;
    assert!(!h.is_open(local(tz, (2026, 8, 19), (8, 29, 0))));
    // SGX Pre-Opening routine: order entry only.
    assert!(h.is_order_entry_only(local(tz, (2026, 8, 19), (8, 30, 0))));
    assert!(h.is_open_regular(local(tz, (2026, 8, 19), (9, 0, 0))));
    assert!(!h.is_open_regular(local(tz, (2026, 8, 19), (12, 30, 0))));
    // Midday pre-opening routine ahead of the afternoon session: order entry.
    assert!(h.is_order_entry_only(local(tz, (2026, 8, 19), (12, 30, 0))));
    assert!(h.is_open_regular(local(tz, (2026, 8, 19), (13, 0, 0))));
    assert!(h.is_open_extended(local(tz, (2026, 8, 19), (17, 15, 0))));
    assert!(!h.is_open(local(tz, (2026, 8, 19), (17, 16, 0))));
    assert_weekend_closed(Exchange::SgxSecurities, tz);
}

#[test]
fn southeast_asia_current_baselines() {
    let date = (2026, 8, 19);

    let tz = Asia::Kuala_Lumpur;
    let h = hours_for_exchange(
        Exchange::BursaMalaysia,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(!h.is_open(local(tz, date, (8, 29, 0))));
    assert!(h.is_open_extended(local(tz, date, (8, 30, 0))));
    assert!(h.is_open_regular(local(tz, date, (9, 0, 0))));
    assert!(h.is_open_regular(local(tz, date, (12, 15, 0))));
    assert!(h.is_open_regular(local(tz, date, (12, 29, 59))));
    assert!(!h.is_open(local(tz, date, (12, 30, 0))));
    assert!(h.is_open_extended(local(tz, date, (14, 0, 0))));
    assert!(h.is_open_regular(local(tz, date, (14, 30, 0))));
    assert!(h.is_open_extended(local(tz, date, (16, 45, 0))));
    assert!(!h.is_open(local(tz, date, (17, 0, 0))));
    assert_weekend_closed(Exchange::BursaMalaysia, tz);

    let tz = Asia::Bangkok;
    let h = hours_for_exchange(
        Exchange::SetThailand,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(!h.is_open(local(tz, date, (9, 29, 0))));
    // SET pre-open order accumulation ahead of the randomised auction.
    assert!(h.is_order_entry_only(local(tz, date, (9, 30, 0))));
    assert!(h.is_open_regular(local(tz, date, (10, 0, 0))));
    assert!(h.is_open_regular(local(tz, date, (12, 30, 0))));
    // 13:30 sits in SET's afternoon pre-open for ordinary shares, but eligible
    // Europe/Americas DRs trade continuously through the break, so the venue is
    // genuinely open here rather than order-entry-only.
    assert!(h.is_open(local(tz, date, (13, 30, 0))));
    assert!(h.is_open_regular(local(tz, date, (14, 0, 0))));
    assert!(h.is_open_extended(local(tz, date, (16, 30, 0))));
    assert!(!h.is_open(local(tz, date, (17, 0, 0))));
    // Night-session pre-open: order accumulation, no matching.
    assert!(h.is_order_entry_only(local(tz, date, (18, 45, 0))));
    assert!(h.is_open_regular(local(tz, date, (19, 0, 0))));
    // Night-session pre-close AND off-hour window: off-hour trades print, so
    // this stays tradeable rather than order-entry-only.
    assert!(h.is_open_extended(local(tz, (2026, 8, 20), (2, 45, 0))));
    assert!(!h.is_open(local(tz, (2026, 8, 20), (3, 0, 0))));
    assert_weekend_closed(Exchange::SetThailand, tz);

    let tz = Asia::Jakarta;
    let h = hours_for_exchange(
        Exchange::Idx,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(!h.is_open(local(tz, date, (8, 44, 0))));
    // IDX pre-opening order collection 08:45-08:55; the 08:55 pre-open match
    // that follows it is tradeable and stays in `extended`.
    assert!(h.is_order_entry_only(local(tz, date, (8, 45, 0))));
    assert!(h.is_open_regular(local(tz, date, (9, 0, 0))));
    assert!(!h.is_open(local(tz, date, (12, 0, 0))));
    assert!(h.is_open_regular(local(tz, date, (13, 30, 0))));
    assert!(h.is_open_extended(local(tz, date, (15, 50, 0))));
    assert!(h.is_open_extended(local(tz, date, (16, 29, 59))));
    assert!(!h.is_open(local(tz, date, (16, 30, 0))));
    assert!(!h.is_open(local(tz, (2026, 8, 21), (11, 30, 0))));
    assert!(h.is_open_regular(local(tz, (2026, 8, 21), (14, 0, 0))));
    assert_weekend_closed(Exchange::Idx, tz);

    let tz = Asia::Manila;
    let h = hours_for_exchange(
        Exchange::Pse,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(!h.is_open(local(tz, date, (8, 59, 0))));
    assert!(h.is_open_extended(local(tz, date, (9, 0, 0))));
    assert!(h.is_open_regular(local(tz, date, (9, 30, 0))));
    assert!(!h.is_open(local(tz, date, (12, 30, 0))));
    assert!(h.is_open_regular(local(tz, date, (13, 0, 0))));
    assert!(h.is_open_extended(local(tz, date, (14, 45, 0))));
    assert!(!h.is_open(local(tz, date, (15, 15, 0))));
    assert_weekend_closed(Exchange::Pse, tz);

    let tz = Asia::Ho_Chi_Minh;
    let h = hours_for_exchange(
        Exchange::Hose,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(!h.is_open(local(tz, date, (8, 59, 0))));
    assert!(h.is_open_extended(local(tz, date, (9, 0, 0))));
    assert!(h.is_open_regular(local(tz, date, (9, 15, 0))));
    assert!(!h.is_open(local(tz, date, (11, 30, 0))));
    assert!(h.is_open_regular(local(tz, date, (13, 0, 0))));
    assert!(h.is_open_extended(local(tz, date, (14, 30, 0))));
    assert!(h.is_open_extended(local(tz, date, (14, 59, 59))));
    assert!(!h.is_open(local(tz, date, (15, 0, 0))));
    assert_weekend_closed(Exchange::Hose, tz);
}

#[test]
fn mainland_china_current_baselines() {
    let tz = Asia::Shanghai;
    for exchange in [Exchange::Sse, Exchange::Szse] {
        let h = hours_for_exchange(
            exchange,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
        );
        assert!(!h.is_open(local(tz, (2026, 8, 19), (9, 14, 0))));
        assert!(h.is_open_extended(local(tz, (2026, 8, 19), (9, 15, 0))));
        assert!(!h.is_open(local(tz, (2026, 8, 19), (9, 25, 0))));
        assert!(h.is_open_regular(local(tz, (2026, 8, 19), (9, 30, 0))));
        assert!(!h.is_open(local(tz, (2026, 8, 19), (11, 30, 0))));
        assert!(h.is_open_regular(local(tz, (2026, 8, 19), (13, 0, 0))));
        assert!(h.is_open_extended(local(tz, (2026, 8, 19), (14, 57, 0))));
        assert!(h.is_open_extended(local(tz, (2026, 8, 19), (15, 0, 0))));
        assert!(!h.is_open(local(tz, (2026, 8, 19), (15, 30, 0))));
        assert_weekend_closed(exchange, tz);
    }
}

#[test]
fn korea_current_baseline() {
    let h = hours_for_exchange(
        Exchange::Krx,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let tz = Asia::Seoul;
    assert!(!h.is_open(local(tz, (2026, 8, 19), (7, 59, 59))));
    assert!(h.is_open_extended(local(tz, (2026, 8, 19), (8, 0, 0))));
    assert!(h.is_open_extended(local(tz, (2026, 8, 19), (8, 30, 0))));
    assert!(h.is_open_regular(local(tz, (2026, 8, 19), (9, 0, 0))));
    assert!(h.is_open_extended(local(tz, (2026, 8, 19), (15, 20, 0))));
    assert!(!h.is_open(local(tz, (2026, 8, 19), (18, 0, 0))));
    assert_weekend_closed(Exchange::Krx, tz);
}

#[test]
fn taiwan_current_baseline() {
    let h = hours_for_exchange(
        Exchange::Twse,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let tz = Asia::Taipei;
    assert!(!h.is_open(local(tz, (2026, 8, 19), (7, 59, 59))));
    assert!(h.is_open_extended(local(tz, (2026, 8, 19), (8, 0, 0))));
    assert!(h.is_open_regular(local(tz, (2026, 8, 19), (9, 0, 0))));
    assert!(h.is_open_extended(local(tz, (2026, 8, 19), (13, 25, 0))));
    assert!(h.is_open_extended(local(tz, (2026, 8, 19), (16, 59, 59))));
    assert!(!h.is_open(local(tz, (2026, 8, 19), (17, 0, 0))));
    assert_weekend_closed(Exchange::Twse, tz);
}
