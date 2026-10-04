// SPDX-License-Identifier: MIT-0

//! CME Group product-family current and historical contracts.

use super::prelude::*;

#[test]
fn interest_rates_current_profile_is_the_extended_17_to_16_grid() {
    let profile = session_profile(MarketHoursKey::GlobexInterestRates);
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexInterestRates);

    assert!(profile.regular.is_empty());
    assert!(!profile.is_open(ct((2026, 4, 19), (15, 59, 59))));
    // 16:00 CT Sunday is the pre-open queue: order entry, not a session.
    assert!(profile.is_order_entry_only(ct((2026, 4, 19), (16, 0, 0))));
    assert!(profile.is_open(ct((2026, 4, 19), (17, 0, 0))));
    assert!(profile.is_open(ct((2026, 4, 20), (15, 59, 59))));
    assert!(!profile.is_open(ct((2026, 4, 20), (16, 0, 0))));
    assert!(!profile.is_open(ct((2026, 4, 20), (16, 44, 59))));
    assert!(profile.is_order_entry_only(ct((2026, 4, 20), (16, 45, 0))));
    assert!(profile.is_open(ct((2026, 4, 20), (17, 0, 0))));
    assert!(!profile.is_open(ct((2026, 4, 25), (12, 0, 0))));
    // `globex_interest_rates` serves the #79 quarter-hour's sourced
    // intersection, with the disputed 16:00-16:15 CT slice disclosed as a
    // residual on the bracket-era Sundays: the identity-backed
    // queue scan answers the disputed Sunday instant with the served
    // intersection's closed verdict, while the Monday beside it answers from
    // the sourced weekday grid, so the 16:00→16:45 CT maintenance gap reports
    // as `Maintenance` rather than as a closure. The grid itself, including
    // that gap, stays asserted above through `session_profile`, and the
    // queries answer below.
    assert_eq!(
        calendar.session_state(ct((2026, 4, 20), (16, 30, 0))),
        Ok(SessionState::Maintenance),
        "the Monday 16:00-16:45 CT maintenance gap is sourced: a weekday of the \
         bracket era answers, and only the bracket-era Sundays refuse"
    );
    assert_eq!(
        calendar.session_state(ct((2026, 4, 19), (16, 5, 0))),
        Ok(SessionState::Closed),
        "the disputed quarter-hour answers as the served intersection's closed \
         verdict (the 2026-10-04 residual convention), never as a refusal"
    );
    assert_eq!(
        calendar
            .candle_end(ct((2026, 4, 20), (10, 0, 0)), CalendarResolution::Daily,)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2026, 4, 20), (16, 0, 0))),
    );
    assert_eq!(
        calendar
            .next_session_open_after(ct((2026, 4, 24), (16, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2026, 4, 26), (17, 0, 0))),
    );
}

#[test]
fn interest_rates_keep_central_wall_clock_across_dst() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexInterestRates);

    assert_eq!(
        calendar
            .session_bounds(ct((2026, 3, 1), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2026, 3, 1), (17, 0, 0)), ct((2026, 3, 2), (16, 0, 0)))),
    );
    assert_eq!(
        calendar
            .session_bounds(ct((2026, 3, 8), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2026, 3, 8), (17, 0, 0)), ct((2026, 3, 9), (16, 0, 0)))),
    );
}

#[test]
fn interest_rates_open_changed_on_the_sourced_2011_opening_day() {
    let before = hours_for_market_hours_key(
        MarketHoursKey::GlobexInterestRates,
        ct((2011, 10, 1), (12, 0, 0)),
    );
    let after = hours_for_market_hours_key(
        MarketHoursKey::GlobexInterestRates,
        ct((2011, 10, 2), (0, 0, 0)),
    );

    assert!(!before.is_open(ct((2011, 10, 2), (16, 14, 59))));
    assert!(before.is_order_entry_only(ct((2011, 10, 2), (16, 15, 0))));
    assert!(before.is_order_entry_only(ct((2011, 10, 2), (17, 29, 59))));
    assert!(before.is_open_extended(ct((2011, 10, 2), (17, 30, 0))));
    assert!(!after.is_open(ct((2011, 10, 2), (16, 59, 59))));
    assert!(after.is_open_extended(ct((2011, 10, 2), (17, 0, 0))));
}

#[test]
fn cme_weekday_preopen_changed_on_the_sourced_2010_day() {
    for key in [
        MarketHoursKey::GlobexEquityIndex,
        MarketHoursKey::GlobexFx,
        MarketHoursKey::GlobexInterestRates,
    ] {
        let before = hours_for_market_hours_key(key, ct((2010, 11, 14), (12, 0, 0)));
        let after = hours_for_market_hours_key(key, ct((2010, 11, 15), (0, 0, 0)));
        let early = ct((2010, 11, 15), (16, 45, 0));
        let predecessor = ct((2010, 11, 15), (16, 50, 0));

        assert!(
            !before.is_open(early),
            "{key:?} predecessor starts at 16:50"
        );
        assert!(before.is_order_entry_only(predecessor));
        assert!(after.is_order_entry_only(early));
    }
}

#[test]
fn equity_pause_was_removed_on_the_sourced_2021_opening_day() {
    let before = hours_for_market_hours_key(
        MarketHoursKey::GlobexEquityIndex,
        ct((2021, 6, 26), (12, 0, 0)),
    );
    let after = hours_for_market_hours_key(
        MarketHoursKey::GlobexEquityIndex,
        ct((2021, 6, 27), (0, 0, 0)),
    );
    let pause_start = ct((2021, 6, 28), (15, 15, 0));

    assert!(!before.is_open(pause_start));
    assert!(after.is_open_extended(pause_start));
    assert!(after.is_open_extended(ct((2021, 6, 28), (15, 29, 59))));
}

#[test]
fn grains_keep_exact_sourced_order_phase_revisions() {
    let before_pcp =
        hours_for_market_hours_key(MarketHoursKey::GlobexGrains, ct((2010, 4, 18), (12, 0, 0)));
    let expanded_pcp =
        hours_for_market_hours_key(MarketHoursKey::GlobexGrains, ct((2010, 4, 19), (0, 0, 0)));
    let early_pcp = ct((2010, 4, 19), (13, 15, 30));
    assert!(!before_pcp.is_open(early_pcp));
    assert!(expanded_pcp.is_order_entry_only(early_pcp));

    let before_morning_change =
        hours_for_market_hours_key(MarketHoursKey::GlobexGrains, ct((2011, 12, 26), (12, 0, 0)));
    let after_morning_change =
        hours_for_market_hours_key(MarketHoursKey::GlobexGrains, ct((2011, 12, 27), (0, 0, 0)));
    assert!(before_morning_change.is_order_entry_only(ct((2011, 12, 27), (7, 15, 0))));
    assert!(!after_morning_change.is_open(ct((2011, 12, 27), (7, 59, 59))));
    assert!(after_morning_change.is_order_entry_only(ct((2011, 12, 27), (8, 0, 0))));

    let before_2013_queue =
        hours_for_market_hours_key(MarketHoursKey::GlobexGrains, ct((2013, 8, 17), (12, 0, 0)));
    let from_2013_queue =
        hours_for_market_hours_key(MarketHoursKey::GlobexGrains, ct((2013, 8, 18), (0, 0, 0)));
    assert!(!before_2013_queue.is_open(ct((2013, 8, 19), (8, 0, 0))));
    assert!(before_2013_queue.is_order_entry_only(ct((2013, 8, 19), (8, 15, 0))));
    assert!(from_2013_queue.is_order_entry_only(ct((2013, 8, 19), (8, 0, 0))));
}

#[test]
fn grains_queues_and_pcp_begin_on_the_sourced_2013_opening_day() {
    // The 2013-03-22 Global Command Center notice states every queue's onset
    // with the 19:00 open: Sunday 16:00-19:00, Monday-Thursday 16:45-19:00,
    // morning 08:15-08:30 (widened to 08:00 on 2013-08-18), and PCP
    // 14:30-16:00. Only the 21-hour 2012-05-20..2013-04-06 regime's queues
    // stay omitted.
    let before =
        hours_for_market_hours_key(MarketHoursKey::GlobexGrains, ct((2013, 4, 6), (12, 0, 0)));
    let after =
        hours_for_market_hours_key(MarketHoursKey::GlobexGrains, ct((2013, 4, 7), (12, 0, 0)));

    let sunday_queue = ct((2013, 4, 7), (16, 30, 0));
    assert!(!before.is_open(sunday_queue));
    assert!(after.is_order_entry_only(sunday_queue));
    assert!(after.is_order_entry_only(ct((2013, 4, 7), (18, 59, 59))));
    assert!(after.is_open_extended(ct((2013, 4, 7), (19, 0, 0))));

    let morning_queue = ct((2013, 4, 8), (8, 15, 0));
    let pcp = ct((2013, 4, 8), (15, 0, 0));
    let evening_queue = ct((2013, 4, 8), (17, 0, 0));
    assert!(!after.is_open(ct((2013, 4, 8), (8, 14, 59))));
    assert!(!after.is_order_entry_only(ct((2013, 4, 8), (8, 14, 59))));
    assert!(after.is_order_entry_only(morning_queue));
    assert!(after.is_order_entry_only(ct((2013, 4, 8), (8, 29, 59))));
    assert!(after.is_open_regular(ct((2013, 4, 8), (8, 30, 0))));
    assert!(!before.is_open(pcp));
    assert!(!after.is_order_entry_only(ct((2013, 4, 8), (14, 29, 59))));
    assert!(after.is_order_entry_only(pcp));
    assert!(after.is_order_entry_only(ct((2013, 4, 8), (15, 59, 59))));
    assert!(!after.is_order_entry_only(ct((2013, 4, 8), (16, 0, 0))));
    assert!(after.is_order_entry_only(evening_queue));
    assert!(after.is_open_extended(ct((2013, 4, 8), (19, 0, 0))));
}

#[test]
fn livestock_current_profile_is_the_weekday_day_session() {
    let profile = session_profile(MarketHoursKey::GlobexLivestock);
    let hours = hours_for_market_hours_key(
        MarketHoursKey::GlobexLivestock,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexLivestock);

    // Livestock has a day session plus queues and no electronic overnight
    // session, so `extended` is legitimately empty and the queues are
    // `order_entry`.
    assert!(profile.extended.is_empty());
    assert!(!profile.order_entry.is_empty());
    assert!(!profile.is_open(ct((2026, 4, 20), (7, 59, 59))));
    assert!(profile.is_order_entry_only(ct((2026, 4, 20), (8, 0, 0))));
    assert!(hours.is_order_entry_only(ct((2026, 4, 20), (8, 29, 59))));
    assert!(profile.is_open(ct((2026, 4, 20), (8, 30, 0))));
    assert!(profile.is_open(ct((2026, 4, 20), (13, 4, 59))));
    assert!(!profile.is_open(ct((2026, 4, 20), (13, 5, 0))));
    assert!(!profile.is_open(ct((2026, 4, 20), (14, 29, 59))));
    assert!(hours.is_order_entry_only(ct((2026, 4, 20), (14, 30, 0))));
    assert!(!profile.is_open(ct((2026, 4, 20), (16, 0, 0))));
    assert!(!profile.is_open(ct((2026, 4, 25), (10, 0, 0))));
    assert_eq!(
        calendar
            .next_session_open_after(ct((2026, 4, 24), (13, 5, 0)))
            .expect("the coverage contract must answer a covered date"),
        // 08:00 is livestock's pre-open queue; the session opens at 08:30.
        Some(ct((2026, 4, 27), (8, 30, 0))),
    );
    assert_eq!(
        calendar
            .candle_end(ct((2026, 4, 20), (9, 0, 0)), CalendarResolution::Daily,)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2026, 4, 20), (13, 5, 0))),
    );
    let saturday = chrono::NaiveDate::from_ymd_opt(2026, 4, 25).expect("valid fixture date");
    assert!(
        calendar
            .is_closed_all_day_on(saturday, SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
}

#[test]
fn fixed_current_cme_families_include_operator_published_order_phases() {
    let equity = hours_for_market_hours_key(
        MarketHoursKey::GlobexEquityIndex,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(!equity.is_open(ct((2026, 4, 19), (15, 59, 59))));
    assert!(equity.is_order_entry_only(ct((2026, 4, 19), (16, 0, 0))));
    assert!(equity.is_order_entry_only(ct((2026, 4, 20), (16, 45, 0))));
    assert!(equity.is_open_extended(ct((2026, 4, 20), (15, 15, 0))));

    let energy = hours_for_market_hours_key(
        MarketHoursKey::GlobexEnergy,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(energy.is_order_entry_only(ct((2026, 4, 19), (16, 0, 0))));
    assert!(energy.is_order_entry_only(ct((2026, 4, 20), (16, 45, 0))));

    let fx = hours_for_market_hours_key(
        MarketHoursKey::GlobexFx,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(fx.is_order_entry_only(ct((2026, 4, 19), (16, 0, 0))));
    assert!(fx.is_order_entry_only(ct((2026, 4, 20), (16, 45, 0))));

    let grains = hours_for_market_hours_key(
        MarketHoursKey::GlobexGrains,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert!(grains.is_order_entry_only(ct((2026, 4, 19), (16, 0, 0))));
    assert!(!grains.is_open(ct((2026, 4, 20), (7, 59, 59))));
    assert!(grains.is_order_entry_only(ct((2026, 4, 20), (8, 0, 0))));
    assert!(!grains.is_open(ct((2026, 4, 20), (13, 20, 0))));
    assert!(grains.is_order_entry_only(ct((2026, 4, 20), (14, 30, 0))));
    assert!(!grains.is_open(ct((2026, 4, 20), (16, 0, 0))));
    assert!(grains.is_order_entry_only(ct((2026, 4, 20), (16, 45, 0))));
}

#[test]
fn dated_cme_calendars_expose_the_undated_phase_limit_without_inventing_cutovers() {
    for key in [
        MarketHoursKey::GlobexEquityIndex,
        MarketHoursKey::GlobexEnergy,
        MarketHoursKey::GlobexFx,
        MarketHoursKey::GlobexInterestRates,
    ] {
        let fixed = hours_for_market_hours_key(
            key,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
        );
        let dated = calendar_for_market_hours_key(key);
        let sunday_queue = ct((2026, 4, 19), (16, 30, 0));
        assert!(
            fixed.is_order_entry_only(sunday_queue),
            "{key:?} fixed current queue"
        );
        assert!(
            !dated
                .is_open(sunday_queue)
                .expect("the coverage contract must answer a covered date"),
            "{key:?} dated history omits the queue whose onset day is unsourced"
        );
    }

    let crypto_dated = calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency);
    assert!(
        !crypto_dated
            .is_open(ct((2025, 4, 20), (16, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        crypto_dated
            .is_open(ct((2025, 4, 20), (17, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

#[test]
fn livestock_history_keeps_both_sourced_reductions() {
    let before_2014 = hours_for_market_hours_key(
        MarketHoursKey::GlobexLivestock,
        ct((2014, 10, 26), (12, 0, 0)),
    );
    let from_2014 = hours_for_market_hours_key(
        MarketHoursKey::GlobexLivestock,
        ct((2014, 10, 27), (0, 0, 0)),
    );
    assert!(before_2014.is_open_regular(ct((2014, 10, 27), (17, 0, 0))));
    assert!(!from_2014.is_open(ct((2014, 10, 27), (17, 0, 0))));
    assert!(from_2014.is_open_regular(ct((2014, 10, 28), (8, 0, 0))));

    let before_2016 = hours_for_market_hours_key(
        MarketHoursKey::GlobexLivestock,
        ct((2016, 2, 28), (12, 0, 0)),
    );
    let from_2016 = hours_for_market_hours_key(
        MarketHoursKey::GlobexLivestock,
        ct((2016, 2, 29), (0, 0, 0)),
    );
    assert!(!before_2016.is_open(ct((2016, 2, 29), (8, 30, 0))));
    assert!(from_2016.is_open_regular(ct((2016, 2, 29), (8, 30, 0))));
    assert!(!from_2016.is_open(ct((2016, 2, 29), (13, 5, 0))));
}

#[test]
fn livestock_dated_pcp_begins_on_the_sourced_2016_day() {
    // The 2016-05-30 Globex notice implements the Post-Close state Monday
    // through Friday 14:30-16:00 CT for LE, GF, and HE effective
    // 2016-06-06; trading-hours captures that omit the row between November
    // 2016 and March 2020 are a published-table gap, not a stated removal.
    let before = hours_for_market_hours_key(
        MarketHoursKey::GlobexLivestock,
        ct((2016, 6, 5), (12, 0, 0)),
    );
    let after = hours_for_market_hours_key(
        MarketHoursKey::GlobexLivestock,
        ct((2016, 6, 6), (12, 0, 0)),
    );

    let pcp = ct((2016, 6, 6), (15, 0, 0));
    assert!(!before.is_open(pcp));
    assert!(!after.is_open(ct((2016, 6, 6), (14, 29, 59))));
    assert!(!after.is_order_entry_only(ct((2016, 6, 6), (14, 29, 59))));
    assert!(after.is_order_entry_only(pcp));
    assert!(after.is_order_entry_only(ct((2016, 6, 6), (15, 59, 59))));
    assert!(!after.is_order_entry_only(ct((2016, 6, 6), (16, 0, 0))));
}

#[test]
fn livestock_dated_preopen_begins_at_the_sourced_2020_revision() {
    let before = hours_for_market_hours_key(
        MarketHoursKey::GlobexLivestock,
        ct((2020, 5, 30), (12, 0, 0)),
    );
    let after = hours_for_market_hours_key(
        MarketHoursKey::GlobexLivestock,
        ct((2020, 5, 31), (0, 0, 0)),
    );

    assert!(
        !before.is_open(ct((2020, 6, 1), (6, 0, 0))),
        "the predecessor queue's onset is not invented"
    );
    assert!(!after.is_open(ct((2020, 6, 1), (7, 59, 59))));
    assert!(after.is_order_entry_only(ct((2020, 6, 1), (8, 0, 0))));
}

#[test]
fn cryptocurrency_current_profile_preserves_exact_open_state() {
    let profile = session_profile(MarketHoursKey::GlobexCryptocurrency);
    let hours = hours_for_market_hours_key(
        MarketHoursKey::GlobexCryptocurrency,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );

    assert!(profile.regular.is_empty());
    assert!(!profile.has_weekend_close);
    assert!(profile.is_open(ct((2026, 6, 5), (15, 59, 59))));
    assert!(!profile.is_open(ct((2026, 6, 5), (16, 0, 0))));
    assert!(!profile.is_open(ct((2026, 6, 5), (16, 0, 59))));
    // 16:01-16:02 is the operator's Pre-Open: "Order Entry, modification, and
    // cancel are allowed. No order matching." So it is order entry, not a
    // session, and matching resumes at the 16:02 `open`.
    assert!(!profile.is_open(ct((2026, 6, 5), (16, 1, 0))));
    assert!(!hours.is_open_extended(ct((2026, 6, 5), (16, 1, 0))));
    assert!(hours.is_order_entry_only(ct((2026, 6, 5), (16, 1, 0))));
    assert!(profile.is_open(ct((2026, 6, 5), (16, 2, 0))));
    assert!(profile.is_open(ct((2026, 6, 6), (1, 59, 59))));
    assert!(!profile.is_open(ct((2026, 6, 6), (2, 0, 0))));
    assert!(!profile.is_open(ct((2026, 6, 6), (3, 44, 59))));
    assert!(!hours.is_open_extended(ct((2026, 6, 6), (3, 45, 0))));
    assert!(hours.is_order_entry_only(ct((2026, 6, 6), (3, 45, 0))));
    assert!(profile.is_open(ct((2026, 6, 6), (4, 0, 0))));
    assert!(profile.is_open(ct((2026, 6, 7), (0, 0, 0))));
}

#[test]
fn cryptocurrency_calendar_joins_weekend_pieces_and_assigns_monday_trade_date() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency);
    let monday = chrono::NaiveDate::from_ymd_opt(2026, 6, 8).expect("valid fixture date");
    let first_block = (ct((2026, 6, 5), (16, 2, 0)), ct((2026, 6, 6), (2, 0, 0)));
    let second_block = (ct((2026, 6, 6), (4, 0, 0)), ct((2026, 6, 8), (16, 0, 0)));

    for instant in [ct((2026, 6, 5), (17, 0, 0)), ct((2026, 6, 6), (1, 0, 0))] {
        assert_eq!(
            calendar
                .session_bounds_with(instant, SessionKind::Extended)
                .expect("the coverage contract must answer a covered date"),
            Some(first_block),
        );
        assert_eq!(
            calendar
                .session_bounds(instant)
                .expect("the coverage contract must answer a covered date"),
            Some(first_block)
        );
        assert_eq!(
            calendar
                .trade_date(instant)
                .expect("the coverage contract must answer a covered date"),
            Some(monday)
        );
    }

    for instant in [
        ct((2026, 6, 6), (5, 0, 0)),
        ct((2026, 6, 7), (12, 0, 0)),
        ct((2026, 6, 8), (10, 0, 0)),
    ] {
        assert_eq!(
            calendar
                .session_bounds_with(instant, SessionKind::Extended)
                .expect("the coverage contract must answer a covered date"),
            Some(second_block),
        );
        assert_eq!(
            calendar
                .session_bounds(instant)
                .expect("the coverage contract must answer a covered date"),
            Some(second_block)
        );
        assert_eq!(
            calendar
                .trade_date(instant)
                .expect("the coverage contract must answer a covered date"),
            Some(monday)
        );
    }

    // `globex_cryptocurrency` states every session it publishes from its
    // 2026-05-29 bridge row on, so the maintenance-gap queries on Saturday
    // 2026-06-06 are answered rather than refused: the 02:00-04:00 CT window is
    // `Maintenance`, a gap that carries no trade date of its own, and the fixed
    // snapshot names the same state without key identity.
    assert_eq!(
        calendar
            .trade_date(ct((2026, 6, 6), (3, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        None,
        "a maintenance gap carries no trade date of its own",
    );
    assert_eq!(
        calendar
            .session_state(ct((2026, 6, 6), (3, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        SessionState::Maintenance,
    );
    assert!(
        calendar
            .is_maintenance(ct((2026, 6, 6), (3, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
    );
    let fixed = hours_for_market_hours_key(
        MarketHoursKey::GlobexCryptocurrency,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert_eq!(
        fixed.session_state(ct((2026, 6, 6), (3, 0, 0))),
        SessionState::Maintenance,
        "the source-designated exception is exact without carrying key identity",
    );
    assert_eq!(
        calendar
            .next_session_after(ct((2026, 6, 5), (17, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(second_block),
    );
    assert_eq!(
        calendar
            .next_session_after(ct((2026, 6, 6), (5, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2026, 6, 8), (16, 2, 0)), ct((2026, 6, 9), (16, 0, 0)),)),
    );

    for instant in [
        ct((2026, 6, 5), (17, 0, 0)),
        ct((2026, 6, 6), (5, 0, 0)),
        ct((2026, 6, 7), (12, 0, 0)),
        ct((2026, 6, 8), (10, 0, 0)),
    ] {
        assert_eq!(
            calendar
                .candle_start(instant, CalendarResolution::Daily)
                .expect("the coverage contract must answer a covered date"),
            Some(ct((2026, 6, 5), (16, 2, 0))),
        );
        assert_eq!(
            calendar
                .candle_end(instant, CalendarResolution::Daily)
                .expect("the coverage contract must answer a covered date"),
            Some(ct((2026, 6, 8), (16, 0, 0))),
        );
    }

    for instant in [
        ct((2026, 6, 5), (17, 0, 0)),
        ct((2026, 6, 7), (12, 0, 0)),
        ct((2026, 6, 11), (10, 0, 0)),
    ] {
        assert_eq!(
            calendar
                .candle_start(instant, CalendarResolution::Weekly)
                .expect("the coverage contract must answer a covered date"),
            Some(ct((2026, 6, 5), (16, 2, 0))),
        );
        assert_eq!(
            calendar
                .candle_end(instant, CalendarResolution::Weekly)
                .expect("the coverage contract must answer a covered date"),
            Some(ct((2026, 6, 12), (16, 0, 0))),
        );
    }

    assert_eq!(
        calendar
            .candle_start(ct((2026, 6, 5), (15, 0, 0)), CalendarResolution::Weekly)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2026, 5, 29), (16, 2, 0))),
    );
    assert_eq!(
        calendar
            .candle_end(ct((2026, 6, 5), (15, 0, 0)), CalendarResolution::Weekly)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2026, 6, 5), (16, 0, 0))),
    );

    assert_eq!(
        candle_end(
            &fixed,
            ct((2026, 6, 7), (12, 0, 0)),
            CalendarResolution::Weekly,
        ),
        None,
        "the identity-erased snapshot cannot infer CME's trade-week boundary",
    );
}

#[test]
fn cryptocurrency_history_covers_launch_24_7_and_temporary_maintenance() {
    let before_launch = hours_for_market_hours_key(
        MarketHoursKey::GlobexCryptocurrency,
        ct((2017, 12, 16), (12, 0, 0)),
    );
    let launch = hours_for_market_hours_key(
        MarketHoursKey::GlobexCryptocurrency,
        ct((2017, 12, 17), (0, 0, 0)),
    );
    assert!(!before_launch.is_open(ct((2017, 12, 17), (17, 0, 0))));
    assert!(!launch.is_open(ct((2017, 12, 17), (16, 59, 59))));
    assert!(launch.is_open_extended(ct((2017, 12, 17), (17, 0, 0))));

    let five_day = hours_for_market_hours_key(
        MarketHoursKey::GlobexCryptocurrency,
        ct((2026, 5, 28), (12, 0, 0)),
    );
    let seven_day = hours_for_market_hours_key(
        MarketHoursKey::GlobexCryptocurrency,
        ct((2026, 5, 29), (0, 0, 0)),
    );
    assert!(!five_day.is_open(ct((2026, 5, 29), (16, 1, 0))));
    assert!(!seven_day.is_open(ct((2026, 5, 29), (16, 0, 59))));
    assert!(!seven_day.is_open_extended(ct((2026, 5, 29), (16, 1, 0))));
    assert!(seven_day.is_order_entry_only(ct((2026, 5, 29), (16, 1, 0))));
    assert!(seven_day.is_open_extended(ct((2026, 5, 29), (16, 2, 0))));
    assert_eq!(
        session_bounds(&seven_day, ct((2026, 5, 29), (15, 0, 0))),
        Some((ct((2026, 5, 28), (17, 0, 0)), ct((2026, 5, 29), (16, 0, 0)),)),
        "the transition snapshot retains the real Thursday session open"
    );
    let next = session_bounds(&seven_day, ct((2026, 5, 29), (16, 0, 0)))
        .expect("the transition snapshot reopens Friday");
    assert_eq!(next.0, ct((2026, 5, 29), (16, 2, 0)));

    let transition_day = hours_for_market_hours_key(
        MarketHoursKey::GlobexCryptocurrency,
        ct((2026, 5, 29), (23, 59, 59)),
    );
    let recurring_week = hours_for_market_hours_key(
        MarketHoursKey::GlobexCryptocurrency,
        ct((2026, 5, 30), (0, 0, 0)),
    );
    assert!(!transition_day.is_open(ct((2026, 5, 30), (0, 0, 0))));
    assert!(recurring_week.is_open_extended(ct((2026, 5, 30), (0, 0, 0))));

    let normal_before_temporary = hours_for_market_hours_key(
        MarketHoursKey::GlobexCryptocurrency,
        ct((2026, 7, 31), (23, 59, 59)),
    );

    let temporary = hours_for_market_hours_key(
        MarketHoursKey::GlobexCryptocurrency,
        ct((2026, 8, 1), (0, 0, 0)),
    );
    assert!(!normal_before_temporary.is_open_extended(ct((2026, 8, 1), (3, 45, 0))));
    assert!(!temporary.is_open(ct((2026, 8, 1), (3, 45, 0))));
    assert!(!temporary.is_open(ct((2026, 8, 1), (8, 59, 59))));
    assert!(temporary.is_open_extended(ct((2026, 8, 1), (9, 0, 0))));

    let temporary_before_restoration = hours_for_market_hours_key(
        MarketHoursKey::GlobexCryptocurrency,
        ct((2026, 8, 1), (23, 59, 59)),
    );
    let restored = hours_for_market_hours_key(
        MarketHoursKey::GlobexCryptocurrency,
        ct((2026, 8, 2), (0, 0, 0)),
    );
    assert!(!temporary_before_restoration.is_open(ct((2026, 8, 8), (3, 45, 0))));
    assert!(!restored.is_open(ct((2026, 8, 8), (3, 44, 59))));
    assert!(!restored.is_open_extended(ct((2026, 8, 8), (3, 45, 0))));
    assert!(restored.is_open_extended(ct((2026, 8, 8), (4, 0, 0))));
}

/// Notices 20260824 and 20260921 name four one-day Saturday extensions for the
/// same channels, each reverting to the standard window the following week.
/// The October 24 extension runs 02:00-15:30 CT for the FIA industry
/// disaster-recovery exercise: a 13.5-hour operator-designated gap, past the
/// four-hour bound the crate's maintenance policy keeps, so `session_state`
/// must classify it `Closed` and `is_maintenance` must answer false there.
#[test]
fn cryptocurrency_models_the_later_saturday_extensions() {
    for (saturday, (reopen_hour, reopen_minute), next_saturday) in [
        ((2026, 8, 29), (6u32, 0u32), (2026, 9, 5)),
        ((2026, 9, 19), (8, 0), (2026, 9, 26)),
        ((2026, 10, 3), (5, 0), (2026, 10, 10)),
        ((2026, 10, 24), (15, 30), (2026, 10, 31)),
    ] {
        let temporary = hours_for_market_hours_key(
            MarketHoursKey::GlobexCryptocurrency,
            ct(saturday, (0, 0, 0)),
        );
        assert!(
            temporary.is_open_extended(ct(saturday, (1, 59, 59))),
            "{saturday:?}"
        );
        assert!(
            !temporary.is_open(ct(saturday, (3, 45, 0))),
            "{saturday:?}: no Pre-Open"
        );
        let reopen = ct(saturday, (reopen_hour, reopen_minute, 0));
        assert!(
            !temporary.is_open(reopen - chrono::Duration::seconds(1)),
            "{saturday:?}"
        );
        assert!(
            temporary.is_open_extended(reopen),
            "{saturday:?}: reopens at {reopen_hour}:{reopen_minute:02} CT"
        );
        let restored = hours_for_market_hours_key(
            MarketHoursKey::GlobexCryptocurrency,
            ct(next_saturday, (0, 0, 0)),
        );
        assert!(
            !restored.is_open(ct(next_saturday, (3, 44, 59))),
            "{next_saturday:?}"
        );
        // The extension notices name the window's end and publish no
        // replacement Pre-Open, so the standard 03:45 queue is absent and the
        // revert is to the 04:00 open.
        assert!(
            !restored.is_open_extended(ct(next_saturday, (3, 45, 0))),
            "{next_saturday:?}"
        );
        assert!(
            restored.is_open_extended(ct(next_saturday, (4, 0, 0))),
            "{next_saturday:?}"
        );
    }

    // The 13.5-hour October 24 closure is not a maintenance window: the gap
    // exceeds the four-hour operator-designated bound the crate keeps, and it
    // falls inside one trade date — the weekend block carries the following
    // Monday's — so the policy classifies it Closed, and `is_maintenance`
    // must answer false there.
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency);
    for probe in [
        ct((2026, 10, 24), (2, 0, 0)),
        ct((2026, 10, 24), (8, 0, 0)),
        ct((2026, 10, 24), (15, 29, 59)),
    ] {
        assert_eq!(
            calendar
                .session_state(probe)
                .expect("the coverage contract must answer a covered date"),
            SessionState::Closed,
            "{probe}: the FIA drill closure is same-trade-date closed, not maintenance"
        );
        assert!(
            !calendar
                .is_maintenance(probe)
                .expect("the coverage contract must answer a covered date"),
            "{probe}: `is_maintenance` must stay exactly the maintenance case"
        );
    }

    // Each Saturday's weekend block carries the following Monday's trade
    // date, so the October rows are keyed to the operator's own roll —
    // matching runs on the Saturday, but the trade date it belongs to is the
    // next open business date's.
    assert_eq!(
        calendar
            .trade_date(ct((2026, 10, 3), (6, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(zoned(US::Central, (2026, 10, 5), (0, 0, 0)).date_naive())
    );
    assert_eq!(
        calendar
            .trade_date(ct((2026, 10, 24), (16, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(zoned(US::Central, (2026, 10, 26), (0, 0, 0)).date_naive())
    );
}

#[test]
fn family_calendars_reselect_the_new_cme_histories() {
    let interest = calendar_for_market_hours_key(MarketHoursKey::GlobexInterestRates);
    // 2011-09-25 and 2011-10-02 are the two sides of the family's Sunday-queue
    // revision, and both are pre-floor: the date-aware calendar refuses them as
    // `BeforeSupportFloor` rather than confirming the reselection. The dated
    // selector's own answers stay asserted through `hours_for_market_hours_key`,
    // which is the surface these revision rows are sourced from.
    assert_refuses_before_floor(
        interest.is_open(ct((2011, 9, 25), (16, 14, 59))),
        interest,
        ct((2011, 9, 25), (16, 14, 59)),
    );
    assert_refuses_before_floor(
        interest.is_order_entry_only(ct((2011, 9, 25), (16, 15, 0))),
        interest,
        ct((2011, 9, 25), (16, 15, 0)),
    );
    assert_refuses_before_floor(
        interest.is_open(ct((2011, 10, 2), (16, 59, 59))),
        interest,
        ct((2011, 10, 2), (16, 59, 59)),
    );
    assert_refuses_before_floor(
        interest.is_open(ct((2011, 10, 2), (17, 0, 0))),
        interest,
        ct((2011, 10, 2), (17, 0, 0)),
    );

    // The livestock revision is dated 2014-10-20 and its era is pre-floor, so
    // the date-aware calendar refuses both sides of it as
    // `BeforeSupportFloor`. The revision's own answers stay asserted through
    // `hours_for_market_hours_key`, which is the surface its row is sourced
    // from.
    let livestock = calendar_for_market_hours_key(MarketHoursKey::GlobexLivestock);
    assert_refuses_before_floor(
        livestock.is_open(ct((2014, 10, 20), (17, 0, 0))),
        livestock,
        ct((2014, 10, 20), (17, 0, 0)),
    );
    assert_refuses_before_floor(
        livestock.is_open(ct((2014, 10, 27), (17, 0, 0))),
        livestock,
        ct((2014, 10, 27), (17, 0, 0)),
    );

    let crypto = calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency);
    assert!(
        !crypto
            .is_open(ct((2026, 5, 29), (16, 0, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        crypto
            .is_open_extended(ct((2026, 5, 29), (16, 2, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        crypto
            .is_open(ct((2026, 5, 29), (16, 2, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        crypto
            .session_bounds(ct((2026, 5, 29), (15, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2026, 5, 28), (17, 0, 0)), ct((2026, 5, 29), (16, 0, 0)),)),
    );
    let next = crypto
        .session_bounds(ct((2026, 5, 29), (16, 0, 0)))
        .expect("the coverage contract must answer a covered date")
        .expect("the date-aware calendar reopens Friday");
    assert_eq!(next.0, ct((2026, 5, 29), (16, 2, 0)));
}
