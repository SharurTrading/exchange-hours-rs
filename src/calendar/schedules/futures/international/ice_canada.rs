// SPDX-License-Identifier: MIT-0

//! Closed legacy ICE Futures Canada Canola identity.

use chrono_tz::America;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::SUN_PLUS_MON_THU;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// ICE Futures Canada's Winnipeg Canola profile has a fully sourced baseline
// and four observable revisions before the venue identity closes.
// The official 2009 calendar pins the January-2010 baseline: pre-open 19:00,
// continuous trading 20:00-13:15 CT. The 2011 notice explicitly moves the
// pre-open/open to 18:30/19:00 on Monday 2011-02-28 for trade date 2011-03-01.
// https://www.ice.com/publicdocs/futures_canada/member_notices/Trading_Calendar_2009.pdf
// https://www.ice.com/publicdocs/futures_canada/member_notices/Feb1_2011_revised_trading_hours.pdf
//
// The 2012 notice moves the close to 14:00 for trade date 2012-06-25, whose
// session opened Sunday 2012-06-24. A 2013 reminder restores it to 13:15 for
// trade date 2013-04-08, whose session opened Sunday 2013-04-07. The 2016
// notice then pins the final legacy close extension from 13:15 to 13:20
// beginning trade date 2016-01-25.
// https://www.ice.com/publicdocs/futures_canada/member_notices/June_13_2012_ICE_Futures_Canada_notice-Trading_Hours_and_Settlement_Time_Change.pdf
// https://www.ice.com/publicdocs/futures_canada/member_notices/April_8_2013_Reminder_Closing_time_and_Settlement_time_changes_today.pdf
// https://www.ice.com/publicdocs/futures_canada/member_notices/2016_01_18_Reminder_Canola_Trade_At_Settlement.pdf
//
// The 2017 holiday notice corroborates the final 19:00-13:20 CT grid. The
// 2018 transfer notice removes the product from IFCA at the start of trading
// for trade date 2018-07-30, i.e. the Sunday 2018-07-29 opening. Contract
// specifications were otherwise unchanged.
// https://www.ice.com/publicdocs/futures_canada/member_notices/2017_11_27_Christmas_2017_and_New_Years_2018_Schedules.pdf
// https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US-Notice-Canola-20180501.pdf
static ICE_CANADA_2010_REGULAR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 20 * 3600,
    close_ssm: 13 * 3600 + 15 * 60,
}];
// The phase preceding each open is the pre-open the sources name as such - the
// 2009 calendar's "pre-open 19:00" and the 2011 notice's move of the
// "pre-open/open" to 18:30/19:00. Orders are entered ahead of the open and
// nothing matches until continuous trading starts, so it is order_entry;
// Canola published no tradeable phase outside continuous trading, so the
// extended slices stay empty.
static ICE_CANADA_2010_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 19 * 3600,
    close_ssm: 20 * 3600,
}];
static ICE_CANADA_2010: StaticHoursProfile = StaticHoursProfile {
    tz: America::Winnipeg,
    regular: ICE_CANADA_2010_REGULAR,
    extended: &[],
    order_entry: ICE_CANADA_2010_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

static ICE_CANADA_2011_REGULAR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 19 * 3600,
    close_ssm: 13 * 3600 + 15 * 60,
}];
static ICE_CANADA_2011_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 18 * 3600 + 30 * 60,
    close_ssm: 19 * 3600,
}];
static ICE_CANADA_2011: StaticHoursProfile = StaticHoursProfile {
    tz: America::Winnipeg,
    regular: ICE_CANADA_2011_REGULAR,
    extended: &[],
    order_entry: ICE_CANADA_2011_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

static ICE_CANADA_2012_REGULAR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 19 * 3600,
    close_ssm: 14 * 3600,
}];
static ICE_CANADA_2012: StaticHoursProfile = StaticHoursProfile {
    tz: America::Winnipeg,
    regular: ICE_CANADA_2012_REGULAR,
    extended: &[],
    order_entry: ICE_CANADA_2011_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

static ICE_CANADA_2016_REGULAR: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 19 * 3600,
    close_ssm: 13 * 3600 + 20 * 60,
}];
static ICE_CANADA_2016: StaticHoursProfile = StaticHoursProfile {
    tz: America::Winnipeg,
    regular: ICE_CANADA_2016_REGULAR,
    extended: &[],
    order_entry: ICE_CANADA_2011_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// Canola moved to ICE Futures U.S. for trade date 2018-07-30. This identity is
// closed from that session's Sunday opening, not an alias for the IFUS product.
// https://www.ice.com/historical-volumes-ifus-futures
pub(crate) static ICE_CANADA_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: America::Winnipeg,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

static ICE_CANADA_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2011, 2, 28),
        profile: &ICE_CANADA_2011,
    },
    Revision {
        effective: effective_date(2012, 6, 24),
        profile: &ICE_CANADA_2012,
    },
    Revision {
        effective: effective_date(2013, 4, 7),
        profile: &ICE_CANADA_2011,
    },
    Revision {
        effective: effective_date(2016, 1, 24),
        profile: &ICE_CANADA_2016,
    },
    Revision {
        effective: effective_date(2018, 7, 29),
        profile: &ICE_CANADA_PROFILE,
    },
];

pub(crate) fn ice_canada_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, America::Winnipeg),
        &ICE_CANADA_2010,
        ICE_CANADA_REVISIONS,
    )
}
