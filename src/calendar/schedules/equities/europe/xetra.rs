// SPDX-License-Identifier: MIT-0

//! Xetra cash equities, represented by the liquid DAX share segment.

use chrono_tz::Europe;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// Deutsche Börse circulars 207/08 and 091/18 bracket the Jan-2010 baseline and
// confirm that the DAX grid remained: pre-trading from 07:30, opening auction
// 08:50-09:00, intraday auction 13:00-13:02, continuous trading to 17:30, and
// closing auction to 17:35, followed by order-entry-only post-trading through
// 20:30. Each auction can end in a 30-second random period, so regular trading
// begins only at the latest possible edge.
// https://cashmarket.deutsche-boerse.com/resource/blob/197910/0890768f3f753299e4c268b80fe7944d/data/207_08e.pdf
// https://www.cashmarket.deutsche-boerse.com/resource/blob/1431340/a23cc3ff15d46a3b649bd23f1618b928/data/091_18e.pdf
static REGULAR: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 30,
        close_ssm: 13 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 2 * 60 + 30,
        close_ssm: 17 * 3600 + 30 * 60,
    },
];
static BASE_EXTENDED: &[SessionRule] = &[
    // Pre-trading.
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 30 * 60,
        close_ssm: 8 * 3600 + 50 * 60,
    },
    // Opening auction.
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 50 * 60,
        close_ssm: 9 * 3600 + 30,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600,
        close_ssm: 13 * 3600 + 2 * 60 + 30,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 35 * 60 + 30,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 35 * 60 + 30,
        close_ssm: 20 * 3600 + 30 * 60,
    },
];
static BASE_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: REGULAR,
    extended: BASE_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

// T7 Release 9.0 entered production on 2020-11-23; the operator's factsheet
// states that Trade-at-Close itself launched on 2020-11-24, one day later. It
// inserted executable trading after the DAX closing auction through 17:45,
// before the existing post-trading period resumed.
// https://www.cashmarket.deutsche-boerse.com/cash-en/Stay-Informed/circulars-newsletters/deutsche-boerse-circulars/Introduction-of-T7-Release-9.0-1978838
// https://www.cashmarket.deutsche-boerse.com/cash-en/Stay-Informed/newsroom/press-releases/Xetra-Trade-at-Close-enables-trading-at-the-official-closing-price-2346762
// https://www.xetra.com/resource/blob/2317768/0e06651d8b87a834e417209504dfd22a/data/Xetra-Trade-at-Close-factsheet_de.pdf
static TAC_EXTENDED: &[SessionRule] = &[
    BASE_EXTENDED[0],
    BASE_EXTENDED[1],
    BASE_EXTENDED[2],
    BASE_EXTENDED[3],
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 35 * 60 + 30,
        close_ssm: 17 * 3600 + 45 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 45 * 60,
        close_ssm: 20 * 3600 + 30 * 60,
    },
];
static TAC_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: REGULAR,
    extended: TAC_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

// Extended Xetra Retail became effective on 2025-12-01. The DAX envelope now
// begins at 07:00, Trade-at-Close ends at 17:40, participant-restricted late
// retail trading continues through 22:00, and post-trading ends at 22:05. The
// non-continuous, order-entry-only, and participant-restricted retail phases
// are `extended`; only the unrestricted continuous phases stay `regular`.
// https://www.cashmarket.deutsche-boerse.com/cash-en/Stay-Informed/circulars-newsletters/deutsche-boerse-circulars/Introduction-of-the-Extended-Xetra-Retail-Service-early-and-late-trading-Planned-changes-to-the-trading-process-valid-from-1-December-2025-4793480
// https://www.cashmarket.deutsche-boerse.com/resource/blob/250890/24d50260d22cd63e0f600ae2543ca529/data/trading-parameters-xetra.pdf
static CURRENT_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600,
        close_ssm: 8 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 8 * 3600 + 55 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 55 * 60,
        close_ssm: 9 * 3600 + 30,
    },
    BASE_EXTENDED[2],
    BASE_EXTENDED[3],
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 35 * 60 + 30,
        close_ssm: 17 * 3600 + 40 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 40 * 60,
        close_ssm: 22 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 22 * 3600,
        close_ssm: 22 * 3600 + 5 * 60,
    },
];
pub(crate) static XETRA_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: REGULAR,
    extended: CURRENT_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

static REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2020, 11, 24),
        profile: &TAC_PROFILE,
    },
    Revision {
        effective: effective_date(2025, 12, 1),
        profile: &XETRA_PROFILE,
    },
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, Europe::Berlin), &BASE_PROFILE, REVISIONS)
}
