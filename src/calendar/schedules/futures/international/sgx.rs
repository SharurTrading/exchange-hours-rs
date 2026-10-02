// SPDX-License-Identifier: MIT-0

//! SGX Three-Month SORA Futures.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// The SGX derivatives default is Three-Month SORA Futures, not a venue-wide derivatives clock. Narrative:
// docs/evidence/sgx.md.
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

// The two opening routines: the T Pre-Opening/Non-Cancel window that precedes the 07:25 open, and the shorter T+1 routine that precedes the 18:15 reopen. Narrative:
// docs/evidence/sgx.md.
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

// Evidence: docs/evidence/sgx.md, docs/evidence/sgx_key.md
static SGX_REVISIONS: &[Revision] = revisions![(
    2024,
    7,
    29,
    &SGX_CURRENT,
    "SGX launch announcement 2024-07-29"
),];

pub(crate) fn sgx_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Asia::Singapore),
        &SGX_CLOSED,
        SGX_REVISIONS,
    )
}
