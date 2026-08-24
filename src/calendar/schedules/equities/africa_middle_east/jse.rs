// SPDX-License-Identifier: MIT-0

//! Johannesburg Stock Exchange main/liquid ZA01 equity segment.

use chrono_tz::Africa;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static JSE_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 16 * 3600 + 50 * 60,
}];
// No ZA01 phase is order-entry-only, so `order_entry` stays empty on every
// profile below. The two bounds here are the opening and closing auction call
// sessions, each of which uncrosses into a printed auction trade, and the CPX
// and EOD tails are crossing sessions that print at the closing price.
static JSE_OPEN_CLOSE_CURRENT: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 30 * 60,
        close_ssm: 9 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 50 * 60,
        close_ssm: 17 * 3600,
    },
];
static JSE_OPEN_CLOSE_PRE_2012: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600 + 35 * 60,
        close_ssm: 9 * 3600,
    },
    JSE_OPEN_CLOSE_CURRENT[1],
];
static JSE_EXTENDED_CURRENT: &[SessionRule] = &[
    JSE_OPEN_CLOSE_CURRENT[0],
    JSE_OPEN_CLOSE_CURRENT[1],
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 2 * 60,
        close_ssm: 17 * 3600 + 10 * 60,
    },
];
static JSE_EXTENDED_2021_02_01: &[SessionRule] = &[
    JSE_OPEN_CLOSE_CURRENT[0],
    JSE_OPEN_CLOSE_CURRENT[1],
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 60,
        close_ssm: 17 * 3600 + 10 * 60,
    },
];
static JSE_EXTENDED_CPX_ONLY: &[SessionRule] = &[
    JSE_OPEN_CLOSE_CURRENT[0],
    JSE_OPEN_CLOSE_CURRENT[1],
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 5 * 60,
        close_ssm: 17 * 3600 + 10 * 60,
    },
];
static JSE_EXTENDED_CPX_EOD: &[SessionRule] = &[
    JSE_OPEN_CLOSE_CURRENT[0],
    JSE_OPEN_CLOSE_CURRENT[1],
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 5 * 60,
        close_ssm: 17 * 3600 + 15 * 60,
    },
];

// Current ZA01: opening auction 08:30–09:00, continuous 09:00–16:50,
// closing auction 16:50–17:00, publication gap, then CPX 17:02–17:10. The
// EOD/GDX auction remains disabled; ZA03's midday auction is product-specific.
// Volume 00E v4.09 and the May 2026 session workbook were published with
// Release 7.8 on 2026-08-17; the release changed no ZA01 session boundary.
// https://clientportal.jse.co.za/technical-library/trading-and-market-data-documentation
// https://clientportal.jse.co.za/Content/JSE%20Contract%20Specification%20Items/Volume%2000E%20-%20Trading%20and%20Information%20Overview%20for%20Equity%20Market%20v4.09.pdf
// https://clientportal.jse.co.za/Content/JSE%20Contract%20Specification%20Items/JSE%20Trading%20Session%20Times%20May%202026.xls
pub(crate) static JSE_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Africa::Johannesburg,
    regular: JSE_REGULAR,
    extended: JSE_EXTENDED_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// The following primary notices date every modeled change: opening auction
// 08:30 from 2012-07-02; CPX from 2013-11-11; EOD auction from 2016-09-26;
// EOD disabled 2020-08-24; CPX start 17:01 from 2021-02-01 and 17:02 from
// 2021-02-15.
// https://clientportal.jse.co.za/Content/JSENoticesandCircularsItems/20120525-049C.pdf
// https://clientportal.jse.co.za/Content/JSENoticesandCircularsItems/2013_158B.pdf
// https://clientportal.jse.co.za/Content/JSENoticesandCircularsItems/461A.pdf
// https://clientportal.jse.co.za/Content/JSEHotlinesItems/JSE%20Service%20Hotline%2018520%20EDM%2C%20EQM%20and%20FXM%20-%20JSE%20Trading%20and%20Information%20System%20Upgrade%20-%20Final%20Go%20Live%20Cutover.pdf
// https://clientportal.jse.co.za/Content/JSEHotlinesItems/JSE%20Service%20Hotline%2028220%20EQM%20-%20JSE%20Closing%20Price%20Cross%20%28CPX%29%20Session%20Extension.pdf
// https://clientportal.jse.co.za/Content/JSEHotlinesItems/JSE%20Service%20Hotline%2003721%20EQM%20-%20JSE%20Closing%20Price%20Cross%20%28CPX%29%20Session%20Extension.pdf
pub(crate) static JSE_PROFILE_POST_2021_02_01: StaticHoursProfile = StaticHoursProfile {
    tz: Africa::Johannesburg,
    regular: JSE_REGULAR,
    extended: JSE_EXTENDED_2021_02_01,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static JSE_PROFILE_POST_2020_08_24: StaticHoursProfile = StaticHoursProfile {
    tz: Africa::Johannesburg,
    regular: JSE_REGULAR,
    extended: JSE_EXTENDED_CPX_ONLY,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static JSE_PROFILE_POST_2016_09_26: StaticHoursProfile = StaticHoursProfile {
    tz: Africa::Johannesburg,
    regular: JSE_REGULAR,
    extended: JSE_EXTENDED_CPX_EOD,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static JSE_PROFILE_POST_2013_11_11: StaticHoursProfile = StaticHoursProfile {
    tz: Africa::Johannesburg,
    regular: JSE_REGULAR,
    extended: JSE_EXTENDED_CPX_ONLY,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static JSE_PROFILE_POST_2012_07_02: StaticHoursProfile = StaticHoursProfile {
    tz: Africa::Johannesburg,
    regular: JSE_REGULAR,
    extended: JSE_OPEN_CLOSE_CURRENT,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static JSE_PROFILE_PRE_2012_07_02: StaticHoursProfile = StaticHoursProfile {
    tz: Africa::Johannesburg,
    regular: JSE_REGULAR,
    extended: JSE_OPEN_CLOSE_PRE_2012,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &JSE_PROFILE_CURRENT;

static REVISIONS: &[Revision] = revisions![
    (
        2012,
        7,
        2,
        &JSE_PROFILE_POST_2012_07_02,
        "JSE notice 20120525-049C"
    ),
    (
        2013,
        11,
        11,
        &JSE_PROFILE_POST_2013_11_11,
        "JSE notice 2013_158B"
    ),
    (2016, 9, 26, &JSE_PROFILE_POST_2016_09_26, "JSE notice 461A"),
    (
        2020,
        8,
        24,
        &JSE_PROFILE_POST_2020_08_24,
        "JSE Service Hotline 18520"
    ),
    (
        2021,
        2,
        1,
        &JSE_PROFILE_POST_2021_02_01,
        "JSE Service Hotline 28220"
    ),
    (
        2021,
        2,
        15,
        &JSE_PROFILE_CURRENT,
        "JSE Service Hotline 03721"
    ),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &JSE_PROFILE_PRE_2012_07_02,
        REVISIONS,
    )
}
