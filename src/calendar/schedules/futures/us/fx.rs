// SPDX-License-Identifier: MIT-0

//! CME FX futures on the standard 17:00-16:00 CT Globex grid.

use chrono_tz::US;

use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};
use crate::calendar::{FuturesSessionProfile, SessionRule};

// CME's 2010 guide publishes the 17:00-16:00 matching grid for its standard FX
// futures. This family is not a promise for eFix, BTIC, TAS, options, or any
// product whose own specification publishes a different grid. The exact
// Monday-Thursday Pre-Open changed from 16:50 to 16:45 on 2010-11-15. Current
// primary material publishes Sunday 16:00-17:00, but calls it a long-term
// practice without stating the day on which the earlier queue moved. The
// fixed-current profile includes that exact current phase; dated profiles omit
// only the unresolved Sunday queue.
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20081229.html
// https://www.cmegroup.com/trading/fx/files/FX248-2010_FX_Product_Guide_and_Calendar.pdf
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
// https://www.cmegroup.com/trading/fx/fx-report/files/q1-2018-cme-fx-products.pdf
// https://www.cmegroup.com/trading/fx/files/emfx-brochure-q3-2020.pdf
// https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf
// https://www.cmegroup.com/articles/faqs/frequently-asked-questions-cme-fx-futures-calendar-spreads.html
// ORDER-ENTRY CLASSIFICATION. The 17:00-16:00 window is the matching grid the
// 2010 product guide publishes. Every other phase here is a Globex queue: the
// weekday "Pre-Open" the comment above names (16:50, then 16:45, to 17:00) and
// the Sunday 16:00-17:00 queue accept, amend, and cancel orders while the
// matching engine is stopped, so no trade can print until 17:00. They are
// `order_entry`.
static MATCHING_GRID: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];
static ORDER_ENTRY_AT_2010_FLOOR: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 16 * 3600 + 50 * 60,
    close_ssm: 17 * 3600,
}];
static ORDER_ENTRY_DATED_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 16 * 3600 + 45 * 60,
    close_ssm: 17 * 3600,
}];
pub(crate) static ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600,
    },
];

pub(crate) static CURRENT_FUTURES_PROFILE: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: MATCHING_GRID,
    order_entry: ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static AT_2010_FLOOR: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: MATCHING_GRID,
    order_entry: ORDER_ENTRY_AT_2010_FLOOR,
    has_daily_close: true,
    has_weekend_close: true,
};
static DATED_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: MATCHING_GRID,
    order_entry: ORDER_ENTRY_DATED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2010, 11, 15),
    profile: &DATED_CURRENT,
}];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &AT_2010_FLOOR, REVISIONS)
}
