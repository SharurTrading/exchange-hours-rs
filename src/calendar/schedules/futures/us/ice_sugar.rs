// SPDX-License-Identifier: MIT-0

//! ICE Futures U.S. Sugar No. 11 (`SB`) futures and options schedules.

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// Sugar No. 11 runs one same-day executable session; the ICE master hours
// table carries no footnote marker on its row, so nothing commences on the
// previous calendar evening. The 20:00 pre-open is modelled Monday-Thursday
// because a wrapping rule cannot carry a Friday evening across the weekend.
// Narrative: docs/evidence/ice_us_sugar.md
pub(crate) static SUGAR_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3 * 3600 + 30 * 60,
    close_ssm: 13 * 3600,
}];

// Two order-entry-only phases, neither of which matches: the post-close
// pre-open ("PCPO") from 30 minutes after the 13:00 close, and the 20:00
// pre-open running to the next morning's open. Sugar No. 11 publishes no
// tradeable phase outside its executable session, so `extended` is empty.
// Narrative: docs/evidence/ice_us_sugar.md
pub(crate) static SUGAR_EXTENDED_CURRENT: &[SessionRule] = &[];
pub(crate) static SUGAR_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 30 * 60,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 20 * 3600,
        close_ssm: 3 * 3600 + 30 * 60,
    },
];

pub(crate) static SUGAR_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: SUGAR_REGULAR_CURRENT,
    extended: SUGAR_EXTENDED_CURRENT,
    order_entry: SUGAR_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2014-02-03 through 2018-10-05: the executable session is already today's
// 03:30-13:00 grid, but the PCPO order-entry window does not exist yet, leaving
// the 20:00 pre-open as the only non-executable phase. It is order entry for
// the same reason as the current one, so these eras carry an empty extended
// slice too.
static SUGAR_ORDER_ENTRY_2014: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 20 * 3600,
    close_ssm: 3 * 3600 + 30 * 60,
}];

static SUGAR_2014: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: SUGAR_REGULAR_CURRENT,
    extended: &[],
    order_entry: SUGAR_ORDER_ENTRY_2014,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2012-11-05 through 2014-01-31: open fixed at 02:30 NY year-round.
static SUGAR_REGULAR_2012_NOV: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 2 * 3600 + 30 * 60,
    close_ssm: 14 * 3600,
}];

static SUGAR_2012_NOV: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: SUGAR_REGULAR_2012_NOV,
    extended: &[],
    order_entry: SUGAR_ORDER_ENTRY_2014,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2012-01-30 through 2012-11-02: open moved to 01:30 NY. The same notice
// announced a standing summer adjustment to 02:30 NY, which ICE itself labelled
// a "temporary change to the opening time"; seasonal opening shifts are
// exceptional-day changes and are not modelled as normal-week revisions here.
static SUGAR_REGULAR_2012_JAN: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3600 + 30 * 60,
    close_ssm: 14 * 3600,
}];

static SUGAR_2012_JAN: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: SUGAR_REGULAR_2012_JAN,
    extended: &[],
    order_entry: SUGAR_ORDER_ENTRY_2014,
    has_daily_close: true,
    has_weekend_close: true,
};

// Baseline before 2012-01-30: ICE's AUGUST 2011 master hours table, carried
// back. No primary source gives a January 2010 close, and the 2010-2011
// interval cannot be sourced because ICE sets these hours administratively
// rather than by rule.
// Narrative: docs/evidence/ice_us_sugar.md
static SUGAR_REGULAR_BASELINE: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3 * 3600 + 30 * 60,
    close_ssm: 14 * 3600,
}];

pub(crate) static SUGAR_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: SUGAR_REGULAR_BASELINE,
    extended: &[],
    order_entry: SUGAR_ORDER_ENTRY_2014,
    has_daily_close: true,
    has_weekend_close: true,
};

// Every revision row below is T1; each row's effective day and citation
// literal are its own fields, and the notice, quotation and URL behind each
// are in the evidence file.
// Evidence: docs/evidence/ice_us_sugar.md
pub(crate) static SUGAR_REVISIONS: &[Revision] = revisions![
    (2012, 1, 30, &SUGAR_2012_JAN, "ICE ExNot 121911 S11 hours"),
    (2012, 11, 5, &SUGAR_2012_NOV, "ICE ExNot 1018912 S11 hours"),
    (2014, 2, 3, &SUGAR_2014, "ICE ExNot 012714 hours"),
    (2018, 10, 8, &SUGAR_CURRENT, "ICE PCPO notice 20180920"),
];

/// Selects the Sugar No. 11 profile in force on `as_of`'s New York day.
pub(crate) fn sugar_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &SUGAR_BASELINE,
        SUGAR_REVISIONS,
    )
}
