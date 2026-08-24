// SPDX-License-Identifier: MIT-0

//! London Stock Exchange (SETS).

use chrono_tz::Europe;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// The 2009 compliance parameters and the September 2010 Millennium Exchange
// rehearsal timetable establish the Jan-2010 SETS baseline: pre-trading at
// 07:00, opening call at 07:50, continuous trading after the randomized 08:00
// uncross, and closing call from 16:30 to its latest 16:35:30 edge.
// https://docs.londonstockexchange.com/sites/default/files/documents/compliance_update_mar_09.pdf
// https://docs.londonstockexchange.com/sites/default/files/documents/live-001-300910-appendix-a.pdf
static BASE_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30,
    close_ssm: 16 * 3600 + 30 * 60,
}];
static BASE_EXTENDED: &[SessionRule] = &[
    // Opening auction call and its randomized uncross: the uncrossing prints
    // trades at the opening price, so the whole window stays tradeable.
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 50 * 60,
        close_ssm: 8 * 3600 + 30,
    },
    // Closing auction.
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 35 * 60 + 30,
    },
];
// Pre-trading is order-entry-only: MIT201 section 4.4 lists it as a scheduled
// trading session that precedes the opening auction call, separate from the
// executable phases of the order-book day (opening auction, regular trading,
// closing auction, and the closing price crossing session). No on-book
// execution can occur before the opening auction uncrosses.
// https://docs.londonstockexchange.com/sites/default/files/documents/mit201-guide-to-the-trading-system-15-6-20240429.pdf
static ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600,
    close_ssm: 7 * 3600 + 50 * 60,
}];
static BASE_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::London,
    regular: BASE_REGULAR,
    extended: BASE_EXTENDED,
    order_entry: ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// Notice N15/12's official attachment states that the closing-auction uncross
// starts at 16:35 and that CPX is the up-to-five-minute executable session
// immediately following it. MIT501 confirms that CPX was introduced in April
// 2012 with a default five-minute duration; the operator's service description
// gives its scheduled 16:35:01-16:40:00 grid. MIT201's operator-maintained
// document history records the production functional release on 2012-04-30.
// https://docs.londonstockexchange.com/sites/default/files/documents/n1512_attach1.pdf
// https://docs.londonstockexchange.com/sites/default/files/documents/mit501.pdf
// https://docs.londonstockexchange.com/sites/default/files/documents/servicetechnicaldescriptionintroductionofnewtradingcurrencies.pdf
// https://docs.londonstockexchange.com/sites/default/files/documents/mit201-guide-to-the-trading-system-15-6-20240429.pdf
static CPX_EXTENDED: &[SessionRule] = &[
    BASE_EXTENDED[0],
    // Closing auction.
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 35 * 60 + 30,
    },
    // Closing Price Crossing: MIT201 section 4.5 calls CPX "a short, modified
    // regular trading session" whose executions occur at the closing auction
    // price, so it is tradeable, not order entry.
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 35 * 60 + 30,
        close_ssm: 16 * 3600 + 40 * 60,
    },
];
static CPX_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::London,
    regular: BASE_REGULAR,
    extended: CPX_EXTENDED,
    order_entry: ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// Notice N01/16 made the SETS intraday auction effective on 2016-03-21. It
// starts at 12:00, runs for two minutes, and has a random end of up to 30
// seconds. Current technical parameters preserve that grid and CPX to 16:40.
// https://docs.londonstockexchange.com/sites/default/files/documents/n0116.pdf
// https://www.londonstockexchange.com/resources/equities-trading-resources?tab=technical-library
static CURRENT_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30,
        close_ssm: 12 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600 + 2 * 60 + 30,
        close_ssm: 16 * 3600 + 30 * 60,
    },
];
static CURRENT_EXTENDED: &[SessionRule] = &[
    BASE_EXTENDED[0],
    // Intraday auction: its uncross prints trades.
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600,
        close_ssm: 12 * 3600 + 2 * 60 + 30,
    },
    CPX_EXTENDED[1],
    CPX_EXTENDED[2],
];

pub(crate) static LSE_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::London,
    regular: CURRENT_REGULAR,
    extended: CURRENT_EXTENDED,
    order_entry: ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

static REVISIONS: &[Revision] = revisions![
    (2012, 4, 30, &CPX_PROFILE, "LSE MIT201 document history"),
    (2016, 3, 21, &LSE_PROFILE, "LSE notice N01/16"),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, Europe::London), &BASE_PROFILE, REVISIONS)
}
