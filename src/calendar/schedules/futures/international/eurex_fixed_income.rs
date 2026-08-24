// SPDX-License-Identifier: MIT-0

//! Eurex fixed income futures (`FGBL`, `FGBM`, `FGBS`, `FGBX`) schedules.
//!
//! Euro-Bund, Euro-Bobl, Euro-Schatz and Euro-Buxl Futures share one grid,
//! verified row by row in Annex C of the Eurex Contract Specifications (as of
//! 17.08.2026) and on all four product pages, so the four contracts are
//! modelled as a single family here.

use chrono_tz::{Europe, UTC};

use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{
    Revision, local_date, reference_delta_seconds, revisions, select_revision,
};

// Eurex publishes a phase machine, not a single open/close pair: Pre-Trading,
// Opening auction, Continuous Trading, Closing auction, Post-Trading. The
// executable on-exchange order book is the Continuous Trading phase, so that
// phase alone is `regular`; Pre-Trading and Post-Trading accept order entry and
// maintenance without matching, so they are `order_entry`.
//
// Classification note: no trade can print in Pre-Trading or Post-Trading. The
// trading-phases page defines them as the phases in which orders and quotes may
// be entered, modified and deleted while the order book is not executable, and
// the two auctions that do print sit inside the Continuous Trading row modelled
// as `regular` here. Both phases are therefore `order_entry`, not `extended`,
// and the `extended` slice for this family is empty: this grid has no tradeable
// phase outside continuous trading.
//
// The morning phases are anchored to 08:00 Singapore time, not to a fixed
// Berlin wall clock: Annex C states Continuous Trading as "01:10-22:00 MEZ /
// CET" and "02:10-22:00 MESZ / CEST", and the 2026 trading calendar prints
// 01:10-22:00 with footnote 8 reading "02:10-22:00 CEST". The close stays at
// 22:00 local in both seasons. A single fixed-local-wall-clock profile would
// therefore be wrong for roughly half of every year, so the seasons are two
// profiles selected by the venue's own UTC offset, matching how the sibling
// Eurex benchmark-index module in `europe.rs` models the same Asian-hours
// slice. `_CURRENT` is the CEST grid there, and is the CEST grid here too.
//
// Excluded deliberately: the Eurex T7 Entry Service (off-book TES, 01:15-22:00
// CET / 02:15-22:00 CEST) is bilateral block/EFP/vola business rather than the
// central order book, and clearing hours are not a trading phase at all.
// Per-contract last-trading-day hours (continuous trading ending 12:30) are an
// exceptional-day matter and are outside this normal-week model.
//
// https://www.eurex.com/resource/blob/2824010/044ff047cefd531f61ffe58d55404ca3/data/2026_08_17_eurex_d_kontraktspezifikationen_annexe_en.pdf
// https://www.eurex.com/resource/blob/4873184/0ca7669a8cb9a2f917d99a801fb3f2de/data/tradingcalendar_2026_en.pdf
// https://www.eurex.com/ex-en/markets/int/long-term-interest-rates/fix/government-bonds/Euro-Bund-Futures-137298
// https://www.eurex.com/ex-en/trade/trading-hours/trading-phases
pub(crate) static EUREX_FIXED_INCOME_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 2 * 3600 + 10 * 60,
    close_ssm: 22 * 3600,
}];

// No tradeable non-continuous phase exists on this grid; see the classification
// note above. Kept as an explicit empty slice so the family's shape stays
// visible next to its `order_entry` twin.
pub(crate) static EUREX_FIXED_INCOME_EXTENDED_CURRENT: &[SessionRule] = &[];

// Pre-Trading 02:00-02:10 CEST, then Post-Trading 22:00-22:10 local. Annex C
// gives the post-trading row as "Post-Trading Periode bis / Post-Trading Period
// Until = 22:10*". Order entry only in both windows.
pub(crate) static EUREX_FIXED_INCOME_ORDER_ENTRY_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 2 * 3600,
        close_ssm: 2 * 3600 + 10 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 22 * 3600,
        close_ssm: 22 * 3600 + 10 * 60,
    },
];

pub(crate) static EUREX_FIXED_INCOME_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_FIXED_INCOME_REGULAR_CURRENT,
    extended: EUREX_FIXED_INCOME_EXTENDED_CURRENT,
    order_entry: EUREX_FIXED_INCOME_ORDER_ENTRY_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// The same regime on a CET (winter) day: every morning phase sits one hour
// earlier on the Berlin clock because it is pinned to 08:00 Singapore, while
// the 22:00 close and the 22:10 post-trading end do not move.
static EUREX_FIXED_INCOME_REGULAR_WINTER: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3600 + 10 * 60,
    close_ssm: 22 * 3600,
}];

static EUREX_FIXED_INCOME_ORDER_ENTRY_WINTER: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 3600,
        close_ssm: 3600 + 10 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 22 * 3600,
        close_ssm: 22 * 3600 + 10 * 60,
    },
];

static EUREX_FIXED_INCOME_WINTER: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_FIXED_INCOME_REGULAR_WINTER,
    extended: &[],
    order_entry: EUREX_FIXED_INCOME_ORDER_ENTRY_WINTER,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2018-12-10 through 2019-02-24: the executable session is already today's
// Asian-hours grid, but the post-trading phase still ran to 22:30 rather than
// 22:10. The 22:30 value is the one the February 2019 Contract Specifications
// amendment records as the state it replaced. That whole window sits inside
// CET, so the CEST twin below is never selected in practice; it is kept so the
// regime is described by its dates rather than by an accident of the calendar.
// Both windows are Pre-Trading and Post-Trading, so both are order entry only.
static EUREX_FIXED_INCOME_ORDER_ENTRY_2018_SUMMER: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 2 * 3600,
        close_ssm: 2 * 3600 + 10 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 22 * 3600,
        close_ssm: 22 * 3600 + 30 * 60,
    },
];

static EUREX_FIXED_INCOME_ORDER_ENTRY_2018_WINTER: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 3600,
        close_ssm: 3600 + 10 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 22 * 3600,
        close_ssm: 22 * 3600 + 30 * 60,
    },
];

static EUREX_FIXED_INCOME_2018_SUMMER: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_FIXED_INCOME_REGULAR_CURRENT,
    extended: &[],
    order_entry: EUREX_FIXED_INCOME_ORDER_ENTRY_2018_SUMMER,
    has_daily_close: true,
    has_weekend_close: true,
};

static EUREX_FIXED_INCOME_2018_WINTER: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_FIXED_INCOME_REGULAR_WINTER,
    extended: &[],
    order_entry: EUREX_FIXED_INCOME_ORDER_ENTRY_2018_WINTER,
    has_daily_close: true,
    has_weekend_close: true,
};

// Baseline before 2018-12-10. Circular 088/18 states the phase it replaced:
// Pre-Trading ran 07:30-08:00 CET, so continuous trading began at 08:00 and ran
// to the unchanged 22:00 close - one same-day grid with no seasonal split,
// because nothing in it was anchored to an Asian clock.
//
// The 22:30 post-trading end is carried back into this baseline rather than
// dated: the earliest primary statement of that value inside the modelled
// window is the February 2019 amendment recording the change away from it, and
// no Eurex document dates an earlier change to it. Carrying the value back is
// preferred here over inventing a cutover.
//
// https://www.eurex.com/resource/blob/1412768/e61a2c41d65ad165af7909002223b943/data/er18088e.pdf
// https://www.eurex.com/ex-en/rules-regs/eurex-rules-regulations/03.-Contract-Specifications-4347288
static EUREX_FIXED_INCOME_REGULAR_BASELINE: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 22 * 3600,
}];

// Pre-Trading 07:30-08:00 CET and Post-Trading 22:00-22:30: order entry only,
// same phase machine as the current grid.
static EUREX_FIXED_INCOME_ORDER_ENTRY_BASELINE: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 7 * 3600 + 30 * 60,
        close_ssm: 8 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 22 * 3600,
        close_ssm: 22 * 3600 + 30 * 60,
    },
];

pub(crate) static EUREX_FIXED_INCOME_BASELINE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_FIXED_INCOME_REGULAR_BASELINE,
    extended: &[],
    order_entry: EUREX_FIXED_INCOME_ORDER_ENTRY_BASELINE,
    has_daily_close: true,
    has_weekend_close: true,
};

// 2018-12-10: Eurex Circular 088/18, dated 15 November 2018, "Extension of
//   trading hours for selected benchmark futures and MSCI futures": "With the
//   introduction of extended trading hours planned as of 10 December 2018, ..."
//   Pre-Trading moved from 07:30-08:00 CET to 01:00-01:10 CET / 02:00-02:10
//   CEST and continuous trading became 01:10-22:00 CET / 02:10-22:00 CEST. This
//   is the only change to the executable session across the modelled window.
//   https://www.eurex.com/resource/blob/1412768/e61a2c41d65ad165af7909002223b943/data/er18088e.pdf
// 2019-02-25: Contract Specifications amendment, indexed by Eurex under the
//   title "Shortening of post-trading phase for products traded until 22:00
//   CET" and headed "Contract Specifications for Futures Contracts and Options
//   Contracts at Eurex Deutschland". For FGBL, FGBM, FGBS and FGBX the
//   Post-Trading Period Until changed from 22:30 to 22:10; continuous trading
//   was untouched.
//   https://www.eurex.com/resource/blob/1493194/bee8965900f0124d7ff7c7993d5f969b/data/2019_02_25_cs_4_history.pdf
//
// Revision evidence — both seasonal tables carry the same two rows, and each
// row's day-level effective date is stated by the primary source quoted in
// full above:
//   2018-12-10 "Eurex Circular 088/18"
//     https://www.eurex.com/resource/blob/1412768/e61a2c41d65ad165af7909002223b943/data/er18088e.pdf
//   2019-02-25 "Eurex CS amendment 2019-02-25"
//     https://www.eurex.com/resource/blob/1493194/bee8965900f0124d7ff7c7993d5f969b/data/2019_02_25_cs_4_history.pdf
//
// This is the CEST (summer) timeline; `EUREX_FIXED_INCOME_WINTER_REVISIONS`
// carries the identical dates against the CET grid.
pub(crate) static EUREX_FIXED_INCOME_REVISIONS: &[Revision] = revisions![
    (
        2018,
        12,
        10,
        &EUREX_FIXED_INCOME_2018_SUMMER,
        "Eurex Circular 088/18"
    ),
    (
        2019,
        2,
        25,
        &EUREX_FIXED_INCOME_CURRENT,
        "Eurex CS amendment 2019-02-25"
    ),
];

static EUREX_FIXED_INCOME_WINTER_REVISIONS: &[Revision] = revisions![
    (
        2018,
        12,
        10,
        &EUREX_FIXED_INCOME_2018_WINTER,
        "Eurex Circular 088/18"
    ),
    (
        2019,
        2,
        25,
        &EUREX_FIXED_INCOME_WINTER,
        "Eurex CS amendment 2019-02-25"
    ),
];

/// The `reference - venue` delta that marks a CEST (summer) Berlin day: UTC
/// minus Berlin's +02:00 summer offset.
const BERLIN_SUMMER_DELTA_SECONDS: i32 = -2 * 3600;

/// Selects the Eurex fixed income profile in force on `as_of`'s Berlin day.
///
/// Both seasonal timelines carry the same effective dates; the venue's own UTC
/// offset on that day picks the CET or CEST grid. Berlin's changeover falls on
/// a Sunday, which is not a Eurex trading day, so the two never disagree about
/// a day that has a session.
pub(crate) fn eurex_fixed_income_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    let revisions =
        if reference_delta_seconds(as_of, Europe::Berlin, UTC) == BERLIN_SUMMER_DELTA_SECONDS {
            EUREX_FIXED_INCOME_REVISIONS
        } else {
            EUREX_FIXED_INCOME_WINTER_REVISIONS
        };
    select_revision(
        local_date(as_of, Europe::Berlin),
        &EUREX_FIXED_INCOME_BASELINE,
        revisions,
    )
}
