// SPDX-License-Identifier: MIT-0

//! US listed-equity-options profiles.

use chrono_tz::America;

use super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// Every profile in this module is deliberately scoped to ordinary options on
// individual US stocks. Generic pre-open order acceptance is part of the
// exchange envelope, but execution in this product family begins at 09:30, so
// those windows are order entry rather than trading. ETF, ETN, index, FLEX,
// floor-only, and venue-designated extended-hours classes remain separate
// product families because their executable sessions vary.
//
// The 2006 coordinated rule changes moved individual-stock options from a
// 16:02 to a 16:00 ET close before this repository's January-2010 history
// floor. Each older venue has its own primary baseline (not merely a shared
// industry inference):
// C1: https://www.sec.gov/rules/sro/cboe/2006/34-53246.pdf
// Arca: https://www.sec.gov/rules/sro/pcx/34-53249.pdf
// American: https://www.sec.gov/rules/sro/amex/2006/34-53244.pdf
// PHLX: https://www.sec.gov/rules/sro/phlx/34-53247.pdf
// ISE: https://www.sec.gov/rules/sro/ise/2006/34-53248.pdf
// BOX: https://www.sec.gov/rules/sro/bse/2006/34-53245.pdf
// NOM's approved rules set 09:30–16:00 for this family, and its launch alert
// identifies AMAT (an individual stock) among the 2008-03-31 launch classes:
// https://www.sec.gov/rules/sro/nasdaq/2008/34-57478.pdf
// https://www.nasdaqtrader.com/MicroNews.aspx?id=OTA2008-001
// The current operator rules retain 09:30–16:00 RTH for this exact product
// family. C1 has an additional session for venue-designated classes. MRX's
// approved Options 3C session remains unencoded until the required trader alert
// makes it operative.
// https://cdn.cboe.com/resources/release_notes/2026/Schedule_Update_C1_Options_to_Offer_GTH_Sessions_for_Multi_List_Options_Series.html
// https://www.sec.gov/rules-regulations/self-regulatory-organization-rulemaking/sr-mrx-2026-11
// https://cdn.cboe.com/resources/regulation/rule_book/C1_Exchange_Rule_Book.pdf
// https://cdn.cboe.com/resources/regulation/rule_book/C2_Exchange_Rule_Book.pdf
// https://cdn.cboe.com/resources/regulation/rule_book/BZX_Exchange_Rulebook.pdf
// https://cdn.cboe.com/resources/regulation/rule_book/EDGX_Rulebook.pdf
// https://nysearcaguide.srorules.com/rules
// https://nyseamericanguide.srorules.com/rules
// https://listingcenter.nasdaq.com/RuleBook/Nasdaq/rules/nasdaq-options-3
// https://listingcenter.nasdaq.com/rulebook/phlx/rules/Phlx%20Options%203
// https://listingcenter.nasdaq.com/rulebook/ise/rules/ISE%20Options%203
// https://listingcenter.nasdaq.com/rulebook/gemx/rules/GEMX%20Options%203
// https://listingcenter.nasdaq.com/rulebook/mrx/rules/MRX%20Options%203
// https://listingcenter.nasdaq.com/rulebook/nasdaqtx/rules/NTX%20Options%203
// https://www.miaxglobal.com/markets/us-options/miax-options/trade-hours-calendar
// https://www.miaxglobal.com/markets/us-options/pearl-options/trade-hours-calendar
// https://www.miaxglobal.com/markets/us-options/emerald-options/trade-hours-calendar
// https://www.miaxglobal.com/markets/us-options/sapphire-options/trade-hours-calendar
// https://rules.boxexchange.com
// https://info.memxtrading.com/market-hours-and-holiday-schedule/
static LISTED_EQUITY_OPTIONS_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 30 * 60,
    close_ssm: 16 * 3600,
}];

// Order-entry-only pre-open queues. Each venue below opens its book to order
// entry, amendment, and cancellation at the stated time, but no contract in
// this product family can match until the opening process runs at 09:30 ET —
// the cited operator system-settings and hours pages describe these windows as
// order acceptance/queuing, and the first execution of the day is the 09:30
// opening. They are therefore `order_entry`, not tradeable extended sessions.
static ORDER_ENTRY_0600: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 6 * 3600,
    close_ssm: 9 * 3600 + 30 * 60,
}];

static ORDER_ENTRY_0700: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600,
    close_ssm: 9 * 3600 + 30 * 60,
}];

static ORDER_ENTRY_0730: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 30 * 60,
    close_ssm: 9 * 3600 + 30 * 60,
}];

// Current ordinary-stock-option order-acceptance edges. The reviewed primary
// sources do not supply a complete day-level amendment chain for these queues,
// so each venue's timeline carries them only from its 2026-08-22
// knowledge-bound row onward while earlier dated queries retain the exact
// 09:30–16:00 execution history. Because nothing matches in a queue, carrying
// them as `order_entry` leaves earlier dated grids — which hold no queue at
// all — unaffected in their tradeable coverage.
// https://www.cboe.com/about/hours/us-options
// https://www.nyse.com/trade/hours-calendars?os=.
// https://www.nasdaq.com/docs/PHLXSystemSettings
// https://www.nasdaq.com/docs/NOMSystemSettings
// https://www.nasdaq.com/docs/ISESystemSettings
// https://www.nasdaq.com/docs/GEMXSystemSettings.pdf
// https://www.nasdaq.com/docs/MRXSystemSettings
// https://nasdaqtrader.com/Content/BXOptions/BXOptions_FAQs.pdf
// https://www.miaxglobal.com/markets/us-options/all-options-exchanges/trade-hours-calendar
// https://boxexchange.com/assets/BOX-Exchange-Quoting-Requirements-Summary_10.15.pdf
// https://info.memxtrading.com/wp-content/uploads/2023/05/MEMX-Options-User-Manual.pdf
pub(crate) static CBOE_OPTIONS_C1_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static CBOE_C2_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static CBOE_BZX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static CBOE_EDGX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static NYSE_ARCA_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0600);
pub(crate) static NYSE_AMERICAN_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0600);
pub(crate) static NASDAQ_PHLX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static NASDAQ_ISE_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0600);
pub(crate) static NASDAQ_NOM_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static NASDAQ_MRX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0600);
pub(crate) static NASDAQ_GEMX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0600);
pub(crate) static NASDAQ_BX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static MIAX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static MIAX_EMERALD_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static MIAX_PEARL_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static MIAX_SAPPHIRE_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0730);
pub(crate) static BOX_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile(ORDER_ENTRY_0700);
pub(crate) static MEMX_OPTIONS_PROFILE: StaticHoursProfile = listed_equity_options_profile(&[]);

static LISTED_EQUITY_OPTIONS_HISTORICAL: StaticHoursProfile = listed_equity_options_profile(&[]);

mod history;

pub(crate) use history::{
    box_options_profile_at, bzx_options_profile_at, c1_profile_at, c2_options_profile_at,
    edgx_options_profile_at, memx_options_profile_at, miax_emerald_options_profile_at,
    miax_options_profile_at, miax_pearl_options_profile_at, miax_sapphire_options_profile_at,
    nasdaq_bx_options_profile_at, nasdaq_gemx_profile_at, nasdaq_ise_profile_at,
    nasdaq_mrx_profile_at, nasdaq_nom_profile_at, nasdaq_phlx_profile_at,
    nyse_american_options_profile_at, nyse_arca_options_profile_at,
};

// This family has no tradeable session outside 09:30–16:00: every non-regular
// window a venue publishes here is a pre-open order-acceptance queue, so
// `extended` is empty and the queue lands in `order_entry`.
const fn listed_equity_options_profile(order_entry: &'static [SessionRule]) -> StaticHoursProfile {
    StaticHoursProfile {
        tz: America::New_York,
        regular: LISTED_EQUITY_OPTIONS_REGULAR,
        extended: &[],
        order_entry,
        has_daily_close: true,
        has_weekend_close: true,
    }
}
