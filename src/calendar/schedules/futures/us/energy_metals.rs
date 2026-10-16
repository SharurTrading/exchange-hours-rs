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

pub(crate) static ENERGY_METALS_AT_2010_FLOOR: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: ENERGY_METALS_EXTENDED_AT_2010_FLOOR,
    order_entry: ENERGY_METALS_ORDER_ENTRY_DATED,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static ENERGY_METALS_DATED_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: ENERGY_METALS_EXTENDED_CURRENT,
    order_entry: ENERGY_METALS_ORDER_ENTRY_DATED,
    has_daily_close: true,
    has_weekend_close: true,
};
// The 2016-01-03 Sunday widening: the 2015-09-20 grid with the Sunday queue
// onset market data witnesses from that Sunday (2026-10-16 UTC). Every Sunday
// 2016-01-03..2026-04-05 in the archive opens the queue at exactly 16:00:00
// CT, this family's products in step with the whole complex.
pub(crate) static ENERGY_METALS_PROFILE_2016_01_03: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: ENERGY_METALS_EXTENDED_CURRENT,
    order_entry: ENERGY_METALS_ORDER_ENTRY_CURRENT,
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
    // 2016-01-03 — T2 — MARKET-DATA — the Sunday Pre-Open opens at 16:00 CT
    // from this Sunday, witnessed on every archive Sunday 2016-01-03..2026-04-05
    // (2026-10-16 UTC). The disputed quarter-hour shrinks to the Sundays of
    // 2012-05-28..2016-01-02, below every witness the archive holds. A sourced
    // onset day for the 2012 move replaces the residual.
    (2016, 1, 3, &ENERGY_METALS_PROFILE_2016_01_03, "MARKET-DATA"),
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
