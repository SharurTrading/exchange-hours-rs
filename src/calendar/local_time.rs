// SPDX-License-Identifier: MIT-0

//! The single place a local wall-clock becomes an instant.
//!
//! Every `(local day, seconds-since-local-midnight)` pair in this crate is
//! resolved here, so DST policy is stated once rather than at each call site.
//! The policy is asymmetric on purpose: an **open** takes the earliest valid
//! mapping and a **close** takes the latest, which keeps a session maximally
//! inclusive while its close stays end-exclusive at the true boundary. A
//! wall-clock inside a spring-forward gap does not exist, so the resolver steps
//! forward minute by minute — bounded — to the first representable instant.
//! The resolver is **total**: it never panics, for any time zone or input.
//!
//! Callers must resolve through [`mk_local_open`] / [`mk_local_close`] rather
//! than [`chrono::TimeZone::from_local_datetime`]: picking a bias ad hoc is how
//! fall-back hours produce sessions that appear to run backwards.

use chrono::{Duration, LocalResult, NaiveDateTime, NaiveTime, TimeZone};
use chrono_tz::Tz;

use super::MarketHours;

/// Holiday hook. Always `false` today: the crate ships normal-week,
/// exchange-level defaults and deliberately owns no holiday calendar.
///
/// Every query path routes its "may this session exist on this local date?"
/// question through here, so landing a real calendar is a body change, not a
/// control-flow change. The session-existence contract every caller
/// implements: a **same-day** session on local day `D` exists iff `D` is not
/// a holiday, and a **wrap** session opening on `D` and closing on `D+1`
/// exists iff neither `D` nor `D+1` is a holiday. `is_open_with`,
/// `session_bounds_with`, `next_session_after_with`, and the daily-close
/// finder all apply these gates identically — the cross-query fence in
/// `tests/contract/session_invariants.rs` is what keeps them from drifting.
pub(crate) fn is_holiday(_hours: &MarketHours, _d: chrono::NaiveDate) -> bool {
    false
}

#[derive(Clone, Copy)]
enum AmbigBias {
    Earliest,
    Latest,
}

/// Resolve a local wall-clock (day + SSM) into a concrete `DateTime<Tz>`,
/// choosing a deterministic mapping across DST transitions.
/// - Ambiguous: pick earliest/latest according to `bias`.
/// - Skipped (spring forward): pick the earliest valid instant *after* the gap,
///   at minute granularity.
///
/// Total by construction: the gap search covers 50 hours — the largest gaps in
/// IANA history are the 24-hour date-line skips (Pacific/Apia 2011,
/// Pacific/Kiritimati 1994), and ordinary DST gaps are one hour — and if a
/// hypothetical zone exhausted even that, the wall-clock is deterministically
/// reinterpreted as UTC rather than panicking.
fn mk_local_biased(
    tz: Tz,
    day: chrono::NaiveDate,
    ssm: u32,
    bias: AmbigBias,
) -> chrono::DateTime<Tz> {
    let base: NaiveDateTime = day.and_time(NaiveTime::MIN) + Duration::seconds(i64::from(ssm));
    match tz.from_local_datetime(&base) {
        LocalResult::Single(dt) => dt,
        LocalResult::Ambiguous(a, b) => match bias {
            AmbigBias::Earliest => a.min(b),
            AmbigBias::Latest => a.max(b),
        },
        LocalResult::None => {
            // Step forward until we land on a representable instant (bounded).
            // 3_000 one-minute steps cover a 50-hour gap.
            let mut trial = base + Duration::minutes(1);
            for _ in 0..3_000 {
                match tz.from_local_datetime(&trial) {
                    LocalResult::Single(dt) => return dt,
                    LocalResult::Ambiguous(a, b) => {
                        return match bias {
                            AmbigBias::Earliest => a.min(b),
                            AmbigBias::Latest => a.max(b),
                        };
                    }
                    LocalResult::None => {
                        trial += Duration::minutes(1);
                    }
                }
            }
            // Unreachable for real time-zone data. Reinterpreting the
            // wall-clock as UTC keeps the resolver total and deterministic
            // without inventing a nearby local instant.
            tz.from_utc_datetime(&base)
        }
    }
}

/// Resolves a session **open**: on an ambiguous fall-back hour the earliest of
/// the two valid instants wins, so the session starts as early as it legitimately can.
#[inline]
pub(crate) fn mk_local_open(tz: Tz, day: chrono::NaiveDate, ssm: u32) -> chrono::DateTime<Tz> {
    mk_local_biased(tz, day, ssm, AmbigBias::Earliest)
}

/// Resolves a session **close**: on an ambiguous fall-back hour the latest of
/// the two valid instants wins, so the end-exclusive close lands on the true
/// boundary rather than an hour early.
#[inline]
pub(crate) fn mk_local_close(tz: Tz, day: chrono::NaiveDate, ssm: u32) -> chrono::DateTime<Tz> {
    mk_local_biased(tz, day, ssm, AmbigBias::Latest)
}
