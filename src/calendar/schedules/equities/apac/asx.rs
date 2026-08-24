// SPDX-License-Identifier: MIT-0

//! Australian Securities Exchange cash equities.

use chrono_tz::Australia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// ASX cash market, from ASX Operating Rules Procedures Appendix 4013 and the
// cash-market hours page. Service Release 15 replaced symbol-group staggered
// opens with one randomized 09:59:45–10:00 opening and added Post Close on
// 2025-06-23. A deterministic venue default conservatively starts continuous
// trading at 10:00; the opening process and close-side trading are extended.
// Sources:
// https://www.asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours
// https://www.asxonline.com/public/notices/2025/may/0473.25.05.html
static ASX_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 10 * 3600,
    close_ssm: 16 * 3600,
}];

// Tradeable close-side and open-side windows. The ASX phase timetable prints
// Pre-open 07:00:00–09:59:00, Opening Single Price Auction 09:59:00–09:59:45,
// Open (Normal Trading) 09:59:45–16:00:00, Pre-CSPA 16:00:00–16:10:00, Closing
// Single Price Auction 16:10:00–16:11:00 and Post Close 16:11:00–16:21:30. Both
// single-price auctions match, and in Post Close "ASX matches orders at the CSPA
// price", so the auction and Post Close windows are tradeable; the deterministic
// venue default still defers continuous trading to 10:00, so 09:59–10:00 stays
// extended rather than regular.
static ASX_EXTENDED_CURRENT: &[SessionRule] = &[
    // Pre-open: ASX Trade does not match here, but overnight and overseas
    // trades report until 09:45 and other allowable trades may be reported
    // under the Operating Rules — reported trades print, so a price can occur
    // and the window is tradeable extended, never order-entry.
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600,
        close_ssm: 9 * 3600 + 59 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 59 * 60,
        close_ssm: 10 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 10 * 60,
        close_ssm: 16 * 3600 + 21 * 60 + 30,
    },
];

// Order entry only: the one window in which no trade can print is Pre-CSPA
// 16:00–16:10, the phase in which "continuous matching ceases" and only entry
// and amendment are accepted, ahead of the 16:10 CSPA uncrossing. Pre-open is
// not here: ASX Trade does not match in it, but reported overnight, overseas
// and allowable trades print, so it stays tradeable in `extended`.
static ASX_ORDER_ENTRY_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 16 * 3600,
    close_ssm: 16 * 3600 + 10 * 60,
}];

// Before Service Release 15, five symbol groups opened at nominal times from
// 10:00 through 10:09, each randomized by +/- 15 seconds, and the CSPA ended
// at 16:12. Regular starts at the venue's earliest continuous-trading edge;
// the overlapping extended rule preserves the opening-auction envelope through
// the latest possible Group 5 transition at 10:09:15.
// Source: ASX SR15 marked operating-rule procedure amendments:
// https://www.asxonline.com/content/dam/asxonline/public/notices/2025/april/asx-sr15asx-operating-rule-procedure-amendments.pdf
// The earliest possible print of the old staggered open is the Group 1
// transition at 10:00 less its 15-second randomization, i.e. 09:59:45; the
// latest is the Group 5 transition at 10:09:15. That envelope and the 16:10
// CSPA through its old 16:12 end are tradeable.
static ASX_EXTENDED_PRE_2025_06_23: &[SessionRule] = &[
    // Pre-open: trade reporting printed here in this era too. SR15 changed the
    // staggered open and added Post Close; it did not change reporting rules.
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600,
        close_ssm: 9 * 3600 + 59 * 60 + 45,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 59 * 60 + 45,
        close_ssm: 10 * 3600 + 9 * 60 + 15,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 10 * 60,
        close_ssm: 16 * 3600 + 12 * 60,
    },
];

// Order entry only in the pre-SR15 era: Pre-CSPA 16:00–16:10, with continuous
// matching ceased and only entry and amendment accepted. Pre-open stayed
// tradeable for the same reported-trades reason as the current era.
static ASX_ORDER_ENTRY_PRE_2025_06_23: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 16 * 3600,
    close_ssm: 16 * 3600 + 10 * 60,
}];

pub(crate) static ASX_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Australia::Sydney,
    regular: ASX_REGULAR,
    extended: ASX_EXTENDED_CURRENT,
    order_entry: ASX_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

pub(crate) static ASX_PROFILE_PRE_2025_06_23: StaticHoursProfile = StaticHoursProfile {
    tz: Australia::Sydney,
    regular: ASX_REGULAR,
    extended: ASX_EXTENDED_PRE_2025_06_23,
    order_entry: ASX_ORDER_ENTRY_PRE_2025_06_23,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &ASX_PROFILE_CURRENT;

static REVISIONS: &[Revision] = revisions![(
    2025,
    6,
    23,
    &ASX_PROFILE_CURRENT,
    "ASX SR15 notice 0473.25.05"
),];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &ASX_PROFILE_PRE_2025_06_23,
        REVISIONS,
    )
}
