// SPDX-License-Identifier: MIT-0

//! ICE Endex Dutch TTF Natural Gas Futures.

use chrono_tz::{America, Europe};

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, SUN_ONLY, TUE_FRI};
use crate::calendar::schedules::timeline::{effective_date, local_date, reference_delta_seconds};

// ICE Endex is scoped to the post-combination Dutch TTF Natural Gas Futures contract. Narrative:
// docs/evidence/ice_endex.md.
static PRE_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 18 * 3600,
}];
// PHASE CLASSIFICATION. Narrative:
// docs/evidence/ice_endex.md.
static PRE_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 45 * 60,
    close_ssm: 8 * 3600,
}];
static ALIGNED_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 23 * 3600 + 50 * 60,
        close_ssm: 23 * 3600,
    },
    SessionRule {
        days: TUE_FRI,
        open_ssm: 3600 + 50 * 60,
        close_ssm: 23 * 3600,
    },
];
static ALIGNED_ORDER_ENTRY: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 23 * 3600 + 40 * 60,
        close_ssm: 23 * 3600 + 50 * 60,
    },
    SessionRule {
        days: TUE_FRI,
        open_ssm: 3600 + 40 * 60,
        close_ssm: 3600 + 50 * 60,
    },
];
static MISMATCH_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 22 * 3600 + 50 * 60,
        close_ssm: 22 * 3600,
    },
    SessionRule {
        days: TUE_FRI,
        open_ssm: 50 * 60,
        close_ssm: 22 * 3600,
    },
];
static MISMATCH_ORDER_ENTRY: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 22 * 3600 + 40 * 60,
        close_ssm: 22 * 3600 + 50 * 60,
    },
    SessionRule {
        days: TUE_FRI,
        open_ssm: 40 * 60,
        close_ssm: 50 * 60,
    },
];

static PRE: StaticHoursProfile = amsterdam_profile(PRE_REGULAR, PRE_ORDER_ENTRY);
static CLOSED: StaticHoursProfile = amsterdam_profile(&[], &[]);
static EXTENSION_EVE: StaticHoursProfile = amsterdam_profile(
    &[SessionRule {
        days: SUN_ONLY,
        open_ssm: 23 * 3600 + 50 * 60,
        close_ssm: 23 * 3600,
    }],
    &[SessionRule {
        days: SUN_ONLY,
        open_ssm: 23 * 3600 + 40 * 60,
        close_ssm: 23 * 3600 + 50 * 60,
    }],
);
pub(crate) static CURRENT: StaticHoursProfile =
    amsterdam_profile(ALIGNED_REGULAR, ALIGNED_ORDER_ENTRY);
static MISMATCH: StaticHoursProfile = amsterdam_profile(MISMATCH_REGULAR, MISMATCH_ORDER_ENTRY);

const fn amsterdam_profile(
    regular: &'static [SessionRule],
    order_entry: &'static [SessionRule],
) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: Europe::Amsterdam,
        regular,
        extended: &[],
        order_entry,
        has_daily_close: true,
        has_weekend_close: true,
    }
}

const TRANSFER: chrono::NaiveDate = effective_date(2013, 10, 7);
const EXTENSION_OPENING_DAY: chrono::NaiveDate = effective_date(2026, 4, 12);
const EXTENSION: chrono::NaiveDate = effective_date(2026, 4, 13);

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, Europe::Amsterdam);
    if day < TRANSFER {
        return &CLOSED;
    }
    if day < EXTENSION_OPENING_DAY {
        return &PRE;
    }
    if day < EXTENSION {
        return &EXTENSION_EVE;
    }
    if reference_delta_seconds(as_of, Europe::Amsterdam, America::New_York) == -6 * 3600 {
        &CURRENT
    } else {
        &MISMATCH
    }
}
