// SPDX-License-Identifier: MIT-0

//! Handwritten cutover expectations kept independent of production histories.

use super::prelude::*;
use crate::support::zones::{
    AMSTERDAM, AUCKLAND, BANGKOK, BERLIN, BRUSSELS, CHICAGO, COPENHAGEN, DUBAI, DUBLIN, HELSINKI,
    HO_CHI_MINH, HONG_KONG, ISTANBUL, JAKARTA, JOHANNESBURG, KOLKATA, LISBON, LONDON, MADRID,
    MANILA, MEXICO_CITY, NEW_YORK, PARIS, RIYADH, ROME, SAO_PAULO, SEOUL, SHANGHAI, SINGAPORE,
    STOCKHOLM, SYDNEY, TAIPEI, TOKYO, US_CENTRAL, VIENNA, WINNIPEG, ZURICH,
};
use jiff::tz::TimeZone;

/// A calendar date as `(year, month, day)`.
pub(super) type Ymd = (i16, i8, i8);

/// A UTC instant as `(year, month, day, hour, minute, second)`.
pub(super) type UtcYmdHms = (i16, i8, i8, i8, i8, i8);

/// Observable profile changes whose primary-sourced boundary is not local
/// midnight and therefore cannot be represented in [`HISTORICAL_CUTOVERS`].
pub(super) const HISTORICAL_INSTANT_CUTOVERS: &[(Exchange, UtcYmdHms)] = &[
    (Exchange::BinanceFutures, (2019, 9, 13, 4, 0, 0)),
    // ICE Canada's 2011 pre-open/open move states 18:30/19:00 CT on the
    // 2011-02-28 civil day, whose midnight falls inside the running Sunday
    // session: the sourced boundary is the 18:30 CT pre-open instant.
    (Exchange::IceCanada, (2011, 3, 1, 0, 30, 0)),
    // FairX, now Coinbase Derivatives, opened for trading on Monday 2021-06-28
    // at 09:00 ET (08:00 CT); no Sunday-evening session preceded it.
    (Exchange::CoinbaseDerivatives, (2021, 6, 28, 13, 0, 0)),
];

/// Every observable point-in-time profile change shipped by the crate.
///
/// This independent list drives rule-domain validation, historical cross-query
/// fences, and the exact venue-local-midnight contract. A temporary range has
/// entries at both its start and restoration; adjacent dates with the same
/// profile (PSE's two-day closure) need only the observable entry and exit.
pub(super) const HISTORICAL_CUTOVERS: &[(Exchange, Ymd, TimeZone)] = &[
    (Exchange::Nasdaq, (2013, 3, 18), NEW_YORK),
    (Exchange::NasdaqBx, (2011, 4, 18), NEW_YORK),
    (Exchange::NasdaqPsx, (2010, 10, 8), NEW_YORK),
    (Exchange::NasdaqPsx, (2010, 12, 13), NEW_YORK),
    (Exchange::Cme, (2010, 11, 15), US_CENTRAL),
    (Exchange::Cme, (2012, 11, 18), US_CENTRAL),
    (Exchange::Cme, (2015, 9, 20), US_CENTRAL),
    (Exchange::Cme, (2021, 6, 27), US_CENTRAL),
    (Exchange::Comex, (2015, 9, 20), US_CENTRAL),
    (Exchange::Nymex, (2015, 9, 20), US_CENTRAL),
    (Exchange::Eurex, (2018, 12, 10), BERLIN),
    (Exchange::Eex, (2024, 3, 25), BERLIN),
    (Exchange::IceEuropeFinancials, (2014, 11, 17), LONDON),
    (Exchange::IceEuropeFinancials, (2015, 2, 16), LONDON),
    (Exchange::IceEuropeFinancials, (2015, 10, 1), LONDON),
    (Exchange::IceEndex, (2013, 10, 7), AMSTERDAM),
    (Exchange::IceEndex, (2026, 4, 12), AMSTERDAM),
    (Exchange::IceEndex, (2026, 4, 13), AMSTERDAM),
    (Exchange::IceAbuDhabi, (2021, 3, 29), DUBAI),
    (Exchange::Sgx, (2024, 7, 29), SINGAPORE),
    (Exchange::Lse, (2012, 4, 30), LONDON),
    (Exchange::Lse, (2016, 3, 21), LONDON),
    (Exchange::Xetra, (2020, 11, 24), BERLIN),
    (Exchange::Xetra, (2025, 12, 1), BERLIN),
    (Exchange::EuronextParis, (2023, 3, 20), PARIS),
    (Exchange::EuronextAmsterdam, (2023, 3, 20), AMSTERDAM),
    (Exchange::EuronextBrussels, (2023, 3, 20), BRUSSELS),
    (Exchange::EuronextLisbon, (2023, 3, 20), LISBON),
    (Exchange::EuronextDublin, (2019, 2, 4), DUBLIN),
    (Exchange::EuronextDublin, (2023, 3, 20), DUBLIN),
    (Exchange::EuronextMilan, (2013, 9, 30), ROME),
    (Exchange::EuronextMilan, (2015, 11, 23), ROME),
    (Exchange::EuronextMilan, (2023, 3, 27), ROME),
    (Exchange::NasdaqStockholm, (2015, 11, 16), STOCKHOLM),
    (Exchange::NasdaqHelsinki, (2015, 11, 16), HELSINKI),
    (Exchange::NasdaqCopenhagen, (2015, 11, 16), COPENHAGEN),
    (Exchange::NasdaqCopenhagen, (2019, 5, 1), COPENHAGEN),
    (Exchange::Iceus, (2017, 11, 7), NEW_YORK),
    (Exchange::Iceus, (2017, 11, 8), NEW_YORK),
    (Exchange::IceCanada, (2012, 6, 24), WINNIPEG),
    (Exchange::IceCanada, (2013, 4, 7), WINNIPEG),
    (Exchange::IceCanada, (2016, 1, 24), WINNIPEG),
    (Exchange::IceCanada, (2018, 7, 29), WINNIPEG),
    (Exchange::Iex, (2016, 8, 19), NEW_YORK),
    (Exchange::Ltse, (2020, 8, 28), NEW_YORK),
    (Exchange::TwentyFourX, (2025, 10, 14), NEW_YORK),
    (Exchange::Txse, (2026, 7, 10), NEW_YORK),
    (Exchange::CboeBzxOptions, (2010, 2, 26), NEW_YORK),
    (Exchange::CboeC2Options, (2010, 10, 29), NEW_YORK),
    (Exchange::CboeEdgxOptions, (2015, 11, 2), NEW_YORK),
    (Exchange::NasdaqBxOptions, (2012, 6, 29), NEW_YORK),
    (Exchange::NasdaqGemx, (2013, 8, 5), NEW_YORK),
    (Exchange::NasdaqMrx, (2016, 2, 16), NEW_YORK),
    (Exchange::MiaxOptions, (2012, 12, 7), NEW_YORK),
    (Exchange::MiaxPearlOptions, (2017, 2, 6), NEW_YORK),
    (Exchange::MiaxEmeraldOptions, (2019, 3, 1), NEW_YORK),
    (Exchange::MiaxSapphireOptions, (2024, 8, 12), NEW_YORK),
    (Exchange::MemxOptions, (2023, 9, 27), NEW_YORK),
    (Exchange::Cfe, (2010, 12, 10), US_CENTRAL),
    (Exchange::Cfe, (2011, 9, 26), US_CENTRAL),
    (Exchange::Cfe, (2013, 10, 28), US_CENTRAL),
    (Exchange::Cfe, (2013, 11, 4), US_CENTRAL),
    (Exchange::Cfe, (2014, 6, 22), US_CENTRAL),
    (Exchange::Cfe, (2018, 2, 25), US_CENTRAL),
    (Exchange::Cfe, (2018, 8, 12), US_CENTRAL),
    (Exchange::CoinbaseDerivatives, (2021, 6, 29), CHICAGO),
    (Exchange::CoinbaseDerivatives, (2026, 9, 11), CHICAGO),
    (Exchange::Smfe, (2020, 5, 18), CHICAGO),
    (Exchange::Smfe, (2024, 11, 4), CHICAGO),
    (Exchange::Smfe, (2025, 3, 24), CHICAGO),
    (Exchange::Cfe, (2021, 12, 6), US_CENTRAL),
    (Exchange::NyseTexas, (2019, 11, 4), NEW_YORK),
    (Exchange::BlueOceanAts, (2021, 10, 5), NEW_YORK),
    (Exchange::FinraTrfCarteret, (2026, 3, 30), NEW_YORK),
    (Exchange::FinraTrfChicago, (2018, 9, 10), NEW_YORK),
    (Exchange::FinraTrfChicago, (2026, 3, 30), NEW_YORK),
    (Exchange::FinraTrfNyse, (2026, 3, 30), NEW_YORK),
    // Cbot's 2010-04-19 PCP expansion carries no entry: its changeover back to
    // 14:30 is undated (bracketed 2010-04-19..2012-05-11 by the operator's own
    // trading-hours capture), so the pre-regime span serves the sourced
    // intersection and nothing observable changes on that day (#283).
    (Exchange::Cbot, (2011, 12, 27), US_CENTRAL),
    (Exchange::Cbot, (2012, 5, 20), US_CENTRAL),
    (Exchange::Cbot, (2013, 4, 7), US_CENTRAL),
    (Exchange::Cbot, (2013, 8, 18), US_CENTRAL),
    (Exchange::Cbot, (2015, 7, 5), US_CENTRAL),
    (Exchange::CboeBzx, (2014, 12, 2), NEW_YORK),
    (Exchange::CboeBzx, (2016, 5, 25), NEW_YORK),
    (Exchange::CboeBzx, (2018, 7, 30), NEW_YORK),
    (Exchange::CboeBzx, (2025, 5, 1), NEW_YORK),
    (Exchange::CboeByx, (2010, 10, 15), NEW_YORK),
    (Exchange::CboeByx, (2014, 12, 1), NEW_YORK),
    (Exchange::CboeByx, (2016, 5, 23), NEW_YORK),
    (Exchange::CboeByx, (2018, 8, 27), NEW_YORK),
    (Exchange::CboeEdga, (2010, 7, 2), NEW_YORK),
    // SEC Release 34-73592 approved SR-EDGA-2014-20, which first stated the
    // 06:00 order-entry start in Rule 11.1(a)(1); 34-73468 did the same for
    // EDGX two weeks earlier.
    (Exchange::CboeEdga, (2014, 11, 13), NEW_YORK),
    (Exchange::CboeEdga, (2016, 5, 24), NEW_YORK),
    (Exchange::CboeEdgx, (2010, 7, 2), NEW_YORK),
    (Exchange::CboeEdgx, (2014, 10, 29), NEW_YORK),
    (Exchange::CboeEdgx, (2016, 5, 26), NEW_YORK),
    (Exchange::CboeEdgx, (2021, 3, 8), NEW_YORK),
    (Exchange::CboeEdgx, (2021, 9, 7), NEW_YORK),
    (Exchange::NyseAmerican, (2017, 7, 24), NEW_YORK),
    (Exchange::NyseNational, (2010, 8, 2), NEW_YORK),
    (Exchange::NyseNational, (2014, 5, 16), NEW_YORK),
    (Exchange::NyseNational, (2014, 5, 31), NEW_YORK),
    (Exchange::NyseNational, (2015, 12, 22), NEW_YORK),
    (Exchange::NyseNational, (2017, 2, 1), NEW_YORK),
    (Exchange::NyseNational, (2018, 5, 21), NEW_YORK),
    (Exchange::MemxEq, (2020, 9, 21), NEW_YORK),
    (Exchange::MemxEq, (2020, 10, 5), NEW_YORK),
    (Exchange::MemxEq, (2023, 2, 1), NEW_YORK),
    (Exchange::MemxEq, (2025, 5, 19), NEW_YORK),
    (Exchange::MiaxPearlEq, (2020, 9, 29), NEW_YORK),
    (Exchange::MiaxPearlEq, (2025, 2, 20), NEW_YORK),
    (Exchange::Asx, (2025, 6, 23), SYDNEY),
    (Exchange::TmxAustralia, (2011, 10, 31), SYDNEY),
    (Exchange::TmxAustralia, (2013, 12, 9), SYDNEY),
    (Exchange::TmxAustralia, (2015, 8, 31), SYDNEY),
    (Exchange::TmxAustralia, (2025, 3, 17), SYDNEY),
    (Exchange::Nzx, (2020, 4, 6), AUCKLAND),
    (Exchange::Tse, (2011, 11, 21), TOKYO),
    (Exchange::Tse, (2024, 11, 5), TOKYO),
    (Exchange::NseIndia, (2010, 1, 4), KOLKATA),
    (Exchange::NseIndia, (2010, 10, 18), KOLKATA),
    (Exchange::NseIndia, (2011, 10, 3), KOLKATA),
    (Exchange::NseIndia, (2018, 1, 1), KOLKATA),
    (Exchange::NseIndia, (2026, 8, 3), KOLKATA),
    (Exchange::NseIndia, (2026, 12, 7), KOLKATA),
    (Exchange::BseIndia, (2010, 1, 4), KOLKATA),
    (Exchange::BseIndia, (2010, 10, 18), KOLKATA),
    (Exchange::BseIndia, (2026, 8, 3), KOLKATA),
    (Exchange::Hkex, (2011, 3, 7), HONG_KONG),
    (Exchange::Hkex, (2016, 7, 25), HONG_KONG),
    (Exchange::SgxSecurities, (2011, 8, 1), SINGAPORE),
    (Exchange::SgxSecurities, (2017, 11, 13), SINGAPORE),
    (Exchange::SgxSecurities, (2019, 6, 3), SINGAPORE),
    (Exchange::SetThailand, (2024, 3, 25), BANGKOK),
    (Exchange::SetThailand, (2025, 5, 6), BANGKOK),
    (Exchange::SetThailand, (2025, 5, 7), BANGKOK),
    (Exchange::Idx, (2013, 1, 2), JAKARTA),
    (Exchange::Idx, (2020, 3, 30), JAKARTA),
    (Exchange::Idx, (2023, 4, 3), JAKARTA),
    (Exchange::Pse, (2011, 10, 1), MANILA),
    (Exchange::Pse, (2012, 1, 2), MANILA),
    (Exchange::Pse, (2013, 11, 4), MANILA),
    (Exchange::Pse, (2020, 3, 16), MANILA),
    (Exchange::Pse, (2020, 3, 17), MANILA),
    (Exchange::Pse, (2020, 3, 19), MANILA),
    (Exchange::Pse, (2021, 12, 6), MANILA),
    (Exchange::Pse, (2022, 1, 14), MANILA),
    (Exchange::Pse, (2022, 2, 2), MANILA),
    (Exchange::Pse, (2024, 3, 1), MANILA),
    (Exchange::Hose, (2010, 9, 13), HO_CHI_MINH),
    (Exchange::Hose, (2012, 3, 5), HO_CHI_MINH),
    (Exchange::Hose, (2013, 7, 22), HO_CHI_MINH),
    (Exchange::Sse, (2018, 8, 20), SHANGHAI),
    (Exchange::Szse, (2016, 5, 9), SHANGHAI),
    (Exchange::Krx, (2016, 8, 1), SEOUL),
    (Exchange::Krx, (2019, 4, 29), SEOUL),
    (Exchange::Twse, (2020, 3, 23), TAIPEI),
    (Exchange::Six, (2020, 6, 22), ZURICH),
    (Exchange::Bme, (2023, 12, 4), MADRID),
    (Exchange::Vienna, (2017, 7, 31), VIENNA),
    (Exchange::Vienna, (2019, 5, 2), VIENNA),
    (Exchange::Vienna, (2020, 12, 1), VIENNA),
    (Exchange::BorsaIstanbul, (2012, 3, 2), ISTANBUL),
    (Exchange::BorsaIstanbul, (2012, 7, 16), ISTANBUL),
    (Exchange::BorsaIstanbul, (2013, 4, 5), ISTANBUL),
    (Exchange::BorsaIstanbul, (2013, 6, 10), ISTANBUL),
    (Exchange::BorsaIstanbul, (2015, 11, 30), ISTANBUL),
    (Exchange::BorsaIstanbul, (2016, 3, 28), ISTANBUL),
    (Exchange::BorsaIstanbul, (2016, 11, 14), ISTANBUL),
    (Exchange::BorsaIstanbul, (2019, 10, 4), ISTANBUL),
    (Exchange::Jse, (2012, 7, 2), JOHANNESBURG),
    (Exchange::Jse, (2013, 11, 11), JOHANNESBURG),
    (Exchange::Jse, (2016, 9, 26), JOHANNESBURG),
    (Exchange::Jse, (2020, 8, 24), JOHANNESBURG),
    (Exchange::Jse, (2021, 2, 1), JOHANNESBURG),
    (Exchange::Jse, (2021, 2, 15), JOHANNESBURG),
    (Exchange::Tadawul, (2013, 6, 29), RIYADH),
    (Exchange::Tadawul, (2016, 4, 3), RIYADH),
    (Exchange::Tadawul, (2018, 5, 27), RIYADH),
    (Exchange::Tadawul, (2019, 5, 12), RIYADH),
    (Exchange::Tadawul, (2020, 3, 26), RIYADH),
    (Exchange::Tadawul, (2020, 5, 31), RIYADH),
    (Exchange::B3, (2010, 3, 15), SAO_PAULO),
    (Exchange::B3, (2010, 10, 18), SAO_PAULO),
    (Exchange::B3, (2011, 3, 14), SAO_PAULO),
    (Exchange::B3, (2011, 10, 17), SAO_PAULO),
    (Exchange::B3, (2012, 3, 12), SAO_PAULO),
    (Exchange::B3, (2012, 12, 3), SAO_PAULO),
    (Exchange::B3, (2013, 7, 8), SAO_PAULO),
    (Exchange::B3, (2015, 12, 21), SAO_PAULO),
    (Exchange::Bmv, (2010, 3, 16), MEXICO_CITY),
    (Exchange::Bmv, (2010, 4, 1), MEXICO_CITY),
    (Exchange::Bmv, (2010, 11, 1), MEXICO_CITY),
    (Exchange::Bmv, (2016, 9, 5), MEXICO_CITY),
    (Exchange::Bmv, (2023, 5, 29), MEXICO_CITY),
    (Exchange::Bmv, (2023, 11, 6), MEXICO_CITY),
    // MIAX Options' 07:30 window existed at launch but did not affect the live
    // book; the first capture showing it live-book is 2013-05-07.
    (Exchange::MiaxOptions, (2013, 5, 7), NEW_YORK),
    // NYSE began trading UTP securities on Pillar on 2018-04-09, the day its
    // Rule 7.34 06:30 acceptance edge and 07:00 Early Trading Session first
    // reached the venue envelope.
    (Exchange::Nyse, (2018, 4, 9), NEW_YORK),
    // NYSE Arca moved its rulebook acceptance edge from 03:30 to 02:30 on
    // Monday 2021-09-13, the day its own Trader Update names.
    (Exchange::NyseArca, (2021, 9, 13), NEW_YORK),
    // The CME venues' Sunday 16:00 CT order-acceptance onset, which the
    // 2026-10-16 market-data sweep moved down to the first witnessed Sunday
    // from its 2026-08-22 review-date key. The seventeen US options venues
    // no longer appear here - their queues are carried across history, so no
    // profile changes on that day. Cboe EDGA no longer appears either: its
    // queue now starts on the sourced 2014-11-13 approval day. Nor do NYSE
    // and NYSE Arca, whose acceptance and early-session phases are dated
    // above.
    (Exchange::Cme, (2016, 1, 3), US_CENTRAL),
    (Exchange::Comex, (2016, 1, 3), US_CENTRAL),
    (Exchange::Nymex, (2016, 1, 3), US_CENTRAL),
];
