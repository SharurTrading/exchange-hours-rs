// SPDX-License-Identifier: MIT-0

//! CME cryptocurrency futures schedules.
//!
//! Evidence, quotations and URLs: `docs/evidence/globex_cryptocurrency.md`.

use chrono_tz::US;

use crate::calendar::rule::{FRI, MON_FRI, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};
use crate::calendar::{FuturesSessionProfile, SessionRule};

const SAT_ONLY: [bool; 7] = [false, false, false, false, false, true, false];
const THU_ONLY: [bool; 7] = [false, false, false, true, false, false, false];

// CME non-spot-quoted cryptocurrency futures: the 2017-12-17 five-day launch
// grid, then the 24/7 Globex grid CME filing 26-114 introduced for trade date
// 2026-05-30, plus three one-day Saturday extensions. See
// docs/evidence/globex_cryptocurrency.md.

// `SessionRule` spans at most one local midnight, so the multi-day weekend
// session is stored in adjacent pieces. The key-backed calendar joins those at
// query time, retaining the 02:00-03:45 Saturday closed break and the
// 03:45-04:00 Pre-Open.

// Both weekend blocks carry the following open business date: normally Monday,
// or Tuesday when a caller policy closes Monday. The corresponding daily bar
// runs from Friday 16:01 Pre-Open through that business date's 16:00 close.
static FIVE_DAY_EXTENDED: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];

static CURRENT_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 0,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 60,
        close_ssm: 24 * 3600,
    },
    SessionRule {
        days: SAT_ONLY,
        open_ssm: 0,
        close_ssm: 2 * 3600,
    },
    SessionRule {
        days: SAT_ONLY,
        open_ssm: 3 * 3600 + 45 * 60,
        close_ssm: 24 * 3600,
    },
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 0,
        close_ssm: 24 * 3600,
    },
];

// The permanent 24/7 normal week did not govern the morning of its first day:
// that trading block still began Thursday at 17:00 and closed Friday at 16:00.
// The new schedule then entered Pre-Open at 16:01 and matching resumed at
// 16:02. Keeping this one-day bridge prevents the historical Thursday open
// from being rewritten as Friday midnight.
static EXTENDED_2026_05_29: &[SessionRule] = &[
    SessionRule {
        days: THU_ONLY,
        open_ssm: 17 * 3600,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: FRI,
        open_ssm: 16 * 3600 + 60,
        close_ssm: 24 * 3600,
    },
];

// One-day Saturday extensions: the weekday and Sunday pieces are the normal
// grid; only the Saturday reopen moves, and no replacement Pre-Open is
// published, so the 03:45 queue is absent on those days.
macro_rules! saturday_extended_to {
    ($name:ident, $reopen_hour:expr) => {
        static $name: &[SessionRule] = &[
            SessionRule {
                days: MON_FRI,
                open_ssm: 0,
                close_ssm: 16 * 3600,
            },
            SessionRule {
                days: MON_FRI,
                open_ssm: 16 * 3600 + 60,
                close_ssm: 24 * 3600,
            },
            SessionRule {
                days: SAT_ONLY,
                open_ssm: 0,
                close_ssm: 2 * 3600,
            },
            SessionRule {
                days: SAT_ONLY,
                open_ssm: $reopen_hour * 3600,
                close_ssm: 24 * 3600,
            },
            SessionRule {
                days: SUN_ONLY,
                open_ssm: 0,
                close_ssm: 24 * 3600,
            },
        ];
    };
}
saturday_extended_to!(EXTENDED_2026_08_01, 9);
saturday_extended_to!(EXTENDED_2026_08_29, 6);
saturday_extended_to!(EXTENDED_2026_09_19, 8);

pub(crate) static CURRENT_FUTURES_PROFILE: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: CURRENT_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: false,
};

static CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static FIVE_DAY: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: FIVE_DAY_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: CURRENT_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: false,
};

static TRANSITION_2026_05_29: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_2026_05_29,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: false,
};

macro_rules! temporary_saturday {
    ($name:ident, $extended:ident) => {
        static $name: StaticHoursProfile = StaticHoursProfile {
            tz: US::Central,
            regular: &[],
            extended: $extended,
            order_entry: &[],
            has_daily_close: true,
            has_weekend_close: false,
        };
    };
}
temporary_saturday!(TEMPORARY_2026_08_01, EXTENDED_2026_08_01);
temporary_saturday!(TEMPORARY_2026_08_29, EXTENDED_2026_08_29);
temporary_saturday!(TEMPORARY_2026_09_19, EXTENDED_2026_09_19);

// Evidence: docs/evidence/globex_cryptocurrency.md
static REVISIONS: &[Revision] = revisions![
    // 2017-12-17 — T1 — CME SER-8051R — five-day launch grid, 17:00-16:00 CT.
    (2017, 12, 17, &FIVE_DAY, "CME SER-8051R"),
    // 2026-05-29 — T1 — CME filing 26-114 — one-day bridge into the 24/7 grid.
    (2026, 5, 29, &TRANSITION_2026_05_29, "CME filing 26-114"),
    // 2026-05-30 — T1 — CME filing 26-114 — permanent 24/7 normal week.
    (2026, 5, 30, &CURRENT, "CME filing 26-114"),
    // 2026-08-01 — T1 — CME Globex notice 20260727 — Saturday reopen 09:00 CT.
    (
        2026,
        8,
        1,
        &TEMPORARY_2026_08_01,
        "CME Globex notice 20260727"
    ),
    // 2026-08-02 — T1 — CME Globex notice 20260727 — revert to the standard
    // 02:00-04:00 CT window.
    (2026, 8, 2, &CURRENT, "CME Globex notice 20260727"),
    // 2026-08-29 — T1 — CME Globex notice 20260824 — Saturday reopen 06:00 CT.
    (
        2026,
        8,
        29,
        &TEMPORARY_2026_08_29,
        "CME Globex notice 20260824"
    ),
    // 2026-08-30 — T1 — CME Globex notice 20260824 — revert to the standard
    // 02:00-04:00 CT window.
    (2026, 8, 30, &CURRENT, "CME Globex notice 20260824"),
    // 2026-09-19 — T1 — CME Globex notice 20260824 — Saturday reopen 08:00 CT.
    // Forward-dated on the operator's statement; confirm before the day.
    (
        2026,
        9,
        19,
        &TEMPORARY_2026_09_19,
        "CME Globex notice 20260824"
    ),
    // 2026-09-20 — T1 — CME Globex notice 20260824 — revert to the standard
    // 02:00-04:00 CT window.
    (2026, 9, 20, &CURRENT, "CME Globex notice 20260824"),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, REVISIONS)
}
