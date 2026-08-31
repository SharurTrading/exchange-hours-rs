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

// No datable revision exists for this grid, so the baseline is the current
// grid: see the revision note below.
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

// SGX EQUITY-INDEX HISTORY, REBUILT 2026-08-31 FROM SIX CALENDAR EDITIONS. These
// grids previously asserted today's schedule back to the January-2010 floor
// across transitions this module itself recorded as real, which made them the
// only rows in the crate that could answer *open* on bounds that differed at the
// time. SGX's own Derivatives Trading Calendar - static, readable PDFs under
// api2.sgx.com/sites/default/files/ - supplies the dated grids, and reading six
// editions rather than two shows there were TWO changes, not one:
//
//   edition   Japan T / T+1        China T+1  SiMSCI T+1  Taiwan T+1  NTR T+1
//   2020      07:30-14:25 / 14:55  17:00      17:50       14:15       19:00
//   2021-07   07:30-14:25 / 14:55  17:00      17:50       14:15       19:00
//   2024      07:30-14:25 / 14:55  17:00      17:50       14:15       19:00
//   2025-01   07:30-14:55 / 15:25  17:00      17:50       14:15       19:00
//   2025-11   07:30-14:55 / 15:10  16:45      17:35       14:00       18:45
//   2026-01   07:30-14:55 / 15:10  16:45      17:35       14:00       18:45
//
// So Japan's T session lengthened at the 2024/2025 boundary while its T+1 moved
// to 15:25, and only later in 2025 did Japan's T+1 settle at 15:10 and the other
// four families pull their T+1 opens fifteen minutes earlier. An intersection
// computed from the 2021 and 2026 editions alone - which is what this module
// briefly shipped - puts Japan's T+1 at 15:10 and therefore reports the market
// open between 15:10 and 15:25 through 2025, when it was not. Each era is now
// carried as the grid its editions actually state.
//
// Boundaries are keyed to the trading year each annual edition governs, which is
// the strongest reading two consecutive annual documents support; neither
// transition day is stated, so both are approached from the conservative side.
// The 2025-11 revision already shows the third era, so keying it at 2026-01-01
// under-reports the last weeks of 2025 rather than over-reporting them.
//
// Routines are dropped from the historical eras deliberately: the calendar
// states session bounds only, and each Pre-Opening/Non-Cancel window and the
// closing routine moved with the session it brackets, so their historical
// positions are not sourced.
//
// Before the 2020 edition nothing is sourced and SGX's own member newsletters
// place an hours change immediately there - "Titan DTDC Newsletter - Change of
// Trading Hours" (2018-12), "- Extension of T+1 Trading Hours" (2019-07) and
// "- Ext of T+1 Trading Hours Go Live Schedule" (2019-10), all password-locked
// member documents. Those dates are modelled sessionless, matching the crate's
// treatment of every era it cannot source.
// https://api2.sgx.com/sites/default/files/2020-01/SGX%20Derivatives%20Trading%20Calendar%202020.pdf
// https://api2.sgx.com/sites/default/files/2021-07/SGX_Derivatives%20Trading%20Calendar%202021%20%28Final%20-%20Jul%29.pdf
// https://api2.sgx.com/sites/default/files/2024-01/SGX%20Calendar%202024.pdf
// https://api2.sgx.com/sites/default/files/2025-01/SGX%20Calendar%202025.pdf
// https://api2.sgx.com/sites/default/files/2025-11/DT%20Trading%20Calendar%202025.pdf
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
static SGX_EQUITY_INDEX_JAPAN_REGULAR_ERA_2020: &[SessionRule] = &[
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
static SGX_EQUITY_INDEX_JAPAN_ERA_2020: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_JAPAN_REGULAR_ERA_2020,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static SGX_EQUITY_INDEX_JAPAN_REGULAR_ERA_2025: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 30 * 60,
        close_ssm: 14 * 3600 + 55 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 25 * 60,
        close_ssm: 5 * 3600 + 15 * 60,
    },
];
static SGX_EQUITY_INDEX_JAPAN_ERA_2025: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_JAPAN_REGULAR_ERA_2025,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static SGX_EQUITY_INDEX_CHINA_REGULAR_ERA_2020: &[SessionRule] = &[
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
static SGX_EQUITY_INDEX_CHINA_ERA_2020: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_CHINA_REGULAR_ERA_2020,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static SGX_EQUITY_INDEX_SINGAPORE_REGULAR_ERA_2020: &[SessionRule] = &[
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
static SGX_EQUITY_INDEX_SINGAPORE_ERA_2020: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_SINGAPORE_REGULAR_ERA_2020,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// RETRIEVAL NOTE, and why these rows stay Partial. The SGX circular archive
// could not be reached by automated means: www.sgx.com/regulation/circulars
// redirects to the regco.sgx.com single-page app, which renders nothing without
// executing its JavaScript, and no public content API for it is reachable. The
// Derivatives Trading Calendar PDFs read above are the productive channel, and
// they date the grids but not the transition days; the newsletters that would
// date them are password-locked member documents. So these rows keep their
// review dates and their Partial basis, and the gap should be read as "the
// transition days were not stated anywhere reachable", not as "no dated
// evidence exists". A member reading those newsletters closes this.
//
// DIRECTION OF THE ERROR. Before 2026-08-31 these rows carried today's grid to
// the January-2010 floor across the moves tabulated above, which made them the
// only rows in the crate that could **over**-report — answer open on bounds
// that differed at the time. They no longer can: each era serves the grid its
// editions state, both undated transitions are approached from the conservative
// side, and dates before the 2020 edition are sessionless. Like every other
// Partial row in this crate they now err toward Closed, which is the safe
// direction for an order router.
//
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
// https://api2.sgx.com/sites/default/files/2025-11/DT%20Trading%20Calendar%202025.pdf
pub(crate) static SGX_EQUITY_INDEX_JAPAN_REVISIONS: &[Revision] = revisions![
    (
        2020,
        1,
        1,
        &SGX_EQUITY_INDEX_JAPAN_ERA_2020,
        "SGX Derivatives Trading Calendar 2020"
    ),
    (
        2025,
        1,
        1,
        &SGX_EQUITY_INDEX_JAPAN_ERA_2025,
        "SGX Calendar 2025"
    ),
    (
        2026,
        1,
        1,
        &SGX_EQUITY_INDEX_JAPAN_BASELINE,
        "SGX Calendar 2026"
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

// No datable revision exists for this grid, so the baseline is the current
// grid: see the revision note below.
pub(crate) static SGX_EQUITY_INDEX_CHINA_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_CHINA_REGULAR_CURRENT,
    extended: SGX_EQUITY_INDEX_CHINA_EXTENDED_CURRENT,
    order_entry: SGX_EQUITY_INDEX_CHINA_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Deliberately empty, for the same reason as the Japan grid: SGX's China
// equity-index hours changed inside the modelled window, but each change is
// only bracketed between successive derivatives trading calendars and the SGX
// circular archive publishes no day-level effective date for any of them.
// Inventing a cutover date is not acceptable, so the current grid is carried
// back and this family is Partial rather than Primary.
//
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
// https://api2.sgx.com/sites/default/files/2025-11/DT%20Trading%20Calendar%202025.pdf
pub(crate) static SGX_EQUITY_INDEX_CHINA_REVISIONS: &[Revision] = revisions![
    (
        2020,
        1,
        1,
        &SGX_EQUITY_INDEX_CHINA_ERA_2020,
        "SGX Derivatives Trading Calendar 2020"
    ),
    (
        2026,
        1,
        1,
        &SGX_EQUITY_INDEX_CHINA_BASELINE,
        "SGX Calendar 2026"
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

// No datable revision exists for this grid, so the baseline is the current
// grid: see the revision note below.
pub(crate) static SGX_EQUITY_INDEX_SINGAPORE_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_SINGAPORE_REGULAR_CURRENT,
    extended: SGX_EQUITY_INDEX_SINGAPORE_EXTENDED_CURRENT,
    order_entry: SGX_EQUITY_INDEX_SINGAPORE_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Deliberately empty. As with the other SGX equity-index families, the hours
// changes are bracketed between successive SGX derivatives trading calendars
// and no SGX circular dates them to a calendar day. The current grid is
// therefore carried back across the whole modelled window and this family is
// Partial rather than Primary.
//
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
// https://api2.sgx.com/sites/default/files/2025-11/DT%20Trading%20Calendar%202025.pdf
pub(crate) static SGX_EQUITY_INDEX_SINGAPORE_REVISIONS: &[Revision] = revisions![
    (
        2020,
        1,
        1,
        &SGX_EQUITY_INDEX_SINGAPORE_ERA_2020,
        "SGX Derivatives Trading Calendar 2020"
    ),
    (
        2026,
        1,
        1,
        &SGX_EQUITY_INDEX_SINGAPORE_BASELINE,
        "SGX Calendar 2026"
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
