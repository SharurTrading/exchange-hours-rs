<!-- SPDX-License-Identifier: MIT-0 -->

# `nasdaq_bx` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`equities.rs`](../../src/calendar/schedules/equities/us/equities.rs)<br>[`history.rs`](../../src/calendar/schedules/equities/us/history.rs)
- **Source sets:** [`US-NASDAQ-EQUITIES`](../schedules/sources.md#us-nasdaq-equities)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Stable wire identity for Nasdaq Texas; January-2010 08:00–19:00 hours and the exact 2011-04-18 move to the current 07:00–19:00 grid are primary-sourced. **Systems in scope (2026-09-02):** the BX/Nasdaq Texas equities matching system (07:00–19:00) is the envelope; BX Options is `nasdaq_bx_options`. This SRO operates no other facility. No discrepancy.

## Revision rows

- 2011-04-18 — T1 — Nasdaq Equity Trader Alert 2011-20 — system-hours open moves from 08:00 to 07:00 ET; the 19:00 close is unchanged.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://listingcenter.nasdaq.com/rulebook/nasdaqtx/rules/Nasdaq%20Texas%20Equity%201> — Nasdaq Texas Equity 1, the current rulebook for the renamed venue.
- <https://www.nasdaqtrader.com/content/technicalsupport/nasdaq_sys_hours.pdf> — *Nasdaq Systems — Hours of Operation*, the 07:00–19:00 platform hours.
- <https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2009-003> — the official launch alert, which supplies the 08:00 predecessor open and the unchanged 19:00 close.
- <https://www.nasdaqtrader.com/content/newsalerts/2009/bx_infocirculars/QQQQ_01152009.pdf> — the 2009 BX information circular that fixes the 08:00–19:00 January-2010 baseline.
- <https://www.sec.gov/rules/sro/bx/2011/34-64105.pdf> — SR-BX-2011-016, the 08:00 to 07:00 system-hours change.
- <https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2011-20> — Equity Trader Alert 2011-20, which makes the production date Monday 2011-04-18.
- <https://www.nasdaqtrader.com/TraderNews.aspx?id=dtn2014-08> — a March-2014 Nasdaq data notice independently confirming the 07:00 platform open.
- <https://nasdaqtrader.com/TraderNews.aspx?id=ETA2026-8> — the BX-to-Nasdaq-Texas conversion alert.

## Gaps and residual risks

- **None open.** The January-2010 baseline is sourced at the floor by the 2009
  circular and the 2011 move is dated by the operator's own implementation
  alert, so nothing above the floor is carried.
- **Identity risk, not a schedule risk.** The operator renamed this exchange
  Nasdaq Texas; the wire name stays `nasdaq_bx` because a rename would break
  persisted consumer data, and this venue is unrelated to the `nyse_texas`
  identity in `nyse.rs`.
- **System coverage (2026-09-02).** No discrepancy. BX Options is
  `nasdaq_bx_options`; this SRO operates no other facility.

> Shared module. The narrative for
> [`equities.rs`](../../src/calendar/schedules/equities/us/equities.rs) and [`history.rs`](../../src/calendar/schedules/equities/us/history.rs)
> lives in [`nasdaq`](nasdaq.md#module-narrative-moved-from-srccalendarschedulesequitiesusequitiesrs-on-2026-09-12-utc) and [`nasdaq`](nasdaq.md#module-narrative-moved-from-srccalendarschedulesequitiesushistoryrs-on-2026-09-12-utc).
> Sibling identities: [`nasdaq_psx`](nasdaq_psx.md), [`memx_eq`](memx_eq.md), [`miax_pearl_eq`](miax_pearl_eq.md).
