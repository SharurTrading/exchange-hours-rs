// SPDX-License-Identifier: MIT-0

//! CME Live Cattle, Feeder Cattle, and Lean Hog futures schedules.

use chrono_tz::US;

use crate::calendar::rule::{FRI, MON_FRI, MON_ONLY};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};
use crate::calendar::{FuturesSessionProfile, SessionRule};

const MON_WED: [bool; 7] = [true, true, true, false, false, false, false];
const THU_ONLY: [bool; 7] = [false, false, false, true, false, false, false];
const TUE_THU: [bool; 7] = [false, true, true, true, false, false, false];

// CME Live Cattle, Feeder Cattle and Lean Hog futures. The January-2010 floor
// grid is the 2007 around-the-clock schedule with the pre-floor 13:55 CT Friday
// close; the dated rows below carry it to the current 08:30-13:05 CT weekday
// session. See docs/evidence/globex_livestock.md.
static REGULAR_AT_2010_FLOOR: &[SessionRule] = &[
    SessionRule {
        days: MON_ONLY,
        open_ssm: 9 * 3600 + 5 * 60,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: MON_WED,
        open_ssm: 17 * 3600,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: THU_ONLY,
        open_ssm: 17 * 3600,
        close_ssm: 13 * 3600 + 55 * 60,
    },
];

static REGULAR_2014_10_27: &[SessionRule] = &[
    SessionRule {
        days: MON_ONLY,
        open_ssm: 9 * 3600 + 5 * 60,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: TUE_THU,
        open_ssm: 8 * 3600,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: FRI,
        open_ssm: 8 * 3600,
        close_ssm: 13 * 3600 + 55 * 60,
    },
];

static REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 5 * 60,
}];
// ORDER-ENTRY CLASSIFICATION. Both phases modelled after 2016-02-29 are
// non-matching: 08:00-08:30 is the Pre-Open that queues orders until the 08:30
// regular open, and 14:30-16:00 is the PCP that follows the 13:05 close. The
// family therefore has no tradeable extended session at all: `extended` is
// empty and both phases are `order_entry`.
pub(crate) static ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 30 * 60,
        close_ssm: 16 * 3600,
    },
];

pub(crate) static CURRENT_FUTURES_PROFILE: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: REGULAR_CURRENT,
    extended: &[],
    order_entry: ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_AT_2010_FLOOR: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: REGULAR_AT_2010_FLOOR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_2014_10_27: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: REGULAR_2014_10_27,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// PRE-2020 MORNING QUEUE, CARRIED BACK TO THE MATCHING GRID IT BELONGS TO.
// SER-8599R states the outgoing 06:00 value when it dates the move to 08:00, so
// 06:00-08:30 is carried across 2016-02-29..2020-05-31 rather than omitted. It
// is deliberately NOT carried further back: before 2016-02-29 there was no
// 08:30 open for a morning queue to precede. See
// docs/evidence/globex_livestock.md.
static ORDER_ENTRY_PRE_2020_MORNING: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 6 * 3600,
    close_ssm: 8 * 3600 + 30 * 60,
}];
static ORDER_ENTRY_PRE_2020_MORNING_AND_PCP: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 6 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 30 * 60,
        close_ssm: 16 * 3600,
    },
];

static PROFILE_2016_02_29: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: REGULAR_CURRENT,
    extended: &[],
    order_entry: ORDER_ENTRY_PRE_2020_MORNING,
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_2016_06_06: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: REGULAR_CURRENT,
    extended: &[],
    order_entry: ORDER_ENTRY_PRE_2020_MORNING_AND_PCP,
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: REGULAR_CURRENT,
    extended: &[],
    order_entry: ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Evidence: docs/evidence/globex_livestock.md
static REVISIONS: &[Revision] = revisions![
    // 2014-10-27 — T1 — CME SER-7194 — the evening sessions are removed from the
    // around-the-clock grid.
    (2014, 10, 27, &PROFILE_2014_10_27, "CME SER-7194"),
    // 2016-02-29 — T1 — CME SER-7591 — the current 08:30-13:05 CT weekday
    // session is established for LE, GF and HE.
    (2016, 2, 29, &PROFILE_2016_02_29, "CME SER-7591"),
    // 2016-06-06 — T1 — CME Globex notice 20160530 — a Monday-Friday 14:30-16:00
    // CT Post-Close order-entry period begins.
    (
        2016,
        6,
        6,
        &PROFILE_2016_06_06,
        "CME Globex notice 20160530"
    ),
    // 2020-05-31 — T1 — CME SER-8599R — the morning Pre-Open start moves from
    // 06:00 to 08:00 CT for trade date Monday 2020-06-01.
    (2020, 5, 31, &PROFILE_CURRENT, "CME SER-8599R"),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, US::Central),
        &PROFILE_AT_2010_FLOOR,
        REVISIONS,
    )
}
