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

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.ice.com/products/219/Brent-Crude-Futures> — live Brent Crude Futures specification: 20:00–18:00 New York session, 19:45 pre-open, and the special Sunday 17:00 pre-open / 18:00 open.
- <https://www.ice.com/publicdocs/futures/Trading_Schedule_Temporary_Trading_Hours_for_DST.pdf> — ICE trading schedule confirming that the published grid is locked to US Eastern time.
- <https://www.ice.com/publicdocs/circulars/10070.pdf> — ICE Europe circular 10070 (2010), confirming the same ET grid at the January-2010 floor.

## Gaps and residual risks

- The commodities segment is modelled by one contract, Brent Crude Futures. Other ICE Europe commodity families are not covered by this row.
- The row deliberately duplicates the `iceeu` profile; if either is ever re-sourced, both must be reviewed together.
- **Dormant note, 2026-11-18 UTC: the operator's 2027 schedule is published and captured.** The live "IFEU Trading Schedule: Oil, Utilities and Agricultural" edition (header "November 2026") now carries a "2027 Trading Schedule" section with a `Year: 2027` holiday table (2027-01-01 New Year's Day Closed, MLK Day 18 January marked `A`, and the year's rows following). Captured at T1 from <https://www.ice.com/publicdocs/Trading_Schedule.pdf>; artifact `holidays/raw/ice-dormant-2027-schedules-2026-11-18/20261118090229_Trading_Schedule.pdf` (sha256 `518da0a932f71d81bd1f04b09ecd6598731531f90d587382dbcf3e3b23e0ac1e`) with its `pdftotext` extraction and the wave's `INDEX.md` in the research store. No runtime row moves: this identity is dormant, so the 2027 table is worked up into data only when a consumer reaches it (LAW-SERVICE-TIERS); this capture is the dormant-tier obligation, not a completeness claim.

## Shared module

> Shared module. [`ice_europe.rs`](../../src/calendar/schedules/futures/international/ice_europe.rs) also carries [`iceeu`](iceeu.md) and [`ice_europe_financials`](ice_europe_financials.md). The single copy of this module's narrative belongs in [`iceeu`](iceeu.md); the module still holds that narrative in source, and it moves in the migration that empties the fence's narrative-debt list.
