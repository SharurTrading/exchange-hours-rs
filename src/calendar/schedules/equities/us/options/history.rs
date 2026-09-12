// SPDX-License-Identifier: MIT-0

//! Point-in-time selectors for US listed-equity-options schedules.
//!
//! The profile tables live in the parent module; this module owns their dated
//! revision rows and the selector every venue routes through. One citation
//! line sits beside each row; the narrative evidence lives in
//! `docs/evidence/<owner>.md`, declared by the `// Evidence:` line above each
//! block.

use chrono::{DateTime, Utc};
use chrono_tz::America;

use super::{
    BOX_OPTIONS_PROFILE, CBOE_BZX_OPTIONS_PROFILE, CBOE_C2_OPTIONS_PROFILE,
    CBOE_EDGX_OPTIONS_PROFILE, CBOE_OPTIONS_C1_PROFILE, LISTED_EQUITY_OPTIONS_HISTORICAL,
    MEMX_OPTIONS_PROFILE, MIAX_EMERALD_OPTIONS_PROFILE, MIAX_OPTIONS_PROFILE,
    MIAX_PEARL_OPTIONS_PROFILE, MIAX_SAPPHIRE_OPTIONS_PROFILE, NASDAQ_BX_OPTIONS_PROFILE,
    NASDAQ_GEMX_OPTIONS_PROFILE, NASDAQ_ISE_OPTIONS_PROFILE, NASDAQ_MRX_OPTIONS_PROFILE,
    NASDAQ_NOM_OPTIONS_PROFILE, NASDAQ_PHLX_OPTIONS_PROFILE, NYSE_AMERICAN_OPTIONS_PROFILE,
    NYSE_ARCA_OPTIONS_PROFILE,
};
use crate::calendar::schedules::CLOSED_NEW_YORK;
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// Evidence: docs/evidence/cboe_bzx_options.md
// 2010-02-26 — T1 — BATS Options launch press release — BZX Options launch.
static BZX_REVISIONS: &[Revision] = revisions![(
    2010,
    2,
    26,
    &CBOE_BZX_OPTIONS_PROFILE,
    "BATS Options launch press release"
),];

// Evidence: docs/evidence/cboe_c2_options.md
// 2010-10-29 — T1 — Cboe circular IC-CBOE-2010-168 — C2 Options launch.
static C2_REVISIONS: &[Revision] = revisions![(
    2010,
    10,
    29,
    &CBOE_C2_OPTIONS_PROFILE,
    "Cboe circular IC-CBOE-2010-168"
),];

// Evidence: docs/evidence/cboe_edgx_options.md
// 2015-11-02 — T1 — Bats EDGX options update 2015-11-10 — EDGX phase-one launch.
static EDGX_REVISIONS: &[Revision] = revisions![(
    2015,
    11,
    2,
    &CBOE_EDGX_OPTIONS_PROFILE,
    "Bats EDGX options update 2015-11-10"
),];

// Evidence: docs/evidence/nasdaq_bx_options.md
// 2012-06-29 — T1 — Nasdaq OTA 2012-41 — BX Options launch.
static BX_REVISIONS: &[Revision] = revisions![(
    2012,
    6,
    29,
    &NASDAQ_BX_OPTIONS_PROFILE,
    "Nasdaq OTA 2012-41"
),];

// Evidence: docs/evidence/nasdaq_gemx.md
// 2013-08-05 — T1 — SEC filing 16019242 — GEMX launch.
static GEMX_REVISIONS: &[Revision] = revisions![(
    2013,
    8,
    5,
    &NASDAQ_GEMX_OPTIONS_PROFILE,
    "SEC filing 16019242"
),];

// Evidence: docs/evidence/nasdaq_mrx.md
// 2016-02-16 — T1 — SEC 34-77256 — MRX launch.
static MRX_REVISIONS: &[Revision] =
    revisions![(2016, 2, 16, &NASDAQ_MRX_OPTIONS_PROFILE, "SEC 34-77256"),];

// Evidence: docs/evidence/miax_options.md
// 2012-12-07 — T1 — MIAX launch alert 2012-12-06 — launch on a queue-free grid.
// 2013-05-07 — T1 — first capture showing the window affecting the live book —
// the 07:30 ET window becomes a live order-acceptance queue.
static MIAX_REVISIONS: &[Revision] = revisions![
    (
        2012,
        12,
        7,
        &LISTED_EQUITY_OPTIONS_HISTORICAL,
        "MIAX launch alert 2012-12-06"
    ),
    (
        2013,
        5,
        7,
        &MIAX_OPTIONS_PROFILE,
        "first capture showing the window affecting the live book"
    ),
];

// Evidence: docs/evidence/miax_pearl_options.md
// 2017-02-06 — T1 — MIAX Pearl launch alert 2017-02-01 — MIAX Pearl launch.
static MIAX_PEARL_REVISIONS: &[Revision] = revisions![(
    2017,
    2,
    6,
    &MIAX_PEARL_OPTIONS_PROFILE,
    "MIAX Pearl launch alert 2017-02-01"
),];

// Evidence: docs/evidence/miax_emerald_options.md
// 2019-03-01 — T1 — MIAX Emerald launch announcement — MIAX Emerald launch.
static MIAX_EMERALD_REVISIONS: &[Revision] = revisions![(
    2019,
    3,
    1,
    &MIAX_EMERALD_OPTIONS_PROFILE,
    "MIAX Emerald launch announcement"
),];

// Evidence: docs/evidence/miax_sapphire_options.md
// 2024-08-12 — T1 — MIAX press release 2024-09-10 — MIAX Sapphire launch.
static MIAX_SAPPHIRE_REVISIONS: &[Revision] = revisions![(
    2024,
    8,
    12,
    &MIAX_SAPPHIRE_OPTIONS_PROFILE,
    "MIAX press release 2024-09-10"
),];

// Evidence: docs/evidence/memx_options.md
// 2023-09-27 — T1 — MEMX trader alert 23-42 — MEMX Options launch, no queue.
static MEMX_REVISIONS: &[Revision] = revisions![(
    2023,
    9,
    27,
    &MEMX_OPTIONS_PROFILE,
    "MEMX trader alert 23-42"
),];

// Venues whose 09:30-16:00 ET execution history predates the January-2010
// floor. They carry no dated revision row, so `select_revision` serves the
// baseline profile — which already holds the venue's order-acceptance queue —
// at every date the crate answers for. Evidence: the file each of these seven
// identities' ledger rows links under docs/evidence/.
static C1_REVISIONS: &[Revision] = &[];
static NYSE_ARCA_OPTIONS_REVISIONS: &[Revision] = &[];
static NYSE_AMERICAN_OPTIONS_REVISIONS: &[Revision] = &[];
static NASDAQ_PHLX_REVISIONS: &[Revision] = &[];
static NASDAQ_ISE_REVISIONS: &[Revision] = &[];
static NASDAQ_NOM_REVISIONS: &[Revision] = &[];
static BOX_REVISIONS: &[Revision] = &[];

macro_rules! carried_selector {
    ($($name:ident, $baseline:ident, $revisions:ident),+ $(,)?) => {
        $(
            pub(crate) fn $name(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
                select_revision(
                    local_date(as_of, America::New_York),
                    &$baseline,
                    $revisions,
                )
            }
        )+
    };
}

carried_selector!(
    c1_profile_at,
    CBOE_OPTIONS_C1_PROFILE,
    C1_REVISIONS,
    nyse_arca_options_profile_at,
    NYSE_ARCA_OPTIONS_PROFILE,
    NYSE_ARCA_OPTIONS_REVISIONS,
    nyse_american_options_profile_at,
    NYSE_AMERICAN_OPTIONS_PROFILE,
    NYSE_AMERICAN_OPTIONS_REVISIONS,
    nasdaq_phlx_profile_at,
    NASDAQ_PHLX_OPTIONS_PROFILE,
    NASDAQ_PHLX_REVISIONS,
    nasdaq_ise_profile_at,
    NASDAQ_ISE_OPTIONS_PROFILE,
    NASDAQ_ISE_REVISIONS,
    nasdaq_nom_profile_at,
    NASDAQ_NOM_OPTIONS_PROFILE,
    NASDAQ_NOM_REVISIONS,
    box_options_profile_at,
    BOX_OPTIONS_PROFILE,
    BOX_REVISIONS,
);

macro_rules! launch_selector {
    ($name:ident, $revisions:ident) => {
        pub(crate) fn $name(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
            select_revision(
                local_date(as_of, America::New_York),
                &CLOSED_NEW_YORK,
                $revisions,
            )
        }
    };
}

launch_selector!(c2_options_profile_at, C2_REVISIONS);
launch_selector!(bzx_options_profile_at, BZX_REVISIONS);
launch_selector!(edgx_options_profile_at, EDGX_REVISIONS);
launch_selector!(nasdaq_bx_options_profile_at, BX_REVISIONS);
launch_selector!(nasdaq_gemx_profile_at, GEMX_REVISIONS);
launch_selector!(nasdaq_mrx_profile_at, MRX_REVISIONS);
launch_selector!(miax_options_profile_at, MIAX_REVISIONS);
launch_selector!(miax_pearl_options_profile_at, MIAX_PEARL_REVISIONS);
launch_selector!(miax_emerald_options_profile_at, MIAX_EMERALD_REVISIONS);
launch_selector!(miax_sapphire_options_profile_at, MIAX_SAPPHIRE_REVISIONS);
launch_selector!(memx_options_profile_at, MEMX_REVISIONS);
