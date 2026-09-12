<!-- SPDX-License-Identifier: MIT-0 -->

# `ice_europe_commodities` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`ice_europe.rs`](../../src/calendar/schedules/futures/international/ice_europe.rs)
- **Source sets:** [`ICE-DERIVATIVES`](../schedules/sources.md#ice-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Brent Crude Futures commodities identity; intentionally owns the same fully sourced ET profile as `iceeu`.

## Revision rows

None. ice_europe.rs holds a single static profile for this identity with no dated revision row.

## Sources

- <https://www.ice.com/products/219/Brent-Crude-Futures> — live Brent Crude Futures specification: 20:00–18:00 New York session, 19:45 pre-open, and the special Sunday 17:00 pre-open / 18:00 open.
- <https://www.ice.com/publicdocs/futures/Trading_Schedule_Temporary_Trading_Hours_for_DST.pdf> — ICE trading schedule confirming that the published grid is locked to US Eastern time.
- <https://www.ice.com/publicdocs/circulars/10070.pdf> — ICE Europe circular 10070 (2010), confirming the same ET grid at the January-2010 floor.

## Gaps and residual risks

- The commodities segment is modelled by one contract, Brent Crude Futures. Other ICE Europe commodity families are not covered by this row.
- The row deliberately duplicates the `iceeu` profile; if either is ever re-sourced, both must be reviewed together.

## Shared module

> Shared module. [`ice_europe.rs`](../../src/calendar/schedules/futures/international/ice_europe.rs) also carries [`iceeu`](iceeu.md) and [`ice_europe_financials`](ice_europe_financials.md). The single copy of this module's narrative belongs in [`iceeu`](iceeu.md); the module still holds that narrative in source, and it moves in the migration that empties the fence's narrative-debt list.
