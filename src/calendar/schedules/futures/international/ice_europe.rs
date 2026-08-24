// SPDX-License-Identifier: MIT-0

//! ICE Futures Europe Brent and FTSE defaults.

use chrono_tz::{America, Europe};

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, MON_THU, SUN_ONLY};
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// ICE Europe does not have a venue-wide schedule. `iceeu` and the commodities
// row are scoped to Brent Crude Futures (B). The live product specification
// publishes the governing 20:00-18:00 New York session and 19:45 pre-open,
// plus the special Sunday 17:00 pre-open / 18:00 open. ICE's platform
// maintenance is locked to US Eastern time; using that published reference
// zone expresses the rule directly instead of encoding annual UK/US DST
// mismatch exceptions. The official 2010 circular confirms the same ET grid.
// https://www.ice.com/products/219/Brent-Crude-Futures
// https://www.ice.com/publicdocs/futures/Trading_Schedule_Temporary_Trading_Hours_for_DST.pdf
// https://www.ice.com/publicdocs/circulars/10070.pdf
static BRENT_REGULAR: &[SessionRule] = &[
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
// The 19:45 weekday and 17:00 Sunday phases are the pre-open the product
// specification publishes: orders may be entered, amended and cancelled, and
// nothing matches until the 20:00 / 18:00 open. They are order-entry phases, so
// they are held in order_entry rather than extended; Brent publishes no
// tradeable phase outside its near-24-hour session, leaving extended empty.
static BRENT_ORDER_ENTRY: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 17 * 3600,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 19 * 3600 + 45 * 60,
        close_ssm: 20 * 3600,
    },
];

pub(crate) static ICEEU_CURRENT: StaticHoursProfile =
    brent_profile(BRENT_REGULAR, BRENT_ORDER_ENTRY);
pub(crate) static ICE_EUROPE_COMMODITIES_CURRENT: StaticHoursProfile =
    brent_profile(BRENT_REGULAR, BRENT_ORDER_ENTRY);

const fn brent_profile(
    regular: &'static [SessionRule],
    order_entry: &'static [SessionRule],
) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: America::New_York,
        regular,
        extended: &[],
        order_entry,
        has_daily_close: true,
        has_weekend_close: true,
    }
}

pub(crate) fn iceeu_profile_at(
    _as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    &ICEEU_CURRENT
}

pub(crate) fn ice_europe_commodities_profile_at(
    _as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    &ICE_EUROPE_COMMODITIES_CURRENT
}

// The financials row is scoped to FTSE 100 Index Futures (Z). The live ICE
// specification publishes 00:45 pre-open, 01:00-21:00 trading. Circulars
// 14/146, 15/016, and 15/169 give the day-level migration and subsequent
// changes. The named ICE Futures Europe product is closed before its
// 2014-11-17 first trade date, yielding a complete January-2010-on timeline.
// https://www.ice.com/products/38716764/FTSE-100-INDEX-
// https://www.ice.com/publicdocs/circulars/14146.pdf
// https://www.ice.com/publicdocs/circulars/15016.pdf
// https://www.ice.com/publicdocs/circulars/15169.pdf
static FTSE_0800_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 21 * 3600,
}];
static FTSE_0700_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600,
    close_ssm: 21 * 3600,
}];
static FTSE_CURRENT_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3600,
    close_ssm: 21 * 3600,
}];
// The non-executable phases are pre-opens, and Circular 15/016 says so in its
// own column heading: its two FTSE tables read "Pre-open 06:03 | Open 08:00 |
// Close 21:00" before 16 February 2015 and "Pre-open 06:03 | Open 07:00 | Close
// 21:00" after it, so the 06:03 window is order entry ahead of the open rather
// than a session in which anything prints. The live specification's 00:45
// pre-open is the same phase on the current 01:00 open. All three therefore sit
// in order_entry; FTSE publishes no tradeable phase outside its executable
// session, so extended stays empty.
static FTSE_0603_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 6 * 3600 + 3 * 60,
    close_ssm: 8 * 3600,
}];
static FTSE_0603_TO_0700_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 6 * 3600 + 3 * 60,
    close_ssm: 7 * 3600,
}];
static FTSE_CURRENT_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 45 * 60,
    close_ssm: 3600,
}];
static FTSE_CLOSED: StaticHoursProfile = london_profile(&[], &[]);
static FTSE_0800: StaticHoursProfile = london_profile(FTSE_0800_REGULAR, FTSE_0603_ORDER_ENTRY);
static FTSE_0700: StaticHoursProfile =
    london_profile(FTSE_0700_REGULAR, FTSE_0603_TO_0700_ORDER_ENTRY);
pub(crate) static ICE_EUROPE_FINANCIALS_CURRENT: StaticHoursProfile =
    london_profile(FTSE_CURRENT_REGULAR, FTSE_CURRENT_ORDER_ENTRY);

const fn london_profile(
    regular: &'static [SessionRule],
    order_entry: &'static [SessionRule],
) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: Europe::London,
        regular,
        extended: &[],
        order_entry,
        has_daily_close: true,
        has_weekend_close: true,
    }
}

static FTSE_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2014, 11, 17),
        profile: &FTSE_0800,
    },
    Revision {
        effective: effective_date(2015, 2, 16),
        profile: &FTSE_0700,
    },
    Revision {
        effective: effective_date(2015, 10, 1),
        profile: &ICE_EUROPE_FINANCIALS_CURRENT,
    },
];

pub(crate) fn ice_europe_financials_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Europe::London),
        &FTSE_CLOSED,
        FTSE_REVISIONS,
    )
}
