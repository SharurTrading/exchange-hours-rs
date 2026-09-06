// SPDX-License-Identifier: MIT-0

//! The pre-2020 eras of the SGX Japan, China and Singapore equity-index
//! grids, and the dated 2024 Japan era.
//!
//! Every profile here is a session-bound grid SGX itself published, read from
//! the retired portal's archived pages, SGX's own product pages and content
//! API, the calendar editions that are still served, or a circular. The
//! narrative that orders these states, the law each row rests on and the
//! residual risks are in the sibling `history` module; this file holds the
//! tables so that module stays readable.

use chrono_tz::Asia;

use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::StaticHoursProfile;

/// A Monday-Friday rule from `(open_h, open_m, close_h, close_m)`; a close
/// earlier than its open wraps into the next local day.
macro_rules! rules {
    ($( ($oh:expr, $om:expr, $ch:expr, $cm:expr) ),* $(,)?) => {
        &[ $( SessionRule {
            days: MON_FRI,
            open_ssm: $oh * 3600 + $om * 60,
            close_ssm: $ch * 3600 + $cm * 60,
        } ),* ]
    };
}

macro_rules! era {
    ($name:ident, regular: [$($r:tt),* $(,)?], extended: [$($e:tt),* $(,)?], order_entry: [$($o:tt),* $(,)?]) => {
        pub(super) static $name: StaticHoursProfile = StaticHoursProfile {
            tz: Asia::Singapore,
            regular: rules![$($r),*],
            extended: rules![$($e),*],
            order_entry: rules![$($o),*],
            has_daily_close: true,
            has_weekend_close: true,
        };
    };
}

// --- Japan -------------------------------------------------------------------

// THE FLOOR: the intersection of the two oldest states SGX published for the
// NK grid this key models. SGX's own pre-portal NK specification page,
// captured 2009-03-08: "T
// Session: Pre -Opening 7.30am -7.43 am / Non -Cancel Period 7.43am -7.45 am
// / Opening 7.45 am - 2.25 pm / Pre-Closing 2.25 pm- 2.29 pm / Non-Cancel
// Period 2. 29 pm - 2.30 pm // T+1 Session: Pre -Opening 3.15 pm - 3.28 pm /
// Non -Cancel Period 3.28 pm - 3.30pm / Opening 3.30 pm - 10.55 pm". The
// portal's Trading Hours table captured 2013-08-20: "SGX Nikkei 225 Index
// Futures / SGX USD Nikkei 225 Index Futures | 7.45am to 2.25pm | 3.15pm to
// 2.00am", routines excluded by its footnote. The T session bounds are
// identical in both, and the closing routine is NK's alone in 2009 - SGX's
// separate NU page of 2009-02-27 prints "Opening 7.45 am - 2.30 pm" with no
// pre-closing block, the divergence `history` records as residual risk (4);
// the 2013 table is the first of the two that puts NK and NU on one row. The
// T+1 leg moved (open 15:30 -> 15:15,
// close 22:55 -> 02:00) on days no SGX artifact states, so from the
// January-2010 floor to the 2013 witness the key serves what both hold: T+1
// 15:30-22:55. The T pre-open queue 07:30-07:45 is stated in 2009 on the same
// 07:45 open the floor serves and carried with it; 15:15-15:30 is a queue in
// 2009 and matching in 2013, so order entry is the phase both support. This
// is the timeline baseline: it asserts no revision row, and serves below the
// floor too. Residual risk: third-party press attests 22:55 -> 01:00 on
// 2010-01-11 and 01:00 -> 02:00 on 2010-08-30, inside this interval and
// inadmissible for a row; the intersection under-reports the T+1 leg after
// those days and never over-reports it.
// https://web.archive.org/web/20090308012135id_/http://sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_Nikkei_225_Index.shtml
// https://web.archive.org/web/20130820090335id_/http://www.sgx.com/wps/portal/sgxweb/home/trading/derivatives/trading_hours_calendar
era!(SGX_JAPAN_FLOOR,
    regular: [(7, 45, 14, 25), (15, 30, 22, 55)],
    extended: [(14, 25, 14, 30)],
    order_entry: [(7, 30, 7, 45), (15, 15, 15, 30)]);

// 2013-08-26. The 2013-08-20 table's state - T+1 15:15-02:00 - keyed to the
// following Monday because it creates a wrapping overnight close - the
// floor's T+1 leg closes 22:55 on its own local day - per the convention
// `AGENTS.md` records ("lengthens or creates"): a capture is a knowledge
// boundary, never a cutover, and a mid-week key would report Monday's leg
// running past the 22:55 it opened under. The T queue is carried on its unchanged 07:45
// anchor; the T+1 queue is not, because its anchor moved and the 2013 page
// states no length (#65).
era!(SGX_JAPAN_FROM_2013_08_26,
    regular: [(7, 45, 14, 25), (15, 15, 2, 0)],
    extended: [(14, 25, 14, 30)],
    order_entry: [(7, 30, 7, 45)]);

// 2017-07-10. The portal's Trading Hours table captured 2017-07-05 (08:02 SGT,
// before that day's open) and again 2017-09-27, byte-identical: "SGX Nikkei
// 225 Index Futures | 7.30 am to 2.25 pm | 2.55 pm to 4.45 am". SGX's
// content API states the routines on the same grid (capture 2019-02-04:
// "Pre -Opening : 7.15 am - 7.28 am / Non -Cancel : 7.28 am - 7.30 am /
// Opening : 7.30 am - 2.25 pm / Pre-Closing : 2.25 pm - 2.29 pm / Non-Cancel :
// 2.29 pm - 2.30 pm // Pre -Opening : 2.45 pm - 2.53 pm / Non -Cancel : 2.53 pm
// - 2.55 pm / Opening : 2.55 pm - 4.45 am"). A knowledge boundary keyed to the
// Monday after the capture, not a cutover: see `history`.
// https://web.archive.org/web/20170705000242id_/http://sgx.com/wps/portal/sgxweb_ch/home/trading/derivatives/trading_hours_calendar
// https://web.archive.org/web/20170927124017id_/http://www.sgx.com/wps/portal/sgxweb/home/trading/derivatives/trading_hours_calendar
era!(SGX_JAPAN_FROM_2017_07_10,
    regular: [(7, 30, 14, 25), (14, 55, 4, 45)],
    extended: [(14, 25, 14, 30)],
    order_entry: [(7, 15, 7, 30), (14, 45, 14, 55)]);

// 2024-11-04. SGX-DT Circular No. DT/AM – 50 of 2024, "Extension of T-session
// for SGX Japan Derivatives and Intraday Margin Cycle 2 Timing Change", 9
// September 2024, signed Leno Lee, SVP Trading and Clearing Services, on
// Singapore Exchange Derivatives Trading Limited letterhead: "with effect from
// Monday, 4 November 2024, the T session trading hours for SGX Nikkei
// derivatives and SGX FTSE Blossom Japan Index Futures will be extended by 30
// minutes". Its "Revised Trading Hours" column: "Pre-Opening: 7.15 am – 7.28
// am / Non-Cancel: 7.28 am – 7.30 am / Opening: 7.30 am – 2.55 pm /
// Pre-Closing: 2.55 pm – 2.59 pm / Non-Cancel: 2.59 pm – 3.00 pm //
// Pre-Opening: 3.15 pm – 3.23 pm / Non-Cancel: 3.23 pm – 3.25 pm / Opening:
// 3.25 pm – 5.15 am". Read from the verbatim member-hosted copy (Fubon
// Futures, via the web archive; the file carries SGX's own document metadata,
// created and last saved 2024-09-09 17:53 Singapore time), as SGX publishes no
// circular at a publicly reachable address - see `history`.
// https://web.archive.org/web/20241114183232id_/https://www.fubon.com/futures/wcm/home/bulletin/bulletin_20240912_137396/SGXChange.pdf
era!(SGX_JAPAN_FROM_2024_11_04,
    regular: [(7, 30, 14, 55), (15, 25, 5, 15)],
    extended: [(14, 55, 15, 0)],
    order_entry: [(7, 15, 7, 30), (15, 15, 15, 25)]);

// --- China -------------------------------------------------------------------

// THE FLOOR: the intersection of three sourced states. SGX's pre-portal A50
// specification page, captured 2009-02-27: "T session Pre -Opening 9.00am
// -9.13 am / Non -Cancel Period 9.13 am -9.15 am / Opening 9.15am -11.35am /
// 1.00pm - 3.05pm // T+1 session Pre -Opening 3.35 pm - 3.38 pm / Non -Cancel
// Period 3.38 pm - 3.40 pm / Opening 3.40pm - 10.55pm". The ~2012 fragment:
// "FTSE Xinhua China A50 | 9.00 am - 3.25 pm | 4.10 pm - 2.00 am". The
// 2013-08-20 table: "SGX FTSE China A50 Index Futures | 9.00am to 3.55pm |
// 4.40pm to 2.00am" (its holiday tables still print 3.25 pm, ordering the
// fragment before it). Every changeover is undated, so the floor serves what
// all three hold: the T session without the 2009 lunch break's 11:35-13:00
// and only to 15:05, the T+1 open held at 17:00 - the narrowest value sourced
// anywhere in the undated span (15:40, 16:10, 16:40, 17:00), because a
// knowledge boundary may widen but never narrow - and the T+1 close 22:55.
// 09:00-09:15 is a queue in 2009 and matching later, so order entry is the
// phase all support; the 15:25-15:30 closing routine is closed in 2009 and
// withheld.
// https://web.archive.org/web/20090227040521id_/http://www.sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_FTSE_Xinhua_China_A50_Index.shtml
// https://web.archive.org/web/20180711020353id_/http://www.sgx.com/wps/wcm/connect/mp_en/site/trading_on_sgx/derivatives_market/derivatives_trading_hours_and_calendar/Trading+Hours?%20noCache=1531274630984.837727.133108399
era!(SGX_CHINA_FLOOR,
    regular: [(9, 15, 11, 35), (13, 0, 15, 5), (17, 0, 22, 55)],
    extended: [],
    order_entry: [(9, 0, 9, 15)]);

// 2013-08-26. The 2013-08-20 table's state, with the routines its
// specification captured the same day states ("Pre - Opening 8.45 am - 8.58
// am / Non - Cancel 8.58 am - 9.00 am / Opening 9.00 am - 3.55 pm / Pre -
// Closing 3.55 pm - 3.59 pm / Non - Cancel 3.59 pm - 4.00 pm"), keyed to the
// following Monday because it creates a wrapping overnight close (the
// floor's T+1 leg ends 22:55 the same day; this row runs it to 02:00) as well
// as widening the T close; the T+1 open stays held at 17:00, so the leaf's
// 16:30-16:40 queue, anchored to a 16:40 open this row does not serve, is
// withheld.
era!(SGX_CHINA_FROM_2013_08_26,
    regular: [(9, 0, 15, 55), (17, 0, 2, 0)],
    extended: [(15, 55, 16, 0)],
    order_entry: [(8, 45, 9, 0)]);

// 2017-07-10. Portal table, captures 2017-07-05 and 2017-09-27: "FTSE China
// A50 Index Futures | 9:00 am to 4:30 pm | 5.00 pm to 4.45 am". Routines from
// the content API (capture 2019-01-16, chinaa50 page: "Pre - Opening: 8.45 am
// - 8.58 am / Non - Cancel: 8.58 am - 9.00 am / Opening: 9.00 am - 4.30 pm /
// Pre - Closing: 4.30 pm - 4.34 pm / Non - Cancel: 4.34 pm - 4.35 pm // Pre -
// Opening: 4.50 pm - 4.58 pm / Non - Cancel: 4.58 pm - 5.00 pm / Opening: 5.00
// pm - 4.45 am").
era!(SGX_CHINA_FROM_2017_07_10,
    regular: [(9, 0, 16, 30), (17, 0, 4, 45)],
    extended: [(16, 30, 16, 35)],
    order_entry: [(8, 45, 9, 0), (16, 50, 17, 0)]);

// --- Singapore ---------------------------------------------------------------

// THE FLOOR: the intersection of SGX's 2009 specification pages and the
// 2013-08-20 table. The SiMSCI page captured 2009-03-08 (the STI page of
// 2009-02-20 is identical): "T Session: Pre -Opening 8.15am -8.28 am / Non
// -Cancel Period 8.28am -8.30 am / Opening 8.30am-5.10 pm / Pre-Closing 5.10
// pm-5.14 pm / Non-Cancel Period 5.14 pm- 5.15 pm // T+1 Session: Pre
// -Opening 6.00 pm – 6.13 pm / Non -Cancel Period 6.13 pm – 6.15pm / Opening
// 6.15 pm - 10.55pm". The 2013 table: "SGX MSCI Singapore Index Futures / SGX
// Straits Times Index Futures | 8.30am to 5.10pm | 6.15pm to 2.00am". Only the
// T+1 close moved (22:55 -> 02:00), undated, so the floor serves T+1
// 18:15-22:55; both queues are stated in 2009 on opens that did not move
// (the T queue 08:15-08:30 is, to the minute, what SGX still prints in 2020)
// and are carried with them. The ~2012 fragment prints the same bounds for
// MSCI Singapore and a 07:55 open for the Straits Times future.
// https://web.archive.org/web/20090308120909id_/http://www.sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_MSCI_Singapore_Index.shtml
// https://web.archive.org/web/20090220005028id_/http://sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_Straits_Times_Index.shtml
era!(SGX_SINGAPORE_FLOOR,
    regular: [(8, 30, 17, 10), (18, 15, 22, 55)],
    extended: [(17, 10, 17, 15)],
    order_entry: [(8, 15, 8, 30), (18, 0, 18, 15)]);

// 2013-08-26. The 2013-08-20 table's state - T+1 to 02:00 - keyed to the
// following Monday because it creates a wrapping overnight close (22:55 the
// same day to 02:00 the next).
era!(SGX_SINGAPORE_FROM_2013_08_26,
    regular: [(8, 30, 17, 10), (18, 15, 2, 0)],
    extended: [(17, 10, 17, 15)],
    order_entry: [(8, 15, 8, 30), (18, 0, 18, 15)]);

// 2017-07-10. Portal table, captures 2017-07-05 and 2017-09-27: "SGX MSCI
// Singapore Index Futures | 8.30am to 5.10 pm | 5.40 pm to 4.45 am". Routines
// from the content API (capture 2019-02-04: "Pre - Opening: 8:15 am - 8:28 am
// / Non - Cancel: 8:28 am - 8:30 am / Opening: 8:30 am - 5:10 pm / Pre -
// Closing: 5:10 pm - 5:14 pm / Non - Cancel: 5:14 pm - 5:15 pm // Pre -
// Opening: 5:30 pm - 5:38 pm / Non - Cancel: 5:38 pm - 5:40 pm / Opening: 5:40
// pm - 4:45 am").
era!(SGX_SINGAPORE_FROM_2017_07_10,
    regular: [(8, 30, 17, 10), (17, 40, 4, 45)],
    extended: [(17, 10, 17, 15)],
    order_entry: [(8, 15, 8, 30), (17, 30, 17, 40)]);

// 2019-06-10. SGX's Derivatives Products Description change log, entry
// issued 2019-05-21 (v6.1): "Amended trading hours for SGP, SGPO and ST eff 10
// Jun" - a single cell, no scoping step; the year is the entry's own. The
// grid it moved to is the one the content API prints on 2019-06-11, the next
// day (revisit 2019-06-21): "Pre - Opening: 8:15 am - 8:28 am / Non - Cancel:
// 8:28 am - 8:30 am / Opening: 8:30 am - 5:20 pm / Pre - Closing: 5:20 pm -
// 5:24 pm / Non - Cancel: 5:24 pm - 5:25 pm // Pre - Opening: 5:40 pm - 5:48
// pm / Non - Cancel: 5:48 pm - 5:50 pm / Opening: 5:50 pm - 4:45 am", and the
// state before it is the one the API printed on 2019-02-04 (17:10 / 17:40).
era!(SGX_SINGAPORE_FROM_2019_06_10,
    regular: [(8, 30, 17, 20), (17, 50, 4, 45)],
    extended: [(17, 20, 17, 25)],
    order_entry: [(8, 15, 8, 30), (17, 40, 17, 50)]);
