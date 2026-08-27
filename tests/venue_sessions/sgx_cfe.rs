// SPDX-License-Identifier: MIT-0

//! SGX and CFE session boundaries.

use super::prelude::*;

// ---------------------------------------------------------------------------
// SGX Three-Month SORA Futures
//   T continuous: Mon–Fri 07:25–17:55 SGT
//   T+1 continuous: Mon–Fri 18:15–05:15 SGT (wrap)
//   Auctions/order entry are extended; 18:00–18:05 is closed.
// ---------------------------------------------------------------------------

#[test]
fn sgx_day_session() {
    let h = hours_for_exchange(
        Exchange::Sgx,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let t = sgt((2026, 4, 20), (10, 0, 0));
    assert!(h.is_open(t), "SGX Mon 10:00 SGT");
    assert!(h.is_open_regular(t), "10:00 is regular (day)");
}

#[test]
fn sgx_day_open() {
    let h = hours_for_exchange(
        Exchange::Sgx,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(!h.is_open(sgt((2026, 4, 20), (7, 9, 59))));
    assert!(h.is_order_entry_only(sgt((2026, 4, 20), (7, 10, 0))));
    assert!(!h.is_open_regular(sgt((2026, 4, 20), (7, 24, 59))));
    assert!(h.is_open_regular(sgt((2026, 4, 20), (7, 25, 0))));
}

#[test]
fn sgx_day_close() {
    let h = hours_for_exchange(
        Exchange::Sgx,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(h.is_open_regular(sgt((2026, 4, 20), (17, 54, 59))));
    assert!(!h.is_open_regular(sgt((2026, 4, 20), (17, 55, 0))));
    assert!(h.is_open_extended(sgt((2026, 4, 20), (17, 55, 0))));
    assert!(!h.is_open(sgt((2026, 4, 20), (18, 0, 0))));
    assert!(h.is_order_entry_only(sgt((2026, 4, 20), (18, 5, 0))));
}

#[test]
fn sgx_t1_wrap() {
    let h = hours_for_exchange(
        Exchange::Sgx,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let t = sgt((2026, 4, 20), (22, 0, 0));
    assert!(h.is_open(t), "SGX T+1 wrap Mon 22:00 SGT");
    assert!(
        h.is_open_regular(t),
        "SORA T+1 continuous trading is regular"
    );
}

#[test]
fn sgx_t1_wrap_into_tuesday() {
    let h = hours_for_exchange(
        Exchange::Sgx,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let t = sgt((2026, 4, 21), (3, 0, 0));
    assert!(h.is_open(t), "SGX T+1 wrap Tue 03:00 SGT");
}

#[test]
fn sgx_t1_wrap_close() {
    let h = hours_for_exchange(
        Exchange::Sgx,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let t = sgt((2026, 4, 21), (5, 15, 0));
    assert!(!h.is_open(t), "SGX T+1 closes 05:15 SGT (end-exclusive)");
}

#[test]
fn sgx_gap_before_day() {
    let h = hours_for_exchange(
        Exchange::Sgx,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let t = sgt((2026, 4, 21), (6, 0, 0));
    assert!(!h.is_open(t), "SGX gap between 05:15 and 07:10 SGT");
}

#[test]
fn sgx_sora_is_closed_before_its_sourced_launch() {
    let cutover = sgt((2024, 7, 29), (0, 0, 0));
    let before = hours_for_exchange(Exchange::Sgx, cutover - chrono::Duration::seconds(1));
    let after = hours_for_exchange(Exchange::Sgx, cutover);

    assert!(before.regular.is_empty());
    assert!(before.extended.is_empty());
    assert!(!before.is_open(sgt((2024, 7, 26), (10, 0, 0))));
    assert!(after.is_order_entry_only(sgt((2024, 7, 29), (7, 10, 0))));
    assert!(after.is_open_regular(sgt((2024, 7, 29), (7, 25, 0))));
}

#[test]
fn sgx_friday_t1_wrap_into_saturday() {
    let h = hours_for_exchange(
        Exchange::Sgx,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let t = sgt((2026, 4, 25), (3, 0, 0));
    assert!(
        h.is_open(t),
        "SGX Fri T+1 wrap Sat 03:00 SGT (Friday's wrap extends into Sat)"
    );
}

#[test]
fn sgx_saturday_after_t1_close() {
    let h = hours_for_exchange(
        Exchange::Sgx,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let t = sgt((2026, 4, 25), (6, 0, 0));
    assert!(!h.is_open(t), "SGX closed Sat after 05:15 SGT");
}

#[test]
fn sgx_sunday_closed() {
    let h = hours_for_exchange(
        Exchange::Sgx,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(
        !h.is_open(sgt((2026, 4, 26), (10, 0, 0))),
        "SGX closed Sunday"
    );
}

// ---------------------------------------------------------------------------
// CFE (Cboe Futures — VIX)
//   RTH 08:30–15:00 CT, post-settlement ETH 15:00–16:00 CT (Mon–Fri),
//   Sunday queue 16:00:06–17:00, Mon–Thu queue 16:45:06–17:00, and overnight
//   Sun+Mon–Thu 17:00→08:30. Queue starts use the conservative latest edge of
//   CFE's randomized acceptance window. Effective 2021-12-06.
// ---------------------------------------------------------------------------

#[test]
fn cfe_sunday_overnight() {
    let h = hours_for_exchange(
        Exchange::Cfe,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let t = ct((2026, 4, 19), (17, 0, 0));
    assert!(h.is_open(t), "CFE overnight trading starts Sun 17:00 CT");
}

#[test]
fn cfe_pre_open_queues_are_extended() {
    // CFE-2021-028 P&P XIX accepts non-executable orders in queues whose
    // starts are randomized through six seconds after 16:00 / 16:45.
    // https://cdn.cboe.com/resources/regulation/rule_filings/pending/2021/21-028-VX-VXM-and-AMERIBOR-Trading-Hours.pdf
    for h in [
        hours_for_exchange(
            Exchange::Cfe,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
        ),
        hours_for_market_hours_key(
            MarketHoursKey::CfeVix,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
        ),
    ] {
        assert!(!h.is_open(ct((2026, 4, 19), (16, 0, 5))));
        assert!(h.is_order_entry_only(ct((2026, 4, 19), (16, 0, 6))));
        assert!(h.is_order_entry_only(ct((2026, 4, 19), (16, 30, 0))));
        assert!(!h.is_open(ct((2026, 4, 20), (16, 45, 5))));
        assert!(h.is_order_entry_only(ct((2026, 4, 20), (16, 45, 6))));
    }
}

#[test]
fn cfe_rth() {
    let h = hours_for_exchange(
        Exchange::Cfe,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let t = ct((2026, 4, 20), (10, 0, 0));
    assert!(h.is_open(t), "CFE RTH Mon 10:00 CT");
    assert!(h.is_open_regular(t), "10:00 is RTH");
}

#[test]
fn cfe_rth_close() {
    let h = hours_for_exchange(
        Exchange::Cfe,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(
        !h.is_open_regular(ct((2026, 4, 20), (15, 0, 0))),
        "CFE RTH ends 15:00 CT (end-exclusive) since 2021-12-06"
    );
    assert!(
        !h.is_open_regular(ct((2026, 4, 20), (15, 15, 0))),
        "15:15 CT is past the RTH close"
    );
}

#[test]
fn cfe_post_settlement_window() {
    let h = hours_for_exchange(
        Exchange::Cfe,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let t = ct((2026, 4, 20), (15, 45, 0));
    assert!(h.is_open(t), "CFE post-settlement ETH 15:00–16:00 CT");
    assert!(h.is_open_extended(t), "post-settlement is extended");
}

#[test]
fn cfe_daily_maintenance() {
    let h = hours_for_exchange(
        Exchange::Cfe,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let t = ct((2026, 4, 20), (16, 30, 0));
    assert!(!h.is_open(t), "CFE suspension 16:00–16:45 CT");
}

#[test]
fn cfe_friday_close() {
    let h = hours_for_exchange(
        Exchange::Cfe,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(
        !h.is_open(ct((2026, 4, 24), (16, 0, 0))),
        "CFE closes Fri 16:00 CT"
    );
    assert!(
        !h.is_open(ct((2026, 4, 24), (17, 0, 0))),
        "CFE no Fri overnight"
    );
}

// SR-CFE-2010-013 introduced a 07:20-08:30 CT VX extended session effective
// 2010-12-10; the prior normal week began at the 08:30 RTH open.
// https://cdn.cboe.com/resources/regulation/rule_filings/approved/2010/SR-CFE-2010-013.pdf
#[test]
fn cfe_morning_extension_began_on_2010_12_10() {
    let cutover = ct((2010, 12, 10), (0, 0, 0));
    let before = hours_for_exchange(Exchange::Cfe, cutover - chrono::Duration::seconds(1));
    let after = hours_for_exchange(Exchange::Cfe, cutover);

    assert!(!before.is_open(ct((2010, 12, 10), (7, 20, 0))));
    assert!(before.is_open_regular(ct((2010, 12, 10), (8, 30, 0))));
    assert!(!after.is_open(ct((2010, 12, 10), (7, 19, 59))));
    assert!(after.is_open_extended(ct((2010, 12, 10), (7, 20, 0))));
    assert!(after.is_open_regular(ct((2010, 12, 10), (8, 30, 0))));

    let calendar = calendar_for_exchange(Exchange::Cfe);
    assert!(!calendar.is_open(ct((2010, 12, 10), (7, 19, 59))));
    assert!(calendar.is_open_extended(ct((2010, 12, 10), (7, 20, 0))));
}

// SR-CFE-2011-019 moved the VX extended-session start from 07:20 to 07:00 CT
// effective 2011-09-26; the 08:30 RTH boundary stayed unchanged.
// https://cdn.cboe.com/resources/regulation/rule_filings/approved/2011/SR-CFE-2011-019.pdf
#[test]
fn cfe_morning_extension_moved_to_0700_on_2011_09_26() {
    let cutover = ct((2011, 9, 26), (0, 0, 0));
    let before = hours_for_exchange(Exchange::Cfe, cutover - chrono::Duration::seconds(1));
    let after = hours_for_exchange(Exchange::Cfe, cutover);

    assert!(!before.is_open(ct((2011, 9, 26), (7, 0, 0))));
    assert!(before.is_open_extended(ct((2011, 9, 26), (7, 20, 0))));
    assert!(!after.is_open(ct((2011, 9, 26), (6, 59, 59))));
    assert!(after.is_open_extended(ct((2011, 9, 26), (7, 0, 0))));
    assert!(after.is_open_regular(ct((2011, 9, 26), (8, 30, 0))));

    let calendar = calendar_for_exchange(Exchange::Cfe);
    assert!(!calendar.is_open(ct((2011, 9, 26), (6, 59, 59))));
    assert!(calendar.is_open_extended(ct((2011, 9, 26), (7, 0, 0))));
}

// CFE IC13-041 pins the two phases to 2013-10-28 and 2013-11-04 and publishes
// the 15:29–15:30 Monday–Thursday pre-open in both resulting schedules. Cboe's
// year-end retrospective independently records both actual launch dates.
// https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2013-041.pdf
// https://ir.cboe.com/news/news-details/2013/CBOE-Futures-Exchange-Announces-Launch-Dates-For-VIX-Futures-Extended-Trading-Hours-09-30-2013/default.aspx
// https://ir.cboe.com/news/news-details/2014/2013-Trading-Volume-Reaches-New-All-Time-High-At-CBOE-Futures-Exchange-01-02-2014/default.aspx
#[test]
fn cfe_afternoon_extension_launched_on_2013_10_28() {
    let cutover = ct((2013, 10, 28), (0, 0, 0));
    let before = hours_for_exchange(Exchange::Cfe, cutover - chrono::Duration::seconds(1));
    let after = hours_for_exchange(Exchange::Cfe, cutover);

    assert!(before.is_open_extended(ct((2013, 10, 28), (7, 0, 0))));
    assert!(!before.is_open(ct((2013, 10, 28), (15, 29, 0))));
    assert!(!before.is_open(ct((2013, 10, 28), (15, 30, 0))));

    assert!(!after.is_open(ct((2013, 10, 28), (15, 28, 59))));
    assert!(after.is_order_entry_only(ct((2013, 10, 28), (15, 29, 0))));
    assert!(after.is_order_entry_only(ct((2013, 10, 28), (15, 29, 59))));
    assert!(after.is_open_extended(ct((2013, 10, 28), (15, 30, 0))));
    assert!(after.is_open_extended(ct((2013, 10, 28), (16, 14, 59))));
    assert!(!after.is_open(ct((2013, 10, 28), (16, 15, 0))));
    assert!(
        !after.is_open(ct((2013, 11, 1), (15, 30, 0))),
        "the new afternoon period ran Monday–Thursday, not Friday"
    );

    let calendar = calendar_for_exchange(Exchange::Cfe);
    assert!(!calendar.is_open(ct((2013, 10, 28), (15, 28, 59))));
    assert!(calendar.is_order_entry_only(ct((2013, 10, 28), (15, 29, 0))));
    assert!(calendar.is_open_extended(ct((2013, 10, 28), (15, 30, 0))));
}

#[test]
fn cfe_morning_extension_launched_on_2013_11_04() {
    let cutover = ct((2013, 11, 4), (0, 0, 0));
    let before = hours_for_exchange(Exchange::Cfe, cutover - chrono::Duration::seconds(1));
    let after = hours_for_exchange(Exchange::Cfe, cutover);

    assert!(!before.is_open(ct((2013, 11, 4), (2, 0, 0))));
    assert!(before.is_open_extended(ct((2013, 11, 4), (7, 0, 0))));
    assert!(before.is_order_entry_only(ct((2013, 11, 4), (15, 29, 0))));

    assert!(!after.is_open(ct((2013, 11, 4), (1, 59, 59))));
    assert!(after.is_open_extended(ct((2013, 11, 4), (2, 0, 0))));
    assert!(after.is_open_extended(ct((2013, 11, 4), (8, 29, 59))));
    assert!(!after.is_open_extended(ct((2013, 11, 4), (8, 30, 0))));
    assert!(after.is_open_regular(ct((2013, 11, 4), (8, 30, 0))));
    assert!(after.is_order_entry_only(ct((2013, 11, 4), (15, 29, 0))));

    let calendar = calendar_for_exchange(Exchange::Cfe);
    assert!(!calendar.is_open(ct((2013, 11, 4), (1, 59, 59))));
    assert!(calendar.is_open_extended(ct((2013, 11, 4), (2, 0, 0))));
}

// CFE-2014-010 records the continuous Mon–Thu 15:30→08:30 schedule. IC14-036
// publishes the new 16:15–17:00 Sunday pre-open and retained 15:29–15:30
// weekday pre-open; RG-CFE-2014-020 pins the launch to Sunday 2014-06-22.
// https://cdn.cboe.com/resources/regulation/rule_filings/approved/2014/SR-CFE-2014-010.pdf
// https://ir.cboe.com/news/news-details/2014/CBOE-Futures-Exchange-Set-For-June-22-Launch-Of-24-Hour-VIX-Futures-Trading-06-09-2014/default.aspx
// https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2014-036.pdf
// https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2014-020.pdf
#[test]
fn cfe_nearly_24_hour_week_launched_on_2014_06_22() {
    let cutover = ct((2014, 6, 22), (0, 0, 0));
    let before = hours_for_exchange(Exchange::Cfe, cutover - chrono::Duration::seconds(1));
    let after = hours_for_exchange(Exchange::Cfe, cutover);

    assert!(before.is_open_extended(ct((2014, 6, 20), (2, 0, 0))));
    assert!(before.is_open_extended(ct((2014, 6, 19), (16, 14, 59))));
    assert!(!before.is_open(ct((2014, 6, 19), (16, 15, 0))));
    assert!(!before.is_open(ct((2014, 6, 22), (17, 0, 0))));

    assert!(!after.is_open(ct((2014, 6, 22), (16, 14, 59))));
    assert!(after.is_order_entry_only(ct((2014, 6, 22), (16, 15, 0))));
    assert!(after.is_order_entry_only(ct((2014, 6, 22), (16, 59, 59))));
    assert!(after.is_open_extended(ct((2014, 6, 22), (17, 0, 0))));
    assert!(!after.is_open(ct((2014, 6, 23), (15, 28, 59))));
    assert!(after.is_order_entry_only(ct((2014, 6, 23), (15, 29, 0))));
    assert!(after.is_open_extended(ct((2014, 6, 23), (15, 30, 0))));
    assert!(after.is_open_extended(ct((2014, 6, 23), (16, 30, 0))));
    assert!(after.is_open_extended(ct((2014, 6, 24), (2, 0, 0))));

    let calendar = calendar_for_exchange(Exchange::Cfe);
    assert!(!calendar.is_open(ct((2014, 6, 22), (16, 14, 59))));
    assert!(calendar.is_order_entry_only(ct((2014, 6, 22), (16, 15, 0))));
    assert!(calendar.is_order_entry_only(ct((2014, 6, 22), (16, 59, 59))));
    assert!(calendar.is_open_extended(ct((2014, 6, 22), (17, 0, 0))));
}

// SR-CFE-2017-017 tied the 16:00–16:45 CT suspension and randomized opening
// queues to CFE's system migration. RG18-005 confirms the migration completed
// Sunday 2018-02-25 for business date Monday 2018-02-26.
// https://cdn.cboe.com/resources/regulation/rule_filings/approved/2017/SR-CFE-2017-017.pdf
// https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2018-005.pdf
#[test]
fn cfe_system_migration_restored_daily_gap_on_2018_02_25() {
    let cutover = ct((2018, 2, 25), (0, 0, 0));
    let before = hours_for_exchange(Exchange::Cfe, cutover - chrono::Duration::seconds(1));
    let after = hours_for_exchange(Exchange::Cfe, cutover);
    let monday = (2018, 2, 26);

    assert!(before.is_open_extended(ct(monday, (16, 59, 59))));
    assert!(!before.is_open(ct((2018, 2, 25), (16, 0, 0))));
    assert!(before.is_order_entry_only(ct((2018, 2, 25), (16, 15, 0))));

    assert!(!after.is_open(ct((2018, 2, 25), (16, 0, 2))));
    assert!(after.is_order_entry_only(ct((2018, 2, 25), (16, 0, 3))));
    assert!(after.is_order_entry_only(ct(monday, (15, 15, 0))));
    assert!(after.is_order_entry_only(ct(monday, (15, 20, 0))));
    assert!(!after.is_open_regular(ct(monday, (15, 20, 0))));
    assert!(!after.is_open(ct(monday, (16, 45, 2))));
    assert!(after.is_order_entry_only(ct(monday, (16, 45, 3))));
    assert!(after.is_open_extended(ct(monday, (15, 30, 0))));
    assert!(after.is_open_extended(ct(monday, (15, 59, 59))));
    assert!(!after.is_open(ct(monday, (16, 0, 0))));
    assert!(after.is_order_entry_only(ct(monday, (16, 59, 59))));
    assert!(after.is_open_extended(ct(monday, (17, 0, 0))));

    let calendar = calendar_for_exchange(Exchange::Cfe);
    assert!(!calendar.is_open(ct(monday, (16, 30, 0))));
    assert!(!calendar.is_open(ct(monday, (16, 45, 2))));
    assert!(calendar.is_order_entry_only(ct(monday, (16, 45, 3))));
    assert!(calendar.is_open_extended(ct(monday, (17, 0, 0))));
}

// C2018071603 moved TAS opening-queue starts into the three-to-six-second
// randomized window effective with the Sunday 2018-08-12 opening. The
// all-contract conservative edge therefore moved from +3 to +6 seconds.
// https://cdn.cboe.com/resources/release_notes/2018/Change-to-CFE-Pre-Open-Time-for-TAS-Contracts-and-Order-Submission-Commencement-Times.pdf
#[test]
fn cfe_queue_envelope_widened_on_2018_08_12() {
    let cutover = ct((2018, 8, 12), (0, 0, 0));
    let exchange_before = hours_for_exchange(Exchange::Cfe, cutover - chrono::Duration::seconds(1));
    let exchange_after = hours_for_exchange(Exchange::Cfe, cutover);
    let key_before = hours_for_market_hours_key(
        MarketHoursKey::CfeVix,
        cutover - chrono::Duration::seconds(1),
    );
    let key_after = hours_for_market_hours_key(MarketHoursKey::CfeVix, cutover);

    for (before, after) in [(exchange_before, exchange_after), (key_before, key_after)] {
        assert!(before.is_order_entry_only(ct((2018, 8, 12), (16, 0, 3))));
        assert!(!after.is_open(ct((2018, 8, 12), (16, 0, 5))));
        assert!(after.is_order_entry_only(ct((2018, 8, 12), (16, 0, 6))));
        assert!(before.is_order_entry_only(ct((2018, 8, 13), (16, 45, 3))));
        assert!(!after.is_open(ct((2018, 8, 13), (16, 45, 5))));
        assert!(after.is_order_entry_only(ct((2018, 8, 13), (16, 45, 6))));
    }

    let calendar = calendar_for_exchange(Exchange::Cfe);
    assert!(!calendar.is_open(ct((2018, 8, 12), (16, 0, 5))));
    assert!(calendar.is_order_entry_only(ct((2018, 8, 12), (16, 0, 6))));
}
