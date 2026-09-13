<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_cryptocurrency` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`cryptocurrency.rs`](../../src/calendar/schedules/futures/us/cryptocurrency.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. CME non-spot-quoted cryptocurrency futures. The exact current 24/7 phases, 2026-05-29 transition, multi-day bounds, weekly close, and following-open-business-day convention are retained, as are the three one-day Saturday maintenance extensions CME's Globex notices state for this family's channels 326/327 — 2026-08-01 to 09:00 CT (notice 20260727), 2026-08-29 to 06:00 and 2026-09-19 to 08:00 CT (notice 20260824, restated by 20260831) — each without a replacement Pre-Open and each reverting to the 02:00–04:00 standard window; the September row is forward-dated on the operator's statement, and the two later rows were added on 2026-09-06 (#61). The 2017–2026 matching grid is exact, but primary evidence does not date the five-day era's Sunday/weekday Pre-Open onset, so dated history omits those queues. The 2026-08-31 review confirmed this at the source: the bitcoin contract specification captured 2017-12-14 — carrying the launch statement for trade date 2017-12-18 — and the 2017-12-22 and 2018-01-04 captures publish the Globex matching grid only and state no Pre-Open. Later member-product listings remain catalog data.

## Revision rows

- 2017-12-17 — T1 — CME SER-8051R — five-day launch grid, 17:00–16:00 CT.
- 2026-05-29 — T1 — CME filing 26-114 — one-day bridge into the 24/7 grid.
- 2026-05-30 — T1 — CME filing 26-114 — permanent 24/7 normal week.
- 2026-08-01 — T1 — CME Globex notice 20260727 — Saturday reopen 09:00 CT.
- 2026-08-02 — T1 — CME Globex notice 20260727 — revert to the standard window.
- 2026-08-29 — T1 — CME Globex notice 20260824 — Saturday reopen 06:00 CT.
- 2026-08-30 — T1 — CME Globex notice 20260824 — revert to the standard window.
- 2026-09-19 — T1 — CME Globex notice 20260824 — Saturday reopen 08:00 CT.
- 2026-09-20 — T1 — CME Globex notice 20260824 — revert to the standard window.

## Holidays

**Coverage:** 2025-01-01..2027-12-31 (inclusive venue-local trade dates). Tier:
T2 throughout — CME's own trading-hours service, the machine channel the
operator's `trading-hours.html` calls to render its per-asset-class Holiday
Hours table, read as bytes and saved. Rows: 24 — 20 `closed`, 4 `early close`,
no `late open` and no `unsourced`.

**Zone.** Verbatim from the operator page: "Trading hours are subject to change
and are in U.S. Central Time unless otherwise stated." This channel prints no
Eastern column, so no ET value is asserted anywhere below; every instant is CT
as printed.

**Event vocabulary**, verbatim from the same page: `preopen` — "Order Entry,
modification, and cancel are allowed. No order matching."; `open` — "Start of
continuous trading phase. Order matching begins."; `closed` — "Final Close of
the date. Day and GTD (current trade date) orders are eliminated." A `closed`
event is therefore the operator's own statement of which trade date ended, and
`/TD <date>` is the trade date the service prints beside an event.

**How an event-date record became a trade-date row.** CME keys a holiday by the
civil day its events fall on and prints each event's own trade date. The crate
keys by the trade date. So a record whose events all carry the *following*
business date is evidence that the holiday has **no trade date of its own**, and
that is a `closed` row on the holiday — not a row on the next day, and not a
`normal` day. The retrieval's own status words map onto that as follows:

| Retrieval status and note | Crate row |
|---|---|
| `closed` with `no events published` | `closed` |
| `closed` `[N17]` — only a 16:00 CT pre-open and a 17:00 CT open, both already carrying the next business date | `closed` |
| `modified` `[N3]` — no final close on the date and a 16:00 CT rather than 16:45 CT pre-open, the whole span rolling into the next business day | no row (trade-date merge; the printed 16:00 CT stop is the family's own normal close) |
| `modified` `[N10]` — 24/7 era, Monday or Thursday holiday: the 16:00 CT final close is omitted outright | `closed` |
| `closed` `[N19]` — 24/7 era, Friday holiday: the 16:00 CT final close is printed but carries the following Monday's trade date | `closed` |
| `early_close` with `close_instant` | `early close` at that instant |
| `modified` `[N6]` — a normal final close with no evening re-open, the leg the *neighbouring* `closed` row already removes | no row |
| `modified` `[N18]` / `normal` `[N8]` — the date settles its own trade date and only the re-open rolls | no row |

**Family label.** The service groups products, and `Cryptocurrency (BTC)` — the
Bitcoin future as the group's representative product — is the line this key is
built from. No separate line for ETH, MBT or MET is published in this channel.

### 2025

Trade dates 2025-01-01 .. 2025-12-31 fall entirely in the five-day
17:00–16:00 CT era, which closes over the weekend, so a `closed` row removes the
complete trading day including the session that opened the previous evening.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `16:00 preopen; 17:00 open` — CT; no final close on this date | `CME-SVC-2024-12-31` | T2 | eventDate 2025-01-01, CME trade date 2025-01-02; eventDate 2024-12-31 prints `16:00 closed` for trade date 2024-12-31 with no evening re-open |
| 2025-04-18 | closed | `no events published` | `CME-SVC-2025-04-17` | T2 | eventDate 2025-04-18; eventDate 2025-04-17 prints `16:00 closed` for trade date 2025-04-17 with no evening re-open |
| 2025-07-04 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2025-07-03` | T2 | eventDate 2025-07-04, CME trade date 2025-07-04 |
| 2025-11-28 | early close | `07:00 preopen; 07:30 open; 13:45 closed` — 13:45 CT | `CME-SVC-2025-11-26` | T2 | eventDate 2025-11-28, CME trade date 2025-11-28 |
| 2025-11-29 | closed | `no events published` | `CME-SVC-2025-11-26-SAT` | T2 | eventDate 2025-11-29; the Saturday-extended window publishes an empty schedule for all ten products, and CME's 2025 Globex table states the period as "27 - 29 November 2025" |
| 2025-12-24 | early close | `12:45 closed` — 12:45 CT, no evening re-open | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-24, CME trade date 2025-12-24 |
| 2025-12-25 | closed | `16:00 preopen; 17:00 open` — CT; no final close on this date | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-25, CME trade date 2025-12-26 |

**Interpretive steps, 2025.**

- Saturday 2025-11-29 is a sourced closure — the live service publishes a
  2025-11-29 schedule for all ten headline products and every one of them is
  empty — and it ships a `closed` row even though the five-day week has no
  Saturday trade date for it to remove. One audited operator closure ships in
  every family that routes to the venue, so the D17 venue intersection is
  computed from one uniform input rather than from eight family judgements.
- The six Monday and Thursday holidays 2025-01-20, 2025-02-17, 2025-05-26,
  2025-06-19, 2025-09-01 and 2025-11-27 carry `modified` `[N3]` records and key
  **no row**: see the trade-date merge below.
- Columbus Day and Veterans Day appear nowhere in CME's Globex holiday list;
  Globex trades a normal session on both. Inside the coverage window they
  therefore read as normal, which is a statement, not an omission.
- 2025-04-17, 2025-07-03 and 2025-12-31 carry `modified` `[N6]` records — a
  normal 16:00 CT final close with no evening re-open. The missing leg is
  deleted by the neighbouring `closed` row, so no row of their own.

**Gaps, 2025.**

- **Trade-date merge — the five-day era's `[N3]` holidays (no row).** On
  2025-01-20, 2025-02-17, 2025-05-26, 2025-06-19, 2025-09-01 and 2025-11-27 CME
  printed a 16:00 CT *pre-open* where a normal day prints a 16:00 CT final
  close, with every event carrying the following business date. No `[N6]`
  predecessor exists on the preceding evening, so the previous 17:00 CT open was
  normal: matching ran from that open through to 16:00 CT exactly as on a normal
  day and resumed at 17:00 CT, and what changed is only the trade-date label CME
  attached to the span. No `is_open` answer moves. The scalar vocabulary cannot
  merge two crate trade dates into one operator trade date, so these dates carry
  **no row** — the same reading `globex_fx` gives the same shared records, and
  the same family of records `globex_equity_index`, `globex_interest_rates` and
  `globex_energy` read as their own early closes. A `closed` row here would
  instead delete the whole trading day, because the five-day profile's weekend
  close short-circuits the business-date roll. Closing condition: the design
  memo's block rows (#93), which can state a merged trade date.
- **Intraday topology — 2025-11-28.** The finalised publication prints
  `07:00 preopen; 07:30 open` ahead of the 13:45 CT close. That is a mid-day
  pause and re-open inside a session that opened the previous evening; scalar
  boundaries cannot state it, and 07:30 CT is not a late open because the day
  began at 17:00 CT the evening before. Recorded, not modelled. Closing
  condition: the block rows of design memo §7 follow-up 8 (#93).
- **Order entry.** On the full-closure dates the pre-open starts at 16:00 CT
  rather than the normal 16:45 CT. The holiday vocabulary copies `DayPolicy`,
  which has no order-entry boundary, so the deviation is unrepresentable. It
  changes no `is_open` answer. Closing condition: an order-entry boundary on
  `DayPolicy`, or the block rows of design memo §7 follow-up 8 (#93).
- **Residual risk — the pre-finalisation captures.** 2025-01-01, 2025-01-20,
  2025-02-17, 2025-04-18, 2025-05-26, 2025-06-19, 2025-07-04 and 2025-09-01 rest
  on the single archive capture 2024-12-20T15:53:40Z of the service, which is a
  pre-holiday publication (of those, only 2025-01-01, 2025-04-18, 2025-07-04 and
  2025-09-01's window still key or bound a row); CME states on the same page that the schedule is
  subject to change and is usually finalised about two weeks before. The service
  no longer answers for those windows — re-probed 2026-09-12, they fall past its
  retention edge and return the products with empty schedules. Closing
  condition: any later archived call of the service over those windows, or a CME
  notice restating the finalised Globex hours. 2025-11-27/28 and 2025-12-24/25
  are **not** exposed to this: both rest on post-holiday captures.
- **Tier.** No T1 per-asset-class rendering of these hours exists. The operator
  page renders the table client-side from this same service, so the archived
  HTML carries no rows.

### 2026

Trade dates through 2026-05-25 are the five-day era; from trade date 2026-05-30
the family is 24/7 and assigns a block to the following open business date, so
from 2026-06-19 onward a `closed` row deletes no trading — it makes that roll
skip the date.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `16:00 preopen; 17:00 open` — CT; no final close on this date | `CME-SVC-2025-12-31` | T2 | eventDate 2026-01-01, CME trade date 2026-01-02; eventDate 2025-12-31 prints `16:00 closed` for trade date 2025-12-31 with no evening re-open |
| 2026-04-03 | early close | `10:15 closed` — 10:15 CT | `CME-SVC-2026-04-01` | T2 | eventDate 2026-04-03, CME trade date 2026-04-03 |
| 2026-06-19 | closed | `16:00 closed /TD 2026-06-22; 16:01 preopen /TD 2026-06-22; 16:02 open /TD 2026-06-22` — CT | `CME-SVC-2026-06-18` | T2 | eventDate 2026-06-19, CME trade date 2026-06-22 |
| 2026-07-03 | closed | `16:00 closed /TD 2026-07-06; 16:01 preopen /TD 2026-07-06; 16:02 open /TD 2026-07-06` — CT | `CME-SVC-2026-07-03` | T2 | eventDate 2026-07-03, CME trade date 2026-07-06 |
| 2026-09-07 | closed | `16:01 preopen; 16:02 open` — CT; the 16:00 CT final close is omitted | `CME-SVC-2026-09-06` | T2 | eventDate 2026-09-07, CME trade date 2026-09-08 |
| 2026-11-26 | closed | `16:01 preopen; 16:02 open` — CT; the 16:00 CT final close is omitted | `CME-SVC-2026-11-25` | T2 | eventDate 2026-11-26, CME trade date 2026-11-27 |
| 2026-12-25 | closed | `16:00 closed /TD 2026-12-28; 16:01 preopen /TD 2026-12-28; 16:02 open /TD 2026-12-28` — CT | `CME-SVC-2026-12-24` | T2 | eventDate 2026-12-25, CME trade date 2026-12-28 |

**Interpretive steps, 2026.**

- The reference week 2026-10-18 .. 2026-10-24, pulled from the same service, is
  what "normal" means here: an ordinary Friday prints
  `16:00 closed /TD 2026-10-23; 16:01 preopen /TD 2026-10-26; 16:02 open /TD 2026-10-26`.
  On the six 24/7-era holiday Fridays the 16:00 CT close instead carries the
  following Monday, which is how the holiday's own trade date is known to be
  absent even though the event times are unchanged.
- 2026-06-17 and 2026-07-02 are `normal` and `modified` `[N18]` respectively:
  both settle their own trade date and only the re-open rolls, so neither keys a
  row. The roll they describe is produced by the 2026-06-19 and 2026-07-03
  `closed` rows.
- The Saturday sessions CME publishes on 2026-06-20 and 2026-07-04
  — `05:00 open; 17:00 closed` for trade date the following Monday — belong to
  the 24-hour *non*-cryptocurrency groups. Cryptocurrency trades its ordinary
  24/7 Saturday grid on both, so neither is a row or a gap for this family.
- 2026-11-27 and 2026-12-24, early closes for most other families, carry no
  cryptocurrency record: the 24/7 grid runs normally on both.

**Gaps, 2026.**

- **Trade-date merge — the five-day era's `[N3]` holidays (no row)**, as 2025:
  2026-01-19, 2026-02-16 and 2026-05-25. Same mechanism, same closing condition.
- **Executable hours — the 60-second maintenance minute.** On the 24/7-era
  Monday and Thursday holidays 2026-09-07 and 2026-11-26 CME omits the 16:00 CT
  final close outright, so its ordinary 16:00–16:02 CT maintenance window is
  absent that day. The crate's normal week keeps a 16:00–16:01 CT gap, so
  `is_open` answers false for 60 seconds the operator published as continuous.
  The scalar vocabulary cannot delete a gap; recorded, not modelled, with the
  block rows of design memo §7 follow-up 8 (#93) the closing condition. The Friday
  holidays are not affected — there the 16:00 CT close *is* printed.
- **Order entry**, as 2025, on 2026-01-01, and on the three trade-date-merge
  dates above.
- **Tier**, as 2025.

### 2027

The whole year is the 24/7 era.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `16:00 closed /TD 2027-01-04; 16:01 preopen /TD 2027-01-04; 16:02 open /TD 2027-01-04` — CT | `CME-SVC-2026-12-31` | T2 | eventDate 2027-01-01, CME trade date 2027-01-04 |
| 2027-01-18 | closed | `16:01 preopen; 16:02 open` — CT; the 16:00 CT final close is omitted | `CME-SVC-2027-01-17` | T2 | eventDate 2027-01-18, CME trade date 2027-01-19 |
| 2027-02-15 | closed | `16:01 preopen; 16:02 open` — CT; the 16:00 CT final close is omitted | `CME-SVC-2027-02-14` | T2 | eventDate 2027-02-15, CME trade date 2027-02-16 |
| 2027-03-26 | closed | `16:00 closed /TD 2027-03-29; 16:01 preopen /TD 2027-03-29; 16:02 open /TD 2027-03-29` — CT | `CME-SVC-2027-03-25` | T2 | eventDate 2027-03-26, CME trade date 2027-03-29 |
| 2027-05-31 | closed | `16:01 preopen; 16:02 open` — CT; the 16:00 CT final close is omitted | `CME-SVC-2027-05-30` | T2 | eventDate 2027-05-31, CME trade date 2027-06-01 |
| 2027-06-18 | closed | `16:00 closed /TD 2027-06-21; 16:01 preopen /TD 2027-06-21; 16:02 open /TD 2027-06-21` — CT | `CME-SVC-2027-06-17` | T2 | eventDate 2027-06-18, CME trade date 2027-06-21 |
| 2027-07-05 | closed | `16:01 preopen; 16:02 open` — CT; the 16:00 CT final close is omitted | `CME-SVC-2027-07-04` | T2 | eventDate 2027-07-05, CME trade date 2027-07-06 |
| 2027-09-06 | closed | `16:01 preopen; 16:02 open` — CT; the 16:00 CT final close is omitted | `CME-SVC-2027-09-05` | T2 | eventDate 2027-09-06, CME trade date 2027-09-07 |
| 2027-11-25 | closed | `16:01 preopen; 16:02 open` — CT; the 16:00 CT final close is omitted | `CME-SVC-2027-11-24` | T2 | eventDate 2027-11-25, CME trade date 2027-11-26 |
| 2027-12-24 | closed | `16:00 closed /TD 2027-12-23; 16:01 preopen /TD 2027-12-27; 16:02 open /TD 2027-12-27` — CT, on eventDate 2027-12-23 | `CME-SVC-2027-12-22` | T2 | eventDate 2027-12-23, CME trade date 2027-12-27 — the re-open skips 2027-12-24; the same document prints `no events published` for eventDate 2027-12-24 for the nine other product groups, with no cryptocurrency record because the 24/7 grid never stops |

**Interpretive steps, 2027.**

- CME's own 2027 Christmas holiday date is Thursday 2027-12-23, with Globex shut
  on Friday 2027-12-24 and 25 December falling on a Saturday. Cryptocurrency
  settles its own trade date on 2027-12-23 — `16:00 closed /TD 2027-12-23` — so
  that date keys **no** row. The row is 2027-12-24, and it is keyed from the
  trade date the 16:01/16:02 re-open carries: 2027-12-27, skipping the Friday.
  This is the one row in the table whose *only* evidence is the trade date
  printed beside a neighbouring day's events, which is precisely what the
  trade-date key is for.
- 2028-01-01 is a Saturday and prints the ordinary 24/7 Saturday grid
  `02:00 closed; 03:45 preopen; 04:00 open` for trade date 2028-01-03. It is
  outside the coverage window and is not a row.
- 2027-12-31 carries no cryptocurrency record: the 24/7 grid runs normally.

**Gaps, 2027.**

- **Executable hours — the 60-second maintenance minute**, as 2026, on
  2027-01-18, 2027-02-15, 2027-05-31, 2027-07-05, 2027-09-06 and 2027-11-25.
- **Sourced but unrepresentable — the Saturday session 2027-06-19.** CME
  publishes `05:00 open; 17:00 closed` for trade date 2027-06-21 for the 24-hour
  groups after the Juneteenth Friday. Cryptocurrency is not among them and runs
  its normal Saturday grid, so this is recorded only so a reader does not look
  for a row.
- **Tier**, as 2025.
- **Forward-dated rows.** Every 2027 row and the 2026 rows from 2026-09-07 on
  are encoded ahead of their effective day on the operator's published future,
  which LAW-NO-FABRICATED-DATES permits for an unconditional, fully sourced
  date. They must be confirmed against the operator before each day
  (LAW-WATCH); CME states the schedule is subject to change and is usually
  finalised about two weeks before.

### Documents

Every id below resolves to the CME Group trading-hours service, tier T2.
`[THBP-A]` expands to
`https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true`
and `WA:<ts>id_/` to `https://web.archive.org/web/<ts>id_/`. `cmegroup.com`
returns HTTP 403 to the retrieving machine, so live calls were read through the
public reader `https://r.jina.ai/<url>` and history through the Wayback Machine
with `id_` replay. Bytes and manifests: `holidays/raw/cme-2025-2027*` in the
research store.

| Document | Window | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|
| `CME-SVC-2024-12-31` | 2024-12-31 .. 2025-01-02 | archive capture 2024-12-20T15:53:40Z | T2 | `375c70eecd19c5c6204ecb408d1b3210a9da4c9a03b85ef1c7dbbcde1397ab63` |
| `CME-SVC-2025-01-19` | 2025-01-19 .. 2025-01-21 | archive capture 2024-12-20T15:53:40Z | T2 | `4f2ab56af14e7b3a6978e7fa6db8e2cfc63a82d06428cd88844f0e5bcf534f40` |
| `CME-SVC-2025-02-16` | 2025-02-16 .. 2025-02-18 | archive capture 2024-12-20T15:53:40Z | T2 | `5bec2ca6b4999a534e4d9818035aaa18ec8626b6c912cf7e3d2c57015536f2fa` |
| `CME-SVC-2025-04-17` | 2025-04-17 .. 2025-04-19 | archive capture 2024-12-20T15:53:40Z | T2 | `865a1d4f08102e00151bd87ab2b8e8a7720e9203a17aaaba24627ade3ed26e74` |
| `CME-SVC-2025-05-25` | 2025-05-25 .. 2025-05-27 | archive capture 2024-12-20T15:53:40Z | T2 | `5f42869879c826f5949b79236aabb3d26d74e7565d92d7cc5e8784c63973210b` |
| `CME-SVC-2025-06-18` | 2025-06-18 .. 2025-06-20 | archive capture 2024-12-20T15:53:40Z | T2 | `a572706907175776255261103b393493ebdf5a8106ec5374d129145bdf89105e` |
| `CME-SVC-2025-07-03` | 2025-07-03 .. 2025-07-05 | archive capture 2024-12-20T15:53:40Z | T2 | `b80cd4bfed0ae72865bfacc1936e107eb8febfcc94b37fcce1d05505c659147b` |
| `CME-SVC-2025-08-31` | 2025-08-31 .. 2025-09-02 | archive capture 2024-12-20T15:53:40Z | T2 | `e075762ed34a86048d94900e10edba10d95b3d766052908ffbb5133f6b64bab0` |
| `CME-SVC-2025-11-26` | 2025-11-26 .. 2025-11-28 | archive capture 2026-01-29T01:23:09Z | T2 | `6c4c598791058dd9a11aff0ddb072c761a436c6d1054b891def74c6935f020f1` |
| `CME-SVC-2025-11-26-SAT` | 2025-11-26 .. 2025-11-29 | live retrieval 2026-09-12T08:55:12Z | T2 | `2e9f34f20085de3ccbdff1dc29cb7463bcff93713ef0c550740d6f15e0635ab7` |
| `CME-SVC-2025-12-24` | 2025-12-24 .. 2025-12-26 | archive capture 2026-01-29T01:21:59Z | T2 | `322a2be989b67f5f4cc0ec12fd63a393383d574badd4aacc87a0c9637533d386` |
| `CME-SVC-2025-12-31` | 2025-12-31 .. 2026-01-02 | archive capture 2026-06-19T11:41:05Z | T2 | `0ed61f8328eda4746265cc8e197f10cd53aec06c2b393927bab27c913993d314` |
| `CME-SVC-2026-01-18` | 2026-01-18 .. 2026-01-20 | archive capture 2026-06-19T11:41:05Z | T2 | `5e3ff08bdc7d07474b96b8dc8c18ed0d5e48d12dc4bcad81a5f68820cb2aa89e` |
| `CME-SVC-2026-02-15` | 2026-02-15 .. 2026-02-17 | archive capture 2026-06-19T11:41:05Z | T2 | `5dd507dd959d0029e838ec88b1bdb63c32444ea36a121de002606f5d7b206e2f` |
| `CME-SVC-2026-04-01` | 2026-04-01 .. 2026-04-03 | archive capture 2026-06-19T11:41:18Z | T2 | `54bcc271e9ba9737a99a2fe608e658de0c657075284d050fbfec4fe1aee2a2a5` |
| `CME-SVC-2026-05-24` | 2026-05-24 .. 2026-05-26 | archive capture 2026-06-19T11:41:05Z | T2 | `f7e30d204ce2cbe08e5f486ded6518f623369159f3a36161288a4708288314da` |
| `CME-SVC-2026-06-18` | 2026-06-18 .. 2026-06-20 | archive capture 2026-06-19T11:34:04Z | T2 | `97fd5da371309f4486a8fb49ff2105c6c1c2396939ab7c76f1a2a1097b6f015c` |
| `CME-SVC-2026-07-03` | 2026-07-03 .. 2026-07-05 | archive capture 2026-06-19T11:41:08Z | T2 | `4b89a026358e998277f9c1ff7e095e5d4e625cdc45115fd141dc92201833155b` |
| `CME-SVC-2026-09-06` | 2026-09-06 .. 2026-09-08 | live retrieval 2026-09-12T04:30Z | T2 | `01fb78ffaac10eac466fed53674214222f05aed518b9d93a4b42cf8957147bca` |
| `CME-SVC-2026-11-25` | 2026-11-25 .. 2026-11-27 | live retrieval 2026-09-12T04:30Z | T2 | `e1f35a5623b3c5d15e7468b2cb4119e587411a9714f920605dab11bf688756d1` |
| `CME-SVC-2026-12-24` | 2026-12-24 .. 2026-12-26 | live retrieval 2026-09-12T04:30Z | T2 | `bdc1fe831adb794bcf8aeb7e99baf6af2009d1ff9969d0a48b18b2ebc2e1e829` |
| `CME-SVC-2026-12-31` | 2026-12-31 .. 2027-01-02 | live retrieval 2026-09-12T04:30Z | T2 | `7162652821c16f1bd05e3ec533bd5b82af03833c7186a64c7734b0b650364dcd` |
| `CME-SVC-2027-01-17` | 2027-01-17 .. 2027-01-19 | live retrieval 2026-09-12T04:30Z | T2 | `7155c4b7ee8b299b3033eb3daf002b6ceecf0fbd53f6f98a7036048022275743` |
| `CME-SVC-2027-02-14` | 2027-02-14 .. 2027-02-16 | live retrieval 2026-09-12T04:30Z | T2 | `41f5aa8cde3879f8b10490386c134a294a0f1509edde2022a22ec3ffcaed1183` |
| `CME-SVC-2027-03-25` | 2027-03-25 .. 2027-03-27 | live retrieval 2026-09-12T04:30Z | T2 | `9bd7225d440e00139f30892f3914c9b38beb8bf29d4272039b6cd8f2de926880` |
| `CME-SVC-2027-05-30` | 2027-05-30 .. 2027-06-01 | live retrieval 2026-09-12T04:30Z | T2 | `1283649724c30163fa08ba7ab02d1230fa9a7dd0613b8b4b3d96cd1d9dc4febd` |
| `CME-SVC-2027-06-17` | 2027-06-17 .. 2027-06-19 | live retrieval 2026-09-12T04:30Z | T2 | `60c9a2f5106d61039a616986b463cd852861ee4d3b91b11fac8badfa1b97b01c` |
| `CME-SVC-2027-07-04` | 2027-07-04 .. 2027-07-06 | live retrieval 2026-09-12T04:30Z | T2 | `93ff8232886435c94be682bf968aa30749011cdf8dadeb7d2425a3b0b9e0bf71` |
| `CME-SVC-2027-09-05` | 2027-09-05 .. 2027-09-07 | live retrieval 2026-09-12T04:30Z | T2 | `aa08a3bd102812928e69cf1ea4c8a84f738eaa5d14f967acee7d2571e74aedb9` |
| `CME-SVC-2027-11-24` | 2027-11-24 .. 2027-11-26 | live retrieval 2026-09-12T04:30Z | T2 | `6aa7c0fd701a02480dabeac1fbae1a69b56e77643a29e3a9b2223c56e822ce9f` |
| `CME-SVC-2027-12-22` | 2027-12-22 .. 2027-12-25 | live retrieval 2026-09-12T04:30Z | T2 | `5edc4dd588a32faa74f841494c10a3df48692dca29843c3581bad3e18c30fef9` |


## Sources

Row review: 2026-09-06 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.cmegroup.com/notices/ser/2017/12/SER-8051R.html> — CME SER-8051R, bitcoin futures launch, the 2017-12-17 revision's source.
- <https://www.cmegroup.com/market-regulation/rule-filings/2017/12/17-417.pdf> — CME rule filing 17-417, the launch filing carrying the original 17:00–16:00 CT weekday grid.
- <https://web.archive.org/web/20171214071544id_/http://www.cmegroup.com/trading/equity-index/us-index/bitcoin_contract_specifications.html> — CME bitcoin contract specification — capture 2017-12-14, carrying the launch statement and publishing the matching grid only.
- <https://www.cmegroup.com/notices/clearing/2021/01/Chadv21-028.pdf> — CME clearing advisory Chadv21-028, the ETH launch.
- <https://www.cmegroup.com/notices/electronic-trading/2021/04/20210426.html> — CME Globex notice 20210426, the MBT launch.
- <https://www.cmegroup.com/notices/electronic-trading/2021/11/20211129.html> — CME Globex notice 20211129, the MET launch.
- <https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2026/5/26-114.pdf> — CME rule filing 26-114, 24/7 cryptocurrency trading, the 2026-05-29 and 2026-05-30 revisions' source.
- <https://www.cmegroup.com/notices/electronic-trading/2026/05/20260525.html> — CME Globex notice 20260525, corroborating the 24/7 transition.
- <https://www.cmegroup.com/articles/faqs/frequently-asked-questions-cryptocurrency-futures.html> — CME cryptocurrency futures FAQ, current grid.
- <https://www.cmegroup.com/notices/electronic-trading/2026/07/20260727.html> — CME Globex notice 20260727, the 2026-08-01 Saturday extension.
- <https://www.cmegroup.com/notices/electronic-trading/2026/08/20260824.html> — CME Globex notice 20260824, the 2026-08-29 and 2026-09-19 Saturday extensions.
- <https://www.cmegroup.com/notices/electronic-trading/2026/08/20260831.html> — CME Globex notice 20260831, restating notice 20260824.

Official origin of the specification capture: <http://www.cmegroup.com/trading/equity-index/us-index/bitcoin_contract_specifications.html>.

## Gaps and residual risks

- **order-entry** — the five-day era's Sunday and weekday Pre-Open onset is undated; the 2017-12-14, 2017-12-22 and 2018-01-04 contract-specification captures publish the matching grid only. Closing condition: a CME artifact that states the Pre-Open in session language on a day-level effective date. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **forward-dated row** — the 2026-09-19 Saturday extension is encoded ahead of its effective day on CME Globex notice 20260824, restated by notice 20260831. It must be confirmed against the operator before that day (LAW-WATCH).
- **holidays** — the built-in holiday table covers trade dates 2025-01-01 .. 2027-12-31 at T2. Its per-year gaps, interpretive steps and residual risks are recorded under `## Holidays`, beside the rows they are about, rather than pooled here. The load-bearing ones: the five-day era's `closed` rows delete trading CME kept open on nine dates, the 24/7 era's Monday and Thursday holidays lose a 60-second maintenance gap, 2025-11-28 carries an unrepresentable intraday re-open, and no T1 rendering of these hours exists.
- **scope** — ETH, MBT and MET joined this already-live family in 2021 and later member-product listings remain catalog data; their individual launch dates are not family-clock revisions.

## Module narrative (moved from src/calendar/schedules/futures/us/cryptocurrency.rs on 2026-09-12 UTC)

2026-08-31 five-day-era Pre-Open review — confirmed knowledge-bound. The CME
bitcoin contract specification captured 2017-12-14, which itself carries the
launch statement "Effective Sunday 17 December 2017 for trade date Monday 18
December 2017 ... CME will launch Bitcoin Futures", publishes only the
matching grid: "CME Globex: Sunday - Friday 6:00 p.m. - 5:00 p.m. (5:00 p.m.
- 4:00 p.m. CT) with a 60-minute break each day beginning at 5:00 p.m. (4:00
p.m. CT)". It states no Pre-Open, and neither do the 2017-12-22 or
2018-01-04 captures. The five-day era's Sunday/weekday Pre-Open onset is
therefore undated at the source, not merely unsearched. Official origin
http://www.cmegroup.com/trading/equity-index/us-index/bitcoin_contract_specifications.html
delivered via
https://web.archive.org/web/20171214071544id_/http://www.cmegroup.com/trading/equity-index/us-index/bitcoin_contract_specifications.html

Bitcoin futures opened Sunday 2017-12-17 at 17:00 CT for trade date Monday
2017-12-18. The launch filing gives the original 17:00-16:00 weekday grid.
ETH, MBT, and MET joined this already-live family in 2021; their individual
launch dates are intentionally not family-clock revisions.

CME filing 26-114 changed all non-spot-quoted cryptocurrency futures to
24/7 Globex trading effective Friday 2026-05-29: matching maintenance is
16:00-16:02 CT Monday-Friday with Pre-Open from 16:01, and 02:00-04:00 CT
Saturday with Pre-Open from 03:45. Three one-day Globex notices then
temporarily extended the Saturday window for the 24/7 markets — 2026-08-01
through 09:00 CT (notice 20260727), 2026-08-29 through 06:00 CT and
2026-09-19 through 08:00 CT (notice 20260824, restated by 20260831) — each
without publishing a replacement Pre-Open, each followed by the standard
02:00-04:00 window. The notices' tables name this family's channels, "CME
Crypto Futures | 74 | 326" and "CME Crypto Options | 327", alongside the
event-contract channels. The September row is forward-dated on the
operator's statement.

`SessionRule` spans at most one local midnight, so the multi-day weekend
session is stored in adjacent pieces. The key-backed calendar joins those
storage-only pieces at query time, while retaining the 02:00-03:45 Saturday
closed break and 03:45-04:00 Pre-Open. Both weekend blocks carry the following
open business date: normally Monday, or Tuesday when a caller policy closes
Monday. The corresponding daily bar runs from Friday 16:01 Pre-Open through
that business date's 16:00 close.
https://www.cmegroup.com/notices/ser/2017/12/SER-8051R.html
https://www.cmegroup.com/market-regulation/rule-filings/2017/12/17-417.pdf
https://www.cmegroup.com/notices/clearing/2021/01/Chadv21-028.pdf
https://www.cmegroup.com/notices/electronic-trading/2021/04/20210426.html
https://www.cmegroup.com/notices/electronic-trading/2021/11/20211129.html
https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2026/5/26-114.pdf
https://www.cmegroup.com/notices/electronic-trading/2026/05/20260525.html
https://www.cmegroup.com/articles/faqs/frequently-asked-questions-cryptocurrency-futures.html
https://www.cmegroup.com/notices/electronic-trading/2026/07/20260727.html
https://www.cmegroup.com/notices/electronic-trading/2026/08/20260824.html
https://www.cmegroup.com/notices/electronic-trading/2026/08/20260831.html
