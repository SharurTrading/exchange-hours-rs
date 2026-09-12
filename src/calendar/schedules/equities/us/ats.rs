// SPDX-License-Identifier: MIT-0

//! US alternative-trading-system profiles and the IEX exchange identity.

use chrono_tz::America;

use super::StaticHoursProfile;
use super::equities::{equity_profile, profile};
use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, SUN_PLUS_MON_THU};
use crate::calendar::schedules::CLOSED_NEW_YORK;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// IEX System Hours are 08:00–17:00 ET around the 09:30–16:00 regular session.
// Both wings are executable: the exchange rules run the same continuous order
// book in the pre-market and post-market sessions, so trades print throughout
// and neither wing is order entry. See docs/evidence/iex.md.
static IEX_EXTENDED: &[SessionRule] = &[
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
pub(crate) static IEX_PROFILE: StaticHoursProfile = equity_profile(IEX_EXTENDED);

// 2016-08-19 — T1 — IEX Trading Alert 2016-042 — the exchange launches with
//   its first production symbols.
// Evidence: docs/evidence/iex.md
static IEX_REVISIONS: &[Revision] =
    revisions![(2016, 8, 19, &IEX_PROFILE, "IEX Trading Alert 2016-042"),];

pub(crate) fn iex_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        IEX_REVISIONS,
    )
}

/// Blue Ocean ATS 20:00→04:00 ET new-order trading window, Sunday through Thursday.
///
/// The window matches continuously, so it is a tradeable extended session, not
/// order entry. Friday night is excluded because the reporting facility is
/// unavailable on Saturday. See `docs/evidence/blue_ocean_ats.md`.
static BLUE_OCEAN_EXTENDED: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 20 * 3600,
    close_ssm: 4 * 3600,
}];

pub(crate) static BLUE_OCEAN_PROFILE: StaticHoursProfile = profile(&[], BLUE_OCEAN_EXTENDED);

// The `blue_ocean_ats` profile is scoped to Blue Ocean's production ATS
// new-order service; testing and beta activity are outside this identity and
// are not backfilled as trading.
// 2021-10-05 — T1 — Blue Ocean launch announcement 2021-10-05 — the production
//   ATS new-order service opens.
// Evidence: docs/evidence/blue_ocean_ats.md
static BLUE_OCEAN_REVISIONS: &[Revision] = revisions![(
    2021,
    10,
    5,
    &BLUE_OCEAN_PROFILE,
    "Blue Ocean launch announcement 2021-10-05"
),];

pub(crate) fn blue_ocean_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        BLUE_OCEAN_REVISIONS,
    )
}
