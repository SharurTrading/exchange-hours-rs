// SPDX-License-Identifier: MIT-0

//! Philippine Stock Exchange cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static PSE_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 30 * 60,
        close_ssm: 12 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600,
        close_ssm: 14 * 3600 + 45 * 60,
    },
];
static PSE_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 45 * 60,
        close_ssm: 15 * 3600 + 15 * 60,
    },
];
// https://www.pse.com.ph/investing-at-pse/
// VWAP extended the close-side envelope effective 2024-03-01:
// https://documents.pse.com.ph/CircularOPSPDF/CN-2024-0012.pdf
pub(crate) static PSE_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Manila,
    regular: PSE_REGULAR_CURRENT,
    extended: PSE_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static PSE_PREOPEN: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 9 * 3600 + 30 * 60,
};

// PSE's primary circulars give each exact effective date and phase table.
// Baseline and 2011 phase one:
// https://documents.pse.com.ph/AnnouncementOPSPDF/Proposed%20amendment%20of%20Trading%20Rules%20in%20relation%20to%20Extended%20Trading.pdf
// https://documents.pse.com.ph/CircularOPSPDF/CN-2011-0013.pdf
static PSE_REGULAR_BASELINE: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 11 * 3600 + 57 * 60,
}];
static PSE_EXTENDED_BASELINE: &[SessionRule] = &[
    PSE_PREOPEN,
    SessionRule {
        days: MON_FRI,
        open_ssm: 11 * 3600 + 57 * 60,
        close_ssm: 12 * 3600 + 10 * 60,
    },
];
static PSE_REGULAR_PHASE_ONE: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 12 * 3600 + 47 * 60,
}];
static PSE_EXTENDED_PHASE_ONE: &[SessionRule] = &[
    PSE_PREOPEN,
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600 + 47 * 60,
        close_ssm: 13 * 3600,
    },
];

// Full morning/afternoon schedule effective 2012-01-02; its pre-close moved
// two minutes earlier on 2013-11-04.
// https://documents.pse.com.ph/AnnouncementOPSPDF/PSE%20New%20Trading%20Hours.pdf
// https://documents.pse.com.ph/wp-content/uploads/sites/15/2024/08/4_Extended-Pre-Close_TPA_2013-0185.pdf
static PSE_REGULAR_2012: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 30 * 60,
        close_ssm: 12 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 30 * 60,
        close_ssm: 15 * 3600 + 17 * 60,
    },
];
static PSE_EXTENDED_2012: &[SessionRule] = &[
    PSE_PREOPEN,
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 17 * 60,
        close_ssm: 15 * 3600 + 30 * 60,
    },
];
static PSE_REGULAR_2013: &[SessionRule] = &[
    PSE_REGULAR_2012[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 30 * 60,
        close_ssm: 15 * 3600 + 15 * 60,
    },
];
static PSE_EXTENDED_2013: &[SessionRule] = &[
    PSE_PREOPEN,
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 15 * 60,
        close_ssm: 15 * 3600 + 30 * 60,
    },
];

// Short pandemic/Omicron profile and the restored five-hour full-day profile.
// https://documents.pse.com.ph/CircularOPSPDF/CN-2020-0017.pdf
// https://documents.pse.com.ph/CircularOPSPDF/CN-2020-0025.pdf
// https://documents.pse.com.ph/CircularOPSPDF/CN-2021-0059.pdf
// https://documents.pse.com.ph/wp-content/uploads/sites/15/2022/01/CN_2022-0004.pdf
// https://documents.pse.com.ph/wp-content/uploads/sites/15/2022/01/CN_2022-0007.pdf
static PSE_REGULAR_SHORT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 12 * 3600 + 45 * 60,
}];
static PSE_EXTENDED_SHORT: &[SessionRule] = &[
    PSE_PREOPEN,
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600 + 45 * 60,
        close_ssm: 13 * 3600,
    },
];
static PSE_REGULAR_FULL_DAY: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 30 * 60,
        close_ssm: 12 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600,
        close_ssm: 14 * 3600 + 45 * 60,
    },
];
static PSE_EXTENDED_FULL_DAY: &[SessionRule] = &[
    PSE_PREOPEN,
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 45 * 60,
        close_ssm: 15 * 3600,
    },
];

macro_rules! pse_profile {
    ($name:ident, $regular:ident, $extended:ident) => {
        pub(crate) static $name: StaticHoursProfile = StaticHoursProfile {
            tz: Asia::Manila,
            regular: $regular,
            extended: $extended,
            order_entry: &[],
            has_daily_close: true,
            has_weekend_close: true,
        };
    };
}

pse_profile!(
    PSE_PROFILE_BASELINE,
    PSE_REGULAR_BASELINE,
    PSE_EXTENDED_BASELINE
);
pse_profile!(
    PSE_PROFILE_PHASE_ONE,
    PSE_REGULAR_PHASE_ONE,
    PSE_EXTENDED_PHASE_ONE
);
pse_profile!(PSE_PROFILE_2012, PSE_REGULAR_2012, PSE_EXTENDED_2012);
pse_profile!(PSE_PROFILE_2013, PSE_REGULAR_2013, PSE_EXTENDED_2013);
pse_profile!(PSE_PROFILE_SHORT, PSE_REGULAR_SHORT, PSE_EXTENDED_SHORT);
pse_profile!(
    PSE_PROFILE_FULL_DAY,
    PSE_REGULAR_FULL_DAY,
    PSE_EXTENDED_FULL_DAY
);

// PSE suspended all trading on 2020-03-17 and resumed on 2020-03-19, making
// March 17–18 full no-session dates.
// https://documents.pse.com.ph/CircularOPSPDF/CN-2020-0021.pdf
// https://documents.pse.com.ph/CircularOPSPDF/CN-2020-0025.pdf
pub(crate) static PSE_PROFILE_CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Manila,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &PSE_PROFILE_CURRENT;

static REVISIONS: &[Revision] = revisions![
    (
        2011,
        10,
        1,
        &PSE_PROFILE_PHASE_ONE,
        "PSE circular CN-2011-0013"
    ),
    (
        2012,
        1,
        2,
        &PSE_PROFILE_2012,
        "PSE announcement New Trading Hours"
    ),
    (2013, 11, 4, &PSE_PROFILE_2013, "PSE advisory TPA 2013-0185"),
    (2020, 3, 16, &PSE_PROFILE_SHORT, "PSE circular CN-2020-0017"),
    (
        2020,
        3,
        17,
        &PSE_PROFILE_CLOSED,
        "PSE circular CN-2020-0021"
    ),
    (2020, 3, 19, &PSE_PROFILE_SHORT, "PSE circular CN-2020-0025"),
    (
        2021,
        12,
        6,
        &PSE_PROFILE_FULL_DAY,
        "PSE circular CN-2021-0059"
    ),
    (2022, 1, 14, &PSE_PROFILE_SHORT, "PSE circular CN-2022-0004"),
    (
        2022,
        2,
        2,
        &PSE_PROFILE_FULL_DAY,
        "PSE circular CN-2022-0007"
    ),
    (
        2024,
        3,
        1,
        &PSE_PROFILE_CURRENT,
        "PSE circular CN-2024-0012"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &PSE_PROFILE_BASELINE,
        REVISIONS,
    )
}
