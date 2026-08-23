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
use crate::calendar::rule::SUN_PLUS_MON_THU;
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, select_revision};

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
    has_daily_close: true,
    has_weekend_close: true,
};

// NKD's sourced history cannot be encoded as a timeline without asserting an
// undated cutover, so this family is deliberately modelled current-only.
//
// Two CME changes ARE primary-sourced and dated:
//   2012-11-19  SER-6465 moved the CBOT/CME Equity Index close from 15:15 CT to
//               16:15 CT and introduced a 15:15-15:30 CT electronic halt.
//               https://www.cmegroup.com/rulebook/files/ser-6465_Equity_Index_Futures_Options_on_Equity_Index_Futures_20121114.pdf
//   2013-03-04  SER-6554R eliminated that halt for CME International Equity
//               Index futures, naming "Nikkei 225 Dollar Futures" explicitly.
//               https://www.cmegroup.com/rulebook/files/ser_6554R_-_CME_Modifies_Trading_Hours_for_International_Equity_Index_futures_on_20130304.pdf
//
// Both leave the close at 16:15 CT. NKD's close is 16:00 CT today, so a third
// change occurred, and no retrievable CME document dates it. Encoding only the
// two sourced rows would make the timeline's tail authoritative and wrong: every
// present-day query would return a 16:15 CT close, contradicting the contract
// specification. Since a correct current profile outranks a partial history, the
// current grid is carried across the whole window and this row is Partial.
//
// A separate 2010 change is also known to have occurred; its only retrievable
// statement is a third-party aggregator, so it is not dated here either.
pub(crate) static NKD_REVISIONS: &[Revision] = &[];

/// Selects the CME Nikkei 225 Dollar profile in force on `as_of`'s Chicago day.
///
/// The revision list is empty by design; see the note above.
pub(crate) fn nkd_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &NKD_CURRENT, NKD_REVISIONS)
}
