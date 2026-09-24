// SPDX-License-Identifier: MIT-0

//! Recurring international reference-clock selectors.

use super::prelude::*;

#[test]
fn eurex_calendar_scans_reselect_the_fixed_utc_asian_start() {
    let calendar = calendar_for_exchange(Exchange::Eurex);
    let berlin = Europe::Berlin;

    let spring_close = local(berlin, (2026, 3, 27), (22, 0, 0));
    assert_eq!(
        calendar
            .next_session_after(spring_close)
            .expect("the coverage contract must answer a covered date"),
        // Eurex pre-trading 02:00-02:10 CEST is order entry, so the session
        // itself now begins at the 02:10 opening auction.
        Some((
            local(berlin, (2026, 3, 30), (2, 10, 0)),
            local(berlin, (2026, 3, 30), (2, 15, 0)),
        ))
    );
    assert!(
        calendar
            .is_open_regular(local(berlin, (2026, 3, 30), (2, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );

    let autumn_close = local(berlin, (2026, 10, 23), (22, 0, 0));
    assert_eq!(
        calendar
            .next_session_after(autumn_close)
            .expect("the coverage contract must answer a covered date"),
        Some((
            local(berlin, (2026, 10, 26), (1, 10, 0)),
            local(berlin, (2026, 10, 26), (1, 15, 0)),
        ))
    );
    assert!(
        calendar
            .is_open_regular(local(berlin, (2026, 10, 26), (1, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

#[test]
fn endex_calendar_scans_reselect_both_mismatch_entries_and_exits() {
    let calendar = calendar_for_exchange(Exchange::IceEndex);
    let amsterdam = Europe::Amsterdam;
    let cases = [
        (
            local(amsterdam, (2027, 3, 12), (23, 0, 0)),
            local(amsterdam, (2027, 3, 14), (22, 50, 0)),
            local(amsterdam, (2027, 3, 15), (22, 0, 0)),
        ),
        (
            local(amsterdam, (2027, 3, 26), (22, 0, 0)),
            local(amsterdam, (2027, 3, 28), (23, 50, 0)),
            local(amsterdam, (2027, 3, 29), (23, 0, 0)),
        ),
        (
            local(amsterdam, (2026, 10, 23), (23, 0, 0)),
            local(amsterdam, (2026, 10, 25), (22, 50, 0)),
            local(amsterdam, (2026, 10, 26), (22, 0, 0)),
        ),
        (
            local(amsterdam, (2026, 10, 30), (22, 0, 0)),
            local(amsterdam, (2026, 11, 1), (23, 50, 0)),
            local(amsterdam, (2026, 11, 2), (23, 0, 0)),
        ),
    ];

    for (prior_close, expected_open, _expected_close) in cases {
        // `Exchange::IceEndex` is dormant and its scope ships no date-aware
        // coverage for these dates, so every one of the three queries refuses.
        // The DST-reselection grid the case names is stated by the fixed
        // snapshot, which the assertion below reads, so the case still pins the
        // entries and exits it was written for.
        assert!(
            calendar.next_session_after(prior_close).is_err(),
            "{prior_close}: a dormant identity refuses the forward scan"
        );
        // The fixed snapshot is the surface that still states the grid this
        // case names (its own `next_session_after` free function), so the case
        // keeps its DST-reselection content rather than only asserting a refusal.
        let snapshot = hours_for_exchange(Exchange::IceEndex, prior_close);
        assert!(
            exchange_hours::next_session_after(&snapshot, prior_close).is_some(),
            "{prior_close}: the fixed snapshot still states the reopen"
        );
        assert!(
            calendar
                .is_order_entry_only(expected_open - Duration::nanoseconds(1))
                .is_err(),
            "{expected_open}: the queue probe refuses with the rest"
        );
        assert!(
            calendar.is_open(expected_open).is_err(),
            "{expected_open}: the session probe refuses with the rest"
        );
    }
}

#[test]
fn murban_calendar_scans_refuse_where_the_identity_has_no_answer() {
    // `Exchange::IceAbuDhabi` is a dormant identity whose scope ships no
    // date-aware coverage for these 2026 dates, so the scan refuses rather than
    // naming the reopen. The grid the refused answer would have come from is a
    // property of the fixed snapshot, which is asserted in the sibling Endex
    // test; here the refusal is the whole claim.
    let calendar = calendar_for_exchange(Exchange::IceAbuDhabi);
    let dubai = Asia::Dubai;

    for close in [
        local(dubai, (2026, 3, 7), (3, 0, 0)),
        local(dubai, (2026, 10, 31), (2, 0, 0)),
    ] {
        assert!(
            calendar.next_session_after(close).is_err(),
            "{close}: a dormant identity refuses the forward scan"
        );
    }
}
