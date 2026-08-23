// SPDX-License-Identifier: MIT-0

//! Agreement between the fixed snapshot and the dated calendar.
//!
//! `hours_for_market_hours_key` returns the current sourced grid.
//! `hours_for_market_hours_key_as_of` reselects from the dated timeline. For
//! most families the two agree at every instant. For a handful they do not, and
//! that divergence is deliberate: where a phase is primary-sourced today but its
//! onset day cannot be dated, the dated selector omits the phase rather than
//! asserting a cutover the record does not support. See the `omits`/`omitted`
//! notes on the corresponding rows of `docs/schedules/verification.md`.
//!
//! Deliberate or not, an unasserted divergence is indistinguishable from an
//! accidental one. These tests enumerate the divergent set with its reason, so a
//! *new* divergence fails the build, and pin the direction so the dated surface
//! can never report a market open that the fixed surface calls closed.

use chrono::{DateTime, TimeZone as _, Utc};
use exchange_hours::{
    MarketHoursKey, hours_for_market_hours_key, hours_for_market_hours_key_as_of,
};

/// Families whose dated selector deliberately omits an undatable phase, with
/// the phase each one drops. Every entry must actually diverge; a stale entry
/// fails just as loudly as an unexpected one.
const DOCUMENTED_DIVERGENCE: &[(MarketHoursKey, &str)] = &[
    (
        MarketHoursKey::GlobexEquityIndex,
        "Sunday queue omitted: its 16:15->16:00 CT cutover day is unknown",
    ),
    (
        MarketHoursKey::GlobexEnergy,
        "queues omitted from history: unconditional onset chain incomplete",
    ),
    (
        MarketHoursKey::GlobexGrains,
        "post-2012 afternoon/Sunday queue and PCP onset days unresolved",
    ),
    (
        MarketHoursKey::GlobexFx,
        "Sunday pre-open omitted: its 16:15->16:00 CT cutover day is unavailable",
    ),
    (
        MarketHoursKey::GlobexInterestRates,
        "Sunday queue omitted after 2011: cutover day unavailable",
    ),
    (
        MarketHoursKey::GlobexLivestock,
        "PCP 14:30-16:00 omitted: unconditional onset chain unavailable",
    ),
];

/// A Monday-to-Sunday week in June 2026, sampled every five minutes.
fn normal_week_samples() -> impl Iterator<Item = DateTime<Utc>> {
    (15..=21u32).flat_map(|day| {
        (0..24u32).flat_map(move |hour| {
            (0..60u32).step_by(5).filter_map(move |minute| {
                Utc.with_ymd_and_hms(2026, 6, day, hour, minute, 0).single()
            })
        })
    })
}

/// Minutes per week where the two surfaces disagree, and whether the dated
/// surface was ever the more permissive of the two.
fn divergence(key: MarketHoursKey) -> (u32, bool) {
    let fixed = hours_for_market_hours_key(key);
    let mut minutes = 0;
    let mut dated_ever_wider = false;
    for instant in normal_week_samples() {
        let dated = hours_for_market_hours_key_as_of(key, instant);
        let (fixed_open, dated_open) = (fixed.is_open(instant), dated.is_open(instant));
        if fixed_open != dated_open {
            minutes += 5;
            if dated_open {
                dated_ever_wider = true;
            }
        }
    }
    (minutes, dated_ever_wider)
}

#[test]
fn only_documented_families_diverge_between_the_two_surfaces() {
    let documented: Vec<MarketHoursKey> =
        DOCUMENTED_DIVERGENCE.iter().map(|&(key, _)| key).collect();

    for key in MarketHoursKey::ALL {
        let (minutes, _) = divergence(*key);
        let is_documented = documented.contains(key);

        assert!(
            minutes == 0 || is_documented,
            "{} diverges by {minutes} min/week between hours_for_market_hours_key and \
             hours_for_market_hours_key_as_of, but no documented reason is recorded. \
             Either the dated timeline's head no longer matches the current snapshot, \
             or this is a deliberate omission that must be added to DOCUMENTED_DIVERGENCE \
             with its verification.md reason.",
            key.as_str()
        );
    }
}

#[test]
fn every_documented_divergence_is_still_real() {
    for &(key, reason) in DOCUMENTED_DIVERGENCE {
        let (minutes, _) = divergence(key);
        assert!(
            minutes > 0,
            "{} is listed as deliberately divergent ({reason}) but the two surfaces now \
             agree. If the onset was sourced and encoded, remove the entry.",
            key.as_str()
        );
    }
}

#[test]
fn the_dated_surface_is_never_more_permissive_than_the_fixed_one() {
    // Containment matters more than agreement. A consumer scanning with the
    // dated calendar may see fewer open minutes than the fixed snapshot, which
    // is conservative. The reverse would mean the dated surface reports a market
    // open that the current sourced grid says is closed.
    for key in MarketHoursKey::ALL {
        let (_, dated_ever_wider) = divergence(*key);
        assert!(
            !dated_ever_wider,
            "{}: the dated selector reported open where the current snapshot reports \
             closed. Divergence must only ever drop phases, never add them.",
            key.as_str()
        );
    }
}
