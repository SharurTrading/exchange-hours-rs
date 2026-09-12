// SPDX-License-Identifier: MIT-0

//! COMEX gold, silver and copper Trading at Settlement (TAS) schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// COMEX Trading at Settlement books in America/Chicago, three keys on one
// envelope shape and three histories: Gold TAS (CME Globex `GCT`, security
// group TG), Silver TAS (`SIT`, group MT) and Copper TAS (`HGT`, group HT).
// They share the MRAN chain that states their windows and both of the
// order-entry revisions below, so they live in one module; they are three keys
// because their closes differ, their launch days differ and their pit
// eligibility differs. Platinum and palladium TAS are a separate lineage on a
// separate exchange and live in `pgm_tas.rs`.
//
// MEMBER LISTINGS ARE CALLER CATALOG DATA, NOT REVISIONS. Gold TAS covers
// `GCT` from 2010-04-11, E-micro/Micro Gold `MGT` from 2018-09-23 (Globex
// notice 20180903), and E-mini Gold `QOT` and 1-Ounce Gold `1OT` from
// 2025-07-27 (Globex notice 20250721, corroborated by COMEX Submission 25-240,
// "effective on Sunday, July 27, 2025, for trade date Monday, July 28, 2025").
// Silver TAS covers `SIT` and Micro Silver `MST`, also 2025-07-27. Copper TAS
// covers `HGT`, Micro Copper `MHT` (2025-07-27) and the spot-month root `HG0`
// — "known as TAS zero or TAS flat", in CME's own education article — whose
// listing day is undated: CME's copper Product Code cell shows it by
// 2018-08-10 but demonstrably drops TAS codes that exist (it shows none at all
// at 2015-10-08 and 2017-06-13, while `HGT` had traded since 2011-01-24), and
// all seventy archived weekly Globex notices from 2017-06-05 to 2018-08-27
// name it nowhere. Tracked as issue #80. Each of these joined a group that
// already had a grid, so none dates a revision row here.
// https://web.archive.org/web/20260214052421id_/https://www.cmegroup.com/notices/electronic-trading/2018/09/20180903.html
// https://www.cmegroup.com/notices/electronic-trading/2025/07/20250721.html
// https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2025/7/25-240.pdf
// https://web.archive.org/web/20250909102259id_/https://www.cmegroup.com/education/articles-and-reports/trade-at-settlement-for-metals.html
//
// NO REGULAR SESSION, AND THE PIT CLAUSE DOES NOT CREATE ONE. Four channels
// carry the negative for the TAS shape. CME, CBOT, NYMEX and COMEX Rule 524.A
// enumerate the execution routes — CME Globex, Rule 526 block trades, Rule 538
// EFP/EFR — and name no other. CME's ProductSlate carries forty-five TAS
// product records and every one reads `floor "-"`, `floorVol "0"` and a venues
// string of "Globex ClearPort " with no Floor component. The ContractSpecs API
// does have an "Open Outcry:" venue label — it prints one for SOFR options —
// and never attaches it to a TAS hours line. And the Daily Bulletin carries
// TAS as a volume column inside the ex-pit breakdown ("TAS DAILY TOTALS"),
// never as a priced product row, which already held on the 2015-03-20 pit-era
// edition where RTH volume was live.
//
// The 2009-2015 clause, recorded so the next reader does not "discover" it and
// conclude the key is wrong: NYMEX & COMEX Rule 524.A.1, from its adoption on
// 2009-09-14 until a bracket of 2015-03-06..2016-08-09, read "TAS transactions
// executed in the pit must be made open and competitively pursuant to the
// requirements of Rule 521 during the hours designated for pit trading in the
// particular contract". There WAS a pit-era COMEX TAS, for gold and silver
// only: SER S-5166 heads its sections "A. Gold Futures (GC) (CME Globex TAS
// code GCT) Traded on. CME Globex and COMEX Pit" and its cover letter dates the
// floor launch one day after the electronic one, "on the COMEX trading floor on
// April 12"; RA1107-4 confirms and adds that copper never was — "TAS
// transactions are allowed in the active contract month in Gold and Silver
// futures trading in the pit or on CME Globex and in the first active contract
// month in Copper futures trading on CME Globex. TAS transactions are not
// allowed in any pit-traded Copper futures contract month."
//
// It still gives these keys no regular session, on three grounds. The pit route
// had no window of its own — Rule 524.A.1 ties it to the underlying's pit
// hours. CME's own advisories list the pit route by the UNDERLYING's product
// code under "Pit-Traded Contracts" and the electronic route by the TAS root's
// own Globex code under "CME Globex Contracts", so the pit route is a pricing
// convention inside the underlying's pit and the instrument these keys model is
// the Globex TAS book. And CME's own pit column is unusable per row anyway: the
// metals hours page gives Gold TAS an Open Outcry cell of "08:20-13:30 ET
// (07:20-12:30 CT)" — the gold pit window — at every capture 2011-10-29 to
// 2015-03-22, gives Silver TAS none at any of them, and prints "08:10-13:00 ET"
// for Copper TAS once, which is CME's error: that is the copper pit
// MATCHED-ORDER window, stated as such by RA1005-4, RA1006-4 and RA1107-4
// ("Regular trading hours for open outcry trading in the Copper futures pit are
// from 8:10a.m. until1:00 p.m. Eastern Time"), and the cell is empty from
// 2012-05-01 on. `energy_metals.rs` already ships `regular: &[]` for the
// NYMEX/COMEX outrights in every era, so no NYMEX or COMEX pit is modelled as
// regular anywhere in this crate. Do not encode a pit or open-outcry window
// here.
// https://www.cmegroup.com/rulebook/files/cme-group-Rule-524.pdf
// https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul101811nymexandcomex001.pdf
//
// THE PRE-LAUNCH ERA IS A SOURCED CLOSURE, NOT AN UNWORKED GAP, SO NOTHING IS
// CARRIED BACK TO THE JANUARY-2010 FLOOR. RA0907-4 (2009-08-31) prints CME's
// complete "CME Globex Contracts" TAS list as eleven codes — `WST`, `RTI`,
// `BHT`, `HPT`, `HHT`, `BBT`, `CLT`, `HOT`, `NGT`, `RBT`, `RET` — and copper
// appears in it only as a pit-traded MATCHED ORDER product, which is not TAS.
// RA1001-4 / RA1002-4 (2010-02-02), one month after the audit floor, reprints
// the complete list with fifteen codes and still no gold, silver or copper. So
// the pre-2010-11-01 Rule 524.A.2 sentence that supplies the energy-TAS floor
// era — "TAS transactions on Globex may take place at any time the applicable
// contracts are available for trading on Globex" — is inapplicable here: gold
// was not an applicable contract. RA1005-4 (2010-10-11) and RA1006-4
// (2010-10-22) carry `GCT` and `SIT` rows and no `HGT` row, which closes
// copper's own pre-launch era the same way.
// https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul083109nymexandcomex001.pdf
// https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul020210nymexandcomex001.pdf
//
// LAUNCH DAYS, ALL UNCONDITIONAL AND KEYED TO THE LOCAL OPENING DAY. Gold and
// silver: NYMEX/COMEX Submission 10-070 with COMEX Special Executive Report
// S-5166 attached, a Regulation 40.6 self-certification whose cover letter
// states "the launch of TAS pricing for the active month in each of Gold
// Futures (Chapter 113) and Silver Futures (Chapter 112) on Globex on April 11
// (for trade date April 12)". The grid opens 17:00 CT on the Sunday, so the row
// is keyed to Sunday 2010-04-11. S-5166 is not separately retrievable at any
// CME path; it survives as an attachment inside the CFTC filing, and is cited
// that way. Copper: COMEX SER-5542 (2010-12-22), "effective Sunday, January 23,
// 2011 for trade date Monday, January 24, 2011 … COMEX will offer TAS in the
// first and second active months for Copper Futures on CME Globex only" —
// keyed to Sunday 2011-01-23, the same day RA1101-4 carries as its own
// Effective Date while printing `HGT`'s window.
// https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul031110nymexandcomex001.pdf
// https://web.archive.org/web/20120512095523id_/http://www.cmegroup.com/rulebook/files/SER-5542__10-12-22__TAS_COMEX.pdf
//
// THE WINDOW, IN CME'S OWN SESSION LANGUAGE. RA1006-4 is the earliest clean
// statement, six months after the gold and silver launch, and it is a session
// statement rather than a calculation window: "the end of the trading session
// is defined by receipt of the security status message indicating that group is
// closed". Its table reads "GCT COMEX Gold … No-Activity Periods: 1:30 p.m.-
// 5:45 p.m. (ET) Monday- Thursday / 1:30 p.m. (ET) Friday- 5:15 p.m. (ET)
// Sunday" and "SIT COMEX Silver … 1:25 p.m.- 5:45 p.m. (ET) Monday -Thursday /
// 1:25 p.m. (ET) Friday-5:15p.m. (ET) Sunday"; RA1101-4 adds "HGT COMEX Copper
// … 1:00 p.m.- 5:45 p.m. (ET) Monday- Thursday / 1:00 p.m. (ET)
// Friday-5:15p.m. (ET) Sunday". So the closes are 12:30, 12:25 and 12:00 CT;
// the weekday queue opens 16:45 CT and the Sunday queue 16:15 CT; and there is
// NO Friday-evening reopen, because the dark run goes Friday close straight to
// Sunday 17:15 ET. RA1102-4 (advisory 2011-01-07, effective 2011-01-23, sha256
// a71fb96d6821ed4396be3c726c42beb9e299b55ae3f9e1de2a2b2706df193607) reprints
// all three rows unchanged and is the fourth printing of the same table.
//
// The 17:00 CT OPEN is not in the MRAN chain, which prints only dark windows.
// It comes from CME's metals trading-hours page, whose Gold TAS row reads
// "18:00-13:30 ET (17:00-12:30 ET)" — the page's own zone typo in the second
// pair, preserved — with Silver TAS at 18:00-13:25 ET and Copper TAS at
// 18:00-13:00 ET, and it is carried back to each launch day with no revision
// asserted, exactly as `AGENTS.md`'s carry-back convention provides.
// https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul102110nymexandcomex001.pdf
// https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul010311nymexandcomex001.pdf
// https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul010611nymexandcomex001.pdf
// https://web.archive.org/web/20111029073737id_/http://www.cmegroup.com/trading_hours/metals-hours.html
//
// EVERY INTER-TRADE-DATE GAP EXCEEDS FOUR HOURS, SO IT IS `Closed`, NOT
// `Maintenance`. Gold 12:30->17:00 is 4h30m, silver 4h35m, copper 5h. The
// crate's convention retains an inter-trade-date gap as `Maintenance` only up
// to four elapsed hours; none of these three qualifies. Stated here so nobody
// "fixes" it later.
//
// ZERO EXECUTABLE REVISION ROWS, LAUNCH TO TODAY, AND THAT IS SOURCED
// CONSTANCY RATHER THAN AN UNWORKED ROW. Gold's 12:30 CT close is stated at
// RA1006-4, RA1101-4, RA1102-4 and RA1104-4; on the metals hours page at eight
// captures between 2011-10-29 and 2015-03-22; on the server-rendered contract
// specification from 2019-11-18 to 2021-06-21 ("TAS: Sun-Fri 6:00 p.m. - 1:30
// p.m. ET (5:00 - 12:30 CT)"); on CME's archived ContractSpecs API at
// 2021-07-10, 2022-08-26, 2023-06-17, 2025-06-04, 2025-10-16, 2026-05-27 and
// 2026-08-27; and live. Silver carries the same shape at 12:25 (seven archived
// API captures 2021-07-10..2026-05-27 alone) and copper at 12:00. No source
// names a change at any point, so no cutover is asserted.
//
// OBSERVATION GAP TO RECORD, 2015-03-22 -> 2019-11-18. It is channel
// exhaustion rather than a sampling choice: `metals-hours.html` has no
// 200-status capture after 2015-03-22 (301s thereafter), and the
// contract-specification channel carried no TAS hours line until autumn 2019
// (gold absent 2019-09-24, present 2019-11-18; copper absent 2019-07-20,
// present 2019-12-20). This is why the row is `Partial` with an executable gap.
// https://web.archive.org/web/20120914235904id_/http://www.cmegroup.com/trading_hours/metals-hours.html
// https://web.archive.org/web/20150322121442id_/http://www.cmegroup.com/trading_hours/metals-hours.html
// https://web.archive.org/web/20191118131805id_/http://www.cmegroup.com/trading/metals/precious/gold_contract_specifications.html
// https://www.cmegroup.com/CmeWS/mvc/ContractSpecs/List/productId/437
//
// THE 2015-09-20 DCM-WIDE MOVE DID NOT REACH METALS TAS, on two independent
// arguments. Scope, in the notice's own words: Globex notice 20150817 says the
// daily maintenance period begins fifteen minutes earlier and "the closing
// times for the following markets will now occur 15 minutes earlier Monday
// through Friday at 16:00 CT" for CME Equity, CBOT Equity, COMEX, NYMEX and
// DME. A 12:30 / 12:25 / 12:00 CT close is not a 16:15 CT close and cannot move
// to 16:00. And direct observation: the last TAS statement before the gap
// (2015-03-22) and the first after it (2019-11-18) are the same values. Note
// the notice's own stated day is Monday 2015-09-21 — 2015-09-20 was a Sunday —
// so "trade date 2015-09-20" is wrong wherever it appears. The control that
// makes these separate keys from `globex_energy`: the same notice demonstrably
// DID move the metals outrights, gold's specification going from "5:15 p.m. …
// 45-minute break" on 2015-09-06 to "5:00 p.m. … 60-minute break" on
// 2015-10-09.
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150817.html
// https://web.archive.org/web/20150906022503id_/http://www.cmegroup.com/trading/metals/precious/gold_contract_specifications.html
// https://web.archive.org/web/20151009145054id_/http://www.cmegroup.com/trading/metals/precious/gold_contract_specifications.html

static GOLD_EXTENDED: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 12 * 3600 + 30 * 60,
}];
static SILVER_EXTENDED: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 12 * 3600 + 25 * 60,
}];
static COPPER_EXTENDED: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 12 * 3600,
}];

// THE TWO DATED REVISIONS ARE BOTH ORDER-ENTRY ONLY, AND BOTH TOUCH ONLY THESE
// THREE KEYS.
//
// 2011-04-10 (Sunday opening day; trade date Monday 2011-04-11) — RA1104-4:
// "Effective on Sunday, April 10, 2011, for trade date Monday, April 11, 2011,
// the pre-opening times for TAS trading on CME Globex will be as follows (all
// times are in Eastern Time): … COMEX Gold Monday - Thursday at 5:48 p.m.,
// Sunday at 5:18 p.m. … COMEX Silver Monday - Thursday at 5:49 p.m., Sunday at
// 5:19 p.m. … COMEX Copper Monday - Thursday at 5:50 p.m., Sunday at 5:20
// p.m." — 16:48/16:18, 16:49/16:19 and 16:50/16:20 CT. The same table leaves
// every close unchanged at 1:30 / 1:25 / 1:00 p.m. ET, and its stated purpose
// is queue management: the revised times "are being implemented to stagger and
// more evenly distribute order flow during the pre-opening time period".
// Corroborated by the hours page's 17:18 / 17:48 ET cells at 2011-10-29 and
// 2012-05-01. RA1104-4 is the last MRAN edition to print per-product clock
// times; RA1106-4 replaces the table with a security-status-message definition.
// https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul033111nymexandcomex001.pdf
//
// 2012-04-15 (Sunday opening day; trade date Monday 2012-04-16) — CME Globex
// notice 20120409, announced a week earlier in 20120402: "On Sunday, April 15
// (trade date Monday, April 16) … CME Group will randomize the timing of each
// TAS groups' pre-open state. … There will be a market pause … at 16:15:00 on
// Sunday and 16:45:00 Monday through Thursday. TAS groups will then go into
// pre-open on Sundays between 16:15:00 and 16:16:00 Central time (CT), and
// Mondays through Thursdays between 16:45:00 and 16:46:00 CT." Unconditional,
// day-level, and it ends the stagger: every TAS group returns to the shared
// 16:15 / 16:45 CT onsets. The crate encodes the nominal onset, never the
// randomized second, which RA2302-5 confirms is still the live construction.
// That the hours page still printed 16:48/16:49/16:50 on 2012-06-16 is page
// staleness and is evidenced rather than asserted: the same URL prints
// "Pre-Open Electronic Trading (Weekday) 17:45-17:46 ET (16:45-16:46 CT)" for
// all three metals at 2012-09-14 and again at 2013-09-02, so the notice's
// adoption is observed within five months.
// https://web.archive.org/web/20190716065836id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120409.html
// https://web.archive.org/web/20190722114429id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120402.html
// https://web.archive.org/web/20130902013752id_/http://www.cmegroup.com/trading_hours/metals-hours.html
//
// THE SUNDAY QUEUE SERVES THE SOURCED INTERSECTION, AND 16:00-16:15 IS
// WITHHELD. The Sunday onset is sourced at 16:15 (RA1006-4, RA1101-4,
// RA1102-4), then 16:18/16:19/16:20 (RA1104-4), then 16:15 again (the
// 2012-04-09 notice), then 16:00 — undated, bracketed to
// (2012-04-15, 2012-06-16] by the hours page's "17:00-17:01 ET (16:00-16:01
// CT)" cell of 2012-06-16, stated by CME's client-systems wiki in 2025, and
// announced by none of the twelve archived weekly Globex notices covering
// 2012-04-16 to 2012-06-25. `AGENTS.md`'s "a knowledge boundary may only
// widen" therefore holds the bound at its narrowest value across the whole
// undated span: these keys serve 16:15->17:00 (16:18/16:19/16:20 inside the
// 2011-04-10 row) and withhold the 16:00-16:15 quarter-hour, exactly as
// `globex_equity_index`'s Sunday Pre-Open already does — and it is almost
// certainly the same CME-wide 2012 move, because the wiki's paragraph is
// word-for-word the 2012-04-09 notice with that one number changed. Whoever
// dates one dates all four rows; tracked as issue #79. The weekday onset needs
// no withholding: 16:45 -> 16:48/16:49/16:50 -> 16:45, all three states dated.
// https://cmegroupclientsite.atlassian.net/wiki/spaces/EPICSANDBOX/pages/457223974/Trade+at+Settlement+-+TAS
// https://web.archive.org/web/20120616193920id_/http://www.cmegroup.com/trading_hours/metals-hours.html

/// The shared TAS queue: Sunday 16:15-17:00 and Monday-Thursday 16:45-17:00 CT.
///
/// In force from each key's launch to 2011-04-09 and again from 2012-04-15, and
/// the current table. The Sunday rule starts at the sourced intersection's
/// 16:15, not at the wiki's undated 16:00.
pub(crate) static METALS_TAS_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
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

/// RA1104-4's per-product stagger, in force 2011-04-10 to 2012-04-14.
macro_rules! staggered_queue {
    ($name:ident, $sunday_minute:expr, $weekday_minute:expr) => {
        static $name: &[SessionRule] = &[
            SessionRule {
                days: SUN_ONLY,
                open_ssm: 16 * 3600 + $sunday_minute * 60,
                close_ssm: 17 * 3600,
            },
            SessionRule {
                days: MON_THU,
                open_ssm: 16 * 3600 + $weekday_minute * 60,
                close_ssm: 17 * 3600,
            },
        ];
    };
}
staggered_queue!(GOLD_ORDER_ENTRY_2011_04_10, 18, 48);
staggered_queue!(SILVER_ORDER_ENTRY_2011_04_10, 19, 49);
staggered_queue!(COPPER_ORDER_ENTRY_2011_04_10, 20, 50);

const fn profile(
    extended: &'static [SessionRule],
    order_entry: &'static [SessionRule],
) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: US::Central,
        regular: &[],
        extended,
        order_entry,
        has_daily_close: true,
        has_weekend_close: true,
    }
}

/// Before the launch day the root was not TAS-eligible, and CME's own complete
/// eligibility lists say so, so the pre-launch era is a sourced closure.
static CLOSED: StaticHoursProfile = profile(&[], &[]);

static GOLD_AT_LAUNCH: StaticHoursProfile = profile(GOLD_EXTENDED, METALS_TAS_ORDER_ENTRY_CURRENT);
static GOLD_FROM_2011_04_10: StaticHoursProfile =
    profile(GOLD_EXTENDED, GOLD_ORDER_ENTRY_2011_04_10);
static SILVER_AT_LAUNCH: StaticHoursProfile =
    profile(SILVER_EXTENDED, METALS_TAS_ORDER_ENTRY_CURRENT);
static SILVER_FROM_2011_04_10: StaticHoursProfile =
    profile(SILVER_EXTENDED, SILVER_ORDER_ENTRY_2011_04_10);
static COPPER_AT_LAUNCH: StaticHoursProfile =
    profile(COPPER_EXTENDED, METALS_TAS_ORDER_ENTRY_CURRENT);
static COPPER_FROM_2011_04_10: StaticHoursProfile =
    profile(COPPER_EXTENDED, COPPER_ORDER_ENTRY_2011_04_10);

// Revision evidence — each row's day-level effective date, keyed to the local
// Sunday opening day of the first session it governs, and the primary source
// that states it (full quotations sit in the blocks above):
//   2010-04-11 "COMEX Submission 10-070 with SER S-5166"
//     https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul031110nymexandcomex001.pdf
//   2011-01-23 "COMEX SER-5542"
//     https://web.archive.org/web/20120512095523id_/http://www.cmegroup.com/rulebook/files/SER-5542__10-12-22__TAS_COMEX.pdf
//   2011-04-10 "NYMEX & COMEX MRAN RA1104-4"
//     https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul033111nymexandcomex001.pdf
//   2012-04-15 "CME Globex notice 20120409"
//     https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120409.html
//
// The 2012-04-15 rows point back at each key's launch profile because the
// notice restores exactly the shared 16:15 / 16:45 CT onsets that RA1104-4
// staggered; the tables are shared rather than copied so a future correction
// cannot move one and not the other.
static GOLD_REVISIONS: &[Revision] = revisions![
    (
        2010,
        4,
        11,
        &GOLD_AT_LAUNCH,
        "COMEX Submission 10-070 with SER S-5166"
    ),
    (
        2011,
        4,
        10,
        &GOLD_FROM_2011_04_10,
        "NYMEX & COMEX MRAN RA1104-4"
    ),
    (2012, 4, 15, &GOLD_AT_LAUNCH, "CME Globex notice 20120409"),
];
static SILVER_REVISIONS: &[Revision] = revisions![
    (
        2010,
        4,
        11,
        &SILVER_AT_LAUNCH,
        "COMEX Submission 10-070 with SER S-5166"
    ),
    (
        2011,
        4,
        10,
        &SILVER_FROM_2011_04_10,
        "NYMEX & COMEX MRAN RA1104-4"
    ),
    (2012, 4, 15, &SILVER_AT_LAUNCH, "CME Globex notice 20120409"),
];
static COPPER_REVISIONS: &[Revision] = revisions![
    (2011, 1, 23, &COPPER_AT_LAUNCH, "COMEX SER-5542"),
    (
        2011,
        4,
        10,
        &COPPER_FROM_2011_04_10,
        "NYMEX & COMEX MRAN RA1104-4"
    ),
    (2012, 4, 15, &COPPER_AT_LAUNCH, "CME Globex notice 20120409"),
];

/// The current gold TAS matching leg: Sunday-Thursday 17:00 CT wrapping to
/// 12:30 CT, with no Friday-evening reopen.
pub(crate) static GOLD_TAS_EXTENDED_CURRENT: &[SessionRule] = GOLD_EXTENDED;
/// The current silver TAS matching leg, wrapping to 12:25 CT.
pub(crate) static SILVER_TAS_EXTENDED_CURRENT: &[SessionRule] = SILVER_EXTENDED;
/// The current copper TAS matching leg, wrapping to 12:00 CT.
pub(crate) static COPPER_TAS_EXTENDED_CURRENT: &[SessionRule] = COPPER_EXTENDED;

pub(crate) fn gold_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, GOLD_REVISIONS)
}

pub(crate) fn silver_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, SILVER_REVISIONS)
}

pub(crate) fn copper_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, COPPER_REVISIONS)
}
