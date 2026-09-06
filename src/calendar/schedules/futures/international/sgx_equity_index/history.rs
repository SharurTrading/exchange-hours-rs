// SPDX-License-Identifier: MIT-0

//! Sourced history for the SGX equity-index families.
//!
//! The current-grid tables and the dated revision rows live in the parent
//! module and the pre-2020 tables in the sibling `eras` module; this module
//! owns the published evidence behind every row — which artifacts were read,
//! where they disagree, how the undated changeovers are served, and what
//! residual risk each era carries.

use chrono_tz::Asia;

use super::{MON_FRI, SessionRule, StaticHoursProfile};

// SGX EQUITY-INDEX HISTORY.
//
// THE CALENDAR EDITIONS. Nine published files of SGX's Derivatives Trading
// Calendar — static, readable PDFs under api2.sgx.com/sites/default/files/, of
// which eight are distinct documents — state these grids from 2020 on:
//
//   edition                    Japan T / T+1        China  SiMSCI  Taiwan  NTR
//   2020, 2021-01, 2021-07,
//   2022-06, 2024              07:30-14:25 / 14:55  17:00  17:50   14:15   19:00
//   2025-01                    07:30-14:55 / 15:25  17:00  17:50   14:15   19:00
//   2025-07 (= 2025-11), 2026  07:30-14:55 / 15:10  16:45  17:35   14:00   18:45
//
// (2025-11/DT Trading Calendar 2025.pdf is byte-identical to 2025-07/DT Trading
// Calendar 2025 (updated 31 Jul 2025).pdf - verified by digest - so those two
// files are one edition. The 2022 edition has no text layer and was read from
// rendered pages; no 2023 edition was located.) Two earlier editions are still
// served from the same host although the web archive never captured them: the
// 2018 (Apr) edition (PDF created 2018-04-11) and the 2019 edition ("accurate
// as of 15 January 2019"). Both print the 04:45 T+1 close.
//
// THREE OF THE FOUR MOVES INSIDE THE MODELLED WINDOW ARE DATED.
//
// 2025-04-07. SGX-DT Circular DT/AM - 15 of 2025, "Revision of T+1 Session
// Trading Hours for SGX Equity Index Futures/Options, Dividend Index Futures
// and United States Single Stock Futures (US SSFs)", 24 February 2025: "with
// effect from Monday, [7] April 2025" the T+1 pre-open routine comes forward
// ten minutes and shortens to five, with "no change to the T session trading
// hours". Its Appendix A lists every affected contract's current and revised
// Pre-Opening/Non-Cancel/Opening times, which are exactly the current grids.
//
// 2024-11-04. SGX-DT Circular DT/AM - 50 of 2024, "Extension of T-session for
// SGX Japan Derivatives and Intraday Margin Cycle 2 Timing Change", 9
// September 2024, signed Leno Lee, SVP Trading and Clearing Services, on
// Singapore Exchange Derivatives Trading Limited letterhead: "with effect from
// Monday, 4 November 2024, the T session trading hours for SGX Nikkei
// derivatives and SGX FTSE Blossom Japan Index Futures will be extended by 30
// minutes", listing NK, NKO, NR, ND, NU, NS, NC, EJRT and EJP, with a
// Current -> Revised table of every routine (T Opening 7.30 am - 2.25 pm ->
// 2.55 pm; Pre-Closing 2.25-2.29 -> 2.55-2.59; T+1 Pre-Opening 2.45-2.53 ->
// 3.15-3.23, Opening 2.55 pm -> 3.25 pm - 5.15 am). It names no China,
// Singapore, Taiwan or NTR contract, so only the Japan key splits there. Read
// from a verbatim member mirror (Fubon Futures), whose file carries SGX's own
// Word metadata (created and last saved 2024-09-09 17:53 Singapore time, the
// circular's dateline) - the same channel DT/AM 15 came through, see below.
//
// TWO MOVES STAY UNDATED AND ARE SERVED AS INTERSECTIONS. The T+1 close moved
// 04:45 -> 05:15 between SGX's content API of 2019-06-21 (04:45) and its
// 2020-01-09 payload and the 2020 edition (05:15); SGX's Annual Report 2020
// says "Our extension of trading hours in November 2019" and two SGX
// factsheets in the api2 2019-11 and 2019-12 directories print 05:15, but no
// SGX document reachable under the crate's laws states the day, so 04:45 is
// served until the 2020 row. The SiMSCI T close and T+1 open moved 17:10 /
// 17:40 -> 17:20 / 17:50 between the content API's 2019-02-04 and 2019-06-11
// payloads; the Singapore key serves the window both hold in between.
//
// SGX'S OWN PRODUCT-CATALOGUE CHANGE LOG STATES BOTH DAYS. The "Derivatives
// Products Description" workbook served from api2.sgx.com carries a dated,
// SGX-authored change log (sheet read_me, header "Issue Date (mm/dd/yyyy) |
// Version Number | Authors | Change Description", no merged cells, multi-row
// entries partitioned by border styles in the file). Its entry issued
// 2019-05-21, v6.1, reads "Amended trading hours for SGP, SGPO and ST eff 10
// Jun"; its entry issued 2019-10-07, v6.9, reads "Effective 11 Nov:" / "(T+1)
// session Closing hours to 5:15am all T+1 traded contracts"; and its later
// entries "(eff 4 Nov)" and "(eff 7 Apr)" match DT/AM 50 and DT/AM 15 exactly.
// Whether that channel may key a revision row is an open maintainer decision
// (#45); until it is taken these two moves stay intersections, and the rows
// that would change are exactly the Singapore 2019-02-06 and 2019-06-11 pair
// (one dated Monday 2019-06-10) and the 05:15 close on all five keys (Monday
// 2019-11-11 instead of the 2020 row).
//
// BEFORE THE 2020 EDITION: THE STATES SGX PUBLISHED. All Singapore time, T and
// T+1 session bounds, routines excluded per the pages' own footnote.
//
//   W   ~2012   Nikkei 07:45-14:25 / 15:15-02:00; A50 09:00-15:25 / 16:10-02:00;
//               MSCI S'pore 08:30-17:10 / 18:15-02:00; MSCI Taiwan 08:45-13:45 /
//               14:35-02:00. An orphaned wcm/connect fragment captured
//               2018-07-11 whose contract set ("S&P CNX Nifty", "FTSE Xinhua",
//               "MSCI Asia Apex 50") dates its content to about 2012.
//   S0  2013-08-20 portal Trading Hours table: as W except A50 09:00-15:55 /
//               16:40-02:00. The same page's holiday tables still print the
//               A50 at 15:25, which orders W before S0.
//   A   2017-07-05 and 2017-09-27 portal captures, byte-identical: Nikkei
//               07:30-14:25 / 14:55-04:45; A50 09:00-16:30 / 17:00-04:45; MSCI
//               S'pore 08:30-17:10 / 17:40-04:45; MSCI Taiwan 08:45-13:45 /
//               14:15-04:45. No NTR row.
//   B   2018 (Apr) and 2019 editions, content API 2019-01-16 and 2019-02-04:
//               as A, adding NTR (USD) 07:25-18:30 / 19:00-04:45.
//   B'  content API 2019-06-11: as B except SiMSCI 08:30-17:20 / 17:50-04:45.
//   C   content API 2020-01-09 and the 2020 edition: as B' with 05:15.
//
// A CAPTURE DATES THE OBSERVATION, NEVER THE STATE. State A was live on
// 2017-07-05 while the W fragment, a year older in content, was still being
// served in 2018; SGX ran two content trees. So the 2017-07-05 capture is an
// upper bound on state A's onset and can only under-date it - and SGX's
// change log agrees: its entries of 2016-04-22 ("Equity Index & Dividend Index
// products T/T+1 gap increased to 15mins", the fifteen-minute gap state A
// leaves between each closing routine and its T+1 pre-open) and 2016-07-15
// ("Amendments to NK suite, CH, CHO and CN trading hours") place the move in
// 2016, though neither states a day (#66). The 21 September 2017 "Change of
// Trading Hours" newsletter is not the move: captures straddle it and are
// identical.
//
// HOW THE ROWS ARE KEYED. Japan, China and Singapore have members listed with
// a grid in the oldest artifact, so their oldest state is carried to the
// January-2010 floor as each key's `select_revision` baseline - it asserts no
// revision row - under the carry-back convention. FTSE Taiwan and the NTR
// (USD) suite did not exist then and stay sessionless below their own first
// listing. The State-A boundary is a knowledge boundary, keyed to Monday
// 2017-07-10 rather than the Wednesday capture: a boundary that lengthens a
// wrapping overnight close, keyed mid-week, would report the previous
// evening's leg running past the close in force when it opened - the running
// session LAW-NO-FABRICATED-DATES says a boundary must not split - and no leg
// wraps into a Monday morning. The NTR boundary moves to Monday 2018-04-16 for
// the same reason. Boundaries that change only a daytime close (2013-08-20,
// 2019-06-11) or narrow an overnight open (2019-02-06) keep their artifact's
// date. A knowledge boundary may widen what is served and never narrow it:
// the A50's T+1 open is sourced at 16:10, 16:40 and 17:00 across three undated
// states, so 17:00 is held from the floor and only its T close, which widens,
// moves at the boundaries.
//
// ROUTINES. The calendars print session bounds only, but SGX's content API
// prints every family's Pre-Opening/Non-Cancel/Pre-Closing windows, and its
// 2020-01-09 payload states them inside the 2020 row's own interval - so the
// 2020 rows carry them on all five keys, as do the earlier eras those payloads
// and the 2013 specification page source; the floor eras of the Japan and
// Singapore keys have no pre-open queue because no artifact of that era states
// its length (#65). DT/AM 50 states Japan's routines on both sides of
// 2024-11-04 and DT/AM 15 every family's from 2025-04-07; between the 2020
// payload and those circulars the routines are carried forward (#64).
//
// RESIDUAL RISKS, STATED. (1) Third-party press attests a 01:00 T+1 close
// until 30 August 2010; the floor rows serve 02:00 from the January-2010
// floor, so the first eight months may over-report an hour of T+1 - carried
// anyway on the convention's explicit instruction, press being inadmissible
// for a row. (2) The 2014-2016 calendar editions lived in a WCM store that is
// in neither the archive nor the live site, so states between S0 and A are
// unwitnessed; the intersection covers the sourced endpoints only. (3) NKO's T
// close is five minutes later than NK's in every era and is not modelled, as
// FCHO is not on the China grid. (4) The Mini Nikkei (NS) and Dividend Point
// (ND) contracts ran their own later closes in W, S0 and - for ND - A before
// joining the NK grid (NS by 2017-07-05, ND by the 2018 edition); the family
// clock is the NK/NU grid throughout.
//
// CHANNELS. SGX publishes no DT/AM circular at a publicly reachable sgx.com
// address (regco.sgx.com's /circulars route answers `null`; the api2 file
// store is not listable), so both circulars above were read from verbatim
// member-hosted copies - SGX letterhead, circular number, signatory, both
// appendices - CITIC Futures International for DT/AM 15 and Fubon Futures
// (via the web archive) for DT/AM 50; SGX's own Titan DT/DC portal corroborates
// each issue date. The retired portal's pages are archived; the 2018 and 2019
// calendars and the content API's payloads are SGX-served.
//
// https://api2.sgx.com/sites/default/files/2018-05/SGX%20Derivatives%20Trading%20Calendar%202018%20%28Apr%29.pdf
// https://api2.sgx.com/sites/default/files/2019-01/2019%20DT%20Calendar.pdf
// https://api2.sgx.com/sites/default/files/2020-01/SGX%20Derivatives%20Trading%20Calendar%202020.pdf
// https://api2.sgx.com/sites/default/files/2021-01/SGX%20Derivatives%20Trading%20Calendar%202021.pdf
// https://api2.sgx.com/sites/default/files/2021-07/SGX_Derivatives%20Trading%20Calendar%202021%20%28Final%20-%20Jul%29.pdf
// https://api2.sgx.com/sites/default/files/2022-06/DT%20Trading%20Calendar%202022%20%28Final%29.pdf
// https://api2.sgx.com/sites/default/files/2024-01/SGX%20Calendar%202024.pdf
// https://api2.sgx.com/sites/default/files/2025-01/SGX%20Calendar%202025.pdf
// https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
// https://web.archive.org/web/20130820090335id_/http://www.sgx.com/wps/portal/sgxweb/home/trading/derivatives/trading_hours_calendar
// https://web.archive.org/web/20170705000242id_/http://sgx.com/wps/portal/sgxweb_ch/home/trading/derivatives/trading_hours_calendar
// https://web.archive.org/web/20170927124017id_/http://www.sgx.com/wps/portal/sgxweb/home/trading/derivatives/trading_hours_calendar
// https://web.archive.org/web/20190204200905id_/https://api2.sgx.com/content-api?queryId=00c0b9e1a1b4dc0b3e0a3a8f3b0c2d5e6f7a8b9c:derivatives_products_list
// https://web.archive.org/web/20190611051800id_/https://api2.sgx.com/content-api
// https://web.archive.org/web/20200109051211id_/https://api2.sgx.com/content-api
// https://web.archive.org/web/20241114183232id_/https://www.fubon.com/futures/wcm/home/bulletin/bulletin_20240912_137396/SGXChange.pdf
// https://www.citicsf.com.hk/attachment?aid=95&uid=a1207308-0e3a-4a16-a869-a4d1b808a2b3
// https://www.sgx.com/titan-dt-dc-portal
// https://api2.sgx.com/sites/default/files/2026-08/Derivatives+Products+Description+v17.6%20eff%2020260824,%2020260907.zip

// THE 2020 ROWS. Session bounds from the 2020 edition; routines from the
// content API's 2020-01-09 payload, which states them for every family on
// this grid: Japan "Pre -Opening : 7.15 am - 7.28 am / Non -Cancel : 7.28 am -
// 7.30 am / Opening : 7.30 am - 2.25 pm / Pre-Closing : 2.25 pm - 2.29 pm /
// Non-Cancel : 2. 29 pm - 2.30 pm // Pre -Opening : 2.45 pm - 2.53 pm / Non
// -Cancel : 2.53 pm - 2.55 pm / Opening : 2.55 pm - 5.15 am"; China "Pre -
// Opening: 8.45 am - 8.58 am / ... Opening: 9.00 am - 4.30 pm / Pre - Closing:
// 4.30 pm - 4.34 pm / Non - Cancel: 4.34 pm - 4.35 pm // Pre - Opening: 4.50
// pm - 4.58 pm / Non - Cancel: 4.58 pm - 5.00 pm / Opening: 5.00 pm - 5.15
// am"; Singapore "Pre - Opening: 8:15 am - 8:28 am / ... Opening: 8:30 am -
// 5:20 pm / Pre - Closing: 5:20 pm - 5:24 pm / Non - Cancel: 5:24 pm - 5:25 pm
// // Pre - Opening: 5:40 pm - 5:48 pm / Non - Cancel: 5:48 pm - 5:50 pm /
// Opening: 5:50 pm - 5:15 am". Each Pre-Opening/Non-Cancel pair is one
// order-entry window and each Pre-Closing/Non-Cancel pair one extended window.
static SGX_EQUITY_INDEX_JAPAN_REGULAR_2020: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 30 * 60,
        close_ssm: 14 * 3600 + 25 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 55 * 60,
        close_ssm: 5 * 3600 + 15 * 60,
    },
];
static SGX_EQUITY_INDEX_JAPAN_EXTENDED_2020: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 14 * 3600 + 25 * 60,
    close_ssm: 14 * 3600 + 30 * 60,
}];
static SGX_EQUITY_INDEX_JAPAN_ORDER_ENTRY_2020: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 15 * 60,
        close_ssm: 7 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 45 * 60,
        close_ssm: 14 * 3600 + 55 * 60,
    },
];
pub(super) static SGX_EQUITY_INDEX_JAPAN_SOURCED_WINDOW: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_JAPAN_REGULAR_2020,
    extended: SGX_EQUITY_INDEX_JAPAN_EXTENDED_2020,
    order_entry: SGX_EQUITY_INDEX_JAPAN_ORDER_ENTRY_2020,
    has_daily_close: true,
    has_weekend_close: true,
};

static SGX_EQUITY_INDEX_CHINA_REGULAR_2020: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 16 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600,
        close_ssm: 5 * 3600 + 15 * 60,
    },
];
static SGX_EQUITY_INDEX_CHINA_EXTENDED_2020: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 16 * 3600 + 30 * 60,
    close_ssm: 16 * 3600 + 35 * 60,
}];
static SGX_EQUITY_INDEX_CHINA_ORDER_ENTRY_2020: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 45 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 50 * 60,
        close_ssm: 17 * 3600,
    },
];
pub(super) static SGX_EQUITY_INDEX_CHINA_SOURCED_WINDOW: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_CHINA_REGULAR_2020,
    extended: SGX_EQUITY_INDEX_CHINA_EXTENDED_2020,
    order_entry: SGX_EQUITY_INDEX_CHINA_ORDER_ENTRY_2020,
    has_daily_close: true,
    has_weekend_close: true,
};

static SGX_EQUITY_INDEX_SINGAPORE_REGULAR_2020: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 20 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 50 * 60,
        close_ssm: 5 * 3600 + 15 * 60,
    },
];
static SGX_EQUITY_INDEX_SINGAPORE_EXTENDED_2020: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 17 * 3600 + 20 * 60,
    close_ssm: 17 * 3600 + 25 * 60,
}];
static SGX_EQUITY_INDEX_SINGAPORE_ORDER_ENTRY_2020: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 15 * 60,
        close_ssm: 8 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 40 * 60,
        close_ssm: 17 * 3600 + 50 * 60,
    },
];
pub(super) static SGX_EQUITY_INDEX_SINGAPORE_SOURCED_WINDOW: StaticHoursProfile =
    StaticHoursProfile {
        tz: Asia::Singapore,
        regular: SGX_EQUITY_INDEX_SINGAPORE_REGULAR_2020,
        extended: SGX_EQUITY_INDEX_SINGAPORE_EXTENDED_2020,
        order_entry: SGX_EQUITY_INDEX_SINGAPORE_ORDER_ENTRY_2020,
        has_daily_close: true,
        has_weekend_close: true,
    };
