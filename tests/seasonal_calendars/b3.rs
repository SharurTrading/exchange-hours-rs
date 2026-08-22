// SPDX-License-Identifier: MIT-0

//! B3 explicit-history and recurring reference-zone selection.

use super::prelude::*;

#[test]
fn b3_selects_current_short_and_northern_winter_long_grids() {
    let calendar = ExchangeCalendar::new(Exchange::B3);
    let tz = America::Sao_Paulo;
    let short_day = (2026, 8, 19);
    assert_eq!(
        hours_for_exchange(Exchange::B3),
        calendar.hours_at(local(tz, short_day, (12, 0, 0)))
    );

    assert!(!calendar.is_open(local(tz, short_day, (9, 44, 59))));
    assert!(calendar.is_open_extended(local(tz, short_day, (9, 45, 0))));
    assert!(calendar.is_open_regular(local(tz, short_day, (10, 0, 0))));
    assert!(!calendar.is_open_regular(local(tz, short_day, (16, 55, 0))));
    assert!(calendar.is_open_extended(local(tz, short_day, (16, 55, 0))));
    assert!(!calendar.is_open(local(tz, short_day, (17, 0, 0))));
    assert!(calendar.is_open_extended(local(tz, short_day, (17, 30, 0))));
    assert!(!calendar.is_open(local(tz, short_day, (18, 0, 0))));
    assert_eq!(
        regular_window(&calendar.hours_at(local(tz, short_day, (12, 0, 0)))),
        (10 * 3600, 16 * 3600 + 55 * 60)
    );
    assert_eq!(
        calendar.normal_week_open_seconds_containing(local(tz, short_day, (12, 0, 0))),
        139_500
    );

    let long_day = (2026, 1, 14);
    assert!(calendar.is_open_regular(local(tz, long_day, (17, 30, 0))));
    assert!(!calendar.is_open_regular(local(tz, long_day, (17, 55, 0))));
    assert!(calendar.is_open_extended(local(tz, long_day, (17, 55, 0))));
    assert!(!calendar.is_open(local(tz, long_day, (18, 0, 0))));
    assert_eq!(
        regular_window(&calendar.hours_at(local(tz, long_day, (12, 0, 0)))),
        (10 * 3600, 17 * 3600 + 55 * 60)
    );
    assert_eq!(
        calendar.normal_week_open_seconds_containing(local(tz, long_day, (12, 0, 0))),
        148_500
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
    assert!(calendar.is_open_extended(local(tz, old_long_day, (10, 45, 0))));
    assert!(calendar.is_open_regular(local(tz, old_long_day, (11, 0, 0))));
    assert!(calendar.is_open_extended(local(tz, old_long_day, (17, 55, 0))));
    assert!(calendar.is_open_extended(local(tz, old_long_day, (18, 30, 0))));
    assert!(!calendar.is_open(local(tz, old_long_day, (19, 30, 0))));

    let old_short_day = (2010, 4, 1);
    assert!(calendar.is_open_extended(local(tz, old_short_day, (9, 45, 0))));
    assert!(calendar.is_open_regular(local(tz, old_short_day, (10, 0, 0))));
    assert!(calendar.is_open_extended(local(tz, old_short_day, (17, 30, 0))));
    assert!(!calendar.is_open(local(tz, old_short_day, (19, 0, 0))));

    let interim_day = (2013, 1, 9);
    assert!(!calendar.is_open(local(tz, interim_day, (17, 30, 0))));
    assert!(calendar.is_open_extended(local(tz, interim_day, (18, 0, 0))));
    assert!(!calendar.is_open(local(tz, interim_day, (19, 30, 0))));
}
