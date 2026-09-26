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

**Coverage:** 2025-01-01..2026-12-31 (inclusive trade dates). Tier: T1 throughout.

One table serves the `cfe` venue and the `cfe_vix` key: Cboe publishes one holiday schedule for all CFE futures, and VIX futures are the only family the crate routes to the venue, so the venue intersection is that one family's own table.

The key and the venue resolve the same `holidays/cfe.rs` table, so every row, quotation, URL, retrieval date and interpretive step below is also the `cfe` row's; [`cfe.md`](cfe.md) carries the fuller narrative, including the Good Friday reading and the mourning-day replacement. The two files are kept in step by the fence that checks each shipped row against the evidence file the module declares.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `TUESDAY DECEMBER 31, 2024` `RTH Close 3:00 PM` / `ETH Close 4:00 PM`; `WEDNESDAY JANUARY 1, 2025` both cells empty; `THURSDAY JANUARY 2, 2025` `ETH Start 5:00 PM` — `Trade Date Tuesday, December 31, 2024 … Thursday, January 2, 2025` | `CBOE-SU-2025-NEW-YEAR` | T1 | Cboe event date 2025-01-01; no cell is printed under the holiday column and the Trade Date row skips it |
| 2025-01-09 | replacement blocks | `5:00 p.m. (Wednesday, January 8, 2025) to 8:30 a.m. (Extended Trading Hours) for VX, VXM, VXT, and VXMT` / `No Regular Trading Hours for VX, VXM, VXT, and VXMT` | `CBOE-SU-2025-MOURNING` | T1 | Cboe trade date named verbatim — “for trade date Thursday, January 9, 2025”; one extended block opening 17:00 CT on 2025-01-08, so the row is `extended(-1, 17:00, 08:30)` and nothing else |
| 2025-01-20 | early close | `SUNDAY JANUARY 19, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM`; `MONDAY JANUARY 20, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 3:00 PM`, `ETH Close 4:00 PM` — 10:30 CT | `CBOE-SU-2025-MLK` | T1 | Cboe event date 2025-01-20; the Sunday-evening leg is trade date 2025-01-20's own and stops at 10:30 |
| 2025-02-17 | early close | `SUNDAY FEBRUARY 16, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM`; `MONDAY FEBRUARY 17, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 3:00 PM`, `ETH Close 4:00 PM` — 10:30 CT | `CBOE-SU-2025-PRESIDENTS` | T1 | Cboe event date 2025-02-17; same two-leg shape as MLK Day |
| 2025-04-18 | closed | `THURSDAY APRIL 17, 2025` `RTH Close 3:00 PM`, `ETH Close 4:00 PM`; `FRIDAY APRIL 18, 2025` **no cell at all**; `SUNDAY APRIL 20, 2025` **no cell at all**; `MONDAY APRIL 21, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM` — `Trade Date Thursday, April 17, 2025 … Monday, April 21, 2025` | `CBOE-SU-2025-GOOD-FRIDAY` | T1 | Cboe event date 2025-04-18; the notice prints no Friday close and no Friday trade date, so the day is deleted rather than clipped at the regular open |
| 2025-05-26 | early close | `SUNDAY MAY 25, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM`; `MONDAY MAY 26, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 3:00 PM`, `ETH Close 4:00 PM` — 10:30 CT | `CBOE-SU-2025-MEMORIAL` | T1 | Cboe event date 2025-05-26 |
| 2025-06-19 | early close | `WEDNESDAY JUNE 18, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM`; `THURSDAY JUNE 19, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 3:00 PM`, `ETH Close 4:00 PM` — 10:30 CT | `CBOE-SU-2025-JUNETEENTH` | T1 | Cboe event date 2025-06-19; a Thursday, so the Wednesday-evening leg is its own |
| 2025-07-03 | early close | `WEDNESDAY JULY 2, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 12:15 PM`; `THURSDAY JULY 3, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM` — 10:30 CT | `CBOE-SU-2025-INDEPENDENCE` | T1 | Cboe event date 2025-07-04 observed Friday, so the shortened day lands on the Thursday trade date that opened 2025-07-02 at 17:00 CT |
| 2025-07-04 | closed | `FRIDAY JULY 4, 2025` **no cell at all**; `SUNDAY JULY 6, 2025` `ETH Start 5:00 PM`; `MONDAY JULY 7, 2025` `RTH Start 8:30 AM` — `Trade Date Thursday, July 3, 2025 … Monday, July 7, 2025` | `CBOE-SU-2025-INDEPENDENCE` | T1 | Cboe event date 2025-07-04; the holiday column prints no session and the Trade Date row skips it |
| 2025-09-01 | early close | `SUNDAY AUGUST 31, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM`; `MONDAY SEPTEMBER 1, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 3:00 PM`, `ETH Close 4:00 PM` — 10:30 CT | `CBOE-SU-2025-LABOR` | T1 | Cboe event date 2025-09-01 |
| 2025-11-27 | early close | `WEDNESDAY NOVEMBER 26, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM`; `THURSDAY NOVEMBER 27, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 12:15 PM` — 10:30 CT | `CBOE-SU-2025-THANKSGIVING` | T1 | Cboe event date 2025-11-27; the Wednesday-evening leg is its own and stops at 10:30, and the Thursday-evening leg belongs to trade date 2025-11-28 |
| 2025-11-28 | early close | `FRIDAY NOVEMBER 28, 2025` `RTH Start 8:30 AM`, `RTH Close 12:15 PM`; `Trade Date … Friday, November 28, 2025` — 12:15 CT | `CBOE-SU-2025-THANKSGIVING` | T1 | Cboe trade date named verbatim; the first open is the normal Thursday 17:00 CT, so only the close moves |
| 2025-12-24 | early close | `WEDNESDAY DECEMBER 24, 2025` `RTH Start 8:30 AM`, `RTH Close 12:15 PM`; `Trade Date Wednesday, December 24, 2025 … Friday, December 26, 2025` — 12:15 CT | `CBOE-SU-2025-CHRISTMAS` | T1 | Cboe trade date named verbatim; no ETH close is printed for that column |
| 2025-12-25 | closed | `THURSDAY DECEMBER 25, 2025` `ETH Start`, `ETH Close`, `RTH Start`, `RTH Close` all empty; `FRIDAY DECEMBER 26, 2025` `ETH Start 5:00 PM` | `CBOE-SU-2025-CHRISTMAS` | T1 | Cboe event date 2025-12-25; every cell under the holiday column is empty and the Trade Date row skips it |

**Gaps, 2025:** none in this window. Every date Cboe's 2025 notices modify ships a row, and every other 2025 trade date is audited normal. No 2025 row is a late open.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `CBOE-SU-2025-NEW-YEAR` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2024/CFE-Modified-Trading-Hours-for-the-New-Year-s-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `9bbaa3bb3f8484356a756bd3bd04677450d6215ff5e96c0e514a63a4aac3e7d7` |
| `CBOE-SU-2025-MLK` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2024/CFE-Modified-Trading-Hours-for-the-Martin-Luther-King-Jr-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `6f2a45b2948628dc2e0867c94790d23cae9bce1728b5953ce81d6e0c5ed1732c` |
| `CBOE-SU-2025-MOURNING` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/Update-Cboe-to-Observe-National-Day-of-Mourning-on-Thursday-January-9-2025.pdf> | retrieved 2026-09-26 UTC | T1 | `ecabcf3de38b3eaa3a9c7b8a384082a66405401299fba8bc733e44061bbfc6ba` |
| `CBOE-SU-2025-MOURNING-BASE` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/Cboe-to-Observe-National-Day-of-Mourning-on-Thursday-January-9-2025.pdf> | retrieved 2026-09-26 UTC | T1 | `21b2158711940f8291ca1478a463edd02123bb5e4530d679d6c59a155525558e` |
| `CBOE-SU-2025-PRESIDENTS` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/CFE-Modified-Trading-Hours-for-the-Presidents-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `258c88e938471039fdd0f66f92efa11b4c22e65ce46fb848da6e16f068766ad2` |
| `CBOE-SU-2025-GOOD-FRIDAY` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/CFE-Modified-Trading-Hours-for-the-Good-Friday-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `cf1cb9146747b3db9af7add58b0706a19da49ffe32a7352bba3cd811a780f7d1` |
| `CBOE-SU-2025-MEMORIAL` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/CFE-Modified-Trading-Hours-for-the-Memorial-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `bef7ff2f0dc5c9da962ffd6a38e6026e2229df4b513f65cc9c743f285ede7970` |
| `CBOE-SU-2025-JUNETEENTH` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/CFE-Modified-Trading-Hours-for-the-Juneteenth-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `dd85b8b307ccfc441d857f121ebaa860b3779a77511feb77ef72ef2e5a9a3ad9` |
| `CBOE-SU-2025-INDEPENDENCE` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/CFE-Modified-Trading-Hours-for-the-Independence-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `6ffd33e4ca688d57a0eab247689fff3a34464a8374d5a2ae08f1d56221586c7a` |
| `CBOE-SU-2025-LABOR` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/CFE-Modified-Trading-Hours-for-the-Labor-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `d5ca2ecc6f7ee6ac5d68503b667192d51ff28aedac91770153e1a6b47c56f2c7` |
| `CBOE-SU-2025-THANKSGIVING` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/CFE-Modified-Trading-Hours-for-the-Thanksgiving-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `585ae94eb540d1ca810a8684ecd63d29e9e7969ea0ff37a74c9cc82e88acea55` |
| `CBOE-SU-2025-CHRISTMAS` | 2025-01-01 .. 2026-12-31 | <https://cdn.cboe.com/resources/schedule_update/2025/CFE-Modified-Trading-Hours-for-the-Christmas-Day-Holiday.pdf> | retrieved 2026-09-21 UTC | T1 | `f2fef1107ab5e774aec3c8c3fb0cda5354b16d3608a9e3b1752ff3dd34fbc3f5` |
| `CBOE-HOURS-USFUT-2026` | 2025-01-01 .. 2026-12-31 | <https://www.cboe.com/about/hours/us-futures> | retrieved 2026-09-12 04:55 UTC | T1 | `b1b38ba66a870c06b9169eed58329407cc0ea23903a44d96e5284f7bf97e7759` |

**Interpretive steps, 2025:** the key reads the 2025 rows exactly as `cfe.md` does. The seven Monday or Thursday holidays whose regular cell is empty are early closes of the leg that opened at 17:00 CT the previous evening; 2025-01-01, 2025-04-18 and 2025-07-04 print no session and no trade date of their own and are closures; and 2025-01-09 is the `ReplacementBlocks` mourning day. `cfe.md` carries the quotations and the reasoning, and `tests/futures_family_boundaries/holidays_cfe_vix.rs` fences each date.

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

**Gaps, 2026:** none inside the window, and the horizon is the operator's. Cboe has published no 2027 CFE holiday schedule: verified 2026-09-26 UTC, <https://www.cboe.com/about/hours/us-futures> carries only a “2026 Futures Holiday Schedule” heading, and its CSV twin <https://www.cboe.com/us/futures/holidays/csv/> (`# Generated: 2026:09:18 17:47:39`) lists 2026 rows only. Nothing is being withheld by this crate — there is simply no 2027 schedule yet. **Closing condition:** the Cboe 2027 futures holiday schedule, at which point the window extends to 2027-12-31. Re-checked monthly per LAW-WATCH. No row in this window changes intraday phase topology, and no row is a late open.

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
