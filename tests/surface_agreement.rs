// SPDX-License-Identifier: MIT-0

//! Agreement between the fixed snapshot and the dated calendar.
//!
//! `hours_for_market_hours_key` at the knowledge-bound instant returns the
//! current sourced grid; at a sampled instant it reselects from the dated
//! timeline. For most families the two agree at every instant. For a handful
//! they do not, and
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
use exchange_hours::{MarketHoursKey, hours_for_market_hours_key};

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
        MarketHoursKey::GlobexFx,
        "Sunday pre-open omitted: its 16:15->16:00 CT cutover day is unavailable",
    ),
    (
        MarketHoursKey::GlobexInterestRates,
        "Sunday queue omitted after 2011: cutover day unavailable",
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

/// Minutes per week where the two surfaces disagree on `predicate`, and whether
/// the dated surface was ever the more permissive of the two.
fn divergence(
    key: MarketHoursKey,
    predicate: fn(&exchange_hours::MarketHours, DateTime<Utc>) -> bool,
) -> (u32, bool) {
    let fixed = hours_for_market_hours_key(
        key,
        chrono::DateTime::<chrono::Utc>::UNIX_EPOCH + chrono::Duration::seconds(1_787_400_000),
    );
    let mut minutes = 0;
    let mut dated_ever_wider = false;
    for instant in normal_week_samples() {
        let dated = hours_for_market_hours_key(key, instant);
        let (fixed_yes, dated_yes) = (predicate(&fixed, instant), predicate(&dated, instant));
        if fixed_yes != dated_yes {
            minutes += 5;
            if dated_yes {
                dated_ever_wider = true;
            }
        }
    }
    (minutes, dated_ever_wider)
}

fn is_open(hours: &exchange_hours::MarketHours, t: DateTime<Utc>) -> bool {
    hours.is_open(t)
}

fn is_accepting_orders(hours: &exchange_hours::MarketHours, t: DateTime<Utc>) -> bool {
    hours.is_accepting_orders(t)
}

#[test]
fn the_two_surfaces_now_agree_on_whether_a_trade_can_print() {
    // Before the order-entry phase existed, six futures families disagreed here
    // by up to 1110 min/week. Every one of those divergences was a pre-open
    // queue sitting in `extended`; with queues moved to `order_entry`, `is_open`
    // means "a trade can print" and the two surfaces agree everywhere.
    for key in MarketHoursKey::ALL {
        let (minutes, _) = divergence(*key, is_open);
        assert_eq!(
            minutes,
            0,
            "{}: the fixed snapshot and the dated calendar disagree on is_open by \
             {minutes} min/week. Tradeable sessions must not diverge between surfaces.",
            key.as_str()
        );
    }
}

#[test]
fn only_documented_families_diverge_on_order_acceptance() {
    let documented: Vec<MarketHoursKey> =
        DOCUMENTED_DIVERGENCE.iter().map(|&(key, _)| key).collect();

    for key in MarketHoursKey::ALL {
        let (minutes, _) = divergence(*key, is_accepting_orders);
        assert!(
            minutes == 0 || documented.contains(key),
            "{} diverges by {minutes} min/week on order acceptance with no documented \
             reason. Either the dated timeline's head no longer matches the current \
             snapshot, or this is a deliberate omission that must be added to \
             DOCUMENTED_DIVERGENCE with its verification.md reason.",
            key.as_str()
        );
    }
}

#[test]
fn every_documented_divergence_is_still_real() {
    for &(key, reason) in DOCUMENTED_DIVERGENCE {
        let (minutes, _) = divergence(key, is_accepting_orders);
        assert!(
            minutes > 0,
            "{} is listed as deliberately divergent ({reason}) but the two surfaces now \
             agree on order acceptance. If the onset was sourced and encoded, remove it.",
            key.as_str()
        );
    }
}

#[test]
fn the_dated_surface_is_never_more_permissive_than_the_fixed_one() {
    // Containment, on both predicates. The dated calendar may report fewer
    // minutes than the current snapshot; reporting more would mean telling a
    // consumer a market is open, or an order workable, when the current sourced
    // grid says otherwise.
    for key in MarketHoursKey::ALL {
        for predicate in [
            is_open as fn(&exchange_hours::MarketHours, DateTime<Utc>) -> bool,
            is_accepting_orders,
        ] {
            let (_, dated_ever_wider) = divergence(*key, predicate);
            assert!(
                !dated_ever_wider,
                "{}: the dated selector was more permissive than the current snapshot. \
                 Divergence must only ever drop phases, never add them.",
                key.as_str()
            );
        }
    }
}
