// SPDX-License-Identifier: MIT-0

//! ICE Futures U.S. Sugar No. 11 (`SB`) futures and options schedules.

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// Sugar No. 11 runs one same-day executable session; the ICE master hours table
// carries no footnote marker on its row, so nothing commences on the previous
// calendar evening. Order entry is a separate, non-matching phase.
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
// https://www.ice.com/products/23/Sugar-No-11-Futures
pub(crate) static SUGAR_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3 * 3600 + 30 * 60,
    close_ssm: 13 * 3600,
}];

// Two order-entry-only phases: the post-close pre-open ("PCPO") beginning 30
// minutes after the 13:00 close, and the regular pre-open from 20:00 running to
// the next morning's open.
//
// Both are classified order_entry, not extended: neither matches. The 2018
// notice that created the PCPO calls it an extension of the "pre-open order
// entry session" and kills Day orders entered in it at its end, and the
// pre-open only accepts orders ahead of the Opening Match that occurs at the
// open itself. Sugar No. 11 publishes no tradeable phase outside its executable
// session, so the extended slice is empty.
pub(crate) static SUGAR_EXTENDED_CURRENT: &[SessionRule] = &[];
pub(crate) static SUGAR_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 30 * 60,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 20 * 3600,
        close_ssm: 3 * 3600 + 30 * 60,
    },
];

pub(crate) static SUGAR_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: SUGAR_REGULAR_CURRENT,
    extended: SUGAR_EXTENDED_CURRENT,
    order_entry: SUGAR_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2014-02-03 through 2018-10-05: the executable session is already today's
// 03:30-13:00 grid, but the PCPO order-entry window does not exist yet, leaving
// the 20:00 pre-open as the only non-executable phase. It is order entry for
// the same reason as the current one, so these eras carry an empty extended
// slice too.
static SUGAR_ORDER_ENTRY_2014: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 20 * 3600,
    close_ssm: 3 * 3600 + 30 * 60,
}];

static SUGAR_2014: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: SUGAR_REGULAR_CURRENT,
    extended: &[],
    order_entry: SUGAR_ORDER_ENTRY_2014,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2012-11-05 through 2014-01-31: open fixed at 02:30 NY year-round.
static SUGAR_REGULAR_2012_NOV: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 2 * 3600 + 30 * 60,
    close_ssm: 14 * 3600,
}];

static SUGAR_2012_NOV: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: SUGAR_REGULAR_2012_NOV,
    extended: &[],
    order_entry: SUGAR_ORDER_ENTRY_2014,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2012-01-30 through 2012-11-02: open moved to 01:30 NY. The same notice
// announced a standing summer adjustment to 02:30 NY, which ICE itself labelled
// a "temporary change to the opening time"; seasonal opening shifts are
// exceptional-day changes and are not modelled as normal-week revisions here.
static SUGAR_REGULAR_2012_JAN: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3600 + 30 * 60,
    close_ssm: 14 * 3600,
}];

static SUGAR_2012_JAN: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: SUGAR_REGULAR_2012_JAN,
    extended: &[],
    order_entry: SUGAR_ORDER_ENTRY_2014,
    has_daily_close: true,
    has_weekend_close: true,
};

// Baseline before 2012-01-30. The earliest primary statement of the Sugar open
// inside the modelled window is ICE's March 2010 DST notice, which names "the
// normal 3:30 am NY time" open; the earliest full grid is ICE's August 2011
// master table, showing electronic 3:30 - 14:00. No primary source gives a
// January 2010 close, so the August 2011 grid is carried back as the baseline
// rather than inventing an earlier cutover.
//
//
// 2026-09-01: WHY THE 2010-2011 INTERVAL CANNOT BE SOURCED. ICE Futures U.S.
// sets these hours administratively, not by rule. Its product rulebook chapters
// - Sugar No. 11, Cotton No. 2, Coffee, Cocoa, FCOJ and USDX, all captured
// December 2011 - contain no hours provision at all, and chapter 4 is
// trade-practice rules. No SEC or CFTC filing therefore fixes an ICE Futures
// U.S. trading hour, and the master hours table is the only source; its earliest
// surviving edition is August 2011. This interval is bounded by document
// availability, not by an unfinished search, so the carry-back above is the
// terminal answer unless an earlier edition surfaces.
// https://web.archive.org/web/20111213011033id_/https://www.theice.com/publicdocs/rulebooks/futures_us/11_Sugar_11.pdf
// https://web.archive.org/web/20111213011055id_/https://www.theice.com/publicdocs/rulebooks/futures_us/8_Coffee.pdf
// https://web.archive.org/web/20111213011442id_/https://www.theice.com/publicdocs/rulebooks/futures_us/9_Cocoa.pdf
// 2026-08-31: confirmed negative. Every archived edition of the master table
// was enumerated and the AUGUST 2011 one (captured 2011-12-12) is the earliest
// in existence; there is no 2010 edition to consult. The JANUARY 2, 2013
// edition reads "Sugar No. 11(R)  2:30 - 14:00", consistent with the dated
// 2012-01-30 revision already modelled. The January-2010 close therefore
// remains unsourced by document availability, not by an unfinished search.
// Dated editions of ICE's own master table, official origin
// https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
// delivered via
// https://web.archive.org/web/20111212140120id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
// https://web.archive.org/web/20130122132629id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
//
// https://www.ice.com/publicdocs/futures_us/exchange_notices/exnot03XX10DST.pdf
// https://web.archive.org/web/20111212140120/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
static SUGAR_REGULAR_BASELINE: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3 * 3600 + 30 * 60,
    close_ssm: 14 * 3600,
}];

pub(crate) static SUGAR_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: SUGAR_REGULAR_BASELINE,
    extended: &[],
    order_entry: SUGAR_ORDER_ENTRY_2014,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2012-01-30: "Effective with the start of trading on Monday, January 30, 2012
//   the daily electronic trading session for Sugar No. 11 futures and options
//   contracts will begin at 1:30 am NY time."
//   https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot121911S11_Hours_12312.pdf
// 2012-11-05: "Effective with the start of trading on Monday, November 5, 2012
//   the daily trading session for Sugar No. 11 futures and options contracts
//   will begin at 2:30 am NY time."
//   https://www.ice.com/publicdocs/futures_us/exchange_notices/1018912ExNotS11Hours.pdf
// 2014-02-03: "Effective with the start of trading for trade date Monday,
//   February 3, 2014, the Exchange will implement changes to daily trading hours
//   for Sugar No. 11 ... Sugar No. 11 3:30 13:00"
//   https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf
// 2018-10-08: "Commencing for trade date Monday, October 8, 2018, the pre-open
//   order entry session for Coffee "C", Cotton No. 2, Cocoa, FCOJ, Sugar No. 11
//   and Sugar No. 16 futures contracts will be enhanced by the addition of a new
//   post-close pre-open ("PCPO") session that will start at 30 minutes after the
//   end of trading for the contract and end at 6:00 pm on the Exchange business
//   day prior to each trading day."
//   https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf
pub(crate) static SUGAR_REVISIONS: &[Revision] = revisions![
    (2012, 1, 30, &SUGAR_2012_JAN, "ICE ExNot 121911 S11 hours"),
    (2012, 11, 5, &SUGAR_2012_NOV, "ICE ExNot 1018912 S11 hours"),
    (2014, 2, 3, &SUGAR_2014, "ICE ExNot 012714 hours"),
    (2018, 10, 8, &SUGAR_CURRENT, "ICE PCPO notice 20180920"),
];

/// Selects the Sugar No. 11 profile in force on `as_of`'s New York day.
pub(crate) fn sugar_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &SUGAR_BASELINE,
        SUGAR_REVISIONS,
    )
}
