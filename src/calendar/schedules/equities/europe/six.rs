// SPDX-License-Identifier: MIT-0

//! SIX Swiss Exchange cash equities.

use chrono_tz::Europe;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// SIX Swiss Exchange — shares segments (Blue Chip / Mid-/Small-Cap), which is what `Exchange::Six` denotes. Narrative:
// docs/evidence/six.md.
static SIX_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 2 * 60,
    close_ssm: 17 * 3600 + 20 * 60,
}];
static SIX_EXTENDED_CURRENT: &[SessionRule] = &[
    // Opening: the Trading Guide's period overview gives Pre-Opening as
    // "06:00 CET until Opening" and the Opening its own two-minute random time,
    // so 09:00-09:02 is the opening auction itself. It prints trades - orders
    // entered during Pre-Opening are "included in the Opening Auction" and any
    // non-executed part expires there - so this window stays tradeable.
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 9 * 3600 + 2 * 60,
    },
    // Closing auction.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 20 * 60,
        close_ssm: 17 * 3600 + 32 * 60,
    },
    // Trading-At-Last: execution at the closing price after the auction.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 32 * 60,
        close_ssm: 17 * 3600 + 40 * 60,
    },
];
// Pre-Opening and Post Trading are order-entry-only. Narrative:
// docs/evidence/six.md.
static SIX_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    // Pre-opening, ending when the opening auction starts at 09:00.
    SessionRule {
        days: MON_FRI,
        open_ssm: 6 * 3600,
        close_ssm: 9 * 3600,
    },
    // Post-trading: orders for a future day may be entered but cannot execute.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 40 * 60,
        close_ssm: 22 * 3600,
    },
];
static SIX_EXTENDED_PRE_TAL: &[SessionRule] = &[
    SIX_EXTENDED_CURRENT[0],
    // Before TAL, the randomized closing auction itself kept the venue open
    // for as long as two minutes after its nominal 17:30 run time.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 20 * 60,
        close_ssm: 17 * 3600 + 32 * 60,
    },
];
// Before TAL, post-trading began as soon as the closing auction ended.
static SIX_ORDER_ENTRY_PRE_TAL: &[SessionRule] = &[
    SIX_ORDER_ENTRY_CURRENT[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 32 * 60,
        close_ssm: 22 * 3600,
    },
];

pub(crate) static SIX_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Zurich,
    regular: SIX_REGULAR,
    extended: SIX_EXTENDED_CURRENT,
    order_entry: SIX_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};
static SIX_PROFILE_PRE_2020_06_22: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Zurich,
    regular: SIX_REGULAR,
    extended: SIX_EXTENDED_PRE_TAL,
    order_entry: SIX_ORDER_ENTRY_PRE_TAL,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// Trading-At-Last launched with SMR8.2 on 2020-06-22. The readiness document
// gives both the production date and the added 17:30-17:40 phase.
// https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/participation/SWXess-maintenance-releases/smr82_participant_readiness.pdf
// Evidence: docs/evidence/six.md
static REVISIONS: &[Revision] = revisions![(
    2020,
    6,
    22,
    &SIX_PROFILE,
    "SIX SMR8.2 participant readiness"
),];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Europe::Zurich),
        &SIX_PROFILE_PRE_2020_06_22,
        REVISIONS,
    )
}
