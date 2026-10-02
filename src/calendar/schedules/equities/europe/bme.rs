// SPDX-License-Identifier: MIT-0

//! BME cash equities.

use chrono_tz::Europe;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// BME cash equities: opening auction 08:30-09:00, continuous trading 09:00-17:30, closing auction 17:30-17:35, and Trading-at-Last to 17:45. Narrative:
// docs/evidence/bme.md.
static BME_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30,
    close_ssm: 17 * 3600 + 30 * 60,
}];
static BME_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30 * 60,
        close_ssm: 9 * 3600 + 30,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 35 * 60 + 30,
    },
    // Trading-at-Last begins after the latest possible auction uncross.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 35 * 60 + 30,
        close_ssm: 17 * 3600 + 45 * 60,
    },
];
static BME_EXTENDED_PRE_TAL: &[SessionRule] = &[
    BME_EXTENDED_CURRENT[0],
    // Before TAL, the closing auction's 30-second random period was the final
    // executable phase of the normal day.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 35 * 60 + 30,
    },
];

pub(crate) static BME_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Madrid,
    regular: BME_REGULAR,
    extended: BME_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
static BME_PROFILE_PRE_2023_12_04: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Madrid,
    regular: BME_REGULAR,
    extended: BME_EXTENDED_PRE_TAL,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// Circular 1/2023 introduced the ten-minute Trading-at-Last phase for general
// trading. Operating Instruction 47/2023 records its day-level 2023-12-04
// entry into force and expressly excludes only the separate Fixing system.
// https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/circular/2023/circular-1-23-english.pdf
// https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2023/oi-47-2023-application-of-tal-phase-for-fixing-instruments.pdf
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
