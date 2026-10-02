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
// Order entry only. B3's own timetable annexes list "Pre-Opening" as a phase
// distinct from "Trading": the 15 minutes before the open collect and price a
// book that does not match, and the first print is the opening call at the
// start of continuous trading. The immediately preceding "Order Cancellation"
// window (09:30–09:45) stays excluded from the model entirely.
// https://www.b3.com.br/data/files/AE/22/40/4A/1131A910F51990A9AC094EA8/CL%20043-2025-VNC%20NOVOS%20HORARIOS%20DE%20NEGOCIACAO_EN.pdf
static B3_ORDER_ENTRY_0945: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 45 * 60,
    close_ssm: 10 * 3600,
}];
// The 11:00-open grids carry no pre-opening phase. Narrative:
// docs/evidence/b3.md.
static B3_EXTENDED_SHORT: &[SessionRule] = &[
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
static B3_EXTENDED_LONG: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 17 * 3600 + 55 * 60,
    close_ssm: 18 * 3600,
}];
static B3_EXTENDED_INTERIM: &[SessionRule] = &[
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
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 19 * 3600,
    },
];
static B3_EXTENDED_OLD_LONG: &[SessionRule] = &[
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

// B3's 2010–2012 cash market alternated explicit old short and long grids. Narrative:
// docs/evidence/b3.md.
pub(crate) static B3_PROFILE_OLD_SHORT: StaticHoursProfile = StaticHoursProfile {
    tz: America::Sao_Paulo,
    regular: B3_REGULAR_OLD_SHORT,
    extended: B3_EXTENDED_OLD_SHORT,
    order_entry: B3_ORDER_ENTRY_0945,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static B3_PROFILE_OLD_LONG: StaticHoursProfile = StaticHoursProfile {
    tz: America::Sao_Paulo,
    regular: B3_REGULAR_OLD_LONG,
    extended: B3_EXTENDED_OLD_LONG,
    order_entry: &[],
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
    order_entry: B3_ORDER_ENTRY_0945,
    has_daily_close: true,
    has_weekend_close: true,
};

// B3's cash-equity grid has a short day aligned with US daylight time and a long day otherwise. Narrative:
// docs/evidence/b3.md.
pub(crate) static B3_PROFILE_SHORT: StaticHoursProfile = StaticHoursProfile {
    tz: America::Sao_Paulo,
    regular: B3_REGULAR_SHORT,
    extended: B3_EXTENDED_SHORT,
    order_entry: B3_ORDER_ENTRY_0945,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static B3_PROFILE_LONG: StaticHoursProfile = StaticHoursProfile {
    tz: America::Sao_Paulo,
    regular: B3_REGULAR_LONG,
    extended: B3_EXTENDED_LONG,
    order_entry: B3_ORDER_ENTRY_0945,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{
    Revision, effective_date, local_date, reference_delta_seconds, revisions, select_revision,
};

// Official circulars pin every old-grid switch from the January-2010
// baseline. Circular 127/2015-DP introduced the recurring pair from
// 2015-12-21 and tied it to the Brazil/New York daylight-time relationship.
// https://www.b3.com.br/data/files/CF/31/79/3D/611B25107399EA25790D8AA8/127-2015DP.pdf
// Evidence: docs/evidence/b3.md
static EXPLICIT_REVISIONS: &[Revision] = revisions![
    (2010, 3, 15, &B3_PROFILE_OLD_SHORT, "B3 OC 009/2010-DP"),
    (2010, 10, 18, &B3_PROFILE_OLD_LONG, "B3 OC 002/2010-DO"),
    (2011, 3, 14, &B3_PROFILE_OLD_SHORT, "B3 OC 001/2011-DO"),
    (2011, 10, 17, &B3_PROFILE_OLD_LONG, "B3 OC 009/2011-DO"),
    (2012, 3, 12, &B3_PROFILE_OLD_SHORT, "B3 OC 009/2012-DP"),
    (2012, 12, 3, &B3_PROFILE_INTERIM, "B3 OC 066/2012-DP"),
    (2013, 7, 8, &B3_PROFILE_SHORT, "B3 OC 042/2013-DP"),
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
