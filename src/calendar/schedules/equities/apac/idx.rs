// SPDX-License-Identifier: MIT-0

//! Indonesia Stock Exchange cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::{FRI, MON_FRI, MON_THU};
static IDX_REGULAR_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_THU,
        open_ssm: 9 * 3600,
        close_ssm: 12 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 13 * 3600 + 30 * 60,
        close_ssm: 15 * 3600 + 50 * 60,
    },
    SessionRule {
        days: FRI,
        open_ssm: 9 * 3600,
        close_ssm: 11 * 3600 + 30 * 60,
    },
    SessionRule {
        days: FRI,
        open_ssm: 14 * 3600,
        close_ssm: 15 * 3600 + 50 * 60,
    },
];
static IDX_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 45 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 50 * 60,
        close_ssm: 16 * 3600 + 15 * 60,
    },
];
// https://www.idx.id/en/products-services/trading-hours-and-mechanism/
pub(crate) static IDX_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Jakarta,
    regular: IDX_REGULAR_CURRENT,
    extended: IDX_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// IDX before 2013: the official annual report dates the 2013 extension and
// prints the prior continuous sessions. An archived copy of IDX's own 2010
// trading-hours page prints 09:10–09:25 order input followed by price forming
// and allocation through 09:29:59; the static profile uses their contiguous
// nominal 09:10–09:30 envelope.
// https://www.idx.co.id/Media/1208/2013.pdf
// https://web.archive.org/web/20100831234522id_/http://www.idx.co.id/MainMenu/Trading/JamPerdagangan/tabid/214/lang/en-US/language/en-US/Default.aspx
static IDX_REGULAR_PRE_2013: &[SessionRule] = &[
    SessionRule {
        days: MON_THU,
        open_ssm: 9 * 3600 + 30 * 60,
        close_ssm: 12 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 13 * 3600 + 30 * 60,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: FRI,
        open_ssm: 9 * 3600 + 30 * 60,
        close_ssm: 11 * 3600 + 30 * 60,
    },
    SessionRule {
        days: FRI,
        open_ssm: 14 * 3600,
        close_ssm: 16 * 3600,
    },
];
static IDX_EXTENDED_PRE_2013: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 10 * 60,
    close_ssm: 9 * 3600 + 30 * 60,
}];
pub(crate) static IDX_PROFILE_PRE_2013_01_02: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Jakarta,
    regular: IDX_REGULAR_PRE_2013,
    extended: IDX_EXTENDED_PRE_2013,
    has_daily_close: true,
    has_weekend_close: true,
};

// OJK shortened all weekday sessions effective 2020-03-30; IDX restored its
// pre-pandemic table on 2023-04-03.
// https://ojk.go.id/id/berita-dan-kegiatan/info-terkini/Pages/Siaran-Pers-Perubahan-Jam-Perdagangan-di-Bursa-Efek.aspx
// https://www.idx.id/id/tentang-bei/ikhtisar-dan-sejarah-bei
static IDX_REGULAR_PANDEMIC: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 11 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600 + 30 * 60,
        close_ssm: 14 * 3600 + 50 * 60,
    },
];
static IDX_EXTENDED_PANDEMIC: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 45 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 50 * 60,
        close_ssm: 15 * 3600 + 15 * 60,
    },
];
pub(crate) static IDX_PROFILE_PANDEMIC: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Jakarta,
    regular: IDX_REGULAR_PANDEMIC,
    extended: IDX_EXTENDED_PANDEMIC,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &IDX_PROFILE_CURRENT;

static REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2013, 1, 2),
        profile: &IDX_PROFILE_CURRENT,
    },
    Revision {
        effective: effective_date(2020, 3, 30),
        profile: &IDX_PROFILE_PANDEMIC,
    },
    Revision {
        effective: effective_date(2023, 4, 3),
        profile: &IDX_PROFILE_CURRENT,
    },
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &IDX_PROFILE_PRE_2013_01_02,
        REVISIONS,
    )
}
