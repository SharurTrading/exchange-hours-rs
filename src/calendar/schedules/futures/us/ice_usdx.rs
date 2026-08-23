// SPDX-License-Identifier: MIT-0

//! ICE Futures U.S. US Dollar Index (`USDX` / `DX`) futures and options
//! schedules.

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// USDX is a near-24-hour contract whose session commences on the evening before
// its trade date, with a Sunday evening that opens two hours earlier than the
// weekday evenings. ICE's June 2026 master hours table gives the row as
// "USDX(R)   20:00** - 17:00", where footnote ** reads "Trading commences on
// previous business day and on Sunday evenings only trading commences at
// 18:00".
//
// The 18:00 Sunday open is stated by three independent primary ICE documents,
// so it is modelled as a distinct rule rather than folded into the weekday
// evening grid:
//   1. the master hours table footnote ** quoted above;
//   2. the ICE US Dollar Index FAQ - "Trading ends at 5:00 p.m. ET on Friday
//      afternoon. On Sunday evening, trading in the contracts begins at 6:00
//      p.m. ET; the trading session that begins on Sunday evening ends at 5:00
//      p.m. ET on the following Monday";
//   3. product page 194 - "Market opens at 6:00 pm on Sunday for Monday's trade
//      date. Pre-open at 5:30 pm Sunday, 7:30 pm Monday through Thursday."
//
// The weekday evening rule stops at Thursday: a Friday 20:00 open would belong
// to a Saturday trade date, which does not exist, and the FAQ fixes the weekly
// wrap at "5:00 p.m. ET on Friday afternoon".
//
// https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
// https://www.ice.com/products/194/US-Dollar-Index-USDX-Futures
// https://www.ice.com/publicdocs/futures_us/ICE_Dollar_Index_FAQ.pdf
pub(crate) static ICE_USDX_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 18 * 3600,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 20 * 3600,
        close_ssm: 17 * 3600,
    },
];

// Order entry only, 30 minutes ahead of each open, and nothing else: USDX has
// no post-close pre-open ("PCPO") window. The 2018 PCPO notice that added one
// to the ICE softs names only Coffee "C", Cotton No. 2, Cocoa, FCOJ, Sugar
// No. 11 and Sugar No. 16, and neither the USDX product page nor the FAQ
// publishes a post-close order-entry phase; the product page's only extended
// phase is the Pre-Open column.
//
// Verbatim from the product page: "NEW YORK  8:00 PM - 5:00 PM*  20:00 - 17:00
// 7:30 PM  19:30" and "Pre-open at 5:30 pm Sunday, 7:30 pm Monday through
// Thursday."; the specification block adds "*The trading platform is available
// 30 minutes before the opening for order entry. Open on Sunday night is 6:00
// PM ET; Pre-Open at 5:30 PM ET".
//
// https://www.ice.com/products/194/US-Dollar-Index-USDX-Futures
pub(crate) static ICE_USDX_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 19 * 3600 + 30 * 60,
        close_ssm: 20 * 3600,
    },
];

pub(crate) static ICE_USDX_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: ICE_USDX_REGULAR_CURRENT,
    extended: ICE_USDX_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Baseline before 2011-02-14: the same evening opens, closing an hour later at
// 18:00 NY. The February 2011 notice changed only the close and said so
// explicitly - "This change does not affect the start of the electronic trading
// day for these products" - so the Sunday 18:00 and weekday-evening 20:00 opens
// are carried back rather than reconstructed.
//
// The Sunday rule has equal endpoints here. A SessionRule with `open_ssm ==
// close_ssm` encodes exactly one complete local-day span, so Sunday 18:00 runs
// continuously to Monday 18:00, which is what the pre-2011 grid described.
//
// The notice's parenthetical lists the unchanged weekday opens as "8:00 pm NY
// time Tuesday through Thursday evenings". That wording would leave Tuesday's
// trade date with no open at all, and the master hours table footnote applies
// the previous-business-day rule to every trade date, so the weekday evening
// mask stays Monday through Thursday. No primary source dates any change to
// which evenings carry an open, so none is encoded.
static ICE_USDX_REGULAR_BASELINE: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 18 * 3600,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 20 * 3600,
        close_ssm: 18 * 3600,
    },
];

// The 30-minute pre-open is carried back unchanged. ICE states it as a standing
// platform property rather than a dated one - the April 2011 edition of the US
// Dollar Index FAQ already reads "The ICE trading platform is available for
// order entry thirty minutes before the opening of trading", and the current
// product page repeats it. No primary source dates its introduction, so no
// cutover is encoded for the extended phases; only the close moves in 2011.
//
// https://www.ice.com/publicdocs/futures_us/ICE_Dollar_Index_FAQ.pdf
pub(crate) static ICE_USDX_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: ICE_USDX_REGULAR_BASELINE,
    extended: ICE_USDX_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2011-02-14: "Effective with the start of trading for trade date Monday,
//   February 14, 2011, the Exchange is implementing a change to electronic
//   trading hours for the USDX futures and options contracts, the Euro Index
//   futures contract and for all currency pair futures contracts on the ICE
//   electronic trading platform: the trading day for these contracts will now
//   end at 5:00 pm NY time. This change does not affect the start of the
//   electronic trading day for these products (which remains as 6:00 pm NY time
//   on Sunday evenings, 8:00 pm NY time Tuesday through Thursday evenings) or
//   the settlement window for these products (which remains 2:59 to 3:00 pm)."
//   ICE Futures U.S. Exchange Notice, "Change To Electronic Trading Hours For
//   IFUS Currency Contracts", February 7, 2011. The notice has been retired
//   from ice.com; per the crate's established practice for superseded operator
//   documents, the archive capture is cited as-is.
//   https://web.archive.org/web/20110222135802/https://www.theice.com/publicdocs/futures_us/exchange_notices/ExNot020311DXhours.pdf
pub(crate) static ICE_USDX_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2011, 2, 14),
    profile: &ICE_USDX_CURRENT,
}];

/// Selects the USDX profile in force on `as_of`'s New York day.
pub(crate) fn ice_usdx_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &ICE_USDX_BASELINE,
        ICE_USDX_REVISIONS,
    )
}
