// SPDX-License-Identifier: MIT-0

//! US cash-equity venue profiles.

use chrono_tz::America;

use super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

/// Reg NMS regular-hours rule set: 09:30–16:00 ET, Monday–Friday.
///
/// Shared by the venue-owned US-equity profiles without allocation.
pub(crate) static US_EQUITY_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 16 * 3600,
}];

/// Reg NMS extended-hours rule set: 04:00–09:30 and 16:00–20:00 ET.
pub(crate) static US_EQUITY_EXTENDED: &[SessionRule] = &[
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

// Nasdaq, MEMX and MIAX Pearl publish the same 04:00–20:00 shape.
// See docs/evidence/nasdaq.md, docs/evidence/memx_eq.md and
// docs/evidence/miax_pearl_eq.md.
pub(crate) static NASDAQ_PROFILE: StaticHoursProfile = equity_profile(US_EQUITY_EXTENDED);
pub(crate) static MEMX_EQ_PROFILE: StaticHoursProfile = equity_profile(US_EQUITY_EXTENDED);
pub(crate) static MIAX_PEARL_EQ_PROFILE: StaticHoursProfile = equity_profile(US_EQUITY_EXTENDED);

// Nasdaq's pre-2013-03-18 grid: 07:00–20:00 ET.
// See docs/evidence/nasdaq.md.
pub(super) static NASDAQ_PROFILE_PRE_2013_03_18: StaticHoursProfile =
    equity_profile(EXTENDED_0700_2000);

// Nasdaq BX, renamed Nasdaq Texas by the operator. This is not the unrelated
// NYSE Texas venue, whose profile lives in `nyse.rs`. The stable public
// identity here remains `nasdaq_bx`. The venue publishes 07:00–19:00 ET system
// hours around the 09:30–16:00 core session.
// See docs/evidence/nasdaq_bx.md.
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

// Nasdaq PSX publishes 08:00–17:00 ET system hours; it launched with a 09:00
// ET start and kept the same 17:00 close. See docs/evidence/nasdaq_psx.md.
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

pub(super) static MEMX_EQ_PROFILE_PRE_2025_05_19: StaticHoursProfile =
    equity_profile(EXTENDED_0700_2000);
pub(super) static MEMX_EQ_PROFILE_2020_10_05: StaticHoursProfile =
    equity_profile(EXTENDED_0700_1700);
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

// MEMX's 2020-10-05 to 2023-02-01 Post-Market Session close: 17:00 ET.
// See docs/evidence/memx_eq.md.
static EXTENDED_0700_1700: &[SessionRule] = &[
    EXTENDED_0700_2000[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
];

pub(super) const fn equity_profile(extended: &'static [SessionRule]) -> StaticHoursProfile {
    profile(US_EQUITY_REGULAR, extended)
}

/// Reg NMS regular hours plus an order-entry-only phase that precedes matching.
///
/// Used by the venues that publish an order-acceptance edge ahead of their
/// first executable session; nothing can print inside `order_entry`.
pub(super) const fn equity_profile_with_entry(
    extended: &'static [SessionRule],
    order_entry: &'static [SessionRule],
) -> StaticHoursProfile {
    profile_with_entry(US_EQUITY_REGULAR, extended, order_entry)
}

pub(super) const fn profile(
    regular: &'static [SessionRule],
    extended: &'static [SessionRule],
) -> StaticHoursProfile {
    profile_with_entry(regular, extended, &[])
}

pub(super) const fn profile_with_entry(
    regular: &'static [SessionRule],
    extended: &'static [SessionRule],
    order_entry: &'static [SessionRule],
) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: America::New_York,
        regular,
        extended,
        order_entry,
        has_daily_close: true,
        has_weekend_close: true,
    }
}
