<!-- SPDX-License-Identifier: MIT-0 -->

# `cfe` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`cfe.rs`](../../src/calendar/schedules/futures/us/cfe.rs)
- **Source sets:** [`US-CFE`](../schedules/sources.md#us-cfe)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

VIX futures normal-week history is complete from January 2010. Old-system pre-opens have exact sourced onsets; randomized new-system queue starts use conservative latest edges of 16:00:03/16:45:03 from 2018-02-25 and 16:00:06/16:45:06 from 2018-08-12.

## Revision rows

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

Two artifacts key 2025 and one keys 2026, so the 2025 rows rest on per-holiday Cboe notices and the 2026 rows on Cboe's consolidated `Hours & Holidays` page and CSV. The document ids record which: `CBOE-SU-<holiday year>-<HOLIDAY>` for a per-holiday `schedule_update` notice published ahead of that holiday year, and `CBOE-HOURS-USFUT-2026` for the page. The year in the id is the **holiday year the notice governs**, not the year of the directory it is served from: Cboe files each New Year's and MLK notice under the *previous* year, so the notices governing 2025-01-01 and 2025-01-20 are served from `schedule_update/2024/` and the ones governing 2026-01-01 and 2026-01-19 from `schedule_update/2025/`.

### 2025

The 2025 block is the eleven CFE notices the operator's own `schedule-update/2025` listing names, plus the four notices governing 2025's first two holidays that the same listing files under 2024 (Cboe publishes a New Year and an MLK notice in the December before the holiday year) and the National Day of Mourning notice. Every row below was read from the notice's own PDF bytes with `pdftotext -layout` **and** `-bbox` word coordinates, because the 2025 corpus has no consolidated table: unlike 2026 there is no single CSV or page that states all of these dates, and one notice in the set has a layout the store's parser could not read at all.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `TUESDAY DECEMBER 31, 2024` `RTH Close 3:00 PM` / `ETH Close 4:00 PM`; `WEDNESDAY JANUARY 1, 2025` both cells empty; `THURSDAY JANUARY 2, 2025` `ETH Start 5:00 PM` — `Trade Date Tuesday, December 31, 2024 … Thursday, January 2, 2025` | `CBOE-SU-2025-NEW-YEAR` | T1 | Cboe event date 2025-01-01; no cell is printed under the holiday column and the Trade Date row skips it, so no session belongs to it |
| 2025-01-09 | replacement blocks | `5:00 p.m. (Wednesday, January 8, 2025) to 8:30 a.m. (Extended Trading Hours) for VX, VXM, VXT, and VXMT` / `No Regular Trading Hours for VX, VXM, VXT, and VXMT` | `CBOE-SU-2025-MOURNING` | T1 | Cboe trade date named verbatim — “for trade date Thursday, January 9, 2025”; the only session is one extended block opening 17:00 CT on 2025-01-08, so the row states `extended(-1, 17:00, 08:30)` and nothing else |
| 2025-01-20 | early close | `SUNDAY JANUARY 19, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM`; `MONDAY JANUARY 20, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 3:00 PM`, `ETH Close 4:00 PM` — 10:30 CT | `CBOE-SU-2025-MLK` | T1 | Cboe event date 2025-01-20; the Sunday-evening leg is trade date 2025-01-20's own and stops at 10:30, and the Monday-evening leg is trade date 2025-01-21's |
| 2025-02-17 | early close | `SUNDAY FEBRUARY 16, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM`; `MONDAY FEBRUARY 17, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 3:00 PM`, `ETH Close 4:00 PM` — 10:30 CT | `CBOE-SU-2025-PRESIDENTS` | T1 | Cboe event date 2025-02-17; same two-leg shape as MLK Day |
| 2025-04-18 | closed | `THURSDAY APRIL 17, 2025` `RTH Close 3:00 PM`, `ETH Close 4:00 PM`; `FRIDAY APRIL 18, 2025` **no cell at all**; `SUNDAY APRIL 20, 2025` **no cell at all**; `MONDAY APRIL 21, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM` — `Trade Date Thursday, April 17, 2025 … Monday, April 21, 2025` | `CBOE-SU-2025-GOOD-FRIDAY` | T1 | Cboe event date 2025-04-18; see the interpretive step below — the Friday and Sunday trade dates are both absent from the notice's Trade Date row |
| 2025-05-26 | early close | `SUNDAY MAY 25, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM`; `MONDAY MAY 26, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 3:00 PM`, `ETH Close 4:00 PM` — 10:30 CT | `CBOE-SU-2025-MEMORIAL` | T1 | Cboe event date 2025-05-26 |
| 2025-06-19 | early close | `WEDNESDAY JUNE 18, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM`; `THURSDAY JUNE 19, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 3:00 PM`, `ETH Close 4:00 PM` — 10:30 CT | `CBOE-SU-2025-JUNETEENTH` | T1 | Cboe event date 2025-06-19; a Thursday, so the Wednesday-evening leg is its own |
| 2025-07-03 | early close | `WEDNESDAY JULY 2, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 12:15 PM`; `THURSDAY JULY 3, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM` — 10:30 CT | `CBOE-SU-2025-INDEPENDENCE` | T1 | Cboe event date 2025-07-04 observed Friday, so the shortened day lands on the Thursday trade date that opened 2025-07-02 at 17:00 CT |
| 2025-07-04 | closed | `FRIDAY JULY 4, 2025` **no cell at all**; `SUNDAY JULY 6, 2025` `ETH Start 5:00 PM`; `MONDAY JULY 7, 2025` `RTH Start 8:30 AM` — `Trade Date Thursday, July 3, 2025 … Monday, July 7, 2025` | `CBOE-SU-2025-INDEPENDENCE` | T1 | Cboe event date 2025-07-04; the holiday column prints no session and the Trade Date row skips it |
| 2025-09-01 | early close | `SUNDAY AUGUST 31, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM`; `MONDAY SEPTEMBER 1, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 3:00 PM`, `ETH Close 4:00 PM` — 10:30 CT | `CBOE-SU-2025-LABOR` | T1 | Cboe event date 2025-09-01 |
| 2025-11-27 | early close | `WEDNESDAY NOVEMBER 26, 2025` `ETH Start 5:00 PM`, `ETH Close 10:30 AM`; `THURSDAY NOVEMBER 27, 2025` `ETH Start 5:00 PM`, `RTH Start 8:30 AM`, `RTH Close 12:15 PM` — 10:30 CT | `CBOE-SU-2025-THANKSGIVING` | T1 | Cboe event date 2025-11-27; the Wednesday-evening leg is trade date 2025-11-27's own and stops at 10:30, and the Thursday-evening leg belongs to trade date 2025-11-28 |
| 2025-11-28 | early close | `FRIDAY NOVEMBER 28, 2025` `RTH Start 8:30 AM`, `RTH Close 12:15 PM`; `Trade Date … Friday, November 28, 2025` — 12:15 CT | `CBOE-SU-2025-THANKSGIVING` | T1 | Cboe trade date named verbatim; the first open is the normal Thursday 17:00 CT, so only the close moves |
| 2025-12-24 | early close | `WEDNESDAY DECEMBER 24, 2025` `RTH Start 8:30 AM`, `RTH Close 12:15 PM`; `Trade Date Wednesday, December 24, 2025 … Friday, December 26, 2025` — 12:15 CT | `CBOE-SU-2025-CHRISTMAS` | T1 | Cboe trade date named verbatim; the Wednesday column carries no ETH close, because the Thursday-evening leg would belong to the closed trade date 2025-12-25 |
| 2025-12-25 | closed | `THURSDAY DECEMBER 25, 2025` `ETH Start`, `ETH Close`, `RTH Start`, `RTH Close` all empty; `FRIDAY DECEMBER 26, 2025` `ETH Start 5:00 PM` | `CBOE-SU-2025-CHRISTMAS` | T1 | Cboe event date 2025-12-25; every cell under the holiday column is empty and the Trade Date row skips it |

**Supersession, 2025-01-09.** Two mourning artifacts are on disk. The base notice `2025__Cboe-to-Observe-National-Day-of-Mourning-on-Thursday-January-9-2025.pdf` (sha256 `21b2158711940f8291ca1478a463edd02123bb5e4530d679d6c59a155525558e`) is **superseded** by the `Update-` file the row cites (`ecabcf3d…`), which is why the id resolves to the update: the update is the artifact whose CFE TRADING SCHEDULE table states the session. Both are listed in the table below so the supersession is auditable, and both were retrieved in the same pass on 2026-09-26, so the update is not a later correction of a value this file once recorded differently.

**Two 2025 differences from the 2026 shapes.**

1. **Good Friday 2025-04-18 is a closure, where 2026-04-03 is an early close at the regular open.** Cboe's 2025 notice prints the `FRIDAY APRIL 18, 2025` column header but no cell beneath it: the Thursday column carries `RTH Close 3:00 PM` and `ETH Close 4:00 PM`, and the next printed times are Monday's `ETH Start 5:00 PM` and `RTH Start 8:30 AM`, with the `SUNDAY APRIL 20, 2025` column likewise empty. Its Trade Date row reads `Thursday, April 17, 2025 … Monday, April 21, 2025` — **2025-04-18 is not named as a trade date at all**, although the same notice's other rows name their event date explicitly. The 2026 notice of the same holiday states the opposite shape in its own bytes: `FRIDAY APRIL 3, 2026` carries `ETH Close 8:30 AM`, and the row is `None` for RTH, which is the 08:30 early close the crate ships for that date. The 2025 bytes therefore state no close for 2025-04-18, and the crate derives the 2025 row from the 2025 bytes rather than copying the 2026 shape. No 2025 notice names a trade date for that Friday, and no ETH Close appears under it at any x-position in the PDF.
2. **2025-01-09 is a trading day with no regular session.** The mourning notice states the session in prose rather than in the two-column holiday table, so the scalar vocabulary cannot state it: `Closed` would delete a session the operator kept open, and an `EarlyClose` at 08:30 would leave the ordinary 08:30-15:00 CT regular session standing. `ReplacementBlocks` states the day exactly — one `extended(-1, 17:00, 08:30)` block and nothing else — so the 15:00-16:00 CT extended window is absent, the regular session is absent, and trading resumes with the ordinary 2025-01-10 trade date, which the notice states: `will resume a normal trading schedule for trade date Friday, January 10, 2025`.

**Settlement is not a session boundary here (LAW-SESSION-NOT-EXPIRY).** The mourning notice adds: `The Thursday, January 9, 2025, business day will end at 8:30 a.m. CT on January 9, 2025, and the daily settlement prices of VX, VXM, IBHY, IBIG, and IEMD futures will be determined at that time.` That instant coincides with the block's close and is **not** modelled as a boundary of its own; the close is sourced in session language by `to 8:30 a.m. (Extended Trading Hours)`, and the settlement sentence is quoted here only so a reader can check that the coincidence was noticed rather than overlooked. The notice's `TAS transaction prices in VXT and VXMT … will be based on the daily settlement prices of VX and VXM futures determined at 8:30 a.m.` is the same instant read for a different product and likewise adds nothing.

**Out-of-scope sibling rows on 2025-01-09.** The mourning notice's CFE TRADING SCHEDULE table also lists `UX Options on Futures`, `IBYO and IBGO Options on Futures` and `VA Futures`, each reading `None`. Those three are options and variance contracts on the same exchange, not VIX futures: the `cfe` venue row's documented scope is VIX futures (see **Scope** under Gaps and residual risks), and `cfe_vix` is the key the consumer routes VX to. They are recorded here so the table's shape is complete, and no row ships for them.

### Documents

Every 2025 artifact below was retrieved from the operator's own CDN in one pass on the date recorded, saved under `holidays/raw/cfe-2010-2025/live/` in the research store, and listed with its URL and sha256 in that store's `live_index.json`. Ids follow `CBOE-SU-<holiday year>-<HOLIDAY>`; `CBOE-SU-2025-MOURNING-BASE` is the superseded twin and is no row's document. The 2026 id is repeated here because one `### Documents` table resolves every id this module cites, and its own evidence is the 2026 section below.

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

**Retrieval times are date-level for the twelve 2025 id rows.** The store records the 2025 pass at day granularity only: `live_index.json` carries no capture timestamp per artifact, and the filesystem timestamps that remain are the retrieval host's local clock, not UTC. The date each was fetched is on record; the wall-clock time is not, so none is written here rather than a time being inferred from a file mtime (LAW-UTC-DATES, LAW-NO-FABRICATED-DATES). `CBOE-HOURS-USFUT-2026`'s time is the one the store already recorded.

**Why the 2025 window needs no `Unsourced` row.** The eleven notices cover the eleven dates above, and every other 2025 trade date is a date no Cboe artifact modifies — the operator's own publication pattern is one notice per holiday it observes, so a date with no notice is audited normal rather than unworked. The store's `derived_rows.json` is **not** the source of this table: it carries only nine 2025 rows, missing 2025-11-28, 2025-12-24 and 2025-12-25, because `parsed.json` marks the Christmas notice `"unparsed": true` and its stacked sub-headers break the store's extractor. Those three rows were re-derived by hand from that notice's bytes, and the same pass re-checked the other nine.

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

**Gaps, 2025:** none in this window. Every date Cboe's 2025 notices modify ships a row, and every other 2025 trade date is audited normal. No 2025 row is a late open.

**Interpretive steps, 2025.** The 2025 corpus is per-holiday notices rather than one table, and their layouts are not uniform, so each row below was read from its own notice's bytes at word coordinates. On the seven Monday or Thursday holidays whose regular cell is empty while an extended block is printed — MLK, Presidents, Memorial, Juneteenth, Labor, Thanksgiving and the Thursday half day's own leg — the crate reads an **early close** of the leg that opened at 17:00 CT the previous evening, exactly as the 2026 section reads the same shape; the Trade Date row of each notice confirms it, naming the day after the holiday. Three dates are read differently, each because its own notice prints something the others do not: **2025-01-01** and **2025-07-04** print no cell under the holiday column and are skipped by the Trade Date row, so they are `closed`; **2025-04-18** prints neither a Friday close nor a Friday trade date and is likewise `closed`; and **2025-01-09** is stated in prose as a trading day with no regular hours and ships as `ReplacementBlocks`. The 2025-11-28 and 2025-12-24 half days are `09:00` rows in the sense the 2026 section means: Cboe prints `RTH Start 8:30 AM` and a shortened close, so only the close moves. This reading is fenced per date in `tests/futures_family_boundaries/holidays_cfe_vix.rs`.

**Gaps, 2026:** none inside the window, and the horizon is the operator's. Cboe has published no 2027 CFE holiday schedule: verified 2026-09-26 UTC, <https://www.cboe.com/about/hours/us-futures> carries only a “2026 Futures Holiday Schedule” heading, and its CSV twin <https://www.cboe.com/us/futures/holidays/csv/> (`# Generated: 2026:09:18 17:47:39`) lists 2026 rows only. Nothing is being withheld by this crate — there is simply no 2027 schedule yet. **Closing condition:** the Cboe 2027 futures holiday schedule, at which point the window extends to 2027-12-31. Re-checked monthly per LAW-WATCH. No row in this window changes intraday phase topology, and no row is a late open.

**Interpretive steps, 2026:** Cboe prints two cells per holiday, `Regular Trading Hours` and `Extended Trading Hours`. On nine of these dates the regular cell reads `None` while the extended cell still names a block. On the crate's trade-date key that is an **early close** of the leg which opened at 17:00 CT the previous evening, not a closure: the block printed as “5:00 PM (Sun) to 10:30 AM (Mon)” *is* trade date Monday's own session, and the second block “5:00 PM (Mon) to 8:30 AM (Tue)” belongs to the next trade date and needs no row. Two dates are different. 2026-01-01 is `closed` because the only block printed is the Thursday-evening leg of trade date 2026-01-02, so no session at all belongs to 2026-01-01; and 2026-12-25 prints `None,None`. This reading answers the retrieval's own advisory on 2026-01-01 and is fenced in `tests/futures_family_boundaries/holidays_cfe_vix.rs`.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

The documents below stand behind the row and behind the narrative moved below. All times are Chicago time.

- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2010/SR-CFE-2010-013.pdf> — Cboe SR-CFE-2010-013, the 2010-12-10 extended-session filing — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2011/SR-CFE-2011-019.pdf> — Cboe SR-CFE-2011-019, the 2011-09-26 07:00 start — T1.
- <https://ir.cboe.com/news/news-details/2013/CBOE-Futures-Exchange-Announces-Launch-Dates-For-VIX-Futures-Extended-Trading-Hours-09-30-2013/default.aspx> — the 2013 phased-hours announcement, describing the predecessor 07:00–15:15 trading day and the exact phase shapes — T1.
- <https://ir.cboe.com/news/news-details/2014/2013-Trading-Volume-Reaches-New-All-Time-High-At-CBOE-Futures-Exchange-01-02-2014/default.aspx> — Cboe's year-end retrospective, recording the actual 2013-10-28 and 2013-11-04 launches — T1.
- <https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2013-041.pdf> — Cboe IC13-041, pinning both 2013 phases and publishing the 15:29–15:30 weekday pre-open queue — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2013/SR-CFE-2013-032.pdf> — Cboe SR-CFE-2013-032, the 2013 extended-hours filing — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2013/SR-CFE-2013-034.pdf> — Cboe SR-CFE-2013-034, the 2013 extended-hours filing — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2014/SR-CFE-2014-010.pdf> — Cboe SR-CFE-2014-010, introducing nearly-24-hour VX trading — T1.
- <https://ir.cboe.com/news/news-details/2014/CBOE-Futures-Exchange-Set-For-June-22-Launch-Of-24-Hour-VIX-Futures-Trading-06-09-2014/default.aspx> — the 2014 round-the-clock launch announcement — T1.
- <https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2014-036.pdf> — Cboe IC14-036, publishing the Sunday 16:15–17:00 pre-open and the retained 15:29–15:30 weekday pre-open — T1.
- <https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2014-020.pdf> — Cboe RG-CFE-2014-020, pinning the launch to Sunday 2014-06-22 — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2017/SR-CFE-2017-017.pdf> — Cboe SR-CFE-2017-017, the migration-era phase enumeration — T1.
- <https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2018-005.pdf> — Cboe RG-CFE-2018-005, confirming the migration completed Sunday 2018-02-25 for business date Monday 2018-02-26 — T1.
- <https://cdn.cboe.com/resources/release_notes/2018/Change-to-CFE-Pre-Open-Time-for-TAS-Contracts-and-Order-Submission-Commencement-Times.pdf> — Cboe C2018071603, the 2018-08-12 TAS queue-commencement change — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/pending/2021/21-028-VX-VXM-and-AMERIBOR-Trading-Hours.pdf> — rule certification CFE-2021-028 behind Cboe notice C2021102603, the current queues and trading phases — T1.
- <https://www.cboe.com/about/hours/us-futures> — CFE trading hours, the current-schedule monitoring entry point — T1.
- <https://www.cboe.com/tradable-products/vix/vix-futures/specifications> — VIX futures specifications — T1.
- <https://www.cboe.com/markets/us/futures/regulation/circulars/cfe/regulatory/> — CFE regulatory circulars, the watch channel — T1.

## Gaps and residual risks

- **No dated-history gap.** The normal-week history is primary-supported from
  the January-2010 floor, so the row is `Primary` and its horizon is the floor
  itself rather than a carried-back date.
- **Randomized queue starts are modelled conservatively, not exactly.** From
  2018-02-25 CFE begins each opening queue at a randomized instant within three
  seconds of the nominal boundary, and from 2018-08-12 within six seconds for
  TAS contracts. The profile publishes the latest acceptance edge
  (16:00:03 / 16:45:03, then 16:00:06 / 16:45:06 CT) because each queue follows a
  closed or suspended period, so a caller never sees an open queue that CFE had
  not yet started. No source states a per-contract instant, and none is claimed.
- **Scope.** The `cfe` venue default is specifically VIX futures. Any other CFE
  contract family needs its own review and its own profile; a listing venue is
  not a venue-wide clock.

## Module narrative (moved from src/calendar/schedules/futures/us/cfe.rs on 2026-09-12 UTC)

CFE (VIX) — current schedule, effective 2021-12-06.

RTH is 08:30–15:00 CT. ETH runs 15:00–16:00 and, from Sunday plus
Monday–Thursday, 17:00–08:30. Order-entry queues run Sunday 16:00–17:00 and
Monday–Thursday 16:45–17:00, with starts randomized through six seconds after
the nominal boundary. Because each queue follows a closed/suspended period,
the profile uses the conservative latest 16:00:06 and 16:45:06 edges. The
change removed the former 15:15–15:30 queue and 15:00–15:15 RTH segment.

Sources: Cboe notice C2021102603, effective 2021-12-06; rule certification
CFE-2021-028 (all times in Chicago time).
<https://cdn.cboe.com/resources/regulation/rule_filings/pending/2021/21-028-VX-VXM-and-AMERIBOR-Trading-Hours.pdf>

ORDER-ENTRY CLASSIFICATION. The notice quoted above calls the Sunday
16:00-17:00 and Monday-Thursday 16:45-17:00 windows "order-entry queues":
CFE accepts non-market orders that cannot execute until trading resumes at
17:00, so no trade can print inside them. They are `order_entry`. The
15:00-16:00 and 17:00-08:30 ETH windows match and stay in `extended`.

At the January-2010 audit floor, VX traded 08:30-15:15 CT. CFE then
introduced a 07:20-08:30 extended session effective 2010-12-10 and moved
that start to 07:00 effective 2011-09-26. The filings state both day-level
effective dates and preserve the 08:30-15:15 regular session.
<https://cdn.cboe.com/resources/regulation/rule_filings/approved/2010/SR-CFE-2010-013.pdf>
<https://cdn.cboe.com/resources/regulation/rule_filings/approved/2011/SR-CFE-2011-019.pdf>

CFE expanded VX hours in two phases during 2013. Its announcement describes
the predecessor 07:00–15:15 trading day and the exact phase shapes; Cboe's
year-end retrospective records the actual launches as 2013-10-28 for the
Monday–Thursday 15:30–16:15 session and 2013-11-04 for the move from a 07:00
to a 02:00 morning open. IC13-041 pins those phases to 2013-10-28 and
2013-11-04 and publishes the 15:29–15:30 Monday–Thursday pre-open queue in
both. RTH remained 08:30–15:15 throughout.
<https://ir.cboe.com/news/news-details/2013/CBOE-Futures-Exchange-Announces-Launch-Dates-For-VIX-Futures-Extended-Trading-Hours-09-30-2013/default.aspx>
<https://ir.cboe.com/news/news-details/2014/2013-Trading-Volume-Reaches-New-All-Time-High-At-CBOE-Futures-Exchange-01-02-2014/default.aspx>
<https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2013-041.pdf>
<https://cdn.cboe.com/resources/regulation/rule_filings/approved/2013/SR-CFE-2013-032.pdf>
<https://cdn.cboe.com/resources/regulation/rule_filings/approved/2013/SR-CFE-2013-034.pdf>

IC13-041 publishes 15:29-15:30 as a Monday-Thursday "pre-open queue" ahead of
the 15:30 session, so it accepts orders without matching and is `order_entry`.

CFE-2014-010 introduced nearly-24-hour VX trading on Sunday 2014-06-22.
IC14-036 publishes the resulting 16:15–17:00 Sunday pre-open and retained
15:29–15:30 weekday pre-open; RG-CFE-2014-020 pins the launch to that Sunday.
<https://cdn.cboe.com/resources/regulation/rule_filings/approved/2014/SR-CFE-2014-010.pdf>
<https://ir.cboe.com/news/news-details/2014/CBOE-Futures-Exchange-Set-For-June-22-Launch-Of-24-Hour-VIX-Futures-Trading-06-09-2014/default.aspx>
<https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2014-036.pdf>
<https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2014-020.pdf>

IC14-036 names both the Sunday 16:15-17:00 phase and the retained 15:29-15:30
weekday phase as pre-opens: orders queue, nothing matches.

SR-CFE-2017-017 tied a revised VX schedule to CFE's system migration:
08:30–15:15 RTH, a 15:15–15:30 order-entry-only queue, 15:30–16:00 ETH, a
16:00–16:45 weekday suspension, a 16:45–17:00 queue, then 17:00–08:30 ETH.
Sunday has a 16:00–17:00 opening queue. The queues accept non-market orders
that cannot execute until trading resumes, so the crate classifies them as
`order_entry` under the order-entry-phase convention — `cfe.rs` puts all three
(the 15:15–15:30 weekday queue and the Sunday 16:00 and weekday 16:45 opening
queues) in `CFE_ORDER_ENTRY_2018_02_25`, never in `extended`. The new-system opening
queues begin at randomized instants through three seconds after the nominal
boundary, so their conservative edges are 16:00:03 and 16:45:03. RG18-005
confirms that the migration completed Sunday 2018-02-25, for business date
Monday 2018-02-26.
<https://cdn.cboe.com/resources/regulation/rule_filings/approved/2017/SR-CFE-2017-017.pdf>
<https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2018-005.pdf>
The filing above enumerates the migration-era phases separately: RTH ends at
15:15, 15:15-15:30 is an "order-entry-only queue", 15:30-16:00 is ETH, and
the 16:00/16:45 opening queues run to the 17:00 ETH open. Only the two ETH
windows match, so the 15:15-15:30 queue is split out of the former merged
15:15-16:00 rule and joins the evening queues in `order_entry`.

C2018071603 changed TAS queue commencement to a randomized instant three to
six seconds after the nominal Sunday 16:00 and weekday 16:45 boundaries,
effective with the Sunday 2018-08-12 opening. Non-TAS queues remained within
zero to three seconds. The all-contract profile therefore advances its
conservative latest edge from three to six seconds on that opening day.
<https://cdn.cboe.com/resources/release_notes/2018/Change-to-CFE-Pre-Open-Time-for-TAS-Contracts-and-Order-Submission-Commencement-Times.pdf>
Only the queue-commencement seconds change here, so the matching grid is the
one `CFE_EXT_2018_02_25` already carries.

Row evidence — each revision's day-level effective date and the primary
source that states it (full quotations sit in the blocks above):

```text
  2010-12-10 "Cboe SR-CFE-2010-013"
    https://cdn.cboe.com/resources/regulation/rule_filings/approved/2010/SR-CFE-2010-013.pdf
  2011-09-26 "Cboe SR-CFE-2011-019"
    https://cdn.cboe.com/resources/regulation/rule_filings/approved/2011/SR-CFE-2011-019.pdf
  2013-10-28 "Cboe IC13-041" and 2013-11-04 "Cboe IC13-041"
    https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2013-041.pdf
  2014-06-22 "Cboe RG-CFE-2014-020"
    https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2014-020.pdf
  2018-02-25 "Cboe RG-CFE-2018-005"
    https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2018-005.pdf
  2018-08-12 "Cboe C2018071603"
    https://cdn.cboe.com/resources/release_notes/2018/Change-to-CFE-Pre-Open-Time-for-TAS-Contracts-and-Order-Submission-Commencement-Times.pdf
  2021-12-06 "Cboe C2021102603"
    https://cdn.cboe.com/resources/regulation/rule_filings/pending/2021/21-028-VX-VXM-and-AMERIBOR-Trading-Hours.pdf
```

The 2018-02-25 tuple is keyed to the Sunday implementation; the revised
weekday hours first occur on CFE's Monday 2018-02-26 business date.
