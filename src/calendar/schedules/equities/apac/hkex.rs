// SPDX-License-Identifier: MIT-0

//! Hong Kong Exchanges and Clearing securities market.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static HKEX_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 16 * 3600,
}];
static HKEX_REGULAR_PRE_2011: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 10 * 3600,
    close_ssm: 16 * 3600,
}];
static HKEX_PREOPEN_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 9 * 3600 + 30 * 60,
}];
static HKEX_PREOPEN_OLD: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 10 * 3600,
}];
static HKEX_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 9 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 16 * 3600 + 10 * 60,
    },
];

// Current HKEX securities venue envelope: POS 09:00–09:30, continuous
// trading 09:30–16:00, then CAS with a randomized 16:08–16:10 close. The
// Extended Morning Session keeps eligible securities continuously tradable
// through the ordinary-board lunch, so the venue-level regular envelope has no
// midday gap. The static profile uses the maximum scheduled CAS edge; not
// every security is eligible for every phase.
// https://www.hkex.com.hk/Services/Trading-hours-and-Severe-Weather-Arrangements/Trading-Hours/Securities-Market?sc_lang=en
pub(crate) static HKEX_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Hong_Kong,
    regular: HKEX_REGULAR_CURRENT,
    extended: HKEX_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// HKEX Phase One took effect 2011-03-07: the 09:30–12:00 morning session,
// 12:00–13:30 Extended Morning Session, and 13:30–16:00 afternoon session form
// one continuous venue envelope. Phase Two moved the internal Extended
// Morning/afternoon handoff to 13:00 on 2012-03-05 without changing that
// envelope. CAS first changed the venue envelope for a subset of securities on
// 2016-07-25; later eligibility expansions do not create new exchange-level
// open/close cutovers.
// https://www.hkex.com.hk/News/News-Release/2011/110303news?sc_lang=en
// https://www.hkex.com.hk/News/Regulatory-Announcements/2012/120301news?sc_lang=en
// https://www.hkex.com.hk/News/Market-Communications/2016/160725news?sc_lang=en
pub(crate) static HKEX_PROFILE_POST_2011_03_07: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Hong_Kong,
    regular: HKEX_REGULAR_CURRENT,
    extended: HKEX_PREOPEN_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static HKEX_PROFILE_PRE_2011_03_07: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Hong_Kong,
    regular: HKEX_REGULAR_PRE_2011,
    extended: HKEX_PREOPEN_OLD,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &HKEX_PROFILE_CURRENT;

static REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2011, 3, 7),
        profile: &HKEX_PROFILE_POST_2011_03_07,
    },
    Revision {
        effective: effective_date(2016, 7, 25),
        profile: &HKEX_PROFILE_CURRENT,
    },
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &HKEX_PROFILE_PRE_2011_03_07,
        REVISIONS,
    )
}
