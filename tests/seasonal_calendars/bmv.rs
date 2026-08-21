// SPDX-License-Identifier: MIT-0

//! BMV bounded notices and recurring reference-zone selection.

use super::prelude::*;

#[test]
fn bmv_2010_sources_switch_at_local_midnight() {
    let calendar = calendar_for_exchange(Exchange::Bmv);
    let tz = America::Mexico_City;
    let cases = [
        (
            (2010, 3, 16),
            (8 * 3600 + 30 * 60, 15 * 3600),
            (7 * 3600 + 30 * 60, 14 * 3600),
        ),
        (
            (2010, 4, 1),
            (7 * 3600 + 30 * 60, 14 * 3600),
            (8 * 3600 + 30 * 60, 15 * 3600),
        ),
        (
            (2010, 11, 1),
            (8 * 3600 + 30 * 60, 15 * 3600),
            (7 * 3600 + 30 * 60, 14 * 3600),
        ),
    ];

    for (date, before, after) in cases {
        let midnight = local(tz, date, (0, 0, 0));
        assert_eq!(
            regular_window(&calendar.hours_at(midnight - Duration::nanoseconds(1))),
            before
        );
        assert_eq!(regular_window(&calendar.hours_at(midnight)), after);
    }

    let probe = (2026, 8, 19);
    let normal = calendar.hours_at(local(tz, (2010, 1, 4), (12, 0, 0)));
    assert!(!normal.is_open(local(tz, probe, (7, 59, 59))));
    assert!(normal.is_open_extended(local(tz, probe, (8, 0, 0))));
    assert!(normal.is_open_regular(local(tz, probe, (8, 30, 0))));
    assert!(!normal.is_open(local(tz, probe, (15, 0, 0))));

    let early = calendar.hours_at(local(tz, (2010, 3, 16), (12, 0, 0)));
    assert!(!early.is_open(local(tz, probe, (6, 59, 59))));
    assert!(early.is_open_extended(local(tz, probe, (7, 0, 0))));
    assert!(early.is_open_regular(local(tz, probe, (7, 30, 0))));
    assert!(!early.is_open(local(tz, probe, (14, 0, 0))));
}

#[test]
fn bmv_reference_rule_recurs_across_us_clock_transitions() {
    let calendar = calendar_for_exchange(Exchange::Bmv);
    let tz = America::Mexico_City;
    let normal = (8 * 3600 + 30 * 60, 15 * 3600);
    let early = (7 * 3600 + 30 * 60, 14 * 3600);
    let cases = [
        ((2010, 11, 5), early),
        ((2010, 11, 8), normal),
        ((2011, 3, 11), normal),
        ((2011, 3, 14), early),
        ((2011, 4, 1), early),
        ((2011, 4, 4), normal),
        ((2023, 3, 10), normal),
        ((2023, 3, 13), early),
        ((2023, 11, 3), early),
        ((2023, 11, 6), normal),
        ((2024, 3, 8), normal),
        ((2024, 3, 11), early),
        ((2024, 11, 1), early),
        ((2024, 11, 4), normal),
        ((2026, 3, 6), normal),
        ((2026, 3, 9), early),
        ((2026, 10, 30), early),
        ((2026, 11, 2), normal),
    ];

    assert_eq!(
        hours_for_exchange(Exchange::Bmv),
        calendar.hours_at(local(tz, (2026, 8, 19), (12, 0, 0)))
    );

    for (date, expected) in cases {
        assert_eq!(
            regular_window(&calendar.hours_at(local(tz, date, (12, 0, 0)))),
            expected,
            "unexpected BMV grid on {date:?}"
        );
    }
}
