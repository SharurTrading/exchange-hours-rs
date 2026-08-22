// SPDX-License-Identifier: MIT-0

//! Euronext cash equities.
//!
//! The profiles represent the principal continuous-trading share segment.
//! Euronext publishes one Central-European clock; Lisbon and Dublin rules are
//! shifted one civil-clock hour while retaining their venue IANA zones.

mod dublin;
mod milan;

use chrono_tz::{Europe, Tz};

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

pub(crate) use dublin::{EURONEXT_DUB_PROFILE, profile_at as dublin_profile_at};
pub(crate) use milan::{EURONEXT_MIL_PROFILE, profile_at as milan_profile_at};

// The operator's 2010 special-day appendix and 2014 normal-hours appendix show
// the legacy 07:15 CET pre-opening and the principal-share opening at 09:00.
// Euronext notice PAR_20150924_07448_EUR documents zero-to-30-second randomized
// uncrosses for Belgian, Dutch, French, and Portuguese trading groups. Those
// instrument-level micro-events do not define one exchange-wide transition instant,
// so this exchange-level profile retains the published nominal boundaries:
// continuous trading starts at 09:00, the closing auction ends at 17:35, and
// Trading-at-Last then runs to 17:40.
// https://www.euronext.com/sites/default/files/european_cash_markets_trading_hours_for_24th_and_31st_december_2010.pdf
// https://connect.euronext.com/nl/listview/notice-download?attachmentId=201416&id=581906&type=PDF
// https://live.euronext.com/en/listview/notice-download?id=598779&type=PDF&attachmentId=218289
// https://live.euronext.com/en/listview/notice-download?id=598933&type=PDF&attachmentId=218443
static CENTRAL_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 17 * 3600 + 30 * 60,
}];
static CENTRAL_LEGACY_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 15 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 35 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 35 * 60,
        close_ssm: 17 * 3600 + 40 * 60,
    },
];
static CENTRAL_CURRENT_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 30 * 60,
        close_ssm: 9 * 3600,
    },
    // Nominal closing auction.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 30 * 60,
        close_ssm: 17 * 3600 + 35 * 60,
    },
    // Trading-at-Last.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 35 * 60,
        close_ssm: 17 * 3600 + 40 * 60,
    },
];

static LISBON_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 16 * 3600 + 30 * 60,
}];
static LISBON_LEGACY_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 6 * 3600 + 15 * 60,
        close_ssm: 8 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 35 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 35 * 60,
        close_ssm: 16 * 3600 + 40 * 60,
    },
];
static LISBON_CURRENT_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 6 * 3600 + 30 * 60,
        close_ssm: 8 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 30 * 60,
        close_ssm: 16 * 3600 + 35 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 35 * 60,
        close_ssm: 16 * 3600 + 40 * 60,
    },
];

// The current 4-01/4-03 trading appendix confirms pre-opening at 07:30 CET,
// nominal continuous trading from 09:00 through 17:30, a closing auction to
// 17:35, and Trading-at-Last through 17:40 for principal shares. Its randomized
// zero-to-30-second uncross timing is per security and is outside the scope of
// these exchange-level boundaries.
// https://www.euronext.com/sites/default/files/2026-07/appendix%20to%20Euronext%20Instructions%204-01%204-03%20Trading%20Manuals_0.xlsx

macro_rules! profile {
    ($name:ident, $tz:expr, $regular:ident, $extended:ident) => {
        static $name: StaticHoursProfile = StaticHoursProfile {
            tz: $tz,
            regular: $regular,
            extended: $extended,
            has_daily_close: true,
            has_weekend_close: true,
        };
    };
}

profile!(
    PARIS_LEGACY,
    Europe::Paris,
    CENTRAL_REGULAR,
    CENTRAL_LEGACY_EXTENDED
);
profile!(
    AMSTERDAM_LEGACY,
    Europe::Amsterdam,
    CENTRAL_REGULAR,
    CENTRAL_LEGACY_EXTENDED
);
profile!(
    BRUSSELS_LEGACY,
    Europe::Brussels,
    CENTRAL_REGULAR,
    CENTRAL_LEGACY_EXTENDED
);
profile!(
    LISBON_LEGACY,
    Europe::Lisbon,
    LISBON_REGULAR,
    LISBON_LEGACY_EXTENDED
);

pub(crate) static EURONEXT_PARIS_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Paris,
    regular: CENTRAL_REGULAR,
    extended: CENTRAL_CURRENT_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static EURONEXT_AMS_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Amsterdam,
    regular: CENTRAL_REGULAR,
    extended: CENTRAL_CURRENT_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static EURONEXT_BRU_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Brussels,
    regular: CENTRAL_REGULAR,
    extended: CENTRAL_CURRENT_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static EURONEXT_LIS_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Lisbon,
    regular: LISBON_REGULAR,
    extended: LISBON_CURRENT_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

// The phase-one timetable made the legacy Euronext pre-opening change from
// 07:15 to 07:30 CET effective 2023-03-20. The same operator go-live document
// distinguishes that legacy-markets date from Milan's 2023-03-27 migration.
// https://connect.euronext.com/sites/default/files/it-documentation/Guide%20to%20Trading%20System%20-%20Borsa%20Italiana%20Migration%20to%20Optiq%20-%20Functional%20Changes%20v.2.0.pdf
// https://connect.euronext.com/sites/default/files/it-documentation/Go-Live%20Weekend%20Guidelines%20-%20Borsa%20Italiana%20Optiq%20Migration.pdf
fn phase_one_profile(
    as_of: chrono::DateTime<chrono::Utc>,
    tz: Tz,
    baseline: &'static StaticHoursProfile,
    current: &'static StaticHoursProfile,
) -> &'static StaticHoursProfile {
    let revisions = [Revision {
        effective: effective_date(2023, 3, 20),
        profile: current,
    }];
    select_revision(local_date(as_of, tz), baseline, &revisions)
}

pub(crate) fn paris_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    phase_one_profile(as_of, Europe::Paris, &PARIS_LEGACY, &EURONEXT_PARIS_PROFILE)
}

pub(crate) fn amsterdam_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    phase_one_profile(
        as_of,
        Europe::Amsterdam,
        &AMSTERDAM_LEGACY,
        &EURONEXT_AMS_PROFILE,
    )
}

pub(crate) fn brussels_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    phase_one_profile(
        as_of,
        Europe::Brussels,
        &BRUSSELS_LEGACY,
        &EURONEXT_BRU_PROFILE,
    )
}

pub(crate) fn lisbon_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    phase_one_profile(as_of, Europe::Lisbon, &LISBON_LEGACY, &EURONEXT_LIS_PROFILE)
}
