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
//
// 2026-08-31 Sunday-queue review — bracket narrowed, notice channels negative.
// Three archived captures of CME's own trading-hours pages, unused by the
// earlier review, move the bracket from 2012-05-03..2012-06-15 down to
// 2012-05-28..2012-06-07. The move was platform-wide and simultaneous: on the
// 2012-05-28 capture the Sunday Pre-Open is 16:15 for E-mini S&P 500,
// Eurodollar, 30-Year Interest Rate Swap, Euroyen TIBOR and (as "17:15 ET
// (16:15 CT)") Gold, Silver, Light Sweet Crude and Henry Hub; on the
// 2012-06-07 capture every one of them reads 16:00. Weekday Pre-Opens are
// unchanged across both captures, so this is a Sunday-only change.
// CBOT grains are NOT part of it: the 2012-05-11 capture still shows the
// pre-expansion 18:00-07:15/09:30-13:15 grain grid with a 16:15 Sunday
// Pre-Open, and the 2012-05-28 capture shows the expanded 17:00-14:00 grid
// with 16:00 — so grains moved at the separately sourced 2012-05-20
// expansion (CME Globex Advisory #20120518), which the grains module already
// dates.
// Both of CME's dated notice channels were then read in full across the
// narrowed window and none announces the change: CME Globex Notices of
// 2012-05-21, 2012-05-28 and 2012-06-04, and Market Data Notices of
// 2012-05-28, contain no occurrence of "Pre-Open", "trading hours", "16:00"
// or "16:15". The change was therefore made without a dated operator notice,
// which is why no cutover is encoded. (The only Sunday inside the narrowed
// bracket is 2012-06-03; that is an observation about the bracket, not a
// source-stated effective day, so LAW-NO-FABRICATED-DATES keeps it out of the
// tables.) Official origin http://www.cmegroup.com/trading_hours/ delivered
// via:
// https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities
// https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html
// https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/
// https://web.archive.org/web/20190820012118id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120521.html
// https://web.archive.org/web/20190716070058id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120528.html
// https://web.archive.org/web/20190720204402id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120604.html
// https://web.archive.org/web/20120622070557id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120528.html
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
// SUNDAY QUEUE, CARRIED BACK AS THE SOURCED INTERSECTION. CME's Sunday Pre-Open
// only ever widened inside the modelled window: the audit-floor material pins it
// at 16:15 and the verified-current value is 16:00, with the undated 2012 move
// (bracketed 2012-05-28..2012-06-07) the only change between them. The
// 16:15-17:00 window is therefore order-entry under *every* sourced state, so
// carrying it from the January-2010 floor asserts no cutover at all - it is the
// intersection of the two regimes, not a guess at either. The undated change
// adds only the 16:00-16:15 quarter-hour, which the knowledge-bound row supplies
// from the repository review date onward. Previously these dated profiles
// omitted the Sunday queue entirely, which under-reported order acceptance for
// the whole 16:00-17:00 hour rather than only the disputed quarter-hour.
// The two profiles above already carry Sunday 16:15; this one dropped it purely
// because the 16:15->16:00 day is undated. It now keeps the same intersection.
static ORDER_ENTRY_DATED_2011_10_02: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600,
    },
];
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
    // Knowledge-bound row: only the disputed 16:00–16:15 quarter-hour depends
    // on the undated 2012 move. The 16:15–17:00 remainder is order-entry under
    // every sourced Sunday value and is already carried from the January-2010
    // floor by the dated profiles above, so this row widens the queue rather
    // than introducing it. A sourced onset day replaces this row.
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
