// SPDX-License-Identifier: MIT-0

//! Deterministic randomized and pinned transition properties.

use super::prelude::*;
use super::property_support::*;
use crate::support::zones::UTC;
use crate::support::{PROFILE_INSTANT, local_opt};

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn calendar_queries_are_total_and_consistent_over_random_instants() {
    const ITERATIONS: usize = 256;
    for seed in SEEDS {
        let mut state = seed;
        for iter in 0..ITERATIONS {
            let exch = PROPERTY_VENUES[bounded(&mut state, PROPERTY_VENUES.len())];
            let hours = hours_for_exchange(exch, PROFILE_INSTANT);
            let instant = random_instant(&mut state);
            let label = format!(
                "[total+consistency invariant] seed={seed:#018x} iter={iter}/{ITERATIONS} \
                 venues={} resolutions={} kinds={}",
                PROPERTY_VENUES.len(),
                RESOLUTIONS.len(),
                KINDS.len()
            );
            assert_instant_invariants(exch, &hours, instant, &label);

            // The point-in-time surface must satisfy the same invariants with
            // the profile the venue actually published at the sampled instant
            // — the ~14-year window crosses every recorded cutover, including
            // the pre-go-live no-session profiles.
            let historical = hours_for_exchange(exch, instant);
            let historical_label = format!("{label} [as-of profile]");
            assert_instant_invariants(exch, &historical, instant, &historical_label);
        }
    }
}

#[test]
fn next_session_after_walk_strictly_advances_opens() {
    const STEPS: usize = 160;
    const WALK_VENUES: [Exchange; 4] = [
        Exchange::Cme,
        Exchange::Eurex,
        Exchange::BinanceFutures,
        Exchange::Unknown,
    ];
    for seed in SEEDS {
        let mut state = seed;
        for exch in WALK_VENUES {
            let hours = hours_for_exchange(exch, PROFILE_INSTANT);
            let mut cursor = random_instant(&mut state);
            for step in 0..STEPS {
                let (open, close) = next_session_after(&hours, cursor)
                    .expect("walk venues run sessions every week");
                let ctx = format!(
                    "[strictly-advancing session walk] seed={seed:#018x} venue={exch:?} \
                     step={step}/{STEPS} cursor={cursor} open={open} close={close}"
                );
                // Advancing by the previous open guarantees strict progress: the
                // next session must open strictly after the cursor, so a stall
                // or backward move fails here.
                assert!(
                    open > cursor,
                    "session walk stalled or moved backward ({ctx})"
                );
                assert!(
                    close > open,
                    "session must have positive width during walk ({ctx})"
                );
                cursor = open;
            }
        }
    }
}

#[test]
fn always_open_venues_stay_open_without_maintenance() {
    const ITERATIONS: usize = 128;
    const ALWAYS_OPEN: [Exchange; 2] = [Exchange::BinanceFutures, Exchange::Unknown];
    for seed in SEEDS {
        let mut state = seed;
        for exch in ALWAYS_OPEN {
            let hours = hours_for_exchange(exch, PROFILE_INSTANT);
            for iter in 0..ITERATIONS {
                let instant = random_instant(&mut state);
                let ctx = format!(
                    "[always-open invariant] seed={seed:#018x} venue={exch:?} \
                     iter={iter}/{ITERATIONS} instant={instant}"
                );
                assert!(
                    hours.is_open(instant),
                    "always-open venue reported closed ({ctx})"
                );
                assert!(
                    !hours.is_maintenance(instant),
                    "always-open venue reported maintenance ({ctx})"
                );
                assert!(
                    !hours.is_closed_all_day_on(
                        instant.to_zoned_datetime(hours.tz.clone()).date(),
                        SessionKind::Both,
                    ),
                    "always-open venue reported closed all day ({ctx})"
                );
            }
        }
    }
}

#[test]
fn dst_transition_queries_are_stable_and_total() {
    let hours = hours_for_exchange(Exchange::Cme, PROFILE_INSTANT);
    // UTC instants bracketing the US 2025 spring-forward (Mar 9) and fall-back
    // (Nov 2) transitions, where local wall-clock mapping is a gap or a fold.
    let instants = [
        local_opt(&UTC, (2025, 3, 9), (6, 0, 0)).expect("valid UTC instant"),
        local_opt(&UTC, (2025, 3, 9), (7, 0, 0)).expect("valid UTC instant"),
        local_opt(&UTC, (2025, 3, 9), (8, 0, 0)).expect("valid UTC instant"),
        local_opt(&UTC, (2025, 11, 2), (5, 0, 0)).expect("valid UTC instant"),
        local_opt(&UTC, (2025, 11, 2), (6, 0, 0)).expect("valid UTC instant"),
        local_opt(&UTC, (2025, 11, 2), (7, 0, 0)).expect("valid UTC instant"),
    ];
    for instant in instants {
        assert_instant_invariants(
            Exchange::Cme,
            &hours,
            instant,
            "[pinned DST stability fixture]",
        );
    }
}
