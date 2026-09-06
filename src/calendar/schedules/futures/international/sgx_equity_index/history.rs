// SPDX-License-Identifier: MIT-0

//! Sourced history for the SGX equity-index families.
//!
//! The current-grid tables and the dated revision rows live in the parent
//! module and the floor and pre-2020 tables in the sibling `eras` module; this module
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
// ALL FOUR MOVES INSIDE THE MODELLED WINDOW ARE DATED - TWO BY CIRCULARS, TWO BY
// SGX'S OWN CHANGE LOG.
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
// 2019-06-10 AND 2019-11-11. SGX's "Derivatives Products Description" workbook,
// served from api2.sgx.com, carries a dated, SGX-authored change log (sheet
// read_me, header "Issue Date (mm/dd/yyyy) | Version Number | Authors | Change
// Description", 167 dated entries from 2016-02-19, zero merged cells,
// multi-row entries partitioned by border styles in the file). Its entry
// issued 2019-05-21, v6.1, is one cell: "Amended trading hours for SGP, SGPO
// and ST eff 10 Jun". Its entry issued 2019-10-07, v6.9, is five rows (r86 to
// r90): one editorial item, then "Effective 11 Nov:" / "Editorial change
// Contracts_mainmenu:" / "(T+1) session Closing hours to 5:15am all T+1
// traded contracts" / "Editorial change session_mainmenu: SURV_INT to 5:15,
// REMOVE_DAY_ORDERS to 5:20, ... CLOSE to 5:30 all T+1 traded contracts",
// the header scoping the three rows under it.
// Its later entries "(eff 4 Nov)" (issued 2024-11-04) and "(eff 7 Apr)"
// (issued 2025-03-19) match DT/AM 50 and DT/AM 15 to the day. The log is
// admitted as a primary source for those two effective days under the
// convention `AGENTS.md` records: operator-authored, dated in the file,
// session language, calibrated. The one interpretive step is on v6.9, where
// "Effective 11 Nov:" is a header row scoping the items below it inside one
// file-encoded entry - SGX's own recurring convention in that column: of its
// six "Effective <day>:" headers, four share the items' cell (rows 60, 61, 62
// and 79) and two stand on their own row (83 and 87). A bare day-and-month
// means the occurrence of that day nearest the entry's issue date: every one
// of the sheet's 195 bare day-and-month tokens resolves that way to within
// eleven weeks of its issue date (-77 to +60 days), 193 of them in the issue
// year, and the two that cross are both in the entry issued 2024-12-31 (r166,
// "eff 6 Jan", "eff 20 Jan" - 2025). SGX spelled the year at the 2018/19 and
// 2019/20 turns ("eff 22 Dec 18" and "eff 21 Jan 19" issued 2018-12-18, r68;
// "eff 13 Jan 2020" issued 2019-12-23, r94) but not at 2024/25, so the
// warrant is the clustering, not a spelling habit. Both
// days sit inside SGX-artifact brackets: the SiMSCI move between
// the content API's 2019-02-04 payload (17:10 / 17:40) and its 2019-06-11
// payload (17:20 / 17:50), the day after the stated 10 June; the 05:15 close
// between the content API's 2019-06-21 payload (04:45) and SGX factsheets in
// the api2 2019-11 and 2019-12 directories (05:15), with SGX's Annual Report
// 2020 saying "Our extension of trading hours in November 2019". Both stated
// days are Mondays, which is why neither needs the rounding applied below.
// The channel was ruled out on 2026-09-05 as "bound only by cell-border
// formatting"; parsing the OOXML showed the partition is the document's own
// structure, and the ruling was reversed on 2026-09-06 (#45).
//
// BEFORE THE 2020 EDITION: THE STATES SGX PUBLISHED. All Singapore time, T and
// T+1 session bounds; routines excluded from the portal tables per their own
// footnote, stated on the 2009 pages.
//
//   P   2009-02/03 SGX's pre-portal psv contract-specification pages
//               (sgx.com/psv/derivatives/futures_options/equity_index/
//               *.shtml, archived 2006 to 2009-03-27 and never again): Nikkei
//               07:45-14:25 / 15:30-22:55; A50 09:15-11:35, 13:00-15:05 /
//               15:40-22:55; MSCI S'pore and STI 08:30-17:10 / 18:15-22:55;
//               MSCI Taiwan 08:45-13:45 / 14:45-22:55. Each page states both
//               pre-opening routines - the T one as 13 minutes plus a
//               2-minute non-cancel on every page (Nikkei "Pre -Opening
//               7.30am -7.43 am / Non -Cancel Period 7.43am -7.45 am"; SiMSCI
//               "Pre -Opening 8.15am -8.28 am / Non -Cancel Period 8.28am
//               -8.30 am"; A50 "Pre -Opening 9.00am -9.13 am"), the T+1 one
//               likewise (SiMSCI "Pre -Opening 6.00 pm – 6.13 pm / Non -Cancel
//               Period 6.13 pm – 6.15pm") except the A50's ("Pre -Opening
//               3.35 pm - 3.38 pm / Non -Cancel Period 3.38 pm - 3.40 pm") -
//               and every page but the A50's states the same 4+1 pre-closing
//               routine the later pages print (Nikkei "Pre-Closing 2.25 pm-
//               2.29 pm / Non-Cancel Period 2. 29 pm - 2.30 pm"; SiMSCI
//               "Pre-Closing 5.10 pm-5.14 pm / Non-Cancel Period 5.14 pm-
//               5.15 pm"; MSCI Taiwan "Pre-Closing 1.45 pm - 1.49 pm / Non
//               -Cancel Period 1. 49pm - 1.50 pm"). The A50 page prints none
//               - its T block ends "1.00pm - 3.05pm" and "T+1 session" begins
//               - which is why `SGX_CHINA_FLOOR` serves no extended phase.
//               Below the January-2010 floor.
//   W   ~2012   Nikkei 07:45-14:25 / 15:15-02:00; A50 09:00-15:25 / 16:10-02:00;
//               MSCI S'pore 08:30-17:10 / 18:15-02:00; MSCI Taiwan 08:45-13:45 /
//               14:35-02:00. An orphaned wcm/connect fragment captured
//               2018-07-11 whose contract set ("S&P CNX Nifty", "FTSE Xinhua",
//               "MSCI Asia Apex 50") dates its content to about 2012. Not
//               uniformly older than P row by row: its Straits Times open
//               (07:55) and its A50 grid without a lunch break both post-date
//               February 2009, when SGX's own pages print 08:30 and the break.
//   S0  2013-08-20 portal Trading Hours table: as W except A50 09:00-15:55 /
//               16:40-02:00. The same page's holiday tables still print the
//               A50 at 15:25, which orders W before S0. Its Notes assert a
//               pre-opening routine and non-cancel period before both
//               sessions of every equity-index future and state no length,
//               referring to specification leaves the archive holds only for
//               the A50 (captured the same day: 08:45-08:58 / 08:58-09:00,
//               16:30-16:38 / 16:38-16:40). Witnessed unchanged as of
//               2015-12-24 by SGX's Derivatives Trading Calendar 2016 - a
//               verbatim SGX PDF (SGX cover and imprint, created 2016-08-31)
//               served by KGI Futures, an SGX-DT member, that prints the S0
//               grid for every family and states "All dates and information
//               are accurate as of 24 December 2015". Read at its as-of
//               date, never its creation date: SGX regenerated a stale
//               edition eight months on.
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
// 2016, though neither states a day (#66). The lower bound is the 2016
// calendar edition's as-of date, 2015-12-24, so the S0 -> A window is
// (2015-12-24, 2017-07-05), with both change-log entries, the Titan DT/DC
// newsletter SGX released on 27 July 2016 and the Titan launch itself inside
// it. That newsletter's subject is a later label: SGX's Titan portal index
// captured 2017-06-17 lists the 27 Jul 2016 row only as "Titan DTDC
// Newsletter - New Feature Overview 2", and the title "Extended Trading
// Hours, Price Limits and Trade at Settlement" appears only in the 2018-12
// re-upload the current index serves; the file is password-locked either
// way. SGX's Futures
// Trading Rules carry thirty-six dated annotations for that system cutover
// ("Amended on 14 November 2016" and its variants), none of them Rule 4.1.5,
// which delegates hours to the contract specifications; the rulebook dates
// the system, not the grid. The
// 21 September 2017 "Change of
// Trading Hours" newsletter is not the move: captures straddle it and are
// identical, and the change log's own entry for it (issued 2017-10-05, v3.3,
// "Change of Trading Hours") states no day either.
//
// HOW THE ROWS ARE KEYED. Japan, China and Singapore have members listed with
// a grid in the oldest artifact, so their floor state is carried to the
// January-2010 floor as each key's `select_revision` baseline - it asserts no
// revision row - under the carry-back convention. That floor state is the
// intersection of P and S0, not S0 alone: the P pages are below the floor,
// but the changes they bound (the T+1 close 22:55 -> 02:00 on every key, the
// Nikkei T+1 open 15:30 -> 15:15, the A50's lunch break and its 15:05 close)
// fall inside the audit window on days no SGX artifact states, so from the
// floor to the 2013 witness each key serves what both states hold - T+1 to
// 22:55; Nikkei T+1 from 15:30 with 15:15-15:30 as order entry, the phase P
// (a queue) and S0 (open) both support; the A50 without its lunch break, to
// 15:05, with 09:00-09:15 as order entry for the same reason and its T+1 open
// held at 17:00 (below). The 2009 pre-open queues are served where their
// anchor did not move (Nikkei T 07:30-07:45; SiMSCI 08:15-08:30 and
// 18:00-18:15) and withheld where it did (A50 T+1, whose 15:35-15:40 sits
// inside the floor's closed gap between the 15:05 T close and the 17:00 T+1
// open); the Nikkei's T+1 queue 15:15-15:30 and the A50's T queue
// 09:00-09:15 each coincide with an order-entry window the intersection
// already serves, so the carrying rule decides nothing there. The S0 state
// then arrives as a dated row on all three keys, keyed to Monday 2013-08-26
// rather than the Tuesday capture because it creates a wrapping overnight
// close - the floor's T+1 leg ends 22:55 on its own local day; this row runs
// it to 02:00 - and `AGENTS.md` routes a row that lengthens or creates one to
// the Monday. Its T+1 queues part three ways. SiMSCI's 18:00-18:15 carries:
// its 18:15 anchor did not move. Japan's is withheld because the portal
// table states no length, the archive holds no Nikkei leaf of that day, and
// the anchor moved 15:30 -> 15:15. The A50's is withheld although the leaf
// captured the same day states it exactly (16:30-16:38 / 16:38-16:40, the
// leaf that sources the 08:45-09:00 queue this row does serve): it anchors
// to the 16:40 open the row does not serve, the T+1 open being held at 17:00
// under the widen-only rule, and granting it would have to be withdrawn at
// the 2017-07-10 boundary, whose queue is 16:50-17:00.
// FTSE Taiwan and the NTR (USD) suite did not exist then and stay sessionless
// below their own first listing. The State-A boundary is a knowledge boundary, keyed to Monday
// 2017-07-10 rather than the Wednesday capture: a boundary that lengthens a
// wrapping overnight close, keyed mid-week, would report the previous
// evening's leg running past the close in force when it opened - the running
// session LAW-NO-FABRICATED-DATES says a boundary must not split - and no leg
// wraps into a Monday morning. The NTR boundary moves to Monday 2018-04-16
// for the same reason; the 2019-11-11 rows, which lengthen the close from
// 04:45 to 05:15, need no rounding because the day SGX states is a Monday, as
// is 2019-06-10. A knowledge boundary may widen what is served and never
// narrow it: the A50's T+1 open is sourced at 15:40, 16:10, 16:40 and 17:00
// across four undated states, so 17:00 is held from the floor and only its T
// close, which widens, moves at the boundaries.
//
// ROUTINES. The calendars print session bounds only, but SGX's content API
// prints every family's Pre-Opening/Non-Cancel/Pre-Closing windows, and its
// 2020-01-09 payload states them inside the 2019-11-11 row's own interval - so
// those rows carry them on all five keys, as do the earlier eras those
// payloads, the 2013 A50 leaf and the 2009 pages source. They are sourced, not
// carried, through the interval to 2025: all 22 archived payloads of 2021 to
// 2024 (at least one per half-year, 2021-01-05 to 2024-10-07, across
// rotating query hashes) reproduce the 2020-01-09 strings for every key to
// the minute, and the edits SGX made to the catalogue in that span (the US
// single-stock futures' T+1 session of 2023-11-27; the removal of the Nifty
// rows) touch no equity-index row.
// DT/AM 50 states Japan's routines on both sides of 2024-11-04, and SGX's own
// server-rendered product pages corroborate it routine by routine across the
// content API's 2024-10-07 to 2025-04-07 capture gap: nikkei225futuresoptions
// ?cc=NK at 2024-10-07 still prints "Opening : 7.30 am - 2.25 pm ... Opening
// : 2.55 pm - 5.15 am" and ?cc=NU at 2024-11-14 prints the circular's Revised
// column ("Opening : 7.30 am - 2.55 pm / Pre-Closing : 2.55 pm - 2.59 pm /
// Non-Cancel : 2.59 pm - 3.00 pm / ... Pre-Opening : 3.15 pm - 3.23 pm /
// Non-Cancel : 3.23 pm - 3.25 pm / Opening : 3.25 pm - 5.15 am"), while the
// China, Singapore, Taiwan and NTR pages are identical either side of the day.
// DT/AM 15 states every family's routines from 2025-04-07, and the content
// API's 2025-04-07 payload prints them.
//
// RESIDUAL RISKS, STATED. (1) The T+1 close moved from 22:55 to 02:00
// between the P pages and the 2013 table on days SGX does not state;
// third-party press puts 22:55 -> 01:00 on 2010-01-11 and 01:00 -> 02:00 on
// 2010-08-30, both inside the floor interval and inadmissible for a row. The
// floor rows serve 22:55 to 2013-08-25, so they under-report the T+1 leg by
// up to three hours across that interval and never over-report it - the side
// the intersection convention chooses. (2) The 2014, 2015 and later-2016
// calendar editions lived in a WCM store that is in neither the archive nor
// the live site; the August-2016 edition survives on a member mirror and
// witnesses S0 as of 2015-12-24, so only 2016 to mid-2017 is unwitnessed and
// the intersection covers the sourced endpoints only. Member notices,
// inadmissible for a row, date two events inside that window: the Nikkei
// suite's T open 07:45 -> 07:30 "with effect from 11 July 2016" (Phillip
// Futures, 1 July 2016) and the market-wide hours revision "with effect from
// 14 November 2016", the Titan DT/DC launch (KGI Futures and Phillip Futures,
// November 2016), citing SGX Circular DT/AM 80 of 2016, Appendix 1, whose
// SGX copy and mirror are both dead and unarchived. Were that appendix
// recovered, the three 2017-07-10 boundaries would become a cutover keyed to
// Monday 2016-11-14 and the keys would stop under-reporting about eight
// months of the longer T+1 leg; until then the day is a risk, not a row.
// (3) NKO's T
// close is five minutes later than NK's in every era and is not modelled, as
// FCHO is not on the China grid. (4) The Mini Nikkei (NS) and Dividend Point
// (ND) contracts ran their own later closes in P (NU and NS to 14:30, with
// no pre-closing block), W, S0 and - for ND - A before joining the NK grid
// (NS by 2017-07-05, ND by the 2018 edition), and the Straits Times Index
// future opened 07:55 rather than 08:30 in W before sharing the SiMSCI row
// from S0 on; each family's clock is its index future's grid throughout.
//
// CHANNELS. SGX publishes no DT/AM circular at a publicly reachable sgx.com
// address (regco.sgx.com's /circulars route answers `null`; the api2 file
// store is not listable), so both circulars above were read from verbatim
// member-hosted copies - SGX letterhead, circular number, signatory, both
// appendices - CITIC Futures International for DT/AM 15 and Fubon Futures
// (via the web archive) for DT/AM 50, and KGI Futures (Singapore) for the
// 2016 calendar edition and, verified on DT/AM 103 of 2020, for circulars;
// SGX's own Titan DT/DC portal corroborates each issue date. The retired portal's pages and SGX's pre-portal psv pages
// are archived; the 2018 and 2019 calendars, the content API's payloads and
// the server-rendered product pages are SGX-served.
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
// https://web.archive.org/web/20090308012135id_/http://sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_Nikkei_225_Index.shtml
// https://web.archive.org/web/20090227040521id_/http://www.sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_FTSE_Xinhua_China_A50_Index.shtml
// https://web.archive.org/web/20090308120909id_/http://www.sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_MSCI_Singapore_Index.shtml
// https://web.archive.org/web/20090220005028id_/http://sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_Straits_Times_Index.shtml
// https://web.archive.org/web/20130820090335id_/http://www.sgx.com/wps/portal/sgxweb/home/trading/derivatives/trading_hours_calendar
// https://web.archive.org/web/20170705000242id_/http://sgx.com/wps/portal/sgxweb_ch/home/trading/derivatives/trading_hours_calendar
// https://web.archive.org/web/20170927124017id_/http://www.sgx.com/wps/portal/sgxweb/home/trading/derivatives/trading_hours_calendar
// https://web.archive.org/web/20190116144725id_/https://api2.sgx.com/content-api?queryId=e8c4b75927723d2bae18ec762abab178e0efcd9a%3Apage&variables=%7B%22path%22%3A%22%2Fderivatives%2Fproducts%2Fchinaa50%22%2C%22lang%22%3A%22EN%22%7D
// https://web.archive.org/web/20190204200905id_/https://api2.sgx.com/content-api?queryId=9756cc24703868bca7da492a8e1aebd1268eaf70%3Aderivatives_products_list&variables=%7B%22limit%22%3A10000%2C%22lang%22%3A%22EN%22%7D
// https://web.archive.org/web/20190611051800id_/https://api2.sgx.com/content-api?queryId=5adaa923edc3b334f3d4a62a324e055c4be65025%3Aderivatives_products_list&variables=%7B%22limit%22%3A10000%2C%22lang%22%3A%22EN%22%7D
// https://web.archive.org/web/20200109051211id_/https://api2.sgx.com/content-api?queryId=ef44c5f861fc84577240761863bf1f842f189d9f%3Aderivatives_products_list&variables=%7B%22limit%22%3A10000%2C%22lang%22%3A%22EN%22%7D
// https://web.archive.org/web/20241007180652id_/https://www.sgx.com/derivatives/products/nikkei225futuresoptions?cc=NK
// https://web.archive.org/web/20241114152443id_/https://www.sgx.com/derivatives/products/nikkei225futuresoptions?cc=NU
// https://web.archive.org/web/20241114183232id_/https://www.fubon.com/futures/wcm/home/bulletin/bulletin_20240912_137396/SGXChange.pdf
// https://www.citicsf.com.hk/attachment?aid=95&uid=a1207308-0e3a-4a16-a869-a4d1b808a2b3
// https://www.kgieworld.sg/docs/SGXDerivativesTradingCalendar2016_AUG.pdf
// https://rulebook.sgx.com/rulebook/futures-trading-rules
// https://www.sgx.com/titan-dt-dc-portal
// https://api2.sgx.com/sites/default/files/2026-08/Derivatives+Products+Description+v17.6%20eff%2020260824,%2020260907.zip

// THE 2019-11-11 ROWS. The 05:15 T+1 close from the day SGX's change log
// states; session bounds confirmed by the 2020 edition and routines from the
// content API's 2020-01-09 payload, which states them for every family on this
// grid: Japan "Pre -Opening : 7.15 am - 7.28 am / Non -Cancel : 7.28 am -
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
static SGX_EQUITY_INDEX_JAPAN_REGULAR_FROM_2019_11_11: &[SessionRule] = &[
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
static SGX_EQUITY_INDEX_JAPAN_EXTENDED_FROM_2019_11_11: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 14 * 3600 + 25 * 60,
    close_ssm: 14 * 3600 + 30 * 60,
}];
static SGX_EQUITY_INDEX_JAPAN_ORDER_ENTRY_FROM_2019_11_11: &[SessionRule] = &[
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
    regular: SGX_EQUITY_INDEX_JAPAN_REGULAR_FROM_2019_11_11,
    extended: SGX_EQUITY_INDEX_JAPAN_EXTENDED_FROM_2019_11_11,
    order_entry: SGX_EQUITY_INDEX_JAPAN_ORDER_ENTRY_FROM_2019_11_11,
    has_daily_close: true,
    has_weekend_close: true,
};

static SGX_EQUITY_INDEX_CHINA_REGULAR_FROM_2019_11_11: &[SessionRule] = &[
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
static SGX_EQUITY_INDEX_CHINA_EXTENDED_FROM_2019_11_11: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 16 * 3600 + 30 * 60,
    close_ssm: 16 * 3600 + 35 * 60,
}];
static SGX_EQUITY_INDEX_CHINA_ORDER_ENTRY_FROM_2019_11_11: &[SessionRule] = &[
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
    regular: SGX_EQUITY_INDEX_CHINA_REGULAR_FROM_2019_11_11,
    extended: SGX_EQUITY_INDEX_CHINA_EXTENDED_FROM_2019_11_11,
    order_entry: SGX_EQUITY_INDEX_CHINA_ORDER_ENTRY_FROM_2019_11_11,
    has_daily_close: true,
    has_weekend_close: true,
};

static SGX_EQUITY_INDEX_SINGAPORE_REGULAR_FROM_2019_11_11: &[SessionRule] = &[
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
static SGX_EQUITY_INDEX_SINGAPORE_EXTENDED_FROM_2019_11_11: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 17 * 3600 + 20 * 60,
    close_ssm: 17 * 3600 + 25 * 60,
}];
static SGX_EQUITY_INDEX_SINGAPORE_ORDER_ENTRY_FROM_2019_11_11: &[SessionRule] = &[
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
        regular: SGX_EQUITY_INDEX_SINGAPORE_REGULAR_FROM_2019_11_11,
        extended: SGX_EQUITY_INDEX_SINGAPORE_EXTENDED_FROM_2019_11_11,
        order_entry: SGX_EQUITY_INDEX_SINGAPORE_ORDER_ENTRY_FROM_2019_11_11,
        has_daily_close: true,
        has_weekend_close: true,
    };
