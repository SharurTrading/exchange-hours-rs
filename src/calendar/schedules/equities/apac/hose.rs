// SPDX-License-Identifier: MIT-0

//! Ho Chi Minh Stock Exchange cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// HOSE venue envelope: opening call 09:00–09:15, continuous
// 09:15–11:30/13:00–14:30, closing call 14:30–14:45, then negotiated
// put-through trading through 15:00. Put-through is extended by convention;
// not every security is eligible for every phase. The current table explicitly
// prints the 13:00–15:00 put-through window; HOSE's 2013 annual report dates
// the 45-minute extension to 2013-07-22.
// https://staticfile.hsx.vn/Uploads/UploadDocuments/2372209/2.Trading%20hours.pdf
// https://web.archive.org/web/20140501225025id_/http://www.hsx.vn:80/hsx_en/Modules/annual/annual_files/BCTN-ANNUAL%20REPORT%202013.pdf
static HOSE_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 15 * 60,
        close_ssm: 11 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600,
        close_ssm: 14 * 3600 + 30 * 60,
    },
];
static HOSE_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 9 * 3600 + 15 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 30 * 60,
        close_ssm: 14 * 3600 + 45 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 45 * 60,
        close_ssm: 15 * 3600,
    },
];
pub(crate) static HOSE_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Ho_Chi_Minh,
    regular: HOSE_REGULAR_CURRENT,
    extended: HOSE_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// HOSE's 2012 annual report prints the complete pilot schedule and dates its
// start to 2012-03-05: continuous II ends 13:45, the closing call ends 14:00,
// and negotiated put-through trading remains available through 14:15.
// https://staticfile.hsx.vn/Uploads/Annual/6dfe6cf6-93b2-4871-966f-2bb9bb92c110/10dd075f-c751-46d2-b598-022850e517f6
static HOSE_REGULAR_2012: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 15 * 60,
        close_ssm: 11 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600,
        close_ssm: 13 * 3600 + 45 * 60,
    },
];
static HOSE_EXTENDED_2012: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 9 * 3600 + 15 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 45 * 60,
        close_ssm: 14 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600,
        close_ssm: 14 * 3600 + 15 * 60,
    },
];
pub(crate) static HOSE_PROFILE_2012: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Ho_Chi_Minh,
    regular: HOSE_REGULAR_2012,
    extended: HOSE_EXTENDED_2012,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// HOSE's archived operator notice makes this grid effective 2010-09-13. The
// notice prints negotiated trading through the 11:00 market close; HOSE's 2010
// annual report independently confirms negotiation throughout the extended
// session from that day.
// https://web.archive.org/web/20100830155813id_/http://www.hsx.vn/hsx/Modules/News/NewsDetail.aspx?id=48784
// https://staticfile.hsx.vn/Uploads/Annual/20326c45-3ba9-4fe4-89c3-fe16f9777467/10dd075f-c751-46d2-b598-022850e517f6
static HOSE_REGULAR_2010_09_13: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 45 * 60,
    close_ssm: 10 * 3600 + 30 * 60,
}];
static HOSE_EXTENDED_2010_09_13: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30 * 60,
        close_ssm: 8 * 3600 + 45 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 10 * 3600 + 30 * 60,
        close_ssm: 10 * 3600 + 45 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 10 * 3600 + 45 * 60,
        close_ssm: 11 * 3600,
    },
];
static HOSE_PROFILE_2010_09_13: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Ho_Chi_Minh,
    regular: HOSE_REGULAR_2010_09_13,
    extended: HOSE_EXTENDED_2010_09_13,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// HOSE's own trading-hours PDF, archived on 2010-02-15, supplies the exact
// January-2010 audit-floor grid, including 10:30–11:00 put-through trading.
// https://web.archive.org/web/20100215053559id_/http://www.hsx.vn:80/hsx/Uploaded/quy_dinh_file/2.Thoi%20gian%20giao%20dich..pdf
static HOSE_REGULAR_AT_2010_FLOOR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 10 * 3600 + 15 * 60,
}];
static HOSE_EXTENDED_AT_2010_FLOOR: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 10 * 3600 + 15 * 60,
        close_ssm: 10 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 10 * 3600 + 30 * 60,
        close_ssm: 11 * 3600,
    },
];
static HOSE_PROFILE_AT_2010_FLOOR: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Ho_Chi_Minh,
    regular: HOSE_REGULAR_AT_2010_FLOOR,
    extended: HOSE_EXTENDED_AT_2010_FLOOR,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &HOSE_PROFILE_CURRENT;

// Revision evidence — each row's day-level effective date and the primary
// source that states it (full quotations sit in the blocks above):
//   2010-09-13 "HOSE news notice 48784"
//     https://web.archive.org/web/20100830155813id_/http://www.hsx.vn/hsx/Modules/News/NewsDetail.aspx?id=48784
//   2012-03-05 "HOSE 2012 annual report"
//     https://staticfile.hsx.vn/Uploads/Annual/6dfe6cf6-93b2-4871-966f-2bb9bb92c110/10dd075f-c751-46d2-b598-022850e517f6
//   2013-07-22 "HOSE 2013 annual report"
//     https://web.archive.org/web/20140501225025id_/http://www.hsx.vn:80/hsx_en/Modules/annual/annual_files/BCTN-ANNUAL%20REPORT%202013.pdf
static REVISIONS: &[Revision] = revisions![
    (
        2010,
        9,
        13,
        &HOSE_PROFILE_2010_09_13,
        "HOSE news notice 48784"
    ),
    (2012, 3, 5, &HOSE_PROFILE_2012, "HOSE 2012 annual report"),
    (
        2013,
        7,
        22,
        &HOSE_PROFILE_CURRENT,
        "HOSE 2013 annual report"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &HOSE_PROFILE_AT_2010_FLOOR,
        REVISIONS,
    )
}
