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

// SGX EQUITY-INDEX HISTORY. The evidence, the calendar editions, the dated
// cutovers, the undated moves and how each is served are recorded once in the
// `sgx_equity_index::history` module; that note governs these two families
// exactly as it governs the other three. In short: each family serves its
// sourced states from its own knowledge boundary - Taiwan from its 2020-07-20
// launch, NTR (USD) from the 2018 (Apr) calendar edition - with the undated
// 04:45 -> 05:15 T+1 close move served as an intersection until the 2020 row,
// and from 2025-04-07 the current grid applies on the authority of SGX-DT
// Circular DT/AM 15 of 2025, which pulled both T+1 opens fifteen minutes
// earlier. Routines are sourced for every era here from SGX's content API,
// which states each family's Pre-Opening/Non-Cancel/Pre-Closing windows
// (captures 2019-02-04, 2019-06-11, 2020-01-09 and 2020-07-15).
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

// 2018-04-16 to 2019-12-31: the suite's first listing. The 2018 (Apr) calendar
// edition (PDF created 2018-04-11) prints thirteen NTR (USD) rows, every one
// "7.25am to 6.30pm / 7.00pm to 4.45am", and the 2019 edition repeats them;
// neither 2017 portal table lists the suite. Keyed to the Monday after the
// edition's own creation date rather than to its edition year: SGX's product
// change log records "Change of Trading Hours for EM and NTR suite" in an
// entry issued 2017-12-29 with no day, so a 1 January key would carry the grid
// across an undated change with no second state to intersect, and the row
// creates a wrapping overnight close, which the history note explains is why
// it lands on a Monday. Routines as above, from the content API's 2019-02-04
// payload.
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

// https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf
// TAIWAN'S KNOWLEDGE BOUNDARY IS ITS OWN LAUNCH, NOT AN EDITION. This family's
// contracts do not exist in the 2020 calendar edition, which lists only the
// MSCI Taiwan predecessors - "SGX MSCI Taiwan Index Futures" (TW), its options
// (TWO) and its NTR (USD) sibling (NTW) - and the 2021 edition is the first
// calendar to list "SGX FTSE Taiwan Index Futures" under TWN. But an edition is
// one channel: SGX's own media release of 1 July 2020 states the launch day
// (20 July 2020), and SGX's content API lists "SGX FTSE Taiwan Index Futures"
// with its full grid and routines on 2020-07-15 while its 2020-06-02 payload
// still carries only MSCI Taiwan. So the family's sourced history starts on its
// stated launch day, 2020-07-20, on the grid the content API prints, and the
// months between the launch and the 2021 edition are no longer sessionless.
//
// The predecessor's hours were identical (T 08:45-13:45, T+1 14:15-05:15), so
// starting at the 2020 edition would serve the right *times*. It would still be
// wrong: this profile is scoped to the FTSE suite, and reporting those
// contracts open before 20 July 2020 asserts a product SGX had not yet listed.
// Dates before the launch are sessionless, which is exact rather than
// conservative: the family was not trading.
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
// continuously 07:25-18:30, and the T+1 session reopens at 18:45 and runs to
// 05:15 the next calendar day, so it wraps. SGX's Calendar 2026 lists this
// whole series uniformly as "7:25am 6:30pm 6:45pm 5:15am" (T start, T end, T+1
// start, T+1 end), which is why one profile covers the entire NTR (USD) and
// global-index family rather than one profile per contract code.
//
// THE SUITE'S MEMBERSHIP GREW; ITS GRID DID NOT. The codes listed above are
// today's. The 2018 (Apr) edition is the first to list the suite - thirteen
// NTR (USD) rows including NSG, one of the two codes this key names, all on
// one 07:25-18:30 / 19:00-04:45 pair; neither 2017 portal table has an NTR
// row, and SGX's launch release of 12 June 2017 names four contracts and no
// hours. NSP joins at the 2020 edition, which carries an MSCI-branded suite
// (NJP, NTW, NSP); the FN* series appears from the 2021 edition and the MCN*
// series from the 2024 one. Every edition puts whichever contracts it lists on
// the identical pair, and NSG is present in all of them, so the grid this key
// models is continuously sourced from the 2018 (Apr) edition. That is the difference from the FTSE Taiwan suite below, whose
// boundary is a launch day rather than a first listing.
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

// Three rows: the knowledge boundary at the 2018 (Apr) edition, the 2020 row
// that carries the 05:15 close (keyed to Monday 2020-01-06, as every boundary
// that lengthens the overnight close is), then the current grid on the effective day
// stated by SGX-DT Circular DT/AM 15 of 2025, which moved this family's T+1
// open from 19:00 to 18:45. Partial because the 2019 T+1 close move is undated
// and served as an intersection.
//
// https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
// https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf
pub(crate) static SGX_EQUITY_INDEX_NTR_USD_REVISIONS: &[Revision] = revisions![
    (
        2018,
        4,
        16,
        &SGX_EQUITY_INDEX_NTR_USD_FROM_2018_04_16,
        "SGX Derivatives Trading Calendar 2018 (Apr) edition, the first listing the NTR (USD) suite"
    ),
    (
        2020,
        1,
        6,
        &SGX_EQUITY_INDEX_NTR_USD_SOURCED_WINDOW,
        "SGX Derivatives Trading Calendar 2020 edition: T+1 close 05:15, keyed to the Monday"
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
