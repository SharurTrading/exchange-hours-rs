// SPDX-License-Identifier: MIT-0

//! US cash-equity venue profiles.

use chrono_tz::America;

use super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, SUN_PLUS_MON_THU};

/// Reg NMS regular-hours rule set: 09:30–16:00 ET, Monday–Friday.
///
/// Shared by the venue-owned US-equity profiles without allocation.
pub static US_EQUITY_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 16 * 3600,
}];

/// Reg NMS extended-hours rule set: 04:00–09:30 and 16:00–20:00 ET.
pub static US_EQUITY_EXTENDED: &[SessionRule] = &[
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

// Nasdaq, MEMX, and MIAX Pearl publish the 04:00–20:00 shape. Sources: Nasdaq
// Equity Rules Equity 2 § 8; MEMX market-hours notice; MIAX Pearl Equities
// alert 2024-11-13 and its trading-hours page.
pub(crate) static NASDAQ_PROFILE: StaticHoursProfile = equity_profile(US_EQUITY_EXTENDED);
pub(crate) static MEMX_EQ_PROFILE: StaticHoursProfile = equity_profile(US_EQUITY_EXTENDED);
pub(crate) static MIAX_PEARL_EQ_PROFILE: StaticHoursProfile = equity_profile(US_EQUITY_EXTENDED);

// Nasdaq operated 07:00–20:00 ET before moving its pre-market open to 04:00
// effective 2013-03-18.
// https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2013-21
pub(super) static NASDAQ_PROFILE_PRE_2013_03_18: StaticHoursProfile =
    equity_profile(EXTENDED_0700_2000);

// Nasdaq Texas (the stable public identity remains `nasdaq_bx`) publishes
// 07:00–19:00 ET system hours around the 09:30–16:00 core session.
// An official 2009 circular proves an 08:00–19:00 January-2010 baseline, and
// SR-BX-2011-016 proves the later 08:00→07:00 system-hours change, and Equity
// Trader Alert 2011-20 makes its production date Monday 2011-04-18. A
// March-2014 Nasdaq data notice independently confirms the 07:00 platform open.
// https://www.nasdaqtrader.com/content/technicalsupport/nasdaq_sys_hours.pdf
// https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2009-003
// https://www.sec.gov/rules/sro/bx/2011/34-64105.pdf
// https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2011-20
// https://www.nasdaqtrader.com/TraderNews.aspx?id=dtn2014-08
static NASDAQ_BX_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 19 * 3600,
    },
];
pub(crate) static NASDAQ_BX_PROFILE: StaticHoursProfile = equity_profile(NASDAQ_BX_EXTENDED);
static NASDAQ_BX_EXTENDED_PRE_2011_04_18: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 19 * 3600,
    },
];
pub(super) static NASDAQ_BX_PROFILE_PRE_2011_04_18: StaticHoursProfile =
    equity_profile(NASDAQ_BX_EXTENDED_PRE_2011_04_18);

// Nasdaq PSX currently publishes 08:00–17:00 ET system hours. PSX launched
// with a 09:00 ET start and kept the same 17:00 close before the 2010-12-13
// expansion.
// https://listingcenter.nasdaq.com/rulebook/phlx/rules/phlx-psx-legacy-3000
// https://www.sec.gov/files/rules/sro/phlx/2010/34-63492.pdf
static NASDAQ_PSX_EXTENDED: &[SessionRule] = &[
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
static NASDAQ_PSX_EXTENDED_AT_LAUNCH: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
];
pub(crate) static NASDAQ_PSX_PROFILE: StaticHoursProfile = equity_profile(NASDAQ_PSX_EXTENDED);
pub(super) static NASDAQ_PSX_PROFILE_AT_LAUNCH: StaticHoursProfile =
    equity_profile(NASDAQ_PSX_EXTENDED_AT_LAUNCH);

// Nasdaq Equity Trader Alert 2026-46 makes the new Night Session effective
// Sunday 2026-12-06. Nasdaq Equity 1 defines it as 21:00–04:00 ET Sunday
// through Thursday, followed by the existing 04:00–20:00 Day Session on
// business days. The 20:00–21:00 interval remains the daily pause.
// https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2026-46
// https://listingcenter.nasdaq.com/rulebook/nasdaq/rules/Nasdaq%20Equity%201
static NASDAQ_EXTENDED_POST_2026_12_06: &[SessionRule] = &[
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 21 * 3600,
        close_ssm: 4 * 3600,
    },
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
pub(super) static NASDAQ_PROFILE_POST_2026_12_06: StaticHoursProfile =
    equity_profile(NASDAQ_EXTENDED_POST_2026_12_06);

pub(super) static MEMX_EQ_PROFILE_PRE_2025_05_19: StaticHoursProfile =
    equity_profile(EXTENDED_0700_2000);
pub(super) static MIAX_PEARL_EQ_PROFILE_PRE_2025_02_20: StaticHoursProfile = equity_profile(&[]);

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

pub(super) const fn equity_profile(extended: &'static [SessionRule]) -> StaticHoursProfile {
    profile(US_EQUITY_REGULAR, extended)
}

pub(super) const fn profile(
    regular: &'static [SessionRule],
    extended: &'static [SessionRule],
) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: America::New_York,
        regular,
        extended,
        has_daily_close: true,
        has_weekend_close: true,
    }
}
