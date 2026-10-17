// SPDX-License-Identifier: MIT-0

//! OANDA Asia CFD books. Evidence: `docs/evidence/oanda_china50_cfd.md`
//! and `docs/evidence/oanda_singapore30_cfd.md`.

use crate::calendar::SessionRule;
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{effective_date, local_date};
use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Asia::Singapore;

const MON_FRI: [bool; 7] = [true, true, true, true, true, false, false];

// The Singapore books' changeover, dated by the operator's own machine
// channel (`MARKET-DATA`, T2, artifacts retained privately): the legacy
// closes tick deep into their final minutes through Friday 2024-02-09, and
// the current grid leads its published opens from Monday 2024-02-12.
// Opening-day keying never splits a running session (LAW-NO-FABRICATED-DATES)
// — docs/evidence/oanda_china50_cfd.md holds the sweep.
const SINGAPORE_CURRENT_FIRST_OPEN_DAY: NaiveDate = effective_date(2024, 2, 12);

// No separately declared regular session; the published executable blocks are extended.
// Evidence: docs/evidence/oanda_china50_cfd.md
pub(crate) static OANDA_CHINA50_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 60,
        close_ssm: 16 * 3600 + 29 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 60,
        close_ssm: 4 * 3600 + 44 * 60,
    },
];

// The earlier `09:00 - 16:30; 17:00 - 04:45` grid, witnessed
// 2021-03-03..2023-06-05 by the archived page prints and through the
// 2024-02-09 opening day by the operator's machine channel.
// Evidence: docs/evidence/oanda_china50_cfd.md
pub(crate) static OANDA_CHINA50_LEGACY_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 16 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600,
        close_ssm: 4 * 3600 + 45 * 60,
    },
];

// No separately declared regular session; the published executable blocks are extended.
// Evidence: docs/evidence/oanda_singapore30_cfd.md
pub(crate) static OANDA_SINGAPORE30_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 31 * 60,
        close_ssm: 17 * 3600 + 19 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 51 * 60,
        close_ssm: 5 * 3600 + 14 * 60,
    },
];

// The earlier `08:30 - 17:20; 17:50 - 04:45` grid, witnessed
// 2021-03-03..2023-06-05 by the archived page prints and through the
// 2024-02-09 opening day by the operator's machine channel.
// Evidence: docs/evidence/oanda_singapore30_cfd.md
pub(crate) static OANDA_SINGAPORE30_LEGACY_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 20 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 50 * 60,
        close_ssm: 4 * 3600 + 45 * 60,
    },
];

static CHINA50: StaticHoursProfile = StaticHoursProfile {
    tz: Singapore,
    regular: &[],
    extended: OANDA_CHINA50_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
static CHINA50_LEGACY: StaticHoursProfile = StaticHoursProfile {
    tz: Singapore,
    regular: &[],
    extended: OANDA_CHINA50_LEGACY_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
static SINGAPORE30: StaticHoursProfile = StaticHoursProfile {
    tz: Singapore,
    regular: &[],
    extended: OANDA_SINGAPORE30_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
static SINGAPORE30_LEGACY: StaticHoursProfile = StaticHoursProfile {
    tz: Singapore,
    regular: &[],
    extended: OANDA_SINGAPORE30_LEGACY_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) fn oanda_china50_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, Singapore);
    if day >= SINGAPORE_CURRENT_FIRST_OPEN_DAY {
        &CHINA50
    } else {
        &CHINA50_LEGACY
    }
}
pub(crate) fn oanda_singapore30_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, Singapore);
    if day >= SINGAPORE_CURRENT_FIRST_OPEN_DAY {
        &SINGAPORE30
    } else {
        &SINGAPORE30_LEGACY
    }
}
