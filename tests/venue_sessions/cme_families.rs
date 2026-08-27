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
    assert_eq!(
        calendar.session_state(ct((2026, 4, 20), (16, 30, 0))),
        SessionState::Maintenance,
    );
    assert_eq!(
        calendar.candle_end(ct((2026, 4, 20), (10, 0, 0)), CalendarResolution::Daily,),
        Some(ct((2026, 4, 20), (16, 0, 0))),
    );
    assert_eq!(
        calendar.next_session_open_after(ct((2026, 4, 24), (16, 30, 0))),
        Some(ct((2026, 4, 26), (17, 0, 0))),
    );
}

#[test]
fn interest_rates_keep_central_wall_clock_across_dst() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexInterestRates);

    assert_eq!(
        calendar.session_bounds(ct((2026, 3, 1), (18, 0, 0))),
        Some((ct((2026, 3, 1), (17, 0, 0)), ct((2026, 3, 2), (16, 0, 0)))),
    );
    assert_eq!(
        calendar.session_bounds(ct((2026, 3, 8), (18, 0, 0))),
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
    assert!(from_2013_queue.is_order_entry_only(ct((2013, 8, 19), (8, 0, 0))));
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
        calendar.next_session_open_after(ct((2026, 4, 24), (13, 5, 0))),
        // 08:00 is livestock's pre-open queue; the session opens at 08:30.
        Some(ct((2026, 4, 27), (8, 30, 0))),
    );
    assert_eq!(
        calendar.candle_end(ct((2026, 4, 20), (9, 0, 0)), CalendarResolution::Daily,),
        Some(ct((2026, 4, 20), (13, 5, 0))),
    );
    let saturday = chrono::NaiveDate::from_ymd_opt(2026, 4, 25).expect("valid fixture date");
    assert!(calendar.is_closed_all_day_on(saturday, SessionKind::Both));
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
            !dated.is_open(sunday_queue),
            "{key:?} dated history omits the queue whose onset day is unsourced"
        );
    }

    let grains_fixed = hours_for_market_hours_key(
        MarketHoursKey::GlobexGrains,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let grains_dated = calendar_for_market_hours_key(MarketHoursKey::GlobexGrains);
    let sunday_queue = ct((2026, 4, 19), (16, 30, 0));
    let pcp = ct((2026, 4, 20), (15, 0, 0));
    assert!(grains_fixed.is_order_entry_only(sunday_queue));
    assert!(!grains_dated.is_open(sunday_queue));
    assert!(grains_fixed.is_order_entry_only(pcp));
    assert!(!grains_dated.is_open(pcp));

    let livestock_fixed = hours_for_market_hours_key(
        MarketHoursKey::GlobexLivestock,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let livestock_dated = calendar_for_market_hours_key(MarketHoursKey::GlobexLivestock);
    assert!(livestock_fixed.is_order_entry_only(pcp));
    assert!(!livestock_dated.is_open(pcp));

    let crypto_dated = calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency);
    assert!(!crypto_dated.is_open(ct((2025, 4, 20), (16, 30, 0))));
    assert!(crypto_dated.is_open(ct((2025, 4, 20), (17, 0, 0))));
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
    assert!(profile.is_open(ct((2026, 6, 5), (16, 1, 0))));
    assert!(hours.is_open_extended(ct((2026, 6, 5), (16, 1, 0))));
    assert!(profile.is_open(ct((2026, 6, 5), (16, 2, 0))));
    assert!(profile.is_open(ct((2026, 6, 6), (1, 59, 59))));
    assert!(!profile.is_open(ct((2026, 6, 6), (2, 0, 0))));
    assert!(!profile.is_open(ct((2026, 6, 6), (3, 44, 59))));
    assert!(hours.is_open_extended(ct((2026, 6, 6), (3, 45, 0))));
    assert!(profile.is_open(ct((2026, 6, 6), (4, 0, 0))));
    assert!(profile.is_open(ct((2026, 6, 7), (0, 0, 0))));
}

#[test]
fn cryptocurrency_calendar_joins_weekend_pieces_and_assigns_monday_trade_date() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency);
    let monday = chrono::NaiveDate::from_ymd_opt(2026, 6, 8).expect("valid fixture date");
    let first_block = (ct((2026, 6, 5), (16, 1, 0)), ct((2026, 6, 6), (2, 0, 0)));
    let second_block = (ct((2026, 6, 6), (3, 45, 0)), ct((2026, 6, 8), (16, 0, 0)));

    for instant in [ct((2026, 6, 5), (17, 0, 0)), ct((2026, 6, 6), (1, 0, 0))] {
        assert_eq!(
            calendar.session_bounds_with(instant, SessionKind::Extended),
            Some(first_block),
        );
        assert_eq!(calendar.session_bounds(instant), Some(first_block));
        assert_eq!(calendar.trade_date(instant), Some(monday));
    }

    for instant in [
        ct((2026, 6, 6), (5, 0, 0)),
        ct((2026, 6, 7), (12, 0, 0)),
        ct((2026, 6, 8), (10, 0, 0)),
    ] {
        assert_eq!(
            calendar.session_bounds_with(instant, SessionKind::Extended),
            Some(second_block),
        );
        assert_eq!(calendar.session_bounds(instant), Some(second_block));
        assert_eq!(calendar.trade_date(instant), Some(monday));
    }

    assert_eq!(calendar.trade_date(ct((2026, 6, 6), (3, 0, 0))), None);
    assert_eq!(
        calendar.session_state(ct((2026, 6, 6), (3, 0, 0))),
        SessionState::Maintenance,
        "CME designates the short break inside its continuous week as maintenance",
    );
    assert!(calendar.is_maintenance(ct((2026, 6, 6), (3, 0, 0))));
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
        calendar.next_session_after(ct((2026, 6, 5), (17, 0, 0))),
        Some(second_block),
    );
    assert_eq!(
        calendar.next_session_after(ct((2026, 6, 6), (5, 0, 0))),
        Some((ct((2026, 6, 8), (16, 1, 0)), ct((2026, 6, 9), (16, 0, 0)),)),
    );

    for instant in [
        ct((2026, 6, 5), (17, 0, 0)),
        ct((2026, 6, 6), (5, 0, 0)),
        ct((2026, 6, 7), (12, 0, 0)),
        ct((2026, 6, 8), (10, 0, 0)),
    ] {
        assert_eq!(
            calendar.candle_start(instant, CalendarResolution::Daily),
            Some(ct((2026, 6, 5), (16, 1, 0))),
        );
        assert_eq!(
            calendar.candle_end(instant, CalendarResolution::Daily),
            Some(ct((2026, 6, 8), (16, 0, 0))),
        );
    }

    for instant in [
        ct((2026, 6, 5), (17, 0, 0)),
        ct((2026, 6, 7), (12, 0, 0)),
        ct((2026, 6, 11), (10, 0, 0)),
    ] {
        assert_eq!(
            calendar.candle_start(instant, CalendarResolution::Weekly),
            Some(ct((2026, 6, 5), (16, 1, 0))),
        );
        assert_eq!(
            calendar.candle_end(instant, CalendarResolution::Weekly),
            Some(ct((2026, 6, 12), (16, 0, 0))),
        );
    }

    assert_eq!(
        calendar.candle_start(ct((2026, 6, 5), (15, 0, 0)), CalendarResolution::Weekly),
        Some(ct((2026, 5, 29), (16, 1, 0))),
    );
    assert_eq!(
        calendar.candle_end(ct((2026, 6, 5), (15, 0, 0)), CalendarResolution::Weekly),
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
    assert!(seven_day.is_open_extended(ct((2026, 5, 29), (16, 1, 0))));
    assert!(seven_day.is_open_extended(ct((2026, 5, 29), (16, 2, 0))));
    assert_eq!(
        session_bounds(&seven_day, ct((2026, 5, 29), (15, 0, 0))),
        Some((ct((2026, 5, 28), (17, 0, 0)), ct((2026, 5, 29), (16, 0, 0)),)),
        "the transition snapshot retains the real Thursday session open"
    );
    let next = session_bounds(&seven_day, ct((2026, 5, 29), (16, 0, 0)))
        .expect("the transition snapshot reopens Friday");
    assert_eq!(next.0, ct((2026, 5, 29), (16, 1, 0)));

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
    assert!(normal_before_temporary.is_open_extended(ct((2026, 8, 1), (3, 45, 0))));
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
    assert!(restored.is_open_extended(ct((2026, 8, 8), (3, 45, 0))));
}

#[test]
fn family_calendars_reselect_the_new_cme_histories() {
    let interest = calendar_for_market_hours_key(MarketHoursKey::GlobexInterestRates);
    assert!(!interest.is_open(ct((2011, 9, 25), (16, 14, 59))));
    assert!(interest.is_order_entry_only(ct((2011, 9, 25), (16, 15, 0))));
    assert!(!interest.is_open(ct((2011, 10, 2), (16, 59, 59))));
    assert!(interest.is_open(ct((2011, 10, 2), (17, 0, 0))));

    let livestock = calendar_for_market_hours_key(MarketHoursKey::GlobexLivestock);
    assert!(livestock.is_open(ct((2014, 10, 20), (17, 0, 0))));
    assert!(!livestock.is_open(ct((2014, 10, 27), (17, 0, 0))));

    let crypto = calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency);
    assert!(!crypto.is_open(ct((2026, 5, 29), (16, 0, 59))));
    assert!(crypto.is_open_extended(ct((2026, 5, 29), (16, 1, 0))));
    assert!(crypto.is_open(ct((2026, 5, 29), (16, 2, 0))));
    assert_eq!(
        crypto.session_bounds(ct((2026, 5, 29), (15, 0, 0))),
        Some((ct((2026, 5, 28), (17, 0, 0)), ct((2026, 5, 29), (16, 0, 0)),)),
    );
    let next = crypto
        .session_bounds(ct((2026, 5, 29), (16, 0, 0)))
        .expect("the date-aware calendar reopens Friday");
    assert_eq!(next.0, ct((2026, 5, 29), (16, 1, 0)));
}
