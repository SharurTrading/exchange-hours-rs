// SPDX-License-Identifier: MIT-0

//! Public-surface instant grids shared by current and historical fences.

use super::prelude::*;

/// Builds the UTC instant for `ssm` seconds after local midnight on `day` in
/// `tz`, biased earliest across a fall-back hour. Returns `None` for a
/// wall-clock that does not exist (spring-forward gap) or for the end-of-day
/// sentinel `86_400`, neither of which is a usable grid sample.
fn local_sample(tz: chrono_tz::Tz, day: NaiveDate, ssm: u32) -> Option<DateTime<Utc>> {
    if ssm >= 86_400 {
        return None;
    }
    tz.with_ymd_and_hms(
        day.year(),
        day.month(),
        day.day(),
        ssm / 3600,
        (ssm % 3600) / 60,
        ssm % 60,
    )
    .earliest()
    .map(|local| local.with_timezone(&Utc))
}

/// The instants each venue is probed at.
///
/// Three overlapping layers, because each catches a different failure shape:
/// an hourly sweep of a full reference week (weekends, overnight sessions, and
/// maintenance gaps); the exact open and close instant of every rule on every
/// weekday it is enabled, plus one second either side (off-by-one at a boundary,
/// and the wrap open/close sides); and hourly sweeps around the 2026 US, EU,
/// Sydney, and Auckland DST transition dates.
pub(super) fn probe_instants(hours: &MarketHours) -> Vec<DateTime<Utc>> {
    // Sunday 2026-04-19 through Monday 2026-04-27, the reference week the
    // per-venue suite pins.
    let week_start = Utc
        .with_ymd_and_hms(2026, 4, 19, 0, 0, 0)
        .single()
        .expect("valid reference week start");

    let mut instants: Vec<DateTime<Utc>> = (0..8 * 24)
        .map(|hour| week_start + Duration::hours(hour))
        .collect();

    // Rule-derived boundaries across the reference week.
    let first_day = week_start.with_timezone(&hours.tz).date_naive();
    for offset in 0..9 {
        let Some(day) = first_day.checked_add_signed(Duration::days(offset)) else {
            continue;
        };
        for rule in hours.regular.iter().chain(hours.extended.iter()) {
            for ssm in [rule.open_ssm, rule.close_ssm] {
                // The end-exclusive `close_ssm == 86_400` sentinel is local
                // midnight of the next day; probe it there so full-day rules
                // get their close boundary +/-1s like every other rule.
                let (boundary_day, boundary_ssm) = if ssm == 86_400 {
                    let Some(next_day) = day.checked_add_signed(Duration::days(1)) else {
                        continue;
                    };
                    (next_day, 0)
                } else {
                    (day, ssm)
                };
                let Some(boundary) = local_sample(hours.tz, boundary_day, boundary_ssm) else {
                    continue;
                };
                instants.push(boundary - Duration::seconds(1));
                instants.push(boundary);
                instants.push(boundary + Duration::seconds(1));
            }
        }
    }

    // DST transitions: US spring-forward / fall-back and the EU equivalents,
    // which land on different dates. Swept hourly around their UTC dates.
    for (year, month, day) in [(2026, 3, 8), (2026, 11, 1), (2026, 3, 29), (2026, 10, 25)] {
        let Some(midnight) = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).single() else {
            continue;
        };
        for hour in -6..30 {
            instants.push(midnight + Duration::hours(hour));
        }
    }

    // Sydney and Auckland transitions occur on the preceding UTC date, so a
    // UTC-midnight anchor misses them. April 5 is shared; the other local
    // transition dates differ. Resolve these anchors in each venue's zone.
    for (year, month, day) in [(2026, 4, 5), (2026, 9, 27), (2026, 10, 4)] {
        let Some(midnight) = hours
            .tz
            .with_ymd_and_hms(year, month, day, 0, 0, 0)
            .single()
            .map(|local| local.with_timezone(&Utc))
        else {
            continue;
        };
        for hour in -6..30 {
            instants.push(midnight + Duration::hours(hour));
        }
    }

    instants
}
