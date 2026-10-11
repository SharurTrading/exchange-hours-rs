// SPDX-License-Identifier: MIT-0

//! OANDA's own book: the FX (ex-TRY) grid and the TRY-pair grid.
//!
//! Narrative evidence, sources and residual risks:
//! `docs/evidence/oanda_fx.md` and `docs/evidence/oanda_try_fx.md`
//! (LAW-EVIDENCE-FILES).

use chrono::{DateTime, Utc};
use chrono_tz::America::New_York;
use chrono_tz::Europe::Berlin;

use crate::calendar::SessionRule;
use crate::calendar::rule::SUN_PLUS_MON_THU;
use crate::calendar::schedules::StaticHoursProfile;

// The FX book's one printed grid: Sunday 17:05 through Friday 16:59 New York
// time, the six-minute 16:59-17:05 daily break, and no other maintenance. The
// NZD-pair Auckland break is caller catalog data, not a rule in this grid.
// Evidence: docs/evidence/oanda_fx.md
pub(crate) static OANDA_FX_REGULAR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3_600 + 5 * 60,
    close_ssm: 16 * 3_600 + 59 * 60,
}];

static OANDA_FX: StaticHoursProfile = StaticHoursProfile {
    tz: New_York,
    regular: OANDA_FX_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// The TRY book prints one daily span, Monday-Friday 08:00-17:00 Central
// European time, with an overnight gap; "Central Europe" anchors to CET/CEST
// and Europe/Berlin is the crate's CET anchor.
// Evidence: docs/evidence/oanda_try_fx.md
const MON_FRI: [bool; 7] = [true, true, true, true, true, false, false];
pub(crate) static OANDA_TRY_FX_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3_600,
    close_ssm: 17 * 3_600,
}];

static OANDA_TRY_FX: StaticHoursProfile = StaticHoursProfile {
    tz: Berlin,
    regular: OANDA_TRY_FX_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// One sourced grid each, no dated change and no seasonal switch; the
// evidence files record the carry and what closes it.
pub(crate) fn oanda_fx_profile_at(_as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    &OANDA_FX
}

pub(crate) fn oanda_try_fx_profile_at(_as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    &OANDA_TRY_FX
}
