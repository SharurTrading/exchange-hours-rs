// SPDX-License-Identifier: MIT-0

//! OANDA's own book: the FX (ex-TRY) grid, the TRY grid, the spot-metals
//! grid, the two US-anchored continuous grids, and the Brent grid.
//!
//! Narrative evidence, sources and residual risks: `docs/evidence/oanda*.md`
//! (LAW-EVIDENCE-FILES).

use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::America::New_York;
use chrono_tz::Europe::{Berlin, London};

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU, TUE_FRI};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{effective_date, local_date};

// The changeover to the current grid, dated by the operator's own machine
// channel (`MARKET-DATA`, T2, artifacts retained privately): every opening
// day of the former span witnessed per day, the last earlier-grid session
// opened Thursday 2024-02-08, the first current-grid session Sunday
// 2024-02-11. Opening-day keying never splits a running session
// (LAW-NO-FABRICATED-DATES) — docs/evidence/oanda_fx.md holds the sweep.
const NEW_YORK_CURRENT_FIRST_OPEN_DAY: NaiveDate = effective_date(2024, 2, 11);

/// Selects the witnessed normal-week era for a New York-anchored book:
/// opening days before 2024-02-11 answer from the earlier grid — witnessed
/// by the archived page prints through 2023-06-05 and per day by the
/// operator's machine channel through 2024-02-08 — and opening days from
/// 2024-02-11 answer from the current grid. An opening day below the
/// witnessed span still selects the earlier grid — a session straddling the
/// horizon resolves complete bounds while its trade dates refuse through the
/// coverage gate.
fn new_york_era_select(
    day: NaiveDate,
    legacy: &'static StaticHoursProfile,
    current: &'static StaticHoursProfile,
) -> &'static StaticHoursProfile {
    if day >= NEW_YORK_CURRENT_FIRST_OPEN_DAY {
        current
    } else {
        legacy
    }
}

// Evidence: docs/evidence/oanda_fx.md
pub(crate) static OANDA_FX_REGULAR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3_600 + 5 * 60,
    close_ssm: 16 * 3_600 + 59 * 60,
}];

// The earlier `Sun-Fri: 17:03 - 16:58` grid, witnessed 2021-12-08..2023-06-05
// with its five-minute 16:58-17:03 break.
// Evidence: docs/evidence/oanda_fx.md (`OANDA-UK-HOURS-2021-DEC-WAYBACK`)
pub(crate) static OANDA_FX_LEGACY_REGULAR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3_600 + 3 * 60,
    close_ssm: 16 * 3_600 + 58 * 60,
}];

static OANDA_FX: StaticHoursProfile = StaticHoursProfile {
    tz: New_York,
    regular: OANDA_FX_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static OANDA_FX_LEGACY: StaticHoursProfile = StaticHoursProfile {
    tz: New_York,
    regular: OANDA_FX_LEGACY_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// Evidence: docs/evidence/oanda_try_fx.md
const MON_FRI: [bool; 7] = [true, true, true, true, true, false, false];
pub(crate) static OANDA_TRY_FX_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3_600,
    close_ssm: 17 * 3_600,
}];

static OANDA_TRY_FX: StaticHoursProfile = StaticHoursProfile {
    tz: Berlin,
    regular: OANDA_TRY_FX_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// Evidence: docs/evidence/oanda_metals.md
pub(crate) static OANDA_METALS_REGULAR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 18 * 3_600 + 5 * 60,
    close_ssm: 16 * 3_600 + 59 * 60,
}];

// The earlier XAU/USD and XAG/USD grid `Sun-Fri: 18:03 - 16:58`, witnessed
// 2021-12-08..2023-06-05. The era's cross rows printed an 18:10 open; that
// per-instrument minute is disclosed beside the rows in the evidence file,
// like the NZD Auckland break.
// Evidence: docs/evidence/oanda_metals.md
pub(crate) static OANDA_METALS_LEGACY_REGULAR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 18 * 3_600 + 3 * 60,
    close_ssm: 16 * 3_600 + 58 * 60,
}];

static OANDA_METALS: StaticHoursProfile = StaticHoursProfile {
    tz: New_York,
    regular: OANDA_METALS_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static OANDA_METALS_LEGACY: StaticHoursProfile = StaticHoursProfile {
    tz: New_York,
    regular: OANDA_METALS_LEGACY_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// Evidence: docs/evidence/oanda_us_index_cfd.md, docs/evidence/oanda_us_bond_cfd.md, docs/evidence/oanda_us_commodity_cfd.md
pub(crate) static OANDA_US_CONTINUOUS_REGULAR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 18 * 3_600 + 60,
    close_ssm: 16 * 3_600 + 59 * 60,
}];

// The earlier whole-hour US clock `18:00 - 17:00` New York (Chicago-labelled
// rows print the same instants as `17:00 - 16:00`), witnessed from the
// 2021-08-04 index print and the 2021-03-03 bond/commodity prints through
// 2023-06-05 and per day by the operator's machine channel through the
// 2024-02-08 opening day.
// Evidence: docs/evidence/oanda_us_index_cfd.md
pub(crate) static OANDA_US_LEGACY_REGULAR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 18 * 3_600,
    close_ssm: 17 * 3_600,
}];

static OANDA_US_INDEX: StaticHoursProfile = StaticHoursProfile {
    tz: New_York,
    regular: OANDA_US_CONTINUOUS_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static OANDA_US_INDEX_LEGACY: StaticHoursProfile = StaticHoursProfile {
    tz: New_York,
    regular: OANDA_US_LEGACY_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static OANDA_US_COMMODITY: StaticHoursProfile = StaticHoursProfile {
    tz: New_York,
    regular: OANDA_US_CONTINUOUS_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static OANDA_US_COMMODITY_LEGACY: StaticHoursProfile = StaticHoursProfile {
    tz: New_York,
    regular: OANDA_US_LEGACY_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// Evidence: docs/evidence/oanda_brent.md
pub(crate) static OANDA_BRENT_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 23 * 3_600 + 60,
        close_ssm: 21 * 3_600 + 59 * 60,
    },
    SessionRule {
        days: TUE_FRI,
        open_ssm: 3_600 + 60,
        close_ssm: 21 * 3_600 + 59 * 60,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 22 * 3_600 + 5 * 60,
        close_ssm: 22 * 3_600 + 59 * 60,
    },
];

static OANDA_BRENT: StaticHoursProfile = StaticHoursProfile {
    tz: London,
    regular: OANDA_BRENT_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

pub(crate) fn oanda_fx_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, New_York);
    new_york_era_select(day, &OANDA_FX_LEGACY, &OANDA_FX)
}

pub(crate) fn oanda_try_fx_profile_at(_as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    &OANDA_TRY_FX
}

pub(crate) fn oanda_metals_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, New_York);
    new_york_era_select(day, &OANDA_METALS_LEGACY, &OANDA_METALS)
}

pub(crate) fn oanda_us_index_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, New_York);
    new_york_era_select(day, &OANDA_US_INDEX_LEGACY, &OANDA_US_INDEX)
}

pub(crate) fn oanda_us_commodity_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, New_York);
    new_york_era_select(day, &OANDA_US_COMMODITY_LEGACY, &OANDA_US_COMMODITY)
}

pub(crate) fn oanda_brent_profile_at(_as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    &OANDA_BRENT
}
