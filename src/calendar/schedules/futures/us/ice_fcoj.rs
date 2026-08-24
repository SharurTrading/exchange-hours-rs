// SPDX-License-Identifier: MIT-0

//! ICE Futures U.S. FCOJ-A (frozen concentrated orange juice) futures and
//! options schedules.

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// FCOJ-A runs one same-day executable session. The ICE master hours table
// carries no footnote marker on the FCOJ-A row - neither the single asterisk
// ("Trading commences on previous business day") nor the double asterisk that
// additionally moves the Sunday open - so nothing commences on the previous
// calendar evening and there is no Sunday session. The executable week runs
// Monday 08:00 through Friday 14:00 New York.
//
// Master table, verbatim: "FCOJ-A   8:00 - 14:00". Corroborated by the ICE
// product page: "NEW YORK  8:00 AM - 2:00 PM  08:00 - 14:00".
//
// https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
// https://www.ice.com/products/30/FCOJ-A-Futures
pub(crate) static FCOJ_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 14 * 3600,
}];

// Two order-entry-only phases, neither of which matches: the post-close pre-open
// ("PCPO") beginning 30 minutes after the 14:00 close and ending at 18:00, and
// the regular pre-open from 20:00 running to the next morning's 08:00 open. Both
// attach to the Exchange business day prior to the trade date, so the windows
// feeding a Tuesday session are Monday 14:30-18:00 and Monday 20:00 onward.
//
// The 20:00 pre-open start is modelled Monday-Thursday only. ICE places it "on
// the prior Exchange business day", which for a Monday trade date is the
// preceding Friday - a 60-hour order-entry window spanning the weekend. No ICE
// document states that order entry stays live across the weekend, so the
// Friday-evening leg is deliberately omitted rather than asserted. The PCPO is
// modelled Monday-Friday because Friday 14:30-18:00 is an ordinary same-week
// window that ICE's own worked example confirms (the PCPO for trade date Monday
// 8 October 2018 fell on Friday 5 October 2018).
//
// Product page footnote, verbatim: "**In addition to the Pre-Open start time
// shown above, there will be a Post-Close Pre-Open order entry session from 2:30
// pm to 6:00 pm NY time on the prior Exchange business day."
//
// Both windows are order_entry, not extended: the product page names the PCPO
// an "order entry session", and Rule 4.22(a) confines the Pre-Trading Session
// to Limit order entry with the Opening Match at the open. FCOJ-A publishes no
// tradeable phase outside its 08:00-14:00 executable session, so the extended
// slice is empty.
//
// https://www.ice.com/products/30/FCOJ-A-Futures
// https://www.ice.com/publicdocs/rulebooks/futures_us/4_Trading.pdf
pub(crate) static FCOJ_EXTENDED_CURRENT: &[SessionRule] = &[];
pub(crate) static FCOJ_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 30 * 60,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 20 * 3600,
        close_ssm: 8 * 3600,
    },
];

pub(crate) static FCOJ_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: FCOJ_REGULAR_CURRENT,
    extended: FCOJ_EXTENDED_CURRENT,
    order_entry: FCOJ_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Baseline before 2018-10-08: the executable session is already today's
// 08:00-14:00 grid, but the PCPO order-entry window does not exist yet, leaving
// the 20:00 pre-open as the only non-executable phase - order entry, like the
// current one, so this era carries no extended phase either.
//
// The executable grid is carried back unchanged rather than dated to an earlier
// cutover. ICE's 2014 softs hours notice re-tabulated Sugar No. 11, Coffee "C",
// Cocoa, Cotton No. 2 and Sugar No. 16 and then stated "Daily trading hours for
// all other products are unchanged", which excludes FCOJ-A; no primary ICE
// document inside the modelled window states a different FCOJ-A open or close,
// so no earlier revision is invented here.
//
// https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf
static FCOJ_ORDER_ENTRY_BASELINE: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 20 * 3600,
    close_ssm: 8 * 3600,
}];

pub(crate) static FCOJ_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: FCOJ_REGULAR_CURRENT,
    extended: &[],
    order_entry: FCOJ_ORDER_ENTRY_BASELINE,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2018-10-08: "Commencing for trade date Monday, October 8, 2018, the pre-open
//   order entry session for Coffee "C", Cotton No. 2, Cocoa, FCOJ, Sugar No. 11
//   and Sugar No. 16 futures contracts will be enhanced by the addition of a new
//   post-close pre-open ("PCPO") session that will start at 30 minutes after the
//   end of trading for the contract and end at 6:00 pm on the Exchange business
//   day prior to each trading day."
//   https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf
//
// FCOJ is named in the notice by contract, so the 14:00 close puts the FCOJ-A
// PCPO at 14:30-18:00, matching the 2:30 pm figure the product page footnote
// carries today. The executable session is untouched by this notice.
//
// No other FCOJ-A schedule change inside the modelled window is dated by a
// primary ICE source. The pre-open start itself is only ever stated in undated
// product-page form ("Pre-Open ... 8:00 PM 20:00"); ICE publishes no notice
// establishing or moving it, so it is carried in the baseline rather than given
// a cutover.
pub(crate) static FCOJ_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2018, 10, 8),
    profile: &FCOJ_CURRENT,
}];

/// Selects the FCOJ-A profile in force on `as_of`'s New York day.
pub(crate) fn fcoj_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &FCOJ_BASELINE,
        FCOJ_REVISIONS,
    )
}
