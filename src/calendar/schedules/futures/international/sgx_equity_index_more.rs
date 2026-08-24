// SPDX-License-Identifier: MIT-0

//! SGX equity-index derivatives: Taiwan and NTR (USD) grids.
//!
//! Continuation of the `sgx_equity_index` module, which documents the shared
//! shape of the SGX equity-index families and the sourcing decision behind the
//! empty revision timelines. Split out only to keep each production file under
//! the 300-line ceiling.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, local_date, select_revision};

// --- Taiwan (FTSE Taiwan suite: TWN, MTWN, TWNO, CTWN) -----------------------

// T session trades continuously 08:45-13:45; the T+1 session reopens at 14:00
// and runs to 05:15 the next calendar day, so it is encoded as a wrapping rule.
// The Friday T+1 leg ends Saturday 05:15 and there is no Sunday session, so
// both rules are Monday-Friday. SGX describes the combined result as "more than
// 20 hours of trading across Asia, Europe and U.S. hours", which the 14:00
// through 05:15 wrap plus the daytime session reproduces.
//
// https://www.sgx.com/derivatives/products/twnfc
// https://www.sgx.com/asia-simplified/equity-derivatives
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
pub(crate) static SGX_EQUITY_INDEX_TAIWAN_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 45 * 60,
        close_ssm: 13 * 3600 + 45 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600,
        close_ssm: 5 * 3600 + 15 * 60,
    },
];

// SGX states the futures routines as "Pre - Opening: 8.30 am - 8.43 am / Non -
// Cancel: 8.43 am - 8.45 am", "Pre - Closing: 1.45 pm - 1.49 pm / Non - Cancel:
// 1.49 pm - 1.50 pm" and a T+1 "Pre - Opening: 1.55 pm - 1.58 pm / Non -
// Cancel: 1.58 pm - 2.00 pm". Each contiguous pair is merged into one window.
// The options variant publishes a single "Order Cancellation" window over the
// same spans - 08:30-08:45 and 13:55-14:00 - so the merged windows cover both.
pub(crate) static SGX_EQUITY_INDEX_TAIWAN_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30 * 60,
        close_ssm: 8 * 3600 + 45 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 45 * 60,
        close_ssm: 13 * 3600 + 50 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 55 * 60,
        close_ssm: 14 * 3600,
    },
];

// No datable revision exists for this grid, so the baseline is the current
// grid: see the revision note below.
pub(crate) static SGX_EQUITY_INDEX_TAIWAN_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_TAIWAN_REGULAR_CURRENT,
    extended: SGX_EQUITY_INDEX_TAIWAN_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// Deliberately empty. The SGX Taiwan grid moved inside the modelled window, but
// each move is only bracketed between successive editions of the SGX
// derivatives trading calendar; the SGX circular archive does not expose a
// day-level effective date for any hours change. No cutover is invented, so the
// current grid is carried back across the whole modelled window, which makes
// this family Partial rather than Primary.
//
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
// https://api2.sgx.com/sites/default/files/2025-11/DT%20Trading%20Calendar%202025.pdf
pub(crate) static SGX_EQUITY_INDEX_TAIWAN_REVISIONS: &[Revision] = &[];

/// Selects the SGX Taiwan equity-index profile in force on `as_of`'s Singapore day.
pub(crate) fn sgx_equity_index_taiwan_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Asia::Singapore),
        &SGX_EQUITY_INDEX_TAIWAN_BASELINE,
        SGX_EQUITY_INDEX_TAIWAN_REVISIONS,
    )
}

// --- NTR (USD) / global index grid (NSG, NSP, FN*/F*/E*/MCN* series) ---------

// The widest of the five SGX equity-index grids: the T session trades
// continuously 07:25-18:30, and the T+1 session reopens at 18:45 and runs to
// 05:15 the next calendar day, so it wraps. SGX's Calendar 2026 lists this
// whole series uniformly as "7:25am 6:30pm 6:45pm 5:15am" (T start, T end, T+1
// start, T+1 end), which is why one profile covers the entire NTR (USD) and
// global-index family rather than one profile per contract code.
//
// https://www.sgx.com/derivatives/products/sgxsimsci
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
pub(crate) static SGX_EQUITY_INDEX_NTR_USD_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 25 * 60,
        close_ssm: 18 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 18 * 3600 + 45 * 60,
        close_ssm: 5 * 3600 + 15 * 60,
    },
];

// SGX states the routines as "Pre - Opening: 7.10 am - 7.23 am / Non - Cancel:
// 7.23 am - 7.25 am", "Pre - Closing: 6.30 pm - 6.34 pm / Non - Cancel: 6.34 pm
// - 6.35 pm" and "Pre - Opening: 6.40 pm - 6.43 pm / Non - Cancel: 6.43 pm -
// 6.45 pm". Each contiguous pair is merged into a single order-entry window.
pub(crate) static SGX_EQUITY_INDEX_NTR_USD_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 10 * 60,
        close_ssm: 7 * 3600 + 25 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 18 * 3600 + 30 * 60,
        close_ssm: 18 * 3600 + 35 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 18 * 3600 + 40 * 60,
        close_ssm: 18 * 3600 + 45 * 60,
    },
];

// No datable revision exists for this grid, so the baseline is the current
// grid: see the revision note below.
pub(crate) static SGX_EQUITY_INDEX_NTR_USD_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_NTR_USD_REGULAR_CURRENT,
    extended: SGX_EQUITY_INDEX_NTR_USD_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// Deliberately empty, for the reason recorded on every SGX equity-index family
// here: the hours changes are bracketed between successive SGX derivatives
// trading calendars and the SGX circular archive publishes no day-level
// effective date for them. The current grid is carried back across the whole
// modelled window, so this family is Partial rather than Primary.
//
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
// https://api2.sgx.com/sites/default/files/2025-11/DT%20Trading%20Calendar%202025.pdf
pub(crate) static SGX_EQUITY_INDEX_NTR_USD_REVISIONS: &[Revision] = &[];

/// Selects the SGX NTR (USD) equity-index profile in force on `as_of`'s Singapore day.
pub(crate) fn sgx_equity_index_ntr_usd_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Asia::Singapore),
        &SGX_EQUITY_INDEX_NTR_USD_BASELINE,
        SGX_EQUITY_INDEX_NTR_USD_REVISIONS,
    )
}
