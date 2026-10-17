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

// The changeover of the two offset-grid books, positively witnessed in both
// directions by the operator's machine channel (`MARKET-DATA`, T2, retained
// privately): 17:03/18:03-minute ticks on every opening day through
// 2024-02-08, then the Sunday 2024-02-11-opened session quoting inside the
// earlier grid's break at its Monday close. Friday and Saturday open no
// session — docs/evidence/oanda_fx.md.
const FX_METALS_CURRENT_FIRST_OPEN_DAY: NaiveDate = effective_date(2024, 2, 11);

/// Selects the witnessed normal-week era for a New York-anchored book:
/// opening days before `first_current` answer from the earlier grid and
/// opening days from `first_current` answer from the current grid. An
/// opening day below the witnessed span still selects the earlier grid — a
/// session straddling the horizon resolves complete bounds while its trade
/// dates refuse through the coverage gate.
fn new_york_era_select(
    day: NaiveDate,
    first_current: NaiveDate,
    legacy: &'static StaticHoursProfile,
    current: &'static StaticHoursProfile,
) -> &'static StaticHoursProfile {
    if day >= first_current {
        current
    } else {
        legacy
    }
}

/// The same selection with a sourced-intersection span between the eras,
/// for the books whose changeover day carries no distinguishing witness
/// (the charter's 2026-10-04 rule).
fn new_york_era_select_intersected(
    day: NaiveDate,
    first_intersection: NaiveDate,
    first_current: NaiveDate,
    legacy: &'static StaticHoursProfile,
    intersection: &'static StaticHoursProfile,
    current: &'static StaticHoursProfile,
) -> &'static StaticHoursProfile {
    if day >= first_current {
        current
    } else if day >= first_intersection {
        intersection
    } else {
        legacy
    }
}

// The US continuous books' era bounds: their current grid is a strict
// subset of the whole-hour legacy grid, so no candle observation can
// positively prove the current state and the changeover stays undated —
// the bounds are the archived page-print observations, and the span
// between keeps the sourced intersection per the 2026-10-04 rule.
const US_INTERSECTION_FIRST_OPEN_DAY: NaiveDate = effective_date(2023, 6, 5);
const US_CURRENT_FIRST_OPEN_DAY: NaiveDate = effective_date(2024, 2, 20);

// Evidence: docs/evidence/oanda_fx.md
pub(crate) static OANDA_FX_REGULAR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3_600 + 5 * 60,
    close_ssm: 16 * 3_600 + 59 * 60,
}];

// The earlier `Sun-Fri: 17:03 - 16:58` grid, witnessed 2021-12-08..2023-06-05
// by the page prints and per day through the 2024-02-08 opening day by the
// operator's machine channel (each day carries a 17:03-minute tick, inside
// the current grid's break).
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
// 2023-06-05. The current grid nests inside it, so the undated changeover's
// sourced intersection is the current grid's own shape.
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
    new_york_era_select(
        day,
        FX_METALS_CURRENT_FIRST_OPEN_DAY,
        &OANDA_FX_LEGACY,
        &OANDA_FX,
    )
}

pub(crate) fn oanda_try_fx_profile_at(_as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    &OANDA_TRY_FX
}

pub(crate) fn oanda_metals_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, New_York);
    new_york_era_select(
        day,
        FX_METALS_CURRENT_FIRST_OPEN_DAY,
        &OANDA_METALS_LEGACY,
        &OANDA_METALS,
    )
}

pub(crate) fn oanda_us_index_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, New_York);
    // The current grid nests inside the legacy one, so the undated span's
    // sourced intersection is the current profile itself; the span is dated
    // and disclosed in the evidence file, not by a third profile.
    new_york_era_select_intersected(
        day,
        US_INTERSECTION_FIRST_OPEN_DAY,
        US_CURRENT_FIRST_OPEN_DAY,
        &OANDA_US_INDEX_LEGACY,
        &OANDA_US_INDEX,
        &OANDA_US_INDEX,
    )
}

pub(crate) fn oanda_us_commodity_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, New_York);
    new_york_era_select_intersected(
        day,
        US_INTERSECTION_FIRST_OPEN_DAY,
        US_CURRENT_FIRST_OPEN_DAY,
        &OANDA_US_COMMODITY_LEGACY,
        &OANDA_US_COMMODITY,
        &OANDA_US_COMMODITY,
    )
}

pub(crate) fn oanda_brent_profile_at(_as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    &OANDA_BRENT
}
