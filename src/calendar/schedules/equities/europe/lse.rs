// SPDX-License-Identifier: MIT-0

//! London Stock Exchange (SETS). Evidence: `docs/evidence/lse.md`.

use crate::calendar::time_zones::{Europe, SelectionInstant};

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

static BASE_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30,
    close_ssm: 16 * 3600 + 30 * 60,
}];
static BASE_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 50 * 60,
        close_ssm: 8 * 3600 + 30,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 35 * 60 + 30,
    },
];
static ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600,
    close_ssm: 7 * 3600 + 50 * 60,
}];
static BASE_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::London,
    regular: BASE_REGULAR,
    extended: BASE_EXTENDED,
    order_entry: ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

static CPX_EXTENDED: &[SessionRule] = &[
    BASE_EXTENDED[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 35 * 60 + 30,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 35 * 60 + 30,
        close_ssm: 16 * 3600 + 40 * 60,
    },
];
static CPX_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::London,
    regular: BASE_REGULAR,
    extended: CPX_EXTENDED,
    order_entry: ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

static MIDDAY_AUCTION_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30,
        close_ssm: 12 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600 + 2 * 60 + 30,
        close_ssm: 16 * 3600 + 30 * 60,
    },
];
static MIDDAY_AUCTION_EXTENDED: &[SessionRule] = &[
    BASE_EXTENDED[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600,
        close_ssm: 12 * 3600 + 2 * 60 + 30,
    },
    CPX_EXTENDED[1],
    CPX_EXTENDED[2],
];

static MIDDAY_AUCTION_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::London,
    regular: MIDDAY_AUCTION_REGULAR,
    extended: MIDDAY_AUCTION_EXTENDED,
    order_entry: ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// Evidence: docs/evidence/lse.md
static REVISIONS: &[Revision] = revisions![
    (2012, 4, 30, &CPX_PROFILE, "LSE MIT201 document history"),
    (2016, 3, 21, &MIDDAY_AUCTION_PROFILE, "LSE notice N01/16"),
    (2020, 9, 21, &CPX_PROFILE, "LSE notice N15/20"),
];

pub(crate) fn profile_at(as_of: SelectionInstant<'_>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, Europe::London), &BASE_PROFILE, REVISIONS)
}
