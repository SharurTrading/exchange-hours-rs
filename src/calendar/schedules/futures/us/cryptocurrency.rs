// SPDX-License-Identifier: MIT-0

//! CME cryptocurrency futures schedules.
//!
//! Evidence, quotations and URLs: `docs/evidence/globex_cryptocurrency.md`.

use chrono_tz::US;

use crate::calendar::rule::{FRI, MON_FRI, MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};
use crate::calendar::{FuturesSessionProfile, SessionRule};

const SAT_ONLY: [bool; 7] = [false, false, false, false, false, true, false];
const THU_ONLY: [bool; 7] = [false, false, false, true, false, false, false];

// CME non-spot-quoted cryptocurrency futures: the 2017-12-17 five-day launch
// grid, then the 24/7 Globex grid CME filing 26-114 introduced for trade date
// 2026-05-30, plus six one-day Saturday extensions. See
// docs/evidence/globex_cryptocurrency.md.

// `SessionRule` spans at most one local midnight, so the multi-day weekend
// session is stored in adjacent pieces. The key-backed calendar joins those at
// query time, retaining the 02:00-03:45 Saturday closed break and the
// 03:45-04:00 Pre-Open, which is `order_entry` and not a session.

// Both weekend blocks carry the following open business date: normally Monday,
// or Tuesday when a caller policy closes Monday. The corresponding daily bar
// runs from Friday 16:02, where matching resumes, through that business date's
// 16:00 close.
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
        open_ssm: 16 * 3600 + 120,
        close_ssm: 24 * 3600,
    },
    SessionRule {
        days: SAT_ONLY,
        open_ssm: 0,
        close_ssm: 2 * 3600,
    },
    SessionRule {
        days: SAT_ONLY,
        open_ssm: 4 * 3600,
        close_ssm: 24 * 3600,
    },
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 0,
        close_ssm: 24 * 3600,
    },
];

// The grid's two Pre-Open queues: weekday 16:01-16:02 CT and Saturday
// 03:45-04:00 CT.
static CURRENT_ORDER_ENTRY: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 60,
        close_ssm: 16 * 3600 + 120,
    },
    SessionRule {
        days: SAT_ONLY,
        open_ssm: 3 * 3600 + 45 * 60,
        close_ssm: 4 * 3600,
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
        open_ssm: 16 * 3600 + 120,
        close_ssm: 24 * 3600,
    },
];

/// The bridge day's own Pre-Open queue, as on the permanent grid.
static ORDER_ENTRY_2026_05_29: &[SessionRule] = &[SessionRule {
    days: FRI,
    open_ssm: 16 * 3600 + 60,
    close_ssm: 16 * 3600 + 120,
}];

// One-day Saturday extensions: the weekday and Sunday pieces are the normal
// grid; only the Saturday reopen moves. The sourced September 19 and October 3
// replacement queues are stated separately below; other extensions omit
// their unsourced replacement queue. The reopen is stated
// as hour and minute because notice 20260921's FIA drill extension ends at
// 15:30 CT, not on the hour.
macro_rules! saturday_extended_to {
    ($name:ident, $reopen_hour:expr, $reopen_minute:expr) => {
        static $name: &[SessionRule] = &[
            SessionRule {
                days: MON_FRI,
                open_ssm: 0,
                close_ssm: 16 * 3600,
            },
            SessionRule {
                days: MON_FRI,
                open_ssm: 16 * 3600 + 120,
                close_ssm: 24 * 3600,
            },
            SessionRule {
                days: SAT_ONLY,
                open_ssm: 0,
                close_ssm: 2 * 3600,
            },
            SessionRule {
                days: SAT_ONLY,
                open_ssm: $reopen_hour * 3600 + $reopen_minute * 60,
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
saturday_extended_to!(EXTENDED_2026_08_01, 9, 0);
saturday_extended_to!(EXTENDED_2026_08_29, 6, 0);
saturday_extended_to!(EXTENDED_2026_09_19, 8, 0);
saturday_extended_to!(EXTENDED_2026_10_03, 5, 0);
saturday_extended_to!(EXTENDED_2026_10_24, 15, 30);
saturday_extended_to!(EXTENDED_2026_10_31, 5, 0);

pub(crate) static CURRENT_FUTURES_PROFILE: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: CURRENT_EXTENDED,
    order_entry: CURRENT_ORDER_ENTRY,
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

// Five-day Pre-Open queues — T2 — MARKET-DATA.
// Evidence: docs/evidence/globex_cryptocurrency.md.
static FIVE_DAY_ORDER_ENTRY: &[SessionRule] = &[
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

static FIVE_DAY: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: FIVE_DAY_EXTENDED,
    order_entry: FIVE_DAY_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

static CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: CURRENT_EXTENDED,
    order_entry: CURRENT_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: false,
};

static TRANSITION_2026_05_29: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_2026_05_29,
    order_entry: ORDER_ENTRY_2026_05_29,
    has_daily_close: true,
    has_weekend_close: false,
};

// 2026-09-19 — T2 — CME-SVC-SATURDAY-2026-09-19 — replacement Pre-Open.
// Evidence: docs/evidence/globex_cryptocurrency.md.
static ORDER_ENTRY_2026_09_19: &[SessionRule] = &[SessionRule {
    days: SAT_ONLY,
    open_ssm: 7 * 3600 + 45 * 60,
    close_ssm: 8 * 3600,
}];
// 2026-10-03 — T2 — CME-SVC-SATURDAY-2026-10-03 — replacement Pre-Open.
// Evidence: docs/evidence/globex_cryptocurrency.md.
static ORDER_ENTRY_2026_10_03: &[SessionRule] = &[SessionRule {
    days: SAT_ONLY,
    open_ssm: 4 * 3600 + 45 * 60,
    close_ssm: 5 * 3600,
}];

macro_rules! temporary_saturday {
    ($name:ident, $extended:ident, $order_entry:expr) => {
        static $name: StaticHoursProfile = StaticHoursProfile {
            tz: US::Central,
            regular: &[],
            extended: $extended,
            order_entry: $order_entry,
            has_daily_close: true,
            has_weekend_close: false,
        };
    };
}
temporary_saturday!(TEMPORARY_2026_08_01, EXTENDED_2026_08_01, &[]);
temporary_saturday!(TEMPORARY_2026_08_29, EXTENDED_2026_08_29, &[]);
temporary_saturday!(
    TEMPORARY_2026_09_19,
    EXTENDED_2026_09_19,
    ORDER_ENTRY_2026_09_19
);
temporary_saturday!(
    TEMPORARY_2026_10_03,
    EXTENDED_2026_10_03,
    ORDER_ENTRY_2026_10_03
);
temporary_saturday!(TEMPORARY_2026_10_24, EXTENDED_2026_10_24, &[]);
temporary_saturday!(TEMPORARY_2026_10_31, EXTENDED_2026_10_31, &[]);

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
    // 2026-09-19 — T1 — CME Globex notice 20260824 — reopen 08:00 CT;
    // the operator service confirms the replacement 07:45 Pre-Open (T2).
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
    // 2026-10-03 — T1 — CME Globex notice 20260921 — reopen 05:00 CT;
    // the operator service confirms the replacement 04:45 Pre-Open (T2).
    (
        2026,
        10,
        3,
        &TEMPORARY_2026_10_03,
        "CME Globex notice 20260921"
    ),
    // 2026-10-04 — T1 — CME Globex notice 20260921 — revert to the standard
    // 02:00-04:00 CT window.
    (2026, 10, 4, &CURRENT, "CME Globex notice 20260921"),
    // 2026-10-24 — T1 — CME Globex notice 20260921 — FIA drill to 15:30 CT.
    // Forward-dated: confirm before the day. The 13.5-hour gap is Closed.
    (
        2026,
        10,
        24,
        &TEMPORARY_2026_10_24,
        "CME Globex notice 20260921"
    ),
    // 2026-10-25 — T1 — CME Globex notice 20260921 — revert to the standard
    // 02:00-04:00 CT window.
    (2026, 10, 25, &CURRENT, "CME Globex notice 20260921"),
    // 2026-10-31 — T1 — CME Globex notice 20261005 — maintenance to 05:00 CT.
    // Forward-dated: confirm before the day. The three-hour gap is Maintenance.
    (
        2026,
        10,
        31,
        &TEMPORARY_2026_10_31,
        "CME Globex notice 20261005"
    ),
    // 2026-11-01 — T1 — CME Globex notice 20261005 — restore the standard grid.
    (2026, 11, 1, &CURRENT, "CME Globex notice 20261005"),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, REVISIONS)
}
