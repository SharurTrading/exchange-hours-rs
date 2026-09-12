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
//! The pre-2020 and 2024 era tables for the three families here live in the
//! `eras` submodule, and the published evidence behind every family's dated
//! history — which artifacts were read, where they disagree, and how the
//! undated changeovers are served — lives in the `history` submodule.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// --- Japan (Nikkei 225 suite: NK, NS, NU, NC, NR, ND, EJP, EJRT) -------------

// Two executable phases per trade date: the T session, and the T+1 session
// that reopens in the afternoon and wraps past local midnight. The closing
// routine matches at a single price and stays `extended`; the two opening
// routines match at the session opens, so they are `order_entry`.
// Narrative: docs/evidence/sgx_equity_index_japan.md
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

mod eras;
mod history;

use eras::{
    SGX_CHINA_FLOOR, SGX_CHINA_FROM_2013_08_26, SGX_CHINA_FROM_2017_07_10, SGX_JAPAN_FLOOR,
    SGX_JAPAN_FROM_2013_08_26, SGX_JAPAN_FROM_2017_07_10, SGX_JAPAN_FROM_2024_11_04,
    SGX_SINGAPORE_FLOOR, SGX_SINGAPORE_FROM_2013_08_26, SGX_SINGAPORE_FROM_2017_07_10,
    SGX_SINGAPORE_FROM_2019_06_10,
};
use history::{
    SGX_EQUITY_INDEX_CHINA_SOURCED_WINDOW, SGX_EQUITY_INDEX_JAPAN_SOURCED_WINDOW,
    SGX_EQUITY_INDEX_SINGAPORE_SOURCED_WINDOW,
};

// Six eras, five of them dated; the one undated move is the close and open
// between the 2013 table and the 2017 captures. Every revision row below is
// T1; each row's effective day and citation literal are its own fields, and
// the quotations, captures and residual risks are in the evidence file.
// Evidence: docs/evidence/sgx_equity_index_japan.md
pub(crate) static SGX_EQUITY_INDEX_JAPAN_REVISIONS: &[Revision] = revisions![
    (
        2013,
        8,
        26,
        &SGX_JAPAN_FROM_2013_08_26,
        "SGX portal Trading Hours table, capture 2013-08-20, keyed to the Monday"
    ),
    (
        2017,
        7,
        10,
        &SGX_JAPAN_FROM_2017_07_10,
        "SGX derivatives Trading Hours page, captures 2017-07-05 and 2017-09-27"
    ),
    (
        2019,
        11,
        11,
        &SGX_EQUITY_INDEX_JAPAN_SOURCED_WINDOW,
        "SGX Derivatives Products Description change log v6.9: Effective 11 Nov, T+1 close 05:15"
    ),
    (
        2024,
        11,
        4,
        &SGX_JAPAN_FROM_2024_11_04,
        "SGX-DT Circular DT/AM 50 of 2024"
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
        &SGX_JAPAN_FLOOR,
        SGX_EQUITY_INDEX_JAPAN_REVISIONS,
    )
}

// --- China (FTSE China A50 / H50: CN, FCH, FCHO) -----------------------------

// T session trades continuously 09:00-16:30; the T+1 session reopens at 16:45
// and runs to 05:15 the next calendar day, so it wraps. Both rules stay
// Monday-Friday: the T+1 leg starts on a Monday-Friday trade date and the
// Friday leg ends Saturday 05:15.
// Narrative: docs/evidence/sgx_equity_index_japan.md
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

// Four rows: the floor grid is this key's baseline, 2013-08-26 widens the T
// close and creates the wrapping T+1 close, 2017-07-10 is a knowledge
// boundary, 2019-11-11 carries the 05:15 close, and the current grid begins on
// DT/AM 15 of 2025. Every row is T1; the evidence file holds the quotations.
// Evidence: docs/evidence/sgx_equity_index_china.md
pub(crate) static SGX_EQUITY_INDEX_CHINA_REVISIONS: &[Revision] = revisions![
    (
        2013,
        8,
        26,
        &SGX_CHINA_FROM_2013_08_26,
        "SGX portal Trading Hours table and FTSE China A50 specification, captures 2013-08-20, keyed to the Monday"
    ),
    (
        2017,
        7,
        10,
        &SGX_CHINA_FROM_2017_07_10,
        "SGX derivatives Trading Hours page, captures 2017-07-05 and 2017-09-27"
    ),
    (
        2019,
        11,
        11,
        &SGX_EQUITY_INDEX_CHINA_SOURCED_WINDOW,
        "SGX Derivatives Products Description change log v6.9: Effective 11 Nov, T+1 close 05:15"
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
        &SGX_CHINA_FLOOR,
        SGX_EQUITY_INDEX_CHINA_REVISIONS,
    )
}

// --- Singapore (SiMSCI / STI / S-REIT: SGP, SGPO, ST, SRT, AJRT) -------------

// T session trades continuously 08:30-17:20; the T+1 session reopens at 17:35
// and runs to 05:15 the next calendar day, so it wraps.
// Narrative: docs/evidence/sgx_equity_index_japan.md
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

// The two opening routines, each contiguous Pre-Opening and Non-Cancel pair
// merged into one window. Neither matches: the opening matches land on the
// session opens that already begin `regular` windows, so both are
// `order_entry`.
// Narrative: docs/evidence/sgx_equity_index_japan.md
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

// Five rows: the floor grid is this key's baseline, 2013-08-26 creates the
// wrapping T+1 close, 2017-07-10 is a knowledge boundary, 2019-06-10 and
// 2019-11-11 come from SGX's own dated change log, and the current grid begins
// on DT/AM 15 of 2025. Every row is T1; the evidence file holds the quotations.
// Evidence: docs/evidence/sgx_equity_index_singapore.md
pub(crate) static SGX_EQUITY_INDEX_SINGAPORE_REVISIONS: &[Revision] = revisions![
    (
        2013,
        8,
        26,
        &SGX_SINGAPORE_FROM_2013_08_26,
        "SGX portal Trading Hours table, capture 2013-08-20, keyed to the Monday"
    ),
    (
        2017,
        7,
        10,
        &SGX_SINGAPORE_FROM_2017_07_10,
        "SGX derivatives Trading Hours page, captures 2017-07-05 and 2017-09-27"
    ),
    (
        2019,
        6,
        10,
        &SGX_SINGAPORE_FROM_2019_06_10,
        "SGX Derivatives Products Description change log v6.1: SGP, SGPO and ST eff 10 Jun"
    ),
    (
        2019,
        11,
        11,
        &SGX_EQUITY_INDEX_SINGAPORE_SOURCED_WINDOW,
        "SGX Derivatives Products Description change log v6.9: Effective 11 Nov, T+1 close 05:15"
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
        &SGX_SINGAPORE_FLOOR,
        SGX_EQUITY_INDEX_SINGAPORE_REVISIONS,
    )
}
