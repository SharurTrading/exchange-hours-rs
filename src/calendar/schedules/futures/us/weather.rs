// SPDX-License-Identifier: MIT-0

//! CME weather temperature-index futures schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// CME weather temperature-index FUTURES in America/Chicago, all listed with
// CME Globex tag 55-Symbol `HW` and quoted in Chicago time whatever city the
// index measures. One family, no sub-split, and no `regular` session in any
// era: weather futures have never been pit-eligible. Excludes options on
// weather futures and CME ClearPort; the evidence file says why for both.
// Narrative: docs/evidence/globex_weather.md

/// The executable CME Globex leg from the January-2010 floor until
/// SER-9519's cutover: Sunday and Monday-Thursday 17:00 CT, wrapping local
/// midnight to a 15:15 CT close, with the operator's 15:15-17:00 CT daily
/// trading halt in the gap.
static WEATHER_EXTENDED_AT_2010_FLOOR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 15 * 3600 + 15 * 60,
}];

/// The executable CME Globex leg SER-9519 established for 2025-04-13: the
/// same 17:00 CT opens, wrapping to a 16:00 CT close, with the daily halt
/// moved to 16:00-17:00 CT. Friday's close is therefore 16:00 CT and there is
/// no Friday-evening reopen.
pub(crate) static WEATHER_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];

/// The CME Globex Pre-Open queues SER-9519 tabulates as "(unchanged)" and
/// CME's live session service still publishes: Sunday 16:00-17:00 CT and
/// Monday-Thursday 16:45-17:00 CT. Orders queue; nothing matches until 17:00.
pub(crate) static WEATHER_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600,
    },
];

// DATED QUEUES: THE SOURCED INTERSECTION, CARRIED BACK, WITH NO CUTOVER
// ASSERTED. Two Sunday queue states are sourced and no primary source dates
// the move between them, so the dated profiles carry the window that holds
// under both — Sunday 16:15-17:00 — from the January-2010 floor, and only the
// disputed 16:00-16:15 quarter-hour waits on the knowledge-bound row below.
// Narrative: docs/evidence/globex_weather.md
static WEATHER_ORDER_ENTRY_DATED: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600,
    },
];

const fn profile(
    extended: &'static [SessionRule],
    order_entry: &'static [SessionRule],
) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: US::Central,
        regular: &[],
        extended,
        order_entry,
        has_daily_close: true,
        has_weekend_close: true,
    }
}

static AT_2010_FLOOR: StaticHoursProfile =
    profile(WEATHER_EXTENDED_AT_2010_FLOOR, WEATHER_ORDER_ENTRY_DATED);
static FROM_2025_04_13: StaticHoursProfile =
    profile(WEATHER_EXTENDED_CURRENT, WEATHER_ORDER_ENTRY_DATED);
/// The knowledge-bound current state: SER-9519's matching grid plus both
/// queues CME publishes today, including the Sunday quarter-hour whose onset
/// day no source states. `profiles.rs` builds the fixed-current
/// `FuturesSessionProfile` from the same two rule tables.
static DATED_CURRENT: StaticHoursProfile =
    profile(WEATHER_EXTENDED_CURRENT, WEATHER_ORDER_ENTRY_CURRENT);

// Revision evidence — the day-level effective date and the primary source that
// states it, plus the knowledge-bound row that carries the undated Sunday
// quarter-hour:
//   2025-04-13 "CME SER-9519"
//     https://www.cmegroup.com/content/dam/cmegroup/notices/ser/2025/03/ser-9519.pdf
// Evidence: docs/evidence/globex_weather.md
static REVISIONS: &[Revision] = revisions![
    (2025, 4, 13, &FROM_2025_04_13, "CME SER-9519"),
    // Knowledge-bound row, dated at the repository review that verified this
    // family's phases. It adds only the 16:00-16:15 CT Sunday quarter-hour,
    // whose onset day no source states: it makes no onset claim, its date
    // never moves forward, and a sourced onset day replaces it.
    // Narrative: docs/evidence/globex_weather.md
    (
        2026,
        9,
        5,
        &DATED_CURRENT,
        "2026-09-05 review: verified current, onset undated"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &AT_2010_FLOOR, REVISIONS)
}
