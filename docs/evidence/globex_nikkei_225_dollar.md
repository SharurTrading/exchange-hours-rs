<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_nikkei_225_dollar` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`cme_nikkei.rs`](../../src/calendar/schedules/futures/us/cme_nikkei.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — the uncertainty touches a window where trades print, so this row's history is served conservatively and the basis below says how. Nikkei 225 Dollar (`NKD`) only. Current 17:00→16:00 CT grid sourced from the contract specification, with a dated timeline from 2012-11-18: SER-6465 (session-opening day 2012-11-18) extended the close to 16:15 CT with a 15:15–15:30 CT halt, SER-6554R (2013-03-03) removed that halt for International Equity Index futures naming NKD explicitly, and CME Globex Notice #20150817 (2015-09-20, trade date 2015-09-21) moved the CME Equity close to 16:00 CT. Partial because the pre-2011 interval is omitted rather than modelled. The 2026-09-01 review found why it must be: CME's own trading-hours pages captured 2010-03-10 and 2010-04-07 show a **materially different** NKD grid — CDT 03:00–15:15 reopening 15:30–16:30 and 17:00–18:00, CST 02:00–15:15 with **no Sunday hours** — so the 17:00–15:15 continuous grid cannot be carried across 2010 without reporting the contract open all night when it was closed. An earlier revision of this branch did exactly that; it is corrected. The changeover is undated (2010-04-07 still shows the old grid, 2011-01-12 already shows the new one, with no capture or located notice between), so dates before the first sourced appearance of the served grid are sessionless and the 2010 grid is left sourced-but-unmodelled — encoding it would need seasonal CDT/CST rules and a boundary that is still undated. CME's trading-hours pages captured 2011-01-12 onward state the served grid — Sunday Pre-Open 16:15, ETH (Sunday) 17:00-15:15, weekday Pre-Open "15:25, 16:45", ETH (Weekday) "15:30-16:30, 17:00-15:15", byte-identical to the E-mini S&P 500 row on the same page — so the pre-2012 evening open is 17:00 CT and is primary-sourced. What stays undated is when that grid began, since the 2010 change is attested only by a third-party aggregator; keying a revision to a capture date would fabricate a cutover. [Sentence removed at the 2026-09-12 migration: it claimed the pre-2012 grid was carried to the January-2010 floor, which contradicts `nkd_profile_at` (NKD_CLOSED before 2011-01-12).]

## Revision rows

- 2011-01-12 — T1 — first sourced CME trading-hours capture of this grid — knowledge boundary: 17:00–15:15 CT with the 15:30–16:30 CT post-halt segment.
- 2012-11-18 — T1 — CME SER-6465 — the close is extended to 16:15 CT with a 15:15–15:30 CT electronic halt.
- 2013-03-03 — T1 — CME SER-6554R — the 15:15–15:30 CT halt is removed for International Equity Index futures, naming NKD explicitly.
- 2015-09-20 — T1 — CME Globex notice 20150817 — the CME Equity close moves to 16:00 CT for trade date Monday 2015-09-21.

## Holidays

**Coverage:** 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 (inclusive venue-local trade dates).

**Four audited eras, with the unworked years between them.** The table declares 4 coverage
windows: `2016-01-01..2018-12-31`, from the operators' own published holiday schedules at **T1**;
`2019-01-01..2021-12-31`, from CME's own published Globex holiday schedules at **T1**;
`2022-01-01..2024-12-31`, from the 2022 workbooks and 2023 one-pagers at T1 and CME's service
responses at **T2**; and `2025-01-01..2027-12-31`, from the trading-hours service at T2. The
2022-2024 and 2025-2027 windows are adjacent; what lies outside every declared window is 2010-2015
— this family did not declare the 2010-2012 era at all — so `holiday_on` has **no answer** there
rather than reporting an unaudited date as normal. 2013-2015 is the one remaining stage-2.2 wave,
and the eras before 2010 are out of scope below the crate's January-2010 floor.
`HolidayCoverage::windows()` lists the 4, and `contains` answers per date.
**Channel.** Every row below comes from
`https://www.cmegroup.com/services/trading-hours-by-product?id=<set>&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=<from>&toEventDate=<to>`
— the endpoint `cmegroup.com/trading-hours.html` itself calls to render its per-asset-class
Holiday Hours table. That is the operator's own machine channel, so **T2** under
LAW-PRIMARY-SOURCES, not T1. Two product-id sets are in play. `THBP-A` is
`id=316,133,425,300,58,437,22,8478,5201,10191` — the ten headline products CME's own page
queries: `ES`, `ZN`, `6E`, `CL`, `GC`, `ZC`, `LE`, `CSC`, `BTC`, `LBR`. `THBP-B` is
`id=168,167,320,323,19,27` — `NKD`, `NIY`, `ZS`, `ZW`, `HE`, `DC`, and it is the only set
that carries the Nikkei line. Document ids taken from `THBP-B` carry a `-B-` infix;
unsuffixed `CME-SVC-` ids are `THBP-A` captures.

**Zone.** Quoted verbatim from the operator's page: "Trading hours are subject to change
and are in U.S. Central Time unless otherwise stated." CME prints no ET column in this
channel, so no ET value is asserted here and none is bracketed in.

**Event vocabulary**, verbatim from the same page: `preopen` — "Order Entry, modification,
and cancel are allowed. No order matching."; `open` — "Start of continuous trading phase.
Order matching begins."; `closed` — "Final Close of the date. Day and GTD (current trade
date) orders are eliminated." `/TD` below is CME's own `tradingDate` field on each event.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
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
| `CME-SVC-B-2025-11-26` | 2025-11-26 .. 2025-11-29 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-11-26&toEventDate=2025-11-29> | live retrieval 2026-09-12T08:54:32Z | T2 | `50da5636342b8352826debc016850594cc84bc2d50ac0f3121f1c79843435fa2` |
| `CME-SVC-B-2025-12-24` | 2025-12-24 .. 2025-12-26 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-12-24&toEventDate=2025-12-26> | live retrieval 2026-09-12T08:54:56Z | T2 | `11dc4de5bf662e60d6bf71e37adb6247e217433991e1b29af0cbbf973c9288fd` |
| `CME-SVC-B-2025-12-31` | 2025-12-31 .. 2026-01-02 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-12-31&toEventDate=2026-01-02> | live retrieval 2026-09-12T08:54:57Z | T2 | `c558c9f399eb1b83b55f6dd1c00dac81a8a55e8d25b5eaa9cecd1b6bb229a216` |
| `CME-SVC-B-2026-01-18` | 2026-01-18 .. 2026-01-20 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-01-18&toEventDate=2026-01-20> | live retrieval 2026-09-12T08:54:58Z | T2 | `15f55c10115e7a37cf85f0583e577c2fb7421b08a545ee03f3c0261e46c5d09d` |
| `CME-SVC-B-2026-02-15` | 2026-02-15 .. 2026-02-17 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-02-15&toEventDate=2026-02-17> | live retrieval 2026-09-12T08:54:58Z | T2 | `5bbecf07eecbdb2dbe5325f39f6b464bf817f535d38841fccdafef19103aaef0` |
| `CME-SVC-B-2026-04-01` | 2026-04-01 .. 2026-04-03 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-04-01&toEventDate=2026-04-03> | live retrieval 2026-09-12T08:55:08Z | T2 | `b4569c685450baab17749fbed20c8c0e37c40910b77c411f72b6c596cd3c0567` |
| `CME-SVC-B-2026-05-24` | 2026-05-24 .. 2026-05-26 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-05-24&toEventDate=2026-05-26> | live retrieval 2026-09-12T08:55:09Z | T2 | `3c7628b898d8069067836a36c44769f2f2b76a1dee5edbd48225f76b363881d0` |
| `CME-SVC-B-2026-06-18` | 2026-06-18 .. 2026-06-20 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-06-18&toEventDate=2026-06-20> | live retrieval 2026-09-12T08:55:10Z | T2 | `41791460e029bb7db0d280374048c36f015594fb71e9f1aae86a6145429ce2e5` |
| `CME-SVC-B-2026-06-21` | 2026-06-21 .. 2026-06-23 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-06-21&toEventDate=2026-06-23> | live retrieval 2026-09-25T08:44:59Z via `https://r.jina.ai/` | T2 | `09714a527385207db4926843cda9df6d2a0ea6515356584e1f8e1f9f69b4f209` |
| `CME-SVC-B-2026-07-03` | 2026-07-03 .. 2026-07-05 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-07-03&toEventDate=2026-07-05> | live retrieval 2026-09-12T08:55:11Z | T2 | `dfc4aff36f0e44fb8fdb78de59d13bad90707c0d108673094ad0a012cefad898` |
| `CME-SVC-B-2026-09-06` | 2026-09-06 .. 2026-09-08 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-09-06&toEventDate=2026-09-08> | live retrieval 2026-09-12T04:30Z | T2 | `f7cc43f8d90b571b945901f826277ca43ec21c4438c36bbb26e231c859a83923` |
| `CME-SVC-B-2026-11-25` | 2026-11-25 .. 2026-11-27 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-11-25&toEventDate=2026-11-27> | live retrieval 2026-09-12T04:30Z | T2 | `f6007a75d6009dada85fe6c57d660598f21ed8a8385364c454ee015565f94dd4` |
| `CME-SVC-B-2026-12-24` | 2026-12-24 .. 2026-12-26 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-12-24&toEventDate=2026-12-26> | live retrieval 2026-09-12T04:30Z | T2 | `b622712c1c45c3efb90443172617636ba4c7d7dfb4c6851b70a22ee1c9fa978d` |
| `CME-SVC-B-2026-12-31` | 2026-12-31 .. 2027-01-02 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-12-31&toEventDate=2027-01-02> | live retrieval 2026-09-12T04:30Z | T2 | `b21ac047d6d67cb9026940a37ad345c7b2ece40e73dbca7be69cf7171646613c` |
| `CME-SVC-B-2027-01-17` | 2027-01-17 .. 2027-01-19 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-01-17&toEventDate=2027-01-19> | live retrieval 2026-09-12T04:30Z | T2 | `3fc8c80ea1222cd0bb3d46c8a7df904acd3859441ffdfd9de6145625177f9bfc` |
| `CME-SVC-B-2027-02-14` | 2027-02-14 .. 2027-02-16 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-02-14&toEventDate=2027-02-16> | live retrieval 2026-09-12T04:30Z | T2 | `c8ca83f4594756ed338368a0035b790e7a27efe6872506985ca6f7d16bcc97eb` |
| `CME-SVC-B-2027-03-25` | 2027-03-25 .. 2027-03-27 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-03-25&toEventDate=2027-03-27> | live retrieval 2026-09-12T04:30Z | T2 | `022dcd1f61e54cc316a621e01f519bbb723a446c000323dd9725d2d6434effe8` |
| `CME-SVC-B-2027-05-30` | 2027-05-30 .. 2027-06-01 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-05-30&toEventDate=2027-06-01> | live retrieval 2026-09-12T04:30Z | T2 | `1c28c61151bd26e844c5b2ea6f046102b6da45a5f88566e91a421604a8c566e6` |
| `CME-SVC-B-2027-06-17` | 2027-06-17 .. 2027-06-19 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-06-17&toEventDate=2027-06-19> | live retrieval 2026-09-12T04:30Z | T2 | `011d4f666198a4427faa01d7e91ebf2412f4614ae27188ccd35f84c726a5d05d` |
| `CME-SVC-B-2027-06-20` | 2027-06-20 .. 2027-06-22 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-06-20&toEventDate=2027-06-22> | live retrieval 2026-09-25T08:44:59Z via `https://r.jina.ai/` | T2 | `173d07af2d7621480b4a6653d4c2295f3f2b83197c1df3b21ce3bf38179f0d12` |
| `CME-SVC-B-2027-07-04` | 2027-07-04 .. 2027-07-06 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-07-04&toEventDate=2027-07-06> | live retrieval 2026-09-12T04:30Z | T2 | `1a9550357fbf3c1615fcbeefebbc64e271a6dfe0ad0ffdff10c770a7e2d77aaf` |
| `CME-SVC-B-2027-09-05` | 2027-09-05 .. 2027-09-07 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-09-05&toEventDate=2027-09-07> | live retrieval 2026-09-12T04:30Z | T2 | `9ceb6df48d2278a807fd2e3081828eb41dd207cc64c389078b3a529f762da928` |
| `CME-SVC-B-2027-11-24` | 2027-11-24 .. 2027-11-26 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-11-24&toEventDate=2027-11-26> | live retrieval 2026-09-12T04:30Z | T2 | `90320ed581b1d09f6c1b85d98da3a7a097b09d5f4eaac4abbb3d9368ee1cfdfa` |
| `CME-SVC-B-2027-12-22` | 2027-12-22 .. 2027-12-25 | <https://www.cmegroup.com/services/trading-hours-by-product?id=168,167,320,323,19,27&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-12-22&toEventDate=2027-12-25> | live retrieval 2026-09-12T04:30Z | T2 | `1ee3bd5fb9a99f765f96ac60377e8013cfca1c54ceba53beeb616d12862e2b12` |

The research store holds the bytes: the `D01`–`D08` captures under
`holidays/raw/cme-2025-2027/arc/`, `D21`–`D47` under
`holidays/raw/cme-2025-2027/live/extra/`, and `D54`–`D65` under
`holidays/raw/cme-2025-2027-repair/live/`, each with its own `INDEX.md` row.
`cme-2025-2027.verify.json` (round 2, 2026-09-12) is the verdict that governs; it re-parsed
all 313 family rows from the cited bytes with zero mismatches, and its four material
findings — the nine `THBP-B` windows the retrieval had wrongly declared missing among them —
are the reason the Nikkei line exists at all from Thanksgiving 2025 onward.


### 2019

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `Closed for New Year's` | `2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2019-01-21 | early close | `1200 CT / 1800 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-martin-luther-king-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Equity` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-02-18 | early close | `1200 CT / 1800 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-presidents-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Equity` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-04-19 | closed | `Closed for Good Friday` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-good-friday-holiday-compact.xls @2021-01-26T09:48:37Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2019-05-27 | early close | `1200 CT / 1700 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-memorial-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Equity` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-06-19 | unsourced | no CME document covers this date (see `2019-holiday-calendars.zip @2021-01-26T09:48:37Z`) | `2019-holiday-calendars.zip @2021-01-26T09:48:37Z` | T1 | The operator published no Juneteenth schedule for 2019, 2020 or 2021: the year's own consolidated bundle `2019-holiday-calendars.zip @2021-01-26T09:48:37Z` — CME's account of every Globex holiday schedule it published that year — carries no Juneteenth sheet, the archived holiday-calendar.html index pages name none, and a fresh 2018-2027 prefix CDX enumeration finds no `juneteenth` filename before 2022. Silence inside the window would read as audited normal on a date the operator later marks as a holiday, so the row is `Unsourced`, which clips nothing. |
| 2019-07-03 | early close | `1215 CT / 1715 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Equity` line prints `12:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-07-04 | early close | `1200 CT / 1700 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Equity` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-09-02 | early close | `1200 CT / 1700 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-labor-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Equity` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-11-28 | early close | `1200 CT / 1800 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Equity` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-11-29 | early close | `1215 CT / 1815 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Equity` line prints `12:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-12-24 | early close | `1215 CT / 1815 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Equity` line prints `12:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-12-25 | closed | `Globex Closed` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |

### 2020

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `Globex Closed` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2020-01-20 | early close | `1200 CT / 1800 UTC` | `2020-holiday-calendars.zip#2020-martin-luther-king-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `Equity` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-02-17 | early close | `1200 CT / 1800 UTC` | `2020-holiday-calendars.zip#2020-presidents-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `Equity` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-04-10 | closed | `Globex Closed` | `2020-holiday-calendars.zip#2020-good-friday-holiday-compact.xls @2026-07-30T11:18:34Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2020-05-25 | early close | `1200 CT / 1700 UTC` | `2020-holiday-calendars.zip#2020-memorial-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `Equity` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-06-19 | unsourced | no CME document covers this date (see `2020-holiday-calendars.zip @2026-07-30T11:18:34Z`) | `2020-holiday-calendars.zip @2026-07-30T11:18:34Z` | T1 | The operator published no Juneteenth schedule for 2019, 2020 or 2021: the year's own consolidated bundle `2020-holiday-calendars.zip @2026-07-30T11:18:34Z` — CME's account of every Globex holiday schedule it published that year — carries no Juneteenth sheet, the archived holiday-calendar.html index pages name none, and a fresh 2018-2027 prefix CDX enumeration finds no `juneteenth` filename before 2022. Silence inside the window would read as audited normal on a date the operator later marks as a holiday, so the row is `Unsourced`, which clips nothing. |
| 2020-07-03 | early close | `1200 CT / 1700 UTC` | `2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `Equity` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-09-07 | early close | `1200 CT / 1700 UTC` | `2020-holiday-calendars.zip#2020-labor-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `Equity` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-11-26 | early close | `1200 CT / 1800 UTC` | `2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `Equity` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-11-27 | early close | `1215 CT / 1815 UTC` | `2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `Equity` line prints `12:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-12-24 | early close | `1215 CT / 1815 UTC` | `2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `Equity` line prints `12:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-12-25 | closed | `Globex Closed` | `2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |

### 2021

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `Globex Closed` | `2020-holiday-calendars.zip#2021-new-years-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2021-01-18 | early close | `1200 CT / 1800 UTC` | `2021-holiday-calendars.zip#2021-mlk-day-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `Equity` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-02-15 | early close | `1200 CT / 1800 UTC` | `2021-holiday-calendars.zip#2021-presidents-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `Equity` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-04-02 | early close | `0815 CT / 1315 UTC` | `2021-holiday-calendars.zip#2021-good-friday-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `Equity` line prints `08:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-05-31 | early close | `1200 CT / 1700 UTC` | `2021-holiday-calendars.zip#2021-memorial-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `Equity` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-06-19 | unsourced | no CME document covers this date (see `2021-holiday-calendars.zip @2026-08-30T10:03:27Z`) | `2021-holiday-calendars.zip @2026-08-30T10:03:27Z` | T1 | The operator published no Juneteenth schedule for 2019, 2020 or 2021: the year's own consolidated bundle `2021-holiday-calendars.zip @2026-08-30T10:03:27Z` — CME's account of every Globex holiday schedule it published that year — carries no Juneteenth sheet, the archived holiday-calendar.html index pages name none, and a fresh 2018-2027 prefix CDX enumeration finds no `juneteenth` filename before 2022. Silence inside the window would read as audited normal on a date the operator later marks as a holiday, so the row is `Unsourced`, which clips nothing. |
| 2021-07-05 | early close | `1200 CT / 1700 UTC` | `2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `Equity` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-09-06 | early close | `1200 CT / 1700 UTC` | `2021-holiday-calendars.zip#2021-labor-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `Equity` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-11-25 | early close | `"12:00" (Equity Products line, cell as printed in the full sheet's Thursday, November 25 Halt column, which is this trade date's final close; the Equity Products line governs this family); the full sheet's "Nikkei & BTIC" row prints row 12 "Nikkei & BTIC · 00:00 ·  · 10:30 · 11:00 · 16:00 · 16:45 · 17:00 ·  ·  ·  · 00:00 · 16:45 · 17:00 ·  ·  ·  ·  ·  · 00:00". (CME's cell separator is written as U+00B7 here)` | `2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `Equity` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-11-26 | early close | `"12:15" (Equity Products line, cell as printed in the full sheet's Friday, November 26 Close column; the Equity Products line governs this family); the full sheet's "Nikkei & BTIC" row prints row 12 "Nikkei & BTIC · 00:00 ·  · 10:30 · 11:00 · 16:00 · 16:45 · 17:00 ·  ·  ·  · 00:00 · 16:45 · 17:00 ·  ·  ·  ·  ·  · 00:00". (CME's cell separator is written as U+00B7 here)` | `2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `Equity` line prints `12:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-12-24 | closed | `Globex Closed` | `2021-holiday-calendars.zip#2021-christmas-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |

### Documents
This era's rows cite the ids below — CME Group's own Globex holiday schedules, all at **T1**. The 2020 and 2021 sheets and the three `Unsourced` Juneteenth rows are members of their year's consolidated annual bundle; the 2019 sheets likewise, and 1-2 January 2019 come from the December-2018 supplement CME published before the bundle. Each id resolves to the URL the bytes were read at — an Internet Archive raw replay of CME's own file — with the capture time in UTC, the tier and the sha256. All of them resolve in the research store's `holidays/raw/cme-2019-2021/`, whose `INDEX.md` carries the byte counts and whose `shasum.txt` hashes every workbook. The next `### Documents` table in this file is the 2022-2024 era's.
| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `2019-holiday-calendars.zip @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `1e861e355238903b013c1288f4eb9e8026e6ddd5fc1001dfd2acdbfcc1832e05` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `8546b2a9e42c92a4bcf2d2e8209906f482467aeb549ed2a02eb95690954ddd16` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `8ccc652c9e420f20000110bf9e6522ce5ab3a4ac1acd6cca465320da42409dfc` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-good-friday-holiday-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `cfaa08b30242010a65af781d3e485f72642a9ed55ecfe8fbad1aa9b3b20b7ad9` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-labor-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `68b3ac93f8b9077bb82b561d8043c0661475de44f5e5b869b900faa33cadf758` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-martin-luther-king-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `d439d73aa100a9e960a121cac158d2813dceb570f7af80f9cf163e17abedc42a` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-memorial-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `4d55425087a171b10b37a634d9d95cba0ae863e4e2cba282599ef1016b404fbe` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `6625c29097309fb5c209459438b0e868d88623a350324db9404430cf3f13d7fe` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-presidents-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `cd6c895435ef2e7a14408ee74ab4fba140d462abefe00d2ee0333577253049c3` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `99ea9b63ef7d756116312e0061275b1ee7cd50ac547a55f09098a4e58973cb31` |
| `2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20180107041343id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2019-new-years-holiday-schedule-compact.xls> | archive capture 2018-01-07T04:13:43Z | T1 | `2684a5f1b3a9f65802f6911ca6089e2cb68c3cdf2520dfaf3cdcf3105328c188` |
| `2020-holiday-calendars.zip @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `5263a4a5e9076bc0e69c7cd0e3fd9f82dc4d80b66f1b56f14070f08c1d762b59` |
| `2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `05fee0fd5ec2e9d403ac32ad103612569c8f269299b4bfef931c50c311b8740a` |
| `2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `d342b6b0a075591255b45b0dc58dfede5bf5cc28077aa8b362591b82224c24ae` |
| `2020-holiday-calendars.zip#2020-good-friday-holiday-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `aa0936e9278904f40bc23abba57c2c379207af49d0cb6971f07289010fe1745e` |
| `2020-holiday-calendars.zip#2020-labor-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `85acc34db0b86dfe09824c3cfe2f4c7fb21f02ce8ad5593587afe7affa84221f` |
| `2020-holiday-calendars.zip#2020-martin-luther-king-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `bacf30ad37e6843f7162a6457e17dec7e0024474e9b8e0d35f9ef12fc9671d25` |
| `2020-holiday-calendars.zip#2020-memorial-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `f3052963d22aca969eefb81ff8996662fc1b0a17f0bfdf53e8df37b0af5ddc5b` |
| `2020-holiday-calendars.zip#2020-presidents-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `a42697d72f03fdcc9229a17f99320418fd61a52ceb7bb82636c01cb8d818606d` |
| `2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `bcf6b5e616b91663abe532953d47dd85af9fb05e26cc4d9fe72329414ef598b4` |
| `2020-holiday-calendars.zip#2021-new-years-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `f7b782e1effb08ff84b75daf4a7d59e1cd7d64645c8504cf10d54af9d2199928` |
| `2021-holiday-calendars.zip @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `0ee0860a3a0e035eb9d079419aca3cafcc4256d296fa916647987c396dda8c59` |
| `2021-holiday-calendars.zip#2021-christmas-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `e80ee0b766f582091d04f04e1a79595c988848dec269e37d0045d7dc33a68235` |
| `2021-holiday-calendars.zip#2021-good-friday-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `2d54d45609bc774e61d65d92d9a3165372950999400dd5b102b7a9319c560356` |
| `2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `6444dd5b3513a79c4bce6b9db0746c969e432a3c2cdb5bb918c80ad44bf8fe9b` |
| `2021-holiday-calendars.zip#2021-labor-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `613623b6d3bb4ac76120d284e198799e4eedf6870119449df762a613ab53888e` |
| `2021-holiday-calendars.zip#2021-memorial-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `75b06b120bd92f77aa85f8acaca2ec661e06dba98bb64f3863a8d24987475968` |
| `2021-holiday-calendars.zip#2021-mlk-day-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `83724afe5e7f073bf58d3fd0118269c92d7d1c0c8c18ff716fbebe5e51b7c64c` |
| `2021-holiday-calendars.zip#2021-presidents-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `7a8d5ce35c639998abf64723de2cc2d649dc015b1aa6cfc3bc326ab5cdfd0c4f` |
| `2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `df6cdbc4c996109fe5a5829b5e8aaf622c20c6f16f1464dcdd9d5250c4163a47` |
### Gaps and residual risks, 2019-2021
**This era declares the family's fourth audited window.** The table as a whole carries 143 rows over 4 windows — 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 — and this era's share is **36 rows**: 8 full closures, 25 early closes and 3 `Unsourced` rows. Every row is at T1. Every date before the first window lies outside it, so `holiday_coverage` reports it as unaudited rather than as audited normal.
**Juneteenth 2019, 2020 and 2021 — three `Unsourced` rows.** CME published no Juneteenth schedule in any of the three years. Each row cites that year's own consolidated bundle — `2019-holiday-calendars.zip @2021-01-26T09:48:37Z`, `2020-holiday-calendars.zip @2026-07-30T11:18:34Z` and `2021-holiday-calendars.zip @2026-08-30T10:03:27Z` — whose member lists are CME's own account of every Globex holiday schedule it published that year and which carry no Juneteenth sheet; the four archived `holiday-calendar.html` index pages name none either, and a fresh 2018-2027 prefix CDX enumeration (`raw/cme-2019-2021-fix/cdx/cdx-files-2018-2027.json`, 369 rows, 340 distinct filenames) finds no `juneteenth` filename before 2022. Inside a contiguous window silence is the positive claim that a date was audited normal, which is false for a date the operator later marks as a holiday, so all three ship `Unsourced`, which clips nothing. 2021-06-19 is a **Saturday**: no family has a trade date there and the row changes no answer, and the row is keyed to the operator's own calendar date for the holiday rather than to an observed date CME never states. Closing condition: a CME holiday schedule naming Juneteenth in one of these three years.
**Columbus Day and Veterans Day — six dates with no row.** 2019-10-14, 2019-11-11, 2020-10-12, 2020-11-11, 2021-10-11 and 2021-11-11 lie inside this window and carry no row, so the family's ordinary week stands there. CME published settlement-time and OTC-clearing advisories for these dates — the 2019 ZIP's `settlement-notices/*-settlement-times.pdf` members and, for example, `2021-veterans-day-advisory.pdf` — but never a Globex trading schedule for them. A settlement notice is not session language (LAW-SESSION-NOT-EXPIRY), so no row is keyed to one and the block's `missing` register records the dates as gaps rather than as sourced normality. Closing condition: a CME Globex holiday schedule naming one of these dates.
**Interpretive step: this family is keyed to the `Equity` / `Equity Products` line.** No CME sheet in this era prints an outright Nikkei row. The Nikkei 225 futures trade under the compact sheets' `Equity` line and the full sheets' `Equity Products` line, and that is the line every row above records; the block's `globex_nikkei_225_dollar` rows say so on each of the 44 dates. Five dates carry `modified` status because CME's own Nikkei-labelled BTIC row diverges from that line: 2020-12-31 and 2021-12-31, where the row prints `Globex Closed` and `Closed` in a column where `Equity Products` closes at 16:00; 2021-04-02, where the sheet splits `Nikkei BTIC` from `TOPIX BTIC`; and 2021-11-25 and 2021-11-26, where it prints `Nikkei & BTIC` with its own 10:30 pre-open, 11:00 open, 16:00 halt and 00:00 close. A BTIC row is a trade-type variant, not the outright future — the consumer marks that flag itself — so the `Equity` line governs every row above. The two year-end divergences move no boundary the crate holds, because the `Equity Products` 16:00 close is that grid's ordinary close on both dates and both ship no row; the two Thanksgiving 2021 divergences are recorded beside the rows they touch, which carry the `Equity` line's 12:00 CT and 12:15 CT closes. Closing condition for a row of this family's own: a CME document that breaks out the outright Nikkei 225 future's session on a holiday date.

### 2022

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2022-01-17 | early close | `12:00` | `2022-mlk-day-holiday-schedule.xls @2022-01-17T21:22:30Z` | T1 | CME prints `12:00` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2022-02-21 | early close | `12:00` | `2022-presidents-day-holiday-schedule.xls @2022-07-04T07:38:10Z` | T1 | CME prints `12:00` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2022-04-15 | closed | `no session printed` — the operator prints no session for this date | `2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2022-05-30 | unsourced | `1:00 (close)` / `16:00` / `1:00 (close)` — the sheet merges the outright Nikkei and BTIC lines under one `Nikkei & BTIC` label and its Friday column prints the BTIC-style `1:00`; the crate models the outright line, whose close the sheet does not print here, so the row withholds the instant | `2022-memorial-day-holiday-schedule.xls @2022-07-04T06:54:38Z` | T1 | the block carries this date as an `early_close` at the BTIC-style 01:00, which is not the outright Nikkei's close, so the crate withholds the instant: the row is `unsourced` at T1 and clips nothing |
| 2022-06-20 | early close | `12:00` | `2022-juneteenth-holiday-schedule.xls @2022-06-20T20:02:10Z` | T1 | CME prints `12:00` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2022-07-04 | early close | `12:00` | `2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z` | T1 | CME prints `12:00` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2022-09-05 | early close | `12:00` | `2022-labor-day-holiday-schedule.xls @2022-07-04T06:54:41Z` | T1 | CME prints `12:00` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2022-11-24 | early close | `12:00` | `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | T1 | CME prints `12:00` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2022-11-25 | early close | `12:15` | `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | T1 | CME prints `12:15` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2022-12-26 | closed | `no session printed` — the operator prints no session for this date | `2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |

### 2023

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `no session printed` — the operator prints no session for this date | `2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2023-01-16 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2023-01-15` | T2 | no document this crate read prints a Nikkei line for this date: the 2023 one-pagers carry no Nikkei row at all, and the ten representative products this service window returns do not include it, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2023-02-20 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2023-02-19` | T2 | no document this crate read prints a Nikkei line for this date: the 2023 one-pagers carry no Nikkei row at all, and the ten representative products this service window returns do not include it, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2023-04-07 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2023-04-06` | T2 | no document this crate read prints a Nikkei line for this date: the 2023 one-pagers carry no Nikkei row at all, and the ten representative products this service window returns do not include it, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2023-05-29 | early close | `12:00` | `memorial-day-2023.pdf @2023-04-20T22:40:18Z` | T1 | CME prints `12:00` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2023-06-19 | early close | `12:00` | `juneteenth-2023.pdf @2023-06-13T18:59:49Z` | T1 | CME prints `12:00` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2023-07-03 | early close | `12:15` | `4th-of-july-2023.pdf @2023-06-27T12:50:57Z` | T1 | CME prints `12:15` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2023-07-04 | early close | `12:00` | `4th-of-july-2023.pdf @2023-06-27T12:50:57Z` | T1 | CME prints `12:00` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2023-09-04 | early close | `12:00` | `labor-day-2023.pdf @2023-08-02T19:24:46Z` | T1 | CME prints `12:00` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2023-11-23 | early close | `12:00` | `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | T1 | CME prints `12:00` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2023-11-24 | early close | `12:15` | `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | T1 | CME prints `12:15` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2023-12-25 | closed | `no Nikkei-specific line` — the EQUITIES row governs | `christmas-day-2023.pdf @2026-07-19T09:52:48Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |

### 2024

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `no Nikkei-specific line` — the EQUITIES row governs | `new-years-day-2024.pdf @2026-08-11T16:57:16Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2024-01-15 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2024-01-14` | T2 | no document this crate read prints a Nikkei line for this date: the 2023 one-pagers carry no Nikkei row at all, and the ten representative products this service window returns do not include it, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2024-02-19 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2024-02-18` | T2 | no document this crate read prints a Nikkei line for this date: the 2023 one-pagers carry no Nikkei row at all, and the ten representative products this service window returns do not include it, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2024-03-29 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2024-03-28` | T2 | no document this crate read prints a Nikkei line for this date: the 2023 one-pagers carry no Nikkei row at all, and the ten representative products this service window returns do not include it, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2024-05-27 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2024-05-26` | T2 | no document this crate read prints a Nikkei line for this date: the 2023 one-pagers carry no Nikkei row at all, and the ten representative products this service window returns do not include it, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2024-06-19 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2024-06-18` | T2 | no document this crate read prints a Nikkei line for this date: the 2023 one-pagers carry no Nikkei row at all, and the ten representative products this service window returns do not include it, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2024-07-03 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2024-07-03` | T2 | no document this crate read prints a Nikkei line for this date: the 2023 one-pagers carry no Nikkei row at all, and the ten representative products this service window returns do not include it, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2024-07-04 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2024-07-03` | T2 | no document this crate read prints a Nikkei line for this date: the 2023 one-pagers carry no Nikkei row at all, and the ten representative products this service window returns do not include it, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2024-09-02 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2024-09-01` | T2 | no document this crate read prints a Nikkei line for this date: the 2023 one-pagers carry no Nikkei row at all, and the ten representative products this service window returns do not include it, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2024-11-28 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2024-11-27` | T2 | no document this crate read prints a Nikkei line for this date: the 2023 one-pagers carry no Nikkei row at all, and the ten representative products this service window returns do not include it, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2024-11-29 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2024-11-27` | T2 | no document this crate read prints a Nikkei line for this date: the 2023 one-pagers carry no Nikkei row at all, and the ten representative products this service window returns do not include it, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2024-12-24 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2024-12-24` | T2 | no document this crate read prints a Nikkei line for this date: the 2023 one-pagers carry no Nikkei row at all, and the ten representative products this service window returns do not include it, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2024-12-25 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2024-12-24` | T2 | no document this crate read prints a Nikkei line for this date: the 2023 one-pagers carry no Nikkei row at all, and the ten representative products this service window returns do not include it, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2024-12-31 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2024-12-31..2025-01-02` | T2 | no document this crate read prints a Nikkei line for this date: the 2023 one-pagers carry no Nikkei row at all, and the ten representative products this service window returns do not include it, so the row is `unsourced` and clips nothing rather than reading as audited normal |
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
| `CME-SVC-2024-12-31..2025-01-02` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-12-31&toEventDate=2025-01-02&isProtected&_t=1734710019538> | archive capture 2024-12-20T15:53:40Z | T2 | `6b6399a1de34bbc9eea0a135691dfed90edee2a0c705162b1840e214256e4c46` |
### Gaps and residual risks, 2022-2024

**The three 2023 dates the operator published nothing for — 2023-01-16, 2023-02-20 and 2023-04-07.** They ship `Unsourced`, cited to the T2 captures `CME-SVC-2023-01-15`, `CME-SVC-2023-02-19` and `CME-SVC-2023-04-06`, each of which is the operator's own machine channel read as bytes and returns an empty event list for its window: 2023 Dr. Martin Luther King, Jr. Day (Monday 16 January 2023), 2023 Presidents Day (Monday 20 February 2023) and 2023 Good Friday (Friday 7 April 2023). The block's `missing` register records the channels searched — `holiday-calendar/files/*.xls`, of which only a compact MGEX/DME workbook exists for MLK, and `trading-hours/files/<holiday>-2023.pdf`, which 404s — so the gap is "not worked up", not "no source exists". `Unsourced` clips nothing, so each of these dates still resolves to the family's ordinary week. Closing condition: a CME holiday schedule for 2023 covering this date at T1, or a T2 window that carries its events. The day after each of the three is a second, unmodelled gap: 2023-01-17 and 2023-02-21 (and 2023-04-10 for `globex_grains`) may have lost their prior-evening leg the way 2024-01-02 and 2023-12-26 did, and no artifact this crate read states whether they did, so they ship no row and the family's ordinary week stands there.

**The 2024-04-01 re-open.** The Good Friday window `CME-SVC-2024-03-28` ends at 2024-03-30 and prints empty event lists for both 2024-03-29 and 2024-03-30, so CME published nothing this crate read for the Sunday 2024-03-31 evening leg that would carry trade date 2024-04-01. Inside a contiguous window silence would read as audited normal, so 2024-04-01 ships **no row** and is a declared gap rather than an audited one. Closing condition: a CME service response covering 2024-03-31, or a T1 statement of that Sunday's re-open. For `globex_grains` the same silence is what withholds a possible late open on 2024-04-01.

**The 2022-05-30 merged `Nikkei & BTIC` row.** The 2022 Memorial Day workbook prints one merged line — "Nikkei & BTIC ... Friday, May 27 Regular Fri. Close 1:00, Pre-opening 10:30, Open 11:00, Close 16:00 ;; Sunday, May 29 Pre-opening** 16:00, Open 17:00 ;; Monday, May 30 Halt \"1:00 (close)\" ..." — so its Friday close is the BTIC-style `1:00`, while the MLK and Presidents Day sheets print the outright Nikkei's own `16:00` Friday close beside a separate `Nikkei BTIC` row. The crate models the outright Nikkei, so 2022-05-30 ships `Unsourced` at T1 against `2022-memorial-day-holiday-schedule.xls @2022-07-04T06:54:38Z` rather than an early close at 01:00 that would read as sourced and be wrong for the identity. Closing condition: a CME document that states the outright Nikkei's own close on a Memorial Day, or a tiling that separates the two lines.

**Seventeen `Unsourced` rows: 2022-05-30, the three 2023 dates and thirteen 2024 dates.** The crate withholds every date in this window on which no document it read prints a Nikkei line: the 2023 one-pagers carry no Nikkei row at all (measured: zero occurrences of the string in each PDF), and the ten `THBP-A` service responses CME's own page queries carry only ten representative products, none of them Nikkei. The 2023 and 2024 rows therefore prove a channel limit rather than a quiet market, which is why they ship `Unsourced` and not silence. Closing condition: a `THBP-B` window (`id=168,167,320,323,19,27`, the only product set carrying the Nikkei line) over those dates, or a T1 restatement.

**The 2024-12-31 citation was ambiguous and is now re-keyed.** This module cites the 2022-2024 window's T2 response under `CME-SVC-2024-12-31..2025-01-02` and the 2025-2027 window's under `CME-SVC-2024-12-31`, so each id names the window it resolves to: the two responses are different artifacts — both address `fromEventDate=2024-12-31&toEventDate=2025-01-02`, and they differ in that the 2022-2024 window's request carries `isProtected` and `_t=1734710019538` while the 2025-2027 window's omits both (sha256 `6b6399a1…` and `375c70ee…` respectively) — and one id resolved to both. The 2024-12-31 `Unsourced` row and the 2022-2024 documents table carry the long id; the 2025-01-01 row keeps the short one.

**Normal-week notes that ship no row.** Where a printed token falls outside the family's ordinary week but moves no boundary a scalar holiday row can state, the date ships nothing and the token is recorded here: the 2022 New Year's workbooks print the `Nikkei/TOPIX BTIC` `Close 00:00` on 2022-01-01, a BTIC close for the next trade date; the 2023 and 2024 Independence Day and New Year grain entries print the next trade date's `06:00 (PREOPEN)`, which is the no-evening-leg marker the six `globex_grains` late opens are read from — those rows are in that family's own table and evidence file and this family has none; and the 2024-12-31 grain entry points at `2025-01-02 06:00 preopen`, outside this window, so no row ships and the 2025-2027 table must state that trade date.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `16:00 preopen /TD 2025-01-02; 17:00 open /TD 2025-01-02` — no close event on 2025-01-01 | `CME-SVC-2024-12-31` | T2 | eventDates 2024-12-31 and 2025-01-01; CME prints `16:00 closed /TD 2024-12-31` with no evening re-open, and assigns no trade date 2025-01-01 |
| 2025-01-20 | early close | `12:00 preopen /TD 2025-01-21; 17:00 open /TD 2025-01-21` — 12:00 CT | `CME-SVC-2025-01-19` | T2 | eventDate 2025-01-20, CME trade date 2025-01-21; the Sunday-evening leg opened 2025-01-19 17:00 CT |
| 2025-02-17 | early close | `12:00 preopen /TD 2025-02-18; 17:00 open /TD 2025-02-18` — 12:00 CT | `CME-SVC-2025-02-16` | T2 | eventDate 2025-02-17, CME trade date 2025-02-18 |
| 2025-04-18 | closed | `no events published` | `CME-SVC-2025-04-17` | T2 | eventDates 2025-04-17 and 2025-04-18; CME prints `16:00 closed /TD 2025-04-17` with no evening re-open |
| 2025-05-26 | early close | `12:00 preopen /TD 2025-05-27; 17:00 open /TD 2025-05-27` — 12:00 CT | `CME-SVC-2025-05-25` | T2 | eventDate 2025-05-26, CME trade date 2025-05-27 |
| 2025-06-19 | early close | `12:00 preopen /TD 2025-06-20; 17:00 open /TD 2025-06-20` — 12:00 CT | `CME-SVC-2025-06-18` | T2 | eventDate 2025-06-19, CME trade date 2025-06-20 |
| 2025-07-03 | early close | `12:15 closed /TD 2025-07-03; 16:45 preopen /TD 2025-07-04; 17:00 open /TD 2025-07-04` — 12:15 CT | `CME-SVC-2025-07-03` | T2 | eventDate 2025-07-03, CME trade date 2025-07-03; the evening leg runs normally |
| 2025-07-04 | early close | `12:00 closed /TD 2025-07-04` — 12:00 CT | `CME-SVC-2025-07-03` | T2 | eventDate 2025-07-04, CME trade date 2025-07-04 |
| 2025-09-01 | early close | `12:00 preopen /TD 2025-09-02; 17:00 open /TD 2025-09-02` — 12:00 CT | `CME-SVC-2025-08-31` | T2 | eventDate 2025-09-01, CME trade date 2025-09-02 |
| 2025-11-27 | early close | `12:00 preopen /TD 2025-11-28; 17:00 open /TD 2025-11-28` — 12:00 CT | `CME-SVC-B-2025-11-26` | T2 | eventDate 2025-11-27, CME trade date 2025-11-28; `NKD` and `NIY` print this line themselves |
| 2025-11-28 | early close | `07:00 preopen /TD 2025-11-28; 07:30 open /TD 2025-11-28; 12:15 closed /TD 2025-11-28` — 12:15 CT | `CME-SVC-B-2025-11-26` | T2 | eventDate 2025-11-28, CME trade date 2025-11-28; the morning pre-open pair is the gap recorded below |
| 2025-11-29 | closed | `no events published` | `CME-SVC-B-2025-11-26` | T2 | eventDate 2025-11-29; `NKD` and `NIY` publish no events, and CME's 2025 Globex table states the period as "27 - 29 November 2025" |
| 2025-12-24 | early close | `12:15 closed /TD 2025-12-24` — 12:15 CT, no evening re-open | `CME-SVC-B-2025-12-24` | T2 | eventDate 2025-12-24, CME trade date 2025-12-24 |
| 2025-12-25 | closed | `16:00 preopen /TD 2025-12-26; 17:00 open /TD 2025-12-26` — no close event on 2025-12-25 | `CME-SVC-B-2025-12-24` | T2 | eventDates 2025-12-24 and 2025-12-25; the 2025-12-24 record's missing evening re-open is what this row removes |

**Interpretive steps, 2025.**

- **The Nikkei line is absent through Labor Day 2025, and those nine rows are taken from
  the Equity Index line of the same capture.** CME's service still answers for past windows
  back to Thanksgiving 2025 but no further, so `NKD`/`NIY` return empty schedules for the
  eight windows New Year 2025 through Labor Day 2025 (`raw/cme-2025-2027-repair/live/edgeB_*.md`,
  probed 2026-09-12). The rows for 2025-01-01, 2025-01-20, 2025-02-17, 2025-04-18,
  2025-05-26, 2025-06-19, 2025-07-03, 2025-07-04 and 2025-09-01 are therefore the `ES` line
  of the ten-product capture, and they ship with the `THBP-A` document id. The corroboration
  is direct rather than assumed: on every one of the 36 product-dates from Thanksgiving 2025
  to 2028-01-01 where CME publishes both lines, `NKD` and `NIY` match `ES` event for event
  and trade date for trade date, including the single date on which the Equity Index line
  diverges from its neighbours — Good Friday 2026, where `NKD`/`NIY` close 08:15 CT with `ES`
  and not 10:15 CT with `ZN`, `6E` and `BTC`. The date this inference carries most weight on
  is 2025-07-03, where `ES` closes 12:15 CT while `ZN`, `6E`, `CL`, `GC` and `BTC` all close
  16:00 CT; it is listed as a residual risk below.
- **Trade-date key, and where it parts company with CME's printed trade date.** The table is
  keyed by the crate's own venue-local trade date — the Chicago date of the containing
  session's final close (design memo D1). On the seven dates whose noon event CME publishes
  as a `preopen` rather than a `closed`, CME assigns the whole span the *following* business
  day's trade date, because no settlement occurs on the holiday. The crate keeps the holiday
  as the trade date, because a final close does occur there at 12:00 CT. `is_open` agrees
  with CME event for event on every one of those dates; only the label differs. Recorded as
  a residual risk below.
- **Eve records are evidence, not rows.** `16:00 closed /TD <eve>` with no `16:45 preopen`
  and no `17:00 open` — CME's `[N6]` shape, printed on 2024-12-31, 2025-04-17 and 2025-12-31 —
  is the family's ordinary daytime close plus a missing evening leg. The neighbouring
  `Closed` row already deletes that leg, so the eve carries no row of its own.

**Gaps, 2025.**

- **executable** — 2025-11-28 additionally prints `07:00 preopen /TD 2025-11-28; 07:30 open
  /TD 2025-11-28` before the 12:15 CT close, with no `closed` or `paused` event between the
  2025-11-27 17:00 CT open and that 07:00 pre-open. Read literally, matching stopped at some
  unstated instant that morning and resumed at 07:30 CT. That is intraday topology, not a
  scalar boundary, so LAW-HOLIDAY-SCOPE records it as a gap rather than approximating it;
  the early-close row is unaffected. It is the only date in the whole 2025-2027 window where
  this shape appears for this family — Thanksgiving 2026 and 2027 print the 12:15 CT close
  alone. Closing condition: an operator statement naming the morning halt's start, or the
  block rows of design memo §7 follow-up 8 (#93).
- **residual risk** — the nine 2025 rows through Labor Day 2025 rest on the Equity Index
  line rather than on a published Nikkei line, because CME's channel no longer answers for
  those windows and no archived capture of the `THBP-B` id set exists for them. Closing
  condition: an archived `THBP-B` capture of any of the eight windows, or a `THBP-A` capture
  that carries `NKD`.
- **residual risk** — the crate's trade date differs from CME's printed trade date on every
  `preopen`-at-noon holiday. No query the crate answers is wrong; a consumer comparing the
  crate's `trade_date` against a CME settlement file will see the holiday where CME shows the
  next business day.
- **not representable** — order-entry deviations. On 2025-01-01 and 2025-12-25 the pre-open
  starts at 16:00 CT rather than the normal 16:45 CT. The table's vocabulary is `DayPolicy`'s
  and has no order-entry boundary; the family models no order-entry phase at all, so this
  changes nothing the crate reports.
- **Saturday 2025-11-29 ships a row.** `CME-SVC-B-2025-11-26` publishes no events for it,
  the ten-product capture agrees, and CME's 2025 Globex table states the period as
  "27 - 29 November 2025", so it is a sourced closure. The family's grid has no
  Friday-evening open and therefore no Saturday trade date, so the row changes no answer
  here; it ships because one audited operator closure ships in every family that routes to
  the venue, so the D17 venue intersection is computed from one uniform input.
- **audited, no row** — Columbus Day and Veterans Day. CME publishes settlement and clearing
  advisories but no Globex trading schedule for either, and the service returns the normal
  grid. Coverage is contiguous, so those dates read as audited normal; that is stated here
  rather than left implicit.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `no events published` | `CME-SVC-B-2025-12-31` | T2 | eventDates 2025-12-31 and 2026-01-01; CME prints `16:00 closed /TD 2025-12-31` with no evening re-open |
| 2026-01-19 | early close | `12:00 preopen /TD 2026-01-20; 17:00 open /TD 2026-01-20` — 12:00 CT | `CME-SVC-B-2026-01-18` | T2 | eventDate 2026-01-19, CME trade date 2026-01-20; the Sunday-evening leg opened 2026-01-18 17:00 CT carrying the same CME trade date |
| 2026-02-16 | early close | `12:00 preopen /TD 2026-02-17; 17:00 open /TD 2026-02-17` — 12:00 CT | `CME-SVC-B-2026-02-15` | T2 | eventDate 2026-02-16, CME trade date 2026-02-17 |
| 2026-04-03 | early close | `08:15 closed /TD 2026-04-03` — 08:15 CT | `CME-SVC-B-2026-04-01` | T2 | eventDate 2026-04-03, CME trade date 2026-04-03; `NKD` and `NIY` track the Equity Index instant here, not the 10:15 CT of `ZN`, `6E` and `BTC` |
| 2026-05-25 | early close | `12:00 preopen /TD 2026-05-26; 17:00 open /TD 2026-05-26` — 12:00 CT | `CME-SVC-B-2026-05-24` | T2 | eventDate 2026-05-25, CME trade date 2026-05-26 |
| 2026-06-19 | early close | `12:00 closed /TD 2026-06-22` — 12:00 CT | `CME-SVC-B-2026-06-18` | T2 | eventDate 2026-06-19, CME trade date 2026-06-22; the Thursday-evening leg opened 2026-06-18 17:00 CT |
| 2026-06-22 | replacement blocks | `05:00 open; 17:00 closed /TD 2026-06-22` on eventDate 2026-06-20 (`CME-SVC-B-2026-06-18`); `16:00 preopen; 17:00 open /TD 2026-06-22` on eventDate 2026-06-21 and `16:00 closed /TD 2026-06-22` on eventDate 2026-06-22 (`CME-SVC-B-2026-06-21`) | `CME-SVC-B-2026-06-18` | T2 | the complete trade date: the Saturday session from the first window, and the Sunday Pre-Open and Sunday-17:00-to-Monday-16:00 session from the second — the first window stops at the Saturday and prints no Sunday entry at all |
| 2026-07-03 | early close | `12:00 closed /TD 2026-07-06` — 12:00 CT | `CME-SVC-B-2026-07-03` | T2 | eventDate 2026-07-03, CME trade date 2026-07-06 |
| 2026-07-06 | replacement blocks | `05:00 open; 17:00 closed /TD 2026-07-06` on eventDate 2026-07-04; `16:00 preopen; 17:00 open` on the Sunday, both CME trade date 2026-07-06 | `CME-SVC-B-2026-07-03` | T2 | the complete trade date: the Saturday session, the Sunday Pre-Open and the Sunday-17:00-to-Monday-16:00 session |
| 2026-09-07 | early close | `12:00 preopen /TD 2026-09-08; 17:00 open /TD 2026-09-08` — 12:00 CT | `CME-SVC-B-2026-09-06` | T2 | eventDate 2026-09-07, CME trade date 2026-09-08 |
| 2026-11-26 | early close | `12:00 preopen /TD 2026-11-27; 17:00 open /TD 2026-11-27` — 12:00 CT | `CME-SVC-B-2026-11-25` | T2 | eventDate 2026-11-26, CME trade date 2026-11-27 |
| 2026-11-27 | early close | `12:15 closed /TD 2026-11-27` — 12:15 CT | `CME-SVC-B-2026-11-25` | T2 | eventDate 2026-11-27, CME trade date 2026-11-27 |
| 2026-12-24 | early close | `12:15 closed /TD 2026-12-24` — 12:15 CT, no evening re-open | `CME-SVC-B-2026-12-24` | T2 | eventDate 2026-12-24, CME trade date 2026-12-24 |
| 2026-12-25 | closed | `no events published` | `CME-SVC-B-2026-12-24` | T2 | eventDates 2026-12-24 and 2026-12-25; the 2026-12-24 record's missing evening re-open is what this row removes |

**Interpretive steps, 2026.**

- Every row is the family's own published line: `NKD` and `NIY` appear in the `THBP-B`
  captures for all twelve dates, so no inference from the Equity Index line is made anywhere
  in 2026.
- Good Friday 2026 is the date that proves the 2025 inference rather than resting on it.
  `NKD` and `NIY` print `08:15 closed /TD 2026-04-03`; `ZN`, `6E` and `BTC` print 10:15 CT on
  the same date. The Nikkei follows the Equity Index instant, which is what the 2025 rows
  assume.
- On 2026-06-19 and 2026-07-03 CME prints a real `closed` event at 12:00 CT and still labels
  it with the following Monday's trade date. The crate keys the row to the Friday, for the
  same reason as the 2025 `preopen` dates.

**Gaps, 2026.**

- **executable, was not representable — resolved 2026-09-25 UTC by Stage 4 (#116).** The
  Saturday sessions of 2026-06-20 and 2026-07-04, `05:00 open /TD 2026-06-22;
  17:00 closed /TD 2026-06-22` and `05:00 open /TD 2026-07-06; 17:00 closed /TD 2026-07-06`,
  on a week whose normal grid has no Saturday session. The block rows #93 shipped supply the
  vocabulary and this table now states both trade dates, so the crate no longer reports these
  Saturdays closed. Each row states the **complete** trade date. For 2026-06-22 the two
  halves come from two windows: the Saturday from `CME-SVC-B-2026-06-18`, and the Sunday
  Pre-Open and Sunday-17:00-to-Monday-16:00 session from `CME-SVC-B-2026-06-21`, because
  that Saturday window stops at 2026-06-20 and prints no Sunday entry at all. For 2026-07-06
  a single window carries both halves — `CME-SVC-B-2026-07-03`, which runs
  2026-07-03 .. 2026-07-05, prints the Saturday `05:00 open; 17:00 closed` and the Sunday
  Pre-Open and evening open itself.
- **residual risk** — the trade-date divergence described under 2025 applies to 2026-01-19,
  2026-02-16, 2026-05-25, 2026-06-19, 2026-07-03, 2026-09-07 and 2026-11-26.
- **audited, no row** — Saturday 2026-04-04, the day after Good Friday. CME's service
  publishes no events for it; the grid has no Saturday trade date.

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `no events published` | `CME-SVC-B-2026-12-31` | T2 | eventDates 2026-12-31 and 2027-01-01; CME prints `16:00 closed /TD 2026-12-31` with no evening re-open |
| 2027-01-18 | early close | `12:00 preopen /TD 2027-01-19; 17:00 open /TD 2027-01-19` — 12:00 CT | `CME-SVC-B-2027-01-17` | T2 | eventDate 2027-01-18, CME trade date 2027-01-19 |
| 2027-02-15 | early close | `12:00 preopen /TD 2027-02-16; 17:00 open /TD 2027-02-16` — 12:00 CT | `CME-SVC-B-2027-02-14` | T2 | eventDate 2027-02-15, CME trade date 2027-02-16 |
| 2027-03-26 | closed | `no events published` | `CME-SVC-B-2027-03-25` | T2 | eventDates 2027-03-25 and 2027-03-26; CME prints `16:00 closed /TD 2027-03-25` with no evening re-open |
| 2027-05-31 | early close | `12:00 preopen /TD 2027-06-01; 17:00 open /TD 2027-06-01` — 12:00 CT | `CME-SVC-B-2027-05-30` | T2 | eventDate 2027-05-31, CME trade date 2027-06-01 |
| 2027-06-18 | early close | `12:00 closed /TD 2027-06-21` — 12:00 CT | `CME-SVC-B-2027-06-17` | T2 | eventDate 2027-06-18, CME trade date 2027-06-21; the Thursday-evening leg opened 2027-06-17 17:00 CT |
| 2027-06-21 | replacement blocks | `05:00 open; 17:00 closed /TD 2027-06-21` on eventDate 2027-06-19 (`CME-SVC-B-2027-06-17`); `16:00 preopen; 17:00 open /TD 2027-06-21` on eventDate 2027-06-20 and `16:00 closed /TD 2027-06-21` on eventDate 2027-06-21 (`CME-SVC-B-2027-06-20`) | `CME-SVC-B-2027-06-17` | T2 | as 2026-06-22: the Saturday from the first window, the Sunday legs from the second |
| 2027-07-05 | early close | `12:00 preopen /TD 2027-07-06; 17:00 open /TD 2027-07-06` — 12:00 CT | `CME-SVC-B-2027-07-04` | T2 | eventDate 2027-07-05, CME trade date 2027-07-06 |
| 2027-09-06 | early close | `12:00 preopen /TD 2027-09-07; 17:00 open /TD 2027-09-07` — 12:00 CT | `CME-SVC-B-2027-09-05` | T2 | eventDate 2027-09-06, CME trade date 2027-09-07 |
| 2027-11-25 | early close | `12:00 preopen /TD 2027-11-26; 17:00 open /TD 2027-11-26` — 12:00 CT | `CME-SVC-B-2027-11-24` | T2 | eventDate 2027-11-25, CME trade date 2027-11-26 |
| 2027-11-26 | early close | `12:15 closed /TD 2027-11-26` — 12:15 CT | `CME-SVC-B-2027-11-24` | T2 | eventDate 2027-11-26, CME trade date 2027-11-26 |
| 2027-12-24 | closed | `no events published` | `CME-SVC-B-2027-12-22` | T2 | eventDates 2027-12-23 and 2027-12-24; CME prints `16:00 closed /TD 2027-12-23` with no evening re-open |

**Interpretive steps, 2027.**

- All eleven rows are the family's own published line, from the `THBP-B` live retrievals of
  2026-09-12.
- 2027-12-23 is CME's own holiday date for Christmas 2027 but is **not** a crate row. Its
  daytime close is the ordinary 16:00 CT; what is missing is the Thursday-evening leg, and
  `Closed(2027-12-24)` already removes it. 2027-12-31 is likewise normal: `16:00 closed
  /TD 2027-12-31` is the family's own Friday close, and the grid has no Friday-evening open
  to lose.

**Gaps, 2027.**

- **executable, was not representable — resolved 2026-09-25 UTC by Stage 4 (#116).** The
  Saturday session of 2027-06-19, `05:00 open /TD 2027-06-21; 17:00 closed /TD 2027-06-21`.
  Same shape as the two 2026 Saturdays, and now a row on the same terms as 2026-06-22: the
  Saturday from `CME-SVC-B-2027-06-17`, whose window ends at 2027-06-19, and the Sunday legs
  from `CME-SVC-B-2027-06-20`.
- **residual risk** — the trade-date divergence described under 2025 applies to 2027-01-18,
  2027-02-15, 2027-05-31, 2027-06-18, 2027-07-05, 2027-09-06 and 2027-11-25.
- **coverage limit** — CME's published future for this family ends with the 2027-12-30 ..
  2028-01-02 window, whose only non-normal record is Saturday 2028-01-01 and which CME itself
  labels normal for this grid. Coverage therefore stops at 2027-12-31 and 2028-01-01 ships no
  row; a query for it gets the normal week and `holiday_on` answers `None` because the date
  is outside the window, which `holiday_coverage` is what distinguishes.

### 2016

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `no session printed` | `2016-new-years-holiday-schedule.pdf @2016-01-08` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2016-01-18 | early close | `1200 CT / 1300 ET / 1800 UTC` | `2016-holiday-calendars.zip#2016-martin-luther-king-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-02-15 | early close | `1200 CT / 1300 ET / 1800 UTC` | `2016-holiday-calendars.zip#2016-presidents-day-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-03-25 | closed | `no session printed` | `2016-holiday-calendars.zip#2016-good-friday-holiday-schedule.pdf @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2016-05-30 | early close | `1200 CT / 1300 ET / 1700 UTC` | `2016-holiday-calendars.zip#2016-memorial-day-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-07-04 | early close | `1200 CT / 1300 ET / 1700 UTC` | `2016-holiday-calendars.zip#2016-4th-of-july-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-09-05 | early close | `1200 CT / 1300 ET / 1700 UTC` | `2016-holiday-calendars.zip#2016-labor-day-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-11-24 | early close | `1200 CT / 1300 ET / 1800 UTC` | `2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-11-25 | early close | `1215 CT / 1315 ET / 1815 UTC` | `2016-holiday-calendars.zip#2016-thanksgiving-holiday-schedule.pdf @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2016-12-26 | closed | `no session printed` | `2016-holiday-calendars.zip#2016-christmas-holiday-schedule.pdf @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |

### 2017

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | `no session printed` | `2016-holiday-calendars.zip#2017-new-years-holiday-schedule.pdf @2017-06-28` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2017-01-16 | early close | `12:00 CT / 13:00 ET` | `2017-martin-luther-king-holiday-schedule.xls @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-02-20 | early close | `12:00 CT / 13:00 ET` | `2017-presidents-day-holiday-schedule.xls @2017-06-28` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-04-14 | closed | `no session printed` | `2017-good-friday-holiday-schedule.xls @2017-05-05` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2017-05-29 | early close | `12:00 CT / 13:00 ET` | `2017-memorial-day-holiday-schedule.xls @2017-10-25` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-07-03 | early close | `12:15 CT / 13:15 ET` | `2017-4th-of-july-holiday-schedule.xls @2017-10-25` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-07-04 | early close | `12:00 CT / 13:00 ET` | `2017-4th-of-july-holiday-schedule.xls @2017-10-25` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-09-04 | early close | `12:00 CT / 13:00 ET` | `2017-labor-day-holiday-schedule.xls @2017-10-25` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-11-23 | early close | `12:00 CT / 13:00 ET` | `2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-11-24 | early close | `12:15 CT / 13:15 ET` | `2017-holiday-calendars.zip#2017-thanksgiving-holiday-schedule.xls @2021-01-26` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2017-12-25 | closed | `no session printed` | `2017-holiday-calendars.zip#2017-christmas-holiday-schedule.xls @2021-01-26` | T1 | CME prints the closure for this date; trade date = the operator's event date |

### 2018

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `no session printed` | `2018-new-years-holiday-schedule.xls @2018-01-06` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2018-01-15 | early close | `12:00 CT / 13:00 ET` | `2018-martin-luther-king-holiday-schedule.xls @2018-05-08` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-02-19 | early close | `12:00 CT / 13:00 ET` | `2018-presidents-day-holiday-schedule.xls @2018-05-08` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-03-30 | closed | `no session printed` | `2018-holiday-calendars.zip#2018-good-friday-holiday-schedule.xls @2026-08-30` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2018-05-28 | early close | `12:00 CT / 13:00 ET` | `2018-holiday-calendars.zip#2018-memorial-day-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-07-03 | early close | `12:15 CT / 13:15 ET` | `2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-07-04 | early close | `12:00 CT / 13:00 ET` | `2018-holiday-calendars.zip#2018-4th-of-july-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-09-03 | early close | `12:00 CT / 13:00 ET` | `2018-holiday-calendars.zip#2018-labor-day-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-11-22 | early close | `12:00 CT / 13:00 ET` | `2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-11-23 | early close | `12:15 CT / 13:15 ET` | `2018-holiday-calendars.zip#2018-thanksgiving-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-12-24 | early close | `12:15 CT / 13:15 ET` | `2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30` | T1 | CME prints this date's own final close; the session that opened the previous evening is clipped here, and trade date = the operator's event date |
| 2018-12-25 | closed | `no session printed` | `2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30` | T1 | CME prints the closure for this date; trade date = the operator's event date |
| 2018-12-26 | late open | `15:30 CT / 16:30 ET (Wednesday, December 26)` | `2018-holiday-calendars.zip#2018-christmas-holiday-schedule.xls @2026-08-30` | T1 | the operator prints this date's own first open after the holiday; the row is keyed to the trade date that open belongs to |

**Interpretive step, 2018-12-26:** the block records this date's status as `normal`, and the crate ships a row for it. No Nikkei-specific line is printed; the family is governed by the Equity line, whose sheet prints `Pre-opening` 15:15 and `Open` 15:30 on 26 December against a 16:00 CT close. That is the era's routine extended-session handoff and the trade date's first open is later than the grid's, so the row is a late open keyed to 2018-12-26.

## Sources

Row review: 2026-08-24 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.cmegroup.com/markets/equities/international-indices/nikkei-225-dollar.contractSpecs.html> — CME Nikkei 225 Dollar contract specification, the current grid.
- <https://www.cmegroup.com/CmeWS/mvc/ContractSpecs/List/productId/168> — CME `ContractSpecs` service for product id 168, corroborating the current grid.
- <https://www.cmegroup.com/markets/equities/files/trade-japanese-equity-index-futures-fact-card.pdf> — CME Japanese equity index futures fact card.
- <https://www.cmegroup.com/trading-hours.html> — CME trading-hours page, which states that hours are U.S. Central unless otherwise noted.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150817.html> — CME Globex Notice #20150817 of 17 August 2015, the 2015-09-20 revision's source.
- <https://www.cmegroup.com/rulebook/files/ser-6465_Equity_Index_Futures_Options_on_Equity_Index_Futures_20121114.pdf> — CME SER-6465, the 2012-11-18 revision's source.
- <https://www.cmegroup.com/rulebook/files/ser_6554R_-_CME_Modifies_Trading_Hours_for_International_Equity_Index_futures_on_20130304.pdf> — CME SER-6554R, the 2013-03-03 revision's source, naming Nikkei 225 Dollar Futures explicitly.
- <https://web.archive.org/web/20150905151851/http://www.cmegroup.com/trading/equity-index/international-index/nikkei-225-dollar_contract_specifications.html> — NKD contract specification — capture 2015-09-05, "5:00 p.m. previous day - 4:15 p.m.".
- <https://web.archive.org/web/20151127190940/http://www.cmegroup.com:80/trading/equity-index/international-index/nikkei-225-dollar_contract_specifications.html> — NKD contract specification — capture 2015-11-27, "5:00 p.m. - 4:00 p.m. Chicago Time/CT".
- <https://web.archive.org/web/20100310022002id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2010-03-10, the materially different 2010 NKD grid.
- <https://web.archive.org/web/20100407094843id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2010-04-07, still the 2010 grid.
- <https://web.archive.org/web/20110112032949id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2011-01-12, the first sourced appearance of the served grid.
- <https://web.archive.org/web/20110811113223id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2011-08-11, the served grid restated.

Official origin of the trading-hours captures: <http://www.cmegroup.com/trading_hours/>.

## Gaps and residual risks

- **executable** — the 2010 grid is sourced but structurally different and its changeover day is undated, so dates before 2011-01-12 resolve to a sessionless profile rather than carrying either grid. CME's 2010-03-10 and 2010-04-07 captures read a daytime-anchored, DST-dependent grid — CDT 03:00–15:15 reopening 15:30–16:30 and 17:00–18:00, CST 02:00–15:15 with no Sunday hours — and the 2011-01-12 capture already reads the served grid, with no capture and no located CME notice in between. Serving the continuous grid across 2010 would report the contract open all night when it was closed, which an earlier revision of the module did and which is corrected. Closing condition: a CME document that dates the changeover, or a capture inside the 2010-04-07..2011-01-12 window. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **residual risk** — encoding the 2010 grid itself would need seasonal CDT/CST rules and a boundary that is still undated, so it is left sourced-but-unmodelled.
- **corrected at the migration** — the ledger note carried into this file ended with a sentence, written for an earlier revision of the module, saying the pre-2012 grid "is now extended to the January-2010 floor". The module does not do that and must not: `nkd_profile_at` returns `NKD_CLOSED` below 2011-01-12. That sentence was removed on 2026-09-12 and an editorial marker left in its place; the surviving correction earlier in the same note is the authoritative statement.
- **scope** — Nikkei 225 Dollar outrights only. BTIC (`NKT`) is separately scheduled on its own CME-published hours and takes its own key if a consumer maps one; the 16:00–17:00 CT daily break is a maintenance period, not an order-entry phase.

## Module narrative (moved from src/calendar/schedules/futures/us/cme_nikkei.rs on 2026-09-12 UTC)

NKD outrights run one continuous Globex envelope per trade date: the session
opens 17:00 CT on the previous calendar evening and closes 16:00 CT on the
trade date, with the 60-minute 16:00-17:00 CT break separating consecutive
trade dates. CME's own wording: "Sunday - Friday 6:00 p.m. - 5:00 p.m. ET
(5:00 p.m. - 4:00 p.m. CT) with a 60-minute break each day beginning at
5:00 p.m. ET (4:00 p.m. CT)". There is no intraday halt as of the 2026-08-24 review. Friday is
absent from the opening-day mask because a Friday-evening open would belong
to a Saturday trade date, which does not exist; that omission is what
produces the Friday 16:00 CT weekly wrap.

HOW THE NKD GRID DIFFERS FROM THE STANDARD CME EQUITY-INDEX GRID (the reason
`MarketHoursKey::GlobexEquityIndex` explicitly excludes NKD): the U.S.-grid
contracts carry a pit-anchored 08:30-15:15 CT regular session with the
electronic envelope modelled around it as extended hours, and they carried a
15:15-15:30 CT halt until 2021-06-27. NKD is a pure-Globex international
equity-index contract: it has no pit/RTH split, so its entire envelope is the
regular session, and its 15:15-15:30 CT halt was removed eight years earlier,
on 2013-03-04, by a notice scoped to International Equity Index futures only.
The two grids differed from the (undatable, see below) 2010 change through
2012-11-18, and again through the 2013-03-04 halt removal. Today the envelopes
coincide, but the regular/extended split does not, so the key stays separate.

https://www.cmegroup.com/markets/equities/international-indices/nikkei-225-dollar.contractSpecs.html
https://www.cmegroup.com/CmeWS/mvc/ContractSpecs/List/productId/168
https://www.cmegroup.com/markets/equities/files/trade-japanese-equity-index-futures-fact-card.pdf
https://www.cmegroup.com/trading-hours.html

CME publishes no normal-week pre-open or order-entry start time for NKD on the
contract specs page, the ContractSpecs service, or the Japanese equity index
fact card, so no extended phase is asserted. The 16:00-17:00 CT daily break is
a maintenance/closed period, not an order-entry phase, and BTIC ("Sunday -
Friday 6:00 p.m. ET - 3:30 p.m. Tokyo time ... and Monday - Friday Noon to
5:00 p.m. ET") is separately scheduled, on its own published hours, so it is
not a phase of this outright order book. Both are deliberately omitted rather
than modelled as extended sessions.

That is a statement about scope, not about tradability: the Nikkei BTIC
instruments are their own order book with their own CME-published hours, and
the quoted sentence above is itself the primary source for their second daily
window. Should they be authored, they take their own key rather than becoming
a phase here — see the trade-type handoff in `docs/plans/`, whose survey found
exactly one trade-type root out of roughly 180 that genuinely rides its
underlying's clock.

NKD now carries a dated timeline from 2012-11-18. The 16:15 -> 16:00 CT
close, which was previously undatable and forced this family to be modelled
current-only, is dated by CME Globex Notice #20150817 of 17 August 2015:

  "Effective Monday, September 21, the daily CME Globex maintenance period
   will begin 15 minutes earlier Monday through Thursday from 16:00 until
   16:45 Central Time (CT). ... With this change, the closing times for the
   following markets will now occur 15 minutes earlier Monday through Friday
   at 16:00 CT. CME Equity / CBOT Equity / COMEX / NYMEX / DME. All other CME
   Globex markets trading hours remain unchanged."

https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150817.html

NKD sits in the named "CME Equity" Globex product group, and CME's own NKD
contract-specification captures bracket the change directly: 2015-09-05 reads
"MON - FRI: 5:00 p.m. previous day - 4:15 p.m.", and 2015-11-27 reads
"5:00 p.m. - 4:00 p.m. Chicago Time/CT".
https://web.archive.org/web/20150905151851/http://www.cmegroup.com/trading/equity-index/international-index/nikkei-225-dollar_contract_specifications.html
https://web.archive.org/web/20151127190940/http://www.cmegroup.com:80/trading/equity-index/international-index/nikkei-225-dollar_contract_specifications.html

Revisions are keyed by the local session-opening day, matching `cme_group`:
the first close at 16:00 CT is trade date Monday 2015-09-21, whose session
opened Sunday 2015-09-20.

THE 2011 GRID, AND WHY IT IS NOT CARRIED TO THE FLOOR. CME's trading-hours
pages publish this grid — Electronic Trading (Sunday) "17:00-15:15" and
(Weekday) "15:30-16:30, 17:00-15:15", byte-identical to the E-mini S&P 500 row
beside it — from the 2011-01-12 capture onward. SER-6465 corroborates the
outgoing 15:15 CT close by describing its own change as an extension of it.

It is NOT carried back to the January-2010 floor, because the 2010 grid was
materially different and is sourced. The 2010-03-10 and 2010-04-07 captures of
the same page read, for "Nikkei 225 (Dollar) Futures":

  Electronic (weekday)  CDT: 03:00-15:15 reopens 15:30-16:30; closes
                        16:30-17:00; reopens 17:00-18:00
                        CST: 02:00-15:15; reopens 15:30-16:30; closes 16:30
  Sunday                CDT: Opens 17:00-18:00    CST: No Sunday Hours

That is a daytime-anchored, DST-dependent grid whose evening segment ran only
17:00-18:00 and which had no Sunday session at all in CST. Serving the
17:00-15:15 continuous grid across it would report the contract open through
the whole overnight window when it was closed — a false open, in executable
hours. An earlier revision of this module did exactly that, on the reasoning
that no source named a cutover; a source does, and the carry-back convention
requires that none exists.

The transition is undated: 2010-04-07 still shows the old grid and 2011-01-12
already shows the new one, with no capture and no located CME notice in
between. Dates before the first sourced appearance of the served grid are
therefore modelled sessionless, the same knowledge boundary a launch day
provides. Encoding the 2010 grid itself would need seasonal CDT/CST rules and
a boundary that is still undated, so it is left as sourced-but-unmodelled and
recorded here.
Official origin http://www.cmegroup.com/trading_hours/ delivered via:
https://web.archive.org/web/20100310022002id_/http://www.cmegroup.com/trading_hours/
https://web.archive.org/web/20100407094843id_/http://www.cmegroup.com/trading_hours/
https://web.archive.org/web/20110112032949id_/http://www.cmegroup.com/trading_hours/
https://web.archive.org/web/20110811113223id_/http://www.cmegroup.com/trading_hours/

2012-11-18: "CME Group announces that the new daily trading hour schedule for
  CBOT and CME Equity Index futures and Options on Equity Index futures will
  begin on Sunday, November 18, 2012 for trade date Monday, November 19, 2012."
  https://www.cmegroup.com/rulebook/files/ser-6465_Equity_Index_Futures_Options_on_Equity_Index_Futures_20121114.pdf
2013-03-03: "The modified Globex trading hours will be effective Monday,
  March 4, 2013. The 15 minute trading halt between 3:15 p.m. and 3:30 p.m.,
  Central Time, Monday through Friday, will be eliminated for CME
  International Equity [Index futures] ..." - NKD is named explicitly as
  "Nikkei 225 Dollar Futures". Keyed to the Sunday session-opening day.
  https://www.cmegroup.com/rulebook/files/ser_6554R_-_CME_Modifies_Trading_Hours_for_International_Equity_Index_futures_on_20130304.pdf
2015-09-20: CME Globex Notice #20150817, quoted above; trade date Monday
  2015-09-21, session-opening day Sunday 2015-09-20.
