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
// Pre-opening Session split. SEHK Rule 501G divides the 09:00–09:30 POS into
// four named periods: order input 09:00–09:15, pre-order matching (renamed
// no-cancellation in 2020) 09:15–09:20, order matching from 09:20, then a
// blocking period to 09:30. HKEX states that orders "will be accumulated and
// updated but no matching will occur" during the first two periods, so
// 09:00–09:20 is order entry. Matching begins at 09:20 and prints the opening
// trades at the final IEP, so 09:20–09:30 stays extended (the blocking tail is
// kept with the match because the 2020 enhancement randomised the match end).
// https://www.hkex.com.hk/Global/Exchange/FAQ/Securities-Market/Trading/Pre_opening-Session?sc_lang=en
// https://www.hkex.com.hk/-/media/HKEX-Market/Services/Rules-and-Forms-and-Fees/Rules/SEHK/Securities/Rule-Update_Rules-of-the-Exchange/05-11-SEHK-StampDuty-TradingHour_e.pdf
static HKEX_PREOPEN_MATCH_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 20 * 60,
    close_ssm: 9 * 3600 + 30 * 60,
}];
static HKEX_ORDER_ENTRY_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 9 * 3600 + 20 * 60,
}];
// Pre-2011-03-07 POS ran 09:30–10:00 against a 10:00 morning open. No primary
// SEHK text for that era's period boundaries was located, so the whole window
// is left extended rather than guessing where its matching period began.
static HKEX_PREOPEN_OLD: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 10 * 3600,
}];
// CAS 16:00–16:10 ends in a randomised uncrossing that prints the closing
// trades, so the whole auction stays extended.
static HKEX_EXTENDED_CURRENT: &[SessionRule] = &[
    HKEX_PREOPEN_MATCH_CURRENT[0],
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
    order_entry: HKEX_ORDER_ENTRY_CURRENT,
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
    extended: HKEX_PREOPEN_MATCH_CURRENT,
    order_entry: HKEX_ORDER_ENTRY_CURRENT,
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
