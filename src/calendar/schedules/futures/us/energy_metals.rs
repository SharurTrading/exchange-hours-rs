// SPDX-License-Identifier: MIT-0

//! NYMEX energy/PGM and COMEX gold/silver/copper futures schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// This family covers the shared grid used by NYMEX CL/MCL/QM, NG/MNG/QG,
// HO/RB/BZ, and PL/PA plus COMEX GC/MGC, SI/SIL, and HG/MHG. Platinum and
// palladium are NYMEX products, not COMEX products. At the January-2010 audit
// floor these families opened 17:00 CT and closed 16:15 CT. CME's 2015 Globex
// notice moved every COMEX and NYMEX close to 16:00 CT for Monday 2015-09-21
// while leaving opens unchanged, so a separate metals clock would duplicate
// both the current grid and the in-scope history. Current CME material also
// publishes Sunday 16:00-17:00 and Monday-Thursday 16:45-17:00 Pre-Open. Its
// correction calls both queues a long-term practice without giving their
// unconditional onset days. A 2010 notice observes that the weekday queue was
// already 16:45 but likewise supplies no onset. The fixed-current table
// includes both sourced current queues; dated profiles retain matching only.
// The revision is keyed to Sunday 2015-09-20, the local opening day of that
// Monday trade-date session, so a wrapped rule gives Monday the sourced close.
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20090130.html
// https://www.cmegroup.com/trading/metals/files/MT-027_GoldFuturesVsETFCheatSheet_r3.pdf
// https://www.cmegroup.com/tools-information/lookups/advisories/market-regulation/SER-5391.html
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150907.html
// https://www.cmegroup.com/markets/metals/precious/gold.contractSpecs.html
// https://www.cmegroup.com/markets/energy/crude-oil/light-sweet-crude.contractSpecs.html
// https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf
static ENERGY_METALS_EXTENDED_AT_2010_FLOOR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600 + 15 * 60,
}];
static ENERGY_METALS_MATCHING_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];
pub(crate) static ENERGY_METALS_EXTENDED_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: SUN_PLUS_MON_THU,
        open_ssm: 17 * 3600,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600,
    },
];

static ENERGY_METALS_AT_2010_FLOOR: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: ENERGY_METALS_EXTENDED_AT_2010_FLOOR,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static ENERGY_METALS_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: ENERGY_METALS_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};
static ENERGY_METALS_DATED_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: ENERGY_METALS_MATCHING_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static ENERGY_METALS_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2015, 9, 20),
    profile: &ENERGY_METALS_DATED_CURRENT,
}];

pub(crate) fn energy_metals_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, US::Central),
        &ENERGY_METALS_AT_2010_FLOOR,
        ENERGY_METALS_REVISIONS,
    )
}
