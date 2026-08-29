// SPDX-License-Identifier: MIT-0

//! CBOT U.S. Treasury/Fed Funds and CME SOFR futures schedules.

use chrono_tz::US;

use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};
use crate::calendar::{FuturesSessionProfile, SessionRule};

// The January-2008 CBOT migration notice establishes the 17:30-16:00 CT
// schedule inherited by the January-2010 audit-floor Treasury and 30-Day Fed
// Funds family. CME moved every legacy CBOT interest-rate open to 17:00 CT
// effective Sunday 2011-10-02 (trade date Monday 2011-10-03), aligning the
// family with the current 17:00-16:00 grid. The 2009 table also pins the
// audit-floor queues at Sunday 16:15 and weekdays 16:50; CME moved the weekday
// queue to 16:45 on 2010-11-15. Current material publishes a Sunday 16:00
// queue, but no primary source states the exact day on which 16:15 moved to
// 16:00: the holiday workbook updated 2012-05-03 still schedules every
// complex's Sunday pre-opening at 16:15, the interest-rate hours page crawled
// 2012-06-16 already shows 16:00, and no notice in between states the day.
// The fixed-current profile includes the exact current queue. The dated
// selector retains the sourced audit-floor queue, then omits only that Sunday
// phase after the exact 2011 matching-open revision rather than inventing a
// queue cutover.
//
// SOFR joined this already-live
// family in May 2018; individual contract launch dates remain catalog facts,
// not separate revisions of this product-neutral family clock.
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20080121.html
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20090326.html
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20110926.html
// https://www.cmegroup.com/notices/electronic-trading/2018/04/20180409.html
// https://www.cmegroup.com/trading/interest-rates/files/us-treasury-futures-delivery-process.pdf
// https://www.cmegroup.com/markets/interest-rates/stirs/30-day-federal-fund.contractSpecs.html
// https://www.cmegroup.com/education/articles-and-reports/understanding-sofr-futures
// https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf
// https://web.archive.org/web/20120505161543/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-memorial-day.xls
// https://web.archive.org/web/20120616195651/http://www.cmegroup.com:80/trading_hours/interest-rates-hours.html
// ORDER-ENTRY CLASSIFICATION. The comment above calls the Sunday 16:15 (later
// 16:00) and weekday 16:50 (later 16:45) phases "queues": Globex accepts,
// amends, and cancels orders in them while the matching engine is stopped, and
// nothing can print until the 17:30 (later 17:00) open. They are `order_entry`.
// The matching windows below stay in `extended`.
static EXTENDED_1730_1600: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600 + 30 * 60,
    close_ssm: 16 * 3600,
}];
static EXTENDED_1700_1600: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];
static ORDER_ENTRY_AT_2010_FLOOR: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 50 * 60,
        close_ssm: 17 * 3600 + 30 * 60,
    },
];
static ORDER_ENTRY_2010_11_15: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600 + 30 * 60,
    },
];
static ORDER_ENTRY_DATED_2011_10_02: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 16 * 3600 + 45 * 60,
    close_ssm: 17 * 3600,
}];
pub(crate) static ORDER_ENTRY_CURRENT: &[SessionRule] = &[
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

pub(crate) static CURRENT_FUTURES_PROFILE: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_1700_1600,
    order_entry: ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_AT_2010_FLOOR: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_1730_1600,
    order_entry: ORDER_ENTRY_AT_2010_FLOOR,
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_2010_11_15: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_1730_1600,
    order_entry: ORDER_ENTRY_2010_11_15,
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_2011_10_02: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_1700_1600,
    order_entry: ORDER_ENTRY_DATED_2011_10_02,
    has_daily_close: true,
    has_weekend_close: true,
};

// Verified-current grid: identical to PROFILE_2011_10_02 except the Sunday
// 16:00–17:00 queue, whose onset day no reviewed primary source states.
static PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_1700_1600,
    order_entry: ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static REVISIONS: &[Revision] = revisions![
    (
        2010,
        11,
        15,
        &PROFILE_2010_11_15,
        "CME Globex notice 20101025"
    ),
    (
        2011,
        10,
        2,
        &PROFILE_2011_10_02,
        "CME Globex notice 20110926"
    ),
    // Knowledge-bound row: the Sunday 16:00–17:00 queue is primary-verified
    // in the current envelope, but no reviewed source states its onset day,
    // so earlier dated queries conservatively omit it. From the 2026-08-22
    // repository review onward the verified-current grid applies; a sourced
    // onset day replaces this row.
    (
        2026,
        8,
        22,
        &PROFILE_CURRENT,
        "2026-08-22 review: verified current, onset undated"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, US::Central),
        &PROFILE_AT_2010_FLOOR,
        REVISIONS,
    )
}
