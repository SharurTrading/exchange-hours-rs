// SPDX-License-Identifier: MIT-0

//! SIX Swiss Exchange cash equities.

use chrono_tz::Europe;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

// SIX Swiss Exchange — shares segments (Blue Chip / Mid-/Small-Cap), which is
// what `Exchange::Six` denotes. SIX does NOT follow the Xetra pattern:
// continuous trading ends at 17:20, the closing auction starts at 17:20 and
// can uncross as late as 17:32, and Trading-At-Last then runs to 17:40,
// followed by order-entry-only post-trading through 22:00. The 17:30–17:35
// auction belongs to the ETF/ETP/Sponsored Funds segments only, which have no
// TAL.
//
// The Trading Guide's 09:00 opening is randomized over two minutes. The
// deterministic profile therefore keeps the auction/pre-opening classification
// through 09:01:59 and starts regular trading at the latest possible edge,
// 09:02. Its segment row is "Blue Chip Shares 06:00 09:00 17:20 17:30 17:30
// 17:40 22:00"; the current page confirms the two-minute opening slot.
//
// Trading Guide, Blue Chip Shares: "Trading Hours 09:00 - 17:30 CET /
// Continuous Trading 09:00 - 17:20 CET / Closing Auction 17:20 - 17:30 CET /
// Trading-At-Last Start: 17:30 - 17:32 CET End: 17:40 CET".
// Sources: SIX Group, "Trading hours"
// (https://www.six-group.com/en/products-services/the-swiss-stock-exchange/trading/trading-provisions/trading-hours.html)
// and the SIX Swiss Exchange Trading Guide
// (https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/trading-provisions/regulation/trading-guides/trading-guide.pdf).
// SIX's official guide valid from 2018-05-28 records the same Blue Chip grid,
// including the two-minute randomized opening and closing auction windows.
// https://www.six-group.com/dam/download/sites/education/preparatory-documentation/trading-module/trading-guide.pdf
//
// The January-2010 baseline is independently established by operator archives.
// Directive 1, effective 2007-09-07, gives exchange hours 06:00-22:00,
// pre-opening from 06:00 until the opening, and post-trading from the close
// through 22:00. The Equity Market Product Guide valid from 2009-07-22 gives
// the exact shares grid used below: continuous 09:00-17:20, closing auction
// 17:20-17:30, and two-minute randomized opening and closing windows ending at
// 09:02 and 17:32 respectively.
// https://web.archive.org/web/20081123115341id_/http://www.six-swiss-exchange.com/download/trading/regulation/directives/swx_dir01_en.pdf
// https://web.archive.org/web/20090824132532id_/http://www.six-swiss-exchange.com:80/download/marketpulse/news/newsboard/product_guides/product_guide_equities_en.pdf
//
// SIX labels its times "CET" year-round; they are local Zurich wall-clock, so
// `Europe::Zurich` (CET/CEST) is the correct zone, not a fixed offset.
static SIX_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3600 + 2 * 60,
    close_ssm: 17 * 3600 + 20 * 60,
}];
static SIX_EXTENDED_CURRENT: &[SessionRule] = &[
    // Pre-opening: order entry with a theoretical opening price, from the start
    // of the business day until the opening auction uncrosses at 09:00. No
    // continuous trading happens in this window.
    SessionRule {
        days: MON_FRI,
        open_ssm: 6 * 3600,
        close_ssm: 9 * 3600 + 2 * 60,
    },
    // Closing auction.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 20 * 60,
        close_ssm: 17 * 3600 + 32 * 60,
    },
    // Trading-At-Last: execution at the closing price after the auction.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 32 * 60,
        close_ssm: 17 * 3600 + 40 * 60,
    },
    // Post-trading: orders for a future day may be entered but cannot execute.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 40 * 60,
        close_ssm: 22 * 3600,
    },
];
static SIX_EXTENDED_PRE_TAL: &[SessionRule] = &[
    SIX_EXTENDED_CURRENT[0],
    // Before TAL, the randomized closing auction itself kept the venue open
    // for as long as two minutes after its nominal 17:30 run time.
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 20 * 60,
        close_ssm: 17 * 3600 + 32 * 60,
    },
    SessionRule {
        days: MON_FRI,
        open_ssm: 17 * 3600 + 32 * 60,
        close_ssm: 22 * 3600,
    },
];

pub(crate) static SIX_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Zurich,
    regular: SIX_REGULAR,
    extended: SIX_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};
static SIX_PROFILE_PRE_2020_06_22: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Zurich,
    regular: SIX_REGULAR,
    extended: SIX_EXTENDED_PRE_TAL,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{Revision, effective_date, local_date, select_revision};

// Trading-At-Last launched with SMR8.2 on 2020-06-22. The readiness document
// gives both the production date and the added 17:30-17:40 phase.
// https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/participation/SWXess-maintenance-releases/smr82_participant_readiness.pdf
static REVISIONS: &[Revision] = &[Revision {
    effective: effective_date(2020, 6, 22),
    profile: &SIX_PROFILE,
}];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(
        local_date(as_of, Europe::Zurich),
        &SIX_PROFILE_PRE_2020_06_22,
        REVISIONS,
    )
}
