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
