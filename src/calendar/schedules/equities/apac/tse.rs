// SPDX-License-Identifier: MIT-0

//! Tokyo Stock Exchange cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// JPX publishes the current 09:00–11:30 and 12:30–15:30 auction-trading
// sessions, with order acceptance from 08:00 and 12:05. Arrowhead continuous
// matching ends at 15:25; the final five minutes are the closing call.
// Sources:
// https://www.jpx.co.jp/english/equities/trading/domestic/01.html
// https://www.jpx.co.jp/english/systems/equities-trading/01.html
static TSE_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 11 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600 + 30 * 60,
        close_ssm: 15 * 3600 + 25 * 60,
    },
];
static TSE_REGULAR_POST_2011: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 11 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600 + 30 * 60,
        close_ssm: 15 * 3600,
    },
];
static TSE_REGULAR_PRE_2011: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 11 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600 + 30 * 60,
        close_ssm: 15 * 3600,
    },
];
// TSE cash products trade on both the arrowhead auction system and the
// off-auction ToSTNeT system. Arrowhead accepts orders from 08:00 and ToSTNeT
// single-stock trading keeps the venue available through 18:00. ToSTNeT is
// classified extended so the regular rules continue to describe the central
// auction market. Not every security or order type is eligible for every
// phase.
// https://www.jpx.co.jp/english/systems/equities-trading/
// https://www.jpx.co.jp/english/equities/trading/tostnet/02.html
//
// Order-entry split: JPX's ToSTNeT hours page puts single-issue and basket
// trading (ToSTNeT-1) at 08:20–18:00 today and 08:20–17:30 before the
// 2024-11-05 upgrade, so the earliest executable edge of the venue is 08:20 in
// both eras. Arrowhead only accepts orders from 08:00; its first Itayose match
// is the 09:00 open, and JPX Working Paper No.3 records those 08:00 orders as
// entered outside the matching session. Nothing can print between 08:00 and
// 08:20, so that leading window is order entry rather than extended trading.
// The 08:00 acceptance is dated for each historical era by the operator's own
// record: Working Paper No.3 analyzes arrowhead order-book data from
// 2010-01-04 (the pre-2011 profile), and the Investigation Report of
// November 30, 2020 into the October 1, 2020 system failure states "Order
// acceptance began as normal at 08:00" (the post-2011 profile).
// https://www.jpx.co.jp/english/corporate/news/news-releases/0020/b5b4pj000003xrsa-att/InvestigationReport.pdf
static TSE_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 20 * 60,
    close_ssm: 18 * 3600,
}];
static TSE_EXTENDED_PRE_2024: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 20 * 60,
    close_ssm: 17 * 3600 + 30 * 60,
}];
// Arrowhead order acceptance ahead of the 08:20 ToSTNeT open. Orders may be
// entered, amended and cancelled; no matching engine runs, so no trade prints.
static TSE_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 8 * 3600 + 20 * 60,
}];

// JPX's official trading-hours transition table dates the arrowhead morning
// extension to 2011-11-21. The 2024 extension appendix expressly changes
// ToSTNeT single-stock/basket trading to 18:00, and the final go-live release
// confirms the upgraded arrowhead and ToSTNeT systems launched on 2024-11-05.
// Its 2010 shareholder report establishes that ToSTNeT had already been
// extended to 17:30 in November 2009, before this repository's January-2010
// audit floor.
// JPX Working Paper No.3 analyzes the operator's own FLEX order-book data from
// 2010-01-04 and explicitly identifies orders entered from 08:00 outside the
// matching session; the November 2020 Investigation Report's "Order acceptance
// began as normal at 08:00" carries that acceptance through the post-2011
// profile's era. The report does not state an exact pre-floor day for the
// 2009 tail change, so none is invented here.
// https://www.jpx.co.jp/english/equities/trading/domestic/tvdivq0000006blj-att/tradinghours_eg.pdf
// https://www.jpx.co.jp/english/corporate/news/news-releases/1030/uorii50000002f2a-att/pressrelease_extension_of_trading_hours_en.pdf
// https://www.jpx.co.jp/english/corporate/news/news-releases/1030/20241103-01.html
// https://www.jpx.co.jp/english/corporate/investor-relations/shareholders/meeting/tvdivq000000958w-att/tse04.pdf
// https://www.jpx.co.jp/corporate/research-study/working-paper/tvdivq0000008q5y-att/JPX_working_paper_No.3.pdf
pub(crate) static TSE_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Tokyo,
    regular: TSE_REGULAR_CURRENT,
    extended: TSE_EXTENDED_CURRENT,
    order_entry: TSE_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static TSE_PROFILE_POST_2011_11_21: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Tokyo,
    regular: TSE_REGULAR_POST_2011,
    extended: TSE_EXTENDED_PRE_2024,
    order_entry: TSE_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static TSE_PROFILE_PRE_2011_11_21: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Tokyo,
    regular: TSE_REGULAR_PRE_2011,
    extended: TSE_EXTENDED_PRE_2024,
    order_entry: TSE_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &TSE_PROFILE_CURRENT;

static REVISIONS: &[Revision] = revisions![
    (
        2011,
        11,
        21,
        &TSE_PROFILE_POST_2011_11_21,
        "JPX trading-hours transition table"
    ),
    (
        2024,
        11,
        5,
        &TSE_PROFILE_CURRENT,
        "JPX news release 20241103-01"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &TSE_PROFILE_PRE_2011_11_21,
        REVISIONS,
    )
}
