// SPDX-License-Identifier: MIT-0

//! Point-in-time Southeast Asian cash-equity schedule revisions.

use super::prelude::*;

#[test]
fn bursa_january_2010_baseline_matches_current_grid() {
    let tz = Asia::Kuala_Lumpur;
    let as_of = local(tz, (2010, 1, 4), (12, 0, 0));
    let probe = (2026, 8, 19);
    let hours = hours_for_exchange_as_of(Exchange::BursaMalaysia, as_of);

    assert!(hours.is_open_regular(local(tz, probe, (12, 29, 59))));
    assert!(!hours.is_open(local(tz, probe, (12, 30, 0))));
}

#[test]
fn thailand_cutover() {
    let tz = Asia::Bangkok;
    let (pre, post) = cutover_sides(Exchange::SetThailand, tz, (2024, 3, 25));
    let at_1415 = local(tz, (2026, 8, 19), (14, 15, 0));
    assert!(!pre.is_open_regular(at_1415));
    assert!(pre.is_open_extended(at_1415));
    assert!(post.is_open_regular(at_1415));
}

#[test]
fn thailand_dr_night_launch_and_trade_date() {
    let tz = Asia::Bangkok;
    let calendar = calendar_for_exchange(Exchange::SetThailand);
    let monday_lunch = local(tz, (2025, 5, 5), (12, 30, 0));
    let launch_lunch = local(tz, (2025, 5, 6), (12, 30, 0));
    let prelaunch_tail = local(tz, (2025, 5, 6), (2, 50, 0));
    let night_preopen = local(tz, (2025, 5, 6), (18, 45, 0));
    let night_regular = local(tz, (2025, 5, 6), (19, 0, 0));
    let night_close_call = local(tz, (2025, 5, 7), (2, 45, 0));
    let final_close = local(tz, (2025, 5, 7), (3, 0, 0));

    assert!(!calendar.is_open(monday_lunch));
    assert!(!calendar.is_open(prelaunch_tail));
    assert!(calendar.is_open_regular(launch_lunch));
    assert!(calendar.is_open_extended(night_preopen));
    assert!(calendar.is_open_regular(night_regular));
    assert!(calendar.is_open_extended(night_close_call));
    assert!(!calendar.is_open(final_close));

    let trade_date = launch_lunch.with_timezone(&tz).date_naive();
    for instant in [launch_lunch, night_preopen, night_regular, night_close_call] {
        assert_eq!(calendar.trade_date(instant), Some(trade_date));
    }
    assert_eq!(
        calendar.candle_end(night_regular, CalendarResolution::Daily),
        Some(final_close)
    );
    assert_eq!(
        calendar.session_state(local(tz, (2025, 5, 6), (17, 30, 0))),
        SessionState::Halt
    );
    assert_eq!(calendar.session_state(final_close), SessionState::Closed);
    assert!(!calendar.is_closed_trade_date(trade_date, SessionKind::Both));
}

#[test]
fn thailand_monthly_candles_group_the_after_midnight_close_by_trade_date() {
    let tz = Asia::Bangkok;
    let calendar = calendar_for_exchange(Exchange::SetThailand);
    let march_31 = local(tz, (2026, 3, 31), (12, 0, 0));
    let march_close = local(tz, (2026, 4, 1), (3, 0, 0));

    assert_eq!(
        calendar.candle_end(march_31, CalendarResolution::Daily),
        Some(march_close)
    );
    assert_eq!(
        calendar.candle_end(march_31, CalendarResolution::Monthly),
        Some(march_close)
    );
    assert_eq!(
        calendar.candle_start(march_31, CalendarResolution::Monthly),
        Some(local(tz, (2026, 3, 2), (9, 30, 0)))
    );
}

#[test]
fn indonesia_cutovers() {
    let tz = Asia::Jakarta;
    let probe = (2026, 8, 19);

    let (pre, post) = cutover_sides(Exchange::Idx, tz, (2013, 1, 2));
    let at_0910 = local(tz, probe, (9, 10, 0));
    assert!(!pre.is_open(local(tz, probe, (9, 9, 59))));
    assert!(pre.is_open_extended(at_0910));
    assert!(pre.is_open_extended(local(tz, probe, (9, 29, 59))));
    assert!(post.is_open_regular(at_0910));

    let (pre, post) = cutover_sides(Exchange::Idx, tz, (2020, 3, 30));
    let at_1530 = local(tz, probe, (15, 30, 0));
    assert!(pre.is_open_regular(at_1530));
    assert!(!post.is_open(at_1530));

    let (pre, post) = cutover_sides(Exchange::Idx, tz, (2023, 4, 3));
    assert!(!pre.is_open(at_1530));
    assert!(post.is_open_regular(at_1530));
    let at_1620 = local(tz, probe, (16, 20, 0));
    assert!(!pre.is_open(at_1620));
    assert!(post.is_open_extended(at_1620));
}

#[test]
fn philippines_cutovers() {
    let tz = Asia::Manila;
    let probe = (2026, 8, 19);

    let (pre, post) = cutover_sides(Exchange::Pse, tz, (2011, 10, 1));
    let at_1230 = local(tz, probe, (12, 30, 0));
    assert!(!pre.is_open(at_1230));
    assert!(post.is_open_regular(at_1230));

    let (pre, post) = cutover_sides(Exchange::Pse, tz, (2012, 1, 2));
    let at_1400 = local(tz, probe, (14, 0, 0));
    assert!(!pre.is_open(at_1400));
    assert!(post.is_open_regular(at_1400));

    let (pre, post) = cutover_sides(Exchange::Pse, tz, (2013, 11, 4));
    let at_1516 = local(tz, probe, (15, 16, 0));
    assert!(pre.is_open_regular(at_1516));
    assert!(!post.is_open_regular(at_1516));
    assert!(post.is_open_extended(at_1516));

    let (pre, post) = cutover_sides(Exchange::Pse, tz, (2020, 3, 16));
    assert!(pre.is_open_regular(at_1400));
    assert!(!post.is_open(at_1400));

    let at_1000_probe = local(tz, probe, (10, 0, 0));
    let (pre, closed) = cutover_sides(Exchange::Pse, tz, (2020, 3, 17));
    assert!(pre.is_open_regular(at_1000_probe));
    assert!(!closed.is_open(at_1000_probe));
    let (closed, resumed) = cutover_sides(Exchange::Pse, tz, (2020, 3, 19));
    assert!(!closed.is_open(at_1000_probe));
    assert!(resumed.is_open_regular(at_1000_probe));

    let at_1000 = |date| local(tz, date, (10, 0, 0));
    for date in [(2020, 3, 17), (2020, 3, 18)] {
        let hours = hours_for_exchange_as_of(Exchange::Pse, at_1000(date));
        assert!(!hours.is_open(at_1000(date)));
    }
    let resumed = hours_for_exchange_as_of(Exchange::Pse, at_1000((2020, 3, 19)));
    assert!(resumed.is_open_regular(at_1000((2020, 3, 19))));

    let (pre, post) = cutover_sides(Exchange::Pse, tz, (2021, 12, 6));
    let at_1330 = local(tz, probe, (13, 30, 0));
    assert!(!pre.is_open(at_1330));
    assert!(post.is_open_regular(at_1330));

    let (pre, post) = cutover_sides(Exchange::Pse, tz, (2022, 1, 14));
    assert!(pre.is_open_regular(at_1330));
    assert!(!post.is_open(at_1330));

    let (pre, post) = cutover_sides(Exchange::Pse, tz, (2022, 2, 2));
    assert!(!pre.is_open(at_1330));
    assert!(post.is_open_regular(at_1330));

    let (pre, post) = cutover_sides(Exchange::Pse, tz, (2024, 3, 1));
    let at_1505 = local(tz, probe, (15, 5, 0));
    assert!(!pre.is_open(at_1505));
    assert!(post.is_open_extended(at_1505));
}

#[test]
fn vietnam_cutover_and_oldest_profile() {
    let tz = Asia::Ho_Chi_Minh;
    let probe = (2026, 8, 19);

    let (pre, post) = cutover_sides(Exchange::Hose, tz, (2010, 9, 13));
    let at_0845 = local(tz, probe, (8, 45, 0));
    assert!(pre.is_open_extended(at_0845));
    assert!(post.is_open_regular(at_0845));
    let at_1030 = local(tz, probe, (10, 30, 0));
    assert!(pre.is_open_extended(at_1030));
    assert!(post.is_open_extended(at_1030));
    assert!(post.is_open_extended(local(tz, probe, (10, 50, 0))));

    let (pre, post) = cutover_sides(Exchange::Hose, tz, (2012, 3, 5));
    let at_1045 = local(tz, probe, (10, 45, 0));
    assert!(!pre.is_open_regular(at_1045));
    assert!(pre.is_open_extended(at_1045));
    assert!(post.is_open_regular(at_1045));
    assert!(post.is_open_extended(local(tz, probe, (14, 10, 0))));

    let (pre, post) = cutover_sides(Exchange::Hose, tz, (2013, 7, 22));
    let at_1415 = local(tz, probe, (14, 15, 0));
    assert!(!pre.is_open(at_1415));
    assert!(post.is_open_regular(at_1415));
    assert!(post.is_open_extended(local(tz, probe, (14, 50, 0))));

    // The archived operator PDF supplies the exact January-2010 baseline.
    let oldest = hours_for_exchange_as_of(Exchange::Hose, local(tz, (2010, 1, 4), (10, 0, 0)));
    assert!(oldest.is_open_regular(local(tz, probe, (10, 0, 0))));
    assert!(oldest.is_open_extended(local(tz, probe, (10, 15, 0))));
    assert!(oldest.is_open_extended(local(tz, probe, (10, 30, 0))));
    assert!(!oldest.is_open(local(tz, probe, (11, 0, 0))));
    assert!(!oldest.is_open(local(tz, probe, (14, 15, 0))));
}
