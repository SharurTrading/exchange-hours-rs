// SPDX-License-Identifier: MIT-0

//! ICE Futures U.S. Cotton No. 2 (`CT`) futures schedules.
//! See the [evidence](../../../../../docs/evidence/ice_us_cotton.md).

use crate::calendar::time_zones::{America, SelectionInstant};

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

pub(crate) static COTTON_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 21 * 3600,
    close_ssm: 14 * 3600 + 20 * 60,
}];

// Cotton has one matching session per trade date and no separate extended phase.
pub(crate) static COTTON_EXTENDED_CURRENT: &[SessionRule] = &[];
pub(crate) static COTTON_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 50 * 60,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: SUN_PLUS_MON_THU,
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

static COTTON_ORDER_ENTRY_2014: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
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

static COTTON_REGULAR_BASELINE: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
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

// Evidence: docs/evidence/ice_us_cotton.md
pub(crate) static COTTON_REVISIONS: &[Revision] = revisions![
    // 2014-02-02 — T1 — ICE ExNot 012714 hours — first session ending at 14:20 NY.
    (2014, 2, 2, &COTTON_2014, "ICE ExNot 012714 hours"),
    // 2018-10-05 — T1 — ICE PCPO notice 20180920 — first Friday post-close queue.
    (2018, 10, 5, &COTTON_CURRENT, "ICE PCPO notice 20180920"),
];

/// Selects the Cotton No. 2 profile in force on `as_of`'s New York day.
pub(crate) fn cotton_profile_at(as_of: SelectionInstant<'_>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &COTTON_BASELINE,
        COTTON_REVISIONS,
    )
}
