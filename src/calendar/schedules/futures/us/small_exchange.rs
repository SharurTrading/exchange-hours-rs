// SPDX-License-Identifier: MIT-0

//! Small Exchange, Inc. current venue default.

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::StaticHoursProfile;

// The exchange's CFTC-filed S5C contract specification states ordinary
// trading hours of 08:30-15:00 CT Monday-Friday. This is the venue default;
// a product with a separately published grid needs a family key at the caller.
// https://www.cftc.gov/filings/ptc/ptc1121249243.pdf
static REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 15 * 3600,
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
