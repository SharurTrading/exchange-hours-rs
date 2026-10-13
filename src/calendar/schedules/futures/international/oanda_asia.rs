// SPDX-License-Identifier: MIT-0

//! OANDA Asia CFD books. Evidence: `docs/evidence/oanda_china50_cfd.md`
//! and `docs/evidence/oanda_singapore30_cfd.md`.

use crate::calendar::SessionRule;
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{effective_date, local_date};
use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Asia::Singapore;

const MON_FRI: [bool; 7] = [true, true, true, true, true, false, false];

// The UK hours page's era bounds for the Singapore books, keyed on Singapore
// opening days: the last earlier-grid observation (2023-06-05 05:35 UTC =
// 13:35 Singapore, inside that day's day session) and the first current-grid
// observation (2024-02-21 00:55 UTC = 08:55 Singapore, inside that day's day
// session). The undated changeovers serve the sourced intersections; these
// key observations, never effective days — docs/evidence/oanda_china50_cfd.md.
const SINGAPORE_INTERSECTION_FIRST_OPEN_DAY: NaiveDate = effective_date(2023, 6, 6);
const SINGAPORE_CURRENT_FIRST_OPEN_DAY: NaiveDate = effective_date(2024, 2, 21);

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
// 2021-03-03..2023-06-05. The current grid nests inside it, so the undated
// changeover's sourced intersection is the current grid's own shape.
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
// 2021-03-03..2023-06-05.
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

// The undated changeover's sourced intersection: each session bounded at the
// later open and the earlier close of the two witnessed grids — the night
// close stays at the earlier 04:45 rather than adopting the current 05:14.
// Evidence: docs/evidence/oanda_singapore30_cfd.md
pub(crate) static OANDA_SINGAPORE30_INTERSECTION_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 31 * 60,
        close_ssm: 17 * 3600 + 19 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 51 * 60,
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
static SINGAPORE30_INTERSECTION: StaticHoursProfile = StaticHoursProfile {
    tz: Singapore,
    regular: &[],
    extended: OANDA_SINGAPORE30_INTERSECTION_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

pub(crate) fn oanda_china50_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, Singapore);
    // The current grid nests inside the legacy one, so the undated span's
    // sourced intersection is the current profile itself; the span is dated
    // and disclosed in the evidence file, not by a third profile.
    if day >= SINGAPORE_INTERSECTION_FIRST_OPEN_DAY {
        &CHINA50
    } else {
        &CHINA50_LEGACY
    }
}
pub(crate) fn oanda_singapore30_profile_at(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, Singapore);
    if day >= SINGAPORE_CURRENT_FIRST_OPEN_DAY {
        &SINGAPORE30
    } else if day >= SINGAPORE_INTERSECTION_FIRST_OPEN_DAY {
        &SINGAPORE30_INTERSECTION
    } else {
        &SINGAPORE30_LEGACY
    }
}
