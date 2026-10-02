// SPDX-License-Identifier: MIT-0

//! Hong Kong Exchanges and Clearing securities market.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static HKEX_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 16 * 3600,
}];
static HKEX_REGULAR_PRE_2011: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 10 * 3600,
    close_ssm: 16 * 3600,
}];
// Pre-opening Session split. Narrative:
// docs/evidence/hkex.md.
static HKEX_PREOPEN_MATCH_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 20 * 60,
    close_ssm: 9 * 3600 + 30 * 60,
}];
static HKEX_ORDER_ENTRY_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 9 * 3600 + 20 * 60,
}];
// Pre-2011-03-07 POS ran 09:30–10:00 against a 10:00 morning open. No primary
// SEHK text for that era's period boundaries was located, so the whole window
// is left extended rather than guessing where its matching period began.
static HKEX_PREOPEN_OLD: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 10 * 3600,
}];
// CAS 16:00–16:10 ends in a randomised uncrossing that prints the closing
// trades, so the whole auction stays extended.
static HKEX_EXTENDED_CURRENT: &[SessionRule] = &[
    HKEX_PREOPEN_MATCH_CURRENT[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 16 * 3600 + 10 * 60,
    },
];

// Current HKEX securities venue envelope: POS 09:00–09:30, continuous trading 09:30–16:00, then CAS with a randomized 16:08–16:10 close. Narrative:
// docs/evidence/hkex.md.
pub(crate) static HKEX_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Hong_Kong,
    regular: HKEX_REGULAR_CURRENT,
    extended: HKEX_EXTENDED_CURRENT,
    order_entry: HKEX_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// HKEX Phase One took effect 2011-03-07: the 09:30–12:00 morning session, 12:00–13:30 Extended Morning Session, and 13:30–16:00 afternoon session form one continuous venue envelope. Narrative:
// docs/evidence/hkex.md.
pub(crate) static HKEX_PROFILE_POST_2011_03_07: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Hong_Kong,
    regular: HKEX_REGULAR_CURRENT,
    extended: HKEX_PREOPEN_MATCH_CURRENT,
    order_entry: HKEX_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static HKEX_PROFILE_PRE_2011_03_07: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Hong_Kong,
    regular: HKEX_REGULAR_PRE_2011,
    extended: HKEX_PREOPEN_OLD,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &HKEX_PROFILE_CURRENT;

// Evidence: docs/evidence/hkex.md
static REVISIONS: &[Revision] = revisions![
    (
        2011,
        3,
        7,
        &HKEX_PROFILE_POST_2011_03_07,
        "HKEX news release 110303news"
    ),
    (
        2016,
        7,
        25,
        &HKEX_PROFILE_CURRENT,
        "HKEX market communication 160725news"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &HKEX_PROFILE_PRE_2011_03_07,
        REVISIONS,
    )
}
