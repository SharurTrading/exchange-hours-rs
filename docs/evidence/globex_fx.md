<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_fx` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`fx.rs`](../../src/calendar/schedules/futures/us/fx.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Standard-grid CME FX futures only; eFix, BTIC, TAS, options, and separately specified products are excluded. Matching and the 2010 weekday-queue revision are exact; the current Sunday Pre-Open is sourced but its 16:15→16:00 cutover day is unavailable — bracketed to 2012-05-28..2012-06-07 by CME's own trading-hours captures, with both CME notice channels read in full across that window and silent on it. Dated profiles now carry the sourced Sunday 16:15–17:00 intersection from the January-2010 floor, so only the 16:00–16:15 quarter-hour remains withheld — so the dated selector omits that phase.

## Revision rows

- 2010-11-15 — T1 — CME Globex notice 20101025 — Monday–Thursday Pre-Open moves from 16:50 to 16:45 CT.
- 2026-08-22 — T1 — 2026-08-22 review: verified current, onset undated — knowledge-bound row widening the Sunday queue to the sourced current 16:00–17:00 CT Pre-Open.

## Evidence documents
Every id below resolves to one saved artifact behind this file's holiday rows.
The 2025-2027 ids are responses of CME's own `trading-hours-by-product` service
at tier T2, and they are the only ids the shipped rows cite: the pre-floor eras
left with Stage 5 of the release plan (#117), and the artifacts behind their rows
stay in the research store. Each id's row resolves it to the URL it was read at — an Internet Archive
raw replay for a saved capture, the operator's own endpoint for a live retrieval — and to
the capture time in UTC, the tier and the sha256, as the design memo's section 3.2
requires. The byte counts and, for a bundle member, the artifact's path inside the bundle
are in the research store's `holidays/raw/` indexes.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `CME-SVC-2024-12-31` | 2024-12-31 .. 2025-01-02 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-12-31&toEventDate=2025-01-02> | archive capture 2024-12-20T15:53:40Z | T2 | `375c70eecd19c5c6204ecb408d1b3210a9da4c9a03b85ef1c7dbbcde1397ab63` |
| `CME-SVC-2025-01-19` | 2025-01-19 .. 2025-01-21 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-01-19&toEventDate=2025-01-21> | archive capture 2024-12-20T15:53:40Z | T2 | `4f2ab56af14e7b3a6978e7fa6db8e2cfc63a82d06428cd88844f0e5bcf534f40` |
| `CME-SVC-2025-02-16` | 2025-02-16 .. 2025-02-18 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-02-16&toEventDate=2025-02-18> | archive capture 2024-12-20T15:53:40Z | T2 | `5bec2ca6b4999a534e4d9818035aaa18ec8626b6c912cf7e3d2c57015536f2fa` |
| `CME-SVC-2025-04-17` | 2025-04-17 .. 2025-04-19 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-04-17&toEventDate=2025-04-19> | archive capture 2024-12-20T15:53:40Z | T2 | `865a1d4f08102e00151bd87ab2b8e8a7720e9203a17aaaba24627ade3ed26e74` |
| `CME-SVC-2025-05-25` | 2025-05-25 .. 2025-05-27 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-05-25&toEventDate=2025-05-27> | archive capture 2024-12-20T15:53:40Z | T2 | `5f42869879c826f5949b79236aabb3d26d74e7565d92d7cc5e8784c63973210b` |
| `CME-SVC-2025-06-18` | 2025-06-18 .. 2025-06-20 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-06-18&toEventDate=2025-06-20> | archive capture 2024-12-20T15:53:40Z | T2 | `a572706907175776255261103b393493ebdf5a8106ec5374d129145bdf89105e` |
| `CME-SVC-2025-08-31` | 2025-08-31 .. 2025-09-02 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-08-31&toEventDate=2025-09-02> | archive capture 2024-12-20T15:53:40Z | T2 | `e075762ed34a86048d94900e10edba10d95b3d766052908ffbb5133f6b64bab0` |
| `CME-SVC-2025-07-03` | 2025-07-03 .. 2025-07-05 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-07-03&toEventDate=2025-07-05> | archive capture 2024-12-20T15:53:40Z | T2 | `b80cd4bfed0ae72865bfacc1936e107eb8febfcc94b37fcce1d05505c659147b` |
| `CME-SVC-2025-11-26` | 2025-11-26 .. 2025-11-28 | <https://web.archive.org/web/20260129012309id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-11-26&toEventDate=2025-11-28> | archive capture 2026-01-29T01:23:09Z | T2 | `6c4c598791058dd9a11aff0ddb072c761a436c6d1054b891def74c6935f020f1` |
| `CME-SVC-2025-11-26-SAT` | 2025-11-26 .. 2025-11-29 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-11-26&toEventDate=2025-11-29> | live retrieval 2026-09-12T08:55:12Z | T2 | `2e9f34f20085de3ccbdff1dc29cb7463bcff93713ef0c550740d6f15e0635ab7` |
| `CME-SVC-2025-12-24` | 2025-12-24 .. 2025-12-26 | <https://web.archive.org/web/20260129012159id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-12-24&toEventDate=2025-12-26> | archive capture 2026-01-29T01:21:59Z | T2 | `322a2be989b67f5f4cc0ec12fd63a393383d574badd4aacc87a0c9637533d386` |
| `CME-SVC-2025-12-31` | 2025-12-31 .. 2026-01-02 | <https://web.archive.org/web/20260619114105id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-12-31&toEventDate=2026-01-02> | archive capture 2026-06-19T11:41:05Z | T2 | `0ed61f8328eda4746265cc8e197f10cd53aec06c2b393927bab27c913993d314` |
| `CME-SVC-2026-04-01` | 2026-04-01 .. 2026-04-03 | <https://web.archive.org/web/20260619114118id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-04-01&toEventDate=2026-04-03> | archive capture 2026-06-19T11:41:18Z | T2 | `54bcc271e9ba9737a99a2fe608e658de0c657075284d050fbfec4fe1aee2a2a5` |
| `CME-SVC-2026-06-18` | 2026-06-18 .. 2026-06-20 | <https://web.archive.org/web/20260619113404id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-06-18&toEventDate=2026-06-20> | archive capture 2026-06-19T11:34:04Z | T2 | `97fd5da371309f4486a8fb49ff2105c6c1c2396939ab7c76f1a2a1097b6f015c` |
| `CME-SVC-2026-07-03` | 2026-07-03 .. 2026-07-05 | <https://web.archive.org/web/20260619114108id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-07-03&toEventDate=2026-07-05> | archive capture 2026-06-19T11:41:08Z | T2 | `4b89a026358e998277f9c1ff7e095e5d4e625cdc45115fd141dc92201833155b` |
| `CME-SVC-2026-11-25` | 2026-11-25 .. 2026-11-27 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-11-25&toEventDate=2026-11-27> | live retrieval 2026-09-12T04:30Z | T2 | `e1f35a5623b3c5d15e7468b2cb4119e587411a9714f920605dab11bf688756d1` |
| `CME-SVC-2026-12-22` | 2026-12-22 .. 2026-12-24 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-12-22&toEventDate=2026-12-24> | live retrieval 2026-09-12T04:30Z | T2 | `c8c0267da8cf171409ad8ca188082b3aa326e8d04a89d12503dcf9f57bf3b7ab` |
| `CME-SVC-2026-12-24` | 2026-12-24 .. 2026-12-26 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-12-24&toEventDate=2026-12-26> | live retrieval 2026-09-12T04:30Z | T2 | `bdc1fe831adb794bcf8aeb7e99baf6af2009d1ff9969d0a48b18b2ebc2e1e829` |
| `CME-SVC-2026-12-31` | 2026-12-31 .. 2027-01-02 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-12-31&toEventDate=2027-01-02> | live retrieval 2026-09-12T04:30Z | T2 | `7162652821c16f1bd05e3ec533bd5b82af03833c7186a64c7734b0b650364dcd` |
| `CME-SVC-2026-01-18` | 2026-01-18 .. 2026-01-20 | <https://web/20260619114105id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-01-18&toEventDate=2026-01-20> | archive capture 2026-06-19T11:41:05Z | T2 | `5e3ff08bdc7d07474b96b8dc8c18ed0d5e48d12dc4bcad81a5f68820cb2aa89e` |
| `CME-SVC-2026-02-15` | 2026-02-15 .. 2026-02-17 | <https://web/20260619114105id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-02-15&toEventDate=2026-02-17> | archive capture 2026-06-19T11:41:05Z | T2 | `5dd507dd959d0029e838ec88b1bdb63c32444ea36a121de002606f5d7b206e2f` |
| `CME-SVC-2026-05-24` | 2026-05-24 .. 2026-05-26 | <https://web/20260619114105id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-05-24&toEventDate=2026-05-26> | archive capture 2026-06-19T11:41:05Z | T2 | `f7e30d204ce2cbe08e5f486ded6518f623369159f3a36161288a4708288314da` |
| `CME-SVC-2026-09-06` | 2026-09-06 .. 2026-09-08 | <https://web/20260830142930id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-09-06&toEventDate=2026-09-08> | archive capture 2026-08-30T14:29:30Z | T2 | `01fb78ffaac10eac466fed53674214222f05aed518b9d93a4b42cf8957147bca` |
| `CME-SVC-2027-03-25` | 2027-03-25 .. 2027-03-27 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-03-25&toEventDate=2027-03-27> | live retrieval 2026-09-12T04:30Z | T2 | `9bd7225d440e00139f30892f3914c9b38beb8bf29d4272039b6cd8f2de926880` |
| `CME-SVC-2027-06-17` | 2027-06-17 .. 2027-06-19 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-06-17&toEventDate=2027-06-19> | live retrieval 2026-09-12T04:30Z | T2 | `60c9a2f5106d61039a616986b463cd852861ee4d3b91b11fac8badfa1b97b01c` |
| `CME-SVC-2027-11-24` | 2027-11-24 .. 2027-11-26 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-11-24&toEventDate=2027-11-26> | live retrieval 2026-09-12T04:30Z | T2 | `6aa7c0fd701a02480dabeac1fbae1a69b56e77643a29e3a9b2223c56e822ce9f` |
| `CME-SVC-2027-12-22` | 2027-12-22 .. 2027-12-25 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-12-22&toEventDate=2027-12-25> | live retrieval 2026-09-12T04:30Z | T2 | `5edc4dd588a32faa74f841494c10a3df48692dca29843c3581bad3e18c30fef9` |
| `CME-SVC-2027-01-17` | 2027-01-17 .. 2027-01-19 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-01-17&toEventDate=2027-01-19> | live retrieval 2026-09-12T04:30Z via `https://r.jina.ai/` | T2 | `7155c4b7ee8b299b3033eb3daf002b6ceecf0fbd53f6f98a7036048022275743` |
| `CME-SVC-2027-02-14` | 2027-02-14 .. 2027-02-16 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-02-14&toEventDate=2027-02-16> | live retrieval 2026-09-12T04:30Z via `https://r.jina.ai/` | T2 | `41f5aa8cde3879f8b10490386c134a294a0f1509edde2022a22ec3ffcaed1183` |
| `CME-SVC-2027-05-30` | 2027-05-30 .. 2027-06-01 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-05-30&toEventDate=2027-06-01> | live retrieval 2026-09-12T04:30Z via `https://r.jina.ai/` | T2 | `1283649724c30163fa08ba7ab02d1230fa9a7dd0613b8b4b3d96cd1d9dc4febd` |
| `CME-SVC-2027-07-04` | 2027-07-04 .. 2027-07-06 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-07-04&toEventDate=2027-07-06> | live retrieval 2026-09-12T04:30Z via `https://r.jina.ai/` | T2 | `93ff8232886435c94be682bf968aa30749011cdf8dadeb7d2425a3b0b9e0bf71` |
| `CME-SVC-2027-09-05` | 2027-09-05 .. 2027-09-07 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-09-05&toEventDate=2027-09-07> | live retrieval 2026-09-12T04:30Z via `https://r.jina.ai/` | T2 | `aa08a3bd102812928e69cf1ea4c8a84f738eaa5d14f967acee7d2571e74aedb9` |
| `CME-SVC-2026-06-21` | 2026-06-21 .. 2026-06-23 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-06-21&toEventDate=2026-06-23> | live retrieval 2026-09-25T08:44:59Z via `https://r.jina.ai/` | T2 | `91534cfd3ae56920ef744734216d2f5944cb9057c483d4b12f1550d91d91bcaf` |
| `CME-SVC-2026-07-05` | 2026-07-05 .. 2026-07-07 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-07-05&toEventDate=2026-07-07> | live retrieval 2026-09-26T02:55:11Z via `https://r.jina.ai/` | T2 | `f6e1900b4971eda307f63d1c905f94741c268350fe4623e916580065c4c194cb` |
| `CME-SVC-2027-06-20` | 2027-06-20 .. 2027-06-22 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-06-20&toEventDate=2027-06-22> | live retrieval 2026-09-25T08:44:59Z via `https://r.jina.ai/` | T2 | `9ab30e85bb6947803f35369ed29e24cc98c20c86498bf48f87ec6515d8431107` |

## Holidays

**Coverage:** 2025-01-01..2027-12-31 (inclusive venue-local trade dates).
Tier: **T2** throughout — the operator's own `trading-hours-by-product` service,
read as bytes and saved, since those dates have no published schedule this crate
could read. Inside the window a date with no row is audited normal; outside it
this table has no answer at all.

**One audited era, opening at the support floor.** The table declares one
coverage window: `2025-01-01..2027-12-31`. The eras before 2025-01-01 left with
Stage 5 of the release plan (#117): their rows and their windows are recorded in
Git history, and the artifacts behind them stay in the research store. Dates
below the floor are refused by the coverage contract, so no removed row could
answer anything the crate still asks. `HolidayCoverage::windows()` lists the
one, and `contains` answers per date.
### 2025-2027 (T2)

**Zone.** Verbatim from the operator's page: "Trading hours are subject to
change and are in U.S. Central Time unless otherwise stated." CME prints no ET
column in this channel, so every instant below is quoted in CT exactly as
printed and **no ET value is asserted**.

**The conversion (design memo D1).** This family's grid is one wrapping
Sunday-to-Thursday 17:00→16:00 CT matching block, so a trading day always opens
on the previous local evening. CME keys its records by *event date* and prints
its own trade date beside each event; the crate keys by *trade date*. An eve
record is therefore evidence for the holiday's row, not a row of its own —
except where the eve carries an early close of its own trade date, which is
what Christmas Eve does. The **Derived from** column records the operator event
dates and the operator's own printed trade date for every row, so the
conversion can be re-checked.

**Grid this was compared against.** CME's own reference week 2026-10-18 ..
2026-10-24, pulled from the same service: Sun `16:00 preopen`, `17:00 open`;
Mon–Thu `16:00 closed`, `16:45 preopen`, `17:00 open`; Fri `16:00 closed`;
Sat none.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `16:00 preopen; 17:00 open` — both events carry CME trade date 2025-01-02, so no session belongs to trade date 2025-01-01 | `CME-SVC-2024-12-31` | T2 | eventDate 2024-12-31 (`16:00 closed`, CME trade date 2024-12-31, no evening re-open) and eventDate 2025-01-01, CME trade date 2025-01-02 |
| 2025-01-21 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2025-01-19 and on eventDate 2025-01-20, both carrying CME trade date 2025-01-21; `16:00 closed` on eventDate 2025-01-21, CME trade date 2025-01-21 | `CME-SVC-2025-01-19` | T2 | Martin Luther King Day publishes no final close for its own trade date, so the Sunday-17:00-to-Tuesday-16:00 CT span carries this trade date. Unlike the ordinary weekday, the holiday's Pre-Open is `16:00` for this family, not `16:45` |
| 2025-02-18 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2025-02-16 and on eventDate 2025-02-17, both carrying CME trade date 2025-02-18; `16:00 closed` on eventDate 2025-02-18, CME trade date 2025-02-18 | `CME-SVC-2025-02-16` | T2 | Presidents Day; as 2025-01-21 |
| 2025-04-18 | closed | `no events published` | `CME-SVC-2025-04-17` | T2 | eventDate 2025-04-17 (`16:00 closed`, CME trade date 2025-04-17, no evening re-open) and eventDate 2025-04-18 |
| 2025-05-27 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2025-05-25 and on eventDate 2025-05-26, both carrying CME trade date 2025-05-27; `16:00 closed` on eventDate 2025-05-27, CME trade date 2025-05-27 | `CME-SVC-2025-05-25` | T2 | Memorial Day; as 2025-01-21 |
| 2025-06-20 | replacement blocks | `16:45 preopen; 17:00 open` on eventDate 2025-06-18 and `16:00 preopen; 17:00 open` on eventDate 2025-06-19, both carrying CME trade date 2025-06-20; `16:00 closed` on eventDate 2025-06-20, CME trade date 2025-06-20 | `CME-SVC-2025-06-18` | T2 | Juneteenth falls on the Thursday, so the merged span opens Wednesday evening. Its `-2` day is therefore an ordinary weekday and the row carries the family's weekday `16:45` Pre-Open there, where the Sunday-opening merges carry `16:00` |
| 2025-07-04 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2025-07-03` | T2 | eventDate 2025-07-04, CME trade date 2025-07-04 |
| 2025-09-02 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2025-08-31 and on eventDate 2025-09-01, both carrying CME trade date 2025-09-02; `16:00 closed` on eventDate 2025-09-02, CME trade date 2025-09-02 | `CME-SVC-2025-08-31` | T2 | Labor Day; as 2025-01-21 |
| 2025-11-28 | replacement blocks | `16:45 preopen; 17:00 open` on eventDate 2025-11-26 and `16:00 preopen; 17:00 open` on eventDate 2025-11-27, both carrying CME trade date 2025-11-28; `07:00 preopen; 07:30 open; 13:45 closed` on eventDate 2025-11-28, all three carrying CME trade date 2025-11-28 | `CME-SVC-2025-11-26` | T2 | Thanksgiving Day publishes no final close of its own, so this trade date owns the span from Wednesday evening, and its own close is the operator's `13:45` CT. The `-2` day is a Wednesday, so that queue is the weekday `16:45`. The Friday pair is the operator's own Pre-Open — `preopen` is "Order Entry, modification, and cancel are allowed. No order matching." — so `07:00-07:30` CT is order entry and `07:30` is where matching resumes, which is what the row's blocks state |
| 2025-11-29 | closed | `no events published` | `CME-SVC-2025-11-26-SAT` | T2 | eventDate 2025-11-29 |
| 2025-12-24 | early close | `12:45 closed` — 12:45 CT | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-24, CME trade date 2025-12-24 |
| 2025-12-25 | closed | `16:00 preopen; 17:00 open` — both events carry CME trade date 2025-12-26 | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-25, CME trade date 2025-12-26 |

**Interpretive steps, 2025.**

- **2025-01-01 and 2025-12-25 are closures, not late opens.** CME publishes a
  16:00 CT pre-open and a 17:00 CT open on each of those dates, but both events
  already carry the *following* trade date. The crate's own Tuesday-evening
  (resp. Wednesday-evening) leg is the one whose trade date is the holiday, and
  `Closed` removes exactly that leg; the holiday-evening leg, whose trade date
  is the next day, is untouched and opens at its normal 17:00 CT.
- **2025-04-18 needs no eve row.** `16:00 closed` on Thursday 2025-04-17 is the
  family's ordinary final close for its own trade date. What is missing that
  evening is the 17:00 CT leg, and `Closed(2025-04-18)` already removes it —
  the eve record is evidence for the Good Friday row, not a row of its own.
- **2025-12-24 keeps its own trade date.** CME prints `12:45 closed (trade date
  2025-12-24)`, so this is an early final close of the Christmas-Eve trading
  day, which opened Tuesday 2025-12-23 at 17:00 CT. The absent evening re-open
  is carried by `Closed(2025-12-25)`.
- **2025-11-29 (Saturday) is a sourced closure, not an unchecked date.** The
  live service publishes a 2025-11-29 schedule for all ten headline products
  with no events. The family's normal week has no Saturday session, so the row
  changes no answer; it ships so that the venue-level intersection tables of a
  later wave see the same audited dates in every family.

**Gaps, 2025.**

- **Trade-date merge — 2025-01-20, 2025-02-17, 2025-05-26, 2025-06-19,
  2025-09-01, 2025-11-27 — now stated, no longer a gap.** On these Monday and
  Thursday holidays CME publishes `16:00 preopen; 17:00 open` for this family
  instead of `16:00 closed; 16:45 preopen; 17:00 open`, with both events carrying
  the next business day's trade date. Matching still stops at 16:00 CT and still
  resumes at 17:00 CT, so no executable phase moves and no `is_open` answer
  changes; what changes is that the holiday has no final close of its own and the
  whole Sunday-evening-through-Tuesday-16:00 CT (resp.
  Wednesday-evening-through-Friday-16:00 CT) span carries one trade date. The
  block-row vocabulary of #93 states it, so the six spans now ship as
  replacement rows keyed to the operator's trade date — 2025-01-21, 2025-02-18,
  2025-05-27, 2025-06-20, 2025-09-02 and 2025-11-28 — and a `Closed` row is not
  used, because it would delete a full evening and day of trading CME in fact
  ran. The 2026 and 2027 instances of the same shape now ship as replacement
  rows too (2026-01-20, 2026-02-17, 2026-05-26, 2026-09-08, 2026-11-27,
  2027-01-19, 2027-02-16, 2027-06-01, 2027-07-06, 2027-09-07 and 2027-11-26), so
  #140 no longer tracks an unstated merge for this family. What is still open on
  it is the order-entry-only class — 2025-01-02, 2025-12-26 and 2026-01-02,
  whose queue opens at 16:00 CT rather than the ordinary 16:45 and whose trade
  date is already correct, so a merge-template row would rewrite a right answer
  — the `07:00 preopen; 07:30 open` pair this family's 2025-11-28 row does not
  split (the intraday-topology gap below), and `globex_nikkei_225_dollar`,
  which states only twelve of the seventeen.
- **Order-entry window — the same six dates, plus 2025-01-01 and 2025-12-25.**
  The Globex pre-open opens at 16:00 CT instead of the normal 16:45 CT. The
  table shares `DayPolicy`'s vocabulary, which has no order-entry boundary, so
  this is not representable. It changes no `is_open` answer, only
  `is_accepting_orders` and `is_order_entry_only`, for 45 minutes.
- **The 2025-11-28 morning Pre-Open is served, and it is order entry.** CME's
  finalised publication prints `07:00 preopen; 07:30 open; 13:45 closed` on
  eventDate 2025-11-28, all three carrying CME trade date 2025-11-28. CME's own
  event vocabulary defines `preopen` as "Order Entry, modification, and cancel
  are allowed. No order matching." and `open` as "Start of continuous trading
  phase. Order matching begins.", so `07:00-07:30` CT is a queue and matching
  resumes at `07:30` — the trade date's matching runs in two pieces, not one.
  This family has no separate regular open, which is why the pair sits inside
  the overnight run rather than at a regular-open handoff, but the queue is
  still a queue. The row's `MERGED_SESSION_EARLY_CLOSE_BLOCKS_2025_11_28` states
  exactly that; until this correction the whole morning was one `extended` block
  and `is_open` answered `true` in the operator's queue. It is the only date in
  the 2025-2027 window with this shape: the 2026 and 2027 Thanksgiving Fridays
  print the `13:45 closed` line alone and keep the four-block static. The
  superseded 2024-12-20 publication does not print the pair and agrees on the
  13:45 CT instant.
- **Residual risk — 2025-01-01, 2025-04-18 and 2025-07-04.** These three rows
  rest on the single Wayback capture 2024-12-20T15:53:40Z of the service, i.e.
  CME's published future as of that date rather than a post-holiday statement;
  CME's own page says "This schedule is subject to change. Trading hours are
  usually finalized approximately two weeks prior to the holiday." A fresh CDX
  enumeration of the endpoint (688 rows, 2026-09-12) confirms no later capture
  of those windows exists, and the live service's retention edge falls between
  Labor Day 2025 and Thanksgiving 2025, so the channel cannot restate them. The
  rows are internally consistent with the 2026 and 2027 rows for the same
  holidays, which are sourced from later captures and from live retrieval.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `16:00 preopen; 17:00 open` — both events carry CME trade date 2026-01-02 | `CME-SVC-2025-12-31` | T2 | eventDate 2025-12-31 (`16:00 closed`, CME trade date 2025-12-31, no evening re-open) and eventDate 2026-01-01, CME trade date 2026-01-02 |
| 2026-01-20 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2026-01-18 and `16:00 preopen; 17:00 open` on eventDate 2026-01-19, both carrying CME trade date 2026-01-20; `16:00 closed` on eventDate 2026-01-20, CME trade date 2026-01-20 | `CME-SVC-2026-01-18` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it through 2026-01-20 16:00 CT carries this trade date. The holiday's own Pre-Open is `16:00` for this family, not the ordinary weekday `16:45` |
| 2026-02-17 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2026-02-15 and `16:00 preopen; 17:00 open` on eventDate 2026-02-16, both carrying CME trade date 2026-02-17; `16:00 closed` on eventDate 2026-02-17, CME trade date 2026-02-17 | `CME-SVC-2026-02-15` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it through 2026-02-17 16:00 CT carries this trade date. The holiday's own Pre-Open is `16:00` for this family, not the ordinary weekday `16:45` |
| 2026-04-03 | early close | `10:15 closed` — 10:15 CT | `CME-SVC-2026-04-01` | T2 | eventDate 2026-04-03, CME trade date 2026-04-03 |
| 2026-05-26 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2026-05-24 and `16:00 preopen; 17:00 open` on eventDate 2026-05-25, both carrying CME trade date 2026-05-26; `16:00 closed` on eventDate 2026-05-26, CME trade date 2026-05-26 | `CME-SVC-2026-05-24` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it through 2026-05-26 16:00 CT carries this trade date. The holiday's own Pre-Open is `16:00` for this family, not the ordinary weekday `16:45` |
| 2026-06-19 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2026-06-18` | T2 | eventDate 2026-06-19, CME trade date 2026-06-22 |
| 2026-06-22 | replacement blocks | `16:45 preopen; 17:00 open /TD 2026-06-22` on eventDate 2026-06-18 and `12:00 closed /TD 2026-06-22` on eventDate 2026-06-19 (`CME-SVC-2026-06-18`); `05:00 open; 17:00 closed /TD 2026-06-22` on eventDate 2026-06-20 (`CME-SVC-2026-06-18`); `16:00 preopen; 17:00 open /TD 2026-06-22` on eventDate 2026-06-21 and `16:00 closed /TD 2026-06-22` on eventDate 2026-06-22 (`CME-SVC-2026-06-21`) | `CME-SVC-2026-06-18` | T2 | the complete crate trade date: every one of its five phases carries CME trade date 2026-06-22 — the Thursday Pre-Open queue and the session the Friday noon close ends, the Saturday session, the Sunday Pre-Open and the Sunday-17:00-to-Monday-16:00 session. The first window ends on the Saturday and prints no Sunday entry at all, so the row reads two documents |
| 2026-07-03 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2026-07-03` | T2 | eventDate 2026-07-03, CME trade date 2026-07-06 |
| 2026-07-06 | replacement blocks | `16:45 preopen; 17:00 open /TD 2026-07-06` on eventDate 2026-07-02 and `12:00 closed /TD 2026-07-06` on eventDate 2026-07-03; `05:00 open; 17:00 closed /TD 2026-07-06` on eventDate 2026-07-04 and `16:00 preopen; 17:00 open /TD 2026-07-06` on eventDate 2026-07-05 (`CME-SVC-2026-07-03`); `16:00 closed /TD 2026-07-06` on eventDate 2026-07-06 (`CME-SVC-2026-07-05`) | `CME-SVC-2026-07-03` | T2 | the complete crate trade date, all five phases carrying CME trade date 2026-07-06: the Thursday Pre-Open queue and the session the Friday noon close ends, the Saturday session, the Sunday Pre-Open and the Sunday-17:00-to-Monday-16:00 session. The first window runs through its own Sunday and prints four of the five phases; the Monday final close is read from the second, which is why this row cites two documents |
| 2026-09-08 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2026-09-06 and `16:00 preopen; 17:00 open` on eventDate 2026-09-07, both carrying CME trade date 2026-09-08; `16:00 closed` on eventDate 2026-09-08, CME trade date 2026-09-08 | `CME-SVC-2026-09-06` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it through 2026-09-08 16:00 CT carries this trade date. The holiday's own Pre-Open is `16:00` for this family, not the ordinary weekday `16:45` |
| 2026-11-27 | replacement blocks | `16:45 preopen; 17:00 open` on eventDate 2026-11-25 and `16:00 preopen; 17:00 open` on eventDate 2026-11-26, both carrying CME trade date 2026-11-27; `13:45 closed` on eventDate 2026-11-27, CME trade date 2026-11-27 | `CME-SVC-2026-11-25` | T2 | Thanksgiving Day publishes no final close of its own, so this trade date owns the span from Wednesday evening and its own close is the operator's `13:45` CT |
| 2026-12-24 | early close | `12:45 closed` — 12:45 CT | `CME-SVC-2026-12-22` | T2 | eventDate 2026-12-24, CME trade date 2026-12-24 |
| 2026-12-25 | closed | `no events published` | `CME-SVC-2026-12-24` | T2 | eventDate 2026-12-24 (early close above, no evening re-open) and eventDate 2026-12-25 |

**Interpretive steps, 2026.**

- **Good Friday 2026 is CME's own flagged exception.** Verbatim from the T1
  operator page D50: "Due to the US Employment Situation Release on April 3,
  2026, CME Group Equities, FX, Cryptocurrency and Interest Rate products will
  have unique Closes and Settlements for trade date April 3rd." D50 is T1
  corroboration that this family traded that morning and states no instants;
  the 10:15 CT close comes only from the T2 service document, which is what the
  row cites.
- **2026-06-19 and 2026-07-03 keep the crate's own trade date.** CME prints
  `12:00 closed` on each of those Fridays but attaches its own trade date
  2026-06-22 / 2026-07-06 to the event. Under design memo D1 the row is keyed
  by the crate's venue-local trade date, which is the Friday itself; the early
  close therefore lands on the correct civil instant and clips the leg that
  opened Thursday at 17:00 CT. The operator's differing trade-date label is a
  declared gap below, not a reason to withhold the instant.
- **2026-12-24 keeps its own trade date**, exactly as 2025-12-24 does, and its
  missing evening re-open is carried by `Closed(2026-12-25)`.

**Gaps, 2026.**

- **Trade-date merge — 2026-01-19, 2026-02-16, 2026-05-25, 2026-09-07,
  2026-11-26 — now stated, no longer a gap.** The same shape as the 2025 entry:
  the holiday publishes no final close, so the span carries the next business
  day's trade date. The rows ship on 2026-01-20, 2026-02-17, 2026-05-26,
  2026-09-08 and 2026-11-27, the last of those ending at the operator's
  day-after-Thanksgiving `13:45` CT close.
- **Trade-date attribution — 2026-06-19 and 2026-07-03: resolved 2026-09-26 UTC,
  the crate now agrees.** The operator assigns the shortened day to trade date
  2026-06-22 / 2026-07-06 and the crate previously assigned it to the Friday.
  The composite rows now state the Thursday-evening leg that ends at the Friday
  noon close, so the crate's label matches the operator's on both dates.
- **Saturday sessions — 2026-06-20 and 2026-07-04: resolved 2026-09-25 UTC by
  Stage 4 (#116), rows ship.** CME publishes `05:00 open; 17:00 closed`, both
  carrying the following Monday's trade date, on a grid whose normal week has no
  Saturday session at all; `late_open_ssm` can only push an existing occurrence
  later and cannot create one (design memo §1.5 / D7), so the scalar layer never
  stated them. The block vocabulary Stage 3 shipped (#93) supplies the form, and
  trade dates 2026-06-22 and 2026-07-06 now state the **complete crate trade
  date**: the Saturday session from `CME-SVC-2026-06-18` / `CME-SVC-2026-07-03`,
  and the Sunday Pre-Open and the Sunday-17:00-to-Monday-16:00 session from
  `CME-SVC-2026-06-21` (2026-06-22 only — the 2026-07-06 window prints its own
  Sunday). Saturday 2026-04-04 after Good Friday 2026 carries no such session,
  checked against the same service.
- **Order-entry window — 2026-01-19, 2026-02-16, 2026-05-25, 2026-09-07,
  2026-11-26, 2026-01-01.** Pre-open at 16:00 CT rather than 16:45 CT, as in
  2025.

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `no events published` | `CME-SVC-2026-12-31` | T2 | eventDate 2026-12-31 (`16:00 closed`, CME trade date 2026-12-31, no evening re-open) and eventDate 2027-01-01 |
| 2027-01-19 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2027-01-17 and `16:00 preopen; 17:00 open` on eventDate 2027-01-18, both carrying CME trade date 2027-01-19; `16:00 closed` on eventDate 2027-01-19, CME trade date 2027-01-19 | `CME-SVC-2027-01-17` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it through 2027-01-19 16:00 CT carries this trade date. The holiday's own Pre-Open is `16:00` for this family, not the ordinary weekday `16:45` |
| 2027-02-16 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2027-02-14 and `16:00 preopen; 17:00 open` on eventDate 2027-02-15, both carrying CME trade date 2027-02-16; `16:00 closed` on eventDate 2027-02-16, CME trade date 2027-02-16 | `CME-SVC-2027-02-14` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it through 2027-02-16 16:00 CT carries this trade date. The holiday's own Pre-Open is `16:00` for this family, not the ordinary weekday `16:45` |
| 2027-03-26 | closed | `no events published` | `CME-SVC-2027-03-25` | T2 | eventDate 2027-03-25 (`16:00 closed`, CME trade date 2027-03-25, no evening re-open) and eventDate 2027-03-26 |
| 2027-06-01 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2027-05-30 and `16:00 preopen; 17:00 open` on eventDate 2027-05-31, both carrying CME trade date 2027-06-01; `16:00 closed` on eventDate 2027-06-01, CME trade date 2027-06-01 | `CME-SVC-2027-05-30` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it through 2027-06-01 16:00 CT carries this trade date. The holiday's own Pre-Open is `16:00` for this family, not the ordinary weekday `16:45` |
| 2027-06-18 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2027-06-17` | T2 | eventDate 2027-06-18, CME trade date 2027-06-21 |
| 2027-06-21 | replacement blocks | `16:45 preopen; 17:00 open /TD 2027-06-21` on eventDate 2027-06-17 and `12:00 closed /TD 2027-06-21` on eventDate 2027-06-18 (`CME-SVC-2027-06-17`); `05:00 open; 17:00 closed /TD 2027-06-21` on eventDate 2027-06-19 (`CME-SVC-2027-06-17`); `16:00 preopen; 17:00 open /TD 2027-06-21` on eventDate 2027-06-20 and `16:00 closed /TD 2027-06-21` on eventDate 2027-06-21 (`CME-SVC-2027-06-20`) | `CME-SVC-2027-06-17` | T2 | as 2026-06-22: the Thursday leg and the Saturday from the first window, the Sunday legs from the second, which the Saturday window does not itself print |
| 2027-07-06 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2027-07-04 and `16:00 preopen; 17:00 open` on eventDate 2027-07-05, both carrying CME trade date 2027-07-06; `16:00 closed` on eventDate 2027-07-06, CME trade date 2027-07-06 | `CME-SVC-2027-07-04` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it through 2027-07-06 16:00 CT carries this trade date. The holiday's own Pre-Open is `16:00` for this family, not the ordinary weekday `16:45` |
| 2027-09-07 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2027-09-05 and `16:00 preopen; 17:00 open` on eventDate 2027-09-06, both carrying CME trade date 2027-09-07; `16:00 closed` on eventDate 2027-09-07, CME trade date 2027-09-07 | `CME-SVC-2027-09-05` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it through 2027-09-07 16:00 CT carries this trade date. The holiday's own Pre-Open is `16:00` for this family, not the ordinary weekday `16:45` |
| 2027-11-26 | replacement blocks | `16:45 preopen; 17:00 open` on eventDate 2027-11-24 and `16:00 preopen; 17:00 open` on eventDate 2027-11-25, both carrying CME trade date 2027-11-26; `13:45 closed` on eventDate 2027-11-26, CME trade date 2027-11-26 | `CME-SVC-2027-11-24` | T2 | Thanksgiving Day publishes no final close of its own, so this trade date owns the span from Wednesday evening and its own close is the operator's `13:45` CT |
| 2027-12-24 | closed | `no events published` | `CME-SVC-2027-12-22` | T2 | eventDate 2027-12-23 (`16:00 closed`, CME trade date 2027-12-23, no evening re-open) and eventDate 2027-12-24 |

**Interpretive steps, 2027.**

- **Christmas 2027 has no early close for this family.** CME's holiday date is
  Thursday 2027-12-23 and Globex is closed Friday 2027-12-24, with 25 December
  falling on a Saturday. On 2027-12-23 the family publishes the ordinary
  `16:00 closed` for its own trade date and no evening re-open, so that date is
  normal and the missing leg is carried by `Closed(2027-12-24)`.
- **Independence Day 2027 has no row.** The observed holiday is Monday
  2027-07-05, and CME publishes the Monday-holiday shape for this family
  (`16:00 preopen; 17:00 open`, trade date 2027-07-06), which moves no
  executable phase. See the trade-date-merge gap below.
- **2027-06-18 keeps the crate's own trade date**, exactly as 2026-06-19 does.
- **2028-01-01 is outside coverage.** CME's record for it (`no events
  published`, a Saturday) was read with the 2027 year-end window and is the
  ordinary Saturday answer for this family; it ships no row and the coverage
  window stops at 2027-12-31.

**Gaps, 2027.**

- **Trade-date merge — 2027-01-18, 2027-02-15, 2027-05-31, 2027-07-05,
  2027-09-06, 2027-11-25 — now stated, no longer a gap.** Same shape as the 2025
  entry. The rows ship on 2027-01-19, 2027-02-16, 2027-06-01, 2027-07-06,
  2027-09-07 and 2027-11-26, the last ending at the `13:45` CT close.
- **Trade-date attribution — 2027-06-18: resolved 2026-09-26 UTC.** As
  2026-06-19, the composite row now carries the Thursday-evening leg.
- **Saturday session — 2027-06-19: resolved 2026-09-25 UTC by Stage 4 (#116),
  row ships.** `05:00 open; 17:00 closed`, carrying trade date 2027-06-21, on a
  week with no Saturday session. Same shape as the two 2026 Saturdays, and now a
  row on the same terms: the Saturday from `CME-SVC-2027-06-17` and the Sunday
  legs from `CME-SVC-2027-06-20`, which the Saturday window does not print.
- **Order-entry window — 2027-01-18, 2027-02-15, 2027-05-31, 2027-07-05,
  2027-09-06, 2027-11-25.** Pre-open at 16:00 CT rather than 16:45 CT.

### All years

**Gaps, all years.**

- **No T1 rendering, 2025-2027.** Every row is T2. CME's trading-hours page
  renders only the next upcoming holiday by default and its holiday selector
  could not be driven from the URL, so no T1 per-asset-class rendering could be
  captured for these years; one T1 capture (Thanksgiving 2026) was used to
  validate that the service rows group exactly as the page prints them, and
  D50 corroborates Good Friday 2026 without stating instants. Closing
  condition: a captured T1 rendering of the per-asset-class holiday table.
- **Columbus Day and Veterans Day are not CME Globex holidays.** They appear
  nowhere in CME's own Globex holiday list; CME publishes settlement and
  clearing advisories for them and Globex trades a normal session. Coverage
  here is contiguous, so those dates read as audited normal, which is stated
  rather than left implicit.
- **Scope.** These rows are the standard-grid CME FX futures holiday calendar
  only. eFix, BTIC, TAS, options and separately specified products are excluded
  from this key and from these rows; CME prints TAS/TAM holiday instants in the
  same page's free-text notes and none of them is a family session.
- **Late opens.** This family ships none in the coverage window: CME never
  reopens standard-grid FX later than its normal 17:00 CT. Both branches of the
  late-open disambiguation are therefore untested *by this family's rows*; they
  are exercised by the engine's own suite.

## Sources

Row review: 2026-08-29 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20081229.html> — CME Globex notice 20081229.
- <https://www.cmegroup.com/trading/fx/files/FX248-2010_FX_Product_Guide_and_Calendar.pdf> — CME `FX248` 2010 FX product guide and calendar, which publishes the 17:00–16:00 CT matching grid at the audit floor.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html> — CME Globex notice 20101025, the 2010-11-15 revision's source.
- <https://www.cmegroup.com/trading/fx/fx-report/files/q1-2018-cme-fx-products.pdf> — CME FX products report, Q1 2018.
- <https://www.cmegroup.com/trading/fx/files/emfx-brochure-q3-2020.pdf> — CME emerging-market FX brochure, Q3 2020.
- <https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf> — CME SER-8921, current-grid corroboration.
- <https://www.cmegroup.com/articles/faqs/frequently-asked-questions-cme-fx-futures-calendar-spreads.html> — CME FX calendar-spread FAQ.
- <https://web.archive.org/web/20120503103452/http://www.cmegroup.com/trading_hours/fx-hours.html> — CME FX trading-hours page — capture 2012-05-03, Sunday Pre-Open still 16:15. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120616190153/http://www.cmegroup.com/trading_hours/fx-hours.html> — CME FX trading-hours page — capture 2012-06-16, Sunday Pre-Open already 16:00. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities> — CME trading-hours index — capture 2012-05-11. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html> — CME trading-hours index — capture 2012-05-28, Sunday Pre-Open 16:15 platform-wide. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2012-06-07, Sunday Pre-Open 16:00 platform-wide. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20190820012118id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120521.html> — CME Globex Notice 2012-05-21 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20190716070058id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120528.html> — CME Globex Notice 2012-05-28 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20190720204402id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120604.html> — CME Globex Notice 2012-06-04 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.
- <https://web.archive.org/web/20120622070557id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120528.html> — CME Market Data Notice 2012-05-28 — read in full, silent on the Pre-Open. **Read at the 2026-08-31 targeted Sunday-queue review**, which is later than this row's review date and governs for this source.

Official origin of the trading-hours captures: <http://www.cmegroup.com/trading_hours/>.

## Gaps and residual risks

- **order-entry** — **the crate holds no admissible artifact whose own scope covers an ordinary week inside the claimed interval**, so the 16:00–16:15 CT quarter-hour is withheld. The 2012 changeover day is also unstated — the bracket is 2012-05-28..2012-06-07 — but that is **not** the operative reason: no sourced state has printed 16:15 since 2012-05-28, because Globex notice 20120402 dates the old value (2012-04-15, in the venue's own market-state language) and notice 20121112 prints the new one, and the charter says in terms that "a later observation alone does not prove the intervening period complete". The 2026-08-31 review narrowed the bracket to 2012-05-28..2012-06-07 from CME's own trading-hours captures and read both CME dated notice channels in full across that window without finding an announcement, so the dated profiles serve the sourced 16:15–17:00 CT intersection and withhold only the 16:00–16:15 CT quarter-hour. **Search record**, so the next attempt does not repeat it: 41 of the 42 Globex notice date-filenames for 2012 were retrieved and text-scanned, together with all 187 real CME 2012 CFTC rule filings, the market-data notice channel (zero occurrences of "pre-open" or "16:15" in it) and the three in-bracket Federal Register notices — no effective day in any of them; CME's own Globex Initiative Calendar for June 2012, the month of the change, carries no pre-open or trading-hours entry at all; the Wayback archive holds 703 captures of the operator's trading-hours service covering 29 distinct Sundays, **every one a holiday eve**, so it can never carry a normal-week capture; and Save Page Now on the 2025-01-05..07 window returned `"hasEvents": false`, which is the channel's retention limit rather than the operator's word. **Closing conditions**, best first: (a) a written reply from CME's own desk giving the Sunday Pre-Open time in force for the week of 2025-01-05, or the effective date of the 2012 change — T1 in session language, and it closes this under either reading; (b) a T1 or T2 artifact covering an ordinary week at or near the 2025 floor, from a channel that still retains it and checked for `hasEvents: true`; (c) a May/June 2012 Global Command Center one-pager of the class the 2013 grain notice proves exists. Do not key 2012-06-03: the only Sunday inside the bracket is an inference from the bracket, not an operator statement. Tracked as #79. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES). Horizon 2012-05-03: below that capture the Sunday 16:15–17:00 CT queue is carried, not sourced.
- **special sessions — resolved 2026-09-26 UTC by Stage 4 (#116) and Stage 5; the declaration is gone.** This family used to declare a whole-domain `#93` gap, because CME publishes sessions the scalar vocabulary could not state. Every one of them now ships as a row: the Saturday sessions of 2026-06-20 and 2026-07-04 as replacement blocks (see the 2026 section), and the merged trade dates of a Monday or Thursday holiday as the complete trade date of the span they label (see the 2025, 2026 and 2027 sections). The declaration in `schedules/sourcing.rs` is removed with them, so this family now declares only the quarter-hour above. Verified across the whole published surface: a trade-date walk from the 2025 floor to the operator's published future agrees with the printed `tradingDate` on every date the family answers, with no date left unstated.
- **residual risk** — the only Sunday inside the narrowed bracket is 2012-06-03; that is an observation about the bracket, not a source-stated effective day, so LAW-NO-FABRICATED-DATES keeps it out of the tables.
- **scope** — standard-grid CME FX futures only; eFix, BTIC, TAS, options and separately specified products are excluded.

## Module narrative (moved from src/calendar/schedules/futures/us/fx.rs on 2026-09-12 UTC)

CME's 2010 guide publishes the 17:00-16:00 matching grid for its standard FX
futures. This family is not a promise for eFix, BTIC, TAS, options, or any
product whose own specification publishes a different grid. The exact
Monday-Thursday Pre-Open changed from 16:50 to 16:45 on 2010-11-15. Current
primary material publishes Sunday 16:00-17:00, but calls it a long-term
practice without stating the day on which the earlier queue moved; primary
documents updated 2012-05-03 still publish Sunday 16:15 while pages crawled
2012-06-15/16 already publish 16:00, and no notice in between states the
day. The fixed-current profile includes that exact current phase; dated
profiles carry the sourced Sunday 16:15–17:00 intersection from the
January-2010 floor and withhold only the disputed 16:00–16:15 quarter-hour.
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20081229.html
https://www.cmegroup.com/trading/fx/files/FX248-2010_FX_Product_Guide_and_Calendar.pdf
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
https://www.cmegroup.com/trading/fx/fx-report/files/q1-2018-cme-fx-products.pdf
https://www.cmegroup.com/trading/fx/files/emfx-brochure-q3-2020.pdf
https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf
https://www.cmegroup.com/articles/faqs/frequently-asked-questions-cme-fx-futures-calendar-spreads.html
https://web.archive.org/web/20120503103452/http://www.cmegroup.com/trading_hours/fx-hours.html
https://web.archive.org/web/20120616190153/http://www.cmegroup.com/trading_hours/fx-hours.html

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
