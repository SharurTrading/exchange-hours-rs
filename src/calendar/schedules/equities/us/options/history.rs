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
// Knowledge-bound rows (the final 2026-08-22 row in every table below except
// MEMX_REVISIONS, whose profile carries no queue at all — MEMX rejects orders
// before 09:30, so its sourced launch day is its only row): each
// venue's current order-acceptance queue is primary-verified in the current
// envelope, but the reviewed sources state no day-level amendment chain for
// these queues, so earlier dated queries retain only the exact 09:30–16:00
// execution history. From the 2026-08-22 repository review onward each
// verified-current grid applies; a sourced onset day replaces its row.
//
// Why no onset day exists — the structural finding of the 2026-08-31 queue
// review. On every venue in this module the generic order-acceptance start is
// an operator *system setting* published on a mutable hours or system-settings
// page, not a rulebook boundary carrying an SEC-filed operative date. The two
// filings that codified the Cboe queuing periods say so explicitly, each
// declining to change the time they wrote down:
//   SR-C2-2019-009 (84 FR 20673, 2019-05-10) — "The Queuing Period begins
//   at 7:30 a.m. for all class[es]. This is the same time at which the System
//   begins accepting orders and quotes today." Its footnote records that Cboe
//   Options Rule 6.2(a) bounds the pre-opening period ("no earlier than 2:00
//   a.m. Central time") rather than fixing it.
//   https://www.federalregister.gov/documents/2019/05/10/2019-09634/
//   SR-CboeBZX-2020-012 (85 FR 6246, 2020-02-04) — same sentence for BZX.
//   https://www.federalregister.gov/documents/2020/02/04/2020-02049/
// Nasdaq states each venue's start in its per-venue "System Settings" document
// ("System begins accepting orders"), NYSE on its hours-and-calendars page
// ("Pre-Opening Session: 6:00 a.m. ET"), and MIAX on its trade-hours calendar
// ("Firm Interface Startup Time"). None of those channels publishes a dated
// change notice for the value, so these rows are knowledge-bound by the shape
// of the evidence rather than by an unfinished search.
//
// Sourced lower bounds recovered by that review — each queue was already at
// its current value on the stated day, so any onset precedes it:
//   C2 7:30 — 2019-05-10 (SR-C2-2019-009, above).
//   BZX Options 7:30 — 2020-02-04 (SR-CboeBZX-2020-012, above).
//   ISE 6:00 — 2019-10-17 "Nasdaq ISE INET System Settings", official origin
//   https://www.nasdaq.com/docs/ISESystemSettings.pdf delivered via
//   https://web.archive.org/web/20191017150502id_/https://www.nasdaq.com/docs/ISESystemSettings.pdf
//   MIAX Options 7:30 — 2013-05-07 (the live-book capture below).
//
// MIAX Options is the one venue whose queue onset is now bracketed rather than
// open. Its 07:30 window existed at the sourced 2012-12-07 launch but was not
// order acceptance: the official hours page captured two days later states
// that pre-Live-Quote-Window activity "will be acknowledged for connectivity
// verification purposes, but WILL NOT affect the live quote state", while the
// next capture states that the same activity WILL affect the live book. The
// launch-era row below is therefore positively sourced as queue-free, and the
// order-acceptance onset falls in 2012-12-09..2013-05-07 with no operator
// notice in the archived alert channels stating the day. Official origin
// http://www.miaxoptions.com/hours-operation-miax-options-exchange delivered
// via
// https://web.archive.org/web/20121209014257id_/http://www.miaxoptions.com/hours-operation-miax-options-exchange
// https://web.archive.org/web/20130507151726id_/http://www.miaxoptions.com/hours-operation-miax-options-exchange
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
