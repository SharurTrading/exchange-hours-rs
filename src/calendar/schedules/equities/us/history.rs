// SPDX-License-Identifier: MIT-0

//! Point-in-time selectors for US cash-equity schedules.

use chrono_tz::America;

use super::StaticHoursProfile;
use super::equities::{
    MEMX_EQ_PROFILE, MEMX_EQ_PROFILE_PRE_2025_05_19, MIAX_PEARL_EQ_PROFILE,
    MIAX_PEARL_EQ_PROFILE_PRE_2025_02_20, NASDAQ_BX_PROFILE, NASDAQ_BX_PROFILE_PRE_2011_04_18,
    NASDAQ_PROFILE, NASDAQ_PROFILE_POST_2026_12_06, NASDAQ_PROFILE_PRE_2013_03_18,
    NASDAQ_PSX_PROFILE, NASDAQ_PSX_PROFILE_AT_LAUNCH,
};
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// Nasdaq Equity Trader Alert 2013-21 moved the pre-market open from 07:00 to
// 04:00 ET effective Monday 2013-03-18. Equity Trader Alert 2026-46 announces
// that the Nasdaq Stock Market Night Session will begin Sunday 2026-12-06. The
// operator's living Equity 1 rule gives the Sunday-through-Thursday day mask
// and retains the 20:00–21:00 ET daily pause. This selector intentionally
// applies only to `nasdaq`, not to Nasdaq Texas or PSX.
// https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2013-21
// https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2026-46
// https://listingcenter.nasdaq.com/rulebook/nasdaq/rules/Nasdaq%20Equity%201
static NASDAQ_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2013, 3, 18),
        profile: &NASDAQ_PROFILE,
    },
    Revision {
        effective: effective_date(2026, 12, 6),
        profile: &NASDAQ_PROFILE_POST_2026_12_06,
    },
];

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
static NASDAQ_BX_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2011, 4, 18),
    profile: &NASDAQ_BX_PROFILE,
}];

pub(crate) fn nasdaq_bx_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &NASDAQ_BX_PROFILE_PRE_2011_04_18,
        NASDAQ_BX_REVISIONS,
    )
}

static NASDAQ_PSX_CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: &[],
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// Nasdaq's launch alert dates PSX production to 2010-10-08. The initial rules
// operated 09:00–17:00 ET; SR-Phlx-2010-172 explicitly identifies 2010-12-13
// as the implementation date for the 08:00 ET opening.
// https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2010-56
// https://www.sec.gov/files/rules/sro/phlx/2010/34-63492.pdf
static NASDAQ_PSX_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2010, 10, 8),
        profile: &NASDAQ_PSX_PROFILE_AT_LAUNCH,
    },
    Revision {
        effective: effective_date(2010, 12, 13),
        profile: &NASDAQ_PSX_PROFILE,
    },
];

pub(crate) fn nasdaq_psx_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &NASDAQ_PSX_CLOSED,
        NASDAQ_PSX_REVISIONS,
    )
}

static MEMX_CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: &[],
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// MEMX began live trading on 2020-09-21. Its own 2025-06-06 retrospective
// identifies 2025-05-19 as the actual launch of its 04:00 ET pre-market. The
// earlier rule filing proposed a March date, so the exchange's stated
// production launch is the operative boundary.
// https://memx.com/insights/day-1
// https://memx.com/insights/pre-market-share-gains-and-new-options-active-risk-feature
static MEMX_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2020, 9, 21),
        profile: &MEMX_EQ_PROFILE_PRE_2025_05_19,
    },
    Revision {
        effective: effective_date(2025, 5, 19),
        profile: &MEMX_EQ_PROFILE,
    },
];

pub(crate) fn memx_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &MEMX_CLOSED,
        MEMX_REVISIONS,
    )
}

static MIAX_PEARL_CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: &[],
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// MIAX Pearl Equities launched on 2020-09-29. Regulatory Circular 2025-02
// later made the Early Trading Session (04:00–09:30 ET) and Late Trading
// Session (16:00–20:00 ET) available beginning 2025-02-20. Before that
// amendment the exchange-level profile contains Regular Trading Hours only.
// https://www.miaxglobal.com/company/markets/us-equities
// https://www.miaxglobal.com/sites/default/files/circular-files/MIAX_Pearl_Equities_RC_2025_02_0.pdf
static MIAX_PEARL_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2020, 9, 29),
        profile: &MIAX_PEARL_EQ_PROFILE_PRE_2025_02_20,
    },
    Revision {
        effective: effective_date(2025, 2, 20),
        profile: &MIAX_PEARL_EQ_PROFILE,
    },
];

pub(crate) fn miax_pearl_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &MIAX_PEARL_CLOSED,
        MIAX_PEARL_REVISIONS,
    )
}
