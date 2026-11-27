// SPDX-License-Identifier: MIT-0

//! Point-in-time Indian cash-equity schedule revisions.

use super::prelude::*;
use crate::support::zones::KOLKATA;

#[test]
fn india_cutovers() {
    let tz = KOLKATA;
    let probe = (2026, 8, 19);
    for exchange in [Exchange::NseIndia, Exchange::BseIndia] {
        let (pre, post) = cutover_sides(exchange, &tz, (2010, 1, 4));
        let at_0930 = local(&tz, probe, (9, 30, 0));
        assert!(!pre.is_open_regular(at_0930));
        assert!(post.is_open_regular(at_0930));

        let (pre, post) = cutover_sides(exchange, &tz, (2010, 10, 18));
        let at_0905 = local(&tz, probe, (9, 5, 0));
        assert!(pre.is_open_regular(at_0905));
        assert!(!post.is_open_regular(at_0905));
        // The 2010 pre-open call introduced an order-collection window here.
        assert!(post.is_order_entry_only(at_0905));

        let (pre, post) = cutover_sides(exchange, &tz, (2026, 8, 3));
        let at_1520 = local(&tz, probe, (15, 20, 0));
        assert!(!pre.is_open_extended(at_1520));
        assert!(post.is_open_extended(at_1520));
    }

    let (pre, post) = cutover_sides(Exchange::NseIndia, &tz, (2011, 10, 3));
    let at_1545 = local(&tz, probe, (15, 45, 0));
    assert!(!pre.is_open(at_1545));
    assert!(post.is_open_extended(at_1545));
}

#[test]
fn nse_weekday_morning_block_deal_begins_on_the_published_cutover() {
    let calendar = exchange_hours::calendar_for_exchange(Exchange::NseIndia);
    let tz = KOLKATA;
    let before = local(&tz, (2017, 12, 29), (8, 45, 0));
    let first_day = (2018, 1, 1);
    assert_eq!(calendar.is_open(before), Ok(false));
    for date in [first_day, (2026, 8, 19)] {
        let just_before = local(&tz, date, (8, 44, 59));
        let open = local(&tz, date, (8, 45, 0));
        let close = local(&tz, date, (9, 0, 0));
        assert_eq!(calendar.is_accepting_orders(just_before), Ok(false));
        assert_eq!(calendar.is_open_extended(open), Ok(true));
        assert_eq!(calendar.is_open_regular(open), Ok(false));
        assert_eq!(
            calendar.trade_date(open),
            Ok(Some(jiff::civil::date(date.0, date.1, date.2)))
        );
        assert_eq!(calendar.is_open(close), Ok(false));
        assert_eq!(calendar.is_order_entry_only(close), Ok(true));
    }
}
