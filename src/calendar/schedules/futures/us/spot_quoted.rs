// SPDX-License-Identifier: MIT-0

//! CME/CBOT Spot-Quoted Futures ("SQF") schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// CME/CBOT Spot-Quoted Futures ("SQF"), Rulebook Chapter 24, in
// America/Chicago: eight tradeable roots on one clock. Excludes the non-trade
// clearing legs, the financing-adjustment marker codes and CME ClearPort. No
// `regular` session in any era: four CME channels publish these hours and not
// one splits the window into RTH and ETH.
// Narrative: docs/evidence/globex_spot_quoted.md

/// The executable CME Globex leg SER-9506R established at launch and CME still
/// publishes: Sunday and Monday-Thursday 17:00 CT, wrapping local midnight to
/// a 16:00 CT close, with the operator's 60-minute maintenance period in the
/// 16:00-17:00 CT gap. Friday is not an opening day, which is what produces
/// the weekend close and the absent Friday-evening reopen.
pub(crate) static SPOT_QUOTED_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];

/// The CME Globex Pre-Open queues SER-9506R states at launch and CME's session
/// service still publishes: Sunday 16:00-17:00 CT and Monday-Thursday
/// 16:45-17:00 CT. Orders queue, amend and cancel; nothing matches until
/// 17:00, so the first 45 minutes of the daily break accept nothing at all.
pub(crate) static SPOT_QUOTED_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600,
    },
];

// CLOSED BEFORE LAUNCH. The family did not exist before its listing day, so
// the pre-launch baseline is an explicit sessionless profile rather than an
// absence — the same treatment `cryptocurrency.rs`, `ice_us.rs` and `sgx.rs`
// give their own launch-dated families. An instant below the January-2010
// audit floor therefore resolves to this closure, which is the correct oldest
// state on record for a venue that launched in 2025.
static CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

/// The one grid this family has ever published, from its launch day onward.
static FROM_2025_06_29: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: SPOT_QUOTED_EXTENDED_CURRENT,
    order_entry: SPOT_QUOTED_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2025-06-29 — THE LAUNCH, AND THE FAMILY'S ONLY REVISION ROW. SER-9506R
// states "Effective Sunday, June 29, 2025 for trade date Monday, June 30,
// 2025" and sources the wrap and both Pre-Open queues on the same day, so the
// row is T1 and everything before it is a sourced closure. The quotations and
// the one residual risk are in the evidence file.
// Evidence: docs/evidence/globex_spot_quoted.md
static REVISIONS: &[Revision] = revisions![(2025, 6, 29, &FROM_2025_06_29, "CME SER-9506R")];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, REVISIONS)
}
