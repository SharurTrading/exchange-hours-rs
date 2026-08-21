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

    // Direct Edge's operator release dates both exchange launches to
    // 2010-07-21.
    // https://www.globenewswire.com/news-release/2010/07/21/425534/9381/en/Direct-Edge-Launches-Exchange-Operations.html
    for exch in [Exchange::CboeEdga, Exchange::CboeEdgx] {
        let before = profile_before(exch, (2010, 7, 21));
        let launched = profile_from(exch, (2010, 7, 21));
        assert!(before.regular.is_empty() && before.extended.is_empty());
        assert!(!launched.is_open(et((2010, 7, 21), (7, 59, 59))));
        assert!(launched.is_open_extended(et((2010, 7, 21), (8, 0, 0))));
        assert!(launched.is_open_extended(et((2010, 7, 21), (19, 59, 59))));
        assert!(!launched.is_open(et((2010, 7, 21), (20, 0, 0))));
    }
}

#[test]
fn cboe_2016_early_open_changes_use_each_venue_date() {
    // The operator's implementation notice gives a distinct production date
    // for each 08:00→07:00 matching-start change.
    // https://cdn.cboe.com/resources/release_notes/2016/Update-Bats-to-Begin-Equity-Order-Matching-and-Routing-at-7-am-ET.pdf
    for (exch, date) in [
        (Exchange::CboeByx, (2016, 5, 23)),
        (Exchange::CboeEdga, (2016, 5, 24)),
        (Exchange::CboeBzx, (2016, 5, 25)),
        (Exchange::CboeEdgx, (2016, 5, 26)),
    ] {
        let before = profile_before(exch, date);
        let after = profile_from(exch, date);
        let session_day = date;

        assert!(!before.is_open(et(session_day, (7, 0, 0))), "{exch:?}");
        assert!(
            before.is_open_extended(et(session_day, (8, 0, 0))),
            "{exch:?}"
        );
        assert!(
            after.is_open_extended(et(session_day, (7, 0, 0))),
            "{exch:?}"
        );
        assert!(
            after.is_open_regular(et(session_day, (9, 30, 0))),
            "{exch:?}"
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
    // 2017-07-24; the functional update defines the new 07:00–20:00 envelope.
    // https://www.nyse.com/publicdocs/nyse/markets/nyse-american/Pillar_Update_NYSE_American_Weekend_Test_Update_July21_2017.pdf
    let before = profile_before(Exchange::NyseAmerican, (2017, 7, 24));
    let pillar = profile_from(Exchange::NyseAmerican, (2017, 7, 24));

    assert!(!before.is_open(et((2017, 7, 24), (8, 0, 0))));
    assert!(before.is_open_regular(et((2017, 7, 24), (10, 0, 0))));
    assert!(!before.is_open(et((2017, 7, 24), (16, 0, 0))));
    assert!(pillar.is_open_extended(et((2017, 7, 24), (7, 0, 0))));
    assert!(pillar.is_open_regular(et((2017, 7, 24), (9, 30, 0))));
    assert!(pillar.is_open_extended(et((2017, 7, 24), (19, 59, 59))));
    assert!(!pillar.is_open(et((2017, 7, 24), (20, 0, 0))));
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
    assert!(pillar.is_open_extended(et((2018, 5, 21), (7, 0, 0))));
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

#[test]
fn intelligentcross_gap_does_not_use_company_start_as_ats_launch() {
    // The operator dates the ATS launch only to September 2018, while the
    // exact 2018-01-17 date in the audited statement is company commencement.
    // The first public electronic ATS-N on 2019-07-22 supplies the hours but
    // not their launch-day onset, so no historical cutover is fabricated.
    // https://www.imperativex.com/news/imperative-execution-closes-9-million-series-a-financing-round
    // https://www.sec.gov/Archives/edgar/data/1708826/000170882619000008/xslATS-N_X01/primary_doc.xml
    let historical =
        hours_for_exchange_as_of(Exchange::IntelligentcrossIqx, et((2018, 1, 17), (12, 0, 0)));
    let current = hours_for_exchange(Exchange::IntelligentcrossIqx);

    assert_eq!(historical.regular, current.regular);
    assert_eq!(historical.extended, current.extended);
    assert!(!historical.is_open(et((2018, 1, 17), (8, 59, 59))));
    assert!(historical.is_open_extended(et((2018, 1, 17), (9, 0, 0))));
    assert!(historical.is_open_regular(et((2018, 1, 17), (9, 30, 0))));
    assert!(!historical.is_open(et((2018, 1, 17), (16, 0, 0))));
    assert_eq!(
        serde_json::to_string(&Exchange::IntelligentcrossIqx).expect("serialize exchange"),
        "\"intelligentcross_iqx\""
    );
}
