// SPDX-License-Identifier: MIT-0

//! ICE Futures U.S. Cotton No. 2 (`CT`) futures schedules.

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// Cotton No. 2 is a wrapping contract: the session for trade date D commences at
// 21:00 NY on calendar day D-1 and closes at 14:20 NY on D. The ICE master hours
// table states the row verbatim as "Cotton No. 2(R)   21:00* - 14:20" with the
// footnote "*Trading commences on previous business day."; the product page
// carries the same 21:00 open, 14:20 close and `closeNextDay` true on
// America/New_York. Rulebook Rule 4.25(c)(ii) puts the last-trading-day close at
// the same clock ("for Cotton No. 2 Futures at 2:20 PM") and Rule 4.25(b)(iii)
// puts the daily settlement window at 14:14 - 14:15, so 14:20 is the session end
// on every trade date, not just ordinary ones.
//
// SUNDAY EVENING — A KNOWN GAP, NOT A MODELLED PHASE. The master-table row
// "*Trading commences on previous business day." pins Monday-Thursday
// evening opens: each of those evenings is the previous business day of the
// next trade date. No ICE primary document names Sunday in connection with
// Cotton No. 2 — the product page carries no day names, and the master
// table's explicit Sunday footnotes ("**... on Sunday evenings only trading
// commences at 18:00", "*** ... 17:50") belong to other products. A Sunday
// 21:00 open is the only reading consistent with the holiday notices (the
// 2021 Labor Day notice runs "Mon, Sep 6 Closed / Tue, Sep 7 Regular Hours",
// and the 2025 Good Friday notice gives Cotton "Regular Hours" on the Monday
// while the morning-opening softs take a late open), but that reading is
// assembled from indirect material rather than stated by any ICE sentence.
// Confirmed independently on 2026-08-31: the JANUARY 2, 2013 edition of the
// master table repeats the same contrast - "Cotton No. 2(R)  21:00* - 14:30"
// against "Grains and Oilseeds", "Russell Index", "USDX(R)" and the currency
// rows all carrying "**", and the energy rows "***". ICE therefore drew the
// Sunday distinction explicitly at two independent dated points and did not
// extend it to Cotton, which is positive evidence for the omission rather than
// mere silence.
// Dated editions of ICE's own master table, official origin
// https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
// delivered via
// https://web.archive.org/web/20111212140120id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
// https://web.archive.org/web/20130122132629id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
// Under LAW-PRIMARY-SOURCES an unasserted phase is omitted, so the Sunday
// evening open and its pre-open are modelled as closed; the documented
// weekend boundary is therefore Friday 14:20 to Monday 19:30 NY.
//
// WEEKEND WRAP: the tradeable week runs Monday 21:00 open through Friday
// 14:20 close, with Monday-Thursday 19:30 pre-opens. There is no
// Friday-evening open, because a Friday 21:00 open would belong to a
// Saturday trade date, which does not exist. The Friday 14:50-18:00 PCPO is
// retained: the product-page footnote and the 2018 notice's worked example
// state it on the prior Exchange business day, so it remains the week's
// final order-entry window, feeding Monday's session — orders accepted
// there wait for the modelled Monday 21:00 open instead of the unsourced
// Sunday one.
//
// https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
// https://www.ice.com/products/254/cotton-no-2-futures
// https://www.ice.com/publicdocs/rulebooks/futures_us/4_Trading.pdf
// https://www.ice.com/publicdocs/rulebooks/futures_us/10_Cotton.pdf
// https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2021LaborDay_Holiday_20210713.pdf
// https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_GoodFridayHoliday20250210.pdf
// https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_Christmas_Holiday_20251031.pdf
pub(crate) static COTTON_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 21 * 3600,
    close_ssm: 14 * 3600 + 20 * 60,
}];

// Two order-entry-only phases, neither of them executable.
//
// The regular Pre-Open / Pre-Trading Session runs 19:30 - 21:00 NY immediately
// before each open, so it carries the same opening-day mask as the executable
// session. The product page gives Pre-Open "7:30 PM" / "19:30",
// and the 2018 PCPO notice's own table lists Cotton No. 2 with Pre-Open Start
// 7:30 PM and End 9:00 PM. Only Limit orders are accepted (Rule 4.22(a):
// "Prior to the opening of a trading session for an Exchange Commodity
// Contract, there will be a Pre-Trading Session designated by the Exchange
// during which time only Limit orders may be entered."), with an Opening Match
// uncrossing between the Pre-Open and the open (Rules 4.22(b), 4.23).
//
// The Post-Close Pre-Open ("PCPO") runs 14:50 - 18:00 NY, 30 minutes after the
// 14:20 close, "on the prior Exchange business day" - product page footnote
// verbatim: "**In addition to the Pre-Open start time shown above, there will be
// a Post-Close Pre-Open order entry session from 2:50 pm to 6:00 pm NY time on
// the prior Exchange business day." Every Monday-Friday business day is the
// prior business day of some trading day, so the mask is MON_FRI. The Friday leg
// is stated rather than inferred: the 2018 notice's own worked example places
// the PCPO for trade date Monday 8 October 2018 on Friday 5 October, making the
// Friday PCPO the last order-entry window of the week. The PCPO accepts GTC /
// GTD / GTD&T entry and amendment only; Day orders entered in it are killed at
// its end.
//
// https://www.ice.com/products/254/cotton-no-2-futures
// https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf
// CLASSIFICATION. Both phases are order_entry, not extended. Rule 4.22(a)
// restricts the Pre-Trading Session to Limit order entry and puts the Opening
// Match at the open, so no trade prints between 19:30 and 21:00; the PCPO is an
// order entry/amendment window whose own Day orders are killed at its end, so
// no trade prints between 14:50 and 18:00 either. Cotton No. 2 publishes no
// tradeable phase outside its executable session, which leaves the extended
// slice empty.
//
// https://www.ice.com/publicdocs/rulebooks/futures_us/4_Trading.pdf
pub(crate) static COTTON_EXTENDED_CURRENT: &[SessionRule] = &[];
pub(crate) static COTTON_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 50 * 60,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 19 * 3600 + 30 * 60,
        close_ssm: 21 * 3600,
    },
];

pub(crate) static COTTON_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: COTTON_REGULAR_CURRENT,
    extended: COTTON_EXTENDED_CURRENT,
    order_entry: COTTON_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2014-02-03 through 2018-10-05: the executable grid is already today's
// 21:00 - 14:20, but the PCPO order-entry window does not exist yet. The
// September 2018 notice describes the PCPO as an addition to the existing
// pre-open order entry session, so the 19:30 - 21:00 Pre-Open is the whole of
// the non-executable grid in this regime - and, being order entry under Rule
// 4.22(a), it sits in order_entry, leaving these eras with no extended phase.
static COTTON_ORDER_ENTRY_2014: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 19 * 3600 + 30 * 60,
    close_ssm: 21 * 3600,
}];

static COTTON_2014: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: COTTON_REGULAR_CURRENT,
    extended: &[],
    order_entry: COTTON_ORDER_ENTRY_2014,
    has_daily_close: true,
    has_weekend_close: true,
};

// Baseline before 2014-02-03. The January 2014 notice moved only the close, from
// 14:30 to 14:20 NY, and left the 21:00 open on the previous day untouched, so
// the baseline regular grid is the current one with the older close.
//
// The notice does not address order entry at all, and no primary ICE document
// inside the modelled window states a pre-open time for Cotton earlier than the
// 19:30 on the product page and in the 2018 notice table. The 19:30 - 21:00
// Pre-Open is therefore carried back unchanged rather than inventing an earlier
// cutover for it; if ICE moved it at some point before 2014, no primary source
// dates that move.
//
// https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf
static COTTON_REGULAR_BASELINE: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 21 * 3600,
    close_ssm: 14 * 3600 + 30 * 60,
}];

pub(crate) static COTTON_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: COTTON_REGULAR_BASELINE,
    extended: &[],
    order_entry: COTTON_ORDER_ENTRY_2014,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2014-02-03: ICE Futures U.S. Exchange Notice, originally issued 6 January
//   2014, "Changes to Daily Trading Hours": "Effective with the start of trading
//   for trade date Monday, February 3, 2014, the Exchange will implement changes
//   to daily trading hours for Sugar No. 11, Coffee "C", Cocoa, Cotton No. 2 and
//   Sugar No. 16 futures and options contracts." Cotton's row moves the close
//   from 14:30 to 14:20; the 21:00 open on the previous day is unchanged.
//   https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf
// 2018-10-08: ICE Futures U.S. Notice, 20 September 2018, "EXTENSION OF THE
//   PRE-OPEN ORDER ENTRY SESSION FOR COFFEE "C", COTTON NO. 2, COCOA, FCOJ,
//   SUGAR NO. 11 AND SUGAR NO. 16 FUTURES CONTRACTS": "Commencing for trade date
//   Monday, October 8, 2018, the pre-open order entry session for Coffee "C",
//   Cotton No. 2, Cocoa, FCOJ, Sugar No. 11 and Sugar No. 16 futures contracts
//   will be enhanced by the addition of a new post-close pre-open ("PCPO")
//   session that will start at 30 minutes after the end of trading for the
//   contract and end at 6:00 pm on the Exchange business day prior to each
//   trading day." For Cotton the 14:20 close puts that start at 14:50.
//   https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf
pub(crate) static COTTON_REVISIONS: &[Revision] = revisions![
    (2014, 2, 3, &COTTON_2014, "ICE ExNot 012714 hours"),
    (2018, 10, 8, &COTTON_CURRENT, "ICE PCPO notice 20180920"),
];

/// Selects the Cotton No. 2 profile in force on `as_of`'s New York day.
pub(crate) fn cotton_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &COTTON_BASELINE,
        COTTON_REVISIONS,
    )
}
