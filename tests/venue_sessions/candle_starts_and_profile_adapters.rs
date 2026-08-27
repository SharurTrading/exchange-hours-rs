// SPDX-License-Identifier: MIT-0

//! Candle starts, profile adapters, and schedule-duration behavior.

use super::prelude::*;

#[test]
fn candle_start_daily_uses_the_overnight_session_open() {
    let hours = hours_for_market_hours_key(
        MarketHoursKey::GlobexEquityIndex,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let first_trade = ct((2026, 1, 29), (18, 0, 0));

    assert_eq!(
        candle_start(&hours, first_trade, CalendarResolution::Daily),
        Some(ct((2026, 1, 29), (17, 0, 0))),
        "the trading-day candle starts at the session open, not at the 16:45 Pre-Open\n         queue - no trade can print in a queue, so no bar may begin there"
    );
    assert_eq!(
        candle_end(&hours, first_trade, CalendarResolution::Daily),
        Some(ct((2026, 1, 30), (16, 0, 0))),
        "the paired daily close remains the following civil day"
    );
}

#[test]
fn candle_start_resolves_the_post_dst_globex_open() {
    let hours = hours_for_market_hours_key(
        MarketHoursKey::GlobexEquityIndex,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let post_spring_forward_trade = ct((2026, 3, 8), (18, 0, 0));
    let start = candle_start(&hours, post_spring_forward_trade, CalendarResolution::Daily)
        .expect("the Globex week is open");

    assert_eq!(start, ct((2026, 3, 8), (17, 0, 0)));
    assert_eq!(
        start,
        utc((2026, 3, 8), (22, 0, 0)),
        "17:00 CT is 22:00 UTC after the spring DST transition"
    );
}

#[test]
fn candle_start_monthly_can_open_in_the_preceding_civil_month() {
    let hours = hours_for_market_hours_key(
        MarketHoursKey::GlobexEquityIndex,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let first_june_session_trade = ct((2026, 5, 31), (18, 0, 0));

    assert_eq!(
        candle_start(
            &hours,
            first_june_session_trade,
            CalendarResolution::Monthly,
        ),
        // 16:00 CT is the Sunday queue; the session opens at 17:00.
        Some(ct((2026, 5, 31), (17, 0, 0))),
        "June's first trading session opens on the final civil day of May"
    );
    assert_eq!(
        candle_start(
            &hours,
            ct((2026, 6, 30), (12, 0, 0)),
            CalendarResolution::Monthly,
        ),
        Some(ct((2026, 5, 31), (17, 0, 0))),
        "every instant in the same trading month shares one canonical start"
    );
}

#[test]
fn hours_for_market_hours_key_matches_session_profile() {
    // The resolver reproduces the session profile's open/closed decisions; it is
    // a field copy of the same static table, never a second source of truth.
    for key in [
        MarketHoursKey::GlobexEquityIndex,
        MarketHoursKey::GlobexEnergy,
        MarketHoursKey::Eurex,
    ] {
        let hours = hours_for_market_hours_key(
            key,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
        );
        let profile = session_profile(key);
        for t in [
            utc((2026, 1, 5), (10, 0, 0)),
            utc((2026, 1, 5), (23, 30, 0)),
            utc((2026, 1, 4), (12, 0, 0)),
            utc((2026, 1, 3), (12, 0, 0)),
        ] {
            assert_eq!(
                hours.is_open(t),
                profile.is_open(t),
                "key={key:?} t={t} open mismatch"
            );
        }
    }
}

#[test]
fn hours_for_market_hours_key_drives_calendar_boundaries() {
    // The resolver exists for the calendar query surface (`candle_end`,
    // `session_bounds`, weekly/monthly consolidation), so validate the boundary
    // math on its hours — not only `is_open`. A field-copy bug that preserved
    // `is_open` while shifting boundaries would otherwise pass.
    let hours = hours_for_market_hours_key(
        MarketHoursKey::GlobexEquityIndex,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );

    // Daily: a weekday instant resolves to a strictly-later close.
    let wed = ct((2026, 1, 7), (9, 0, 0));
    assert!(
        candle_end(&hours, wed, CalendarResolution::Daily).expect("mid-week close exists") > wed,
        "daily close must be after the input instant"
    );

    // Weekly: idempotent within an ISO week, distinct across weeks.
    let wed_weekly = candle_end(&hours, wed, CalendarResolution::Weekly);
    assert!(wed_weekly.is_some(), "the trading week has a weekly close");
    let thu = ct((2026, 1, 8), (9, 0, 0));
    assert_eq!(
        candle_end(&hours, thu, CalendarResolution::Weekly),
        wed_weekly,
        "Wed and Thu of the same week share one weekly boundary"
    );
    let next_wed = ct((2026, 1, 14), (9, 0, 0));
    assert_ne!(
        candle_end(&hours, next_wed, CalendarResolution::Weekly),
        wed_weekly,
        "the following week has a distinct weekly boundary"
    );

    // Monthly: a mid-January instant resolves to a January close whose next daily
    // close rolls into February.
    let mid_jan = ct((2026, 1, 15), (12, 0, 0));
    let monthly =
        candle_end(&hours, mid_jan, CalendarResolution::Monthly).expect("January has daily closes");
    assert_eq!(
        monthly.with_timezone(&US::Central).month(),
        1,
        "monthly boundary stays in January (exchange-local)"
    );
    assert_eq!(
        candle_end(&hours, monthly, CalendarResolution::Daily)
            .expect("trading continues in February")
            .with_timezone(&US::Central)
            .month(),
        2,
        "the next daily close after the monthly boundary is in February"
    );
}

#[test]
fn normal_week_open_seconds_unions_overlapping_session_rules() {
    let hours = hours_for_market_hours_key(
        MarketHoursKey::GlobexEquityIndex,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );

    assert_eq!(
        hours.normal_week_open_seconds(),
        414_000,
        "down 7_200s from 421_200: Pre-Open queues are order entry and no longer\n         count as scheduled open time; still no post-2021 RTH pause"
    );

    let always_open = hours_for_market_hours_key(
        MarketHoursKey::AlwaysOpen,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert_eq!(
        always_open.normal_week_open_seconds(),
        7 * 86_400,
        "overlapping coverage must never count more than one complete week"
    );
}

#[test]
fn market_hours_derives_eq() {
    // Two profiles built for the same exchange compare equal (the derive Task 3
    // depends on); distinct venues do not.
    let a = hours_for_exchange(
        Exchange::Cme,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let b = hours_for_exchange(
        Exchange::Cme,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    assert_eq!(a, b, "same exchange yields equal hours");
    assert_ne!(
        a,
        hours_for_exchange(
            Exchange::Eurex,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000)
        ),
        "distinct exchanges yield unequal hours"
    );
}
