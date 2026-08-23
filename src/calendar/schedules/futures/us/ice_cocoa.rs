// SPDX-License-Identifier: MIT-0

//! ICE Futures U.S. Cocoa (`CC`) futures and options schedules.

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// Cocoa runs one same-day executable session; the ICE master hours table carries
// no footnote marker on its row, so nothing commences on the previous calendar
// evening. Order entry is a separate, non-matching phase.
//
// The 20:00 pre-open is modelled Monday-Thursday. ICE runs it "on the prior
// Exchange business day", so a Monday trade date is fed by the preceding Friday
// evening. That leg is not expressible here: a wrapping SessionRule always
// wraps into the NEXT local day, so a Friday rule would assert order entry on
// Saturday morning rather than carrying through to Monday. The Friday PCPO is
// unaffected and is modelled MON_FRI, because it opens and closes inside one
// local day. The omission is a limit of the normal-week rule model, not a claim
// that ICE closes order entry over the weekend.
//
// https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
// https://www.ice.com/products/7/Cocoa-Futures
pub(crate) static COCOA_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 4 * 3600 + 45 * 60,
    close_ssm: 13 * 3600 + 30 * 60,
}];

// Two order-entry-only phases: the post-close pre-open ("PCPO") beginning 30
// minutes after the 13:30 close, and the regular pre-open from 20:00 running to
// the next morning's open.
//
// https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf
pub(crate) static COCOA_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 20 * 3600,
        close_ssm: 4 * 3600 + 45 * 60,
    },
];

pub(crate) static COCOA_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: COCOA_REGULAR_CURRENT,
    extended: COCOA_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2014-02-03 through 2018-10-05: the executable session is already today's
// 04:45-13:30 grid, but the PCPO order-entry window does not exist yet. The
// 2014 hours notice does not address the pre-open, so the 20:00 start carries
// through unchanged; only its end moves with the open it feeds.
static COCOA_EXTENDED_2014: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 20 * 3600,
    close_ssm: 4 * 3600 + 45 * 60,
}];

static COCOA_2014: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: COCOA_REGULAR_CURRENT,
    extended: COCOA_EXTENDED_2014,
    has_daily_close: true,
    has_weekend_close: true,
};

// Baseline before 2014-02-03: open 04:00 NY, close 14:00 NY.
//
// Sourcing caveat, stated plainly: the January 2014 notice prints only the NEW
// grid, marking in bold which of those figures moved. It never prints the times
// it replaced. The pre-2014 04:00 open and 14:00 close are therefore corroborated
// by the surrounding primary record rather than stated by the notice that dates
// the change, and they are recorded here on that basis. Because no primary
// document dates a cutover earlier than 2014-02-03 inside the modelled window,
// this grid is carried back as the baseline rather than inventing an earlier
// revision.
//
// The pre-open likewise runs from 20:00 on the prior Exchange business day; only
// its end differs, tracking the 04:00 open of this era.
//
// https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf
static COCOA_REGULAR_BASELINE: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 4 * 3600,
    close_ssm: 14 * 3600,
}];

static COCOA_EXTENDED_BASELINE: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 20 * 3600,
    close_ssm: 4 * 3600,
}];

pub(crate) static COCOA_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: COCOA_REGULAR_BASELINE,
    extended: COCOA_EXTENDED_BASELINE,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2014-02-03: "Effective with the start of trading for trade date Monday,
//   February 3, 2014, the Exchange will implement changes to daily trading hours
//   for Sugar No. 11, Coffee "C", Cocoa, Cotton No. 2 and Sugar No. 16 futures
//   and option contracts. ... Cocoa 4:45 13:30"
//   https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf
// 2018-10-08: "Commencing for trade date Monday, October 8, 2018, the pre-open
//   order entry session for Coffee "C", Cotton No. 2, Cocoa, FCOJ, Sugar No. 11
//   and Sugar No. 16 futures contracts will be enhanced by the addition of a new
//   post-close pre-open ("PCPO") session that will start at 30 minutes after the
//   end of trading for the contract and end at 6:00 pm on the Exchange business
//   day prior to each trading day."
//   https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf
pub(crate) static COCOA_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2014, 2, 3),
        profile: &COCOA_2014,
    },
    Revision {
        effective: effective_date(2018, 10, 8),
        profile: &COCOA_CURRENT,
    },
];

/// Selects the Cocoa profile in force on `as_of`'s New York day.
pub(crate) fn cocoa_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &COCOA_BASELINE,
        COCOA_REVISIONS,
    )
}
