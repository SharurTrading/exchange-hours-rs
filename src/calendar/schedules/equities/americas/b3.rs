// SPDX-License-Identifier: MIT-0

//! B3 cash-equity grids, including its New York reference-zone regime.

use chrono_tz::America;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static B3_REGULAR_SHORT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 10 * 3600,
    close_ssm: 16 * 3600 + 55 * 60,
}];
static B3_REGULAR_INTERIM: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 10 * 3600,
    close_ssm: 17 * 3600 + 25 * 60,
}];
static B3_REGULAR_OLD_SHORT: &[SessionRule] = B3_REGULAR_SHORT;
static B3_REGULAR_OLD_LONG: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 11 * 3600,
    close_ssm: 17 * 3600 + 55 * 60,
}];
static B3_REGULAR_LONG: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 10 * 3600,
    close_ssm: 17 * 3600 + 55 * 60,
}];
static B3_EXTENDED_SHORT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 45 * 60,
        close_ssm: 10 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 55 * 60,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 18 * 3600,
    },
];
static B3_EXTENDED_LONG: &[SessionRule] = &[
    B3_EXTENDED_SHORT[0],
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 55 * 60,
        close_ssm: 18 * 3600,
    },
];
static B3_EXTENDED_INTERIM: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 45 * 60,
        close_ssm: 10 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 25 * 60,
        close_ssm: 17 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 18 * 3600,
        close_ssm: 19 * 3600 + 30 * 60,
    },
];
static B3_EXTENDED_OLD_SHORT: &[SessionRule] = &[
    B3_EXTENDED_SHORT[0],
    B3_EXTENDED_SHORT[1],
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 19 * 3600,
    },
];
static B3_EXTENDED_OLD_LONG: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 10 * 3600 + 45 * 60,
        close_ssm: 11 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 55 * 60,
        close_ssm: 18 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 18 * 3600 + 30 * 60,
        close_ssm: 19 * 3600 + 30 * 60,
    },
];

// B3's 2010–2012 cash market alternated explicit old short and long grids.
// Closing calls are extended here, and adjacent after-market pre-open/trading
// phases are merged into one order-capable envelope.
// Sources (January-2010 baseline and every observable switch):
// https://www.b3.com.br/data/files/5F/52/26/49/AF0B25107399EA25790D8AA8/062-2009DP.pdf
// https://www.b3.com.br/data/files/E4/15/1D/13/BF0B25107399EA25790D8AA8/009-2010DP.pdf
// https://www.b3.com.br/data/files/0A/61/12/46/DF0B25107399EA25790D8AA8/002-2010DO.pdf
// https://www.b3.com.br/data/files/40/47/EE/5E/EF0B25107399EA25790D8AA8/001-2011DO.pdf
// https://www.b3.com.br/data/files/F5/E5/DD/F8/101B25107399EA25790D8AA8/009-2011DO.pdf
// https://www.b3.com.br/data/files/47/96/AE/D8/301B25107399EA25790D8AA8/009-2012DP.pdf
// https://www.b3.com.br/data/files/DC/81/9B/44/601B25107399EA25790D8AA8/005-2012DO.pdf
pub(crate) static B3_PROFILE_OLD_SHORT: StaticHoursProfile = StaticHoursProfile {
    tz: America::Sao_Paulo,
    regular: B3_REGULAR_OLD_SHORT,
    extended: B3_EXTENDED_OLD_SHORT,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static B3_PROFILE_OLD_LONG: StaticHoursProfile = StaticHoursProfile {
    tz: America::Sao_Paulo,
    regular: B3_REGULAR_OLD_LONG,
    extended: B3_EXTENDED_OLD_LONG,
    has_daily_close: true,
    has_weekend_close: true,
};

// OC066/2012-DP, effective 2012-12-03, moved cash equities to a 09:45
// pre-open, 10:00–17:25 continuous phase, 17:25–17:30 closing call, and
// 18:00–19:30 after-market order-entry/trading envelope.
// https://www.b3.com.br/data/files/59/C6/12/A0/701B25107399EA25790D8AA8/066-2012DP.pdf
pub(crate) static B3_PROFILE_INTERIM: StaticHoursProfile = StaticHoursProfile {
    tz: America::Sao_Paulo,
    regular: B3_REGULAR_INTERIM,
    extended: B3_EXTENDED_INTERIM,
    has_daily_close: true,
    has_weekend_close: true,
};

// B3's cash-equity grid has a short day aligned with US daylight time and a
// long day otherwise. Cancellation-only phases (09:30–09:45 and, on the short
// grid, 17:25–17:30) are excluded. OC042/2013 pins the fixed short grid from
// 2013-07-08; the 2015 circular publishes both recurring grids. Current
// circulars 043/2025-VNC and 005/2026-PRE pin the two forms today. Selection
// happens in `profile_at`; the rules themselves remain São Paulo time.
// Sources:
// https://www.b3.com.br/data/files/A6/C0/2C/BB/901B25107399EA25790D8AA8/042-2013DP.pdf
// https://www.b3.com.br/data/files/CF/31/79/3D/611B25107399EA25790D8AA8/127-2015DP.pdf
// https://www.b3.com.br/data/files/72/D7/AD/2E/5FD715107623A41592D828A8/OC.127-2015-Horarios-de-Negociacao-Regular-Segmento-BOVESPA.pdf
// https://www.b3.com.br/data/files/AE/C7/86/2E/5FD715107623A41592D828A8/OC.127-2015-Horarios-de-Negociacao-Verao-Segmento-BOVESPA.pdf
// https://www.b3.com.br/pt_br/noticias/horarios-de-negociacao.htm
// https://www.b3.com.br/data/files/AE/22/40/4A/1131A910F51990A9AC094EA8/CL%20043-2025-VNC%20NOVOS%20HORARIOS%20DE%20NEGOCIACAO_EN.pdf
// https://www.b3.com.br/data/files/54/23/16/15/EC09C910F37907C9AC094EA8/OC%20005-2026%20PRE%20NOVOS%20HORARIOS%20DE%20NEGOCIACAO_ING.pdf
pub(crate) static B3_PROFILE_SHORT: StaticHoursProfile = StaticHoursProfile {
    tz: America::Sao_Paulo,
    regular: B3_REGULAR_SHORT,
    extended: B3_EXTENDED_SHORT,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static B3_PROFILE_LONG: StaticHoursProfile = StaticHoursProfile {
    tz: America::Sao_Paulo,
    regular: B3_REGULAR_LONG,
    extended: B3_EXTENDED_LONG,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{
    Revision, effective_date, local_date, reference_delta_seconds, select_revision,
};

pub(crate) const CURRENT: &StaticHoursProfile = &B3_PROFILE_SHORT;

// Official circulars pin every old-grid switch from the January-2010
// baseline. Circular 127/2015-DP introduced the recurring pair from
// 2015-12-21 and tied it to the Brazil/New York daylight-time relationship.
// https://www.b3.com.br/data/files/CF/31/79/3D/611B25107399EA25790D8AA8/127-2015DP.pdf
static EXPLICIT_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2010, 3, 15),
        profile: &B3_PROFILE_OLD_SHORT,
    },
    Revision {
        effective: effective_date(2010, 10, 18),
        profile: &B3_PROFILE_OLD_LONG,
    },
    Revision {
        effective: effective_date(2011, 3, 14),
        profile: &B3_PROFILE_OLD_SHORT,
    },
    Revision {
        effective: effective_date(2011, 10, 17),
        profile: &B3_PROFILE_OLD_LONG,
    },
    Revision {
        effective: effective_date(2012, 3, 12),
        profile: &B3_PROFILE_OLD_SHORT,
    },
    Revision {
        effective: effective_date(2012, 12, 3),
        profile: &B3_PROFILE_INTERIM,
    },
    Revision {
        effective: effective_date(2013, 7, 8),
        profile: &B3_PROFILE_SHORT,
    },
];

const REFERENCE_GRID: chrono::NaiveDate = effective_date(2015, 12, 21);

/// Resolves B3's explicit old grids and its published recurring offset regime.
pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, America::Sao_Paulo);
    if day < REFERENCE_GRID {
        return select_revision(day, &B3_PROFILE_OLD_LONG, EXPLICIT_REVISIONS);
    }
    if reference_delta_seconds(as_of, America::Sao_Paulo, America::New_York) == -3600 {
        &B3_PROFILE_SHORT
    } else {
        &B3_PROFILE_LONG
    }
}
