// SPDX-License-Identifier: MIT-0

//! SGX equity-index derivatives: Japan, China and Singapore grids.
//!
//! SGX runs each equity-index family on its own clock rather than one venue
//! schedule, so every family below is modelled separately. All five SGX
//! equity-index grids share one shape: a daytime "T" session that opens after a
//! Pre-Opening/Non-Cancel order-entry routine and ends with a Pre-Closing /
//! Non-Cancel closing routine, then an overnight "T+1" session that reopens
//! after a short second order-entry routine and runs to 05:15 the next calendar
//! day. Continuous, executable phases are `regular`; the order-entry and
//! auction-matching phases, which accept orders but do not trade continuously,
//! are `extended`.
//!
//! The Taiwan and NTR (USD) grids live in the `sgx_equity_index_more` module.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, local_date, select_revision};

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

// Three non-continuous phases. SGX states the T routine as "Pre-Opening : 7.15
// am - 7.28 am / Non-Cancel : 7.28 am - 7.30 am"; the two are contiguous and
// are merged into one 07:15-07:30 order-entry window. The closing routine
// ("Pre-Closing : 2.55 pm - 2.59 pm / Non-Cancel : 2.59 pm - 3.00 pm") is
// merged the same way. The T+1 routine is "Pre-Opening : 3.05 pm - 3.08 pm /
// Non-Cancel : 3.08 pm - 3.10 pm".
pub(crate) static SGX_EQUITY_INDEX_JAPAN_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 15 * 60,
        close_ssm: 7 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 55 * 60,
        close_ssm: 15 * 3600,
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
    has_daily_close: true,
    has_weekend_close: true,
};

// Deliberately empty. SGX's equity-index hours demonstrably moved inside the
// modelled window, but every move is only bracketed between successive editions
// of the SGX derivatives trading calendar - the circular archive does not
// expose a day-level effective date for any of them. No primary source dates a
// cutover, so none is encoded and the current grid is carried back across the
// whole modelled window. That makes this family Partial rather than Primary.
//
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
// https://api2.sgx.com/sites/default/files/2025-11/DT%20Trading%20Calendar%202025.pdf
pub(crate) static SGX_EQUITY_INDEX_JAPAN_REVISIONS: &[Revision] = &[];

/// Selects the SGX Japan equity-index profile in force on `as_of`'s Singapore day.
pub(crate) fn sgx_equity_index_japan_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Asia::Singapore),
        &SGX_EQUITY_INDEX_JAPAN_BASELINE,
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

// SGX states the routines as "Pre - Opening: 8.45 am - 8.58 am / Non - Cancel:
// 8.58 am - 9.00 am", "Pre - Closing: 4.30 pm - 4.34 pm / Non - Cancel: 4.34 pm
// - 4.35 pm" and "Pre - Opening: 4.40 pm - 4.43 pm / Non - Cancel: 4.43 pm -
// 4.45 pm"; each contiguous pair is merged into a single window. The H50
// options row (FCHO) ends its continuous T phase at 16:35 rather than 16:30
// because options carry no closing auction; the futures grid is modelled here.
pub(crate) static SGX_EQUITY_INDEX_CHINA_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 45 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 35 * 60,
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
pub(crate) static SGX_EQUITY_INDEX_CHINA_REVISIONS: &[Revision] = &[];

/// Selects the SGX China equity-index profile in force on `as_of`'s Singapore day.
pub(crate) fn sgx_equity_index_china_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Asia::Singapore),
        &SGX_EQUITY_INDEX_CHINA_BASELINE,
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

// SGX states the futures routines as "Pre - Opening: 8:15 am - 8:28 am / Non -
// Cancel: 8:28 am - 8:30 am", "Pre - Closing: 5:20 pm - 5:24 pm / Non - Cancel:
// 5:24 pm - 5:25 pm" and "Pre - Opening: 5:30 pm - 5:33 pm / Non - Cancel: 5:33
// pm - 5:35 pm". The options variant (CSGP) publishes a single "Order
// Cancellation" window over the same spans - 08:15-08:30 and 17:30-17:35 - so
// the merged windows below cover futures and options alike.
pub(crate) static SGX_EQUITY_INDEX_SINGAPORE_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 15 * 60,
        close_ssm: 8 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 20 * 60,
        close_ssm: 17 * 3600 + 25 * 60,
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
pub(crate) static SGX_EQUITY_INDEX_SINGAPORE_REVISIONS: &[Revision] = &[];

/// Selects the SGX Singapore equity-index profile in force on `as_of`'s Singapore day.
pub(crate) fn sgx_equity_index_singapore_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Asia::Singapore),
        &SGX_EQUITY_INDEX_SINGAPORE_BASELINE,
        SGX_EQUITY_INDEX_SINGAPORE_REVISIONS,
    )
}
