// SPDX-License-Identifier: MIT-0

//! Cboe US cash-equity profiles and dated schedule history.

use chrono_tz::America;

use super::StaticHoursProfile;
use super::equities::{US_EQUITY_EXTENDED, equity_profile, equity_profile_with_entry};
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::CLOSED_NEW_YORK;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// BZX's January-2010 baseline and BYX's launch grid: 08:00–17:00 ET.
// See docs/evidence/cboe_bzx.md and docs/evidence/cboe_byx.md.
static EXTENDED_0800_1700: &[SessionRule] = &[
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

// Cboe's hours table names these windows "Early Order Acceptance": orders are
// accepted, amended and cancelled but nothing matches until the trading
// session opens, so they are `order_entry` rather than `extended`.
// See docs/evidence/cboe_bzx.md.
static ENTRY_0230_0400: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 2 * 3600 + 30 * 60,
    close_ssm: 4 * 3600,
}];

static ENTRY_0330_0400: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3 * 3600 + 30 * 60,
    close_ssm: 4 * 3600,
}];

static ENTRY_0600_0700: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 6 * 3600,
    close_ssm: 7 * 3600,
}];

// Direct Edge's launch-era acceptance window: orders accepted from 07:00,
// nothing matching until the 08:00 Pre-Market Session.
// See docs/evidence/cboe_edga.md and docs/evidence/cboe_edgx.md.
static ENTRY_0700_0800: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600,
    close_ssm: 8 * 3600,
}];

// From each exchange's 06:00 queue onset through its 2016 matching change,
// orders were accepted from 06:00 but nothing matched until 08:00, so the
// whole window is `order_entry`.
// See docs/evidence/cboe_edga.md and docs/evidence/cboe_edgx.md.
static ENTRY_0600_0800: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 6 * 3600,
    close_ssm: 8 * 3600,
}];

static EXTENDED_0700_1700: &[SessionRule] = &[
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

static EXTENDED_0800_2000: &[SessionRule] = &[
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

// Venue profile tables. See docs/evidence/cboe_bzx.md,
// docs/evidence/cboe_byx.md, docs/evidence/cboe_edga.md and
// docs/evidence/cboe_edgx.md.
static BZX_0800_1700: StaticHoursProfile = equity_profile(EXTENDED_0800_1700);
static BZX_QUEUE_2014: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0800_1700, ENTRY_0600_0800);
static BZX_0600_1700: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_1700, ENTRY_0600_0700);
static BZX_0600_2000: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_2000, ENTRY_0600_0700);
pub(crate) static CBOE_BZX_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(US_EQUITY_EXTENDED, ENTRY_0230_0400);

static BYX_0800_1700: StaticHoursProfile = equity_profile(EXTENDED_0800_1700);
static BYX_QUEUE_2014: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0800_1700, ENTRY_0600_0800);
static BYX_0600_1700: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_1700, ENTRY_0600_0700);
pub(crate) static CBOE_BYX_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_2000, ENTRY_0600_0700);

static EDGA_LAUNCH_2010: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0800_2000, ENTRY_0700_0800);
static EDGA_QUEUE_2014: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0800_2000, ENTRY_0600_0800);
pub(crate) static CBOE_EDGA_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_2000, ENTRY_0600_0700);

static EDGX_LAUNCH_2010: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0800_2000, ENTRY_0700_0800);
static EDGX_QUEUE_2014: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0800_2000, ENTRY_0600_0800);
static EDGX_0600_2000: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_2000, ENTRY_0600_0700);
static EDGX_0330_2000: StaticHoursProfile =
    equity_profile_with_entry(US_EQUITY_EXTENDED, ENTRY_0330_0400);
pub(crate) static CBOE_EDGX_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(US_EQUITY_EXTENDED, ENTRY_0230_0400);

// 2014-12-02 — T1 — SEC 34-73745 — the 06:00 order-acceptance queue opens.
// 2016-05-25 — T1 — Bats release note 2016 7am matching — matching and routing
//   start at 07:00, so the queue narrows to 06:00–07:00.
// 2018-07-30 — T1 — Bats release note 2018 8pm post-market — the post-market
//   close extends to 20:00.
// 2025-05-01 — T1 — Cboe insights May 2025 — the queue moves to 02:30 and the
//   active session to 04:00.
// Evidence: docs/evidence/cboe_bzx.md
static BZX_REVISIONS: &[Revision] = revisions![
    (2014, 12, 2, &BZX_QUEUE_2014, "SEC 34-73745"),
    (
        2016,
        5,
        25,
        &BZX_0600_1700,
        "Bats release note 2016 7am matching"
    ),
    (
        2018,
        7,
        30,
        &BZX_0600_2000,
        "Bats release note 2018 8pm post-market"
    ),
    (2025, 5, 1, &CBOE_BZX_PROFILE, "Cboe insights May 2025"),
];

pub(crate) fn bzx_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &BZX_0800_1700,
        BZX_REVISIONS,
    )
}

// 2010-10-15 — T1 — SEC 34-63097 — BYX launches on 08:00–17:00 ET.
// 2014-12-01 — T1 — SEC 34-73744 — the 06:00 order-acceptance queue opens.
// 2016-05-23 — T1 — Bats release note 2016 7am matching — matching and routing
//   start at 07:00, so the queue narrows to 06:00–07:00.
// 2018-08-27 — T1 — Bats release note 2018 8pm post-market — the post-market
//   close extends to 20:00.
// Evidence: docs/evidence/cboe_byx.md
static BYX_REVISIONS: &[Revision] = revisions![
    (2010, 10, 15, &BYX_0800_1700, "SEC 34-63097"),
    (2014, 12, 1, &BYX_QUEUE_2014, "SEC 34-73744"),
    (
        2016,
        5,
        23,
        &BYX_0600_1700,
        "Bats release note 2016 7am matching"
    ),
    (
        2018,
        8,
        27,
        &CBOE_BYX_PROFILE,
        "Bats release note 2018 8pm post-market"
    ),
];

pub(crate) fn byx_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        BYX_REVISIONS,
    )
}

// 2010-07-02 — T1 — SEC 34-62431 — first-symbol production launch with the
//   07:00–08:00 acceptance queue.
// 2014-11-13 — T1 — SEC 34-73592 — the 06:00 entry start enters Rule
//   11.1(a)(1).
// 2016-05-24 — T1 — Bats release note 2016 7am matching — matching and routing
//   start at 07:00, so the queue narrows to 06:00–07:00.
// Evidence: docs/evidence/cboe_edga.md
static EDGA_REVISIONS: &[Revision] = revisions![
    (2010, 7, 2, &EDGA_LAUNCH_2010, "SEC 34-62431"),
    (2014, 11, 13, &EDGA_QUEUE_2014, "SEC 34-73592"),
    (
        2016,
        5,
        24,
        &CBOE_EDGA_PROFILE,
        "Bats release note 2016 7am matching"
    ),
];

pub(crate) fn edga_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        EDGA_REVISIONS,
    )
}

// 2010-07-02 — T1 — SEC 34-62431 — first-symbol production launch with the
//   07:00–08:00 acceptance queue.
// 2014-10-29 — T1 — SEC 34-73468 — the 06:00 entry start enters Rule
//   11.1(a)(1).
// 2016-05-26 — T1 — Bats release note 2016 7am matching — matching and routing
//   start at 07:00, so the queue narrows to 06:00–07:00.
// 2021-03-08 — T1 — Cboe press release 2021-02-08 — the 03:30 queue and 04:00
//   active session begin.
// 2021-09-07 — T1 — SEC 34-92914 — the queue moves to 02:30.
// Evidence: docs/evidence/cboe_edgx.md
static EDGX_REVISIONS: &[Revision] = revisions![
    (2010, 7, 2, &EDGX_LAUNCH_2010, "SEC 34-62431"),
    (2014, 10, 29, &EDGX_QUEUE_2014, "SEC 34-73468"),
    (
        2016,
        5,
        26,
        &EDGX_0600_2000,
        "Bats release note 2016 7am matching"
    ),
    (2021, 3, 8, &EDGX_0330_2000, "Cboe press release 2021-02-08"),
    (2021, 9, 7, &CBOE_EDGX_PROFILE, "SEC 34-92914"),
];

pub(crate) fn edgx_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        EDGX_REVISIONS,
    )
}
