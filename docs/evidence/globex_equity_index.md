<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_equity_index` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`cme_group.rs`](../../src/calendar/schedules/futures/us/cme_group.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Scoped modern CME/CBOT U.S.-grid family, including YM/MYM but excluding full-size `SP`, NKD, BTIC, and TACO. Current RTH/ETH and queues are exact; sourced dated history retains the 2010 weekday-queue change and 2012/2015/2021 matching revisions, but omits the Sunday queue whose 16:15→16:00 move is bracketed to 2012-05-28..2012-06-07 by CME's own trading-hours captures, with both CME notice channels read in full across that window and silent on it. Dated profiles now carry the sourced Sunday 16:15–17:00 intersection from the January-2010 floor, so only the 16:00–16:15 quarter-hour remains withheld.

## Revision rows

- 2010-11-15 — T1 — CME Globex notice 20101025 — Monday–Thursday Pre-Open moves from 16:50 to 16:45 CT.
- 2012-11-18 — T1 — CME Globex notice 20121022 — new daily trading-hour schedule; post-halt slice becomes 15:30–16:15 CT including Fridays.
- 2015-09-20 — T1 — CME Globex notice 20150817 — CME Equity and CBOT Equity closes move 15 minutes earlier to 16:00 CT.
- 2021-06-27 — T1 — CME Globex notice 20210621 — the 15:15–15:30 CT halt is removed, producing the continuous 17:00–16:00 CT ETH envelope.
- 2026-08-22 — T1 — 2026-08-22 review: verified current, onset undated — knowledge-bound row widening the Sunday queue to the sourced current 16:00–17:00 CT Pre-Open.

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

- **order-entry** — **the crate holds no admissible artifact whose own scope covers an ordinary week inside the claimed interval**, so the 16:00–16:15 CT quarter-hour is withheld. The 2012 changeover day is also unstated — the bracket is 2012-05-28..2012-06-07 — but that is **not** the operative reason: no sourced state has printed 16:15 since 2012-05-28, because Globex notice 20120402 dates the old value (2012-04-15, in the venue's own market-state language) and notice 20121112 prints the new one, and the charter says in terms that "a later observation alone does not prove the intervening period complete". The 2026-08-31 review narrowed the bracket to 2012-05-28..2012-06-07 from CME's own trading-hours captures and read both CME dated notice channels in full across that window without finding an announcement, so the dated profiles serve the sourced 16:15–17:00 CT intersection and withhold only the 16:00–16:15 CT quarter-hour. **Search record**, so the next attempt does not repeat it: 41 of the 42 Globex notice date-filenames for 2012 were retrieved and text-scanned, together with all 187 real CME 2012 CFTC rule filings, the market-data notice channel (zero occurrences of "pre-open" or "16:15" in it) and the three in-bracket Federal Register notices — no effective day in any of them; CME's own Globex Initiative Calendar for June 2012, the month of the change, carries no pre-open or trading-hours entry at all; the Wayback archive holds 703 captures of the operator's trading-hours service covering 29 distinct Sundays, **every one a holiday eve**, so it can never carry a normal-week capture; and Save Page Now on the 2025-01-05..07 window returned `"hasEvents": false`, which is the channel's retention limit rather than the operator's word. **Closing conditions**, best first: (a) a written reply from CME's own desk giving the Sunday Pre-Open time in force for the week of 2025-01-05, or the effective date of the 2012 change — T1 in session language, and it closes this under either reading; (b) a T1 or T2 artifact covering an ordinary week at or near the 2025 floor, from a channel that still retains it and checked for `hasEvents: true`; (c) a May/June 2012 Global Command Center one-pager of the class the 2013 grain notice proves exists. Do not key 2012-06-03: the only Sunday inside the bracket is an inference from the bracket, not an operator statement. Tracked as #79. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES). Horizon 2012-05-03: below that capture the Sunday 16:15–17:00 CT queue is carried, not sourced.
- **residual risk** — the only Sunday inside the narrowed bracket is 2012-06-03; that is an observation about the bracket, not a source-stated effective day, so LAW-NO-FABRICATED-DATES keeps it out of the tables.
- **scope** — the family is the scoped modern CME/CBOT U.S.-grid set including YM and MYM; full-size `SP`, NKD, BTIC and TACO are excluded.

> Shared module. The narrative for
> [`cme_group.rs`](../../src/calendar/schedules/futures/us/cme_group.rs) lives in
> [`cme`](cme.md#module-narrative-moved-from-srccalendarschedulesfuturesuscme_grouprs-on-2026-09-12-utc).
> Sibling identities: [`cme`](cme.md).

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

Two of the 2025-2027 ids are the **second window** of a trade date whose day is
printed by two different windows. `CME-SVC-2026-06-18` and `CME-SVC-2027-06-17`
each stop on their Saturday and print no Sunday entry at all, so the Sunday
Pre-Open, the Sunday-17:00 open and the trade date's `16:00 closed` final close
all come from `CME-SVC-2026-06-21` and `CME-SVC-2027-06-20` for trade dates
2026-06-22 and 2027-06-21. The 2026-07-06 row's single window,
`CME-SVC-2026-07-03`, runs through its Sunday and prints both halves itself. Each
row names the window each half comes from.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `CME-SVC-2024-12-31` | 2024-12-31 .. 2025-01-02 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-12-31&toEventDate=2025-01-02> | archive capture 2024-12-20T15:53:40Z | T2 | `375c70eecd19c5c6204ecb408d1b3210a9da4c9a03b85ef1c7dbbcde1397ab63` |
| `CME-SVC-2025-01-19` | 2025-01-19 .. 2025-01-21 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-01-19&toEventDate=2025-01-21> | archive capture 2024-12-20T15:53:40Z | T2 | `4f2ab56af14e7b3a6978e7fa6db8e2cfc63a82d06428cd88844f0e5bcf534f40` |
| `CME-SVC-2025-02-16` | 2025-02-16 .. 2025-02-18 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-02-16&toEventDate=2025-02-18> | archive capture 2024-12-20T15:53:40Z | T2 | `5bec2ca6b4999a534e4d9818035aaa18ec8626b6c912cf7e3d2c57015536f2fa` |
| `CME-SVC-2025-04-17` | 2025-04-17 .. 2025-04-19 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-04-17&toEventDate=2025-04-19> | archive capture 2024-12-20T15:53:40Z | T2 | `865a1d4f08102e00151bd87ab2b8e8a7720e9203a17aaaba24627ade3ed26e74` |
| `CME-SVC-2025-05-25` | 2025-05-25 .. 2025-05-27 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-05-25&toEventDate=2025-05-27> | archive capture 2024-12-20T15:53:40Z | T2 | `5f42869879c826f5949b79236aabb3d26d74e7565d92d7cc5e8784c63973210b` |
| `CME-SVC-2025-06-18` | 2025-06-18 .. 2025-06-20 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-06-18&toEventDate=2025-06-20> | archive capture 2024-12-20T15:53:40Z | T2 | `a572706907175776255261103b393493ebdf5a8106ec5374d129145bdf89105e` |
| `CME-SVC-2025-07-03` | 2025-07-03 .. 2025-07-05 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-07-03&toEventDate=2025-07-05> | archive capture 2024-12-20T15:53:40Z | T2 | `b80cd4bfed0ae72865bfacc1936e107eb8febfcc94b37fcce1d05505c659147b` |
| `CME-SVC-2025-08-31` | 2025-08-31 .. 2025-09-02 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-08-31&toEventDate=2025-09-02> | archive capture 2024-12-20T15:53:40Z | T2 | `e075762ed34a86048d94900e10edba10d95b3d766052908ffbb5133f6b64bab0` |
| `CME-SVC-2025-11-26` | 2025-11-26 .. 2025-11-28 | <https://web.archive.org/web/20260129012309id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-11-26&toEventDate=2025-11-28> | archive capture 2026-01-29T01:23:09Z | T2 | `6c4c598791058dd9a11aff0ddb072c761a436c6d1054b891def74c6935f020f1` |
| `CME-SVC-2025-11-26-SAT` | 2025-11-26 .. 2025-11-29 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-11-26&toEventDate=2025-11-29> | live retrieval 2026-09-12T08:55:12Z | T2 | `2e9f34f20085de3ccbdff1dc29cb7463bcff93713ef0c550740d6f15e0635ab7` |
| `CME-SVC-2025-12-24` | 2025-12-24 .. 2025-12-26 | <https://web.archive.org/web/20260129012159id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-12-24&toEventDate=2025-12-26> | archive capture 2026-01-29T01:21:59Z | T2 | `322a2be989b67f5f4cc0ec12fd63a393383d574badd4aacc87a0c9637533d386` |
| `CME-SVC-2025-12-31` | 2025-12-31 .. 2026-01-02 | <https://web.archive.org/web/20260619114105id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-12-31&toEventDate=2026-01-02> | archive capture 2026-06-19T11:41:05Z | T2 | `0ed61f8328eda4746265cc8e197f10cd53aec06c2b393927bab27c913993d314` |
| `CME-SVC-2026-01-18` | 2026-01-18 .. 2026-01-20 | <https://web.archive.org/web/20260619114105id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-01-18&toEventDate=2026-01-20> | archive capture 2026-06-19T11:41:05Z | T2 | `5e3ff08bdc7d07474b96b8dc8c18ed0d5e48d12dc4bcad81a5f68820cb2aa89e` |
| `CME-SVC-2026-02-15` | 2026-02-15 .. 2026-02-17 | <https://web.archive.org/web/20260619114105id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-02-15&toEventDate=2026-02-17> | archive capture 2026-06-19T11:41:05Z | T2 | `5dd507dd959d0029e838ec88b1bdb63c32444ea36a121de002606f5d7b206e2f` |
| `CME-SVC-2026-04-01` | 2026-04-01 .. 2026-04-03 | <https://web.archive.org/web/20260619114118id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-04-01&toEventDate=2026-04-03> | archive capture 2026-06-19T11:41:18Z | T2 | `54bcc271e9ba9737a99a2fe608e658de0c657075284d050fbfec4fe1aee2a2a5` |
| `CME-SVC-2026-05-24` | 2026-05-24 .. 2026-05-26 | <https://web.archive.org/web/20260619114105id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-05-24&toEventDate=2026-05-26> | archive capture 2026-06-19T11:41:05Z | T2 | `f7e30d204ce2cbe08e5f486ded6518f623369159f3a36161288a4708288314da` |
| `CME-SVC-2026-06-18` | 2026-06-18 .. 2026-06-20 | <https://web.archive.org/web/20260619113404id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-06-18&toEventDate=2026-06-20> | archive capture 2026-06-19T11:34:04Z | T2 | `97fd5da371309f4486a8fb49ff2105c6c1c2396939ab7c76f1a2a1097b6f015c` |
| `CME-SVC-2026-07-03` | 2026-07-03 .. 2026-07-05 | <https://web.archive.org/web/20260619114108id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-07-03&toEventDate=2026-07-05> | archive capture 2026-06-19T11:41:08Z | T2 | `4b89a026358e998277f9c1ff7e095e5d4e625cdc45115fd141dc92201833155b` |
| `CME-SVC-2026-09-06` | 2026-09-06 .. 2026-09-08 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-09-06&toEventDate=2026-09-08> | live retrieval 2026-09-12T04:30Z | T2 | `01fb78ffaac10eac466fed53674214222f05aed518b9d93a4b42cf8957147bca` |
| `CME-SVC-2026-11-25` | 2026-11-25 .. 2026-11-27 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-11-25&toEventDate=2026-11-27> | live retrieval 2026-09-12T04:30Z | T2 | `e1f35a5623b3c5d15e7468b2cb4119e587411a9714f920605dab11bf688756d1` |
| `CME-SVC-2026-12-22` | 2026-12-22 .. 2026-12-24 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-12-22&toEventDate=2026-12-24> | live retrieval 2026-09-12T04:30Z | T2 | `c8c0267da8cf171409ad8ca188082b3aa326e8d04a89d12503dcf9f57bf3b7ab` |
| `CME-SVC-2026-12-24` | 2026-12-24 .. 2026-12-26 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-12-24&toEventDate=2026-12-26> | live retrieval 2026-09-12T04:30Z | T2 | `bdc1fe831adb794bcf8aeb7e99baf6af2009d1ff9969d0a48b18b2ebc2e1e829` |
| `CME-SVC-2026-12-31` | 2026-12-31 .. 2027-01-02 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-12-31&toEventDate=2027-01-02> | live retrieval 2026-09-12T04:30Z | T2 | `7162652821c16f1bd05e3ec533bd5b82af03833c7186a64c7734b0b650364dcd` |
| `CME-SVC-2027-01-17` | 2027-01-17 .. 2027-01-19 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-01-17&toEventDate=2027-01-19> | live retrieval 2026-09-12T04:30Z | T2 | `7155c4b7ee8b299b3033eb3daf002b6ceecf0fbd53f6f98a7036048022275743` |
| `CME-SVC-2027-02-14` | 2027-02-14 .. 2027-02-16 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-02-14&toEventDate=2027-02-16> | live retrieval 2026-09-12T04:30Z | T2 | `41f5aa8cde3879f8b10490386c134a294a0f1509edde2022a22ec3ffcaed1183` |
| `CME-SVC-2027-03-25` | 2027-03-25 .. 2027-03-27 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-03-25&toEventDate=2027-03-27> | live retrieval 2026-09-12T04:30Z | T2 | `9bd7225d440e00139f30892f3914c9b38beb8bf29d4272039b6cd8f2de926880` |
| `CME-SVC-2027-05-30` | 2027-05-30 .. 2027-06-01 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-05-30&toEventDate=2027-06-01> | live retrieval 2026-09-12T04:30Z | T2 | `1283649724c30163fa08ba7ab02d1230fa9a7dd0613b8b4b3d96cd1d9dc4febd` |
| `CME-SVC-2027-06-17` | 2027-06-17 .. 2027-06-19 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-06-17&toEventDate=2027-06-19> | live retrieval 2026-09-12T04:30Z | T2 | `60c9a2f5106d61039a616986b463cd852861ee4d3b91b11fac8badfa1b97b01c` |
| `CME-SVC-2027-07-04` | 2027-07-04 .. 2027-07-06 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-07-04&toEventDate=2027-07-06> | live retrieval 2026-09-12T04:30Z | T2 | `93ff8232886435c94be682bf968aa30749011cdf8dadeb7d2425a3b0b9e0bf71` |
| `CME-SVC-2027-09-05` | 2027-09-05 .. 2027-09-07 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-09-05&toEventDate=2027-09-07> | live retrieval 2026-09-12T04:30Z | T2 | `aa08a3bd102812928e69cf1ea4c8a84f738eaa5d14f967acee7d2571e74aedb9` |
| `CME-SVC-2027-11-24` | 2027-11-24 .. 2027-11-26 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-11-24&toEventDate=2027-11-26> | live retrieval 2026-09-12T04:30Z | T2 | `6aa7c0fd701a02480dabeac1fbae1a69b56e77643a29e3a9b2223c56e822ce9f` |
| `CME-SVC-2027-12-22` | 2027-12-22 .. 2027-12-25 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-12-22&toEventDate=2027-12-25> | live retrieval 2026-09-12T04:30Z | T2 | `5edc4dd588a32faa74f841494c10a3df48692dca29843c3581bad3e18c30fef9` |
| `CME-SVC-2026-06-21` | 2026-06-21 .. 2026-06-23 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-06-21&toEventDate=2026-06-23> | live retrieval 2026-09-25 via `https://r.jina.ai/` | T2 | `91534cfd3ae56920ef744734216d2f5944cb9057c483d4b12f1550d91d91bcaf` |
| `CME-SVC-2026-07-05` | 2026-07-05 .. 2026-07-07 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-07-05&toEventDate=2026-07-07> | live retrieval 2026-09-26T02:55:11Z via `https://r.jina.ai/` | T2 | `f6e1900b4971eda307f63d1c905f94741c268350fe4623e916580065c4c194cb` |
| `CME-SVC-2027-06-20` | 2027-06-20 .. 2027-06-22 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-06-20&toEventDate=2027-06-22> | live retrieval 2026-09-25 via `https://r.jina.ai/` | T2 | `9ab30e85bb6947803f35369ed29e24cc98c20c86498bf48f87ec6515d8431107` |

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

Every instant is quoted exactly as the service prints it. CME states the zone once, verbatim: "Trading hours are subject to change and are in U.S. Central Time unless otherwise stated." This channel prints no Eastern column, so no ET value is asserted anywhere below; the `CT` token in each row is this file's editorial expansion of that sentence, not text CME printed beside the instant.

**Vocabulary.** CME publishes ten headline product groups; this family is the group it prints as **Equity Index**, queried as `ES` (E-mini S&P 500). The family is the scoped modern CME/CBOT U.S.-grid equity-index set including YM and MYM; the separately printed Nikkei lines (`NKD`, `NIY`) are `globex_nikkei_225_dollar` and are not recorded here, even where they track this family's instants.

**Event types, verbatim from the operator.** `closed` — "Final Close of the date. Day and GTD (current trade date) orders are eliminated." `preopen` — "Order Entry, modification, and cancel are allowed. No order matching." `open` — "Start of continuous trading phase. Order matching begins."

**The conversion, once.** Rows are keyed by the crate's venue-local trade date, never by the operator's event date (design memo D1). This family's trading day for trade date `D` opens 17:00 CT on the preceding business evening and ends at its 16:00 CT final close on `D`, so an eve record that merely lacks its evening leg is evidence for the *following* date's `closed` row and never a row of its own.

### Interpretive notes

The `Derived from` cells of the 2025-2027 tables below annotate some rows with the
retrieval's own short codes — `N1`, `N15` and `N17`. Those codes are defined in the
research store's `raw/cme-2025-2027/INDEX.md` (`N1`..`N16`, `EC`) and in its round-1
addendum (`N17`, `N18`), neither of which is committed, so a reader following a row to
its reasoning currently reaches a dead reference. Each code is defined here beside the
bytes that exhibit it. Artifact paths are relative to the research store's `holidays/`
directory, and every quotation below is a verbatim substring of the JSON at the path
named beside it, given with the product id and the `eventDate` it was read from.

| Note | In one line | Crate row |
|---|---|---|
| `N1` | the service publishes no events for the date | `HolidayKind::Closed` on that trade date |
| `N15` | matching halts at 12:00 CT and resumes at 17:00 CT, the span on both sides carrying the next business day's trade date | `HolidayKind::EarlyClose { close_ssm: NOON }` on that trade date |
| `N17` | no day session: only a 16:00 CT pre-open and a 17:00 CT open, both already carrying the next business day's trade date | `HolidayKind::Closed` on that trade date |

**`N1` — no market events published for the date (the family does not trade).**

- **Evidence.** `raw/cme-2025-2027/live/thbp/thbp_2026-12-24_2026-12-26.json` — document `CME-SVC-2026-12-24`, sha256 `bdc1fe831adb794bcf8aeb7e99baf6af2009d1ff9969d0a48b18b2ebc2e1e829`, live retrieval 2026-09-12T04:30Z; the identical bytes are also saved at `raw/cme-2025-2027/arc/thbp_2026-12-24_2026-12-26_20260830142904.json`. Product `133` (E-mini S&P 500 Futures, group `ES`), `eventDate` `2026-12-25`:

  ```json
  {"groupCode":"ES","eventDate":"2026-12-25","events":[]}
  ```

  The same response's neighbouring records, for contrast — the date before it, and the Saturday after it:

  ```json
  {"groupCode":"ES","eventDate":"2026-12-24","events":[{"tradingDate":"2026-12-24","eventTime":"12:15","marketEventType":"closed"}]}
  {"groupCode":"ES","eventDate":"2026-12-26","events":[]}
  ```

- **Crate row.** `HolidayKind::Closed` on trade date 2026-12-25 — `(2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24")` in `src/calendar/schedules/holidays/globex_equity_index.rs`. Reading the empty list as a closure is the crate's interpretive step: the operator prints no `closed` event here, so no event in session language says the market shut. What makes the step the right one is that the service answers per requested `eventDate` — an empty list is its answer for that date, not a missing answer — and 2026-12-25 is a Friday, a weekday on which this family's ordinary week does carry a session whose trade date is the date itself. The prior evening's leg does not run either: the same response's 2026-12-24 record prints one `12:15 closed` for trade date 2026-12-24 and no evening re-open, so the leg that would carry trade date 2026-12-25 is absent as well. Both halves of the trading day are therefore gone, and `Closed` removes exactly them.

  The same empty list on the Saturday 2026-12-26 is the ordinary weekend and ships no row, so the code is read as a closure only where the family's ordinary week has a session whose trade date is the date.

- **Falsified by.** A later CME publication printing any event against `eventDate` 2026-12-25 for product 133 — a `closed`, `preopen` or `open` would make the date something other than a complete closure.

**`N15` — matching halts at 12:00 CT on the holiday (published as a `preopen` event, not as a final close) and resumes at 17:00 CT; the span on both sides of the halt carries the next business day's trade date.**

- **Evidence.** `raw/cme-2025-2027/arc/thbp_2026-01-18_2026-01-20_20260619114105.json` — document `CME-SVC-2026-01-18`, sha256 `5e3ff08bdc7d07474b96b8dc8c18ed0d5e48d12dc4bcad81a5f68820cb2aa89e`, archive capture 2026-06-19T11:41:05Z. Product `133`, `eventDate` `2026-01-19` (Monday, Martin Luther King Jr. Day):

  ```json
  {"groupCode":"ES","eventDate":"2026-01-19","events":[{"tradingDate":"2026-01-20","eventTime":"12:00","marketEventType":"preopen"},{"tradingDate":"2026-01-20","eventTime":"17:00","marketEventType":"open"}]}
  ```

  The same artifact's Sunday and Tuesday records, which are what make the span legible:

  ```json
  {"groupCode":"ES","eventDate":"2026-01-18","events":[{"tradingDate":"2026-01-20","eventTime":"16:00","marketEventType":"preopen"},{"tradingDate":"2026-01-20","eventTime":"17:00","marketEventType":"open"}]}
  {"groupCode":"ES","eventDate":"2026-01-20","events":[{"tradingDate":"2026-01-20","eventTime":"16:00","marketEventType":"closed"},{"tradingDate":"2026-01-21","eventTime":"16:45","marketEventType":"preopen"},{"tradingDate":"2026-01-21","eventTime":"17:00","marketEventType":"open"}]}
  ```

  Every event from the Sunday-evening queue through Tuesday's 16:00 CT final close carries trade date 2026-01-20; no event anywhere in the response carries trade date 2026-01-19.

- **Crate row.** `HolidayKind::EarlyClose { close_ssm: NOON }` on trade date 2026-01-19 — `(2026, 1, 19, early_close(NOON), T2, "CME-SVC-2026-01-18")`, where `NOON` is `12 * 3_600`. That this is an **interpretive step** has to be said plainly: the operator publishes no `closed` event on this date, so 12:00 CT is not a final close it printed. It is the instant its own event list shows matching stopping, published under the `preopen` type quoted above ("Order Entry, modification, and cancel are allowed. No order matching."), with the 17:00 CT `open` where matching starts again. What makes the reading the right one: the family's trading day for trade date `D` is the span that ends at its 16:00 CT final close on `D`, and on 2026-01-19 the only boundary the operator prints is this 12:00 CT instant. The alternative readings are both excluded — `Closed` would delete a morning that traded from 17:00 CT Sunday to 12:00 CT Monday, and no row at all would report the date as an ordinary Monday when the operator prints a boundary four hours early. The crate assigns that halted morning to the holiday's own trade date while the operator assigns the whole span to 2026-01-20; the divergence is in the trade-date label only, and `is_open` agrees with the operator minute for minute. The companion `ReplacementBlocks` row for trade date 2026-01-20 states the rest of the merged span, so the split is stated rather than implied.

  `LAW-SESSION-NOT-EXPIRY` is not in play here. The operator prints no expiry, settlement, marker or termination row on this date at all, and nothing above reads an order-entry cutoff *as* a session close: the crate's step is that a printed matching halt is the end of the holiday's own trading day, which is the only boundary the operator's bytes contain.

- **Falsified by.** A CME publication printing a `closed` event for trade date 2026-01-19, which would make the date a full closure rather than an early close, or printing no stop at 12:00 CT, which would make it an ordinary Monday.

**`N17` — no day session on the holiday: the operator publishes only a 16:00 CT pre-open and a 17:00 CT open, both already carrying the next business day's trade date.**

- **Evidence.** `raw/cme-2025-2027/arc/thbp_2025-12-31_2026-01-02_20260619114105.json` — document `CME-SVC-2025-12-31`, sha256 `0ed61f8328eda4746265cc8e197f10cd53aec06c2b393927bab27c913993d314`, archive capture 2026-06-19T11:41:05Z. Product `133`, `eventDate` `2026-01-01`:

  ```json
  {"groupCode":"ES","eventDate":"2026-01-01","events":[{"tradingDate":"2026-01-02","eventTime":"16:00","marketEventType":"preopen"},{"tradingDate":"2026-01-02","eventTime":"17:00","marketEventType":"open"}]}
  ```

  The same artifact's records for the evening before and for the trade date both events name:

  ```json
  {"groupCode":"ES","eventDate":"2025-12-31","events":[{"tradingDate":"2025-12-31","eventTime":"16:00","marketEventType":"closed"}]}
  {"groupCode":"ES","eventDate":"2026-01-02","events":[{"tradingDate":"2026-01-02","eventTime":"16:00","marketEventType":"closed"}]}
  ```

- **Crate row.** `HolidayKind::Closed` on trade date 2026-01-01 — `(2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31")`. This is the crate's interpretive step: the two events printed against the holiday are session language for the **next** trade date's trading day, not for this date, so the date settles no trade of its own and nothing is assignable to it. What makes the reading the right one: the operator's own printed trade date on both events is 2026-01-02, the crate keys its rows by that printed trade date, and no event in the response carries 2026-01-01. The 17:00 CT open is the beginning of 2026-01-02's session exactly as the ordinary week has it — and that session's end is the 16:00 CT `closed` the 2026-01-02 record prints. Nothing is inferred from an expiry or a settlement instant: the previous evening's leg is simply absent, its record showing one ordinary 16:00 CT final close for trade date 2025-12-31 and no re-open.

- **Falsified by.** An event on `eventDate` 2026-01-01 carrying trade date 2026-01-01, or an evening re-open printed on 2025-12-31 and carrying it.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `16:00 preopen; 17:00 open` | `CME-SVC-2024-12-31` | T2 | eventDate 2025-01-01, CME trade date 2025-01-02 on both events; note N17 |
| 2025-01-20 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2025-01-19` | T2 | eventDate 2025-01-20, CME trade date 2025-01-21; note N15 |
| 2025-01-21 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2025-01-19; `12:00 preopen; 17:00 open` on eventDate 2025-01-20; `16:00 closed` on eventDate 2025-01-21 | `CME-SVC-2025-01-19` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it carries 2025-01-21; it still ends at the holiday's own 12:00 CT close, where the queue then opens. Both matching envelopes are split at the family's regular boundaries, so the holiday's clipped morning and the trade date's own 08:30-15:15 regular session both keep answering |
| 2025-02-17 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2025-02-16` | T2 | eventDate 2025-02-17, CME trade date 2025-02-18; note N15 |
| 2025-02-18 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2025-02-16; `12:00 preopen; 17:00 open` on eventDate 2025-02-17; `16:00 closed` on eventDate 2025-02-18 | `CME-SVC-2025-02-16` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it carries 2025-02-18; it still ends at the holiday's own 12:00 CT close, where the queue then opens. Both matching envelopes are split at the family's regular boundaries, so the holiday's clipped morning and the trade date's own 08:30-15:15 regular session both keep answering |
| 2025-04-18 | closed | `no events published` | `CME-SVC-2025-04-17` | T2 | eventDate 2025-04-18, no CME trade date; note N1 |
| 2025-05-26 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2025-05-25` | T2 | eventDate 2025-05-26, CME trade date 2025-05-27; note N15 |
| 2025-05-27 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2025-05-25; `12:00 preopen; 17:00 open` on eventDate 2025-05-26; `16:00 closed` on eventDate 2025-05-27 | `CME-SVC-2025-05-25` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it carries 2025-05-27; it still ends at the holiday's own 12:00 CT close, where the queue then opens. Both matching envelopes are split at the family's regular boundaries, so the holiday's clipped morning and the trade date's own 08:30-15:15 regular session both keep answering |
| 2025-06-19 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2025-06-18` | T2 | eventDate 2025-06-19, CME trade date 2025-06-20; note N15 |
| 2025-06-20 | replacement blocks | `16:45 preopen; 17:00 open` on eventDate 2025-06-18; `12:00 preopen; 17:00 open` on eventDate 2025-06-19; `16:00 closed` on eventDate 2025-06-20 | `CME-SVC-2025-06-18` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it carries 2025-06-20; it still ends at the holiday's own 12:00 CT close, where the queue then opens. Both matching envelopes are split at the family's regular boundaries, so the holiday's clipped morning and the trade date's own 08:30-15:15 regular session both keep answering; its `-2` day is a Wednesday, so that queue is the weekday `16:45` |
| 2025-07-03 | early close | `12:15 closed` - 12:15 CT | `CME-SVC-2025-07-03` | T2 | eventDate 2025-07-03, CME trade date 2025-07-03; the evening leg runs normally |
| 2025-07-04 | early close | `12:00 closed` - 12:00 CT | `CME-SVC-2025-07-03` | T2 | eventDate 2025-07-04, CME trade date 2025-07-04 |
| 2025-09-01 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2025-08-31` | T2 | eventDate 2025-09-01, CME trade date 2025-09-02; note N15 |
| 2025-09-02 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2025-08-31; `12:00 preopen; 17:00 open` on eventDate 2025-09-01; `16:00 closed` on eventDate 2025-09-02 | `CME-SVC-2025-08-31` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it carries 2025-09-02; it still ends at the holiday's own 12:00 CT close, where the queue then opens. Both matching envelopes are split at the family's regular boundaries, so the holiday's clipped morning and the trade date's own 08:30-15:15 regular session both keep answering |
| 2025-11-27 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2025-11-26` | T2 | eventDate 2025-11-27, CME trade date 2025-11-28; note N15 |
| 2025-11-28 | replacement blocks | `16:45 preopen; 17:00 open` on eventDate 2025-11-26; `12:00 preopen; 17:00 open` on eventDate 2025-11-27; `12:15 closed` on eventDate 2025-11-28 | `CME-SVC-2025-11-26` | T2 | Thanksgiving Day publishes no final close, so this trade date owns the span from Wednesday evening and its own regular session is cut at 12:15 CT |
| 2025-11-29 | closed | `no events published` | `CME-SVC-2025-11-26-SAT` | T2 | eventDate 2025-11-29; all ten products publish an empty schedule, and CME's 2025 Globex table states the period as "27 - 29 November 2025" |
| 2025-12-24 | early close | `12:15 closed` - 12:15 CT | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-24, CME trade date 2025-12-24; no evening re-open |
| 2025-12-25 | closed | `16:00 preopen; 17:00 open` | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-25, CME trade date 2025-12-26 on both events; note N17 |

**Interpretive steps, 2025.** On the Monday and Thursday holidays (MLK, Presidents', Memorial, Juneteenth, Labor, Thanksgiving) CME publishes no `closed` event at all: it prints `12:00 preopen`, matching stops, and it carries the whole Sunday- or Wednesday-evening-through-holiday span under the *following* business day's trade date. The crate keys a trading day by its final close, so that span is trade date 2025-01-20 (and its siblings) and the printed 12:00 CT instant is where it ends. The row is therefore an early close, exactly as design memo section 1.1 works it. The consequence to state plainly: on these dates the crate assigns the morning's trading to the holiday's own trade date while CME assigns it to the next business day. That is a trade-date divergence, not an hours divergence - `is_open` agrees with the operator minute for minute.

The eve records CME labels `modified` ship no row. 2024-12-31 and 2025-04-17 each print `16:00 closed` and no evening re-open; that is the family's normal grid minus the evening leg, and the leg is already removed by the `closed` row on the following trade date (2025-01-01, 2025-04-18) because the leg's trade date is that date. 2024-12-31 is below the coverage window in any case.

Columbus Day and Veterans Day appear nowhere in CME's Globex holiday list and Globex trades a normal session on both, so inside this window they read as audited normal with no row, which is correct rather than accidental.

**Gaps, 2025.**

- **order entry, not representable.** On 2025-01-01 and 2025-12-25 the pre-open opens 16:00 CT instead of the family's normal 16:45 CT. The holiday vocabulary is `DayPolicy`'s and has no order-entry boundary, so this cannot be stated. It changes no `is_open` answer, only `is_accepting_orders` for 45 minutes.
- **the 2025-11-28 morning Pre-Open is served, and it is order entry.** CME's finalised publication prints `07:00 preopen; 07:30 open; 12:15 closed` on eventDate 2025-11-28, all three carrying CME trade date 2025-11-28. CME's own event vocabulary defines `preopen` as "Order Entry, modification, and cancel are allowed. No order matching." and `open` as "Start of continuous trading phase. Order matching begins.", so `07:00-07:30` CT is a queue, not a session. 07:30 CT is *earlier* than the family's 08:30 CT regular open, so it is not a late open either: the queue belongs to the overnight session and matching runs from 07:30 until the regular session takes over at 08:30. The row's `MERGED_SESSION_EARLY_CLOSE_BLOCKS_2025_11_28` states exactly that, and the 12:15 CT final close is unchanged. Until this correction the whole morning was one `extended` block, so `is_open` answered `true` in the operator's queue. It is the only date in the 2025-2027 window with this shape: the 2026 and 2027 Thanksgiving Fridays print the close line alone and keep the six-block static.
- **Saturday 2025-11-29 ships a row.** The live service publishes a 2025-11-29 schedule for all ten products with no events (`CME-SVC-2025-11-26-SAT`), and CME's 2025 Globex table states the holiday period as "27 - 29 November 2025". The family's normal week has no Saturday session, so the row changes no answer here — but one audited operator closure ships in every family that routes to the venue, so the D17 venue intersection for `Cme`, `Cbot`, `Comex` and `Nymex` is computed from one uniform input rather than from eight family judgements.
- **residual risk, sourcing vintage.** The eight windows New Year 2025 through Labor Day 2025 rest on a single pre-holiday capture, 2024-12-20T15:53:40Z, and CME states on the same page that hours are usually finalised about two weeks before a holiday. The service's own retention edge falls between Labor Day 2025 and Thanksgiving 2025, so the channel cannot restate them. They are internally consistent with the 2026 and 2027 rows for the same holidays, which are sourced from post-holiday captures and live reads.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `16:00 preopen; 17:00 open` | `CME-SVC-2025-12-31` | T2 | eventDate 2026-01-01, CME trade date 2026-01-02 on both events; note N17 |
| 2026-01-19 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2026-01-18` | T2 | eventDate 2026-01-19, CME trade date 2026-01-20; note N15 |
| 2026-01-20 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2026-01-18; `12:00 preopen; 17:00 open` on eventDate 2026-01-19; `16:00 closed` on eventDate 2026-01-20 | `CME-SVC-2026-01-18` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it carries 2026-01-20; it still ends at the holiday's own 12:00 CT close, where the queue then opens. Both matching envelopes are split at the family's regular boundaries, so the holiday's clipped morning and the trade date's own 08:30-15:15 regular session both keep answering |
| 2026-02-16 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2026-02-15` | T2 | eventDate 2026-02-16, CME trade date 2026-02-17; note N15 |
| 2026-02-17 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2026-02-15; `12:00 preopen; 17:00 open` on eventDate 2026-02-16; `16:00 closed` on eventDate 2026-02-17 | `CME-SVC-2026-02-15` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it carries 2026-02-17; it still ends at the holiday's own 12:00 CT close, where the queue then opens. Both matching envelopes are split at the family's regular boundaries, so the holiday's clipped morning and the trade date's own 08:30-15:15 regular session both keep answering |
| 2026-04-03 | early close | `08:15 closed` - 08:15 CT | `CME-SVC-2026-04-01` | T2 | eventDate 2026-04-03, CME trade date 2026-04-03; corroborated at T1 by CME-TRADING-HOURS-2025-08-30, which states no instant |
| 2026-05-25 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2026-05-24` | T2 | eventDate 2026-05-25, CME trade date 2026-05-26; note N15 |
| 2026-05-26 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2026-05-24; `12:00 preopen; 17:00 open` on eventDate 2026-05-25; `16:00 closed` on eventDate 2026-05-26 | `CME-SVC-2026-05-24` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it carries 2026-05-26; it still ends at the holiday's own 12:00 CT close, where the queue then opens. Both matching envelopes are split at the family's regular boundaries, so the holiday's clipped morning and the trade date's own 08:30-15:15 regular session both keep answering |
| 2026-06-19 | early close | `12:00 closed` - 12:00 CT | `CME-SVC-2026-06-18` | T2 | eventDate 2026-06-19, CME trade date 2026-06-22; see the 2026 interpretive steps |
| 2026-06-22 | replacement blocks | `16:45 preopen; 17:00 open` on eventDate 2026-06-18 and `12:00 closed` on eventDate 2026-06-19 (`CME-SVC-2026-06-18`); `05:00 open; 17:00 closed` on eventDate 2026-06-20 (`CME-SVC-2026-06-18`); `16:00 preopen; 17:00 open` on eventDate 2026-06-21 and `16:00 closed` on eventDate 2026-06-22 (`CME-SVC-2026-06-21`), all CME trade date 2026-06-22 | `CME-SVC-2026-06-18` | T2 | the complete trade date: every phase the operator prints against CME trade date 2026-06-22 — the Thursday Pre-Open queue and the session the Friday noon close ends, the Saturday session, and the Sunday Pre-Open and the Sunday-17:00-to-Monday-16:00 continuous envelope. The first window stops at the Saturday and prints no Sunday entry at all, so the row reads two documents. The Friday 12:00 close carries trade date 2026-06-22 itself, so the morning it ends belongs here; no Friday-**evening** open is published for this date |
| 2026-07-03 | early close | `12:00 closed` - 12:00 CT | `CME-SVC-2026-07-03` | T2 | eventDate 2026-07-03, CME trade date 2026-07-06; see the 2026 interpretive steps |
| 2026-07-06 | replacement blocks | `16:45 preopen; 17:00 open` on eventDate 2026-07-02 and `12:00 closed` on eventDate 2026-07-03; `05:00 open; 17:00 closed` on eventDate 2026-07-04 and `16:00 preopen; 17:00 open` on eventDate 2026-07-05 — all CME trade date 2026-07-06 (`CME-SVC-2026-07-03`); `16:00 closed` on eventDate 2026-07-06 carrying CME trade date 2026-07-06 (`CME-SVC-2026-07-05`) | `CME-SVC-2026-07-03` | T2 | the complete trade date, as 2026-06-22: the Thursday queue and the session the Friday noon close ends, the Saturday session, the Sunday Pre-Open and the Sunday evening-to-Monday envelope. The first window prints four of the five phases; the Monday final close is read from the second, which is why this row cites two documents |
| 2026-09-07 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2026-09-06` | T2 | eventDate 2026-09-07, CME trade date 2026-09-08; note N15 |
| 2026-09-08 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2026-09-06; `12:00 preopen; 17:00 open` on eventDate 2026-09-07; `16:00 closed` on eventDate 2026-09-08 | `CME-SVC-2026-09-06` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it carries 2026-09-08; it still ends at the holiday's own 12:00 CT close, where the queue then opens. Both matching envelopes are split at the family's regular boundaries, so the holiday's clipped morning and the trade date's own 08:30-15:15 regular session both keep answering |
| 2026-11-26 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2026-11-25` | T2 | eventDate 2026-11-26, CME trade date 2026-11-27; note N15 |
| 2026-11-27 | replacement blocks | `16:45 preopen; 17:00 open` on eventDate 2026-11-25; `12:00 preopen; 17:00 open` on eventDate 2026-11-26; `12:15 closed` on eventDate 2026-11-27 | `CME-SVC-2026-11-25` | T2 | Thanksgiving Day publishes no final close, so this trade date owns the span from Wednesday evening and its own regular session is cut at 12:15 CT |
| 2026-12-24 | early close | `12:15 closed` - 12:15 CT | `CME-SVC-2026-12-22` | T2 | eventDate 2026-12-24, CME trade date 2026-12-24; no evening re-open |
| 2026-12-25 | closed | `no events published` | `CME-SVC-2026-12-24` | T2 | eventDate 2026-12-25, no CME trade date; note N1 |

**Interpretive steps, 2026.** Good Friday 2026 is the one date CME flags itself, and it is the only row in this window corroborated at T1. From the operator page: "Due to the US Employment Situation Release on April 3, 2026, CME Group Equities, FX, Cryptocurrency and Interest Rate products will have unique Closes and Settlements for trade date April 3rd", and, of everything else, "No trading for Friday April 3th  trade date in observence of Good Friday." (sic, twice). That page states no instants; the 08:15 CT close comes from the service alone. 08:15 CT is earlier than the family's own 08:30 CT regular open, so the row removes the whole day session and leaves only the Thursday-evening leg.

On 2026-06-19 and 2026-07-03 the `12:00 closed` event carries trade date 2026-06-22 and 2026-07-06 - the following Monday, not the Friday. In CME's terms the holiday has no trade date of its own. The crate cannot say that and keep the trading: a `closed` row would delete the Thursday-evening-through-Friday-noon span that did trade. The row is an early close on the Friday, and the divergence from the operator's printed trade date is recorded here. This is the same shape as the Monday holidays above and is resolved the same way.

2026-12-31 prints `16:00 closed` with no evening re-open: the normal grid minus the leg that `closed` on 2027-01-01 already removes. No row.

**Gaps, 2026.**

- **executable, was not representable — resolved 2026-09-25 UTC by Stage 4 (#116).** The Saturday sessions of 2026-06-20 and 2026-07-04, `05:00 open; 17:00 closed`, carrying trade dates 2026-06-22 and 2026-07-06, on a week whose normal grid has no Saturday session. The block rows #93 shipped supply the vocabulary and this table now states both trade dates, so the crate no longer reports these Saturdays closed. The rows state the **complete** trade date — the Saturday from `CME-SVC-2026-06-18` / `CME-SVC-2026-07-03`, and the Sunday Pre-Open and Sunday-17:00-to-Monday-16:00 envelope from `CME-SVC-2026-06-21` / `CME-SVC-2026-07-03`; the 2026-06-18 window stops at the Saturday and prints no Sunday entry at all, while the 2026-07-03 window runs through its Sunday and prints both halves. Saturday 2026-04-04, after Good Friday 2026, carries no such session - checked against the service for window 2026-04-02..04.
- **order entry, not representable.** 2026-01-01 repeats the 16:00 CT pre-open of the 2025 New Year rows.

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `no events published` | `CME-SVC-2026-12-31` | T2 | eventDate 2027-01-01, no CME trade date; note N1 |
| 2027-01-18 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2027-01-17` | T2 | eventDate 2027-01-18, CME trade date 2027-01-19; note N15 |
| 2027-01-19 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2027-01-17; `12:00 preopen; 17:00 open` on eventDate 2027-01-18; `16:00 closed` on eventDate 2027-01-19 | `CME-SVC-2027-01-17` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it carries 2027-01-19; it still ends at the holiday's own 12:00 CT close, where the queue then opens. Both matching envelopes are split at the family's regular boundaries, so the holiday's clipped morning and the trade date's own 08:30-15:15 regular session both keep answering |
| 2027-02-15 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2027-02-14` | T2 | eventDate 2027-02-15, CME trade date 2027-02-16; note N15 |
| 2027-02-16 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2027-02-14; `12:00 preopen; 17:00 open` on eventDate 2027-02-15; `16:00 closed` on eventDate 2027-02-16 | `CME-SVC-2027-02-14` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it carries 2027-02-16; it still ends at the holiday's own 12:00 CT close, where the queue then opens. Both matching envelopes are split at the family's regular boundaries, so the holiday's clipped morning and the trade date's own 08:30-15:15 regular session both keep answering |
| 2027-03-26 | closed | `no events published` | `CME-SVC-2027-03-25` | T2 | eventDate 2027-03-26, no CME trade date; note N1 |
| 2027-05-31 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2027-05-30` | T2 | eventDate 2027-05-31, CME trade date 2027-06-01; note N15 |
| 2027-06-01 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2027-05-30; `12:00 preopen; 17:00 open` on eventDate 2027-05-31; `16:00 closed` on eventDate 2027-06-01 | `CME-SVC-2027-05-30` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it carries 2027-06-01; it still ends at the holiday's own 12:00 CT close, where the queue then opens. Both matching envelopes are split at the family's regular boundaries, so the holiday's clipped morning and the trade date's own 08:30-15:15 regular session both keep answering |
| 2027-06-18 | early close | `12:00 closed` - 12:00 CT | `CME-SVC-2027-06-17` | T2 | eventDate 2027-06-18, CME trade date 2027-06-21; see the 2027 interpretive steps |
| 2027-06-21 | replacement blocks | `16:45 preopen; 17:00 open` on eventDate 2027-06-17 and `12:00 closed` on eventDate 2027-06-18 (`CME-SVC-2027-06-17`); `05:00 open; 17:00 closed` on eventDate 2027-06-19 (`CME-SVC-2027-06-17`); `16:00 preopen; 17:00 open` on eventDate 2027-06-20 and `16:00 closed` on eventDate 2027-06-21 (`CME-SVC-2027-06-20`), all CME trade date 2027-06-21 | `CME-SVC-2027-06-17` | T2 | the complete trade date, as 2026-06-22: the Thursday queue and the session the Friday noon close ends, plus the Saturday session from the first window, and the Sunday Pre-Open, the Sunday-17:00 open and the Monday final close from the second — the first window stops at the Saturday and prints no Sunday entry at all |
| 2027-07-05 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2027-07-04` | T2 | eventDate 2027-07-05, CME trade date 2027-07-06; note N15 |
| 2027-07-06 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2027-07-04; `12:00 preopen; 17:00 open` on eventDate 2027-07-05; `16:00 closed` on eventDate 2027-07-06 | `CME-SVC-2027-07-04` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it carries 2027-07-06; it still ends at the holiday's own 12:00 CT close, where the queue then opens. Both matching envelopes are split at the family's regular boundaries, so the holiday's clipped morning and the trade date's own 08:30-15:15 regular session both keep answering |
| 2027-09-06 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2027-09-05` | T2 | eventDate 2027-09-06, CME trade date 2027-09-07; note N15 |
| 2027-09-07 | replacement blocks | `16:00 preopen; 17:00 open` on eventDate 2027-09-05; `12:00 preopen; 17:00 open` on eventDate 2027-09-06; `16:00 closed` on eventDate 2027-09-07 | `CME-SVC-2027-09-05` | T2 | the holiday publishes no final close for its own trade date, so the span from the evening before it carries 2027-09-07; it still ends at the holiday's own 12:00 CT close, where the queue then opens. Both matching envelopes are split at the family's regular boundaries, so the holiday's clipped morning and the trade date's own 08:30-15:15 regular session both keep answering |
| 2027-11-25 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2027-11-24` | T2 | eventDate 2027-11-25, CME trade date 2027-11-26; note N15 |
| 2027-11-26 | replacement blocks | `16:45 preopen; 17:00 open` on eventDate 2027-11-24; `12:00 preopen; 17:00 open` on eventDate 2027-11-25; `12:15 closed` on eventDate 2027-11-26 | `CME-SVC-2027-11-24` | T2 | Thanksgiving Day publishes no final close, so this trade date owns the span from Wednesday evening and its own regular session is cut at 12:15 CT |
| 2027-12-24 | closed | `no events published` | `CME-SVC-2027-12-22` | T2 | eventDate 2027-12-24, no CME trade date; note N1 |

**Interpretive steps, 2027.** 2027-03-25 and 2027-12-23 each print `16:00 closed` and no evening re-open - the normal grid minus a leg that the `closed` row on the following trade date (2027-03-26, 2027-12-24) already removes. Neither ships a row. CME keys its Christmas 2027 holiday to Thursday 2027-12-23; Globex is shut on Friday 2027-12-24 and 25 December falls on a Saturday, so the crate's row is the Friday.

2027-06-18 repeats the 2026 Friday-holiday shape: `12:00 closed` carrying trade date 2027-06-21. The row is an early close on the Friday, for the reason given under 2026.

2027-12-31 is a normal Friday - a 16:00 CT final close, and the normal Friday grid has no evening leg either - so it ships no row.

**Gaps, 2027.**

- **executable, was not representable — resolved 2026-09-25 UTC by Stage 4 (#116).** The Saturday session of 2027-06-19, `05:00 open; 17:00 closed`, carrying trade date 2027-06-21. Same class as the two 2026 Saturdays, and now a row on the same terms: the Saturday from `CME-SVC-2027-06-17` and the Sunday Pre-Open, Sunday-17:00 open and Monday final close from `CME-SVC-2027-06-20`, which the Saturday window does not print.
- **coverage edge.** CME's published future runs to 2028-01-01 and the table stops at 2027-12-31, so trade date 2028-01-03 - the first trading day of 2028 - is outside coverage and the crate has no holiday answer for it. 2028-01-01 itself is a Saturday on which CME publishes nothing, which is the ordinary Saturday answer.
- **no T1 rendering.** No per-asset-class T1 rendering could be driven out of the operator page for any holiday in this window except Thanksgiving 2026, which was used to validate the service group by group. Every row here is T2.
