// SPDX-License-Identifier: MIT-0

//! The 2019-11-11 sourced-window tables for the SGX equity-index families.
//!
//! The current-grid tables and the dated revision rows live in the parent
//! module and the floor and pre-2020 tables in the sibling `eras` module; the
//! published evidence behind every row lives in the anchor evidence file,
//! `docs/evidence/sgx_equity_index_japan.md`, which holds the narrative this
//! module carried until it moved there on 2026-10-01 UTC.

use chrono_tz::Asia;

use super::{MON_FRI, SessionRule, StaticHoursProfile};

// The published evidence behind every row - the calendar editions, the
// two circulars and the product-catalogue change log, the pre-2020 states
// and their keying, the routines and the residual risks - is in the anchor
// evidence file's history sections below the per-family narratives.
// Narrative: docs/evidence/sgx_equity_index_japan.md
static SGX_EQUITY_INDEX_JAPAN_REGULAR_FROM_2019_11_11: &[SessionRule] = &[
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
static SGX_EQUITY_INDEX_JAPAN_EXTENDED_FROM_2019_11_11: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 14 * 3600 + 25 * 60,
    close_ssm: 14 * 3600 + 30 * 60,
}];
static SGX_EQUITY_INDEX_JAPAN_ORDER_ENTRY_FROM_2019_11_11: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 15 * 60,
        close_ssm: 7 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 45 * 60,
        close_ssm: 14 * 3600 + 55 * 60,
    },
];
pub(super) static SGX_EQUITY_INDEX_JAPAN_SOURCED_WINDOW: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_JAPAN_REGULAR_FROM_2019_11_11,
    extended: SGX_EQUITY_INDEX_JAPAN_EXTENDED_FROM_2019_11_11,
    order_entry: SGX_EQUITY_INDEX_JAPAN_ORDER_ENTRY_FROM_2019_11_11,
    has_daily_close: true,
    has_weekend_close: true,
};

static SGX_EQUITY_INDEX_CHINA_REGULAR_FROM_2019_11_11: &[SessionRule] = &[
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
static SGX_EQUITY_INDEX_CHINA_EXTENDED_FROM_2019_11_11: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 16 * 3600 + 30 * 60,
    close_ssm: 16 * 3600 + 35 * 60,
}];
static SGX_EQUITY_INDEX_CHINA_ORDER_ENTRY_FROM_2019_11_11: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 45 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 50 * 60,
        close_ssm: 17 * 3600,
    },
];
pub(super) static SGX_EQUITY_INDEX_CHINA_SOURCED_WINDOW: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Singapore,
    regular: SGX_EQUITY_INDEX_CHINA_REGULAR_FROM_2019_11_11,
    extended: SGX_EQUITY_INDEX_CHINA_EXTENDED_FROM_2019_11_11,
    order_entry: SGX_EQUITY_INDEX_CHINA_ORDER_ENTRY_FROM_2019_11_11,
    has_daily_close: true,
    has_weekend_close: true,
};

static SGX_EQUITY_INDEX_SINGAPORE_REGULAR_FROM_2019_11_11: &[SessionRule] = &[
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
static SGX_EQUITY_INDEX_SINGAPORE_EXTENDED_FROM_2019_11_11: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 17 * 3600 + 20 * 60,
    close_ssm: 17 * 3600 + 25 * 60,
}];
static SGX_EQUITY_INDEX_SINGAPORE_ORDER_ENTRY_FROM_2019_11_11: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 15 * 60,
        close_ssm: 8 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 40 * 60,
        close_ssm: 17 * 3600 + 50 * 60,
    },
];
pub(super) static SGX_EQUITY_INDEX_SINGAPORE_SOURCED_WINDOW: StaticHoursProfile =
    StaticHoursProfile {
        tz: Asia::Singapore,
        regular: SGX_EQUITY_INDEX_SINGAPORE_REGULAR_FROM_2019_11_11,
        extended: SGX_EQUITY_INDEX_SINGAPORE_EXTENDED_FROM_2019_11_11,
        order_entry: SGX_EQUITY_INDEX_SINGAPORE_ORDER_ENTRY_FROM_2019_11_11,
        has_daily_close: true,
        has_weekend_close: true,
    };
