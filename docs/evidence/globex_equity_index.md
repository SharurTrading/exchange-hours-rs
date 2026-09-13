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

- **order-entry** — the Sunday Pre-Open's move from 16:15 to 16:00 CT has no operator-stated effective day. The 2026-08-31 review narrowed the bracket to 2012-05-28..2012-06-07 from CME's own trading-hours captures and read both CME dated notice channels in full across that window without finding an announcement, so the dated profiles serve the sourced 16:15–17:00 CT intersection and withhold only the 16:00–16:15 CT quarter-hour. Closing condition: a CME document that states the new Sunday Pre-Open in session language on a day-level effective date. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES). Horizon 2012-05-03: below that capture the Sunday 16:15–17:00 CT queue is carried, not sourced.
- **residual risk** — the only Sunday inside the narrowed bracket is 2012-06-03; that is an observation about the bracket, not a source-stated effective day, so LAW-NO-FABRICATED-DATES keeps it out of the tables.
- **scope** — the family is the scoped modern CME/CBOT U.S.-grid set including YM and MYM; full-size `SP`, NKD, BTIC and TACO are excluded.

> Shared module. The narrative for
> [`cme_group.rs`](../../src/calendar/schedules/futures/us/cme_group.rs) lives in
> [`cme`](cme.md#module-narrative-moved-from-srccalendarschedulesfuturesuscme_grouprs-on-2026-09-12-utc).
> Sibling identities: [`cme`](cme.md).

## Evidence documents

Every id below resolves to one saved artifact behind this file's holiday rows.
The 2010-2012 ids are CME Group's own holiday-calendar PDFs at tier T1, retrieved
through the Internet Archive and saved; the 2025-2027 ids are responses of CME's
own `trading-hours-by-product` service at tier T2. Byte counts, capture times and
sha256 are in each id's row, so a row can be re-verified from this file together
with the research store's `holidays/raw/` indexes.

| Document | Window | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|
| `2010-martin-luther-king.pdf @2010-03-31T06:42:26Z` | `2010-martin-luther-king.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-martin-luther-king.pdf, capture 2010-03-31T06:42:26Z, 54521 bytes | T1 | `11af96423f788565a434209ebb94a82f69dd2c129621927cd898c2ff6c0978d3` |
| `2010-presidents-day.pdf @2010-02-15T06:46:41Z` | `2010-presidents-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-presidents-day.pdf, capture 2010-02-15T06:46:41Z, 49335 bytes | T1 | `ba379a7fa57efef43820583ada0002ea6cd8ccf0caf1b650d1cb6e8561f84253` |
| `2010-good-friday.pdf @2010-06-01T11:19:16Z` | `2010-good-friday.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-good-friday.pdf, capture 2010-06-01T11:19:16Z, 44125 bytes | T1 | `d196ca746c20ecd416d38f8f95020e2e7d6cb7fa9ead089e0d58c88bed0ab1f5` |
| `2010-memorial-day.pdf @2010-06-01T09:42:25Z` | `2010-memorial-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-memorial-day.pdf, capture 2010-06-01T09:42:25Z, 90478 bytes | T1 | `46a2f00f0f23c82189d86953092ee1171e0ec1c460891bda3ef0578518d12859` |
| `2010-4th-of-july.pdf @2010-06-02T00:56:37Z` | `2010-4th-of-july.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-4th-of-july.pdf, capture 2010-06-02T00:56:37Z, 49205 bytes | T1 | `c6a0f8c0b079b85e4500d30d942247bc9aa14d6c70fe3ed73d6d77b21b65ee2c` |
| `2010-labor-day.pdf @2010-06-02T00:56:41Z` | `2010-labor-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-labor-day.pdf, capture 2010-06-02T00:56:41Z, 48708 bytes | T1 | `2aecfe737c9613f82b975a01812607e96c9284638e8bd9b9bd403edfae510620` |
| `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | `2010-thanksgiving.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2010-thanksgiving.pdf, capture 2010-11-22T09:40:12Z, 97946 bytes | T1 | `4732afab4ca78ce21b3640f8ac41ced714123179c7cee1cb2b8c044bf9f2e2b5` |
| `2011-martin-luther-king.pdf @2011-10-28T02:34:29Z` | `2011-martin-luther-king.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-martin-luther-king.pdf, capture 2011-10-28T02:34:29Z, 126115 bytes | T1 | `2e389e2688d6760705220a11657329e76a9eb3a88d78664b4775eb7481be7b17` |
| `2011-presidents-day.pdf @2011-10-28T02:35:16Z` | `2011-presidents-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-presidents-day.pdf, capture 2011-10-28T02:35:16Z, 146317 bytes | T1 | `0342359e135acada5cfaa1b74f806477f759924a0b40625e035242c7e81321d4` |
| `2011-memorial-day.pdf @2013-09-30T10:56:52Z` | `2011-memorial-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-memorial-day.pdf, capture 2013-09-30T10:56:52Z, 66172 bytes | T1 | `5482f7bf47e0ee61448cf5f60fd4a5373cc39cb0e46220150c1f6a2ab2d6caec` |
| `2011-4th-of-july.pdf @2011-11-01T14:40:54Z` | `2011-4th-of-july.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-4th-of-july.pdf, capture 2011-11-01T14:40:54Z, 126279 bytes | T1 | `4c3bfbbe927ed799006edce76b5f263b6299bab29be7ff1eb06326e184a9b443` |
| `2011-labor-day.pdf @2011-11-01T14:43:45Z` | `2011-labor-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-labor-day.pdf, capture 2011-11-01T14:43:45Z, 129524 bytes | T1 | `03f38fea761a6da7633c3e636a40de61431c866a87f2229f270685bf9bb4470b` |
| `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | `2011-thanksgiving.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-thanksgiving.pdf, capture 2011-11-24T18:52:46Z, 131748 bytes | T1 | `bf75c3e0d3e18cbb8599458574bc7232513b737a664582454a7b34ca2b6caeb5` |
| `2011-christmas.pdf @2012-01-25T02:05:48Z` | `2011-christmas.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2011-christmas.pdf, capture 2012-01-25T02:05:48Z, 108183 bytes | T1 | `a0d34878fd70534afb2e0a2585a04ce1efc8c4aa0451575266cfb5f9dcf08029` |
| `2012-new-years.pdf @2012-01-25T02:54:30Z` | `2012-new-years.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-new-years.pdf, capture 2012-01-25T02:54:30Z, 130304 bytes | T1 | `aa8593edfde40a70ce2ab4818cd8984082eaeaa4a4893e2a8f3e3dc9ef4b1347` |
| `2012-martin-luther-king.pdf @2012-05-05T16:15:26Z` | `2012-martin-luther-king.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-martin-luther-king.pdf, capture 2012-05-05T16:15:26Z, 153369 bytes | T1 | `458c94af0fd7b7d8445c229de6bbe05648ea588c2fea88180eefe46486e4d265` |
| `2012-presidents-day.pdf @2012-05-05T16:15:39Z` | `2012-presidents-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-presidents-day.pdf, capture 2012-05-05T16:15:39Z, 208816 bytes | T1 | `4b5daad7813d41cdd9a104cca3a656bf303672f03c6c51ff5a13771d840ef131` |
| `2012-good-friday.pdf @2012-05-05T16:16:49Z` | `2012-good-friday.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-good-friday.pdf, capture 2012-05-05T16:16:49Z of the canonical URL (the research store's INDEX.md still records the byte-identical query-string-variant capture 2012-04-17T00:42:47Z), 60600 bytes | T1 | `81440c44afb97ea4b3a44b86aa4cf21e2e4cb7ba5839fabd95b29d0c928b2ea8` |
| `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | `2012-memorial-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-memorial-day.pdf, capture 2012-09-15T00:37:14Z, 70132 bytes | T1 | `5dc5cf9883232978ec1e80bd5bd50a2043535e93d7e52acddf4fd7e68938e848` |
| `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | `2012-4th-of-july.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-4th-of-july.pdf, capture 2012-09-15T00:39:23Z, 68805 bytes | T1 | `9b35b802ff0e399226ac0811761fc7e03487d8dec401c19a7e383750cbca5faf` |
| `2012-labor-day.pdf @2012-09-15T00:34:37Z` | `2012-labor-day.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-labor-day.pdf, capture 2012-09-15T00:34:37Z, 71440 bytes | T1 | `2f951bede1d6084977c8bed2f1cc4c993ebedd904aa0d56027668d2cb808ee39` |
| `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | `2012-thanksgiving.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-thanksgiving.pdf, capture 2013-01-27T22:39:01Z, 73205 bytes | T1 | `052e381bbd4eb0790c6d38e3866874738d6da081da62643e525c25674b2608e1` |
| `2012-christmas.pdf @2013-04-14T19:40:27Z` | `2012-christmas.pdf` | Internet Archive raw replay of http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-christmas.pdf, capture 2013-04-14T19:40:27Z, 125821 bytes | T1 | `de3b16aaae2ef887e46c965f902d8d0e43afa6e18dc1f721baaa40ea6b18b5e9` |
| `CME-SVC-2024-12-31` | 2024-12-31 .. 2025-01-02 | archive capture 2024-12-20T15:53:40Z | T2 | `375c70eecd19c5c6204ecb408d1b3210a9da4c9a03b85ef1c7dbbcde1397ab63` |
| `CME-SVC-2025-01-19` | 2025-01-19 .. 2025-01-21 | archive capture 2024-12-20T15:53:40Z | T2 | `4f2ab56af14e7b3a6978e7fa6db8e2cfc63a82d06428cd88844f0e5bcf534f40` |
| `CME-SVC-2025-02-16` | 2025-02-16 .. 2025-02-18 | archive capture 2024-12-20T15:53:40Z | T2 | `5bec2ca6b4999a534e4d9818035aaa18ec8626b6c912cf7e3d2c57015536f2fa` |
| `CME-SVC-2025-04-17` | 2025-04-17 .. 2025-04-19 | archive capture 2024-12-20T15:53:40Z | T2 | `865a1d4f08102e00151bd87ab2b8e8a7720e9203a17aaaba24627ade3ed26e74` |
| `CME-SVC-2025-05-25` | 2025-05-25 .. 2025-05-27 | archive capture 2024-12-20T15:53:40Z | T2 | `5f42869879c826f5949b79236aabb3d26d74e7565d92d7cc5e8784c63973210b` |
| `CME-SVC-2025-06-18` | 2025-06-18 .. 2025-06-20 | archive capture 2024-12-20T15:53:40Z | T2 | `a572706907175776255261103b393493ebdf5a8106ec5374d129145bdf89105e` |
| `CME-SVC-2025-07-03` | 2025-07-03 .. 2025-07-05 | archive capture 2024-12-20T15:53:40Z | T2 | `b80cd4bfed0ae72865bfacc1936e107eb8febfcc94b37fcce1d05505c659147b` |
| `CME-SVC-2025-08-31` | 2025-08-31 .. 2025-09-02 | archive capture 2024-12-20T15:53:40Z | T2 | `e075762ed34a86048d94900e10edba10d95b3d766052908ffbb5133f6b64bab0` |
| `CME-SVC-2025-11-26` | 2025-11-26 .. 2025-11-28 | archive capture 2026-01-29T01:23:09Z | T2 | `6c4c598791058dd9a11aff0ddb072c761a436c6d1054b891def74c6935f020f1` |
| `CME-SVC-2025-11-26-PRE` | 2025-11-26 .. 2025-11-28 | archive capture 2024-12-20T15:53:40Z | T2 | `34f38de416a997f7a1a095ce33261327c0b604abecdbe221a35305005ffa2764` |
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
| `CME-SVC-NORMALWEEK-2026-10-18` | 2026-10-18 .. 2026-10-24 | live retrieval 2026-09-12 | T2 | `d3bd6e890bdc427d2ebd3ece48dedc56eb231178be82c69ab3ac427f0b09dc0a` |
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
| `CME-TRADING-HOURS-2025-08-30` | n/a | archive capture 2025-08-30T02:14:20Z | T1 | `62fc524c79e0bd1269ad89b5fe1ae9b617c1b082243d4f5386c5d3fdf6ed5e58` |
| `CME-TRADING-HOURS-2026-09-12` | n/a | live retrieval 2026-09-12 | T1 | `ac85d05d1fcf2c6fc5afaad7bef5efa7bed407d53df7bdfc9bc724ae18c3449f` |

## Holidays

**Coverage:** 2010-01-01..2012-12-31, 2025-01-01..2027-12-31 (inclusive venue-local trade dates in
`America/Chicago`), audited per era: **T1** for 2010-2012 in the per-year sections
below and **T2** for 2025-2027. Inside the window a date with no row is audited
normal; outside it this table has no answer at all.

**Two audited eras, and a gap between them.** The table declares two coverage
windows: `2010-01-01..2012-12-31`, from this era's documents, and
`2025-01-01..2027-12-31`, from the trading-hours service. The 2013-2024 interval
is audited by neither — those years are the remaining stage-2.2 waves — so it
lies outside every declared window and `holiday_on` has **no answer** there
rather than reporting an unaudited date as normal. `HolidayCoverage::windows()`
lists the two, and `contains` answers per date.

### 2010

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2010-01-18 | early close | `1030 CT` | `2010-martin-luther-king.pdf @2010-03-31T06:42:26Z` | T1 | `1030 CT` is the date's own final close; CME prints trade date 2010-01-18 beside it |
| 2010-02-15 | early close | `1030 CT` | `2010-presidents-day.pdf @2010-02-15T06:46:41Z` | T1 | `1030 CT` is the date's own final close; CME prints trade date 2010-02-15 beside it |
| 2010-04-02 | early close | `0815 CT` | `2010-good-friday.pdf @2010-06-01T11:19:16Z` | T1 | `0815 CT` is the date's own final close; CME prints trade date 2010-04-02 beside it |
| 2010-05-31 | early close | `1030 CT` | `2010-memorial-day.pdf @2010-06-01T09:42:25Z` | T1 | `1030 CT` is the date's own final close; CME prints trade date 2010-05-31 beside it |
| 2010-07-05 | early close | `1030 CT` | `2010-4th-of-july.pdf @2010-06-02T00:56:37Z` | T1 | `1030 CT` is the date's own final close; CME prints trade date 2010-07-05 beside it |
| 2010-09-06 | early close | `1030 CT` | `2010-labor-day.pdf @2010-06-02T00:56:41Z` | T1 | `1030 CT` is the date's own final close; CME prints trade date 2010-09-06 beside it |
| 2010-11-25 | early close | `1030 CT` | `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | T1 | `1030 CT` is the date's own final close; CME prints trade date 2010-11-25 beside it |
| 2010-11-26 | early close | `1215 CT` | `2010-thanksgiving.pdf @2010-11-22T09:40:12Z` | T1 | `1215 CT` is the date's own final close; CME prints trade date 2010-11-26 beside it |

### 2011

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2011-01-17 | early close | `1030 CT` | `2011-martin-luther-king.pdf @2011-10-28T02:34:29Z` | T1 | `1030 CT` is the date's own final close; CME prints trade date 2011-01-17 beside it |
| 2011-02-21 | early close | `1030 CT` | `2011-presidents-day.pdf @2011-10-28T02:35:16Z` | T1 | `1030 CT` is the date's own final close; CME prints trade date 2011-02-21 beside it |
| 2011-05-30 | early close | `1030 CT` | `2011-memorial-day.pdf @2013-09-30T10:56:52Z` | T1 | `1030 CT` is the date's own final close; CME prints trade date 2011-05-30 beside it |
| 2011-07-04 | early close | `1030 CT` | `2011-4th-of-july.pdf @2011-11-01T14:40:54Z` | T1 | `1030 CT` is the date's own final close; CME prints trade date 2011-07-04 beside it |
| 2011-09-05 | early close | `1030 CT` | `2011-labor-day.pdf @2011-11-01T14:43:45Z` | T1 | `1030 CT` is the date's own final close; CME prints trade date 2011-09-05 beside it |
| 2011-11-24 | early close | `1030 CT` | `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | T1 | `1030 CT` is the date's own final close; CME prints trade date 2011-11-24 beside it |
| 2011-11-25 | early close | `1215 CT` | `2011-thanksgiving.pdf @2011-11-24T18:52:46Z` | T1 | `1215 CT` is the date's own final close; CME prints trade date 2011-11-25 beside it |
| 2011-12-27 | late open | `0500 CT` | `2011-christmas.pdf @2012-01-25T02:05:48Z` | T1 | `0500 CT` is the trade date's own first open; the evening leg that would have opened earlier did not run |

### 2012

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2012-01-03 | late open | `0500 CT` | `2012-new-years.pdf @2012-01-25T02:54:30Z` | T1 | `0500 CT` is the trade date's own first open; the evening leg that would have opened earlier did not run |
| 2012-01-16 | early close | `1030 CT` | `2012-martin-luther-king.pdf @2012-05-05T16:15:26Z` | T1 | `1030 CT` is the date's own final close; CME prints trade date 2012-01-16 beside it |
| 2012-02-20 | early close | `1030 CT` | `2012-presidents-day.pdf @2012-05-05T16:15:39Z` | T1 | `1030 CT` is the date's own final close; CME prints trade date 2012-02-20 beside it |
| 2012-04-06 | early close | `0815 CT` | `2012-good-friday.pdf @2012-05-05T16:16:49Z` | T1 | `0815 CT` is the date's own final close; CME prints trade date 2012-04-06 beside it |
| 2012-05-28 | early close | `1030 CT` | `2012-memorial-day.pdf @2012-09-15T00:37:14Z` | T1 | `1030 CT` is the date's own final close; CME prints trade date 2012-05-28 beside it |
| 2012-07-03 | early close | `1215 CT` | `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | T1 | `1215 CT` is the date's own final close; CME prints trade date 2012-07-03 beside it |
| 2012-07-04 | early close | `1030 CT` | `2012-4th-of-july.pdf @2012-09-15T00:39:23Z` | T1 | `1030 CT` is the date's own final close; CME prints trade date 2012-07-04 beside it |
| 2012-09-03 | early close | `1030 CT` | `2012-labor-day.pdf @2012-09-15T00:34:37Z` | T1 | `1030 CT` is the date's own final close; CME prints trade date 2012-09-03 beside it |
| 2012-11-22 | early close | `1030 CT` | `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | T1 | `1030 CT` is the date's own final close; CME prints trade date 2012-11-22 beside it |
| 2012-11-23 | early close | `1215 CT` | `2012-thanksgiving.pdf @2013-01-27T22:39:01Z` | T1 | `1215 CT` is the date's own final close; CME prints trade date 2012-11-23 beside it |
| 2012-12-24 | early close | `1215 CT` | `2012-christmas.pdf @2013-04-14T19:40:27Z` | T1 | `1215 CT` is the date's own final close; CME prints trade date 2012-12-24 beside it |
| 2012-12-26 | late open | `0500 CT` | `2012-christmas.pdf @2013-04-14T19:40:27Z` | T1 | `0500 CT` is the trade date's own first open; the evening leg that would have opened earlier did not run |

#### Gaps, 2010-2012

- **The two Good Friday eves ship no row.** CME's 2010 and 2012 Good Friday
  sheets print `1530 CT - Regular CME Globex open for trade date Thursday, Apr 1`
  (and the 2012 twin for 04-05) after a 15:15 CT close. That instant is the
  trade date's *own* evening open at the start of the next trading day, not a
  delayed first open: the crate's 2010-2012 grid already ends the eve at 15:15
  CT and reopens at the ordinary 17:00 CT. A `LateOpen` row at 15:30 would land
  after the trade date's own close and delete ~19 hours of sourced trading, so
  the two dates ship nothing and are audited normal. Closing condition: a
  vocabulary that can state a same-day re-open after a close (#93).
- **2012-07-03's 15:30 CT re-open.** The same sheet prints
  `1530 CT - Regular CME Globex open for trade date Thursday, July 5`, so the
  07-03 row ships only its 12:15 CT early close; the evening leg CME states for
  07-05 is not modelled and is this window's one unrepresentable instant.
  Closing condition: as above.

### 2025-2027 (T2)

Every instant is quoted exactly as the service prints it. CME states the zone once, verbatim: "Trading hours are subject to change and are in U.S. Central Time unless otherwise stated." This channel prints no Eastern column, so no ET value is asserted anywhere below; the `CT` token in each row is this file's editorial expansion of that sentence, not text CME printed beside the instant.

**Vocabulary.** CME publishes ten headline product groups; this family is the group it prints as **Equity Index**, queried as `ES` (E-mini S&P 500). The family is the scoped modern CME/CBOT U.S.-grid equity-index set including YM and MYM; the separately printed Nikkei lines (`NKD`, `NIY`) are `globex_nikkei_225_dollar` and are not recorded here, even where they track this family's instants.

**Event types, verbatim from the operator.** `closed` — "Final Close of the date. Day and GTD (current trade date) orders are eliminated." `preopen` — "Order Entry, modification, and cancel are allowed. No order matching." `open` — "Start of continuous trading phase. Order matching begins."

**The conversion, once.** Rows are keyed by the crate's venue-local trade date, never by the operator's event date (design memo D1). This family's trading day for trade date `D` opens 17:00 CT on the preceding business evening and ends at its 16:00 CT final close on `D`, so an eve record that merely lacks its evening leg is evidence for the *following* date's `closed` row and never a row of its own.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `16:00 preopen; 17:00 open` | `CME-SVC-2024-12-31` | T2 | eventDate 2025-01-01, CME trade date 2025-01-02 on both events; note N17 |
| 2025-01-20 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2025-01-19` | T2 | eventDate 2025-01-20, CME trade date 2025-01-21; note N15 |
| 2025-02-17 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2025-02-16` | T2 | eventDate 2025-02-17, CME trade date 2025-02-18; note N15 |
| 2025-04-18 | closed | `no events published` | `CME-SVC-2025-04-17` | T2 | eventDate 2025-04-18, no CME trade date; note N1 |
| 2025-05-26 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2025-05-25` | T2 | eventDate 2025-05-26, CME trade date 2025-05-27; note N15 |
| 2025-06-19 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2025-06-18` | T2 | eventDate 2025-06-19, CME trade date 2025-06-20; note N15 |
| 2025-07-03 | early close | `12:15 closed` - 12:15 CT | `CME-SVC-2025-07-03` | T2 | eventDate 2025-07-03, CME trade date 2025-07-03; the evening leg runs normally |
| 2025-07-04 | early close | `12:00 closed` - 12:00 CT | `CME-SVC-2025-07-03` | T2 | eventDate 2025-07-04, CME trade date 2025-07-04 |
| 2025-09-01 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2025-08-31` | T2 | eventDate 2025-09-01, CME trade date 2025-09-02; note N15 |
| 2025-11-27 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2025-11-26` | T2 | eventDate 2025-11-27, CME trade date 2025-11-28; note N15 |
| 2025-11-28 | early close | `07:00 preopen; 07:30 open; 12:15 closed` - 12:15 CT | `CME-SVC-2025-11-26` | T2 | eventDate 2025-11-28, CME trade date 2025-11-28; the morning pair is a declared gap below |
| 2025-11-29 | closed | `no events published` | `CME-SVC-2025-11-26-SAT` | T2 | eventDate 2025-11-29; all ten products publish an empty schedule, and CME's 2025 Globex table states the period as "27 - 29 November 2025" |
| 2025-12-24 | early close | `12:15 closed` - 12:15 CT | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-24, CME trade date 2025-12-24; no evening re-open |
| 2025-12-25 | closed | `16:00 preopen; 17:00 open` | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-25, CME trade date 2025-12-26 on both events; note N17 |

**Interpretive steps, 2025.** On the Monday and Thursday holidays (MLK, Presidents', Memorial, Juneteenth, Labor, Thanksgiving) CME publishes no `closed` event at all: it prints `12:00 preopen`, matching stops, and it carries the whole Sunday- or Wednesday-evening-through-holiday span under the *following* business day's trade date. The crate keys a trading day by its final close, so that span is trade date 2025-01-20 (and its siblings) and the printed 12:00 CT instant is where it ends. The row is therefore an early close, exactly as design memo section 1.1 works it. The consequence to state plainly: on these dates the crate assigns the morning's trading to the holiday's own trade date while CME assigns it to the next business day. That is a trade-date divergence, not an hours divergence - `is_open` agrees with the operator minute for minute.

The eve records CME labels `modified` ship no row. 2024-12-31 and 2025-04-17 each print `16:00 closed` and no evening re-open; that is the family's normal grid minus the evening leg, and the leg is already removed by the `closed` row on the following trade date (2025-01-01, 2025-04-18) because the leg's trade date is that date. 2024-12-31 is below the coverage window in any case.

Columbus Day and Veterans Day appear nowhere in CME's Globex holiday list and Globex trades a normal session on both, so inside this window they read as audited normal with no row, which is correct rather than accidental.

**Gaps, 2025.**

- **order entry, not representable.** On 2025-01-01 and 2025-12-25 the pre-open opens 16:00 CT instead of the family's normal 16:45 CT. The holiday vocabulary is `DayPolicy`'s and has no order-entry boundary, so this cannot be stated. It changes no `is_open` answer, only `is_accepting_orders` for 45 minutes.
- **intraday topology, sourced and not representable.** CME's finalised publication for 2025-11-28 additionally prints `07:00 preopen; 07:30 open` ahead of the early close. 07:30 CT is *earlier* than the family's 08:30 CT regular open, so it is not a late open and the scalar vocabulary cannot state it. The 12:15 CT final close is unaffected and is what the row carries.
- **Saturday 2025-11-29 ships a row.** The live service publishes a 2025-11-29 schedule for all ten products with no events (`CME-SVC-2025-11-26-SAT`), and CME's 2025 Globex table states the holiday period as "27 - 29 November 2025". The family's normal week has no Saturday session, so the row changes no answer here — but one audited operator closure ships in every family that routes to the venue, so the D17 venue intersection for `Cme`, `Cbot`, `Comex` and `Nymex` is computed from one uniform input rather than from eight family judgements.
- **residual risk, sourcing vintage.** The eight windows New Year 2025 through Labor Day 2025 rest on a single pre-holiday capture, 2024-12-20T15:53:40Z, and CME states on the same page that hours are usually finalised about two weeks before a holiday. The service's own retention edge falls between Labor Day 2025 and Thanksgiving 2025, so the channel cannot restate them. They are internally consistent with the 2026 and 2027 rows for the same holidays, which are sourced from post-holiday captures and live reads.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `16:00 preopen; 17:00 open` | `CME-SVC-2025-12-31` | T2 | eventDate 2026-01-01, CME trade date 2026-01-02 on both events; note N17 |
| 2026-01-19 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2026-01-18` | T2 | eventDate 2026-01-19, CME trade date 2026-01-20; note N15 |
| 2026-02-16 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2026-02-15` | T2 | eventDate 2026-02-16, CME trade date 2026-02-17; note N15 |
| 2026-04-03 | early close | `08:15 closed` - 08:15 CT | `CME-SVC-2026-04-01` | T2 | eventDate 2026-04-03, CME trade date 2026-04-03; corroborated at T1 by CME-TRADING-HOURS-2025-08-30, which states no instant |
| 2026-05-25 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2026-05-24` | T2 | eventDate 2026-05-25, CME trade date 2026-05-26; note N15 |
| 2026-06-19 | early close | `12:00 closed` - 12:00 CT | `CME-SVC-2026-06-18` | T2 | eventDate 2026-06-19, CME trade date 2026-06-22; see the 2026 interpretive steps |
| 2026-07-03 | early close | `12:00 closed` - 12:00 CT | `CME-SVC-2026-07-03` | T2 | eventDate 2026-07-03, CME trade date 2026-07-06; see the 2026 interpretive steps |
| 2026-09-07 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2026-09-06` | T2 | eventDate 2026-09-07, CME trade date 2026-09-08; note N15 |
| 2026-11-26 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2026-11-25` | T2 | eventDate 2026-11-26, CME trade date 2026-11-27; note N15 |
| 2026-11-27 | early close | `12:15 closed` - 12:15 CT | `CME-SVC-2026-11-25` | T2 | eventDate 2026-11-27, CME trade date 2026-11-27 |
| 2026-12-24 | early close | `12:15 closed` - 12:15 CT | `CME-SVC-2026-12-22` | T2 | eventDate 2026-12-24, CME trade date 2026-12-24; no evening re-open |
| 2026-12-25 | closed | `no events published` | `CME-SVC-2026-12-24` | T2 | eventDate 2026-12-25, no CME trade date; note N1 |

**Interpretive steps, 2026.** Good Friday 2026 is the one date CME flags itself, and it is the only row in this window corroborated at T1. From the operator page: "Due to the US Employment Situation Release on April 3, 2026, CME Group Equities, FX, Cryptocurrency and Interest Rate products will have unique Closes and Settlements for trade date April 3rd", and, of everything else, "No trading for Friday April 3th  trade date in observence of Good Friday." (sic, twice). That page states no instants; the 08:15 CT close comes from the service alone. 08:15 CT is earlier than the family's own 08:30 CT regular open, so the row removes the whole day session and leaves only the Thursday-evening leg.

On 2026-06-19 and 2026-07-03 the `12:00 closed` event carries trade date 2026-06-22 and 2026-07-06 - the following Monday, not the Friday. In CME's terms the holiday has no trade date of its own. The crate cannot say that and keep the trading: a `closed` row would delete the Thursday-evening-through-Friday-noon span that did trade. The row is an early close on the Friday, and the divergence from the operator's printed trade date is recorded here. This is the same shape as the Monday holidays above and is resolved the same way.

2026-12-31 prints `16:00 closed` with no evening re-open: the normal grid minus the leg that `closed` on 2027-01-01 already removes. No row.

**Gaps, 2026.**

- **Saturday sessions, sourced and not representable.** 2026-06-20 and 2026-07-04 each publish `05:00 open; 17:00 closed`, carrying trade date 2026-06-22 and 2026-07-06. The family's normal week has no Saturday session, and a late open can only push an existing occurrence later, never create one (design memo D7). Both are declared gaps; the block-row follow-up named in the memo's section 7 (#93) is what would turn them into rows. Saturday 2026-04-04, after Good Friday 2026, carries no such session - checked against the service for window 2026-04-02..04.
- **order entry, not representable.** 2026-01-01 repeats the 16:00 CT pre-open of the 2025 New Year rows.

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `no events published` | `CME-SVC-2026-12-31` | T2 | eventDate 2027-01-01, no CME trade date; note N1 |
| 2027-01-18 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2027-01-17` | T2 | eventDate 2027-01-18, CME trade date 2027-01-19; note N15 |
| 2027-02-15 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2027-02-14` | T2 | eventDate 2027-02-15, CME trade date 2027-02-16; note N15 |
| 2027-03-26 | closed | `no events published` | `CME-SVC-2027-03-25` | T2 | eventDate 2027-03-26, no CME trade date; note N1 |
| 2027-05-31 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2027-05-30` | T2 | eventDate 2027-05-31, CME trade date 2027-06-01; note N15 |
| 2027-06-18 | early close | `12:00 closed` - 12:00 CT | `CME-SVC-2027-06-17` | T2 | eventDate 2027-06-18, CME trade date 2027-06-21; see the 2027 interpretive steps |
| 2027-07-05 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2027-07-04` | T2 | eventDate 2027-07-05, CME trade date 2027-07-06; note N15 |
| 2027-09-06 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2027-09-05` | T2 | eventDate 2027-09-06, CME trade date 2027-09-07; note N15 |
| 2027-11-25 | early close | `12:00 preopen` - 12:00 CT | `CME-SVC-2027-11-24` | T2 | eventDate 2027-11-25, CME trade date 2027-11-26; note N15 |
| 2027-11-26 | early close | `12:15 closed` - 12:15 CT | `CME-SVC-2027-11-24` | T2 | eventDate 2027-11-26, CME trade date 2027-11-26 |
| 2027-12-24 | closed | `no events published` | `CME-SVC-2027-12-22` | T2 | eventDate 2027-12-24, no CME trade date; note N1 |

**Interpretive steps, 2027.** 2027-03-25 and 2027-12-23 each print `16:00 closed` and no evening re-open - the normal grid minus a leg that the `closed` row on the following trade date (2027-03-26, 2027-12-24) already removes. Neither ships a row. CME keys its Christmas 2027 holiday to Thursday 2027-12-23; Globex is shut on Friday 2027-12-24 and 25 December falls on a Saturday, so the crate's row is the Friday.

2027-06-18 repeats the 2026 Friday-holiday shape: `12:00 closed` carrying trade date 2027-06-21. The row is an early close on the Friday, for the reason given under 2026.

2027-12-31 is a normal Friday - a 16:00 CT final close, and the normal Friday grid has no evening leg either - so it ships no row.

**Gaps, 2027.**

- **Saturday session, sourced and not representable.** 2027-06-19 publishes `05:00 open; 17:00 closed` carrying trade date 2027-06-21. Same class as the two 2026 Saturdays, same declared gap.
- **coverage edge.** CME's published future runs to 2028-01-01 and the table stops at 2027-12-31, so trade date 2028-01-03 - the first trading day of 2028 - is outside coverage and the crate has no holiday answer for it. 2028-01-01 itself is a Saturday on which CME publishes nothing, which is the ordinary Saturday answer.
- **no T1 rendering.** No per-asset-class T1 rendering could be driven out of the operator page for any holiday in this window except Thanksgiving 2026, which was used to validate the service group by group. Every row here is T2.
