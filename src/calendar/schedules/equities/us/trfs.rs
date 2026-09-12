// SPDX-License-Identifier: MIT-0

//! FINRA Trade Reporting Facility schedules and revisions.

use chrono::{DateTime, Utc};
use chrono_tz::America;

use super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::CLOSED_NEW_YORK;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// A TRF is a reporting facility, not a matching engine and not an order book:
// it never accepts orders, so `order_entry` is empty because the concept does
// not apply here. Every window below is one in which an executed trade is
// reported and disseminated, so a print does occur inside it and all of them
// stay in `extended`. See docs/evidence/finra_trf_carteret.md.
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

// 2026-03-30 — T1 — FINRA Notice 25-15 — system hours move from 08:00–20:00 to
//   04:00–20:00 ET.
// Evidence: docs/evidence/finra_trf_carteret.md
static CARTERET_REVISIONS: &[Revision] = revisions![(
    2026,
    3,
    30,
    &FINRA_TRF_CARTERET_PROFILE,
    "FINRA Notice 25-15"
),];

// 2018-09-10 — T1 — FINRA/Nasdaq TRF Chicago technical notice — the facility
//   commences operation on 08:00–20:00 ET.
// 2026-03-30 — T1 — FINRA Notice 25-15 — system hours move to 04:00–20:00 ET.
// Evidence: docs/evidence/finra_trf_chicago.md
static CHICAGO_REVISIONS: &[Revision] = revisions![
    (
        2018,
        9,
        10,
        &FINRA_TRF_CHICAGO_PROFILE_PRE_2026_03_30,
        "FINRA/Nasdaq TRF Chicago technical notice",
    ),
    (
        2026,
        3,
        30,
        &FINRA_TRF_CHICAGO_PROFILE,
        "FINRA Notice 25-15"
    ),
];

// 2026-03-30 — T1 — FINRA Notice 25-15 — system hours move from 08:00–20:00 to
//   04:00–20:00 ET.
// Evidence: docs/evidence/finra_trf_nyse.md
static NYSE_REVISIONS: &[Revision] =
    revisions![(2026, 3, 30, &FINRA_TRF_NYSE_PROFILE, "FINRA Notice 25-15"),];

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
        order_entry: &[],
        has_daily_close: true,
        has_weekend_close: true,
    }
}
