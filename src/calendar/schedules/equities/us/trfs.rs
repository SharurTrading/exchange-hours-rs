// SPDX-License-Identifier: MIT-0

//! FINRA Trade Reporting Facility schedules and revisions.

use chrono::{DateTime, Utc};
use chrono_tz::America;

use super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::CLOSED_NEW_YORK;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// FINRA Regulatory Notice 25-15 identifies all three active TRFs, states that
// their system hours changed from 08:00–20:00 to 04:00–20:00 ET on 2026-03-30,
// and classifies 09:30–16:00 as Regular Trading Hours. Rules 6380A and 6380B
// require the outside-RTH reports to carry the corresponding modifier.
// https://www.finra.org/filing-reporting/trade-reporting-facility-trf
// https://www.finra.org/rules-guidance/rulebooks/finra-rules/6380a
// https://www.finra.org/rules-guidance/rulebooks/finra-rules/6380b
// https://www.finra.org/rules-guidance/notices/25-15
static REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 16 * 3600,
}];

static EXTENDED_PRE_2026_03_30: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 20 * 3600,
    },
];

static EXTENDED_POST_2026_03_30: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 4 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 20 * 3600,
    },
];

static FINRA_TRF_CARTERET_PROFILE_PRE_2026_03_30: StaticHoursProfile =
    profile(EXTENDED_PRE_2026_03_30);
static FINRA_TRF_CHICAGO_PROFILE_PRE_2026_03_30: StaticHoursProfile =
    profile(EXTENDED_PRE_2026_03_30);
static FINRA_TRF_NYSE_PROFILE_PRE_2026_03_30: StaticHoursProfile = profile(EXTENDED_PRE_2026_03_30);

pub(crate) static FINRA_TRF_CARTERET_PROFILE: StaticHoursProfile =
    profile(EXTENDED_POST_2026_03_30);
pub(crate) static FINRA_TRF_CHICAGO_PROFILE: StaticHoursProfile = profile(EXTENDED_POST_2026_03_30);
pub(crate) static FINRA_TRF_NYSE_PROFILE: StaticHoursProfile = profile(EXTENDED_POST_2026_03_30);

// FINRA's later Sunday-through-Friday expansion remains monitored but is not
// selected while its anticipated implementation day is conditional on the SIP
// rollout. See the schedule update guide for the outstanding confirmation.
// https://www.finra.org/sites/default/files/2026-07/SR-FINRA-2026-015.pdf
static CARTERET_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2026, 3, 30),
    profile: &FINRA_TRF_CARTERET_PROFILE,
}];
static CHICAGO_REVISIONS: &[Revision] = &[
    Revision {
        // FINRA says the Chicago facility commenced operation on 2018-09-10.
        // It accepted test securities only through 2018-09-21; all NMS stocks
        // became reportable on 2018-09-24.
        // https://www.finra.org/filing-reporting/trf/technical-notices/reminder-finranasdaq-trf-chicago
        effective: effective_date(2018, 9, 10),
        profile: &FINRA_TRF_CHICAGO_PROFILE_PRE_2026_03_30,
    },
    Revision {
        effective: effective_date(2026, 3, 30),
        profile: &FINRA_TRF_CHICAGO_PROFILE,
    },
];
static NYSE_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2026, 3, 30),
    profile: &FINRA_TRF_NYSE_PROFILE,
}];

pub(crate) fn carteret_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &FINRA_TRF_CARTERET_PROFILE_PRE_2026_03_30,
        CARTERET_REVISIONS,
    )
}

pub(crate) fn chicago_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        CHICAGO_REVISIONS,
    )
}

pub(crate) fn nyse_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &FINRA_TRF_NYSE_PROFILE_PRE_2026_03_30,
        NYSE_REVISIONS,
    )
}

const fn profile(extended: &'static [SessionRule]) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: America::New_York,
        regular: REGULAR,
        extended,
        has_daily_close: true,
        has_weekend_close: true,
    }
}
