// SPDX-License-Identifier: MIT-0

//! Point-in-time selectors for US listed-equity-options schedules.
//!
//! The profile tables live in the parent module; this module owns their dated
//! revision rows, the launch and knowledge-bound evidence for each row, and
//! the selector every venue routes through.

use chrono::{DateTime, Utc};
use chrono_tz::America;

use super::{
    BOX_OPTIONS_PROFILE, CBOE_BZX_OPTIONS_PROFILE, CBOE_C2_OPTIONS_PROFILE,
    CBOE_EDGX_OPTIONS_PROFILE, CBOE_OPTIONS_C1_PROFILE, LISTED_EQUITY_OPTIONS_HISTORICAL,
    MEMX_OPTIONS_PROFILE, MIAX_EMERALD_OPTIONS_PROFILE, MIAX_OPTIONS_PROFILE,
    MIAX_PEARL_OPTIONS_PROFILE, MIAX_SAPPHIRE_OPTIONS_PROFILE, NASDAQ_BX_OPTIONS_PROFILE,
    NASDAQ_GEMX_OPTIONS_PROFILE, NASDAQ_ISE_OPTIONS_PROFILE, NASDAQ_MRX_OPTIONS_PROFILE,
    NASDAQ_NOM_OPTIONS_PROFILE, NASDAQ_PHLX_OPTIONS_PROFILE, NYSE_AMERICAN_OPTIONS_PROFILE,
    NYSE_ARCA_OPTIONS_PROFILE,
};
use crate::calendar::schedules::CLOSED_NEW_YORK;
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

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
// QUEUE CARRY-BACK, DECIDED 2026-09-01 — READ THIS BEFORE TRUSTING A HISTORICAL
// QUEUE ANSWER. Each venue's current order-acceptance queue is now served across
// its whole modelled history rather than only from the repository review date.
// The queues are `order_entry`: nothing matches in them, and every venue's
// 09:30-16:00 execution history is sourced independently and unaffected.
//
// THE ASSUMPTION, STATED PLAINLY. No primary source says when any of these
// queues began. They are operator *system settings* published on mutable
// hours/system-settings pages, not rulebook boundaries with filed operative
// dates — SR-C2-2019-009 and SR-CboeBZX-2020-012 each write down 07:30 as "the
// same time at which the System begins accepting orders and quotes today" while
// declining to change it, and Cboe Options Rule 6.2(a) bounds the pre-opening
// period rather than fixing it. Carrying the queue back therefore asserts
// continuity that no document states. It is a deliberate, recorded choice, not
// a sourced fact: a venue almost certainly accepted orders before its open, and
// under-reporting order acceptance for sixteen years was judged the worse error.
//
// WHERE THE ASSUMPTION IS NOT MADE. MIAX Options is the counterexample and is
// modelled from evidence instead. Its 07:30 window existed at the sourced
// 2012-12-07 launch but was connectivity verification only — the official hours
// page captured 2012-12-09 says activity before the Live Quote Window "WILL NOT
// affect the live quote state" — and the next capture, 2013-05-07, says it WILL
// affect the live book. So MIAX carries a queue-free launch row and gains the
// queue at that second capture, not at launch.
//
// The three evidence classes below:
//   * no launch inside the window (C1, NYSE Arca/American, PHLX, ISE, NOM, BOX)
//     — queue carried from the January-2010 audit floor, assumption applies;
//   * launch-dated (C2, BZX, EDGX, BX, GEMX, MRX, MIAX Pearl/Emerald/Sapphire)
//     — queue carried from the sourced launch day, assumption applies;
//   * MIAX Options — sourced on both sides, no assumption.
// MEMX Options is outside all three: it has no queue at all, rejecting orders
// before 09:30, so its sourced launch row is its only row.
static BZX_REVISIONS: &[Revision] = revisions![(
    2010,
    2,
    26,
    &CBOE_BZX_OPTIONS_PROFILE,
    "BATS Options launch press release"
),];
static C2_REVISIONS: &[Revision] = revisions![(
    2010,
    10,
    29,
    &CBOE_C2_OPTIONS_PROFILE,
    "Cboe circular IC-CBOE-2010-168"
),];
static EDGX_REVISIONS: &[Revision] = revisions![(
    2015,
    11,
    2,
    &CBOE_EDGX_OPTIONS_PROFILE,
    "Bats EDGX options update 2015-11-10"
),];
static BX_REVISIONS: &[Revision] = revisions![(
    2012,
    6,
    29,
    &NASDAQ_BX_OPTIONS_PROFILE,
    "Nasdaq OTA 2012-41"
),];
static GEMX_REVISIONS: &[Revision] = revisions![(
    2013,
    8,
    5,
    &NASDAQ_GEMX_OPTIONS_PROFILE,
    "SEC filing 16019242"
),];
static MRX_REVISIONS: &[Revision] =
    revisions![(2016, 2, 16, &NASDAQ_MRX_OPTIONS_PROFILE, "SEC 34-77256"),];
static MIAX_REVISIONS: &[Revision] = revisions![
    (
        2012,
        12,
        7,
        &LISTED_EQUITY_OPTIONS_HISTORICAL,
        "MIAX launch alert 2012-12-06"
    ),
    (
        2013,
        5,
        7,
        &MIAX_OPTIONS_PROFILE,
        "first capture showing the window affecting the live book"
    ),
];
static MIAX_PEARL_REVISIONS: &[Revision] = revisions![(
    2017,
    2,
    6,
    &MIAX_PEARL_OPTIONS_PROFILE,
    "MIAX Pearl launch alert 2017-02-01"
),];
static MIAX_EMERALD_REVISIONS: &[Revision] = revisions![(
    2019,
    3,
    1,
    &MIAX_EMERALD_OPTIONS_PROFILE,
    "MIAX Emerald launch announcement"
),];
static MIAX_SAPPHIRE_REVISIONS: &[Revision] = revisions![(
    2024,
    8,
    12,
    &MIAX_SAPPHIRE_OPTIONS_PROFILE,
    "MIAX press release 2024-09-10"
),];
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
static C1_REVISIONS: &[Revision] = &[];
static NYSE_ARCA_OPTIONS_REVISIONS: &[Revision] = &[];
static NYSE_AMERICAN_OPTIONS_REVISIONS: &[Revision] = &[];
static NASDAQ_PHLX_REVISIONS: &[Revision] = &[];
static NASDAQ_ISE_REVISIONS: &[Revision] = &[];
static NASDAQ_NOM_REVISIONS: &[Revision] = &[];
static BOX_REVISIONS: &[Revision] = &[];

macro_rules! carried_selector {
    ($($name:ident, $baseline:ident, $revisions:ident),+ $(,)?) => {
        $(
            pub(crate) fn $name(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
                select_revision(
                    local_date(as_of, America::New_York),
                    &$baseline,
                    $revisions,
                )
            }
        )+
    };
}

carried_selector!(
    c1_profile_at,
    CBOE_OPTIONS_C1_PROFILE,
    C1_REVISIONS,
    nyse_arca_options_profile_at,
    NYSE_ARCA_OPTIONS_PROFILE,
    NYSE_ARCA_OPTIONS_REVISIONS,
    nyse_american_options_profile_at,
    NYSE_AMERICAN_OPTIONS_PROFILE,
    NYSE_AMERICAN_OPTIONS_REVISIONS,
    nasdaq_phlx_profile_at,
    NASDAQ_PHLX_OPTIONS_PROFILE,
    NASDAQ_PHLX_REVISIONS,
    nasdaq_ise_profile_at,
    NASDAQ_ISE_OPTIONS_PROFILE,
    NASDAQ_ISE_REVISIONS,
    nasdaq_nom_profile_at,
    NASDAQ_NOM_OPTIONS_PROFILE,
    NASDAQ_NOM_REVISIONS,
    box_options_profile_at,
    BOX_OPTIONS_PROFILE,
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
