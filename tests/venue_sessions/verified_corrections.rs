// SPDX-License-Identifier: MIT-0

//! Primary-source-verified venue-data corrections.

use super::prelude::*;

// ---------------------------------------------------------------------------
// Venue-data corrections (primary-source verified)
// ---------------------------------------------------------------------------

// IEX runs narrower extended hours than the Reg NMS default: "System Hours" are
// 08:00–17:00 ET, with pre-market 08:00–09:30 and post-market 16:00–17:00.
// Source: IEX Exchange, "Trading Hours & Holidays"
// (https://www.iex.io/resources/trading/trading-hours-holidays); Investors
// Exchange Rule Book Rule 1.160(z)/(aa)/(gg).
#[test]
fn iex_premarket_opens_at_0800_not_0400() {
    let h = hours_for_exchange(
        Exchange::Iex,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(
        !h.is_open(et((2026, 4, 20), (4, 0, 0))),
        "IEX closed 04:00 ET; System Hours start at 08:00"
    );
    assert!(
        !h.is_open(et((2026, 4, 20), (7, 59, 0))),
        "IEX closed 07:59 ET"
    );
    assert!(
        h.is_open_extended(et((2026, 4, 20), (8, 0, 0))),
        "IEX pre-market opens 08:00 ET"
    );
}

#[test]
fn iex_postmarket_closes_at_1700_not_2000() {
    let h = hours_for_exchange(
        Exchange::Iex,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(
        h.is_open_extended(et((2026, 4, 20), (16, 30, 0))),
        "IEX post-market runs 16:00–17:00 ET"
    );
    assert!(
        !h.is_open(et((2026, 4, 20), (17, 0, 0))),
        "IEX closed at 17:00 ET (end-exclusive)"
    );
    assert!(
        !h.is_open(et((2026, 4, 20), (19, 0, 0))),
        "IEX closed 19:00 ET; it runs no 20:00 post-market"
    );
}

#[test]
fn iex_exchange_identity_begins_at_the_2016_08_19_launch() {
    // IEX Trading Alert #2016-042 dates Investors Exchange's first non-test
    // symbols to 2016-08-19 and the final ATS-to-exchange transition to
    // 2016-09-02. The `iex` variant represents the exchange, not its ATS
    // predecessor.
    // https://iextrading.com/trading/alerts/2016/042/
    let launch = et((2016, 8, 19), (0, 0, 0));
    let before = hours_for_exchange(Exchange::Iex, launch - chrono::Duration::seconds(1));
    let exchange = hours_for_exchange(Exchange::Iex, launch);

    assert!(before.regular.is_empty());
    assert!(before.extended.is_empty());
    assert!(exchange.is_open_extended(et((2016, 8, 19), (8, 0, 0))));
    assert!(exchange.is_open_regular(et((2016, 8, 19), (9, 30, 0))));
    assert!(!exchange.is_open(et((2016, 8, 19), (17, 0, 0))));
}

#[test]
fn iex_extended_hours_differ_from_the_generic_us_equities_profile() {
    let iex = hours_for_exchange(
        Exchange::Iex,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let generic = hours_for_exchange(
        Exchange::Nasdaq,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert_eq!(
        iex.regular, generic.regular,
        "IEX shares the Reg NMS regular session"
    );
    assert_ne!(
        iex.extended, generic.extended,
        "IEX must not inherit the 04:00/20:00 generic extended windows"
    );
    assert!(
        generic.is_open(et((2026, 4, 20), (4, 0, 0))),
        "the generic profile does trade at 04:00 ET, so the contrast is real"
    );
}

// The public `blue_ocean_ats` identity represents the production ATS service,
// which launched on 2021-10-05; pre-production beta/testing is out of scope.
// Source: Blue Ocean Technologies, "Announcing Launch of Blue Ocean ATS
// Afterhours Trading" (2021-10-05).
#[test]
fn blue_ocean_has_no_sessions_before_its_production_launch() {
    let before = hours_for_exchange(Exchange::BlueOceanAts, et((2021, 10, 4), (22, 0, 0)));
    assert!(
        before.regular.is_empty() && before.extended.is_empty(),
        "Blue Ocean ATS had not launched on 2021-10-04"
    );

    let after = hours_for_exchange(Exchange::BlueOceanAts, et((2021, 10, 5), (22, 0, 0)));
    assert!(
        !after.extended.is_empty(),
        "Blue Ocean ATS trades from its 2021-10-05 production launch"
    );
    assert!(
        after.is_open(et((2021, 10, 5), (22, 0, 0))),
        "Blue Ocean new-order window starts at 20:00 ET"
    );
}

#[test]
fn blue_ocean_runs_sunday_through_thursday_only() {
    // No Friday-evening session: the NYSE TRF cannot report the trades on
    // Saturday. Source: Blue Ocean SEC Form ATS-N, "Hours of Operations".
    let h = hours_for_exchange(
        Exchange::BlueOceanAts,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(
        h.is_open(et((2026, 4, 19), (22, 0, 0))),
        "Sunday evening session trades"
    );
    assert!(
        !h.is_open(et((2026, 4, 24), (22, 0, 0))),
        "no Friday-evening session"
    );
    assert!(
        !h.is_open(et((2026, 4, 25), (2, 0, 0))),
        "nothing wraps into Saturday morning"
    );
}

#[test]
fn blue_ocean_uses_the_ats_n_new_order_close() {
    // The stable product scope is the new-order trading window. The live
    // ATS-N permits a bounded resting-book cleanup after this edge, but no new
    // orders may enter and that cleanup is outside the profile's stated scope.
    let h = hours_for_exchange(
        Exchange::BlueOceanAts,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(h.is_open_extended(et((2026, 4, 21), (3, 59, 59))));
    assert!(!h.is_open(et((2026, 4, 21), (4, 0, 0))));
}

// CFE removed the 15:15–15:30 queuing period and moved the RTH close to 15:00
// on 2021-12-06, so RTH now flows seamlessly into post-settlement ETH.
// Sources: Cboe notice C2021102603 ("Effective December 6, 2021 … CFE will
// eliminate the queuing period which occurs between 3:15 p.m. CT and 3:30 p.m.
// CT, Monday through Friday … and redefine regular trading hours as 8:30 a.m.
// CT to 3:00 p.m. CT"); CFE rule filing CFE-2021-028 (2021-11-04); CFE Rulebook
// Rule 1202, whose amendment history stamps "December 6, 2021 (21-028)".
#[test]
fn cfe_rth_transitions_seamlessly_into_post_settlement_at_1500() {
    let h = hours_for_exchange(
        Exchange::Cfe,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let close = ct((2026, 4, 20), (15, 0, 0));
    assert!(
        !h.is_open_regular(close),
        "RTH is end-exclusive at 15:00 CT"
    );
    assert!(
        h.is_open_extended(close),
        "ETH takes over at 15:00 CT with no pause"
    );
    assert!(h.is_open(close), "the venue never closes at 15:00 CT");
}

#[test]
fn cfe_has_no_queuing_gap_between_1515_and_1530() {
    // The old model left 15:15–15:30 closed. That queuing period is gone.
    let h = hours_for_exchange(
        Exchange::Cfe,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    for minute in [15, 20, 29] {
        let t = ct((2026, 4, 20), (15, minute, 0));
        assert!(
            h.is_open(t),
            "CFE open at 15:{minute:02} CT since 2021-12-06"
        );
    }
}

#[test]
fn cfe_post_settlement_runs_on_friday_too() {
    // The session is Monday through Friday in both the pre- and post-2021
    // rulebook charts; the old model wrongly limited it to Mon–Thu.
    let h = hours_for_exchange(
        Exchange::Cfe,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(
        h.is_open(ct((2026, 4, 24), (15, 45, 0))),
        "CFE post-settlement trades on Friday"
    );
    assert!(
        !h.is_open(ct((2026, 4, 24), (17, 0, 0))),
        "but there is still no Friday overnight session"
    );
}

#[test]
fn cfe_before_the_2021_cutover_classifies_the_order_queue_as_order_entry() {
    let before = hours_for_exchange(Exchange::Cfe, ct((2021, 12, 3), (12, 0, 0)));
    assert!(
        before.is_open_regular(ct((2021, 12, 3), (15, 10, 0))),
        "RTH ran to 15:15 CT before the cutover"
    );
    assert!(
        before.is_order_entry_only(ct((2021, 12, 3), (15, 20, 0))),
        "15:15–15:30 accepted non-executable orders in the queuing state"
    );
    assert!(
        !before.is_open_regular(ct((2021, 12, 3), (15, 20, 0))),
        "the order-entry queue was not continuous trading"
    );

    let after = hours_for_exchange(Exchange::Cfe, ct((2021, 12, 6), (12, 0, 0)));
    assert!(
        !after.is_open_regular(ct((2021, 12, 6), (15, 10, 0))),
        "RTH ends 15:00 CT from the cutover"
    );
    assert!(
        after.is_open_extended(ct((2021, 12, 6), (15, 20, 0))),
        "the queuing period is replaced by executable extended trading"
    );
}

// SIX Swiss Exchange shares segments: continuous trading ends 17:20, the
// closing auction can uncross as late as 17:32, and Trading-At-Last then runs
// to 17:40. The 17:30–17:35 auction the model previously used belongs to the
// ETF/ETP segments, which have no TAL.
// Source: SIX Group "Trading hours" and the SIX Swiss Exchange Trading Guide
// ("Continuous Trading 09:00 - 17:20 CET / Closing Auction 17:20 - 17:30 CET /
// Trading-At-Last … End: 17:40 CET").
#[test]
fn six_randomized_open_uses_the_latest_two_minute_edge() {
    let h = hours_for_exchange(
        Exchange::Six,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(h.is_open_extended(cet((2026, 4, 20), (9, 1, 59))));
    assert!(!h.is_open_regular(cet((2026, 4, 20), (9, 1, 59))));
    assert!(h.is_open_regular(cet((2026, 4, 20), (9, 2, 0))));
}

#[test]
fn six_continuous_trading_ends_at_1720() {
    let h = hours_for_exchange(
        Exchange::Six,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(
        h.is_open_regular(cet((2026, 4, 20), (17, 19, 0))),
        "SIX continuous trading runs to 17:20"
    );
    assert!(
        !h.is_open_regular(cet((2026, 4, 20), (17, 20, 0))),
        "17:20 is the end-exclusive continuous close"
    );
}

#[test]
fn six_closing_auction_and_trading_at_last_then_post_trading() {
    let h = hours_for_exchange(
        Exchange::Six,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(
        h.is_open_extended(cet((2026, 4, 20), (17, 25, 0))),
        "closing auction 17:20–17:32 at the latest randomized edge"
    );
    assert!(
        h.is_open_extended(cet((2026, 4, 20), (17, 35, 0))),
        "Trading-At-Last begins after the randomized uncross and ends at 17:40"
    );
    let (tal_open, tal_close) =
        session_bounds_with(&h, cet((2026, 4, 20), (17, 35, 0)), SessionKind::Extended)
            .expect("SIX Trading-at-Last phase");
    assert_eq!(tal_open, cet((2026, 4, 20), (17, 32, 0)));
    assert_eq!(tal_close, cet((2026, 4, 20), (17, 40, 0)));
    assert!(
        h.is_order_entry_only(cet((2026, 4, 20), (17, 40, 0))),
        "order-entry-only post-trading follows TAL"
    );
    assert!(h.is_order_entry_only(cet((2026, 4, 20), (21, 59, 59))));
    assert!(!h.is_open(cet((2026, 4, 20), (22, 0, 0))));
}

#[test]
fn six_does_not_share_the_xetra_schedule() {
    let six = hours_for_exchange(
        Exchange::Six,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let xetra = hours_for_exchange(
        Exchange::Xetra,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert_ne!(
        six.regular, xetra.regular,
        "SIX closes continuous trading 10 minutes before Xetra"
    );
    assert!(
        xetra.is_open_regular(cet((2026, 4, 20), (17, 25, 0))),
        "Xetra still trades continuously at 17:25, so the contrast is real"
    );
}
