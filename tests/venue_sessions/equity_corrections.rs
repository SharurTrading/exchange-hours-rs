// SPDX-License-Identifier: MIT-0

//! Primary-source-verified US and European equity schedules.

use super::prelude::*;

// ---------------------------------------------------------------------------
// Venue data verified against primary sources (2026-08 pass): NYSE-family and
// Cboe early sessions, the Nasdaq Nordic closes, and the European post-close
// price-formation windows. Reference week: Monday 2026-04-20.
// ---------------------------------------------------------------------------

#[test]
fn nyse_accepts_orders_from_0630_before_its_core_session() {
    // NYSE's living hours table opens its pre-opening order queue at 06:30 ET.
    let hours = hours_for_exchange(Exchange::Nyse);
    assert!(!hours.is_open(et((2026, 4, 20), (6, 29, 59))));
    assert!(hours.is_order_entry_only(et((2026, 4, 20), (6, 30, 0))));
    assert!(!hours.is_open(et((2026, 4, 20), (16, 30, 0))));
    assert!(hours.is_open_regular(et((2026, 4, 20), (10, 0, 0))));
}

#[test]
fn current_nyse_and_cboe_queues_precede_their_active_early_sessions() {
    for exch in [
        Exchange::NyseAmerican,
        Exchange::NyseNational,
        Exchange::NyseTexas,
    ] {
        let hours = hours_for_exchange(exch);
        assert!(!hours.is_open(et((2026, 4, 20), (6, 29, 59))), "{exch:?}");
        assert!(
            hours.is_order_entry_only(et((2026, 4, 20), (6, 30, 0))),
            "{exch:?}"
        );
        assert!(
            hours.is_open_extended(et((2026, 4, 20), (19, 59, 59))),
            "{exch:?}"
        );
    }

    for exch in [Exchange::CboeByx, Exchange::CboeEdga] {
        let hours = hours_for_exchange(exch);
        assert!(!hours.is_open(et((2026, 4, 20), (5, 59, 59))), "{exch:?}");
        assert!(
            hours.is_order_entry_only(et((2026, 4, 20), (6, 0, 0))),
            "{exch:?}"
        );
        assert!(
            hours.is_open_extended(et((2026, 4, 20), (19, 59, 59))),
            "{exch:?}"
        );
    }
}

#[test]
fn nyse_national_dormancy_and_relaunch_use_sourced_dates() {
    // SR-NSX-2017-04 ceased trading before market open on 2017-02-01; NYSE
    // National records its Pillar relaunch on 2018-05-21.
    // https://www.sec.gov/files/rules/sro/nsx/2017/34-80018.pdf
    // https://www.nyse.com/publicdocs/nyse/markets/nyse-national/rule-filings/filings/2020/SR-NYSENat-2020-05.pdf
    let cessation = et((2017, 2, 1), (0, 0, 0));
    let before = hours_for_exchange_as_of(
        Exchange::NyseNational,
        cessation - chrono::Duration::seconds(1),
    );
    let dormant = hours_for_exchange_as_of(Exchange::NyseNational, cessation);

    assert!(before.is_open_regular(et((2017, 2, 1), (10, 0, 0))));
    assert!(dormant.regular.is_empty());
    assert!(dormant.extended.is_empty());
    assert!(!calendar_for_exchange(Exchange::NyseNational).is_open(et((2017, 2, 1), (10, 0, 0))));

    let relaunch = et((2018, 5, 21), (0, 0, 0));
    let still_dormant = hours_for_exchange_as_of(
        Exchange::NyseNational,
        relaunch - chrono::Duration::seconds(1),
    );
    let reopened = hours_for_exchange_as_of(Exchange::NyseNational, relaunch);

    assert!(still_dormant.regular.is_empty());
    assert!(reopened.is_open_extended(et((2018, 5, 21), (7, 0, 0))));
    assert!(reopened.is_open_regular(et((2018, 5, 21), (9, 30, 0))));
    assert!(
        calendar_for_exchange(Exchange::NyseNational)
            .is_open_regular(et((2018, 5, 21), (10, 0, 0)))
    );

    // The dormant profile has no sessions. This pins the inclusive fourteen-day
    // search horizon needed to reach the exact relaunch session.
    assert_eq!(
        calendar_for_exchange(Exchange::NyseNational)
            .next_session_after(et((2018, 5, 7), (0, 0, 0))),
        // 06:30 is the order-acceptance edge; the session opens at 07:00.
        Some((et((2018, 5, 21), (7, 0, 0)), et((2018, 5, 21), (9, 30, 0)),))
    );
}

#[test]
fn bzx_edgx_and_arca_accept_orders_at_0230_today() {
    for exch in [Exchange::CboeBzx, Exchange::CboeEdgx, Exchange::NyseArca] {
        let hours = hours_for_exchange(exch);
        assert!(!hours.is_open(et((2026, 4, 20), (2, 29, 59))));
        assert!(
            hours.is_order_entry_only(et((2026, 4, 20), (2, 30, 0))),
            "{exch:?}"
        );
    }
}

#[test]
fn edgx_exact_2021_queue_changes_are_date_aware() {
    let before = hours_for_exchange_as_of(Exchange::CboeEdgx, et((2021, 3, 5), (12, 0, 0)));
    assert!(!before.is_open(et((2021, 3, 5), (5, 0, 0))));
    assert!(before.is_open_extended(et((2021, 3, 5), (7, 30, 0))));

    let after = hours_for_exchange_as_of(Exchange::CboeEdgx, et((2021, 3, 8), (12, 0, 0)));
    assert!(!after.is_open(et((2021, 3, 8), (3, 29, 59))));
    assert!(after.is_order_entry_only(et((2021, 3, 8), (3, 30, 0))));

    let september = hours_for_exchange_as_of(Exchange::CboeEdgx, et((2021, 9, 7), (12, 0, 0)));
    assert!(!september.is_open(et((2021, 9, 7), (2, 29, 59))));
    assert!(september.is_order_entry_only(et((2021, 9, 7), (2, 30, 0))));
}

#[test]
fn edgx_unconfirmed_overnight_session_is_not_encoded() {
    // The SEC-approved EDGX rule still conditions commencement on data-plan
    // readiness and a later EDGX filing, so future lookups retain current hours.
    // https://www.cboe.com/document/tech-spec/document/technical-specifications/cboe-titanium-u.s.-equities-opening-process
    // https://www.sec.gov/files/rules/sro/cboeedgx/2026/34-105587.pdf
    let fixed = hours_for_exchange(Exchange::CboeEdgx);
    let future = hours_for_exchange_as_of(Exchange::CboeEdgx, et((2026, 12, 7), (12, 0, 0)));
    let sunday_night = et((2026, 12, 6), (21, 0, 0));

    assert!(!fixed.is_open(sunday_night));
    assert!(!future.is_open(sunday_night));
    assert!(!calendar_for_exchange(Exchange::CboeEdgx).is_open(sunday_night));
}

#[test]
fn bzx_0230_queue_started_with_the_2025_early_session_expansion() {
    // https://www.cboe.com/insights/posts/early-birds-and-night-owls-how-extended-trading-hours-are-reshaping-u-s-equities-markets-
    let before = hours_for_exchange_as_of(Exchange::CboeBzx, et((2025, 4, 30), (12, 0, 0)));
    assert!(!before.is_open(et((2025, 4, 30), (5, 0, 0))));
    assert!(before.is_order_entry_only(et((2025, 4, 30), (6, 0, 0))));

    let after = hours_for_exchange_as_of(Exchange::CboeBzx, et((2025, 5, 1), (12, 0, 0)));
    assert!(!after.is_open(et((2025, 5, 1), (2, 29, 59))));
    assert!(after.is_order_entry_only(et((2025, 5, 1), (2, 30, 0))));
}

#[test]
fn memx_is_closed_before_its_2020_09_21_launch() {
    // MEMX's official Day 1 retrospective dates live trading to September 21,
    // 2020.
    // https://memx.com/insights/day-1
    let cutover = et((2020, 9, 21), (0, 0, 0));
    let before = hours_for_exchange_as_of(Exchange::MemxEq, cutover - chrono::Duration::seconds(1));
    let launched = hours_for_exchange_as_of(Exchange::MemxEq, cutover);

    assert!(before.regular.is_empty());
    assert!(before.extended.is_empty());
    assert!(launched.is_open_extended(et((2020, 9, 21), (7, 0, 0))));
    assert!(launched.is_open_regular(et((2020, 9, 21), (10, 0, 0))));
}

#[test]
fn memx_0400_premarket_started_2025_05_19() {
    // MEMX's 2025-06-06 retrospective identifies May 19 as the production
    // launch of 04:00 ET trading.
    // https://memx.com/insights/pre-market-share-gains-and-new-options-active-risk-feature
    let cutover = et((2025, 5, 19), (0, 0, 0));
    let before = hours_for_exchange_as_of(Exchange::MemxEq, cutover - chrono::Duration::seconds(1));
    let after = hours_for_exchange_as_of(Exchange::MemxEq, cutover);

    assert!(!before.is_open(et((2025, 5, 19), (5, 0, 0))));
    assert!(before.is_open_extended(et((2025, 5, 19), (7, 0, 0))));
    assert!(after.is_open_extended(et((2025, 5, 19), (4, 0, 0))));
}

#[test]
fn memx_postmarket_close_changed_in_2020_and_2023() {
    // MEMX's operator alerts make the 20:00→17:00 and restoration boundaries
    // effective 2020-10-05 and 2023-02-01, respectively.
    // https://info.memxtrading.com/trader-alert-20-06-memx-market-hours-change/
    // https://info.memxtrading.com/trader-alert-23-04-memx-trading-hours-change/
    let launch_grid = hours_for_exchange_as_of(Exchange::MemxEq, et((2020, 10, 2), (12, 0, 0)));
    let short_grid = hours_for_exchange_as_of(Exchange::MemxEq, et((2020, 10, 5), (0, 0, 0)));
    let restored = hours_for_exchange_as_of(Exchange::MemxEq, et((2023, 2, 1), (0, 0, 0)));

    assert!(launch_grid.is_open_extended(et((2020, 10, 2), (19, 59, 59))));
    assert!(short_grid.is_open_extended(et((2020, 10, 5), (16, 59, 59))));
    assert!(!short_grid.is_open(et((2020, 10, 5), (17, 0, 0))));
    assert!(restored.is_open_regular(et((2023, 2, 1), (15, 59, 59))));
    assert!(restored.is_open_extended(et((2023, 2, 1), (16, 0, 0))));
    assert!(restored.is_open_extended(et((2023, 2, 1), (19, 59, 59))));
    assert!(!restored.is_open(et((2023, 2, 1), (20, 0, 0))));
}

#[test]
fn miax_pearl_is_closed_before_its_2020_09_29_launch() {
    // MIAX's official US-equities history dates Pearl Equities' launch to
    // September 29, 2020.
    // https://www.miaxglobal.com/company/markets/us-equities
    let cutover = et((2020, 9, 29), (0, 0, 0));
    let before = hours_for_exchange_as_of(
        Exchange::MiaxPearlEq,
        cutover - chrono::Duration::seconds(1),
    );
    let launched = hours_for_exchange_as_of(Exchange::MiaxPearlEq, cutover);

    assert!(before.regular.is_empty());
    assert!(before.extended.is_empty());
    assert!(launched.is_open_regular(et((2020, 9, 29), (10, 0, 0))));
    assert!(!launched.is_open(et((2020, 9, 29), (17, 0, 0))));
}

#[test]
fn miax_pearl_extended_sessions_started_2025_02_20() {
    // MIAX Pearl Equities Regulatory Circular 2025-02 made 04:00–09:30 and
    // 16:00–20:00 ET available beginning February 20, 2025.
    // https://www.miaxglobal.com/sites/default/files/circular-files/MIAX_Pearl_Equities_RC_2025_02_0.pdf
    let cutover = et((2025, 2, 20), (0, 0, 0));
    let before = hours_for_exchange_as_of(
        Exchange::MiaxPearlEq,
        cutover - chrono::Duration::seconds(1),
    );
    let after = hours_for_exchange_as_of(Exchange::MiaxPearlEq, cutover);

    assert!(!before.is_open(et((2025, 2, 20), (5, 0, 0))));
    assert!(!before.is_open(et((2025, 2, 20), (17, 0, 0))));
    assert!(after.is_open_extended(et((2025, 2, 20), (4, 0, 0))));
    assert!(after.is_open_extended(et((2025, 2, 20), (19, 59, 59))));
    assert!(!after.is_open(et((2025, 2, 20), (20, 0, 0))));
}

#[test]
fn nasdaq_stockholm_continuous_ends_1725() {
    // Nasdaq Nordic Market Model 2026:03 section 3.1: Stockholm opens each
    // order book randomly from 09:00:00 through 09:00:05 CET, then trades
    // continuously to 17:25 with a closing call to 17:30 and post-trading to
    // 18:00.
    let hours = hours_for_exchange(Exchange::NasdaqStockholm);
    assert!(hours.is_open_extended(cet((2026, 4, 20), (9, 0, 4))));
    assert!(!hours.is_open_regular(cet((2026, 4, 20), (9, 0, 4))));
    assert!(hours.is_open_regular(cet((2026, 4, 20), (9, 0, 5))));
    assert!(hours.is_open_regular(cet((2026, 4, 20), (17, 24, 59))));
    assert!(!hours.is_open_regular(cet((2026, 4, 20), (17, 25, 0))));
    assert!(hours.is_open_extended(cet((2026, 4, 20), (17, 27, 0))));
    assert!(hours.is_open_extended(cet((2026, 4, 20), (17, 45, 0))));
    assert!(!hours.is_open(cet((2026, 4, 20), (18, 0, 0))));
    // Pre-opening from 08:00.
    assert!(hours.is_open_extended(cet((2026, 4, 20), (8, 30, 0))));
}

#[test]
fn nasdaq_helsinki_trades_1000_to_1825_local_eet() {
    // Nasdaq Nordic Market Model 2026:03 publishes Helsinki on the common CET
    // clock. Its venue-local open is therefore the same randomized edge at
    // 10:00:05 EET/EEST, simultaneous with 09:00:05 CET.
    let hours = hours_for_exchange(Exchange::NasdaqHelsinki);
    assert!(!hours.is_open_regular(zoned(Europe::Helsinki, (2026, 4, 20), (9, 30, 0))));
    assert!(hours.is_open_extended(zoned(Europe::Helsinki, (2026, 4, 20), (9, 30, 0))));
    assert!(hours.is_open_extended(zoned(Europe::Helsinki, (2026, 4, 20), (10, 0, 4))));
    assert!(!hours.is_open_regular(zoned(Europe::Helsinki, (2026, 4, 20), (10, 0, 4))));
    assert!(hours.is_open_regular(zoned(Europe::Helsinki, (2026, 4, 20), (10, 0, 5))));
    assert!(hours.is_open_regular(zoned(Europe::Helsinki, (2026, 4, 20), (18, 24, 59))));
    assert!(!hours.is_open_regular(zoned(Europe::Helsinki, (2026, 4, 20), (18, 25, 0))));
    assert!(hours.is_open_extended(zoned(Europe::Helsinki, (2026, 4, 20), (18, 45, 0))));
    assert!(!hours.is_open(zoned(Europe::Helsinki, (2026, 4, 20), (19, 0, 0))));
    // Helsinki's regular open coincides with Stockholm's in absolute time.
    assert_eq!(
        zoned(Europe::Helsinki, (2026, 4, 20), (10, 0, 5)),
        cet((2026, 4, 20), (9, 0, 5)),
        "the Nordic books align on CET"
    );
}

#[test]
fn nasdaq_copenhagen_closes_1655_with_optional_trade_at_close() {
    // Nasdaq Nordic Market Model 2026:03: randomized open through 09:00:05,
    // continuous trading to 16:55, closing call to 17:00, and
    // Trading@Closing Price 17:00-17:10, then post-trading through 17:20.
    let hours = hours_for_exchange(Exchange::NasdaqCopenhagen);
    assert!(hours.is_open_extended(cet((2026, 4, 20), (9, 0, 4))));
    assert!(!hours.is_open_regular(cet((2026, 4, 20), (9, 0, 4))));
    assert!(hours.is_open_regular(cet((2026, 4, 20), (9, 0, 5))));
    assert!(hours.is_open_regular(cet((2026, 4, 20), (16, 54, 59))));
    assert!(!hours.is_open_regular(cet((2026, 4, 20), (16, 55, 0))));
    assert!(hours.is_open_extended(cet((2026, 4, 20), (17, 5, 0))));
    assert!(hours.is_open_extended(cet((2026, 4, 20), (17, 15, 0))));
    assert!(!hours.is_open(cet((2026, 4, 20), (17, 20, 0))));
}

#[test]
fn euronext_central_books_use_nominal_exchange_boundaries() {
    // Euronext's current 4-01/4-03 appendix publishes the nominal venue-level
    // timetable: pre-open 07:30, continuous trading 09:00-17:30, closing
    // auction 17:30-17:35, and Trading-at-Last 17:35-17:40 CET. Per-security
    // randomized uncross microtiming does not move these exchange boundaries.
    for exch in [
        Exchange::EuronextParis,
        Exchange::EuronextAmsterdam,
        Exchange::EuronextBrussels,
    ] {
        let hours = hours_for_exchange(exch);
        assert!(!hours.is_open(cet((2026, 4, 20), (7, 29, 59))));
        assert!(hours.is_order_entry_only(cet((2026, 4, 20), (7, 30, 0))));
        assert!(hours.is_order_entry_only(cet((2026, 4, 20), (8, 59, 59))));
        assert!(hours.is_open_regular(cet((2026, 4, 20), (9, 0, 0))));
        assert!(!hours.is_open_extended(cet((2026, 4, 20), (9, 0, 0))));
        assert!(hours.is_open_regular(cet((2026, 4, 20), (17, 29, 59))));
        assert!(hours.is_open_extended(cet((2026, 4, 20), (17, 30, 0))));
        assert!(!hours.is_open_regular(cet((2026, 4, 20), (17, 30, 0))));
        let (_, auction_close) = session_bounds_with(
            &hours,
            cet((2026, 4, 20), (17, 34, 59)),
            SessionKind::Extended,
        )
        .expect("Euronext closing auction");
        assert_eq!(auction_close, cet((2026, 4, 20), (17, 35, 0)));
        let (tal_open, tal_close) = session_bounds_with(
            &hours,
            cet((2026, 4, 20), (17, 35, 0)),
            SessionKind::Extended,
        )
        .expect("Euronext Trading-at-Last");
        assert_eq!(tal_open, cet((2026, 4, 20), (17, 35, 0)));
        assert_eq!(tal_close, cet((2026, 4, 20), (17, 40, 0)));
        assert!(
            hours.is_open_extended(cet((2026, 4, 20), (17, 37, 0))),
            "{exch:?} must run Trading-at-Last to 17:40 CET"
        );
        assert!(
            !hours.is_open(cet((2026, 4, 20), (17, 40, 0))),
            "{exch:?} must be closed at 17:40 (end-exclusive)"
        );
    }
}

#[test]
fn euronext_lisbon_keeps_its_zone_but_follows_the_cet_book() {
    // Euronext publishes all cash-market hours in CET/CEST. Lisbon keeps its
    // venue zone, so every phase is one local-clock hour earlier while
    // remaining simultaneous with the continental books.
    let hours = hours_for_exchange(Exchange::EuronextLisbon);
    assert_eq!(hours.tz, Europe::Lisbon);
    assert!(!hours.is_open(zoned(Europe::Lisbon, (2026, 4, 20), (6, 29, 59))));
    assert!(hours.is_order_entry_only(zoned(Europe::Lisbon, (2026, 4, 20), (6, 30, 0))));
    assert!(hours.is_order_entry_only(zoned(Europe::Lisbon, (2026, 4, 20), (7, 59, 59))));
    assert!(hours.is_open_regular(zoned(Europe::Lisbon, (2026, 4, 20), (8, 0, 0))));
    assert!(!hours.is_open_extended(zoned(Europe::Lisbon, (2026, 4, 20), (8, 0, 0))));
    assert_eq!(
        zoned(Europe::Lisbon, (2026, 4, 20), (8, 0, 0)),
        cet((2026, 4, 20), (9, 0, 0)),
        "Lisbon and continental Euronext books must open simultaneously"
    );
    assert!(hours.is_open_regular(zoned(Europe::Lisbon, (2026, 4, 20), (16, 29, 59))));
    assert!(hours.is_open_extended(zoned(Europe::Lisbon, (2026, 4, 20), (16, 30, 0))));
    assert!(!hours.is_open_regular(zoned(Europe::Lisbon, (2026, 4, 20), (16, 30, 0))));
    let (_, auction_close) = session_bounds_with(
        &hours,
        zoned(Europe::Lisbon, (2026, 4, 20), (16, 34, 59)),
        SessionKind::Extended,
    )
    .expect("Euronext Lisbon closing auction");
    assert_eq!(
        auction_close,
        zoned(Europe::Lisbon, (2026, 4, 20), (16, 35, 0))
    );
    let (tal_open, tal_close) = session_bounds_with(
        &hours,
        zoned(Europe::Lisbon, (2026, 4, 20), (16, 35, 0)),
        SessionKind::Extended,
    )
    .expect("Euronext Lisbon Trading-at-Last");
    assert_eq!(tal_open, zoned(Europe::Lisbon, (2026, 4, 20), (16, 35, 0)));
    assert_eq!(tal_close, zoned(Europe::Lisbon, (2026, 4, 20), (16, 40, 0)));
    assert!(hours.is_open_extended(zoned(Europe::Lisbon, (2026, 4, 20), (16, 37, 0))));
    assert!(!hours.is_open(zoned(Europe::Lisbon, (2026, 4, 20), (16, 40, 0))));
}

#[test]
fn euronext_dublin_uses_the_distinct_cet_close_in_its_venue_zone() {
    // The current 4-01/4-03 appendix gives Dublin shares the common CET open
    // but a 17:28 CET continuous close, followed by closing/TAL through 17:40.
    let hours = hours_for_exchange(Exchange::EuronextDublin);
    assert_eq!(hours.tz, Europe::Dublin);
    assert!(!hours.is_open(zoned(Europe::Dublin, (2026, 4, 20), (6, 29, 59))));
    assert!(hours.is_order_entry_only(zoned(Europe::Dublin, (2026, 4, 20), (6, 30, 0))));
    assert!(hours.is_open_extended(zoned(Europe::Dublin, (2026, 4, 20), (8, 0, 29))));
    assert!(hours.is_open_regular(zoned(Europe::Dublin, (2026, 4, 20), (8, 0, 30))));
    assert!(hours.is_open_regular(zoned(Europe::Dublin, (2026, 4, 20), (16, 27, 59))));
    assert!(!hours.is_open_regular(zoned(Europe::Dublin, (2026, 4, 20), (16, 28, 0))));
    assert!(hours.is_open_extended(zoned(Europe::Dublin, (2026, 4, 20), (16, 29, 0))));
    let (_, auction_close) = session_bounds_with(
        &hours,
        zoned(Europe::Dublin, (2026, 4, 20), (16, 30, 29)),
        SessionKind::Extended,
    )
    .expect("Dublin closing auction");
    assert_eq!(
        auction_close,
        zoned(Europe::Dublin, (2026, 4, 20), (16, 30, 30))
    );
    let (tal_open, tal_close) = session_bounds_with(
        &hours,
        zoned(Europe::Dublin, (2026, 4, 20), (16, 30, 30)),
        SessionKind::Extended,
    )
    .expect("Dublin Trading-at-Last");
    assert_eq!(tal_open, zoned(Europe::Dublin, (2026, 4, 20), (16, 30, 30)));
    assert_eq!(tal_close, zoned(Europe::Dublin, (2026, 4, 20), (16, 40, 0)));
    assert!(hours.is_open_extended(zoned(Europe::Dublin, (2026, 4, 20), (16, 35, 0))));
    assert!(!hours.is_open(zoned(Europe::Dublin, (2026, 4, 20), (16, 40, 0))));
}

#[test]
fn lse_closing_price_crossing_runs_to_1640() {
    // LSE Guide to the Trading System (MIT201): Closing Auction Call
    // 16:30-16:35, Closing Price Crossing Session to 16:40.
    let hours = hours_for_exchange(Exchange::Lse);
    assert!(hours.is_open_extended(lon((2026, 4, 20), (16, 37, 0))));
    assert!(!hours.is_open(lon((2026, 4, 20), (16, 40, 0))));
}

#[test]
fn bme_and_vienna_run_post_close_phases() {
    let bme = hours_for_exchange(Exchange::Bme);
    assert!(bme.is_open_extended(cet((2026, 4, 20), (17, 40, 0))));
    assert!(!bme.is_open(cet((2026, 4, 20), (17, 45, 0))));

    let vienna = hours_for_exchange(Exchange::Vienna);
    assert!(vienna.is_open_extended(cet((2026, 4, 20), (17, 40, 0))));
    assert!(vienna.is_order_entry_only(cet((2026, 4, 20), (17, 45, 0))));
    assert!(!vienna.is_open(cet((2026, 4, 20), (17, 50, 0))));
}

#[test]
fn xetra_extended_retail_has_a_sourced_current_cutover() {
    let current = hours_for_exchange(Exchange::Xetra);
    assert!(!current.is_open(cet((2026, 4, 20), (6, 59, 59))));
    assert!(current.is_order_entry_only(cet((2026, 4, 20), (7, 0, 0))));
    assert!(!current.is_open_regular(cet((2026, 4, 20), (9, 0, 29))));
    assert!(current.is_open_regular(cet((2026, 4, 20), (9, 0, 30))));
    assert!(current.is_open_extended(cet((2026, 4, 20), (13, 1, 0))));
    assert!(current.is_open_regular(cet((2026, 4, 20), (13, 2, 30))));
    assert!(!current.is_open_regular(cet((2026, 4, 20), (17, 30, 0))));
    assert!(current.is_open_extended(cet((2026, 4, 20), (17, 40, 0))));
    assert!(current.is_open_extended(cet((2026, 4, 20), (21, 59, 59))));
    assert!(current.is_order_entry_only(cet((2026, 4, 20), (22, 4, 59))));
    assert!(!current.is_open(cet((2026, 4, 20), (22, 5, 0))));

    let before = hours_for_exchange_as_of(Exchange::Xetra, cet((2025, 11, 28), (12, 0, 0)));
    assert!(!before.is_open(cet((2025, 11, 28), (7, 29, 59))));
    assert!(before.is_order_entry_only(cet((2025, 11, 28), (7, 30, 0))));
    assert!(before.is_open_extended(cet((2025, 11, 28), (17, 44, 59))));
    assert!(before.is_order_entry_only(cet((2025, 11, 28), (20, 29, 59))));
    assert!(!before.is_open(cet((2025, 11, 28), (20, 30, 0))));

    let after = hours_for_exchange_as_of(Exchange::Xetra, cet((2025, 12, 1), (0, 0, 0)));
    assert!(after.is_order_entry_only(cet((2025, 12, 1), (7, 0, 0))));
    assert!(after.is_open_extended(cet((2025, 12, 1), (21, 59, 59))));
    assert!(after.is_order_entry_only(cet((2025, 12, 1), (22, 4, 59))));
}
