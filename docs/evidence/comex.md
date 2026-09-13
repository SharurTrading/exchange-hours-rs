<!-- SPDX-License-Identifier: MIT-0 -->

# `comex` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`energy_metals.rs`](../../src/calendar/schedules/futures/us/energy_metals.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Compatibility default for the named COMEX metals/NYMEX energy family. Current matching and Pre-Open queues are primary-supported and the January-2010/2015 matching revisions are exact, but the Sunday queue's 16:15→16:00 onset day is unavailable: the 2026-08-31 review narrowed it to 2012-05-28..2012-06-07 with both CME notice channels silent across that window; dated profiles now serve the sourced Sunday 16:15–17:00 intersection from the January-2010 floor, leaving only the 16:00–16:15 quarter-hour undated. TAS/TAM/BTIC, options, and other clocks are excluded.

## Revision rows

- 2015-09-20 — T1 — CME Globex notice 20150907 — every COMEX and NYMEX close moves to 16:00 CT for trade date Monday 2015-09-21.
- 2026-08-22 — T1 — 2026-08-22 review: verified current, onset undated — knowledge-bound row widening the Sunday queue to the sourced current 16:00–17:00 CT Pre-Open.

## Holidays

**Coverage:** 2025-01-01 .. 2027-12-31 (inclusive venue-local trade dates in
`America/Chicago`). Tier: **T2** throughout. Inside the window a date with no row is
audited normal; outside it this table has no answer at all.

**This table is derived, not retrieved, and the intersection is total.** It is the
intersection of the families that route to `Exchange::Comex` — which is one family,
`globex_energy`, whose metals half is this venue's documented scope — so there is no
disagreement to withhold: the venue carries that family's table unchanged, all
thirty-six rows, and every id resolves to the same CME Group
`trading-hours-by-product` artifact `globex_energy`'s evidence file already records.
The per-family rows, with their event-date-to-trade-date conversions and the
`13:30 preopen` reading that turns CME's pre-open-only records into early closes,
are there.

**Why one family rather than two.** CME prints the metals and NYMEX energy products as one
product-group row on every date in this window — `CL` and `GC` carry identical event
lists — so `globex_energy` is one key and one table, and design memo D17's
intersection rule is never reached. `docs/evidence/globex_energy.md` records the same
fact from the retrieval side. A consumer that maps a COMEX product to this calendar
is therefore using a clock the crate claims for both halves at once; if the operator
ever publishes them apart, the two halves become two keys and this table splits with
them.

**No date is dropped.** There is no `unsourced` row and no declared gap behind one in
this window. The table is the family's own evidence at one remove, and nothing here
rests on a document the family module does not carry.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `no events published` | `CME-SVC-2024-12-31` | T2 | energy closed |
| 2025-01-20 | early close | `early close 13:30 CT` | `CME-SVC-2025-01-19` | T2 | energy early close 13:30 CT |
| 2025-02-17 | early close | `early close 13:30 CT` | `CME-SVC-2025-02-16` | T2 | energy early close 13:30 CT |
| 2025-04-18 | closed | `no events published` | `CME-SVC-2025-04-17` | T2 | energy closed |
| 2025-05-26 | early close | `early close 13:30 CT` | `CME-SVC-2025-05-25` | T2 | energy early close 13:30 CT |
| 2025-06-19 | early close | `early close 13:30 CT` | `CME-SVC-2025-06-18` | T2 | energy early close 13:30 CT |
| 2025-07-04 | early close | `early close 12:00 CT` | `CME-SVC-2025-07-03` | T2 | energy early close 12:00 CT |
| 2025-09-01 | early close | `early close 13:30 CT` | `CME-SVC-2025-08-31` | T2 | energy early close 13:30 CT |
| 2025-11-27 | early close | `early close 13:30 CT` | `CME-SVC-2025-11-26` | T2 | energy early close 13:30 CT |
| 2025-11-28 | early close | `early close 13:45 CT` | `CME-SVC-2025-11-26` | T2 | energy early close 13:45 CT |
| 2025-11-29 | closed | `no events published` | `CME-SVC-2025-11-26-SAT` | T2 | energy closed |
| 2025-12-24 | early close | `early close 12:45 CT` | `CME-SVC-2025-12-24` | T2 | energy early close 12:45 CT |
| 2025-12-25 | closed | `no events published` | `CME-SVC-2025-12-24` | T2 | energy closed |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `no events published` | `CME-SVC-2025-12-31` | T2 | energy closed |
| 2026-01-19 | early close | `early close 13:30 CT` | `CME-SVC-2026-01-18` | T2 | energy early close 13:30 CT |
| 2026-02-16 | early close | `early close 13:30 CT` | `CME-SVC-2026-02-15` | T2 | energy early close 13:30 CT |
| 2026-04-03 | closed | `no events published` | `CME-SVC-2026-04-01` | T2 | energy closed |
| 2026-05-25 | early close | `early close 13:30 CT` | `CME-SVC-2026-05-24` | T2 | energy early close 13:30 CT |
| 2026-06-19 | early close | `early close 12:00 CT` | `CME-SVC-2026-06-18` | T2 | energy early close 12:00 CT |
| 2026-07-03 | early close | `early close 12:00 CT` | `CME-SVC-2026-07-03` | T2 | energy early close 12:00 CT |
| 2026-09-07 | early close | `early close 13:30 CT` | `CME-SVC-2026-09-06` | T2 | energy early close 13:30 CT |
| 2026-11-26 | early close | `early close 13:30 CT` | `CME-SVC-2026-11-25` | T2 | energy early close 13:30 CT |
| 2026-11-27 | early close | `early close 13:45 CT` | `CME-SVC-2026-11-25` | T2 | energy early close 13:45 CT |
| 2026-12-24 | early close | `early close 12:45 CT` | `CME-SVC-2026-12-22` | T2 | energy early close 12:45 CT |
| 2026-12-25 | closed | `no events published` | `CME-SVC-2026-12-24` | T2 | energy closed |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `no events published` | `CME-SVC-2026-12-31` | T2 | energy closed |
| 2027-01-18 | early close | `early close 13:30 CT` | `CME-SVC-2027-01-17` | T2 | energy early close 13:30 CT |
| 2027-02-15 | early close | `early close 13:30 CT` | `CME-SVC-2027-02-14` | T2 | energy early close 13:30 CT |
| 2027-03-26 | closed | `no events published` | `CME-SVC-2027-03-25` | T2 | energy closed |
| 2027-05-31 | early close | `early close 13:30 CT` | `CME-SVC-2027-05-30` | T2 | energy early close 13:30 CT |
| 2027-06-18 | early close | `early close 12:00 CT` | `CME-SVC-2027-06-17` | T2 | energy early close 12:00 CT |
| 2027-07-05 | early close | `early close 13:30 CT` | `CME-SVC-2027-07-04` | T2 | energy early close 13:30 CT |
| 2027-09-06 | early close | `early close 13:30 CT` | `CME-SVC-2027-09-05` | T2 | energy early close 13:30 CT |
| 2027-11-25 | early close | `early close 13:30 CT` | `CME-SVC-2027-11-24` | T2 | energy early close 13:30 CT |
| 2027-11-26 | early close | `early close 13:45 CT` | `CME-SVC-2027-11-24` | T2 | energy early close 13:45 CT |
| 2027-12-24 | closed | `no events published` | `CME-SVC-2027-12-22` | T2 | energy closed |

**Gaps, 2025-2027.**

- **No intersection gap exists in this window.** Every date on which `globex_energy`
  states a row ships that row here, because the metals half and the NYMEX energy half agree
  on all of them. This is a positive statement, checked per date by
  `the_venue_table_is_the_intersection_of_its_families`.
- **The operator's own gaps are inherited, not restated.** The unrepresentable
  order-entry deviation on 2025-11-28, the 2025 windows that survive only in a
  pre-holiday capture, and the Saturday sessions after the Friday holidays are gaps in
  `docs/evidence/globex_energy.md`; they are gaps here too, and that file is where
  their closing conditions live.
- **The window is the family's window, not the venue's horizon.** Coverage starts
  2025-01-01 because that is where the family's table starts. The 2010-2024 blocks land
  in stage 2.2's waves, each of which extends this table over its own years; the
  cross-wave agreement audit (#95) closes with the last of them, and it is what would
  catch a future divergence between the two halves.
- **2028-01-01 ships no row.** CME's service publishes a 2027-12-30 .. 2028-01-02
  window and the family's table ends at 2027-12-31, so the venue's window ends there
  too. Extending it is stage 2.4's refresh, not a gap in this change.

**Interpretive steps, 2025-2027.**

- **The routing is read from production, not assumed.** `hours_for_exchange`'s
  `Exchange::Comex` arm resolves to
  `energy_metals_profile_at`, the same selector `globex_energy` resolves to; the test
  `the_venue_table_is_the_intersection_of_its_families` recomputes the table from that
  key's public `holiday_on` answers on every run.
- **The two venue tables are identical by construction, not by copying.** Both are
  derived from the one key, so a change to `globex_energy` reaches both or neither,
  and a divergence between the two venue tables cannot be introduced by an edit to one
  of them that the family table does not justify.
- **`comex` and `nymex` already carry a `monthly` ledger cadence**, so this change
  moves no cadence cell for them; it moves only their `Holidays` cell.

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

- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20090130.html> — CME Globex notice 20090130, the pre-floor 17:00–16:15 CT energy and metals grid.
- <https://www.cmegroup.com/trading/metals/files/MT-027_GoldFuturesVsETFCheatSheet_r3.pdf> — CME `MT-027` gold futures cheat sheet.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-regulation/SER-5391.html> — CME SER-5391.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html> — CME Globex notice 20101025, which observes the weekday 16:45 queue already in effect without dating its onset.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150907.html> — CME Globex notice 20150907, the 2015-09-20 revision's source.
- <https://www.cmegroup.com/markets/metals/precious/gold.contractSpecs.html> — CME gold futures contract specification, current grid.
- <https://www.cmegroup.com/markets/energy/crude-oil/light-sweet-crude.contractSpecs.html> — CME light sweet crude contract specification, current grid.
- <https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf> — CME SER-8921, current-grid corroboration.
- <https://web.archive.org/web/20120501182431/http://www.cmegroup.com/trading_hours/metals-hours.html> — CME metals trading-hours page — capture 2012-05-01, "17:15 ET (16:15 CT)". **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120616193920/http://www.cmegroup.com/trading_hours/metals-hours.html> — CME metals trading-hours page — capture 2012-06-16, "17:00 ET (16:00 CT)". **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities> — CME trading-hours index — capture 2012-05-11, the earliest artifact in the Sunday-queue intersection. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html> — CME trading-hours index — capture 2012-05-28, Sunday Pre-Open 16:15 platform-wide. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2012-06-07, Sunday Pre-Open 16:00 platform-wide. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20190820012118id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120521.html> — CME Globex Notice 2012-05-21 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20190716070058id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120528.html> — CME Globex Notice 2012-05-28 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20190720204402id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120604.html> — CME Globex Notice 2012-06-04 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120622070557id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120528.html> — CME Market Data Notice 2012-05-28 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.

Official origin of the trading-hours captures: <http://www.cmegroup.com/trading_hours/>.

## Gaps and residual risks

- **order-entry** — the Sunday Pre-Open's move from 16:15 to 16:00 CT has no operator-stated effective day. The 2026-08-31 review narrowed the bracket to 2012-05-28..2012-06-07 from CME's own trading-hours captures and read both CME dated notice channels in full across that window without finding an announcement, so the dated profiles serve the sourced 16:15–17:00 CT intersection and withhold only the 16:00–16:15 CT quarter-hour. Closing condition: a CME document that states the new Sunday Pre-Open in session language on a day-level effective date. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **residual risk** — the only Sunday inside the narrowed bracket is 2012-06-03; that is an observation about the bracket, not a source-stated effective day, so LAW-NO-FABRICATED-DATES keeps it out of the tables.
- **scope** — TAS/TAM/BTIC, options and other product clocks are excluded; they take their own keys when a consumer maps one.
- **holidays** — this venue ships the intersection of the families that route to it; see `## Holidays` above. The intersection is total in this window, so nothing is dropped, and the cross-wave agreement audit that memo §7 follow-up 10 asks for (#95) is still open: it closes with the last stage-2.2 family wave, when the same assertion can be re-run over 2010-2027 rather than over this window alone.

## Module narrative (moved from src/calendar/schedules/futures/us/energy_metals.rs on 2026-09-12 UTC)

This family covers the shared grid used by NYMEX CL/MCL/QM, NG/MNG/QG,
HO/RB/BZ, and PL/PA plus COMEX GC/MGC, SI/SIL, and HG/MHG. Platinum and
palladium are NYMEX products, not COMEX products. At the January-2010 audit
floor these families opened 17:00 CT and closed 16:15 CT. CME's 2015 Globex
notice moved every COMEX and NYMEX close to 16:00 CT for Monday 2015-09-21
while leaving opens unchanged, so a separate metals clock would duplicate
both the current grid and the in-scope history. Current CME material also
publishes Sunday 16:00-17:00 and Monday-Thursday 16:45-17:00 Pre-Open. Its
correction calls both queues a long-term practice without giving their
unconditional onset days. A 2010 notice observes that the weekday queue was
already 16:45 but likewise supplies no onset, and the Sunday queue's
16:15→16:00 move is only bracketed: metals hours pages crawled 2012-05-01
still publish "17:15 ET (16:15 CT)" while the 2012-06-16 crawl shows
"17:00 ET (16:00 CT)", with no notice stating the day. The fixed-current
table includes both sourced current queues; dated profiles retain matching
only.
The revision is keyed to Sunday 2015-09-20, the local opening day of that
Monday trade-date session, so a wrapped rule gives Monday the sourced close.
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20090130.html
https://www.cmegroup.com/trading/metals/files/MT-027_GoldFuturesVsETFCheatSheet_r3.pdf
https://www.cmegroup.com/tools-information/lookups/advisories/market-regulation/SER-5391.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150907.html
https://www.cmegroup.com/markets/metals/precious/gold.contractSpecs.html
https://www.cmegroup.com/markets/energy/crude-oil/light-sweet-crude.contractSpecs.html
https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf
https://web.archive.org/web/20120501182431/http://www.cmegroup.com/trading_hours/metals-hours.html
https://web.archive.org/web/20120616193920/http://www.cmegroup.com/trading_hours/metals-hours.html

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

DATED QUEUES, CARRIED BACK AS THE SOURCED INTERSECTION. These profiles
previously ran with no queue at all, which reported the venue as closed
through windows it was demonstrably accepting orders in. Both phases are now
carried from the January-2010 floor at their narrowest sourced values, so no
cutover is asserted:
  Sunday 16:15-17:00 - CME's trading-hours pages read "17:15 ET (16:15 CT)"
  for Light Sweet Crude, Henry Hub, Gold and Silver on the 2012-05-11 and
  2012-05-28 captures and "17:00 ET (16:00 CT)" on 2012-06-07. The queue only
  widened, so 16:15-17:00 holds under both regimes; the knowledge-bound row
  supplies the extra 16:00-16:15 quarter-hour.
  Monday-Thursday 16:45-17:00 - the 2010 notice cited above observes this
  queue was already in effect, and no primary source names an earlier value.
