// SPDX-License-Identifier: MIT-0

//! Cross-query and exact-boundary fences for historical venue profiles.

use super::historical_expectations::*;
use super::prelude::*;
use super::probe_support::*;

// ---------------------------------------------------------------------------
// Point-in-time fence: the as-of surface
//
// `hours_for_exchange_as_of` selects between whole profiles, so the grid fence
// above (which sweeps `hours_for_exchange` only) never exercised the
// historical tables. Two properties close that gap: every historical profile
// must hold the same is_open/session_bounds agreement, and the cutover
// boundary itself must flip at the venue-local midnight of the effective date
// — never an hour early or late through a UTC comparison.
// ---------------------------------------------------------------------------

#[test]
fn historical_profiles_hold_the_cross_query_fence() {
    let kinds = [
        SessionKind::Regular,
        SessionKind::Extended,
        SessionKind::Both,
    ];
    let mut checked = 0_u32;

    for &(exchange, (year, month, day), tz) in HISTORICAL_CUTOVERS {
        let at = tz
            .with_ymd_and_hms(year, month, day, 0, 0, 0)
            .earliest()
            .expect("cutover midnight resolves in the venue zone")
            .with_timezone(&Utc);
        for epoch in [at - Duration::seconds(1), at] {
            let hours = hours_for_exchange_as_of(exchange, epoch);
            for instant in probe_instants(&hours) {
                for kind in kinds {
                    let bounds = session_bounds_with(&hours, instant, kind);
                    let contained =
                        bounds.is_some_and(|(open, close)| open <= instant && instant < close);
                    assert_eq!(
                        hours.is_open_with(instant, kind),
                        contained,
                        "{exchange:?} as of {epoch} / {kind:?}: is_open_with disagrees with \
                         session_bounds_with at {instant} (bounds {bounds:?}, \
                         venue-local {})",
                        instant.with_timezone(&hours.tz),
                    );
                    checked += 1;
                }
            }
        }
    }

    // Two sides of every cutover x hundreds of probe instants x three kinds.
    // The floor guards against the probe set silently collapsing, not an exact
    // count.
    assert!(
        checked > 50_000,
        "as-of grid collapsed to {checked} checks; the probe set no longer covers the regimes"
    );
}

#[test]
fn cutovers_flip_exactly_at_the_venue_local_midnight() {
    // The new profile applies from venue-local midnight of the effective date:
    // one second before still sees the old hours.
    for &(exchange, (year, month, day), tz) in HISTORICAL_CUTOVERS {
        let local_midnight = tz
            .with_ymd_and_hms(year, month, day, 0, 0, 0)
            .earliest()
            .expect("cutover midnight resolves in the venue zone");
        let at = local_midnight.with_timezone(&Utc);
        let just_before = at - Duration::seconds(1);

        let hours_before = hours_for_exchange_as_of(exchange, just_before);
        let hours_at = hours_for_exchange_as_of(exchange, at);

        // All three phases participate: several sourced cutovers changed only a
        // pre-open queue, which now lives in `order_entry` rather than
        // `extended`, and comparing two phases would miss them.
        assert_ne!(
            (
                hours_before.regular.clone(),
                hours_before.extended.clone(),
                hours_before.order_entry.clone(),
            ),
            (
                hours_at.regular.clone(),
                hours_at.extended.clone(),
                hours_at.order_entry.clone(),
            ),
            "{exchange:?}: the {year}-{month:02}-{day:02} cutover did not change the profile \
             at venue-local midnight"
        );
        // One second after the boundary must already be stable on the new side.
        assert_eq!(
            hours_at,
            hours_for_exchange_as_of(exchange, at + Duration::seconds(1)),
            "{exchange:?}: profile unstable immediately after the cutover instant"
        );
    }
}

#[test]
fn intraday_cutovers_flip_at_the_exact_utc_instant() {
    for &(exchange, (year, month, day, hour, minute, second)) in HISTORICAL_INSTANT_CUTOVERS {
        let at = Utc
            .with_ymd_and_hms(year, month, day, hour, minute, second)
            .single()
            .expect("sourced UTC cutover resolves");
        let just_before = at - Duration::nanoseconds(1);
        let hours_before = hours_for_exchange_as_of(exchange, just_before);
        let hours_at = hours_for_exchange_as_of(exchange, at);

        assert_ne!(
            (hours_before.regular.clone(), hours_before.extended.clone()),
            (hours_at.regular.clone(), hours_at.extended.clone()),
            "{exchange:?}: exact UTC cutover did not change the profile"
        );
        assert_eq!(
            hours_at,
            hours_for_exchange_as_of(exchange, at + Duration::nanoseconds(1)),
            "{exchange:?}: profile unstable immediately after exact UTC cutover"
        );
    }
}
