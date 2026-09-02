// SPDX-License-Identifier: MIT-0

//! Normal-week open duration for fixed and date-aware schedules.

use chrono::{DateTime, Datelike, Duration, Timelike, Utc};

use super::schedule::{QueryContext, RuleSet, find_occurrence, rules};
use crate::calendar::hours::MarketHours;
use crate::calendar::local_time::bounded_utc;
use crate::calendar::rule::{SECONDS_PER_NORMAL_WEEK, SessionKind, normal_week_rule_intervals};

const WEEK_SECONDS: i64 = SECONDS_PER_NORMAL_WEEK.cast_signed();
const DAY_SECONDS: i64 = 86_400;

fn union_seconds(mut intervals: Vec<(u64, u64)>) -> u64 {
    intervals.sort_unstable();
    let mut total = 0_u64;
    let mut merged: Option<(u64, u64)> = None;
    for (start, end) in intervals {
        match merged {
            Some((current_start, current_end)) if start <= current_end => {
                merged = Some((current_start, current_end.max(end)));
            }
            Some((current_start, current_end)) => {
                total = total.saturating_add(current_end.saturating_sub(current_start));
                merged = Some((start, end));
            }
            None => merged = Some((start, end)),
        }
    }
    if let Some((start, end)) = merged {
        total = total.saturating_add(end.saturating_sub(start));
    }
    total.min(SECONDS_PER_NORMAL_WEEK)
}

pub(in crate::calendar) fn fixed_normal_week_open_seconds(hours: &MarketHours) -> u64 {
    let intervals = rules(hours, RuleSet::Sessions(SessionKind::Both))
        .flat_map(normal_week_rule_intervals)
        .collect();
    union_seconds(intervals)
}

pub(in crate::calendar) fn normal_week_open_seconds_containing(
    context: &QueryContext<'_>,
    instant: DateTime<Utc>,
) -> u64 {
    let tz = context.tz();
    let local_day = bounded_utc(instant, tz).with_timezone(&tz).date_naive();
    let weekday = i64::from(local_day.weekday().num_days_from_monday());
    let Some(monday) = local_day.checked_sub_signed(Duration::days(weekday)) else {
        let selected = context.profile_for_open_day(local_day);
        return fixed_normal_week_open_seconds(selected.as_ref());
    };
    let mut intervals = Vec::new();

    for offset in -1_i64..=6 {
        let Some(open_day) = monday.checked_add_signed(Duration::days(offset)) else {
            continue;
        };
        // The probe never stops the scan: it collects every occurrence, normal
        // or caller-supplied, that opens on this local day.
        let _: Option<()> = find_occurrence(
            context,
            open_day,
            RuleSet::Sessions(SessionKind::Both),
            false,
            |resolved_open, resolved_close| {
                let local_open = resolved_open.with_timezone(&tz);
                let local_close = resolved_close.with_timezone(&tz);
                let start = local_open
                    .date_naive()
                    .signed_duration_since(monday)
                    .num_days()
                    .saturating_mul(DAY_SECONDS)
                    .saturating_add(i64::from(local_open.time().num_seconds_from_midnight()));
                let end = local_close
                    .date_naive()
                    .signed_duration_since(monday)
                    .num_days()
                    .saturating_mul(DAY_SECONDS)
                    .saturating_add(i64::from(local_close.time().num_seconds_from_midnight()));
                let clipped_start = start.max(0);
                let clipped_end = end.min(WEEK_SECONDS);
                if clipped_start < clipped_end
                    && let (Ok(start), Ok(end)) =
                        (u64::try_from(clipped_start), u64::try_from(clipped_end))
                {
                    intervals.push((start, end));
                }
                None
            },
        );
    }
    union_seconds(intervals)
}
