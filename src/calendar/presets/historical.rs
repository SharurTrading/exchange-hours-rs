// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! Point-in-time hours: the profile a venue actually published at an instant.
//!
//! Cutover dates are compared in the venue's **own** local zone, not UTC — a
//! rule change announced for "2016-03-04" took effect on that Chicago date, and
//! comparing in UTC would shift the boundary by hours. Only venues with a
//! recorded change appear here; everything else falls through to
//! [`hours_for_exchange`](super::hours_for_exchange), so this module is an
//! overlay, never a second venue table.

use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::{America, Europe, US};

use crate::calendar::profiles::{
    BLUE_OCEAN_PROFILE, C1_PROFILE_POST_2024_08_26, C1_PROFILE_PRE_2024_08_26,
    CBOT_PROFILE_POST2013, CBOT_PROFILE_PRE2013, CFE_PROFILE, CFE_PROFILE_PRE_2021_12_06,
    CFE_PROFILE_PRE2014, CME_PROFILE_POST2016, CME_PROFILE_PRE2016, EUREX_PROFILE_NO_ASIAN,
    EUREX_PROFILE_WITH_ASIAN, IEX_PROFILE_POST2015, IEX_PROFILE_PRE2015, NYSE_TEXAS_PROFILE,
    from_profile,
};
use crate::calendar::{Exchange, MarketHours};

use super::hours_for_exchange;

/// Builds a cutover date at compile time. Used only to initialise the `const`
/// cutover items below: an invalid literal fails the build during constant
/// evaluation, so no panic path exists at runtime.
const fn cutover(year: i32, month: u32, day: u32) -> NaiveDate {
    match NaiveDate::from_ymd_opt(year, month, day) {
        Some(date) => date,
        None => panic!("invalid hard-coded cutover date"),
    }
}

/// Cboe Options (C1): GTH close extended 09:15 -> 09:25 ET.
const C1_GTH_CUTOVER: NaiveDate = cutover(2024, 8, 26);
/// CME equity index: short window close moved 16:15 -> 16:00 CT.
const CME_CUTOVER: NaiveDate = cutover(2016, 3, 4);
/// EUREX: Asian trading window added.
const EUREX_ASIAN_CUTOVER: NaiveDate = cutover(2018, 12, 10);
/// IEX: extended pre/post sessions introduced.
const IEX_EXTENDED_CUTOVER: NaiveDate = cutover(2015, 8, 21);
/// CFE: overnight ETH introduced.
const CFE_ETH_CUTOVER: NaiveDate = cutover(2014, 6, 1);
/// CFE: queuing period abolished, RTH close moved to 15:00 CT.
const CFE_QUEUING_CUTOVER: NaiveDate = cutover(2021, 12, 6);
/// NYSE Texas: go-live.
const NYSE_TEXAS_GO_LIVE: NaiveDate = cutover(2025, 3, 31);
/// Blue Ocean ATS: production launch.
const BLUE_OCEAN_GO_LIVE: NaiveDate = cutover(2021, 10, 5);
/// CBOT: grain trading hours reduced.
const CBOT_CUTOVER: NaiveDate = cutover(2013, 4, 8);

/// Time-aware market hours: returns the exchange-level `MarketHours` profile appropriate
/// for the provided `as_of` timestamp. Defaults to the latest profile if no historical
/// revision is defined for the exchange.
///
/// Cutover semantics: `as_of` is converted to the venue's local calendar date
/// and compared against the effective date. The new profile applies from the
/// venue-local **midnight** of the effective date — an `as_of` at exactly that
/// midnight already sees the new hours, and one nanosecond before it sees the
/// old ones.
#[must_use]
pub fn hours_for_exchange_as_of(exch: Exchange, as_of: DateTime<Utc>) -> MarketHours {
    match exch {
        // Cboe Options (C1): GTH end extended from 09:15 → 09:25 ET on 2024-08-26.
        Exchange::CboeOptionsC1 => {
            let d = as_of.with_timezone(&America::New_York).date_naive();
            if d < C1_GTH_CUTOVER {
                from_profile(Exchange::CboeOptionsC1, &C1_PROFILE_PRE_2024_08_26)
            } else {
                from_profile(Exchange::CboeOptionsC1, &C1_PROFILE_POST_2024_08_26)
            }
        }

        // CME equity index defaults:
        //  - < 2016-03-04: RTH 08:30–15:15; short window 15:30–16:15
        //  - >= 2016-03-04: RTH 08:30–15:15; short window 15:30–16:00
        Exchange::Cme => {
            let d = as_of.with_timezone(&US::Central).date_naive();
            if d < CME_CUTOVER {
                from_profile(Exchange::Cme, &CME_PROFILE_PRE2016)
            } else {
                from_profile(Exchange::Cme, &CME_PROFILE_POST2016)
            }
        }

        // EUREX: Asian hours added 2018-12-10. Before that, omit the 01:00–08:00 slice.
        Exchange::Eurex => {
            let d = as_of.with_timezone(&Europe::Berlin).date_naive();
            let asian = d >= EUREX_ASIAN_CUTOVER;
            if asian {
                from_profile(Exchange::Eurex, &EUREX_PROFILE_WITH_ASIAN)
            } else {
                from_profile(Exchange::Eurex, &EUREX_PROFILE_NO_ASIAN)
            }
        }

        // IEX: Pre 08:00–09:30 and Post 16:00–17:00 begin 2015-08-21; before then, use RTH only.
        // Source: IEX Trading Alert #2015-015 (2015-07-13), "Effective Friday,
        // August 21, 2015, IEX will begin to rollout extended hours trading with
        // Pre-Market and Post-Market Sessions."
        // (https://iextrading.com/trading/alerts/2015/015/)
        Exchange::Iex => {
            let d = as_of.with_timezone(&America::New_York).date_naive();
            let has_ext = d >= IEX_EXTENDED_CUTOVER;
            if has_ext {
                from_profile(Exchange::Iex, &IEX_PROFILE_POST2015)
            } else {
                from_profile(Exchange::Iex, &IEX_PROFILE_PRE2015)
            }
        }

        // CFE (VIX): before 2014-06-01, approximate as RTH + curb only (no
        // overnight). From 2014-06-01 to 2021-12-05, RTH ends 15:15 CT and ETH
        // resumes at 15:30 after a dead 15-minute queuing period. From
        // 2021-12-06 the queuing period is gone and RTH ends 15:00 CT, flowing
        // seamlessly into ETH until 16:00.
        //
        // Source for the 2021 cutover: Cboe notice C2021102603, "Effective
        // December 6, 2021 … CFE will eliminate the queuing period which occurs
        // between 3:15 p.m. CT and 3:30 p.m. CT, Monday through Friday … CFE
        // will also redefine regular trading hours as 8:30 a.m. CT to 3:00 p.m.
        // CT". The date is corroborated by the CFE Rulebook amendment history,
        // which stamps Rule 1202 and seven other rules "December 6, 2021
        // (21-028)". Rule filing CFE-2021-028 (2021-11-04) carries the redline.
        Exchange::Cfe => {
            let d = as_of.with_timezone(&US::Central).date_naive();
            if d < CFE_ETH_CUTOVER {
                from_profile(Exchange::Cfe, &CFE_PROFILE_PRE2014)
            } else if d < CFE_QUEUING_CUTOVER {
                from_profile(Exchange::Cfe, &CFE_PROFILE_PRE_2021_12_06)
            } else {
                from_profile(Exchange::Cfe, &CFE_PROFILE)
            }
        }

        // NYSE Texas: go-live 2025-03-31; before that, no trading sessions.
        Exchange::NyseTexas => {
            let d = as_of.with_timezone(&America::New_York).date_naive();
            if d < NYSE_TEXAS_GO_LIVE {
                MarketHours {
                    exchange: Exchange::NyseTexas,
                    tz: America::New_York,
                    regular: Vec::new().into(),
                    extended: Vec::new().into(),
                    has_daily_close: true,
                    has_weekend_close: true,
                }
            } else {
                from_profile(Exchange::NyseTexas, &NYSE_TEXAS_PROFILE)
            }
        }

        // Blue Ocean ATS: production launch 2021-10-05; prior to that, no sessions.
        // Source: Blue Ocean Technologies, "Announcing Launch of Blue Ocean ATS
        // Afterhours Trading" (2021-10-05) — "the official launch of The Blue
        // Ocean ATS, known as BOATS. After going live in beta in June 2021…"
        // (https://blueocean-tech.io/2021/10/05/announcing-launch-of-blue-ocean-ats-afterhours-trading/)
        // The June 2021 beta is deliberately excluded: no primary source gives it
        // a day-level date, and pre-production beta liquidity is not tradable
        // history. The 20:00→04:00 ET Sun–Thu mask matches the venue's SEC Form
        // ATS-N, which also explains the missing Friday session (the NYSE TRF is
        // unavailable to report Saturday).
        Exchange::BlueOceanAts => {
            let d = as_of.with_timezone(&America::New_York).date_naive();
            if d < BLUE_OCEAN_GO_LIVE {
                MarketHours {
                    exchange: Exchange::BlueOceanAts,
                    tz: America::New_York,
                    regular: Vec::new().into(),
                    extended: Vec::new().into(),
                    has_daily_close: true,
                    has_weekend_close: true,
                }
            } else {
                from_profile(Exchange::BlueOceanAts, &BLUE_OCEAN_PROFILE)
            }
        }

        // CBOT: reduced hours effective 2013-04-08; before that, use 17:00–07:45 overnight and 08:30–13:15 day.
        Exchange::Cbot => {
            let d = as_of.with_timezone(&US::Central).date_naive();
            if d < CBOT_CUTOVER {
                from_profile(Exchange::Cbot, &CBOT_PROFILE_PRE2013)
            } else {
                from_profile(Exchange::Cbot, &CBOT_PROFILE_POST2013)
            }
        }

        // Default: return the current exchange profile.
        _ => hours_for_exchange(exch),
    }
}
