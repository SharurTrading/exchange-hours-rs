// SPDX-License-Identifier: MIT-0

//! Coinbase Derivatives recurring 23x5 futures default.

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_ONLY, MON_THU, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{effective_date, local_date};

// The venue default is CDE's recurring 23x5 futures grid: Sunday through
// Friday, 17:00-16:00 CT, with the daily 16:00-17:00 break. The four launch
// certifications filed 2021-06-04 state that grid, and every later dated
// filing through #2026-24 restates it, so no revision separates launch from
// today. Since #2026-24 (on or after trade date 2026-05-04) most CDE futures
// trade 24x7; this grid is the one copper, platinum, nano crude oil, natural
// gas and Mag7 + Crypto equity index futures retain. The 24x7 family needs its
// own product-family key at the caller.
// https://www.cftc.gov/filings/ptc/ptc060421lmxdcm001.pdf
// https://www.cftc.gov/filings/orgrules/rules0416261571.pdf
// https://docs.cdp.coinbase.com/derivatives/introduction/market-hours
static REGULAR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];
// ORDER ENTRY, NOT TRADING. Pre-Open quoting accepts orders for the coming
// session and nothing matches until the 17:00 open. The Pre-Open phase is
// documented from 2021 without a time; its 16:50 start is first witnessed in a
// 2025 capture of the market-hours page and, with no source naming a change,
// is carried back to launch.
static ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 16 * 3600 + 50 * 60,
    close_ssm: 17 * 3600,
}];

static PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: America::Chicago,
    regular: REGULAR,
    extended: &[],
    order_entry: ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// FairX, as CDE then traded, opened for trading on Monday 2021-06-28 at 09:00
// ET with no Sunday-evening session before it. The operator's homepage banner
// names the day and time; its captures from 2021-06-22 fix the year, and the
// 2021-08-02 capture reports the venue open. The launch-day profile starts
// that first session at the launch instant, so its bounds never reach back to
// a Sunday 17:00 that did not trade. The Monday-evening session that runs into
// the next day is identical in both profiles, so the day-level switch to the
// full grid never splits it.
// https://web.archive.org/web/20210622222253/https://www.fairx.com/
// https://web.archive.org/web/20210802230534/https://www.fairx.com/
static LAUNCH_DAY_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: MON_ONLY,
        open_ssm: 8 * 3600,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 16 * 3600,
    },
];
static LAUNCH_DAY_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 16 * 3600 + 50 * 60,
    close_ssm: 17 * 3600,
}];
static LAUNCH_DAY: StaticHoursProfile = StaticHoursProfile {
    tz: America::Chicago,
    regular: LAUNCH_DAY_REGULAR,
    extended: &[],
    order_entry: LAUNCH_DAY_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

static CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: America::Chicago,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// 2021-06-28 13:00:00 UTC, 08:00 CDT. An exact instant is required; this
// launch is not a venue-local-midnight revision.
const LAUNCH_UNIX_SECONDS: i64 = 1_624_885_200;
const FIRST_FULL_DAY: chrono::NaiveDate = effective_date(2021, 6, 29);

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    if as_of.timestamp() < LAUNCH_UNIX_SECONDS {
        &CLOSED
    } else if local_date(as_of, America::Chicago) < FIRST_FULL_DAY {
        &LAUNCH_DAY
    } else {
        &PROFILE
    }
}
