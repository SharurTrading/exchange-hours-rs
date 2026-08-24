// SPDX-License-Identifier: MIT-0

//! NYSE-family US cash-equity profiles and dated schedule history.

use chrono_tz::America;

use super::StaticHoursProfile;
use super::equities::{US_EQUITY_EXTENDED, equity_profile, equity_profile_with_entry};
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::CLOSED_NEW_YORK;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// Pillar order-entry edges. NYSE's hours table lists "Order Entry" starting at
// 06:30 (02:30 on Arca) with the first executable phase — the Early Trading
// Session — beginning only at 07:00 (04:00 on Arca). Nothing can print inside
// these windows: they exist so orders can be entered, amended and cancelled
// ahead of the first matching session, so they are `order_entry`, not
// `extended`.
// https://www.nyse.com/markets/hours-calendars
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

// NYSE accepts orders from 06:30 ET. Tape A queues them for the core opening;
// Tapes B/C also enter an active early session at 07:00. The 06:30–07:00 leg is
// therefore acceptance only — no trade can print before the Early Trading
// Session opens — and is classified `order_entry`; 07:00–09:30 stays Extended
// because Tape B/C trades execute there. The reviewed sources do not state an
// unconditional exchange-wide onset day, so the historical selector retains the
// prior core-only representation.
// https://www.nyse.com/trade/hours-calendars?os=.
// https://www.nyse.com/markets/hours-calendars
static NYSE_HISTORICAL_PROFILE: StaticHoursProfile = equity_profile(&[]);
pub(crate) static NYSE_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_0930, ENTRY_0630_0700);

pub(crate) fn nyse_profile_at(
    _as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    &NYSE_HISTORICAL_PROFILE
}

// NYSE Arca currently accepts and queues orders at 02:30 before its 04:00
// active early session. That 02:30–04:00 queue matches nothing, so it is
// `order_entry`; the 04:00–20:00 execution grid stays Extended. The execution
// grid predates the audit floor, but the primary amendment chain reviewed here
// does not give an unconditional day for the later 02:30 queue. The current
// fixed profile includes the queue; the historical selector retains only the
// sourced 04:00 execution envelope.
// https://www.sec.gov/files/rules/sro/nysearca/2008/34-57505.pdf
// https://www.nyse.com/trade/hours-calendars?os=.
// https://www.nyse.com/publicdocs/nyse/data/ArcaBook_Client_Specification.pdf
// https://www.nyse.com/publicdocs/nyse/markets/nyse-arca/rule-filings/filings/2021/SR-NYSEArca-2021-71.pdf
static NYSE_ARCA_HISTORICAL_PROFILE: StaticHoursProfile = equity_profile(US_EQUITY_EXTENDED);
pub(crate) static NYSE_ARCA_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(US_EQUITY_EXTENDED, ENTRY_0230_0400);

pub(crate) fn nyse_arca_profile_at(
    _as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    &NYSE_ARCA_HISTORICAL_PROFILE
}

// NYSE American's Pillar launch added its current 06:30 order-acceptance edge
// around the 07:00–20:00 execution grid on 2017-07-24. The acceptance edge is
// `order_entry` — 06:30–07:00 matches nothing — and the execution grid is
// Extended. Before Pillar, the
// sourced continuous session was 09:30–16:00. Legacy off-hours crosses are not
// backfilled because the exact January-2010 phase table and complete amendment
// chain have not been established; this history remains explicitly partial.
// https://www.sec.gov/rules/sro/nyseamex/2010/34-61890.pdf
// https://www.nyse.com/publicdocs/nyse/markets/nyse-american/Pillar_Update_NYSE_American_March_2017.pdf
// https://www.nyse.com/publicdocs/nyse/markets/nyse-american/Pillar_Update_NYSE_American_Weekend_Test_Update_July21_2017.pdf
static NYSE_AMERICAN_PRE_PILLAR: StaticHoursProfile = equity_profile(&[]);
pub(crate) static NYSE_AMERICAN_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_2000, ENTRY_0630_0700);

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
pub(crate) static NYSE_NATIONAL_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_2000, ENTRY_0630_0700);

// NSX's operative 2010 filing dates the 18:30→20:00 close extension to
// 2010-08-02. Its immediately operative 2014 filing shortened that close to
// 17:00 on 2014-05-16. Trading ceased after the 2014-05-30 close. The SEC's
// 2015 approval says the resumed marketplace would use the rules then in
// effect, and the exchange's SEC-filed Form 1 dates its phased relaunch to
// 2015-12-22. It ceased again before the 2017-02-01 open. NYSE National
// launched on Pillar on 2018-05-21 with its current 06:30 order-acceptance edge
// around the 07:00–20:00 execution grid; the acceptance edge is `order_entry`
// because no trade prints before the 07:00 Early Trading Session.
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
        profile: &CLOSED_NEW_YORK,
    },
    Revision {
        effective: effective_date(2015, 12, 22),
        profile: &NATIONAL_2015_12_22,
    },
    Revision {
        effective: effective_date(2017, 2, 1),
        profile: &CLOSED_NEW_YORK,
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

// NYSE Texas is the same registered exchange formerly called NYSE Chicago and
// CHX; its 2025-03-28 conversion and rename were non-substantive. At the audit
// floor, CHX accepted orders from 07:00 through 17:00 ET: early trading to
// 09:30, core trading to 16:00, the late session to 16:15, and cross-only late
// crossing through 17:00. Its Pillar migration established the current 06:30
// order-acceptance edge around the 07:00–20:00 three-session grid on 2019-11-04.
// That 06:30–07:00 edge is `order_entry`; the pre-Pillar CHX slice stays wholly
// Extended because its 07:00 early session, 16:00–16:15 late session and
// 16:15–17:00 cross-only late crossing all print trades.
// https://www.sec.gov/rules/sro/chx/2009/34-60775.pdf
// https://www.sec.gov/files/rules/sro/nysechx/2019/34-86709.pdf
// https://www.nyse.com/publicdocs/nyse/markets/nyse-chicago/NYSE_Chicago_Migration.pdf
// https://www.sec.gov/files/rules/sro/nysechx/2019/34-87264.pdf
// https://www.sec.gov/files/rules/sro/nysechx/2025/34-102507.pdf
// https://www.nyse.com/trade/hours-calendars?os=.
pub(crate) static NYSE_TEXAS_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_2000, ENTRY_0630_0700);
static NYSE_CHICAGO_PROFILE_PRE_PILLAR: StaticHoursProfile =
    equity_profile(NYSE_CHICAGO_EXTENDED_PRE_PILLAR);

static TEXAS_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2019, 11, 4),
    profile: &NYSE_TEXAS_PROFILE,
}];

pub(crate) fn nyse_texas_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &NYSE_CHICAGO_PROFILE_PRE_PILLAR,
        TEXAS_REVISIONS,
    )
}
