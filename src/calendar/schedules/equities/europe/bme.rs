// SPDX-License-Identifier: MIT-0

//! BME cash equities.

use chrono_tz::Europe;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// Evidence: docs/evidence/bme.md
static BME_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30,
    close_ssm: 17 * 3600 + 30 * 60,
}];
static BME_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 9 * 3600 + 30,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 35 * 60,
        close_ssm: 17 * 3600 + 45 * 60,
    },
];
static BME_EXTENDED_PRE_TAL: &[SessionRule] = &[
    BME_EXTENDED_CURRENT[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 35 * 60,
        close_ssm: 17 * 3600 + 35 * 60 + 30,
    },
];

static BME_ORDER_ENTRY: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 35 * 60,
    },
];

pub(crate) static BME_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Madrid,
    regular: BME_REGULAR,
    extended: BME_EXTENDED_CURRENT,
    order_entry: BME_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};
static BME_PROFILE_PRE_2023_12_04: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Madrid,
    regular: BME_REGULAR,
    extended: BME_EXTENDED_PRE_TAL,
    order_entry: BME_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// 2023-12-04 — T1 — BME Operating Instruction 47/2023 — Trading-at-Last launch.
// Evidence: docs/evidence/bme.md
static REVISIONS: &[Revision] = revisions![(
    2023,
    12,
    4,
    &BME_PROFILE,
    "BME Operating Instruction 47/2023"
),];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Europe::Madrid),
        &BME_PROFILE_PRE_2023_12_04,
        REVISIONS,
    )
}
