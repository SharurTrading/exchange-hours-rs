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
// The 08:45–09:00 pre-opening is an input phase followed by a matching phase,
// and JATS only allocates transactions in the tail. IDX's trading-hours table
// under Board decree II-A Kep-00196/BEI/12-2024 prints "Pre opening (Input)
// 08.45.00 – 08.57.59" and "Pre opening (Matching) 08.58.00 – 08.59.59"; the
// earlier rulebook (Kep-00061/BEI/07-2021) prints "pukul 08.45.00 sampai dengan
// 08.55.00 digunakan oleh Anggota Bursa Efek untuk memasukkan penawaran jual
// dan/atau permintaan beli" followed by matching from 08.55.01. Nothing else on
// the venue is open: the Cash and Negotiated Markets both start at 09.00.00.
//
// The boundary is pinned at 08:55, the earliest matching start IDX has ever
// documented, so no second in which a trade could print is marked order entry
// under any regime this profile family spans.
// https://www.idx.id/en/products-services/trading-hours-and-mechanism/
// https://web.archive.org/web/20221220175625/https://www.idx.co.id/media/10022/peraturan_ii_a_perdagangan_efek_bersifat_ekuitas.pdf
static IDX_ORDER_ENTRY_PREOPEN: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 45 * 60,
    close_ssm: 8 * 3600 + 55 * 60,
}];
static IDX_PREOPEN_MATCH: SessionRule = SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 55 * 60,
    close_ssm: 9 * 3600,
};

// The close-side window stays `extended` in full. Pre-closing 15:50–16:00 is an
// input phase for the Regular Market, but the Negotiated Market runs
// continuously to 16:30 (Kep-00061/BEI/07-2021 clause IV.4.1.2), so negotiated
// trades print throughout it; pre-closing then matches, and post-trading
// 16:02–16:15 matches continuously at the closing price.
static IDX_EXTENDED_CURRENT: &[SessionRule] = &[
    IDX_PREOPEN_MATCH,
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 50 * 60,
        close_ssm: 16 * 3600 + 30 * 60,
    },
];
static IDX_EXTENDED_PRE_PANDEMIC: &[SessionRule] = &[
    IDX_EXTENDED_CURRENT[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600 + 50 * 60,
        close_ssm: 16 * 3600 + 15 * 60,
    },
];
// The restored regular-market closing phases end at 16:15, while the
// exchange's Negotiated Market remains available through 16:30. The combined
// venue envelope is extended through the latter boundary.
// https://www.idx.id/en/products-services/trading-hours-and-mechanism/
pub(crate) static IDX_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Jakarta,
    regular: IDX_REGULAR_CURRENT,
    extended: IDX_EXTENDED_CURRENT,
    order_entry: IDX_ORDER_ENTRY_PREOPEN,
    has_daily_close: true,
    has_weekend_close: true,
};
static IDX_PROFILE_PRE_PANDEMIC: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Jakarta,
    regular: IDX_REGULAR_CURRENT,
    extended: IDX_EXTENDED_PRE_PANDEMIC,
    order_entry: IDX_ORDER_ENTRY_PREOPEN,
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
    order_entry: &[],
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
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &IDX_PROFILE_CURRENT;

static REVISIONS: &[Revision] = revisions![
    (
        2013,
        1,
        2,
        &IDX_PROFILE_PRE_PANDEMIC,
        "IDX 2013 annual report"
    ),
    (
        2020,
        3,
        30,
        &IDX_PROFILE_PANDEMIC,
        "OJK Siaran Pers Perubahan Jam Perdagangan"
    ),
    (
        2023,
        4,
        3,
        &IDX_PROFILE_CURRENT,
        "IDX ikhtisar dan sejarah BEI"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &IDX_PROFILE_PRE_2013_01_02,
        REVISIONS,
    )
}
