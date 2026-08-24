// SPDX-License-Identifier: MIT-0

//! ICE Futures Abu Dhabi Murban Crude Oil Futures.

use chrono_tz::{America, Asia};

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_ONLY, TUE_FRI};
use crate::calendar::schedules::timeline::{effective_date, local_date, reference_delta_seconds};

// The IFAD default is the physically delivered Murban Crude Oil Futures (ADM)
// contract. Circular 21/003 launched it on 2021-03-29 and publishes the normal
// Monday-Friday grid plus the two-hour-earlier Monday trading-day open. The
// live contract page gives the same 20:00-18:00 New York schedule and 19:45
// pre-open. IFAD's annual DST circulars confirm that the grid follows US
// Eastern time when London and New York clocks are temporarily misaligned.
// https://www.ice.com/publicdocs/abu_dhabi/circulars/IFAD%20Circular%20-%2021003%20-%20Trading%20information%20publication.pdf
// https://www.ice.com/products/75443578/Murban-Crude-Oil-Futures/
// https://www.ice.com/publicdocs/abu_dhabi/circulars/2026.03_-_IFAD_Trading_Hours_Change_Final.pdf
// `EARLY` is the Dubai translation while New York observes daylight time.
static EARLY_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: MON_ONLY,
        open_ssm: 2 * 3600,
        close_ssm: 2 * 3600,
    },
    SessionRule {
        days: TUE_FRI,
        open_ssm: 4 * 3600,
        close_ssm: 2 * 3600,
    },
];
// The window before each open is the pre-open the contract page publishes
// ("19:45 pre-open" on the 20:00 New York open, an hour earlier on the Monday
// trading-day open): order entry, amendment and cancellation only, with no
// matching until the open. It is therefore held in order_entry. Murban has no
// tradeable phase outside its near-24-hour session, so extended is empty.
static EARLY_ORDER_ENTRY: &[SessionRule] = &[
    SessionRule {
        days: MON_ONLY,
        open_ssm: 3600,
        close_ssm: 2 * 3600,
    },
    SessionRule {
        days: TUE_FRI,
        open_ssm: 3 * 3600 + 45 * 60,
        close_ssm: 4 * 3600,
    },
];
static LATE_REGULAR: &[SessionRule] = &[
    SessionRule {
        days: MON_ONLY,
        open_ssm: 3 * 3600,
        close_ssm: 3 * 3600,
    },
    SessionRule {
        days: TUE_FRI,
        open_ssm: 5 * 3600,
        close_ssm: 3 * 3600,
    },
];
static LATE_ORDER_ENTRY: &[SessionRule] = &[
    SessionRule {
        days: MON_ONLY,
        open_ssm: 2 * 3600,
        close_ssm: 3 * 3600,
    },
    SessionRule {
        days: TUE_FRI,
        open_ssm: 4 * 3600 + 45 * 60,
        close_ssm: 5 * 3600,
    },
];

pub(crate) static ICE_ABU_DHABI_CURRENT: StaticHoursProfile =
    dubai_profile(EARLY_REGULAR, EARLY_ORDER_ENTRY);
static ICE_ABU_DHABI_LATE: StaticHoursProfile = dubai_profile(LATE_REGULAR, LATE_ORDER_ENTRY);
static ICE_ABU_DHABI_CLOSED: StaticHoursProfile = dubai_profile(&[], &[]);

const fn dubai_profile(
    regular: &'static [SessionRule],
    order_entry: &'static [SessionRule],
) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: Asia::Dubai,
        regular,
        extended: &[],
        order_entry,
        has_daily_close: true,
        has_weekend_close: true,
    }
}

const IFAD_LAUNCH: chrono::NaiveDate = effective_date(2021, 3, 29);

pub(crate) fn ice_abu_dhabi_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    if local_date(as_of, Asia::Dubai) < IFAD_LAUNCH {
        return &ICE_ABU_DHABI_CLOSED;
    }
    if reference_delta_seconds(as_of, Asia::Dubai, America::New_York) == -8 * 3600 {
        &ICE_ABU_DHABI_CURRENT
    } else {
        &ICE_ABU_DHABI_LATE
    }
}
