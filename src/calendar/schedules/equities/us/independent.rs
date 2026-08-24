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
// The SEC order records that LTSE commenced operations on 2020-08-28.
// https://ltse.com/trading/trading-schedule
// https://www.sec.gov/rules/sro/ltse/2020/34-89766.pdf
// https://www.sec.gov/rules/sro/ltse/2020/34-88515.pdf
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

static LTSE_REVISIONS: &[Revision] = revisions![(2020, 8, 28, &LTSE_PROFILE, "SEC 34-89766"),];

pub(crate) fn ltse_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        LTSE_REVISIONS,
    )
}

// The SEC's 2026 order confirms that 24X commenced trading on 2025-10-14 and
// that its current sessions are 04:00–09:30, 09:30–16:00, and 16:00–20:00 ET.
// The condition-dependent 21:00–04:00 overnight session is not operative and
// therefore has no runtime revision here.
// https://www.sec.gov/files/rules/exorders/2026/34-106061.pdf
// https://equities.24exchange.com/api/media/download/68e43b4830a49c75a17a8134
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

static TWENTY_FOUR_X_REVISIONS: &[Revision] =
    revisions![(2025, 10, 14, &TWENTY_FOUR_X_PROFILE, "SEC 34-106061"),];

pub(crate) fn twenty_four_x_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        TWENTY_FOUR_X_REVISIONS,
    )
}

// TXSE's production alerts distinguish its July 6–9 test-symbol activity from
// the first NMS-stock production rollout on 2026-07-10. The current exchange
// schedule accepts orders from 08:00 through its 17:00 late-session close. The
// operator names 08:00–09:30 a Pre-Market session rather than an order-entry
// phase and does not document a separate unmatchable acceptance window, so both
// off-core legs stay Extended.
// https://www.txse.com/alerts/6a5e8e60-8753-4eac-906d-ecbbf8682df9
// https://www.txse.com/alerts/txse-production-launch-and-market-activation
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

static TXSE_REVISIONS: &[Revision] =
    revisions![(2026, 7, 10, &TXSE_PROFILE, "TXSE production launch alert"),];

pub(crate) fn txse_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        TXSE_REVISIONS,
    )
}
