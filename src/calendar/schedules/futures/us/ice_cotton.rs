// SPDX-License-Identifier: MIT-0

//! ICE Futures U.S. Cotton No. 2 (`CT`) futures schedules.

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// Cotton No. 2 is a wrapping contract: the session for trade date D commences
// at 21:00 NY on calendar day D-1 and closes at 14:20 NY on D. No ICE primary
// document names Sunday for Cotton, so the Sunday evening open and its
// pre-open are omitted and the documented weekend boundary is Friday 14:20 to
// Monday 19:30 NY.
// Narrative: docs/evidence/ice_us_cotton.md
pub(crate) static COTTON_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 21 * 3600,
    close_ssm: 14 * 3600 + 20 * 60,
}];

// Two order-entry-only phases, neither of them executable: the 19:30-21:00
// Pre-Open, confined to Limit orders by Rule 4.22(a) with the Opening Match at
// the open, and the 14:50-18:00 Post-Close Pre-Open on the prior Exchange
// business day. Cotton publishes no tradeable phase outside its executable
// session, so `extended` is empty.
// Narrative: docs/evidence/ice_us_cotton.md
pub(crate) static COTTON_EXTENDED_CURRENT: &[SessionRule] = &[];
pub(crate) static COTTON_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 50 * 60,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 19 * 3600 + 30 * 60,
        close_ssm: 21 * 3600,
    },
];

pub(crate) static COTTON_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: COTTON_REGULAR_CURRENT,
    extended: COTTON_EXTENDED_CURRENT,
    order_entry: COTTON_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2014-02-03 through 2018-10-05: the executable grid is already today's
// 21:00 - 14:20, but the PCPO order-entry window does not exist yet. The
// September 2018 notice describes the PCPO as an addition to the existing
// pre-open order entry session, so the 19:30 - 21:00 Pre-Open is the whole of
// the non-executable grid in this regime - and, being order entry under Rule
// 4.22(a), it sits in order_entry, leaving these eras with no extended phase.
static COTTON_ORDER_ENTRY_2014: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 19 * 3600 + 30 * 60,
    close_ssm: 21 * 3600,
}];

static COTTON_2014: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: COTTON_REGULAR_CURRENT,
    extended: &[],
    order_entry: COTTON_ORDER_ENTRY_2014,
    has_daily_close: true,
    has_weekend_close: true,
};

// Baseline before 2014-02-03: the current grid with the older 14:30 close. The
// January 2014 notice moved only the close and left the 21:00 open on the
// previous day untouched, and no primary ICE document states an earlier Cotton
// pre-open time, so the 19:30-21:00 window is carried back unchanged.
// Narrative: docs/evidence/ice_us_cotton.md
static COTTON_REGULAR_BASELINE: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 21 * 3600,
    close_ssm: 14 * 3600 + 30 * 60,
}];

pub(crate) static COTTON_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: COTTON_REGULAR_BASELINE,
    extended: &[],
    order_entry: COTTON_ORDER_ENTRY_2014,
    has_daily_close: true,
    has_weekend_close: true,
};

// Both revision rows are T1; each row's effective day and citation literal are
// its own fields, and the notice, quotation and URL behind each are in the
// evidence file.
// Evidence: docs/evidence/ice_us_cotton.md
pub(crate) static COTTON_REVISIONS: &[Revision] = revisions![
    (2014, 2, 3, &COTTON_2014, "ICE ExNot 012714 hours"),
    (2018, 10, 8, &COTTON_CURRENT, "ICE PCPO notice 20180920"),
];

/// Selects the Cotton No. 2 profile in force on `as_of`'s New York day.
pub(crate) fn cotton_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &COTTON_BASELINE,
        COTTON_REVISIONS,
    )
}
