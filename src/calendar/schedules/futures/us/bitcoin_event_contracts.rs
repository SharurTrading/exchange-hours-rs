// SPDX-License-Identifier: MIT-0

//! CME Event Contracts on Bitcoin Futures (`ECBTC`) schedule.

use chrono_tz::US;

use super::event_contracts::EVENT_CONTRACTS_ORDER_ENTRY_CURRENT;
use crate::calendar::SessionRule;
use crate::calendar::rule::{FRI, MON_FRI, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

const SAT_ONLY: [bool; 7] = [false, false, false, false, false, true, false];
const THU_ONLY: [bool; 7] = [false, false, false, true, false, false, false];

// Evidence: docs/evidence/globex_event_contracts_btc.md.

/// The 24/7 executable leg under the sourced intersection: Monday-Friday
/// 00:00-15:00 and 16:02-24:00 CT, Saturday 00:00-02:00 and 04:00-24:00 CT,
/// and all of Sunday. `SessionRule` spans at most one local midnight, so the
/// continuous Saturday-04:00-to-Monday-15:00 block is stored in day pieces
/// that the key-backed calendar joins at query time.
pub(crate) static BITCOIN_EVENT_CONTRACTS_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 0,
        close_ssm: 15 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 2 * 60,
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

/// The Pre-Open queues both documents state: Monday-Friday 16:01-16:02 CT and
/// Saturday 03:45-04:00 CT. Orders queue and nothing matches until the open.
pub(crate) static BITCOIN_EVENT_CONTRACTS_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 60,
        close_ssm: 16 * 3600 + 2 * 60,
    },
    SessionRule {
        days: SAT_ONLY,
        open_ssm: 3 * 3600 + 45 * 60,
        close_ssm: 4 * 3600,
    },
];

// 2023-03-12 — T1 — CME SER-9092 — the root's own five-day session.
// Evidence: docs/evidence/globex_event_contracts_btc.md.
static FIVE_DAY_EXTENDED: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 15 * 3600,
}];

// 2026-05-29 — T1 — CME SER-9740R — first 24/7 queue and Friday reopen.
// Thursday's session retains the preceding five-day grid's 15:00 CT close.
// Evidence: docs/evidence/globex_event_contracts_btc.md.
static EXTENDED_2026_05_29: &[SessionRule] = &[
    SessionRule {
        days: THU_ONLY,
        open_ssm: 17 * 3600,
        close_ssm: 15 * 3600,
    },
    SessionRule {
        days: FRI,
        open_ssm: 16 * 3600 + 2 * 60,
        close_ssm: 24 * 3600,
    },
];
static ORDER_ENTRY_2026_05_29: &[SessionRule] = &[SessionRule {
    days: FRI,
    open_ssm: 16 * 3600 + 60,
    close_ssm: 16 * 3600 + 2 * 60,
}];

// Six one-day Saturday extensions governing channel 329, each
// without a replacement Pre-Open, each followed by the standard window. The
// weekday and Sunday pieces are unchanged; only the Saturday reopen moves.
static ORDER_ENTRY_WEEKDAYS_ONLY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 16 * 3600 + 60,
    close_ssm: 16 * 3600 + 2 * 60,
}];
macro_rules! saturday_extended_to {
    ($name:ident, $reopen_hour:expr, $reopen_minute:expr) => {
        static $name: &[SessionRule] = &[
            SessionRule {
                days: MON_FRI,
                open_ssm: 0,
                close_ssm: 15 * 3600,
            },
            SessionRule {
                days: MON_FRI,
                open_ssm: 16 * 3600 + 2 * 60,
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

// CLOSED BEFORE LISTING. The root did not exist before SER-9092's listing day,
// so the pre-listing baseline is an explicit sessionless profile, as for every
// launch-dated family in this crate.
static CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

/// The root's own five-day grid, from SER-9092 and confirmed with market data.
/// Its 15:00 CT close differs from the other event contracts' 16:00 CT close.
static FROM_2023_03_12: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: FIVE_DAY_EXTENDED,
    order_entry: EVENT_CONTRACTS_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static TRANSITION_2026_05_29: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_2026_05_29,
    order_entry: ORDER_ENTRY_2026_05_29,
    has_daily_close: true,
    has_weekend_close: false,
};

/// The sourced intersection of SER-9740R and the client-systems wiki.
static SOURCED_INTERSECTION: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: BITCOIN_EVENT_CONTRACTS_EXTENDED_CURRENT,
    order_entry: BITCOIN_EVENT_CONTRACTS_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: false,
};

macro_rules! temporary_saturday {
    ($name:ident, $extended:ident) => {
        static $name: StaticHoursProfile = StaticHoursProfile {
            tz: US::Central,
            regular: &[],
            extended: $extended,
            order_entry: ORDER_ENTRY_WEEKDAYS_ONLY,
            has_daily_close: true,
            has_weekend_close: false,
        };
    };
}
temporary_saturday!(TEMPORARY_2026_08_01, EXTENDED_2026_08_01);
temporary_saturday!(TEMPORARY_2026_08_29, EXTENDED_2026_08_29);
temporary_saturday!(TEMPORARY_2026_09_19, EXTENDED_2026_09_19);
temporary_saturday!(TEMPORARY_2026_10_03, EXTENDED_2026_10_03);
temporary_saturday!(TEMPORARY_2026_10_24, EXTENDED_2026_10_24);
temporary_saturday!(TEMPORARY_2026_10_31, EXTENDED_2026_10_31);

// Every revision row below is T1; each row's effective day and citation
// literal are its own fields, and the document, quotation and URL behind each
// are in the evidence file. The 2026-09-19 row is forward-dated on the
// operator's statement.
// Evidence: docs/evidence/globex_event_contracts_btc.md
static REVISIONS: &[Revision] = revisions![
    (2023, 3, 12, &FROM_2023_03_12, "CME SER-9092"),
    (2026, 5, 29, &TRANSITION_2026_05_29, "CME SER-9740R"),
    (2026, 5, 30, &SOURCED_INTERSECTION, "CME SER-9740R"),
    (
        2026,
        8,
        1,
        &TEMPORARY_2026_08_01,
        "CME Globex notice 20260727"
    ),
    (
        2026,
        8,
        2,
        &SOURCED_INTERSECTION,
        "CME Globex notice 20260727"
    ),
    (
        2026,
        8,
        29,
        &TEMPORARY_2026_08_29,
        "CME Globex notice 20260824"
    ),
    (
        2026,
        8,
        30,
        &SOURCED_INTERSECTION,
        "CME Globex notice 20260824"
    ),
    (
        2026,
        9,
        19,
        &TEMPORARY_2026_09_19,
        "CME Globex notice 20260824"
    ),
    (
        2026,
        9,
        20,
        &SOURCED_INTERSECTION,
        "CME Globex notice 20260824"
    ),
    // 2026-10-03 — T1 — CME Globex notice 20260921 — 24/7 maintenance to 05:00 CT.
    (
        2026,
        10,
        3,
        &TEMPORARY_2026_10_03,
        "CME Globex notice 20260921"
    ),
    // 2026-10-04 — T1 — CME Globex notice 20260921 — restore the standard grid.
    (
        2026,
        10,
        4,
        &SOURCED_INTERSECTION,
        "CME Globex notice 20260921"
    ),
    // 2026-10-24 — T1 — CME Globex notice 20260921 — FIA drill to 15:30 CT.
    (
        2026,
        10,
        24,
        &TEMPORARY_2026_10_24,
        "CME Globex notice 20260921"
    ),
    // 2026-10-25 — T1 — CME Globex notice 20260921 — restore the standard grid.
    (
        2026,
        10,
        25,
        &SOURCED_INTERSECTION,
        "CME Globex notice 20260921"
    ),
    // 2026-10-31 — T1 — CME Globex notice 20261005 — maintenance to 05:00 CT.
    (
        2026,
        10,
        31,
        &TEMPORARY_2026_10_31,
        "CME Globex notice 20261005"
    ),
    // 2026-11-01 — T1 — CME Globex notice 20261005 — restore the standard grid.
    (
        2026,
        11,
        1,
        &SOURCED_INTERSECTION,
        "CME Globex notice 20261005"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, REVISIONS)
}
