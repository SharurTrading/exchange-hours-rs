// SPDX-License-Identifier: MIT-0

//! Coinbase Derivatives Exchange recurring non-24x7 futures default.

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::SUN_PLUS_MON_THU;
use crate::calendar::schedules::StaticHoursProfile;

// CDE publishes this as the normal grid for non-24x7 futures: Sunday through
// Friday, 17:00-16:00 CT, with the daily 16:00-17:00 break. The same page
// explicitly assigns selected crypto products a different 24x7 schedule, so
// those products must select a product-family profile rather than this venue
// default.
// https://docs.cdp.coinbase.com/derivatives/introduction/market-hours
static REGULAR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];

static PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: America::Chicago,
    regular: REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

pub(crate) const fn profile_at(
    _as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    &PROFILE
}
