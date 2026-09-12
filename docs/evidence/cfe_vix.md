<!-- SPDX-License-Identifier: MIT-0 -->

# `cfe_vix` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`cfe.rs`](../../src/calendar/schedules/futures/us/cfe.rs)
- **Source sets:** [`US-CFE`](../schedules/sources.md#us-cfe)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Reuses the complete CFE VIX history, including exact old-system pre-open onsets and the sourced 2018-02-25 three-second / 2018-08-12 six-second conservative queue edges.

## Revision rows

The key shares one timeline with the `cfe` exchange row, so its days are the
same eight:

- 2010-12-10 — T1 — Cboe SR-CFE-2010-013 — a 07:20–08:30 CT extended session is added ahead of the unchanged 08:30–15:15 RTH.
- 2011-09-26 — T1 — Cboe SR-CFE-2011-019 — the extended session start moves from 07:20 to 07:00 CT.
- 2013-10-28 — T1 — Cboe IC13-041 — the Monday–Thursday 15:30–16:15 CT session opens, with its 15:29–15:30 pre-open queue.
- 2013-11-04 — T1 — Cboe IC13-041 — the morning open moves from 07:00 to 02:00 CT.
- 2014-06-22 — T1 — Cboe RG-CFE-2014-020 — near-24-hour VX trading begins, adding the Sunday 16:15–17:00 CT pre-open.
- 2018-02-25 — T1 — Cboe RG-CFE-2018-005 — system migration: a 15:15–15:30 CT queue, 15:30–16:00 ETH, and the 16:00:03 / 16:45:03 CT queue edges.
- 2018-08-12 — T1 — Cboe C2018071603 — TAS queue commencement widens to six seconds, so the conservative edges become 16:00:06 and 16:45:06 CT.
- 2021-12-06 — T1 — Cboe C2021102603 — the current grid: RTH 08:30–15:00 CT, ETH 15:00–16:00 and 17:00–08:30, queues at 16:00:06 and 16:45:06 CT.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

The row rests on the same `US-CFE`
documents as the `cfe` exchange row; the full annotated list is in
[`cfe.md`](cfe.md#sources). The documents that key the rows above are:

- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2010/SR-CFE-2010-013.pdf> — Cboe SR-CFE-2010-013 — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2011/SR-CFE-2011-019.pdf> — Cboe SR-CFE-2011-019 — T1.
- <https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2013-041.pdf> — Cboe IC13-041 — T1.
- <https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2014-020.pdf> — Cboe RG-CFE-2014-020 — T1.
- <https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2018-005.pdf> — Cboe RG-CFE-2018-005 — T1.
- <https://cdn.cboe.com/resources/release_notes/2018/Change-to-CFE-Pre-Open-Time-for-TAS-Contracts-and-Order-Submission-Commencement-Times.pdf> — Cboe C2018071603 — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/pending/2021/21-028-VX-VXM-and-AMERIBOR-Trading-Hours.pdf> — rule certification CFE-2021-028 behind Cboe notice C2021102603 — T1.
- <https://www.cboe.com/about/hours/us-futures> — CFE trading hours, the current-schedule monitoring entry point — T1.

## Gaps and residual risks

- **No dated-history gap.** The key serves the same primary-supported
  normal-week history as the exchange row, from the January-2010 floor.
- **Randomized queue starts are modelled conservatively.** The 2018-02-25 and
  2018-08-12 rows publish the latest acceptance edge rather than a per-contract
  instant; see [`cfe.md`](cfe.md#gaps-and-residual-risks).
- **Dormant identity (LAW-SERVICE-TIERS).** No SharurPlatform family-map root
  points at this key, so it is reviewed on demand. Its closing condition for any
  future gap is recorded here rather than as an issue.
- **Scope.** The key is VIX futures, not a venue-wide CFE clock. A consumer that
  maps another CFE product to it would be using the wrong family.

> Shared module. The narrative for
> [`cfe.rs`](../../src/calendar/schedules/futures/us/cfe.rs)
> lives in [`cfe`](cfe.md#module-narrative-moved-from-srccalendarschedulesfuturesuscfers-on-2026-09-12-utc).
