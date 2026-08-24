// SPDX-License-Identifier: MIT-0

//! Point-in-time Indian cash-equity schedule revisions.

use super::prelude::*;

#[test]
fn india_cutovers() {
    let tz = Asia::Kolkata;
    let probe = (2026, 8, 19);
    for exchange in [Exchange::NseIndia, Exchange::BseIndia] {
        let (pre, post) = cutover_sides(exchange, tz, (2010, 1, 4));
        let at_0930 = local(tz, probe, (9, 30, 0));
        assert!(!pre.is_open_regular(at_0930));
        assert!(post.is_open_regular(at_0930));

        let (pre, post) = cutover_sides(exchange, tz, (2010, 10, 18));
        let at_0905 = local(tz, probe, (9, 5, 0));
        assert!(pre.is_open_regular(at_0905));
        assert!(!post.is_open_regular(at_0905));
        // The 2010 pre-open call introduced an order-collection window here.
        assert!(post.is_order_entry_only(at_0905));

        let (pre, post) = cutover_sides(exchange, tz, (2026, 8, 3));
        let at_1520 = local(tz, probe, (15, 20, 0));
        assert!(!pre.is_open_extended(at_1520));
        assert!(post.is_open_extended(at_1520));
    }

    let (pre, post) = cutover_sides(Exchange::NseIndia, tz, (2011, 10, 3));
    let at_1545 = local(tz, probe, (15, 45, 0));
    assert!(!pre.is_open(at_1545));
    assert!(post.is_open_extended(at_1545));
}
