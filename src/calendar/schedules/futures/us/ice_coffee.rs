// SPDX-License-Identifier: MIT-0

//! ICE Futures U.S. Coffee "C" (`KC`) futures and options schedules.

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// Coffee "C" runs one same-day executable session; the ICE master hours table
// carries no footnote marker on its row, and lists "Coffee "C" and Coffee "C"
// Metric" on a single line, so both instruments share this grid. The 20:00
// pre-open is modelled Monday-Thursday because a wrapping rule cannot carry a
// Friday evening across the weekend.
// Narrative: docs/evidence/ice_us_coffee.md
pub(crate) static COFFEE_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 4 * 3600 + 15 * 60,
    close_ssm: 13 * 3600 + 30 * 60,
}];

// Two order-entry-only phases, neither of which matches: the post-close
// pre-open ("PCPO") from 30 minutes after the 13:30 close, and the 20:00
// pre-open running to the next morning's open. Coffee "C" publishes no
// tradeable phase outside its executable session, so `extended` is empty.
// Narrative: docs/evidence/ice_us_coffee.md
pub(crate) static COFFEE_EXTENDED_CURRENT: &[SessionRule] = &[];
pub(crate) static COFFEE_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 20 * 3600,
        close_ssm: 4 * 3600 + 15 * 60,
    },
];

pub(crate) static COFFEE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: COFFEE_REGULAR_CURRENT,
    extended: COFFEE_EXTENDED_CURRENT,
    order_entry: COFFEE_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2014-02-03 through 2018-10-05: the executable session is already today's
// 04:15-13:30 grid, but the PCPO order-entry window does not exist yet. The
// 2014 hours notice does not address the pre-open, so the 20:00 start carries
// through unchanged; only its end moves with the open it feeds. It is order
// entry for the same reason as the current pre-open, so this era's extended
// slice is empty as well.
static COFFEE_ORDER_ENTRY_2014: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 20 * 3600,
    close_ssm: 4 * 3600 + 15 * 60,
}];

static COFFEE_2014: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: COFFEE_REGULAR_CURRENT,
    extended: &[],
    order_entry: COFFEE_ORDER_ENTRY_2014,
    has_daily_close: true,
    has_weekend_close: true,
};

// Baseline before 2014-02-03: open 03:30 NY, close 14:00 NY, printed outright
// by the AUGUST 2011 and JANUARY 2, 2013 editions of ICE's master hours table
// and carried back. The residual gap is January 2010 to August 2011.
// Narrative: docs/evidence/ice_us_coffee.md
static COFFEE_REGULAR_BASELINE: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3 * 3600 + 30 * 60,
    close_ssm: 14 * 3600,
}];

static COFFEE_ORDER_ENTRY_BASELINE: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 20 * 3600,
    close_ssm: 3 * 3600 + 30 * 60,
}];

pub(crate) static COFFEE_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: COFFEE_REGULAR_BASELINE,
    extended: &[],
    order_entry: COFFEE_ORDER_ENTRY_BASELINE,
    has_daily_close: true,
    has_weekend_close: true,
};

// Both revision rows are T1; each row's effective day and citation literal are
// its own fields, and the notice, quotation and URL behind each are in the
// evidence file.
// Evidence: docs/evidence/ice_us_coffee.md
pub(crate) static COFFEE_REVISIONS: &[Revision] = revisions![
    (2014, 2, 3, &COFFEE_2014, "ICE ExNot 012714 hours"),
    (2018, 10, 8, &COFFEE_CURRENT, "ICE PCPO notice 20180920"),
];

/// Selects the Coffee "C" profile in force on `as_of`'s New York day.
pub(crate) fn coffee_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &COFFEE_BASELINE,
        COFFEE_REVISIONS,
    )
}
