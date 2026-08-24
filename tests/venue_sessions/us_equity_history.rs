// SPDX-License-Identifier: MIT-0

//! Independent public-surface contracts for US equity/ATS history.

use super::prelude::*;

fn profile_before(exch: Exchange, date: (i32, u32, u32)) -> MarketHours {
    hours_for_exchange_as_of(exch, et(date, (0, 0, 0)) - chrono::Duration::seconds(1))
}

fn profile_from(exch: Exchange, date: (i32, u32, u32)) -> MarketHours {
    hours_for_exchange_as_of(exch, et(date, (0, 0, 0)))
}

#[test]
fn cboe_exchange_launches_are_not_backfilled() {
    // BYX: official production-symbol rollout began 2010-10-15.
    // https://www.sec.gov/files/rules/sro/byx/2010/34-63097.pdf
    // https://cdn.cboe.com/resources/fee_schedule/2010/BATS-Announces-BATS-Y-Exchange-BYX-Pricing-Effective-October-15-2010-and-New-B2B-TRIM-SLIM-and-One-Under-Routing-Strategies.pdf
    let byx_before = profile_before(Exchange::CboeByx, (2010, 10, 15));
    let byx_launch = profile_from(Exchange::CboeByx, (2010, 10, 15));
    assert!(byx_before.regular.is_empty() && byx_before.extended.is_empty());
    assert!(!byx_launch.is_open(et((2010, 10, 15), (7, 59, 59))));
    assert!(byx_launch.is_open_extended(et((2010, 10, 15), (8, 0, 0))));
    assert!(!byx_launch.is_open(et((2010, 10, 15), (17, 0, 0))));

    // The UTP alert and SEC record date first-symbol production trading on
    // both Direct Edge exchanges to 2010-07-02. The later operator release
    // records completion of the all-symbol rollout on 2010-07-21.
    // https://www.nasdaqtrader.com/TraderNews.aspx?id=uva2010-007
    // https://www.sec.gov/file/34-62431
    // https://www.globenewswire.com/news-release/2010/07/21/425534/9381/en/Direct-Edge-Launches-Exchange-Operations.html
    for exch in [Exchange::CboeEdga, Exchange::CboeEdgx] {
        let before = profile_before(exch, (2010, 7, 2));
        let launched = profile_from(exch, (2010, 7, 2));
        assert!(before.regular.is_empty() && before.extended.is_empty());
        assert!(!launched.is_open(et((2010, 7, 2), (7, 59, 59))));
        assert!(launched.is_open_extended(et((2010, 7, 2), (8, 0, 0))));
        assert!(launched.is_open_extended(et((2010, 7, 2), (19, 59, 59))));
        assert!(!launched.is_open(et((2010, 7, 2), (20, 0, 0))));
        assert_eq!(
            launched,
            profile_from(exch, (2010, 7, 21)),
            "all-symbol completion is not a schedule cutover for {exch:?}"
        );
    }
}

#[test]
fn bzx_and_byx_2014_order_queues_use_the_exact_operator_dates() {
    // The final operator notice makes the 06:00 queues effective on distinct
    // dates. Before the 2016 matching change those queues ran to 08:00, the
    // hour at which matching began.
    // https://cdn.cboe.com/resources/release_notes/2014/BATS-BYX-Exchange-and-BZX-Exchange-Feature-Release-Postponed-Until-December-2014.pdf
    for (exch, date) in [
        (Exchange::CboeByx, (2014, 12, 1)),
        (Exchange::CboeBzx, (2014, 12, 2)),
    ] {
        let before = profile_before(exch, date);
        let after = profile_from(exch, date);

        assert!(!before.is_open(et(date, (6, 0, 0))), "{exch:?}");
        assert!(before.is_open_extended(et(date, (8, 0, 0))), "{exch:?}");
        assert!(!after.is_open(et(date, (5, 59, 59))), "{exch:?}");
        assert!(after.is_order_entry_only(et(date, (6, 0, 0))), "{exch:?}");
    }
}

#[test]
fn bzx_and_byx_2016_matching_start_moves_the_0700_hour_into_extended() {
    // Bats moved equity order matching and routing one hour earlier, to 07:00
    // ET, on staggered days: BYX May 23, BZX May 25. Before each exchange's
    // day orders were accepted from 06:00 but nothing matched until 08:00, so
    // 07:30 was order entry, never a tradeable session.
    // https://cdn.cboe.com/resources/release_notes/2016/Update-Bats-to-Begin-Equity-Order-Matching-and-Routing-at-7-am-ET.pdf
    for (exch, date) in [
        (Exchange::CboeByx, (2016, 5, 23)),
        (Exchange::CboeBzx, (2016, 5, 25)),
    ] {
        let before = profile_before(exch, date);
        let after = profile_from(exch, date);

        assert!(before.is_order_entry_only(et(date, (7, 30, 0))), "{exch:?}");
        assert!(!before.is_open(et(date, (7, 30, 0))), "{exch:?}");
        assert!(before.is_open_extended(et(date, (8, 0, 0))), "{exch:?}");
        assert!(after.is_open_extended(et(date, (7, 30, 0))), "{exch:?}");
        assert!(!after.is_order_entry_only(et(date, (7, 30, 0))), "{exch:?}");
        assert!(after.is_order_entry_only(et(date, (6, 30, 0))), "{exch:?}");
    }
}

#[test]
fn partial_us_equity_histories_do_not_invent_current_queue_onsets() {
    // The current EDGA queue is primary-supported at 06:00, but its original
    // onset day is not. The historical selector keeps the exact 2016 matching
    // change without pretending that it introduced the older queue.
    let before = profile_before(Exchange::CboeEdga, (2016, 5, 24));
    let after = profile_from(Exchange::CboeEdga, (2016, 5, 24));
    assert!(!before.is_open(et((2016, 5, 24), (7, 0, 0))));
    assert!(after.is_open_extended(et((2016, 5, 24), (7, 0, 0))));

    let current = hours_for_exchange(Exchange::CboeEdga);
    let partial_as_of = hours_for_exchange_as_of(Exchange::CboeEdga, et((2026, 4, 20), (12, 0, 0)));
    assert!(current.is_order_entry_only(et((2026, 4, 20), (6, 0, 0))));
    assert!(!partial_as_of.is_open(et((2026, 4, 20), (6, 0, 0))));
    assert!(!calendar_for_exchange(Exchange::CboeEdga).is_open(et((2026, 4, 20), (6, 0, 0))));

    for (exchange, current_queue, dated_open) in [
        (Exchange::Nyse, (6, 30, 0), (9, 30, 0)),
        (Exchange::NyseArca, (2, 30, 0), (4, 0, 0)),
    ] {
        let instant = et((2026, 4, 20), current_queue);
        let dated = hours_for_exchange_as_of(exchange, instant);
        assert!(
            hours_for_exchange(exchange).is_order_entry_only(instant),
            "{exchange:?} at {instant}"
        );
        assert!(!dated.is_open(instant), "{exchange:?} at {instant}");
        assert!(
            !calendar_for_exchange(exchange).is_open(instant),
            "{exchange:?} at {instant}"
        );
        let dated_instant = et((2026, 4, 20), dated_open);
        assert!(
            dated.is_open(dated_instant),
            "{exchange:?} at {dated_instant}"
        );
    }
}

#[test]
fn bzx_and_byx_2018_late_close_changes_are_distinct() {
    // https://cdn.cboe.com/resources/release_notes/2018/BZX-Exchange-and-BYX-Exchange-to-Extend-Post-Market-Session-Hours-to-8PM-ET.pdf
    for (exch, date) in [
        (Exchange::CboeBzx, (2018, 7, 30)),
        (Exchange::CboeByx, (2018, 8, 27)),
    ] {
        let before = profile_before(exch, date);
        let after = profile_from(exch, date);

        assert!(!before.is_open(et(date, (17, 0, 0))), "{exch:?}");
        assert!(after.is_open_extended(et(date, (17, 0, 0))), "{exch:?}");
        assert!(after.is_open_extended(et(date, (19, 59, 59))), "{exch:?}");
        assert!(!after.is_open(et(date, (20, 0, 0))), "{exch:?}");
    }
}

#[test]
fn bzx_january_2010_baseline_is_0800_to_1700() {
    // SEC Release 34-59963 records the 08:00–09:30, 09:30–16:00, and
    // 16:00–17:00 sessions in force before the audit floor.
    // https://www.sec.gov/rules/sro/bats/2009/34-59963.pdf
    let hours = hours_for_exchange_as_of(Exchange::CboeBzx, et((2010, 1, 4), (12, 0, 0)));
    assert!(!hours.is_open(et((2010, 1, 4), (7, 59, 59))));
    assert!(hours.is_open_extended(et((2010, 1, 4), (8, 0, 0))));
    assert!(hours.is_open_regular(et((2010, 1, 4), (9, 30, 0))));
    assert!(!hours.is_open(et((2010, 1, 4), (17, 0, 0))));
}

#[test]
fn nasdaq_bx_uses_the_operator_stated_2011_cutover() {
    // Equity Trader Alert 2011-20 states that BX began accepting and executing
    // orders at 07:00 ET on Monday 2011-04-18. The launch alert supplies the
    // predecessor 08:00–19:00 grid.
    // https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2009-003
    // https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2011-20
    // https://www.sec.gov/rules/sro/bx/2011/34-64105.pdf
    let before = profile_before(Exchange::NasdaqBx, (2011, 4, 18));
    let after = profile_from(Exchange::NasdaqBx, (2011, 4, 18));

    assert!(!before.is_open(et((2011, 4, 18), (7, 59, 59))));
    assert!(before.is_open_extended(et((2011, 4, 18), (8, 0, 0))));
    assert!(after.is_open_extended(et((2011, 4, 18), (7, 0, 0))));
    assert!(!after.is_open(et((2011, 4, 18), (19, 0, 0))));
}

#[test]
fn nyse_arca_grid_is_supported_at_the_2010_audit_floor() {
    // SEC Release 34-57505 records Arca's 04:00–20:00 three-session grid in
    // 2008, before this crate's historical floor.
    // https://www.sec.gov/files/rules/sro/nysearca/2008/34-57505.pdf
    let hours = hours_for_exchange_as_of(Exchange::NyseArca, et((2010, 1, 4), (12, 0, 0)));
    assert!(!hours.is_open(et((2010, 1, 4), (3, 59, 59))));
    assert!(hours.is_open_extended(et((2010, 1, 4), (4, 0, 0))));
    assert!(hours.is_open_regular(et((2010, 1, 4), (9, 30, 0))));
    assert!(hours.is_open_extended(et((2010, 1, 4), (19, 59, 59))));
    assert!(!hours.is_open(et((2010, 1, 4), (20, 0, 0))));
    assert!(!hours.is_open(et((2010, 1, 9), (12, 0, 0))));
}

#[test]
fn nyse_american_extended_trading_begins_with_pillar() {
    // NYSE's launch update dates all-NMS-stock Pillar production to
    // 2017-07-24; the functional update defines a 06:30 order-acceptance edge
    // around the 07:00–20:00 execution grid.
    // https://www.nyse.com/publicdocs/nyse/markets/nyse-american/Pillar_Update_NYSE_American_Weekend_Test_Update_July21_2017.pdf
    let before = profile_before(Exchange::NyseAmerican, (2017, 7, 24));
    let pillar = profile_from(Exchange::NyseAmerican, (2017, 7, 24));

    assert!(!before.is_open(et((2017, 7, 24), (8, 0, 0))));
    assert!(before.is_open_regular(et((2017, 7, 24), (10, 0, 0))));
    assert!(!before.is_open(et((2017, 7, 24), (16, 0, 0))));
    assert!(pillar.is_order_entry_only(et((2017, 7, 24), (6, 30, 0))));
    assert!(pillar.is_open_regular(et((2017, 7, 24), (9, 30, 0))));
    assert!(pillar.is_open_extended(et((2017, 7, 24), (19, 59, 59))));
    assert!(!pillar.is_open(et((2017, 7, 24), (20, 0, 0))));
}

#[test]
fn nyse_texas_preserves_chx_history_through_the_pillar_migration() {
    // NYSE Texas is a non-substantive continuation of NYSE Chicago/CHX. CHX's
    // January-2010 rules retain a 07:00–17:00 envelope, including the
    // 16:15–17:00 cross-only stage. The exact 2019-11-04 Pillar migration
    // added the 06:30 order-entry queue and extended the late session to 20:00.
    // https://www.sec.gov/rules/sro/chx/2009/34-60775.pdf
    // https://www.sec.gov/files/rules/sro/nysechx/2019/34-86709.pdf
    // https://www.nyse.com/publicdocs/nyse/markets/nyse-chicago/NYSE_Chicago_Migration.pdf
    // https://www.sec.gov/files/rules/sro/nysechx/2025/34-102507.pdf
    let baseline = hours_for_exchange_as_of(Exchange::NyseTexas, et((2010, 1, 4), (12, 0, 0)));
    assert!(!baseline.is_open(et((2010, 1, 4), (6, 59, 59))));
    assert!(baseline.is_open_extended(et((2010, 1, 4), (7, 0, 0))));
    assert!(baseline.is_open_regular(et((2010, 1, 4), (9, 30, 0))));
    assert!(baseline.is_open_extended(et((2010, 1, 4), (16, 59, 59))));
    assert!(!baseline.is_open(et((2010, 1, 4), (17, 0, 0))));

    let before = profile_before(Exchange::NyseTexas, (2019, 11, 4));
    let pillar = profile_from(Exchange::NyseTexas, (2019, 11, 4));
    assert!(!before.is_open(et((2019, 11, 4), (17, 0, 0))));
    assert!(!pillar.is_open(et((2019, 11, 4), (6, 29, 59))));
    assert!(pillar.is_order_entry_only(et((2019, 11, 4), (6, 30, 0))));
    assert!(pillar.is_open_extended(et((2019, 11, 4), (17, 0, 0))));
    assert!(pillar.is_open_extended(et((2019, 11, 4), (19, 59, 59))));
    assert!(!pillar.is_open(et((2019, 11, 4), (20, 0, 0))));
    assert_eq!(pillar, profile_from(Exchange::NyseTexas, (2025, 3, 28)));
}

#[test]
fn nyse_national_full_legacy_and_dormant_timeline_is_preserved() {
    // SEC Release 34-62643 gives both sides of the operative 2010-08-02 late
    // close change. SEC and exchange filings date both dormant intervals and
    // the two relaunches.
    let baseline = profile_before(Exchange::NyseNational, (2010, 8, 2));
    let extended = profile_from(Exchange::NyseNational, (2010, 8, 2));
    assert!(baseline.is_open_extended(et((2010, 8, 2), (18, 29, 59))));
    assert!(!baseline.is_open(et((2010, 8, 2), (18, 30, 0))));
    assert!(extended.is_open_extended(et((2010, 8, 2), (19, 59, 59))));

    // SEC Release 34-72215 was operative upon its 2014-05-16 filing and
    // shortened NSX's post-Regular Trading Hours close from 20:00 to 17:00.
    // https://www.sec.gov/files/rules/sro/nsx/2014/34-72215.pdf
    let before_short_close = profile_before(Exchange::NyseNational, (2014, 5, 16));
    let short_close = profile_from(Exchange::NyseNational, (2014, 5, 16));
    assert!(before_short_close.is_open_extended(et((2014, 5, 16), (19, 59, 59))));
    assert!(!short_close.is_open(et((2014, 5, 16), (17, 0, 0))));

    let pre_first_close = profile_before(Exchange::NyseNational, (2014, 5, 31));
    let first_dormant = profile_from(Exchange::NyseNational, (2014, 5, 31));
    assert!(pre_first_close.is_open_regular(et((2014, 6, 2), (10, 0, 0))));
    assert!(first_dormant.regular.is_empty() && first_dormant.extended.is_empty());

    let first_relaunch = profile_from(Exchange::NyseNational, (2015, 12, 22));
    assert!(first_relaunch.is_open_extended(et((2015, 12, 22), (8, 0, 0))));
    assert!(!first_relaunch.is_open(et((2015, 12, 22), (17, 0, 0))));

    let second_dormant = profile_from(Exchange::NyseNational, (2017, 2, 1));
    assert!(second_dormant.regular.is_empty() && second_dormant.extended.is_empty());

    let pillar = profile_from(Exchange::NyseNational, (2018, 5, 21));
    assert!(pillar.is_order_entry_only(et((2018, 5, 21), (6, 30, 0))));
    assert!(pillar.is_open_extended(et((2018, 5, 21), (19, 59, 59))));
    assert!(!pillar.is_open(et((2018, 5, 21), (20, 0, 0))));
}

#[test]
fn blue_ocean_new_order_window_is_stable_from_launch() {
    // The launch-era, 2023, and live ATS-N filings all stop accepting new
    // orders at 04:00. A later sub-minute resting-book cleanup is outside this
    // explicitly scoped new-order window and creates no historical cutover.
    // https://www.sec.gov/Archives/edgar/data/1795131/000153949721000764/primary_doc.xml
    // https://www.sec.gov/Archives/edgar/data/1795131/000153949723000091/primary_doc.xml
    let historical = hours_for_exchange_as_of(Exchange::BlueOceanAts, et((2021, 10, 6), (2, 0, 0)));
    let current = hours_for_exchange(Exchange::BlueOceanAts);

    assert_eq!(historical.regular, current.regular);
    assert_eq!(historical.extended, current.extended);
    assert!(historical.is_open_extended(et((2021, 10, 6), (3, 59, 59))));
    assert!(!historical.is_open(et((2021, 10, 6), (4, 0, 0))));
}
