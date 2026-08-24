// SPDX-License-Identifier: MIT-0

//! Australian Securities Exchange cash equities.

use chrono_tz::Australia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// ASX cash market, from ASX Operating Rules Procedures Appendix 4013 and the
// cash-market hours page. Service Release 15 replaced symbol-group staggered
// opens with one randomized 09:59:45–10:00 opening and added Post Close on
// 2025-06-23. A deterministic venue default conservatively starts continuous
// trading at 10:00; the opening process and close-side trading are extended.
// Sources:
// https://www.asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours
// https://www.asxonline.com/public/notices/2025/may/0473.25.05.html
static ASX_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 10 * 3600,
    close_ssm: 16 * 3600,
}];

static ASX_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600,
        close_ssm: 10 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 16 * 3600 + 21 * 60 + 30,
    },
];

// Before Service Release 15, five symbol groups opened at nominal times from
// 10:00 through 10:09, each randomized by +/- 15 seconds, and the CSPA ended
// at 16:12. Regular starts at the venue's earliest continuous-trading edge;
// the overlapping extended rule preserves the opening-auction envelope through
// the latest possible Group 5 transition at 10:09:15.
// Source: ASX SR15 marked operating-rule procedure amendments:
// https://www.asxonline.com/content/dam/asxonline/public/notices/2025/april/asx-sr15asx-operating-rule-procedure-amendments.pdf
static ASX_EXTENDED_PRE_2025_06_23: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600,
        close_ssm: 10 * 3600 + 9 * 60 + 15,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 16 * 3600 + 12 * 60,
    },
];

pub(crate) static ASX_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Australia::Sydney,
    regular: ASX_REGULAR,
    extended: ASX_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

pub(crate) static ASX_PROFILE_PRE_2025_06_23: StaticHoursProfile = StaticHoursProfile {
    tz: Australia::Sydney,
    regular: ASX_REGULAR,
    extended: ASX_EXTENDED_PRE_2025_06_23,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &ASX_PROFILE_CURRENT;

static REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2025, 6, 23),
    profile: &ASX_PROFILE_CURRENT,
}];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &ASX_PROFILE_PRE_2025_06_23,
        REVISIONS,
    )
}
