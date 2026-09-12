<!-- SPDX-License-Identifier: MIT-0 -->

# `iceeu` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`ice_europe.rs`](../../src/calendar/schedules/futures/international/ice_europe.rs)
- **Source sets:** [`ICE-DERIVATIVES`](../schedules/sources.md#ice-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Brent Crude Futures only: governing ET grid is Sunday 18:00→Monday 18:00 and Monday–Thursday 20:00→18:00, with sourced pre-open phases. The current specification and 2010 circular support the January-2010-on shape.

## Revision rows

None. ice_europe.rs holds a single static profile for this identity with no dated revision row.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://www.ice.com/products/219/Brent-Crude-Futures> — live Brent Crude Futures specification: 20:00–18:00 New York session, 19:45 pre-open, and the special Sunday 17:00 pre-open / 18:00 open.
- <https://www.ice.com/publicdocs/futures/Trading_Schedule_Temporary_Trading_Hours_for_DST.pdf> — ICE trading schedule confirming that platform maintenance and the published grid are locked to US Eastern time.
- <https://www.ice.com/publicdocs/circulars/10070.pdf> — ICE Europe circular 10070 (2010), confirming the same ET grid at the January-2010 floor.

## Gaps and residual risks

- ICE Futures Europe has no venue-wide schedule; this row is Brent Crude Futures only. Any other ICE Europe contract family requires a separately sourced profile before it can use this clock.
- The New York reference zone is used deliberately so annual UK/US daylight-time mismatch is expressed by the rule rather than by exception rows; a UK-zone restatement would need its own source.
- Brent publishes no tradeable phase outside its near-24-hour session, so `extended` is empty by design and needs no further proof.
