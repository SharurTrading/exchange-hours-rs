// SPDX-License-Identifier: MIT-0

//! Euronext Dublin principal shares and the predecessor ISE Xetra book.

use chrono_tz::Europe;

use super::super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// ISE's Release 16 Market Model gives the legacy order-book day: pre-trading
// 06:30-07:50, opening auction to 08:00, continuous trading to 16:28, and a
// closing auction to 16:30. Post-trading then allowed new order entry and
// off-book trades through 17:15, so it is `extended` under the crate's
// order-entry-only convention. The Central Bank's official assessment
// independently records the same 07:50-16:30 order-book envelope.
// https://service.betterregulation.com/sites/default/files/upload/2017-04/ISE%20Xetra%20Release%2016%20Market%20Model.pdf
// https://www.centralbank.ie/docs/default-source/tns/about---tns/peer-reviews-and-reports/tns-1-11-imf-report-on-observance-of-standards-and-codes-on-securities-regulation.pdf?sfvrsn=2
static ISE_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 16 * 3600 + 28 * 60,
}];
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
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 28 * 60,
        close_ssm: 16 * 3600 + 30 * 60,
    },
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
    has_daily_close: true,
    has_weekend_close: true,
};

// Euronext migrated Dublin equities to Optiq on 2019-02-04. Euronext notices
// establish the post-migration 06:15 pre-opening; principal shares open after
// the latest 08:00:30 uncross, close continuous trading at 16:28, and retain a
// closing-auction/Trading-at-Last envelope through 16:40.
// https://www.euronext.com/sites/default/files/190204optiq_migration_dublin_press_release.pdf
// https://www.eurex.com/ex-en/find/circulars/Discontinuation-of-clearing-services-for-Irish-Stock-Exchange-Amendments-to-the-Clearing-Conditions-and-to-the-Price-List-of-Eurex-Clearing-AG-1391874
// https://live.euronext.com/sites/default/files/2021-07/Market%20Notice%20-%20Datalex%20Plc%20-%20Admission.pdf
static OPTIQ_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30,
    close_ssm: 16 * 3600 + 28 * 60,
}];
static OPTIQ_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 6 * 3600 + 15 * 60,
        close_ssm: 8 * 3600 + 30,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 28 * 60,
        close_ssm: 16 * 3600 + 30 * 60 + 30,
    },
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
    has_daily_close: true,
    has_weekend_close: true,
};

// Euronext's phase-one timetable shifted legacy-market pre-opening effective
// 2023-03-20 from 07:15 to 07:30 CET, or 06:15 to 06:30 Dublin local time. The
// operator document separately gives 2023-03-27 for the Italian migration.
// The current trading appendix confirms the resulting principal-share grid.
// https://connect.euronext.com/sites/default/files/it-documentation/Go-Live%20Weekend%20Guidelines%20-%20Borsa%20Italiana%20Optiq%20Migration.pdf
// https://www.euronext.com/sites/default/files/2026-07/appendix%20to%20Euronext%20Instructions%204-01%204-03%20Trading%20Manuals_0.xlsx
static CURRENT_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 6 * 3600 + 30 * 60,
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
    has_daily_close: true,
    has_weekend_close: true,
};

static REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2019, 2, 4),
        profile: &OPTIQ_PROFILE,
    },
    Revision {
        effective: effective_date(2023, 3, 20),
        profile: &EURONEXT_DUB_PROFILE,
    },
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, Europe::Dublin), &ISE_PROFILE, REVISIONS)
}
