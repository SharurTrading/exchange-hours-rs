// SPDX-License-Identifier: MIT-0

//! Cboe US cash-equity profiles and dated schedule history.

use chrono_tz::America;

use super::StaticHoursProfile;
use super::equities::{US_EQUITY_EXTENDED, equity_profile, equity_profile_with_entry};
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::CLOSED_NEW_YORK;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

static EXTENDED_0800_1700: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
];

// Cboe's hours table lists an "Early Order Acceptance" phase that ends where
// the Early Trading Session begins: 02:30–04:00 on BZX/EDGX and 06:00–07:00 on
// BYX/EDGA. Orders are accepted, amended and cancelled there but nothing
// matches until the trading session opens, so those windows are `order_entry`
// rather than `extended`.
// https://www.cboe.com/about/hours/
// https://cdn.cboe.com/resources/release_notes/2016/Update-Bats-to-Begin-Equity-Order-Matching-and-Routing-at-7-am-ET.pdf
static ENTRY_0230_0400: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 2 * 3600 + 30 * 60,
    close_ssm: 4 * 3600,
}];

static ENTRY_0330_0400: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3 * 3600 + 30 * 60,
    close_ssm: 4 * 3600,
}];

static ENTRY_0600_0700: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 6 * 3600,
    close_ssm: 7 * 3600,
}];

// 2014-12 through each exchange's 2016 matching change: orders were accepted
// from 06:00 but matching and routing began at 08:00, so the whole 06:00-08:00
// window is `order_entry`.
static ENTRY_0600_0800: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 6 * 3600,
    close_ssm: 8 * 3600,
}];

static EXTENDED_0700_1700: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
];

static EXTENDED_0800_2000: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 20 * 3600,
    },
];

static EXTENDED_0700_2000: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 20 * 3600,
    },
];

// BZX's January-2010 baseline was 08:00–17:00 ET. BYX launched on
// 2010-10-15 with that same execution envelope. Direct Edge began production
// trading on EDGA and EDGX with one symbol on 2010-07-02, then phased in the
// remaining symbols through 2010-07-21; the exchange-level profile begins on
// the first production day. Cboe's current hours table also publishes the
// order-acceptance queues that precede matching: 02:30 for BZX/EDGX and 06:00
// for BYX/EDGA. Those queues are `order_entry`, not Extended — the hours table
// names them "Early Order Acceptance" and no trade prints until the Early
// Trading Session opens at 04:00 (BZX/EDGX) or 07:00 (BYX/EDGA).
// https://www.sec.gov/rules/sro/bats/2009/34-59963.pdf
// https://www.sec.gov/files/rules/sro/byx/2010/34-63097.pdf
// https://cdn.cboe.com/resources/fee_schedule/2010/BATS-Announces-BATS-Y-Exchange-BYX-Pricing-Effective-October-15-2010-and-New-B2B-TRIM-SLIM-and-One-Under-Routing-Strategies.pdf
// https://www.nasdaqtrader.com/TraderNews.aspx?id=uva2010-007
// https://www.sec.gov/file/34-62431
// https://www.globenewswire.com/news-release/2010/07/21/425534/9381/en/Direct-Edge-Launches-Exchange-Operations.html
// https://cdn.cboe.com/resources/release_notes/2016/Update-Bats-to-Begin-Equity-Order-Matching-and-Routing-at-7-am-ET.pdf
// https://www.cboe.com/about/hours/
static BZX_0800_1700: StaticHoursProfile = equity_profile(EXTENDED_0800_1700);
static BZX_QUEUE_2014: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0800_1700, ENTRY_0600_0800);
static BZX_0600_1700: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_1700, ENTRY_0600_0700);
static BZX_0600_2000: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_2000, ENTRY_0600_0700);
pub(crate) static CBOE_BZX_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(US_EQUITY_EXTENDED, ENTRY_0230_0400);

static BYX_0800_1700: StaticHoursProfile = equity_profile(EXTENDED_0800_1700);
static BYX_QUEUE_2014: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0800_1700, ENTRY_0600_0800);
static BYX_0600_1700: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_1700, ENTRY_0600_0700);
pub(crate) static CBOE_BYX_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_2000, ENTRY_0600_0700);

static EDGA_0800_2000: StaticHoursProfile = equity_profile(EXTENDED_0800_2000);
static EDGA_0700_2000: StaticHoursProfile = equity_profile(EXTENDED_0700_2000);
pub(crate) static CBOE_EDGA_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(EXTENDED_0700_2000, ENTRY_0600_0700);

static EDGX_0800_2000: StaticHoursProfile = equity_profile(EXTENDED_0800_2000);
static EDGX_0700_2000: StaticHoursProfile = equity_profile(EXTENDED_0700_2000);
static EDGX_0330_2000: StaticHoursProfile =
    equity_profile_with_entry(US_EQUITY_EXTENDED, ENTRY_0330_0400);
pub(crate) static CBOE_EDGX_PROFILE: StaticHoursProfile =
    equity_profile_with_entry(US_EQUITY_EXTENDED, ENTRY_0230_0400);

// The final 2014 operator notice dates BYX's and BZX's 06:00 order-acceptance
// queues to 2014-12-01 and 2014-12-02. The 2016 release note then moved
// matching and routing one hour earlier, to 07:00 ET, on staggered days (BYX
// May 23, EDGA May 24, BZX May 25, EDGX May 26) and published the resulting
// sessions: Early Order Acceptance 06:00-07:00, Early Trading Session
// 07:00-08:00, Pre-Market 08:00-09:30. Before each exchange's 2016 day orders
// were accepted from 06:00 but nothing matched until 08:00, so the 2014-2016
// profiles carry the whole 06:00-08:00 leg as `order_entry` and extended
// starts at 08:00; from the 2016 day the 07:00-08:00 hour matches and joins
// `extended`.
// The 2018 notice dates each 20:00 close extension,
// and the operator independently dates BZX's 02:30 queue / 04:00 active-session
// expansion to 2025-05-01.
// https://cdn.cboe.com/resources/release_notes/2014/BATS-BYX-Exchange-and-BZX-Exchange-Feature-Release-Postponed-Until-December-2014.pdf
// https://www.sec.gov/rules/sro/bats/2014/34-73745.pdf
// https://www.sec.gov/rules/sro/byx/2014/34-73744.pdf
// https://cdn.cboe.com/resources/release_notes/2016/Update-Bats-to-Begin-Equity-Order-Matching-and-Routing-at-7-am-ET.pdf
// https://cdn.cboe.com/resources/release_notes/2018/BZX-Exchange-and-BYX-Exchange-to-Extend-Post-Market-Session-Hours-to-8PM-ET.pdf
// https://www.cboe.com/insights/posts/early-birds-and-night-owls-how-extended-trading-hours-are-reshaping-u-s-equities-markets-
// https://res.cboe.com/insights/posts/u-s-cash-equities-may-highlights/
// Row evidence:
//   2014-12-02 "SEC 34-73745"
//     https://www.sec.gov/rules/sro/bats/2014/34-73745.pdf
//   2016-05-25 "Bats release note 2016 7am matching"
//     https://cdn.cboe.com/resources/release_notes/2016/Update-Bats-to-Begin-Equity-Order-Matching-and-Routing-at-7-am-ET.pdf
//   2018-07-30 "Bats release note 2018 8pm post-market"
//     https://cdn.cboe.com/resources/release_notes/2018/BZX-Exchange-and-BYX-Exchange-to-Extend-Post-Market-Session-Hours-to-8PM-ET.pdf
//   2025-05-01 "Cboe insights May 2025"
//     https://res.cboe.com/insights/posts/u-s-cash-equities-may-highlights/
static BZX_REVISIONS: &[Revision] = revisions![
    (2014, 12, 2, &BZX_QUEUE_2014, "SEC 34-73745"),
    (
        2016,
        5,
        25,
        &BZX_0600_1700,
        "Bats release note 2016 7am matching"
    ),
    (
        2018,
        7,
        30,
        &BZX_0600_2000,
        "Bats release note 2018 8pm post-market"
    ),
    (2025, 5, 1, &CBOE_BZX_PROFILE, "Cboe insights May 2025"),
];

pub(crate) fn bzx_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &BZX_0800_1700,
        BZX_REVISIONS,
    )
}

// Row evidence:
//   2010-10-15 "SEC 34-63097"
//     https://www.sec.gov/files/rules/sro/byx/2010/34-63097.pdf
//   2014-12-01 "SEC 34-73744"
//     https://www.sec.gov/rules/sro/byx/2014/34-73744.pdf
//   2016-05-23 "Bats release note 2016 7am matching"
//     https://cdn.cboe.com/resources/release_notes/2016/Update-Bats-to-Begin-Equity-Order-Matching-and-Routing-at-7-am-ET.pdf
//   2018-08-27 "Bats release note 2018 8pm post-market"
//     https://cdn.cboe.com/resources/release_notes/2018/BZX-Exchange-and-BYX-Exchange-to-Extend-Post-Market-Session-Hours-to-8PM-ET.pdf
static BYX_REVISIONS: &[Revision] = revisions![
    (2010, 10, 15, &BYX_0800_1700, "SEC 34-63097"),
    (2014, 12, 1, &BYX_QUEUE_2014, "SEC 34-73744"),
    (
        2016,
        5,
        23,
        &BYX_0600_1700,
        "Bats release note 2016 7am matching"
    ),
    (
        2018,
        8,
        27,
        &CBOE_BYX_PROFILE,
        "Bats release note 2018 8pm post-market"
    ),
];

pub(crate) fn byx_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        BYX_REVISIONS,
    )
}

// Row evidence:
//   2010-07-02 "SEC 34-62431"
//     https://www.sec.gov/file/34-62431
//   2016-05-24 "Bats release note 2016 7am matching"
//     https://cdn.cboe.com/resources/release_notes/2016/Update-Bats-to-Begin-Equity-Order-Matching-and-Routing-at-7-am-ET.pdf
static EDGA_REVISIONS: &[Revision] = revisions![
    (2010, 7, 2, &EDGA_0800_2000, "SEC 34-62431"),
    (
        2016,
        5,
        24,
        &EDGA_0700_2000,
        "Bats release note 2016 7am matching"
    ),
];

pub(crate) fn edga_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        EDGA_REVISIONS,
    )
}

// EDGA and EDGX accepted orders from 06:00 by 2014, but the reviewed primary
// sources do not state the original onset day. Their exact fixed-current
// snapshots include that queue as `order_entry`. EDGA's present and future dated
// queries retain its last sourced 07:00 regime rather than inventing a queue
// cutover; the dated EDGA/EDGX 08:00 and 07:00 eras carry no queue and stay
// wholly Extended.
// EDGX introduced its 03:30 queue / 04:00 active session on 2021-03-08 and
// moved the queue to 02:30 on 2021-09-07; both queue legs are `order_entry`
// because matching only starts at 04:00. Its future overnight-session remains
// unselected until the Equity Data Plan and readiness conditions are satisfied.
// https://www.sec.gov/files/rules/sro/edga/2014/34-73592.pdf
// https://www.sec.gov/rules/sro/edgx/2014/34-73468.pdf
// https://ir.cboe.com/news/news-details/2021/Cboe-EDGX-Equities-Exchange-To-Introduce-Early-Trading-Hours-Beginning-March-8-02-08-2021/default.aspx
// https://www.sec.gov/files/rules/sro/cboeedgx/2021/34-92914.pdf
// Row evidence:
//   2010-07-02 "SEC 34-62431"
//     https://www.sec.gov/file/34-62431
//   2016-05-26 "Bats release note 2016 7am matching"
//     https://cdn.cboe.com/resources/release_notes/2016/Update-Bats-to-Begin-Equity-Order-Matching-and-Routing-at-7-am-ET.pdf
//   2021-03-08 "Cboe press release 2021-02-08"
//     https://ir.cboe.com/news/news-details/2021/Cboe-EDGX-Equities-Exchange-To-Introduce-Early-Trading-Hours-Beginning-March-8-02-08-2021/default.aspx
//   2021-09-07 "SEC 34-92914"
//     https://www.sec.gov/files/rules/sro/cboeedgx/2021/34-92914.pdf
static EDGX_REVISIONS: &[Revision] = revisions![
    (2010, 7, 2, &EDGX_0800_2000, "SEC 34-62431"),
    (
        2016,
        5,
        26,
        &EDGX_0700_2000,
        "Bats release note 2016 7am matching"
    ),
    (2021, 3, 8, &EDGX_0330_2000, "Cboe press release 2021-02-08"),
    (2021, 9, 7, &CBOE_EDGX_PROFILE, "SEC 34-92914"),
];

pub(crate) fn edgx_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        EDGX_REVISIONS,
    )
}
