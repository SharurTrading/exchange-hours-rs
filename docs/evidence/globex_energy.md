<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_energy` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`energy_metals.rs`](../../src/calendar/schedules/futures/us/energy_metals.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Named NYMEX energy/PGM and COMEX metals roots only; TAS/TAM/BTIC, options, and different product clocks are excluded. Current matching and queues are exact and the 2015 close revision is dated; historical selectors omit the queues because the Sunday 16:15→16:00 move is bracketed to 2012-05-28..2012-06-07 by CME's own trading-hours captures, with both CME notice channels read in full across that window and silent on it. Dated profiles now carry the sourced Sunday 16:15–17:00 intersection from the January-2010 floor, so only the 16:00–16:15 quarter-hour remains withheld.

## Revision rows

- 2015-09-20 — T1 — CME Globex notice 20150907 — every COMEX and NYMEX close moves to 16:00 CT for trade date Monday 2015-09-21.
- 2026-08-22 — T1 — 2026-08-22 review: verified current, onset undated — knowledge-bound row widening the Sunday queue to the sourced current 16:00–17:00 CT Pre-Open.

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
- **scope** — named NYMEX energy/PGM and COMEX metals roots only; TAS/TAM/BTIC, options and different product clocks are excluded.

## Evidence documents

Every id below resolves to one saved artifact behind this file's holiday rows.
The 2010-2012 ids are CME Group's own holiday-calendar PDFs at tier T1, retrieved
through the Internet Archive and saved; the 2025-2027 ids are responses of CME's
own `trading-hours-by-product` service at tier T2. Each id's row resolves it to the URL it was read at — an Internet Archive
raw replay for a saved capture, the operator's own endpoint for a live retrieval — and to
the capture time in UTC, the tier and the sha256, as the design memo's section 3.2
requires. The byte counts and, for a bundle member, the artifact's path inside the bundle
are in the research store's `holidays/raw/` indexes.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |

|---|---|---|---|---|---|
| `2010-new-years.pdf @2010-02-15T05:16:52Z` | `2010-new-years.pdf` | <https://web.archive.org/web/20100215051652id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-new-years.pdf> | archive capture 2010-02-15T05:16:52Z | T1 | `c30a6cef73fca23c54b25907f307ad52a2922d1e4b76c0a12de126dc6fc31a6d` |
| `2022-mlk-day-holiday-schedule.xls @2022-01-17T21:22:30Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220117212230id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-mlk-day-holiday-schedule.xls> | archive capture 2022-01-17T21:22:30Z | T1 | `896944fa701062e2e1ee305f8adb696ba8ab03655a96874c7885f2328177626e` |
| `2022-presidents-day-holiday-schedule.xls @2022-07-04T07:38:10Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704073810id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-presidents-day-holiday-schedule.xls> | archive capture 2022-07-04T07:38:10Z | T1 | `07932975d04ccabad0fb53f33f946f1a6fd8a696bafdb21f95f030ede9a84516` |
| `2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065501id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-good-friday-holiday-schedule.xls> | archive capture 2022-07-04T06:55:01Z | T1 | `a82936ab14d1b1f7041583123289c4de401c66ace4eea7e90fa9f60c1a3f3b7e` |
| `2022-memorial-day-holiday-schedule.xls @2022-07-04T06:54:38Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065438id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-memorial-day-holiday-schedule.xls> | archive capture 2022-07-04T06:54:38Z | T1 | `0d1b1f89a315cae22a5857a7027a514e3086c1cccfee42fddddff3caba0c2230` |
| `2022-juneteenth-holiday-schedule.xls @2022-06-20T20:02:10Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220620200210id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-juneteenth-holiday-schedule.xls> | archive capture 2022-06-20T20:02:10Z | T1 | `bc9f2caf26a73a13029f177fcc6468bdc8662899ff935f44329ee1a95c8b667b` |
| `2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065450id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-independence-day-holiday-schedule.xls> | archive capture 2022-07-04T06:54:50Z | T1 | `1ea0459d8aa0fd7ec5147614855f6d0d43607efefdc3aa9c6b1e18ddbfd54fde` |
| `2022-labor-day-holiday-schedule.xls @2022-07-04T06:54:41Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065441id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-labor-day-holiday-schedule.xls> | archive capture 2022-07-04T06:54:41Z | T1 | `28d533f25d1af74af043e411635f1933fd4ff6359c80c049f87f019d2f1d57d0` |
| `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20221122060801id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-thanksgiving-holiday-schedule.xls> | archive capture 2022-11-22T06:08:01Z | T1 | `64341a65350de982a6a05760504132e173611151ae3164c6f9437509a406edcb` |
| `2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065430id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-christmas-holiday-schedule.xls> | archive capture 2022-07-04T06:54:30Z | T1 | `2dd1d531514989845dcb6ce6d767db5dd3f956ac6777ab1cfe36d6843f3762e7` |
| `2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065501id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2023-new-years-holiday-schedule.xls> | archive capture 2022-07-04T06:55:01Z | T1 | `eefafd1066f406edbe6167ddf8ad13c0697b124a337c0893ce5a5da200c783d3` |
| `CME-SVC-2023-01-15` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161438id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2023-01-15&toEventDate=2023-01-17&isProtected&_t=1720455278636> | archive capture 2024-07-08T16:14:38Z | T2 | `507fd196a7654ddd916218b2aa24a146eeaca4e1f7cb7daded4e7a9c689cda82` |
| `CME-SVC-2023-02-19` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161438id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2023-02-19&toEventDate=2023-02-21&isProtected&_t=1720455278640> | archive capture 2024-07-08T16:14:38Z | T2 | `d063238a83e8cb84d4484a86b26cb1d976f1ccfdb91972eecc08dcc0cec49412` |
| `CME-SVC-2023-04-06` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161438id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2023-04-06&toEventDate=2023-04-08&isProtected&_t=1720455278642> | archive capture 2024-07-08T16:14:38Z | T2 | `0543d5f6d2efd4aa6f4132ce9f5425b9ba9f64de08eccd677b78d6b43c6e9d4f` |
| `memorial-day-2023.pdf @2023-04-20T22:40:18Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20230420224018id_/https://www.cmegroup.com/trading-hours/files/memorial-day-2023.pdf> | archive capture 2023-04-20T22:40:18Z | T1 | `7657bc8089ca669cfd244c2e3e697b47a7e650f5d957efe5b32da001622acb35` |
| `juneteenth-2023.pdf @2023-06-13T18:59:49Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20230613185949id_/https://www.cmegroup.com/trading-hours/files/juneteenth-2023.pdf> | archive capture 2023-06-13T18:59:49Z | T1 | `831f7f63e197ce58436780830aa515cb08dcf92bdf82a540265c2d21a4bffa43` |
| `4th-of-july-2023.pdf @2023-06-27T12:50:57Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20230627125057id_/https://www.cmegroup.com/trading-hours/files/4th-of-july-2023.pdf> | archive capture 2023-06-27T12:50:57Z | T1 | `ccbc1f1fc39ba2219fd748372faa985798fee7eaffabf69f08956f5465a769e3` |
| `labor-day-2023.pdf @2023-08-02T19:24:46Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20230802192446id_/https://www.cmegroup.com/trading-hours/files/labor-day-2023.pdf> | archive capture 2023-08-02T19:24:46Z | T1 | `39a4c075437fdc7134166328eca730e3cabf3bea00ab369466dff63c99f8e948` |
| `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20231203205929id_/https://www.cmegroup.com/trading-hours/files/thanksgiving-day-2023.pdf> | archive capture 2023-12-03T20:59:29Z | T1 | `99e187b3f3899e1062d662e148d5978e0cd075b961e6fc79550d4393812307e8` |
| `christmas-day-2023.pdf @2026-07-19T09:52:48Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20260719095248id_/https://www.cmegroup.com/trading-hours/files/christmas-day-2023.pdf> | archive capture 2026-07-19T09:52:48Z | T1 | `edcde0fcf61d3414cee2a332453db861e0e2d7edf81d89a5f379c44b2e72d5cf` |
| `new-years-day-2024.pdf @2026-08-11T16:57:16Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20260811165716id_/https://www.cmegroup.com/trading-hours/files/new-years-day-2024.pdf> | archive capture 2026-08-11T16:57:16Z | T1 | `34e60f8c97623df30e00f0ad8e4eeda20b99001b6c35d64f735828fecec5b9b8` |
| `CME-SVC-2024-01-14` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-01-14&toEventDate=2024-01-16&isProtected&_t=1720455278663> | archive capture 2024-07-08T16:14:39Z | T2 | `a8fe0f3eed4c67939d3add5fa656173bc4119c3765174b084c0622a1934e0d22` |
| `CME-SVC-2024-02-18` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-02-18&toEventDate=2024-02-20&isProtected&_t=1720455278669> | archive capture 2024-07-08T16:14:39Z | T2 | `af5ddb57cfa8bd6d377a0784fcdd33589e2055ede0a09b7fe4ae3e41d0d20f9a` |
| `CME-SVC-2024-03-28` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-03-28&toEventDate=2024-03-30&isProtected&_t=1720455278672> | archive capture 2024-07-08T16:14:39Z | T2 | `9b41709219e36f56296843fe589e362a7513e9a98132a682a1b956f2e40c593b` |
| `CME-SVC-2024-05-26` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-05-26&toEventDate=2024-05-28&isProtected&_t=1720455278675> | archive capture 2024-07-08T16:14:39Z | T2 | `ccff9685aac5670c5eb4a7e2b86cf1d78a5c2c08325cd03673ab8c1e6f560138` |
| `CME-SVC-2024-06-18` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-06-18&toEventDate=2024-06-20&isProtected&_t=1720455278677> | archive capture 2024-07-08T16:14:39Z | T2 | `57acecac3e1ad1a50dda8e3bcd6ca926b4d4c6c2ebc9d8324eb73332ed6a20ec` |
| `CME-SVC-2024-07-03` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-07-03&toEventDate=2024-07-05&isProtected&_t=1720455278680> | archive capture 2024-07-08T16:14:39Z | T2 | `6ef0e2b055c349249f5cc7ea7c98132a31111a01ce7af028e13584c0f82d4d38` |
| `CME-SVC-2024-09-01` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-09-01&toEventDate=2024-09-03&isProtected&_t=1720455278683> | archive capture 2024-07-08T16:14:39Z | T2 | `bbe6c78555cba0437fe4b9c6eac0063983ecae49056877db618ed2b29c109546` |
| `CME-SVC-2024-11-27` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-11-27&toEventDate=2024-11-29&isProtected&_t=1720455278685> | archive capture 2024-07-08T16:14:39Z | T2 | `f6a15f26991d25f8c6821fa6c967d0e76b2ad3f775bed62a2a001ec2fd7e389a` |
| `CME-SVC-2024-12-24` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-12-24&toEventDate=2024-12-26&isProtected&_t=1734710019537> | archive capture 2024-12-20T15:53:40Z | T2 | `183c85160e31d53fde30c422048b8e778937d8866f3ea1c45e2dd570ea9db1c8` |
| `2010-martin-luther-king.pdf @2010-03-31T06:42:26Z` | `2010-martin-luther-king.pdf` | <https://web.archive.org/web/20100331064226id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-martin-luther-king.pdf> | archive capture 2010-03-31T06:42:26Z | T1 | `11af96423f788565a434209ebb94a82f69dd2c129621927cd898c2ff6c0978d3` |
| `2010-presidents-day.pdf @2010-02-15T06:46:41Z` | `2010-presidents-day.pdf` | <https://web.archive.org/web/20100215064641id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-presidents-day.pdf> | archive capture 2010-02-15T06:46:41Z | T1 | `ba379a7fa57efef43820583ada0002ea6cd8ccf0caf1b650d1cb6e8561f84253` |
| `2010-good-friday.pdf @2010-06-01T11:19:16Z` | `2010-good-friday.pdf` | <https://web.archive.org/web/20100601111916id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-good-friday.pdf> | archive capture 2010-06-01T11:19:16Z | T1 | `d196ca746c20ecd416d38f8f95020e2e7d6cb7fa9ead089e0d58c88bed0ab1f5` |
| `2010-memorial-day.pdf @2010-06-01T09:42:25Z` | `2010-memorial-day.pdf` | <https://web.archive.org/web/20100601094225id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-memorial-day.pdf> | archive capture 2010-06-01T09:42:25Z | T1 | `46a2f00f0f23c82189d86953092ee1171e0ec1c460891bda3ef0578518d12859` |
| `2010-4th-of-july.pdf @2010-06-02T00:56:37Z` | `2010-4th-of-july.pdf` | <https://web.archive.org/web/20100602005637id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-4th-of-july.pdf> | archive capture 2010-06-02T00:56:37Z | T1 | `c6a0f8c0b079b85e4500d30d942247bc9aa14d6c70fe3ed73d6d77b21b65ee2c` |
| `2010-labor-day.pdf @2010-06-02T00:56:41Z` | `2010-labor-day.pdf` | <https://web.archive.org/web/20100602005641id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-labor-day.pdf> | archive capture 2010-06-02T00:56:41Z | T1 | `2aecfe737c9613f82b975a01812607e96c9284638e8bd9b9bd403edfae510620` |
| `2010-columbus-day.pdf @2010-08-21T13:31:22Z` | `2010-columbus-day.pdf` | <https://web.archive.org/web/20100821133122id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-columbus-day.pdf> | archive capture 2010-08-21T13:31:22Z | T1 | `e8fbf61d914ebd3cd74de611a7b2d9a4d13a67e1233045ccc2fcbb57556be3f9` |
| `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | `2010-thanksgiving.pdf` | <https://web.archive.org/web/20101122094012id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-thanksgiving.pdf> | archive capture 2010-11-22T09:40:12Z | T1 | `4732afab4ca78ce21b3640f8ac41ced714123179c7cee1cb2b8c044bf9f2e2b5` |
| `2010-christmas.pdf @2010-12-14T06:12:38Z` | `2010-christmas.pdf` | <https://web.archive.org/web/20101214061238id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-christmas.pdf> | archive capture 2010-12-14T06:12:38Z | T1 | `d4adb060f6eb592fb24e3a272db57b3d8c9d69f3f370682ecd8d57e4169c37be` |
| `2011-new-years.pdf @2011-11-01T14:39:45Z` | `2011-new-years.pdf` | <https://web.archive.org/web/20111101143945id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-new-years.pdf> | archive capture 2011-11-01T14:39:45Z | T1 | `42c289804cd3fa0830556ecb7fcc31493c452ba9e7325ebe7d7ce29613e41476` |
| `2011-martin-luther-king.pdf @2011-10-28T02:34:29Z` | `2011-martin-luther-king.pdf` | <https://web.archive.org/web/20111028023429id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-martin-luther-king.pdf> | archive capture 2011-10-28T02:34:29Z | T1 | `2e389e2688d6760705220a11657329e76a9eb3a88d78664b4775eb7481be7b17` |
| `2011-presidents-day.pdf @2011-10-28T02:35:16Z` | `2011-presidents-day.pdf` | <https://web.archive.org/web/20111028023516id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-presidents-day.pdf> | archive capture 2011-10-28T02:35:16Z | T1 | `0342359e135acada5cfaa1b74f806477f759924a0b40625e035242c7e81321d4` |
| `2011-good-friday.pdf @2011-10-28T02:37:07Z` | `2011-good-friday.pdf` | <https://web.archive.org/web/20111028023707id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-good-friday.pdf> | archive capture 2011-10-28T02:37:07Z | T1 | `6cf10359bb438eb49287dcef7c1e75a484df4d6e3b538fa9ee59dc3832210bda` |
| `2011-memorial-day.pdf @2013-09-30T10:56:52Z` | `2011-memorial-day.pdf` | <https://web.archive.org/web/20130930105652id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-memorial-day.pdf> | archive capture 2013-09-30T10:56:52Z | T1 | `5482f7bf47e0ee61448cf5f60fd4a5373cc39cb0e46220150c1f6a2ab2d6caec` |
| `2011-4th-of-july.pdf @2011-11-01T14:40:54Z` | `2011-4th-of-july.pdf` | <https://web.archive.org/web/20111101144054id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-4th-of-july.pdf> | archive capture 2011-11-01T14:40:54Z | T1 | `4c3bfbbe927ed799006edce76b5f263b6299bab29be7ff1eb06326e184a9b443` |
| `2011-labor-day.pdf @2011-11-01T14:43:45Z` | `2011-labor-day.pdf` | <https://web.archive.org/web/20111101144345id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-labor-day.pdf> | archive capture 2011-11-01T14:43:45Z | T1 | `03f38fea761a6da7633c3e636a40de61431c866a87f2229f270685bf9bb4470b` |
| `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | `2011-thanksgiving.pdf` | <https://web.archive.org/web/20111124185246id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-thanksgiving.pdf> | archive capture 2011-11-24T18:52:46Z | T1 | `bf75c3e0d3e18cbb8599458574bc7232513b737a664582454a7b34ca2b6caeb5` |
| `2011-christmas.pdf @2012-01-25T02:05:48Z` | `2011-christmas.pdf` | <https://web.archive.org/web/20120125020548id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-christmas.pdf> | archive capture 2012-01-25T02:05:48Z | T1 | `a0d34878fd70534afb2e0a2585a04ce1efc8c4aa0451575266cfb5f9dcf08029` |
| `2012-new-years.pdf @2012-01-25T02:54:30Z` | `2012-new-years.pdf` | <https://web.archive.org/web/20120125025430id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-new-years.pdf> | archive capture 2012-01-25T02:54:30Z | T1 | `aa8593edfde40a70ce2ab4818cd8984082eaeaa4a4893e2a8f3e3dc9ef4b1347` |
| `2012-martin-luther-king.pdf @2012-05-05T16:15:26Z` | `2012-martin-luther-king.pdf` | <https://web.archive.org/web/20120505161526id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-martin-luther-king.pdf> | archive capture 2012-05-05T16:15:26Z | T1 | `458c94af0fd7b7d8445c229de6bbe05648ea588c2fea88180eefe46486e4d265` |
| `2012-presidents-day.pdf @2012-05-05T16:15:39Z` | `2012-presidents-day.pdf` | <https://web.archive.org/web/20120505161539id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-presidents-day.pdf> | archive capture 2012-05-05T16:15:39Z | T1 | `4b5daad7813d41cdd9a104cca3a656bf303672f03c6c51ff5a13771d840ef131` |
| `2012-good-friday.pdf @2012-04-17T00:42:47Z` | `2012-good-friday.pdf` | <https://web.archive.org/web/20120417004247id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-good-friday.pdf> | archive capture 2012-04-17T00:42:47Z | T1 | `81440c44afb97ea4b3a44b86aa4cf21e2e4cb7ba5839fabd95b29d0c928b2ea8` |
| `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | `2012-memorial-day.pdf` | <https://web.archive.org/web/20120915003714id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-memorial-day.pdf> | archive capture 2012-09-15T00:37:14Z | T1 | `5dc5cf9883232978ec1e80bd5bd50a2043535e93d7e52acddf4fd7e68938e848` |
| `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | `2012-4th-of-july.pdf` | <https://web.archive.org/web/20120915003923id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-4th-of-july.pdf> | archive capture 2012-09-15T00:39:23Z | T1 | `9b35b802ff0e399226ac0811761fc7e03487d8dec401c19a7e383750cbca5faf` |
| `2012-labor-day.pdf @2012-09-15T00:34:37Z` | `2012-labor-day.pdf` | <https://web.archive.org/web/20120915003437id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-labor-day.pdf> | archive capture 2012-09-15T00:34:37Z | T1 | `2f951bede1d6084977c8bed2f1cc4c993ebedd904aa0d56027668d2cb808ee39` |
| `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | `2012-thanksgiving.pdf` | <https://web.archive.org/web/20130127223901id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-thanksgiving.pdf> | archive capture 2013-01-27T22:39:01Z | T1 | `052e381bbd4eb0790c6d38e3866874738d6da081da62643e525c25674b2608e1` |
| `2012-christmas.pdf @2013-04-14T19:40:27Z` | `2012-christmas.pdf` | <https://web.archive.org/web/20130414194027id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-christmas.pdf> | archive capture 2013-04-14T19:40:27Z | T1 | `de3b16aaae2ef887e46c965f902d8d0e43afa6e18dc1f721baaa40ea6b18b5e9` |
| `2016-new-years-holiday-schedule.pdf @2016-01-08` | `2016-new-years-holiday-schedule.pdf` | <https://web.archive.org/web/20160108203007id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-new-years-holiday-schedule.pdf> | archive capture 2016-01-08T20:30:07Z | T1 | `118196a469dd40ad6a40594da273f726f6cb3e503f9cc1b8fd2057cf2fa32c61` |
| `2016-holiday-calendars.zip#2016-martin-luther-king-holiday-schedule.pdf @2017-06-28` | `2016-martin-luther-king-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-martin-luther-king-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `0fac2d08a84f8e9a637573438adfab33ba91c69e76b6dce01f0ca4cbcde8aff1` |
| `2016-holiday-calendars.zip#2016-presidents-day-holiday-schedule.pdf @2017-06-28` | `2016-presidents-day-holiday-schedule.pdf` | <https://web.archive.org/web/20151203081738id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-presidents-day-holiday-schedule.pdf> | archive capture 2015-12-03T08:17:38Z | T1 | `b5264dd479efd69df495de170cdd58326f64e9e31a2ed7cca6d7d96cc424c9c2` |
| `2016-holiday-calendars.zip#2016-good-friday-holiday-schedule.pdf @2017-06-28` | `2016-good-friday-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-good-friday-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `632c4e3e451fd972e4cccb743f7dfa7a41896044cafc0e82953e0c6fb6d0bc42` |
| `2016-holiday-calendars.zip#2016-memorial-day-holiday-schedule.pdf @2017-06-28` | `2016-memorial-day-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-memorial-day-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `03fa4af1988ae7a54c6e5426b83a76e54825ac4398f7a2be6e77be82fe8ab9f9` |
| `2016-holiday-calendars.zip#2016-4th-of-july-holiday-schedule.pdf @2017-06-28` | `2016-4th-of-july-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-4th-of-july-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `533a49be68619dbd003b3e749485be46051018d3613d44a3043bf8a087fec9ad` |
| `2016-holiday-calendars.zip#2016-labor-day-holiday-schedule.pdf @2017-06-28` | `2016-labor-day-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-labor-day-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `a4c0676ad117a63894b84637e8e5a7734ed17d7e2d1ee09d5a96370c7fd53b4a` |
| `2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28` | `2016-thanksgiving-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-thanksgiving-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `b3c4dbe2d60bf5530316653d381ba28f1eb4ed6ec55844dca976b1852941c2b1` |
| `2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28` | `2016-christmas-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2016-christmas-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `42efeb845763d3d8a288dd9f9349bfcaf88badebf49ee1c6bc645a72ec01d252` |
| `2016-holiday-calendars.zip#2017-new-years-holiday-schedule.pdf @2017-06-28` | `2017-new-years-holiday-schedule.pdf` | <https://web.archive.org/web/20170628id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2017-new-years-holiday-schedule.pdf> | 2017-06-28T11:58:19Z | T1 | `493e9cd3fb8ae7ce8b059f30516dedd83631a101e63ecd4c9201fa24c8a2aae8` |
| `2017-martin-luther-king-holiday-schedule.xls @2017-06-28` | `2017-martin-luther-king-holiday-schedule.xls` | <https://web.archive.org/web/20170628172556id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2017-martin-luther-king-holiday-schedule.xls> | archive capture 2017-06-28T17:25:56Z | T1 | `c11937ee9995dd712f23631dd235571d87d2c387bc21e074cc87905b68980d58` |
| `2017-presidents-day-holiday-schedule.xls @2017-06-28` | `2017-presidents-day-holiday-schedule.xls` | <https://web.archive.org/web/20170628174148id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2017-presidents-day-holiday-schedule.xls> | archive capture 2017-06-28T17:41:48Z | T1 | `2ec7b623faa8f17ab5946091492be0dacbd29e6446ab3789e0ba4fa5ccd58b54` |
| `2017-good-friday-holiday-schedule.xls @2017-05-05` | `2017-good-friday-holiday-schedule.xls` | <https://web.archive.org/web/20170505072315id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2017-good-friday-holiday-schedule.xls> | archive capture 2017-05-05T07:23:15Z | T1 | `bdc8775f7061ae1e377421fe7a951446c0066ba11d1dbfc7bebba9e7c9415ccb` |
| `2017-memorial-day-holiday-schedule.xls @2017-10-25` | `2017-memorial-day-holiday-schedule.xls` | <https://web.archive.org/web/20170505072406id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2017-memorial-day-holiday-schedule.xls> | archive capture 2017-05-05T07:24:06Z | T1 | `2d5a64e7ba05c1889993e91cde352312a8fb5901c5f6a4d53439002ab067542a` |
| `2017-4th-of-july-holiday-schedule.xls @2017-10-25` | `2017-4th-of-july-holiday-schedule.xls` | <https://web.archive.org/web/20170505072205id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2017-4th-of-july-holiday-schedule.xls> | archive capture 2017-05-05T07:22:05Z | T1 | `d8d668183f686591879e8ee0d2db13c650c09d5a3c13e2378be068c784679453` |
| `2017-labor-day-holiday-schedule.xls @2017-10-25` | `2017-labor-day-holiday-schedule.xls` | <https://web.archive.org/web/20170505072346id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2017-labor-day-holiday-schedule.xls> | archive capture 2017-05-05T07:23:46Z | T1 | `660473ca09893868560b64e544c31c984a715595b1311f136af389951ef56f63` |
| `2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26` | `2017-thanksgiving-holiday-schedule.xls` | <https://web.archive.org/web/20210126id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2017-thanksgiving-holiday-schedule.xls> | 2021-01-26T09:48:35Z | T1 | `b6bc3dba9f1e0c5a86d543e389d15cea6006f456e6dc2cfd7813eae31db2f869` |
| `2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26` | `2017-christmas-holiday-schedule.xls` | <https://web.archive.org/web/20210126id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2017-christmas-holiday-schedule.xls> | 2021-01-26T09:48:35Z | T1 | `bfb291c9c08764ee2bcf0327690b98c58cbb4cf493723c7b1464eccb0d062d30` |
| `2018-new-years-holiday-schedule.xls @2018-01-06` | `2018-new-years-holiday-schedule.xls` | <https://web.archive.org/web/20170505072653id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2018-new-years-holiday-schedule.xls> | archive capture 2017-05-05T07:26:53Z | T1 | `73332758cbf95363a04cf422a672157c1b1a5bb34d4c9bd496bd7b7491a4d8cd` |
| `2018-martin-luther-king-holiday-schedule.xls @2018-05-08` | `2018-martin-luther-king-holiday-schedule.xls` | <https://web.archive.org/web/20180106225650id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2018-martin-luther-king-holiday-schedule.xls> | archive capture 2018-01-06T22:56:50Z | T1 | `81f30b0be680898b82175b19ee6a2fccc85b7e340fac2418b0ad00973c4a93a8` |
| `2018-presidents-day-holiday-schedule.xls @2018-05-08` | `2018-presidents-day-holiday-schedule.xls` | <https://web.archive.org/web/20180106225732id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2018-presidents-day-holiday-schedule.xls> | archive capture 2018-01-06T22:57:32Z | T1 | `fddc713823c58cfdc4de2486ea3b2953ecda24c01579da64eb3933d8a7efc4b8` |
| `2018-holiday-calendars.zip#2018-good-friday-holiday-schedule.xls @2026-08-30` | `2018-good-friday-holiday-schedule.xls` | <https://web.archive.org/web/20260830id_/http://www.cmegroup.com:80/tools-information/holiday-calendar/files/2018-good-friday-holiday-schedule.xls> | 2026-08-30T10:02:25Z | T1 | `ec3ffb0a85d061adbb53678353428eaeade9817963d6b5069db228844d9fbedf` |
| `2018-holiday-calendars.zip#2018-memorial-day-holiday-schedule.xls @2026-08-30` | `2018-memorial-day-holiday-schedule.xls` | <https://web.archive.org/web/20260830id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2018-memorial-day-holiday-schedule.xls> | 2026-08-30T10:02:25Z | T1 | `b3524e36abf39b038137145620cfe1a0d46022ce732ae623d6b71af8a4b8750b` |
| `2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30` | `2018-4th-of-july-holiday-schedule.xls` | <https://web.archive.org/web/20260830id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2018-4th-of-july-holiday-schedule.xls> | 2026-08-30T10:02:25Z | T1 | `8ed9e0f74c2bcdfdda8fa80c915e6bba5ed3c5ca7ec89476f11db03b256784ae` |
| `2018-holiday-calendars.zip#2018-labor-day-holiday-schedule.xls @2026-08-30` | `2018-labor-day-holiday-schedule.xls` | <https://web.archive.org/web/20260830id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2018-labor-day-holiday-schedule.xls> | 2026-08-30T10:02:25Z | T1 | `9b8c892dd3faba6900aae949f7f0c63b525191a76dccc938aa9859920b71b40d` |
| `2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30` | `2018-thanksgiving-holiday-schedule.xls` | <https://web.archive.org/web/20260830id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2018-thanksgiving-holiday-schedule.xls> | 2026-08-30T10:02:25Z | T1 | `31fc95f5b27f8689477b4abb557f14b2cf9ac481262926e30b78adf1455adff1` |
| `2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30` | `2018-christmas-holiday-schedule.xls` | <https://web.archive.org/web/20260830id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2018-christmas-holiday-schedule.xls> | 2026-08-30T10:02:25Z | T1 | `b97ee5f47d55c4383c7f1fa7d554eba3c7f0e8ded18343aadd06d7908d9e7a91` |
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

## Holidays

**Coverage:** 2010-01-01..2012-12-31, 2016-01-01..2018-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 (inclusive venue-local trade dates).
This wave is 33 rows over 2022-01-01..2024-12-31, **19 at T1** and **14 at T2**. The T1 rows are read from the 2022 per-asset-class workbooks, the 2023 one-pagers, and the two 2024 entries CME's own cmegroup.com worksheets `new-years-day-2024.pdf` and `christmas-day-2023.pdf` serve; the T2 rows are the operator's own `trading-hours-by-product` responses, which carry the rest of 2024 and the three 2023 holiday dates its one-pagers do not cover. 3 of the 33 are `unsourced`.
Tier: **T1** for 2010-2012 and 2016-2018, from the operator's own published
holiday schedules; **T2** for 2025-2027, from its trading-hours service. Inside a
window a date with no row is audited normal; outside every window this table has no
answer at all.

**Four audited eras, and two gaps between them.** The table declares 4 coverage windows: `2010-01-01..2012-12-31` and `2016-01-01..2018-12-31`, from the operators' own published holiday schedules at **T1**; `2022-01-01..2024-12-31`, from the 2022 workbooks and 2023 one-pagers at T1 and CME's service responses at **T2**; and `2025-01-01..2027-12-31`, from the trading-hours service at T2. The 2013-2015 and 2019-2021 intervals lie outside every declared window, so `holiday_on` has **no answer** there rather than reporting an unaudited date as normal: 2019-2021 is the unaudited stage-2.2 wave 4 and 2013-2015 is wave 5, while the eras before 2010 are out of scope below the crate's January-2010 floor. `HolidayCoverage::windows()` lists the 4, and `contains` answers per date.

### 2010

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | `CME Globex is closed` | `2010-new-years.pdf @2010-02-15T05:16:52Z` | T1 | CME prints `Jan 1` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2010-01-15 | early close | `15:15 CT` | `2010-martin-luther-king.pdf @2010-03-31T06:42:26Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Jan 15` above it, and the year comes from the document's own identity |
| 2010-01-18 | early close | `12:15 CT` | `2010-martin-luther-king.pdf @2010-03-31T06:42:26Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Jan 18` above it, and the year comes from the document's own identity |
| 2010-02-12 | early close | `15:15 CT` | `2010-presidents-day.pdf @2010-02-15T06:46:41Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Feb 12` above it, and the year comes from the document's own identity |
| 2010-02-15 | early close | `12:15 CT` | `2010-presidents-day.pdf @2010-02-15T06:46:41Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Feb 15` above it, and the year comes from the document's own identity |
| 2010-04-02 | closed | `CME Globex is closed` | `2010-good-friday.pdf @2010-06-01T11:19:16Z` | T1 | CME prints `Apr 2` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2010-05-28 | early close | `15:15 CT` | `2010-memorial-day.pdf @2010-06-01T09:42:25Z` | T1 | `15:15 CT` is the date's own final close; CME prints `May 28` above it, and the year comes from the document's own identity |
| 2010-05-31 | early close | `12:15 CT` | `2010-memorial-day.pdf @2010-06-01T09:42:25Z` | T1 | `12:15 CT` is the date's own final close; CME prints `May 31` above it, and the year comes from the document's own identity |
| 2010-07-02 | early close | `15:15 CT` | `2010-4th-of-july.pdf @2010-06-02T00:56:37Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Jul 2` above it, and the year comes from the document's own identity |
| 2010-07-05 | early close | `12:15 CT` | `2010-4th-of-july.pdf @2010-06-02T00:56:37Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Jul 5` above it, and the year comes from the document's own identity |
| 2010-09-03 | early close | `15:15 CT` | `2010-labor-day.pdf @2010-06-02T00:56:41Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Sep 3` above it, and the year comes from the document's own identity |
| 2010-09-06 | early close | `12:15 CT` | `2010-labor-day.pdf @2010-06-02T00:56:41Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Sep 6` above it, and the year comes from the document's own identity |
| 2010-10-08 | early close | `15:15 CT` | `2010-columbus-day.pdf @2010-08-21T13:31:22Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Oct 8` above it, and the year comes from the document's own identity |
| 2010-11-25 | early close | `12:15 CT` | `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Nov 25` above it, and the year comes from the document's own identity |
| 2010-11-26 | early close | `12:45 CT` | `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | T1 | `12:45 CT` is the date's own final close; CME prints `Nov 26` above it, and the year comes from the document's own identity |
| 2010-12-24 | closed | `CME Globex is closed` | `2010-christmas.pdf @2010-12-14T06:12:38Z` | T1 | CME prints `Dec 24` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2010-12-31 | early close | `15:15 CT` | `2011-new-years.pdf @2011-11-01T14:39:45Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Dec 31` above it, and the year comes from the document's own identity |

### 2011

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2011-01-14 | early close | `15:15 CT` | `2011-martin-luther-king.pdf @2011-10-28T02:34:29Z` | T1 | `15:15 CT` is the date's own final close; CME prints `Jan 14` above it, and the year comes from the document's own identity |
| 2011-01-17 | early close | `12:15 CT` | `2011-martin-luther-king.pdf @2011-10-28T02:34:29Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Jan 17` above it, and the year comes from the document's own identity |
| 2011-02-21 | early close | `12:15 CT` | `2011-presidents-day.pdf @2011-10-28T02:35:16Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Feb 21` above it, and the year comes from the document's own identity |
| 2011-04-22 | closed | `CME Globex is closed` | `2011-good-friday.pdf @2011-10-28T02:37:07Z` | T1 | CME prints `Apr 22` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2011-05-30 | early close | `12:15 CT` | `2011-memorial-day.pdf @2013-09-30T10:56:52Z` | T1 | `12:15 CT` is the date's own final close; CME prints `May 30` above it, and the year comes from the document's own identity |
| 2011-07-04 | early close | `12:15 CT` | `2011-4th-of-july.pdf @2011-11-01T14:40:54Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Jul 4` above it, and the year comes from the document's own identity |
| 2011-09-05 | early close | `12:15 CT` | `2011-labor-day.pdf @2011-11-01T14:43:45Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Sep 5` above it, and the year comes from the document's own identity |
| 2011-11-24 | early close | `12:15 CT` | `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Nov 24` above it, and the year comes from the document's own identity |
| 2011-11-25 | early close | `12:45 CT` | `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | T1 | `12:45 CT` is the date's own final close; CME prints `Nov 25` above it, and the year comes from the document's own identity |
| 2011-12-26 | closed | `CME Globex is closed` | `2011-christmas.pdf @2012-01-25T02:05:48Z` | T1 | CME prints `Dec 26` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |

### 2012

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2012-01-02 | closed | `CME Globex is closed` | `2012-new-years.pdf @2012-01-25T02:54:30Z` | T1 | CME prints `Jan 2` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2012-01-16 | early close | `12:15 CT` | `2012-martin-luther-king.pdf @2012-05-05T16:15:26Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Jan 16` above it, and the year comes from the document's own identity |
| 2012-02-20 | early close | `12:15 CT` | `2012-presidents-day.pdf @2012-05-05T16:15:39Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Feb 20` above it, and the year comes from the document's own identity |
| 2012-04-06 | closed | `CME Globex is closed` | `2012-good-friday.pdf @2012-04-17T00:42:47Z` | T1 | CME prints `Apr 6` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |
| 2012-05-28 | early close | `12:15 CT` | `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | T1 | `12:15 CT` is the date's own final close; CME prints `May 28` above it, and the year comes from the document's own identity |
| 2012-07-04 | early close | `12:15 CT` | `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Jul 4` above it, and the year comes from the document's own identity |
| 2012-09-03 | early close | `12:15 CT` | `2012-labor-day.pdf @2012-09-15T00:34:37Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Sep 3` above it, and the year comes from the document's own identity |
| 2012-11-22 | early close | `12:15 CT` | `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | T1 | `12:15 CT` is the date's own final close; CME prints `Nov 22` above it, and the year comes from the document's own identity |
| 2012-11-23 | early close | `12:45 CT` | `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | T1 | `12:45 CT` is the date's own final close; CME prints `Nov 23` above it, and the year comes from the document's own identity |
| 2012-12-24 | early close | `12:45 CT` | `2012-christmas.pdf @2013-04-14T19:40:27Z` | T1 | `12:45 CT` is the date's own final close; CME prints `Dec 24` above it, and the year comes from the document's own identity |
| 2012-12-25 | closed | `CME Globex is closed` | `2012-christmas.pdf @2013-04-14T19:40:27Z` | T1 | CME prints `Dec 25` above it as a closed day; the year comes from the document's own identity, because the annual sheets print no year |

### 2016

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `Globex closed` | `2016-new-years-holiday-schedule.pdf @2016-01-08` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2016-01-18 | early close | `1200 CT / 1300 ET / 1800 UTC` | `2016-holiday-calendars.zip#2016-martin-luther-king-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-02-15 | early close | `1200 CT / 1300 ET / 1800 UTC` | `2016-holiday-calendars.zip#2016-presidents-day-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-03-25 | closed | `no session printed` | `2016-holiday-calendars.zip#2016-good-friday-holiday-schedule.pdf @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2016-05-30 | early close | `1200 CT / 1300 ET / 1700 UTC` | `2016-holiday-calendars.zip#2016-memorial-day-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-07-04 | early close | `1200 CT / 1300 ET / 1700 UTC` | `2016-holiday-calendars.zip#2016-4th-of-july-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-09-05 | early close | `1200 CT / 1300 ET / 1700 UTC` | `2016-holiday-calendars.zip#2016-labor-day-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-11-24 | early close | `1200 CT / 1300 ET / 1800 UTC` | `2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-11-25 | early close | `1245 CT / 1345 ET / 1845 UTC` | `2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-12-26 | closed | `Globex closed` | `2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |

### 2017

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | `Globex closed` | `2016-holiday-calendars.zip#2017-new-years-holiday-schedule.pdf @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2017-01-16 | early close | `12:00 CT / 13:00 ET` | `2017-martin-luther-king-holiday-schedule.xls @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-02-20 | early close | `12:00 CT / 13:00 ET` | `2017-presidents-day-holiday-schedule.xls @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-04-14 | closed | `no session printed` | `2017-good-friday-holiday-schedule.xls @2017-05-05` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2017-05-29 | early close | `12:00 CT / 13:00 ET` | `2017-memorial-day-holiday-schedule.xls @2017-10-25` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-07-04 | early close | `12:00 CT / 13:00 ET` | `2017-4th-of-july-holiday-schedule.xls @2017-10-25` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-09-04 | early close | `12:00 CT / 13:00 ET` | `2017-labor-day-holiday-schedule.xls @2017-10-25` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-11-23 | early close | `12:00 CT / 13:00 ET` | `2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-11-24 | early close | `12:45 CT / 13:45 ET` | `2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-12-25 | closed | `no session printed` | `2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26` | T1 | CME prints the closure for this date; trade date = the operator's event date |

### 2018

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `no session printed` | `2018-new-years-holiday-schedule.xls @2018-01-06` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2018-01-15 | early close | `12:00 CT / 13:00 ET` | `2018-martin-luther-king-holiday-schedule.xls @2018-05-08` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-02-19 | early close | `12:00 CT / 13:00 ET` | `2018-presidents-day-holiday-schedule.xls @2018-05-08` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-03-30 | closed | `no session printed` | `2018-holiday-calendars.zip#2018-good-friday-holiday-schedule.xls @2026-08-30` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2018-05-28 | early close | `12:00 CT / 13:00 ET` | `2018-holiday-calendars.zip#2018-memorial-day-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-07-04 | early close | `12:00 CT / 13:00 ET` | `2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-09-03 | early close | `12:00 CT / 13:00 ET` | `2018-holiday-calendars.zip#2018-labor-day-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-11-22 | early close | `12:00 CT / 13:00 ET` | `2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-11-23 | early close | `12:45 CT / 13:45 ET` | `2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-12-24 | early close | `12:45 CT / 13:45 ET` | `2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-12-25 | closed | `no session printed` | `2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30` | T1 | CME prints the closure for this date; trade date = the operator's event date |

### 2022

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2022-01-17 | early close | `13:30` | `2022-mlk-day-holiday-schedule.xls @2022-01-17T21:22:30Z` | T1 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2022-02-21 | early close | `13:30` | `2022-presidents-day-holiday-schedule.xls @2022-07-04T07:38:10Z` | T1 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2022-04-15 | closed | `"Globex Closed"` | `2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2022-05-30 | early close | `13:30` | `2022-memorial-day-holiday-schedule.xls @2022-07-04T06:54:38Z` | T1 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2022-06-20 | early close | `13:30` | `2022-juneteenth-holiday-schedule.xls @2022-06-20T20:02:10Z` | T1 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2022-07-04 | early close | `13:30` | `2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z` | T1 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2022-09-05 | early close | `13:30` | `2022-labor-day-holiday-schedule.xls @2022-07-04T06:54:41Z` | T1 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2022-11-24 | early close | `13:30` | `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | T1 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2022-11-25 | early close | `12:45` | `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | T1 | CME prints `12:45` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2022-12-26 | closed | `"Globex Closed"` | `2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |

### 2023

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `"Globex Closed"` | `2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2023-01-16 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2023-01-15` | T2 | the operator published no document for this date and the cited artifact is the negative control that returns an empty event list, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2023-02-20 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2023-02-19` | T2 | the operator published no document for this date and the cited artifact is the negative control that returns an empty event list, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2023-04-07 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2023-04-06` | T2 | the operator published no document for this date and the cited artifact is the negative control that returns an empty event list, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2023-05-29 | early close | `13:30` | `memorial-day-2023.pdf @2023-04-20T22:40:18Z` | T1 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2023-06-19 | early close | `13:30` | `juneteenth-2023.pdf @2023-06-13T18:59:49Z` | T1 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2023-07-04 | early close | `13:30` | `4th-of-july-2023.pdf @2023-06-27T12:50:57Z` | T1 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2023-09-04 | early close | `13:30` | `labor-day-2023.pdf @2023-08-02T19:24:46Z` | T1 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2023-11-23 | early close | `13:30` | `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | T1 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2023-11-24 | early close | `12:45` | `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | T1 | CME prints `12:45` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2023-12-25 | closed | `16:00 (PREOPEN)` / `17:00 (OPEN)` — the only clocks in this date's own entry are the evening re-open that carries the next trade date | `christmas-day-2023.pdf @2026-07-19T09:52:48Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |

### 2024

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `16:00 (PREOPEN)` / `17:00 (OPEN)` — the only clocks in this date's own entry are the evening re-open that carries the next trade date | `new-years-day-2024.pdf @2026-08-11T16:57:16Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2024-01-15 | early close | `13:30` | `CME-SVC-2024-01-14` | T2 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2024-02-19 | early close | `13:30` | `CME-SVC-2024-02-18` | T2 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2024-03-29 | closed | `events []` — the artifact's own empty event list for this date | `CME-SVC-2024-03-28` | T2 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2024-05-27 | early close | `13:30` | `CME-SVC-2024-05-26` | T2 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2024-06-19 | early close | `13:30` | `CME-SVC-2024-06-18` | T2 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2024-07-04 | early close | `13:30` | `CME-SVC-2024-07-03` | T2 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2024-09-02 | early close | `13:30` | `CME-SVC-2024-09-01` | T2 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2024-11-28 | early close | `13:30` | `CME-SVC-2024-11-27` | T2 | CME prints `13:30` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2024-11-29 | early close | `13:45` | `CME-SVC-2024-11-27` | T2 | CME prints `13:45` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2024-12-24 | early close | `12:45` | `CME-SVC-2024-12-24` | T2 | CME prints `12:45` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2024-12-25 | closed | `16:00 (PREOPEN)` / `17:00 (OPEN)` — the only clocks in this date's own entry are the evening re-open that carries the next trade date | `CME-SVC-2024-12-24` | T2 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
### Documents

This era's rows cite the ids below: CME Group's own published holiday schedules at **T1** (2022 per-asset-class workbooks, 2023 one-pagers, and the 2024 Good Friday and year-end service windows as archived), and responses of CME's own `trading-hours-by-product` service at **T2**. Each id resolves to the URL it was read at — an Internet Archive raw replay for a saved capture, the operator's own endpoint for a live retrieval — with the capture or retrieval time in UTC, the tier and the sha256. All thirty artifacts resolve in the research store's `holidays/raw/cme-2022-2024/` and `holidays/raw/cme-2022-2024-fix/`, whose `INDEX.md` files carry the byte counts.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |

|---|---|---|---|---|---|
| `2022-mlk-day-holiday-schedule.xls @2022-01-17T21:22:30Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220117212230id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-mlk-day-holiday-schedule.xls> | archive capture 2022-01-17T21:22:30Z | T1 | `896944fa701062e2e1ee305f8adb696ba8ab03655a96874c7885f2328177626e` |
| `2022-presidents-day-holiday-schedule.xls @2022-07-04T07:38:10Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704073810id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-presidents-day-holiday-schedule.xls> | archive capture 2022-07-04T07:38:10Z | T1 | `07932975d04ccabad0fb53f33f946f1a6fd8a696bafdb21f95f030ede9a84516` |
| `2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065501id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-good-friday-holiday-schedule.xls> | archive capture 2022-07-04T06:55:01Z | T1 | `a82936ab14d1b1f7041583123289c4de401c66ace4eea7e90fa9f60c1a3f3b7e` |
| `2022-memorial-day-holiday-schedule.xls @2022-07-04T06:54:38Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065438id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-memorial-day-holiday-schedule.xls> | archive capture 2022-07-04T06:54:38Z | T1 | `0d1b1f89a315cae22a5857a7027a514e3086c1cccfee42fddddff3caba0c2230` |
| `2022-juneteenth-holiday-schedule.xls @2022-06-20T20:02:10Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220620200210id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-juneteenth-holiday-schedule.xls> | archive capture 2022-06-20T20:02:10Z | T1 | `bc9f2caf26a73a13029f177fcc6468bdc8662899ff935f44329ee1a95c8b667b` |
| `2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065450id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-independence-day-holiday-schedule.xls> | archive capture 2022-07-04T06:54:50Z | T1 | `1ea0459d8aa0fd7ec5147614855f6d0d43607efefdc3aa9c6b1e18ddbfd54fde` |
| `2022-labor-day-holiday-schedule.xls @2022-07-04T06:54:41Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065441id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-labor-day-holiday-schedule.xls> | archive capture 2022-07-04T06:54:41Z | T1 | `28d533f25d1af74af043e411635f1933fd4ff6359c80c049f87f019d2f1d57d0` |
| `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20221122060801id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-thanksgiving-holiday-schedule.xls> | archive capture 2022-11-22T06:08:01Z | T1 | `64341a65350de982a6a05760504132e173611151ae3164c6f9437509a406edcb` |
| `2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065430id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-christmas-holiday-schedule.xls> | archive capture 2022-07-04T06:54:30Z | T1 | `2dd1d531514989845dcb6ce6d767db5dd3f956ac6777ab1cfe36d6843f3762e7` |
| `2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065501id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2023-new-years-holiday-schedule.xls> | archive capture 2022-07-04T06:55:01Z | T1 | `eefafd1066f406edbe6167ddf8ad13c0697b124a337c0893ce5a5da200c783d3` |
| `CME-SVC-2023-01-15` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161438id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2023-01-15&toEventDate=2023-01-17&isProtected&_t=1720455278636> | archive capture 2024-07-08T16:14:38Z | T2 | `507fd196a7654ddd916218b2aa24a146eeaca4e1f7cb7daded4e7a9c689cda82` |
| `CME-SVC-2023-02-19` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161438id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2023-02-19&toEventDate=2023-02-21&isProtected&_t=1720455278640> | archive capture 2024-07-08T16:14:38Z | T2 | `d063238a83e8cb84d4484a86b26cb1d976f1ccfdb91972eecc08dcc0cec49412` |
| `CME-SVC-2023-04-06` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161438id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2023-04-06&toEventDate=2023-04-08&isProtected&_t=1720455278642> | archive capture 2024-07-08T16:14:38Z | T2 | `0543d5f6d2efd4aa6f4132ce9f5425b9ba9f64de08eccd677b78d6b43c6e9d4f` |
| `memorial-day-2023.pdf @2023-04-20T22:40:18Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20230420224018id_/https://www.cmegroup.com/trading-hours/files/memorial-day-2023.pdf> | archive capture 2023-04-20T22:40:18Z | T1 | `7657bc8089ca669cfd244c2e3e697b47a7e650f5d957efe5b32da001622acb35` |
| `juneteenth-2023.pdf @2023-06-13T18:59:49Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20230613185949id_/https://www.cmegroup.com/trading-hours/files/juneteenth-2023.pdf> | archive capture 2023-06-13T18:59:49Z | T1 | `831f7f63e197ce58436780830aa515cb08dcf92bdf82a540265c2d21a4bffa43` |
| `4th-of-july-2023.pdf @2023-06-27T12:50:57Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20230627125057id_/https://www.cmegroup.com/trading-hours/files/4th-of-july-2023.pdf> | archive capture 2023-06-27T12:50:57Z | T1 | `ccbc1f1fc39ba2219fd748372faa985798fee7eaffabf69f08956f5465a769e3` |
| `labor-day-2023.pdf @2023-08-02T19:24:46Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20230802192446id_/https://www.cmegroup.com/trading-hours/files/labor-day-2023.pdf> | archive capture 2023-08-02T19:24:46Z | T1 | `39a4c075437fdc7134166328eca730e3cabf3bea00ab369466dff63c99f8e948` |
| `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20231203205929id_/https://www.cmegroup.com/trading-hours/files/thanksgiving-day-2023.pdf> | archive capture 2023-12-03T20:59:29Z | T1 | `99e187b3f3899e1062d662e148d5978e0cd075b961e6fc79550d4393812307e8` |
| `christmas-day-2023.pdf @2026-07-19T09:52:48Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20260719095248id_/https://www.cmegroup.com/trading-hours/files/christmas-day-2023.pdf> | archive capture 2026-07-19T09:52:48Z | T1 | `edcde0fcf61d3414cee2a332453db861e0e2d7edf81d89a5f379c44b2e72d5cf` |
| `new-years-day-2024.pdf @2026-08-11T16:57:16Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20260811165716id_/https://www.cmegroup.com/trading-hours/files/new-years-day-2024.pdf> | archive capture 2026-08-11T16:57:16Z | T1 | `34e60f8c97623df30e00f0ad8e4eeda20b99001b6c35d64f735828fecec5b9b8` |
| `CME-SVC-2024-01-14` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-01-14&toEventDate=2024-01-16&isProtected&_t=1720455278663> | archive capture 2024-07-08T16:14:39Z | T2 | `a8fe0f3eed4c67939d3add5fa656173bc4119c3765174b084c0622a1934e0d22` |
| `CME-SVC-2024-02-18` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-02-18&toEventDate=2024-02-20&isProtected&_t=1720455278669> | archive capture 2024-07-08T16:14:39Z | T2 | `af5ddb57cfa8bd6d377a0784fcdd33589e2055ede0a09b7fe4ae3e41d0d20f9a` |
| `CME-SVC-2024-03-28` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-03-28&toEventDate=2024-03-30&isProtected&_t=1720455278672> | archive capture 2024-07-08T16:14:39Z | T2 | `9b41709219e36f56296843fe589e362a7513e9a98132a682a1b956f2e40c593b` |
| `CME-SVC-2024-05-26` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-05-26&toEventDate=2024-05-28&isProtected&_t=1720455278675> | archive capture 2024-07-08T16:14:39Z | T2 | `ccff9685aac5670c5eb4a7e2b86cf1d78a5c2c08325cd03673ab8c1e6f560138` |
| `CME-SVC-2024-06-18` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-06-18&toEventDate=2024-06-20&isProtected&_t=1720455278677> | archive capture 2024-07-08T16:14:39Z | T2 | `57acecac3e1ad1a50dda8e3bcd6ca926b4d4c6c2ebc9d8324eb73332ed6a20ec` |
| `CME-SVC-2024-07-03` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-07-03&toEventDate=2024-07-05&isProtected&_t=1720455278680> | archive capture 2024-07-08T16:14:39Z | T2 | `6ef0e2b055c349249f5cc7ea7c98132a31111a01ce7af028e13584c0f82d4d38` |
| `CME-SVC-2024-09-01` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-09-01&toEventDate=2024-09-03&isProtected&_t=1720455278683> | archive capture 2024-07-08T16:14:39Z | T2 | `bbe6c78555cba0437fe4b9c6eac0063983ecae49056877db618ed2b29c109546` |
| `CME-SVC-2024-11-27` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-11-27&toEventDate=2024-11-29&isProtected&_t=1720455278685> | archive capture 2024-07-08T16:14:39Z | T2 | `f6a15f26991d25f8c6821fa6c967d0e76b2ad3f775bed62a2a001ec2fd7e389a` |
| `CME-SVC-2024-12-24` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-12-24&toEventDate=2024-12-26&isProtected&_t=1734710019537> | archive capture 2024-12-20T15:53:40Z | T2 | `183c85160e31d53fde30c422048b8e778937d8866f3ea1c45e2dd570ea9db1c8` |
### Gaps and residual risks, 2022-2024

**The three 2023 dates the operator published nothing for — 2023-01-16, 2023-02-20 and 2023-04-07.** They ship `Unsourced`, cited to the T2 captures `CME-SVC-2023-01-15`, `CME-SVC-2023-02-19` and `CME-SVC-2023-04-06`, each of which is the operator's own machine channel read as bytes and returns an empty event list for its window: 2023 Dr. Martin Luther King, Jr. Day (Monday 16 January 2023), 2023 Presidents Day (Monday 20 February 2023) and 2023 Good Friday (Friday 7 April 2023). The block's `missing` register records the channels searched — `holiday-calendar/files/*.xls`, of which only a compact MGEX/DME workbook exists for MLK, and `trading-hours/files/<holiday>-2023.pdf`, which 404s — so the gap is "not worked up", not "no source exists". `Unsourced` clips nothing, so each of these dates still resolves to the family's ordinary week. Closing condition: a CME holiday schedule for 2023 covering this date at T1, or a T2 window that carries its events. The day after each of the three is a second, unmodelled gap: 2023-01-17 and 2023-02-21 (and 2023-04-10 for `globex_grains`) may have lost their prior-evening leg the way 2024-01-02 and 2023-12-26 did, and no artifact this crate read states whether they did, so they ship no row and the family's ordinary week stands there.

**The 2024-04-01 re-open.** The Good Friday window `CME-SVC-2024-03-28` ends at 2024-03-30 and prints empty event lists for both 2024-03-29 and 2024-03-30, so CME published nothing this crate read for the Sunday 2024-03-31 evening leg that would carry trade date 2024-04-01. Inside a contiguous window silence would read as audited normal, so 2024-04-01 ships **no row** and is a declared gap rather than an audited one. Closing condition: a CME service response covering 2024-03-31, or a T1 statement of that Sunday's re-open. For `globex_grains` the same silence is what withholds a possible late open on 2024-04-01.

**Normal-week notes that ship no row.** Where a printed token falls outside the family's ordinary week but moves no boundary a scalar holiday row can state, the date ships nothing and the token is recorded here: the 2022 New Year's workbooks print the `Nikkei/TOPIX BTIC` `Close 00:00` on 2022-01-01, a BTIC close for the next trade date; the 2023 and 2024 Independence Day and New Year grain entries print the next trade date's `06:00 (PREOPEN)`, which is the no-evening-leg marker the late opens above are read from; and the 2024-12-31 grain entry points at `2025-01-02 06:00 preopen`, outside this window, so no row ships and the 2025-2027 table must state that trade date.

### 2025-2027 (T2)

**Zone.** CME prints no Eastern column in this channel. The operator states the
zone once, on the page the service backs, quoted verbatim: "Trading hours are
subject to change and are in U.S. Central Time unless otherwise stated." The
calibration is independent of that sentence as well: the same channel prints
`16:00 closed`, `16:45 preopen` and `17:00 open` for an ordinary Monday-Thursday
of the reference week 2026-10-18..2026-10-24, which is this family's sourced
17:00-16:00 CT grid event for event. Every instant below is therefore CT as
printed, and **no ET value is asserted**.

**Energy and metals are one key, and CME prints them as one row.** On every
date in this window the service returns `CL` and `GC` with identical event
lists, and the payload records them under one combined product-group label, so
the memo's D17 intersection rule is never reached and no date is withheld for
a disagreement between the two halves.

**How an event date becomes a trade date.** The family's trading day wraps: one
occurrence opens 17:00 CT on the previous local day and closes 16:00 CT on the
trade date, Sunday through Thursday. Three scalar shapes appear in this
window, and one intraday topology the scalar vocabulary cannot state:

1. a final `closed` event on the date at an instant earlier than 16:00 CT — an
   `EarlyClose` on that trade date, clipping the occurrence that opened the
   previous evening;
2. no final close on the date, only a pre-open and a 17:00 CT `open` carrying
   the **next** business day's trade date, or no events at all — no session
   belongs to the date, so the row is `Closed` and the previous evening's leg
   goes with it;
3. only a `13:30 preopen` on the date, followed by the ordinary `17:00 open`.
   CME's own legend for the event types reads, verbatim, PREOPEN — "Order
   Entry, modification, and cancel are allowed. No order matching." — and OPEN —
   "Start of continuous trading phase. Order matching begins." A pre-open
   therefore ends matching, so 13:30 CT is that trade date's final close and the
   row is an `EarlyClose` at 13:30 CT. This is the `[N16]` shape of the
   retrieval and the same reading the design memo applies to the Equity Index
   `12:00 preopen` row of 2025-01-20.
4. an intraday `preopen`/`open` pair on the date ahead of an early close, on a
   trading day that already opened the previous evening — 2025-11-28 prints
   `07:00 preopen; 07:30 open; 13:45 closed`. The row carries only the 13:45 CT
   `EarlyClose`; the implied halt is a topology change, recorded under
   **Gaps, 2025** below and not modelled.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `16:00 preopen; 17:00 open` — both 2025-01-02, so no session settles on 01-01 | `CME-SVC-2024-12-31` | T2 | eventDate 2024-12-31 `16:00 closed` CME trade date 2024-12-31 with no evening re-open, and eventDate 2025-01-01 CME trade date 2025-01-02 |
| 2025-01-20 | early close | `13:30 preopen` — 13:30 CT; the 17:00 CT open that follows carries 2025-01-21 | `CME-SVC-2025-01-19` | T2 | eventDate 2025-01-20, CME trade date printed as 2025-01-21 |
| 2025-02-17 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2025-02-18 | `CME-SVC-2025-02-16` | T2 | eventDate 2025-02-17, CME trade date printed as 2025-02-18 |
| 2025-04-18 | closed | `no events published` | `CME-SVC-2025-04-17` | T2 | eventDate 2025-04-18, and eventDate 2025-04-17 `16:00 closed` CME trade date 2025-04-17 with no evening re-open |
| 2025-05-26 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2025-05-27 | `CME-SVC-2025-05-25` | T2 | eventDate 2025-05-26, CME trade date printed as 2025-05-27 |
| 2025-06-19 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2025-06-20 | `CME-SVC-2025-06-18` | T2 | eventDate 2025-06-19, CME trade date printed as 2025-06-20 |
| 2025-07-04 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2025-07-03` | T2 | eventDate 2025-07-04, CME trade date 2025-07-04 |
| 2025-09-01 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2025-09-02 | `CME-SVC-2025-08-31` | T2 | eventDate 2025-09-01, CME trade date printed as 2025-09-02 |
| 2025-11-27 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2025-11-28 | `CME-SVC-2025-11-26` | T2 | eventDate 2025-11-27, CME trade date printed as 2025-11-28 |
| 2025-11-28 | early close | `07:00 preopen; 07:30 open; 13:45 closed` — final close 13:45 CT | `CME-SVC-2025-11-26` | T2 | eventDate 2025-11-28, CME trade date 2025-11-28 |
| 2025-11-29 | closed | `no events published` | `CME-SVC-2025-11-26-SAT` | T2 | eventDate 2025-11-29, the Thanksgiving window extended to the Saturday |
| 2025-12-24 | early close | `12:45 closed` — 12:45 CT, and no evening re-open | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-24, CME trade date 2025-12-24 |
| 2025-12-25 | closed | `16:00 preopen; 17:00 open` — both 2025-12-26 | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-25, CME trade date 2025-12-26; the 2025-12-24 record carries no evening leg for 12-25 |

**Interpretive steps, 2025.** The 2025-01-01 and 2025-12-25 rows are the
shape-2 conversion: CME publishes a pre-open and a 17:00 CT open on the holiday
itself, both already carrying the next business day, so the holiday has no
trade date and the row is `Closed` rather than a late open. The 2025-01-20,
2025-02-17, 2025-05-26, 2025-06-19, 2025-09-01 and 2025-11-27 rows are the
shape-3 conversion described above. Saturday 2025-11-29 changes no answer for
this family — the normal week has no Saturday trade date — and ships because
CME's own 2025 Globex table states the Thanksgiving period as "27 - 29 November
2025" and the operator's channel was asked and answered for the Saturday.

**Gaps, 2025.**

- **Intraday topology, 2025-11-28** — the finalised publication prints
  `07:00 preopen; 07:30 open` ahead of the 13:45 CT close, on a trading day
  that opened 17:00 CT the previous evening. That implies a halt CME does not
  print, and the scalar vocabulary of a holiday row cannot state it. Not a late
  open: the trading day's own first open is 17:00 CT on 11-27, and clipping to
  07:30 CT would delete fourteen hours CME publishes as open. Recorded, not
  modelled. Closing condition: a block-row holiday kind (#93), or an operator
  statement of the halt.
- **Order-entry deviations** — on the shape-3 dates the pre-open runs
  13:30-17:00 CT rather than the normal 16:45-17:00 CT, and on 2025-01-01 and
  2025-12-25 it starts 16:00 CT rather than 16:45 CT. A holiday row has no
  order-entry boundary, so this is not representable. It changes no `is_open`
  answer, only `is_accepting_orders` and `is_order_entry_only`.
- **Pre-finalisation publications** — the 2025-01-01 through 2025-09-01 rows
  rest on the single archive capture 2024-12-20T15:53:40Z, on a page that
  states "This schedule is subject to change. Trading hours are usually
  finalized approximately two weeks prior to the holiday." The live service was
  re-probed on 2026-09-12 for all eight of those windows and returns the
  products with empty schedules — a retention edge, not a contradiction — so no
  post-finalisation statement is reachable. Thanksgiving 2025 and Christmas
  2025 are sourced from post-holiday captures instead.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `16:00 preopen; 17:00 open` — both 2026-01-02 | `CME-SVC-2025-12-31` | T2 | eventDate 2026-01-01, CME trade date 2026-01-02; eventDate 2025-12-31 `16:00 closed` with no evening re-open |
| 2026-01-19 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2026-01-20 | `CME-SVC-2026-01-18` | T2 | eventDate 2026-01-19, CME trade date printed as 2026-01-20 |
| 2026-02-16 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2026-02-17 | `CME-SVC-2026-02-15` | T2 | eventDate 2026-02-16, CME trade date printed as 2026-02-17 |
| 2026-04-03 | closed | `no events published` | `CME-SVC-2026-04-01` | T2 | eventDate 2026-04-03, and eventDate 2026-04-02 `16:00 closed` CME trade date 2026-04-02 with no evening re-open |
| 2026-05-25 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2026-05-26 | `CME-SVC-2026-05-24` | T2 | eventDate 2026-05-25, CME trade date printed as 2026-05-26 |
| 2026-06-19 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2026-06-18` | T2 | eventDate 2026-06-19, CME trade date printed as 2026-06-22 |
| 2026-07-03 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2026-07-03` | T2 | eventDate 2026-07-03, CME trade date printed as 2026-07-06 |
| 2026-09-07 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2026-09-08 | `CME-SVC-2026-09-06` | T2 | eventDate 2026-09-07, CME trade date printed as 2026-09-08 |
| 2026-11-26 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2026-11-27 | `CME-SVC-2026-11-25` | T2 | eventDate 2026-11-26, CME trade date printed as 2026-11-27 |
| 2026-11-27 | early close | `13:45 closed` — 13:45 CT | `CME-SVC-2026-11-25` | T2 | eventDate 2026-11-27, CME trade date 2026-11-27 |
| 2026-12-24 | early close | `12:45 closed` — 12:45 CT, and no evening re-open | `CME-SVC-2026-12-22` | T2 | eventDate 2026-12-24, CME trade date 2026-12-24 |
| 2026-12-25 | closed | `no events published` | `CME-SVC-2026-12-24` | T2 | eventDate 2026-12-25; the 2026-12-24 record carries no evening leg for 12-25 |

**Interpretive steps, 2026.** The 2026-06-19 and 2026-07-03 rows are the
Friday-holiday divergence. CME prints the final close on the Friday but dates
it to the following Monday, so on the operator's clearing convention the Friday
has no trade date at all. The crate assigns a session to the venue-local date
of its own final close, and this family has a weekend close and no
following-business-day roll, so a `Closed` row here would delete the whole span
from 17:00 CT Thursday to 12:00 CT Friday — trading CME itself publishes. The
row therefore stays on the Friday as an early close at the sourced instant, and
the trade-date divergence is recorded here rather than modelled. The
cryptocurrency family is treated differently for the same operator records
precisely because its business-date roll preserves the trading instead of
deleting it (design memo D9).

**Gaps, 2026.**

- **Saturday sessions, 2026-06-20 and 2026-07-04** — CME publishes
  `05:00 open; 17:00 closed`, both carrying the following Monday, on Saturdays
  the normal week has none. A holiday row can move an existing occurrence's
  boundaries; it cannot create an occurrence. Sourced and unrepresentable.
  Closing condition: a block-row holiday kind (design memo §7 follow-up 8, #93).
- **Order-entry deviations** — as 2025: the shape-3 pre-opens run 13:30-17:00
  CT and the 2026-01-01 pre-open starts 16:00 CT.
- **A T1 rendering** — the trading-hours page renders only the next upcoming
  holiday and its selector could not be driven from a URL, so no T1
  per-asset-class rendering was obtainable for any date in this block. One T1
  capture, for Thanksgiving 2026, was used to validate the service rows group
  by group and matched row for row.
- **Black Friday finalisation** — 2026-11-27 is sourced from a pre-holiday
  publication. The 2025 equivalent gained a `07:00 preopen; 07:30 open` pair
  when CME finalised it, so this row may gain the same intraday pair. The close
  instant was unchanged by finalisation in 2025.

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `no events published` | `CME-SVC-2026-12-31` | T2 | eventDate 2027-01-01, and eventDate 2026-12-31 `16:00 closed` CME trade date 2026-12-31 with no evening re-open |
| 2027-01-18 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2027-01-19 | `CME-SVC-2027-01-17` | T2 | eventDate 2027-01-18, CME trade date printed as 2027-01-19 |
| 2027-02-15 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2027-02-16 | `CME-SVC-2027-02-14` | T2 | eventDate 2027-02-15, CME trade date printed as 2027-02-16 |
| 2027-03-26 | closed | `no events published` | `CME-SVC-2027-03-25` | T2 | eventDate 2027-03-26, and eventDate 2027-03-25 `16:00 closed` CME trade date 2027-03-25 with no evening re-open |
| 2027-05-31 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2027-06-01 | `CME-SVC-2027-05-30` | T2 | eventDate 2027-05-31, CME trade date printed as 2027-06-01 |
| 2027-06-18 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2027-06-17` | T2 | eventDate 2027-06-18, CME trade date printed as 2027-06-21 |
| 2027-07-05 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2027-07-06 | `CME-SVC-2027-07-04` | T2 | eventDate 2027-07-05, CME trade date printed as 2027-07-06 |
| 2027-09-06 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2027-09-07 | `CME-SVC-2027-09-05` | T2 | eventDate 2027-09-06, CME trade date printed as 2027-09-07 |
| 2027-11-25 | early close | `13:30 preopen` — 13:30 CT; 17:00 CT open carries 2027-11-26 | `CME-SVC-2027-11-24` | T2 | eventDate 2027-11-25, CME trade date printed as 2027-11-26 |
| 2027-11-26 | early close | `13:45 closed` — 13:45 CT | `CME-SVC-2027-11-24` | T2 | eventDate 2027-11-26, CME trade date 2027-11-26 |
| 2027-12-24 | closed | `no events published` | `CME-SVC-2027-12-22` | T2 | eventDate 2027-12-24, and eventDate 2027-12-23 `16:00 closed` CME trade date 2027-12-23 with no evening re-open |

**Interpretive steps, 2027.** 2027-06-18 is the third instance of the
Friday-holiday divergence and is converted exactly as 2026-06-19 above.
CME's holiday date for Christmas 2027 is Thursday 2027-12-23, on which the
family closes at its normal 16:00 CT and therefore ships no row; what the
closure removes is the Friday, so the only row is `Closed(2027-12-24)`.

**Gaps, 2027.**

- **Saturday session, 2027-06-19** — `05:00 open; 17:00 closed` carrying
  2027-06-21, on a Saturday the normal week has none. As 2026 above.
- **Order-entry deviations** — as 2025 and 2026.
- **A T1 rendering** — as 2026.
- **Black Friday finalisation** — 2027-11-26 is sourced from a pre-holiday
  publication; see the 2026 note.

### Holiday coverage notes

- **Dates with no row are audited normal.** Columbus Day and Veterans Day are
  the ones a reader will look for: they appear nowhere in CME's own Globex
  holiday list, CME publishes only clearing advisories and settlement-time PDFs
  for them, and Globex trades a normal session on both. They are therefore not
  holidays for this family, and their absence from the tables above is a
  positive answer rather than a silence.
- **2028-01-01 is outside coverage and ships no row**, although the retrieval
  holds it: coverage ends at the last day of the operator's published future
  for this family, 2027-12-31.
- **No row below 2025-01-01.** Waves 2 to 6 of the migration extend the window
  backwards to the January-2010 floor; until then `holiday_coverage` says so
  and `holiday_on` answers nothing below 2025.
- **Reviewed on** 2026-09-12 (UTC).

> Shared module. The narrative for
> [`energy_metals.rs`](../../src/calendar/schedules/futures/us/energy_metals.rs) lives in
> [`comex`](comex.md#module-narrative-moved-from-srccalendarschedulesfuturesusenergy_metalsrs-on-2026-09-12-utc).
> Sibling identities: [`comex`](comex.md), [`nymex`](nymex.md).
