// SPDX-License-Identifier: MIT-0

//! Cboe US cash-equity profiles and dated schedule history.

use chrono_tz::America;

use super::StaticHoursProfile;
use super::equities::equity_profile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::CLOSED_NEW_YORK;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

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

static EXTENDED_0600_1700: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 6 * 3600,
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

static EXTENDED_0600_2000: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 6 * 3600,
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

static EXTENDED_0330_2000: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 3 * 3600 + 30 * 60,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 20 * 3600,
    },
];

static EXTENDED_0230_2000: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 2 * 3600 + 30 * 60,
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
// for BYX/EDGA. These accepted-order phases are Extended even though orders do
// not execute until the later active-session boundary.
// https://www.sec.gov/rules/sro/bats/2009/34-59963.pdf
// https://www.sec.gov/files/rules/sro/byx/2010/34-63097.pdf
// https://cdn.cboe.com/resources/fee_schedule/2010/BATS-Announces-BATS-Y-Exchange-BYX-Pricing-Effective-October-15-2010-and-New-B2B-TRIM-SLIM-and-One-Under-Routing-Strategies.pdf
// https://www.nasdaqtrader.com/TraderNews.aspx?id=uva2010-007
// https://www.sec.gov/file/34-62431
// https://www.globenewswire.com/news-release/2010/07/21/425534/9381/en/Direct-Edge-Launches-Exchange-Operations.html
// https://cdn.cboe.com/resources/release_notes/2016/Update-Bats-to-Begin-Equity-Order-Matching-and-Routing-at-7-am-ET.pdf
// https://www.cboe.com/about/hours/
static BZX_0800_1700: StaticHoursProfile = equity_profile(EXTENDED_0800_1700);
static BZX_0600_1700: StaticHoursProfile = equity_profile(EXTENDED_0600_1700);
static BZX_0600_2000: StaticHoursProfile = equity_profile(EXTENDED_0600_2000);
pub(crate) static CBOE_BZX_PROFILE: StaticHoursProfile = equity_profile(EXTENDED_0230_2000);

static BYX_0800_1700: StaticHoursProfile = equity_profile(EXTENDED_0800_1700);
static BYX_0600_1700: StaticHoursProfile = equity_profile(EXTENDED_0600_1700);
pub(crate) static CBOE_BYX_PROFILE: StaticHoursProfile = equity_profile(EXTENDED_0600_2000);

static EDGA_0800_2000: StaticHoursProfile = equity_profile(EXTENDED_0800_2000);
static EDGA_0700_2000: StaticHoursProfile = equity_profile(EXTENDED_0700_2000);
pub(crate) static CBOE_EDGA_PROFILE: StaticHoursProfile = equity_profile(EXTENDED_0600_2000);

static EDGX_0800_2000: StaticHoursProfile = equity_profile(EXTENDED_0800_2000);
static EDGX_0700_2000: StaticHoursProfile = equity_profile(EXTENDED_0700_2000);
static EDGX_0330_2000: StaticHoursProfile = equity_profile(EXTENDED_0330_2000);
pub(crate) static CBOE_EDGX_PROFILE: StaticHoursProfile = equity_profile(EXTENDED_0230_2000);

// The final 2014 operator notice dates BYX's and BZX's 06:00 order-acceptance
// queues to 2014-12-01 and 2014-12-02, respectively. Because those queues span
// the later 2016 matching-start changes, the 2016 dates no longer change this
// API's open/closed envelope. The 2018 notice dates each 20:00 close extension,
// and the operator independently dates BZX's 02:30 queue / 04:00 active-session
// expansion to 2025-05-01.
// https://cdn.cboe.com/resources/release_notes/2014/BATS-BYX-Exchange-and-BZX-Exchange-Feature-Release-Postponed-Until-December-2014.pdf
// https://www.sec.gov/rules/sro/bats/2014/34-73745.pdf
// https://www.sec.gov/rules/sro/byx/2014/34-73744.pdf
// https://cdn.cboe.com/resources/release_notes/2018/BZX-Exchange-and-BYX-Exchange-to-Extend-Post-Market-Session-Hours-to-8PM-ET.pdf
// https://www.cboe.com/insights/posts/early-birds-and-night-owls-how-extended-trading-hours-are-reshaping-u-s-equities-markets-
// https://res.cboe.com/insights/posts/u-s-cash-equities-may-highlights/
static BZX_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2014, 12, 2),
        profile: &BZX_0600_1700,
    },
    Revision {
        effective: effective_date(2018, 7, 30),
        profile: &BZX_0600_2000,
    },
    Revision {
        effective: effective_date(2025, 5, 1),
        profile: &CBOE_BZX_PROFILE,
    },
];

pub(crate) fn bzx_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &BZX_0800_1700,
        BZX_REVISIONS,
    )
}

static BYX_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2010, 10, 15),
        profile: &BYX_0800_1700,
    },
    Revision {
        effective: effective_date(2014, 12, 1),
        profile: &BYX_0600_1700,
    },
    Revision {
        effective: effective_date(2018, 8, 27),
        profile: &CBOE_BYX_PROFILE,
    },
];

pub(crate) fn byx_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        BYX_REVISIONS,
    )
}

static EDGA_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2010, 7, 2),
        profile: &EDGA_0800_2000,
    },
    Revision {
        effective: effective_date(2016, 5, 24),
        profile: &EDGA_0700_2000,
    },
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
// snapshots include that queue. EDGA's present and future dated queries retain
// its last sourced 07:00 regime rather than inventing a queue cutover.
// EDGX introduced its 03:30 queue / 04:00 active session on 2021-03-08 and
// moved the queue to 02:30 on 2021-09-07. Its future overnight-session remains
// unselected until the Equity Data Plan and readiness conditions are satisfied.
// https://www.sec.gov/files/rules/sro/edga/2014/34-73592.pdf
// https://www.sec.gov/rules/sro/edgx/2014/34-73468.pdf
// https://ir.cboe.com/news/news-details/2021/Cboe-EDGX-Equities-Exchange-To-Introduce-Early-Trading-Hours-Beginning-March-8-02-08-2021/default.aspx
// https://www.sec.gov/files/rules/sro/cboeedgx/2021/34-92914.pdf
static EDGX_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2010, 7, 2),
        profile: &EDGX_0800_2000,
    },
    Revision {
        effective: effective_date(2016, 5, 26),
        profile: &EDGX_0700_2000,
    },
    Revision {
        effective: effective_date(2021, 3, 8),
        profile: &EDGX_0330_2000,
    },
    Revision {
        effective: effective_date(2021, 9, 7),
        profile: &CBOE_EDGX_PROFILE,
    },
];

pub(crate) fn edgx_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        EDGX_REVISIONS,
    )
}
