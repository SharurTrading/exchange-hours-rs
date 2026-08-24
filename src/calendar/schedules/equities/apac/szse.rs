// SPDX-License-Identifier: MIT-0

//! Shenzhen Stock Exchange cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static CHINA_REGULAR_WITH_CLOSE_CALL: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 30 * 60,
        close_ssm: 11 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 13 * 3600,
        close_ssm: 14 * 3600 + 57 * 60,
    },
];
static CHINA_EXTENDED_CORE: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600 + 15 * 60,
        close_ssm: 9 * 3600 + 25 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 14 * 3600 + 57 * 60,
        close_ssm: 15 * 3600,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 15 * 3600,
        close_ssm: 15 * 3600 + 30 * 60,
    },
];
// The pre-2016 SZSE Trading Rules (2013 revision) set the opening call auction
// at 09:15–09:25 and continuous auction from 09:30 (Art. 2.4.2), and Art. 3.3.1
// adds that "每个交易日 9:25 至 9:30，交易主机只接受申报，但不对买卖申报或撤销
// 申报作处理" — from 09:25 to 09:30 the trading host only accepts declarations
// and processes neither orders nor cancellations. The opening uncrossing has
// already printed at 09:25 and continuous matching does not start until 09:30,
// so this window is order entry, not tradeable time. SZSE stopped accepting
// orders in it on 2016-05-09, which is why it exists only in this era.
// https://docs.static.szse.cn/www/disclosure/notice/W020180328432928783546.pdf
static SZSE_ORDER_ENTRY_PRE_2016: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 25 * 60,
    close_ssm: 9 * 3600 + 30 * 60,
}];
// SZSE has accepted block-trading declarations through 15:30 since before the
// January-2010 audit floor. The 2026-07-06 generic fixed-price expansion
// changed eligibility inside that existing venue envelope, not the
// exchange-level close. Block and fixed-price phases are extended by
// convention; not every security is eligible for them.
// https://www.szse.cn/lawrules/rule/trade/current/t20260424_620190.html
// Block-trading rule effective 2006-07-01:
// https://www.szse.cn/disclosure/notice/general/t20060515_499577.html
pub(crate) static SZSE_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Shanghai,
    regular: CHINA_REGULAR_WITH_CLOSE_CALL,
    extended: CHINA_EXTENDED_CORE,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
// SZSE stopped accepting orders in its 09:25–09:30 blocking interval on
// 2016-05-09; the session intervals otherwise stayed unchanged.
// https://www.szse.cn/aboutus/trends/news/t20160930_518722.html
pub(crate) static SZSE_PROFILE_POST_2016_05_09: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Shanghai,
    regular: CHINA_REGULAR_WITH_CLOSE_CALL,
    extended: CHINA_EXTENDED_CORE,
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static SZSE_PROFILE_PRE_2016_05_09: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Shanghai,
    regular: CHINA_REGULAR_WITH_CLOSE_CALL,
    extended: CHINA_EXTENDED_CORE,
    order_entry: SZSE_ORDER_ENTRY_PRE_2016,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &SZSE_PROFILE_CURRENT;

static REVISIONS: &[Revision] = revisions![(
    2016,
    5,
    9,
    &SZSE_PROFILE_POST_2016_05_09,
    "SZSE notice t20160930_518722"
),];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &SZSE_PROFILE_PRE_2016_05_09,
        REVISIONS,
    )
}
