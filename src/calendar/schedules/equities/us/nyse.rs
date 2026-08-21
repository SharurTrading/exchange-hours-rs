// SPDX-License-Identifier: MIT-0

//! NYSE-family US cash-equity profiles and dated schedule history.

use chrono_tz::America;

use super::StaticHoursProfile;
use super::equities::{US_EQUITY_EXTENDED, equity_profile};
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

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

/// NYSE Texas opening and late sessions: 07:00–09:30 and 16:00–20:00 ET.
pub static NYSE_TEXAS_EXTENDED: &[SessionRule] = EXTENDED_0700_2000;

// This profile is scoped to NYSE Tape A. Its core is 09:30–16:00 ET; the 06:30
// pre-opening accepts and queues orders but is not executable. The operator's
// separately published Tapes B/C early session is outside this row's scope.
// https://www.nyse.com/markets/hours-calendars
pub(crate) static NYSE_PROFILE: StaticHoursProfile = equity_profile(&[]);

// NYSE Arca's 04:00–20:00 exchange-level grid predates the audit floor: the
// SEC's 2008 filing identifies the 04:00–09:30, 09:30–16:00, and 16:00–20:00
// sessions, and the living NYSE table retains those boundaries.
// https://www.sec.gov/files/rules/sro/nysearca/2008/34-57505.pdf
// https://www.nyse.com/markets/hours-calendars
pub(crate) static NYSE_ARCA_PROFILE: StaticHoursProfile = equity_profile(US_EQUITY_EXTENDED);

// NYSE American's Pillar launch added the continuous 07:00–20:00 envelope on
// 2017-07-24. Before Pillar, NYSE Amex/MKT's continuous session was
// 09:30–16:00; discrete off-hours crosses are outside this interval model.
// https://www.sec.gov/rules/sro/nyseamex/2010/34-61890.pdf
// https://www.nyse.com/publicdocs/nyse/markets/nyse-american/Pillar_Update_NYSE_American_March_2017.pdf
// https://www.nyse.com/publicdocs/nyse/markets/nyse-american/Pillar_Update_NYSE_American_Weekend_Test_Update_July21_2017.pdf
static NYSE_AMERICAN_PRE_PILLAR: StaticHoursProfile = equity_profile(&[]);
pub(crate) static NYSE_AMERICAN_PROFILE: StaticHoursProfile = equity_profile(EXTENDED_0700_2000);

static AMERICAN_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2017, 7, 24),
    profile: &NYSE_AMERICAN_PROFILE,
}];

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
pub(crate) static NYSE_NATIONAL_PROFILE: StaticHoursProfile = equity_profile(EXTENDED_0700_2000);

static NATIONAL_CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: &[],
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// NSX's operative 2010 filing dates the 18:30→20:00 close extension to
// 2010-08-02. Its immediately operative 2014 filing shortened that close to
// 17:00 on 2014-05-16. Trading ceased after the 2014-05-30 close. The SEC's
// 2015 approval says the resumed marketplace would use the rules then in
// effect, and the exchange's SEC-filed Form 1 dates its phased relaunch to
// 2015-12-22. It ceased again before the 2017-02-01 open. NYSE National
// launched on Pillar on 2018-05-21 with its current 07:00–20:00 grid.
// https://www.sec.gov/files/rules/sro/nsx/2010/34-62643.pdf
// https://www.sec.gov/files/rules/sro/nsx/2014/34-72215.pdf
// https://www.sec.gov/files/rules/sro/nsx/2014/34-72107.pdf
// https://www.sec.gov/files/rules/sro/nsx/2015/34-76640.pdf
// https://www.sec.gov/Archives/edgar/vprr/1601/16019238.pdf
// https://www.sec.gov/files/rules/sro/nsx/2017/34-80018.pdf
// https://www.nyse.com/publicdocs/nyse/markets/nyse-national/rule-filings/filings/2020/SR-NYSENat-2020-05.pdf
static NATIONAL_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2010, 8, 2),
        profile: &NATIONAL_2010_08_02,
    },
    Revision {
        effective: effective_date(2014, 5, 16),
        profile: &NATIONAL_2015_12_22,
    },
    Revision {
        effective: effective_date(2014, 5, 31),
        profile: &NATIONAL_CLOSED,
    },
    Revision {
        effective: effective_date(2015, 12, 22),
        profile: &NATIONAL_2015_12_22,
    },
    Revision {
        effective: effective_date(2017, 2, 1),
        profile: &NATIONAL_CLOSED,
    },
    Revision {
        effective: effective_date(2018, 5, 21),
        profile: &NYSE_NATIONAL_PROFILE,
    },
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

// NYSE Texas went live on 2025-03-31 with the operator's 07:00–20:00 grid.
// https://www.nyse.com/markets/nyse-texas
pub(crate) static NYSE_TEXAS_PROFILE: StaticHoursProfile = equity_profile(EXTENDED_0700_2000);

static TEXAS_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2025, 3, 31),
    profile: &NYSE_TEXAS_PROFILE,
}];

pub(crate) fn nyse_texas_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &NATIONAL_CLOSED,
        TEXAS_REVISIONS,
    )
}
