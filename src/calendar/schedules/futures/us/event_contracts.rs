// SPDX-License-Identifier: MIT-0

//! CME Group Event Contracts on futures schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// CME Group Event Contracts on futures in America/Chicago: the Chapter 23 and
// Chapter 23A roots on one grid. A Termination of Trading time is not this
// family's session close, which is why six different close values in CME's
// hours cells produce one key rather than six. No `regular` session in any
// era, so the whole 17:00 to 16:00 CT wrap is `extended`.
// Narrative: docs/evidence/globex_event_contracts.md

/// The executable CME Globex leg: Sunday and Monday-Thursday 17:00 CT wrapping
/// local midnight to a 16:00 CT close, leaving the operator's 60-minute daily
/// maintenance period in the 16:00-17:00 CT gap. Friday is not an opening day,
/// which is what produces the weekend close and the absent Friday-evening
/// reopen — CME's Pre-Open row names Sunday and Monday-Thursday only.
pub(crate) static EVENT_CONTRACTS_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];

/// The CME Globex Pre-Open queues SER-8968R states on the launch day and CME
/// has restated in every hours publication since: Sunday 16:00-17:00 CT and
/// Monday-Thursday 16:45-17:00 CT. Orders queue, amend and cancel; nothing
/// matches until 17:00, so the first 45 minutes of the daily break accept
/// nothing at all.
pub(crate) static EVENT_CONTRACTS_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
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
// absence — the same treatment `cryptocurrency.rs`, `spot_quoted.rs`,
// `ice_us.rs` and `sgx.rs` give their own launch-dated families. An instant
// below the January-2010 audit floor therefore resolves to this closure, which
// is the correct oldest state on record for a family listed in 2022.
static CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

/// The one grid this family has ever published, from its launch day onward.
static FROM_2022_09_18: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: EVENT_CONTRACTS_EXTENDED_CURRENT,
    order_entry: EVENT_CONTRACTS_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2022-09-18 — THE LAUNCH, AND THE FAMILY'S ONLY REVISION ROW. SER-8968R
// states "Effective Sunday, September 18, 2022, for trade date Monday,
// September 19, 2022" with the Pre-Opens and the 17:00 CT open but no daily
// close; 16:00 CT is carried back from SER-9624 and SER-9740R with no cutover
// asserted. The row is T1; quotations and the knowledge bound are filed here.
// Evidence: docs/evidence/globex_event_contracts.md
static REVISIONS: &[Revision] = revisions![(2022, 9, 18, &FROM_2022_09_18, "CME SER-8968R")];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, REVISIONS)
}
