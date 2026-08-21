// SPDX-License-Identifier: MIT-0

//! Cross-query consistency fences for every current venue profile.

use super::identity_expectations::*;
use super::prelude::*;
use super::probe_support::*;

// ---------------------------------------------------------------------------
// Cross-query consistency fence
//
// `is_open_with` and `session_bounds_with` are two views of one fact: whether a
// session covers an instant. They now share the same private lookup, and this
// public-surface contract prevents a future adapter or engine change from
// separating them again. Before that consolidation, a wrap rule enabled on a
// weekday contributed
// its *close* side that day as well as its open side, which reported venues
// whose wrap closes late in the day (COMEX/NYMEX 17:00→16:00 CT, ICE US
// FANG+ Sunday 18:00→18:00 ET) open all Sunday morning while `session_bounds` correctly
// returned the Sunday-evening session. `is_open` said yes, the bounds said no.
//
// The invariant below is the fence: for every venue, every session kind, and
// every instant in the grid, `is_open_with(t, kind)` must equal
// "`session_bounds_with(hours, t, kind)` contains t". `None` from a venue with
// no rules contains nothing, which is the correct `false`.
// ---------------------------------------------------------------------------

#[test]
fn is_open_agrees_with_session_bounds_for_every_venue_and_instant() {
    let kinds = [
        SessionKind::Regular,
        SessionKind::Extended,
        SessionKind::Both,
    ];
    let mut checked = 0_u32;

    for &exchange in ALL_EXCHANGES {
        let hours = hours_for_exchange(exchange);
        for instant in probe_instants(&hours) {
            for kind in kinds {
                let bounds = session_bounds_with(&hours, instant, kind);
                let contained =
                    bounds.is_some_and(|(open, close)| open <= instant && instant < close);
                assert_eq!(
                    hours.is_open_with(instant, kind),
                    contained,
                    "{exchange:?} / {kind:?}: is_open_with disagrees with \
                     session_bounds_with at {instant} (bounds {bounds:?}, \
                     venue-local {})",
                    instant.with_timezone(&hours.tz),
                );
                checked += 1;
            }
        }
    }

    assert!(
        checked > 50_000,
        "grid collapsed to {checked} checks; the probe set is no longer covering the venues"
    );
}

#[test]
fn closed_instants_agree_with_the_next_session_for_every_venue() {
    // The other half of the contract: when a venue is closed, the bounds it
    // reports are the *next* session, which must be exactly what
    // `next_session_after_with` reports and must not start in the past.
    for &exchange in ALL_EXCHANGES {
        let hours = hours_for_exchange(exchange);
        for instant in probe_instants(&hours) {
            for kind in [
                SessionKind::Regular,
                SessionKind::Extended,
                SessionKind::Both,
            ] {
                if hours.is_open_with(instant, kind) {
                    continue;
                }
                let bounds = session_bounds_with(&hours, instant, kind);
                let next = next_session_after_with(&hours, instant, kind);
                assert_eq!(
                    bounds.map(|(open, _)| open),
                    next.map(|(open, _)| open),
                    "{exchange:?} / {kind:?}: closed at {instant} but session_bounds_with \
                     and next_session_after_with disagree on the next open"
                );
                if let Some((next_open, _)) = next {
                    assert!(
                        next_open >= instant,
                        "{exchange:?} / {kind:?}: next session at {next_open} precedes {instant}"
                    );
                }
            }
        }
    }
}
