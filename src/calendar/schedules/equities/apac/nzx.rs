// SPDX-License-Identifier: MIT-0

//! New Zealand Exchange Main Board cash equities.

use chrono_tz::Pacific;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// NZX Main Board: pre-open/order entry 08:30–10:00, continuous
// 10:00–16:45, and pre-close/closing-auction orders 16:45–17:00. Both auction
// uncrosses are randomized ±30 seconds around the nominal boundary; this
// deterministic venue profile uses 10:00 and 17:00. Enquiry and Adjust do not
// accept automatically matched orders and are excluded.
// Sources:
// https://www.nzx.com/learning/help-reference/trading-hours
// https://www.nzx.com/learning/issuer-participant-resources/nzx-trading/anatomy-of-a-trading-day
static NZX_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 10 * 3600,
    close_ssm: 16 * 3600 + 45 * 60,
}];
// Tradeable: the opening and closing uncrosses are randomized +/- 30 seconds
// around 10:00 and 17:00, so a print is possible from 09:59:30 and from
// 16:59:30. These windows are shared by both revisions — only the pre-open
// start moved in 2020, and that start now sits in the order-entry slice.
// The closing uncross is randomised within 30 seconds EITHER SIDE of 17:00, so
// the tradeable window runs to 17:00:30; stopping at 17:00 dropped the half of
// the randomisation in which the official closing print most often occurs.
static NZX_SHARED_EXTENDED: [SessionRule; 2] = [
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 59 * 60 + 30,
        close_ssm: 10 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 59 * 60 + 30,
        close_ssm: 17 * 3600 + 30,
    },
];
// Pre-Open is TRADEABLE, not order-entry-only. NZX's Anatomy of a Trading Day
// says of it: "Orders can be placed, amended, and deleted. No trades execute
// until the opening auction. Off-market trades may be reported." Off-market
// reports print, so a price can occur in this window. Only the Pre-Close phase
// is genuinely order-entry-only.
static NZX_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30 * 60,
        close_ssm: 9 * 3600 + 59 * 60 + 30,
    },
    NZX_SHARED_EXTENDED[0],
    NZX_SHARED_EXTENDED[1],
];
static NZX_EXTENDED_PRE_2020_04_06: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 9 * 3600 + 59 * 60 + 30,
    },
    NZX_SHARED_EXTENDED[0],
    NZX_SHARED_EXTENDED[1],
];

// Order entry only. NZX documents the 08:30–10:00 phase as pre-open/order entry
// and 16:45–17:00 as pre-close order entry for the closing auction; neither
// matches. The slices stop 30 seconds short of the nominal boundary so the
// randomized uncross stays inside the tradeable window above.
static NZX_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 16 * 3600 + 45 * 60,
    close_ssm: 16 * 3600 + 59 * 60 + 30,
}];
pub(crate) static NZX_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Pacific::Auckland,
    regular: NZX_REGULAR,
    extended: NZX_EXTENDED,
    order_entry: NZX_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// NZX moved pre-open 09:00 -> 08:30 effective 2020-04-06 and later made the
// initially temporary change indefinite.
// https://www.nzx.com/announcements/350919
// https://www.nzx.com/announcements/353837
pub(crate) static NZX_PROFILE_PRE_2020_04_06: StaticHoursProfile = StaticHoursProfile {
    tz: Pacific::Auckland,
    regular: NZX_REGULAR,
    extended: NZX_EXTENDED_PRE_2020_04_06,
    order_entry: NZX_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &NZX_PROFILE;

static REVISIONS: &[Revision] = revisions![(2020, 4, 6, &NZX_PROFILE, "NZX announcement 350919"),];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &NZX_PROFILE_PRE_2020_04_06,
        REVISIONS,
    )
}
