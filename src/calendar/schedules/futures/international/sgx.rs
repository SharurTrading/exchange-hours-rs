// SPDX-License-Identifier: MIT-0

//! SGX Three-Month SORA Futures.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// The SGX derivatives default is Three-Month SORA Futures, not a venue-wide
// derivatives clock. SGX's launch announcement gives the day-level 2024-07-29
// launch. The live product specification publishes the complete T and T+1
// opening, non-cancel, continuous, pre-close, and overnight routine. SGX's
// 2025 and 2026 derivatives calendars independently retain the same continuous
// windows.
// https://links.sgx.com/1.0.0/corporate-announcements/LG3YO2RZCGZ92J0B/359e83de092b9d70d54305133c92a82e16f676fc43ef4aa06a6976d8bc771fdf
// https://www.sgx.com/derivatives/products/stir-products?cc=SORA
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
// https://rulebook.sgx.com/rulebook/415-trading-hours-opening-and-closing-routines-and-closing-range
pub(crate) static SGX_CURRENT_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 25 * 60,
        close_ssm: 17 * 3600 + 55 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 18 * 3600 + 15 * 60,
        close_ssm: 5 * 3600 + 15 * 60,
    },
];
// The closing routine that follows the T session. It ends in a match at a
// single closing price, so a trade can print in it and it stays `extended`.
pub(crate) static SGX_CURRENT_EXTENDED: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 17 * 3600 + 55 * 60,
    close_ssm: 18 * 3600,
}];

// The two opening routines: the T Pre-Opening/Non-Cancel window that precedes
// the 07:25 open, and the shorter T+1 routine that precedes the 18:15 reopen.
// Both are Pre-Opening/Non-Cancel routines under SGX Rule 4.1.5: they collect
// orders and compute an indicative opening price without matching. The opening
// match itself falls on the session-open instant that already begins the
// `regular` window, so nothing tradeable is lost by classifying these two
// windows as `order_entry`.
pub(crate) static SGX_CURRENT_ORDER_ENTRY: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 10 * 60,
        close_ssm: 7 * 3600 + 25 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 18 * 3600 + 5 * 60,
        close_ssm: 18 * 3600 + 15 * 60,
    },
];

static SGX_CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
static SGX_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_CURRENT_REGULAR,
    extended: SGX_CURRENT_EXTENDED,
    order_entry: SGX_CURRENT_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

static SGX_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2024, 7, 29),
    profile: &SGX_CURRENT,
}];

pub(crate) fn sgx_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Asia::Singapore),
        &SGX_CLOSED,
        SGX_REVISIONS,
    )
}
