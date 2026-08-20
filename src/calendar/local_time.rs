// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! The single place a local wall-clock becomes an instant.
//!
//! Every `(local day, seconds-since-local-midnight)` pair in this crate is
//! resolved here, so DST policy is stated once rather than at each call site.
//! The policy is asymmetric on purpose: an **open** takes the earliest valid
//! mapping and a **close** takes the latest, which keeps a session maximally
//! inclusive while its close stays end-exclusive at the true boundary. A
//! wall-clock inside a spring-forward gap does not exist, so the resolver steps
//! forward minute by minute — bounded — to the first representable instant.
//!
//! Callers must resolve through [`mk_local_open`] / [`mk_local_close`] rather
//! than [`chrono::TimeZone::from_local_datetime`]: picking a bias ad hoc is how
//! fall-back hours produce sessions that appear to run backwards.

use chrono::{Datelike, Duration, LocalResult, NaiveDateTime, TimeZone};
use chrono_tz::Tz;

use super::{MarketHours, SessionKind, SessionRule};

/// Holiday hook. Always `false` today: the crate ships normal-week,
/// exchange-level defaults and deliberately owns no holiday calendar.
///
/// Every wrap-session and daily-close path already routes its
/// "may this session exist on this local date?" question through here, so
/// landing a real calendar is a body change, not a control-flow change.
pub(crate) fn is_holiday(_hours: &MarketHours, _d: chrono::NaiveDate) -> bool {
    false
}

// Unreferenced in-crate today; retained as the date-keyed companion to the
// weekday scans in `session`/`candle`, which is where a holiday calendar lands.
#[allow(
    dead_code,
    reason = "date-keyed rule lookup retained for the holiday-calendar hook; no in-crate caller yet"
)]
pub(crate) fn rule_for_date_in(
    hours: &MarketHours,
    d: chrono::NaiveDate,
    kind: SessionKind,
) -> Option<&SessionRule> {
    if is_holiday(hours, d) {
        return None;
    }
    let w = d.weekday().num_days_from_monday() as usize;
    hours.iter_rules(kind).find(|r| r.days[w])
}

#[derive(Clone, Copy)]
enum AmbigBias {
    Earliest,
    Latest,
}

/// Resolve a local wall-clock (day + SSM) into a concrete `DateTime<Tz>`,
/// choosing a deterministic mapping across DST transitions.
/// - Ambiguous: pick earliest/latest according to `bias`.
/// - Skipped (spring forward): pick the earliest valid instant *after* the gap.
fn mk_local_biased(
    tz: Tz,
    day: chrono::NaiveDate,
    ssm: u32,
    bias: AmbigBias,
) -> chrono::DateTime<Tz> {
    let base: NaiveDateTime = day.and_hms_opt(0, 0, 0).unwrap() + Duration::seconds(i64::from(ssm));
    match tz.from_local_datetime(&base) {
        LocalResult::Single(dt) => dt,
        LocalResult::Ambiguous(a, b) => match bias {
            AmbigBias::Earliest => a.min(b),
            AmbigBias::Latest => a.max(b),
        },
        LocalResult::None => {
            // Step forward until we land on a representable instant (bounded).
            // Typical DST gaps are 60 minutes; 180 provides ample headroom.
            let mut trial = base + Duration::minutes(1);
            for _ in 0..180 {
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
            // If still unresolved (extremely unlikely), assert—calendars should be well-formed.
            panic!("unresolvable local time after DST gap search");
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
