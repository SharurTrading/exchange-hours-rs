// SPDX-License-Identifier: MIT-0

//! US listed-equity-options profiles.

use chrono_tz::America;

use super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// Scope: ordinary options on individual US stocks. Execution in this family
// begins at 09:30 ET, so a venue's generic pre-open acceptance window is
// order entry, not trading; ETF, ETN, index, FLEX, floor-only and
// venue-designated extended-hours classes are separate product families.
// Evidence: docs/evidence/cboe_options_c1.md
static LISTED_EQUITY_OPTIONS_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 16 * 3600,
}];

// Order-entry-only pre-open queues. Each venue accepts, amends and cancels
// orders here, but the first execution of the day is the 09:30 ET opening.
// Evidence: docs/evidence/cboe_options_c1.md
static ORDER_ENTRY_0600: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 6 * 3600,
    close_ssm: 9 * 3600 + 30 * 60,
}];

static ORDER_ENTRY_0700: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600,
    close_ssm: 9 * 3600 + 30 * 60,
}];

static ORDER_ENTRY_0730: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 30 * 60,
    close_ssm: 9 * 3600 + 30 * 60,
}];

// Current order-acceptance edges. Each venue's queue onset is undated and is
// carried across its modelled history (decided 2026-09-01); nothing matches in
// a queue, so every venue's 09:30-16:00 ET execution history is unaffected.
// Evidence: docs/evidence/cboe_options_c1.md
/// C1. Queue carried from the January-2010 floor; onset assumed, not sourced.
pub(crate) static CBOE_OPTIONS_C1_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
/// C2. Queue carried from the sourced 2010-10-29 launch; onset assumed.
pub(crate) static CBOE_C2_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
/// BZX Options. Queue carried from the sourced 2010-02-26 launch; onset
/// assumed.
pub(crate) static CBOE_BZX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
/// EDGX Options. Queue carried from the sourced 2015-11-02 launch; onset
/// assumed.
pub(crate) static CBOE_EDGX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
/// Arca Options. Queue carried from the January-2010 floor; onset assumed.
pub(crate) static NYSE_ARCA_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0600);
/// American Options. Queue carried from the January-2010 floor; onset
/// assumed.
pub(crate) static NYSE_AMERICAN_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0600);
/// PHLX. Queue carried from the January-2010 floor; onset assumed.
pub(crate) static NASDAQ_PHLX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
/// ISE. Queue carried from the January-2010 floor; onset assumed.
pub(crate) static NASDAQ_ISE_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0600);
/// NOM. Queue carried from the January-2010 floor; onset assumed.
pub(crate) static NASDAQ_NOM_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
/// MRX. Queue carried from the sourced 2016-02-16 launch; onset assumed.
pub(crate) static NASDAQ_MRX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0600);
/// GEMX. Queue carried from the sourced 2013-08-05 launch; onset assumed.
pub(crate) static NASDAQ_GEMX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0600);
/// BX Options. Queue carried from the sourced 2012-06-29 launch; onset
/// assumed.
pub(crate) static NASDAQ_BX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
/// MIAX Options. Queue served from 2013-05-07, the first capture showing it
/// affecting the live book - NOT from launch, where the same window was
/// connectivity verification only. Sourced on both sides; no assumption.
pub(crate) static MIAX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
/// MIAX Emerald. Queue carried from the sourced 2019-03-01 launch; onset
/// assumed.
pub(crate) static MIAX_EMERALD_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
/// MIAX Pearl. Queue carried from the sourced 2017-02-06 launch; onset
/// assumed.
pub(crate) static MIAX_PEARL_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
/// MIAX Sapphire. Queue carried from the sourced 2024-08-12 launch; onset
/// assumed.
pub(crate) static MIAX_SAPPHIRE_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
/// BOX. Queue carried from the January-2010 floor; onset assumed.
pub(crate) static BOX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0700);
/// MEMX Options. No queue at all - it rejects orders before 09:30 - so
/// nothing is carried.
pub(crate) static MEMX_OPTIONS_PROFILE: StaticHoursProfile = listed_equity_options_profile(&[]);

static LISTED_EQUITY_OPTIONS_HISTORICAL: StaticHoursProfile = listed_equity_options_profile(&[]);

mod history;

pub(crate) use history::{
    box_options_profile_at, bzx_options_profile_at, c1_profile_at, c2_options_profile_at,
    edgx_options_profile_at, memx_options_profile_at, miax_emerald_options_profile_at,
    miax_options_profile_at, miax_pearl_options_profile_at, miax_sapphire_options_profile_at,
    nasdaq_bx_options_profile_at, nasdaq_gemx_profile_at, nasdaq_ise_profile_at,
    nasdaq_mrx_profile_at, nasdaq_nom_profile_at, nasdaq_phlx_profile_at,
    nyse_american_options_profile_at, nyse_arca_options_profile_at,
};

// This family has no tradeable session outside 09:30–16:00: every non-regular
// window a venue publishes here is a pre-open order-acceptance queue, so
// `extended` is empty and the queue lands in `order_entry`.
const fn listed_equity_options_profile(order_entry: &'static [SessionRule]) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: America::New_York,
        regular: LISTED_EQUITY_OPTIONS_REGULAR,
        extended: &[],
        order_entry,
        has_daily_close: true,
        has_weekend_close: true,
    }
}
