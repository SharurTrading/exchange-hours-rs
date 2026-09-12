// SPDX-License-Identifier: MIT-0

//! NYMEX platinum and palladium Trading at Settlement (TAS) schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// NYMEX platinum-group Trading at Settlement books in America/Chicago: TAS on
// platinum `PLT` and palladium `PAT`, one era each and no revision of any
// kind. Each opens 17:00 CT Sunday through Thursday and wraps to its own close
// with no Friday-evening reopen; `regular` is empty in every era as a sourced
// absence, and each pre-launch era is a sourced closure.
// Narrative: docs/evidence/globex_platinum_tas.md

static PLATINUM_EXTENDED: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 12 * 3600 + 5 * 60,
}];
static PALLADIUM_EXTENDED: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 12 * 3600,
}];

/// The platinum-group TAS queue: Sunday 16:00-17:00 and Monday-Thursday
/// 16:45-17:00 CT, as CME's client-systems wiki states it.
///
/// Nothing is withheld on the Sunday leg, unlike the COMEX TAS keys: both roots
/// launched after the undated 2012 move, so 16:00 is their earliest sourced
/// onset rather than a later state carried back over a wider one.
pub(crate) static PGM_TAS_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600,
    },
];

const fn profile(
    extended: &'static [SessionRule],
    order_entry: &'static [SessionRule],
) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: US::Central,
        regular: &[],
        extended,
        order_entry,
        has_daily_close: true,
        has_weekend_close: true,
    }
}

/// Before the listing notice the root was not TAS-eligible, and CME's own
/// eligibility lists and its pre-launch specification capture say so.
static CLOSED: StaticHoursProfile = profile(&[], &[]);

static PLATINUM_AT_LAUNCH: StaticHoursProfile =
    profile(PLATINUM_EXTENDED, PGM_TAS_ORDER_ENTRY_CURRENT);
static PALLADIUM_AT_LAUNCH: StaticHoursProfile =
    profile(PALLADIUM_EXTENDED, PGM_TAS_ORDER_ENTRY_CURRENT);

// Both revision rows are T1 and are keyed to the local Sunday opening day of
// the first session each governs; the effective day and citation literal are
// the row's own fields, and the notices, quotations and URLs are in the
// evidence files.
// Evidence: docs/evidence/globex_platinum_tas.md
static PLATINUM_REVISIONS: &[Revision] = revisions![(
    2017,
    5,
    21,
    &PLATINUM_AT_LAUNCH,
    "CME Globex notice 20170508"
)];
// Evidence: docs/evidence/globex_palladium_tas.md
static PALLADIUM_REVISIONS: &[Revision] = revisions![(
    2018,
    11,
    18,
    &PALLADIUM_AT_LAUNCH,
    "CME Globex notice 20181112"
)];

/// The current platinum TAS matching leg: Sunday-Thursday 17:00 CT wrapping to
/// 12:05 CT, with no Friday-evening reopen.
pub(crate) static PLATINUM_TAS_EXTENDED_CURRENT: &[SessionRule] = PLATINUM_EXTENDED;
/// The current palladium TAS matching leg, wrapping to 12:00 CT, and with no
/// daily break: the specification's "60-minute break" clause is inherited
/// boilerplate from the outright row and CME deleted it in 2020.
pub(crate) static PALLADIUM_TAS_EXTENDED_CURRENT: &[SessionRule] = PALLADIUM_EXTENDED;

pub(crate) fn platinum_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, PLATINUM_REVISIONS)
}

pub(crate) fn palladium_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, PALLADIUM_REVISIONS)
}
