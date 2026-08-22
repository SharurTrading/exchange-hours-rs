// SPDX-License-Identifier: MIT-0

//! Named futures profiles, addressed by [`MarketHoursKey`] instead of by venue.
//!
//! Futures hours track the *product family*, not just the exchange: CME
//! equity-index and CME FX contracts list on the same venue but have separately
//! sourced profiles. Keying on the family is therefore the correct granularity,
//! and it is the surface `instrument-catalog` maps instrument roots onto.
//!
//! [`MarketHoursKey`] is `#[non_exhaustive]`: adding a family must not break a
//! downstream match. The tables here are the same *current* statics the
//! [`Exchange`]-keyed presets use — one source of truth, two addressing schemes.
//! [`session_profile`] and [`hours_for_market_hours_key`] remain fixed-current
//! compatibility snapshots. [`hours_for_market_hours_key_as_of`] selects every
//! primary-sourced key revision available to the corresponding venue family.
//! It returns one snapshot, not a scanning calendar: callers crossing a
//! transition must resolve again for each date. [`ExchangeCalendar`](super::ExchangeCalendar)
//! remains the date-aware session-scan API for venue identities.

mod key_serde;

use std::borrow::Cow;

use chrono::{DateTime, Utc};
use chrono_tz::UTC;
use chrono_tz::{America, Asia, Europe, Tz, US};

use super::local_time::bounded_utc;
use super::rule::{ALL_DAYS, SUN_PLUS_MON_THU};
use super::schedules::from_profile;
use super::schedules::futures::international::{
    EUREX_CURRENT_EXTENDED, EUREX_CURRENT_REGULAR, SGX_CURRENT_EXTENDED, SGX_CURRENT_REGULAR,
    eurex_profile_at, sgx_profile_at,
};
use super::schedules::futures::us::{
    CBOT_EXTENDED_CURRENT, CBOT_REGULAR_CURRENT, CFE_EXTENDED, CFE_REGULAR, CME_EXTENDED_CURRENT,
    CME_REGULAR, ENERGY_METALS_EXTENDED_CURRENT, ICE_US_FANG_EXTENDED_CURRENT, cbot_profile_at,
    cfe_profile_at, cme_profile_at, energy_metals_profile_at, ice_us_fang_profile_at,
};
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
    pub fn to_market_hours(self, exchange: Exchange) -> MarketHours {
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

/// Names a SHARUR-owned normal-week product-family market-hours profile.
///
/// Serde encodes each key as its stable canonical `snake_case` string in every
/// format, including compact non-self-describing formats.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarketHoursKey {
    /// CME equity-index Globex hours.
    GlobexEquityIndex,
    /// COMEX Gold and NYMEX benchmark-energy Globex hours.
    GlobexEnergy,
    /// CBOT grains/oilseeds Globex hours.
    GlobexGrains,
    /// CME FX Globex hours.
    GlobexFx,
    /// CFE VIX futures hours.
    CfeVix,
    /// Eurex FESX/FDAX/FDXM benchmark-index futures current-season snapshot.
    Eurex,
    /// ICE Futures U.S. NYSE FANG+ Index futures hours.
    IceUs,
    /// SGX Three-Month SORA Futures current profile.
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
    extended: CME_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// This profile borrows the current representative COMEX Gold / NYMEX benchmark
// energy rule owned with that product family's dated history.
static FUTURES_GLOBEX_ENERGY: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: ENERGY_METALS_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

// CME's 2010 FX guide publishes this grid for all major CME FX futures, and
// primary operator snapshots from 2018, 2020, and the current product material
// retain it. The last normal-week change found in CME's electronic-trading
// notices was effective in February 2009, before the crate's January-2010 audit
// floor. This fixed key therefore has no in-scope dated revision.
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20081229.html
// https://www.cmegroup.com/trading/fx/files/FX248-2010_FX_Product_Guide_and_Calendar.pdf
// https://www.cmegroup.com/trading/fx/fx-report/files/q1-2018-cme-fx-products.pdf
// https://www.cmegroup.com/trading/fx/files/emfx-brochure-q3-2020.pdf
// https://www.cmegroup.com/articles/faqs/frequently-asked-questions-cme-fx-futures-calendar-spreads.html
static CME_FX_EXTENDED_CURRENT: &[SessionRule] = &[SessionRule {
    days: SUN_PLUS_MON_THU,
    open_ssm: 17 * 3600,
    close_ssm: 16 * 3600,
}];
static FUTURES_GLOBEX_FX: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: &[],
    extended: CME_FX_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_GLOBEX_GRAINS: FuturesSessionProfile = FuturesSessionProfile {
    tz: US::Central,
    regular: CBOT_REGULAR_CURRENT,
    extended: CBOT_EXTENDED_CURRENT,
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
    regular: EUREX_CURRENT_REGULAR,
    extended: EUREX_CURRENT_EXTENDED,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_ICE_US: FuturesSessionProfile = FuturesSessionProfile {
    tz: America::New_York,
    regular: &[],
    extended: ICE_US_FANG_EXTENDED_CURRENT,
    has_daily_close: true,
    has_weekend_close: true,
};

static FUTURES_SGX: FuturesSessionProfile = FuturesSessionProfile {
    tz: Asia::Singapore,
    regular: SGX_CURRENT_REGULAR,
    extended: SGX_CURRENT_EXTENDED,
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

/// Returns the fixed-current normal-week futures session profile for a well-known key.
///
/// This function does not select historical revisions. Use
/// [`hours_for_market_hours_key_as_of`] for a dated key snapshot. Callers that
/// scan across a transition must re-resolve for each date; the venue-keyed
/// [`ExchangeCalendar`](super::ExchangeCalendar) performs that reselection
/// while scanning.
#[must_use]
pub fn session_profile(key: MarketHoursKey) -> &'static FuturesSessionProfile {
    match key {
        MarketHoursKey::GlobexEquityIndex => &FUTURES_GLOBEX_EQUITY_INDEX,
        MarketHoursKey::GlobexEnergy => &FUTURES_GLOBEX_ENERGY,
        MarketHoursKey::GlobexGrains => &FUTURES_GLOBEX_GRAINS,
        MarketHoursKey::GlobexFx => &FUTURES_GLOBEX_FX,
        MarketHoursKey::CfeVix => &FUTURES_CFE_VIX,
        MarketHoursKey::Eurex => &FUTURES_EUREX,
        MarketHoursKey::IceUs => &FUTURES_ICE_US,
        MarketHoursKey::Sgx => &FUTURES_SGX,
        MarketHoursKey::AlwaysOpen => &FUTURES_ALWAYS_OPEN,
    }
}

/// Resolves a [`MarketHoursKey`] to the fixed-current [`MarketHours`] value the
/// calendar query surface ([`candle_end`](super::candle_end),
/// [`session_bounds`](super::session_bounds), …) consumes.
///
/// This borrows the same static [`FuturesSessionProfile`] table
/// [`session_profile`] returns — not a second source of truth. The `exchange`
/// tag is [`Exchange::Unknown`] because the key, not a venue enum, identifies
/// these shared futures profiles. This function does not select historical
/// revisions.
#[must_use]
pub fn hours_for_market_hours_key(key: MarketHoursKey) -> MarketHours {
    session_profile(key).to_market_hours(Exchange::Unknown)
}

/// Resolves the fixed [`MarketHours`] snapshot in effect for `key` at `as_of`.
///
/// Sourced histories are reused from the corresponding product-family venue:
/// CME equity-index, COMEX Gold/NYMEX benchmark energy, CBOT grains/oilseeds,
/// CFE VIX, Eurex benchmark-index, ICE U.S. NYSE FANG+, and SGX Three-Month
/// SORA futures. Keys with no in-scope recorded change return their current
/// snapshot. Dates before the January-2010 audit floor receive the oldest
/// audited profile. For launch-dated families — currently ICE U.S. NYSE FANG+
/// and SGX Three-Month SORA — a pre-launch date returns an explicit sessionless
/// profile that reports closed at every instant. That result means the market
/// was closed, not that schedule data is missing.
///
/// This returns one snapshot. A caller spanning a schedule transition must
/// invoke it again for each date; there is intentionally no key-keyed calendar
/// type in this release. Use [`ExchangeCalendar`](super::ExchangeCalendar) for
/// date-aware scans when a venue identity is available.
#[must_use]
pub fn hours_for_market_hours_key_as_of(key: MarketHoursKey, as_of: DateTime<Utc>) -> MarketHours {
    let current = hours_for_market_hours_key(key);
    let as_of = bounded_utc(as_of, current.tz);
    let profile = match key {
        MarketHoursKey::GlobexEquityIndex => cme_profile_at(as_of),
        MarketHoursKey::GlobexEnergy => energy_metals_profile_at(as_of),
        MarketHoursKey::GlobexGrains => cbot_profile_at(as_of),
        MarketHoursKey::CfeVix => cfe_profile_at(as_of),
        MarketHoursKey::Eurex => eurex_profile_at(as_of),
        MarketHoursKey::IceUs => ice_us_fang_profile_at(as_of),
        MarketHoursKey::Sgx => sgx_profile_at(as_of),
        MarketHoursKey::GlobexFx | MarketHoursKey::AlwaysOpen => return current,
    };
    from_profile(Exchange::Unknown, profile)
}
