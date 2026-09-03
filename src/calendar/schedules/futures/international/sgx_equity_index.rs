// SPDX-License-Identifier: MIT-0

//! SGX equity-index derivatives: Japan, China and Singapore grids.
//!
//! SGX runs each equity-index family on its own clock rather than one venue
//! schedule, so every family below is modelled separately. All five SGX
//! equity-index grids share one shape: a daytime "T" session that opens after a
//! Pre-Opening/Non-Cancel order-entry routine and ends with a Pre-Closing /
//! Non-Cancel closing routine, then an overnight "T+1" session that reopens
//! after a short second order-entry routine and runs to 05:15 the next calendar
//! day. Continuous, executable phases are `regular`.
//!
//! The non-continuous phases split in two. The Pre-Opening/Non-Cancel routines
//! that precede the T and T+1 opens only collect orders - nothing matches until
//! the open, and the opening match falls on the session-open instant that
//! already starts a `regular` window - so they are `order_entry`. The closing
//! routine is different: it matches at a single closing price, a trade prints
//! in it, so it stays `extended`.
//!
//! The Taiwan and NTR (USD) grids live in the `sgx_equity_index_more` module.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// --- Japan (Nikkei 225 suite: NK, NS, NU, NC, NR, ND, EJP, EJRT) -------------

// Two executable phases per trade date. The T session trades continuously
// 07:30-14:55; the T+1 (night) session reopens at 15:10 and runs to 05:15 the
// following calendar day, so it is encoded as a wrapping rule. The Friday T+1
// session therefore ends Saturday 05:15 and no Sunday session exists, which is
// why both rules are Monday-Friday. The 14:55-15:00 closing routine matches at
// a single price rather than trading continuously, so it is modelled as an
// extended phase, not as part of the continuous T session.
//
// https://www.sgx.com/derivatives/products/nikkei225futuresoptions
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
pub(crate) static SGX_EQUITY_INDEX_JAPAN_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 30 * 60,
        close_ssm: 14 * 3600 + 55 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 10 * 60,
        close_ssm: 5 * 3600 + 15 * 60,
    },
];

// The closing routine, "Pre-Closing : 2.55 pm - 2.59 pm / Non-Cancel : 2.59 pm
// - 3.00 pm", merged into one 14:55-15:00 window. It matches at a single closing
// price, so a trade prints in it and it stays `extended`.
pub(crate) static SGX_EQUITY_INDEX_JAPAN_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 14 * 3600 + 55 * 60,
    close_ssm: 15 * 3600,
}];

// The two opening routines. SGX states the T routine as "Pre-Opening : 7.15 am -
// 7.28 am / Non-Cancel : 7.28 am - 7.30 am" and the T+1 routine as "Pre-Opening
// : 3.05 pm - 3.08 pm / Non-Cancel : 3.08 pm - 3.10 pm"; each contiguous pair is
// merged into one window. Both only collect orders - the opening matches land on
// the 07:30 and 15:10 session opens that already begin `regular` windows - so
// both are `order_entry`, not `extended`.
pub(crate) static SGX_EQUITY_INDEX_JAPAN_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 15 * 60,
        close_ssm: 7 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 5 * 60,
        close_ssm: 15 * 3600 + 10 * 60,
    },
];

// The current grid, in force from the effective day stated by SGX-DT Circular
// DT/AM 15 of 2025: see the revision note below.
pub(crate) static SGX_EQUITY_INDEX_JAPAN_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_JAPAN_REGULAR_CURRENT,
    extended: SGX_EQUITY_INDEX_JAPAN_EXTENDED_CURRENT,
    order_entry: SGX_EQUITY_INDEX_JAPAN_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

/// Sessionless profile for dates before the first sourced SGX calendar edition.
static SGX_EQUITY_INDEX_CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// SGX EQUITY-INDEX HISTORY. NINE PUBLISHED FILES OF SGX'S DERIVATIVES TRADING
// CALENDAR - static, readable PDFs under api2.sgx.com/sites/default/files/, of
// which eight are distinct documents - state these grids, and they disagree in
// exactly two places:
//
//   edition                    Japan T / T+1        China  SiMSCI  Taiwan  NTR
//   2020, 2021-01, 2021-07,
//   2022-06, 2024              07:30-14:25 / 14:55  17:00  17:50   14:15   19:00
//   2025-01                    07:30-14:55 / 15:25  17:00  17:50   14:15   19:00
//   2025-07 (= 2025-11), 2026  07:30-14:55 / 15:10  16:45  17:35   14:00   18:45
//
// (2025-11/DT Trading Calendar 2025.pdf is byte-identical to 2025-07/DT Trading
// Calendar 2025 (updated 31 Jul 2025).pdf, so those two files are one edition,
// sourced at 31 July 2025 rather than at November.)
//
// THE SECOND TRANSITION IS DATED: 2025-04-07. SGX-DT Circular DT/AM - 15 of
// 2025, "Revision of T+1 Session Trading Hours for SGX Equity Index
// Futures/Options, Dividend Index Futures and United States Single Stock Futures
// (US SSFs)", 24 February 2025, states that "with effect from Monday, [7] April
// 2025" the T+1 pre-open routine comes forward ten minutes and shortens from ten
// minutes to five, with "no change to the T session trading hours". Its Appendix
// A lists every affected contract's current and revised Pre-Opening/Non-Cancel/
// Opening times, and the revised column is exactly this module's current grid:
// Japan 15:05/15:08/15:10, China 16:40/16:43/16:45, Singapore 17:30/17:33/17:35,
// Taiwan 13:55/13:58/14:00, NTR 18:40/18:43/18:45. Appendix B repeats the
// unchanged T-session routines and the closing routines modelled as `extended`.
//
// DELIVERY CHANNEL, STATED PLAINLY. SGX publishes no DT/AM circular at a
// publicly reachable sgx.com address: regco.sgx.com/circulars no longer resolves
// (its CMS answers with a null route), the api2.sgx.com file store is not
// listable, and the matching member newsletter is password-locked. The circular
// text above was read from a verbatim copy - SGX letterhead, circular number,
// signatory, both appendices - published by CITIC Futures International, an SGX
// trading member. SGX's own public Titan DT/DC portal corroborates the issue
// date, listing "Titan DTDC Newsletter - Revision of T+1 Trading Hours for SGX
// Equity and Dividend Index Derivatives and US SSF" with release date 24 Feb
// 2025, and SGX's own editions bracket the change (2025-01 before, 2025-07
// after). Re-verify from an SGX-hosted copy if one becomes reachable.
//
// THE FIRST TRANSITION IS STILL UNDATED. Japan's T session lengthened by thirty
// minutes, and its T+1 open moved with it, somewhere between the 2024 and
// 2025-01 editions. SGX's Titan portal dates the newsletter that announced it -
// "SR12.5, Japan Derivatives Trading Hours Extension and I2 Timing Change",
// 28 Aug 2024 - but that document is password-locked, and no SGX artifact
// reachable without a member login states the effective day. A trading member's
// public notice (Phillip Nova, 30 Oct 2024) attests Monday 4 November 2024; a
// third party cannot date a revision row, so the interval keeps the sourced
// intersection instead and this family stays Partial.
//
// SO THE DATED SURFACE HAS TWO ERAS. From the first sourced edition through
// 2025-04-06 it serves the intersection of every state sourced in that interval
// - Japan T 07:30-14:25 and T+1 15:25-05:15, China T+1 17:00, SiMSCI 17:50,
// Taiwan 14:15, NTR 19:00 - so the undated Japan move is approached from the
// conservative side and no instant is reported open on bounds that were not in
// force. From 2025-04-07 it serves the full current grid on the circular's
// authority. Routines stay out of the first era deliberately: the calendars
// state session bounds only, and the circular's "current" column is the state
// after the undated Japan move, not before it.
//
// 2020-01-01 is a knowledge boundary, not a claimed change: no edition of the
// calendar survives before it, so it separates "sourced" from "unsourced"
// exactly as a launch day separates a venue from its pre-launch closure. Four
// of the five families start there. The FTSE Taiwan suite starts at the 2021
// edition instead, because the 2020 one contains no FTSE Taiwan contract at
// all - see the boundary note beside its revisions in `sgx_equity_index_more`. SGX's
// own newsletter index puts an hours change immediately there - "Change of
// Trading Hours" (21 Sep 2017), "Extension of T+1 Trading Hours" (15 Jul 2019),
// "Ext of T+1 Trading Hours Go Live Schedule" (8 Oct 2019) and "Reminder on
// Extension of T+1 Trading Hours" (5 Nov 2019), all password-locked. Those dates
// are modelled sessionless, matching every era this crate cannot source.
// https://api2.sgx.com/sites/default/files/2020-01/SGX%20Derivatives%20Trading%20Calendar%202020.pdf
// https://api2.sgx.com/sites/default/files/2021-01/SGX%20Derivatives%20Trading%20Calendar%202021.pdf
// https://api2.sgx.com/sites/default/files/2021-07/SGX_Derivatives%20Trading%20Calendar%202021%20%28Final%20-%20Jul%29.pdf
// https://api2.sgx.com/sites/default/files/2022-06/DT%20Trading%20Calendar%202022%20%28Final%29.pdf
// https://api2.sgx.com/sites/default/files/2024-01/SGX%20Calendar%202024.pdf
// https://api2.sgx.com/sites/default/files/2025-01/SGX%20Calendar%202025.pdf
// https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf
// https://api2.sgx.com/sites/default/files/2025-11/DT%20Trading%20Calendar%202025.pdf
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
// https://www.sgx.com/titan-dt-dc-portal
// https://www.citicsf.com.hk/attachment?aid=95&uid=a1207308-0e3a-4a16-a869-a4d1b808a2b3
static SGX_EQUITY_INDEX_JAPAN_REGULAR_SOURCED_WINDOW: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 30 * 60,
        close_ssm: 14 * 3600 + 25 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 25 * 60,
        close_ssm: 5 * 3600 + 15 * 60,
    },
];
static SGX_EQUITY_INDEX_JAPAN_SOURCED_WINDOW: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_JAPAN_REGULAR_SOURCED_WINDOW,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static SGX_EQUITY_INDEX_CHINA_REGULAR_SOURCED_WINDOW: &[SessionRule] = &[
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
static SGX_EQUITY_INDEX_CHINA_SOURCED_WINDOW: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_CHINA_REGULAR_SOURCED_WINDOW,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static SGX_EQUITY_INDEX_SINGAPORE_REGULAR_SOURCED_WINDOW: &[SessionRule] = &[
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
static SGX_EQUITY_INDEX_SINGAPORE_SOURCED_WINDOW: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_SINGAPORE_REGULAR_SOURCED_WINDOW,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// RETRIEVAL NOTE, and why these rows stay Partial. The SGX circular archive
// still cannot be read from SGX: www.sgx.com/regulation/circulars redirects to
// the regco.sgx.com single-page app, whose CMS now answers the /circulars route
// with `null`, and the api2.sgx.com file store is not listable. The Derivatives
// Trading Calendar PDFs read above are SGX's productive channel and they date
// the grids, not the transition days. What closed the 2025 transition was the
// circular itself, found through a trading member's public mirror; the 2024
// Japan transition has no equivalent, so these rows keep the Partial basis. The
// gap should be read as "the first transition day is not stated anywhere
// publicly reachable", not as "no dated evidence exists".
//
// DIRECTION OF THE ERROR. Before 2026-08-31 these rows carried today's grid to
// the January-2010 floor across the moves tabulated above, which made them the
// only rows in the crate that could **over**-report — answer open on bounds
// that differed at the time. They no longer can: the sourced-intersection era
// approaches the one remaining undated transition from the conservative side,
// the current grid begins on the circular's stated effective day, and dates
// before the 2020 edition are sessionless. Like every other Partial row in this
// crate they err toward Closed, which is the safe direction for an order router.
//
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
// https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf
pub(crate) static SGX_EQUITY_INDEX_JAPAN_REVISIONS: &[Revision] = revisions![
    (
        2020,
        1,
        1,
        &SGX_EQUITY_INDEX_JAPAN_SOURCED_WINDOW,
        "first sourced SGX calendar edition"
    ),
    (
        2025,
        4,
        7,
        &SGX_EQUITY_INDEX_JAPAN_BASELINE,
        "SGX-DT Circular DT/AM 15 of 2025"
    ),
];

/// Selects the SGX Japan equity-index profile in force on `as_of`'s Singapore day.
pub(crate) fn sgx_equity_index_japan_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Asia::Singapore),
        &SGX_EQUITY_INDEX_CLOSED,
        SGX_EQUITY_INDEX_JAPAN_REVISIONS,
    )
}

// --- China (FTSE China A50 / H50: CN, FCH, FCHO) -----------------------------

// T session trades continuously 09:00-16:30; the T+1 session reopens at 16:45
// and runs to 05:15 the next calendar day, so it wraps. SGX's own A50 page
// notes the contract "is available for trading everyday other than New Year's
// Day", but that describes holiday coverage, not a weekend session: the T+1
// leg still starts on a Monday-Friday trade date and the Friday leg ends
// Saturday 05:15, so both rules stay Monday-Friday.
//
// https://www.sgx.com/derivatives/products/chinaa50
// https://www.sgx.com/derivatives/products/chinah50
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
pub(crate) static SGX_EQUITY_INDEX_CHINA_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 16 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 5 * 3600 + 15 * 60,
    },
];

// The closing routine, "Pre - Closing: 4.30 pm - 4.34 pm / Non - Cancel: 4.34 pm
// - 4.35 pm", merged into one 16:30-16:35 window. It matches at a single closing
// price, so a trade prints in it and it stays `extended`. The H50 options row
// (FCHO) ends its continuous T phase at 16:35 rather than 16:30 because options
// carry no closing auction; the futures grid is modelled here.
pub(crate) static SGX_EQUITY_INDEX_CHINA_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 16 * 3600 + 30 * 60,
    close_ssm: 16 * 3600 + 35 * 60,
}];

// The two opening routines, "Pre - Opening: 8.45 am - 8.58 am / Non - Cancel:
// 8.58 am - 9.00 am" and "Pre - Opening: 4.40 pm - 4.43 pm / Non - Cancel: 4.43
// pm - 4.45 pm", each contiguous pair merged into one window. Neither matches:
// the opening matches land on the 09:00 and 16:45 session opens that already
// begin `regular` windows, so both windows are `order_entry`.
pub(crate) static SGX_EQUITY_INDEX_CHINA_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 45 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 40 * 60,
        close_ssm: 16 * 3600 + 45 * 60,
    },
];

// The current grid, in force from the effective day stated by SGX-DT Circular
// DT/AM 15 of 2025: see the revision note below.
pub(crate) static SGX_EQUITY_INDEX_CHINA_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_CHINA_REGULAR_CURRENT,
    extended: SGX_EQUITY_INDEX_CHINA_EXTENDED_CURRENT,
    order_entry: SGX_EQUITY_INDEX_CHINA_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Two rows, for the reasons recorded in the history note above: a knowledge
// boundary at the first surviving calendar edition, then the current grid on the
// stated effective day of SGX-DT Circular DT/AM 15 of 2025, which moved this
// family's T+1 open from 17:00 to 16:45. The family's T session and closing
// routine are unchanged across the whole window; it is Partial only because
// nothing before the 2020 edition is sourced.
//
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
// https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf
pub(crate) static SGX_EQUITY_INDEX_CHINA_REVISIONS: &[Revision] = revisions![
    (
        2020,
        1,
        1,
        &SGX_EQUITY_INDEX_CHINA_SOURCED_WINDOW,
        "first sourced SGX calendar edition"
    ),
    (
        2025,
        4,
        7,
        &SGX_EQUITY_INDEX_CHINA_BASELINE,
        "SGX-DT Circular DT/AM 15 of 2025"
    ),
];

/// Selects the SGX China equity-index profile in force on `as_of`'s Singapore day.
pub(crate) fn sgx_equity_index_china_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Asia::Singapore),
        &SGX_EQUITY_INDEX_CLOSED,
        SGX_EQUITY_INDEX_CHINA_REVISIONS,
    )
}

// --- Singapore (SiMSCI / STI / S-REIT: SGP, SGPO, ST, SRT, AJRT) -------------

// T session trades continuously 08:30-17:20; the T+1 session reopens at 17:35
// and runs to 05:15 the next calendar day, so it wraps. SGX MSCI Singapore NTR
// (USD) futures (NSG, NSP) do not share this grid and are modelled separately
// in the sibling module.
//
// https://www.sgx.com/derivatives/products/sgxsimsci
// https://www.sgx.com/derivatives/products/sgxsti
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
pub(crate) static SGX_EQUITY_INDEX_SINGAPORE_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 20 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 35 * 60,
        close_ssm: 5 * 3600 + 15 * 60,
    },
];

// The closing routine, "Pre - Closing: 5:20 pm - 5:24 pm / Non - Cancel: 5:24 pm
// - 5:25 pm", merged into one 17:20-17:25 window. It matches at a single closing
// price, so a trade prints in it and it stays `extended`.
pub(crate) static SGX_EQUITY_INDEX_SINGAPORE_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 17 * 3600 + 20 * 60,
    close_ssm: 17 * 3600 + 25 * 60,
}];

// The two opening routines, "Pre - Opening: 8:15 am - 8:28 am / Non - Cancel:
// 8:28 am - 8:30 am" and "Pre - Opening: 5:30 pm - 5:33 pm / Non - Cancel: 5:33
// pm - 5:35 pm", each contiguous pair merged into one window. The options
// variant (CSGP) publishes a single "Order Cancellation" window over the same
// spans - 08:15-08:30 and 17:30-17:35 - so these windows cover futures and
// options alike. Neither matches: the opening matches land on the 08:30 and
// 17:35 session opens that already begin `regular` windows, so both windows are
// `order_entry`.
pub(crate) static SGX_EQUITY_INDEX_SINGAPORE_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 15 * 60,
        close_ssm: 8 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 35 * 60,
    },
];

// The current grid, in force from the effective day stated by SGX-DT Circular
// DT/AM 15 of 2025: see the revision note below.
pub(crate) static SGX_EQUITY_INDEX_SINGAPORE_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_SINGAPORE_REGULAR_CURRENT,
    extended: SGX_EQUITY_INDEX_SINGAPORE_EXTENDED_CURRENT,
    order_entry: SGX_EQUITY_INDEX_SINGAPORE_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Two rows, as for the China grid: the knowledge boundary at the first
// surviving calendar edition, then the current grid on the stated effective day
// of SGX-DT Circular DT/AM 15 of 2025, which moved this family's T+1 open from
// 17:50 to 17:35. Partial only because nothing before the 2020 edition is
// sourced.
//
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
// https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf
pub(crate) static SGX_EQUITY_INDEX_SINGAPORE_REVISIONS: &[Revision] = revisions![
    (
        2020,
        1,
        1,
        &SGX_EQUITY_INDEX_SINGAPORE_SOURCED_WINDOW,
        "first sourced SGX calendar edition"
    ),
    (
        2025,
        4,
        7,
        &SGX_EQUITY_INDEX_SINGAPORE_BASELINE,
        "SGX-DT Circular DT/AM 15 of 2025"
    ),
];

/// Selects the SGX Singapore equity-index profile in force on `as_of`'s Singapore day.
pub(crate) fn sgx_equity_index_singapore_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Asia::Singapore),
        &SGX_EQUITY_INDEX_CLOSED,
        SGX_EQUITY_INDEX_SINGAPORE_REVISIONS,
    )
}
