// SPDX-License-Identifier: MIT-0

//! Eurex benchmark-index futures and EEX Nordic Zonal Power Futures.
//!
//! Narrative evidence, sources and residual risks: `docs/evidence/eurex.md`,
//! `docs/evidence/eurex_key.md` and `docs/evidence/eex.md`
//! (LAW-EVIDENCE-FILES).

use chrono_tz::{Europe, UTC};

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{effective_date, local_date, reference_delta_seconds};

// The Eurex default is FESX/FDAX/FDXM benchmark index futures, not a venue-wide
// clock. The January-2010-to-cutover grid is 07:30-07:50 pre-trading then
// 07:50-22:00 continuous trading; from the 2018-12-10 cutover, 10 minutes of
// pre-trading and a five-minute opening auction precede continuous trading to
// 22:00. Pre-trading stays `order_entry`; the auction prints, so it is extended.
static EUREX_PRE_2018_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 50 * 60,
    close_ssm: 22 * 3600,
}];
// 07:30-07:50 pre-trading: order entry only, no matching.
static EUREX_PRE_2018_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 30 * 60,
    close_ssm: 7 * 3600 + 50 * 60,
}];
static EUREX_WINTER_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3600 + 15 * 60,
    close_ssm: 22 * 3600,
}];
// 01:00-01:10 CET pre-trading: order entry only.
static EUREX_WINTER_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3600,
    close_ssm: 3600 + 10 * 60,
}];
// 01:10-01:15 CET opening auction: a trade prints at the auction price, so this
// window is tradeable and stays `extended`.
static EUREX_WINTER_EXTENDED: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3600 + 10 * 60,
    close_ssm: 3600 + 15 * 60,
}];
pub(crate) static EUREX_CURRENT_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 2 * 3600 + 15 * 60,
    close_ssm: 22 * 3600,
}];
// 02:00-02:10 CEST pre-trading: order entry only.
pub(crate) static EUREX_CURRENT_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 2 * 3600,
    close_ssm: 2 * 3600 + 10 * 60,
}];
// 02:10-02:15 CEST opening auction: a trade prints at the auction price, so this
// window is tradeable and stays `extended`.
pub(crate) static EUREX_CURRENT_EXTENDED: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 2 * 3600 + 10 * 60,
    close_ssm: 2 * 3600 + 15 * 60,
}];

static EUREX_PRE_2018: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_PRE_2018_REGULAR,
    extended: &[],
    order_entry: EUREX_PRE_2018_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};
static EUREX_WINTER: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_WINTER_REGULAR,
    extended: EUREX_WINTER_EXTENDED,
    order_entry: EUREX_WINTER_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static EUREX_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_CURRENT_REGULAR,
    extended: EUREX_CURRENT_EXTENDED,
    order_entry: EUREX_CURRENT_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

const EUREX_ASIAN_HOURS: chrono::NaiveDate = effective_date(2018, 12, 10);

pub(crate) fn eurex_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    if local_date(as_of, Europe::Berlin) < EUREX_ASIAN_HOURS {
        return &EUREX_PRE_2018;
    }
    if reference_delta_seconds(as_of, Europe::Berlin, UTC) == -2 * 3600 {
        &EUREX_CURRENT
    } else {
        &EUREX_WINTER
    }
}

// EEX has no venue-wide grid; this default is Nordic Zonal Power Futures only,
// 08:00-18:00 CE(S)T Monday to Friday from their sourced 2024-03-25 launch.
static EEX_POWER_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 18 * 3600,
}];
pub(crate) static EEX_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EEX_POWER_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
static EEX_CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

const EEX_NORDIC_LAUNCH: chrono::NaiveDate = effective_date(2024, 3, 25);

pub(crate) fn eex_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    if local_date(as_of, Europe::Berlin) < EEX_NORDIC_LAUNCH {
        &EEX_CLOSED
    } else {
        &EEX_PROFILE
    }
}
