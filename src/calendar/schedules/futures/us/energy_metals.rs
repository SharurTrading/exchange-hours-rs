// SPDX-License-Identifier: MIT-0

//! NYMEX energy/PGM and COMEX gold/silver/copper futures schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// This family covers the shared grid used by NYMEX CL/MCL/QM, NG/MNG/QG,
// HO/RB/BZ, and PL/PA plus COMEX GC/MGC, SI/SIL, and HG/MHG. Platinum and
// palladium are NYMEX products, not COMEX products. At the January-2010 audit
// floor these families opened 17:00 CT and closed 16:15 CT. CME's 2015 Globex
// notice moved every COMEX and NYMEX close to 16:00 CT for Monday 2015-09-21
// while leaving opens unchanged, so a separate metals clock would duplicate
// both the current grid and the in-scope history. Current CME material also
// publishes Sunday 16:00-17:00 and Monday-Thursday 16:45-17:00 Pre-Open. Its
// correction calls both queues a long-term practice without giving their
// unconditional onset days. A 2010 notice observes that the weekday queue was
// already 16:45 but likewise supplies no onset, and the Sunday queue's
// 16:15→16:00 move is only bracketed: metals hours pages crawled 2012-05-01
// still publish "17:15 ET (16:15 CT)" while the 2012-06-16 crawl shows
// "17:00 ET (16:00 CT)", with no notice stating the day. The fixed-current
// table includes both sourced current queues; dated profiles retain matching
// only.
// The revision is keyed to Sunday 2015-09-20, the local opening day of that
// Monday trade-date session, so a wrapped rule gives Monday the sourced close.
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20090130.html
// https://www.cmegroup.com/trading/metals/files/MT-027_GoldFuturesVsETFCheatSheet_r3.pdf
// https://www.cmegroup.com/tools-information/lookups/advisories/market-regulation/SER-5391.html
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150907.html
// https://www.cmegroup.com/markets/metals/precious/gold.contractSpecs.html
// https://www.cmegroup.com/markets/energy/crude-oil/light-sweet-crude.contractSpecs.html
// https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf
// https://web.archive.org/web/20120501182431/http://www.cmegroup.com/trading_hours/metals-hours.html
// https://web.archive.org/web/20120616193920/http://www.cmegroup.com/trading_hours/metals-hours.html
//
// 2026-08-31 Sunday-queue review — bracket narrowed, notice channels negative.
// Three archived captures of CME's own trading-hours pages, unused by the
// earlier review, move the bracket from 2012-05-03..2012-06-15 down to
// 2012-05-28..2012-06-07. The move was platform-wide and simultaneous: on the
// 2012-05-28 capture the Sunday Pre-Open is 16:15 for E-mini S&P 500,
// Eurodollar, 30-Year Interest Rate Swap, Euroyen TIBOR and (as "17:15 ET
// (16:15 CT)") Gold, Silver, Light Sweet Crude and Henry Hub; on the
// 2012-06-07 capture every one of them reads 16:00. Weekday Pre-Opens are
// unchanged across both captures, so this is a Sunday-only change.
// CBOT grains are NOT part of it: the 2012-05-11 capture still shows the
// pre-expansion 18:00-07:15/09:30-13:15 grain grid with a 16:15 Sunday
// Pre-Open, and the 2012-05-28 capture shows the expanded 17:00-14:00 grid
// with 16:00 — so grains moved at the separately sourced 2012-05-20
// expansion (CME Globex Advisory #20120518), which the grains module already
// dates.
// Both of CME's dated notice channels were then read in full across the
// narrowed window and none announces the change: CME Globex Notices of
// 2012-05-21, 2012-05-28 and 2012-06-04, and Market Data Notices of
// 2012-05-28, contain no occurrence of "Pre-Open", "trading hours", "16:00"
// or "16:15". The change was therefore made without a dated operator notice,
// which is why no cutover is encoded. (The only Sunday inside the narrowed
// bracket is 2012-06-03; that is an observation about the bracket, not a
// source-stated effective day, so LAW-NO-FABRICATED-DATES keeps it out of the
// tables.) Official origin http://www.cmegroup.com/trading_hours/ delivered
// via:
// https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities
// https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html
// https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/
// https://web.archive.org/web/20190820012118id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120521.html
// https://web.archive.org/web/20190716070058id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120528.html
// https://web.archive.org/web/20190720204402id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120604.html
// https://web.archive.org/web/20120622070557id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120528.html
static ENERGY_METALS_EXTENDED_AT_2010_FLOOR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600 + 15 * 60,
}];
pub(crate) static ENERGY_METALS_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];
// ORDER-ENTRY CLASSIFICATION. The two phases the comment above cites are named
// "Pre-Open" by CME: the book queues orders until the 17:00 Globex open and no
// trade can match inside them, so they are `order_entry` rather than a
// tradeable extended session. The 17:00-16:00 electronic session is matching
// and stays in `extended`.
pub(crate) static ENERGY_METALS_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600,
    },
];

// DATED QUEUES, CARRIED BACK AS THE SOURCED INTERSECTION. These profiles
// previously ran with no queue at all, which reported the venue as closed
// through windows it was demonstrably accepting orders in. Both phases are now
// carried from the January-2010 floor at their narrowest sourced values, so no
// cutover is asserted:
//   Sunday 16:15-17:00 - CME's trading-hours pages read "17:15 ET (16:15 CT)"
//   for Light Sweet Crude, Henry Hub, Gold and Silver on the 2012-05-11 and
//   2012-05-28 captures and "17:00 ET (16:00 CT)" on 2012-06-07. The queue only
//   widened, so 16:15-17:00 holds under both regimes; the knowledge-bound row
//   supplies the extra 16:00-16:15 quarter-hour.
//   Monday-Thursday 16:45-17:00 - the 2010 notice cited above observes this
//   queue was already in effect, and no primary source names an earlier value.
static ENERGY_METALS_ORDER_ENTRY_DATED: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600,
    },
];

static ENERGY_METALS_AT_2010_FLOOR: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: ENERGY_METALS_EXTENDED_AT_2010_FLOOR,
    order_entry: ENERGY_METALS_ORDER_ENTRY_DATED,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static ENERGY_METALS_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: ENERGY_METALS_EXTENDED_CURRENT,
    order_entry: ENERGY_METALS_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};
static ENERGY_METALS_DATED_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: ENERGY_METALS_EXTENDED_CURRENT,
    order_entry: ENERGY_METALS_ORDER_ENTRY_DATED,
    has_daily_close: true,
    has_weekend_close: true,
};

static ENERGY_METALS_REVISIONS: &[Revision] = revisions![
    (
        2015,
        9,
        20,
        &ENERGY_METALS_DATED_CURRENT,
        "CME Globex notice 20150907"
    ),
    // Knowledge-bound row: the dated profiles above already carry the sourced
    // Sunday 16:15–17:00 and Monday–Thursday 16:45–17:00 queues from the
    // January-2010 floor. This row widens the Sunday queue by the disputed
    // 16:00–16:15 quarter-hour, which is the only part depending on the
    // undated 2012 move. A sourced onset day replaces this row.
    (
        2026,
        8,
        22,
        &ENERGY_METALS_CURRENT,
        "2026-08-22 review: verified current, onset undated"
    ),
];

pub(crate) fn energy_metals_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, US::Central),
        &ENERGY_METALS_AT_2010_FLOOR,
        ENERGY_METALS_REVISIONS,
    )
}
