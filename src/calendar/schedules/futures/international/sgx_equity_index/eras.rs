// SPDX-License-Identifier: MIT-0

//! The pre-2020 eras of the SGX Japan, China and Singapore equity-index
//! grids, and the dated 2024 Japan era.
//!
//! Every profile here is a session-bound grid SGX itself published, read from
//! the retired portal's archived pages, SGX's own product pages and content
//! API, the calendar editions that are still served, or a circular. The
//! narrative that orders these states, the law each row rests on and the
//! residual risks live in the anchor evidence file,
//! `docs/evidence/sgx_equity_index_japan.md`, which holds what the sibling
//! modules carried in comments until it moved there on 2026-10-01 UTC.

use chrono_tz::Asia;

use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::StaticHoursProfile;

/// A Monday-Friday rule from `(open_h, open_m, close_h, close_m)`; a close
/// earlier than its open wraps into the next local day.
macro_rules! rules {
    ($( ($oh:expr, $om:expr, $ch:expr, $cm:expr) ),* $(,)?) => {
        &[ $( SessionRule {
            days: MON_FRI,
            open_ssm: $oh * 3600 + $om * 60,
            close_ssm: $ch * 3600 + $cm * 60,
        } ),* ]
    };
}

macro_rules! era {
    ($name:ident, regular: [$($r:tt),* $(,)?], extended: [$($e:tt),* $(,)?], order_entry: [$($o:tt),* $(,)?]) => {
        pub(super) static $name: StaticHoursProfile = StaticHoursProfile {
            tz: Asia::Singapore,
            regular: rules![$($r),*],
            extended: rules![$($e),*],
            order_entry: rules![$($o),*],
            has_daily_close: true,
            has_weekend_close: true,
        };
    };
}

// --- Japan -------------------------------------------------------------------

// THE FLOOR: the intersection of the two oldest states SGX published for the NK grid this key models. Narrative:
// docs/evidence/sgx_equity_index_japan.md.
era!(SGX_JAPAN_FLOOR,
    regular: [(7, 45, 14, 25), (15, 30, 22, 55)],
    extended: [(14, 25, 14, 30)],
    order_entry: [(7, 30, 7, 45), (15, 15, 15, 30)]);

// 2013-08-26. Narrative:
// docs/evidence/sgx_equity_index_japan.md.
era!(SGX_JAPAN_FROM_2013_08_26,
    regular: [(7, 45, 14, 25), (15, 15, 2, 0)],
    extended: [(14, 25, 14, 30)],
    order_entry: [(7, 30, 7, 45)]);

// 2017-07-10. Narrative:
// docs/evidence/sgx_equity_index_japan.md.
era!(SGX_JAPAN_FROM_2017_07_10,
    regular: [(7, 30, 14, 25), (14, 55, 4, 45)],
    extended: [(14, 25, 14, 30)],
    order_entry: [(7, 15, 7, 30), (14, 45, 14, 55)]);

// 2024-11-04. Narrative:
// docs/evidence/sgx_equity_index_japan.md.
era!(SGX_JAPAN_FROM_2024_11_04,
    regular: [(7, 30, 14, 55), (15, 25, 5, 15)],
    extended: [(14, 55, 15, 0)],
    order_entry: [(7, 15, 7, 30), (15, 15, 15, 25)]);

// --- China -------------------------------------------------------------------

// THE FLOOR: the intersection of three sourced states. Narrative:
// docs/evidence/sgx_equity_index_japan.md.
era!(SGX_CHINA_FLOOR,
    regular: [(9, 15, 11, 35), (13, 0, 15, 5), (17, 0, 22, 55)],
    extended: [],
    order_entry: [(9, 0, 9, 15)]);

// 2013-08-26. Narrative:
// docs/evidence/sgx_equity_index_japan.md.
era!(SGX_CHINA_FROM_2013_08_26,
    regular: [(9, 0, 15, 55), (17, 0, 2, 0)],
    extended: [(15, 55, 16, 0)],
    order_entry: [(8, 45, 9, 0)]);

// 2017-07-10. Narrative:
// docs/evidence/sgx_equity_index_japan.md.
era!(SGX_CHINA_FROM_2017_07_10,
    regular: [(9, 0, 16, 30), (17, 0, 4, 45)],
    extended: [(16, 30, 16, 35)],
    order_entry: [(8, 45, 9, 0), (16, 50, 17, 0)]);

// --- Singapore ---------------------------------------------------------------

// THE FLOOR: the intersection of SGX's 2009 specification pages and the 2013-08-20 table. Narrative:
// docs/evidence/sgx_equity_index_japan.md.
era!(SGX_SINGAPORE_FLOOR,
    regular: [(8, 30, 17, 10), (18, 15, 22, 55)],
    extended: [(17, 10, 17, 15)],
    order_entry: [(8, 15, 8, 30), (18, 0, 18, 15)]);

// 2013-08-26. The 2013-08-20 table's state - T+1 to 02:00 - keyed to the
// following Monday because it creates a wrapping overnight close (22:55 the
// same day to 02:00 the next).
era!(SGX_SINGAPORE_FROM_2013_08_26,
    regular: [(8, 30, 17, 10), (18, 15, 2, 0)],
    extended: [(17, 10, 17, 15)],
    order_entry: [(8, 15, 8, 30), (18, 0, 18, 15)]);

// 2017-07-10. Narrative:
// docs/evidence/sgx_equity_index_japan.md.
era!(SGX_SINGAPORE_FROM_2017_07_10,
    regular: [(8, 30, 17, 10), (17, 40, 4, 45)],
    extended: [(17, 10, 17, 15)],
    order_entry: [(8, 15, 8, 30), (17, 30, 17, 40)]);

// 2019-06-10. Narrative:
// docs/evidence/sgx_equity_index_japan.md.
era!(SGX_SINGAPORE_FROM_2019_06_10,
    regular: [(8, 30, 17, 20), (17, 50, 4, 45)],
    extended: [(17, 20, 17, 25)],
    order_entry: [(8, 15, 8, 30), (17, 40, 17, 50)]);
