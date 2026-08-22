// SPDX-License-Identifier: MIT-0

//! Recurring international reference-clock selectors.

use super::prelude::*;

#[test]
fn eurex_calendar_scans_reselect_the_fixed_utc_asian_start() {
    let calendar = calendar_for_exchange(Exchange::Eurex);
    let berlin = Europe::Berlin;

    let spring_close = local(berlin, (2026, 3, 27), (22, 0, 0));
    assert_eq!(
        calendar.next_session_after(spring_close),
        Some((
            local(berlin, (2026, 3, 30), (2, 0, 0)),
            local(berlin, (2026, 3, 30), (2, 15, 0)),
        ))
    );
    assert!(calendar.is_open_regular(local(berlin, (2026, 3, 30), (2, 15, 0))));

    let autumn_close = local(berlin, (2026, 10, 23), (22, 0, 0));
    assert_eq!(
        calendar.next_session_after(autumn_close),
        Some((
            local(berlin, (2026, 10, 26), (1, 0, 0)),
            local(berlin, (2026, 10, 26), (1, 15, 0)),
        ))
    );
    assert!(calendar.is_open_regular(local(berlin, (2026, 10, 26), (1, 15, 0))));
}

#[test]
fn endex_calendar_scans_reselect_both_mismatch_entries_and_exits() {
    let calendar = calendar_for_exchange(Exchange::IceEndex);
    let amsterdam = Europe::Amsterdam;
    let cases = [
        (
            local(amsterdam, (2027, 3, 12), (23, 0, 0)),
            local(amsterdam, (2027, 3, 14), (22, 40, 0)),
            local(amsterdam, (2027, 3, 14), (22, 50, 0)),
        ),
        (
            local(amsterdam, (2027, 3, 26), (22, 0, 0)),
            local(amsterdam, (2027, 3, 28), (23, 40, 0)),
            local(amsterdam, (2027, 3, 28), (23, 50, 0)),
        ),
        (
            local(amsterdam, (2026, 10, 23), (23, 0, 0)),
            local(amsterdam, (2026, 10, 25), (22, 40, 0)),
            local(amsterdam, (2026, 10, 25), (22, 50, 0)),
        ),
        (
            local(amsterdam, (2026, 10, 30), (22, 0, 0)),
            local(amsterdam, (2026, 11, 1), (23, 40, 0)),
            local(amsterdam, (2026, 11, 1), (23, 50, 0)),
        ),
    ];

    for (prior_close, expected_open, expected_close) in cases {
        assert_eq!(
            calendar.next_session_after(prior_close),
            Some((expected_open, expected_close))
        );
        assert!(!calendar.is_open(expected_open - Duration::nanoseconds(1)));
        assert!(calendar.is_open_extended(expected_open));
    }
}

#[test]
fn murban_calendar_scans_reselect_new_york_dst_in_dubai() {
    let calendar = calendar_for_exchange(Exchange::IceAbuDhabi);
    let dubai = Asia::Dubai;

    let spring_close = local(dubai, (2026, 3, 7), (3, 0, 0));
    assert_eq!(
        calendar.next_session_after(spring_close),
        Some((
            local(dubai, (2026, 3, 9), (1, 0, 0)),
            local(dubai, (2026, 3, 9), (2, 0, 0)),
        ))
    );

    let autumn_close = local(dubai, (2026, 10, 31), (2, 0, 0));
    assert_eq!(
        calendar.next_session_after(autumn_close),
        Some((
            local(dubai, (2026, 11, 2), (2, 0, 0)),
            local(dubai, (2026, 11, 2), (3, 0, 0)),
        ))
    );
}
