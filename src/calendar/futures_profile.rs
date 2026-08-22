// SPDX-License-Identifier: MIT-0

//! Named futures profiles, addressed by [`MarketHoursKey`] instead of by venue.
//!
//! Futures hours track the *product family*, not just the exchange: CME
//! equity-index and CME FX contracts list on the same venue but have separately
//! sourced profiles. Keying on the family is therefore the correct granularity,
//! and the caller's sourced instrument catalog must select the exact family.
//! This crate does not map symbols, roots, product codes, or MICs to keys.
//!
//! [`MarketHoursKey`] is `#[non_exhaustive]`. [`session_profile`] is a current
//! snapshot, [`hours_for_market_hours_key_as_of`] selects sourced revisions,
//! and [`ExchangeCalendar`](super::ExchangeCalendar) scans date-aware profiles.

mod key_serde;
mod profiles;

pub use key_serde::ParseMarketHoursKeyError;
use key_serde::market_hours_keys;
pub use profiles::session_profile;

use std::borrow::Cow;

use chrono::{DateTime, Utc};
use chrono_tz::Tz;

use super::local_time::bounded_utc;
use super::schedules::from_profile;
use super::schedules::futures::international::{eurex_profile_at, sgx_profile_at};
use super::schedules::futures::us::{
    cbot_profile_at, cfe_profile_at, cme_profile_at, cryptocurrency_profile_at,
    energy_metals_profile_at, ice_us_fang_profile_at, interest_rates_profile_at,
    livestock_profile_at,
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

market_hours_keys! {
    /// Names a normal-week product-family market-hours profile.
    ///
    /// The enum is `#[non_exhaustive]`; match it with a wildcard and enumerate
    /// the keys in the compiled crate version with [`MarketHoursKey::ALL`]. Each
    /// variant has one stable canonical `snake_case` name, shared by serde,
    /// [`MarketHoursKey::as_str`], [`core::fmt::Display`], and
    /// [`core::str::FromStr`]. Serde uses that string in every format, including
    /// compact non-self-describing formats. Renaming a key is therefore a
    /// breaking persisted-wire change.
    ///
    /// The venue-keyed compatibility defaults are CME →
    /// [`GlobexEquityIndex`](Self::GlobexEquityIndex), CBOT →
    /// [`GlobexGrains`](Self::GlobexGrains), COMEX/NYMEX →
    /// [`GlobexEnergy`](Self::GlobexEnergy), CFE → [`CfeVix`](Self::CfeVix),
    /// Eurex → [`Eurex`](Self::Eurex), ICEUS → [`IceUs`](Self::IceUs), and SGX
    /// → [`Sgx`](Self::Sgx). Those defaults are wrong for products outside the
    /// named families; select a product-family key instead of treating a venue
    /// as one universal clock.
    #[non_exhaustive]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum MarketHoursKey {
        /// U.S. equity-index Globex grid across CME and CBOT, including YM/MYM;
        /// excludes Nikkei 225 Dollar (NKD), whose historical grid differs.
        GlobexEquityIndex => "globex_equity_index",
        /// NYMEX `CL/MCL/QM`, `NG/MNG/QG`, `HO/RB/BZ`, and `PL/PA`, plus
        /// COMEX `GC/MGC`, `SI/SIL`, and `HG/MHG`, including their shared
        /// energy/metals history.
        GlobexEnergy => "globex_energy",
        /// CBOT grains/oilseeds Globex hours.
        GlobexGrains => "globex_grains",
        /// CME FX Globex hours.
        GlobexFx => "globex_fx",
        /// CBOT U.S. Treasury (`ZT/ZF/ZN/TN/ZB/UB` and micros), 30-Day Fed
        /// Funds (`ZQ`), and CME SOFR (`SR1/SR3`) Globex hours.
        GlobexInterestRates => "globex_interest_rates",
        /// CME Live Cattle, Feeder Cattle, and Lean Hog Globex hours.
        GlobexLivestock => "globex_livestock",
        /// CME non-spot-quoted cryptocurrency futures Globex hours.
        ///
        /// The fixed profile uses adjacent rules for exact open/closed state.
        /// [`calendar_for_market_hours_key`](super::calendar_for_market_hours_key)
        /// joins the storage pieces into exact weekend blocks and applies CME's
        /// following-open-business-day convention. That date is normally
        /// Monday; a caller policy that closes Monday rolls the trading to
        /// Tuesday. Friday 16:00 remains the weekly close. Detached fixed
        /// snapshots retain table-piece bounds and do not expose that weekly
        /// boundary or trade dates because they carry no key identity.
        GlobexCryptocurrency => "globex_cryptocurrency",
        /// CFE VIX futures hours.
        CfeVix => "cfe_vix",
        /// Eurex FESX/FDAX/FDXM benchmark-index futures current-season snapshot.
        Eurex => "eurex",
        /// ICE Futures U.S. NYSE FANG+ Index futures hours.
        IceUs => "ice_us",
        /// SGX Three-Month SORA Futures current profile.
        Sgx => "sgx",
        /// Continuous 24x7 UTC profile.
        ///
        /// It has no final daily close, so date-aware trade-date queries return
        /// `None` at every instant.
        AlwaysOpen => "always_open",
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
/// revisions. For [`MarketHoursKey::GlobexCryptocurrency`], open/closed state
/// remains exact, but identity-dependent multi-day bounds, trade dates, and
/// weekly candle boundaries are available only from
/// [`calendar_for_market_hours_key`](super::calendar_for_market_hours_key).
#[must_use]
pub fn hours_for_market_hours_key(key: MarketHoursKey) -> MarketHours {
    session_profile(key).to_market_hours(Exchange::Unknown)
}

/// Resolves the fixed [`MarketHours`] snapshot in effect for `key` at `as_of`.
///
/// Sourced histories are selected independently for each product family,
/// including CME Group equity-index, energy/metals, grains, interest-rate,
/// livestock, and cryptocurrency grids. Keys with no in-scope recorded change
/// return their current snapshot. Dates before the January-2010 audit floor
/// receive the oldest audited profile. For launch-dated families — CME
/// cryptocurrency, ICE U.S. NYSE FANG+, and SGX Three-Month SORA — a pre-launch
/// date returns an explicit sessionless profile. A member listed after its
/// family began does not create a key-level revision; callers enforce product
/// launch dates in their catalog.
///
/// This returns one snapshot. Use
/// [`calendar_for_market_hours_key`](super::calendar_for_market_hours_key) for
/// date-aware scans that cross a family revision.
#[must_use]
pub fn hours_for_market_hours_key_as_of(key: MarketHoursKey, as_of: DateTime<Utc>) -> MarketHours {
    let current = hours_for_market_hours_key(key);
    let as_of = bounded_utc(as_of, current.tz);
    let profile = match key {
        MarketHoursKey::GlobexEquityIndex => cme_profile_at(as_of),
        MarketHoursKey::GlobexEnergy => energy_metals_profile_at(as_of),
        MarketHoursKey::GlobexGrains => cbot_profile_at(as_of),
        MarketHoursKey::GlobexInterestRates => interest_rates_profile_at(as_of),
        MarketHoursKey::GlobexLivestock => livestock_profile_at(as_of),
        MarketHoursKey::GlobexCryptocurrency => cryptocurrency_profile_at(as_of),
        MarketHoursKey::CfeVix => cfe_profile_at(as_of),
        MarketHoursKey::Eurex => eurex_profile_at(as_of),
        MarketHoursKey::IceUs => ice_us_fang_profile_at(as_of),
        MarketHoursKey::Sgx => sgx_profile_at(as_of),
        MarketHoursKey::GlobexFx | MarketHoursKey::AlwaysOpen => return current,
    };
    from_profile(Exchange::Unknown, profile)
}
