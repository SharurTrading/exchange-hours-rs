// SPDX-License-Identifier: MIT-0

//! OANDA's own book: the FX (ex-TRY) grid, the TRY grid, the spot-metals
//! grid, the two US-anchored continuous grids, and the Brent grid.
//!
//! Narrative evidence, sources and residual risks: `docs/evidence/oanda*.md`
//! (LAW-EVIDENCE-FILES).

use chrono::{DateTime, Utc};
use chrono_tz::America::New_York;
use chrono_tz::Europe::{Berlin, London};

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;

// The FX book's one printed grid: Sunday 17:05 through Friday 16:59 New York
// time, the six-minute 16:59-17:05 daily break, and no other maintenance. The
// NZD-pair Auckland break is caller catalog data, not a rule in this grid.
// Evidence: docs/evidence/oanda_fx.md
pub(crate) static OANDA_FX_REGULAR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3_600 + 5 * 60,
    close_ssm: 16 * 3_600 + 59 * 60,
}];

static OANDA_FX: StaticHoursProfile = StaticHoursProfile {
    tz: New_York,
    regular: OANDA_FX_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// The TRY book prints one daily span, Monday-Friday 08:00-17:00 Central
// European time, with an overnight gap; "Central Europe" anchors to CET/CEST
// and Europe/Berlin is the crate's CET anchor.
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

// The spot-metals book (XAU/XAG and their crosses): Sunday 18:05 through
// Friday 16:59 New York time with no daily maintenance. The NZD-pair Auckland
// break is caller catalog data, not a rule in this grid.
// Evidence: docs/evidence/oanda_metals.md
pub(crate) static OANDA_METALS_REGULAR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 18 * 3_600 + 5 * 60,
    close_ssm: 16 * 3_600 + 59 * 60,
}];

static OANDA_METALS: StaticHoursProfile = StaticHoursProfile {
    tz: New_York,
    regular: OANDA_METALS_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// The US-anchored books print one clock in two timezones: Chicago 17:01-15:59
// (indices/bonds) is New York 18:01-16:59 (energies) to the instant; the
// printed pair's daily gap and the two-key split are the holiday page's.
// Evidence: docs/evidence/oanda_us_index_cfd.md, docs/evidence/oanda_us_commodity_cfd.md
pub(crate) static OANDA_US_CONTINUOUS_REGULAR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 18 * 3_600 + 60,
    close_ssm: 16 * 3_600 + 59 * 60,
}];

static OANDA_US_INDEX: StaticHoursProfile = StaticHoursProfile {
    tz: New_York,
    regular: OANDA_US_CONTINUOUS_REGULAR,
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

// The Brent book, anchored to London per the page's own dagger note: opens
// Sunday 23:01, closes Friday 21:59, with the six-minute 21:59-22:05 break
// and the two-hour 22:59-01:01 break Monday-Thursday.
// Evidence: docs/evidence/oanda_brent.md
pub(crate) static OANDA_BRENT_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 23 * 3_600 + 60,
        close_ssm: 21 * 3_600 + 59 * 60,
    },
    SessionRule {
        days: MON_FRI,
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

// One sourced grid each, no dated change and no seasonal switch; the
// evidence files record the carry below the 2024-02-21 witness and what
// closes it.
pub(crate) fn oanda_fx_profile_at(_as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    &OANDA_FX
}

pub(crate) fn oanda_try_fx_profile_at(_as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    &OANDA_TRY_FX
}

pub(crate) fn oanda_metals_profile_at(_as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    &OANDA_METALS
}

pub(crate) fn oanda_us_index_profile_at(_as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    &OANDA_US_INDEX
}

pub(crate) fn oanda_us_commodity_profile_at(_as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    &OANDA_US_COMMODITY
}

pub(crate) fn oanda_brent_profile_at(_as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    &OANDA_BRENT
}
