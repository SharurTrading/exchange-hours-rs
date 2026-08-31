// SPDX-License-Identifier: MIT-0

//! SGX equity-index derivatives: Taiwan and NTR (USD) grids.
//!
//! Continuation of the `sgx_equity_index` module, which documents the shared
//! shape of the SGX equity-index families, the Pre-Opening/Non-Cancel versus
//! closing-routine classification applied below, and the sourcing decision
//! behind the empty revision timelines. Split out only to keep each production
//! file under the 300-line ceiling.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

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

// The closing routine, "Pre - Closing: 1.45 pm - 1.49 pm / Non - Cancel: 1.49 pm
// - 1.50 pm", merged into one 13:45-13:50 window. It matches at a single closing
// price, so a trade prints in it and it stays `extended`.
pub(crate) static SGX_EQUITY_INDEX_TAIWAN_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 13 * 3600 + 45 * 60,
    close_ssm: 13 * 3600 + 50 * 60,
}];

// The two opening routines, "Pre - Opening: 8.30 am - 8.43 am / Non - Cancel:
// 8.43 am - 8.45 am" and the T+1 "Pre - Opening: 1.55 pm - 1.58 pm / Non -
// Cancel: 1.58 pm - 2.00 pm", each contiguous pair merged into one window. The
// options variant publishes a single "Order Cancellation" window over the same
// spans - 08:30-08:45 and 13:55-14:00 - so these windows cover both. Neither
// matches: the opening matches land on the 08:45 and 14:00 session opens that
// already begin `regular` windows, so both windows are `order_entry`.
pub(crate) static SGX_EQUITY_INDEX_TAIWAN_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30 * 60,
        close_ssm: 8 * 3600 + 45 * 60,
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
    order_entry: SGX_EQUITY_INDEX_TAIWAN_ORDER_ENTRY_CURRENT,
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
/// Sessionless profile for dates before the first sourced SGX calendar edition.
static SGX_EQUITY_INDEX_CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// SGX EQUITY-INDEX HISTORY, REBUILT 2026-08-31. These grids previously asserted
// today's schedule back to the January-2010 floor across transitions this module
// itself recorded as real, which made them the only rows in the crate that could
// answer *open* on bounds that differed at the time. SGX's own Derivatives
// Trading Calendar proves the movement: T+1 2.15pm in the 2020/2021 editions against 2:00pm in 2025/2026, T unchanged at 8.45am to 1.45pm.
//
// The dated surface now serves the intersection of every sourced edition - the
// window that is `regular` under all of them - so no cutover is asserted and no
// instant is reported open on hours that were not in force. The routines are
// dropped from the intersection deliberately: the calendar states session bounds
// only, and each Pre-Opening/Non-Cancel window moved with the session it
// precedes, so their historical positions are not sourced.
//
// Before the 2020 edition nothing is sourced and SGX's own member newsletters
// place a hours change immediately there - "Titan DTDC Newsletter - Change of
// Trading Hours" (2018-12), "- Extension of T+1 Trading Hours" (2019-07) and
// "- Ext of T+1 Trading Hours Go Live Schedule" (2019-10), all password-locked
// member documents. Those dates are therefore modelled sessionless, matching the
// crate's treatment of every era it cannot source.
// https://api2.sgx.com/sites/default/files/2020-01/SGX%20Derivatives%20Trading%20Calendar%202020.pdf
// https://api2.sgx.com/sites/default/files/2021-07/SGX_Derivatives%20Trading%20Calendar%202021%20%28Final%20-%20Jul%29.pdf
// https://api2.sgx.com/sites/default/files/2025-11/DT%20Trading%20Calendar%202025.pdf
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
static SGX_EQUITY_INDEX_TAIWAN_REGULAR_SOURCED_SPAN: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 45 * 60,
        close_ssm: 13 * 3600 + 45 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 15 * 60,
        close_ssm: 5 * 3600 + 15 * 60,
    },
];
static SGX_EQUITY_INDEX_TAIWAN_SOURCED_SPAN: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_TAIWAN_REGULAR_SOURCED_SPAN,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// SGX EQUITY-INDEX HISTORY, REBUILT 2026-08-31. These grids previously asserted
// today's schedule back to the January-2010 floor across transitions this module
// itself recorded as real, which made them the only rows in the crate that could
// answer *open* on bounds that differed at the time. SGX's own Derivatives
// Trading Calendar proves the movement: T+1 7.00pm in the 2021 edition against 6:45pm in 2026, T unchanged at 7.25am to 6.30pm.
//
// The dated surface now serves the intersection of every sourced edition - the
// window that is `regular` under all of them - so no cutover is asserted and no
// instant is reported open on hours that were not in force. The routines are
// dropped from the intersection deliberately: the calendar states session bounds
// only, and each Pre-Opening/Non-Cancel window moved with the session it
// precedes, so their historical positions are not sourced.
//
// Before the 2020 edition nothing is sourced and SGX's own member newsletters
// place a hours change immediately there - "Titan DTDC Newsletter - Change of
// Trading Hours" (2018-12), "- Extension of T+1 Trading Hours" (2019-07) and
// "- Ext of T+1 Trading Hours Go Live Schedule" (2019-10), all password-locked
// member documents. Those dates are therefore modelled sessionless, matching the
// crate's treatment of every era it cannot source.
// https://api2.sgx.com/sites/default/files/2020-01/SGX%20Derivatives%20Trading%20Calendar%202020.pdf
// https://api2.sgx.com/sites/default/files/2021-07/SGX_Derivatives%20Trading%20Calendar%202021%20%28Final%20-%20Jul%29.pdf
// https://api2.sgx.com/sites/default/files/2025-11/DT%20Trading%20Calendar%202025.pdf
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
static SGX_EQUITY_INDEX_NTR_USD_REGULAR_SOURCED_SPAN: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 25 * 60,
        close_ssm: 18 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 19 * 3600,
        close_ssm: 5 * 3600 + 15 * 60,
    },
];
static SGX_EQUITY_INDEX_NTR_USD_SOURCED_SPAN: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_NTR_USD_REGULAR_SOURCED_SPAN,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// https://api2.sgx.com/sites/default/files/2025-11/DT%20Trading%20Calendar%202025.pdf
pub(crate) static SGX_EQUITY_INDEX_TAIWAN_REVISIONS: &[Revision] = revisions![
    (
        2020,
        1,
        1,
        &SGX_EQUITY_INDEX_TAIWAN_SOURCED_SPAN,
        "SGX Derivatives Trading Calendar 2020"
    ),
    (
        2026,
        1,
        1,
        &SGX_EQUITY_INDEX_TAIWAN_BASELINE,
        "SGX Calendar 2026"
    ),
];

/// Selects the SGX Taiwan equity-index profile in force on `as_of`'s Singapore day.
pub(crate) fn sgx_equity_index_taiwan_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Asia::Singapore),
        &SGX_EQUITY_INDEX_CLOSED,
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

// The closing routine, "Pre - Closing: 6.30 pm - 6.34 pm / Non - Cancel: 6.34 pm
// - 6.35 pm", merged into one 18:30-18:35 window. It matches at a single closing
// price, so a trade prints in it and it stays `extended`.
pub(crate) static SGX_EQUITY_INDEX_NTR_USD_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 18 * 3600 + 30 * 60,
    close_ssm: 18 * 3600 + 35 * 60,
}];

// The two opening routines, "Pre - Opening: 7.10 am - 7.23 am / Non - Cancel:
// 7.23 am - 7.25 am" and "Pre - Opening: 6.40 pm - 6.43 pm / Non - Cancel: 6.43
// pm - 6.45 pm", each contiguous pair merged into a single window. Neither
// matches: the opening matches land on the 07:25 and 18:45 session opens that
// already begin `regular` windows, so both windows are `order_entry`.
pub(crate) static SGX_EQUITY_INDEX_NTR_USD_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 10 * 60,
        close_ssm: 7 * 3600 + 25 * 60,
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
    order_entry: SGX_EQUITY_INDEX_NTR_USD_ORDER_ENTRY_CURRENT,
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
pub(crate) static SGX_EQUITY_INDEX_NTR_USD_REVISIONS: &[Revision] = revisions![
    (
        2020,
        1,
        1,
        &SGX_EQUITY_INDEX_NTR_USD_SOURCED_SPAN,
        "SGX Derivatives Trading Calendar 2020"
    ),
    (
        2026,
        1,
        1,
        &SGX_EQUITY_INDEX_NTR_USD_BASELINE,
        "SGX Calendar 2026"
    ),
];

/// Selects the SGX NTR (USD) equity-index profile in force on `as_of`'s Singapore day.
pub(crate) fn sgx_equity_index_ntr_usd_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Asia::Singapore),
        &SGX_EQUITY_INDEX_CLOSED,
        SGX_EQUITY_INDEX_NTR_USD_REVISIONS,
    )
}
