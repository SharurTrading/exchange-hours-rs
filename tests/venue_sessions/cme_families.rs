// SPDX-License-Identifier: MIT-0

//! CME Group product-family current and historical contracts.

use super::prelude::*;

#[test]
fn interest_rates_current_profile_is_the_extended_17_to_16_grid() {
    let profile = session_profile(MarketHoursKey::GlobexInterestRates);
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexInterestRates);

    assert!(profile.regular.is_empty());
    assert!(profile.is_open(ct((2026, 4, 19), (17, 0, 0))));
    assert!(profile.is_open(ct((2026, 4, 20), (15, 59, 59))));
    assert!(!profile.is_open(ct((2026, 4, 20), (16, 0, 0))));
    assert!(!profile.is_open(ct((2026, 4, 20), (16, 59, 59))));
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
    let before = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexInterestRates,
        ct((2011, 10, 1), (12, 0, 0)),
    );
    let after = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexInterestRates,
        ct((2011, 10, 2), (0, 0, 0)),
    );

    assert!(!before.is_open(ct((2011, 10, 2), (17, 29, 59))));
    assert!(before.is_open_extended(ct((2011, 10, 2), (17, 30, 0))));
    assert!(!after.is_open(ct((2011, 10, 2), (16, 59, 59))));
    assert!(after.is_open_extended(ct((2011, 10, 2), (17, 0, 0))));
}

#[test]
fn livestock_current_profile_is_the_weekday_day_session() {
    let profile = session_profile(MarketHoursKey::GlobexLivestock);
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexLivestock);

    assert!(profile.extended.is_empty());
    assert!(!profile.is_open(ct((2026, 4, 20), (8, 29, 59))));
    assert!(profile.is_open(ct((2026, 4, 20), (8, 30, 0))));
    assert!(profile.is_open(ct((2026, 4, 20), (13, 4, 59))));
    assert!(!profile.is_open(ct((2026, 4, 20), (13, 5, 0))));
    assert!(!profile.is_open(ct((2026, 4, 25), (10, 0, 0))));
    assert_eq!(
        calendar.next_session_open_after(ct((2026, 4, 24), (13, 5, 0))),
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
fn livestock_history_keeps_both_sourced_reductions() {
    let before_2014 = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexLivestock,
        ct((2014, 10, 26), (12, 0, 0)),
    );
    let from_2014 = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexLivestock,
        ct((2014, 10, 27), (0, 0, 0)),
    );
    assert!(before_2014.is_open_regular(ct((2014, 10, 27), (17, 0, 0))));
    assert!(!from_2014.is_open(ct((2014, 10, 27), (17, 0, 0))));
    assert!(from_2014.is_open_regular(ct((2014, 10, 28), (8, 0, 0))));

    let before_2016 = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexLivestock,
        ct((2016, 2, 28), (12, 0, 0)),
    );
    let from_2016 = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexLivestock,
        ct((2016, 2, 29), (0, 0, 0)),
    );
    assert!(!before_2016.is_open(ct((2016, 2, 29), (8, 30, 0))));
    assert!(from_2016.is_open_regular(ct((2016, 2, 29), (8, 30, 0))));
    assert!(!from_2016.is_open(ct((2016, 2, 29), (13, 5, 0))));
}

#[test]
fn cryptocurrency_current_profile_preserves_exact_open_state() {
    let profile = session_profile(MarketHoursKey::GlobexCryptocurrency);

    assert!(profile.regular.is_empty());
    assert!(!profile.has_weekend_close);
    assert!(profile.is_open(ct((2026, 6, 5), (15, 59, 59))));
    assert!(!profile.is_open(ct((2026, 6, 5), (16, 0, 0))));
    assert!(profile.is_open(ct((2026, 6, 5), (16, 2, 0))));
    assert!(profile.is_open(ct((2026, 6, 6), (1, 59, 59))));
    assert!(!profile.is_open(ct((2026, 6, 6), (2, 0, 0))));
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
    let fixed = hours_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency);
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
        Some((ct((2026, 6, 8), (16, 2, 0)), ct((2026, 6, 9), (16, 0, 0)),)),
    );

    for instant in [
        ct((2026, 6, 5), (17, 0, 0)),
        ct((2026, 6, 6), (5, 0, 0)),
        ct((2026, 6, 7), (12, 0, 0)),
        ct((2026, 6, 8), (10, 0, 0)),
    ] {
        assert_eq!(
            calendar.candle_start(instant, CalendarResolution::Daily),
            Some(ct((2026, 6, 5), (16, 2, 0))),
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
            Some(ct((2026, 6, 5), (16, 2, 0))),
        );
        assert_eq!(
            calendar.candle_end(instant, CalendarResolution::Weekly),
            Some(ct((2026, 6, 12), (16, 0, 0))),
        );
    }

    assert_eq!(
        calendar.candle_start(ct((2026, 6, 5), (15, 0, 0)), CalendarResolution::Weekly),
        Some(ct((2026, 5, 29), (16, 2, 0))),
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
    let before_launch = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexCryptocurrency,
        ct((2017, 12, 16), (12, 0, 0)),
    );
    let launch = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexCryptocurrency,
        ct((2017, 12, 17), (0, 0, 0)),
    );
    assert!(!before_launch.is_open(ct((2017, 12, 17), (17, 0, 0))));
    assert!(!launch.is_open(ct((2017, 12, 17), (16, 59, 59))));
    assert!(launch.is_open_extended(ct((2017, 12, 17), (17, 0, 0))));

    let five_day = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexCryptocurrency,
        ct((2026, 5, 28), (12, 0, 0)),
    );
    let seven_day = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexCryptocurrency,
        ct((2026, 5, 29), (0, 0, 0)),
    );
    assert!(!five_day.is_open(ct((2026, 5, 29), (16, 2, 0))));
    assert!(!seven_day.is_open(ct((2026, 5, 29), (16, 1, 59))));
    assert!(seven_day.is_open_extended(ct((2026, 5, 29), (16, 2, 0))));
    assert_eq!(
        session_bounds(&seven_day, ct((2026, 5, 29), (15, 0, 0))),
        Some((ct((2026, 5, 28), (17, 0, 0)), ct((2026, 5, 29), (16, 0, 0)),)),
        "the transition snapshot retains the real Thursday session open"
    );
    let next = session_bounds(&seven_day, ct((2026, 5, 29), (16, 0, 0)))
        .expect("the transition snapshot reopens Friday");
    assert_eq!(next.0, ct((2026, 5, 29), (16, 2, 0)));

    let temporary = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexCryptocurrency,
        ct((2026, 8, 1), (0, 0, 0)),
    );
    assert!(!temporary.is_open(ct((2026, 8, 1), (8, 59, 59))));
    assert!(temporary.is_open_extended(ct((2026, 8, 1), (9, 0, 0))));

    let restored = hours_for_market_hours_key_as_of(
        MarketHoursKey::GlobexCryptocurrency,
        ct((2026, 8, 2), (0, 0, 0)),
    );
    assert!(restored.is_open_extended(ct((2026, 8, 8), (4, 0, 0))));
}

#[test]
fn family_calendars_reselect_the_new_cme_histories() {
    let interest = calendar_for_market_hours_key(MarketHoursKey::GlobexInterestRates);
    assert!(!interest.is_open(ct((2011, 9, 25), (17, 0, 0))));
    assert!(interest.is_open(ct((2011, 10, 2), (17, 0, 0))));

    let livestock = calendar_for_market_hours_key(MarketHoursKey::GlobexLivestock);
    assert!(livestock.is_open(ct((2014, 10, 20), (17, 0, 0))));
    assert!(!livestock.is_open(ct((2014, 10, 27), (17, 0, 0))));

    let crypto = calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency);
    assert!(!crypto.is_open(ct((2026, 5, 29), (16, 1, 59))));
    assert!(crypto.is_open(ct((2026, 5, 29), (16, 2, 0))));
    assert_eq!(
        crypto.session_bounds(ct((2026, 5, 29), (15, 0, 0))),
        Some((ct((2026, 5, 28), (17, 0, 0)), ct((2026, 5, 29), (16, 0, 0)),)),
    );
    let next = crypto
        .session_bounds(ct((2026, 5, 29), (16, 0, 0)))
        .expect("the date-aware calendar reopens Friday");
    assert_eq!(next.0, ct((2026, 5, 29), (16, 2, 0)));
}
