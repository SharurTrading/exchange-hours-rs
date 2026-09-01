// SPDX-License-Identifier: MIT-0

//! ICE Futures U.S. Cocoa (`CC`) futures and options schedules.

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

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
// Both are classified order_entry, not extended: nothing matches in either. The
// 2018 notice creating the PCPO calls it an extension of the "pre-open order
// entry session" and kills Day orders entered in it at its end, and the
// pre-open only accepts orders ahead of the Opening Match at the open itself.
// Cocoa publishes no tradeable phase outside its executable session, so the
// extended slice is empty.
//
// https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf
pub(crate) static COCOA_EXTENDED_CURRENT: &[SessionRule] = &[];
pub(crate) static COCOA_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
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
    order_entry: COCOA_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2014-02-03 through 2018-10-05: the executable session is already today's
// 04:45-13:30 grid, but the PCPO order-entry window does not exist yet. The
// 2014 hours notice does not address the pre-open, so the 20:00 start carries
// through unchanged; only its end moves with the open it feeds. It is order
// entry for the same reason as the current pre-open, so this era's extended
// slice is empty as well.
static COCOA_ORDER_ENTRY_2014: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 20 * 3600,
    close_ssm: 4 * 3600 + 45 * 60,
}];

static COCOA_2014: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: COCOA_REGULAR_CURRENT,
    extended: &[],
    order_entry: COCOA_ORDER_ENTRY_2014,
    has_daily_close: true,
    has_weekend_close: true,
};

// Baseline before 2014-02-03: open 04:00 NY, close 14:00 NY.
//
// Sourcing caveat, stated plainly: the January 2014 notice prints only the NEW
// grid, marking in bold which of those figures moved. It never prints the times
// it replaced.
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
// SOURCING CAVEAT SUPERSEDED 2026-08-31. The pre-2014 grid is no longer merely
// corroborated: two dated editions of ICE's own "ICE Futures U.S. Regular
// Trading Hours" master table print it outright. The AUGUST 2011 edition and
// the JANUARY 2, 2013 edition both read "Cocoa   4:00 - 14:00", so the
// 04:00 open and 14:00 close are stated by primary ICE documents at two dated
// points spanning up to the 2014-02-03 change. Because no primary document
// dates a cutover earlier than 2014-02-03 inside the modelled window, this grid
// is still carried back as the baseline rather than inventing an earlier
// revision; the residual gap is January 2010 to August 2011, for which no
// edition of the master table survives in the archive.
// Dated editions of ICE's own master table, official origin
// https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
// delivered via
// https://web.archive.org/web/20111212140120id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
// https://web.archive.org/web/20130122132629id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
//
// The pre-open likewise runs from 20:00 on the prior Exchange business day; only
// its end differs, tracking the 04:00 open of this era. It is order entry, not
// trading, so it sits in the order_entry slice.
//
// https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf
static COCOA_REGULAR_BASELINE: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 4 * 3600,
    close_ssm: 14 * 3600,
}];

static COCOA_ORDER_ENTRY_BASELINE: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 20 * 3600,
    close_ssm: 4 * 3600,
}];

pub(crate) static COCOA_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: COCOA_REGULAR_BASELINE,
    extended: &[],
    order_entry: COCOA_ORDER_ENTRY_BASELINE,
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
pub(crate) static COCOA_REVISIONS: &[Revision] = revisions![
    (2014, 2, 3, &COCOA_2014, "ICE ExNot 012714 hours"),
    (2018, 10, 8, &COCOA_CURRENT, "ICE PCPO notice 20180920"),
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
