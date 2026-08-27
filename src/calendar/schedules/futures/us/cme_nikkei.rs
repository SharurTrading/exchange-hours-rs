// SPDX-License-Identifier: MIT-0

//! CME Nikkei 225 Dollar (`NKD`) futures schedules.
//!
//! CME Globex NKD (clearing/ClearPort `NK`, BTIC `NKT`, CME product id 168,
//! CME Rulebook Chapter 352) is quoted by CME in both Eastern and Central time;
//! CME's trading-hours page states that hours are U.S. Central unless otherwise
//! noted, so Central is the zone modelled here. `US::Central` is the IANA link
//! for `America/Chicago` and is the zone constant every other CME/CBOT module
//! in this crate already uses.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// NKD outrights run one continuous Globex envelope per trade date: the session
// opens 17:00 CT on the previous calendar evening and closes 16:00 CT on the
// trade date, with the 60-minute 16:00-17:00 CT break separating consecutive
// trade dates. CME's own wording: "Sunday - Friday 6:00 p.m. - 5:00 p.m. ET
// (5:00 p.m. - 4:00 p.m. CT) with a 60-minute break each day beginning at
// 5:00 p.m. ET (4:00 p.m. CT)". There is no intraday halt today. Friday is
// absent from the opening-day mask because a Friday-evening open would belong
// to a Saturday trade date, which does not exist; that omission is what
// produces the Friday 16:00 CT weekly wrap.
//
// HOW THE NKD GRID DIFFERS FROM THE STANDARD CME EQUITY-INDEX GRID (the reason
// `MarketHoursKey::GlobexEquityIndex` explicitly excludes NKD): the U.S.-grid
// contracts carry a pit-anchored 08:30-15:15 CT regular session with the
// electronic envelope modelled around it as extended hours, and they carried a
// 15:15-15:30 CT halt until 2021-06-27. NKD is a pure-Globex international
// equity-index contract: it has no pit/RTH split, so its entire envelope is the
// regular session, and its 15:15-15:30 CT halt was removed eight years earlier,
// on 2013-03-04, by a notice scoped to International Equity Index futures only.
// The two grids differed from the (undatable, see below) 2010 change through
// 2012-11-18, and again through the 2013-03-04 halt removal. Today the envelopes
// coincide, but the regular/extended split does not, so the key stays separate.
//
// https://www.cmegroup.com/markets/equities/international-indices/nikkei-225-dollar.contractSpecs.html
// https://www.cmegroup.com/CmeWS/mvc/ContractSpecs/List/productId/168
// https://www.cmegroup.com/markets/equities/files/trade-japanese-equity-index-futures-fact-card.pdf
// https://www.cmegroup.com/trading-hours.html
pub(crate) static NKD_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];

// CME publishes no normal-week pre-open or order-entry start time for NKD on the
// contract specs page, the ContractSpecs service, or the Japanese equity index
// fact card, so no extended phase is asserted. The 16:00-17:00 CT daily break is
// a maintenance/closed period, not an order-entry phase, and BTIC ("Sunday -
// Friday 6:00 p.m. ET - 3:30 p.m. Tokyo time ... and Monday - Friday Noon to
// 5:00 p.m. ET") is a separately scheduled trade-registration facility rather
// than a phase of the outright order book. Both are deliberately omitted rather
// than modelled as extended sessions.
pub(crate) static NKD_EXTENDED_CURRENT: &[SessionRule] = &[];

pub(crate) static NKD_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: NKD_REGULAR_CURRENT,
    extended: NKD_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// NKD now carries a dated timeline from 2012-11-18. The 16:15 -> 16:00 CT
// close, which was previously undatable and forced this family to be modelled
// current-only, is dated by CME Globex Notice #20150817 of 17 August 2015:
//
//   "Effective Monday, September 21, the daily CME Globex maintenance period
//    will begin 15 minutes earlier Monday through Thursday from 16:00 until
//    16:45 Central Time (CT). ... With this change, the closing times for the
//    following markets will now occur 15 minutes earlier Monday through Friday
//    at 16:00 CT. CME Equity / CBOT Equity / COMEX / NYMEX / DME. All other CME
//    Globex markets trading hours remain unchanged."
//
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150817.html
//
// NKD sits in the named "CME Equity" Globex product group, and CME's own NKD
// contract-specification captures bracket the change directly: 2015-09-05 reads
// "MON - FRI: 5:00 p.m. previous day - 4:15 p.m.", and 2015-11-27 reads
// "5:00 p.m. - 4:00 p.m. Chicago Time/CT".
// https://web.archive.org/web/20150905151851/http://www.cmegroup.com/trading/equity-index/international-index/nikkei-225-dollar_contract_specifications.html
// https://web.archive.org/web/20151127190940/http://www.cmegroup.com:80/trading/equity-index/international-index/nikkei-225-dollar_contract_specifications.html
//
// Revisions are keyed by the local session-opening day, matching `cme_group`:
// the first close at 16:00 CT is trade date Monday 2015-09-21, whose session
// opened Sunday 2015-09-20.

// 2013-03-03 through 2015-09-19: the halt is gone, the close is still 16:15 CT.
static NKD_REGULAR_2013: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600 + 15 * 60,
}];

static NKD_2013: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: NKD_REGULAR_2013,
    extended: NKD_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// 2012-11-18 through 2013-03-02: close extended to 16:15 CT with a 15-minute
// electronic halt at 15:15-15:30 CT, so the day is two rules. The first rule
// carries the opening days (Sunday–Thursday evenings); the post-halt
// continuation runs on the closing local day of those wrapped sessions,
// which is Monday–Friday — a Sunday-afternoon instance never existed, and
// Friday's post-halt segment belongs to the Thursday-evening session.
static NKD_REGULAR_2012: &[SessionRule] = &[
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 15 * 3600 + 15 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 15 * 60,
    },
];

static NKD_2012: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: NKD_REGULAR_2012,
    extended: NKD_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// Before 2012-11-18 the dated route returns no session. SER-6465 pins the
// outgoing 15:15 CT close by describing the change as extending it, but no
// primary source in the audited set states the pre-2012 evening open
// separately, and a 2010 grid change is attested only by a third-party news
// aggregator. Carrying the post-2012 17:00 CT open back would fabricate a
// session boundary (LAW-NO-FABRICATED-DATES), so dates before the first fully
// sourced profile are modelled sessionless, matching the pre-launch treatment
// of launch-dated families. Callers needing a current NKD clock always have
// `hours_for_market_hours_key`.
static CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// 2012-11-18: "CME Group announces that the new daily trading hour schedule for
//   CBOT and CME Equity Index futures and Options on Equity Index futures will
//   begin on Sunday, November 18, 2012 for trade date Monday, November 19, 2012."
//   https://www.cmegroup.com/rulebook/files/ser-6465_Equity_Index_Futures_Options_on_Equity_Index_Futures_20121114.pdf
// 2013-03-03: "The modified Globex trading hours will be effective Monday,
//   March 4, 2013. The 15 minute trading halt between 3:15 p.m. and 3:30 p.m.,
//   Central Time, Monday through Friday, will be eliminated for CME
//   International Equity [Index futures] ..." - NKD is named explicitly as
//   "Nikkei 225 Dollar Futures". Keyed to the Sunday session-opening day.
//   https://www.cmegroup.com/rulebook/files/ser_6554R_-_CME_Modifies_Trading_Hours_for_International_Equity_Index_futures_on_20130304.pdf
// 2015-09-20: CME Globex Notice #20150817, quoted above; trade date Monday
//   2015-09-21, session-opening day Sunday 2015-09-20.
pub(crate) static NKD_REVISIONS: &[Revision] = revisions![
    (2012, 11, 18, &NKD_2012, "CME SER-6465"),
    (2013, 3, 3, &NKD_2013, "CME SER-6554R"),
    (2015, 9, 20, &NKD_CURRENT, "CME Globex notice 20150817"),
];

/// Selects the CME Nikkei 225 Dollar profile in force on `as_of`'s Chicago day.
///
/// Dates before the first fully sourced profile (2012-11-18) return a
/// sessionless profile: the pre-2012 evening open is not primary-sourced, so
/// the interval is omitted rather than filled with an inferred grid.
pub(crate) fn nkd_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, NKD_REVISIONS)
}
