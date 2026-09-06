// SPDX-License-Identifier: MIT-0

//! CME cryptocurrency futures schedules.

// 2026-08-31 five-day-era Pre-Open review — confirmed knowledge-bound. The CME
// bitcoin contract specification captured 2017-12-14, which itself carries the
// launch statement "Effective Sunday 17 December 2017 for trade date Monday 18
// December 2017 ... CME will launch Bitcoin Futures", publishes only the
// matching grid: "CME Globex: Sunday - Friday 6:00 p.m. - 5:00 p.m. (5:00 p.m.
// - 4:00 p.m. CT) with a 60-minute break each day beginning at 5:00 p.m. (4:00
// p.m. CT)". It states no Pre-Open, and neither do the 2017-12-22 or
// 2018-01-04 captures. The five-day era's Sunday/weekday Pre-Open onset is
// therefore undated at the source, not merely unsearched. Official origin
// http://www.cmegroup.com/trading/equity-index/us-index/bitcoin_contract_specifications.html
// delivered via
// https://web.archive.org/web/20171214071544id_/http://www.cmegroup.com/trading/equity-index/us-index/bitcoin_contract_specifications.html
use chrono_tz::US;

use crate::calendar::rule::{FRI, MON_FRI, SUN_ONLY, SUN_PLUS_MON_THU};
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};
use crate::calendar::{FuturesSessionProfile, SessionRule};

const SAT_ONLY: [bool; 7] = [false, false, false, false, false, true, false];
const THU_ONLY: [bool; 7] = [false, false, false, true, false, false, false];

// Bitcoin futures opened Sunday 2017-12-17 at 17:00 CT for trade date Monday
// 2017-12-18. The launch filing gives the original 17:00-16:00 weekday grid.
// ETH, MBT, and MET joined this already-live family in 2021; their individual
// launch dates are intentionally not family-clock revisions.
//
// CME filing 26-114 changed all non-spot-quoted cryptocurrency futures to
// 24/7 Globex trading effective Friday 2026-05-29: matching maintenance is
// 16:00-16:02 CT Monday-Friday with Pre-Open from 16:01, and 02:00-04:00 CT
// Saturday with Pre-Open from 03:45. Three one-day Globex notices then
// temporarily extended the Saturday window for the 24/7 markets — 2026-08-01
// through 09:00 CT (notice 20260727), 2026-08-29 through 06:00 CT and
// 2026-09-19 through 08:00 CT (notice 20260824, restated by 20260831) — each
// without publishing a replacement Pre-Open, each followed by the standard
// 02:00-04:00 window. The notices' tables name this family's channels, "CME
// Crypto Futures | 74 | 326" and "CME Crypto Options | 327", alongside the
// event-contract channels. The September row is forward-dated on the
// operator's statement.
//
// `SessionRule` spans at most one local midnight, so the multi-day weekend
// session is stored in adjacent pieces. The key-backed calendar joins those
// storage-only pieces at query time, while retaining the 02:00-03:45 Saturday
// closed break and 03:45-04:00 Pre-Open. Both weekend blocks carry the following
// open business date: normally Monday, or Tuesday when a caller policy closes
// Monday. The corresponding daily bar runs from Friday 16:01 Pre-Open through
// that business date's 16:00 close.
// https://www.cmegroup.com/notices/ser/2017/12/SER-8051R.html
// https://www.cmegroup.com/market-regulation/rule-filings/2017/12/17-417.pdf
// https://www.cmegroup.com/notices/clearing/2021/01/Chadv21-028.pdf
// https://www.cmegroup.com/notices/electronic-trading/2021/04/20210426.html
// https://www.cmegroup.com/notices/electronic-trading/2021/11/20211129.html
// https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2026/5/26-114.pdf
// https://www.cmegroup.com/notices/electronic-trading/2026/05/20260525.html
// https://www.cmegroup.com/articles/faqs/frequently-asked-questions-cryptocurrency-futures.html
// https://www.cmegroup.com/notices/electronic-trading/2026/07/20260727.html
// https://www.cmegroup.com/notices/electronic-trading/2026/08/20260824.html
// https://www.cmegroup.com/notices/electronic-trading/2026/08/20260831.html
static FIVE_DAY_EXTENDED: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];

static CURRENT_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 0,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 60,
        close_ssm: 24 * 3600,
    },
    SessionRule {
        days: SAT_ONLY,
        open_ssm: 0,
        close_ssm: 2 * 3600,
    },
    SessionRule {
        days: SAT_ONLY,
        open_ssm: 3 * 3600 + 45 * 60,
        close_ssm: 24 * 3600,
    },
    SessionRule {
        days: SUN_ONLY,
        open_ssm: 0,
        close_ssm: 24 * 3600,
    },
];

// The permanent 24/7 normal week did not govern the morning of its first day:
// that trading block still began Thursday at 17:00 and closed Friday at 16:00.
// The new schedule then entered Pre-Open at 16:01 and matching resumed at
// 16:02. Keeping this one-day bridge prevents the historical Thursday open
// from being rewritten as Friday midnight.
static EXTENDED_2026_05_29: &[SessionRule] = &[
    SessionRule {
        days: THU_ONLY,
        open_ssm: 17 * 3600,
        close_ssm: 16 * 3600,
    },
    SessionRule {
        days: FRI,
        open_ssm: 16 * 3600 + 60,
        close_ssm: 24 * 3600,
    },
];

// One-day Saturday extensions: the weekday and Sunday pieces are the normal
// grid; only the Saturday reopen moves, and no replacement Pre-Open is
// published, so the 03:45 queue is absent on those days.
macro_rules! saturday_extended_to {
    ($name:ident, $reopen_hour:expr) => {
        static $name: &[SessionRule] = &[
            SessionRule {
                days: MON_FRI,
                open_ssm: 0,
                close_ssm: 16 * 3600,
            },
            SessionRule {
                days: MON_FRI,
                open_ssm: 16 * 3600 + 60,
                close_ssm: 24 * 3600,
            },
            SessionRule {
                days: SAT_ONLY,
                open_ssm: 0,
                close_ssm: 2 * 3600,
            },
            SessionRule {
                days: SAT_ONLY,
                open_ssm: $reopen_hour * 3600,
                close_ssm: 24 * 3600,
            },
            SessionRule {
                days: SUN_ONLY,
                open_ssm: 0,
                close_ssm: 24 * 3600,
            },
        ];
    };
}
saturday_extended_to!(EXTENDED_2026_08_01, 9);
saturday_extended_to!(EXTENDED_2026_08_29, 6);
saturday_extended_to!(EXTENDED_2026_09_19, 8);

pub(crate) static CURRENT_FUTURES_PROFILE: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: CURRENT_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: false,
};

static CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static FIVE_DAY: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: FIVE_DAY_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: CURRENT_EXTENDED,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: false,
};

static TRANSITION_2026_05_29: StaticHoursProfile = StaticHoursProfile {
    tz: US::Central,
    regular: &[],
    extended: EXTENDED_2026_05_29,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: false,
};

macro_rules! temporary_saturday {
    ($name:ident, $extended:ident) => {
        static $name: StaticHoursProfile = StaticHoursProfile {
            tz: US::Central,
            regular: &[],
            extended: $extended,
            order_entry: &[],
            has_daily_close: true,
            has_weekend_close: false,
        };
    };
}
temporary_saturday!(TEMPORARY_2026_08_01, EXTENDED_2026_08_01);
temporary_saturday!(TEMPORARY_2026_08_29, EXTENDED_2026_08_29);
temporary_saturday!(TEMPORARY_2026_09_19, EXTENDED_2026_09_19);

static REVISIONS: &[Revision] = revisions![
    (2017, 12, 17, &FIVE_DAY, "CME SER-8051R"),
    (2026, 5, 29, &TRANSITION_2026_05_29, "CME filing 26-114"),
    (2026, 5, 30, &CURRENT, "CME filing 26-114"),
    (
        2026,
        8,
        1,
        &TEMPORARY_2026_08_01,
        "CME Globex notice 20260727"
    ),
    (2026, 8, 2, &CURRENT, "CME Globex notice 20260727"),
    (
        2026,
        8,
        29,
        &TEMPORARY_2026_08_29,
        "CME Globex notice 20260824"
    ),
    (2026, 8, 30, &CURRENT, "CME Globex notice 20260824"),
    (
        2026,
        9,
        19,
        &TEMPORARY_2026_09_19,
        "CME Globex notice 20260824"
    ),
    (2026, 9, 20, &CURRENT, "CME Globex notice 20260824"),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, US::Central), &CLOSED, REVISIONS)
}
