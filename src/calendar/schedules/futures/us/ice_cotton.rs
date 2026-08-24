// SPDX-License-Identifier: MIT-0

//! ICE Futures U.S. Cotton No. 2 (`CT`) futures schedules.

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, SUN_PLUS_MON_THU};
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
// SUNDAY EVENING. The evening open is modelled on Sunday as well as Monday
// through Thursday, at the ordinary 21:00 - there is no Sunday carve-out for
// Cotton. Three primary-source strands force this reading:
//
//   (a) In the ICE master table Cotton carries ONLY the single asterisk,
//       "*Trading commences on previous business day." The Sunday carve-outs
//       belong to the other footnote markers and to other products: "**Trading
//       commences on previous business day and on Sunday evenings only trading
//       commences at 18:00" (USDX, currency pairs, daily gold/silver, the index
//       futures, digital asset) and "*** ... on Sunday evenings only trading
//       commences at 17:50" (natural gas, power, environmental, oil). Cotton is
//       in neither group, so nothing displaces its Sunday open from 21:00.
//   (b) ICE holiday notices show the 21:00 evening open belongs to the NEXT
//       trade date regardless of which calendar day that evening falls on. The
//       2021 Labor Day notice reads "Mon, Sep 6  Closed" / "Tue, Sep 7  Regular
//       Hours", i.e. Tuesday's session commenced at 21:00 on the Monday holiday
//       evening. Reading "previous BUSINESS day" literally would put Tuesday's
//       open on the preceding Friday, which the notice contradicts; ICE's
//       wording therefore means the prior calendar evening, and for a Monday
//       trade date that evening is Sunday.
//   (c) The Good Friday notices settle it. After a Friday closure the
//       morning-opening softs need a delayed Monday start while Cotton does
//       not - 2025 Good Friday notice, Mon Apr 21 row, verbatim: "Sugar No. 11,
//       Coffee "C" and Cocoa - late open at 7:30 am. Regular Hours for all other
//       contracts." (the 2022 notice uses identical wording for Mon Apr 18).
//       Cotton being on Regular Hours that Monday is only possible if its
//       Monday session had already opened on the Sunday evening. The contrast
//       case confirms the mechanism: when a Thursday/Friday holiday does kill
//       Cotton's evening open, Cotton gets the late open too - 2025 Christmas
//       notice, Fri Dec 26 row: "Late Open for Coffee "C", Cocoa, Cotton No. 2
//       and Sugar 11 - 7:30 am".
//
// CAVEAT: no ICE primary document uses the word "Sunday" in connection with
// Cotton No. 2. The Sunday open encoded here is the only reading consistent with
// the primary sources, but it is assembled from them rather than stated outright
// in any one ICE sentence. Treat it as a sourced inference, not a quotation.
//
// WEEKEND WRAP: the week therefore runs Sunday 19:30 pre-open / 21:00 open
// through Friday 14:20 close. There is no Friday-evening open, because a Friday
// 21:00 open would belong to a Saturday trade date, which does not exist.
//
// https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
// https://www.ice.com/products/254/cotton-no-2-futures
// https://www.ice.com/publicdocs/rulebooks/futures_us/4_Trading.pdf
// https://www.ice.com/publicdocs/rulebooks/futures_us/10_Cotton.pdf
// https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2021LaborDay_Holiday_20210713.pdf
// https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_GoodFridayHoliday20250210.pdf
// https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_2025_Christmas_Holiday_20251031.pdf
pub(crate) static COTTON_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 21 * 3600,
    close_ssm: 14 * 3600 + 20 * 60,
}];

// Two order-entry-only phases, neither of them executable.
//
// The regular Pre-Open / Pre-Trading Session runs 19:30 - 21:00 NY immediately
// before each open, so it carries the same opening-day mask as the executable
// session, Sunday included. The product page gives Pre-Open "7:30 PM" / "19:30",
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
        days: SUN_PLUS_MON_THU,
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
    days: SUN_PLUS_MON_THU,
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
    days: SUN_PLUS_MON_THU,
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
