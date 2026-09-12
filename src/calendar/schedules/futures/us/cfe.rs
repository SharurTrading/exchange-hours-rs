// SPDX-License-Identifier: MIT-0

//! Cboe Futures Exchange VIX futures schedules.
//!
//! Narrative evidence, sources and residual risks: `docs/evidence/cfe.md` and
//! `docs/evidence/cfe_vix.md` (LAW-EVIDENCE-FILES).

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

const MON_THU: [bool; 7] = [true, true, true, true, false, false, false];
const SUNDAY: [bool; 7] = [false, false, false, false, false, false, true];

const fn profile(
    regular: &'static [SessionRule],
    extended: &'static [SessionRule],
    order_entry: &'static [SessionRule],
) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: US::Central,
        regular,
        extended,
        order_entry,
        has_daily_close: true,
        has_weekend_close: true,
    }
}

// Current schedule, effective 2021-12-06 (Cboe notice C2021102603): RTH
// 08:30-15:00 CT; ETH 15:00-16:00 and, Sunday plus Monday-Thursday, 17:00-08:30.
pub(crate) static CFE_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 15 * 3600,
}];
pub(crate) static CFE_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
];
// The Sunday 16:00-17:00 and Monday-Thursday 16:45-17:00 windows are CFE's
// "order-entry queues": orders rest, nothing matches, so they are `order_entry`.
// Their starts are randomized through six seconds after the nominal boundary and
// each follows a closed or suspended period, so the profile uses the
// conservative latest 16:00:06 and 16:45:06 edges.
pub(crate) static CFE_ORDER_ENTRY: &[SessionRule] = &[
    SessionRule {
        days: SUNDAY,
        open_ssm: 16 * 3600 + 6,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60 + 6,
        close_ssm: 17 * 3600,
    },
];
static CFE_PROFILE: StaticHoursProfile = profile(CFE_REGULAR, CFE_EXTENDED, CFE_ORDER_ENTRY);

static CFE_REGULAR_0830_1515: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 15 * 3600 + 15 * 60,
}];

// January-2010 floor grid: VX traded 08:30-15:15 CT, with a 07:20-08:30 extended
// session added 2010-12-10 and its start moved to 07:00 on 2011-09-26. RTH is
// unchanged across both.
static CFE_PROFILE_AT_2010_FLOOR: StaticHoursProfile = profile(CFE_REGULAR_0830_1515, &[], &[]);
static CFE_EXT_2010_12_10: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 20 * 60,
    close_ssm: 8 * 3600 + 30 * 60,
}];
static CFE_PROFILE_2010_12_10: StaticHoursProfile =
    profile(CFE_REGULAR_0830_1515, CFE_EXT_2010_12_10, &[]);

// The 2013 two-phase expansion: the Monday-Thursday 15:30-16:15 session on
// 2013-10-28, then the 07:00 morning open moved to 02:00 on 2013-11-04. RTH
// remained 08:30-15:15 throughout both.
static CFE_EXT_PRE_2013_10_28: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600,
    close_ssm: 8 * 3600 + 30 * 60,
}];
// A Monday-Thursday "pre-open queue": orders rest, nothing matches.
static CFE_ORDER_ENTRY_1529: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 15 * 3600 + 29 * 60,
    close_ssm: 15 * 3600 + 30 * 60,
}];
static CFE_EXT_2013_10_28: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 15 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 15 * 60,
    },
];
static CFE_EXT_2013_11_04: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 2 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 15 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 15 * 60,
    },
];
static CFE_PROFILE_PRE_2013_10_28: StaticHoursProfile =
    profile(CFE_REGULAR_0830_1515, CFE_EXT_PRE_2013_10_28, &[]);
static CFE_PROFILE_2013_10_28: StaticHoursProfile = profile(
    CFE_REGULAR_0830_1515,
    CFE_EXT_2013_10_28,
    CFE_ORDER_ENTRY_1529,
);
static CFE_PROFILE_2013_11_04: StaticHoursProfile = profile(
    CFE_REGULAR_0830_1515,
    CFE_EXT_2013_11_04,
    CFE_ORDER_ENTRY_1529,
);

// Nearly-24-hour VX trading from Sunday 2014-06-22, adding the 16:15-17:00
// Sunday pre-open and retaining the 15:29-15:30 weekday one.
static CFE_EXT_2014_06_22: &[SessionRule] = &[
    SessionRule {
        days: SUNDAY,
        open_ssm: 17 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 15 * 3600 + 30 * 60,
        close_ssm: 8 * 3600 + 30 * 60,
    },
];
// Both phases are pre-opens: orders queue, nothing matches.
static CFE_ORDER_ENTRY_2014_06_22: &[SessionRule] = &[
    SessionRule {
        days: SUNDAY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 15 * 3600 + 29 * 60,
        close_ssm: 15 * 3600 + 30 * 60,
    },
];
static CFE_PROFILE_2014_06_22: StaticHoursProfile = profile(
    CFE_REGULAR_0830_1515,
    CFE_EXT_2014_06_22,
    CFE_ORDER_ENTRY_2014_06_22,
);

// The 2018-02-25 system migration: 08:30-15:15 RTH, a 15:15-15:30
// order-entry-only queue, 15:30-16:00 ETH, a 16:00-16:45 weekday suspension, a
// 16:45-17:00 queue, then 17:00-08:30 ETH, with a Sunday 16:00-17:00 opening
// queue. Only the two ETH windows match; the new-system queues start within
// three seconds, so their conservative edges are 16:00:03 and 16:45:03.
static CFE_EXT_2018_02_25: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 30 * 60,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
];
static CFE_ORDER_ENTRY_2018_02_25: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 15 * 60,
        close_ssm: 15 * 3600 + 30 * 60,
    },
    SessionRule {
        days: SUNDAY,
        open_ssm: 16 * 3600 + 3,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60 + 3,
        close_ssm: 17 * 3600,
    },
];
static CFE_PROFILE_2018_02_25: StaticHoursProfile = profile(
    CFE_REGULAR_0830_1515,
    CFE_EXT_2018_02_25,
    CFE_ORDER_ENTRY_2018_02_25,
);

// From 2018-08-12 TAS queue commencement runs three to six seconds after the
// nominal boundary, so the all-contract conservative edge advances to six
// seconds. Only the seconds change, so the matching grid stays
// `CFE_EXT_2018_02_25`.
static CFE_ORDER_ENTRY_2018_08_12: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 15 * 60,
        close_ssm: 15 * 3600 + 30 * 60,
    },
    SessionRule {
        days: SUNDAY,
        open_ssm: 16 * 3600 + 6,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60 + 6,
        close_ssm: 17 * 3600,
    },
];
static CFE_PROFILE_2018_08_12: StaticHoursProfile = profile(
    CFE_REGULAR_0830_1515,
    CFE_EXT_2018_02_25,
    CFE_ORDER_ENTRY_2018_08_12,
);

// Evidence: docs/evidence/cfe.md, docs/evidence/cfe_vix.md
static CFE_REVISIONS: &[Revision] = revisions![
    // 2010-12-10 — T1 — Cboe SR-CFE-2010-013 — 07:20-08:30 CT extended session added.
    (
        2010,
        12,
        10,
        &CFE_PROFILE_2010_12_10,
        "Cboe SR-CFE-2010-013"
    ),
    // 2011-09-26 — T1 — Cboe SR-CFE-2011-019 — extended start moved to 07:00 CT.
    (
        2011,
        9,
        26,
        &CFE_PROFILE_PRE_2013_10_28,
        "Cboe SR-CFE-2011-019"
    ),
    // 2013-10-28 — T1 — Cboe IC13-041 — Monday-Thursday 15:30-16:15 CT session.
    (2013, 10, 28, &CFE_PROFILE_2013_10_28, "Cboe IC13-041"),
    // 2013-11-04 — T1 — Cboe IC13-041 — morning open moved from 07:00 to 02:00 CT.
    (2013, 11, 4, &CFE_PROFILE_2013_11_04, "Cboe IC13-041"),
    // 2014-06-22 — T1 — Cboe RG-CFE-2014-020 — near-24-hour trading; Sunday pre-open.
    (2014, 6, 22, &CFE_PROFILE_2014_06_22, "Cboe RG-CFE-2014-020"),
    // 2018-02-25 — T1 — Cboe RG-CFE-2018-005 — system migration, keyed to the
    // Sunday implementation; the revised weekday hours first occur on CFE's
    // Monday 2018-02-26 business date.
    (2018, 2, 25, &CFE_PROFILE_2018_02_25, "Cboe RG-CFE-2018-005"),
    // 2018-08-12 — T1 — Cboe C2018071603 — queue edges widen to six seconds.
    (2018, 8, 12, &CFE_PROFILE_2018_08_12, "Cboe C2018071603"),
    // 2021-12-06 — T1 — Cboe C2021102603 — current grid; 15:15-15:30 queue removed.
    (2021, 12, 6, &CFE_PROFILE, "Cboe C2021102603"),
];

pub(crate) fn cfe_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, US::Central),
        &CFE_PROFILE_AT_2010_FLOOR,
        CFE_REVISIONS,
    )
}
