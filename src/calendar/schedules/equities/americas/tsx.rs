// SPDX-License-Identifier: MIT-0

//! Toronto Stock Exchange cash equities.

use chrono_tz::America;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// TSX accepts orders from 07:00, trades continuously 09:30–16:00, can run a
// conditional MOC Price Movement Extension through 16:10, and trades at last
// sale 16:15–17:00. The PME rule is the venue's maximum envelope: on ordinary
// days/symbols that interval is cancel-only. Regulator records show both PME
// and the last-sale session existed before the January-2010 history floor; the
// exchange archive contains no later boundary change.
// Sources:
// https://www.tsx.com/en/trading/calendars-and-trading-hours/trading-hours
// https://www.osc.ca/sites/default/files/pdfs/bulletins/oscb_20050114_2802.pdf
static TSX_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 16 * 3600,
}];
// Order entry only. The venue's session table describes Pre-Open as a phase in
// which orders may be entered but will not be executed; the first print of the
// day is the 09:30 Market-on-Open cross that starts continuous trading, so no
// trade can match inside 07:00–09:30.
// https://www.tsx.com/en/trading/calendars-and-trading-hours/trading-hours
static TSX_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600,
    close_ssm: 9 * 3600 + 30 * 60,
}];
// Both close-side phases stay tradeable. When the Price Movement Extension
// fires it is the delayed MOC cross for that symbol and prints, so its maximum
// envelope is not order-entry-only even though ordinary days are cancel-only;
// Extended Trading executes at the last sale price. The separate 16:10–16:15
// Post Market Cancel Session is not modeled at all.
static TSX_EXTENDED: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600,
        close_ssm: 16 * 3600 + 10 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 16 * 3600 + 15 * 60,
        close_ssm: 17 * 3600,
    },
];
pub(crate) static TSX_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: America::Toronto,
    regular: TSX_REGULAR,
    extended: TSX_EXTENDED,
    order_entry: TSX_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

pub(crate) const CURRENT: &StaticHoursProfile = &TSX_PROFILE;
