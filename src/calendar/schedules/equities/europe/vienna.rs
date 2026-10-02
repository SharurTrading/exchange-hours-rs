// SPDX-License-Identifier: MIT-0

//! Vienna Stock Exchange cash equities, represented by the ATX segment.

use chrono::{Datelike, NaiveDate, Weekday};
use chrono_tz::Europe;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{effective_date, local_date};

const fn session(open_ssm: u32, close_ssm: u32) -> SessionRule {
    SessionRule {
        days: MON_FRI,
        open_ssm,
        close_ssm,
    }
}

// The exchange's detailed Xetra 9.1 specification, effective 2009-01-02, establishes the January-2010 ATX baseline. Narrative:
// docs/evidence/vienna.md.
static LEGACY_NORMAL_REGULAR: &[SessionRule] = &[
    session(9 * 3600 + 60, 12 * 3600),
    session(12 * 3600 + 4 * 60, 17 * 3600 + 30 * 60),
];
// Auction windows below cover a call phase and its price determination, which prints trades, so they stay `extended`. Narrative:
// docs/evidence/vienna.md.
static LEGACY_NORMAL_EXTENDED: &[SessionRule] = &[
    session(8 * 3600 + 55 * 60, 9 * 3600 + 60),
    session(12 * 3600, 12 * 3600 + 4 * 60),
    session(17 * 3600 + 30 * 60, 17 * 3600 + 34 * 60),
];
static LEGACY_NORMAL_ORDER_ENTRY: &[SessionRule] = &[
    // Pre-trading.
    session(8 * 3600, 8 * 3600 + 55 * 60),
    // Post-trading.
    session(17 * 3600 + 34 * 60, 17 * 3600 + 45 * 60),
];
static LEGACY_SETTLEMENT_REGULAR: &[SessionRule] = &[
    session(9 * 3600 + 2 * 60 + 30, 12 * 3600),
    session(12 * 3600 + 7 * 60 + 30, 17 * 3600 + 30 * 60),
];
static LEGACY_SETTLEMENT_EXTENDED: &[SessionRule] = &[
    session(8 * 3600 + 55 * 60, 9 * 3600 + 2 * 60 + 30),
    session(12 * 3600, 12 * 3600 + 7 * 60 + 30),
    session(17 * 3600 + 30 * 60, 17 * 3600 + 35 * 60 + 30),
];
static LEGACY_SETTLEMENT_ORDER_ENTRY: &[SessionRule] = &[
    LEGACY_NORMAL_ORDER_ENTRY[0],
    session(17 * 3600 + 35 * 60 + 30, 17 * 3600 + 45 * 60),
];

static LEGACY_NORMAL: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: LEGACY_NORMAL_REGULAR,
    extended: LEGACY_NORMAL_EXTENDED,
    order_entry: LEGACY_NORMAL_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};
static LEGACY_SETTLEMENT: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: LEGACY_SETTLEMENT_REGULAR,
    extended: LEGACY_SETTLEMENT_EXTENDED,
    order_entry: LEGACY_SETTLEMENT_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// ATX equities migrated to Xetra T7 effective 2017-07-31. Narrative:
// docs/evidence/vienna.md.
static T7_NORMAL_REGULAR: &[SessionRule] = &[
    session(9 * 3600 + 30, 12 * 3600),
    session(12 * 3600 + 3 * 60 + 30, 17 * 3600 + 30 * 60),
];
static T7_SETTLEMENT_REGULAR: &[SessionRule] = &[
    session(9 * 3600 + 30, 12 * 3600),
    session(12 * 3600 + 5 * 60 + 30, 17 * 3600 + 30 * 60),
];
static T7_NORMAL_PRE_2019_EXTENDED: &[SessionRule] = &[
    session(8 * 3600 + 55 * 60, 9 * 3600 + 30),
    session(12 * 3600, 12 * 3600 + 3 * 60 + 30),
    session(17 * 3600 + 30 * 60, 17 * 3600 + 33 * 60 + 30),
];
static T7_SETTLEMENT_PRE_2019_EXTENDED: &[SessionRule] = &[
    T7_NORMAL_PRE_2019_EXTENDED[0],
    session(12 * 3600, 12 * 3600 + 5 * 60 + 30),
    T7_NORMAL_PRE_2019_EXTENDED[2],
];
// Pre-trading, then post-trading from the end of the shorter closing call.
static T7_PRE_2019_ORDER_ENTRY: &[SessionRule] = &[
    LEGACY_NORMAL_ORDER_ENTRY[0],
    session(17 * 3600 + 33 * 60 + 30, 17 * 3600 + 45 * 60),
];

static T7_NORMAL_PRE_2019: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: T7_NORMAL_REGULAR,
    extended: T7_NORMAL_PRE_2019_EXTENDED,
    order_entry: T7_PRE_2019_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};
static T7_SETTLEMENT_PRE_2019: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: T7_SETTLEMENT_REGULAR,
    extended: T7_SETTLEMENT_PRE_2019_EXTENDED,
    order_entry: T7_PRE_2019_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// The operator extended the ATX closing call by two minutes effective
// 2019-05-02. Its detailed 2020 specification gives the exact five-minute
// call, maximum 30-second random end, and post-trading through 17:45.
// https://www.wienerborse.at/en/news/vienna-stock-exchange-news/5-new-austrian-listings-in-q1-equity-turnover-reclining-throughout-europe-due-to-brexit/
// https://web.archive.org/web/20200610172528id_/https://www.wienerborse.at/uploads/u/cms/files/trading/xetra-t7-detailed-specifications-market-models.pdf
static PRE_TAC_NORMAL_EXTENDED: &[SessionRule] = &[
    T7_NORMAL_PRE_2019_EXTENDED[0],
    T7_NORMAL_PRE_2019_EXTENDED[1],
    session(17 * 3600 + 30 * 60, 17 * 3600 + 35 * 60 + 30),
];
static PRE_TAC_SETTLEMENT_EXTENDED: &[SessionRule] = &[
    PRE_TAC_NORMAL_EXTENDED[0],
    session(12 * 3600, 12 * 3600 + 5 * 60 + 30),
    PRE_TAC_NORMAL_EXTENDED[2],
];
// The two-minute closing-call extension pushed post-trading back to 17:35:30;
// before Trade-at-Close there was nothing executable after the auction.
static PRE_TAC_ORDER_ENTRY: &[SessionRule] = &[
    LEGACY_NORMAL_ORDER_ENTRY[0],
    session(17 * 3600 + 35 * 60 + 30, 17 * 3600 + 45 * 60),
];
static PRE_TAC_NORMAL: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: T7_NORMAL_REGULAR,
    extended: PRE_TAC_NORMAL_EXTENDED,
    order_entry: PRE_TAC_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};
static PRE_TAC_SETTLEMENT: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: T7_SETTLEMENT_REGULAR,
    extended: PRE_TAC_SETTLEMENT_EXTENDED,
    order_entry: PRE_TAC_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// Trade-at-Close launched on 2020-12-01, making the official closing price executable through 17:45 and moving post-trading to 17:45-17:50. Narrative:
// docs/evidence/vienna.md.
static CURRENT_NORMAL_EXTENDED: &[SessionRule] = &[
    PRE_TAC_NORMAL_EXTENDED[0],
    PRE_TAC_NORMAL_EXTENDED[1],
    PRE_TAC_NORMAL_EXTENDED[2],
    // Trade-at-Close: the official closing price stays executable, so this
    // window is tradeable and only the 17:45-17:50 tail is order entry.
    session(17 * 3600 + 35 * 60 + 30, 17 * 3600 + 45 * 60),
];
static CURRENT_SETTLEMENT_EXTENDED: &[SessionRule] = &[
    CURRENT_NORMAL_EXTENDED[0],
    session(12 * 3600, 12 * 3600 + 5 * 60 + 30),
    CURRENT_NORMAL_EXTENDED[2],
    CURRENT_NORMAL_EXTENDED[3],
];
// Trade-at-Close moved post-trading to the operator's published 17:45-17:50.
static CURRENT_ORDER_ENTRY: &[SessionRule] = &[
    LEGACY_NORMAL_ORDER_ENTRY[0],
    session(17 * 3600 + 45 * 60, 17 * 3600 + 50 * 60),
];

pub(crate) static VIENNA_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: T7_NORMAL_REGULAR,
    extended: CURRENT_NORMAL_EXTENDED,
    order_entry: CURRENT_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};
static CURRENT_NORMAL: &StaticHoursProfile = &VIENNA_PROFILE;
static CURRENT_SETTLEMENT: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: T7_SETTLEMENT_REGULAR,
    extended: CURRENT_SETTLEMENT_EXTENDED,
    order_entry: CURRENT_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

const T7_MIGRATION: NaiveDate = effective_date(2017, 7, 31);
const CLOSING_EXTENSION: NaiveDate = effective_date(2019, 5, 2);
const TRADE_AT_CLOSE: NaiveDate = effective_date(2020, 12, 1);

fn is_settlement_day(day: NaiveDate) -> bool {
    day.weekday() == Weekday::Fri && (15..=21).contains(&day.day())
}

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, Europe::Vienna);
    let settlement = is_settlement_day(day);

    if day < T7_MIGRATION {
        return if settlement {
            &LEGACY_SETTLEMENT
        } else {
            &LEGACY_NORMAL
        };
    }
    if day < CLOSING_EXTENSION {
        return if settlement {
            &T7_SETTLEMENT_PRE_2019
        } else {
            &T7_NORMAL_PRE_2019
        };
    }
    if day < TRADE_AT_CLOSE {
        return if settlement {
            &PRE_TAC_SETTLEMENT
        } else {
            &PRE_TAC_NORMAL
        };
    }
    if settlement {
        &CURRENT_SETTLEMENT
    } else {
        CURRENT_NORMAL
    }
}
