// SPDX-License-Identifier: MIT-0

//! Stock Exchange of Thailand cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

const TUE_SAT: [bool; 7] = [false, true, true, true, true, true, false];

static SET_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 10 * 3600,
        close_ssm: 16 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 19 * 3600,
        close_ssm: 2 * 3600 + 45 * 60,
    },
];
static SET_REGULAR_POST_2024: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 10 * 3600,
        close_ssm: 12 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600,
        close_ssm: 16 * 3600 + 30 * 60,
    },
];
static SET_REGULAR_PRE_2024: &[SessionRule] = &[
    SET_REGULAR_POST_2024[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 30 * 60,
    },
];
// SET's published trading procedure gives each pre-open phase a randomized end
// T: Pre-open I runs 09:30–T1 (T1 random 09:55–10:00), Pre-open II 13:30–T2 (T2
// random 13:55–14:00) and the night Pre-open 18:45–T4 (T4 random 18:55–19:00).
// Each is listed as order entry only — an auction order-collection phase with no
// matching and no trade reports — so the head of each window is order entry and
// only the five-minute randomization tail, where the uncross can print, stays
// extended.
// https://www.set.or.th/en/market/information/trading-procedure/trading-hours

// Randomized uncross windows: a trade can print anywhere inside these.
const SET_OPEN_1_RANDOM: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 55 * 60,
    close_ssm: 10 * 3600,
};
const SET_OPEN_2_RANDOM: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 13 * 3600 + 55 * 60,
    close_ssm: 14 * 3600,
};
// Before 2024-03-25 the afternoon session opened 30 minutes later, carrying the
// same five-minute randomization with it.
const SET_OPEN_2_RANDOM_PRE_2024: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 14 * 3600 + 25 * 60,
    close_ssm: 14 * 3600 + 30 * 60,
};
const SET_OPEN_NIGHT_RANDOM: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 18 * 3600 + 55 * 60,
    close_ssm: 19 * 3600,
};
// Pre-close 16:30–T3 (T3 random 16:35–16:40) and Off-hour T3–17:00 both admit
// Trade Reports, which print, so the whole 16:30–17:00 window stays tradeable.
// The night Pre-close 02:45–T5 (T5 random 02:55–03:00) is left tradeable as
// well: the closing uncross falls inside it and the crate has no primary source
// ruling out trade reports in the night pre-close.
const SET_PRE_CLOSE_AND_OFF_HOUR: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 16 * 3600 + 30 * 60,
    close_ssm: 17 * 3600,
};
const SET_NIGHT_PRE_CLOSE: SessionRule = SessionRule {
    days: TUE_SAT,
    open_ssm: 2 * 3600 + 45 * 60,
    close_ssm: 3 * 3600,
};

// Order entry only: no matching and no trade reports until T.
const SET_PRE_OPEN_1: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 9 * 3600 + 55 * 60,
};
const SET_PRE_OPEN_2: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 13 * 3600 + 30 * 60,
    close_ssm: 13 * 3600 + 55 * 60,
};
const SET_PRE_OPEN_2_PRE_2024: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 14 * 3600,
    close_ssm: 14 * 3600 + 25 * 60,
};
const SET_PRE_OPEN_NIGHT: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 18 * 3600 + 45 * 60,
    close_ssm: 18 * 3600 + 55 * 60,
};

static SET_EXTENDED_CURRENT: &[SessionRule] = &[
    SET_OPEN_1_RANDOM,
    SET_OPEN_2_RANDOM,
    SET_PRE_CLOSE_AND_OFF_HOUR,
    SET_OPEN_NIGHT_RANDOM,
    SET_NIGHT_PRE_CLOSE,
];
static SET_ORDER_ENTRY_CURRENT: &[SessionRule] =
    &[SET_PRE_OPEN_1, SET_PRE_OPEN_2, SET_PRE_OPEN_NIGHT];
static SET_EXTENDED_TRANSITION: &[SessionRule] = &[
    SET_OPEN_1_RANDOM,
    SET_OPEN_2_RANDOM,
    SET_PRE_CLOSE_AND_OFF_HOUR,
    SET_OPEN_NIGHT_RANDOM,
];
static SET_ORDER_ENTRY_TRANSITION: &[SessionRule] = SET_ORDER_ENTRY_CURRENT;
static SET_EXTENDED_POST_2024: &[SessionRule] = &[
    SET_OPEN_1_RANDOM,
    SET_OPEN_2_RANDOM,
    SET_PRE_CLOSE_AND_OFF_HOUR,
];
static SET_ORDER_ENTRY_POST_2024: &[SessionRule] = &[SET_PRE_OPEN_1, SET_PRE_OPEN_2];
static SET_EXTENDED_PRE_2024: &[SessionRule] = &[
    SET_OPEN_1_RANDOM,
    SET_OPEN_2_RANDOM_PRE_2024,
    SET_PRE_CLOSE_AND_OFF_HOUR,
];
static SET_ORDER_ENTRY_PRE_2024: &[SessionRule] = &[SET_PRE_OPEN_1, SET_PRE_OPEN_2_PRE_2024];

// SET's venue-availability envelope includes eligible Europe/Americas DRs.
// From 2025-05-06 those DRs trade continuously through the ordinary-share
// lunch and in a 19:00–03:00 night session. The night pre-open begins 18:45,
// continuous trading ends 02:45, and the randomized closing auction ends no
// later than 03:00. The next-local-day tail belongs to the prior opening day's
// trade date. Not every listed security is eligible for every phase.
// https://www.set.or.th/en/market/information/trading-procedure/trading-hours
// https://www.set.or.th/en/market/news-and-alert/newsdetails?id=95921400&symbol=SET
pub(crate) static SET_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Bangkok,
    regular: SET_REGULAR_CURRENT,
    extended: SET_EXTENDED_CURRENT,
    order_entry: SET_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};
// The first night session opened on Tuesday 2025-05-06. This one-day profile
// prevents the generic Tuesday 02:45–03:00 tail from appearing before launch;
// the complete recurring week starts on the following local day.
pub(crate) static SET_PROFILE_2025_05_06: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Bangkok,
    regular: SET_REGULAR_CURRENT,
    extended: SET_EXTENDED_TRANSITION,
    order_entry: SET_ORDER_ENTRY_TRANSITION,
    has_daily_close: true,
    has_weekend_close: true,
};
// SET moved the ordinary afternoon session 30 minutes earlier on 2024-03-25.
// https://www.set.or.th/en/market/news-and-alert/newsdetails?id=86864800&symbol=SET
pub(crate) static SET_PROFILE_POST_2024_03_25: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Bangkok,
    regular: SET_REGULAR_POST_2024,
    extended: SET_EXTENDED_POST_2024,
    order_entry: SET_ORDER_ENTRY_POST_2024,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static SET_PROFILE_PRE_2024_03_25: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Bangkok,
    regular: SET_REGULAR_PRE_2024,
    extended: SET_EXTENDED_PRE_2024,
    order_entry: SET_ORDER_ENTRY_PRE_2024,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &SET_PROFILE_CURRENT;

static REVISIONS: &[Revision] = revisions![
    (
        2024,
        3,
        25,
        &SET_PROFILE_POST_2024_03_25,
        "SET notification 86864800"
    ),
    (
        2025,
        5,
        6,
        &SET_PROFILE_2025_05_06,
        "SET notification 95921400"
    ),
    (
        2025,
        5,
        7,
        &SET_PROFILE_CURRENT,
        "SET notification 95921400"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &SET_PROFILE_PRE_2024_03_25,
        REVISIONS,
    )
}
