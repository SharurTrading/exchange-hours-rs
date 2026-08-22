// SPDX-License-Identifier: MIT-0

//! Point-in-time Hong Kong and mainland-China schedule revisions.

use super::prelude::*;

#[test]
fn hong_kong_cutovers() {
    let tz = Asia::Hong_Kong;
    let probe = (2026, 8, 19);
    let (pre, post) = cutover_sides(Exchange::Hkex, tz, (2011, 3, 7));
    let at_0945 = local(tz, probe, (9, 45, 0));
    assert!(!pre.is_open_regular(at_0945));
    assert!(post.is_open_regular(at_0945));

    let (pre, post) = cutover_sides(Exchange::Hkex, tz, (2012, 3, 5));
    let at_1315 = local(tz, probe, (13, 15, 0));
    assert!(!pre.is_open(at_1315));
    assert!(post.is_open_regular(at_1315));

    let (pre, post) = cutover_sides(Exchange::Hkex, tz, (2019, 10, 8));
    let at_1605 = local(tz, probe, (16, 5, 0));
    assert!(!pre.is_open(at_1605));
    assert!(post.is_open_extended(at_1605));
}

#[test]
fn mainland_china_cutovers() {
    let tz = Asia::Shanghai;
    let probe = (2026, 8, 19);
    let (pre, post) = cutover_sides(Exchange::Sse, tz, (2018, 8, 20));
    let at_1458 = local(tz, probe, (14, 58, 0));
    assert!(pre.is_open_regular(at_1458));
    assert!(!post.is_open_regular(at_1458));
    assert!(post.is_open_extended(at_1458));

    let (pre, post) = cutover_sides(Exchange::Szse, tz, (2016, 5, 9));
    let at_0927 = local(tz, probe, (9, 27, 0));
    assert!(pre.is_open_extended(at_0927));
    assert!(!post.is_open(at_0927));

    for exchange in [Exchange::Sse, Exchange::Szse] {
        let (pre, post) = cutover_sides(exchange, tz, (2026, 7, 6));
        let at_1502 = local(tz, probe, (15, 2, 0));
        assert!(!pre.is_open(at_1502));
        assert!(post.is_open_extended(at_1502));
    }
}
