// SPDX-License-Identifier: MIT-0

//! Euronext Milan principal shares and the predecessor Borsa Italiana MTA.

use chrono_tz::Europe;

use super::super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// Borsa Italiana changed the MTA opening effective 2009-09-29. The resulting
// Jan-2010 baseline starts pre-opening at 08:00, uncrosses randomly from
// 09:00-09:01, trades continuously through 17:25, and closes its randomized
// auction by 17:31.
// https://www.borsaitaliana.it/borsaitaliana/ufficio-stampa/comunicati-stampa/2009/090908nuoviorari.htm
static BASE_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 60,
    close_ssm: 17 * 3600 + 25 * 60,
}];
// Order-entry classification. The 08:00-09:00 leg is the pre-auction
// order-accumulation phase of the opening auction: orders are collected and no
// contract is concluded until the randomized uncross, which the same source
// places in 09:00-09:01. Only the accumulation leg moves to `order_entry`; the
// 09:00-09:01 uncross itself prints and stays in `extended`.
static BASE_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 9 * 3600,
}];
static BASE_EXTENDED: &[SessionRule] = &[
    // Randomized opening uncross.
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 9 * 3600 + 60,
    },
    // Closing auction.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 25 * 60,
        close_ssm: 17 * 3600 + 31 * 60,
    },
];
static BASE_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Rome,
    regular: BASE_REGULAR,
    extended: BASE_EXTENDED,
    order_entry: BASE_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// Borsa Italiana launched the executable Closing Price Cross phase on Monday
// 2013-09-30, extending the closing envelope through 17:40.
// https://www.borsaitaliana.it/azioni/notiziedettaglio/cpx.en.htm
static CPX_EXTENDED: &[SessionRule] = &[
    // Randomized opening uncross.
    BASE_EXTENDED[0],
    // Closing auction.
    BASE_EXTENDED[1],
    // Closing Price Cross.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 31 * 60,
        close_ssm: 17 * 3600 + 40 * 60,
    },
];
static CPX_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Rome,
    regular: BASE_REGULAR,
    extended: CPX_EXTENDED,
    order_entry: BASE_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// Notice 17016, effective 2015-11-23, moved the continuous close to 17:30,
// the closing auction's latest uncross to 17:35:59, and CPX end to 17:42.
// https://www.borsaitaliana.it/borsaitaliana/regolamenti/avvisi/17016orarineg.pdf
static REGULAR_2015: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 60,
    close_ssm: 17 * 3600 + 30 * 60,
}];
static EXTENDED_2015: &[SessionRule] = &[
    // Randomized opening uncross.
    BASE_EXTENDED[0],
    // Closing auction, including the full one-minute random period.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 36 * 60,
    },
    // Closing Price Cross.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 36 * 60,
        close_ssm: 17 * 3600 + 42 * 60,
    },
];
static PROFILE_2015: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Rome,
    regular: REGULAR_2015,
    extended: EXTENDED_2015,
    order_entry: BASE_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// Milan equities migrated to Euronext Optiq phase one on 2023-03-27. Current
// principal shares pre-open at 07:30, open by the latest 09:00:30 uncross,
// trade continuously to 17:30, and remain executable through 17:40.
// https://connect.euronext.com/sites/default/files/it-documentation/Guide%20to%20Trading%20System%20-%20Borsa%20Italiana%20Migration%20to%20Optiq%20-%20Functional%20Changes%20v.2.0.pdf
// https://connect.euronext.com/sites/default/files/it-documentation/Go-Live%20Weekend%20Guidelines%20-%20Borsa%20Italiana%20Optiq%20Migration.pdf
// https://www.euronext.com/sites/default/files/2026-07/appendix%20to%20Euronext%20Instructions%204-01%204-03%20Trading%20Manuals_0.xlsx
static CURRENT_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30,
    close_ssm: 17 * 3600 + 30 * 60,
}];
// Order-entry classification. Post-Optiq Milan follows the Euronext Call
// (order-accumulation) model: the 07:30 pre-opening collects orders and the
// trading appendix states the opening uncrossing "will randomly occur between
// CET 09:00:00 and 09:00:30". Nothing matches on the central order book before
// 09:00, so 07:30-09:00 is order entry only.
static CURRENT_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 30 * 60,
    close_ssm: 9 * 3600,
}];
static CURRENT_EXTENDED: &[SessionRule] = &[
    // Randomized opening uncross.
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 9 * 3600 + 30,
    },
    // Closing auction, including its latest 30-second random uncross.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 35 * 60 + 30,
    },
    // Trading-at-Last.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 35 * 60 + 30,
        close_ssm: 17 * 3600 + 40 * 60,
    },
];
pub(crate) static EURONEXT_MIL_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Rome,
    regular: CURRENT_REGULAR,
    extended: CURRENT_EXTENDED,
    order_entry: CURRENT_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

static REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2013, 9, 30),
        profile: &CPX_PROFILE,
    },
    Revision {
        effective: effective_date(2015, 11, 23),
        profile: &PROFILE_2015,
    },
    Revision {
        effective: effective_date(2023, 3, 27),
        profile: &EURONEXT_MIL_PROFILE,
    },
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, Europe::Rome), &BASE_PROFILE, REVISIONS)
}
