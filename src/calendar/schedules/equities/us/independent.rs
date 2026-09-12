// SPDX-License-Identifier: MIT-0

//! Independent US cash-equity exchange profiles and launch history.

use chrono::{DateTime, Utc};
use chrono_tz::America;

use super::StaticHoursProfile;
use super::equities::equity_profile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::CLOSED_NEW_YORK;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// LTSE's trading schedule defines an 08:00–09:30 Early Trading Session,
// 09:30–16:00 Regular Market Session, and 16:00–17:00 Late Trading Session.
// See docs/evidence/ltse.md.
static LTSE_EXTENDED: &[SessionRule] = &[
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

pub(crate) static LTSE_PROFILE: StaticHoursProfile = equity_profile(LTSE_EXTENDED);

// 2020-08-28 — T1 — SEC 34-89766 — LTSE commences operations.
// Evidence: docs/evidence/ltse.md
static LTSE_REVISIONS: &[Revision] = revisions![(2020, 8, 28, &LTSE_PROFILE, "SEC 34-89766"),];

pub(crate) fn ltse_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        LTSE_REVISIONS,
    )
}

// 24X's live sessions are 04:00–09:30, 09:30–16:00 and 16:00–20:00 ET. The
// condition-dependent 21:00–04:00 overnight session is not operative and
// therefore has no runtime revision here. See docs/evidence/24x.md.
static TWENTY_FOUR_X_EXTENDED: &[SessionRule] = &[
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

pub(crate) static TWENTY_FOUR_X_PROFILE: StaticHoursProfile =
    equity_profile(TWENTY_FOUR_X_EXTENDED);

// 2025-10-14 — T1 — SEC 34-106061 — 24X commences trading.
// Evidence: docs/evidence/24x.md
static TWENTY_FOUR_X_REVISIONS: &[Revision] =
    revisions![(2025, 10, 14, &TWENTY_FOUR_X_PROFILE, "SEC 34-106061"),];

pub(crate) fn twenty_four_x_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        TWENTY_FOUR_X_REVISIONS,
    )
}

// TXSE accepts orders from 08:00 through its 17:00 late-session close. The
// operator names 08:00–09:30 a Pre-Market session rather than an order-entry
// phase and documents no separate unmatchable acceptance window, so both
// off-core legs stay `extended`. See docs/evidence/txse.md.
static TXSE_EXTENDED: &[SessionRule] = &[
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

pub(crate) static TXSE_PROFILE: StaticHoursProfile = equity_profile(TXSE_EXTENDED);

// 2026-07-10 — T1 — TXSE production launch alert — first live NMS production.
// Evidence: docs/evidence/txse.md
static TXSE_REVISIONS: &[Revision] =
    revisions![(2026, 7, 10, &TXSE_PROFILE, "TXSE production launch alert"),];

pub(crate) fn txse_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        TXSE_REVISIONS,
    )
}
