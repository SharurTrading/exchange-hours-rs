// SPDX-License-Identifier: MIT-0

//! US listed-equity-options profiles.

use chrono::{DateTime, Utc};
use chrono_tz::America;

use super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

static CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: America::New_York,
    regular: &[],
    extended: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// Every profile in this module is deliberately scoped to the published
// regular-session envelope for ordinary options on individual US stocks. It excludes
// ETF, ETN, index, FLEX, floor-only, and venue-designated extended-hours
// classes, whose closing times and additional sessions vary by product.
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
// The current operator rules retain 09:30–16:00 for this exact product family.
// C1 and MRX now offer additional sessions for venue-designated classes; the
// cited notices expressly preserve a 09:30–16:00 RTH and are excluded above.
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

pub(crate) static CBOE_OPTIONS_C1_PROFILE: StaticHoursProfile = listed_equity_options_profile();
pub(crate) static CBOE_C2_OPTIONS_PROFILE: StaticHoursProfile = listed_equity_options_profile();
pub(crate) static CBOE_BZX_OPTIONS_PROFILE: StaticHoursProfile = listed_equity_options_profile();
pub(crate) static CBOE_EDGX_OPTIONS_PROFILE: StaticHoursProfile = listed_equity_options_profile();
pub(crate) static NYSE_ARCA_OPTIONS_PROFILE: StaticHoursProfile = listed_equity_options_profile();
pub(crate) static NYSE_AMERICAN_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile();
pub(crate) static NASDAQ_PHLX_PROFILE: StaticHoursProfile = listed_equity_options_profile();
pub(crate) static NASDAQ_ISE_PROFILE: StaticHoursProfile = listed_equity_options_profile();
pub(crate) static NASDAQ_NOM_PROFILE: StaticHoursProfile = listed_equity_options_profile();
pub(crate) static NASDAQ_MRX_PROFILE: StaticHoursProfile = listed_equity_options_profile();
pub(crate) static NASDAQ_GEMX_PROFILE: StaticHoursProfile = listed_equity_options_profile();
pub(crate) static NASDAQ_BX_OPTIONS_PROFILE: StaticHoursProfile = listed_equity_options_profile();
pub(crate) static MIAX_OPTIONS_PROFILE: StaticHoursProfile = listed_equity_options_profile();
pub(crate) static MIAX_EMERALD_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile();
pub(crate) static MIAX_PEARL_OPTIONS_PROFILE: StaticHoursProfile = listed_equity_options_profile();
pub(crate) static MIAX_SAPPHIRE_OPTIONS_PROFILE: StaticHoursProfile =
    listed_equity_options_profile();
pub(crate) static BOX_OPTIONS_PROFILE: StaticHoursProfile = listed_equity_options_profile();
pub(crate) static MEMX_OPTIONS_PROFILE: StaticHoursProfile = listed_equity_options_profile();

// Launch evidence for venues that began after the January-2010 audit floor.
// BZX launched 2010-02-26 with cash-equity underliers among its 18 classes:
// https://cdn.cboe.com/resources/press_releases/BATSOptionsGoesLive_FINAL.pdf
// C2 launched 2010-10-29 with Ford as its first class:
// https://cdn.cboe.com/resources/regulation/circulars/general/IC-CBOE-2010-168.pdf
// EDGX launched phase one on 2015-11-02; its schedule lists five stock classes:
// https://cdn.cboe.com/resources/release_notes/2015/BATS-EDGX-Options-Update-2015_11_10.pdf
// https://cdn.cboe.com/resources/edgx_options/EDGX_Options_Symbol_Rollout_Schedule.xlsx
// BX launched 2012-06-29 with five stock classes, including AA and INTC:
// https://www.nasdaqtrader.com/MicroNews.aspx?id=OTA2012-41
// GEMX's audited filing calls it an equity-and-index-options exchange and
// states that it formally commenced trading on 2013-08-05:
// https://www.sec.gov/Archives/edgar/vprr/1601/16019242.pdf
// MRX initiated trading on 2016-02-16; the operator reports an equity-and-ETF
// rollout, while the SEC filing independently fixes the exact launch day:
// https://www.deutsche-boerse.com/resource/blob/324026/912f25fc1b9e0cdb916acbd69d4013fb/data/Detailed_volume_statistics_are_found_in_the_following_document-1.pdf
// https://www.sec.gov/files/rules/sro/bats/2016/34-77256.pdf
// MIAX launched 2012-12-07 with stock class CLF:
// https://www.miaxglobal.com/alert/2012/12/06/miax-options-will-commence-trading-friday-december-7-2012
// MIAX Pearl launched 2017-02-06 with IBM:
// https://www.miaxglobal.com/alerts/2017/02/01/market-underlying-security-used-openings-miax-pearl-newly-listed-option-class
// MIAX Emerald launched 2019-03-01 with IBM:
// https://www.miaxglobal.com/news/miax-emerald-successfully-launches-trading-operations
// MIAX Sapphire launched 2024-08-12 with IBM:
// https://www.miaxglobal.com/sites/default/files/alert-files/MIAX_Press_Release_09102024.pdf
// MEMX launched 2023-09-27 with stock classes SBUX and IMGN (plus GLD):
// https://info.memxtrading.com/trader-alert-23-42-memx-options-exchange-schedule-update/
static BZX_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2010, 2, 26),
    profile: &CBOE_BZX_OPTIONS_PROFILE,
}];
static C2_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2010, 10, 29),
    profile: &CBOE_C2_OPTIONS_PROFILE,
}];
static EDGX_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2015, 11, 2),
    profile: &CBOE_EDGX_OPTIONS_PROFILE,
}];
static BX_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2012, 6, 29),
    profile: &NASDAQ_BX_OPTIONS_PROFILE,
}];
static GEMX_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2013, 8, 5),
    profile: &NASDAQ_GEMX_PROFILE,
}];
static MRX_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2016, 2, 16),
    profile: &NASDAQ_MRX_PROFILE,
}];
static MIAX_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2012, 12, 7),
    profile: &MIAX_OPTIONS_PROFILE,
}];
static MIAX_PEARL_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2017, 2, 6),
    profile: &MIAX_PEARL_OPTIONS_PROFILE,
}];
static MIAX_EMERALD_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2019, 3, 1),
    profile: &MIAX_EMERALD_OPTIONS_PROFILE,
}];
static MIAX_SAPPHIRE_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2024, 8, 12),
    profile: &MIAX_SAPPHIRE_OPTIONS_PROFILE,
}];
static MEMX_REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2023, 9, 27),
    profile: &MEMX_OPTIONS_PROFILE,
}];

pub(crate) fn c1_profile_at(_as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
    &CBOE_OPTIONS_C1_PROFILE
}

macro_rules! launch_selector {
    ($name:ident, $revisions:ident) => {
        pub(crate) fn $name(as_of: DateTime<Utc>) -> &'static StaticHoursProfile {
            select_revision(local_date(as_of, America::New_York), &CLOSED, $revisions)
        }
    };
}

launch_selector!(c2_options_profile_at, C2_REVISIONS);
launch_selector!(bzx_options_profile_at, BZX_REVISIONS);
launch_selector!(edgx_options_profile_at, EDGX_REVISIONS);
launch_selector!(nasdaq_bx_options_profile_at, BX_REVISIONS);
launch_selector!(nasdaq_gemx_profile_at, GEMX_REVISIONS);
launch_selector!(nasdaq_mrx_profile_at, MRX_REVISIONS);
launch_selector!(miax_options_profile_at, MIAX_REVISIONS);
launch_selector!(miax_pearl_options_profile_at, MIAX_PEARL_REVISIONS);
launch_selector!(miax_emerald_options_profile_at, MIAX_EMERALD_REVISIONS);
launch_selector!(miax_sapphire_options_profile_at, MIAX_SAPPHIRE_REVISIONS);
launch_selector!(memx_options_profile_at, MEMX_REVISIONS);

const fn listed_equity_options_profile() -> StaticHoursProfile {
    StaticHoursProfile {
        tz: America::New_York,
        regular: LISTED_EQUITY_OPTIONS_REGULAR,
        extended: &[],
        has_daily_close: true,
        has_weekend_close: true,
    }
}
