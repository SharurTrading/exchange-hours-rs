// SPDX-License-Identifier: MIT-0

//! Coinbase Derivatives and Small Exchange launch cutovers and weekly grids.

use super::prelude::*;

// ---------------------------------------------------------------------------
// Coinbase Derivatives (traded as FairX until March 2022)
//   Launch: Monday 2021-06-28 at 08:00 CT; no Sunday-evening session preceded
//   it.
//   Regular: Sun–Thu 17:00 → next day 16:00 CT. Pre-Open 16:50–17:00 from
//   the 2026-09-11 knowledge-bound review row; its onset is undated.
// ---------------------------------------------------------------------------

fn cde(as_of: DateTime<Utc>) -> MarketHours {
    hours_for_exchange(Exchange::CoinbaseDerivatives, as_of)
}

#[test]
fn coinbase_derivatives_is_closed_until_its_launch_instant() {
    let launch = ct((2021, 6, 28), (8, 0, 0));
    let before = cde(launch - chrono::Duration::seconds(1));

    assert!(before.regular.is_empty());
    assert!(before.order_entry.is_empty());
    assert!(!before.is_open(ct((2021, 6, 27), (18, 0, 0))));
    assert!(!before.is_open(ct((2021, 6, 28), (7, 59, 59))));
}

#[test]
fn coinbase_derivatives_first_session_opens_at_the_launch() {
    let launch = ct((2021, 6, 28), (8, 0, 0));
    let h = cde(launch);

    assert!(h.is_open_regular(launch));
    assert_eq!(
        session_bounds(&h, launch),
        Some((launch, ct((2021, 6, 28), (16, 0, 0)))),
        "the first session opens at the launch, not at a Sunday 17:00 that never traded"
    );
    assert!(!h.is_open(ct((2021, 6, 28), (16, 0, 0))));
    assert!(
        !h.is_order_entry_only(ct((2021, 6, 28), (16, 50, 0))),
        "no Pre-Open before the knowledge-bound row"
    );
    assert!(h.is_open_regular(ct((2021, 6, 28), (17, 0, 0))));
}

#[test]
fn coinbase_derivatives_monday_evening_session_survives_the_day_switch() {
    let midnight = ct((2021, 6, 29), (0, 0, 0));
    let evening = (ct((2021, 6, 28), (17, 0, 0)), ct((2021, 6, 29), (16, 0, 0)));

    let launch_day = cde(midnight - chrono::Duration::seconds(1));
    let first_full_day = cde(midnight);
    assert_eq!(
        session_bounds(&launch_day, midnight - chrono::Duration::seconds(1)),
        Some(evening)
    );
    assert_eq!(session_bounds(&first_full_day, midnight), Some(evening));
}

#[test]
fn coinbase_derivatives_weekly_grid_is_unchanged_since_launch() {
    let h = cde(ct((2021, 7, 11), (12, 0, 0)));

    assert!(!h.is_open(ct((2021, 7, 11), (16, 49, 59))));
    assert!(!h.is_order_entry_only(ct((2021, 7, 11), (16, 50, 0))));
    assert!(h.is_open_regular(ct((2021, 7, 11), (17, 0, 0))));
    assert!(!h.is_open(ct((2021, 7, 12), (16, 0, 0))));
    assert!(!h.is_order_entry_only(ct((2021, 7, 12), (16, 50, 0))));
    assert!(h.is_open_regular(ct((2021, 7, 16), (15, 59, 59))));
    assert!(!h.is_open(ct((2021, 7, 16), (16, 0, 0))));
    assert!(!h.is_order_entry_only(ct((2021, 7, 16), (16, 50, 0))));
    assert!(!h.is_open(ct((2021, 7, 18), (16, 0, 0))));

    assert_eq!(h, cde(ct((2026, 8, 24), (12, 0, 0))));
}

#[test]
fn coinbase_derivatives_pre_open_starts_at_the_knowledge_bound_row() {
    let row = ct((2026, 9, 11), (0, 0, 0));
    let before = cde(row - chrono::Duration::seconds(1));
    let after = cde(row);

    assert!(before.order_entry.is_empty());
    assert_eq!(before.regular, after.regular);
    assert!(!after.is_open(ct((2026, 9, 13), (16, 49, 59))));
    assert!(after.is_order_entry_only(ct((2026, 9, 13), (16, 50, 0))));
    assert!(after.is_order_entry_only(ct((2026, 9, 14), (16, 50, 0))));
    assert!(!after.is_order_entry_only(ct((2026, 9, 18), (16, 50, 0))));
    assert!(after.is_open_regular(ct((2026, 9, 13), (17, 0, 0))));
}

// ---------------------------------------------------------------------------
// Small Exchange (now Kraken Derivatives Exchange)
//   Launch: trade date Monday 2020-05-18, 07:00–16:00 CT, Pre-Open from 06:30.
//   From 2024-11-04: 08:30–15:00 CT, Pre-Open from 08:00. The old grid's last
//   sourced statement is 2024-11-03 and S5C's first trade date is 2024-11-25;
//   the new grid lies inside the old one, so it is the sourced intersection.
//   Closed from 2025-03-24: every contract was delisted at the 2025-03-21 close.
// ---------------------------------------------------------------------------

fn smfe(as_of: DateTime<Utc>) -> MarketHours {
    hours_for_exchange(Exchange::Smfe, as_of)
}

#[test]
fn small_exchange_is_closed_before_its_2020_launch() {
    let launch = ct((2020, 5, 18), (0, 0, 0));
    let before = smfe(launch - chrono::Duration::seconds(1));
    assert!(before.regular.is_empty());
    assert!(before.order_entry.is_empty());

    let h = smfe(launch);
    assert!(!h.is_open(ct((2020, 5, 18), (6, 29, 59))));
    assert!(h.is_order_entry_only(ct((2020, 5, 18), (6, 30, 0))));
    assert!(h.is_open_regular(ct((2020, 5, 18), (7, 0, 0))));
    assert!(h.is_open_regular(ct((2020, 5, 18), (15, 59, 59))));
    assert!(!h.is_open(ct((2020, 5, 18), (16, 0, 0))));
    assert!(!h.is_open(ct((2020, 5, 23), (12, 0, 0))));
}

#[test]
fn small_exchange_narrows_to_the_s5c_grid_from_2024_11_04() {
    let old = smfe(ct((2024, 11, 1), (12, 0, 0)));
    assert!(old.is_open_regular(ct((2024, 11, 1), (7, 0, 0))));
    assert!(old.is_open_regular(ct((2024, 11, 1), (15, 30, 0))));

    let h = smfe(ct((2024, 11, 4), (0, 0, 0)));
    assert!(!h.is_open(ct((2024, 11, 4), (7, 0, 0))));
    assert!(h.is_order_entry_only(ct((2024, 11, 4), (8, 0, 0))));
    assert!(h.is_open_regular(ct((2024, 11, 4), (8, 30, 0))));
    assert!(!h.is_open(ct((2024, 11, 4), (15, 0, 0))));
    assert_eq!(
        h,
        smfe(ct((2024, 11, 25), (12, 0, 0))),
        "S5C's first trade date adds no further change"
    );
}

#[test]
fn small_exchange_is_closed_once_nothing_is_listed() {
    let last_listed_day = smfe(ct((2025, 3, 21), (12, 0, 0)));
    assert!(last_listed_day.is_open_regular(ct((2025, 3, 21), (12, 0, 0))));

    let closure = ct((2025, 3, 24), (0, 0, 0));
    assert_eq!(
        smfe(closure - chrono::Duration::seconds(1)),
        last_listed_day
    );
    let delisted = smfe(closure);
    assert!(delisted.regular.is_empty());
    assert!(delisted.order_entry.is_empty());
    assert_eq!(delisted, smfe(ct((2026, 8, 24), (12, 0, 0))));
}
