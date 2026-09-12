// SPDX-License-Identifier: MIT-0

//! ICE Futures U.S. US Dollar Index (`USDX` / `DX`) futures and options
//! schedules.

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// USDX is a near-24-hour contract whose session commences on the evening
// before its trade date, with a Sunday evening that opens two hours earlier
// than the weekday evenings. The weekday rule stops at Thursday: a Friday
// 20:00 open would belong to a Saturday trade date, which does not exist.
// Narrative: docs/evidence/ice_us_dollar_index.md
pub(crate) static ICE_USDX_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 18 * 3600,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 20 * 3600,
        close_ssm: 17 * 3600,
    },
];

// Order entry only, 30 minutes ahead of each open, and nothing else: USDX has
// no post-close pre-open. ICE's own wording settles the classification — the
// platform "is available 30 minutes before the opening for order entry" and
// nothing matches until the open — so `extended` is empty.
// Narrative: docs/evidence/ice_us_dollar_index.md
pub(crate) static ICE_USDX_EXTENDED_CURRENT: &[SessionRule] = &[];
pub(crate) static ICE_USDX_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 19 * 3600 + 30 * 60,
        close_ssm: 20 * 3600,
    },
];

pub(crate) static ICE_USDX_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: ICE_USDX_REGULAR_CURRENT,
    extended: ICE_USDX_EXTENDED_CURRENT,
    order_entry: ICE_USDX_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// Baseline before 2011-02-14: the same evening opens, closing an hour later at
// 18:00 NY. The February 2011 notice changed only the close and said so, so the
// opens are carried back rather than reconstructed. The Sunday rule has equal
// endpoints, which encodes one complete local-day span: Sunday 18:00 runs
// continuously to Monday 18:00.
// Narrative: docs/evidence/ice_us_dollar_index.md
static ICE_USDX_REGULAR_BASELINE: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 18 * 3600,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 20 * 3600,
        close_ssm: 18 * 3600,
    },
];

// The 30-minute pre-open is carried back unchanged: ICE states it as a
// standing platform property rather than a dated one, so no cutover is encoded
// for the order-entry phases and only the close moves in 2011.
// Narrative: docs/evidence/ice_us_dollar_index.md
pub(crate) static ICE_USDX_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: ICE_USDX_REGULAR_BASELINE,
    extended: ICE_USDX_EXTENDED_CURRENT,
    order_entry: ICE_USDX_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// The one revision row below is T1; its effective day and citation literal are
// the row's own fields, and the notice, quotation and archive URL behind it are
// in the evidence file.
// Evidence: docs/evidence/ice_us_dollar_index.md
pub(crate) static ICE_USDX_REVISIONS: &[Revision] =
    revisions![(2011, 2, 14, &ICE_USDX_CURRENT, "ICE ExNot 020311 DX hours"),];

/// Selects the USDX profile in force on `as_of`'s New York day.
pub(crate) fn ice_usdx_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::New_York),
        &ICE_USDX_BASELINE,
        ICE_USDX_REVISIONS,
    )
}
