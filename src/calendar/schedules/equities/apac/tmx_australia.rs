// SPDX-License-Identifier: MIT-0

//! TMX Australia cash equities.

use chrono_tz::Australia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static TMX_AU_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 10 * 3600,
    close_ssm: 16 * 3600 + 13 * 60,
}];
// Tradeable close-side and open-side windows. The Opening Auction uncrosses in
// a randomized 09:59:45–10:00:00 window, so a print is possible there. The
// close-side 16:00–16:20 window stays tradeable throughout: non-auction-eligible
// products keep trading continuously to 16:13, MOC matches at the ASX closing
// auction price around 16:10, the Closing Auction uncrosses 16:12:45–16:13, and
// @Last/MOC trading runs to 16:20.
static TMX_AU_EXTENDED_CURRENT: &[SessionRule] = &[
    // Pre-Open. The Trading System does not MATCH here, but the venue's hours
    // page states that during the pre-market period "trade reports may be
    // lodged in accordance with the Cboe Operating Rules and the Market
    // Integrity Rules" — reported trades print, so the window is tradeable and
    // is not order-entry-only.
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600,
        close_ssm: 9 * 3600 + 59 * 60 + 45,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 59 * 60 + 45,
        close_ssm: 10 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 16 * 3600 + 20 * 60,
    },
];

// Pre-auction eras carry no order-entry-only window: the only non-regular
// phases are @Last and MOC, both of which execute at the closing price.
static TMX_AU_EXTENDED_POST_2015: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 16 * 3600 + 12 * 60,
    close_ssm: 16 * 3600 + 20 * 60,
}];
static TMX_AU_EXTENDED_POST_2013: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 16 * 3600 + 13 * 60,
    close_ssm: 16 * 3600 + 20 * 60,
}];

// Current TMX Australia venue envelope: continuous trading 10:00–16:13;
// auction-eligible products accept orders 07:00–10:00 and 16:00–16:13,
// while @Last/MOC trading continues through 16:20. Product-class overlap is
// represented by overlapping regular/extended rules.
// https://www.tmxaustralia.com/about/hours
// https://cdn.cboe.com/resources/au/tmx/participant_resources/Operating_Rules_Procedures_Clean.pdf
pub(crate) static TMX_AU_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Australia::Sydney,
    regular: TMX_AU_REGULAR,
    extended: TMX_AU_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// ASIC dates Chi-X Australia's launch to 2011-10-31. MOC actually launched
// 2013-12-09 after the 2013-11-25 release was rolled back; @Last moved the
// close-side open to 16:12 on 2015-08-31; auctions launched 2025-03-17.
// https://www.asic.gov.au/about-asic/news-centre/find-a-media-release/2012-releases/12-295mr-asic-releases-first-chi-x-assessment-report/
// https://cdn.cboe.com/resources/compliance_notice/Compliance-Notice-0008-13.pdf
// https://cdn.cboe.com/resources/compliance_notice/Compliance-Notice-0009-13.pdf
// https://cdn.cboe.com/resources/compliance_notice/Compliance-Notice-0006-15.pdf
// https://cdn.cboe.com/resources/technical_notice/Technical-Notice-0003-25.pdf
pub(crate) static TMX_AU_PROFILE_POST_2015_08_31: StaticHoursProfile = StaticHoursProfile {
    tz: Australia::Sydney,
    regular: TMX_AU_REGULAR,
    extended: TMX_AU_EXTENDED_POST_2015,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static TMX_AU_PROFILE_POST_2013_12_09: StaticHoursProfile = StaticHoursProfile {
    tz: Australia::Sydney,
    regular: TMX_AU_REGULAR,
    extended: TMX_AU_EXTENDED_POST_2013,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static TMX_AU_PROFILE_LAUNCH: StaticHoursProfile = StaticHoursProfile {
    tz: Australia::Sydney,
    regular: TMX_AU_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static TMX_AU_PROFILE_CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: Australia::Sydney,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &TMX_AU_PROFILE_CURRENT;

static REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2011, 10, 31),
        profile: &TMX_AU_PROFILE_LAUNCH,
    },
    Revision {
        effective: effective_date(2013, 12, 9),
        profile: &TMX_AU_PROFILE_POST_2013_12_09,
    },
    Revision {
        effective: effective_date(2015, 8, 31),
        profile: &TMX_AU_PROFILE_POST_2015_08_31,
    },
    Revision {
        effective: effective_date(2025, 3, 17),
        profile: &TMX_AU_PROFILE_CURRENT,
    },
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &TMX_AU_PROFILE_CLOSED,
        REVISIONS,
    )
}
