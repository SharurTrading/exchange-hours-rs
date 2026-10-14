// SPDX-License-Identifier: MIT-0

//! MEFF IBEX 35 index-futures family (with Mini and Micro IBEX 35), Madrid.

use chrono_tz::Europe;

use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// The January-2010 floor grid: the operator's own product page, captured
// 2010-06-18, prints "AUCTION PERIOD From 8:30 a.m. to 9:00 a.m. TRADING
// HOURS From 9:00 a.m. to 5:35 p.m.". The 30-minute opening auction accepts
// orders and matches nothing until the 09:00 open, so it is `order_entry`.
// Narrative: docs/evidence/bme_ibex.md
static BME_IBEX_2010_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 17 * 3600 + 35 * 60,
}];
static BME_IBEX_2010_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 9 * 3600,
}];
static BME_IBEX_2010: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Madrid,
    regular: BME_IBEX_2010_REGULAR,
    extended: &[],
    order_entry: BME_IBEX_2010_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// The 08:00-open era: the operator's own press release of 2017-01-23 states
// "Starting today ... Trading then starts at 8:00 with a 5-minute auction
// from 7:55 h to 8:00 h ... Trading Period IBEX 35 and Mini IBEX Futures
// 8:00 - 20:00 h", and the operator's archived Trading Hours page of
// 2017-11-23 prints the same grid. The auction is `order_entry` like 2010's.
// Narrative: docs/evidence/bme_ibex.md
static BME_IBEX_2017_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 20 * 3600,
}];
static BME_IBEX_2017_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 55 * 60,
    close_ssm: 8 * 3600,
}];
static BME_IBEX_2017: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Madrid,
    regular: BME_IBEX_2017_REGULAR,
    extended: &[],
    order_entry: BME_IBEX_2017_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// The current grid: notice MEFF 16/2025 of 2 April 2025 states "on 23 June,
// the trading hours for IBEX 35 Futures, Mini IBEX Futures, and Micro Futures
// on the electronic order system will be extended until 10:00 p.m." and
// prints the electronic order system row "7:55 - 8:00 | 8:00 - 22:00"; the
// operator's live Trading Hours and product pages print the same grid.

// The notice's "additional random period of 30 seconds maximum" extends the
// opening-auction uncross past 08:00; the crate keeps the operator's nominal
// 08:00 trading-period boundary (the charter's boundary convention).
// Narrative: docs/evidence/bme_ibex.md
pub(crate) static BME_IBEX_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 22 * 3600,
}];
pub(crate) static BME_IBEX_ORDER_ENTRY_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 55 * 60,
    close_ssm: 8 * 3600,
}];
pub(crate) static BME_IBEX_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Madrid,
    regular: BME_IBEX_REGULAR_CURRENT,
    extended: &[],
    order_entry: BME_IBEX_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Evidence: docs/evidence/bme_ibex.md
static BME_IBEX_REVISIONS: &[Revision] = revisions![
    (
        2017,
        1,
        23,
        &BME_IBEX_2017,
        "MEFF extends trading in IBEX 35 and Mini IBEX futures by an hour"
    ),
    (2025, 6, 23, &BME_IBEX_CURRENT, "MEFF notice 16/2025"),
];

/// Selects the IBEX 35 futures profile in force at `as_of` (Madrid local day).
pub(crate) fn bme_ibex_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Europe::Madrid),
        &BME_IBEX_2010,
        BME_IBEX_REVISIONS,
    )
}
