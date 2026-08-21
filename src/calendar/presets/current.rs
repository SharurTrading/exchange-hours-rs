// SPDX-License-Identifier: MIT-0

//! Current published hours for every [`Exchange`] variant.
//!
//! The bulk of this module is one exhaustive `match` over `Exchange`, which is
//! load-bearing: there is no catch-all arm, so adding a venue is a compile
//! error until someone decides its hours. Splitting the match across modules
//! would replace that guarantee with a runtime fallthrough, so the arms stay
//! together. The *data* the arms name already lives in
//! [`super::super::profiles`], grouped by product family.

use crate::calendar::profiles::{
    ABU_DHABI_01_23_PROFILE, BLUE_OCEAN_PROFILE, BME_PROFILE, C1_PROFILE_POST_2024_08_26,
    EEX_PROFILE, ENDEX_01_23_PROFILE, EURONEXT_AMS_PROFILE, EURONEXT_BRU_PROFILE,
    EURONEXT_DUB_PROFILE, EURONEXT_LIS_PROFILE, EURONEXT_MIL_PROFILE, EURONEXT_PARIS_PROFILE,
    FINRA_TRF_PROFILE, ICE_CANADA_PROFILE, ICE_EU_LONDON_01_23_PROFILE, IEX_PROFILE_POST2015,
    IQX_PROFILE, LSE_PROFILE, NASDAQ_CPH_PROFILE, NASDAQ_HEL_PROFILE, NASDAQ_STO_PROFILE,
    NYSE_PROFILE, NYSE_TEXAS_PROFILE, SIX_PROFILE, US_EQUITIES_PROFILE,
    US_EQUITY_EARLY_0700_PROFILE, US_OPTIONS_DEFAULT_PROFILE, VIENNA_PROFILE, XETRA_PROFILE,
    from_profile,
};
use crate::calendar::{Exchange, MarketHours, MarketHoursKey, session_profile};

/// Build default futures trading hours per exchange.
/// NOTE: These are exchange-level defaults. Product-level variations may differ.
#[must_use]
pub fn hours_for_exchange(exch: Exchange) -> MarketHours {
    match exch {
        Exchange::Unknown => default_24x7(exch),
        // ==============================
        // US EQUITIES (ET)
        // ==============================
        // Early session opens 04:00 ET on these venues (see the profile's
        // per-venue citations).
        Exchange::Nasdaq
        | Exchange::NasdaqBx
        | Exchange::NasdaqPsx
        | Exchange::CboeBzx
        | Exchange::CboeEdgx
        | Exchange::NyseArca
        | Exchange::MemxEq
        | Exchange::MiaxPearlEq => from_profile(exch, &US_EQUITIES_PROFILE),

        // Early session opens 07:00 ET on these venues: NYSE American and
        // NYSE National ("Early Trading Session 7:00 a.m. to 9:30 a.m. ET" —
        // nyse.com), Cboe BYX and EDGA ("Early Trading Session: 7:00 a.m. to
        // 8:00 a.m." — cboe.com "Hours & Holidays").
        Exchange::CboeByx
        | Exchange::CboeEdga
        | Exchange::NyseAmerican
        | Exchange::NyseNational => from_profile(exch, &US_EQUITY_EARLY_0700_PROFILE),

        // NYSE itself trades the core session only — no early, no late
        // (nyse.com "Trading Hours"; extended trading in NYSE Group happens
        // on Arca, American, National, and Texas).
        Exchange::Nyse => from_profile(Exchange::Nyse, &NYSE_PROFILE),

        // IEX — narrower extended hours than the Reg NMS default: pre-market
        // 08:00–09:30 and post-market 16:00–17:00 ET, together with RTH forming
        // IEX's "System Hours" of 08:00–17:00 ET.
        // Source: IEX Exchange, "Trading Hours & Holidays"
        // (https://www.iex.io/resources/trading/trading-hours-holidays) and
        // Investors Exchange Rule Book Rule 1.160(z)/(aa)/(gg).
        Exchange::Iex => from_profile(Exchange::Iex, &IEX_PROFILE_POST2015),

        // NYSE Texas — opening 07:00–09:30, core 09:30–16:00, late 16:00–20:00 ET.
        Exchange::NyseTexas => from_profile(Exchange::NyseTexas, &NYSE_TEXAS_PROFILE),

        // FINRA TRFs — facility open 08:00–20:00 ET.
        Exchange::FinraTrfCarteret | Exchange::FinraTrfChicago | Exchange::FinraTrfNyse => {
            from_profile(exch, &FINRA_TRF_PROFILE)
        }

        // IntelligentCross — executes RTH 09:30–16:00; accepts orders from 09:00.
        Exchange::IntelligentcrossIqx => from_profile(Exchange::IntelligentcrossIqx, &IQX_PROFILE),

        // Blue Ocean ATS — 20:00–04:00 ET Sun + Mon–Thu (overnight).
        Exchange::BlueOceanAts => from_profile(Exchange::BlueOceanAts, &BLUE_OCEAN_PROFILE),

        // ==============================
        // US OPTIONS (ET)
        // ==============================
        // Cboe Options (C1) — handled in as_of cutover below
        Exchange::CboeOptionsC1 => {
            from_profile(Exchange::CboeOptionsC1, &C1_PROFILE_POST_2024_08_26)
        }

        // Other options venues — pragmatic default RTH 09:30–16:00 ET.
        Exchange::CboeC2Options
        | Exchange::CboeBzxOptions
        | Exchange::CboeEdgxOptions
        | Exchange::NyseArcaOptions
        | Exchange::NyseAmericanOptions
        | Exchange::NasdaqPhlx
        | Exchange::NasdaqIse
        | Exchange::NasdaqNom
        | Exchange::NasdaqMrx
        | Exchange::NasdaqGemx
        | Exchange::NasdaqBxOptions
        | Exchange::MiaxOptions
        | Exchange::MiaxEmeraldOptions
        | Exchange::MiaxPearlOptions
        | Exchange::MiaxSapphireOptions
        | Exchange::BoxOptions
        | Exchange::MemxOptions => from_profile(exch, &US_OPTIONS_DEFAULT_PROFILE),

        // ==============================
        // FUTURES / ENERGY (EU/INTL)
        // ==============================
        // ICE Europe Commodities — many contracts 01:00–23:00 London
        Exchange::IceEuropeCommodities => {
            from_profile(Exchange::IceEuropeCommodities, &ICE_EU_LONDON_01_23_PROFILE)
        }
        // ICE Europe Financials — same broad window. (Individual products vary; override if needed.)
        Exchange::IceEuropeFinancials => {
            from_profile(Exchange::IceEuropeFinancials, &ICE_EU_LONDON_01_23_PROFILE)
        }
        // ICE Endex (Amsterdam) — pragmatic default 01:00–23:00 local.
        Exchange::IceEndex => from_profile(Exchange::IceEndex, &ENDEX_01_23_PROFILE),
        // ICE Abu Dhabi (IFAD) — nearly 24×5 (22–24h); default to 01:00–23:00 GST.
        Exchange::IceAbuDhabi => from_profile(Exchange::IceAbuDhabi, &ABU_DHABI_01_23_PROFILE),
        // ICE Canada — default to 20:00→18:00 ET wrap (ICE pattern).
        Exchange::IceCanada => from_profile(Exchange::IceCanada, &ICE_CANADA_PROFILE),
        // EEX — exchange trading generally 08:00–18:00 CET.
        Exchange::Eex => from_profile(Exchange::Eex, &EEX_PROFILE),

        // ----------------------- EU Equities -----------------------
        // LSE (UK): 08:00–16:30; auctions 07:50–08:00 and 16:30–16:35, then
        // the 16:35–16:40 Closing Price Crossing.
        Exchange::Lse => from_profile(Exchange::Lse, &LSE_PROFILE),

        // Xetra / Frankfurt: 09:00–17:30; auctions 08:50–09:00 and
        // 17:30–17:35, then Trade-at-Close to 17:45.
        Exchange::Xetra => from_profile(Exchange::Xetra, &XETRA_PROFILE),

        // SIX Swiss: 09:00–17:20 continuous; closing auction 17:20–17:30 and
        // Trading-At-Last to 17:40.
        Exchange::Six => from_profile(Exchange::Six, &SIX_PROFILE),

        // Euronext venues: 09:00–17:30 (Dublin 17:28), closing auction to
        // ~17:35 and Trading-at-Last to 17:40.
        Exchange::EuronextParis => from_profile(Exchange::EuronextParis, &EURONEXT_PARIS_PROFILE),
        Exchange::EuronextAmsterdam => {
            from_profile(Exchange::EuronextAmsterdam, &EURONEXT_AMS_PROFILE)
        }
        Exchange::EuronextBrussels => {
            from_profile(Exchange::EuronextBrussels, &EURONEXT_BRU_PROFILE)
        }
        Exchange::EuronextLisbon => from_profile(Exchange::EuronextLisbon, &EURONEXT_LIS_PROFILE),
        Exchange::EuronextDublin => from_profile(Exchange::EuronextDublin, &EURONEXT_DUB_PROFILE),
        Exchange::EuronextMilan => from_profile(Exchange::EuronextMilan, &EURONEXT_MIL_PROFILE),

        // Spain (BME): 09:00–17:30; pre-open 08:30–09:00; closing auction
        // 17:30–17:35 and Trading-at-Last to 17:45.
        Exchange::Bme => from_profile(Exchange::Bme, &BME_PROFILE),

        // Nasdaq Nordic: aligned on CET but with different local hours and
        // closes — Stockholm 09:00–17:25, Helsinki 10:00–18:25 EET,
        // Copenhagen 09:00–16:55 (see the profile citations).
        Exchange::NasdaqStockholm => from_profile(Exchange::NasdaqStockholm, &NASDAQ_STO_PROFILE),
        Exchange::NasdaqHelsinki => from_profile(Exchange::NasdaqHelsinki, &NASDAQ_HEL_PROFILE),
        Exchange::NasdaqCopenhagen => from_profile(Exchange::NasdaqCopenhagen, &NASDAQ_CPH_PROFILE),

        // Vienna: 09:00–17:30; opening auction from 08:55, closing auction
        // 17:30–17:35, Trade-at-Close to 17:45.
        Exchange::Vienna => from_profile(Exchange::Vienna, &VIENNA_PROFILE),
        // ------------------------------------------------------------
        // CME (CME Globex, Equity Index default)
        // Shared GlobexEquityIndex profile (see `session_profile`).
        // Sun 17:00 – Fri 16:00 CT with daily 60-min break at 16:00;
        // RTH 08:30–15:15 CT; short window 15:30–16:00 CT; no Fri overnight.
        // ------------------------------------------------------------
        Exchange::Cme => {
            session_profile(MarketHoursKey::GlobexEquityIndex).to_market_hours(Exchange::Cme)
        }

        // ------------------------------------------------------------
        // CBOT (Grains/Oilseeds default)
        // Shared GlobexGrains profile (see `session_profile`).
        // Sun–Thu overnight 19:00–07:45 CT, day 08:30–13:20 CT; no Fri overnight.
        // ------------------------------------------------------------
        Exchange::Cbot => {
            session_profile(MarketHoursKey::GlobexGrains).to_market_hours(Exchange::Cbot)
        }

        // ------------------------------------------------------------
        // COMEX (Metals default)
        // Shared GlobexEnergy profile (see `session_profile`).
        // 17:00–16:00 CT, daily maintenance 16:00–17:00; no Fri overnight.
        // ------------------------------------------------------------
        Exchange::Comex => {
            session_profile(MarketHoursKey::GlobexEnergy).to_market_hours(Exchange::Comex)
        }

        // ------------------------------------------------------------
        // NYMEX (Energy default)
        // Shared GlobexEnergy profile (see `session_profile`); identical to COMEX.
        // Same “17:00–16:00 CT, daily break at 16:00–17:00”; no Fri overnight.
        // ------------------------------------------------------------
        Exchange::Nymex => {
            session_profile(MarketHoursKey::GlobexEnergy).to_market_hours(Exchange::Nymex)
        }

        // ------------------------------------------------------------
        // EUREX (generic index/IR default)
        // Shared Eurex profile (see `session_profile`).
        // Asian hours 01:00–08:00 CET/CEST, then regular 08:00–22:00 CET/CEST (Mon–Fri).
        // ------------------------------------------------------------
        Exchange::Eurex => session_profile(MarketHoursKey::Eurex).to_market_hours(Exchange::Eurex),

        // ------------------------------------------------------------
        // ICE Futures U.S. (common profile)
        // Shared IceUs profile (see `session_profile`).
        // Many contracts follow ~20:00–18:00 ET (22h) with daily 2h break; no Fri overnight.
        // ------------------------------------------------------------
        Exchange::Iceus => session_profile(MarketHoursKey::IceUs).to_market_hours(Exchange::Iceus),

        // ------------------------------------------------------------
        // ICE Futures Europe (common profile)
        // Typical 01:00–23:00 London (Mon–Fri). Equity-only profile remains GUI-local.
        // ------------------------------------------------------------
        Exchange::Iceeu => from_profile(Exchange::Iceeu, &ICE_EU_LONDON_01_23_PROFILE),

        // ------------------------------------------------------------
        // SGX Derivatives (generic)
        // Shared Sgx profile (see `session_profile`).
        // T session ~07:10–20:00 SGT, T+1 ~20:00–05:15 SGT (Mon–Fri).
        // ------------------------------------------------------------
        Exchange::Sgx => session_profile(MarketHoursKey::Sgx).to_market_hours(Exchange::Sgx),

        // ------------------------------------------------------------
        // CFE (Cboe Futures – VIX default profile)
        // Shared CfeVix profile (see `session_profile`).
        // RTH 08:30–15:00 flowing seamlessly into post-settlement ETH 15:00–16:00
        // (effective 2021-12-06); overnight wrap Sun+Mon–Thu 17:00→08:30.
        // ------------------------------------------------------------
        Exchange::Cfe => session_profile(MarketHoursKey::CfeVix).to_market_hours(Exchange::Cfe),

        // ------------------------------------------------------------
        // Crypto / always-open venues
        //
        // Shared AlwaysOpen profile (see `session_profile`).
        // 24×7 UTC, no maintenance break, no weekend close, midnight UTC boundaries.
        // Deliberately NOT modeled as futures-session venues; the always-open
        // contract is kept explicit and separate from CME-style daily-break calendars.
        //
        // Note: Product-level nuances such as Binance quarterly expiry pauses are
        // deferred to a later product-level calendar layer.
        // ------------------------------------------------------------
        Exchange::BinanceFutures => {
            session_profile(MarketHoursKey::AlwaysOpen).to_market_hours(Exchange::BinanceFutures)
        } // All known exchanges covered; no default arm.
    }
}

/// Build a 24×7 UTC profile for always-open venues (crypto perpetuals, etc.).
///
/// This profile models a market that never closes: every day of the week is
/// active, the session spans the full 24-hour day, and the daily boundary
/// falls at midnight UTC.  It is intentionally **not** used for futures venues
/// that have daily maintenance breaks (CME, COMEX, NYMEX) or weekend gaps;
/// those use venue-specific session rules instead.
///
/// The always-open contract is kept explicit and separate from futures-session
/// rules so that callers can distinguish the two categories by inspecting the
/// session rules (an always-open venue has a single `0..86400` same-day rule
/// active on all seven days, while futures venues use multi-rule wrap patterns).
///
/// The rule data is the shared [`MarketHoursKey::AlwaysOpen`] static — one
/// source of truth for the 24×7 profile, addressed here by venue tag.
fn default_24x7(ex: Exchange) -> MarketHours {
    session_profile(MarketHoursKey::AlwaysOpen).to_market_hours(ex)
}
