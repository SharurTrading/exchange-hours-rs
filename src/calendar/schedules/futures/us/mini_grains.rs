// SPDX-License-Identifier: MIT-0

//! CBOT mini-sized grain and oilseed futures schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

use super::grains::{
    CBOT_EXTENDED_AT_2010_FLOOR, CBOT_EXTENDED_CURRENT, CBOT_ORDER_ENTRY_2013_04_07,
    CBOT_ORDER_ENTRY_CURRENT,
};

// CBOT mini-sized grain and oilseed futures in America/Chicago: Mini-Sized
// Corn (CME Globex `XC`, CBOT Rulebook chapter 10B), Mini-Sized Soybean
// (`XK`, chapter 11B), Mini-Sized Wheat (`XW`, chapter 14B), and Mini-Sized
// KC HRW Wheat (`MKC`, chapter 14N as of 2022). ONE family on ONE grid: the
// 2012 expansion notice names Mini-Sized Corn/Soybeans/Wheat together, the
// 22 March 2013 Global Command Center notice lists "XC Mini-Sized Corn
// Futures", "XW Mini-Sized Wheat Futures" and "XK Mini-Sized Soybean
// Futures" with a single mini-specific day close, SER-9049's amendment table
// gives all four contracts identical current and amended hours, and no
// retrieved document between 2010 and 2026 gives any two of them different
// hours. Excluded: the standard-size ZC/ZS/ZW/KE grain and oilseed contracts
// (`grains.rs`), Rough Rice (`rough_rice.rs`), and the Micro Ag futures
// MZC/MZS/MZW, which launched February 2022 on the standard grid.
//
// WHY THIS IS NOT FOLDABLE INTO `globex_grains`. CME built the minis with a
// deliberate, permanent 30-minute-later day close so positions could be
// offset after the standard settlement — CFTC filing 13-092 footnote 1,
// "To maintain the traditional mini- to full-sized trading hours, both
// electronic and floor hours for CBOT Mini-Sized Corn, Soybean, and Wheat
// futures will close daily at 1:45 p.m. CT", and SER-7044, "because
// mini-sized products have slightly longer trading hours and because mini-
// and standard-sized products allow for offsets, the mini-sized products
// continue to be a popular tool for adjusting positions at the end of each
// trading day". The envelopes coincide only 2012-05-20..2012-09-15 and from
// 2022-10-02; between those windows the standard grain key serves a day
// close 25 or 30 minutes too early for the minis. The minis skipped
// SER-7395R's 2015-07-05 move to 13:20 entirely — the 2022 notice's CURRENT
// column still reads 1:45 p.m. — so their revision list is NOT a copy of the
// grains list: they have a 2012-09-16 revision the standard grains do not,
// no 2015-07-05 revision, and a 2022-10-02 revision the standard grains do
// not.
//
// `MKC` FOLLOWS THE MINI GRID DESPITE ITS KC-WHEAT LINEAGE. On captures
// taken the same day, 5 September 2015, the KC HRW Wheat spec reads
// "Monday – Friday, 8:30 a.m. – 1:20 p.m. CT" while the Mini-Sized KC HRW
// Wheat spec reads "Monday - Friday, 8:30 a.m. - 1:45 p.m. CT"; SER-7044
// launched MKC with "the same contract specifications - ... trading hours
// ... - as the existing CBOT Mini-Sized Corn, Soybean, and Chicago SRW Wheat
// futures". MKC joined on Sunday 2014-03-23 (trade date Monday 2014-03-24)
// on the then-current mini grid; that is a member listing, not a grid
// change, so it creates no revision row here — launch dates remain
// caller-catalog data.
// https://web.archive.org/web/20190718051357id_/https://www.cmegroup.com/tools-information/lookups/advisories/ser/SER-7044.html
// https://web.archive.org/web/20150905115535id_/http://www.cmegroup.com/trading/agricultural/grain-and-oilseed/mini-sized-kc-hrw-wheat_contract_specifications.html
// https://web.archive.org/web/20150905192450id_/http://www.cmegroup.com/trading/agricultural/grain-and-oilseed/kc-wheat_contract_specifications.html
// https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul031313cbot001.pdf
//
// THE JANUARY-2010 FLOOR. Day matching 09:30-13:45 around the 18:00-07:15
// overnight leg. The 13:45 close is stated by Globex notice 20100405's
// mini-specific block — "CBOT Mini-sized grain futures / Trading ends at
// 1:45 p.m. Central time (CT)" inside a change that "will not affect the end
// of trading times" — by the August-2010 contract specs for all three minis
// ("9:30 am - 1:45 pm Central Time, Sunday - Friday"), and retrospectively by
// 13-092's "traditional mini- to full-sized trading hours". CME's own
// trading-hours tables of February-April 2010 show 13:15 in the mini rows'
// Globex column against a 13:45 open-outcry column; those cells are judged
// copied from the standard-grain row in error — a page whose pit column says
// 13:45 cannot have an electronic column closing thirty minutes earlier for
// a product CME describes as electronic-and-floor — and by 24 August 2010
// both cells read 13:45. No 13:15<->13:45 transition is dated and none is
// asserted. Floor queues (Sunday 16:15-18:00, weekday 07:15-09:30, PCP
// 14:30-16:00) come from Globex notice 20100315's category line "Current
// Pre-Open for CBOT, KCBT and MGEX Grain Futures", which does not enumerate
// the minis; the first product-specific enumeration is CME's trading-hours
// table of 27 September 2011, with the same values. The mini rows there also
// list a standalone 16:45 weekday pre-open slot through the 25 January 2012
// capture; no notice dates its onset, so it is not modeled, matching the
// standard grains' treatment of the same slot.
// https://web.archive.org/web/20190718173802id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100405.html
// https://web.archive.org/web/20100329154209id_/http://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100315.html
// https://web.archive.org/web/20100824064602id_/http://www.cmegroup.com/trading/agricultural/grain-and-oilseed/mini-sized-corn_contract_specifications.html
// https://web.archive.org/web/20110927011113id_/http://www.cmegroup.com/trading_hours/commodities-hours.html
//
// 2010-04-19 moves the afternoon post-close pre-open to begin 30 seconds
// after the mini day close — "CBOT Mini-sized grain futures ... Post-close
// pre-open begins at 1:45.30 CT", "GTC and GTD orders may be entered,
// modified and cancelled 1:45.30 - 4:00 p.m. CT" — the same notice that
// moved the standard grains' PCP to 13:15:30, carrying a separate
// mini-specific block. 2011-12-27 moves the Globex morning pre-opening start
// from 07:15 to 08:00 for CBOT agricultural futures (CFTC filing
// rul120711cbot001); CME's trading-hours table of 25 January 2012 corroborates
// the mini row with "Pre-Open Weekday 14:30-16:00 / 16:45 / 08:00". Both
// change only order-entry boundaries; matching is unchanged.
// https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul120711cbot001.pdf
// https://web.archive.org/web/20120125164824id_/http://www.cmegroup.com/trading_hours/commodities-hours.html
//
// 2012-05-20 joins the standard grains' 21-hour continuous session:
// "Effective this Sunday, May 20 (trade date Monday, May 21), the electronic
// trading hours on CME Globex for all CBOT Commodity, KCBT, and MGEX Grain
// and Oilseed futures and options will be expanded to ... Sunday to Friday:
// 17:00 CT to 14:00 Central Time (CT)". The regular slice stays the
// open-outcry window, which CME's tables show at 09:30-13:45 for the minis
// through this era. The advisory states only matching hours, never queue
// times, and CME's trading-hours pages bracket the queue switch inside
// 2012-05-05..2012-06-15 without proving it, so this era conservatively
// serves no order-entry phases — exactly the standard grains' treatment of
// the same undated switch.
// https://web.archive.org/web/20190716055756id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html
// https://web.archive.org/web/20120616192606id_/http://www.cmegroup.com/trading/agricultural/grain-and-oilseed/mini-sized-corn_contract_specifications.html
//
// 2012-09-16 IS THE DIVERGENCE: the minis alone extend to 14:30 while the
// standard grains stay at 14:00. "Expanded CBOT Mini-Sized Grain Futures
// Trading Hour Change / Effective Sunday, September 16 (trade date Monday,
// September 17) ... Sunday to Friday: 17:00 CT to 14:30 Central Time (CT) /
// Pause State: 14:30 (CT) / Close State: 14:35 (CT) / Post Close/Pre Open
// (PCP)14:40 (CT) / Please note: During PCP only GT orders will be allowed
// for these products." The PCP end is not in the advisory; CME's
// trading-hours page captured 15 September 2012 — the day before go-live —
// shows the mini rows at "14:40- 16:00, 16:45 - 17:00". Only the PCP is
// modeled from this row: the Sunday and evening queue slots the same page
// shows have no stated onset day, and GT-only acceptance is still order
// entry, the classification the crate already gives the standard grains'
// GT-order PCP of 2010-04-19. The pit window's move to 09:30-14:30 appears
// only in that table, not in the advisory, so the 13:45-14:30 afternoon
// slice stays classified extended; the open/closed envelope is the same
// either way inside the continuous electronic session.
// https://web.archive.org/web/20190716055549id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120904.html
// https://web.archive.org/web/20120915064618id_/http://www.cmegroup.com/trading_hours/commodities-hours.html
//
// 2013-04-07 is the great reduction, and it restores the named mini premium:
// "Sunday-Friday: 19:00 to 07:45 CT / Monday-Friday: Break from 07:45 to
// 08:30 CT / Monday-Friday: 08:30 to 13:15 CT, Mini-Sized Grains: 08:30 to
// 13:45 CT" (Global Command Center notice of 22 March 2013, whose
// impacted-product list names XC/XW/XK; certified in parallel by CBOT
// Submission 13-092, effective "Sunday, April 7, 2013 for the Monday, April
// 8, 2013 trading date"). The same notice establishes the queue set:
// "Pre-Opens (including MGEX): Sunday night: 16:00-19:00 CT / Monday-Thursday
// night: 16:45-19:00 CT / Monday-Friday morning: 08:15-08:30 CT. Post Close
// Pre-Open: Monday-Friday: 14:30-16:00 CT", plus a 07:45-08:15
// cancellation-only slice inside the break that no order-entry rule models —
// identical values to the standard grains' queues, which `grains.rs` encodes
// from the same notice.
// https://web.archive.org/web/20130423023212id_/http://www.cmegroup.com/globex/files/cmegroup_reduced_grain_and_oilseed_hours.pdf
// https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul031313cbot001.pdf
//
// 2013-08-18 widens the morning Pre-Open to 08:00-08:30: "Effective Sunday,
// August 18, 2013 (trade date Monday, August 19), the Pre-Open market hours
// will be expanded to: Monday – Friday, 08:00 Central Time (CT) to 08:30 CT
// for the following products: CBOT Grain and Oilseed futures and options
// ...", with "The PCP state will remain unchanged Monday – Friday, 14:30 CT
// to 16:00 CT". SER-9049's Pre-Open column later lists the same three
// product-specific pre-open windows for all four minis as "(unchanged)".
// https://web.archive.org/web/20190822053114id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20130812.html
//
// 2022-10-02 IS THE CONVERGENCE, and it ends the premium: "Effective Sunday,
// October 2 (trade date Monday, October 3), the trading hours for the
// following CBOT mini-sized agriculture futures will be amended" from
// "Sunday - Friday, 7:00 p.m. - 7:45 a.m. CT and Monday - Friday, 8:30 a.m. -
// 1:45 p.m. CT" to "Sunday - Friday, 7:00 p.m. - 7:45 a.m. CT and Monday -
// Friday, 8:30 a.m. - 1:20 p.m. CT", one table row per product for XC, MKC,
// XK and XW. SER-9049 (2022-09-01) is the dated CME report behind it and
// states the intent outright: "The Amendments to the CME Globex hours of the
// Contracts will align the trading hours of the Contracts with the
// corresponding standard sized agricultural futures contracts", with
// ClearPort and Pre-Open hours unchanged. From this day the mini grid equals
// the standard grain grid in every phase, so the current tables below are
// the standard grains' tables under mini names. Corroborated by CME's Micro
// Agricultural futures FAQ, captured 11 April 2026: "On CME Globex, trades
// may be entered on: Sunday – Friday: 7:00 p.m. – 7:45 a.m. CT and Monday –
// Friday: 8:30 a.m. – 1:20 p.m. CT". A stale CME fact card still carrying
// the pre-2022 "Mini-sized contracts close at 1:45 p.m. CT" note (capture of
// 28 March 2025) is superseded by both dated documents.
// https://www.cmegroup.com/notices/ser/2022/09/SER-9049.pdf
// https://web.archive.org/web/20260212002829id_/https://www.cmegroup.com/notices/electronic-trading/2022/09/20220905.html
// https://web.archive.org/web/20221022130435id_/https://www.cmegroup.com/notices/electronic-trading/2022/09/20220912.html
// https://web.archive.org/web/20260411073852id_/https://www.cmegroup.com/articles/faqs/faq-micro-agriculture-futures.html
// https://web.archive.org/web/20250328101242id_/https://www.cmegroup.com/trading/agricultural/files/grain-and-oilseed-futures-options-fact-card.pdf
//
// RTH/ETH CLASSIFICATION. No CME document states a literal
// regular-versus-extended label for these products in any era. The split
// follows the operator's own published columns: before 2013 the pages
// separate "Open Outcry" from "Electronic Trading", and the pit window
// (09:30-13:45, then 08:30-13:45 per 13-092 footnote 1's "both electronic
// and floor hours") is regular; from the 2013 reduction the overnight leg is
// extended and the 08:30 day session regular, exactly the standard grains'
// reviewed encoding. The current tables are shared with `grains.rs` because
// the 2022 convergence makes the grids genuinely identical, not because one
// envelope was assumed to transfer — every pre-2022 era here is keyed to the
// minis' own dated sources.

static MINI_REGULAR_0930_1345: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 45 * 60,
}];
static MINI_REGULAR_0830_1345: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 45 * 60,
}];

// ORDER-ENTRY CLASSIFICATION. Only the matching windows can print a trade, so
// the Sunday evening queue, the weekday morning queue (07:15, then 08:00 from
// 2011-12-27, briefly 08:15 from 2013-04-07, back to 08:00 from 2013-08-18,
// up to the day-session open), and the afternoon PCP are `order_entry`; the
// overnight leg and the continuous session's afternoon slice stay `extended`.
static MINI_ORDER_ENTRY_AT_2010_FLOOR: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 15 * 60,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 30 * 60,
        close_ssm: 16 * 3600,
    },
];
// 2010-04-19: the PCP re-anchors to 30 seconds after the mini 13:45 close.
// The matching grid is unchanged, so the revision reuses
// `CBOT_EXTENDED_AT_2010_FLOOR`.
static MINI_ORDER_ENTRY_2010_04_19: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 15 * 60,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 45 * 60 + 30,
        close_ssm: 16 * 3600,
    },
];
// 2011-12-27: the morning queue's start moves to 08:00.
static MINI_ORDER_ENTRY_2011_12_27: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 45 * 60 + 30,
        close_ssm: 16 * 3600,
    },
];
// 2012-09-16: the only queue this era's advisory dates is the PCP, from
// 14:40 to the 16:00 end CME's same-day trading-hours page shows.
static MINI_ORDER_ENTRY_2012_09_16: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 14 * 3600 + 40 * 60,
    close_ssm: 16 * 3600,
}];

// The 21-hour continuous regimes. The wrap reaches the 09:30 day open and the
// afternoon slice resumes at the 13:45 mini day open, so the envelope runs
// 17:00 to 14:00 (then 14:30) continuously while the open-outcry window
// inside it stays regular.
static MINI_EXTENDED_2012_05_20: &[SessionRule] = &[
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 45 * 60,
        close_ssm: 14 * 3600,
    },
];
static MINI_EXTENDED_2012_09_16: &[SessionRule] = &[
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 45 * 60,
        close_ssm: 14 * 3600 + 30 * 60,
    },
];

/// The overnight leg SER-9049 and the 2022 notices carry unchanged:
/// Sunday-Friday 19:00-07:45 CT, wrapping local midnight.
pub(crate) use super::grains::CBOT_EXTENDED_CURRENT as MINI_EXTENDED_CURRENT;
/// The queue set SER-9049 lists as "(unchanged)" for all four minis: Sunday
/// 16:00-19:00, Monday-Friday 08:00-08:30, Monday-Friday 14:30-16:00, and
/// Monday-Thursday 16:45-19:00 CT.
pub(crate) use super::grains::CBOT_ORDER_ENTRY_CURRENT as MINI_ORDER_ENTRY_CURRENT;
/// From the 2022-10-02 convergence the mini day session is the standard
/// grains' 08:30-13:20 CT window, stated for all four products by SER-9049
/// and the October-2022 Globex notices. The table is shared with `grains.rs`
/// because the grids genuinely coincide from that day.
pub(crate) use super::grains::CBOT_REGULAR_CURRENT as MINI_REGULAR_CURRENT;

const fn profile(
    regular: &'static [SessionRule],
    extended: &'static [SessionRule],
    order_entry: &'static [SessionRule],
) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: US::Central,
        regular,
        extended,
        order_entry,
        has_daily_close: true,
        has_weekend_close: true,
    }
}

static AT_2010_FLOOR: StaticHoursProfile = profile(
    MINI_REGULAR_0930_1345,
    CBOT_EXTENDED_AT_2010_FLOOR,
    MINI_ORDER_ENTRY_AT_2010_FLOOR,
);
static FROM_2010_04_19: StaticHoursProfile = profile(
    MINI_REGULAR_0930_1345,
    CBOT_EXTENDED_AT_2010_FLOOR,
    MINI_ORDER_ENTRY_2010_04_19,
);
static FROM_2011_12_27: StaticHoursProfile = profile(
    MINI_REGULAR_0930_1345,
    CBOT_EXTENDED_AT_2010_FLOOR,
    MINI_ORDER_ENTRY_2011_12_27,
);
static FROM_2012_05_20: StaticHoursProfile =
    profile(MINI_REGULAR_0930_1345, MINI_EXTENDED_2012_05_20, &[]);
static FROM_2012_09_16: StaticHoursProfile = profile(
    MINI_REGULAR_0930_1345,
    MINI_EXTENDED_2012_09_16,
    MINI_ORDER_ENTRY_2012_09_16,
);
static FROM_2013_04_07: StaticHoursProfile = profile(
    MINI_REGULAR_0830_1345,
    CBOT_EXTENDED_CURRENT,
    CBOT_ORDER_ENTRY_2013_04_07,
);
static FROM_2013_08_18: StaticHoursProfile = profile(
    MINI_REGULAR_0830_1345,
    CBOT_EXTENDED_CURRENT,
    CBOT_ORDER_ENTRY_CURRENT,
);
static DATED_CURRENT: StaticHoursProfile = profile(
    MINI_REGULAR_CURRENT,
    MINI_EXTENDED_CURRENT,
    MINI_ORDER_ENTRY_CURRENT,
);

// Revision evidence — each row's day-level effective date and the primary
// source that states it (full quotations sit in the blocks above):
//   2010-04-19 "CME Globex notice 20100405, mini-sized block"
//     https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100405.html
//   2011-12-27 "CFTC filing rul120711cbot001"
//     https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul120711cbot001.pdf
//   2012-05-20 "CME market-data advisory 20120518"
//     https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html
//   2012-09-16 "CME market-data advisory 20120904"
//     https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120904.html
//   2013-04-07 "CME GCC notice 2013-03-22 and CBOT Submission 13-092"
//     https://www.cmegroup.com/globex/files/cmegroup_reduced_grain_and_oilseed_hours.pdf
//     https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul031313cbot001.pdf
//   2013-08-18 "CME market-data advisory 20130812"
//     https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20130812.html
//   2022-10-02 "CME SER-9049 and Globex notice 20220905"
//     https://www.cmegroup.com/notices/ser/2022/09/SER-9049.pdf
//     https://www.cmegroup.com/notices/electronic-trading/2022/09/20220905.html
static REVISIONS: &[Revision] = revisions![
    (
        2010,
        4,
        19,
        &FROM_2010_04_19,
        "CME Globex notice 20100405, mini-sized block"
    ),
    (
        2011,
        12,
        27,
        &FROM_2011_12_27,
        "CFTC filing rul120711cbot001"
    ),
    (
        2012,
        5,
        20,
        &FROM_2012_05_20,
        "CME market-data advisory 20120518"
    ),
    (
        2012,
        9,
        16,
        &FROM_2012_09_16,
        "CME market-data advisory 20120904"
    ),
    (
        2013,
        4,
        7,
        &FROM_2013_04_07,
        "CME GCC notice 2013-03-22 and CBOT Submission 13-092"
    ),
    (
        2013,
        8,
        18,
        &FROM_2013_08_18,
        "CME market-data advisory 20130812"
    ),
    (
        2022,
        10,
        2,
        &DATED_CURRENT,
        "CME SER-9049 and Globex notice 20220905"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &AT_2010_FLOOR, REVISIONS)
}
