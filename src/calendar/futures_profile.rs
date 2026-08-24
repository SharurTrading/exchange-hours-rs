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

use super::exchange_calendar::CalendarSource;
use super::local_time::bounded_utc;
use super::schedules::from_profile;
use super::schedules::futures::international::{
    eurex_fixed_income_profile_at, eurex_profile_at, sgx_equity_index_china_profile_at,
    sgx_equity_index_japan_profile_at, sgx_equity_index_ntr_usd_profile_at,
    sgx_equity_index_singapore_profile_at, sgx_equity_index_taiwan_profile_at, sgx_profile_at,
};
use super::schedules::futures::us::{
    cbot_profile_at, cfe_profile_at, cme_profile_at, cocoa_profile_at, coffee_profile_at,
    cotton_profile_at, cryptocurrency_profile_at, energy_metals_profile_at, fcoj_profile_at,
    fx_profile_at, ice_us_fang_profile_at, ice_usdx_profile_at, interest_rates_profile_at,
    livestock_profile_at, nkd_profile_at, sugar_profile_at,
};
use super::{Exchange, MarketHours, SessionRule};

/// A timezone-aware set of normal-week futures session rules.
///
/// `regular` carries primary trading sessions; `extended` carries electronic,
/// overnight, and other non-regular sessions. Holiday and special-session
/// overlays are deliberately modeled outside this normal-week profile.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FuturesSessionProfile {
    /// Exchange local timezone used to interpret `SessionRule` SSM values.
    pub tz: Tz,
    /// Primary trading sessions.
    pub regular: &'static [SessionRule],
    /// Electronic, overnight, and other tradeable non-regular sessions.
    pub extended: &'static [SessionRule],
    /// Order-entry-only phases in which no trade can match.
    pub order_entry: &'static [SessionRule],
    /// True when the venue has a distinct daily close.
    pub has_daily_close: bool,
    /// True when the venue has a true weekend close.
    pub has_weekend_close: bool,
}

impl FuturesSessionProfile {
    /// Returns `true` when any regular or extended normal-week session is active.
    #[must_use]
    pub fn is_open(&self, t: DateTime<Utc>) -> bool {
        self.to_market_hours(CalendarSource::Exchange(Exchange::Unknown))
            .is_open(t)
    }

    /// True when `t` falls in an order-entry-only phase where nothing matches.
    ///
    /// Mirrors [`MarketHours::is_order_entry_only`] so a profile and the hours
    /// it produces answer identically.
    #[must_use]
    pub fn is_order_entry_only(&self, t: DateTime<Utc>) -> bool {
        self.to_market_hours(CalendarSource::Exchange(Exchange::Unknown))
            .is_order_entry_only(t)
    }

    /// True when orders may be entered, amended or cancelled at `t`.
    #[must_use]
    pub fn is_accepting_orders(&self, t: DateTime<Utc>) -> bool {
        self.to_market_hours(CalendarSource::Exchange(Exchange::Unknown))
            .is_accepting_orders(t)
    }

    /// Converts this profile into the [`MarketHours`] value the calendar query
    /// surface ([`candle_end`](super::candle_end),
    /// [`session_bounds`](super::session_bounds), …) consumes, tagged with
    /// `exchange`.
    ///
    /// The rule slices are borrowed (`Cow::Borrowed`), so this allocates
    /// nothing. The tag is passed by the caller because one shared profile can
    /// back several identities — [`hours_for_market_hours_key`] tags with
    /// [`CalendarSource::MarketHoursKey`] since the key, not a venue,
    /// identifies the profile.
    #[must_use]
    pub fn to_market_hours(self, source: CalendarSource) -> MarketHours {
        MarketHours {
            source,
            tz: self.tz,
            regular: Cow::Borrowed(self.regular),
            extended: Cow::Borrowed(self.extended),
            order_entry: Cow::Borrowed(self.order_entry),
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
        /// CME/CBOT equity-index futures on the sourced U.S. grid, including
        /// YM/MYM. Excludes full-size S&P 500 (`SP`), Nikkei 225 Dollar (`NKD`),
        /// and BTIC/TACO products, whose historical or current grids differ.
        GlobexEquityIndex => "globex_equity_index",
        /// NYMEX `CL/MCL/QM`, `NG/MNG/QG`, `HO/RB/BZ`, and `PL/PA`, plus
        /// COMEX `GC/MGC`, `SI/SIL`, and `HG/MHG`, including their shared
        /// energy/metals history. Excludes TAS/TAM/BTIC, options, and products
        /// whose own specification publishes a different grid.
        GlobexEnergy => "globex_energy",
        /// Standard-size CBOT grain/oilseed Globex hours; excludes mini-sized
        /// Corn, Soybean, and Wheat futures, whose 2012 schedule diverged.
        GlobexGrains => "globex_grains",
        /// CME FX futures on the standard 17:00-16:00 CT Globex grid; excludes
        /// eFix, BTIC, TAS, options, and products with a different specification.
        GlobexFx => "globex_fx",
        /// CBOT U.S. Treasury (`ZT/ZF/ZN/TN/ZB/UB` and micros), 30-Day Fed
        /// Funds (`ZQ`), and CME SOFR (`SR1/SR3`) Globex hours. Excludes
        /// options and separately specified interest-rate product families.
        GlobexInterestRates => "globex_interest_rates",
        /// CME Live Cattle, Feeder Cattle, and Lean Hog futures Globex hours;
        /// excludes options and separately specified TAS sessions.
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
        /// ICE Futures U.S. Sugar No. 11 (`SB`) futures and options. One
        /// same-day New York session with separate order-entry phases.
        IceUsSugar => "ice_us_sugar",
        /// ICE Futures U.S. Coffee \"C\" (`KC`) futures and options.
        IceUsCoffee => "ice_us_coffee",
        /// ICE Futures U.S. Cocoa (`CC`) futures and options.
        IceUsCocoa => "ice_us_cocoa",
        /// ICE Futures U.S. Cotton No. 2 (`CT`) futures and options.
        IceUsCotton => "ice_us_cotton",
        /// ICE Futures U.S. FCOJ-A frozen concentrated orange juice (`OJ`) futures.
        IceUsOrangeJuice => "ice_us_orange_juice",
        /// ICE Futures U.S. U.S. Dollar Index (`DX`) futures and options.
        IceUsDollarIndex => "ice_us_dollar_index",
        /// CME Nikkei 225 Dollar (`NKD`) futures on CME Globex.
        GlobexNikkei225Dollar => "globex_nikkei_225_dollar",
        /// Eurex fixed-income futures (`FGBL`/`FGBM`/`FGBS`/`FGBX`).
        EurexFixedIncome => "eurex_fixed_income",
        /// SGX Japan equity-index derivatives (Nikkei 225 suite).
        SgxEquityIndexJapan => "sgx_equity_index_japan",
        /// SGX China equity-index derivatives (FTSE China A50/H50).
        SgxEquityIndexChina => "sgx_equity_index_china",
        /// SGX Singapore equity-index derivatives (`SiMSCI`, Straits Times Index).
        SgxEquityIndexSingapore => "sgx_equity_index_singapore",
        /// SGX Taiwan equity-index derivatives (FTSE Taiwan suite).
        SgxEquityIndexTaiwan => "sgx_equity_index_taiwan",
        /// SGX NTR (USD) global equity-index futures.
        SgxEquityIndexNtrUsd => "sgx_equity_index_ntr_usd",

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
/// [`session_profile`] returns — not a second source of truth. The source is
/// [`CalendarSource::MarketHoursKey`] because the key, not a venue enum,
/// identifies these shared futures profiles. This function does not select
/// historical revisions. For [`MarketHoursKey::GlobexCryptocurrency`], open/closed state
/// remains exact, but identity-dependent multi-day bounds, trade dates, and
/// weekly candle boundaries are available only from
/// [`calendar_for_market_hours_key`](super::calendar_for_market_hours_key).
#[must_use]
pub fn hours_for_market_hours_key(key: MarketHoursKey) -> MarketHours {
    session_profile(key).to_market_hours(CalendarSource::MarketHoursKey(key))
}

/// Resolves the fixed [`MarketHours`] snapshot in effect for `key` at `as_of`.
///
/// Sourced histories are selected independently for each product family,
/// including CME Group equity-index, energy/metals, grains, FX, interest-rate,
/// livestock, and cryptocurrency grids. Keys with no in-scope recorded change
/// return their current snapshot. Dates before the January-2010 audit floor
/// receive the oldest audited profile. For launch-dated families — CME
/// cryptocurrency, ICE U.S. NYSE FANG+, and SGX Three-Month SORA — a pre-launch
/// date returns an explicit sessionless profile. A member listed after its
/// family began does not create a key-level revision; callers enforce product
/// launch dates in their catalog. Some CME histories have an exact current
/// fixed profile but no primary day for an older Pre-Open or PCP onset. Their
/// dated selectors intentionally omit only that unsourced phase rather than
/// fabricate a cutover; use [`session_profile`] for the exact current snapshot.
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
        MarketHoursKey::GlobexFx => fx_profile_at(as_of),
        MarketHoursKey::GlobexInterestRates => interest_rates_profile_at(as_of),
        MarketHoursKey::GlobexLivestock => livestock_profile_at(as_of),
        MarketHoursKey::GlobexCryptocurrency => cryptocurrency_profile_at(as_of),
        MarketHoursKey::CfeVix => cfe_profile_at(as_of),
        MarketHoursKey::Eurex => eurex_profile_at(as_of),
        MarketHoursKey::IceUs => ice_us_fang_profile_at(as_of),
        MarketHoursKey::IceUsSugar => sugar_profile_at(as_of),
        MarketHoursKey::IceUsCoffee => coffee_profile_at(as_of),
        MarketHoursKey::IceUsCocoa => cocoa_profile_at(as_of),
        MarketHoursKey::IceUsCotton => cotton_profile_at(as_of),
        MarketHoursKey::IceUsOrangeJuice => fcoj_profile_at(as_of),
        MarketHoursKey::IceUsDollarIndex => ice_usdx_profile_at(as_of),
        MarketHoursKey::GlobexNikkei225Dollar => nkd_profile_at(as_of),
        MarketHoursKey::EurexFixedIncome => eurex_fixed_income_profile_at(as_of),
        MarketHoursKey::SgxEquityIndexJapan => sgx_equity_index_japan_profile_at(as_of),
        MarketHoursKey::SgxEquityIndexChina => sgx_equity_index_china_profile_at(as_of),
        MarketHoursKey::SgxEquityIndexSingapore => sgx_equity_index_singapore_profile_at(as_of),
        MarketHoursKey::SgxEquityIndexTaiwan => sgx_equity_index_taiwan_profile_at(as_of),
        MarketHoursKey::SgxEquityIndexNtrUsd => sgx_equity_index_ntr_usd_profile_at(as_of),
        MarketHoursKey::Sgx => sgx_profile_at(as_of),
        MarketHoursKey::AlwaysOpen => return current,
    };
    from_profile(CalendarSource::MarketHoursKey(key), profile)
}
