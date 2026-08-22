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

use chrono::{Duration, LocalResult, NaiveDateTime, NaiveTime, Offset, TimeZone};
use chrono_tz::Tz;

/// Preserves `t` whenever its local civil representation exists, moving only
/// an otherwise-unrepresentable chrono edge inward before zone conversion.
pub(crate) fn bounded_utc(
    t: chrono::DateTime<chrono::Utc>,
    tz: Tz,
) -> chrono::DateTime<chrono::Utc> {
    let offset_seconds = tz
        .offset_from_utc_datetime(&t.naive_utc())
        .fix()
        .local_minus_utc();
    if t.naive_utc()
        .checked_add_signed(Duration::seconds(i64::from(offset_seconds)))
        .is_some()
    {
        return t;
    }

    // `FixedOffset` is strictly inside +/-24 hours. One day therefore makes
    // the local representation valid while retaining the nearest
    // representable local date for forward/backward scans.
    let margin = Duration::days(1);
    if offset_seconds.is_positive() {
        t.checked_sub_signed(margin).unwrap_or(t)
    } else {
        t.checked_add_signed(margin).unwrap_or(t)
    }
}

/// Holiday hook for the normal-week model; always `false` until an exception
/// calendar is added. Query contexts own when this policy is consulted.
pub(crate) const fn is_holiday(_day: chrono::NaiveDate) -> bool {
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
    let base = day
        .and_time(NaiveTime::MIN)
        .checked_add_signed(Duration::seconds(i64::from(ssm)))
        .unwrap_or(NaiveDateTime::MAX);
    match tz.from_local_datetime(&base) {
        LocalResult::Single(dt) => dt,
        LocalResult::Ambiguous(a, b) => match bias {
            AmbigBias::Earliest => a.min(b),
            AmbigBias::Latest => a.max(b),
        },
        LocalResult::None => {
            // Step forward until we land on a representable instant (bounded).
            // 3_000 one-minute steps cover a 50-hour gap.
            let Some(mut trial) = base.checked_add_signed(Duration::minutes(1)) else {
                return tz.from_utc_datetime(&base);
            };
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
                        let Some(next) = trial.checked_add_signed(Duration::minutes(1)) else {
                            return tz.from_utc_datetime(&base);
                        };
                        trial = next;
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
