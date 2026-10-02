// SPDX-License-Identifier: MIT-0

//! Bursa Malaysia cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// Bursa Malaysia: two continuous sessions bounded by order-entry/call and afternoon closing-auction/trade-at-last phases. Narrative:
// docs/evidence/bursa_malaysia.md.
// Evidence: docs/evidence/bursa_malaysia.md
static BURSA_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 12 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 45 * 60,
    },
];
// No rule is reclassified as order entry here. Narrative:
// docs/evidence/bursa_malaysia.md.
static BURSA_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600,
        close_ssm: 14 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600,
    },
];
pub(crate) static BURSA_MALAYSIA_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Kuala_Lumpur,
    regular: BURSA_REGULAR,
    extended: BURSA_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

pub(crate) const CURRENT: &StaticHoursProfile = &BURSA_MALAYSIA_PROFILE;

/// No dated revision is recorded: the reviewed grid holds for the whole audit
/// window, so every instant resolves to the one profile. A sourced revision
/// later replaces this with a real timeline row and needs no routing change.
pub(crate) fn profile_at(_as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    CURRENT
}
