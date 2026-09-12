<!-- SPDX-License-Identifier: MIT-0 -->

# `ice_endex` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`ice_endex.rs`](../../src/calendar/schedules/futures/international/ice_endex.rs)
- **Source sets:** [`ICE-ENDEX`](../schedules/sources.md#ice-endex)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Post-combination Dutch TTF Natural Gas Futures: closed before the sourced 2013-10-07 transfer. The immediately preceding WebICE table gives 07:45/08:00/18:00; Circular 13/107 continues the equivalent contract on the same platform, and the first Endex artifacts retain its open/pre-market structure. E26004's 2026 extension and recurring US/EU DST grids are exact.

## Revision rows

None. ice_endex.rs encodes its dated cutovers as constants rather than revisions! tuples, so it has no dated revision row.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://web.archive.org/web/20130831104114id_/https://www.theice.com/productguide/Search.shtml?tradingHours=> — ICE's 2013-08-31 WebICE hours table, showing the transferred contract's 07:45 pre-open / 08:00–18:00 CET grid immediately before the handoff.
- <https://www.ice.com/publicdocs/circulars/10010%20attach%201%20-%20TTF%20Nat%20Gas%20Contract%20Spec.pdf> — ICE circular 10/010 attachment 1, TTF Natural Gas contract specification.
- <https://www.ice.com/publicdocs/circulars/13107.pdf> — ICE circular 13/107, moving the equivalent contract to ICE Endex on 2013-10-07 and expressly keeping it on the same ICE platform.
- <https://www.ice.com/publicdocs/circulars/13134.pdf> — ICE circular 13/134.
- <https://ir.theice.com/press/news-details/2013/IntercontinentalExchange-Completes-Trading-and-Clearing-Transition-for-ICE-Endex-Futures-Markets/default.aspx> — completion release confirming combined WebICE trading from the transfer date.
- <https://web.archive.org/web/20140215045503id_/https://www.theice.com/productguide/ProductSpec.shtml?specId=27996665> — first archived Endex product page, retaining the 08:00 open, 18:00 close and pre-market phase.
- <https://www.ice.com/publicdocs/endex/circulars/ICE-Endex-Derivatives-Rules-V21-2-201403-Appendix-B-1-Operating-Time-Schedule.pdf> — ICE Endex Derivatives Rules v21.2 (March 2014), Appendix B-1 Operating Time Schedule, keeping Pre-Opening a separate phase from Trading.
- <https://www.ice.com/publicdocs/endex/ICE_Endex_Rules.pdf> — current ICE Endex rules.
- <https://www.ice.com/publicdocs/endex/circulars/E19003_attach_2.pdf> — ICE Endex circular E19/003 attachment 2.
- <https://www.ice.com/publicdocs/endex/circulars/E21013_attach_2.pdf> — ICE Endex circular E21/013 attachment 2.
- <https://www.ice.com/publicdocs/endex/circulars/E26004.pdf> — ICE Endex circular E26/004, proving the immediately preceding grid and changing it on 2026-04-13 to a 21-hour day, with the exact US/CET daylight-mismatch variant.
- <https://www.ice.com/products/27996665/Dutch-TTF-Gas-Futures> — live Dutch TTF Gas Futures specification.

## Gaps and residual risks

- The module encodes three day-level cutovers as named constants (`TRANSFER` 2013-10-07, `EXTENSION_OPENING_DAY` 2026-04-12, `EXTENSION` 2026-04-13) plus a daylight-time reference selector rather than as `revisions!` tuples, so this file records no revision-row bullets even though the identity has dated cutovers.
- The original predecessor-venue onset of the 07:45 pre-market is outside this identity's modelled interval; the transfer selector does not claim that the phase began in 2013.
- The 2026-04-12 eve profile exists so the extension's Sunday leg does not appear before its first opening day; it is a one-day bridge, not a separate era.
