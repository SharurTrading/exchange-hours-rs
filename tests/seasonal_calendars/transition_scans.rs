// SPDX-License-Identifier: MIT-0

//! Date-aware scans across recurring profile transitions.

use super::prelude::*;

#[test]
fn date_aware_scan_reselects_bmv_grid_over_spring_and_fall_weekends() {
    let calendar = calendar_for_exchange(Exchange::Bmv);
    let tz = America::Mexico_City;

    let spring_friday_close = local(tz, (2024, 3, 8), (15, 20, 0));
    let spring_next = calendar
        .next_session_after(spring_friday_close)
        .expect("BMV reopens Monday");
    assert_eq!(spring_next.0, local(tz, (2024, 3, 11), (7, 0, 0)));
    assert_eq!(spring_next.1, local(tz, (2024, 3, 11), (7, 30, 0)));
    assert_eq!(
        calendar.session_bounds(spring_friday_close),
        Some(spring_next)
    );
    assert_eq!(
        calendar.next_session_after_with(spring_friday_close, SessionKind::Regular),
        Some((
            local(tz, (2024, 3, 11), (7, 30, 0)),
            local(tz, (2024, 3, 11), (14, 0, 0)),
        ))
    );

    let fall_friday_close = local(tz, (2024, 11, 1), (14, 20, 0));
    assert_eq!(
        calendar.next_session_after(fall_friday_close),
        Some((
            local(tz, (2024, 11, 4), (8, 0, 0)),
            local(tz, (2024, 11, 4), (8, 30, 0)),
        ))
    );
    assert_eq!(
        calendar.next_session_after_with(fall_friday_close, SessionKind::Regular),
        Some((
            local(tz, (2024, 11, 4), (8, 30, 0)),
            local(tz, (2024, 11, 4), (15, 0, 0)),
        ))
    );
}

#[test]
fn resolved_snapshot_is_exact_at_its_instant_but_not_across_a_transition() {
    let calendar = calendar_for_exchange(Exchange::Bmv);
    let tz = America::Mexico_City;
    let friday = local(tz, (2024, 3, 8), (14, 0, 0));
    let friday_snapshot = hours_for_exchange_as_of(Exchange::Bmv, friday);

    assert_eq!(calendar.hours_at(friday), friday_snapshot);
    assert_eq!(calendar.is_open(friday), friday_snapshot.is_open(friday));

    let monday = local(tz, (2024, 3, 11), (7, 45, 0));
    assert!(calendar.is_open_regular(monday));
    assert!(!friday_snapshot.is_open_regular(monday));

    let friday_close = local(tz, (2024, 3, 8), (15, 20, 0));
    assert_eq!(
        calendar.next_session_open_after(friday_close),
        Some(local(tz, (2024, 3, 11), (7, 0, 0)))
    );
    assert_eq!(
        next_session_after(&friday_snapshot, friday_close).map(|(open, _close)| open),
        Some(local(tz, (2024, 3, 11), (8, 0, 0)))
    );
}
