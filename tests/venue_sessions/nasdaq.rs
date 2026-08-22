// SPDX-License-Identifier: MIT-0

//! Nasdaq Stock Market current snapshot and sourced historical contracts.

use super::prelude::*;

#[test]
fn nasdaq_bx_current_snapshot_uses_texas_0700_to_1900_hours() {
    // Nasdaq's current system-hours sheet names the venue Nasdaq Texas after
    // its 2026 rename. The crate intentionally preserves the stable BX wire
    // identity while using the operator's 07:00–19:00 ET schedule.
    // https://www.nasdaqtrader.com/content/technicalsupport/nasdaq_sys_hours.pdf
    let hours = hours_for_exchange(Exchange::NasdaqBx);
    let monday = (2026, 4, 20);

    assert!(!hours.is_open(et(monday, (6, 59, 59))));
    assert!(hours.is_open_extended(et(monday, (7, 0, 0))));
    assert!(hours.is_open_regular(et(monday, (9, 30, 0))));
    assert!(hours.is_open_extended(et(monday, (18, 59, 59))));
    assert!(!hours.is_open(et(monday, (19, 0, 0))));
    assert!(!hours.is_open(et((2026, 4, 25), (12, 0, 0))));
    assert_eq!(
        serde_json::to_string(&Exchange::NasdaqBx).unwrap(),
        "\"nasdaq_bx\""
    );
}

#[test]
fn nasdaq_psx_current_snapshot_uses_0800_to_1700_hours() {
    // PSX Rule 3100 defines System Hours 08:00–17:00 ET, with Market Hours
    // 09:30–16:00 ET.
    // https://listingcenter.nasdaq.com/rulebook/phlx/rules/phlx-psx-legacy-3000
    let hours = hours_for_exchange(Exchange::NasdaqPsx);
    let monday = (2026, 4, 20);

    assert!(!hours.is_open(et(monday, (7, 59, 59))));
    assert!(hours.is_open_extended(et(monday, (8, 0, 0))));
    assert!(hours.is_open_regular(et(monday, (9, 30, 0))));
    assert!(hours.is_open_extended(et(monday, (16, 59, 59))));
    assert!(!hours.is_open(et(monday, (17, 0, 0))));
    assert!(!hours.is_open(et((2026, 4, 25), (12, 0, 0))));
    assert_eq!(
        serde_json::to_string(&Exchange::NasdaqPsx).unwrap(),
        "\"nasdaq_psx\""
    );
}

#[test]
fn nasdaq_0400_premarket_started_on_2013_03_18() {
    // Nasdaq Equity Trader Alert 2013-21 explicitly moved the pre-market open
    // from 07:00 to 04:00 ET on Monday 2013-03-18.
    // https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2013-21
    let cutover = et((2013, 3, 18), (0, 0, 0));
    let before = hours_for_exchange_as_of(Exchange::Nasdaq, cutover - chrono::Duration::seconds(1));
    let after = hours_for_exchange_as_of(Exchange::Nasdaq, cutover);

    assert!(!before.is_open(et((2013, 3, 18), (6, 59, 59))));
    assert!(before.is_open_extended(et((2013, 3, 18), (7, 0, 0))));
    assert!(!after.is_open(et((2013, 3, 18), (3, 59, 59))));
    assert!(after.is_open_extended(et((2013, 3, 18), (4, 0, 0))));
    assert!(calendar_for_exchange(Exchange::Nasdaq).is_open_extended(et((2013, 3, 18), (4, 0, 0))));
}

#[test]
fn nasdaq_psx_launch_and_0800_expansion_use_sourced_dates() {
    // Nasdaq dated the PSX launch to 2010-10-08. SR-Phlx-2010-172 then named
    // 2010-12-13 for the expansion from a 09:00 to an 08:00 ET system open.
    // https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2010-56
    // https://www.sec.gov/files/rules/sro/phlx/2010/34-63492.pdf
    let launch = et((2010, 10, 8), (0, 0, 0));
    let closed =
        hours_for_exchange_as_of(Exchange::NasdaqPsx, launch - chrono::Duration::seconds(1));
    let launched = hours_for_exchange_as_of(Exchange::NasdaqPsx, launch);

    assert!(closed.regular.is_empty());
    assert!(closed.extended.is_empty());
    assert!(!launched.is_open(et((2010, 10, 8), (8, 59, 59))));
    assert!(launched.is_open_extended(et((2010, 10, 8), (9, 0, 0))));
    assert!(launched.is_open_regular(et((2010, 10, 8), (9, 30, 0))));
    assert!(launched.is_open_extended(et((2010, 10, 8), (16, 0, 0))));
    assert!(!launched.is_open(et((2010, 10, 8), (17, 0, 0))));

    let expansion = et((2010, 12, 13), (0, 0, 0));
    let before = hours_for_exchange_as_of(
        Exchange::NasdaqPsx,
        expansion - chrono::Duration::seconds(1),
    );
    let after = hours_for_exchange_as_of(Exchange::NasdaqPsx, expansion);

    assert!(!before.is_open(et((2010, 12, 13), (8, 0, 0))));
    assert!(after.is_open_extended(et((2010, 12, 13), (8, 0, 0))));
    assert!(
        calendar_for_exchange(Exchange::NasdaqPsx).is_open_extended(et((2010, 12, 13), (8, 0, 0)))
    );
}

// Nasdaq's announced Night Session remains conditional on data-plan readiness
// and a later Nasdaq filing. Until that confirmation exists, even future-dated
// lookups deliberately retain the current 04:00–20:00 profile.
// https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2026-46
// https://listingcenter.nasdaq.com/assets/RuleBook/Nasdaq/rules/Nasdaq%20Equity%201.html

#[test]
fn nasdaq_unconfirmed_night_session_is_not_encoded() {
    let current = hours_for_exchange(Exchange::Nasdaq);
    let future = hours_for_exchange_as_of(Exchange::Nasdaq, et((2026, 12, 7), (12, 0, 0)));

    for hours in [&current, &future] {
        assert!(!hours.is_open(et((2026, 12, 6), (21, 0, 0))));
        assert!(hours.is_open_extended(et((2026, 12, 7), (4, 0, 0))));
        assert!(hours.is_open_regular(et((2026, 12, 7), (9, 30, 0))));
        assert!(hours.is_open_extended(et((2026, 12, 7), (16, 0, 0))));
        assert!(!hours.is_open(et((2026, 12, 7), (20, 0, 0))));
        assert!(!hours.is_open(et((2026, 12, 7), (21, 0, 0))));
    }
    assert!(!calendar_for_exchange(Exchange::Nasdaq).is_open(et((2026, 12, 6), (21, 0, 0))));
}
