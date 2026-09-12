// SPDX-License-Identifier: MIT-0

//! Named futures profiles, addressed by [`MarketHoursKey`] instead of by venue.
//!
//! Futures hours track the *product family*, not just the exchange: CME
//! equity-index and CME FX contracts list on the same venue but have separately
//! sourced profiles. Keying on the family is therefore the correct granularity,
//! and the caller's sourced instrument catalog must select the exact family.
//! This crate does not map symbols, roots, product codes, or MICs to keys.
//!
//! [`MarketHoursKey`] is `#[non_exhaustive]`. [`session_profile`] exposes the
//! fixed-current static table, [`hours_for_market_hours_key`] selects sourced
//! revisions at the caller's instant, and
//! [`ExchangeCalendar`](super::ExchangeCalendar) scans date-aware profiles.

mod key_serde;
mod profiles;

pub use key_serde::ParseMarketHoursKeyError;
use key_serde::market_hours_keys;
pub use profiles::session_profile;

use std::borrow::Cow;

use chrono::{DateTime, Utc};
use chrono_tz::Tz;

use super::exchange_calendar::CalendarSource;
use super::schedules::ALWAYS_OPEN_PROFILE;
use super::schedules::from_profile;
use super::schedules::futures::international::{
    eurex_fixed_income_profile_at, eurex_profile_at, sgx_equity_index_china_profile_at,
    sgx_equity_index_japan_profile_at, sgx_equity_index_ntr_usd_profile_at,
    sgx_equity_index_singapore_profile_at, sgx_equity_index_taiwan_profile_at, sgx_profile_at,
};
use super::schedules::futures::us::{
    bitcoin_event_contracts_profile_at, cbot_profile_at, cfe_profile_at, cme_profile_at,
    cocoa_profile_at, coffee_profile_at, copper_tas_profile_at, cotton_profile_at,
    cryptocurrency_profile_at, energy_metals_profile_at, event_contracts_profile_at,
    fcoj_profile_at, fx_profile_at, gold_tas_profile_at, ice_us_fang_profile_at,
    ice_usdx_profile_at, interest_rates_profile_at, livestock_profile_at, mini_grains_profile_at,
    nkd_profile_at, palladium_tas_profile_at, platinum_tas_profile_at, rough_rice_profile_at,
    silver_tas_profile_at, spot_quoted_profile_at, sugar_profile_at, weather_profile_at,
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
    /// `source`.
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
    /// [`GlobexGrains`](Self::GlobexGrains) — which excludes the mini-sized
    /// grains and Rough Rice, whose grids diverged from it (2012 and
    /// 2018-01-21 respectively) and need
    /// [`GlobexMiniGrains`](Self::GlobexMiniGrains) and
    /// [`GlobexRoughRice`](Self::GlobexRoughRice) — COMEX/NYMEX →
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
        /// Standard-size CBOT grain/oilseed Globex hours; excludes the
        /// mini-sized grain futures, which have their own
        /// [`GlobexMiniGrains`](Self::GlobexMiniGrains) key, and Rough Rice,
        /// whose extended session diverged on 2018-01-21 and which has its
        /// own [`GlobexRoughRice`](Self::GlobexRoughRice) key.
        GlobexGrains => "globex_grains",
        /// CBOT mini-sized grain futures — Mini-Sized Corn (`XC`), Mini-Sized
        /// Soybean (`XK`), Mini-Sized Wheat (`XW`), and Mini-Sized KC HRW
        /// Wheat (`MKC`, listed 2014-03-23; launch dates remain caller
        /// catalog data) — on the Globex mini grid. Its day session closed 30
        /// minutes after the standard grains' from before the January-2010
        /// floor, it skipped the standard grid's 2015-07-05 close change
        /// entirely, and it converged with that grid only on 2022-10-02, so
        /// the envelopes match today while the histories do not. Excludes the
        /// standard-size corn/soybean/wheat/KC-wheat contracts, Rough Rice,
        /// and the Micro Ag futures, which follow the standard grid.
        GlobexMiniGrains => "globex_mini_grains",
        /// CME FX futures on the standard 17:00-16:00 CT Globex grid; excludes
        /// eFix, TAS, options, and products with a different specification.
        /// Excludes plain BTIC (`6EB`), which CME's FX BTIC FAQ trades "up to
        /// 3:40 p.m. London time (typically 9:40 a.m. CT)" — a different
        /// window. The same FAQ puts BTIC+ (`6EP`) on this clock: "BTIC+
        /// contracts can be traded through the standard CME Globex trading
        /// hours of Sunday-Friday 5:00 p.m.-4:00 p.m. CT", so the exclusion is
        /// of the trade type's plain flavour, not of the ticker family.
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
        /// CBOT Rough Rice futures (`ZR`) and options (`OZR`), Rulebook
        /// chapters 17 and 17A.
        ///
        /// Rough Rice shared the standard CBOT grain and oilseed grid until
        /// CBOT Submission 18-001 cut its extended session to Sunday-Thursday
        /// 19:00-21:00 CT on 2018-01-21; from that day it has no
        /// midnight-wrapping session and is **not** covered by
        /// [`GlobexGrains`](Self::GlobexGrains). Excludes every other CBOT
        /// grain and oilseed contract, mini-sized grains, and separately
        /// specified session types such as TAS and BTIC. No morning Pre-Open
        /// or post-close Pre-Open is modeled after the divergence, because
        /// CME's Rough Rice specification publishes neither.
        GlobexRoughRice => "globex_rough_rice",
        /// CME weather temperature-index **futures** (CME Globex security
        /// tag 55 symbol `HW`): the HDD, CDD and CAT monthly, seasonal-strip and
        /// quarterly-strip contracts for the US, European and Pacific Rim
        /// cities, all quoted in Chicago time. **Excludes options on weather
        /// futures**, which are a separate Globex security group and traded
        /// on the CME floor for most of the audited history; the day they
        /// moved to CME Globex is unsourced, so they stay caller catalog
        /// data. Also excludes CME `ClearPort`, which is a clearing-submission
        /// window rather than a session, and CME's separate "Wind" products.
        ///
        /// Its current envelope coincides with
        /// [`GlobexFx`](Self::GlobexFx) and
        /// [`GlobexEnergy`](Self::GlobexEnergy) and its history matches
        /// neither: weather closed 15:15 CT from the January-2010 floor until
        /// CME SER-9519 expanded it to 16:00 CT on 2025-04-13.
        GlobexWeather => "globex_weather",
        /// CME/CBOT Spot-Quoted Futures ("SQF"), Rulebook Chapter 24: the
        /// eight tradeable roots `QSPX`, `QNDX`, `QDOW` and `QRTY` on the
        /// equity indices and `QBTC`, `QETH`, `QSOL` and `QXRP` on the
        /// cryptocurrency reference rates. `QDOW` is the CBOT listing and the
        /// other seven are CME. Excludes the non-trade clearing legs (`QSF`,
        /// `QNF`, `QDF`, `QRF`, `QTF`, `QEF`, `QOF`, `QXF`) and the
        /// financing-adjustment marker codes (`QSM`, `QNM`, `QDM`, `QRM`,
        /// `QTM`, `QEM`, `QOM`, `QXM`), which are settlement identifiers
        /// rather than order books, and CME `ClearPort`.
        ///
        /// The family is neither
        /// [`GlobexEquityIndex`](Self::GlobexEquityIndex) — which publishes an
        /// 08:30-15:15 CT RTH that no SQF document states — nor
        /// [`GlobexCryptocurrency`](Self::GlobexCryptocurrency), which moved
        /// to 24/7 trading on 2026-05-29 while CME kept spot-quoted on the
        /// five-day grid by name. Its envelope also coincides exactly with
        /// [`GlobexWeather`](Self::GlobexWeather) today while the two
        /// histories share nothing: weather closed 15:15 CT until 2025-04-13
        /// and this family did not exist until 2025-06-29.
        ///
        /// One key covers all eight roots because their normal week and their
        /// dated history are identical. Their *holiday* grids are not: CME
        /// publishes different half-day closes for the equity roots and the
        /// cryptocurrency roots. Holiday and early-close data is caller-owned,
        /// so it lies outside this key — but a caller attaching date
        /// exceptions to it must build them per subgroup rather than applying
        /// one early-close set to all eight.
        GlobexSpotQuoted => "globex_spot_quoted",
        /// CME Group Event Contracts on futures — daily-expiring,
        /// cash-settled, European-style options on futures under CME, CBOT,
        /// NYMEX and COMEX Rulebook Chapter 23, together with the hourly
        /// contracts under Chapter 23A. The Chapter 23 roots are `ECES`,
        /// `ECNQ`, `ECRTY`, `ECYM`, `EC6E`, `ECCL`, `ECNG`, `ECGC`, `ECSI`
        /// and `ECHG`, plus `ECBTC` from its 2023-03-12 listing until
        /// 2026-05-28; the Chapter 23A roots are the hourly `ECS*`, `ECN*`,
        /// `ECR*`, `ECD*`, `ECC*`, `ECH*` and `ECG*` contracts listed from
        /// 2025-12-08.
        ///
        /// **A contract's Termination of Trading time is not this family's
        /// session close.** CME publishes one hours cell per document with a
        /// different termination time per root — 15:00 CT for the equity
        /// indices down to 12:00 CT for `ECHG` — and those per-root numbers
        /// are stated expiration times, not daily closes. The session is one
        /// grid for every root: 17:00→16:00 CT wrapping midnight with a
        /// 60-minute maintenance period from 16:00, Pre-Opens Sunday
        /// 16:00-17:00 and Monday-Thursday 16:45-17:00 CT, and no
        /// Friday-evening reopen.
        ///
        /// **Excludes `ECBTC` from 2026-05-29**, when CME moved that root
        /// alone to 24/7 trading and left the rest on this schedule; select
        /// [`GlobexEventContractsBtc`](Self::GlobexEventContractsBtc) for that
        /// root in every era instead. Also
        /// excludes the Chapter 22 swap-based economic and cryptocurrency
        /// event contracts, which run a 24/7 grid with a one-minute daily
        /// halt, and the sports and political event contracts on the CME
        /// `FutureSports` Performance Indexes.
        ///
        /// The envelope coincides exactly with
        /// [`GlobexSpotQuoted`](Self::GlobexSpotQuoted) and
        /// [`GlobexWeather`](Self::GlobexWeather) today and the three
        /// histories share nothing: weather closed 15:15 CT until 2025-04-13,
        /// spot-quoted did not exist until 2025-06-29, and this family has run
        /// its one grid since 2022-09-18.
        GlobexEventContracts => "globex_event_contracts",
        /// CME Event Contracts on Bitcoin Futures (`ECBTC`), Rulebook Chapter
        /// 23 — the one event-contract root CME moved to 24/7 trading on
        /// 2026-05-29. The key carries the root's whole life: sessionless
        /// before its 2023-03-12 listing, the
        /// [`GlobexEventContracts`](Self::GlobexEventContracts) grid to
        /// 2026-05-28, and a 24/7 grid from 2026-05-29.
        ///
        /// **The 24/7 weekday close is the sourced intersection of two CME
        /// primaries that disagree by an hour.** SER-9740R states a 16:00-16:02
        /// CT maintenance window; CME's client-systems wiki, written five weeks
        /// earlier for this root alone and never revised, states 15:00-16:01
        /// CT. Both give the 16:01-16:02 Pre-Open, the 16:02 open, and the
        /// Saturday 02:00-04:00 window with its 03:45 Pre-Open. The key serves
        /// what both support — open 16:02→15:00 CT — and withholds the disputed
        /// hour, erring toward closed. CME's Globex notices since the cutover
        /// confirm the Saturday window for this root's channel and also state
        /// three one-day Saturday extensions, which are modelled.
        ///
        /// The fixed profile uses adjacent rules for exact open/closed state;
        /// [`calendar_for_market_hours_key`](super::calendar_for_market_hours_key)
        /// joins the storage pieces into the continuous weekend block and
        /// carries the following open business date, as for
        /// [`GlobexCryptocurrency`](Self::GlobexCryptocurrency). Unlike that
        /// key, this root's Pre-Opens stay `order_entry`.
        GlobexEventContractsBtc => "globex_event_contracts_btc",
        /// COMEX Gold Trading at Settlement (`GCT`, CME Globex security group
        /// TG), listed 2010-04-11 for trade date 2010-04-12.
        ///
        /// Sunday-Thursday 17:00 CT wrapping to 12:30 CT, with **no
        /// Friday-evening reopen** and no regular session: the TAS book is
        /// Globex-only, and CME's 2009-2015 pit route ran in the underlying's
        /// pit on the underlying's hours. `regular` is therefore empty in
        /// every era. The Pre-Open is `order_entry`, because Rule 524 permits
        /// a TAS order only after its group's pre-open state begins.
        ///
        /// Members `MGT`, `QOT` and `1OT` joined later groups-side and are
        /// caller catalog data. The 12:30 CT close has never moved; the two
        /// dated revisions are queue-onset changes in 2011 and 2012. The
        /// Sunday queue serves the sourced 16:15-17:00 CT intersection and
        /// withholds 16:00-16:15, as
        /// [`GlobexEquityIndex`](Self::GlobexEquityIndex) does.
        GlobexGoldTas => "globex_gold_tas",
        /// COMEX Silver Trading at Settlement (`SIT`, security group MT),
        /// listed 2010-04-11 on the same COMEX self-certification as
        /// [`GlobexGoldTas`](Self::GlobexGoldTas).
        ///
        /// Identical shape, closing 12:25 CT instead of 12:30, with the same
        /// empty `regular`, the same absent Friday-evening reopen, the same
        /// two queue revisions and the same withheld Sunday quarter-hour. It
        /// is a separate key because the close differs; `MST` joined in 2025
        /// and is caller catalog data.
        GlobexSilverTas => "globex_silver_tas",
        /// COMEX Copper Trading at Settlement (`HGT`, security group HT),
        /// listed 2011-01-23 for trade date 2011-01-24, nine months after gold
        /// and silver and on CME Globex only.
        ///
        /// Sunday-Thursday 17:00 CT wrapping to 12:00 CT, empty `regular`, no
        /// Friday-evening reopen, and the same two queue revisions. Copper TAS
        /// was **never** a pit product, which both COMEX SER-5542 and MRAN
        /// RA1107-4 state in terms.
        ///
        /// Do not substitute
        /// [`GlobexPalladiumTas`](Self::GlobexPalladiumTas) on the coincident
        /// 12:00 CT close: different exchange, different security group and
        /// launches seven years apart. Members `MHT` and `HG0` are caller
        /// catalog data.
        GlobexCopperTas => "globex_copper_tas",
        /// NYMEX Trading at Settlement on Platinum Futures (`PLT`, security
        /// group PE), listed 2017-05-21 for trade date 2017-05-22.
        ///
        /// One era: Sunday-Thursday 17:00 CT wrapping to 12:05 CT, empty
        /// `regular`, no Friday-evening reopen, and no revision of any kind
        /// since launch. Because the root launched after CME's undated 2012
        /// move of the Sunday TAS pause, its earliest sourced Sunday queue
        /// onset is 16:00 CT and nothing is withheld: Sunday 16:00-17:00 and
        /// Monday-Thursday 16:45-17:00 CT are both served.
        GlobexPlatinumTas => "globex_platinum_tas",
        /// NYMEX Palladium Trading at Settlement (`PAT`, security group PX),
        /// listed 2018-11-18 for trade date 2018-11-19.
        ///
        /// One era: Sunday-Thursday 17:00 CT wrapping to 12:00 CT, with the
        /// same empty `regular` and the same unwithheld queue as
        /// [`GlobexPlatinumTas`](Self::GlobexPlatinumTas).
        ///
        /// **There is no 16:00-17:00 CT daily break.** The clause CME's 2019
        /// specification printed on the TAS line is inherited boilerplate from
        /// the outright row — a book closed at 12:00 CT cannot break at 16:00
        /// — and CME deleted it in 2020 while leaving the instants unchanged.
        GlobexPalladiumTas => "globex_palladium_tas",

        /// SGX Three-Month SORA Futures current profile.
        Sgx => "sgx",
        /// Continuous 24x7 UTC profile.
        ///
        /// It has no final daily close, so date-aware trade-date queries return
        /// `None` at every instant.
        AlwaysOpen => "always_open",
    }
}

/// Resolves the fixed [`MarketHours`] snapshot in effect for `key` at `as_of`.
///
/// This is the key-side's only profile-selection path: every request carries
/// the caller's instant (LAW-DETERMINISM), so a backtest and a live query run
/// identical code. Sourced histories are selected independently for each
/// product family, including CME Group equity-index, energy/metals, grains,
/// FX, interest-rate, livestock, and cryptocurrency grids. Keys with no
/// in-scope recorded change resolve to their one grid at every instant. Dates
/// before the January-2010 audit floor receive the oldest audited profile. For
/// launch-dated families — CME cryptocurrency, CME/CBOT spot-quoted, CME Group
/// event contracts, ICE U.S.
/// NYSE FANG+, and SGX Three-Month SORA — a pre-launch date returns an
/// explicit sessionless profile. A member listed after its family began does
/// not create a key-level
/// revision; callers enforce product launch dates in their catalog. Some CME
/// histories have a verified-current Pre-Open or PCP queue with no primary day
/// for its onset; each of those timelines carries the queue only from its own
/// knowledge-bound review row onward — 2026-08-22 for the equity-index,
/// energy, FX and interest-rate families and 2026-09-05 for weather — rather
/// than fabricating a cutover.
///
/// This returns one snapshot. Use
/// [`calendar_for_market_hours_key`](super::calendar_for_market_hours_key) for
/// date-aware scans that cross a family revision.
#[must_use]
pub fn hours_for_market_hours_key(key: MarketHoursKey, as_of: DateTime<Utc>) -> MarketHours {
    let profile = match key {
        MarketHoursKey::GlobexEquityIndex => cme_profile_at(as_of),
        MarketHoursKey::GlobexEnergy => energy_metals_profile_at(as_of),
        MarketHoursKey::GlobexGrains => cbot_profile_at(as_of),
        MarketHoursKey::GlobexMiniGrains => mini_grains_profile_at(as_of),
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
        MarketHoursKey::GlobexRoughRice => rough_rice_profile_at(as_of),
        MarketHoursKey::GlobexWeather => weather_profile_at(as_of),
        MarketHoursKey::GlobexSpotQuoted => spot_quoted_profile_at(as_of),
        MarketHoursKey::GlobexEventContracts => event_contracts_profile_at(as_of),
        MarketHoursKey::GlobexEventContractsBtc => bitcoin_event_contracts_profile_at(as_of),
        MarketHoursKey::GlobexGoldTas => gold_tas_profile_at(as_of),
        MarketHoursKey::GlobexSilverTas => silver_tas_profile_at(as_of),
        MarketHoursKey::GlobexCopperTas => copper_tas_profile_at(as_of),
        MarketHoursKey::GlobexPlatinumTas => platinum_tas_profile_at(as_of),
        MarketHoursKey::GlobexPalladiumTas => palladium_tas_profile_at(as_of),
        MarketHoursKey::Sgx => sgx_profile_at(as_of),
        MarketHoursKey::AlwaysOpen => &ALWAYS_OPEN_PROFILE,
    };
    from_profile(CalendarSource::MarketHoursKey(key), profile)
}
