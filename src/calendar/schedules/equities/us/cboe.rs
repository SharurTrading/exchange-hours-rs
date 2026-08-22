// SPDX-License-Identifier: MIT-0

//! Cboe US cash-equity profiles and dated schedule history.

use chrono_tz::America;

use super::StaticHoursProfile;
use super::equities::{US_EQUITY_EXTENDED, equity_profile};
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
// the first production day. The operator's 2016 implementation notice confirms
// both old grids and each venue's distinct 07:00 matching-start date.
// Pre-session order acceptance is not treated as executable trading.
// https://www.sec.gov/rules/sro/bats/2009/34-59963.pdf
// https://www.sec.gov/files/rules/sro/byx/2010/34-63097.pdf
// https://cdn.cboe.com/resources/fee_schedule/2010/BATS-Announces-BATS-Y-Exchange-BYX-Pricing-Effective-October-15-2010-and-New-B2B-TRIM-SLIM-and-One-Under-Routing-Strategies.pdf
// https://www.nasdaqtrader.com/TraderNews.aspx?id=uva2010-007
// https://www.sec.gov/file/34-62431
// https://www.globenewswire.com/news-release/2010/07/21/425534/9381/en/Direct-Edge-Launches-Exchange-Operations.html
// https://cdn.cboe.com/resources/release_notes/2016/Update-Bats-to-Begin-Equity-Order-Matching-and-Routing-at-7-am-ET.pdf
static BZX_0800_1700: StaticHoursProfile = equity_profile(EXTENDED_0800_1700);
static BZX_0700_1700: StaticHoursProfile = equity_profile(EXTENDED_0700_1700);
static BZX_0700_2000: StaticHoursProfile = equity_profile(EXTENDED_0700_2000);
pub(crate) static CBOE_BZX_PROFILE: StaticHoursProfile = equity_profile(US_EQUITY_EXTENDED);

static BYX_0800_1700: StaticHoursProfile = equity_profile(EXTENDED_0800_1700);
static BYX_0700_1700: StaticHoursProfile = equity_profile(EXTENDED_0700_1700);
pub(crate) static CBOE_BYX_PROFILE: StaticHoursProfile = equity_profile(EXTENDED_0700_2000);

static EDGA_0800_2000: StaticHoursProfile = equity_profile(EXTENDED_0800_2000);
pub(crate) static CBOE_EDGA_PROFILE: StaticHoursProfile = equity_profile(EXTENDED_0700_2000);

static EDGX_0800_2000: StaticHoursProfile = equity_profile(EXTENDED_0800_2000);
static EDGX_0700_2000: StaticHoursProfile = equity_profile(EXTENDED_0700_2000);
pub(crate) static CBOE_EDGX_PROFILE: StaticHoursProfile = equity_profile(US_EQUITY_EXTENDED);

// The 2018 operator notice dates BZX's and BYX's 20:00 close extensions. Cboe's
// early-hours article and launch retrospective independently date BZX's 04:00
// open to 2025-05-01.
// https://cdn.cboe.com/resources/release_notes/2018/BZX-Exchange-and-BYX-Exchange-to-Extend-Post-Market-Session-Hours-to-8PM-ET.pdf
// https://www.cboe.com/insights/posts/early-birds-and-night-owls-how-extended-trading-hours-are-reshaping-u-s-equities-markets-
// https://res.cboe.com/insights/posts/u-s-cash-equities-may-highlights/
static BZX_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2016, 5, 25),
        profile: &BZX_0700_1700,
    },
    Revision {
        effective: effective_date(2018, 7, 30),
        profile: &BZX_0700_2000,
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
        effective: effective_date(2016, 5, 23),
        profile: &BYX_0700_1700,
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
        profile: &CBOE_EDGA_PROFILE,
    },
];

pub(crate) fn edga_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        EDGA_REVISIONS,
    )
}

// EDGX introduced 04:00 ET trading on 2021-03-08. Its future overnight-session
// announcement remains monitored but unselected until the approved rule's
// Equity Data Plan and later EDGX readiness conditions are satisfied.
// https://www.cboe.com/insights/posts/cboe-edgx-equities-exchange-to-introduce-early-trading-hours-beginning-march-8/
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
