<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_livestock` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`livestock.rs`](../../src/calendar/schedules/futures/us/livestock.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. CME Live Cattle, Feeder Cattle, and Lean Hog futures. Matching revisions in 2014/2016 and the 2020 08:00 Pre-Open start are exact. The 2016-05-30 Globex notice dates the 14:30-16:00 PCP onset to 2016-06-06; official trading-hours captures omit the PCP row between November 2016 and March 2020 with no removal notice, and the pre-2020 06:00 queue's own onset is unresolved, so those remain the open gaps. The 2026-08-31 review checked the contract-specification channel as a second route into that interval and found it silent as well: the Live Cattle specification captured 2016-09-25, 2016-12-04, 2017-09-23 and 2018-04-19 renders only ClearPort/Default 08:30-13:05 CT hours with no Globex Pre-Open or PCP row. The pre-2020 morning queue is no longer omitted, though: SER-8599R states the outgoing 06:00 value when it dates the move to 08:00, so 06:00–08:30 is carried across 2016-02-29..2020-05-31 — the span of the matching grid it runs into — and only its onset before that grid stays unresolved.

## Revision rows

- 2014-10-27 — T1 — CME SER-7194 — the evening sessions are removed from the around-the-clock grid.
- 2016-02-29 — T1 — CME SER-7591 — the current 08:30–13:05 CT weekday session is established for LE, GF and HE.
- 2016-06-06 — T1 — CME Globex notice 20160530 — a Monday–Friday 14:30–16:00 CT Post-Close order-entry period begins.
- 2020-05-31 — T1 — CME SER-8599R — the morning Pre-Open start moves from 06:00 to 08:00 CT for trade date Monday 2020-06-01.

## Holidays

**Coverage:** 2025-01-01 .. 2027-12-31 (inclusive venue-local trade dates in `America/Chicago`). Tier: **T2**
throughout — CME Group's own `trading-hours-by-product` service, the endpoint `cmegroup.com/trading-hours.html`
itself calls to render its per-asset-class Holiday Hours table, read as bytes and saved. Inside the window a date
with no row is audited and normal; outside it this table has no answer at all.

Rows are keyed by the crate's own venue-local trade date, never by CME's event date (design memo D1). For this
family the two coincide: the grid has run one Monday-Friday 08:30-13:05 CT regular session since 2016-02-29, with an
08:00-08:30 CT Pre-Open ahead of it and a 14:30-16:00 CT Post-Close behind it, and nothing wraps a local midnight.
CME's own printed `trade date` is quoted in the `Derived from` column as the corroboration for each conversion.

**Zone.** Verbatim from the operator page: "Trading hours are subject to change and are in U.S. Central Time unless
otherwise stated." This channel prints no Eastern column, so no ET value is asserted anywhere below; the `CT` token
after an em dash in an `Instant as printed` cell is the editorial application of that page-wide statement, never a
token CME printed in the cell. The backticked text before the em dash is the cell as printed.

**Event vocabulary**, verbatim from the same page: `preopen` — "Order Entry, modification, and cancel are allowed.
No order matching."; `open` — "Start of continuous trading phase. Order matching begins."; `pcp` (POST CLOSE -
PREOPEN) — "Allows GTC/GTD orders only placement, modification, and cancellation. No order matching."; `closed` —
"Final Close of the date. Day and GTD (current trade date) orders are eliminated."

**How a status became a row.** The retrieval's own vocabulary maps onto the crate's kinds as follows, and for this
family only three of its statuses ever occur.

| Retrieval status and note | Crate row |
|---|---|
| `closed` with `no events published` `[N1]` | `closed` |
| `early_close` with `close_instant` | `early close` at that instant |
| `normal` `[N8]` — the published event list and its trade dates equal the family's normal grid for that weekday | no row |

The block is 36 rows: 31 `closed` and 5 `early close`. There is no `late open`, no `late open and early close` and
no `unsourced` row in this window, and no row was withheld as an unrepresentable topology change. The rows and this
section were written on 2026-09-12 (UTC) from `exchange-hours-research/holidays/cme-2025-2027.json` as repaired that
day, against its latest verdict `cme-2025-2027.verify.json` (round 2). The verdict's four material findings are all
repaired in the result; only one of them touches Livestock, and it adds the sourced Saturday 2025-11-29 closure
recorded below. Nothing in the verdict disputes a Livestock instant or status.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `no events published` | `CME-SVC-2024-12-31` | T2 | eventDate 2025-01-01, CME publishes no Livestock trade date; research id D01 |
| 2025-01-20 | closed | `no events published` | `CME-SVC-2025-01-19` | T2 | eventDate 2025-01-20, CME publishes no Livestock trade date; research id D02 |
| 2025-02-17 | closed | `no events published` | `CME-SVC-2025-02-16` | T2 | eventDate 2025-02-17, CME publishes no Livestock trade date; research id D03 |
| 2025-04-18 | closed | `no events published` | `CME-SVC-2025-04-17` | T2 | eventDate 2025-04-18, CME publishes no Livestock trade date; research id D04 |
| 2025-05-26 | closed | `no events published` | `CME-SVC-2025-05-25` | T2 | eventDate 2025-05-26, CME publishes no Livestock trade date; research id D05 |
| 2025-06-19 | closed | `no events published` | `CME-SVC-2025-06-18` | T2 | eventDate 2025-06-19, CME publishes no Livestock trade date; research id D06 |
| 2025-07-04 | closed | `no events published` | `CME-SVC-2025-07-03` | T2 | eventDate 2025-07-04, CME publishes no Livestock trade date; research id D07 |
| 2025-09-01 | closed | `no events published` | `CME-SVC-2025-08-31` | T2 | eventDate 2025-09-01, CME publishes no Livestock trade date; research id D08 |
| 2025-11-27 | closed | `no events published` | `CME-SVC-2025-11-26` | T2 | eventDate 2025-11-27, CME publishes no Livestock trade date; research id D09 |
| 2025-11-28 | early close | `08:00 preopen; 08:30 open; 12:05 closed` — 12:05 CT | `CME-SVC-2025-11-26` | T2 | eventDate 2025-11-28, CME trade date 2025-11-28; research id D09 |
| 2025-11-29 | closed | `no events published` | `CME-SVC-2025-11-26-SAT` | T2 | eventDate 2025-11-29 Saturday, CME publishes no Livestock trade date; research id D65 |
| 2025-12-24 | early close | `08:00 preopen; 08:30 open; 12:15 closed` — 12:15 CT | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-24, CME trade date 2025-12-24; research id D10 |
| 2025-12-25 | closed | `no events published` | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-25, CME publishes no Livestock trade date; research id D10 |

**Gaps, 2025.**

- **order-entry, every row** — on a closed or shortened date CME publishes no `14:30 pcp` and no `16:00 closed`, but the crate's 14:30-16:00 CT Post-Close window already carries the **following** trade date, so neither a `Closed` row nor an `EarlyClose` row on the holiday can reach it. `DayPolicy`'s scalar vocabulary — the vocabulary a holiday row copies — has no order-entry boundary, so this is not representable and is recorded here rather than modelled. It changes no `is_open` answer, only `is_accepting_orders` / `is_order_entry_only` for 90 minutes. Fenced in both directions by `tests/futures_family_boundaries/holidays_globex_livestock.rs`, so the residue stays visible rather than becoming folklore. Closing condition: the design memo's §7 follow-up 8 block rows (#93), or an order-entry boundary on `DayPolicy`.
- **no post-finalisation statement, 2025-01-01 through 2025-09-01** — the eight rows `CME-SVC-2024-12-31` through `CME-SVC-2025-08-31` rest on a single archive capture of the service taken 2024-12-20T15:53:40Z, which is CME's published future rather than a post-holiday statement. CME prints on the same page: "This schedule is subject to change. Trading hours are usually finalized approximately two weeks prior to the holiday." The service's retention edge now falls between Labor Day 2025 and Thanksgiving 2025, so the channel itself cannot restate them: all eight windows were re-probed live on 2026-09-12 and return the products with empty schedules. Closing condition: a later archived call of `services/trading-hours-by-product` over one of those windows, or a CME notice restating the finalised Globex hours. Residual risk only — a slipped instant would be a schedule fix, not a fabricated date.
- **no T1 rendering** — from the 2025 calendar year CME publishes no per-holiday Globex hours PDF or XLS; the holiday hours *are* the interactive table on `cmegroup.com/trading-hours.html`, which renders client-side, so the archived HTML carries no table. Every row in this section is therefore T2, the operator's own trading-hours service read as bytes and saved. The T1 page was captured once, for Thanksgiving 2026, and its printed `Livestock` row matches the service's `LE` row event for event. Closing condition: a CME notice or advisory restating these dates per asset class.
- **no late open anywhere in 2025-2027** — CME publishes no delayed first open for this family in the window, so neither `HolidayKind::LateOpen` branch has a sourced instance here. This is an observation rather than a hole in the evidence: every published Livestock holiday is a full closure or an early final close, and the test walks the whole coverage window to assert it.
- **Columbus Day and Veterans Day** — CME publishes settlement and clearing advisories for them but no Globex trading schedule. Coverage here is contiguous, so those dates carry no row and therefore read as audited normal. That is a deliberate reading of CME's silence on days the exchange is known to trade, named here rather than left implicit.

**Interpretive steps, 2025.**

- **Event date to trade date, design memo D1.** The conversion is the identity for this family. Since 2016-02-29 the grid is one Monday-Friday 08:30-13:05 CT regular session with an 08:00-08:30 CT Pre-Open and a 14:30-16:00 CT Post-Close, all inside one local day; nothing wraps a midnight, so every occurrence's trade date is its own civil date, and CME's own printed `trade date` equals the event date on all five early-close rows. No eve record keys a row of its own, which is why this module is 36 rows rather than the ~70 event-date records CME publishes.
- **`Livestock (LE)` is the modelled line.** CME queries `LE` for the whole Livestock group on its own trading-hours page, and `globex_livestock` covers Live Cattle, Feeder Cattle and Lean Hogs. Where CME publishes the Lean Hog line (`HE`) separately — from Thanksgiving 2025 forward, in the `THBP-B` product set — it matches `LE` event for event: `raw/cme-2025-2027-repair/live/probeB_2025-11-26_2025-11-29.md` prints `HE` `08:00 preopen; 08:30 open; 12:05 closed` for trade date 2025-11-28, and `probeB_2025-12-24_2025-12-26.md` prints `12:15 closed` for trade date 2025-12-24. Those are corroboration, not the keying source, so they carry no crate document id. Lumber (`LBR`) and Dairy (`CSC`/`DC`) share several of CME's grouped rows but are separate families and are not modelled by this key.
- **Thanksgiving 2025 was re-sourced.** `CME-SVC-2025-11-26` and `CME-SVC-2025-11-26` quote the latest of 45 captures of that window, taken two months **after** the holiday. The superseded pre-holiday publication (`raw/cme-2025-2027/arc/thbp_2025-11-26_2025-11-28_20241220155340.json`) differs from it on eight product-dates, all on 2025-11-28 — and Livestock is not one of them: both publications print `08:00 preopen; 08:30 open; 12:05 closed` for `LE`. The finalised capture adds a `07:00 preopen; 07:30 open` pair for six other families; there is none on the Livestock line, so this family carries no part of that intraday-topology item.
- **Saturday 2025-11-29 is a sourced closure, not a gap.** CME's own 2025 Globex table states the Thanksgiving period as "27 - 29 November 2025" and no archived call covers the Saturday, but the live service does: queried for 2025-11-26 .. 2025-11-29 on 2026-09-12 it returns a 2025-11-29 schedule for all ten `THBP-A` products, every one of them empty. The family never trades a Saturday, so the row changes no answer; it ships because the operator answered, and the test asserts that neutrality rather than assuming it. This is the repair of the round-2 verdict's material discrepancy 3.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `no events published` | `CME-SVC-2025-12-31` | T2 | eventDate 2026-01-01, CME publishes no Livestock trade date; research id D11 |
| 2026-01-19 | closed | `no events published` | `CME-SVC-2026-01-18` | T2 | eventDate 2026-01-19, CME publishes no Livestock trade date; research id D12 |
| 2026-02-16 | closed | `no events published` | `CME-SVC-2026-02-15` | T2 | eventDate 2026-02-16, CME publishes no Livestock trade date; research id D13 |
| 2026-04-03 | closed | `no events published` | `CME-SVC-2026-04-01` | T2 | eventDate 2026-04-03, CME publishes no Livestock trade date; research id D14 |
| 2026-05-25 | closed | `no events published` | `CME-SVC-2026-05-24` | T2 | eventDate 2026-05-25, CME publishes no Livestock trade date; research id D15 |
| 2026-06-19 | closed | `no events published` | `CME-SVC-2026-06-18` | T2 | eventDate 2026-06-19, CME publishes no Livestock trade date; research id D17 |
| 2026-07-03 | closed | `no events published` | `CME-SVC-2026-07-03` | T2 | eventDate 2026-07-03, CME publishes no Livestock trade date; research id D19 |
| 2026-09-07 | closed | `no events published` | `CME-SVC-2026-09-06` | T2 | eventDate 2026-09-07, CME publishes no Livestock trade date; research id D20 |
| 2026-11-26 | closed | `no events published` | `CME-SVC-2026-11-25` | T2 | eventDate 2026-11-26, CME publishes no Livestock trade date; research id D22 |
| 2026-11-27 | early close | `08:00 preopen; 08:30 open; 12:05 closed` — 12:05 CT | `CME-SVC-2026-11-25` | T2 | eventDate 2026-11-27, CME trade date 2026-11-27; research id D22 |
| 2026-12-24 | early close | `08:00 preopen; 08:30 open; 12:05 closed` — 12:05 CT | `CME-SVC-2026-12-22` | T2 | eventDate 2026-12-24, CME trade date 2026-12-24; research id D24 |
| 2026-12-25 | closed | `no events published` | `CME-SVC-2026-12-24` | T2 | eventDate 2026-12-25, CME publishes no Livestock trade date; research id D26 |

**Gaps, 2026.**

- **order-entry, every row** — on a closed or shortened date CME publishes no `14:30 pcp` and no `16:00 closed`, but the crate's 14:30-16:00 CT Post-Close window already carries the **following** trade date, so neither a `Closed` row nor an `EarlyClose` row on the holiday can reach it. `DayPolicy`'s scalar vocabulary — the vocabulary a holiday row copies — has no order-entry boundary, so this is not representable and is recorded here rather than modelled. It changes no `is_open` answer, only `is_accepting_orders` / `is_order_entry_only` for 90 minutes. Fenced in both directions by `tests/futures_family_boundaries/holidays_globex_livestock.rs`, so the residue stays visible rather than becoming folklore. Closing condition: the design memo's §7 follow-up 8 block rows (#93), or an order-entry boundary on `DayPolicy`.
- **no T1 rendering** — from the 2025 calendar year CME publishes no per-holiday Globex hours PDF or XLS; the holiday hours *are* the interactive table on `cmegroup.com/trading-hours.html`, which renders client-side, so the archived HTML carries no table. Every row in this section is therefore T2, the operator's own trading-hours service read as bytes and saved. The T1 page was captured once, for Thanksgiving 2026, and its printed `Livestock` row matches the service's `LE` row event for event. Closing condition: a CME notice or advisory restating these dates per asset class.
- **Columbus Day and Veterans Day** — CME publishes settlement and clearing advisories for them but no Globex trading schedule. Coverage here is contiguous, so those dates carry no row and therefore read as audited normal. That is a deliberate reading of CME's silence on days the exchange is known to trade, named here rather than left implicit.

**Interpretive steps, 2026.**

- The two Thanksgiving-week rows come from one service window: `CME-SVC-2026-11-25` and `CME-SVC-2026-11-25` both resolve to the 2026-11-25 .. 2026-11-27 response. The T1 trading-hours page was captured for this holiday and its printed `Livestock` row matches the service's `LE` row event for event, which is what calibrates the whole T2 block against the operator's own rendering.
- **The two Christmas Eves differ, and are quoted separately.** `CME-SVC-2025-12-24` is `12:15 closed` and `CME-SVC-2026-12-22` is `12:05 closed`, each read from its own response. Neither is derived from the other, and the test asserts both instants independently so a copied row fails.
- `CME-SVC-2026-12-22` resolves to the 2026-12-22 .. 2026-12-24 window and `CME-SVC-2026-12-24` to the overlapping 2026-12-24 .. 2026-12-26 window; the two agree on 2026-12-24 event for event.
- Lean Hogs (`HE`) corroborates the first half of 2026 from the `THBP-B` probes `raw/cme-2025-2027-repair/live/probeB_2026-01-18_2026-01-20.md` and its siblings, and the second half from `raw/cme-2025-2027/live/extra/extra_2026-*.json`.

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `no events published` | `CME-SVC-2026-12-31` | T2 | eventDate 2027-01-01, CME publishes no Livestock trade date; research id D29 |
| 2027-01-18 | closed | `no events published` | `CME-SVC-2027-01-17` | T2 | eventDate 2027-01-18, CME publishes no Livestock trade date; research id D30 |
| 2027-02-15 | closed | `no events published` | `CME-SVC-2027-02-14` | T2 | eventDate 2027-02-15, CME publishes no Livestock trade date; research id D32 |
| 2027-03-26 | closed | `no events published` | `CME-SVC-2027-03-25` | T2 | eventDate 2027-03-26, CME publishes no Livestock trade date; research id D34 |
| 2027-05-31 | closed | `no events published` | `CME-SVC-2027-05-30` | T2 | eventDate 2027-05-31, CME publishes no Livestock trade date; research id D36 |
| 2027-06-18 | closed | `no events published` | `CME-SVC-2027-06-17` | T2 | eventDate 2027-06-18, CME publishes no Livestock trade date; research id D38 |
| 2027-07-05 | closed | `no events published` | `CME-SVC-2027-07-04` | T2 | eventDate 2027-07-05, CME publishes no Livestock trade date; research id D40 |
| 2027-09-06 | closed | `no events published` | `CME-SVC-2027-09-05` | T2 | eventDate 2027-09-06, CME publishes no Livestock trade date; research id D42 |
| 2027-11-25 | closed | `no events published` | `CME-SVC-2027-11-24` | T2 | eventDate 2027-11-25, CME publishes no Livestock trade date; research id D44 |
| 2027-11-26 | early close | `08:00 preopen; 08:30 open; 12:05 closed` — 12:05 CT | `CME-SVC-2027-11-24` | T2 | eventDate 2027-11-26, CME trade date 2027-11-26; research id D44 |
| 2027-12-24 | closed | `no events published` | `CME-SVC-2027-12-22` | T2 | eventDate 2027-12-24, CME publishes no Livestock trade date; research id D46 |

**Gaps, 2027.**

- **order-entry, every row** — on a closed or shortened date CME publishes no `14:30 pcp` and no `16:00 closed`, but the crate's 14:30-16:00 CT Post-Close window already carries the **following** trade date, so neither a `Closed` row nor an `EarlyClose` row on the holiday can reach it. `DayPolicy`'s scalar vocabulary — the vocabulary a holiday row copies — has no order-entry boundary, so this is not representable and is recorded here rather than modelled. It changes no `is_open` answer, only `is_accepting_orders` / `is_order_entry_only` for 90 minutes. Fenced in both directions by `tests/futures_family_boundaries/holidays_globex_livestock.rs`, so the residue stays visible rather than becoming folklore. Closing condition: the design memo's §7 follow-up 8 block rows (#93), or an order-entry boundary on `DayPolicy`.
- **no T1 rendering** — from the 2025 calendar year CME publishes no per-holiday Globex hours PDF or XLS; the holiday hours *are* the interactive table on `cmegroup.com/trading-hours.html`, which renders client-side, so the archived HTML carries no table. Every row in this section is therefore T2, the operator's own trading-hours service read as bytes and saved. The T1 page was captured once, for Thanksgiving 2026, and its printed `Livestock` row matches the service's `LE` row event for event. Closing condition: a CME notice or advisory restating these dates per asset class.
- **Columbus Day and Veterans Day** — CME publishes settlement and clearing advisories for them but no Globex trading schedule. Coverage here is contiguous, so those dates carry no row and therefore read as audited normal. That is a deliberate reading of CME's silence on days the exchange is known to trade, named here rather than left implicit.

**Interpretive steps, 2027.**

- **2027-12-23 is audited normal and ships no row.** CME's holiday date for Christmas 2027 is Thursday 2027-12-23, but the Livestock line that day prints `08:00 preopen; 08:30 open; 13:05 closed; 14:30 pcp; 16:00 closed` for trade date 2027-12-23 — the family's ordinary Thursday grid, instant for instant, against the reference week 2026-10-18 .. 2026-10-24. Under design memo D3 a row exists only where an answer changes, so the date is recorded here as audited and carries none. The Globex closure is the following day, 2027-12-24, which does ship one; 2027-12-25 is a Saturday.
- **2028-01-01 is outside coverage.** The operator's published future reaches it — CME's 2027-12-30 .. 2028-01-02 response prints no Livestock events for that Saturday — but the declared window ends at 2027-12-31, so no row is keyed to it and `holiday_on` answers `None` there. Extending the window is a later wave's decision, not a gap in this one.
- Lean Hogs (`HE`) corroborates every 2027 row from `raw/cme-2025-2027/live/extra/extra_2027-*.json`.

### Documents

Every id resolves to one response of CME's `trading-hours-by-product` service for the `THBP-A` product set
`id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true`, whose `LE` line is the
Livestock family. Archived responses were replayed through `https://web.archive.org/web/<timestamp>id_/`; live
responses were read through the public reader `https://r.jina.ai/`, because cmegroup.com returns HTTP 403 to the
retrieving machine. Paths are relative to `exchange-hours-research/holidays/`. Several ids resolve to one artifact,
because one service window answers two or three trade dates; the research-store id of each artifact is in the
`Derived from` column of the year tables above.

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

- <https://www.cmegroup.com/media-room/press-releases/2007/3/07/cme_to_offer_around-the-clocktradingofcommodityproductsoncmeglob.html> — CME press release of 2007-03-07, the around-the-clock launch that establishes the Monday 09:05 CT weekly open and the 16:00–17:00 CT daily halts.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-data/Q2008-215.html> — CME market-data advisory Q2008-215, the pre-floor move of the Friday close to 13:55 CT.
- <https://www.cmegroup.com/tools-information/lookups/advisories/ser/files/SER-7194.pdf> — CME SER-7194, the 2014-10-27 revision's source.
- <https://www.cmegroup.com/market-regulation/files/14-408.pdf> — CME rule filing 14-408, the 2014 reduction report confirming the complete old grid.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html> — CME Globex notice 20101025, the generic afternoon-queue notice that does not enumerate livestock.
- <https://www.cmegroup.com/tools-information/lookups/advisories/ser/files/SER-7591.pdf> — CME SER-7591, the 2016-02-29 revision's source.
- <https://www.cmegroup.com/notices/electronic-trading/2016/05/20160530.html> — CME Globex notice of 30 May 2016, the 2016-06-06 Post-Close onset.
- <https://web.archive.org/web/20160605123512/http://www.cmegroup.com:80/notices/electronic-trading/2016/05/20160530.html> — the same notice — capture 2016-06-05.
- <https://www.cmegroup.com/notices/electronic-trading/2020/05/20200511.html> — CME Globex notice 20200511, corroborating the 2020 Pre-Open move.
- <https://www.cmegroup.com/notices/ser/2020/05/SER-8599R.pdf> — CME SER-8599R, the 2020-05-31 revision's source and the statement of the outgoing 06:00 CT value.
- <https://www.cmegroup.com/market-regulation/rule-filings/2020/5/20-232.pdf> — CME rule filing 20-232.
- <https://www.cmegroup.com/trading-hours/files/memorial-day-2023.pdf> — CME Memorial Day 2023 trading-hours sheet, current-grid corroboration.
- <https://www.cmegroup.com/markets/agriculture/livestock/live-cattle.contractSpecs.html> — CME Live Cattle futures contract specification, the operator's own specification page and the channel the **2026-08-31 targeted review** checked as a second route into the 2016-11..2020-03 interval. Read live at that review, and through archived captures of the same page dated **2016-09-25, 2016-12-04, 2017-09-23 and 2018-04-19**, each of which renders only "CME ClearPort" and "Default" hours (Monday–Friday 08:30–13:05 CT) with no CME Globex Pre-Open or PCP row. Those dates are later than this row's review date and govern for this source. The archive replay URLs for the four captures are not recorded in this repository; re-verification starts from the live page above through the `US-CME-GROUP` entry point.
- <https://www.cmegroup.com/education/lessons/live-cattle-product-overview> — CME Live Cattle product overview.

## Gaps and residual risks

- **order-entry** — official CME trading-hours captures omit the 14:30–16:00 CT PCP row between November 2016 and March 2020 with no removal notice. The 2026-08-31 review checked the contract-specification channel as a second route into that interval and found it silent as well: the Live Cattle specification captured 2016-09-25, 2016-12-04, 2017-09-23 and 2018-04-19 renders only ClearPort and Default 08:30–13:05 CT hours with no Globex Pre-Open or PCP row. The omission is treated as a published-table gap rather than an operator-stated removal, so the sourced 2016-06-06 onset stands. Closing condition: a CME document that either removes or restates the PCP in session language on a day-level effective date. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **order-entry** — the pre-2020 06:00 CT morning queue's own onset is unresolved. SER-8599R states the outgoing 06:00 value when it dates the move to 08:00, so 06:00–08:30 is carried across 2016-02-29..2020-05-31 — the span of the matching grid it runs into — and is deliberately not carried further back, because before 2016-02-29 the family ran the old around-the-clock grid with no 08:30 open for a morning queue to precede.
- **residual risk** — a generic 2010 CME Globex queue notice does not enumerate livestock, so it is not used to invent a family-specific afternoon queue in the old around-the-clock grid.

## Module narrative (moved from src/calendar/schedules/futures/us/livestock.rs on 2026-09-12 UTC)

CME's 2007 launch announcement establishes the Monday 09:05 CT weekly open
and 16:00-17:00 daily halts; Q2008-215 moved the Friday close to 13:55 before
the audit floor, and the 2014 reduction report confirms that complete old
grid. SER-7194 removed the evening sessions effective Monday 2014-10-27.
SER-7591 then set the current 08:30-13:05 CT weekday session for LE, GF, and
HE effective Monday 2016-02-29. CME's 30 May 2016 Globex notice implemented
a Post-Close state — GTC/GTD order entry, modification, and cancellation for
the next trade date with "No matching ... during the Post-Close" — Monday
through Friday 14:30-16:00 CT for the same LE, GF, and HE families,
effective Monday 2016-06-06. Official trading-hours captures omit the PCP
row between November 2016 and March 2020 without any removal notice, so the
omission is treated as a published-table gap rather than an operator-stated
removal and the sourced onset stands. CME moved the Pre-Open start from
06:00 to 08:00 effective Sunday 2020-05-31 for trade date Monday 2020-06-01.
2026-08-31 review: the contract-specification channel was checked as a second
route into the 2016-11..2020-03 interval and is silent too — the Live Cattle
specification captured 2016-09-25, 2016-12-04, 2017-09-23 and 2018-04-19
renders only "CME ClearPort" and "Default" hours (Monday-Friday 08:30-13:05
CT) with no CME Globex Pre-Open or PCP row at all. Both the trading-hours and
the specification channels therefore fail to carry the PCP through that
interval, which corroborates the omission below rather than resolving it.
The pre-2020 06:00 queue is now carried across 2016-02-29..2020-05-31 (see the
note beside its rule set below); its onset before that grid is still
unresolved, so the older around-the-clock profiles keep no queue. A generic 2010 Globex
queue notice does not enumerate livestock, so it is not used to invent a
family-specific afternoon queue in the old around-the-clock grid.
https://www.cmegroup.com/media-room/press-releases/2007/3/07/cme_to_offer_around-the-clocktradingofcommodityproductsoncmeglob.html
https://www.cmegroup.com/tools-information/lookups/advisories/market-data/Q2008-215.html
https://www.cmegroup.com/tools-information/lookups/advisories/ser/files/SER-7194.pdf
https://www.cmegroup.com/market-regulation/files/14-408.pdf
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
https://www.cmegroup.com/tools-information/lookups/advisories/ser/files/SER-7591.pdf
https://www.cmegroup.com/notices/electronic-trading/2016/05/20160530.html
https://web.archive.org/web/20160605123512/http://www.cmegroup.com:80/notices/electronic-trading/2016/05/20160530.html
https://www.cmegroup.com/notices/electronic-trading/2020/05/20200511.html
https://www.cmegroup.com/notices/ser/2020/05/SER-8599R.pdf
https://www.cmegroup.com/market-regulation/rule-filings/2020/5/20-232.pdf
https://www.cmegroup.com/trading-hours/files/memorial-day-2023.pdf
https://www.cmegroup.com/education/lessons/live-cattle-product-overview

ORDER-ENTRY CLASSIFICATION. Both phases modelled after 2016-02-29 are
non-matching. The comment above names 08:00-08:30 as the "Pre-Open" (its
start moved from 06:00 on 2020-05-31) which queues orders until the 08:30
regular open, and 14:30-16:00 as PCP, the post-close order-entry period that
follows the 13:05 close. Neither can print a trade, so the family has no
tradeable extended session at all: `extended` is empty and both phases are
`order_entry`.

PRE-2020 MORNING QUEUE, CARRIED BACK TO THE MATCHING GRID IT BELONGS TO. The
2020 notice dates the move of the morning Pre-Open start "from 06:00 to
08:00" on 2020-05-31, so it states the outgoing 06:00 value the same way
SER-6465 states CME's outgoing equity-index close. No primary source names a
cutover between 2016-02-29 - when SER-7591 established the 08:30 open this
queue runs into - and 2020-05-31, so 06:00-08:30 is carried across that
interval rather than omitted. It is deliberately NOT carried further back:
before 2016-02-29 the family ran the old around-the-clock grid with no 08:30
open for a morning queue to precede, and the generic 2010 Globex queue notice
does not enumerate livestock.
