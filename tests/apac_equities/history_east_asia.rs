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
    let at_1300 = local(tz, probe, (13, 0, 0));
    assert!(pre.is_open_regular(at_1300));
    assert!(post.is_open_regular(at_1300));

    let (pre, post) = cutover_sides(Exchange::Hkex, tz, (2016, 7, 25));
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
    // SZSE call-auction order entry before the 2016 change; nothing matched.
    assert!(pre.is_order_entry_only(at_0927));
    assert!(!post.is_open(at_0927));

    for exchange in [Exchange::Sse, Exchange::Szse] {
        let baseline = hours_for_exchange(exchange, local(tz, (2010, 1, 4), (12, 0, 0)));
        assert!(baseline.is_open_extended(local(tz, probe, (15, 20, 0))));
        assert!(!baseline.is_open(local(tz, probe, (15, 30, 0))));
    }
}
