// SPDX-License-Identifier: MIT-0

//! US listed-equity-options profiles.

use chrono::{DateTime, Utc};
use chrono_tz::America;

use super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::CLOSED_NEW_YORK;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// Every profile in this module is deliberately scoped to ordinary options on
// individual US stocks. Generic pre-open order acceptance is part of the
// exchange envelope, but execution in this product family begins at 09:30, so
// those windows are order entry rather than trading. ETF, ETN, index, FLEX,
// floor-only, and venue-designated extended-hours classes remain separate
// product families because their executable sessions vary.
//
// The 2006 coordinated rule changes moved individual-stock options from a
// 16:02 to a 16:00 ET close before this repository's January-2010 history
// floor. Each older venue has its own primary baseline (not merely a shared
// industry inference):
// C1: https://www.sec.gov/rules/sro/cboe/2006/34-53246.pdf
// Arca: https://www.sec.gov/rules/sro/pcx/34-53249.pdf
// American: https://www.sec.gov/rules/sro/amex/2006/34-53244.pdf
// PHLX: https://www.sec.gov/rules/sro/phlx/34-53247.pdf
// ISE: https://www.sec.gov/rules/sro/ise/2006/34-53248.pdf
// BOX: https://www.sec.gov/rules/sro/bse/2006/34-53245.pdf
// NOM's approved rules set 09:30–16:00 for this family, and its launch alert
// identifies AMAT (an individual stock) among the 2008-03-31 launch classes:
// https://www.sec.gov/rules/sro/nasdaq/2008/34-57478.pdf
// https://www.nasdaqtrader.com/MicroNews.aspx?id=OTA2008-001
// The current operator rules retain 09:30–16:00 RTH for this exact product
// family. C1 has an additional session for venue-designated classes. MRX's
// approved Options 3C session remains unencoded until the required trader alert
// makes it operative.
// https://cdn.cboe.com/resources/release_notes/2026/Schedule_Update_C1_Options_to_Offer_GTH_Sessions_for_Multi_List_Options_Series.html
// https://www.sec.gov/rules-regulations/self-regulatory-organization-rulemaking/sr-mrx-2026-11
// https://cdn.cboe.com/resources/regulation/rule_book/C1_Exchange_Rule_Book.pdf
// https://cdn.cboe.com/resources/regulation/rule_book/C2_Exchange_Rule_Book.pdf
// https://cdn.cboe.com/resources/regulation/rule_book/BZX_Exchange_Rulebook.pdf
// https://cdn.cboe.com/resources/regulation/rule_book/EDGX_Rulebook.pdf
// https://nysearcaguide.srorules.com/rules
// https://nyseamericanguide.srorules.com/rules
// https://listingcenter.nasdaq.com/RuleBook/Nasdaq/rules/nasdaq-options-3
// https://listingcenter.nasdaq.com/rulebook/phlx/rules/Phlx%20Options%203
// https://listingcenter.nasdaq.com/rulebook/ise/rules/ISE%20Options%203
// https://listingcenter.nasdaq.com/rulebook/gemx/rules/GEMX%20Options%203
// https://listingcenter.nasdaq.com/rulebook/mrx/rules/MRX%20Options%203
// https://listingcenter.nasdaq.com/rulebook/nasdaqtx/rules/NTX%20Options%203
// https://www.miaxglobal.com/markets/us-options/miax-options/trade-hours-calendar
// https://www.miaxglobal.com/markets/us-options/pearl-options/trade-hours-calendar
// https://www.miaxglobal.com/markets/us-options/emerald-options/trade-hours-calendar
// https://www.miaxglobal.com/markets/us-options/sapphire-options/trade-hours-calendar
// https://rules.boxexchange.com
// https://info.memxtrading.com/market-hours-and-holiday-schedule/
static LISTED_EQUITY_OPTIONS_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 16 * 3600,
}];

// Order-entry-only pre-open queues. Each venue below opens its book to order
// entry, amendment, and cancellation at the stated time, but no contract in
// this product family can match until the opening process runs at 09:30 ET —
// the cited operator system-settings and hours pages describe these windows as
// order acceptance/queuing, and the first execution of the day is the 09:30
// opening. They are therefore `order_entry`, not tradeable extended sessions.
static ORDER_ENTRY_0600: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 6 * 3600,
    close_ssm: 9 * 3600 + 30 * 60,
}];

static ORDER_ENTRY_0700: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600,
    close_ssm: 9 * 3600 + 30 * 60,
}];

static ORDER_ENTRY_0730: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 30 * 60,
    close_ssm: 9 * 3600 + 30 * 60,
}];

// Current ordinary-stock-option order-acceptance edges. The reviewed primary
// sources do not supply a complete day-level amendment chain for these queues,
// so each venue's timeline carries them only from its 2026-08-22
// knowledge-bound row onward while earlier dated queries retain the exact
// 09:30–16:00 execution history. Because nothing matches in a queue, carrying
// them as `order_entry` leaves earlier dated grids — which hold no queue at
// all — unaffected in their tradeable coverage.
// https://www.cboe.com/about/hours/us-options
// https://www.nyse.com/trade/hours-calendars?os=.
// https://www.nasdaq.com/docs/PHLXSystemSettings
// https://www.nasdaq.com/docs/NOMSystemSettings
// https://www.nasdaq.com/docs/ISESystemSettings
// https://www.nasdaq.com/docs/GEMXSystemSettings.pdf
// https://www.nasdaq.com/docs/MRXSystemSettings
// https://nasdaqtrader.com/Content/BXOptions/BXOptions_FAQs.pdf
// https://www.miaxglobal.com/markets/us-options/all-options-exchanges/trade-hours-calendar
// https://boxexchange.com/assets/BOX-Exchange-Quoting-Requirements-Summary_10.15.pdf
// https://info.memxtrading.com/wp-content/uploads/2023/05/MEMX-Options-User-Manual.pdf
pub(crate) static CBOE_OPTIONS_C1_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static CBOE_C2_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static CBOE_BZX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static CBOE_EDGX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static NYSE_ARCA_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0600);
pub(crate) static NYSE_AMERICAN_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0600);
pub(crate) static NASDAQ_PHLX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static NASDAQ_ISE_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0600);
pub(crate) static NASDAQ_NOM_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static NASDAQ_MRX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0600);
pub(crate) static NASDAQ_GEMX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0600);
pub(crate) static NASDAQ_BX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static MIAX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static MIAX_EMERALD_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static MIAX_PEARL_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static MIAX_SAPPHIRE_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static BOX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0700);
pub(crate) static MEMX_OPTIONS_PROFILE: StaticHoursProfile = listed_equity_options_profile(&[]);

static LISTED_EQUITY_OPTIONS_HISTORICAL: StaticHoursProfile = listed_equity_options_profile(&[]);

// Launch evidence for venues that began after the January-2010 audit floor.
// BZX launched 2010-02-26 with cash-equity underliers among its 18 classes:
// https://cdn.cboe.com/resources/press_releases/BATSOptionsGoesLive_FINAL.pdf
// C2 launched 2010-10-29 with Ford as its first class:
// https://cdn.cboe.com/resources/regulation/circulars/general/IC-CBOE-2010-168.pdf
// EDGX launched phase one on 2015-11-02; its schedule lists five stock classes:
// https://cdn.cboe.com/resources/release_notes/2015/BATS-EDGX-Options-Update-2015_11_10.pdf
// https://cdn.cboe.com/resources/edgx_options/EDGX_Options_Symbol_Rollout_Schedule.xlsx
// BX launched 2012-06-29 with five stock classes, including AA and INTC:
// https://www.nasdaqtrader.com/MicroNews.aspx?id=OTA2012-41
// GEMX's audited filing calls it an equity-and-index-options exchange and
// states that it formally commenced trading on 2013-08-05:
// https://www.sec.gov/Archives/edgar/vprr/1601/16019242.pdf
// MRX initiated trading on 2016-02-16; the operator reports an equity-and-ETF
// rollout, while the SEC filing independently fixes the exact launch day:
// https://www.deutsche-boerse.com/resource/blob/324026/912f25fc1b9e0cdb916acbd69d4013fb/data/Detailed_volume_statistics_are_found_in_the_following_document-1.pdf
// https://www.sec.gov/files/rules/sro/bats/2016/34-77256.pdf
// MIAX launched 2012-12-07 with stock class CLF:
// https://www.miaxglobal.com/alert/2012/12/06/miax-options-will-commence-trading-friday-december-7-2012
// MIAX Pearl launched 2017-02-06 with IBM:
// https://www.miaxglobal.com/alerts/2017/02/01/market-underlying-security-used-openings-miax-pearl-newly-listed-option-class
// MIAX Emerald launched 2019-03-01 with IBM:
// https://www.miaxglobal.com/news/miax-emerald-successfully-launches-trading-operations
// MIAX Sapphire launched 2024-08-12 with IBM:
// https://www.miaxglobal.com/sites/default/files/alert-files/MIAX_Press_Release_09102024.pdf
// MEMX launched 2023-09-27 with stock classes SBUX and IMGN (plus GLD):
// https://info.memxtrading.com/trader-alert-23-42-memx-options-exchange-schedule-update/
//
// Row evidence — each table's launch day mapped to its primary source:
//   2010-02-26 "BATS Options launch press release" (BZX)
//     https://cdn.cboe.com/resources/press_releases/BATSOptionsGoesLive_FINAL.pdf
//   2010-10-29 "Cboe circular IC-CBOE-2010-168" (C2)
//     https://cdn.cboe.com/resources/regulation/circulars/general/IC-CBOE-2010-168.pdf
//   2015-11-02 "Bats EDGX options update 2015-11-10" (EDGX)
//     https://cdn.cboe.com/resources/release_notes/2015/BATS-EDGX-Options-Update-2015_11_10.pdf
//   2012-06-29 "Nasdaq OTA 2012-41" (BX)
//     https://www.nasdaqtrader.com/MicroNews.aspx?id=OTA2012-41
//   2013-08-05 "SEC filing 16019242" (GEMX)
//     https://www.sec.gov/Archives/edgar/vprr/1601/16019242.pdf
//   2016-02-16 "SEC 34-77256" (MRX)
//     https://www.sec.gov/files/rules/sro/bats/2016/34-77256.pdf
//   2012-12-07 "MIAX launch alert 2012-12-06"
//     https://www.miaxglobal.com/alert/2012/12/06/miax-options-will-commence-trading-friday-december-7-2012
//   2017-02-06 "MIAX Pearl launch alert 2017-02-01"
//     https://www.miaxglobal.com/alerts/2017/02/01/market-underlying-security-used-openings-miax-pearl-newly-listed-option-class
//   2019-03-01 "MIAX Emerald launch announcement"
//     https://www.miaxglobal.com/news/miax-emerald-successfully-launches-trading-operations
//   2024-08-12 "MIAX press release 2024-09-10" (Sapphire)
//     https://www.miaxglobal.com/sites/default/files/alert-files/MIAX_Press_Release_09102024.pdf
//   2023-09-27 "MEMX trader alert 23-42"
//     https://info.memxtrading.com/trader-alert-23-42-memx-options-exchange-schedule-update/
// Knowledge-bound rows (the final 2026-08-22 row in each table below): each
// venue's current order-acceptance queue is primary-verified in the current
// envelope, but the reviewed sources state no day-level amendment chain for
// these queues, so earlier dated queries retain only the exact 09:30–16:00
// execution history. From the 2026-08-22 repository review onward each
// verified-current grid applies; a sourced onset day replaces its row.
static BZX_REVISIONS: &[Revision] = revisions![
    (
        2010,
        2,
        26,
        &LISTED_EQUITY_OPTIONS_HISTORICAL,
        "BATS Options launch press release"
    ),
    (
        2026,
        8,
        22,
        &CBOE_BZX_OPTIONS_PROFILE,
        "2026-08-22 review: verified current, onset undated"
    ),
];
static C2_REVISIONS: &[Revision] = revisions![
    (
        2010,
        10,
        29,
        &LISTED_EQUITY_OPTIONS_HISTORICAL,
        "Cboe circular IC-CBOE-2010-168"
    ),
    (
        2026,
        8,
        22,
        &CBOE_C2_OPTIONS_PROFILE,
        "2026-08-22 review: verified current, onset undated"
    ),
];
static EDGX_REVISIONS: &[Revision] = revisions![
    (
        2015,
        11,
        2,
        &LISTED_EQUITY_OPTIONS_HISTORICAL,
        "Bats EDGX options update 2015-11-10"
    ),
    (
        2026,
        8,
        22,
        &CBOE_EDGX_OPTIONS_PROFILE,
        "2026-08-22 review: verified current, onset undated"
    ),
];
static BX_REVISIONS: &[Revision] = revisions![
    (
        2012,
        6,
        29,
        &LISTED_EQUITY_OPTIONS_HISTORICAL,
        "Nasdaq OTA 2012-41"
    ),
    (
        2026,
        8,
        22,
        &NASDAQ_BX_OPTIONS_PROFILE,
        "2026-08-22 review: verified current, onset undated"
    ),
];
static GEMX_REVISIONS: &[Revision] = revisions![
    (
        2013,
        8,
        5,
        &LISTED_EQUITY_OPTIONS_HISTORICAL,
        "SEC filing 16019242"
    ),
    (
        2026,
        8,
        22,
        &NASDAQ_GEMX_OPTIONS_PROFILE,
        "2026-08-22 review: verified current, onset undated"
    ),
];
static MRX_REVISIONS: &[Revision] = revisions![
    (
        2016,
        2,
        16,
        &LISTED_EQUITY_OPTIONS_HISTORICAL,
        "SEC 34-77256"
    ),
    (
        2026,
        8,
        22,
        &NASDAQ_MRX_OPTIONS_PROFILE,
        "2026-08-22 review: verified current, onset undated"
    ),
];
static MIAX_REVISIONS: &[Revision] = revisions![
    (
        2012,
        12,
        7,
        &LISTED_EQUITY_OPTIONS_HISTORICAL,
        "MIAX launch alert 2012-12-06"
    ),
    (
        2026,
        8,
        22,
        &MIAX_OPTIONS_PROFILE,
        "2026-08-22 review: verified current, onset undated"
    ),
];
static MIAX_PEARL_REVISIONS: &[Revision] = revisions![
    (
        2017,
        2,
        6,
        &LISTED_EQUITY_OPTIONS_HISTORICAL,
        "MIAX Pearl launch alert 2017-02-01"
    ),
    (
        2026,
        8,
        22,
        &MIAX_PEARL_OPTIONS_PROFILE,
        "2026-08-22 review: verified current, onset undated"
    ),
];
static MIAX_EMERALD_REVISIONS: &[Revision] = revisions![
    (
        2019,
        3,
        1,
        &LISTED_EQUITY_OPTIONS_HISTORICAL,
        "MIAX Emerald launch announcement"
    ),
    (
        2026,
        8,
        22,
        &MIAX_EMERALD_OPTIONS_PROFILE,
        "2026-08-22 review: verified current, onset undated"
    ),
];
static MIAX_SAPPHIRE_REVISIONS: &[Revision] = revisions![
    (
        2024,
        8,
        12,
        &LISTED_EQUITY_OPTIONS_HISTORICAL,
        "MIAX press release 2024-09-10"
    ),
    (
        2026,
        8,
        22,
        &MIAX_SAPPHIRE_OPTIONS_PROFILE,
        "2026-08-22 review: verified current, onset undated"
    ),
];
static MEMX_REVISIONS: &[Revision] = revisions![(
    2023,
    9,
    27,
    &MEMX_OPTIONS_PROFILE,
    "MEMX trader alert 23-42"
),];

// Venues whose execution history predates the audit floor and whose only
// timeline row is the knowledge bound: the dated baseline is the queue-less
// 09:30–16:00 grid, and the 2026-08-22 row applies the verified-current
// order-acceptance queue.
static C1_REVISIONS: &[Revision] = revisions![(
    2026,
    8,
    22,
    &CBOE_OPTIONS_C1_PROFILE,
    "2026-08-22 review: verified current, onset undated"
),];
static NYSE_ARCA_OPTIONS_REVISIONS: &[Revision] = revisions![(
    2026,
    8,
    22,
    &NYSE_ARCA_OPTIONS_PROFILE,
    "2026-08-22 review: verified current, onset undated"
),];
static NYSE_AMERICAN_OPTIONS_REVISIONS: &[Revision] = revisions![(
    2026,
    8,
    22,
    &NYSE_AMERICAN_OPTIONS_PROFILE,
    "2026-08-22 review: verified current, onset undated"
),];
static NASDAQ_PHLX_REVISIONS: &[Revision] = revisions![(
    2026,
    8,
    22,
    &NASDAQ_PHLX_OPTIONS_PROFILE,
    "2026-08-22 review: verified current, onset undated"
),];
static NASDAQ_ISE_REVISIONS: &[Revision] = revisions![(
    2026,
    8,
    22,
    &NASDAQ_ISE_OPTIONS_PROFILE,
    "2026-08-22 review: verified current, onset undated"
),];
static NASDAQ_NOM_REVISIONS: &[Revision] = revisions![(
    2026,
    8,
    22,
    &NASDAQ_NOM_OPTIONS_PROFILE,
    "2026-08-22 review: verified current, onset undated"
),];
static BOX_REVISIONS: &[Revision] = revisions![(
    2026,
    8,
    22,
    &BOX_OPTIONS_PROFILE,
    "2026-08-22 review: verified current, onset undated"
),];

macro_rules! knowledge_selector {
    ($($name:ident, $revisions:ident),+ $(,)?) => {
        $(
            pub(crate) fn $name(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
                select_revision(
                    local_date(as_of, America::New_York),
                    &LISTED_EQUITY_OPTIONS_HISTORICAL,
                    $revisions,
                )
            }
        )+
    };
}

knowledge_selector!(
    c1_profile_at,
    C1_REVISIONS,
    nyse_arca_options_profile_at,
    NYSE_ARCA_OPTIONS_REVISIONS,
    nyse_american_options_profile_at,
    NYSE_AMERICAN_OPTIONS_REVISIONS,
    nasdaq_phlx_profile_at,
    NASDAQ_PHLX_REVISIONS,
    nasdaq_ise_profile_at,
    NASDAQ_ISE_REVISIONS,
    nasdaq_nom_profile_at,
    NASDAQ_NOM_REVISIONS,
    box_options_profile_at,
    BOX_REVISIONS,
);

macro_rules! launch_selector {
    ($name:ident, $revisions:ident) => {
        pub(crate) fn $name(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
            select_revision(
                local_date(as_of, America::New_York),
                &CLOSED_NEW_YORK,
                $revisions,
            )
        }
    };
}

launch_selector!(c2_options_profile_at, C2_REVISIONS);
launch_selector!(bzx_options_profile_at, BZX_REVISIONS);
launch_selector!(edgx_options_profile_at, EDGX_REVISIONS);
launch_selector!(nasdaq_bx_options_profile_at, BX_REVISIONS);
launch_selector!(nasdaq_gemx_profile_at, GEMX_REVISIONS);
launch_selector!(nasdaq_mrx_profile_at, MRX_REVISIONS);
launch_selector!(miax_options_profile_at, MIAX_REVISIONS);
launch_selector!(miax_pearl_options_profile_at, MIAX_PEARL_REVISIONS);
launch_selector!(miax_emerald_options_profile_at, MIAX_EMERALD_REVISIONS);
launch_selector!(miax_sapphire_options_profile_at, MIAX_SAPPHIRE_REVISIONS);
launch_selector!(memx_options_profile_at, MEMX_REVISIONS);

// This family has no tradeable session outside 09:30–16:00: every non-regular
// window a venue publishes here is a pre-open order-acceptance queue, so
// `extended` is empty and the queue lands in `order_entry`.
const fn listed_equity_options_profile(order_entry: &'static [SessionRule]) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: America::New_York,
        regular: LISTED_EQUITY_OPTIONS_REGULAR,
        extended: &[],
        order_entry,
        has_daily_close: true,
        has_weekend_close: true,
    }
}
