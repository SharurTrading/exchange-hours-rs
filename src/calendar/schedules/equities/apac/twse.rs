// SPDX-License-Identifier: MIT-0

//! Taiwan Stock Exchange cash equities.

use chrono_tz::Asia;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static TWSE_REGULAR_CURRENT: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600,
    close_ssm: 13 * 3600 + 25 * 60,
}];
// TWSE's own trading-system page splits the 08:00–17:00 envelope. Paired block
// trading runs 08:00–08:30 and executes on quoted terms; non-paired and paired
// block trading then run 09:00–17:00. Between them, 08:30–09:00 is the regular
// session's order-placing window whose call-auction match lands at 09:00, and
// no block-trading window is open, so nothing can print in that half hour.
// https://www.twse.com.tw/en/products/system/trading.html
static TWSE_EXTENDED_ENVELOPE: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 8 * 3600,
        close_ssm: 8 * 3600 + 30 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3600,
        close_ssm: 17 * 3600,
    },
];
// Opening call-auction order collection: orders may be placed, amended and
// cancelled, but the first match is the 09:00 opening call.
static TWSE_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 9 * 3600,
}];

// TWSE's central order book trades continuously 09:00–13:25, bounded by
// opening and closing calls. Its paired block window begins at 08:00 and block
// trading continues through 17:00, so the exchange-level availability
// envelope spans 08:00–17:00, broken only by the 08:30–09:00 order-collection
// window that now sits in order_entry. Specialized block, odd-lot, auction, and
// after-hours methods are classified extended; not every security is eligible
// for every phase.
// https://www.twse.com.tw/en/products/system/trading.html
pub(crate) static TWSE_PROFILE_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Taipei,
    regular: TWSE_REGULAR_CURRENT,
    extended: TWSE_EXTENDED_ENVELOPE,
    order_entry: TWSE_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// Before continuous trading launched 2020-03-23, central-book intraday matches
// were call auctions every five seconds, so that primary session is extended
// rather than mislabeled regular. TWSE's own investor guide states that it had
// already expanded block trading to the present 08:00/17:00 envelope at the
// beginning of 2009, before the January-2010 audit floor. No unsourced
// pre-floor effective day is encoded.
// https://www.twse.com.tw/en/about/company/history.html
// https://www.twse.com.tw/en/about/company/guide.html
pub(crate) static TWSE_PROFILE_PRE_2020_03_23: StaticHoursProfile = StaticHoursProfile {
    tz: Asia::Taipei,
    regular: &[],
    extended: TWSE_EXTENDED_ENVELOPE,
    order_entry: TWSE_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

pub(crate) const CURRENT: &StaticHoursProfile = &TWSE_PROFILE_CURRENT;

static REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2020, 3, 23),
    profile: &TWSE_PROFILE_CURRENT,
}];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, CURRENT.tz),
        &TWSE_PROFILE_PRE_2020_03_23,
        REVISIONS,
    )
}
