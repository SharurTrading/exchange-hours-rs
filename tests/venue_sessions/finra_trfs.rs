// SPDX-License-Identifier: MIT-0

//! FINRA Trade Reporting Facility current and historical contracts.

use super::prelude::*;

fn assert_current_baseline_and_session_kinds(exchange: Exchange) {
    let hours = hours_for_exchange(exchange);
    let monday = (2026, 4, 20);

    assert!(!hours.is_open(et(monday, (3, 59, 59))));
    assert!(hours.is_open_extended(et(monday, (4, 0, 0))));
    assert!(!hours.is_open_regular(et(monday, (4, 0, 0))));
    assert!(hours.is_open_extended(et(monday, (9, 29, 59))));
    assert!(!hours.is_open_extended(et(monday, (9, 30, 0))));
    assert!(hours.is_open_regular(et(monday, (9, 30, 0))));
    assert!(hours.is_open_regular(et(monday, (15, 59, 59))));
    assert!(!hours.is_open_regular(et(monday, (16, 0, 0))));
    assert!(hours.is_open_extended(et(monday, (16, 0, 0))));
    assert!(hours.is_open_extended(et(monday, (19, 59, 59))));
    assert!(!hours.is_open(et(monday, (20, 0, 0))));
    assert!(!hours.is_open(et((2026, 4, 25), (10, 0, 0))));
}

fn assert_2026_opening_cutover(exchange: Exchange) {
    let cutover = et((2026, 3, 30), (0, 0, 0));
    let before = hours_for_exchange_as_of(exchange, cutover - chrono::Duration::seconds(1));
    let after = hours_for_exchange_as_of(exchange, cutover);
    let prior_business_day = (2026, 3, 27);
    let effective_day = (2026, 3, 30);

    assert!(!before.is_open(et(prior_business_day, (7, 59, 59))));
    assert!(before.is_open_extended(et(prior_business_day, (8, 0, 0))));
    assert!(before.is_open_regular(et(prior_business_day, (9, 30, 0))));
    assert!(before.is_open_extended(et(prior_business_day, (16, 0, 0))));

    assert!(!after.is_open(et(effective_day, (3, 59, 59))));
    assert!(after.is_open_extended(et(effective_day, (4, 0, 0))));
    assert!(!after.is_open_regular(et(effective_day, (4, 0, 0))));

    let calendar = calendar_for_exchange(exchange);
    assert!(!calendar.is_open(et(prior_business_day, (4, 0, 0))));
    assert!(calendar.is_open_extended(et(effective_day, (4, 0, 0))));
}

fn assert_2026_overnight_cutover(exchange: Exchange) {
    let cutover = et((2026, 12, 6), (0, 0, 0));
    let before = hours_for_exchange_as_of(exchange, cutover - chrono::Duration::seconds(1));
    let after = hours_for_exchange_as_of(exchange, cutover);
    let sunday = (2026, 12, 6);
    let monday = (2026, 12, 7);

    assert_ne!(before.extended, after.extended);
    assert!(!before.is_open(et(sunday, (21, 0, 0))));
    assert!(!after.is_open(et(sunday, (20, 59, 59))));
    assert!(after.is_open_extended(et(sunday, (21, 0, 0))));
    assert!(after.is_open_extended(et(monday, (9, 29, 59))));
    assert!(!after.is_open_extended(et(monday, (9, 30, 0))));
    assert!(after.is_open_regular(et(monday, (9, 30, 0))));

    for day in 7..=10 {
        let date = (2026, 12, day);
        assert!(after.is_open_extended(et(date, (19, 59, 59))));
        assert!(!after.is_open(et(date, (20, 0, 0))));
        assert!(!after.is_open(et(date, (20, 59, 59))));
        assert!(after.is_maintenance(et(date, (20, 30, 0))));
        assert!(after.is_open_extended(et(date, (21, 0, 0))));
    }

    assert!(after.is_open_extended(et((2026, 12, 11), (19, 59, 59))));
    assert!(!after.is_open(et((2026, 12, 11), (20, 0, 0))));
    assert!(!after.is_open(et((2026, 12, 11), (21, 0, 0))));
    assert!(!after.is_open(et((2026, 12, 12), (10, 0, 0))));

    let calendar = calendar_for_exchange(exchange);
    assert!(calendar.is_open_extended(et(sunday, (21, 0, 0))));
    assert!(!calendar.is_open(et(monday, (20, 30, 0))));
    assert!(calendar.is_open_extended(et(monday, (21, 0, 0))));
}

// FINRA Regulatory Notice 25-15 states that Carteret, Chicago, and the NYSE
// TRF moved from 08:00 to 04:00 ET on 2026-03-30. It identifies 09:30–16:00 as
// regular hours and the 04:00–09:30 / 16:00–20:00 windows as outside RTH.
// https://www.finra.org/rules-guidance/notices/25-15
// SR-FINRA-2026-015 announces the Sunday-through-Friday regime for 2026-12-06,
// conditional on the SIP Amendment launching then; these contracts cover that
// announced profile.
// https://www.finra.org/sites/default/files/2026-07/SR-FINRA-2026-015.pdf

#[test]
fn finra_trf_carteret_baseline_cutover_and_session_kinds() {
    assert_current_baseline_and_session_kinds(Exchange::FinraTrfCarteret);
    assert_2026_opening_cutover(Exchange::FinraTrfCarteret);
    assert_2026_overnight_cutover(Exchange::FinraTrfCarteret);
}

#[test]
fn finra_trf_chicago_baseline_cutover_and_session_kinds() {
    assert_current_baseline_and_session_kinds(Exchange::FinraTrfChicago);
    assert_2026_opening_cutover(Exchange::FinraTrfChicago);
    assert_2026_overnight_cutover(Exchange::FinraTrfChicago);
}

#[test]
fn finra_trf_chicago_is_closed_before_its_sourced_launch() {
    // FINRA states that the Chicago facility commenced operation 2018-09-10.
    // It was limited to test securities through 2018-09-21, with all NMS
    // stocks enabled from 2018-09-24.
    // https://www.finra.org/filing-reporting/trf/technical-notices/reminder-finranasdaq-trf-chicago
    let cutover = et((2018, 9, 10), (0, 0, 0));
    let before = hours_for_exchange_as_of(
        Exchange::FinraTrfChicago,
        cutover - chrono::Duration::seconds(1),
    );
    let launched = hours_for_exchange_as_of(Exchange::FinraTrfChicago, cutover);

    assert!(before.regular.is_empty());
    assert!(before.extended.is_empty());
    assert!(!launched.is_open(et((2018, 9, 10), (7, 59, 59))));
    assert!(launched.is_open_extended(et((2018, 9, 10), (8, 0, 0))));
    assert!(launched.is_open_regular(et((2018, 9, 10), (9, 30, 0))));
    assert!(!launched.is_open(et((2018, 9, 10), (20, 0, 0))));
    assert!(
        calendar_for_exchange(Exchange::FinraTrfChicago)
            .is_open_regular(et((2018, 9, 10), (10, 0, 0)))
    );
}

#[test]
fn finra_trf_nyse_baseline_cutover_and_session_kinds() {
    assert_current_baseline_and_session_kinds(Exchange::FinraTrfNyse);
    assert_2026_opening_cutover(Exchange::FinraTrfNyse);
    assert_2026_overnight_cutover(Exchange::FinraTrfNyse);
}
