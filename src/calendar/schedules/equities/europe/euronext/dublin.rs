// SPDX-License-Identifier: MIT-0

//! Euronext Dublin principal shares and the predecessor ISE Xetra book.

use chrono_tz::Europe;

use super::super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// ISE's own archived trading-hours page establishes the complete legacy grid
// before the January-2010 audit floor: pre-trading 06:30-07:50, opening auction
// to 08:00, continuous trading to 16:28, closing auction to 16:30, and
// post-trading through 17:15. Post-trading accepted new orders and off-book
// reports, so it is `extended` under the crate's order-entry-only convention.
// Successive archived order-book models through T7 Release 6 retain this exact
// grid, and the official 2018 trading calendar repeats it through the last full
// pre-Optiq year.
// https://web.archive.org/web/20090930042026id_/http://www.ise.ie/index.asp?locID=311&docID=-1
// https://web.archive.org/web/20121004024422id_/http://www.ise.ie/Membership-and-Trading/Market-Infrastructure/Trading/ISE-Xetra%C2%AE/Release-11-1/ISE_Xetra_Rel_11_1_Market_Model_090511.pdf
// https://web.archive.org/web/20120907001910id_/http://www.ise.ie/Membership-and-Trading/Market-Infrastructure/Trading/ISE-Xetra%C2%AE/Release-12/ISE_Xetra_Rel_12_Market_Model.pdf
// https://web.archive.org/web/20130517042005id_/http://ise.ie/Membership-and-Trading/Market-Infrastructure/Trading/ISE-Xetra%C2%AE/Release_13/ISE_Xetra_Market_Model_-_Release_13_0.pdf
// https://web.archive.org/web/20140718094950id_/http://ise.ie/Membership-and-Trading/Market-Infrastructure/Trading/ISE-Xetra%C2%AE/Release_14/ISE_Xetra_Rel_14_0_Market_Model.pdf
// https://web.archive.org/web/20150315070155id_/http://ise.ie/Products-Services/Trading-Members/Equity-Trading-Membership/ISE-Xetra-Release-15-Market-Model.pdf
// https://web.archive.org/web/20170301204221id_/http://www.ise.ie/Products-Services/Trading-Members/Equity-Trading-Membership/ISE%20Xetra%20Release%2016%20Market%20Model.pdf
// https://web.archive.org/web/20171029062447id_/http://www.ise.ie/Products-Services/Trading-Members/Equity-Trading-Membership/ISE-T7-Market-Model-Rel-5-0.pdf
// https://web.archive.org/web/20171108073227id_/http://www.ise.ie/Products-Services/Trading-Members/Equity-Trading-Membership/T7%20Rel%206%20Market%20Model.pdf
// https://web.archive.org/web/20181215004420id_/http://www.ise.ie/Products-Services/Trading-Members/Equity-Trading-Calendar-2018.pdf
// The archived December-2018 operator page identifies Release 7.0 as current,
// links its market model, and states that ISE T7 would remain live through
// 2019-02-01 before the sourced Optiq migration on 2019-02-04.
// https://web.archive.org/web/20181215004420id_/http://www.ise.ie/Products-Services/Trading-Members/Equity-Trading-Membership/
// The Central Bank's official assessment independently records the same
// 07:50-16:30 order-book envelope.
// https://www.centralbank.ie/docs/default-source/tns/about---tns/peer-reviews-and-reports/tns-1-11-imf-report-on-observance-of-standards-and-codes-on-securities-regulation.pdf?sfvrsn=2
static ISE_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 16 * 3600 + 28 * 60,
}];
// Order-entry classification. The same market model states that "the order book
// is only open for trading during auctions and continuous trading in the main
// trading phase", and describes the pre-trading phase as one where participants
// enter, modify and delete orders and quotes with the order book closed. No
// order-book trade can match there, so pre-trading is `order_entry`. The
// post-trading leg stays in `extended`: this file's existing sourced note
// records that it accepted off-book reports as well as orders, and demoting a
// post-close window on that record is the unsafe direction.
static ISE_EXTENDED: &[SessionRule] = &[
    // Pre-trading. The order book is closed to MATCHING here, but the same
    // record that keeps the post-trading leg tradeable shows off-book reports
    // were accepted in both phases - and prints occur wherever off-book reports
    // are accepted. Applying that criterion to the close side but not the open
    // side was the inconsistency; both legs stay tradeable.
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

// Euronext migrated Dublin equities to Optiq on 2019-02-04. Euronext notices
// establish the post-migration 06:15 pre-opening; principal shares open after
// the latest 08:00:30 uncross, close continuous trading at 16:28, and retain a
// closing-auction/Trading-at-Last envelope through 16:40.
// https://www.euronext.com/sites/default/files/190204optiq_migration_dublin_press_release.pdf
// https://www.eurex.com/ex-en/find/circulars/Discontinuation-of-clearing-services-for-Irish-Stock-Exchange-Amendments-to-the-Clearing-Conditions-and-to-the-Price-List-of-Eurex-Clearing-AG-1391874
// https://live.euronext.com/sites/default/files/2021-07/Market%20Notice%20-%20Datalex%20Plc%20-%20Admission.pdf
// https://web.archive.org/web/20191018025213id_/https://www.euronext.com/sites/default/files/2019-09/52118_Euronext-FAQ-2019_v07_0.pdf
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

// Euronext's phase-one timetable shifted legacy-market pre-opening effective
// 2023-03-20 from 07:15 to 07:30 CET, or 06:15 to 06:30 Dublin local time. The
// operator document separately gives 2023-03-27 for the Italian migration.
// The current trading appendix confirms the resulting principal-share grid.
// https://connect.euronext.com/sites/default/files/it-documentation/Go-Live%20Weekend%20Guidelines%20-%20Borsa%20Italiana%20Optiq%20Migration.pdf
// https://www.euronext.com/sites/default/files/2026-07/appendix%20to%20Euronext%20Instructions%204-01%204-03%20Trading%20Manuals_0.xlsx
// Order-entry classification: same Call/uncrossing split as the 2019 grid, with
// the pre-opening start shifted to 06:30 Dublin local time.
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
