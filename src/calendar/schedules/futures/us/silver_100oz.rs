// SPDX-License-Identifier: MIT-0

//! COMEX 100-Ounce Silver futures (`SIL`, Globex security group 4S).

use chrono_tz::US;

use super::energy_metals::{
    ENERGY_METALS_AT_2010_FLOOR, ENERGY_METALS_CURRENT, ENERGY_METALS_DATED_CURRENT,
};
use crate::calendar::SessionRule;
use crate::calendar::rule::{FRI, MON_FRI, SUN_ONLY};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

const SAT_ONLY: [bool; 7] = [false, false, false, false, false, true, false];
const THU_ONLY: [bool; 7] = [false, false, false, true, false, false, false];

// COMEX 100-Ounce Silver futures in America/Chicago. The product rode the
// shared NYMEX/COMEX energy-and-metals grid from the January-2010 floor; CME
// Globex notice 20260907 expands it to 24/7 trading "Effective this Friday,
// September 11", the first weekend leg opening 16:30 CT behind a one-day
// 30-minute maintenance extension, so this key carries the product's whole
// life. Narrative: docs/evidence/globex_silver_100oz.md

// The 24/7 executable week: Monday-Friday 00:00-16:00 and 16:02-24:00 CT
// behind the operator's two-minute daily maintenance, Saturday 00:00-02:00 and
// 04:00-24:00 CT around the two-hour weekly window, all of Sunday. No source
// for this product states a Pre-Open queue in the 24/7 era, so none is modelled;
// the maintenance windows are `Maintenance`, not order entry.
pub(crate) static SILVER_100OZ_EXTENDED_CURRENT: &[SessionRule] = &[
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

// The 2026-09-11 bridge day. Thursday's leg still opens 17:00 CT under the old
// grid and closes Friday 16:00; the first weekend leg then opens 16:30 CT,
// because that one Friday's standard two-minute maintenance is extended to
// thirty minutes. Keyed by opening day, so the running session never splits.
static EXTENDED_2026_09_11: &[SessionRule] = &[
    SessionRule {
        days: THU_ONLY,
        open_ssm: 17 * 3600,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: FRI,
        open_ssm: 16 * 3600 + 30 * 60,
        close_ssm: 24 * 3600,
    },
];

// One-day Saturday extensions that govern this product: notice 20260824
// extends the window for the channel table including MDP channel 329 — the
// channel the same notice moves `SIL` onto — so Saturday 2026-09-19 reopens
// 08:00 CT; notice 20260921 extends it for the 24/7 markets on 2026-10-03
// (to 05:00 CT) and 2026-10-24 (to 15:30 CT, the FIA drill), each reverting
// to standard. The August extensions predate the go-live and never governed it.
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
saturday_extended_to!(EXTENDED_2026_09_19, 8, 0);
saturday_extended_to!(EXTENDED_2026_10_03, 5, 0);
saturday_extended_to!(EXTENDED_2026_10_24, 15, 30);

static TRANSITION_2026_09_11: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_2026_09_11,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: false,
};

static SILVER_24_7: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: SILVER_100OZ_EXTENDED_CURRENT,
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
temporary_saturday!(TEMPORARY_2026_09_19, EXTENDED_2026_09_19);
temporary_saturday!(TEMPORARY_2026_10_03, EXTENDED_2026_10_03);
temporary_saturday!(TEMPORARY_2026_10_24, EXTENDED_2026_10_24);

// The pre-cutover rows restate the family rows this product rode, by reference
// to `energy_metals.rs`'s own tables: notice 20150907 moved every COMEX close
// to 16:00 CT, `SIL` included, and the 2026-08-22 knowledge-bound row is the
// same review. The September 19 row is confirmed effective by the operator's
// T2 service (2026-09-27 UTC); the October rows are forward-dated.
// Evidence: docs/evidence/globex_silver_100oz.md
static REVISIONS: &[Revision] = revisions![
    (
        2015,
        9,
        20,
        &ENERGY_METALS_DATED_CURRENT,
        "CME Globex notice 20150907"
    ),
    (
        2026,
        8,
        22,
        &ENERGY_METALS_CURRENT,
        "2026-08-22 review: verified current, onset undated"
    ),
    (
        2026,
        9,
        11,
        &TRANSITION_2026_09_11,
        "CME Globex notice 20260907"
    ),
    (
        2026,
        9,
        12,
        &SILVER_24_7,
        "CME Globex notice 20260907"
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
        &SILVER_24_7,
        "CME Globex notice 20260824"
    ),
    (
        2026,
        10,
        3,
        &TEMPORARY_2026_10_03,
        "CME Globex notice 20260921"
    ),
    (
        2026,
        10,
        4,
        &SILVER_24_7,
        "CME Globex notice 20260921"
    ),
    (
        2026,
        10,
        24,
        &TEMPORARY_2026_10_24,
        "CME Globex notice 20260921"
    ),
    (
        2026,
        10,
        25,
        &SILVER_24_7,
        "CME Globex notice 20260921"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, US::Central),
        &ENERGY_METALS_AT_2010_FLOOR,
        REVISIONS,
    )
}
