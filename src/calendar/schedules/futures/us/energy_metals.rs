// SPDX-License-Identifier: MIT-0

//! NYMEX energy/PGM and COMEX gold/silver/copper futures schedules.

use chrono_tz::US;

use crate::calendar::SessionRule;
use crate::calendar::rule::{MON_THU, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// The shared grid used by NYMEX CL/MCL/QM, NG/MNG/QG, HO/RB/BZ and PL/PA plus
// COMEX GC/MGC, SI/SIL and HG/MHG. Platinum and palladium are NYMEX products,
// not COMEX products. The 2015 revision is keyed to Sunday 2015-09-20, the
// local opening day of that Monday trade-date session. See
// docs/evidence/comex.md, docs/evidence/nymex.md and
// docs/evidence/globex_energy.md.
static ENERGY_METALS_EXTENDED_AT_2010_FLOOR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600 + 15 * 60,
}];
pub(crate) static ENERGY_METALS_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];
// ORDER-ENTRY CLASSIFICATION. The two phases the comment above cites are named
// "Pre-Open" by CME: the book queues orders until the 17:00 Globex open and no
// trade can match inside them, so they are `order_entry` rather than a
// tradeable extended session. The 17:00-16:00 electronic session is matching
// and stays in `extended`.
pub(crate) static ENERGY_METALS_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600,
        close_ssm: 17 * 3600,
    },
    SessionRule {
        days: MON_THU,
        open_ssm: 16 * 3600 + 45 * 60,
        close_ssm: 17 * 3600,
    },
];

// DATED QUEUES, CARRIED BACK AS THE SOURCED INTERSECTION. Sunday 16:15-17:00
// and Monday-Thursday 16:45-17:00 hold under every sourced state, so carrying
// them from the January-2010 floor asserts no cutover; the knowledge-bound row
// below adds the disputed Sunday 16:00-16:15 quarter-hour. See
// docs/evidence/comex.md.
static ENERGY_METALS_ORDER_ENTRY_DATED: &[SessionRule] = &[
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600,
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
    order_entry: ENERGY_METALS_ORDER_ENTRY_DATED,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static ENERGY_METALS_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: ENERGY_METALS_EXTENDED_CURRENT,
    order_entry: ENERGY_METALS_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};
static ENERGY_METALS_DATED_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: ENERGY_METALS_EXTENDED_CURRENT,
    order_entry: ENERGY_METALS_ORDER_ENTRY_DATED,
    has_daily_close: true,
    has_weekend_close: true,
};

// Evidence: docs/evidence/comex.md, docs/evidence/nymex.md,
// docs/evidence/globex_energy.md
static ENERGY_METALS_REVISIONS: &[Revision] = revisions![
    // 2015-09-20 — T1 — CME Globex notice 20150907 — every COMEX and NYMEX close
    // moves to 16:00 CT for trade date Monday 2015-09-21.
    (
        2015,
        9,
        20,
        &ENERGY_METALS_DATED_CURRENT,
        "CME Globex notice 20150907"
    ),
    // 2026-08-22 — T1 — 2026-08-22 review: verified current, onset undated —
    // knowledge-bound row widening the Sunday queue by the disputed 16:00-16:15
    // CT quarter-hour, the only part depending on the undated 2012 move. A
    // sourced onset day replaces this row.
    (
        2026,
        8,
        22,
        &ENERGY_METALS_CURRENT,
        "2026-08-22 review: verified current, onset undated"
    ),
];

pub(crate) fn energy_metals_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, US::Central),
        &ENERGY_METALS_AT_2010_FLOOR,
        ENERGY_METALS_REVISIONS,
    )
}
