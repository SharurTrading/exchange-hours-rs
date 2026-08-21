// SPDX-License-Identifier: MIT-0

//! Point-in-time Singapore, Korean, and Taiwanese schedule revisions.

use super::prelude::*;

#[test]
fn sgx_securities_cutovers() {
    let tz = Asia::Singapore;
    let probe = (2026, 8, 19);
    let (pre, post) = cutover_sides(Exchange::SgxSecurities, tz, (2011, 8, 1));
    let at_1300 = local(tz, probe, (13, 0, 0));
    assert!(!pre.is_open_regular(at_1300));
    assert!(post.is_open_regular(at_1300));

    let (pre, post) = cutover_sides(Exchange::SgxSecurities, tz, (2017, 11, 13));
    let at_1230 = local(tz, probe, (12, 30, 0));
    assert!(pre.is_open_regular(at_1230));
    assert!(!post.is_open_regular(at_1230));
    assert!(post.is_open_extended(at_1230));

    let (pre, post) = cutover_sides(Exchange::SgxSecurities, tz, (2019, 6, 3));
    let at_1710 = local(tz, probe, (17, 10, 0));
    assert!(!pre.is_open(at_1710));
    assert!(post.is_open_extended(at_1710));
}

#[test]
fn korea_and_taiwan_cutovers() {
    let probe = (2026, 8, 19);
    let (pre, post) = cutover_sides(Exchange::Krx, Asia::Seoul, (2016, 8, 1));
    let at_1510 = local(Asia::Seoul, probe, (15, 10, 0));
    assert!(!pre.is_open_regular(at_1510));
    assert!(post.is_open_regular(at_1510));

    let (pre, post) = cutover_sides(Exchange::Krx, Asia::Seoul, (2019, 4, 29));
    let at_0815 = local(Asia::Seoul, probe, (8, 15, 0));
    assert!(pre.is_open_extended(at_0815));
    assert!(!post.is_open(at_0815));

    let (pre, post) = cutover_sides(Exchange::Twse, Asia::Taipei, (2020, 3, 23));
    let at_1000 = local(Asia::Taipei, probe, (10, 0, 0));
    assert!(!pre.is_open_regular(at_1000));
    assert!(pre.is_open_extended(at_1000));
    assert!(post.is_open_regular(at_1000));
}
