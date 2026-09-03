// SPDX-License-Identifier: MIT-0

//! SGX equity-index derivatives: Taiwan and NTR (USD) grids.
//!
//! Continuation of the `sgx_equity_index` module, which documents the shared
//! shape of the SGX equity-index families, the Pre-Opening/Non-Cancel versus
//! closing-routine classification applied below, and the evidence behind the
//! two-era revision timelines these families share with it. Split out only to
//! keep each production file within the source-reviewability ceiling.

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

// The current grid, in force from the effective day stated by SGX-DT Circular
// DT/AM 15 of 2025: see the revision note below.
pub(crate) static SGX_EQUITY_INDEX_TAIWAN_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_TAIWAN_REGULAR_CURRENT,
    extended: SGX_EQUITY_INDEX_TAIWAN_EXTENDED_CURRENT,
    order_entry: SGX_EQUITY_INDEX_TAIWAN_ORDER_ENTRY_CURRENT,
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

// SGX EQUITY-INDEX HISTORY. The evidence, the nine calendar editions, the dated
// 2025-04-07 cutover and the one transition that remains undated are all
// recorded once beside the Japan grid in the `sgx_equity_index` module; that
// note governs these two families exactly as it governs the other three. In
// short: from the first surviving edition through 2025-04-06 each family serves
// the intersection of every state sourced in that interval - Taiwan T+1 14:15
// (from the 2021 edition; see the boundary note below) and NTR (USD) T+1 19:00,
// the latest opens any edition gives - and from
// 2025-04-07 the current grid applies on the authority of SGX-DT Circular DT/AM
// 15 of 2025, which pulled both T+1 opens fifteen minutes earlier. Routines stay
// out of the earlier era because the calendars state session bounds only.
//
// https://api2.sgx.com/sites/default/files/2025-01/SGX%20Calendar%202025.pdf
// https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
// https://www.citicsf.com.hk/attachment?aid=95&uid=a1207308-0e3a-4a16-a869-a4d1b808a2b3
static SGX_EQUITY_INDEX_TAIWAN_REGULAR_SOURCED_WINDOW: &[SessionRule] = &[
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
static SGX_EQUITY_INDEX_TAIWAN_SOURCED_WINDOW: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_TAIWAN_REGULAR_SOURCED_WINDOW,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static SGX_EQUITY_INDEX_NTR_USD_REGULAR_SOURCED_WINDOW: &[SessionRule] = &[
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
static SGX_EQUITY_INDEX_NTR_USD_SOURCED_WINDOW: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_NTR_USD_REGULAR_SOURCED_WINDOW,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf
// TAIWAN'S KNOWLEDGE BOUNDARY IS A YEAR LATER THAN THE OTHER FOUR, because this
// family's contracts do not exist in the 2020 edition. That edition lists only
// the MSCI Taiwan predecessors - "SGX MSCI Taiwan Index Futures" (TW), its
// options (TWO) and its NTR (USD) sibling (NTW) - and grep finds no FTSE Taiwan
// row anywhere in it; "TWN" appears there only in the country-code legend, as
// the holiday market code for Taiwan (TWSE). The first edition that lists "SGX
// FTSE Taiwan Index Futures" under the code TWN is the 2021 one, so that is
// where this family's sourced history starts.
//
// The predecessor's hours were identical (T 08:45-13:45, T+1 14:15-05:15), so
// starting a year earlier would serve the right *times*. It would still be
// wrong: this profile is scoped to the FTSE suite, and reporting those
// contracts open through 2020 asserts a product that the cited edition does not
// contain. Dates before the 2021 edition are sessionless instead, which
// under-reports the part of 2020 after the FTSE suite launched rather than
// over-reporting the part before it - the safe direction, and the same
// treatment every unsourced era in this crate gets.
pub(crate) static SGX_EQUITY_INDEX_TAIWAN_REVISIONS: &[Revision] = revisions![
    (
        2021,
        1,
        1,
        &SGX_EQUITY_INDEX_TAIWAN_SOURCED_WINDOW,
        "first sourced SGX calendar edition listing the FTSE Taiwan suite"
    ),
    (
        2025,
        4,
        7,
        &SGX_EQUITY_INDEX_TAIWAN_BASELINE,
        "SGX-DT Circular DT/AM 15 of 2025"
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

// The current grid, in force from the effective day stated by SGX-DT Circular
// DT/AM 15 of 2025: see the revision note below.
pub(crate) static SGX_EQUITY_INDEX_NTR_USD_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_NTR_USD_REGULAR_CURRENT,
    extended: SGX_EQUITY_INDEX_NTR_USD_EXTENDED_CURRENT,
    order_entry: SGX_EQUITY_INDEX_NTR_USD_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Two rows, as for the Taiwan grid above: the knowledge boundary at the first
// surviving calendar edition, then the current grid on the effective day stated
// by SGX-DT Circular DT/AM 15 of 2025, which moved this family's T+1 open from
// 19:00 to 18:45. Partial only because nothing before the 2020 edition is
// sourced.
//
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
// https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf
pub(crate) static SGX_EQUITY_INDEX_NTR_USD_REVISIONS: &[Revision] = revisions![
    (
        2020,
        1,
        1,
        &SGX_EQUITY_INDEX_NTR_USD_SOURCED_WINDOW,
        "first sourced SGX calendar edition"
    ),
    (
        2025,
        4,
        7,
        &SGX_EQUITY_INDEX_NTR_USD_BASELINE,
        "SGX-DT Circular DT/AM 15 of 2025"
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
