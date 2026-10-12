// SPDX-License-Identifier: MIT-0

//! OANDA Asia CFD books. Evidence: `docs/evidence/oanda_china50_cfd.md`
//! and `docs/evidence/oanda_singapore30_cfd.md`.

use crate::calendar::SessionRule;
use crate::calendar::schedules::StaticHoursProfile;
use chrono::{DateTime, Utc};
use chrono_tz::Asia::Singapore;

const MON_FRI: [bool; 7] = [true, true, true, true, true, false, false];

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

static CHINA50: StaticHoursProfile = StaticHoursProfile {
    tz: Singapore,
    regular: &[],
    extended: OANDA_CHINA50_EXTENDED,
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

pub(crate) fn oanda_china50_profile_at(_as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    &CHINA50
}
pub(crate) fn oanda_singapore30_profile_at(_as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    &SINGAPORE30
}
