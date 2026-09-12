// SPDX-License-Identifier: MIT-0

//! COMEX gold, silver and copper Trading at Settlement (TAS) schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// COMEX Trading at Settlement books in America/Chicago, three keys on one
// module: gold `GCT`, silver `SIT` and copper `HGT`. Each opens 17:00 CT
// Sunday through Thursday and wraps to its own close with no Friday-evening
// reopen, and `regular` is empty in every era as a sourced absence. Each
// pre-launch era is a sourced closure, not an unworked gap.
// Narrative: docs/evidence/globex_gold_tas.md

static GOLD_EXTENDED: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 12 * 3600 + 30 * 60,
}];
static SILVER_EXTENDED: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 12 * 3600 + 25 * 60,
}];
static COPPER_EXTENDED: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 12 * 3600,
}];

// THE TWO DATED REVISIONS ARE BOTH ORDER-ENTRY ONLY, AND BOTH TOUCH ONLY THESE
// QUEUES. RA1104-4 staggers the onsets per group on 2011-04-10 and Globex
// notice 20120409 restores the shared 16:15 / 16:45 CT values on 2012-04-15.
// The Sunday onset's later, undated move to 16:00 is withheld: a knowledge
// boundary may only widen, so the rows serve 16:15 to 17:00 (issue #79).
// Narrative: docs/evidence/globex_gold_tas.md

/// The shared TAS queue: Sunday 16:15-17:00 and Monday-Thursday 16:45-17:00 CT.
///
/// In force from each key's launch to 2011-04-09 and again from 2012-04-15, and
/// the current table. The Sunday rule starts at the sourced intersection's
/// 16:15, not at the wiki's undated 16:00.
pub(crate) static METALS_TAS_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600,
    },
];

/// RA1104-4's per-product stagger, in force 2011-04-10 to 2012-04-14.
macro_rules! staggered_queue {
    ($name:ident, $sunday_minute:expr, $weekday_minute:expr) => {
        static $name: &[SessionRule] = &[
            SessionRule {
                days: SUN_ONLY,
                open_ssm: 16 * 3600 + $sunday_minute * 60,
                close_ssm: 17 * 3600,
            },
            SessionRule {
                days: MON_THU,
                open_ssm: 16 * 3600 + $weekday_minute * 60,
                close_ssm: 17 * 3600,
            },
        ];
    };
}
staggered_queue!(GOLD_ORDER_ENTRY_2011_04_10, 18, 48);
staggered_queue!(SILVER_ORDER_ENTRY_2011_04_10, 19, 49);
staggered_queue!(COPPER_ORDER_ENTRY_2011_04_10, 20, 50);

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

/// Before the launch day the root was not TAS-eligible, and CME's own complete
/// eligibility lists say so, so the pre-launch era is a sourced closure.
static CLOSED: StaticHoursProfile = profile(&[], &[]);

static GOLD_AT_LAUNCH: StaticHoursProfile = profile(GOLD_EXTENDED, METALS_TAS_ORDER_ENTRY_CURRENT);
static GOLD_FROM_2011_04_10: StaticHoursProfile =
    profile(GOLD_EXTENDED, GOLD_ORDER_ENTRY_2011_04_10);
static SILVER_AT_LAUNCH: StaticHoursProfile =
    profile(SILVER_EXTENDED, METALS_TAS_ORDER_ENTRY_CURRENT);
static SILVER_FROM_2011_04_10: StaticHoursProfile =
    profile(SILVER_EXTENDED, SILVER_ORDER_ENTRY_2011_04_10);
static COPPER_AT_LAUNCH: StaticHoursProfile =
    profile(COPPER_EXTENDED, METALS_TAS_ORDER_ENTRY_CURRENT);
static COPPER_FROM_2011_04_10: StaticHoursProfile =
    profile(COPPER_EXTENDED, COPPER_ORDER_ENTRY_2011_04_10);

// Every revision row below is T1 and is keyed to the local Sunday opening day
// of the first session it governs; each row's effective day and citation
// literal are its own fields, and the documents and quotations behind them are
// in the evidence files. The 2012-04-15 rows point back at each key's launch
// profile because the notice restores exactly the onsets RA1104-4 staggered.
// Evidence: docs/evidence/globex_gold_tas.md
static GOLD_REVISIONS: &[Revision] = revisions![
    (
        2010,
        4,
        11,
        &GOLD_AT_LAUNCH,
        "COMEX Submission 10-070 with SER S-5166"
    ),
    (
        2011,
        4,
        10,
        &GOLD_FROM_2011_04_10,
        "NYMEX & COMEX MRAN RA1104-4"
    ),
    (2012, 4, 15, &GOLD_AT_LAUNCH, "CME Globex notice 20120409"),
];
// Evidence: docs/evidence/globex_silver_tas.md
static SILVER_REVISIONS: &[Revision] = revisions![
    (
        2010,
        4,
        11,
        &SILVER_AT_LAUNCH,
        "COMEX Submission 10-070 with SER S-5166"
    ),
    (
        2011,
        4,
        10,
        &SILVER_FROM_2011_04_10,
        "NYMEX & COMEX MRAN RA1104-4"
    ),
    (2012, 4, 15, &SILVER_AT_LAUNCH, "CME Globex notice 20120409"),
];
// Evidence: docs/evidence/globex_copper_tas.md
static COPPER_REVISIONS: &[Revision] = revisions![
    (2011, 1, 23, &COPPER_AT_LAUNCH, "COMEX SER-5542"),
    (
        2011,
        4,
        10,
        &COPPER_FROM_2011_04_10,
        "NYMEX & COMEX MRAN RA1104-4"
    ),
    (2012, 4, 15, &COPPER_AT_LAUNCH, "CME Globex notice 20120409"),
];

/// The current gold TAS matching leg: Sunday-Thursday 17:00 CT wrapping to
/// 12:30 CT, with no Friday-evening reopen.
pub(crate) static GOLD_TAS_EXTENDED_CURRENT: &[SessionRule] = GOLD_EXTENDED;
/// The current silver TAS matching leg, wrapping to 12:25 CT.
pub(crate) static SILVER_TAS_EXTENDED_CURRENT: &[SessionRule] = SILVER_EXTENDED;
/// The current copper TAS matching leg, wrapping to 12:00 CT.
pub(crate) static COPPER_TAS_EXTENDED_CURRENT: &[SessionRule] = COPPER_EXTENDED;

pub(crate) fn gold_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, GOLD_REVISIONS)
}

pub(crate) fn silver_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, SILVER_REVISIONS)
}

pub(crate) fn copper_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, COPPER_REVISIONS)
}
