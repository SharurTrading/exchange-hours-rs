<!-- SPDX-License-Identifier: MIT-0 -->

# `cme` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`cme_group.rs`](../../src/calendar/schedules/futures/us/cme_group.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Compatibility default for the scoped CME/CBOT equity-index family. Current RTH/ETH and Pre-Open queues are primary-supported; matching and the exact 2010, 2012, 2015, and 2021 revisions are dated, but the Sunday queue's move from 16:15 to 16:00 lacks a source-stated day: the 2026-08-31 review narrowed it to 2012-05-28..2012-06-07 and found no operator notice in either CME channel across that window. Dated profiles now serve the sourced intersection — Sunday 16:15–17:00, carried from the January-2010 floor because the queue only ever widened — so only the disputed 16:00–16:15 quarter-hour waits on the undated move. Full-size `SP`, NKD, BTIC, and TACO products are excluded.

## Revision rows

- 2010-11-15 — T1 — CME Globex notice 20101025 — Monday–Thursday Pre-Open moves from 16:50 to 16:45 CT.
- 2012-11-18 — T1 — CME Globex notice 20121022 — new daily trading-hour schedule; post-halt slice becomes 15:30–16:15 CT including Fridays.
- 2015-09-20 — T1 — CME Globex notice 20150817 — CME Equity and CBOT Equity closes move 15 minutes earlier to 16:00 CT.
- 2021-06-27 — T1 — CME Globex notice 20210621 — the 15:15–15:30 CT halt is removed, producing the continuous 17:00–16:00 CT ETH envelope.
- 2026-08-22 — T1 — 2026-08-22 review: verified current, onset undated — knowledge-bound row widening the Sunday queue to the sourced current 16:00–17:00 CT Pre-Open.

## Holidays

**Coverage:** 2025-01-01 .. 2027-12-31 (inclusive venue-local trade dates in
`America/Chicago`). Tier: **T2** throughout. Inside the window a date with no row is
audited and normal; outside it this table has no answer at all.

**This table is derived, not retrieved.** It is the **intersection** of the holiday
tables the crate ships for the six CME product families that route to `Exchange::Cme`
— `globex_equity_index`, `globex_energy`, `globex_fx`, `globex_grains`,
`globex_interest_rates` and `globex_livestock` — and every id it cites resolves to the
same CME Group `trading-hours-by-product` artifact those families' evidence files
already record. There is no venue evidence of its own to add here, and nothing in the
venue module rests on a document a family module does not carry. The per-family rows,
with their event-date-to-trade-date conversions, are in the six key evidence files;
this section records the derivation, the routing, every date the intersection drops,
and the interpretive steps that produced both.

**The intersection rule (design memo D17).** A `MarketHoursKey` holiday row is a
statement about one family's clock, and the six families do not share one: on a
holiday Monday the equity-index and interest-rate families halt matching at 12:00 CT,
energy and metals at 13:30 CT, the grain and oilseed day session ends at 12:05 CT or
not at all, and livestock prints a 13:05 CT close. A venue row may therefore be stated
only where every routed family states the same row. **Nine dates** qualify in this
window — the Globex full closures — and the other **thirty-two** dates carry
`unsourced`.

**`unsourced` is neither silence nor a compromise.** The coverage window is
contiguous, so a date carrying no row is the positive claim that it was audited
normal, which is false on every one of these dates. Nor can the venue state an
instant: a row copying the shallowest early close (12:00 CT) would delete the grain
and livestock families' sourced 12:05-13:05 CT trading, and one copying the deepest
would delete the energy family's 13:30-16:00 CT session and the equity-index family's
15:15-16:00 CT extended phase. `unsourced` clips nothing, changes no answer, and tells
a caller what the crate knows: the date is special and the venue has no single answer
for it. `iceus`, whose venue table shipped first, is the precedent.

**Cite the family, not the venue, for holiday behaviour.** These thirty-two rows are
the reason the consumer contract tells a caller to route holiday questions through
the product-family key: `globex_equity_index` knows what CME equity index does on
2026-12-24 and this venue deliberately does not claim to.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `no events published` | `CME-SVC-2024-12-31` | T2 | equity index closed; energy closed; FX closed; grains closed; interest rates closed; livestock closed |
| 2025-01-02 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2024-12-31` | T2 | grains late open 08:30 CT; no row in equity index, energy, FX, interest rates, livestock |
| 2025-01-20 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-01-19` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2025-02-17 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-02-16` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2025-04-18 | closed | `no events published` | `CME-SVC-2025-04-17` | T2 | equity index closed; energy closed; FX closed; grains closed; interest rates closed; livestock closed |
| 2025-05-26 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-05-25` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2025-06-19 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-06-18` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2025-07-03 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-07-03` | T2 | equity index early close 12:15 CT; no row in energy, FX, grains, interest rates, livestock |
| 2025-07-04 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-07-03` | T2 | equity index early close 12:00 CT; energy early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2025-09-01 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-08-31` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2025-11-27 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-11-26-SAT` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2025-11-28 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-11-26` | T2 | equity index early close 12:15 CT; energy early close 13:45 CT; FX early close 13:45 CT; grains late open 00:10 CT and early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:05 CT |
| 2025-11-29 | closed | `no events published` | `CME-SVC-2025-11-26-SAT` | T2 | equity index closed; energy closed; FX closed; grains closed; interest rates closed; livestock closed |
| 2025-12-24 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-12-24` | T2 | equity index early close 12:15 CT; energy early close 12:45 CT; FX early close 12:45 CT; grains early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:15 CT |
| 2025-12-25 | closed | `no events published` | `CME-SVC-2025-12-24` | T2 | equity index closed; energy closed; FX closed; grains closed; interest rates closed; livestock closed |
| 2025-12-26 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-12-24` | T2 | grains late open 08:30 CT; no row in equity index, energy, FX, interest rates, livestock |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `no events published` | `CME-SVC-2025-12-31` | T2 | equity index closed; energy closed; FX closed; grains closed; interest rates closed; livestock closed |
| 2026-01-02 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2025-12-31` | T2 | grains late open 08:30 CT; no row in equity index, energy, FX, interest rates, livestock |
| 2026-01-19 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-01-18` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2026-02-16 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-02-15` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2026-04-03 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-04-01` | T2 | equity index early close 08:15 CT; energy closed; FX early close 10:15 CT; grains closed; interest rates early close 10:15 CT; livestock closed |
| 2026-05-25 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-05-24` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2026-06-19 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-06-18` | T2 | equity index early close 12:00 CT; energy early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2026-07-03 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-07-03` | T2 | equity index early close 12:00 CT; energy early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2026-09-07 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-09-06` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2026-11-26 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-11-25` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2026-11-27 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-11-25` | T2 | equity index early close 12:15 CT; energy early close 13:45 CT; FX early close 13:45 CT; grains late open 00:10 CT and early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:05 CT |
| 2026-12-24 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2026-12-22` | T2 | equity index early close 12:15 CT; energy early close 12:45 CT; FX early close 12:45 CT; grains early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:05 CT |
| 2026-12-25 | closed | `no events published` | `CME-SVC-2026-12-24` | T2 | equity index closed; energy closed; FX closed; grains closed; interest rates closed; livestock closed |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `no events published` | `CME-SVC-2026-12-31` | T2 | equity index closed; energy closed; FX closed; grains closed; interest rates closed; livestock closed |
| 2027-01-18 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-01-17` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2027-02-15 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-02-14` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2027-03-26 | closed | `no events published` | `CME-SVC-2027-03-25` | T2 | equity index closed; energy closed; FX closed; grains closed; interest rates closed; livestock closed |
| 2027-05-31 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-05-30` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2027-06-18 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-06-17` | T2 | equity index early close 12:00 CT; energy early close 12:00 CT; FX early close 12:00 CT; grains closed; interest rates early close 12:00 CT; livestock closed |
| 2027-07-05 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-07-04` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 13:30 CT; livestock closed; no row in FX |
| 2027-07-06 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-07-04` | T2 | grains late open 08:30 CT; no row in equity index, energy, FX, interest rates, livestock |
| 2027-09-06 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-09-05` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2027-11-25 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-11-24` | T2 | equity index early close 12:00 CT; energy early close 13:30 CT; grains closed; interest rates early close 12:00 CT; livestock closed; no row in FX |
| 2027-11-26 | unsourced | no single cell — the routed families disagree, so the venue states no instant | `CME-SVC-2027-11-24` | T2 | equity index early close 12:15 CT; energy early close 13:45 CT; FX early close 13:45 CT; grains late open 00:10 CT and early close 12:05 CT; interest rates early close 12:15 CT; livestock early close 12:05 CT |
| 2027-12-24 | closed | `no events published` | `CME-SVC-2027-12-22` | T2 | equity index closed; energy closed; FX closed; grains closed; interest rates closed; livestock closed |

**Gaps, 2025-2027.**

- **The thirty-two `unsourced` dates are the intersection's residue, not a research
  gap.** Each one is a date on which at least two of the six families state different
  rows, and the disagreement is printed per date in the `Derived from` column above.
  Every underlying row is sourced; what is missing is a single venue-wide answer,
  which no operator document states and which the crate will not invent. Closing
  condition: an operator statement of venue-wide holiday hours — CME's holiday-hours
  table on `cmegroup.com/trading-hours.html` is per asset class, not per venue — or a
  `DayPolicy`-shaped boundary that can express a per-family answer inside one venue
  calendar. Neither exists today.
- **`no row in FX` on twenty-two of the thirty-two disputed dates.** `globex_fx`
  carries no row on 2025-01-02, 2025-01-20, 2025-02-17, 2025-05-26, 2025-06-19,
  2025-07-03, 2025-09-01, 2025-11-27, 2025-12-26, 2026-01-02, 2026-01-19, 2026-02-16,
  2026-05-25, 2026-09-07, 2026-11-26, 2027-01-18, 2027-02-15, 2027-05-31, 2027-07-05,
  2027-07-06, 2027-09-06 and 2027-11-25, while at least one of the other five families
  states a row on each of them. FX's own evidence file audits those dates normal: CME
  prints `16:00 closed` for `6E` as the family's **ordinary** final close, not a holiday
  halt. That is a row the FX family deliberately does not carry, so the venue sees a
  disagreement where the families may in fact agree about trading. It is recorded here
  rather than resolved because resolving it would mean the venue second-guessing a
  family's own audited normal, which is exactly what D17 forbids. Closing condition: an
  operator statement that FX's 16:00 CT close on those dates is a holiday halt rather
  than the ordinary grid, which would let `globex_fx` carry a row.
- **The four single-family `late open` dates.** 2025-01-02, 2025-12-26, 2026-01-02 and
  2027-07-06 carry a `globex_grains` late open at 08:30 CT and nothing else. The other
  five families run a continuous overnight leg through those dates, so for them no
  boundary moves and no row exists. The venue therefore cannot state the grain late
  open as its own. It is also inert for this venue calendar, whose profile already opens
  its day session at 08:30 CT; it is recorded because a venue row must be true of every
  routed family, not only of the one that states it.
- **The window is the families' window, not the venue's horizon.** Coverage starts
  2025-01-01 because that is where every routed family's table starts, not because
  this venue has no history: CME's own service reaches no further back for these
  families, and the 2010-2024 blocks land in stage 2.2's waves. Each wave extends this
  table over its own years by the same derivation, and the cross-wave agreement audit
  (#95) closes with the last of them. The venue's *schedule* rows still reach the
  January-2010 floor; the two are different claims, and outside the holiday window the
  crate answers the normal week correctly and has no holiday answer at all.
- **2028-01-01 ships no row.** CME's service publishes a 2027-12-30 .. 2028-01-02
  window and every family's table ends at 2027-12-31, so the venue's window ends
  there too. Extending it is stage 2.4's refresh, not a gap in this change.

**Interpretive steps, 2025-2027.**

- **The routing is read from production, not assumed.** `hours_for_exchange`'s
  `Exchange::Cme` arm resolves to `cme_profile_at`, and the six keys above are the
  `MarketHoursKey` values whose own tables cover the products that profile serves;
  `globex_cryptocurrency`, `globex_nikkei_225_dollar`, the five metals TAS keys and
  the dormant CME keys route elsewhere or ship no table, so they are not part of the
  intersection and a disagreement with them cannot drop a venue row. The test
  `the_venue_table_is_the_intersection_of_its_families` recomputes the whole table
  from the six families' public `holiday_on` answers on every run, so this list and
  the table cannot drift apart silently.
- **A disagreement ships `unsourced` rather than nothing.** This is the one place the
  module departs from D17's literal "ships no venue row", and it follows `iceus`,
  which shipped first and states the reason: with a contiguous coverage window,
  silence is a claim. The row is cited to a document the crate holds, because the
  `holidays!` fence requires a non-empty id and because the id is what lets a reader
  reach the bytes behind the disagreement.
- **The venue's normal week is unchanged.** Every row here is a clip on a schedule
  `hours_for_exchange(Exchange::Cme, _)` already serves; no row creates a session the
  profile does not have, and `unsourced` rows do not touch the schedule at all. The
  golden normal-week grids are therefore untouched by this change.
- **Saturday 2025-11-29 is the only weekend closure in the intersection.** It is a
  full closure in all six families and ships as `closed`. The `cme` profile has no
  Saturday session to remove, so the row changes no answer for this venue; it ships
  because the operator answered, exactly as the family table records.

### Documents

Every id below resolves to one response of CME Group's own
`trading-hours-by-product` service — the endpoint `cmegroup.com/trading-hours.html`
itself calls — for the `THBP-A` product set
`id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true`.
Archived responses were replayed through `https://web.archive.org/web/<timestamp>id_/`;
live responses were read through the public reader `https://r.jina.ai/`, because
cmegroup.com returns HTTP 403 to the retrieving machine. Paths are relative to
`exchange-hours-research/holidays/`. The table is the same one the routed families'
evidence files carry — this venue rests on no artifact they do not — and several ids
resolve to one artifact, because one service window answers two or three trade dates.
The research-store id of each artifact is in the `Derived from` column of the year
tables in the family files.

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
| `CME-SVC-2026-12-22` | 2026-12-22 .. 2026-12-24 | live retrieval 2026-09-12T04:30Z | T2 | `c8c0267da8cf171409ad8ca188082b3aa326e8d04a89d12503dcf9f57bf3b7ab` |
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

### Documents

Every id below resolves to one response of CME Group's own
`trading-hours-by-product` service — the endpoint `cmegroup.com/trading-hours.html`
itself calls — for the `THBP-A` product set
`id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true`.
Archived responses were replayed through `https://web.archive.org/web/<timestamp>id_/`;
live responses were read through the public reader `https://r.jina.ai/`, because
cmegroup.com returns HTTP 403 to the retrieving machine. Paths are relative to
`exchange-hours-research/holidays/`. The table is the same one the routed families'
evidence files carry — this venue rests on no artifact they do not — and several ids
resolve to one artifact, because one service window answers two or three trade dates.
The research-store id of each artifact is in the `Derived from` column of the year
tables in the family files.

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
| `CME-SVC-2026-12-22` | 2026-12-22 .. 2026-12-24 | live retrieval 2026-09-12T04:30Z | T2 | `c8c0267da8cf171409ad8ca188082b3aa326e8d04a89d12503dcf9f57bf3b7ab` |
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

Row review: 2026-08-29 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.cmegroup.com/content/dam/cmegroup/education/modules/files/EQ240_EQ_for_AIT.pdf> — CME `EQ240` equity-index education module, the October-2009 product guide that supplies the complete audit-floor grid.
- <https://www.cmegroup.com/education/files/eq-trading-hours.pdf> — CME equity-index trading-hours sheet.
- <https://www.cmegroup.com/tools-information/lookups/advisories/clearing/Chadv12-423.html> — CME clearing advisory Chadv12-423, the 2012 trade-date boundary change.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20121022.html> — CME Globex notice 20121022, the 2012-11-18 revision's source.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20121015.html> — CME market-data advisory 20121015, corroborating the 2012 change.
- <https://www.cmegroup.com/notices/clearing/2019/06/Chadv19-182.pdf> — CME clearing advisory Chadv19-182.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html> — CME Globex notice 20101025, the 2010-11-15 weekday Pre-Open move.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150817.html> — CME Globex Notice #20150817 of 17 August 2015, the 2015-09-20 revision's source.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150914.html> — CME Globex Notice #20150914, the "Effective this Monday" repeat of the same article.
- <https://www.cmegroup.com/notices/electronic-trading/2021/06/20210621.html> — CME Globex notice 20210621, the 2021-06-27 halt removal.
- <https://www.cmegroup.com/market-regulation/rule-filings/2021/6/21-244R_2.pdf> — CME rule filing 21-244R, corroborating the halt removal.
- <https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf> — CME SER-8921, current-grid corroboration.
- <https://web.archive.org/web/20120503104328/http://www.cmegroup.com/trading_hours/equities-hours.html> — CME equities trading-hours page — capture 2012-05-03, Sunday Pre-Open still 16:15.
- <https://web.archive.org/web/20120616181609/http://www.cmegroup.com/trading_hours/equities-hours.html> — CME equities trading-hours page — capture 2012-06-16, Sunday Pre-Open already 16:00.
- <https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities> — CME trading-hours index — capture 2012-05-11.
- <https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html> — CME trading-hours index — capture 2012-05-28, Sunday Pre-Open 16:15 platform-wide.
- <https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2012-06-07, Sunday Pre-Open 16:00 platform-wide.
- <https://web.archive.org/web/20190820012118id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120521.html> — CME Globex Notice 2012-05-21 — read in full, silent on the Pre-Open.
- <https://web.archive.org/web/20190716070058id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120528.html> — CME Globex Notice 2012-05-28 — read in full, silent on the Pre-Open.
- <https://web.archive.org/web/20190720204402id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120604.html> — CME Globex Notice 2012-06-04 — read in full, silent on the Pre-Open.
- <https://web.archive.org/web/20120622070557id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120528.html> — CME Market Data Notice 2012-05-28 — read in full, silent on the Pre-Open.

Official origin of the four trading-hours captures: <http://www.cmegroup.com/trading_hours/>.

## Gaps and residual risks

- **order-entry** — the Sunday Pre-Open's move from 16:15 to 16:00 CT has no operator-stated effective day. The 2026-08-31 review narrowed the bracket to 2012-05-28..2012-06-07 from CME's own trading-hours captures and read both CME dated notice channels in full across that window without finding an announcement, so the dated profiles serve the sourced 16:15–17:00 CT intersection and withhold only the 16:00–16:15 CT quarter-hour. Closing condition: a CME document that states the new Sunday Pre-Open in session language on a day-level effective date. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES). Horizon 2012-05-03: below that capture the Sunday 16:15–17:00 CT queue is carried, not sourced.
- **residual risk** — the only Sunday inside the narrowed bracket is 2012-06-03; that is an observation about the bracket, not a source-stated effective day, so LAW-NO-FABRICATED-DATES keeps it out of the tables.
- **scope** — full-size `SP`, NKD, BTIC and TACO products are excluded from this compatibility default; NKD has its own key and module.
- **holidays** — this venue ships the intersection of the six CME families' holiday tables; see `## Holidays` above. Every date the intersection drops is named there, and the cross-wave agreement audit that memo §7 follow-up 10 asks for (#95) is still open: it closes with the last stage-2.2 family wave, when the same assertion can be re-run over 2010-2027 rather than over this window alone.

## Module narrative (moved from src/calendar/schedules/futures/us/cme_group.rs on 2026-09-12 UTC)

U.S.-grid CME and CBOT equity-index futures, including CBOT YM/MYM but not
CME Nikkei 225 Dollar (NKD), whose historical grid differs. CME's
October-2009 product guide supplies the complete grid at the audit floor:
Sunday 17:00–Monday 15:15, then Monday–Thursday 17:00–15:15 and
15:30–16:30, with 16:30–17:00 maintenance. The 2012 notice changed the
trade-date boundary and the post-halt slice to 15:30–16:15, including
Fridays, effective Sunday 2012-11-18. CME Globex then moved that close
15 minutes earlier to 16:00 CT effective Sunday 2015-09-20 for trade date
Monday 2015-09-21. CME then removed the 15:15-15:30 halt for the scoped
contracts effective Sunday 2021-06-27, producing the current continuous
17:00-16:00 ETH envelope around the unchanged 08:30-15:15 RTH.

The exact Monday-Thursday Pre-Open changed from 16:50 to 16:45 on
2010-11-15. Current primary material also establishes Sunday 16:00-17:00,
but calls it a long-term practice without giving the day when the earlier
16:15 start moved: primary documents updated 2012-05-03 still publish
Sunday 16:15, trading-hours pages crawled 2012-06-15/16 already publish
16:00, and no notice in between states the day. The fixed-current table
includes that sourced current queue. Dated profiles carry the sourced
Sunday 16:15–17:00 intersection from the January-2010 floor and withhold
only the disputed 16:00–16:15 quarter-hour rather than inventing its
cutover; their executable trading and weekday queues remain exact. Revisions are keyed by the local
session-opening day.
https://www.cmegroup.com/content/dam/cmegroup/education/modules/files/EQ240_EQ_for_AIT.pdf
https://www.cmegroup.com/education/files/eq-trading-hours.pdf
https://www.cmegroup.com/tools-information/lookups/advisories/clearing/Chadv12-423.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20121022.html
https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20121015.html
https://www.cmegroup.com/notices/clearing/2019/06/Chadv19-182.pdf
https://web.archive.org/web/20120503104328/http://www.cmegroup.com/trading_hours/equities-hours.html

2026-08-31 Sunday-queue review — bracket narrowed, notice channels negative.
Three archived captures of CME's own trading-hours pages, unused by the
earlier review, move the bracket from 2012-05-03..2012-06-15 down to
2012-05-28..2012-06-07. The move was platform-wide and simultaneous: on the
2012-05-28 capture the Sunday Pre-Open is 16:15 for E-mini S&P 500,
Eurodollar, 30-Year Interest Rate Swap, Euroyen TIBOR and (as "17:15 ET
(16:15 CT)") Gold, Silver, Light Sweet Crude and Henry Hub; on the
2012-06-07 capture every one of them reads 16:00. Weekday Pre-Opens are
unchanged across both captures, so this is a Sunday-only change.
CBOT grains are NOT part of it: the 2012-05-11 capture still shows the
pre-expansion 18:00-07:15/09:30-13:15 grain grid with a 16:15 Sunday
Pre-Open, and the 2012-05-28 capture shows the expanded 17:00-14:00 grid
with 16:00 — so grains moved at the separately sourced 2012-05-20
expansion (CME Globex Advisory #20120518), which the grains module already
dates.
Both of CME's dated notice channels were then read in full across the
narrowed window and none announces the change: CME Globex Notices of
2012-05-21, 2012-05-28 and 2012-06-04, and Market Data Notices of
2012-05-28, contain no occurrence of "Pre-Open", "trading hours", "16:00"
or "16:15". The change was therefore made without a dated operator notice,
which is why no cutover is encoded. (The only Sunday inside the narrowed
bracket is 2012-06-03; that is an observation about the bracket, not a
source-stated effective day, so LAW-NO-FABRICATED-DATES keeps it out of the
tables.) Official origin http://www.cmegroup.com/trading_hours/ delivered
via:
https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities
https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html
https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/
https://web.archive.org/web/20190820012118id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120521.html
https://web.archive.org/web/20190716070058id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120528.html
https://web.archive.org/web/20190720204402id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120604.html
https://web.archive.org/web/20120622070557id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120528.html
https://web.archive.org/web/20120616181609/http://www.cmegroup.com/trading_hours/equities-hours.html
The 2015-09-20 revision's original announcement, CME Globex Notice #20150817
of 17 August 2015: "Effective Monday, September 21, the daily CME Globex
maintenance period will begin 15 minutes earlier Monday through Thursday from
16:00 until 16:45 Central Time (CT). ... the closing times for the following
markets will now occur 15 minutes earlier Monday through Friday at 16:00 CT.
CME Equity / CBOT Equity / COMEX / NYMEX / DME. All other CME Globex markets
trading hours remain unchanged." The #20150914 repeat below carries the same
article with "Effective this Monday" wording.
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150817.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150914.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
https://www.cmegroup.com/notices/electronic-trading/2021/06/20210621.html
https://www.cmegroup.com/market-regulation/rule-filings/2021/6/21-244R_2.pdf
https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf

ORDER-ENTRY CLASSIFICATION. The evening phases the citations above name as
the "Pre-Open" (Monday-Thursday 16:50, later 16:45, to the 17:00 Globex open)
and the Sunday 16:00-17:00 queue are Globex pre-open states: the book accepts,
amends, and cancels orders, but the matching engine is not running and no
trade can print until the 17:00 open. They are therefore `order_entry`, not
`extended`. Everything that remains in the extended slices below is a
matching phase: the post-halt afternoon slice and the 17:00 electronic
session both print trades.

Pre-Open queues. No trade can match in any of these windows.
SUNDAY QUEUE, CARRIED BACK AS THE SOURCED INTERSECTION. CME's Sunday Pre-Open
only ever widened inside the modelled window: the audit-floor material pins it
at 16:15 and the verified-current value is 16:00, with the undated 2012 move
(bracketed 2012-05-28..2012-06-07) the only change between them. The
16:15-17:00 window is therefore order-entry under *every* sourced state, so
carrying it from the January-2010 floor asserts no cutover at all - it is the
intersection of the two regimes, not a guess at either. The undated change
adds only the 16:00-16:15 quarter-hour, which the knowledge-bound row supplies
from the repository review date onward. Previously these dated profiles
omitted the Sunday queue entirely, which under-reported order acceptance for
the whole 16:00-17:00 hour rather than only the disputed quarter-hour.
