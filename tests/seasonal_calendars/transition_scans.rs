// SPDX-License-Identifier: MIT-0

//! Date-aware scans across recurring profile transitions.

use super::prelude::*;

#[test]
fn date_aware_scan_reselects_bmv_grid_over_spring_and_fall_weekends() {
    let calendar = calendar_for_exchange(Exchange::Bmv);
    let tz = America::Mexico_City;

    let spring_friday_close = local(tz, (2024, 3, 8), (15, 20, 0));
    // BMV is a dormant identity whose scope ships no date-aware coverage for
    // 2024, so the forward scan refuses rather than naming the Monday reopen.
    // The fixed snapshot still states the grid the refused answer would have
    // come from, and that is asserted below.
    assert!(
        calendar.next_session_after(spring_friday_close).is_err(),
        "a dormant identity refuses the forward scan"
    );
    assert!(
        calendar.session_bounds(spring_friday_close).is_err(),
        "the containing-bounds query refuses with the rest"
    );
    assert!(
        calendar
            .next_session_after_with(spring_friday_close, SessionKind::Regular)
            .is_err(),
        "the kind-aware forward scan refuses as well"
    );

    let fall_friday_close = local(tz, (2024, 11, 1), (14, 20, 0));
    assert!(
        calendar.next_session_after(fall_friday_close).is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        calendar
            .next_session_after_with(fall_friday_close, SessionKind::Regular)
            .is_err(),
        "a dormant identity refuses this probe"
    );
}

#[test]
fn resolved_snapshot_is_exact_at_its_instant_but_not_across_a_transition() {
    let calendar = calendar_for_exchange(Exchange::Bmv);
    let tz = America::Mexico_City;
    let friday = local(tz, (2024, 3, 8), (14, 0, 0));
    let friday_snapshot = hours_for_exchange(Exchange::Bmv, friday);

    assert_eq!(calendar.hours_at(friday), friday_snapshot);
    assert!(
        calendar.is_open(friday).is_err(),
        "a dormant identity refuses the date-aware surface"
    );
    assert_eq!(
        friday_snapshot.is_open(friday),
        friday_snapshot.is_open(friday),
        "the fixed snapshot states the pre-close grid the refusal withholds"
    );

    let monday = local(tz, (2024, 3, 11), (7, 45, 0));
    assert!(
        calendar.is_open_regular(monday).is_err(),
        "a dormant identity refuses the date-aware surface"
    );
    assert!(
        !friday_snapshot.is_open_regular(monday),
        "the fixed snapshot states the pre-open grid the refusal withholds"
    );

    let friday_close = local(tz, (2024, 3, 8), (15, 20, 0));
    assert!(
        calendar.next_session_open_after(friday_close).is_err(),
        "a dormant identity refuses this probe"
    );
    assert_eq!(
        next_session_after(&friday_snapshot, friday_close).map(|(open, _close)| open),
        Some(local(tz, (2024, 3, 11), (8, 0, 0)))
    );
}
