// SPDX-License-Identifier: MIT-0

//! Eurex and ICE normal-week sessions and maintenance gaps.

use super::prelude::*;

// ---------------------------------------------------------------------------
// EUREX (Index / IR)
//   Pre-2018: pre-trading 07:30–07:50, continuous 07:50–22:00
//   Post-2018: fixed-00:00-UTC pre-trading/auction for 15 minutes, then
//              continuous trading through 22:00 CET/CEST
//   No weekend trading.
// ---------------------------------------------------------------------------

#[test]
fn eurex_asian_hours() {
    let h = hours_for_exchange(Exchange::Eurex, utc((2026, 4, 20), (5, 0, 0)));
    let t = cet((2026, 4, 20), (5, 0, 0));
    assert!(h.is_open(t), "EUREX Asian hours Mon 05:00 CET");
    assert!(h.is_open_regular(t), "05:00 is continuous trading");
}

#[test]
fn eurex_asian_start_tracks_fixed_midnight_utc() {
    let summer = hours_for_exchange(Exchange::Eurex, cet((2026, 4, 20), (12, 0, 0)));
    assert!(!summer.is_open(cet((2026, 4, 20), (1, 59, 59))));
    assert!(summer.is_order_entry_only(cet((2026, 4, 20), (2, 0, 0))));
    assert!(summer.is_open_extended(cet((2026, 4, 20), (2, 14, 59))));
    assert!(summer.is_open_regular(cet((2026, 4, 20), (2, 15, 0))));

    let winter = hours_for_exchange(Exchange::Eurex, cet((2026, 1, 19), (12, 0, 0)));
    assert!(!winter.is_open(cet((2026, 1, 19), (0, 59, 59))));
    assert!(winter.is_order_entry_only(cet((2026, 1, 19), (1, 0, 0))));
    assert!(winter.is_open_extended(cet((2026, 1, 19), (1, 14, 59))));
    assert!(winter.is_open_regular(cet((2026, 1, 19), (1, 15, 0))));
}

#[test]
fn eurex_regular_hours() {
    let h = hours_for_exchange(Exchange::Eurex, utc((2026, 4, 20), (10, 0, 0)));
    let t = cet((2026, 4, 20), (10, 0, 0));
    assert!(h.is_open(t), "EUREX regular Mon 10:00 CET");
    assert!(h.is_open_regular(t), "10:00 is regular");
}

#[test]
fn eurex_regular_close() {
    let h = hours_for_exchange(Exchange::Eurex, utc((2026, 4, 20), (22, 0, 0)));
    let t = cet((2026, 4, 20), (22, 0, 0));
    assert!(
        !h.is_open(t),
        "EUREX regular ends 22:00 CET (end-exclusive)"
    );
}

#[test]
fn eurex_friday_close() {
    let h = hours_for_exchange(Exchange::Eurex, utc((2026, 4, 24), (22, 0, 0)));
    let t = cet((2026, 4, 24), (22, 0, 0));
    assert!(!h.is_open(t), "EUREX closes Fri 22:00 CET");
}

#[test]
fn eurex_weekend_closed() {
    let h = hours_for_exchange(Exchange::Eurex, utc((2026, 4, 25), (10, 0, 0)));
    assert!(
        !h.is_open(cet((2026, 4, 25), (10, 0, 0))),
        "EUREX closed Saturday"
    );
    assert!(
        !h.is_open(cet((2026, 4, 26), (10, 0, 0))),
        "EUREX closed Sunday"
    );
}

#[test]
fn eurex_no_maintenance_gap() {
    let h = hours_for_exchange(Exchange::Eurex, utc((2026, 4, 20), (7, 55, 0)));
    let t = cet((2026, 4, 20), (7, 55, 0));
    assert!(h.is_open(t), "EUREX seamless handoff Asian→Regular");
}

#[test]
fn eurex_before_asian_hours() {
    let h = hours_for_exchange(Exchange::Eurex, utc((2026, 4, 20), (0, 30, 0)));
    let t = cet((2026, 4, 20), (0, 30, 0));
    assert!(!h.is_open(t), "EUREX closed before 01:00 CET");
}

#[test]
fn eurex_had_no_asian_session_before_2018_12_10() {
    // The fixed-UTC Asian extension was added 2018-12-10; before that the
    // benchmark contracts had 07:30 pre-trading and continuous trading from
    // 07:50. Pins the exact phase edges on both sides of the cutover.
    let before = hours_for_exchange(Exchange::Eurex, cet((2018, 12, 7), (12, 0, 0)));
    assert!(!before.is_open(cet((2018, 12, 7), (7, 29, 59))));
    assert!(before.is_order_entry_only(cet((2018, 12, 7), (7, 30, 0))));
    assert!(before.is_order_entry_only(cet((2018, 12, 7), (7, 49, 59))));
    assert!(before.is_open_regular(cet((2018, 12, 7), (7, 50, 0))));

    let after = hours_for_exchange(Exchange::Eurex, cet((2018, 12, 10), (12, 0, 0)));
    assert!(!after.is_open(cet((2018, 12, 10), (0, 59, 59))));
    assert!(after.is_order_entry_only(cet((2018, 12, 10), (1, 0, 0))));
    assert!(after.is_open_extended(cet((2018, 12, 10), (1, 14, 59))));
    assert!(after.is_open_regular(cet((2018, 12, 10), (1, 15, 0))));
}

// ---------------------------------------------------------------------------
// ICE Futures U.S. (ICEUS)
//   NYSE FANG+: Pre-Open Sunday 17:30 and weekdays 19:30, followed by
//   Sunday 18:00–Monday 18:00 and 20:00–18:00 ET Mon–Thu trading
//   No Fri overnight.
// ---------------------------------------------------------------------------

#[test]
fn iceus_sunday_open() {
    let h = hours_for_exchange(
        Exchange::Iceus,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(!h.is_open(et((2026, 4, 19), (17, 29, 59))));
    assert!(
        h.is_order_entry_only(et((2026, 4, 19), (17, 30, 0))),
        "ICEUS FANG+ Pre-Open starts Sunday 17:30 ET - order entry, not matching"
    );
    assert!(h.is_open_regular(et((2026, 4, 19), (18, 0, 0))));
}

#[test]
fn iceus_overnight_wrap() {
    let h = hours_for_exchange(
        Exchange::Iceus,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let t = et((2026, 4, 20), (10, 0, 0));
    assert!(h.is_open(t), "ICEUS Mon 10:00 ET (in wrap)");
}

#[test]
fn iceus_daily_close() {
    let h = hours_for_exchange(
        Exchange::Iceus,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let t = et((2026, 4, 20), (18, 0, 0));
    assert!(!h.is_open(t), "ICEUS closes Mon 18:00 ET (end-exclusive)");
}

#[test]
fn iceus_daily_break() {
    let h = hours_for_exchange(
        Exchange::Iceus,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let t = et((2026, 4, 20), (19, 0, 0));
    assert!(!h.is_open(t), "ICEUS in 18:00–20:00 ET break");
    assert!(
        h.is_maintenance(t),
        "19:00 ET is inside the four-hour-bounded 18:00→20:00 break"
    );
}

#[test]
fn maintenance_covers_the_whole_break_not_just_its_tail() {
    // Maintenance is classified by the full close-to-reopen span, so the
    // front of a qualifying break counts too. CBOT's current 13:20-14:30 gap
    // stays within one trade date before PCP and is therefore a Halt.
    let ice = hours_for_exchange(
        Exchange::Iceus,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let t = et((2026, 4, 20), (18, 5, 0));
    assert!(!ice.is_open(t), "ICEUS 18:05 ET is closed");
    assert!(
        ice.is_maintenance(t),
        "the start of ICE's two-hour break is maintenance"
    );

    let eurex = hours_for_exchange(
        Exchange::Eurex,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let t = cet((2026, 4, 20), (22, 30, 0));
    // The matching gap runs 22:00 → 02:10 CEST, because the 02:00-02:10
    // pre-trading phase accepts orders but matches nothing. At four hours ten
    // minutes it exceeds the maintenance bound, so it reads Closed.
    assert_eq!(
        eurex.session_state(t),
        SessionState::Closed,
        "Eurex's 22:00→02:10 non-matching gap exceeds the maintenance bound"
    );
    assert!(
        !eurex.is_open(t),
        "Eurex's current-summer 22:00→02:00 daily gap admits no trading end to end"
    );

    let cbot = hours_for_exchange(
        Exchange::Cbot,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let t = ct((2026, 4, 20), (14, 0, 0));
    assert!(!cbot.is_open(t), "CBOT 14:00 CT is closed");
    assert!(!cbot.is_maintenance(t));
    // 14:00 CT sits between the 13:20 grain close and the 14:30 PCP order-entry
    // window. With the queues out of `extended`, the surrounding sessions belong
    // to different trade dates, so this reads Closed rather than an intraday Halt.
    assert_eq!(cbot.session_state(t), SessionState::Closed);
}

#[test]
fn maintenance_starts_at_the_close_instant() {
    // Closes are end-exclusive, so the close instant itself is the break's
    // first closed instant.
    let h = hours_for_exchange(
        Exchange::Cme,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(
        h.is_maintenance(ct((2026, 4, 20), (16, 0, 0))),
        "16:00 CT (the daily close) is the first instant of the break"
    );
    // CME removed the old 15:15-15:30 CT pause in 2021.
    assert_eq!(
        h.session_state(ct((2026, 4, 20), (15, 20, 0))),
        SessionState::OpenExtended
    );
}

#[test]
fn pre_open_and_overnight_windows_are_not_maintenance() {
    // Closed-but-reopening-soon is not maintenance when the enclosing gap is
    // overnight- or weekend-long. Until 0.2.0 the 90-minute heuristic flagged
    // all of these.
    let cme = hours_for_exchange(
        Exchange::Cme,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(cme.is_order_entry_only(ct((2026, 4, 19), (16, 30, 0))));
    assert!(!cme.is_maintenance(ct((2026, 4, 19), (16, 30, 0))));
    assert!(
        !cme.is_maintenance(ct((2026, 4, 24), (16, 30, 0))),
        "Friday 16:30 CT starts the weekend closure, not a break"
    );
    assert!(
        !cme.is_maintenance(ct((2026, 4, 25), (12, 0, 0))),
        "Saturday is a weekend closure"
    );

    let nasdaq = hours_for_exchange(
        Exchange::Nasdaq,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(
        !nasdaq.is_maintenance(et((2026, 4, 21), (3, 30, 0))),
        "Tuesday 03:30 ET sits in the eight-hour equity overnight, not a break"
    );
}

#[test]
fn iceus_friday_close() {
    let h = hours_for_exchange(
        Exchange::Iceus,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let t = et((2026, 4, 24), (18, 0, 0));
    assert!(!h.is_open(t), "ICEUS closes Fri 18:00 ET");
    assert!(
        !h.is_open(et((2026, 4, 24), (20, 0, 0))),
        "ICEUS no Fri overnight"
    );
}

#[test]
fn iceus_weekend_closed() {
    let h = hours_for_exchange(
        Exchange::Iceus,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(
        !h.is_open(et((2026, 4, 25), (10, 0, 0))),
        "ICEUS closed Saturday"
    );
}

// ICE launched NYSE FANG+ futures at the start of trade date 2017-11-08. The
// same notice defines trading at 20:00 on the preceding local day and Pre-Open
// 30 minutes earlier, pinning first order entry to 2017-11-07 at 19:30 ET.
// https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_FANG%2BFuture_20170926.pdf
#[test]
fn iceus_fang_profile_is_closed_before_its_sourced_launch() {
    let launch_eve = et((2017, 11, 7), (0, 0, 0));
    let closed = hours_for_exchange(Exchange::Iceus, launch_eve - chrono::Duration::seconds(1));
    let first_evening = hours_for_exchange(Exchange::Iceus, launch_eve);

    assert!(!closed.is_open(et((2017, 11, 6), (12, 0, 0))));
    assert!(!first_evening.is_open(et((2017, 11, 7), (19, 29, 59))));
    assert!(first_evening.is_order_entry_only(et((2017, 11, 7), (19, 30, 0))));
    assert!(first_evening.is_open_regular(et((2017, 11, 7), (20, 0, 0))));
    assert!(first_evening.is_open_regular(et((2017, 11, 8), (17, 59, 59))));
    assert!(!first_evening.is_open(et((2017, 11, 8), (18, 0, 0))));

    let full_week = hours_for_exchange(Exchange::Iceus, et((2017, 11, 8), (0, 0, 0)));
    assert!(full_week.is_open_regular(et((2017, 11, 8), (20, 0, 0))));

    let calendar = calendar_for_exchange(Exchange::Iceus);
    assert!(!calendar.is_open(et((2017, 11, 7), (19, 29, 59))));
    assert!(calendar.is_order_entry_only(et((2017, 11, 7), (19, 30, 0))));
    assert!(calendar.is_open_regular(et((2017, 11, 7), (20, 0, 0))));
    assert!(!calendar.is_open(et((2017, 11, 8), (18, 0, 0))));
    assert!(calendar.is_open_regular(et((2017, 11, 8), (20, 0, 0))));
}

#[test]
fn ice_canada_canola_january_2010_baseline() {
    let baseline = hours_for_exchange(Exchange::IceCanada, ct((2010, 1, 10), (12, 0, 0)));
    assert_eq!(baseline.tz, America::Winnipeg);
    assert!(!baseline.is_open(ct((2010, 1, 10), (18, 59, 59))));
    assert!(baseline.is_order_entry_only(ct((2010, 1, 10), (19, 0, 0))));
    assert!(baseline.is_order_entry_only(ct((2010, 1, 10), (19, 59, 59))));
    assert!(baseline.is_open_regular(ct((2010, 1, 10), (20, 0, 0))));
    assert!(baseline.is_open_regular(ct((2010, 1, 11), (13, 14, 59))));
    assert!(!baseline.is_open(ct((2010, 1, 11), (13, 15, 0))));
}

#[test]
fn ice_canada_canola_2011_opening_change_uses_actual_opening_day() {
    let before = hours_for_exchange(Exchange::IceCanada, ct((2011, 2, 27), (12, 0, 0)));
    assert!(!before.is_open(ct((2011, 2, 27), (18, 59, 59))));
    assert!(before.is_order_entry_only(ct((2011, 2, 27), (19, 0, 0))));
    assert!(before.is_open_regular(ct((2011, 2, 27), (20, 0, 0))));

    // The 2011 boundary is the sourced 18:30 CT pre-open instant, so a
    // Monday-noon snapshot still carries the old grid; only from the new
    // pre-open does the revised schedule apply.
    let after = hours_for_exchange(Exchange::IceCanada, ct((2011, 2, 28), (20, 0, 0)));
    assert!(after.is_open_regular(ct((2011, 2, 28), (12, 0, 0))));
    assert!(!after.is_open(ct((2011, 2, 28), (18, 29, 59))));
    assert!(after.is_order_entry_only(ct((2011, 2, 28), (18, 30, 0))));
    assert!(after.is_open_regular(ct((2011, 2, 28), (19, 0, 0))));

    let calendar = calendar_for_exchange(Exchange::IceCanada);
    // The Sunday-opened session runs 20:00 CT through Monday 13:15 CT; local
    // midnight of 2011-02-28 falls inside it and must not split or truncate
    // it.
    assert!(calendar.is_open_regular(ct((2011, 2, 28), (0, 0, 0))));
    assert!(calendar.is_open_regular(ct((2011, 2, 28), (12, 0, 0))));
    assert!(!calendar.is_open(ct((2011, 2, 28), (13, 15, 0))));
    assert!(!calendar.is_open(ct((2011, 2, 28), (18, 29, 59))));
    assert!(calendar.is_order_entry_only(ct((2011, 2, 28), (18, 30, 0))));
    assert!(calendar.is_open_regular(ct((2011, 2, 28), (19, 0, 0))));
}

#[test]
fn ice_canada_canola_2012_close_extension_and_2013_restoration() {
    let before = hours_for_exchange(Exchange::IceCanada, ct((2012, 6, 23), (12, 0, 0)));
    assert!(!before.is_open(ct((2012, 6, 25), (13, 15, 0))));

    let longer = hours_for_exchange(Exchange::IceCanada, ct((2012, 6, 24), (12, 0, 0)));
    assert!(longer.is_open_regular(ct((2012, 6, 25), (13, 59, 59))));
    assert!(!longer.is_open(ct((2012, 6, 25), (14, 0, 0))));

    let restored = hours_for_exchange(Exchange::IceCanada, ct((2013, 4, 7), (12, 0, 0)));
    assert!(restored.is_open_regular(ct((2013, 4, 8), (13, 14, 59))));
    assert!(!restored.is_open(ct((2013, 4, 8), (13, 15, 0))));

    let calendar = calendar_for_exchange(Exchange::IceCanada);
    assert!(calendar.is_open_regular(ct((2012, 6, 25), (13, 59, 59))));
    assert!(!calendar.is_open(ct((2012, 6, 25), (14, 0, 0))));
    assert!(calendar.is_open_regular(ct((2013, 4, 8), (13, 14, 59))));
    assert!(!calendar.is_open(ct((2013, 4, 8), (13, 15, 0))));
}

#[test]
fn ice_canada_canola_2016_close_extension_and_2018_transfer() {
    let before = hours_for_exchange(Exchange::IceCanada, ct((2016, 1, 23), (12, 0, 0)));
    assert!(!before.is_open(ct((2016, 1, 25), (13, 15, 0))));

    let extended = hours_for_exchange(Exchange::IceCanada, ct((2016, 1, 24), (12, 0, 0)));
    assert!(extended.is_open_regular(ct((2016, 1, 25), (13, 19, 59))));
    assert!(!extended.is_open(ct((2016, 1, 25), (13, 20, 0))));

    let calendar = calendar_for_exchange(Exchange::IceCanada);
    assert!(calendar.is_open_regular(ct((2016, 1, 25), (13, 19, 59))));
    assert!(!calendar.is_open(ct((2016, 1, 25), (13, 20, 0))));

    let transfer_opening = ct((2018, 7, 29), (0, 0, 0));
    let legacy = hours_for_exchange(
        Exchange::IceCanada,
        transfer_opening - chrono::Duration::seconds(1),
    );
    assert!(legacy.is_open_regular(ct((2018, 7, 27), (13, 19, 59))));

    let closed = hours_for_exchange(Exchange::IceCanada, transfer_opening);
    assert!(closed.regular.is_empty());
    assert!(closed.extended.is_empty());
    assert_eq!(closed.tz, America::Winnipeg);
    assert!(!calendar.is_open(ct((2018, 7, 29), (19, 0, 0))));

    let current = hours_for_exchange(
        Exchange::IceCanada,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(current.regular.is_empty());
    assert!(current.extended.is_empty());
}

// ---------------------------------------------------------------------------
// ICE Futures Europe Brent Crude Futures, governed by US Eastern time.
// ---------------------------------------------------------------------------

#[test]
fn iceeu_monday_open() {
    let h = hours_for_exchange(
        Exchange::Iceeu,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert_eq!(h.tz, America::New_York);
    assert!(!h.is_open(et((2026, 4, 19), (16, 59, 59))));
    assert!(h.is_order_entry_only(et((2026, 4, 19), (17, 0, 0))));
    assert!(h.is_open_regular(et((2026, 4, 19), (18, 0, 0))));
}

#[test]
fn iceeu_trading_hours() {
    let h = hours_for_exchange(
        Exchange::Iceeu,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(
        h.is_open(lon((2026, 4, 20), (12, 0, 0))),
        "ICEEU Mon 12:00 London"
    );
}

#[test]
fn iceeu_close() {
    let h = hours_for_exchange(
        Exchange::Iceeu,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(h.is_open_regular(et((2026, 4, 20), (17, 59, 59))));
    assert!(!h.is_open(et((2026, 4, 20), (18, 0, 0))));
}

#[test]
fn iceeu_before_open() {
    let h = hours_for_exchange(
        Exchange::Iceeu,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(!h.is_open(et((2026, 4, 20), (19, 44, 59))));
    assert!(h.is_order_entry_only(et((2026, 4, 20), (19, 45, 0))));
    assert!(h.is_open_regular(et((2026, 4, 20), (20, 0, 0))));
}

#[test]
fn iceeu_weekend_closed() {
    let h = hours_for_exchange(
        Exchange::Iceeu,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(
        !h.is_open(lon((2026, 4, 25), (12, 0, 0))),
        "ICEEU closed Saturday"
    );
    assert!(
        !h.is_open(et((2026, 4, 26), (12, 0, 0))),
        "ICEEU closed Sunday before its special pre-open"
    );
}
