// SPDX-License-Identifier: MIT-0

//! Point-in-time selectors for US cash-equity schedules.

use chrono_tz::America;

use super::StaticHoursProfile;
use super::equities::{
    MEMX_EQ_PROFILE, MEMX_EQ_PROFILE_2020_10_05, MEMX_EQ_PROFILE_PRE_2025_05_19,
    MIAX_PEARL_EQ_PROFILE, MIAX_PEARL_EQ_PROFILE_PRE_2025_02_20, NASDAQ_BX_PROFILE,
    NASDAQ_BX_PROFILE_PRE_2011_04_18, NASDAQ_PROFILE, NASDAQ_PROFILE_PRE_2013_03_18,
    NASDAQ_PSX_PROFILE, NASDAQ_PSX_PROFILE_AT_LAUNCH,
};
use crate::calendar::schedules::CLOSED_NEW_YORK;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// Nasdaq Equity Trader Alert 2013-21 moved the pre-market open from 07:00 to
// 04:00 ET effective Monday 2013-03-18. Future Night Session announcements are
// monitored in the schedule update guide but are not selected until Nasdaq's
// required readiness filing supplies an unconditional effective day.
// https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2013-21
static NASDAQ_REVISIONS: &[Revision] = revisions![(
    2013,
    3,
    18,
    &NASDAQ_PROFILE,
    "Nasdaq Equity Trader Alert 2013-21"
),];

pub(crate) fn nasdaq_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &NASDAQ_PROFILE_PRE_2013_03_18,
        NASDAQ_REVISIONS,
    )
}

// Nasdaq Equity Trader Alert 2011-20 states that BX began accepting and
// executing orders at 07:00 ET on Monday 2011-04-18. The official launch alert
// supplies the 08:00 ET predecessor open and unchanged 19:00 close.
// https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2009-003
// https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2011-20
static NASDAQ_BX_REVISIONS: &[Revision] = revisions![(
    2011,
    4,
    18,
    &NASDAQ_BX_PROFILE,
    "Nasdaq Equity Trader Alert 2011-20"
),];

pub(crate) fn nasdaq_bx_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &NASDAQ_BX_PROFILE_PRE_2011_04_18,
        NASDAQ_BX_REVISIONS,
    )
}

// Nasdaq's launch alert dates PSX production to 2010-10-08. The initial rules
// operated 09:00–17:00 ET; SR-Phlx-2010-172 explicitly identifies 2010-12-13
// as the implementation date for the 08:00 ET opening.
// Row evidence:
//   2010-10-08 "Nasdaq Equity Trader Alert 2010-56"
//     https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2010-56
//   2010-12-13 "SEC SR-Phlx-2010-172"
//     https://www.sec.gov/files/rules/sro/phlx/2010/34-63492.pdf
static NASDAQ_PSX_REVISIONS: &[Revision] = revisions![
    (
        2010,
        10,
        8,
        &NASDAQ_PSX_PROFILE_AT_LAUNCH,
        "Nasdaq Equity Trader Alert 2010-56"
    ),
    (2010, 12, 13, &NASDAQ_PSX_PROFILE, "SEC SR-Phlx-2010-172"),
];

pub(crate) fn nasdaq_psx_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        NASDAQ_PSX_REVISIONS,
    )
}

// MEMX began live trading on 2020-09-21. It shortened the Post-Market Session
// from 20:00 to 17:00 ET on 2020-10-05 and restored the 20:00 close on
// 2023-02-01. Its own 2025-06-06 retrospective identifies 2025-05-19 as the
// actual launch of its 04:00 ET pre-market. The earlier rule filing proposed a
// March date, so the exchange's stated production launch is the operative
// boundary.
// https://memx.com/insights/day-1
// https://info.memxtrading.com/trader-alert-20-06-memx-market-hours-change/
// https://www.sec.gov/files/rules/sro/memx/2023/34-96773.pdf
// https://info.memxtrading.com/trader-alert-23-04-memx-trading-hours-change/
// https://memx.com/insights/pre-market-share-gains-and-new-options-active-risk-feature
// Row evidence:
//   2020-09-21 "MEMX Day 1 retrospective"
//     https://memx.com/insights/day-1
//   2020-10-05 "MEMX trader alert 20-06"
//     https://info.memxtrading.com/trader-alert-20-06-memx-market-hours-change/
//   2023-02-01 "MEMX trader alert 23-04"
//     https://info.memxtrading.com/trader-alert-23-04-memx-trading-hours-change/
//     https://www.sec.gov/files/rules/sro/memx/2023/34-96773.pdf
//   2025-05-19 "MEMX retrospective 2025-06-06"
//     https://memx.com/insights/pre-market-share-gains-and-new-options-active-risk-feature
static MEMX_REVISIONS: &[Revision] = revisions![
    (
        2020,
        9,
        21,
        &MEMX_EQ_PROFILE_PRE_2025_05_19,
        "MEMX Day 1 retrospective"
    ),
    (
        2020,
        10,
        5,
        &MEMX_EQ_PROFILE_2020_10_05,
        "MEMX trader alert 20-06"
    ),
    (
        2023,
        2,
        1,
        &MEMX_EQ_PROFILE_PRE_2025_05_19,
        "MEMX trader alert 23-04"
    ),
    (
        2025,
        5,
        19,
        &MEMX_EQ_PROFILE,
        "MEMX retrospective 2025-06-06"
    ),
];

pub(crate) fn memx_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        MEMX_REVISIONS,
    )
}

// MIAX Pearl Equities launched on 2020-09-29. Regulatory Circular 2025-02
// later made the Early Trading Session (04:00–09:30 ET) and Late Trading
// Session (16:00–20:00 ET) available beginning 2025-02-20. Before that
// amendment the exchange-level profile contains Regular Trading Hours only.
// https://www.miaxglobal.com/company/markets/us-equities
// https://www.miaxglobal.com/sites/default/files/circular-files/MIAX_Pearl_Equities_RC_2025_02_0.pdf
// Row evidence:
//   2020-09-29 "MIAX Pearl Equities launch notice"
//     https://www.miaxglobal.com/company/markets/us-equities
//   2025-02-20 "MIAX Pearl Regulatory Circular 2025-02"
//     https://www.miaxglobal.com/sites/default/files/circular-files/MIAX_Pearl_Equities_RC_2025_02_0.pdf
static MIAX_PEARL_REVISIONS: &[Revision] = revisions![
    (
        2020,
        9,
        29,
        &MIAX_PEARL_EQ_PROFILE_PRE_2025_02_20,
        "MIAX Pearl Equities launch notice"
    ),
    (
        2025,
        2,
        20,
        &MIAX_PEARL_EQ_PROFILE,
        "MIAX Pearl Regulatory Circular 2025-02"
    ),
];

pub(crate) fn miax_pearl_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &CLOSED_NEW_YORK,
        MIAX_PEARL_REVISIONS,
    )
}
