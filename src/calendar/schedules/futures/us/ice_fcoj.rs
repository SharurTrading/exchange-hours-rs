// SPDX-License-Identifier: MIT-0

//! ICE Futures U.S. FCOJ-A (frozen concentrated orange juice) futures and
//! options schedules.

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// FCOJ-A runs one same-day executable session, Monday 08:00 through Friday
// 14:00 New York. The ICE master hours table carries no footnote marker on the
// FCOJ-A row, so nothing commences on the previous calendar evening and there
// is no Sunday session.
// Narrative: docs/evidence/ice_us_orange_juice.md
pub(crate) static FCOJ_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 14 * 3600,
}];

// Two order-entry-only phases, neither of which matches: the post-close
// pre-open ("PCPO") 14:30-18:00 and the 20:00 pre-open running to the next
// morning's 08:00 open. The 20:00 start is modelled Monday-Thursday because a
// wrapping rule cannot carry a Friday evening across the weekend; the PCPO is
// Monday-Friday, an ordinary same-week window.
// Narrative: docs/evidence/ice_us_orange_juice.md
pub(crate) static FCOJ_EXTENDED_CURRENT: &[SessionRule] = &[];
pub(crate) static FCOJ_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 30 * 60,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 20 * 3600,
        close_ssm: 8 * 3600,
    },
];

pub(crate) static FCOJ_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: FCOJ_REGULAR_CURRENT,
    extended: FCOJ_EXTENDED_CURRENT,
    order_entry: FCOJ_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Baseline before 2018-10-08: the same executable session without the PCPO,
// leaving the 20:00 pre-open as the only non-executable phase. The grid is
// carried back unchanged; ICE's 2014 softs notice excludes FCOJ-A and no
// primary ICE document states a different open or close inside the window.
// Narrative: docs/evidence/ice_us_orange_juice.md
static FCOJ_ORDER_ENTRY_BASELINE: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 20 * 3600,
    close_ssm: 8 * 3600,
}];

pub(crate) static FCOJ_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: FCOJ_REGULAR_CURRENT,
    extended: &[],
    order_entry: FCOJ_ORDER_ENTRY_BASELINE,
    has_daily_close: true,
    has_weekend_close: true,
};

// The one revision row below is T1; its effective day and citation literal are
// the row's own fields, and the notice, quotation and URL behind it are in the
// evidence file.
// Evidence: docs/evidence/ice_us_orange_juice.md
pub(crate) static FCOJ_REVISIONS: &[Revision] =
    revisions![(2018, 10, 8, &FCOJ_CURRENT, "ICE PCPO notice 20180920"),];

/// Selects the FCOJ-A profile in force on `as_of`'s New York day.
pub(crate) fn fcoj_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &FCOJ_BASELINE,
        FCOJ_REVISIONS,
    )
}
