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

## Holidays

**Coverage:** 2026-01-01..2026-12-31 (inclusive trade dates). Tier: T1 throughout.

One table serves the `cfe` venue and the `cfe_vix` key: Cboe publishes one holiday schedule for all CFE futures, and VIX futures are the only family the crate routes to the venue, so the venue intersection is that one family's own table.

**Documents.**

- `CBOE-HOURS-USFUT-2026` — Cboe “Hours & Holidays → U.S. Futures”, heading “2026 Futures Holiday Schedule”. <https://www.cboe.com/about/hours/us-futures> (raw bytes retrieved 2026-09-12 04:55 UTC, sha256 `b1b38ba66a870c06b9169eed58329407cc0ea23903a44d96e5284f7bf97e7759`), with Cboe's own CSV export <https://www.cboe.com/us/futures/holidays/csv/> (retrieved 2026-09-12 04:17 UTC, sha256 `cc98e21e97b8b830368a6ea462b5c262704432d22b163c67cb240311b3561c89`, header `# Generated: 2026:09:11 16:16:09`) — **T1**. The holiday *set* is corroborated by CFE Regulatory Circular 26-004 of 2026-03-31 (sha256 `7291896896d349360f8ecaf3d1a67cf7e074af70ec427eb12ab9c5bc19177a21`), which states no session instants and keys nothing.

All bytes, with each artifact's URL, UTC retrieval time and sha256, are in the research store under `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027/INDEX.md` and `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027-fix/INDEX.md`; the normalised result is `holidays/cfe-eurex-ice-cde-smfe-2026-2027.json`, verified `matches: true` with zero discrepancies in its round-2 adversarial verdict.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `New Year's Day,2026-01-01,None,5:00 PM (Thu) to 8:30 AM (Fri)` | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-01-01; the only block printed is the Thursday-evening leg, which belongs to trade date 2026-01-02 |
| 2026-01-19 | early close | `Martin Luther King Jr. Day,2026-01-19,None,5:00 PM (Sun) to 10:30 AM (Mon) and 5:00 PM (Mon) to 8:30 AM (Tue)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-01-19; the Sunday-evening leg is trade date 2026-01-19's own and stops at 10:30, and the Monday-evening leg is trade date 2026-01-20's |
| 2026-02-16 | early close | `Presidents' Day,2026-02-16,None,5:00 PM (Sun) to 10:30 AM (Mon) and 5:00 PM (Mon) to 8:30 AM (Tue)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-02-16; same two-leg shape as MLK Day |
| 2026-04-03 | early close | `Good Friday,2026-04-03,None,5:00 PM (Thu) to 8:30 AM (Fri)` — 08:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-04-03; the Thursday-evening leg is trade date 2026-04-03's own and stops at its normal regular open |
| 2026-05-25 | early close | `Memorial Day,2026-05-25,None,5:00 PM (Sun) to 10:30 AM (Mon) and 5:00 PM (Mon) to 8:30 AM (Tue)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-05-25 |
| 2026-06-19 | early close | `Juneteenth Holiday,2026-06-19,None,5:00 PM (Thu) to 10:30 AM (Fri)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-06-19; a Friday, so there is no evening leg to reassign |
| 2026-07-03 | early close | `Independence Day Observed,2026-07-03,None,5:00 PM (Thu) to 10:30 AM (Fri)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-07-03 |
| 2026-09-07 | early close | `Labor Day,2026-09-07,None,5:00 PM (Sun) to 10:30 AM (Mon) and 5:00 PM (Mon) to 8:30 AM (Tue)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-09-07 |
| 2026-11-26 | early close | `Thanksgiving Day,2026-11-26,None,5:00 PM (Wed) to 10:30 AM (Thu) and 5:00 PM (Thu) to 8:30 AM (Fri)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-11-26; the Thursday-evening leg belongs to trade date 2026-11-27 |
| 2026-11-27 | early close | `Thanksgiving Early Close,2026-11-27,08:30:00 - 12:15:00` — 12:15 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-11-27; the first open is the normal Thursday 17:00 CT, so only the close moves |
| 2026-12-24 | early close | `Christmas Early Close,2026-12-24,08:30:00 - 12:15:00,5:00 PM (Wed) to 8:30 AM (Thu)` — 12:15 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-12-24; no Thursday-evening leg is printed because trade date 2026-12-25 is closed |
| 2026-12-25 | closed | `Christmas Day,2026-12-25,None,None` | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-12-25; both columns empty |

**Gaps, 2026:** Cboe has published no 2027 CFE holiday schedule; the page carries only the “2026 Futures Holiday Schedule”, the CSV endpoint ignores a `?year=` parameter, and the 2026 regulatory-circular index lists no 2027 holiday circular. Coverage therefore ends 2026-12-31, and the crate answers the pure normal week for 2027. Closed by the Cboe 2027 futures holiday schedule when Cboe publishes it. No row in this window changes intraday phase topology, and no row is a late open.

**Interpretive steps, 2026:** Cboe prints two cells per holiday, `Regular Trading Hours` and `Extended Trading Hours`. On nine of these dates the regular cell reads `None` while the extended cell still names a block. On the crate's trade-date key that is an **early close** of the leg which opened at 17:00 CT the previous evening, not a closure: the block printed as “5:00 PM (Sun) to 10:30 AM (Mon)” *is* trade date Monday's own session, and the second block “5:00 PM (Mon) to 8:30 AM (Tue)” belongs to the next trade date and needs no row. Two dates are different. 2026-01-01 is `closed` because the only block printed is the Thursday-evening leg of trade date 2026-01-02, so no session at all belongs to 2026-01-01; and 2026-12-25 prints `None,None`. This reading answers the retrieval's own advisory on 2026-01-01 and is fenced in `tests/futures_family_boundaries/holidays_cfe_vix.rs`.

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
