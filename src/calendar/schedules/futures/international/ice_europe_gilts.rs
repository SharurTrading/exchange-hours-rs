// SPDX-License-Identifier: MIT-0

//! ICE Futures Europe gilt futures family (Long, Medium, Short and Ultra
//! Long Gilt), London.

use chrono_tz::Europe;

use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// The shared gilt grid through 2026-07-05: the operator's own Long Gilt
// product page, captured 2026-03-11 and 2026-05-09, prints LONDON trading
// "8:00 AM - 6:00 PM" (08:00 - 18:00) with Pre-Open 06:03. The pre-open
// accepts orders and matches nothing until the 08:00 open, so it is
// `order_entry`. Short and Ultra Long Gilt print the same grid, and the
// December-2010 LIFFE notice attachment opens the gilt rows at 08:00 on its

// half days, so the family serves one clock here.
// Narrative: docs/evidence/ice_europe_gilts.md
static GILTS_0800_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 18 * 3600,
}];
static GILTS_0800_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 6 * 3600 + 3 * 60,
    close_ssm: 8 * 3600,
}];
static GILTS_0800: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::London,
    regular: GILTS_0800_REGULAR,
    extended: &[],
    order_entry: GILTS_0800_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// From 2026-07-06 the Long Gilt and Medium Gilt clock: circular 26/063 of
// 23 April 2026 states "Effective from Monday 06 July 2026 the revised daily
// trading hours for the Contracts will be as follows: Long Gilt Futures
// 00:45 | 01:00 | 21:00" (Pre Open, Open, Close, UK Time), and the operator's
// October-2026 temporary-trading-times document reprints 01:00-21:00 for

// "Medium and Long Gilt Futures" while Short and Ultra Long Gilt stay
// 08:00-18:00. This key therefore narrows to the Long and Medium scope on
// that day: the operator's live product page prints Pre-Open 00:35, so the
// order-entry window holds the circular's 00:45 — the bound every source
// states — and the disputed 00:35-00:45 quarter-hour is disclosed in the
// evidence file. Short and Ultra Long Gilt are outside this key's scope from

// the cutover: their unchanged grid is a different clock, not this one.
// Narrative: docs/evidence/ice_europe_gilts.md
pub(crate) static ICE_EUROPE_GILTS_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3600,
    close_ssm: 21 * 3600,
}];
pub(crate) static ICE_EUROPE_GILTS_ORDER_ENTRY_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 45 * 60,
    close_ssm: 3600,
}];
pub(crate) static ICE_EUROPE_GILTS_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::London,
    regular: ICE_EUROPE_GILTS_REGULAR_CURRENT,
    extended: &[],
    order_entry: ICE_EUROPE_GILTS_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Evidence: docs/evidence/ice_europe_gilts.md
static ICE_EUROPE_GILTS_REVISIONS: &[Revision] = revisions![(
    2026,
    7,
    6,
    &ICE_EUROPE_GILTS_CURRENT,
    "ICE Futures Europe circular 26/063"
)];

/// Selects the gilt-family profile in force at `as_of` (London local day).
pub(crate) fn ice_europe_gilts_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Europe::London),
        &GILTS_0800,
        ICE_EUROPE_GILTS_REVISIONS,
    )
}
