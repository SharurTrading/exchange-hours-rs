// SPDX-License-Identifier: MIT-0

//! Eurex benchmark-index futures and EEX Nordic Zonal Power Futures.

use chrono_tz::{Europe, UTC};

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::{effective_date, local_date, reference_delta_seconds};

// The Eurex default is FESX/FDAX/FDXM benchmark index futures, not a
// venue-wide clock. Official 2009, 2013, 2015, and 2017 archived specifications
// pin the January-2010-to-cutover grid: 07:30-07:50 pre-trading followed by
// 07:50-22:00 continuous trading. Circular 088/2018 makes the extension
// effective 2018-12-10 and its redline preserves that predecessor grid.
// https://www.eurex.com/resource/blob/296888/978b08fe3a240a0b4a8fb62a2647a197/data/cs_history_26102009_en.pdf.pdf
// https://www.eurex.com/resource/blob/337416/2598be26dcb9b5521549169d5e8b9e8e/data/2013_09_25_cs_history_en.pdf.pdf
// https://www.eurex.com/resource/blob/298554/d1fb9a8ac7a259104ea157830860e080/data/2015_10_28_cs_1_history_en.pdf
// https://www.eurex.com/resource/blob/317412/ff50dcdf5143258c382b4f682cbaf37b/data/2017_08_01_cs_1_history_en.pdf
// https://www.eurex.com/resource/blob/1412768/e61a2c41d65ad165af7909002223b943/data/er18088e.pdf
// The launch phase diagram distinguishes 10 minutes of pre-trading and a
// five-minute opening auction before continuous trading starts at 01:15 CET /
// 02:15 CEST. The following continuous interval is regular. The seasonal open
// remains a fixed 00:00 UTC instant. Current Annex C retains the same product
// grid.
// https://www.eurex.com/resource/blob/1448250/29a4179e4d28742af5d0ee85f9af89f8/data/Eurex%20Asian%20Trading%20Hours_Nov%202018.pdf
// https://www.eurex.com/resource/blob/2824010/3b94b95cdf5f31cc635294659a5e9786/data/2026_05_04_eurex_d_kontraktspezifikationen_annexe_en.pdf
// https://www.eurex.com/ex-en/trade/trading-hours
// https://www.eurex.com/ex-en/trade/trading-hours/trading-phases
// Classification note: the two non-continuous phases are not the same kind of
// window. Pre-trading is order entry only - Eurex's trading-phases page defines
// it as the phase in which orders and quotes are entered, modified and deleted
// while the order book is not executable - so the first 10 minutes are
// `order_entry`. The opening auction that follows it matches and prints at the
// auction price, so those five minutes stay `extended`. The pre-2018 grid names
// its whole 07:30-07:50 window pre-trading, with the start of trading at 07:50,
// so all of it is `order_entry`.
static EUREX_PRE_2018_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 50 * 60,
    close_ssm: 22 * 3600,
}];
// 07:30-07:50 pre-trading: order entry only, no matching.
static EUREX_PRE_2018_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 30 * 60,
    close_ssm: 7 * 3600 + 50 * 60,
}];
static EUREX_WINTER_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3600 + 15 * 60,
    close_ssm: 22 * 3600,
}];
// 01:00-01:10 CET pre-trading: order entry only.
static EUREX_WINTER_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3600,
    close_ssm: 3600 + 10 * 60,
}];
// 01:10-01:15 CET opening auction: a trade prints at the auction price, so this
// window is tradeable and stays `extended`.
static EUREX_WINTER_EXTENDED: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 3600 + 10 * 60,
    close_ssm: 3600 + 15 * 60,
}];
pub(crate) static EUREX_CURRENT_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 2 * 3600 + 15 * 60,
    close_ssm: 22 * 3600,
}];
// 02:00-02:10 CEST pre-trading: order entry only.
pub(crate) static EUREX_CURRENT_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 2 * 3600,
    close_ssm: 2 * 3600 + 10 * 60,
}];
// 02:10-02:15 CEST opening auction: a trade prints at the auction price, so this
// window is tradeable and stays `extended`.
pub(crate) static EUREX_CURRENT_EXTENDED: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 2 * 3600 + 10 * 60,
    close_ssm: 2 * 3600 + 15 * 60,
}];

static EUREX_PRE_2018: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_PRE_2018_REGULAR,
    extended: &[],
    order_entry: EUREX_PRE_2018_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};
static EUREX_WINTER: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_WINTER_REGULAR,
    extended: EUREX_WINTER_EXTENDED,
    order_entry: EUREX_WINTER_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static EUREX_CURRENT: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EUREX_CURRENT_REGULAR,
    extended: EUREX_CURRENT_EXTENDED,
    order_entry: EUREX_CURRENT_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

const EUREX_ASIAN_HOURS: chrono::NaiveDate = effective_date(2018, 12, 10);

pub(crate) fn eurex_profile_at(
    as_of: chrono::DateTime<chrono::Utc>,
) -> &'static StaticHoursProfile {
    if local_date(as_of, Europe::Berlin) < EUREX_ASIAN_HOURS {
        return &EUREX_PRE_2018;
    }
    if reference_delta_seconds(as_of, Europe::Berlin, UTC) == -2 * 3600 {
        &EUREX_CURRENT
    } else {
        &EUREX_WINTER
    }
}

// EEX does not have one venue-wide grid. This default is Nordic Zonal Power
// Futures. The official customer information gives both their 2024-03-25
// launch and the 08:00-18:00 CE(S)T trading table. The current derivatives
// timetable retains that grid, while the Trading Conditions make product
// hours controlling and define exchange days as Monday-Friday.
// https://www.eex.com/fileadmin/Global/News/EEX/EEX_Customer_Information/2024/20240109_EEX_Customer_Information_Nordic_Zonal_Futures.pdf
// https://www.eex.com/fileadmin/EEX/Downloads/Trading/Trading_Hours/20250701_Trading_Hours_on_EEX_Derivatives_Markets_.pdf
// https://www.eex.com/fileadmin/EEX/Downloads/Rules/Trading_Conditions/20260513_EEX_Trading_Conditions_0073a_E_FINAL.pdf
static EEX_POWER_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 18 * 3600,
}];
pub(crate) static EEX_PROFILE: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: EEX_POWER_REGULAR,
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};
static EEX_CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: Europe::Berlin,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

const EEX_NORDIC_LAUNCH: chrono::NaiveDate = effective_date(2024, 3, 25);

pub(crate) fn eex_profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    if local_date(as_of, Europe::Berlin) < EEX_NORDIC_LAUNCH {
        &EEX_CLOSED
    } else {
        &EEX_PROFILE
    }
}
