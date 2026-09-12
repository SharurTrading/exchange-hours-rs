// SPDX-License-Identifier: MIT-0

//! NYSE-family US cash-equity profiles and dated schedule history.

use chrono_tz::America;

use super::StaticHoursProfile;
use super::equities::{US_EQUITY_EXTENDED, equity_profile, equity_profile_with_entry};
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::CLOSED_NEW_YORK;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// Pillar order-entry edges. Nothing can print inside these windows: they exist
// so orders can be entered, amended and cancelled ahead of the first matching
// session, so they are `order_entry`, not `extended`.
// See docs/evidence/nyse.md and docs/evidence/nyse_arca.md.
static ENTRY_0630_0700: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 6 * 3600 + 30 * 60,
    close_ssm: 7 * 3600,
}];

static ENTRY_0230_0400: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 2 * 3600 + 30 * 60,
    close_ssm: 4 * 3600,
}];

/// NYSE Arca's pre-2021 acceptance edge: 30 minutes before the 04:00 open.
static ENTRY_0330_0400: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3 * 3600 + 30 * 60,
    close_ssm: 4 * 3600,
}];

/// NYSE's executable Early Trading Session, 07:00–09:30 (Tapes B and C).
static EXTENDED_0700_0930: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600,
    close_ssm: 9 * 3600 + 30 * 60,
}];

static EXTENDED_0700_2000: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 20 * 3600,
    },
];

static NATIONAL_0800_1830: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 18 * 3600 + 30 * 60,
    },
];

static NATIONAL_0800_2000: &[SessionRule] = &[
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

static NATIONAL_0800_1700: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
];

static NYSE_CHICAGO_EXTENDED_PRE_PILLAR: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
];

// The 06:30–07:00 acceptance leg matches nothing and is `order_entry`;
// 07:00–09:30 stays `extended` because Tape B/C trades execute there. Before
// 2018-04-09 the modelled grid stays the 09:30–16:00 core session.
// See docs/evidence/nyse.md.
static NYSE_HISTORICAL_PROFILE: StaticHoursProfile = equity_profile(&[]);
pub(crate) static NYSE_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_0930, ENTRY_0630_0700);

// 2018-04-09 — T1 — SEC 34-83230 (NYSE UTP Pillar production) — UTP securities
//   begin trading on Pillar.
// Evidence: docs/evidence/nyse.md
static NYSE_REVISIONS: &[Revision] = revisions![(
    2018,
    4,
    9,
    &NYSE_PROFILE,
    "SEC 34-83230 (NYSE UTP Pillar production)"
),];

pub(crate) fn nyse_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &NYSE_HISTORICAL_PROFILE,
        NYSE_REVISIONS,
    )
}

// NYSE Arca's 02:30 queue matches nothing, so it is `order_entry`; the
// 04:00–20:00 execution grid stays `extended` and predates the audit floor.
// The 30-minute acceptance edge is rulebook text on both sides of the
// January-2010 floor, so 03:30 is carried back with no revision row asserted.
// See docs/evidence/nyse_arca.md.
static NYSE_ARCA_PRE_2021_09_13: StaticHoursProfile =
    equity_profile_with_entry(US_EQUITY_EXTENDED, ENTRY_0330_0400);
pub(crate) static NYSE_ARCA_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(US_EQUITY_EXTENDED, ENTRY_0230_0400);

// 2021-09-13 — T1 — NYSE Trader Update 2021-08-11 — order entry moves from
//   03:30 to 02:30 ET.
// Evidence: docs/evidence/nyse_arca.md
static NYSE_ARCA_REVISIONS: &[Revision] = revisions![(
    2021,
    9,
    13,
    &NYSE_ARCA_PROFILE,
    "NYSE Trader Update 2021-08-11"
),];

pub(crate) fn nyse_arca_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &NYSE_ARCA_PRE_2021_09_13,
        NYSE_ARCA_REVISIONS,
    )
}

// NYSE American's 06:30–07:00 acceptance edge is `order_entry` and the
// 07:00–20:00 execution grid is `extended`. Before Pillar the sourced
// continuous session was 09:30–16:00; legacy off-hours crosses are not
// backfilled. See docs/evidence/nyse_american.md.
static NYSE_AMERICAN_PRE_PILLAR: StaticHoursProfile = equity_profile(&[]);
pub(crate) static NYSE_AMERICAN_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_2000, ENTRY_0630_0700);

// 2017-07-24 — T1 — NYSE American Pillar update 2017-07-21 — the cash-equity
//   market transitions to Pillar.
// Evidence: docs/evidence/nyse_american.md
static AMERICAN_REVISIONS: &[Revision] = revisions![(
    2017,
    7,
    24,
    &NYSE_AMERICAN_PROFILE,
    "NYSE American Pillar update 2017-07-21"
),];

pub(crate) fn nyse_american_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &NYSE_AMERICAN_PRE_PILLAR,
        AMERICAN_REVISIONS,
    )
}

static NATIONAL_PRE_2010_08_02: StaticHoursProfile = equity_profile(NATIONAL_0800_1830);
static NATIONAL_2010_08_02: StaticHoursProfile = equity_profile(NATIONAL_0800_2000);
static NATIONAL_2015_12_22: StaticHoursProfile = equity_profile(NATIONAL_0800_1700);
pub(crate) static NYSE_NATIONAL_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_2000, ENTRY_0630_0700);

// The 06:30–07:00 acceptance edge is `order_entry` because no trade prints
// before the 07:00 Early Trading Session. The pre-2018 NSX grid is rulebook
// text (NSX Rule 11.1) and no NSX order-acceptance phase is modelled.
// See docs/evidence/nyse_national.md.

// 2010-08-02 — T1 — SEC 34-62643 — the post-RTH close extends to 20:00 ET.
// 2014-05-16 — T1 — SEC 34-72215 — the post-RTH close shortens to 17:00 ET.
// 2014-05-31 — T1 — SEC 34-72107 — trading ceases after the 2014-05-30 close.
// 2015-12-22 — T1 — NSX SEC Form 1 relaunch filing — the phased relaunch.
// 2017-02-01 — T1 — SEC 34-80018 — trading ceases again before the open.
// 2018-05-21 — T1 — SR-NYSENat-2020-05 — the NYSE National Pillar launch.
// Evidence: docs/evidence/nyse_national.md
static NATIONAL_REVISIONS: &[Revision] = revisions![
    (2010, 8, 2, &NATIONAL_2010_08_02, "SEC 34-62643"),
    (2014, 5, 16, &NATIONAL_2015_12_22, "SEC 34-72215"),
    (2014, 5, 31, &CLOSED_NEW_YORK, "SEC 34-72107"),
    (
        2015,
        12,
        22,
        &NATIONAL_2015_12_22,
        "NSX SEC Form 1 relaunch filing"
    ),
    (2017, 2, 1, &CLOSED_NEW_YORK, "SEC 34-80018"),
    (2018, 5, 21, &NYSE_NATIONAL_PROFILE, "SR-NYSENat-2020-05"),
];

pub(crate) fn nyse_national_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &NATIONAL_PRE_2010_08_02,
        NATIONAL_REVISIONS,
    )
}

// NYSE Texas is the same registered exchange formerly called NYSE Chicago and
// CHX; the 2025 conversion and rename were non-substantive. The 06:30–07:00
// edge is `order_entry`; the pre-Pillar CHX slice stays wholly `extended`
// because its early, late and cross-only late crossing sessions all print
// trades. See docs/evidence/nyse_texas.md.
pub(crate) static NYSE_TEXAS_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_2000, ENTRY_0630_0700);
static NYSE_CHICAGO_PROFILE_PRE_PILLAR: StaticHoursProfile =
    equity_profile(NYSE_CHICAGO_EXTENDED_PRE_PILLAR);

// 2019-11-04 — T1 — NYSE Chicago migration notice — the Pillar migration
//   establishes the 06:30 edge around the 07:00–20:00 grid.
// Evidence: docs/evidence/nyse_texas.md
static TEXAS_REVISIONS: &[Revision] = revisions![(
    2019,
    11,
    4,
    &NYSE_TEXAS_PROFILE,
    "NYSE Chicago migration notice"
),];

pub(crate) fn nyse_texas_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &NYSE_CHICAGO_PROFILE_PRE_PILLAR,
        TEXAS_REVISIONS,
    )
}
