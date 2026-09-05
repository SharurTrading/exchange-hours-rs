<!-- SPDX-License-Identifier: MIT-0 -->

# Databento venue crosswalk

This is the explicit crosswalk for the Databento venue inventory supplied for
integration on 2026-09-04. The inventory has 50 distinct venue labels: 19
equities, 18 equity-options, and 13 futures venues. The futures venues are also
the complete set repeated under options on futures.

Every label maps to a real, non-`Unknown` [`Exchange`](../../src/calendar/exchange/mod.rs)
variant. No new enum identity is required for this inventory. Repeated feeds for
Nasdaq, NYSE, EEX, and Eurex collapse to their one venue identity, and the
futures and options-on-futures sections deliberately share identities because
the product class does not create a second exchange.

`Primary` and `Partial` have the precise meanings defined by the
[schedule verification ledger](verification.md#basis). Both mean the current
normal-week schedule is backed by public primary material. `Partial` means only
that a named historical phase or exact cutover is still qualified; the ledger
row documents that gap. There are no Secondary, Pragmatic, Known issue,
Synthetic, or unmapped entries in this crosswalk.

This table is an integration map, not an alternate parser. `Exchange::from_str`
continues to accept only the stable canonical wire name in the fourth column;
callers translate the vendor label before parsing. Feed `Available from` dates
are dataset-coverage dates, not exchange launch dates or schedule-revision
evidence, and therefore do not alter the historical selectors.

## Crosswalk

| Dataset section | Databento venue label | `Exchange` variant | Canonical wire name | Ledger basis |
|---|---|---|---|---|
| Equities | Blue Ocean ATS | `Exchange::BlueOceanAts` | `blue_ocean_ats` | Primary |
| Equities | Cboe BYX | `Exchange::CboeByx` | `cboe_byx` | Primary |
| Equities | Cboe BZX | `Exchange::CboeBzx` | `cboe_bzx` | Primary |
| Equities | Cboe EDGA | `Exchange::CboeEdga` | `cboe_edga` | Partial |
| Equities | Cboe EDGX | `Exchange::CboeEdgx` | `cboe_edgx` | Partial |
| Equities | FINRA/Nasdaq TRF Carteret | `Exchange::FinraTrfCarteret` | `finra_trf_carteret` | Primary |
| Equities | FINRA/Nasdaq TRF Chicago | `Exchange::FinraTrfChicago` | `finra_trf_chicago` | Primary |
| Equities | FINRA/NYSE TRF | `Exchange::FinraTrfNyse` | `finra_trf_nyse` | Primary |
| Equities | IEX | `Exchange::Iex` | `iex` | Primary |
| Equities | MEMX | `Exchange::MemxEq` | `memx_eq` | Primary |
| Equities | MIAX Pearl Equities | `Exchange::MiaxPearlEq` | `miax_pearl_eq` | Primary |
| Equities | Nasdaq | `Exchange::Nasdaq` | `nasdaq` | Primary |
| Equities | Nasdaq PSX | `Exchange::NasdaqPsx` | `nasdaq_psx` | Primary |
| Equities | Nasdaq Texas | `Exchange::NasdaqBx` | `nasdaq_bx` | Primary |
| Equities | NYSE | `Exchange::Nyse` | `nyse` | Partial |
| Equities | NYSE American | `Exchange::NyseAmerican` | `nyse_american` | Partial |
| Equities | NYSE Arca | `Exchange::NyseArca` | `nyse_arca` | Primary |
| Equities | NYSE National | `Exchange::NyseNational` | `nyse_national` | Partial |
| Equities | NYSE Texas | `Exchange::NyseTexas` | `nyse_texas` | Primary |
| Equity options | BOX Options | `Exchange::BoxOptions` | `box_options` | Partial |
| Equity options | Cboe BZX Options | `Exchange::CboeBzxOptions` | `cboe_bzx_options` | Partial |
| Equity options | Cboe C2 Options | `Exchange::CboeC2Options` | `cboe_c2_options` | Partial |
| Equity options | Cboe EDGX Options | `Exchange::CboeEdgxOptions` | `cboe_edgx_options` | Partial |
| Equity options | Cboe Options | `Exchange::CboeOptionsC1` | `cboe_options_c1` | Partial |
| Equity options | MEMX Options | `Exchange::MemxOptions` | `memx_options` | Primary |
| Equity options | MIAX Emerald | `Exchange::MiaxEmeraldOptions` | `miax_emerald_options` | Partial |
| Equity options | MIAX Options | `Exchange::MiaxOptions` | `miax_options` | Partial |
| Equity options | MIAX Pearl | `Exchange::MiaxPearlOptions` | `miax_pearl_options` | Partial |
| Equity options | MIAX Sapphire | `Exchange::MiaxSapphireOptions` | `miax_sapphire_options` | Partial |
| Equity options | Nasdaq GEMX | `Exchange::NasdaqGemx` | `nasdaq_gemx` | Partial |
| Equity options | Nasdaq ISE | `Exchange::NasdaqIse` | `nasdaq_ise` | Partial |
| Equity options | Nasdaq MRX | `Exchange::NasdaqMrx` | `nasdaq_mrx` | Partial |
| Equity options | Nasdaq Options | `Exchange::NasdaqNom` | `nasdaq_nom` | Partial |
| Equity options | Nasdaq PHLX | `Exchange::NasdaqPhlx` | `nasdaq_phlx` | Partial |
| Equity options | Nasdaq Texas Options | `Exchange::NasdaqBxOptions` | `nasdaq_bx_options` | Partial |
| Equity options | NYSE American Options | `Exchange::NyseAmericanOptions` | `nyse_american_options` | Partial |
| Equity options | NYSE Arca Options | `Exchange::NyseArcaOptions` | `nyse_arca_options` | Partial |
| Futures and options on futures | CBOT | `Exchange::Cbot` | `cbot` | Partial |
| Futures and options on futures | CFE | `Exchange::Cfe` | `cfe` | Primary |
| Futures and options on futures | CME | `Exchange::Cme` | `cme` | Partial |
| Futures and options on futures | COMEX | `Exchange::Comex` | `comex` | Partial |
| Futures and options on futures | EEX | `Exchange::Eex` | `eex` | Primary |
| Futures and options on futures | Eurex | `Exchange::Eurex` | `eurex` | Primary |
| Futures and options on futures | ICE Abu Dhabi | `Exchange::IceAbuDhabi` | `ice_abu_dhabi` | Primary |
| Futures and options on futures | ICE Canada | `Exchange::IceCanada` | `ice_canada` | Primary |
| Futures and options on futures | ICE Endex | `Exchange::IceEndex` | `ice_endex` | Primary |
| Futures and options on futures | ICE Europe Commodities | `Exchange::IceEuropeCommodities` | `ice_europe_commodities` | Primary |
| Futures and options on futures | ICE Europe Financials | `Exchange::IceEuropeFinancials` | `ice_europe_financials` | Primary |
| Futures and options on futures | ICE Futures US | `Exchange::Iceus` | `iceus` | Primary |
| Futures and options on futures | NYMEX | `Exchange::Nymex` | `nymex` | Partial |

## Identity decisions

- **Nasdaq Texas is `NasdaqBx`.** It is the renamed Nasdaq BX exchange. The
  stable `nasdaq_bx` persisted identity cannot be renamed, and adding a second
  variant would incorrectly split one venue's history. Nasdaq Texas Options
  follows the same rule through `NasdaqBxOptions`.
- **NYSE Texas is distinct.** `NyseTexas` is the continuing CHX / NYSE Chicago
  exchange and is unrelated to Nasdaq Texas.
- **Vendor labels are segment-specific where needed.** `MemxEq` and
  `MiaxPearlEq` keep equities distinct from their options venues; Cboe Options
  maps to the C1 identity, and Nasdaq Options maps to NOM.
- **Futures venue defaults are not product-wide clocks.** A venue mapping proves
  the exchange identity exists; callers still select the exact
  [`MarketHoursKey`](../../src/calendar/futures_profile.rs) for products outside
  the venue default. For example, `Iceus` defaults to NYSE FANG+, not every ICE
  Futures U.S. contract.

