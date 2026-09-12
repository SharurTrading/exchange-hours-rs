// SPDX-License-Identifier: MIT-0

//! Coinbase Derivatives recurring 23x5 futures default.
//!
//! Narrative evidence, sources and residual risks:
//! `docs/evidence/coinbase_derivatives.md` (LAW-EVIDENCE-FILES).

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_ONLY, MON_THU, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{
    Revision, effective_date, local_date, revisions, select_revision,
};

// The venue default is CDE's recurring 23x5 futures grid: Sunday through Friday,
// 17:00-16:00 CT, with the daily 16:00-17:00 break. It is sourced unchanged from
// the 2021 launch certifications through filing #2026-24. The 24x7 family most
// CDE futures joined since #2026-24 is out of scope and needs its own key.
static REGULAR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];
// ORDER ENTRY, NOT TRADING. Pre-Open quoting accepts orders and nothing matches
// until the 17:00 open. No source dates its onset, so the queue enters only at
// the knowledge-bound row below and the dated profiles carry no Pre-Open.
static ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 16 * 3600 + 50 * 60,
    close_ssm: 17 * 3600,
}];

// The dated grid from launch: the sourced trading session, no Pre-Open.
static DATED: StaticHoursProfile = StaticHoursProfile {
    tz: America::Chicago,
    regular: REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
/// The verified-current grid: the dated grid plus the 16:50 Pre-Open.
static CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: America::Chicago,
    regular: REGULAR,
    extended: &[],
    order_entry: ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// FairX, as CDE then traded, opened on Monday 2021-06-28 at 09:00 ET with no
// Sunday-evening session before it, so the launch-day profile starts that first
// session at the launch instant rather than at a Sunday 17:00 that did not
// trade. Its Monday-evening session is identical to the full grid's, so the
// day-level switch never splits a running session.
static LAUNCH_DAY_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: MON_ONLY,
        open_ssm: 8 * 3600,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 16 * 3600,
    },
];
static LAUNCH_DAY: StaticHoursProfile = StaticHoursProfile {
    tz: America::Chicago,
    regular: LAUNCH_DAY_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: America::Chicago,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// 2021-06-28 13:00:00 UTC, 08:00 CDT. An exact instant is required; this
// launch is not a venue-local-midnight revision.
const LAUNCH_UNIX_SECONDS: i64 = 1_624_885_200;
const FIRST_FULL_DAY: chrono::NaiveDate = effective_date(2021, 6, 29);

// Knowledge-bound row, dated at the UTC date of the review that verified the
// 16:50 Pre-Open (LAW-UTC-DATES). It adds only that queue, makes no onset claim,
// never moves forward, and a sourced onset day replaces it.
// Evidence: docs/evidence/coinbase_derivatives.md
static REVISIONS: &[Revision] = revisions![
    // 2026-09-11 — T1 — 2026-09-11 review: verified current, onset undated —
    // adds the 16:50-17:00 CT Pre-Open queue to the sourced 23x5 grid.
    (
        2026,
        9,
        11,
        &CURRENT,
        "2026-09-11 review: verified current, onset undated"
    )
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, America::Chicago);
    if as_of.timestamp() < LAUNCH_UNIX_SECONDS {
        &CLOSED
    } else if day < FIRST_FULL_DAY {
        &LAUNCH_DAY
    } else {
        select_revision(day, &DATED, REVISIONS)
    }
}
