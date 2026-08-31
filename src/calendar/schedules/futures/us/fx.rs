// SPDX-License-Identifier: MIT-0

//! CME FX futures on the standard 17:00-16:00 CT Globex grid.

use chrono_tz::US;

use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};
use crate::calendar::{FuturesSessionProfile, SessionRule};

// CME's 2010 guide publishes the 17:00-16:00 matching grid for its standard FX
// futures. This family is not a promise for eFix, BTIC, TAS, options, or any
// product whose own specification publishes a different grid. The exact
// Monday-Thursday Pre-Open changed from 16:50 to 16:45 on 2010-11-15. Current
// primary material publishes Sunday 16:00-17:00, but calls it a long-term
// practice without stating the day on which the earlier queue moved; primary
// documents updated 2012-05-03 still publish Sunday 16:15 while pages crawled
// 2012-06-15/16 already publish 16:00, and no notice in between states the
// day. The fixed-current profile includes that exact current phase; dated
// profiles carry the sourced Sunday 16:15–17:00 intersection from the
// January-2010 floor and withhold only the disputed 16:00–16:15 quarter-hour.
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20081229.html
// https://www.cmegroup.com/trading/fx/files/FX248-2010_FX_Product_Guide_and_Calendar.pdf
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
// https://www.cmegroup.com/trading/fx/fx-report/files/q1-2018-cme-fx-products.pdf
// https://www.cmegroup.com/trading/fx/files/emfx-brochure-q3-2020.pdf
// https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf
// https://www.cmegroup.com/articles/faqs/frequently-asked-questions-cme-fx-futures-calendar-spreads.html
// https://web.archive.org/web/20120503103452/http://www.cmegroup.com/trading_hours/fx-hours.html
// https://web.archive.org/web/20120616190153/http://www.cmegroup.com/trading_hours/fx-hours.html
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
// ORDER-ENTRY CLASSIFICATION. The 17:00-16:00 window is the matching grid the
// 2010 product guide publishes. Every other phase here is a Globex queue: the
// weekday "Pre-Open" the comment above names (16:50, then 16:45, to 17:00) and
// the Sunday 16:00-17:00 queue accept, amend, and cancel orders while the
// matching engine is stopped, so no trade can print until 17:00. They are
// `order_entry`.
static MATCHING_GRID: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];
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
static ORDER_ENTRY_AT_2010_FLOOR: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 50 * 60,
        close_ssm: 17 * 3600,
    },
];
static ORDER_ENTRY_DATED_CURRENT: &[SessionRule] = &[
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
    extended: MATCHING_GRID,
    order_entry: ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static AT_2010_FLOOR: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: MATCHING_GRID,
    order_entry: ORDER_ENTRY_AT_2010_FLOOR,
    has_daily_close: true,
    has_weekend_close: true,
};
static DATED_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: MATCHING_GRID,
    order_entry: ORDER_ENTRY_DATED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};
// Verified-current grid: identical to DATED_CURRENT except the Sunday
// 16:00–17:00 queue, whose onset day no reviewed primary source states.
static FX_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: MATCHING_GRID,
    order_entry: ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static REVISIONS: &[Revision] = revisions![
    (2010, 11, 15, &DATED_CURRENT, "CME Globex notice 20101025"),
    // Knowledge-bound row: only the disputed 16:00–16:15 quarter-hour depends
    // on the undated 2012 move. The 16:15–17:00 remainder is order-entry under
    // every sourced Sunday value and is already carried from the January-2010
    // floor by the dated profiles above, so this row widens the queue rather
    // than introducing it. A sourced onset day replaces this row.
    (
        2026,
        8,
        22,
        &FX_CURRENT,
        "2026-08-22 review: verified current, onset undated"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &AT_2010_FLOOR, REVISIONS)
}
