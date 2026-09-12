// SPDX-License-Identifier: MIT-0

//! CBOT Rough Rice futures and options schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

use super::grains::{
    CBOT_EXTENDED_2012_05_20, CBOT_EXTENDED_AT_2010_FLOOR, CBOT_EXTENDED_CURRENT,
    CBOT_ORDER_ENTRY_2010_04_19, CBOT_ORDER_ENTRY_2011_12_27, CBOT_ORDER_ENTRY_2013_04_07,
    CBOT_ORDER_ENTRY_AT_2010_FLOOR, CBOT_ORDER_ENTRY_CURRENT, CBOT_REGULAR_0830_1315,
    CBOT_REGULAR_0930_1315, CBOT_REGULAR_CURRENT,
};

// Rough Rice futures (CME Globex `ZR`) and options (`OZR`), CBOT Rulebook
// chapters 17 and 17A, in America/Chicago. The six eras before 2018-01-21 are
// inherited from the standard CBOT grain and oilseed grid in `grains.rs`, not
// sourced independently for Rough Rice; from that day the family has its own
// non-wrapping evening leg and no morning or post-close Pre-Open.
// Narrative: docs/evidence/globex_rough_rice.md

/// The extended leg CBOT Submission 18-001 established: Sunday through Thursday
/// evenings, 19:00-21:00 CT. `close_ssm` exceeds `open_ssm`, so this rule does
/// not wrap — from 2018-01-21 Rough Rice has no session crossing local
/// midnight.
pub(crate) static ROUGH_RICE_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 19 * 3600,
    close_ssm: 21 * 3600,
}];

/// The only order-entry phases this profile claims after the divergence: the
/// Sunday 16:00-19:00 CT and Monday-Thursday 16:45-19:00 CT queues CME's Rough
/// Rice specification publishes, each running up to the electronic open.
/// Neither can match a trade.
pub(crate) static ROUGH_RICE_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600,
        close_ssm: 19 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 19 * 3600,
    },
];

/// Rough Rice's regular session is the standard CBOT 08:30-13:20 CT grid that
/// SER-7395R set on 2015-07-05; 18-001 leaves it untouched and the contract
/// specification restates it as "Monday - Friday: 8:30 a.m. - 1:20 p.m. CT".
/// The table is shared with `grains.rs` because the two genuinely coincide.
pub(crate) use super::grains::CBOT_REGULAR_CURRENT as ROUGH_RICE_REGULAR_CURRENT;

const fn profile(
    regular: &'static [SessionRule],
    extended: &'static [SessionRule],
    order_entry: &'static [SessionRule],
) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: US::Central,
        regular,
        extended,
        order_entry,
        has_daily_close: true,
        has_weekend_close: true,
    }
}

// The six pre-divergence eras. Each is Rough Rice's own named profile built
// from the grain and oilseed tables it shared at the time; see `grains.rs` for
// the quotations and URLs behind every value.
static AT_2010_FLOOR: StaticHoursProfile = profile(
    CBOT_REGULAR_0930_1315,
    CBOT_EXTENDED_AT_2010_FLOOR,
    CBOT_ORDER_ENTRY_AT_2010_FLOOR,
);
static FROM_2010_04_19: StaticHoursProfile = profile(
    CBOT_REGULAR_0930_1315,
    CBOT_EXTENDED_AT_2010_FLOOR,
    CBOT_ORDER_ENTRY_2010_04_19,
);
static FROM_2011_12_27: StaticHoursProfile = profile(
    CBOT_REGULAR_0930_1315,
    CBOT_EXTENDED_AT_2010_FLOOR,
    CBOT_ORDER_ENTRY_2011_12_27,
);
static FROM_2012_05_20: StaticHoursProfile =
    profile(CBOT_REGULAR_0930_1315, CBOT_EXTENDED_2012_05_20, &[]);
static FROM_2013_04_07: StaticHoursProfile = profile(
    CBOT_REGULAR_0830_1315,
    CBOT_EXTENDED_CURRENT,
    CBOT_ORDER_ENTRY_2013_04_07,
);
static FROM_2013_08_18: StaticHoursProfile = profile(
    CBOT_REGULAR_0830_1315,
    CBOT_EXTENDED_CURRENT,
    CBOT_ORDER_ENTRY_CURRENT,
);
static FROM_2015_07_05: StaticHoursProfile = profile(
    CBOT_REGULAR_CURRENT,
    CBOT_EXTENDED_CURRENT,
    CBOT_ORDER_ENTRY_CURRENT,
);
// The divergence. 18-001 dates ONE thing: the executable evening leg, which
// stops wrapping and becomes 19:00-21:00 CT. It is silent on the queues, so
// this interval serves the sourced intersection of the two queue states — the
// narrower one — and no queue cutover is asserted anywhere in this timeline.
// Narrative: docs/evidence/globex_rough_rice.md
static DATED_CURRENT: StaticHoursProfile = profile(
    ROUGH_RICE_REGULAR_CURRENT,
    ROUGH_RICE_EXTENDED_CURRENT,
    ROUGH_RICE_ORDER_ENTRY_CURRENT,
);

// Every revision row below is T1; each row's effective day and citation
// literal are its own fields, and the document, quotation and URL behind each
// are in the evidence file. The first six are the CBOT grain and oilseed chain
// Rough Rice shared; their full quotations sit in `grains.rs`.
// Evidence: docs/evidence/globex_rough_rice.md
static REVISIONS: &[Revision] = revisions![
    (2010, 4, 19, &FROM_2010_04_19, "CME Globex notice 20100405"),
    (
        2011,
        12,
        27,
        &FROM_2011_12_27,
        "CFTC filing rul120711cbot001"
    ),
    (
        2012,
        5,
        20,
        &FROM_2012_05_20,
        "CME market-data advisory 20120518"
    ),
    (
        2013,
        4,
        7,
        &FROM_2013_04_07,
        "CME SER-6617 and GCC notice 2013-03-22"
    ),
    (
        2013,
        8,
        18,
        &FROM_2013_08_18,
        "CME market-data advisory 20130812"
    ),
    (2015, 7, 5, &FROM_2015_07_05, "CME SER-7395R"),
    (2018, 1, 21, &DATED_CURRENT, "CBOT Submission 18-001"),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &AT_2010_FLOOR, REVISIONS)
}
