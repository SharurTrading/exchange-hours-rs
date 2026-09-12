<!-- SPDX-License-Identifier: MIT-0 -->

# `ice_us` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`ice_us.rs`](../../src/calendar/schedules/futures/us/ice_us.rs)
- **Source sets:** [`ICE-DERIVATIVES`](../schedules/sources.md#ice-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

NYSE FANG+ Index Futures only, including the sourced 2017-11-07 launch-eve opening through the dated key API.

## Revision rows

The key shares one timeline with the `iceus` exchange row, so its days are the
same two:

- 2017-11-07 — T1 — ICE FANG+ launch notice 20170926 — the launch-eve profile: a Tuesday 19:30–20:00 ET Pre-Open and the 20:00 matching start, and nothing earlier that day.
- 2017-11-08 — T1 — ICE FANG+ launch notice 20170926 — the full grid for trade date 2017-11-08: Sunday 18:00 open, Monday–Thursday 20:00 opens, matching through 18:00 the next day, with the 30-minute Pre-Open queues.

Everything below the first row is `CLOSED_NEW_YORK`, a sourced closure, so the
row's horizon is `—`.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

The row rests on the same
`ICE-DERIVATIVES` documents as the `iceus` exchange row; the full annotated
list is in [`iceus.md`](iceus.md#sources). The documents that key the rows
above are:

- <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_FANG%2BFuture_20170926.pdf> — the ICE Futures U.S. FANG+ launch notice of 2017-09-26 — T1.
- <https://www.ice.com/products/66380320/NYSE-FANG-Index-Future> — the current NYSE FANG+ Index Future product page, which publishes the 17:30 Sunday and 19:30 weekday queue starts — T1.
- <https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf> — the ICE Futures U.S. *Regular Trading Hours* master table, June-2026 edition — T1.
- <https://www.ice.com/trading-hours> — ICE trading hours, the current-schedule monitoring entry point — T1.

## Gaps and residual risks

- **No dated-history gap.** The pre-launch era is a sourced closure and both
  rows rest on the operator's own launch notice.
- **Scope.** The key names the NYSE FANG+ Index Futures family only. It is not
  an ICE Futures U.S. venue clock, and it must not be reused for the six ICE
  Futures U.S. soft-commodity and index keys, which carry their own dated
  histories and their own January-2010 to August-2011 baseline gap.
- **Queue classification.** The Sunday 17:30–18:00 and Monday–Thursday
  19:30–20:00 phases are `order_entry`; the `extended` slice is empty because
  FANG+ publishes no tradeable phase outside its executable session. See
  [`iceus.md`](iceus.md#gaps-and-residual-risks).
- **Dormant identity (LAW-SERVICE-TIERS).** A venue namespace reaches an
  `Exchange`, never a key, so no SharurPlatform adapter can reach this row and
  no family-map root points at it. It is reviewed on demand, and any future gap
  is recorded here rather than opened as an issue.

> Shared module. The narrative for
> [`ice_us.rs`](../../src/calendar/schedules/futures/us/ice_us.rs)
> lives in [`iceus`](iceus.md#module-narrative-moved-from-srccalendarschedulesfuturesusice_usrs-on-2026-09-12-utc).
