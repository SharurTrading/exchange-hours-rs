// SPDX-License-Identifier: MIT-0

//! B3 explicit-history and recurring reference-zone selection.

use super::prelude::*;

#[test]
fn b3_selects_current_short_and_northern_winter_long_grids() {
    let calendar = ExchangeCalendar::new(Exchange::B3);
    let tz = America::Sao_Paulo;
    let short_day = (2026, 8, 19);
    assert_eq!(
        hours_for_exchange(
            Exchange::B3,
            chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000)
        ),
        calendar.hours_at(local(tz, short_day, (12, 0, 0)))
    );

    // B3 is now a served identity whose built-in holiday table audits
    // 2025-01-01..2026-12-31, so probes inside that window answer through the
    // date-aware surface instead of refusing. 2026-08-19 is an ordinary
    // Wednesday on the short grid.
    assert!(
        !calendar
            .is_open(local(tz, short_day, (9, 44, 59)))
            .expect("an audited date answers"),
        "the short day is closed before its pre-abertura"
    );
    // B3 pre-abertura: order entry ahead of the opening call.
    assert!(
        calendar
            .is_order_entry_only(local(tz, short_day, (9, 45, 0)))
            .expect("an audited date answers"),
        "the short day accepts pre-abertura orders from 09:45"
    );
    assert!(
        calendar
            .is_open_regular(local(tz, short_day, (10, 0, 0)))
            .expect("an audited date answers"),
        "the short day opens its regular session at 10:00"
    );
    assert!(
        !calendar
            .is_open_regular(local(tz, short_day, (16, 55, 0)))
            .expect("an audited date answers"),
        "the regular session ends at 16:55"
    );
    assert!(
        calendar
            .is_open_extended(local(tz, short_day, (16, 55, 0)))
            .expect("an audited date answers"),
        "the closing call at 16:55 is extended"
    );
    assert!(
        !calendar
            .is_open(local(tz, short_day, (17, 0, 0)))
            .expect("an audited date answers"),
        "the short day is closed between the closing call and the after-market"
    );
    assert!(
        calendar
            .is_open_extended(local(tz, short_day, (17, 30, 0)))
            .expect("an audited date answers"),
        "the after-market envelope opens at 17:30"
    );
    assert!(
        !calendar
            .is_open(local(tz, short_day, (18, 0, 0)))
            .expect("an audited date answers"),
        "the after-market envelope ends at 18:00"
    );
    assert_eq!(
        regular_window(&calendar.hours_at(local(tz, short_day, (12, 0, 0)))),
        (10 * 3600, 16 * 3600 + 55 * 60)
    );
    // Five ordinary short-grid days: regular 10:00-16:55, closing call to
    // 17:00, after-market 17:30-18:00.
    assert_eq!(
        calendar
            .normal_week_open_seconds_containing(local(tz, short_day, (12, 0, 0)))
            .expect("an audited week answers"),
        5 * (24_900 + 300 + 1_800)
    );

    let long_day = (2026, 1, 14);
    assert!(
        calendar
            .is_open_regular(local(tz, long_day, (17, 30, 0)))
            .expect("an audited date answers"),
        "the long day still trades its regular session at 17:30"
    );
    assert!(
        !calendar
            .is_open_regular(local(tz, long_day, (17, 55, 0)))
            .expect("an audited date answers"),
        "the long day's regular session ends at 17:55"
    );
    assert!(
        calendar
            .is_open_extended(local(tz, long_day, (17, 55, 0)))
            .expect("an audited date answers"),
        "the long day's closing call is extended"
    );
    assert!(
        !calendar
            .is_open(local(tz, long_day, (18, 0, 0)))
            .expect("an audited date answers"),
        "the long day closes at 18:00"
    );
    assert_eq!(
        regular_window(&calendar.hours_at(local(tz, long_day, (12, 0, 0)))),
        (10 * 3600, 17 * 3600 + 55 * 60)
    );
    // Five ordinary long-grid days: regular 10:00-17:55 and closing call to
    // 18:00.
    assert_eq!(
        calendar
            .normal_week_open_seconds_containing(local(tz, long_day, (12, 0, 0)))
            .expect("an audited week answers"),
        5 * (28_500 + 300)
    );

    // Outside the audited window the identity still refuses rather than
    // answering: 2027 has no audited B3 holiday coverage yet.
    let unaudited = local(tz, (2027, 8, 18), (12, 0, 0));
    assert!(
        calendar
            .normal_week_open_seconds_containing(unaudited)
            .is_err_and(|error| matches!(error, CalendarQueryError::OutsideCoveredRange { .. })),
        "2027 is outside the audited window"
    );
    assert!(
        calendar.is_open(unaudited).is_err(),
        "a date past the audited window refuses"
    );
}

#[test]
fn b3_reference_grid_cutover_and_historical_offset_regimes_are_exact() {
    let calendar = calendar_for_exchange(Exchange::B3);
    let tz = America::Sao_Paulo;
    let fixed_short = local(tz, (2013, 7, 8), (0, 0, 0));

    assert_eq!(
        regular_window(&calendar.hours_at(fixed_short - Duration::nanoseconds(1))),
        (10 * 3600, 17 * 3600 + 25 * 60)
    );
    assert_eq!(
        regular_window(&calendar.hours_at(fixed_short)),
        (10 * 3600, 16 * 3600 + 55 * 60)
    );

    let cutover = local(tz, (2015, 12, 21), (0, 0, 0));

    assert_eq!(
        regular_window(&calendar.hours_at(cutover - Duration::nanoseconds(1))),
        (10 * 3600, 16 * 3600 + 55 * 60)
    );
    assert_eq!(
        regular_window(&calendar.hours_at(cutover)),
        (10 * 3600, 17 * 3600 + 55 * 60)
    );

    // Brazil was on summer time in January 2018, widening its offset from
    // New York; by March, New York alone was on daylight time.
    assert_eq!(
        regular_window(&calendar.hours_at(local(tz, (2018, 1, 15), (12, 0, 0)))),
        (10 * 3600, 17 * 3600 + 55 * 60)
    );
    assert_eq!(
        regular_window(&calendar.hours_at(local(tz, (2018, 3, 15), (12, 0, 0)))),
        (10 * 3600, 16 * 3600 + 55 * 60)
    );
}

#[test]
fn b3_explicit_2010_to_2012_grids_and_cutovers_are_preserved() {
    let calendar = calendar_for_exchange(Exchange::B3);
    let tz = America::Sao_Paulo;
    let old_long = (11 * 3600, 17 * 3600 + 55 * 60);
    let old_short = (10 * 3600, 16 * 3600 + 55 * 60);
    let interim = (10 * 3600, 17 * 3600 + 25 * 60);
    let cases = [
        ((2010, 3, 15), old_long, old_short),
        ((2010, 10, 18), old_short, old_long),
        ((2011, 3, 14), old_long, old_short),
        ((2011, 10, 17), old_short, old_long),
        ((2012, 3, 12), old_long, old_short),
        ((2012, 12, 3), old_short, interim),
    ];
    for (date, before, after) in cases {
        let midnight = local(tz, date, (0, 0, 0));
        assert_eq!(
            regular_window(&calendar.hours_at(midnight - Duration::nanoseconds(1))),
            before
        );
        assert_eq!(regular_window(&calendar.hours_at(midnight)), after);
    }

    let old_long_day = (2010, 1, 6);
    // The 2010 long-session grid's pre-abertura window is not primary-sourced,
    // so 10:45 reads closed rather than order entry.
    assert!(
        calendar
            .is_accepting_orders(local(tz, old_long_day, (10, 45, 0)))
            .is_err(),
        "a dormant identity refuses the closed probe"
    );
    assert!(
        calendar
            .is_open_regular(local(tz, old_long_day, (11, 0, 0)))
            .is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        calendar
            .is_open_extended(local(tz, old_long_day, (17, 55, 0)))
            .is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        calendar
            .is_open_extended(local(tz, old_long_day, (18, 30, 0)))
            .is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        calendar
            .is_open(local(tz, old_long_day, (19, 30, 0)))
            .is_err(),
        "a dormant identity refuses the closed probe"
    );

    let old_short_day = (2010, 4, 1);
    assert!(
        calendar
            .is_order_entry_only(local(tz, old_short_day, (9, 45, 0)))
            .is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        calendar
            .is_open_regular(local(tz, old_short_day, (10, 0, 0)))
            .is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        calendar
            .is_open_extended(local(tz, old_short_day, (17, 30, 0)))
            .is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        calendar
            .is_open(local(tz, old_short_day, (19, 0, 0)))
            .is_err(),
        "a dormant identity refuses the closed probe"
    );

    // The interim-grid day now sits inside the audited holiday window
    // (2011-01-01..2024-12-31), so its probes answer through the date-aware
    // surface instead of refusing: the closing call ends at 17:30, the
    // after-market envelope runs 18:00-19:30.
    let interim_day = (2013, 1, 9);
    assert!(
        !calendar
            .is_open(local(tz, interim_day, (17, 30, 0)))
            .expect("an audited date answers"),
        "the interim day is closed between the closing call and the after-market"
    );
    assert!(
        calendar
            .is_open_extended(local(tz, interim_day, (18, 0, 0)))
            .expect("an audited date answers"),
        "the interim day's after-market envelope opens at 18:00"
    );
    assert!(
        !calendar
            .is_open(local(tz, interim_day, (19, 30, 0)))
            .expect("an audited date answers"),
        "the interim day's after-market envelope ends at 19:30"
    );
}
