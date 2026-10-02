// SPDX-License-Identifier: MIT-0

//! Euronext Dublin principal shares and the predecessor ISE Xetra book.

use chrono_tz::Europe;

use super::super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// ISE's own archived trading-hours page establishes the complete legacy grid before the January-2010 audit floor: pre-trading 06:30-07:50, opening auction to 08:00, continuous trading to 16:28, closing auction to 16:30, and post-trading through 17:15. Narrative:
// docs/evidence/euronext_dublin.md.
static ISE_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 16 * 3600 + 28 * 60,
}];
// Tradeability classification. Narrative:
// docs/evidence/euronext_dublin.md.
static ISE_EXTENDED: &[SessionRule] = &[
    // Pre-trading.
    SessionRule {
        days: MON_FRI,
        open_ssm: 6 * 3600 + 30 * 60,
        close_ssm: 7 * 3600 + 50 * 60,
    },
    // Opening auction.
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 50 * 60,
        close_ssm: 8 * 3600,
    },
    // Closing auction.
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 28 * 60,
        close_ssm: 16 * 3600 + 30 * 60,
    },
    // Post-trading.
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 15 * 60,
    },
];
static ISE_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Dublin,
    regular: ISE_REGULAR,
    extended: ISE_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// Euronext migrated Dublin equities to Optiq on 2019-02-04. Narrative:
// docs/evidence/euronext_dublin.md.
static OPTIQ_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30,
    close_ssm: 16 * 3600 + 28 * 60,
}];
// Order-entry classification. Optiq pre-opening is a Call (order-accumulation)
// phase: orders are collected and the first order-book print of the day is the
// opening uncrossing, which the trading appendix randomizes over the 30 seconds
// from 08:00:00 Dublin local time. Only the accumulation leg moves; the
// uncross, the closing uncrossing and Trading-at-Last all print and stay in
// `extended`.
static OPTIQ_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 6 * 3600 + 15 * 60,
    close_ssm: 8 * 3600,
}];
static OPTIQ_EXTENDED: &[SessionRule] = &[
    // Opening uncrossing, including its latest 30-second random uncross.
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 8 * 3600 + 30,
    },
    // Closing auction.
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 28 * 60,
        close_ssm: 16 * 3600 + 30 * 60 + 30,
    },
    // Trading-at-Last.
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 30 * 60 + 30,
        close_ssm: 16 * 3600 + 40 * 60,
    },
];
static OPTIQ_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Dublin,
    regular: OPTIQ_REGULAR,
    extended: OPTIQ_EXTENDED,
    order_entry: OPTIQ_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// Euronext's phase-one timetable shifted legacy-market pre-opening effective 2023-03-20 from 07:15 to 07:30 CET, or 06:15 to 06:30 Dublin local time. Narrative:
// docs/evidence/euronext_dublin.md.
static CURRENT_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 6 * 3600 + 30 * 60,
    close_ssm: 8 * 3600,
}];
static CURRENT_EXTENDED: &[SessionRule] = &[
    // Opening uncrossing, including its latest 30-second random uncross.
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 8 * 3600 + 30,
    },
    // Closing auction, including its latest 30-second random uncross.
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 28 * 60,
        close_ssm: 16 * 3600 + 30 * 60 + 30,
    },
    // Trading-at-Last.
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 30 * 60 + 30,
        close_ssm: 16 * 3600 + 40 * 60,
    },
];
pub(crate) static EURONEXT_DUB_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Dublin,
    regular: OPTIQ_REGULAR,
    extended: CURRENT_EXTENDED,
    order_entry: CURRENT_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// Evidence: docs/evidence/euronext_dublin.md
static REVISIONS: &[Revision] = revisions![
    (
        2019,
        2,
        4,
        &OPTIQ_PROFILE,
        "Euronext Dublin Optiq migration press release"
    ),
    (
        2023,
        3,
        20,
        &EURONEXT_DUB_PROFILE,
        "Euronext Go-Live Weekend Guidelines"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, Europe::Dublin), &ISE_PROFILE, REVISIONS)
}
