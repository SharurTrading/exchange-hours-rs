// SPDX-License-Identifier: MIT-0

//! Sourced history for the SGX equity-index families.
//!
//! The current-grid tables and the dated revision rows live in the parent
//! module; this module owns the published evidence behind those rows — which
//! calendar editions were read, where they disagree, and the conservative
//! window served across the disagreement that is still undated.

use chrono_tz::Asia;

use super::{MON_FRI, SessionRule, StaticHoursProfile};

/// Sessionless profile for dates before the first sourced SGX calendar edition.
pub(super) static SGX_EQUITY_INDEX_CLOSED: StaticHoursProfile = StaticHoursProfile {
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
// Calendar 2025 (updated 31 Jul 2025).pdf - verified by digest - so those two
// files are one edition, sourced at 31 July 2025 rather than at November. The
// 2022 edition has no text layer, its glyphs being vector outlines, and was
// read from rendered pages; its six values match both neighbours exactly. No
// 2023 edition was located, so each transition below is bracketed between the
// editions actually read rather than between consecutive publication years.)
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
// 2020-01-01 is a knowledge boundary, not a claimed change: it separates what
// this crate has worked up into profiles from what it has not, the way a launch
// day separates a venue from its pre-launch closure. Four of the five families
// start there. The FTSE Taiwan suite starts at the 2021 edition instead,
// because the 2020 one contains no FTSE Taiwan contract at all - see the
// boundary note beside its revisions in `sgx_equity_index_more`.
//
// EARLIER EVIDENCE DOES EXIST; IT IS UNMODELLED, NOT UNSOURCEABLE. Do not read
// the boundary as "nothing survives before 2020". SGX's own derivatives
// Trading Hours page is archived from the retired portal, and the capture of
// 11 July 2018 states a materially different grid: Nikkei 225 T 07:45-14:25
// and T+1 15:15-02:00, FTSE Xinhua China A50 09:00-15:25 and 16:10-02:00, MSCI
// Singapore 08:30-17:10 and 18:15-02:00, MSCI Taiwan 08:45-13:45 and
// 14:35-02:00, under the same footnote convention the later PDFs use. That
// page also links "SGX Derivatives Trading Calendar 2018 (Apr Update)" and a
// September 2017 update.
//
// So the T+1 end moved from 02:00 to 05:15, and three of the four opens moved,
// between that capture and the 2020 edition - which is the change SGX's own
// newsletter index names but does not open: "Change of Trading Hours"
// (21 Sep 2017), "Extension of T+1 Trading Hours" (15 Jul 2019), "Ext of T+1
// Trading Hours Go Live Schedule" (8 Oct 2019) and "Reminder on Extension of
// T+1 Trading Hours" (5 Nov 2019), all password-locked. Modelling the earlier
// grid would need those effective days, and the pre-2019 contract set differs
// again (no NTR (USD) suite, MSCI rather than FTSE branding). Until that work
// is done these dates stay sessionless, which under-reports them rather than
// serving a grid whose end date is unknown.
// https://web.archive.org/web/20180711020353id_/http://www.sgx.com/wps/wcm/connect/mp_en/site/trading_on_sgx/derivatives_market/derivatives_trading_hours_and_calendar/Trading+Hours?%20noCache=1531274630984.837727.133108399
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
pub(super) static SGX_EQUITY_INDEX_JAPAN_SOURCED_WINDOW: StaticHoursProfile = StaticHoursProfile {
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
pub(super) static SGX_EQUITY_INDEX_CHINA_SOURCED_WINDOW: StaticHoursProfile = StaticHoursProfile {
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
pub(super) static SGX_EQUITY_INDEX_SINGAPORE_SOURCED_WINDOW: StaticHoursProfile =
    StaticHoursProfile {
        tz: Asia::Singapore,
        regular: SGX_EQUITY_INDEX_SINGAPORE_REGULAR_SOURCED_WINDOW,
        extended: &[],
        order_entry: &[],
        has_daily_close: true,
        has_weekend_close: true,
    };
