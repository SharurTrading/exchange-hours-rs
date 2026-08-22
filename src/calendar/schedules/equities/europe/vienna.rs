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

// The exchange's detailed Xetra 9.1 specification, effective 2009-01-02,
// establishes the January-2010 ATX baseline. On ordinary days its auction
// market-balancing phases delayed continuous trading until 09:01 and 12:04;
// the closing auction ended at 17:34. On derivatives-settlement days the
// corresponding latest boundaries were 09:02:30, 12:07:30, and 17:35:30.
// The same tables remain in the archived 2012, 2014, and 2015 specifications.
// https://web.archive.org/web/20090219151827id_/http://en.wienerborse.at/static/cms/sites/wbag/media/en/pdf/marketplace_products/feinspez_xetra_marktmodell.pdf
// https://web.archive.org/web/20150529063952id_/http://en.wienerborse.at/static/cms/sites/wbag/media/en/pdf/marketplace_products/feinspez_xetra_marktmodell.pdf
static LEGACY_NORMAL_REGULAR: &[SessionRule] = &[
    session(9 * 3600 + 60, 12 * 3600),
    session(12 * 3600 + 4 * 60, 17 * 3600 + 30 * 60),
];
static LEGACY_NORMAL_EXTENDED: &[SessionRule] = &[
    session(8 * 3600, 8 * 3600 + 55 * 60),
    session(8 * 3600 + 55 * 60, 9 * 3600 + 60),
    session(12 * 3600, 12 * 3600 + 4 * 60),
    session(17 * 3600 + 30 * 60, 17 * 3600 + 34 * 60),
    session(17 * 3600 + 34 * 60, 17 * 3600 + 45 * 60),
];
static LEGACY_SETTLEMENT_REGULAR: &[SessionRule] = &[
    session(9 * 3600 + 2 * 60 + 30, 12 * 3600),
    session(12 * 3600 + 7 * 60 + 30, 17 * 3600 + 30 * 60),
];
static LEGACY_SETTLEMENT_EXTENDED: &[SessionRule] = &[
    session(8 * 3600, 8 * 3600 + 55 * 60),
    session(8 * 3600 + 55 * 60, 9 * 3600 + 2 * 60 + 30),
    session(12 * 3600, 12 * 3600 + 7 * 60 + 30),
    session(17 * 3600 + 30 * 60, 17 * 3600 + 35 * 60 + 30),
    session(17 * 3600 + 35 * 60 + 30, 17 * 3600 + 45 * 60),
];

static LEGACY_NORMAL: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: LEGACY_NORMAL_REGULAR,
    extended: LEGACY_NORMAL_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};
static LEGACY_SETTLEMENT: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: LEGACY_SETTLEMENT_REGULAR,
    extended: LEGACY_SETTLEMENT_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

// ATX equities migrated to Xetra T7 effective 2017-07-31. T7 removed the
// legacy market-balancing phases: ordinary-day continuous trading starts at
// the latest random edges 09:00:30 and 12:03:30. Settlement days retain a
// five-minute midday call and therefore resume at 12:05:30. The archived 2018
// hours page records the pre-extension three-minute closing call; its maximum
// 30-second random period makes 17:33:30 the deterministic boundary.
// https://www.wienerborse.at/uploads/u/cms/files/trading/xetra-detailed-specifications-market-models.pdf
// https://web.archive.org/web/20180214144727id_/https://www.wienerborse.at/en/trading/trading-information/trading-hours/
static T7_NORMAL_REGULAR: &[SessionRule] = &[
    session(9 * 3600 + 30, 12 * 3600),
    session(12 * 3600 + 3 * 60 + 30, 17 * 3600 + 30 * 60),
];
static T7_SETTLEMENT_REGULAR: &[SessionRule] = &[
    session(9 * 3600 + 30, 12 * 3600),
    session(12 * 3600 + 5 * 60 + 30, 17 * 3600 + 30 * 60),
];
static T7_NORMAL_PRE_2019_EXTENDED: &[SessionRule] = &[
    session(8 * 3600, 8 * 3600 + 55 * 60),
    session(8 * 3600 + 55 * 60, 9 * 3600 + 30),
    session(12 * 3600, 12 * 3600 + 3 * 60 + 30),
    session(17 * 3600 + 30 * 60, 17 * 3600 + 33 * 60 + 30),
    session(17 * 3600 + 33 * 60 + 30, 17 * 3600 + 45 * 60),
];
static T7_SETTLEMENT_PRE_2019_EXTENDED: &[SessionRule] = &[
    T7_NORMAL_PRE_2019_EXTENDED[0],
    T7_NORMAL_PRE_2019_EXTENDED[1],
    session(12 * 3600, 12 * 3600 + 5 * 60 + 30),
    T7_NORMAL_PRE_2019_EXTENDED[3],
    T7_NORMAL_PRE_2019_EXTENDED[4],
];

static T7_NORMAL_PRE_2019: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: T7_NORMAL_REGULAR,
    extended: T7_NORMAL_PRE_2019_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};
static T7_SETTLEMENT_PRE_2019: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: T7_SETTLEMENT_REGULAR,
    extended: T7_SETTLEMENT_PRE_2019_EXTENDED,
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
    T7_NORMAL_PRE_2019_EXTENDED[2],
    session(17 * 3600 + 30 * 60, 17 * 3600 + 35 * 60 + 30),
    session(17 * 3600 + 35 * 60 + 30, 17 * 3600 + 45 * 60),
];
static PRE_TAC_SETTLEMENT_EXTENDED: &[SessionRule] = &[
    PRE_TAC_NORMAL_EXTENDED[0],
    PRE_TAC_NORMAL_EXTENDED[1],
    session(12 * 3600, 12 * 3600 + 5 * 60 + 30),
    PRE_TAC_NORMAL_EXTENDED[3],
    PRE_TAC_NORMAL_EXTENDED[4],
];
static PRE_TAC_NORMAL: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: T7_NORMAL_REGULAR,
    extended: PRE_TAC_NORMAL_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};
static PRE_TAC_SETTLEMENT: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: T7_SETTLEMENT_REGULAR,
    extended: PRE_TAC_SETTLEMENT_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

// Trade-at-Close launched on 2020-12-01, making the official closing price
// executable through 17:45 and moving post-trading to 17:45-17:50. Current
// detailed specifications preserve these phases and the recurring settlement
// grid. Auctions include their full maximum 30-second random period.
// https://www.wienerborse.at/en/news/vienna-stock-exchange-news/vienna-stock-exchange-extends-trading-hours/
// https://web.archive.org/web/20210127203612id_/https://www.wienerborse.at/en/trading/trading-information/trading-hours/
// https://www.wienerborse.at/uploads/u/cms/files/trading/xetra-t7-detailed-specifications-market-models.pdf
static CURRENT_NORMAL_EXTENDED: &[SessionRule] = &[
    PRE_TAC_NORMAL_EXTENDED[0],
    PRE_TAC_NORMAL_EXTENDED[1],
    PRE_TAC_NORMAL_EXTENDED[2],
    PRE_TAC_NORMAL_EXTENDED[3],
    session(17 * 3600 + 35 * 60 + 30, 17 * 3600 + 45 * 60),
    session(17 * 3600 + 45 * 60, 17 * 3600 + 50 * 60),
];
static CURRENT_SETTLEMENT_EXTENDED: &[SessionRule] = &[
    CURRENT_NORMAL_EXTENDED[0],
    CURRENT_NORMAL_EXTENDED[1],
    session(12 * 3600, 12 * 3600 + 5 * 60 + 30),
    CURRENT_NORMAL_EXTENDED[3],
    CURRENT_NORMAL_EXTENDED[4],
    CURRENT_NORMAL_EXTENDED[5],
];

pub(crate) static VIENNA_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: T7_NORMAL_REGULAR,
    extended: CURRENT_NORMAL_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};
static CURRENT_NORMAL: &StaticHoursProfile = &VIENNA_PROFILE;
static CURRENT_SETTLEMENT: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Vienna,
    regular: T7_SETTLEMENT_REGULAR,
    extended: CURRENT_SETTLEMENT_EXTENDED,
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
