// SPDX-License-Identifier: MIT-0

//! CME Live Cattle, Feeder Cattle, and Lean Hog futures schedules.

use chrono_tz::US;

use crate::calendar::rule::{FRI, MON_FRI, MON_ONLY};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};
use crate::calendar::{FuturesSessionProfile, SessionRule};

const MON_WED: [bool; 7] = [true, true, true, false, false, false, false];
const THU_ONLY: [bool; 7] = [false, false, false, true, false, false, false];
const TUE_THU: [bool; 7] = [false, true, true, true, false, false, false];

// CME's 2007 launch announcement establishes the Monday 09:05 CT weekly open
// and 16:00-17:00 daily halts; Q2008-215 moved the Friday close to 13:55 before
// the audit floor, and the 2014 reduction report confirms that complete old
// grid. SER-7194 removed the evening sessions effective Monday 2014-10-27.
// SER-7591 then set the current 08:30-13:05 CT weekday session for LE, GF, and
// HE effective Monday 2016-02-29. CME's 30 May 2016 Globex notice implemented
// a Post-Close state — GTC/GTD order entry, modification, and cancellation for
// the next trade date with "No matching ... during the Post-Close" — Monday
// through Friday 14:30-16:00 CT for the same LE, GF, and HE families,
// effective Monday 2016-06-06. Official trading-hours captures omit the PCP
// row between November 2016 and March 2020 without any removal notice, so the
// omission is treated as a published-table gap rather than an operator-stated
// removal and the sourced onset stands. CME moved the Pre-Open start from
// 06:00 to 08:00 effective Sunday 2020-05-31 for trade date Monday 2020-06-01.
// 2026-08-31 review: the contract-specification channel was checked as a second
// route into the 2016-11..2020-03 interval and is silent too — the Live Cattle
// specification captured 2016-09-25, 2016-12-04, 2017-09-23 and 2018-04-19
// renders only "CME ClearPort" and "Default" hours (Monday-Friday 08:30-13:05
// CT) with no CME Globex Pre-Open or PCP row at all. Both the trading-hours and
// the specification channels therefore fail to carry the PCP through that
// interval, which corroborates the omission below rather than resolving it.
// The pre-2020 06:00 queue is now carried across 2016-02-29..2020-05-31 (see the
// note beside its rule set below); its onset before that grid is still
// unresolved, so the older around-the-clock profiles keep no queue. A generic 2010 Globex
// queue notice does not enumerate livestock, so it is not used to invent a
// family-specific afternoon queue in the old around-the-clock grid.
// https://www.cmegroup.com/media-room/press-releases/2007/3/07/cme_to_offer_around-the-clocktradingofcommodityproductsoncmeglob.html
// https://www.cmegroup.com/tools-information/lookups/advisories/market-data/Q2008-215.html
// https://www.cmegroup.com/tools-information/lookups/advisories/ser/files/SER-7194.pdf
// https://www.cmegroup.com/market-regulation/files/14-408.pdf
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
// https://www.cmegroup.com/tools-information/lookups/advisories/ser/files/SER-7591.pdf
// https://www.cmegroup.com/notices/electronic-trading/2016/05/20160530.html
// https://web.archive.org/web/20160605123512/http://www.cmegroup.com:80/notices/electronic-trading/2016/05/20160530.html
// https://www.cmegroup.com/notices/electronic-trading/2020/05/20200511.html
// https://www.cmegroup.com/notices/ser/2020/05/SER-8599R.pdf
// https://www.cmegroup.com/market-regulation/rule-filings/2020/5/20-232.pdf
// https://www.cmegroup.com/trading-hours/files/memorial-day-2023.pdf
// https://www.cmegroup.com/education/lessons/live-cattle-product-overview
static REGULAR_AT_2010_FLOOR: &[SessionRule] = &[
    SessionRule {
        days: MON_ONLY,
        open_ssm: 9 * 3600 + 5 * 60,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: MON_WED,
        open_ssm: 17 * 3600,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: THU_ONLY,
        open_ssm: 17 * 3600,
        close_ssm: 13 * 3600 + 55 * 60,
    },
];

static REGULAR_2014_10_27: &[SessionRule] = &[
    SessionRule {
        days: MON_ONLY,
        open_ssm: 9 * 3600 + 5 * 60,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: TUE_THU,
        open_ssm: 8 * 3600,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: FRI,
        open_ssm: 8 * 3600,
        close_ssm: 13 * 3600 + 55 * 60,
    },
];

static REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 5 * 60,
}];
// ORDER-ENTRY CLASSIFICATION. Both phases modelled after 2016-02-29 are
// non-matching. The comment above names 08:00-08:30 as the "Pre-Open" (its
// start moved from 06:00 on 2020-05-31) which queues orders until the 08:30
// regular open, and 14:30-16:00 as PCP, the post-close order-entry period that
// follows the 13:05 close. Neither can print a trade, so the family has no
// tradeable extended session at all: `extended` is empty and both phases are
// `order_entry`.
pub(crate) static ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 30 * 60,
        close_ssm: 16 * 3600,
    },
];

pub(crate) static CURRENT_FUTURES_PROFILE: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: REGULAR_CURRENT,
    extended: &[],
    order_entry: ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_AT_2010_FLOOR: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: REGULAR_AT_2010_FLOOR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_2014_10_27: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: REGULAR_2014_10_27,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// PRE-2020 MORNING QUEUE, CARRIED BACK TO THE MATCHING GRID IT BELONGS TO. The
// 2020 notice dates the move of the morning Pre-Open start "from 06:00 to
// 08:00" on 2020-05-31, so it states the outgoing 06:00 value the same way
// SER-6465 states CME's outgoing equity-index close. No primary source names a
// cutover between 2016-02-29 - when SER-7591 established the 08:30 open this
// queue runs into - and 2020-05-31, so 06:00-08:30 is carried across that
// interval rather than omitted. It is deliberately NOT carried further back:
// before 2016-02-29 the family ran the old around-the-clock grid with no 08:30
// open for a morning queue to precede, and the generic 2010 Globex queue notice
// does not enumerate livestock.
static ORDER_ENTRY_PRE_2020_MORNING: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 6 * 3600,
    close_ssm: 8 * 3600 + 30 * 60,
}];
static ORDER_ENTRY_PRE_2020_MORNING_AND_PCP: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 6 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 30 * 60,
        close_ssm: 16 * 3600,
    },
];

static PROFILE_2016_02_29: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: REGULAR_CURRENT,
    extended: &[],
    order_entry: ORDER_ENTRY_PRE_2020_MORNING,
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_2016_06_06: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: REGULAR_CURRENT,
    extended: &[],
    order_entry: ORDER_ENTRY_PRE_2020_MORNING_AND_PCP,
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: REGULAR_CURRENT,
    extended: &[],
    order_entry: ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static REVISIONS: &[Revision] = revisions![
    (2014, 10, 27, &PROFILE_2014_10_27, "CME SER-7194"),
    (2016, 2, 29, &PROFILE_2016_02_29, "CME SER-7591"),
    (
        2016,
        6,
        6,
        &PROFILE_2016_06_06,
        "CME Globex notice 20160530"
    ),
    (2020, 5, 31, &PROFILE_CURRENT, "CME SER-8599R"),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, US::Central),
        &PROFILE_AT_2010_FLOOR,
        REVISIONS,
    )
}
