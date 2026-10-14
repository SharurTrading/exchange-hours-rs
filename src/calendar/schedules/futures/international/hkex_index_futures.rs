// SPDX-License-Identifier: MIT-0

//! HKEX index futures (Hang Seng Index futures and the H-shares suite).
//!
//! The family is the HKFE index-futures clock: a Monday-to-Friday day session
//! in two parts, order-entry pre-opening periods before each part, and (from
//! 2013-04-08) the After-Hours Futures Trading T+1 session in `extended`.
//! HKEX names the T+1 session for its own trade-date rule — "Trades executed
//! during the T+1 Session (T+1 Trades) will be registered as the following
//! day's trades" (the AHFT information paper) — which is the crate's own
//! final-close convention: in the current era the session opens 17:00 and
//! closes 03:00 on the following civil day, so its trade date is that
//! following day, and a holiday's Closed row removes the prior evening's leg
//! with the day itself.
//!
//! Dated history: the floor grid is 9:45am-12:30pm and 2:30-4:15pm; the
//! operator-dated two-phase extension moves Phase 1 to 9:15am-12:00 noon and
//! 1:30-4:15pm on 2011-03-07 (the contract page the operator stamped "Updated:
//! 07/03/2011", which also prints Phase 2 "(Effective 5 March 2012)"), Phase 2
//! to an afternoon 1:00-4:15pm with a 12:30-1:00pm pre-opening period on
//! 2012-03-05 (DMD/004/12), and the T+1 session launches 17:00-23:00 on
//! 2013-04-08 (DMD/022/13: "the launch date for AHFT will be on Monday, 8
//! April 2013"). The later cutovers — the T+1 close to 11:45pm, the day close
//! to 4:30pm with the T+1 start to 5:15pm, and the overnight 5:00pm-3:00am
//! shape — are bracketed by dated contract-page snapshots whose changeover
//! days no captured artifact states: the crate serves the sourced
//! intersection across each undated span (the window that holds under both
//! flanking states, so the middle era's T+1 opens 5:15pm before the 2016 page
//! witnesses it), keys each row to the first session it governs, and
//! discloses the disputed remainders as residuals in
//! `docs/evidence/hkex_index_futures.md`.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// --- Floor (to 2011-03-06) ---------------------------------------------------

static HKEX_INDEX_FLOOR_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 45 * 60,
        close_ssm: 12 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 15 * 60,
    },
];

pub(crate) static HKEX_INDEX_FLOOR: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Hong_Kong,
    regular: HKEX_INDEX_FLOOR_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// --- Phase 1 (2011-03-07) ----------------------------------------------------

static HKEX_INDEX_PHASE1_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 15 * 60,
        close_ssm: 12 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 15 * 60,
    },
];

static HKEX_INDEX_PHASE1_ORDER_ENTRY: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 45 * 60,
        close_ssm: 9 * 3600 + 15 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600,
        close_ssm: 13 * 3600 + 30 * 60,
    },
];

static HKEX_INDEX_PHASE1: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Hong_Kong,
    regular: HKEX_INDEX_PHASE1_REGULAR,
    extended: &[],
    order_entry: HKEX_INDEX_PHASE1_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// --- Phase 2 (2012-03-05) ----------------------------------------------------

static HKEX_INDEX_PHASE2_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 15 * 60,
        close_ssm: 12 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600,
        close_ssm: 16 * 3600 + 15 * 60,
    },
];

// The pre-opening periods DMD/004/12 prints for the index futures markets:
// 8:45-9:15am before the morning session and, from Phase 2, 12:30-1:00pm
// before the afternoon session. Both only collect orders.
pub(crate) static HKEX_INDEX_PHASE2_ORDER_ENTRY: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 45 * 60,
        close_ssm: 9 * 3600 + 15 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 12 * 3600 + 30 * 60,
        close_ssm: 13 * 3600,
    },
];

static HKEX_INDEX_PHASE2: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Hong_Kong,
    regular: HKEX_INDEX_PHASE2_REGULAR,
    extended: &[],
    order_entry: HKEX_INDEX_PHASE2_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// --- T+1 session to 11:00pm (2013-04-08) --------------------------------------

static HKEX_INDEX_AHT_2300_EXTENDED: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 17 * 3600,
    close_ssm: 23 * 3600,
}];

static HKEX_INDEX_AHT_2300: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Hong_Kong,
    regular: HKEX_INDEX_PHASE2_REGULAR,
    extended: HKEX_INDEX_AHT_2300_EXTENDED,
    order_entry: HKEX_INDEX_PHASE2_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// --- T+1 to 11:45pm, the sourced intersection (from the 2014 witness) --------
//
// The 03/11/2014 page witnesses the 11:45pm close with the 5:00pm start; the
// 22/07/2016 page witnesses the 5:15pm start and the changeover day is
// uncaptured, so the undated span serves the sourced intersection — the
// window holding under both states; the 17:00-17:15 slice is the residual.

static HKEX_INDEX_AHT_2345_EXTENDED: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 17 * 3600 + 15 * 60,
    close_ssm: 23 * 3600 + 45 * 60,
}];

static HKEX_INDEX_AHT_2345: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Hong_Kong,
    regular: HKEX_INDEX_PHASE2_REGULAR,
    extended: HKEX_INDEX_AHT_2345_EXTENDED,
    order_entry: HKEX_INDEX_PHASE2_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// --- Day close 4:30pm, T+1 5:15pm-11:45pm (first witnessed by the
// 22/07/2016 page; only the day close changes against the served
// intersection above) --------------------------------------------------------------

pub(crate) static HKEX_INDEX_1630_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 15 * 60,
        close_ssm: 12 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600,
        close_ssm: 16 * 3600 + 30 * 60,
    },
];

static HKEX_INDEX_AHT_1715_EXTENDED: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 17 * 3600 + 15 * 60,
    close_ssm: 23 * 3600 + 45 * 60,
}];

static HKEX_INDEX_AHT_1715_2345: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Hong_Kong,
    regular: HKEX_INDEX_1630_REGULAR,
    extended: HKEX_INDEX_AHT_1715_EXTENDED,
    order_entry: HKEX_INDEX_PHASE2_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// --- Current overnight shape: T+1 5:00pm-3:00am (witnessed by the live page
// retrieved 2026-10-11; keyed to 2026-10-12, the local opening day of the
// first session it governs — the Monday-evening leg — so the prior Friday's
// leg is answered entirely under the state in force when it opened) ----------

pub(crate) static HKEX_INDEX_AHT_0300_EXTENDED: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 17 * 3600,
    close_ssm: 3 * 3600,
}];

static HKEX_INDEX_AHT_0300: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Hong_Kong,
    regular: HKEX_INDEX_1630_REGULAR,
    extended: HKEX_INDEX_AHT_0300_EXTENDED,
    order_entry: HKEX_INDEX_PHASE2_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// Six dated rows over a sourced floor. The last three govern bracketed-undated
// changeovers: each is keyed to the opening day of the first session its later
// state governs, the undated span between the flanking witnesses serves the
// sourced intersection, and the disputed remainders are residuals in the file
// below.
// Evidence: docs/evidence/hkex_index_futures.md
pub(crate) static HKEX_INDEX_FUTURES_REVISIONS: &[Revision] = revisions![
    (2011, 3, 7, &HKEX_INDEX_PHASE1, "HKEX-HSIF-PAGE-2011-06"),
    (2012, 3, 5, &HKEX_INDEX_PHASE2, "HKEX-DMD-004-12"),
    (2013, 4, 8, &HKEX_INDEX_AHT_2300, "HKEX-DMD-022-13"),
    (2014, 11, 3, &HKEX_INDEX_AHT_2345, "HKEX-HSIF-PAGE-2015-07"),
    (
        2016,
        7,
        22,
        &HKEX_INDEX_AHT_1715_2345,
        "HKEX-HSIF-PAGE-2016-09"
    ),
    (
        2026,
        10,
        12,
        &HKEX_INDEX_AHT_0300,
        "HKEX-HOURS-DERIV-CURRENT"
    ),
];

/// Selects the HKEX index-futures profile in force on `as_of`'s Hong Kong day.
pub(crate) fn hkex_index_futures_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Asia::Hong_Kong),
        &HKEX_INDEX_FLOOR,
        HKEX_INDEX_FUTURES_REVISIONS,
    )
}
