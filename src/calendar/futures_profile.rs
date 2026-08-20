// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! Named futures profiles, addressed by [`MarketHoursKey`] instead of by venue.
//!
//! Futures hours track the *product family*, not the exchange: CME equity-index
//! and CME FX contracts list on the same venue and keep different hours, while
//! NYMEX energy, COMEX metals, CBOT rates, and CME FX all share one 23-hour
//! Globex schedule. Keying on the family is therefore the correct granularity,
//! and it is the surface `instrument-catalog` maps instrument roots onto.
//!
//! [`MarketHoursKey`] is `#[non_exhaustive]`: adding a family must not break a
//! downstream match. The tables here are the same statics the
//! [`Exchange`]-keyed presets use — one source of truth, two addressing schemes.

use std::borrow::Cow;

use chrono::{DateTime, Utc};
use chrono_tz::UTC;
use chrono_tz::{America, Asia, Europe, Tz, US};
use serde::{Deserialize, Serialize};

use super::profiles::{
    CBOT_EXT_POST2013, CBOT_REGULAR_POST2013, CFE_EXTENDED, CFE_REGULAR, CME_EXT_POST2016,
    CME_REGULAR, EUREX_ASIAN, EUREX_REGULAR, ICE_WRAP_20_18_EXT, MAINT_17_16_EXT, SGX_EXTENDED,
    SGX_REGULAR,
};
use super::rule::ALL_DAYS;
use super::{Exchange, MarketHours, SessionRule};

/// A timezone-aware set of normal-week futures session rules.
///
/// `regular` carries primary trading sessions; `extended` carries electronic,
/// overnight, and other non-regular sessions. Holiday and special-session
/// overlays are deliberately modeled outside this normal-week profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FuturesSessionProfile {
    /// Exchange local timezone used to interpret `SessionRule` SSM values.
    pub tz: Tz,
    /// Primary trading sessions.
    pub regular: &'static [SessionRule],
    /// Electronic, overnight, and other non-regular sessions.
    pub extended: &'static [SessionRule],
    /// True when the venue has a distinct daily close.
    pub has_daily_close: bool,
    /// True when the venue has a true weekend close.
    pub has_weekend_close: bool,
}

impl FuturesSessionProfile {
    /// Returns `true` when any regular or extended normal-week session is active.
    #[must_use]
    pub fn is_open(&self, t: DateTime<Utc>) -> bool {
        self.to_market_hours(Exchange::Unknown).is_open(t)
    }

    /// Converts this profile into the [`MarketHours`] value the calendar query
    /// surface ([`candle_end`](super::candle_end),
    /// [`session_bounds`](super::session_bounds), …) consumes, tagged with
    /// `exchange`.
    ///
    /// The rule slices are borrowed (`Cow::Borrowed`), so this allocates
    /// nothing. The tag is passed by the caller because one shared profile can
    /// back several venues — [`hours_for_market_hours_key`] tags with
    /// [`Exchange::Unknown`] since the key, not a venue, identifies the profile.
    #[must_use]
    pub fn to_market_hours(&self, exchange: Exchange) -> MarketHours {
        MarketHours {
            exchange,
            tz: self.tz,
            regular: Cow::Borrowed(self.regular),
            extended: Cow::Borrowed(self.extended),
            has_daily_close: self.has_daily_close,
            has_weekend_close: self.has_weekend_close,
        }
    }
}

/// Names a SHARUR-owned normal-week market-hours profile.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketHoursKey {
    /// CME equity-index Globex hours.
    GlobexEquityIndex,
    /// Shared 23-hour Globex hours for NYMEX/COMEX energy and metals, CBOT rates, and CME FX.
    GlobexEnergy,
    /// CBOT grains/oilseeds Globex hours.
    GlobexGrains,
    /// CME FX Globex hours.
    GlobexFx,
    /// CFE VIX futures hours.
    CfeVix,
    /// EUREX index and interest-rate futures hours.
    Eurex,
    /// ICE Futures U.S. common profile.
    IceUs,
    /// SGX derivatives generic profile.
    Sgx,
    /// Continuous 24x7 UTC profile.
    AlwaysOpen,
}

static ALWAYS_OPEN_RULE: &[SessionRule] = &[SessionRule {
    days: ALL_DAYS,
    open_ssm: 0,
    close_ssm: 24 * 3600,
}];

static FUTURES_GLOBEX_EQUITY_INDEX: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: CME_REGULAR,
    extended: CME_EXT_POST2016,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_GLOBEX_ENERGY: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: MAINT_17_16_EXT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_GLOBEX_GRAINS: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: CBOT_REGULAR_POST2013,
    extended: CBOT_EXT_POST2013,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_CFE_VIX: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: CFE_REGULAR,
    extended: CFE_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_EUREX: FuturesSessionProfile = FuturesSessionProfile {
    tz: Europe::Berlin,
    regular: EUREX_REGULAR,
    extended: EUREX_ASIAN,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_ICE_US: FuturesSessionProfile = FuturesSessionProfile {
    tz: America::New_York,
    regular: &[],
    extended: ICE_WRAP_20_18_EXT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_SGX: FuturesSessionProfile = FuturesSessionProfile {
    tz: Asia::Singapore,
    regular: SGX_REGULAR,
    extended: SGX_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_ALWAYS_OPEN: FuturesSessionProfile = FuturesSessionProfile {
    tz: UTC,
    regular: ALWAYS_OPEN_RULE,
    extended: &[],
    has_daily_close: false,
    has_weekend_close: false,
};

/// Returns the normal-week futures session profile for a well-known key.
#[must_use]
pub fn session_profile(key: MarketHoursKey) -> &'static FuturesSessionProfile {
    match key {
        MarketHoursKey::GlobexEquityIndex => &FUTURES_GLOBEX_EQUITY_INDEX,
        MarketHoursKey::GlobexEnergy | MarketHoursKey::GlobexFx => &FUTURES_GLOBEX_ENERGY,
        MarketHoursKey::GlobexGrains => &FUTURES_GLOBEX_GRAINS,
        MarketHoursKey::CfeVix => &FUTURES_CFE_VIX,
        MarketHoursKey::Eurex => &FUTURES_EUREX,
        MarketHoursKey::IceUs => &FUTURES_ICE_US,
        MarketHoursKey::Sgx => &FUTURES_SGX,
        MarketHoursKey::AlwaysOpen => &FUTURES_ALWAYS_OPEN,
    }
}

/// Resolves a [`MarketHoursKey`] to the [`MarketHours`] value the calendar query
/// surface ([`candle_end`](super::candle_end), [`session_bounds`](super::session_bounds), …)
/// consumes.
///
/// This borrows the same static [`FuturesSessionProfile`] table
/// [`session_profile`] returns — not a second source of truth. The `exchange`
/// tag is [`Exchange::Unknown`] because the key, not a venue enum, identifies
/// these shared futures profiles.
#[must_use]
pub fn hours_for_market_hours_key(key: MarketHoursKey) -> MarketHours {
    session_profile(key).to_market_hours(Exchange::Unknown)
}
