<!-- SPDX-License-Identifier: MIT-0 -->

# Unsupported futures families in 1.0

`MarketHoursKey` identifies an exact product-family schedule, not an
approximation selected from the same venue. The families below are deferred
from 1.0 and targeted for primary-source evidence work in 1.1. Until a sourced
profile is shipped, both `MarketHoursKey::from_str` and Serde reject these
prospective identifiers:

There are nine deferred families: CME Nikkei 225 Dollar plus six ICE Futures
U.S. families, Eurex fixed income, and SGX equity indexes. The strings below
are prospective names, not reserved wire identities; 1.0 tests require them to
remain rejected until the corresponding sourced profile is actually added.

| Prospective identifier | Unsupported family |
|---|---|
| `globex_nikkei_225_dollar` | CME Nikkei 225 Dollar futures (`NKD`) |
| `ice_us_dollar_index` | ICE Futures U.S. U.S. Dollar Index futures |
| `ice_us_sugar` | ICE Futures U.S. Sugar No. 11 futures |
| `ice_us_coffee` | ICE Futures U.S. Coffee “C” futures |
| `ice_us_cocoa` | ICE Futures U.S. Cocoa futures |
| `ice_us_cotton` | ICE Futures U.S. Cotton No. 2 futures |
| `ice_us_orange_juice` | ICE Futures U.S. FCOJ-A futures |
| `eurex_fixed_income` | Eurex fixed-income futures |
| `sgx_equity_index` | SGX equity-index futures |

Consumers must report an unsupported-family error for these products. Do not
substitute `IceUs`, `Eurex`, or `Sgx`: those existing keys deliberately model
NYSE FANG+ Index futures, FESX/FDAX/FDXM index futures, and Three-Month SORA
futures respectively, and their session grids are not venue-wide defaults.
Likewise, do not substitute `GlobexEquityIndex` for NKD. The crate performs no
symbol-to-family mapping; that refusal belongs in the caller's instrument
catalog.

Adding one of these names later is an explicit new supported identity. It
requires a primary-sourced normal-week profile and history, date-aware routing,
wire-format and boundary tests, and the usual verification-ledger update.
