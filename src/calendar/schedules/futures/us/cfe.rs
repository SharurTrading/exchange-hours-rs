// SPDX-License-Identifier: MIT-0

//! Cboe Futures Exchange VIX futures schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_FRI, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

const MON_THU: [bool; 7] = [true, true, true, true, false, false, false];
const SUNDAY: [bool; 7] = [false, false, false, false, false, false, true];

const fn profile(
    regular: &'static [SessionRule],
    extended: &'static [SessionRule],
    order_entry: &'static [SessionRule],
) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: US::Central,
        regular,
        extended,
        order_entry,
        has_daily_close: true,
        has_weekend_close: true,
    }
}

// CFE (VIX) — current schedule, effective 2021-12-06.
//
// RTH is 08:30–15:00 CT. ETH runs 15:00–16:00 and, from Sunday plus
// Monday–Thursday, 17:00–08:30. Order-entry queues run Sunday 16:00–17:00 and
// Monday–Thursday 16:45–17:00, with starts randomized through six seconds after
// the nominal boundary. Because each queue follows a closed/suspended period,
// the profile uses the conservative latest 16:00:06 and 16:45:06 edges. The
// change removed the former 15:15–15:30 queue and 15:00–15:15 RTH segment.
//
// Sources: Cboe notice C2021102603, effective 2021-12-06; rule certification
// CFE-2021-028 (all times in Chicago time).
// https://cdn.cboe.com/resources/regulation/rule_filings/pending/2021/21-028-VX-VXM-and-AMERIBOR-Trading-Hours.pdf
pub(crate) static CFE_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 15 * 3600,
}];
// ORDER-ENTRY CLASSIFICATION. The notice quoted above calls the Sunday
// 16:00-17:00 and Monday-Thursday 16:45-17:00 windows "order-entry queues":
// CFE accepts non-market orders that cannot execute until trading resumes at
// 17:00, so no trade can print inside them. They are `order_entry`. The
// 15:00-16:00 and 17:00-08:30 ETH windows match and stay in `extended`.
pub(crate) static CFE_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
];
pub(crate) static CFE_ORDER_ENTRY: &[SessionRule] = &[
    SessionRule {
        days: SUNDAY,
        open_ssm: 16 * 3600 + 6,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60 + 6,
        close_ssm: 17 * 3600,
    },
];
static CFE_PROFILE: StaticHoursProfile = profile(CFE_REGULAR, CFE_EXTENDED, CFE_ORDER_ENTRY);

static CFE_REGULAR_0830_1515: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 15 * 3600 + 15 * 60,
}];

// At the January-2010 audit floor, VX traded 08:30-15:15 CT. CFE then
// introduced a 07:20-08:30 extended session effective 2010-12-10 and moved
// that start to 07:00 effective 2011-09-26. The filings state both day-level
// effective dates and preserve the 08:30-15:15 regular session.
// https://cdn.cboe.com/resources/regulation/rule_filings/approved/2010/SR-CFE-2010-013.pdf
// https://cdn.cboe.com/resources/regulation/rule_filings/approved/2011/SR-CFE-2011-019.pdf
static CFE_PROFILE_AT_2010_FLOOR: StaticHoursProfile = profile(CFE_REGULAR_0830_1515, &[], &[]);
static CFE_EXT_2010_12_10: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 20 * 60,
    close_ssm: 8 * 3600 + 30 * 60,
}];
static CFE_PROFILE_2010_12_10: StaticHoursProfile =
    profile(CFE_REGULAR_0830_1515, CFE_EXT_2010_12_10, &[]);

// CFE expanded VX hours in two phases during 2013. Its announcement describes
// the predecessor 07:00–15:15 trading day and the exact phase shapes; Cboe's
// year-end retrospective records the actual launches as 2013-10-28 for the
// Monday–Thursday 15:30–16:15 session and 2013-11-04 for the move from a 07:00
// to a 02:00 morning open. IC13-041 pins those phases to 2013-10-28 and
// 2013-11-04 and publishes the 15:29–15:30 Monday–Thursday pre-open queue in
// both. RTH remained 08:30–15:15 throughout.
// https://ir.cboe.com/news/news-details/2013/CBOE-Futures-Exchange-Announces-Launch-Dates-For-VIX-Futures-Extended-Trading-Hours-09-30-2013/default.aspx
// https://ir.cboe.com/news/news-details/2014/2013-Trading-Volume-Reaches-New-All-Time-High-At-CBOE-Futures-Exchange-01-02-2014/default.aspx
// https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2013-041.pdf
// https://cdn.cboe.com/resources/regulation/rule_filings/approved/2013/SR-CFE-2013-032.pdf
// https://cdn.cboe.com/resources/regulation/rule_filings/approved/2013/SR-CFE-2013-034.pdf
static CFE_EXT_PRE_2013_10_28: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600,
    close_ssm: 8 * 3600 + 30 * 60,
}];
// IC13-041 publishes 15:29-15:30 as a Monday-Thursday "pre-open queue" ahead of
// the 15:30 session, so it accepts orders without matching and is `order_entry`.
static CFE_ORDER_ENTRY_1529: &[SessionRule] = &[SessionRule {
    days: MON_THU,
    open_ssm: 15 * 3600 + 29 * 60,
    close_ssm: 15 * 3600 + 30 * 60,
}];
static CFE_EXT_2013_10_28: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 15 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 15 * 60,
    },
];
static CFE_EXT_2013_11_04: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 2 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 15 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 15 * 60,
    },
];
static CFE_PROFILE_PRE_2013_10_28: StaticHoursProfile =
    profile(CFE_REGULAR_0830_1515, CFE_EXT_PRE_2013_10_28, &[]);
static CFE_PROFILE_2013_10_28: StaticHoursProfile = profile(
    CFE_REGULAR_0830_1515,
    CFE_EXT_2013_10_28,
    CFE_ORDER_ENTRY_1529,
);
static CFE_PROFILE_2013_11_04: StaticHoursProfile = profile(
    CFE_REGULAR_0830_1515,
    CFE_EXT_2013_11_04,
    CFE_ORDER_ENTRY_1529,
);

// CFE-2014-010 introduced nearly-24-hour VX trading on Sunday 2014-06-22.
// IC14-036 publishes the resulting 16:15–17:00 Sunday pre-open and retained
// 15:29–15:30 weekday pre-open; RG-CFE-2014-020 pins the launch to that Sunday.
// https://cdn.cboe.com/resources/regulation/rule_filings/approved/2014/SR-CFE-2014-010.pdf
// https://ir.cboe.com/news/news-details/2014/CBOE-Futures-Exchange-Set-For-June-22-Launch-Of-24-Hour-VIX-Futures-Trading-06-09-2014/default.aspx
// https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2014-036.pdf
// https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2014-020.pdf
static CFE_EXT_2014_06_22: &[SessionRule] = &[
    SessionRule {
        days: SUNDAY,
        open_ssm: 17 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 15 * 3600 + 30 * 60,
        close_ssm: 8 * 3600 + 30 * 60,
    },
];
// IC14-036 names both the Sunday 16:15-17:00 phase and the retained 15:29-15:30
// weekday phase as pre-opens: orders queue, nothing matches.
static CFE_ORDER_ENTRY_2014_06_22: &[SessionRule] = &[
    SessionRule {
        days: SUNDAY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 15 * 3600 + 29 * 60,
        close_ssm: 15 * 3600 + 30 * 60,
    },
];
static CFE_PROFILE_2014_06_22: StaticHoursProfile = profile(
    CFE_REGULAR_0830_1515,
    CFE_EXT_2014_06_22,
    CFE_ORDER_ENTRY_2014_06_22,
);

// SR-CFE-2017-017 tied a revised VX schedule to CFE's system migration:
// 08:30–15:15 RTH, a 15:15–15:30 order-entry-only queue, 15:30–16:00 ETH, a
// 16:00–16:45 weekday suspension, a 16:45–17:00 queue, then 17:00–08:30 ETH.
// Sunday has a 16:00–17:00 opening queue. The queues accept non-market orders
// that cannot execute until trading resumes, so the crate classifies them as
// extended under the order-entry-phase convention. The new-system opening
// queues begin at randomized instants through three seconds after the nominal
// boundary, so their conservative edges are 16:00:03 and 16:45:03. RG18-005
// confirms that the migration completed Sunday 2018-02-25, for business date
// Monday 2018-02-26.
// https://cdn.cboe.com/resources/regulation/rule_filings/approved/2017/SR-CFE-2017-017.pdf
// https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2018-005.pdf
// The filing above enumerates the migration-era phases separately: RTH ends at
// 15:15, 15:15-15:30 is an "order-entry-only queue", 15:30-16:00 is ETH, and
// the 16:00/16:45 opening queues run to the 17:00 ETH open. Only the two ETH
// windows match, so the 15:15-15:30 queue is split out of the former merged
// 15:15-16:00 rule and joins the evening queues in `order_entry`.
static CFE_EXT_2018_02_25: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 30 * 60,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
];
static CFE_ORDER_ENTRY_2018_02_25: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 15 * 60,
        close_ssm: 15 * 3600 + 30 * 60,
    },
    SessionRule {
        days: SUNDAY,
        open_ssm: 16 * 3600 + 3,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60 + 3,
        close_ssm: 17 * 3600,
    },
];
static CFE_PROFILE_2018_02_25: StaticHoursProfile = profile(
    CFE_REGULAR_0830_1515,
    CFE_EXT_2018_02_25,
    CFE_ORDER_ENTRY_2018_02_25,
);

// C2018071603 changed TAS queue commencement to a randomized instant three to
// six seconds after the nominal Sunday 16:00 and weekday 16:45 boundaries,
// effective with the Sunday 2018-08-12 opening. Non-TAS queues remained within
// zero to three seconds. The all-contract profile therefore advances its
// conservative latest edge from three to six seconds on that opening day.
// https://cdn.cboe.com/resources/release_notes/2018/Change-to-CFE-Pre-Open-Time-for-TAS-Contracts-and-Order-Submission-Commencement-Times.pdf
// Only the queue-commencement seconds change here, so the matching grid is the
// one CFE_EXT_2018_02_25 already carries.
static CFE_ORDER_ENTRY_2018_08_12: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 15 * 60,
        close_ssm: 15 * 3600 + 30 * 60,
    },
    SessionRule {
        days: SUNDAY,
        open_ssm: 16 * 3600 + 6,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60 + 6,
        close_ssm: 17 * 3600,
    },
];
static CFE_PROFILE_2018_08_12: StaticHoursProfile = profile(
    CFE_REGULAR_0830_1515,
    CFE_EXT_2018_02_25,
    CFE_ORDER_ENTRY_2018_08_12,
);

// Row evidence — each revision's day-level effective date and the primary
// source that states it (full quotations sit in the blocks above):
//   2010-12-10 "Cboe SR-CFE-2010-013"
//     https://cdn.cboe.com/resources/regulation/rule_filings/approved/2010/SR-CFE-2010-013.pdf
//   2011-09-26 "Cboe SR-CFE-2011-019"
//     https://cdn.cboe.com/resources/regulation/rule_filings/approved/2011/SR-CFE-2011-019.pdf
//   2013-10-28 "Cboe IC13-041" and 2013-11-04 "Cboe IC13-041"
//     https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2013-041.pdf
//   2014-06-22 "Cboe RG-CFE-2014-020"
//     https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2014-020.pdf
//   2018-02-25 "Cboe RG-CFE-2018-005"
//     https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2018-005.pdf
//   2018-08-12 "Cboe C2018071603"
//     https://cdn.cboe.com/resources/release_notes/2018/Change-to-CFE-Pre-Open-Time-for-TAS-Contracts-and-Order-Submission-Commencement-Times.pdf
//   2021-12-06 "Cboe C2021102603"
//     https://cdn.cboe.com/resources/regulation/rule_filings/pending/2021/21-028-VX-VXM-and-AMERIBOR-Trading-Hours.pdf
static CFE_REVISIONS: &[Revision] = revisions![
    (
        2010,
        12,
        10,
        &CFE_PROFILE_2010_12_10,
        "Cboe SR-CFE-2010-013"
    ),
    (
        2011,
        9,
        26,
        &CFE_PROFILE_PRE_2013_10_28,
        "Cboe SR-CFE-2011-019"
    ),
    (2013, 10, 28, &CFE_PROFILE_2013_10_28, "Cboe IC13-041"),
    (2013, 11, 4, &CFE_PROFILE_2013_11_04, "Cboe IC13-041"),
    (2014, 6, 22, &CFE_PROFILE_2014_06_22, "Cboe RG-CFE-2014-020"),
    // Sunday implementation; the revised weekday hours first occur on
    // CFE's Monday 2018-02-26 business date.
    (2018, 2, 25, &CFE_PROFILE_2018_02_25, "Cboe RG-CFE-2018-005"),
    (2018, 8, 12, &CFE_PROFILE_2018_08_12, "Cboe C2018071603"),
    (2021, 12, 6, &CFE_PROFILE, "Cboe C2021102603"),
];

pub(crate) fn cfe_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, US::Central),
        &CFE_PROFILE_AT_2010_FLOOR,
        CFE_REVISIONS,
    )
}
