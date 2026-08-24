// SPDX-License-Identifier: MIT-0

//! National Stock Exchange of India cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static REGULAR_0955_1530: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 55 * 60,
    close_ssm: 15 * 3600 + 30 * 60,
}];
static REGULAR_0900_1530: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 15 * 3600 + 30 * 60,
}];
static REGULAR_0915_1530: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 15 * 60,
    close_ssm: 15 * 3600 + 30 * 60,
}];

static NSE_EXTENDED_PRE_2010_10_18: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 15 * 3600 + 50 * 60,
    close_ssm: 16 * 3600,
}];

// The 09:00–09:15 pre-open is two phases, not one. NSE's own pre-open page
// states the session "is comprised of Order collection period and order
// matching period", that "the order collection period of 8* minutes shall be
// provided for order entry, modification and cancellation (* - System driven
// random closure between 7th and 8th minute)", and that "order matching period
// starts immediately after completion of order collection period". BSE's
// operating notice for the same 2010-10-18 launch prints the grid outright:
// Order Entry Period 9:00am–9:07/08am with "No trades are executed", Order
// Matching & Confirmation Period 9:08am–9:12am, Buffer Period 9:12am–9:15am.
//
// Only the collection phase is `order_entry`. Because the collection phase is
// cut by a random stoppage anywhere in the 7th–8th minute, the boundary is set
// at the earliest second a trade could print (09:07), never later; 09:07–09:15
// stays `extended` so the auction match, its trade confirmations, and the
// transition buffer remain tradeable. The same grid is still current after the
// 2026-08-03 CAS cutover, so every post-2010-10-18 profile shares it.
// https://www.nseindia.com/static/products-services/equity-market-pre-open
// https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars?page=20101014-8
static INDIA_ORDER_ENTRY_PREOPEN: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 9 * 3600 + 7 * 60,
}];
static INDIA_PREOPEN_MATCH: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 7 * 60,
    close_ssm: 9 * 3600 + 15 * 60,
};

static NSE_EXTENDED_POST_2010_10_18: &[SessionRule] = &[
    INDIA_PREOPEN_MATCH,
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 50 * 60,
        close_ssm: 16 * 3600,
    },
];
static INDIA_EXTENDED_PRE_CAS: &[SessionRule] = &[
    INDIA_PREOPEN_MATCH,
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 40 * 60,
        close_ssm: 16 * 3600,
    },
];
static INDIA_EXTENDED_CURRENT: &[SessionRule] = &[
    INDIA_PREOPEN_MATCH,
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 15 * 60,
        close_ssm: 15 * 3600 + 35 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 50 * 60,
        close_ssm: 16 * 3600,
    },
];
// Current NSE/BSE venue envelope. Effective 2026-08-03, derivative-eligible
// cash stocks enter CAS at 15:15 while non-CAS stocks continue normally to
// 15:30; the overlapping regular/extended rules preserve both venue-wide
// states. CAS ends 15:35, transition runs to 15:50, and post-close ends 16:00.
// Sources:
// https://www.nseindia.com/static/products-services/closing-auction-session
// https://www.sebi.gov.in/legal/circulars/jan-2026/introduction-of-closing-auction-session-cas-in-the-equity-cash-segment-and-certain-modifications-in-the-pre-open-auction-session_99122.html
// https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars?page=20260801-1
//
// Both close-side windows stay `extended`. CAS 15:15–15:35 contains
// order-entry-only sub-phases for CAS-eligible stocks, but BSE's detailed
// operating guidelines also state that securities not eligible for CAS "shall
// continue to be available for continuous trading till 3:30pm", so trades print
// venue-wide throughout; the CAS itself then matches 15:30–15:35. The
// 15:50–16:00 post-close is a fixed-price session in which trades execute.
// https://www.bseindia.com/downloads/UploadDocs/Notices/20260610-41/20260610-41.pdf
pub(crate) static NSE_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Kolkata,
    regular: REGULAR_0915_1530,
    extended: INDIA_EXTENDED_CURRENT,
    order_entry: INDIA_ORDER_ENTRY_PREOPEN,
    has_daily_close: true,
    has_weekend_close: true,
};
// NSE introduced pre-open 09:00–09:15 on 2010-10-18, then moved its
// post-close start from 15:50 to 15:40 on 2011-10-03.
// Sources: NSE circular NSE/CMTR/15981 and NSE/CMTR/19013.
// https://nsearchives.nseindia.com/global/content/about_us/NSEIL_Annual_Report_2011.pdf
// https://nsearchives.nseindia.com/content/circulars/cmtr19013.pdf
pub(crate) static NSE_PROFILE_POST_2011_10_03: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Kolkata,
    regular: REGULAR_0915_1530,
    extended: INDIA_EXTENDED_PRE_CAS,
    order_entry: INDIA_ORDER_ENTRY_PREOPEN,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static NSE_PROFILE_POST_2010_10_18: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Kolkata,
    regular: REGULAR_0915_1530,
    extended: NSE_EXTENDED_POST_2010_10_18,
    order_entry: INDIA_ORDER_ENTRY_PREOPEN,
    has_daily_close: true,
    has_weekend_close: true,
};

// NSE and BSE jointly moved the continuous open 09:55 -> 09:00 on
// 2010-01-04. BSE's notice is 20091217-15; NSE's official release follows.
// https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars?page=20091217-15
// https://nsearchives.nseindia.com/content/press/17122009.htm
//
// No pre-open existed before 2010-10-18, so these two profiles have nothing to
// classify as order entry: their only extended window is the 15:50–16:00
// post-close, a fixed-price session in which trades execute.
pub(crate) static NSE_PROFILE_POST_2010_01_04: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Kolkata,
    regular: REGULAR_0900_1530,
    extended: NSE_EXTENDED_PRE_2010_10_18,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static NSE_PROFILE_PRE_2010_01_04: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Kolkata,
    regular: REGULAR_0955_1530,
    extended: NSE_EXTENDED_PRE_2010_10_18,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &NSE_PROFILE_CURRENT;

static REVISIONS: &[Revision] = revisions![
    (
        2010,
        1,
        4,
        &NSE_PROFILE_POST_2010_01_04,
        "NSE press release 17122009"
    ),
    (
        2010,
        10,
        18,
        &NSE_PROFILE_POST_2010_10_18,
        "NSE circular NSE/CMTR/15981"
    ),
    (
        2011,
        10,
        3,
        &NSE_PROFILE_POST_2011_10_03,
        "NSE circular NSE/CMTR/19013"
    ),
    (2026, 8, 3, &NSE_PROFILE_CURRENT, "SEBI circular 99122"),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &NSE_PROFILE_PRE_2010_01_04,
        REVISIONS,
    )
}
