// SPDX-License-Identifier: MIT-0

//! SGX equity-index derivatives: Taiwan and NTR (USD) grids.
//!
//! Continuation of the `sgx_equity_index` module, which documents the shared
//! shape of the SGX equity-index families and the Pre-Opening/Non-Cancel
//! versus closing-routine classification applied below; its `history`
//! submodule holds the evidence behind every family's timeline, whose
//! boundaries now differ per family (the FTSE Taiwan launch and the NTR (USD)
//! suite's first calendar edition here). Split out only to keep each
//! production file within the source-reviewability ceiling.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// --- Taiwan (FTSE Taiwan suite: TWN, MTWN, TWNO, CTWN) -----------------------

// T session trades continuously 08:45-13:45; the T+1 session reopens at 14:00
// and runs to 05:15 the next calendar day, so it wraps. Both rules stay
// Monday-Friday.
// Narrative: docs/evidence/sgx_equity_index_taiwan.md
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

// The two opening routines, each contiguous Pre-Opening and Non-Cancel pair
// merged into one window. Neither matches, so both are `order_entry`.
// Narrative: docs/evidence/sgx_equity_index_taiwan.md
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

/// Sessionless profile for dates before each family's own first sourced
/// listing: the FTSE Taiwan suite's launch day and the NTR (USD) suite's
/// first calendar edition.
static SGX_EQUITY_INDEX_CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// SGX EQUITY-INDEX HISTORY. The calendar editions, the dated circulars, the
// content-API captures and the residual risks behind every era below live in
// the evidence files; this module carries only the rule tables and timelines.
// Narrative: docs/evidence/sgx_equity_index_taiwan.md
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
// The routines the content API prints for the suite from its listing on:
// "Pre - Opening: 8.30 am - 8.43 am / Non - Cancel: 8.43 am - 8.45 am / Opening:
// 8.45 am - 1.45 pm / Pre - Closing: 1.45 pm - 1.49 pm / Non - Cancel: 1.49 pm -
// 1.50 pm // Pre - Opening: 2.05 pm - 2.13 pm / Non - Cancel: 2.13 pm - 2.15 pm
// / Opening: 2.15 pm - 5.15 am" (capture 2020-07-15).
static SGX_EQUITY_INDEX_TAIWAN_EXTENDED_SOURCED_WINDOW: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 13 * 3600 + 45 * 60,
    close_ssm: 13 * 3600 + 50 * 60,
}];
static SGX_EQUITY_INDEX_TAIWAN_ORDER_ENTRY_SOURCED_WINDOW: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30 * 60,
        close_ssm: 8 * 3600 + 45 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 5 * 60,
        close_ssm: 14 * 3600 + 15 * 60,
    },
];
static SGX_EQUITY_INDEX_TAIWAN_SOURCED_WINDOW: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_TAIWAN_REGULAR_SOURCED_WINDOW,
    extended: SGX_EQUITY_INDEX_TAIWAN_EXTENDED_SOURCED_WINDOW,
    order_entry: SGX_EQUITY_INDEX_TAIWAN_ORDER_ENTRY_SOURCED_WINDOW,
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
// The routines the content API prints for the suite ("Pre - Opening: 7.10 am
// - 7.23 am / Non - Cancel: 7.23 am - 7.25 am / Opening: 7.25 am - 6.30 pm /
// Pre - Closing: 6.30 pm - 6.34 pm / Non - Cancel: 6.34 pm - 6.35 pm // Pre -
// Opening: 6.50 pm - 6.58 pm / Non - Cancel: 6.58 pm - 7.00 pm / Opening: 7.00
// pm - 5.15 am", capture 2020-01-09; the same windows with a 4.45 am close on
// 2019-02-04).
static SGX_EQUITY_INDEX_NTR_USD_EXTENDED_SOURCED_WINDOW: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 18 * 3600 + 30 * 60,
    close_ssm: 18 * 3600 + 35 * 60,
}];
static SGX_EQUITY_INDEX_NTR_USD_ORDER_ENTRY_SOURCED_WINDOW: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 10 * 60,
        close_ssm: 7 * 3600 + 25 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 18 * 3600 + 50 * 60,
        close_ssm: 19 * 3600,
    },
];
static SGX_EQUITY_INDEX_NTR_USD_SOURCED_WINDOW: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_NTR_USD_REGULAR_SOURCED_WINDOW,
    extended: SGX_EQUITY_INDEX_NTR_USD_EXTENDED_SOURCED_WINDOW,
    order_entry: SGX_EQUITY_INDEX_NTR_USD_ORDER_ENTRY_SOURCED_WINDOW,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2018-04-16 to 2019-11-10: the suite's first listing, T 07:25-18:30 and T+1
// 19:00-04:45 from the 2018 (Apr) calendar edition, keyed to the Monday after
// that edition's own creation date.
// Narrative: docs/evidence/sgx_equity_index_taiwan.md
static SGX_EQUITY_INDEX_NTR_USD_REGULAR_2018: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 25 * 60,
        close_ssm: 18 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 19 * 3600,
        close_ssm: 4 * 3600 + 45 * 60,
    },
];
static SGX_EQUITY_INDEX_NTR_USD_FROM_2018_04_16: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_NTR_USD_REGULAR_2018,
    extended: SGX_EQUITY_INDEX_NTR_USD_EXTENDED_SOURCED_WINDOW,
    order_entry: SGX_EQUITY_INDEX_NTR_USD_ORDER_ENTRY_SOURCED_WINDOW,
    has_daily_close: true,
    has_weekend_close: true,
};

// Two rows: the launch grid from 2020-07-20 and the current grid from
// 2025-04-07 on DT/AM 15 of 2025. Both rows are T1 for their day; the launch
// grid itself comes from SGX's own content API at T2, and the captures,
// quotations and URLs are in the evidence file.
// Evidence: docs/evidence/sgx_equity_index_taiwan.md
pub(crate) static SGX_EQUITY_INDEX_TAIWAN_REVISIONS: &[Revision] = revisions![
    (
        2020,
        7,
        20,
        &SGX_EQUITY_INDEX_TAIWAN_SOURCED_WINDOW,
        "SGX FTSE Taiwan launch, 20 July 2020; grid from the SGX content API, capture 2020-07-15"
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
// continuously 07:25-18:30 and the T+1 session reopens at 18:45 and wraps to
// 05:15. One profile covers the whole NTR (USD) and global-index family
// because SGX lists every contract in it on the identical four instants.
// Narrative: docs/evidence/sgx_equity_index_taiwan.md (this module's narrative
//   anchor); this key's own evidence file is docs/evidence/sgx_equity_index_ntr_usd.md
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

// Three rows: the knowledge boundary at the 2018 (Apr) edition, the 2019-11-11
// close move on SGX's own dated change log, and the current grid on DT/AM 15
// of 2025. Every row is T1; the editions, payloads and URLs behind them are in
// the evidence file.
// Evidence: docs/evidence/sgx_equity_index_ntr_usd.md
pub(crate) static SGX_EQUITY_INDEX_NTR_USD_REVISIONS: &[Revision] = revisions![
    (
        2018,
        4,
        16,
        &SGX_EQUITY_INDEX_NTR_USD_FROM_2018_04_16,
        "SGX Derivatives Trading Calendar 2018 (Apr) edition, the first listing the NTR (USD) suite"
    ),
    (
        2019,
        11,
        11,
        &SGX_EQUITY_INDEX_NTR_USD_SOURCED_WINDOW,
        "SGX Derivatives Products Description change log v6.9: Effective 11 Nov, T+1 close 05:15"
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
