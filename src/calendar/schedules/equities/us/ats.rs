// SPDX-License-Identifier: MIT-0

//! US alternative-trading-system profiles and the IEX exchange identity.

use chrono_tz::America;

use super::StaticHoursProfile;
use super::equities::{equity_profile, profile};
use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, SUN_PLUS_MON_THU};
use crate::calendar::schedules::CLOSED_NEW_YORK;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// Investors Exchange's initial exchange rules and living hours table define
// System Hours as 08:00–17:00 ET around the 09:30–16:00 regular session.
// Trading Alert 2016-042 dates the exchange launch to 2016-08-19 and records
// the symbol-by-symbol phase-in through 2016-09-02, when the predecessor ATS
// ceased. The stable `iex` exchange identity is therefore closed before the
// first non-test securities transitioned; ATS history is not conflated with it.
// https://www.sec.gov/files/rules/sro/iex/2016/34-78447.pdf
// https://www.iex.io/resources/trading/trading-hours-holidays
// https://iextrading.com/trading/alerts/2016/042/
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

static IEX_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2016, 8, 19),
    profile: &IEX_PROFILE,
}];

pub(crate) fn iex_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        IEX_REVISIONS,
    )
}

/// Blue Ocean ATS 20:00→04:00 ET new-order trading window, Sunday through Thursday.
///
/// The live SEC Form ATS-N ends new-order acceptance at 04:00. It permits
/// resting-book clearing matches for less than a further minute, but that
/// bounded cleanup is outside this new-order session scope. Friday night is
/// excluded because the reporting facility is unavailable on Saturday.
/// <https://www.sec.gov/Archives/edgar/data/1795131/000090266426001359/xslATS-N_X01/primary_doc.xml>
static BLUE_OCEAN_EXTENDED: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 20 * 3600,
    close_ssm: 4 * 3600,
}];

pub(crate) static BLUE_OCEAN_PROFILE: StaticHoursProfile = profile(&[], BLUE_OCEAN_EXTENDED);

// The `blue_ocean_ats` profile is scoped to Blue Ocean's production ATS
// service, whose official launch was 2021-10-05. The operator describes an
// earlier June 2021 beta without a day-level start; testing/beta activity is
// outside this production-service identity and is not backfilled as trading.
// The launch-era, 2023, and live ATS-N filings all end new-order acceptance at
// 04:00. The live filing's sub-minute resting-book cleanup is excluded from the
// stated new-order trading-window scope, so it creates no schedule revision.
// https://blueocean-tech.io/2021/10/05/announcing-launch-of-blue-ocean-ats-afterhours-trading/
// https://blueocean-tech.io/timeline/
// https://www.sec.gov/Archives/edgar/data/1795131/000153949721000764/primary_doc.xml
// https://www.sec.gov/Archives/edgar/data/1795131/000153949723000091/primary_doc.xml
static BLUE_OCEAN_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2021, 10, 5),
    profile: &BLUE_OCEAN_PROFILE,
}];

pub(crate) fn blue_ocean_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        BLUE_OCEAN_REVISIONS,
    )
}
