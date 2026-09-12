// SPDX-License-Identifier: MIT-0

//! Point-in-time selectors for US cash-equity schedules.

use chrono_tz::America;

use super::StaticHoursProfile;
use super::equities::{
    MEMX_EQ_PROFILE, MEMX_EQ_PROFILE_2020_10_05, MEMX_EQ_PROFILE_PRE_2025_05_19,
    MIAX_PEARL_EQ_PROFILE, MIAX_PEARL_EQ_PROFILE_PRE_2025_02_20, NASDAQ_BX_PROFILE,
    NASDAQ_BX_PROFILE_PRE_2011_04_18, NASDAQ_PROFILE, NASDAQ_PROFILE_PRE_2013_03_18,
    NASDAQ_PSX_PROFILE, NASDAQ_PSX_PROFILE_AT_LAUNCH,
};
use crate::calendar::schedules::CLOSED_NEW_YORK;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// 2013-03-18 — T1 — Nasdaq Equity Trader Alert 2013-21 — the pre-market open
//   moves from 07:00 to 04:00 ET.
// Evidence: docs/evidence/nasdaq.md
static NASDAQ_REVISIONS: &[Revision] = revisions![(
    2013,
    3,
    18,
    &NASDAQ_PROFILE,
    "Nasdaq Equity Trader Alert 2013-21"
),];

pub(crate) fn nasdaq_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &NASDAQ_PROFILE_PRE_2013_03_18,
        NASDAQ_REVISIONS,
    )
}

// 2011-04-18 — T1 — Nasdaq Equity Trader Alert 2011-20 — the system-hours open
//   moves from 08:00 to 07:00 ET; the 19:00 close is unchanged.
// Evidence: docs/evidence/nasdaq_bx.md
static NASDAQ_BX_REVISIONS: &[Revision] = revisions![(
    2011,
    4,
    18,
    &NASDAQ_BX_PROFILE,
    "Nasdaq Equity Trader Alert 2011-20"
),];

pub(crate) fn nasdaq_bx_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &NASDAQ_BX_PROFILE_PRE_2011_04_18,
        NASDAQ_BX_REVISIONS,
    )
}

// 2010-10-08 — T1 — Nasdaq Equity Trader Alert 2010-56 — PSX launches on
//   09:00–17:00 ET.
// 2010-12-13 — T1 — SEC SR-Phlx-2010-172 — the 08:00 ET opening.
// Evidence: docs/evidence/nasdaq_psx.md
static NASDAQ_PSX_REVISIONS: &[Revision] = revisions![
    (
        2010,
        10,
        8,
        &NASDAQ_PSX_PROFILE_AT_LAUNCH,
        "Nasdaq Equity Trader Alert 2010-56"
    ),
    (2010, 12, 13, &NASDAQ_PSX_PROFILE, "SEC SR-Phlx-2010-172"),
];

pub(crate) fn nasdaq_psx_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        NASDAQ_PSX_REVISIONS,
    )
}

// 2020-09-21 — T1 — MEMX Day 1 retrospective — live launch, 07:00–20:00 ET.
// 2020-10-05 — T1 — MEMX trader alert 20-06 — the post-market close moves from
//   20:00 to 17:00 ET.
// 2023-02-01 — T1 — MEMX trader alert 23-04 — the 20:00 close is restored.
// 2025-05-19 — T1 — MEMX retrospective 2025-06-06 — the 04:00 pre-market opens.
// Evidence: docs/evidence/memx_eq.md
static MEMX_REVISIONS: &[Revision] = revisions![
    (
        2020,
        9,
        21,
        &MEMX_EQ_PROFILE_PRE_2025_05_19,
        "MEMX Day 1 retrospective"
    ),
    (
        2020,
        10,
        5,
        &MEMX_EQ_PROFILE_2020_10_05,
        "MEMX trader alert 20-06"
    ),
    (
        2023,
        2,
        1,
        &MEMX_EQ_PROFILE_PRE_2025_05_19,
        "MEMX trader alert 23-04"
    ),
    (
        2025,
        5,
        19,
        &MEMX_EQ_PROFILE,
        "MEMX retrospective 2025-06-06"
    ),
];

pub(crate) fn memx_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        MEMX_REVISIONS,
    )
}

// 2020-09-29 — T1 — MIAX Pearl Equities launch notice — launch on Regular
//   Trading Hours only.
// 2025-02-20 — T1 — MIAX Pearl Regulatory Circular 2025-02 — the Early
//   (04:00–09:30) and Late (16:00–20:00) Trading Sessions become available.
// Evidence: docs/evidence/miax_pearl_eq.md
static MIAX_PEARL_REVISIONS: &[Revision] = revisions![
    (
        2020,
        9,
        29,
        &MIAX_PEARL_EQ_PROFILE_PRE_2025_02_20,
        "MIAX Pearl Equities launch notice"
    ),
    (
        2025,
        2,
        20,
        &MIAX_PEARL_EQ_PROFILE,
        "MIAX Pearl Regulatory Circular 2025-02"
    ),
];

pub(crate) fn miax_pearl_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        MIAX_PEARL_REVISIONS,
    )
}
