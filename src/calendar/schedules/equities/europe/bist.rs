// SPDX-License-Identifier: MIT-0

//! Borsa Istanbul Equity Market.

use chrono_tz::Europe;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

macro_rules! rule {
    ($open:expr, $close:expr) => {
        SessionRule {
            days: MON_FRI,
            open_ssm: $open,
            close_ssm: $close,
        }
    };
}

static REG_PRE_2012: &[SessionRule] = &[
    rule!(9 * 3600 + 50 * 60, 12 * 3600 + 30 * 60),
    rule!(14 * 3600 + 20 * 60, 17 * 3600 + 30 * 60),
];
static EXT_PRE_2012: &[SessionRule] = &[
    rule!(9 * 3600 + 30 * 60, 9 * 3600 + 50 * 60),
    rule!(14 * 3600, 14 * 3600 + 20 * 60),
];
static REG_2012_MARCH: &[SessionRule] = &[
    rule!(9 * 3600 + 50 * 60, 12 * 3600 + 30 * 60),
    rule!(14 * 3600 + 15 * 60, 17 * 3600 + 17 * 60),
];
static EXT_2012_MARCH: &[SessionRule] = &[
    rule!(9 * 3600 + 30 * 60, 9 * 3600 + 50 * 60),
    rule!(14 * 3600, 14 * 3600 + 15 * 60),
    rule!(17 * 3600 + 17 * 60, 17 * 3600 + 30 * 60),
];
static REG_2012_JULY: &[SessionRule] = &[
    rule!(9 * 3600 + 50 * 60, 12 * 3600 + 30 * 60),
    rule!(14 * 3600 + 15 * 60, 17 * 3600 + 30 * 60),
];
static EXT_2012_JULY: &[SessionRule] = &[
    rule!(9 * 3600 + 30 * 60, 9 * 3600 + 50 * 60),
    rule!(14 * 3600, 14 * 3600 + 15 * 60),
    rule!(17 * 3600 + 30 * 60, 17 * 3600 + 40 * 60),
];
static REG_2013_APRIL: &[SessionRule] = &[
    rule!(9 * 3600 + 45 * 60, 12 * 3600 + 30 * 60),
    REG_2012_JULY[1],
];
static EXT_2013_APRIL: &[SessionRule] = &[
    rule!(9 * 3600 + 15 * 60, 9 * 3600 + 45 * 60),
    EXT_2012_JULY[1],
    EXT_2012_JULY[2],
];
static REG_2013_JUNE: &[SessionRule] = &[
    rule!(9 * 3600 + 35 * 60, 12 * 3600 + 30 * 60),
    REG_2012_JULY[1],
];
static EXT_2013_JUNE: &[SessionRule] = &[
    rule!(9 * 3600 + 15 * 60, 9 * 3600 + 35 * 60),
    EXT_2012_JULY[1],
    EXT_2012_JULY[2],
];
static REG_2015: &[SessionRule] = &[
    REG_2013_JUNE[0],
    rule!(13 * 3600 + 30 * 60, 17 * 3600 + 30 * 60),
];
static EXT_2015: &[SessionRule] = &[
    EXT_2013_JUNE[0],
    rule!(12 * 3600 + 30 * 60, 13 * 3600 + 30 * 60),
    rule!(17 * 3600 + 30 * 60, 17 * 3600 + 40 * 60),
];
static REG_2016_MARCH: &[SessionRule] = &[
    rule!(9 * 3600 + 35 * 60, 13 * 3600),
    rule!(14 * 3600, 17 * 3600 + 30 * 60),
];
static EXT_2016_MARCH: &[SessionRule] =
    &[EXT_2013_JUNE[0], rule!(13 * 3600, 14 * 3600), EXT_2015[2]];
static REG_2016_NOVEMBER: &[SessionRule] =
    &[rule!(10 * 3600, 13 * 3600), rule!(14 * 3600, 18 * 3600)];
// Order-entry classification. The 2016-11-14 announcement cited below states
// that "the trading session shall start at 09:40 with order collection" and
// that "[f]ollowing the end of the order collection phase at 09:55, continuous
// auction shall start at 10:00". The Equity Market Procedure's call-auction
// rules add that "[n]o transactions are executed during the order collection
// period", and its session table splits the 09:40-10:00 opening auction into an
// Order Collection Process (09:40-09:55) and Determination of Opening Price
// (09:55 onward). Only the collection leg moves; 09:55-10:00 carries the
// opening print and stays in `extended`. The midday single-price call and the
// 18:00-18:10 closing/single-price envelope each bundle collection with a
// price-determination leg that prints, so both stay in `extended` whole.
static ORDER_ENTRY_2016_NOVEMBER: &[SessionRule] = &[rule!(9 * 3600 + 40 * 60, 9 * 3600 + 55 * 60)];
static EXT_2016_NOVEMBER: &[SessionRule] = &[
    // Determination of the opening price.
    rule!(9 * 3600 + 55 * 60, 10 * 3600),
    rule!(13 * 3600, 14 * 3600),
    rule!(18 * 3600, 18 * 3600 + 10 * 60),
];
static REG_CURRENT: &[SessionRule] = &[rule!(10 * 3600, 18 * 3600)];
static EXT_CURRENT: &[SessionRule] = &[EXT_2016_NOVEMBER[0], EXT_2016_NOVEMBER[2]];
static ORDER_ENTRY_CURRENT: &[SessionRule] = &[ORDER_ENTRY_2016_NOVEMBER[0]];

macro_rules! profile {
    ($name:ident, $regular:ident, $extended:ident) => {
        profile!($name, $regular, $extended, &[]);
    };
    ($name:ident, $regular:ident, $extended:ident, $order_entry:expr) => {
        pub(crate) static $name: StaticHoursProfile = StaticHoursProfile {
            tz: Europe::Istanbul,
            regular: $regular,
            extended: $extended,
            order_entry: $order_entry,
            has_daily_close: true,
            has_weekend_close: true,
        };
    };
}

// Current: opening 09:40–10:00, continuous 10:00–18:00, closing auction and
// trade-at-last 18:00–18:10.
// https://www.borsaistanbul.com/files/equity-market-procedure.pdf
profile!(
    BIST_PROFILE_CURRENT,
    REG_CURRENT,
    EXT_CURRENT,
    ORDER_ENTRY_CURRENT
);

// Each source below states the exact effective date and replacement table.
// 2012-03-02 closing auction:
// https://www.borsaistanbul.com/datum/closing_session.pdf
// 2012-07-16 afternoon extension:
// https://www.borsaistanbul.com/data/Genelge/gn2012394.pdf
// 2013-04-05 and 2013-06-10 opening changes:
// https://www.borsaistanbul.com/data/Genelge/gn2013421.pdf
// https://www.borsaistanbul.com/data/Genelge/gn2013430.pdf
// 2015-11-30 and 2016-03-28 midday calls:
// https://www.borsaistanbul.com/en/announcement/13472/single-session-era-borsa-istanbul
// https://www.borsaistanbul.com/en/announcement/13446/new-arrangement-borsa-istanbul-equity-market-midday-session
// 2016-11-14 extended day and 2019-10-04 midday-call removal:
// https://www.borsaistanbul.com/en/announcement/13376/borsa-istanbul-trading-session-hours-change
// https://www.borsaistanbul.com/duyuru/11640/pay-piyasasi-seansinda-gun-ortasi-tek-fiyat-bolumu-hk-201956-sayili-duyuru
profile!(BIST_PROFILE_PRE_2012_03_02, REG_PRE_2012, EXT_PRE_2012);
profile!(BIST_PROFILE_POST_2012_03_02, REG_2012_MARCH, EXT_2012_MARCH);
profile!(BIST_PROFILE_POST_2012_07_16, REG_2012_JULY, EXT_2012_JULY);
profile!(BIST_PROFILE_POST_2013_04_05, REG_2013_APRIL, EXT_2013_APRIL);
profile!(BIST_PROFILE_POST_2013_06_10, REG_2013_JUNE, EXT_2013_JUNE);
profile!(BIST_PROFILE_POST_2015_11_30, REG_2015, EXT_2015);
profile!(BIST_PROFILE_POST_2016_03_28, REG_2016_MARCH, EXT_2016_MARCH);
profile!(
    BIST_PROFILE_POST_2016_11_14,
    REG_2016_NOVEMBER,
    EXT_2016_NOVEMBER,
    ORDER_ENTRY_2016_NOVEMBER
);

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &BIST_PROFILE_CURRENT;

static REVISIONS: &[Revision] = revisions![
    (
        2012,
        3,
        2,
        &BIST_PROFILE_POST_2012_03_02,
        "Borsa Istanbul closing_session"
    ),
    (
        2012,
        7,
        16,
        &BIST_PROFILE_POST_2012_07_16,
        "Borsa Istanbul Genelge gn2012394"
    ),
    (
        2013,
        4,
        5,
        &BIST_PROFILE_POST_2013_04_05,
        "Borsa Istanbul Genelge gn2013421"
    ),
    (
        2013,
        6,
        10,
        &BIST_PROFILE_POST_2013_06_10,
        "Borsa Istanbul Genelge gn2013430"
    ),
    (
        2015,
        11,
        30,
        &BIST_PROFILE_POST_2015_11_30,
        "Borsa Istanbul announcement 13472"
    ),
    (
        2016,
        3,
        28,
        &BIST_PROFILE_POST_2016_03_28,
        "Borsa Istanbul announcement 13446"
    ),
    (
        2016,
        11,
        14,
        &BIST_PROFILE_POST_2016_11_14,
        "Borsa Istanbul announcement 13376"
    ),
    (
        2019,
        10,
        4,
        &BIST_PROFILE_CURRENT,
        "Borsa Istanbul duyuru 2019/56"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &BIST_PROFILE_PRE_2012_03_02,
        REVISIONS,
    )
}
