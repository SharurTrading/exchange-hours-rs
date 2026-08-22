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
    assert!(!pre.is_open(at_1030));
    assert!(post.is_open_extended(at_1030));

    let (pre, post) = cutover_sides(Exchange::Hose, tz, (2012, 3, 5));
    let at_1045 = local(tz, probe, (10, 45, 0));
    assert!(!pre.is_open(at_1045));
    assert!(post.is_open_regular(at_1045));

    let (pre, post) = cutover_sides(Exchange::Hose, tz, (2013, 7, 22));
    let at_1415 = local(tz, probe, (14, 15, 0));
    assert!(!pre.is_open(at_1415));
    assert!(post.is_open_regular(at_1415));

    // The archived operator PDF supplies the exact January-2010 baseline.
    let oldest = hours_for_exchange_as_of(Exchange::Hose, local(tz, (2010, 1, 4), (10, 0, 0)));
    assert!(oldest.is_open_regular(local(tz, probe, (10, 0, 0))));
    assert!(oldest.is_open_extended(local(tz, probe, (10, 15, 0))));
    assert!(!oldest.is_open(local(tz, probe, (10, 30, 0))));
    assert!(!oldest.is_open(local(tz, probe, (14, 15, 0))));
}
