// SPDX-License-Identifier: MIT-0

//! Nasdaq Stockholm, Helsinki, and Copenhagen principal shares.
//!
//! The books synchronise on CET, but Helsinki publishes one-hour-later local
//! values and Copenhagen has a shorter continuous session.

use chrono_tz::{Europe, Tz};

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// Nasdaq's Jan-2010 INET notices made the migration effective 2010-02-08 and
// explicitly state that it did not materially change Nordic trading hours.
// They also identify the existing randomized close. The operator's 2015 INET
// notice says the opening uncross had previously occurred exactly at 09:00 CET
// and introduced its five-second randomization effective 2015-11-16.
// https://www.globenewswire.com/news-release/2010/01/25/151379/0/en/INET-Nordic-NASDAQ-OMX-Market-Model.html
// https://www.globenewswire.com/news-release/2010/02/05/153049/0/en/INET-Activities-re-migration-starts-today-5-February-at-17-30-CET.html
// The operator-authored Market Model 1.1 attached to the migration notice gives
// every exact Jan-2010 phase below, including post-trading through 18:00 CET
// (17:20 CET for Copenhagen).
// https://www.globenewswire.com/en/Attachment/DownloadAttachment?articleid=153059&fileId=93908&filename=market+model+version+1_1+januar+21+2010.pdf&filetype=3&islogo=0
// https://www.globenewswire.com/news-release/2015/11/16/787323/0/en/IT-INET-REMINDER-Introduction-of-functional-changes-to-INET-auctions-61-15.html
// The current Market Model confirms the resulting five-second opening edge and
// each principal-share continuous and closing phase used below.
// https://www.nasdaq.com/docs/2026/06/17/Nasdaq_Nordic_Market_Model_2026_03_Clean.pdf

static STO_BASE_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 17 * 3600 + 25 * 60,
}];
static STO_BASE_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 25 * 60,
        close_ssm: 17 * 3600 + 30 * 60,
    },
    // Post-trading: cancellation, limited order updates, and manual trades.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 18 * 3600,
    },
];
static STO_BASE_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Stockholm,
    regular: STO_BASE_REGULAR,
    extended: STO_BASE_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
static STO_CURRENT_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 5,
    close_ssm: 17 * 3600 + 25 * 60,
}];
static STO_CURRENT_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600 + 5,
    },
    STO_BASE_EXTENDED[1],
    STO_BASE_EXTENDED[2],
];
pub(crate) static NASDAQ_STO_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Stockholm,
    regular: STO_CURRENT_REGULAR,
    extended: STO_CURRENT_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static HEL_BASE_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 10 * 3600,
    close_ssm: 18 * 3600 + 25 * 60,
}];
static HEL_BASE_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 10 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 18 * 3600 + 25 * 60,
        close_ssm: 18 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 18 * 3600 + 30 * 60,
        close_ssm: 19 * 3600,
    },
];
static HEL_BASE_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Helsinki,
    regular: HEL_BASE_REGULAR,
    extended: HEL_BASE_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
static HEL_CURRENT_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 10 * 3600 + 5,
    close_ssm: 18 * 3600 + 25 * 60,
}];
static HEL_CURRENT_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 10 * 3600 + 5,
    },
    HEL_BASE_EXTENDED[1],
    HEL_BASE_EXTENDED[2],
];
pub(crate) static NASDAQ_HEL_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Helsinki,
    regular: HEL_CURRENT_REGULAR,
    extended: HEL_CURRENT_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static CPH_BASE_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 16 * 3600 + 55 * 60,
}];
static CPH_BASE_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 55 * 60,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600,
        close_ssm: 17 * 3600 + 20 * 60,
    },
];
static CPH_BASE_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Copenhagen,
    regular: CPH_BASE_REGULAR,
    extended: CPH_BASE_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
static CPH_RANDOM_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 5,
    close_ssm: 16 * 3600 + 55 * 60,
}];
static CPH_RANDOM_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 9 * 3600 + 5,
    },
    CPH_BASE_EXTENDED[1],
    CPH_BASE_EXTENDED[2],
];
static CPH_RANDOM_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Copenhagen,
    regular: CPH_RANDOM_REGULAR,
    extended: CPH_RANDOM_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// Nasdaq Copenhagen launched executable Trading@Closing Price for its main
// market on 2019-05-01, adding the executable 17:00-17:10 CET phase before
// post-trading resumes through 17:20. The current Nordic Market Model confirms
// all three principal-share grids and the five-second randomized opening edge.
// https://view.news.eu.nasdaq.com/view?id=b6276fe1aed34c7412a4d454976025d2d&lang=da
// https://www.nasdaq.com/docs/2026/06/17/Nasdaq_Nordic_Market_Model_2026_03_Clean.pdf
static CPH_CURRENT_EXTENDED: &[SessionRule] = &[
    CPH_RANDOM_EXTENDED[0],
    CPH_RANDOM_EXTENDED[1],
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600,
        close_ssm: 17 * 3600 + 10 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 10 * 60,
        close_ssm: 17 * 3600 + 20 * 60,
    },
];
pub(crate) static NASDAQ_CPH_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Copenhagen,
    regular: CPH_RANDOM_REGULAR,
    extended: CPH_CURRENT_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

fn randomized_open_profile(
    as_of: chrono::DateTime<chrono::Utc>,
    tz: Tz,
    baseline: &'static StaticHoursProfile,
    current: &'static StaticHoursProfile,
) -> &'static StaticHoursProfile {
    let revisions = [Revision {
        effective: effective_date(2015, 11, 16),
        profile: current,
    }];
    select_revision(local_date(as_of, tz), baseline, &revisions)
}

pub(crate) fn stockholm_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    randomized_open_profile(
        as_of,
        Europe::Stockholm,
        &STO_BASE_PROFILE,
        &NASDAQ_STO_PROFILE,
    )
}

pub(crate) fn helsinki_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    randomized_open_profile(
        as_of,
        Europe::Helsinki,
        &HEL_BASE_PROFILE,
        &NASDAQ_HEL_PROFILE,
    )
}

static CPH_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2015, 11, 16),
        profile: &CPH_RANDOM_PROFILE,
    },
    Revision {
        effective: effective_date(2019, 5, 1),
        profile: &NASDAQ_CPH_PROFILE,
    },
];

pub(crate) fn copenhagen_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Europe::Copenhagen),
        &CPH_BASE_PROFILE,
        CPH_REVISIONS,
    )
}
