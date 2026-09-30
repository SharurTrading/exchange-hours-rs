// SPDX-License-Identifier: MIT-0

//! Point-in-time Oceania and Japan schedule revisions.

use super::prelude::*;

#[test]
fn oceania_and_japan_cutovers() {
    let probe = (2026, 8, 19);
    let (pre, post) = cutover_sides(Exchange::Asx, Australia::Sydney, (2025, 6, 23));
    let at_100900 = local(Australia::Sydney, probe, (10, 9, 0));
    let at_100914 = local(Australia::Sydney, probe, (10, 9, 14));
    let at_100915 = local(Australia::Sydney, probe, (10, 9, 15));
    assert!(pre.is_open_extended(at_100900));
    assert!(pre.is_open_extended(at_100914));
    assert!(!pre.is_open_extended(at_100915));
    assert!(!post.is_open_extended(at_100900));
    let at_1615 = local(Australia::Sydney, probe, (16, 15, 0));
    assert!(!pre.is_open(at_1615));
    assert!(post.is_open_extended(at_1615));

    let (pre, post) = cutover_sides(Exchange::Nzx, Pacific::Auckland, (2020, 4, 6));
    let at_0845 = local(Pacific::Auckland, probe, (8, 45, 0));
    assert!(!pre.is_open(at_0845));
    assert!(post.is_open_extended(at_0845));

    let (pre, post) = cutover_sides(Exchange::Tse, Asia::Tokyo, (2011, 11, 21));
    let at_1115 = local(Asia::Tokyo, probe, (11, 15, 0));
    assert!(!pre.is_open_regular(at_1115));
    assert!(post.is_open_regular(at_1115));

    let (pre, post) = cutover_sides(Exchange::Tse, Asia::Tokyo, (2024, 11, 5));
    let at_1515 = local(Asia::Tokyo, probe, (15, 15, 0));
    assert!(!pre.is_open_regular(at_1515));
    assert!(pre.is_open_extended(at_1515));
    assert!(post.is_open_regular(at_1515));
    let at_1745 = local(Asia::Tokyo, probe, (17, 45, 0));
    assert!(!pre.is_open(at_1745));
    assert!(post.is_open_extended(at_1745));
}

#[test]
fn nzx_pre_2020_grid_is_the_2010_page_print() {
    // The operator's key-dates trading-hours page as served 2010-01-05 prints
    // Pre-open 9:00-10:00 (tradeable: off-market reports print), Normal
    // Trading 10:00-4:45pm, Pre-close 4:45-5:00, and the same grid stands on
    // the revision eve. The ±30-second uncross envelopes are the carried
    // randomisation statement (docs/evidence/nzx.md, Normal week).
    for probe in [(2010, 1, 7), (2019, 12, 31)] {
        let h = hours_for_exchange(Exchange::Nzx, local(Pacific::Auckland, probe, (12, 0, 0)));
        assert!(!h.is_open(local(Pacific::Auckland, probe, (8, 59, 0))));
        assert!(h.is_open_extended(local(Pacific::Auckland, probe, (9, 45, 0))));
        assert!(!h.is_open_regular(local(Pacific::Auckland, probe, (9, 45, 0))));
        assert!(h.is_open_extended(local(Pacific::Auckland, probe, (9, 59, 40))));
        assert!(h.is_open_regular(local(Pacific::Auckland, probe, (10, 0, 0))));
        assert!(h.is_open_regular(local(Pacific::Auckland, probe, (16, 44, 0))));
        assert!(h.is_order_entry_only(local(Pacific::Auckland, probe, (16, 50, 0))));
        assert!(!h.is_open_extended(local(Pacific::Auckland, probe, (16, 50, 0))));
        assert!(h.is_open_extended(local(Pacific::Auckland, probe, (17, 0, 15))));
        assert!(!h.is_open(local(Pacific::Auckland, probe, (17, 1, 0))));
    }
}

#[test]
fn asx_pre_sr15_grid_is_the_phase_page_print() {
    // The operator's cash-market phase table (2013-09-16, restated 2020-10-22)
    // prints Pre-opening from 7:00 am, the five staggered group opens inside
    // 9:59:45-10:09:15, Normal Trading 10:00-4:00, Pre-CSPA 4:00-4:10 and the
    // CSPA envelope to 4:12 (docs/evidence/asx.md, Normal week).
    for probe in [(2013, 9, 17), (2020, 10, 23), (2025, 6, 20)] {
        let h = hours_for_exchange(Exchange::Asx, local(Australia::Sydney, probe, (12, 0, 0)));
        assert!(h.is_open_extended(local(Australia::Sydney, probe, (7, 0, 0))));
        assert!(h.is_open_extended(local(Australia::Sydney, probe, (9, 59, 30))));
        assert!(h.is_open_regular(local(Australia::Sydney, probe, (10, 0, 0))));
        assert!(h.is_open_extended(local(Australia::Sydney, probe, (10, 5, 0))));
        assert!(h.is_order_entry_only(local(Australia::Sydney, probe, (16, 5, 0))));
        assert!(!h.is_open_extended(local(Australia::Sydney, probe, (16, 5, 0))));
        assert!(h.is_open_extended(local(Australia::Sydney, probe, (16, 11, 0))));
        assert!(!h.is_open(local(Australia::Sydney, probe, (16, 13, 0))));
    }
}

#[test]
fn tmx_australia_cutovers() {
    let tz = Australia::Sydney;
    let probe = (2026, 8, 19);

    let (pre, post) = cutover_sides(Exchange::TmxAustralia, tz, (2011, 10, 31));
    let at_1030 = local(tz, probe, (10, 30, 0));
    assert!(!pre.is_open(at_1030));
    assert!(post.is_open_regular(at_1030));

    let (pre, post) = cutover_sides(Exchange::TmxAustralia, tz, (2013, 12, 9));
    let at_1615 = local(tz, probe, (16, 15, 0));
    assert!(!pre.is_open(at_1615));
    assert!(post.is_open_extended(at_1615));

    let (pre, post) = cutover_sides(Exchange::TmxAustralia, tz, (2015, 8, 31));
    let at_161230 = local(tz, probe, (16, 12, 30));
    assert!(!pre.is_open_extended(at_161230));
    assert!(post.is_open_extended(at_161230));

    let (pre, post) = cutover_sides(Exchange::TmxAustralia, tz, (2025, 3, 17));
    let at_0800 = local(tz, probe, (8, 0, 0));
    assert!(!pre.is_open(at_0800));
    assert!(post.is_open_extended(at_0800));
}
