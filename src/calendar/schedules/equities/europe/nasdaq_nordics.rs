// SPDX-License-Identifier: MIT-0

//! Nasdaq Stockholm, Helsinki, and Copenhagen principal shares.
//!
//! The books synchronise on CET, but Helsinki publishes one-hour-later local
//! values and Copenhagen has a shorter continuous session.

use chrono_tz::{Europe, Tz};

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// The January-2010 baseline phases, the February-2010 INET migration and the
// 2015 five-second opening randomization are evidenced in the anchor file.
// Narrative: docs/evidence/nasdaq_stockholm.md

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

// Copenhagen's executable 17:00-17:10 CET Trading@Closing Price phase.
// Narrative: docs/evidence/nasdaq_stockholm.md
//   This key's own evidence file is docs/evidence/nasdaq_copenhagen.md
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

// 2015-11-16 — T1 — Nasdaq INET notice 61/15 — five-second randomization
//   moves the earliest continuous-trading edge to 09:00:05.
// Evidence: docs/evidence/nasdaq_stockholm.md
static STO_REVISIONS: &[Revision] = revisions![(
    2015,
    11,
    16,
    &NASDAQ_STO_PROFILE,
    "Nasdaq INET notice 61/15"
),];

// 2015-11-16 — T1 — Nasdaq INET notice 61/15 — five-second randomization
//   moves the earliest continuous-trading edge to 10:00:05 Helsinki local time.
// Evidence: docs/evidence/nasdaq_helsinki.md
static HEL_REVISIONS: &[Revision] = revisions![(
    2015,
    11,
    16,
    &NASDAQ_HEL_PROFILE,
    "Nasdaq INET notice 61/15"
),];

fn randomized_open_profile(
    as_of: chrono::DateTime<chrono::Utc>,
    tz: Tz,
    baseline: &'static StaticHoursProfile,
    revisions: &[Revision],
) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, tz), baseline, revisions)
}

pub(crate) fn stockholm_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    randomized_open_profile(as_of, Europe::Stockholm, &STO_BASE_PROFILE, STO_REVISIONS)
}

pub(crate) fn helsinki_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    randomized_open_profile(as_of, Europe::Helsinki, &HEL_BASE_PROFILE, HEL_REVISIONS)
}

// 2015-11-16 — T1 — Nasdaq INET notice 61/15 — five-second randomization
//   moves the earliest continuous-trading edge to 09:00:05.
// 2019-05-01 — T1 — Nasdaq Copenhagen Trading@Closing Price announcement —
//   executable Trading@Closing Price, 17:00-17:10 CET, before post-trading
//   resumes through 17:20.
// Evidence: docs/evidence/nasdaq_copenhagen.md
static CPH_REVISIONS: &[Revision] = revisions![
    (
        2015,
        11,
        16,
        &CPH_RANDOM_PROFILE,
        "Nasdaq INET notice 61/15"
    ),
    (
        2019,
        5,
        1,
        &NASDAQ_CPH_PROFILE,
        "Nasdaq Copenhagen Trading@Closing Price announcement"
    ),
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
