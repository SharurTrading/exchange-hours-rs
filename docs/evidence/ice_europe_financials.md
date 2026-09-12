<!-- SPDX-License-Identifier: MIT-0 -->

# `ice_europe_financials` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`ice_europe.rs`](../../src/calendar/schedules/futures/international/ice_europe.rs)
- **Source sets:** [`ICE-DERIVATIVES`](../schedules/sources.md#ice-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

FTSE 100 Index Futures: closed before the sourced 2014-11-17 ICE migration, with sourced 2015-02-16 and 2015-10-01 extensions to the current 00:45 pre-open / 01:00–21:00 London grid.

## Revision rows

- 2014-11-17 — T1 — ICE Europe circular 14/146 — FTSE 100 Index Futures migrate to ICE Futures Europe; pre-open 06:03, trading 08:00–21:00 London.
- 2015-02-16 — T1 — ICE Europe circular 15/016 — open moves to 07:00, with the same 06:03 pre-open running to it.
- 2015-10-01 — T1 — ICE Europe circular 15/169 — current grid: 00:45 pre-open, 01:00–21:00 trading.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.ice.com/products/38716764/FTSE-100-INDEX-> — live ICE specification: 00:45 pre-open, 01:00–21:00 trading.
- <https://www.ice.com/publicdocs/circulars/14146.pdf> — ICE Europe circular 14/146, the 2014-11-17 migration and first trade date.
- <https://www.ice.com/publicdocs/circulars/15016.pdf> — ICE Europe circular 15/016. Its two FTSE tables read "Pre-open 06:03 | Open 08:00 | Close 21:00" before 16 February 2015 and "Pre-open 06:03 | Open 07:00 | Close 21:00" after it, which is why the 06:03 window is classified order entry rather than a session in which anything prints.
- <https://www.ice.com/publicdocs/circulars/15169.pdf> — ICE Europe circular 15/169, the 2015-10-01 extension to the current 01:00 open.

## Gaps and residual risks

- The row is scoped to FTSE 100 Index Futures (Z) only; other ICE Europe financial contracts are not covered.
- FTSE publishes no tradeable phase outside its executable session, so `extended` stays empty in every era.
- The pre-2014 FTSE 100 contract traded on a different venue identity and is deliberately out of scope: this row is the ICE Futures Europe product, not an alias for its predecessor.

## Shared module

> Shared module. [`ice_europe.rs`](../../src/calendar/schedules/futures/international/ice_europe.rs) also carries [`iceeu`](iceeu.md) and [`ice_europe_commodities`](ice_europe_commodities.md). The single copy of this module's narrative belongs in [`iceeu`](iceeu.md); the module still holds that narrative in source, and it moves in the migration that empties the fence's narrative-debt list.
