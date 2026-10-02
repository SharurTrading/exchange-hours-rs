// SPDX-License-Identifier: MIT-0

//! BMV cash-equity grids, including its New York reference-zone regime.

use chrono_tz::America;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static BMV_REGULAR_NORMAL: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 15 * 3600,
}];
static BMV_REGULAR_EARLY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 30 * 60,
    close_ssm: 14 * 3600,
}];
// No BMV phase is order-entry-only, so `order_entry` stays empty on every
// profile below. The window before the open is the venue's opening-auction
// stage — the cancellation-only setup that precedes it is already excluded
// from the model — and the HD/ID tail is executable, so both print.
static BMV_EXTENDED_NORMAL_PRE_2016: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 60,
        close_ssm: 15 * 3600 + 6 * 60,
    },
];
static BMV_EXTENDED_EARLY_PRE_2016: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600,
        close_ssm: 7 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 60,
        close_ssm: 14 * 3600 + 6 * 60,
    },
];
static BMV_EXTENDED_NORMAL_POST_2016: &[SessionRule] = &[
    BMV_EXTENDED_NORMAL_PRE_2016[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 60,
        close_ssm: 15 * 3600 + 10 * 60,
    },
];
static BMV_EXTENDED_EARLY_POST_2016: &[SessionRule] = &[
    BMV_EXTENDED_EARLY_PRE_2016[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 60,
        close_ssm: 14 * 3600 + 10 * 60,
    },
];
static BMV_EXTENDED_NORMAL_CURRENT: &[SessionRule] = &[
    BMV_EXTENDED_NORMAL_PRE_2016[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 60,
        close_ssm: 15 * 3600 + 20 * 60,
    },
];
static BMV_EXTENDED_EARLY_CURRENT: &[SessionRule] = &[
    BMV_EXTENDED_EARLY_PRE_2016[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 60,
        close_ssm: 14 * 3600 + 20 * 60,
    },
];

// BMV publishes a normal grid and an hour-earlier grid used to remain aligned with New York. Narrative:
// docs/evidence/bmv.md.
pub(crate) static BMV_PROFILE_NORMAL_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: America::Mexico_City,
    regular: BMV_REGULAR_NORMAL,
    extended: BMV_EXTENDED_NORMAL_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static BMV_PROFILE_EARLY_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: America::Mexico_City,
    regular: BMV_REGULAR_EARLY,
    extended: BMV_EXTENDED_EARLY_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static BMV_PROFILE_NORMAL_POST_2016: StaticHoursProfile = StaticHoursProfile {
    tz: America::Mexico_City,
    regular: BMV_REGULAR_NORMAL,
    extended: BMV_EXTENDED_NORMAL_POST_2016,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
static BMV_PROFILE_EARLY_POST_2016: StaticHoursProfile = StaticHoursProfile {
    tz: America::Mexico_City,
    regular: BMV_REGULAR_EARLY,
    extended: BMV_EXTENDED_EARLY_POST_2016,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
static BMV_PROFILE_EARLY_POST_2023_05_29: StaticHoursProfile = StaticHoursProfile {
    tz: America::Mexico_City,
    regular: BMV_REGULAR_EARLY,
    extended: BMV_EXTENDED_EARLY_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
static BMV_PROFILE_NORMAL_PRE_2016: StaticHoursProfile = StaticHoursProfile {
    tz: America::Mexico_City,
    regular: BMV_REGULAR_NORMAL,
    extended: BMV_EXTENDED_NORMAL_PRE_2016,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
static BMV_PROFILE_EARLY_PRE_2016: StaticHoursProfile = StaticHoursProfile {
    tz: America::Mexico_City,
    regular: BMV_REGULAR_EARLY,
    extended: BMV_EXTENDED_EARLY_PRE_2016,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{
    Revision, effective_date, local_date, reference_delta_seconds, revisions, select_revision,
};

// BMV's 2010 spring notice gives an exact bounded early grid. Narrative:
// docs/evidence/bmv.md.
// Evidence: docs/evidence/bmv.md
static SOURCED_REVISIONS: &[Revision] = revisions![
    (
        2010,
        3,
        16,
        &BMV_PROFILE_EARLY_PRE_2016,
        "BMV DST notice 20100218"
    ),
    (
        2010,
        4,
        1,
        &BMV_PROFILE_NORMAL_PRE_2016,
        "BMV DST notice 20100218"
    ),
];

const REFERENCE_GRID: chrono::NaiveDate = effective_date(2010, 11, 1);
const HD_EXTENSION_2016: chrono::NaiveDate = effective_date(2016, 9, 5);
const EARLY_HD_EXTENSION_2023: chrono::NaiveDate = effective_date(2023, 5, 29);
const NORMAL_HD_EXTENSION_2023: chrono::NaiveDate = effective_date(2023, 11, 6);

// BMV's regulator-filed 2016 report dates the first HD extension to 2016-09-05. Narrative:
// docs/evidence/bmv.md.

fn profile_for_regime(day: chrono::NaiveDate, early: bool) -> &'static StaticHoursProfile {
    if day < HD_EXTENSION_2016 {
        return if early {
            &BMV_PROFILE_EARLY_PRE_2016
        } else {
            &BMV_PROFILE_NORMAL_PRE_2016
        };
    }
    if day < EARLY_HD_EXTENSION_2023 {
        return if early {
            &BMV_PROFILE_EARLY_POST_2016
        } else {
            &BMV_PROFILE_NORMAL_POST_2016
        };
    }
    if day < NORMAL_HD_EXTENSION_2023 {
        return if early {
            &BMV_PROFILE_EARLY_POST_2023_05_29
        } else {
            &BMV_PROFILE_NORMAL_POST_2016
        };
    }
    if early {
        &BMV_PROFILE_EARLY_CURRENT
    } else {
        &BMV_PROFILE_NORMAL_CURRENT
    }
}

/// Resolves BMV's sourced bounded exceptions and recurring offset regime.
pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, America::Mexico_City);
    if day < REFERENCE_GRID {
        return select_revision(day, &BMV_PROFILE_NORMAL_PRE_2016, SOURCED_REVISIONS);
    }
    let early = reference_delta_seconds(as_of, America::Mexico_City, America::New_York) == 2 * 3600;
    profile_for_regime(day, early)
}
