// SPDX-License-Identifier: MIT-0

//! Eurex fixed income futures (`FGBL`, `FGBM`, `FGBS`, `FGBX`) schedules.
//!
//! Euro-Bund, Euro-Bobl, Euro-Schatz and Euro-Buxl Futures share one grid,
//! verified row by row in Annex C of the Eurex Contract Specifications (as of
//! 17.08.2026) and on all four product pages, so the four contracts are
//! modelled as a single family here.

use chrono_tz::{Europe, UTC};

use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{
    Revision, local_date, reference_delta_seconds, revisions, select_revision,
};

// Eurex publishes a phase machine, not a single open/close pair. Continuous
// Trading is the only executable phase, so it alone is `regular`; Pre-Trading
// and Post-Trading accept order entry without matching, so both are
// `order_entry` and `extended` is empty. The morning phases are anchored to
// 08:00 Singapore, so the family is two seasonal profiles, not one.
// Narrative: docs/evidence/eurex_fixed_income.md
pub(crate) static EUREX_FIXED_INCOME_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 2 * 3600 + 10 * 60,
    close_ssm: 22 * 3600,
}];

// No tradeable non-continuous phase exists on this grid; see the classification
// note above. Kept as an explicit empty slice so the family's shape stays
// visible next to its `order_entry` twin.
pub(crate) static EUREX_FIXED_INCOME_EXTENDED_CURRENT: &[SessionRule] = &[];

// Pre-Trading 02:00-02:10 CEST, then Post-Trading 22:00-22:10 local. Annex C
// gives the post-trading row as "Post-Trading Periode bis / Post-Trading Period
// Until = 22:10*". Order entry only in both windows.
pub(crate) static EUREX_FIXED_INCOME_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 2 * 3600,
        close_ssm: 2 * 3600 + 10 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 22 * 3600,
        close_ssm: 22 * 3600 + 10 * 60,
    },
];

pub(crate) static EUREX_FIXED_INCOME_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_FIXED_INCOME_REGULAR_CURRENT,
    extended: EUREX_FIXED_INCOME_EXTENDED_CURRENT,
    order_entry: EUREX_FIXED_INCOME_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// The same regime on a CET (winter) day: every morning phase sits one hour
// earlier on the Berlin clock because it is pinned to 08:00 Singapore, while
// the 22:00 close and the 22:10 post-trading end do not move.
static EUREX_FIXED_INCOME_REGULAR_WINTER: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3600 + 10 * 60,
    close_ssm: 22 * 3600,
}];

static EUREX_FIXED_INCOME_ORDER_ENTRY_WINTER: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 3600,
        close_ssm: 3600 + 10 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 22 * 3600,
        close_ssm: 22 * 3600 + 10 * 60,
    },
];

static EUREX_FIXED_INCOME_WINTER: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_FIXED_INCOME_REGULAR_WINTER,
    extended: &[],
    order_entry: EUREX_FIXED_INCOME_ORDER_ENTRY_WINTER,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2018-12-10 through 2019-02-24: today's Asian-hours executable session with
// the post-trading phase still running to 22:30. That window sits inside CET,
// so the CEST twin is never selected in practice; it is kept so the regime is
// described by its dates rather than by an accident of the calendar.
// Narrative: docs/evidence/eurex_fixed_income.md
static EUREX_FIXED_INCOME_ORDER_ENTRY_2018_SUMMER: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 2 * 3600,
        close_ssm: 2 * 3600 + 10 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 22 * 3600,
        close_ssm: 22 * 3600 + 30 * 60,
    },
];

static EUREX_FIXED_INCOME_ORDER_ENTRY_2018_WINTER: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 3600,
        close_ssm: 3600 + 10 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 22 * 3600,
        close_ssm: 22 * 3600 + 30 * 60,
    },
];

static EUREX_FIXED_INCOME_2018_SUMMER: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_FIXED_INCOME_REGULAR_CURRENT,
    extended: &[],
    order_entry: EUREX_FIXED_INCOME_ORDER_ENTRY_2018_SUMMER,
    has_daily_close: true,
    has_weekend_close: true,
};

static EUREX_FIXED_INCOME_2018_WINTER: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_FIXED_INCOME_REGULAR_WINTER,
    extended: &[],
    order_entry: EUREX_FIXED_INCOME_ORDER_ENTRY_2018_WINTER,
    has_daily_close: true,
    has_weekend_close: true,
};

// Baseline before 2018-12-10: Pre-Trading 07:30-08:00 CET with continuous
// trading 08:00-22:00, one same-day grid with no seasonal split. Circular
// 088/18 states the phase it replaced; the 22:30 post-trading end is carried
// back rather than dated.
// Narrative: docs/evidence/eurex_fixed_income.md
static EUREX_FIXED_INCOME_REGULAR_BASELINE: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 22 * 3600,
}];

// Pre-Trading 07:30-08:00 CET and Post-Trading 22:00-22:30: order entry only,
// same phase machine as the current grid.
static EUREX_FIXED_INCOME_ORDER_ENTRY_BASELINE: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 30 * 60,
        close_ssm: 8 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 22 * 3600,
        close_ssm: 22 * 3600 + 30 * 60,
    },
];

pub(crate) static EUREX_FIXED_INCOME_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_FIXED_INCOME_REGULAR_BASELINE,
    extended: &[],
    order_entry: EUREX_FIXED_INCOME_ORDER_ENTRY_BASELINE,
    has_daily_close: true,
    has_weekend_close: true,
};

// Both revision rows are T1; each row's effective day and citation literal are
// its own fields, and the circular, amendment and URLs behind them are in the
// evidence file. This is the CEST timeline; the WINTER twin below carries the
// identical two days against the CET grid.
// Evidence: docs/evidence/eurex_fixed_income.md
pub(crate) static EUREX_FIXED_INCOME_REVISIONS: &[Revision] = revisions![
    (
        2018,
        12,
        10,
        &EUREX_FIXED_INCOME_2018_SUMMER,
        "Eurex Circular 088/18"
    ),
    (
        2019,
        2,
        25,
        &EUREX_FIXED_INCOME_CURRENT,
        "Eurex CS amendment 2019-02-25"
    ),
];

// Evidence: docs/evidence/eurex_fixed_income.md
static EUREX_FIXED_INCOME_WINTER_REVISIONS: &[Revision] = revisions![
    (
        2018,
        12,
        10,
        &EUREX_FIXED_INCOME_2018_WINTER,
        "Eurex Circular 088/18"
    ),
    (
        2019,
        2,
        25,
        &EUREX_FIXED_INCOME_WINTER,
        "Eurex CS amendment 2019-02-25"
    ),
];

/// The `reference - venue` delta that marks a CEST (summer) Berlin day: UTC
/// minus Berlin's +02:00 summer offset.
const BERLIN_SUMMER_DELTA_SECONDS: i32 = -2 * 3600;

/// Selects the Eurex fixed income profile in force on `as_of`'s Berlin day.
///
/// Both seasonal timelines carry the same effective dates; the venue's own UTC
/// offset on that day picks the CET or CEST grid. Berlin's changeover falls on
/// a Sunday, which is not a Eurex trading day, so the two never disagree about
/// a day that has a session.
pub(crate) fn eurex_fixed_income_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    let revisions =
        if reference_delta_seconds(as_of, Europe::Berlin, UTC) == BERLIN_SUMMER_DELTA_SECONDS {
            EUREX_FIXED_INCOME_REVISIONS
        } else {
            EUREX_FIXED_INCOME_WINTER_REVISIONS
        };
    select_revision(
        local_date(as_of, Europe::Berlin),
        &EUREX_FIXED_INCOME_BASELINE,
        revisions,
    )
}
