// SPDX-License-Identifier: MIT-0

//! Handwritten cutover expectations kept independent of production histories.

use super::prelude::*;

/// A calendar date as `(year, month, day)`.
pub(super) type Ymd = (i32, u32, u32);

/// A UTC instant as `(year, month, day, hour, minute, second)`.
pub(super) type UtcYmdHms = (i32, u32, u32, u32, u32, u32);

/// Observable profile changes whose primary-sourced boundary is not local
/// midnight and therefore cannot be represented in [`HISTORICAL_CUTOVERS`].
pub(super) const HISTORICAL_INSTANT_CUTOVERS: &[(Exchange, UtcYmdHms)] = &[
    (Exchange::BinanceFutures, (2019, 9, 13, 4, 0, 0)),
    // ICE Canada's 2011 pre-open/open move states 18:30/19:00 CT on the
    // 2011-02-28 civil day, whose midnight falls inside the running Sunday
    // session: the sourced boundary is the 18:30 CT pre-open instant.
    (Exchange::IceCanada, (2011, 3, 1, 0, 30, 0)),
];

/// Every observable point-in-time profile change shipped by the crate.
///
/// This independent list drives rule-domain validation, historical cross-query
/// fences, and the exact venue-local-midnight contract. A temporary range has
/// entries at both its start and restoration; adjacent dates with the same
/// profile (PSE's two-day closure) need only the observable entry and exit.
pub(super) const HISTORICAL_CUTOVERS: &[(Exchange, Ymd, chrono_tz::Tz)] = &[
    (
        Exchange::Nasdaq,
        (2013, 3, 18),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::NasdaqBx,
        (2011, 4, 18),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::NasdaqPsx,
        (2010, 10, 8),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::NasdaqPsx,
        (2010, 12, 13),
        chrono_tz::America::New_York,
    ),
    (Exchange::Cme, (2010, 11, 15), chrono_tz::US::Central),
    (Exchange::Cme, (2012, 11, 18), chrono_tz::US::Central),
    (Exchange::Cme, (2015, 9, 20), chrono_tz::US::Central),
    (Exchange::Cme, (2021, 6, 27), chrono_tz::US::Central),
    (Exchange::Comex, (2015, 9, 20), chrono_tz::US::Central),
    (Exchange::Nymex, (2015, 9, 20), chrono_tz::US::Central),
    (Exchange::Eurex, (2018, 12, 10), chrono_tz::Europe::Berlin),
    (Exchange::Eex, (2024, 3, 25), chrono_tz::Europe::Berlin),
    (
        Exchange::IceEuropeFinancials,
        (2014, 11, 17),
        chrono_tz::Europe::London,
    ),
    (
        Exchange::IceEuropeFinancials,
        (2015, 2, 16),
        chrono_tz::Europe::London,
    ),
    (
        Exchange::IceEuropeFinancials,
        (2015, 10, 1),
        chrono_tz::Europe::London,
    ),
    (
        Exchange::IceEndex,
        (2013, 10, 7),
        chrono_tz::Europe::Amsterdam,
    ),
    (
        Exchange::IceEndex,
        (2026, 4, 12),
        chrono_tz::Europe::Amsterdam,
    ),
    (
        Exchange::IceEndex,
        (2026, 4, 13),
        chrono_tz::Europe::Amsterdam,
    ),
    (Exchange::IceAbuDhabi, (2021, 3, 29), chrono_tz::Asia::Dubai),
    (Exchange::Sgx, (2024, 7, 29), chrono_tz::Asia::Singapore),
    (Exchange::Lse, (2012, 4, 30), chrono_tz::Europe::London),
    (Exchange::Lse, (2016, 3, 21), chrono_tz::Europe::London),
    (Exchange::Xetra, (2020, 11, 24), chrono_tz::Europe::Berlin),
    (Exchange::Xetra, (2025, 12, 1), chrono_tz::Europe::Berlin),
    (
        Exchange::EuronextParis,
        (2023, 3, 20),
        chrono_tz::Europe::Paris,
    ),
    (
        Exchange::EuronextAmsterdam,
        (2023, 3, 20),
        chrono_tz::Europe::Amsterdam,
    ),
    (
        Exchange::EuronextBrussels,
        (2023, 3, 20),
        chrono_tz::Europe::Brussels,
    ),
    (
        Exchange::EuronextLisbon,
        (2023, 3, 20),
        chrono_tz::Europe::Lisbon,
    ),
    (
        Exchange::EuronextDublin,
        (2019, 2, 4),
        chrono_tz::Europe::Dublin,
    ),
    (
        Exchange::EuronextDublin,
        (2023, 3, 20),
        chrono_tz::Europe::Dublin,
    ),
    (
        Exchange::EuronextMilan,
        (2013, 9, 30),
        chrono_tz::Europe::Rome,
    ),
    (
        Exchange::EuronextMilan,
        (2015, 11, 23),
        chrono_tz::Europe::Rome,
    ),
    (
        Exchange::EuronextMilan,
        (2023, 3, 27),
        chrono_tz::Europe::Rome,
    ),
    (
        Exchange::NasdaqStockholm,
        (2015, 11, 16),
        chrono_tz::Europe::Stockholm,
    ),
    (
        Exchange::NasdaqHelsinki,
        (2015, 11, 16),
        chrono_tz::Europe::Helsinki,
    ),
    (
        Exchange::NasdaqCopenhagen,
        (2015, 11, 16),
        chrono_tz::Europe::Copenhagen,
    ),
    (
        Exchange::NasdaqCopenhagen,
        (2019, 5, 1),
        chrono_tz::Europe::Copenhagen,
    ),
    (Exchange::Iceus, (2017, 11, 7), chrono_tz::America::New_York),
    (Exchange::Iceus, (2017, 11, 8), chrono_tz::America::New_York),
    (
        Exchange::IceCanada,
        (2012, 6, 24),
        chrono_tz::America::Winnipeg,
    ),
    (
        Exchange::IceCanada,
        (2013, 4, 7),
        chrono_tz::America::Winnipeg,
    ),
    (
        Exchange::IceCanada,
        (2016, 1, 24),
        chrono_tz::America::Winnipeg,
    ),
    (
        Exchange::IceCanada,
        (2018, 7, 29),
        chrono_tz::America::Winnipeg,
    ),
    (Exchange::Iex, (2016, 8, 19), chrono_tz::America::New_York),
    (Exchange::Ltse, (2020, 8, 28), chrono_tz::America::New_York),
    (
        Exchange::TwentyFourX,
        (2025, 10, 14),
        chrono_tz::America::New_York,
    ),
    (Exchange::Txse, (2026, 7, 10), chrono_tz::America::New_York),
    (
        Exchange::CboeBzxOptions,
        (2010, 2, 26),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::CboeC2Options,
        (2010, 10, 29),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::CboeEdgxOptions,
        (2015, 11, 2),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::NasdaqBxOptions,
        (2012, 6, 29),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::NasdaqGemx,
        (2013, 8, 5),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::NasdaqMrx,
        (2016, 2, 16),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::MiaxOptions,
        (2012, 12, 7),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::MiaxPearlOptions,
        (2017, 2, 6),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::MiaxEmeraldOptions,
        (2019, 3, 1),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::MiaxSapphireOptions,
        (2024, 8, 12),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::MemxOptions,
        (2023, 9, 27),
        chrono_tz::America::New_York,
    ),
    (Exchange::Cfe, (2010, 12, 10), chrono_tz::US::Central),
    (Exchange::Cfe, (2011, 9, 26), chrono_tz::US::Central),
    (Exchange::Cfe, (2013, 10, 28), chrono_tz::US::Central),
    (Exchange::Cfe, (2013, 11, 4), chrono_tz::US::Central),
    (Exchange::Cfe, (2014, 6, 22), chrono_tz::US::Central),
    (Exchange::Cfe, (2018, 2, 25), chrono_tz::US::Central),
    (Exchange::Cfe, (2018, 8, 12), chrono_tz::US::Central),
    (Exchange::Cfe, (2021, 12, 6), chrono_tz::US::Central),
    (
        Exchange::NyseTexas,
        (2019, 11, 4),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::BlueOceanAts,
        (2021, 10, 5),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::FinraTrfCarteret,
        (2026, 3, 30),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::FinraTrfChicago,
        (2018, 9, 10),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::FinraTrfChicago,
        (2026, 3, 30),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::FinraTrfNyse,
        (2026, 3, 30),
        chrono_tz::America::New_York,
    ),
    (Exchange::Cbot, (2010, 4, 19), chrono_tz::US::Central),
    (Exchange::Cbot, (2011, 12, 27), chrono_tz::US::Central),
    (Exchange::Cbot, (2012, 5, 20), chrono_tz::US::Central),
    (Exchange::Cbot, (2013, 4, 7), chrono_tz::US::Central),
    (Exchange::Cbot, (2013, 8, 18), chrono_tz::US::Central),
    (Exchange::Cbot, (2015, 7, 5), chrono_tz::US::Central),
    (
        Exchange::CboeBzx,
        (2014, 12, 2),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::CboeBzx,
        (2016, 5, 25),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::CboeBzx,
        (2018, 7, 30),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::CboeBzx,
        (2025, 5, 1),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::CboeByx,
        (2010, 10, 15),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::CboeByx,
        (2014, 12, 1),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::CboeByx,
        (2016, 5, 23),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::CboeByx,
        (2018, 8, 27),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::CboeEdga,
        (2010, 7, 2),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::CboeEdga,
        (2016, 5, 24),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::CboeEdgx,
        (2010, 7, 2),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::CboeEdgx,
        (2016, 5, 26),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::CboeEdgx,
        (2021, 3, 8),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::CboeEdgx,
        (2021, 9, 7),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::NyseAmerican,
        (2017, 7, 24),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::NyseNational,
        (2010, 8, 2),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::NyseNational,
        (2014, 5, 16),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::NyseNational,
        (2014, 5, 31),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::NyseNational,
        (2015, 12, 22),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::NyseNational,
        (2017, 2, 1),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::NyseNational,
        (2018, 5, 21),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::MemxEq,
        (2020, 9, 21),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::MemxEq,
        (2020, 10, 5),
        chrono_tz::America::New_York,
    ),
    (Exchange::MemxEq, (2023, 2, 1), chrono_tz::America::New_York),
    (
        Exchange::MemxEq,
        (2025, 5, 19),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::MiaxPearlEq,
        (2020, 9, 29),
        chrono_tz::America::New_York,
    ),
    (
        Exchange::MiaxPearlEq,
        (2025, 2, 20),
        chrono_tz::America::New_York,
    ),
    (Exchange::Asx, (2025, 6, 23), chrono_tz::Australia::Sydney),
    (
        Exchange::TmxAustralia,
        (2011, 10, 31),
        chrono_tz::Australia::Sydney,
    ),
    (
        Exchange::TmxAustralia,
        (2013, 12, 9),
        chrono_tz::Australia::Sydney,
    ),
    (
        Exchange::TmxAustralia,
        (2015, 8, 31),
        chrono_tz::Australia::Sydney,
    ),
    (
        Exchange::TmxAustralia,
        (2025, 3, 17),
        chrono_tz::Australia::Sydney,
    ),
    (Exchange::Nzx, (2020, 4, 6), chrono_tz::Pacific::Auckland),
    (Exchange::Tse, (2011, 11, 21), chrono_tz::Asia::Tokyo),
    (Exchange::Tse, (2024, 11, 5), chrono_tz::Asia::Tokyo),
    (Exchange::NseIndia, (2010, 1, 4), chrono_tz::Asia::Kolkata),
    (Exchange::NseIndia, (2010, 10, 18), chrono_tz::Asia::Kolkata),
    (Exchange::NseIndia, (2011, 10, 3), chrono_tz::Asia::Kolkata),
    (Exchange::NseIndia, (2026, 8, 3), chrono_tz::Asia::Kolkata),
    (Exchange::BseIndia, (2010, 1, 4), chrono_tz::Asia::Kolkata),
    (Exchange::BseIndia, (2010, 10, 18), chrono_tz::Asia::Kolkata),
    (Exchange::BseIndia, (2026, 8, 3), chrono_tz::Asia::Kolkata),
    (Exchange::Hkex, (2011, 3, 7), chrono_tz::Asia::Hong_Kong),
    (Exchange::Hkex, (2016, 7, 25), chrono_tz::Asia::Hong_Kong),
    (
        Exchange::SgxSecurities,
        (2011, 8, 1),
        chrono_tz::Asia::Singapore,
    ),
    (
        Exchange::SgxSecurities,
        (2017, 11, 13),
        chrono_tz::Asia::Singapore,
    ),
    (
        Exchange::SgxSecurities,
        (2019, 6, 3),
        chrono_tz::Asia::Singapore,
    ),
    (
        Exchange::SetThailand,
        (2024, 3, 25),
        chrono_tz::Asia::Bangkok,
    ),
    (
        Exchange::SetThailand,
        (2025, 5, 6),
        chrono_tz::Asia::Bangkok,
    ),
    (
        Exchange::SetThailand,
        (2025, 5, 7),
        chrono_tz::Asia::Bangkok,
    ),
    (Exchange::Idx, (2013, 1, 2), chrono_tz::Asia::Jakarta),
    (Exchange::Idx, (2020, 3, 30), chrono_tz::Asia::Jakarta),
    (Exchange::Idx, (2023, 4, 3), chrono_tz::Asia::Jakarta),
    (Exchange::Pse, (2011, 10, 1), chrono_tz::Asia::Manila),
    (Exchange::Pse, (2012, 1, 2), chrono_tz::Asia::Manila),
    (Exchange::Pse, (2013, 11, 4), chrono_tz::Asia::Manila),
    (Exchange::Pse, (2020, 3, 16), chrono_tz::Asia::Manila),
    (Exchange::Pse, (2020, 3, 17), chrono_tz::Asia::Manila),
    (Exchange::Pse, (2020, 3, 19), chrono_tz::Asia::Manila),
    (Exchange::Pse, (2021, 12, 6), chrono_tz::Asia::Manila),
    (Exchange::Pse, (2022, 1, 14), chrono_tz::Asia::Manila),
    (Exchange::Pse, (2022, 2, 2), chrono_tz::Asia::Manila),
    (Exchange::Pse, (2024, 3, 1), chrono_tz::Asia::Manila),
    (Exchange::Hose, (2010, 9, 13), chrono_tz::Asia::Ho_Chi_Minh),
    (Exchange::Hose, (2012, 3, 5), chrono_tz::Asia::Ho_Chi_Minh),
    (Exchange::Hose, (2013, 7, 22), chrono_tz::Asia::Ho_Chi_Minh),
    (Exchange::Sse, (2018, 8, 20), chrono_tz::Asia::Shanghai),
    (Exchange::Szse, (2016, 5, 9), chrono_tz::Asia::Shanghai),
    (Exchange::Krx, (2016, 8, 1), chrono_tz::Asia::Seoul),
    (Exchange::Krx, (2019, 4, 29), chrono_tz::Asia::Seoul),
    (Exchange::Twse, (2020, 3, 23), chrono_tz::Asia::Taipei),
    (Exchange::Six, (2020, 6, 22), chrono_tz::Europe::Zurich),
    (Exchange::Bme, (2023, 12, 4), chrono_tz::Europe::Madrid),
    (Exchange::Vienna, (2017, 7, 31), chrono_tz::Europe::Vienna),
    (Exchange::Vienna, (2019, 5, 2), chrono_tz::Europe::Vienna),
    (Exchange::Vienna, (2020, 12, 1), chrono_tz::Europe::Vienna),
    (
        Exchange::BorsaIstanbul,
        (2012, 3, 2),
        chrono_tz::Europe::Istanbul,
    ),
    (
        Exchange::BorsaIstanbul,
        (2012, 7, 16),
        chrono_tz::Europe::Istanbul,
    ),
    (
        Exchange::BorsaIstanbul,
        (2013, 4, 5),
        chrono_tz::Europe::Istanbul,
    ),
    (
        Exchange::BorsaIstanbul,
        (2013, 6, 10),
        chrono_tz::Europe::Istanbul,
    ),
    (
        Exchange::BorsaIstanbul,
        (2015, 11, 30),
        chrono_tz::Europe::Istanbul,
    ),
    (
        Exchange::BorsaIstanbul,
        (2016, 3, 28),
        chrono_tz::Europe::Istanbul,
    ),
    (
        Exchange::BorsaIstanbul,
        (2016, 11, 14),
        chrono_tz::Europe::Istanbul,
    ),
    (
        Exchange::BorsaIstanbul,
        (2019, 10, 4),
        chrono_tz::Europe::Istanbul,
    ),
    (Exchange::Jse, (2012, 7, 2), chrono_tz::Africa::Johannesburg),
    (
        Exchange::Jse,
        (2013, 11, 11),
        chrono_tz::Africa::Johannesburg,
    ),
    (
        Exchange::Jse,
        (2016, 9, 26),
        chrono_tz::Africa::Johannesburg,
    ),
    (
        Exchange::Jse,
        (2020, 8, 24),
        chrono_tz::Africa::Johannesburg,
    ),
    (Exchange::Jse, (2021, 2, 1), chrono_tz::Africa::Johannesburg),
    (
        Exchange::Jse,
        (2021, 2, 15),
        chrono_tz::Africa::Johannesburg,
    ),
    (Exchange::Tadawul, (2013, 6, 29), chrono_tz::Asia::Riyadh),
    (Exchange::Tadawul, (2016, 4, 3), chrono_tz::Asia::Riyadh),
    (Exchange::Tadawul, (2018, 5, 27), chrono_tz::Asia::Riyadh),
    (Exchange::Tadawul, (2019, 5, 12), chrono_tz::Asia::Riyadh),
    (Exchange::Tadawul, (2020, 3, 26), chrono_tz::Asia::Riyadh),
    (Exchange::Tadawul, (2020, 5, 31), chrono_tz::Asia::Riyadh),
    (Exchange::B3, (2010, 3, 15), chrono_tz::America::Sao_Paulo),
    (Exchange::B3, (2010, 10, 18), chrono_tz::America::Sao_Paulo),
    (Exchange::B3, (2011, 3, 14), chrono_tz::America::Sao_Paulo),
    (Exchange::B3, (2011, 10, 17), chrono_tz::America::Sao_Paulo),
    (Exchange::B3, (2012, 3, 12), chrono_tz::America::Sao_Paulo),
    (Exchange::B3, (2012, 12, 3), chrono_tz::America::Sao_Paulo),
    (Exchange::B3, (2013, 7, 8), chrono_tz::America::Sao_Paulo),
    (Exchange::B3, (2015, 12, 21), chrono_tz::America::Sao_Paulo),
    (
        Exchange::Bmv,
        (2010, 3, 16),
        chrono_tz::America::Mexico_City,
    ),
    (Exchange::Bmv, (2010, 4, 1), chrono_tz::America::Mexico_City),
    (
        Exchange::Bmv,
        (2010, 11, 1),
        chrono_tz::America::Mexico_City,
    ),
    (Exchange::Bmv, (2016, 9, 5), chrono_tz::America::Mexico_City),
    (
        Exchange::Bmv,
        (2023, 5, 29),
        chrono_tz::America::Mexico_City,
    ),
    (
        Exchange::Bmv,
        (2023, 11, 6),
        chrono_tz::America::Mexico_City,
    ),
    // MIAX Options' 07:30 window existed at launch but did not affect the live
    // book; the first capture showing it live-book is 2013-05-07.
    (
        Exchange::MiaxOptions,
        (2013, 5, 7),
        chrono_tz::America::New_York,
    ),
    // Knowledge-bound rows that remain: the cash-equity and CME venues whose
    // verified-current order-acceptance (and, for NYSE, early-session) phases
    // apply from the 2026-08-22 repository review. The seventeen US options
    // venues no longer appear here - their queues are carried across history,
    // so no profile changes on that day.
    (
        Exchange::CboeEdga,
        (2026, 8, 22),
        chrono_tz::America::New_York,
    ),
    (Exchange::Nyse, (2026, 8, 22), chrono_tz::America::New_York),
    (
        Exchange::NyseArca,
        (2026, 8, 22),
        chrono_tz::America::New_York,
    ),
    (Exchange::Cme, (2026, 8, 22), chrono_tz::US::Central),
    (Exchange::Comex, (2026, 8, 22), chrono_tz::US::Central),
    (Exchange::Nymex, (2026, 8, 22), chrono_tz::US::Central),
];
