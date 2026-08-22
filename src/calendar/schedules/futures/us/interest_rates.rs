// SPDX-License-Identifier: MIT-0

//! CBOT U.S. Treasury/Fed Funds and CME SOFR futures schedules.

use chrono_tz::US;

use crate::calendar::rule::SUN_PLUS_MON_THU;
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};
use crate::calendar::{FuturesSessionProfile, SessionRule};

// The January-2008 CBOT migration notice establishes the 17:30-16:00 CT
// schedule inherited by the January-2010 audit-floor Treasury and 30-Day Fed
// Funds family. CME moved every legacy CBOT interest-rate open to 17:00 CT
// effective Sunday 2011-10-02 (trade date Monday 2011-10-03), aligning the
// family with the current 17:00-16:00 grid. SOFR joined this already-live
// family in May 2018; individual contract launch dates remain catalog facts,
// not separate revisions of this product-neutral family clock.
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20080121.html
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20110926.html
// https://www.cmegroup.com/notices/electronic-trading/2018/04/20180409.html
// https://www.cmegroup.com/trading/interest-rates/files/us-treasury-futures-delivery-process.pdf
// https://www.cmegroup.com/markets/interest-rates/stirs/30-day-federal-fund.contractSpecs.html
// https://www.cmegroup.com/education/articles-and-reports/understanding-sofr-futures
static EXTENDED_AT_2010_FLOOR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600 + 30 * 60,
    close_ssm: 16 * 3600,
}];

static EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];

pub(crate) static CURRENT_FUTURES_PROFILE: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_AT_2010_FLOOR: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_AT_2010_FLOOR,
    has_daily_close: true,
    has_weekend_close: true,
};

static PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2011, 10, 2),
    profile: &PROFILE_CURRENT,
}];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, US::Central),
        &PROFILE_AT_2010_FLOOR,
        REVISIONS,
    )
}
