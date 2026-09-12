<!-- SPDX-License-Identifier: MIT-0 -->

# `eurex` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`europe.rs`](../../src/calendar/schedules/futures/international/europe.rs)
- **Source sets:** [`EU-EUREX`](../schedules/sources.md#eu-eurex)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

FESX/FDAX/FDXM benchmark-index futures; the dated key API reuses the sourced January-2010 baseline, 2018 extension, and recurring CET/CEST phase selection.

## Revision rows

None. `europe.rs` carries no `revisions!` block for this identity; its 2018-12-10 Asian-hours cutover and the seasonal CET/CEST choice are encoded as comparisons inside `eurex_profile_at` instead.

The key selects through the same `eurex_profile_at` function as the `eurex`
exchange row: a date comparison against the 2018-12-10 Asian-hours cutover,
then a UTC-offset comparison that picks the CET or CEST phase table. Eurex
circular 088/2018 states the cutover day unconditionally at T1, so the date is
sourced even though it is not carried as a tuple.

## Dated selectors

Day-level boundaries this identity's `profile_at` selects on directly, outside
any `revisions!` block. They are invisible to the module-declaration fences, so
they are recorded here in revision-row grammar and checked against
`HISTORICAL_CUTOVERS` / `HISTORICAL_INSTANT_CUTOVERS` in
`tests/contract/session_invariants/historical_expectations.rs`.

- 2018-12-10 — T1 — Eurex circular 088/2018 (`EUREX_ASIAN_HOURS`) — the Asian-hours open. The key selects through the same `eurex_profile_at` function as the `eurex` exchange row, so the boundary is shared; the seasonal CET/CEST table is a UTC-offset comparison beside it and asserts no day.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

The row rests on the same `EU-EUREX`
documents as the `eurex` exchange row; the full annotated list is in
[`eurex.md`](eurex.md#sources). The documents that key the modelled states are:

- <https://www.eurex.com/resource/blob/296888/978b08fe3a240a0b4a8fb62a2647a197/data/cs_history_26102009_en.pdf.pdf> — the official 2009 archived contract specifications, pinning the grid in force at the January-2010 floor — T1.
- <https://www.eurex.com/resource/blob/1412768/e61a2c41d65ad165af7909002223b943/data/er18088e.pdf> — Eurex circular 088/2018 and its Annex C, effective 2018-12-10 — T1.
- <https://www.eurex.com/resource/blob/1448250/29a4179e4d28742af5d0ee85f9af89f8/data/Eurex%20Asian%20Trading%20Hours_Nov%202018.pdf> — the Asian-hours launch phase diagram, which splits pre-trading from the opening auction — T1.
- <https://www.eurex.com/resource/blob/2824010/3b94b95cdf5f31cc635294659a5e9786/data/2026_05_04_eurex_d_kontraktspezifikationen_annexe_en.pdf> — the current contract-specification Annex C — T1.
- <https://www.eurex.com/ex-en/trade/trading-hours> — Eurex trading hours, the current-schedule monitoring entry point — T1.

## Gaps and residual risks

- **No dated-history gap.** The 2009 archived specification predates the
  January-2010 floor, so the baseline is sourced through the floor and the
  horizon is the floor itself.
- **The key is seasonal, so it must be queried with an instant.** The CET and
  CEST phase tables differ by an hour at the open, and the Asian-hours open is a
  fixed 00:00 UTC instant rather than a fixed local time. A detached fixed
  snapshot carries whichever table it was built from and has no seasonal
  behaviour of its own.
- **Scope.** The key names the FESX/FDAX/FDXM benchmark-index family, not a
  venue-wide Eurex clock. Eurex fixed-income futures have their own key and
  their own dated revisions.
- **Dormant identity (LAW-SERVICE-TIERS).** A venue namespace reaches an
  `Exchange`, never a key, so no SharurPlatform adapter can reach this row and
  no family-map root points at it. It is reviewed on demand, and any future gap
  is recorded here rather than opened as an issue.

> Shared module. The narrative for
> [`europe.rs`](../../src/calendar/schedules/futures/international/europe.rs)
> lives in [`eurex`](eurex.md#module-narrative-moved-from-srccalendarschedulesfuturesinternationaleuropers-on-2026-09-12-utc).
> Sibling identity: [`eex`](eex.md).
