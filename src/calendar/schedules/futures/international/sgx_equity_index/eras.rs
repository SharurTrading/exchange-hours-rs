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

// THE FLOOR. The oldest grid SGX published for NK/NU: "SGX Nikkei 225 Index
// Futures / SGX USD Nikkei 225 Index Futures | 7.45am to 2.25pm | 3.15pm to
// 2.00am" in the portal's Trading Hours table captured 2013-08-20, and the
// same times in the orphaned wcm/connect fragment whose contract set dates
// its content to ~2012. The page's Notes state a "4-minute pre-closing
// routine ... followed by a 1-minute non-cancel period at the end of the T
// session" for every equity-index future, which is the 14:25-14:30 closing
// auction; no artifact of this era that has been read states the pre-opening
// routine's length (#65), so no order-entry phase is served. Carried to the
// January-2010 floor as this key's baseline: it asserts no revision row.
// https://web.archive.org/web/20130820090335id_/http://www.sgx.com/wps/portal/sgxweb/home/trading/derivatives/trading_hours_calendar
era!(SGX_JAPAN_FLOOR,
    regular: [(7, 45, 14, 25), (15, 15, 2, 0)],
    extended: [(14, 25, 14, 30)],
    order_entry: []);

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

// THE FLOOR. The A50 is the one family whose two oldest states differ: the
// ~2012 fragment prints "FTSE Xinhua China A50 | 9.00 am - 3.25 pm | 4.10 pm
// - 2.00 am" and the 2013-08-20 table "SGX FTSE China A50 Index Futures |
// 9.00am to 3.55pm | 4.40pm to 2.00am"; the 2013-08-20 page's own holiday
// tables still print the 3.25 pm close, which fixes the order. Between them
// the changeover is undated, so the floor serves the T close both hold (15:25)
// and — because a knowledge boundary may widen but never narrow — holds the
// T+1 open at 17:00, the narrowest value sourced anywhere in the undated span
// (16:10, 16:40 and 17:00), rather than granting 16:10 here and withdrawing it
// at a capture. The A50 specification captured 2013-08-20 states the T
// pre-opening routine ("Pre - Opening 8.45 am - 8.58 am / Non - Cancel 8.58 am
// - 9.00 am"), carried with the grid, and the page's Notes state the closing
// routine for every equity-index future, so the 15:25-15:30 auction — open
// under both states this floor intersects — is served as `extended`.
// https://web.archive.org/web/20180711020353id_/http://www.sgx.com/wps/wcm/connect/mp_en/site/trading_on_sgx/derivatives_market/derivatives_trading_hours_and_calendar/Trading+Hours?%20noCache=1531274630984.837727.133108399
// https://web.archive.org/web/20130820082329id_/http://www.sgx.com/wps/portal/sgxweb/home/products/derivatives/equity/chinaa50/!ut/p/c5/04_SB8K8xLLM9MSSzPy8xBz9CP0os3gjR0cTDwNnA0t_AzMjA09Dz-CQADcvQyMfA6B8JG75YEOSdLu7G7oZeIaGhLk4hnkZu3qZEdDtpR-VnpOfBHSln0d-bqp-QW5oRKWjoiIAihb-kQ!!/dl3/d3/L0lDU0lKSWdra0EhIS9JTlJBQUlpQ2dBek15cUEhL1lCSlAxTkMxTktfMjd3ISEvN18yQUE0SDBDMDk4Mkc3MEkxNlFDMVRDMzBNMA!!/?WCM_PORTLET=PC_7_2AA4H0C0982G70I16QC1TC30M0017268_WCM&WCM_GLOBAL_CONTEXT=/wps/wcm/connect/sgx_en/home/products/derivatives/equity/chinaa50/specifications/
era!(SGX_CHINA_FLOOR,
    regular: [(9, 0, 15, 25), (17, 0, 2, 0)],
    extended: [(15, 25, 15, 30)],
    order_entry: [(8, 45, 9, 0)]);

// 2013-08-20. The T close widens to 15:55 on the portal table and the
// specification captured that day ("Opening 9.00 am - 3.55 pm / Pre - Closing
// 3.55 pm - 3.59 pm / Non - Cancel 3.59 pm - 4.00 pm"); the T+1 open stays
// held at 17:00 for the reason above.
era!(SGX_CHINA_FROM_2013_08_20,
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

// THE FLOOR. "SGX MSCI Singapore Index Futures / SGX Straits Times Index
// Futures | 8.30am to 5.10pm | 6.15pm to 2.00am" on the 2013-08-20 table; the
// ~2012 fragment prints the same bounds for MSCI Singapore and a 07:55 open
// for the Straits Times future, which shares the SiMSCI row from 2013 on.
// Closing routine from the page's Notes; no artifact of this era that has
// been read states the pre-opening routine's length (#65).
era!(SGX_SINGAPORE_FLOOR,
    regular: [(8, 30, 17, 10), (18, 15, 2, 0)],
    extended: [(17, 10, 17, 15)],
    order_entry: []);

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

// 2019-02-06 to 2019-06-10: the sourced intersection across the undated
// SiMSCI move. The content API prints "Opening: 8:30 am - 5:10 pm / ...
// Opening: 5:40 pm - 4:45 am" on 2019-02-04 (= 2019-02-05 04:09 SGT) and
// "Opening: 8:30 am - 5:20 pm / ... Opening: 5:50 pm - 4:45 am" on
// 2019-06-11. Serving what both hold — T to 17:10, T+1 from 17:50 — the
// pre-open queue moves with the later open.
era!(SGX_SINGAPORE_FROM_2019_02_06,
    regular: [(8, 30, 17, 10), (17, 50, 4, 45)],
    extended: [(17, 10, 17, 15)],
    order_entry: [(8, 15, 8, 30), (17, 40, 17, 50)]);

// 2019-06-11. Content API, capture 2019-06-11 (revisit 2019-06-21): "Pre -
// Opening: 8:15 am - 8:28 am / Non - Cancel: 8:28 am - 8:30 am / Opening: 8:30
// am - 5:20 pm / Pre - Closing: 5:20 pm - 5:24 pm / Non - Cancel: 5:24 pm -
// 5:25 pm // Pre - Opening: 5:40 pm - 5:48 pm / Non - Cancel: 5:48 pm - 5:50 pm
// / Opening: 5:50 pm - 4:45 am".
era!(SGX_SINGAPORE_FROM_2019_06_11,
    regular: [(8, 30, 17, 20), (17, 50, 4, 45)],
    extended: [(17, 20, 17, 25)],
    order_entry: [(8, 15, 8, 30), (17, 40, 17, 50)]);
