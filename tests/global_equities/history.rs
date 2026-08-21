// SPDX-License-Identifier: MIT-0

//! Point-in-time schedule amendments for global cash-equity venues.

use super::prelude::*;

fn cutover_sides(exchange: Exchange, tz: Tz, date: (i32, u32, u32)) -> (MarketHours, MarketHours) {
    let midnight = local(tz, date, (0, 0, 0));
    (
        hours_for_exchange_as_of(exchange, midnight - Duration::nanoseconds(1)),
        hours_for_exchange_as_of(exchange, midnight),
    )
}

#[test]
fn lse_cutovers() {
    let tz = Europe::London;
    let probe = (2026, 8, 19);

    let (pre, post) = cutover_sides(Exchange::Lse, tz, (2012, 4, 30));
    let at_1638 = local(tz, probe, (16, 38, 0));
    assert!(!pre.is_open(at_1638));
    assert!(post.is_open_extended(at_1638));

    let (pre, post) = cutover_sides(Exchange::Lse, tz, (2016, 3, 21));
    let at_1201 = local(tz, probe, (12, 1, 0));
    assert!(pre.is_open_regular(at_1201));
    assert!(!post.is_open_regular(at_1201));
    assert!(post.is_open_extended(at_1201));
}

#[test]
fn xetra_cutovers() {
    let tz = Europe::Berlin;
    let probe = (2026, 8, 19);

    let (pre, post) = cutover_sides(Exchange::Xetra, tz, (2020, 11, 24));
    let at_1740 = local(tz, probe, (17, 40, 0));
    assert!(pre.is_open_extended(at_1740));
    assert!(post.is_open_extended(at_1740));
    let (_, pre_phase_close) =
        exchange_hours::session_bounds_with(&pre, at_1740, exchange_hours::SessionKind::Extended)
            .expect("pre-TAC post-trading phase");
    let (_, post_phase_close) =
        exchange_hours::session_bounds_with(&post, at_1740, exchange_hours::SessionKind::Extended)
            .expect("TAC phase");
    assert_eq!(pre_phase_close, local(tz, probe, (20, 30, 0)));
    assert_eq!(post_phase_close, local(tz, probe, (17, 45, 0)));

    let (pre, post) = cutover_sides(Exchange::Xetra, tz, (2025, 12, 1));
    let at_0715 = local(tz, probe, (7, 15, 0));
    assert!(!pre.is_open(at_0715));
    assert!(post.is_open_extended(at_0715));
}

#[test]
fn euronext_core_pre_open_cutovers() {
    for (exchange, tz, probe_time) in [
        (Exchange::EuronextParis, Europe::Paris, (7, 20, 0)),
        (Exchange::EuronextAmsterdam, Europe::Amsterdam, (7, 20, 0)),
        (Exchange::EuronextBrussels, Europe::Brussels, (7, 20, 0)),
        (Exchange::EuronextLisbon, Europe::Lisbon, (6, 20, 0)),
    ] {
        let (pre, post) = cutover_sides(exchange, tz, (2023, 3, 20));
        let probe = local(tz, (2026, 8, 19), probe_time);
        assert!(pre.is_open_extended(probe), "{exchange:?}");
        assert!(!post.is_open(probe), "{exchange:?}");
    }
}

#[test]
fn euronext_dublin_cutovers() {
    let tz = Europe::Dublin;
    let probe = (2026, 8, 19);

    let (pre, post) = cutover_sides(Exchange::EuronextDublin, tz, (2019, 2, 4));
    let at_0620 = local(tz, probe, (6, 20, 0));
    assert!(!pre.is_open(at_0620));
    assert!(post.is_open_extended(at_0620));
    let at_1700 = local(tz, probe, (17, 0, 0));
    assert!(pre.is_open_extended(at_1700));
    assert!(!post.is_open(at_1700));

    let (pre, post) = cutover_sides(Exchange::EuronextDublin, tz, (2023, 3, 20));
    assert!(pre.is_open_extended(at_0620));
    assert!(!post.is_open(at_0620));
}

#[test]
fn euronext_milan_cutovers() {
    let tz = Europe::Rome;
    let probe = (2026, 8, 19);

    let (pre, post) = cutover_sides(Exchange::EuronextMilan, tz, (2013, 9, 30));
    let at_1735 = local(tz, probe, (17, 35, 0));
    assert!(!pre.is_open(at_1735));
    assert!(post.is_open_extended(at_1735));

    let (pre, post) = cutover_sides(Exchange::EuronextMilan, tz, (2015, 11, 23));
    let at_1728 = local(tz, probe, (17, 28, 0));
    assert!(!pre.is_open_regular(at_1728));
    assert!(post.is_open_regular(at_1728));

    let (pre, post) = cutover_sides(Exchange::EuronextMilan, tz, (2023, 3, 27));
    let at_090045 = local(tz, probe, (9, 0, 45));
    assert!(pre.is_open_extended(at_090045));
    assert!(post.is_open_regular(at_090045));
}

#[test]
fn nasdaq_nordic_cutovers() {
    for (exchange, tz, probe_time) in [
        (Exchange::NasdaqStockholm, Europe::Stockholm, (9, 0, 3)),
        (Exchange::NasdaqHelsinki, Europe::Helsinki, (10, 0, 3)),
        (Exchange::NasdaqCopenhagen, Europe::Copenhagen, (9, 0, 3)),
    ] {
        let (pre, post) = cutover_sides(exchange, tz, (2015, 11, 16));
        let probe = local(tz, (2026, 8, 19), probe_time);
        assert!(pre.is_open_regular(probe), "{exchange:?}");
        assert!(!post.is_open_regular(probe), "{exchange:?}");
        assert!(post.is_open_extended(probe), "{exchange:?}");
    }

    let tz = Europe::Copenhagen;
    let (pre, post) = cutover_sides(Exchange::NasdaqCopenhagen, tz, (2019, 5, 1));
    let at_1705 = local(tz, (2026, 8, 19), (17, 5, 0));
    // Both sides are extended: pre-launch this is non-executable post-trading;
    // afterward it is executable Trading@Closing Price. The public rule table
    // preserves the sourced phase boundary even though `SessionKind` groups
    // both under `Extended`.
    assert!(pre.is_open_extended(at_1705));
    assert!(post.is_open_extended(at_1705));
    assert_ne!(pre.extended, post.extended);
}

#[test]
fn jse_cutovers() {
    let tz = Africa::Johannesburg;
    let probe = (2026, 8, 19);

    let (pre, post) = cutover_sides(Exchange::Jse, tz, (2012, 7, 2));
    let at_0832 = local(tz, probe, (8, 32, 0));
    assert!(!pre.is_open(at_0832));
    assert!(post.is_open_extended(at_0832));

    let (pre, post) = cutover_sides(Exchange::Jse, tz, (2013, 11, 11));
    let at_1706 = local(tz, probe, (17, 6, 0));
    assert!(!pre.is_open(at_1706));
    assert!(post.is_open_extended(at_1706));

    let (pre, post) = cutover_sides(Exchange::Jse, tz, (2016, 9, 26));
    let at_1712 = local(tz, probe, (17, 12, 0));
    assert!(!pre.is_open(at_1712));
    assert!(post.is_open_extended(at_1712));

    let (pre, post) = cutover_sides(Exchange::Jse, tz, (2020, 8, 24));
    assert!(pre.is_open_extended(at_1712));
    assert!(!post.is_open(at_1712));

    let (pre, post) = cutover_sides(Exchange::Jse, tz, (2021, 2, 1));
    let at_170130 = local(tz, probe, (17, 1, 30));
    assert!(!pre.is_open(at_170130));
    assert!(post.is_open_extended(at_170130));

    let (pre, post) = cutover_sides(Exchange::Jse, tz, (2021, 2, 15));
    assert!(pre.is_open_extended(at_170130));
    assert!(!post.is_open(at_170130));
    assert!(post.is_open_extended(local(tz, probe, (17, 2, 0))));
}

#[test]
fn borsa_istanbul_cutovers() {
    let tz = Europe::Istanbul;
    let probe = (2026, 8, 19);

    let (pre, post) = cutover_sides(Exchange::BorsaIstanbul, tz, (2012, 3, 2));
    let at_1720 = local(tz, probe, (17, 20, 0));
    assert!(pre.is_open_regular(at_1720));
    assert!(!post.is_open_regular(at_1720));
    assert!(post.is_open_extended(at_1720));

    let (pre, post) = cutover_sides(Exchange::BorsaIstanbul, tz, (2012, 7, 16));
    let at_1725 = local(tz, probe, (17, 25, 0));
    assert!(!pre.is_open_regular(at_1725));
    assert!(pre.is_open_extended(at_1725));
    assert!(post.is_open_regular(at_1725));

    let (pre, post) = cutover_sides(Exchange::BorsaIstanbul, tz, (2013, 4, 5));
    let at_0947 = local(tz, probe, (9, 47, 0));
    assert!(pre.is_open_extended(at_0947));
    assert!(post.is_open_regular(at_0947));

    let (pre, post) = cutover_sides(Exchange::BorsaIstanbul, tz, (2013, 6, 10));
    let at_0940 = local(tz, probe, (9, 40, 0));
    assert!(pre.is_open_extended(at_0940));
    assert!(post.is_open_regular(at_0940));

    let (pre, post) = cutover_sides(Exchange::BorsaIstanbul, tz, (2015, 11, 30));
    let at_1300 = local(tz, probe, (13, 0, 0));
    assert!(!pre.is_open(at_1300));
    assert!(post.is_open_extended(at_1300));

    let (pre, post) = cutover_sides(Exchange::BorsaIstanbul, tz, (2016, 3, 28));
    let at_1345 = local(tz, probe, (13, 45, 0));
    assert!(pre.is_open_regular(at_1345));
    assert!(!post.is_open_regular(at_1345));
    assert!(post.is_open_extended(at_1345));

    let (pre, post) = cutover_sides(Exchange::BorsaIstanbul, tz, (2016, 11, 14));
    let at_1745 = local(tz, probe, (17, 45, 0));
    assert!(!pre.is_open(at_1745));
    assert!(post.is_open_regular(at_1745));

    let (pre, post) = cutover_sides(Exchange::BorsaIstanbul, tz, (2019, 10, 4));
    let at_1330 = local(tz, probe, (13, 30, 0));
    assert!(!pre.is_open_regular(at_1330));
    assert!(pre.is_open_extended(at_1330));
    assert!(post.is_open_regular(at_1330));
}

#[test]
fn tadawul_cutovers() {
    let tz = Asia::Riyadh;
    let probe = (2026, 8, 19);

    let (pre, post) = cutover_sides(Exchange::Tadawul, tz, (2013, 6, 29));
    let saturday_noon = local(tz, (2026, 8, 22), (12, 0, 0));
    assert!(pre.is_open_regular(saturday_noon));
    assert!(!post.is_open(saturday_noon));

    let (pre, post) = cutover_sides(Exchange::Tadawul, tz, (2016, 4, 3));
    let at_1030 = local(tz, probe, (10, 30, 0));
    assert!(!pre.is_open_regular(at_1030));
    assert!(pre.is_open_extended(at_1030));
    assert!(post.is_open_regular(at_1030));

    let (pre, post) = cutover_sides(Exchange::Tadawul, tz, (2018, 5, 27));
    let at_1505 = local(tz, probe, (15, 5, 0));
    assert!(!pre.is_open(at_1505));
    assert!(post.is_open_extended(at_1505));

    let (pre, post) = cutover_sides(Exchange::Tadawul, tz, (2019, 5, 12));
    let at_1515 = local(tz, probe, (15, 15, 0));
    assert!(!pre.is_open(at_1515));
    assert!(post.is_open_extended(at_1515));

    let (pre, post) = cutover_sides(Exchange::Tadawul, tz, (2020, 3, 26));
    let at_1330 = local(tz, probe, (13, 30, 0));
    assert!(pre.is_open_regular(at_1330));
    assert!(!post.is_open(at_1330));

    let (pre, post) = cutover_sides(Exchange::Tadawul, tz, (2020, 5, 31));
    assert!(!pre.is_open(at_1330));
    assert!(post.is_open_regular(at_1330));
}

#[test]
fn six_trading_at_last_cutover() {
    let tz = Europe::Zurich;
    let probe = local(tz, (2026, 8, 19), (17, 35, 0));
    let (pre, post) = cutover_sides(Exchange::Six, tz, (2020, 6, 22));

    assert!(pre.is_open_extended(probe));
    assert!(post.is_open_extended(probe));
    let (_, pre_phase_close) =
        exchange_hours::session_bounds_with(&pre, probe, exchange_hours::SessionKind::Extended)
            .expect("pre-TAL post-trading phase");
    let (_, post_phase_close) =
        exchange_hours::session_bounds_with(&post, probe, exchange_hours::SessionKind::Extended)
            .expect("TAL phase");
    let (post_phase_open, _) =
        exchange_hours::session_bounds_with(&post, probe, exchange_hours::SessionKind::Extended)
            .expect("TAL phase");
    assert_eq!(pre_phase_close, local(tz, (2026, 8, 19), (22, 0, 0)));
    assert_eq!(post_phase_open, local(tz, (2026, 8, 19), (17, 32, 0)));
    assert_eq!(post_phase_close, local(tz, (2026, 8, 19), (17, 40, 0)));

    let last_randomized_auction_second = local(tz, (2026, 8, 19), (17, 31, 59));
    let randomized_auction_close = local(tz, (2026, 8, 19), (17, 32, 0));
    assert!(pre.is_open_extended(last_randomized_auction_second));
    assert!(pre.is_open_extended(randomized_auction_close));
    assert!(post.is_open_extended(randomized_auction_close));
}

#[test]
fn bme_trading_at_last_cutover() {
    let tz = Europe::Madrid;
    let probe = local(tz, (2026, 8, 19), (17, 40, 0));
    let (pre, post) = cutover_sides(Exchange::Bme, tz, (2023, 12, 4));

    assert!(!pre.is_open(probe));
    assert!(post.is_open_extended(probe));

    let last_randomized_auction_second = local(tz, (2026, 8, 19), (17, 35, 29));
    let randomized_auction_close = local(tz, (2026, 8, 19), (17, 35, 30));
    assert!(pre.is_open_extended(last_randomized_auction_second));
    assert!(!pre.is_open(randomized_auction_close));
    assert!(post.is_open_extended(randomized_auction_close));
}

#[test]
fn vienna_xetra_t7_migration_and_closing_extension() {
    let tz = Europe::Vienna;
    let probe_day = (2026, 8, 19);

    let (pre, post) = cutover_sides(Exchange::Vienna, tz, (2017, 7, 31));
    let at_090045 = local(tz, probe_day, (9, 0, 45));
    assert!(pre.is_open_extended(at_090045));
    assert!(post.is_open_regular(at_090045));
    let at_120345 = local(tz, probe_day, (12, 3, 45));
    assert!(pre.is_open_extended(at_120345));
    assert!(post.is_open_regular(at_120345));

    let (pre, post) = cutover_sides(Exchange::Vienna, tz, (2019, 5, 2));
    let at_1734 = local(tz, probe_day, (17, 34, 0));
    let (pre_open, pre_close) =
        exchange_hours::session_bounds_with(&pre, at_1734, exchange_hours::SessionKind::Extended)
            .expect("pre-extension post-trading phase");
    let (post_open, post_close) =
        exchange_hours::session_bounds_with(&post, at_1734, exchange_hours::SessionKind::Extended)
            .expect("extended closing auction");
    assert_eq!(pre_open, local(tz, probe_day, (17, 33, 30)));
    assert_eq!(pre_close, local(tz, probe_day, (17, 45, 0)));
    assert_eq!(post_open, local(tz, probe_day, (17, 30, 0)));
    assert_eq!(post_close, local(tz, probe_day, (17, 35, 30)));
}

#[test]
fn vienna_trade_at_close_cutover() {
    let tz = Europe::Vienna;
    let probe = local(tz, (2026, 8, 19), (17, 47, 0));
    let (pre, post) = cutover_sides(Exchange::Vienna, tz, (2020, 12, 1));

    assert!(!pre.is_open(probe));
    assert!(post.is_open_extended(probe));
    let (post_phase_open, post_phase_close) =
        exchange_hours::session_bounds_with(&post, probe, exchange_hours::SessionKind::Extended)
            .expect("post-TAC post-trading phase");
    assert_eq!(post_phase_open, local(tz, (2026, 8, 19), (17, 45, 0)));
    assert_eq!(post_phase_close, local(tz, (2026, 8, 19), (17, 50, 0)));
}

#[test]
fn vienna_third_friday_settlement_grid_is_date_aware() {
    let tz = Europe::Vienna;
    let calendar = exchange_hours::calendar_for_exchange(Exchange::Vienna);

    let ordinary = local(tz, (2026, 8, 14), (12, 4, 0));
    let third_friday = local(tz, (2026, 8, 21), (12, 4, 0));
    assert!(calendar.is_open_regular(ordinary));
    assert!(!calendar.is_open_regular(third_friday));
    assert!(calendar.is_open_extended(third_friday));

    let legacy_ordinary = local(tz, (2010, 1, 8), (9, 1, 30));
    let legacy_third_friday = local(tz, (2010, 1, 15), (9, 1, 30));
    assert!(calendar.is_open_regular(legacy_ordinary));
    assert!(!calendar.is_open_regular(legacy_third_friday));
    assert!(calendar.is_open_extended(legacy_third_friday));
}
